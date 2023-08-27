use crate::app_state::AppState;
use crate::models::{Branch, BranchBase, Project, ProjectSlug};
use crate::storage::Storer;
use crate::versions;
use anyhow::{anyhow, bail};
use async_recursion::async_recursion;
use std::collections::HashMap;

pub fn get_branch<'p>(project: &'p Project, branch_name: &str) -> Option<&'p Branch> {
    project.branches.iter().find(|b| b.name == branch_name)
}

pub fn get_branch_mut<'p>(project: &'p mut Project, branch_name: &str) -> Option<&'p mut Branch> {
    project.branches.iter_mut().find(|b| b.name == branch_name)
}

pub async fn create_branch<'p>(
    state: &'p mut AppState,
    project_slug: &ProjectSlug,
    branch_name: &str,
    base: BranchBase,
) -> anyhow::Result<Option<&'p Branch>> {
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

    let version = project
        .branches
        .iter()
        .find(|b| b.name == base.name)
        .and_then(|b| b.versions.iter().find(|v| v.id == base.version_id));

    let Some(_) = version else {
        bail!(
            "Source branch `{}` must contain specified version {} to fork it",
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
    state: &AppState,
    project_slug: &ProjectSlug,
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
        state
            .projects
            .get(project_slug)
            .map(|p| p.default_branch.clone())
            .ok_or(anyhow!("Project {} not found", project_slug))?
    };

    let version_id = if let Some(base_version_id) = base_version_id {
        base_version_id
    } else {
        // use base branch latest version if not version specified
        let versions =
            versions::crud::get_versions(state, project_slug, &name).ok_or(anyhow::anyhow!(
                "Versions not found for branch `{}` of {}",
                name,
                project_slug
            ))?;

        if let Some(last) = versions.last() {
            last.id
        } else {
            bail!("Base branch must have at least one version")
        }
    };

    Ok(BranchBase { name, version_id })
}
