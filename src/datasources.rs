use crate::app_state::AppState;
use crate::models::{DataSourceSource, ProjectSlug};
use crate::settings::Settings;
use crate::{branches, versions};
use anyhow::anyhow;
use chrono::{Duration, Utc};

pub async fn pull_project_datasource(
    settings: &Settings,
    state: &mut AppState,
    project_slug: &ProjectSlug,
    branch_name: &str,
    force: bool,
) -> anyhow::Result<()> {
    let (content, branch_name) = {
        let Some(project) = state.projects.get_mut(project_slug) else {
            return Err(anyhow!("Project not found"));
        };

        let Some(datasource) = project
            .data_sources
            .iter_mut()
            .find(|datasource| datasource.branch == branch_name)
        else {
            return Ok(());
        };

        let status = datasource
            .status
            .as_mut()
            .expect("Datasource status must be loaded from persistent storage");

        if !status.pull_enabled && !force {
            return Ok(());
        }

        let now = Utc::now();

        if !force {
            if let Some(pull_last_at) = status.pull_last_at {
                if pull_last_at + Duration::minutes(status.pull_interval_minutes as i64) > now {
                    println!(
                        "Skip pulling: {}::{}::{}",
                        &project.slug, &datasource.branch, &datasource.name
                    );
                    return Ok(());
                }
            }
        }

        status.pull_last_at = Some(now);

        let content = match &datasource.source {
            DataSourceSource::Url { url } => {
                let resp = reqwest::get(url).await?;
                if resp.status().is_success() {
                    status.pull_attempt = 0;
                    status.pull_error = false;
                    status.pull_error_message = None;

                    Some(resp.text().await?)
                } else {
                    status.pull_attempt += 1;
                    status.pull_error = true;
                    status.pull_error_message = Some(resp.status().as_str().to_string());

                    None
                }
            }
        };

        if content.is_none()
            && settings.pull_disable_after_attempt != 0
            && settings.pull_disable_after_attempt <= status.pull_attempt
        {
            status.pull_enabled = false;
        }

        let branch_name = datasource.branch.to_owned();

        project
            .persist_datasource(&state.storage, &branch_name)
            .await?;

        (content, branch_name)
    };

    if let Some(content) = content {
        let message = Some("Pull from datasource".to_owned());
        versions::services::create_version(
            settings,
            state,
            project_slug,
            &branch_name,
            message,
            &content,
        )
        .await?;
    }

    Ok(())
}

pub async fn pull_project_datasources(
    settings: &Settings,
    state: &mut AppState,
    project_slug: &ProjectSlug,
    force: bool,
) -> anyhow::Result<()> {
    let Some(project) = state.projects.get(project_slug) else {
        return Err(anyhow!("Project not found"));
    };

    let branches_names = project
        .data_sources
        .iter()
        .map(|ds| ds.branch.clone())
        .collect::<Vec<String>>();

    // run sequent for now
    for branch_name in &branches_names {
        let _ = branches::create_branch_if_not_exists(
            state,
            project_slug,
            branch_name,
            None::<String>,
            None,
        )
        .await?;

        pull_project_datasource(settings, state, project_slug, branch_name, force).await?;
    }

    Ok(())
}
