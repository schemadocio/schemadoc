use crate::app_state::AppState;
use crate::models::{ProjectSlug, Version};

pub fn get_versions<'s>(
    state: &'s AppState,
    project_slug: &ProjectSlug,
    branch_name: &str,
) -> Option<&'s Vec<Version>> {
    let Some(project) = state.projects.get(project_slug) else {
        return None;
    };

    project
        .branches
        .iter()
        .find(|b| b.name == branch_name)
        .map(|x| &x.versions)
}

pub fn get_version<'s>(
    state: &'s AppState,
    project_slug: &ProjectSlug,
    branch_name: &str,
    id: u32,
) -> Option<&'s Version> {
    let Some(versions) = get_versions(state, project_slug, branch_name) else {
        return None;
    };

    versions.iter().find(|v| v.id == id)
}

pub async fn delete_version(
    state: &mut AppState,
    project_slug: &ProjectSlug,
    branch_name: &str,
    id: u32,
) -> anyhow::Result<bool> {
    let Some(project) = state.projects.get_mut(project_slug) else {
        return Ok(false);
    };

    let versions = project
        .branches
        .iter_mut()
        .find(|b| b.name == branch_name)
        .map(|x| &mut x.versions);

    let Some(versions) = versions else {
        return Ok(false);
    };

    let len = versions.len();
    versions.retain(|v| v.id != id);
    let len_after = versions.len();

    project.persist_branches(&state.storage).await?;

    Ok(len != len_after)
}
