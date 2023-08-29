use crate::app_state::AppState;
use crate::models::{Branch, BranchBase, Project, ProjectSlug};
use crate::storage::Storer;
use anyhow::bail;
use async_recursion::async_recursion;
use regex::Regex;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub fn get_branch<'p>(project: &'p Project, branch_name: &str) -> Option<&'p Branch> {
    project.branches.iter().find(|b| b.name == branch_name)
}

pub fn get_branch_mut<'p>(project: &'p mut Project, branch_name: &str) -> Option<&'p mut Branch> {
    project.branches.iter_mut().find(|b| b.name == branch_name)
}

pub async fn create_branch_if_not_exists<'p, S>(
    state: &'p mut AppState,
    project_slug: &ProjectSlug,
    branch_name: &str,
    base_name: Option<S>,
    base_version_id: Option<u32>,
) -> anyhow::Result<Option<&'p Branch>>
where
    S: Into<String>,
{
    let Some(project) = state.projects.get_mut(project_slug) else {
        bail!("Project {} not found", project_slug);
    };

    let exists = project
        .branches
        .iter()
        .any(|branch| branch.name == branch_name);
    if exists {
        return Ok(None);
    }

    let base = get_branch_base(project, base_name, base_version_id).await?;

    let version = project
        .branches
        .iter()
        .find(|b| b.name == base.name)
        .and_then(|b| b.versions.iter().find(|v| v.id == base.version_id));

    let Some(_) = version else {
        bail!(
            "Source branch {}/{} must contain specified version {} to fork it",
            project_slug,
            base.name,
            base.version_id
        )
    };

    let branch = Branch {
        versions: vec![],
        base: Some(base),
        name: branch_name.to_string(),
    };

    project.branches.push(branch);

    let branch = project
        .branches
        .last()
        .expect("Just inserted one to branches");

    project.persist_branches(&state.storage).await?;

    Ok(Some(branch))
}

#[async_recursion]
pub async fn delete_branch(
    state: &mut AppState,
    project_slug: &ProjectSlug,
    branch_name: &str,
    force: bool,
    persist: bool,
) -> anyhow::Result<()> {
    let Some(project) = state.projects.get(project_slug) else {
        bail!("Project {} not found", project_slug);
    };

    let forks: Vec<_> = project
        .branches
        .iter()
        .filter_map(|b| {
            b.base
                .as_ref()
                .filter(|bb| bb.name == branch_name)
                .map(|_| b.name.clone())
        })
        .collect();

    println!(
        "Forks {} of {}/{} with force={}",
        forks.len(),
        project_slug,
        branch_name,
        force
    );

    if !forks.is_empty() && !force {
        bail!(
            "`{}` of {} is base for other branches. Remove children branches at first",
            branch_name,
            project_slug
        )
    }

    for fork in forks {
        delete_branch(state, project_slug, &fork, true, false).await?;
    }

    let Some(project) = state.projects.get_mut(project_slug) else {
        bail!("Project {} not found", project_slug);
    };
    let Some(branch) = get_branch(project, branch_name) else {
        bail!("Fork `{}` of {} not found", branch_name, project_slug);
    };

    //remove branch versions diff files
    for version in &branch.versions {
        state.storage.remove_file(&version.diff_file_path).await?;
    }

    // remove branch from branch list
    project.branches.retain(|b| b.name != branch_name);

    // remove branch versions schema files
    let files_paths_iter = project
        .branches
        .iter()
        .fold(HashMap::<&str, Vec<&str>>::new(), |mut acc, b| {
            for v in &b.versions {
                acc.entry(&v.file_path).or_default().push(&b.name)
            }
            acc
        })
        .into_iter()
        // leave only entries where the current branch is only owner of stored file
        .filter(|(_, branches)| branches.len() == 1 && branches.contains(&branch_name))
        .map(|(file_path, _)| file_path);

    for file_path in files_paths_iter {
        // clean up branch not shared versions files
        state.storage.remove_file(file_path).await?;
    }

    if persist {
        project.persist_branches(&state.storage).await?;
    }

    Ok(())
}

pub async fn get_branch_base<S>(
    project: &Project,
    base_name: Option<S>,
    base_version_id: Option<u32>,
) -> anyhow::Result<BranchBase>
where
    S: Into<String>,
{
    let name = if let Some(base_name) = base_name {
        base_name.into()
    } else {
        // use project default branch if not base branch specified
        project.default_branch.clone()
    };

    let version_id = if let Some(base_version_id) = base_version_id {
        base_version_id
    } else {
        // use base branch latest version if not version specified
        let last = project
            .branches
            .iter()
            .find(|b| b.name == name)
            .and_then(|b| b.versions.last());

        if let Some(last) = last {
            last.id
        } else {
            bail!("Base branch must have at least one version")
        }
    };

    Ok(BranchBase { name, version_id })
}

pub fn sanitise_branch_name<S: Into<String>>(input: S) -> String {
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
    use crate::branches::sanitise_branch_name;

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
            assert_eq!(sanitise_branch_name(input), result)
        }
    }
}
