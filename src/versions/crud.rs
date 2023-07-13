use crate::app_state::AppState;
use crate::models::{ProjectSlug, Version};

pub async fn get_versions<'s>(
    app_state: &'s AppState,
    project_slug: &ProjectSlug,
) -> Option<&'s Vec<Version>> {
    let Some(project) = app_state.projects.get(project_slug) else {
        return None;
    };

    project.versions.as_ref()
}

pub async fn get_version<'s>(
    app_state: &'s AppState,
    project_slug: &ProjectSlug,
    id: u32,
) -> Option<&'s Version> {
    let Some(project) = app_state.projects.get(project_slug) else {
        return None;
    };

    let Some(versions) = &project.versions else {
        return None;
    };

    versions.iter().find(|v| v.id == id)
}

pub async fn delete_version(
    app_state: &mut AppState,
    project_slug: &ProjectSlug,
    id: u32,
) -> anyhow::Result<bool> {
    let Some(project) = app_state.projects.get_mut(project_slug) else {
        return Ok(false);
    };

    let Some(versions) = &mut project.versions else {
        return Ok(false);
    };

    let len = versions.len();
    versions.retain(|v| v.id != id);
    let len_after = versions.len();

    project.persist_versions(&app_state.storage).await?;

    Ok(len != len_after)
}
