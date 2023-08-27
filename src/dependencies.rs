use schemadoc_diff::checker::validate;

use crate::app_state::AppState;
use crate::models::{Dependency, ProjectSlug};
use crate::versions;

pub async fn get_dependency<'s>(
    state: &'s AppState,
    src_project_slug: &ProjectSlug,
    tgt_project_slug: &ProjectSlug,
) -> Option<&'s Dependency> {
    let Some(project) = state.projects.get(src_project_slug) else {
        return None;
    };

    project
        .dependencies
        .iter()
        .find(|v| &v.project == tgt_project_slug)
}

pub async fn setup_project_dependencies(state: &mut AppState) -> anyhow::Result<()> {
    let slugs = state
        .projects
        .keys()
        .map(|slug| slug.to_owned())
        .collect::<Vec<_>>();

    for tgt_project_slug in &slugs {
        update_dependent_projects(state, tgt_project_slug, None).await?;
    }

    Ok(())
}

pub async fn update_dependent_projects(
    state: &mut AppState,
    tgt_project_slug: &ProjectSlug,
    tgt_branch_name: Option<&str>,
) -> anyhow::Result<Vec<ProjectSlug>> {
    let mut state_updates = vec![];

    let mut affected_project_slugs = vec![];

    // collect dependencies state information
    for src_project in state.projects.values() {
        let dependencies = src_project
            .dependencies
            .iter()
            .enumerate()
            .filter_map(|(idx, d)| {
                if &d.project != tgt_project_slug {
                    return None;
                }
                if let Some(tgt_branch_name) = tgt_branch_name {
                    if d.branch == tgt_branch_name {
                        Some((idx, d))
                    } else {
                        None
                    }
                } else {
                    Some((idx, d))
                }
            })
            .collect::<Vec<_>>();

        if dependencies.is_empty() {
            continue;
        }

        affected_project_slugs.push(src_project.slug.clone());

        for (idx, dependency) in dependencies.into_iter() {
            let (tgt_version_id, branch_name) = {
                let versions =
                    versions::crud::get_versions(state, tgt_project_slug, &dependency.branch);

                let Some(tgt_versions) = versions else {
                    continue;
                };

                if tgt_versions.is_empty() {
                    eprintln!(
                        "Dependency latest version was not found for {} of {}",
                        tgt_project_slug, src_project.slug
                    );
                    continue;
                }

                let tgt_version = tgt_versions.last().expect("At least one item must be");

                (tgt_version.id, dependency.branch.to_owned())
            };

            let breaking = {
                let diff = versions::services::compare_versions(
                    state,
                    tgt_project_slug,
                    &branch_name,
                    dependency.version,
                    &branch_name,
                    tgt_version_id,
                )
                .await?;

                diff.get().map(|diff| !validate(diff, &["*"]).is_empty())
            };

            let outdated = dependency.version != tgt_version_id;

            state_updates.push((src_project.slug.clone(), idx, Some(outdated), breaking));
        }
    }

    // apply dependencies state updates
    state_updates
        .into_iter()
        .for_each(|(slug, index, outdated, breaking)| {
            let Some(project) = state.projects.get_mut(&slug) else {
                return;
            };

            let Some((_, dependency)) = project
                .dependencies
                .iter_mut()
                .enumerate()
                .find(|(idx, _)| idx == &index)
            else {
                return;
            };

            dependency.outdated = outdated;
            dependency.breaking = breaking;
        });

    Ok(affected_project_slugs)
}
