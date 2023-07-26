use anyhow::anyhow;
use schemadoc_diff::checker::validate;

use crate::versions;
use crate::app_state::AppState;
use crate::models::{Dependency, ProjectSlug};

pub async fn update_dependency_status(
    app_state: &mut AppState,
    src_project_slug: &ProjectSlug,
    tgt_project_slug: &ProjectSlug,
) -> anyhow::Result<()> {
    let tgt_version_id = {
        let Some(tgt_project) = app_state.projects.get(tgt_project_slug) else {
            return Err(anyhow!("Target project not found"));
        };

        let Some(tgt_versions) = &tgt_project.versions else {
            return Ok(());
        };

        let tgt_version = tgt_versions.last()
            .ok_or(anyhow!("Dependency latest version not found"))?;

        tgt_version.id
    };

    let dependency = get_dependency(app_state, src_project_slug, tgt_project_slug)
        .await.ok_or(anyhow!("Dependency not found"))?;

    let breaking = {
        let diff = versions::services::compare_versions(
            app_state, tgt_project_slug, dependency.version, tgt_version_id,
        ).await?;

        diff
            .get()
            .map(|diff| !validate(&diff, &["*"]).is_empty())
    };

    let outdated = Some(dependency.version != tgt_version_id);

    let Some(src_project) = app_state.projects.get_mut(src_project_slug) else {
        return Err(anyhow!("Source project not found"));
    };

    let Some(dependencies) = &mut src_project.dependencies else {
        return Ok(());
    };

    let dependency = dependencies
        .iter_mut()
        .find(|d| &d.project == tgt_project_slug)
        .ok_or(anyhow!("Dependency not found"))?;

    dependency.outdated = outdated;
    dependency.breaking = breaking;

    Ok(())
}

pub async fn get_dependency<'s>(
    app_state: &'s AppState,
    src_project_slug: &ProjectSlug,
    tgt_project_slug: &ProjectSlug,
) -> Option<&'s Dependency> {
    let Some(project) = app_state.projects.get(src_project_slug) else {
        return None;
    };

    let Some(dependencies) = &project.dependencies else {
        return None;
    };

    dependencies.iter().find(|v| &v.project == tgt_project_slug)
}


pub async fn update_dependent_projects(
    app_state: &mut AppState, tgt_project_slug: &ProjectSlug,
) -> anyhow::Result<Vec<ProjectSlug>> {
    let slugs = app_state.projects.values().filter_map(|p| {
        let has_dep = p.dependencies.as_ref()
            .map(|d| d.iter().any(|d| &d.project == tgt_project_slug))
            .unwrap_or(false);

        if has_dep {
            Some(p.slug.clone())
        } else {
            None
        }
    }).collect::<Vec<_>>();

    for src_project_slug in slugs.iter() {
        update_dependency_status(app_state, src_project_slug, tgt_project_slug).await?;
    }

    Ok(slugs)
}