use anyhow::bail;
use chrono::Utc;
use regex::Regex;
use sha2::{Digest, Sha256};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use schemadoc_diff::checker::validate;
use schemadoc_diff::core::DiffResult;
use schemadoc_diff::schema_diff::HttpSchemaDiff;

use crate::app_state::AppState;
use crate::settings::Settings;
use crate::storage::Storer;

use crate::models::{ProjectSlug, Version};
use crate::{alerts, branches, dependencies, versions};

pub struct CreatedVersion {
    pub version: Version,
    pub diff: HttpSchemaDiff,
    pub src_version_id: u32,
    pub src_branch_name: String,
}

async fn create_version_inner(
    state: &mut AppState,
    project_slug: &ProjectSlug,
    branch_name: &str,
    message: Option<String>,
    content: &str,
) -> anyhow::Result<Option<CreatedVersion>> {
    let (src_branch_name, src_version) = get_source_version(state, project_slug, branch_name)?;

    let Some(project) = state.projects.get_mut(project_slug) else {
        bail!("Project versions not found")
    };

    let Some(branch) = branches::get_branch_mut(project, branch_name) else {
        bail!("Project branch not found")
    };

    let diff = match src_version.as_ref() {
        Some(src_version) => {
            let src_schema_content = state.storage.read_file(&src_version.file_path).await?;
            compare_schemas_content(&String::from_utf8_lossy(&src_schema_content), content)?
        }
        None => {
            // For first version compare to itself
            compare_schemas_content(content, content)?
        }
    };

    // Skip this version if it has no changes and there are any versions before it
    if diff.is_same_or_none() && src_version.is_some() {
        return Ok(None);
    }

    let diff = diff.take().expect(
        "Root diff of two schemas must not be empty.\
             Probably two null schemas were provided.",
    );

    let hash = Sha256::digest(content);
    // Write versions to shared folder, in that way we are caching them,
    //  but someday we need to add check for collisions here
    let file_path = format!("projects/{project_slug}/versions/{hash:x}.json");
    if !state.storage.exists(&file_path).await? {
        state
            .storage
            .put_file(&file_path, content.as_bytes())
            .await?;
    }

    let next_id = branch
        .versions
        .iter()
        .max_by_key(|v| v.id)
        .map(|v| &v.id + 1)
        .unwrap_or(0);

    let sanitized_branch_name = _sanitise_branch_name(branch_name);

    let diff_file_path =
        format!("projects/{project_slug}/diffs/{sanitized_branch_name}/{next_id}.json");

    state
        .storage
        .put_file(&diff_file_path, &serde_json::to_vec(&diff)?)
        .await?;

    let diff_file_version = diff.get_diff_version();

    let version = diff
        .info
        .get()
        .and_then(|info| info.version.get().map(|v| v.to_owned()));

    let statistics = versions::statistics::get_diff_statistics(&diff);

    let version = Version {
        id: next_id,
        version,
        message,
        file_path,
        statistics,
        diff_file_path,
        diff_file_version,
        created_at: Utc::now(),
    };

    branch.versions.push(version.clone());

    project.persist_branches(&state.storage).await?;

    let src_version_id = src_version.as_ref().map(|lv| lv.id).unwrap_or(next_id);

    Ok(Some(CreatedVersion {
        diff,
        version,
        src_version_id,
        src_branch_name,
    }))
}

fn get_source_version(
    state: &AppState,
    project_slug: &ProjectSlug,
    branch_name: &str,
) -> anyhow::Result<(String, Option<Version>)> {
    let Some(project) = state.projects.get(project_slug) else {
        bail!("Project versions not found")
    };

    let Some(branch) = branches::get_branch(project, branch_name) else {
        bail!("Project branch not found")
    };
    // Use latest version from branch as source version
    let src_version = branch.versions.iter().max_by_key(|v| v.id);

    if src_version.is_some() {
        return Ok((branch.name.clone(), src_version.cloned()));
    }
    // Use version specified as branch base version from base branch

    let Some(base) = &branch.base else {
        return Ok((branch.name.clone(), None));
    };

    let Some(base_branch) = branches::get_branch(project, &base.name) else {
        bail!(
            "Base branch {} not found for {}/{}",
            base.name,
            project_slug,
            branch_name
        )
    };

    let base_version = base_branch
        .versions
        .iter()
        .find(|v| v.id == base.version_id);

    Ok((base_branch.name.clone(), base_version.cloned()))
}

pub async fn create_version(
    settings: &Settings,
    state: &mut AppState,
    project_slug: &ProjectSlug,
    branch_name: &str,
    message: Option<String>,
    content: &str,
) -> anyhow::Result<Option<Version>> {
    let result = create_version_inner(state, project_slug, branch_name, message, content).await?;

    let Some(result) = result else {
        return Ok(None);
    };

    let src_projects_slugs =
        dependencies::update_dependent_projects(state, project_slug, Some(branch_name)).await?;

    //handle alerts
    let project = state
        .projects
        .get(project_slug)
        .expect("Project must not be removed during add version operation.");

    let validations = validate(&result.diff, &["*"]);

    // own alerts
    if !project.alerts.is_empty() {
        let alerts = alerts::get_own_alerts_info(
            settings,
            project,
            branch_name,
            result.version.id,
            &result.diff,
            &validations,
        )
        .await?;
        for alert in alerts {
            println!("Send own alert: {}", alert.service);
            alerts::send_alert(alert).await?;
        }
    };

    // deps alerts
    let dep_projects: Vec<_> = src_projects_slugs
        .iter()
        .filter_map(|slug| state.projects.get(slug))
        .collect();
    if !dep_projects.is_empty() {
        let alerts = alerts::get_deps_alerts_info(
            settings,
            project,
            &result.src_branch_name,
            result.src_version_id,
            branch_name,
            result.version.id,
            dep_projects,
            &result.diff,
            &validations,
        )
        .await?;

        for alert in alerts {
            println!("Send deps alert: {}", alert.service);
            alerts::send_alert(alert).await?;
        }
    }

    Ok(Some(result.version))
}

pub async fn compare_versions(
    state: &AppState,
    project_slug: &ProjectSlug,
    src_branch_name: &str,
    src_id: u32,
    tgt_branch_name: &str,
    tgt_id: u32,
) -> anyhow::Result<DiffResult<HttpSchemaDiff>> {
    let src_version = versions::crud::get_version(state, project_slug, src_branch_name, src_id)
        .ok_or(anyhow::Error::msg("Source version not found"))?;
    let tgt_version = versions::crud::get_version(state, project_slug, tgt_branch_name, tgt_id)
        .ok_or(anyhow::Error::msg("Target version not found"))?;

    let storage = &state.storage;

    let src_schema_content = storage.read_file(&src_version.file_path).await?;
    let tgt_schema_content = storage.read_file(&tgt_version.file_path).await?;

    let diff = compare_schemas_content(
        &String::from_utf8_lossy(&src_schema_content),
        &String::from_utf8_lossy(&tgt_schema_content),
    )?;

    Ok(diff)
}

pub fn compare_schemas_content(
    src_schema_content: &str,
    tgt_schema_content: &str,
) -> Result<DiffResult<HttpSchemaDiff>, schemadoc_diff::error::Error> {
    let (src_schema, tgt_schema) =
        schemadoc_diff::try_deserialize_schema(src_schema_content, tgt_schema_content)?;

    let diff = schemadoc_diff::get_schema_diff(src_schema, tgt_schema);

    Ok(diff)
}

fn _sanitise_branch_name<S: Into<String>>(input: S) -> String {
    let input = input.into();
    let re = Regex::new(r"[^0-9a-zA-Z]").unwrap();
    let replaced = re.replace_all(&input, "-").to_string();

    // calculate input str hash
    let mut s = DefaultHasher::new();
    input.hash(&mut s);
    let hash = s.finish();

    // add hash suffix to replaced branch name
    format!("{}-{}", replaced, hash)
}

#[cfg(test)]
mod tests {
    use crate::versions::services::_sanitise_branch_name;

    #[test]
    fn test_encode_branch_name() {
        let values = vec![
            (
                "feat: ad:d new functions123",
                "feat--ad-d-new-functions123-3847141747850018672",
            ),
            (
                "feat/ Add New+Functions",
                "feat--Add-New-Functions-17811559900772270264",
            ),
            (
                "feat /Add New:Functions",
                "feat--Add-New-Functions-7569046335700543031",
            ),
        ];

        for (input, result) in values {
            assert_eq!(_sanitise_branch_name(input), result)
        }
    }
}
