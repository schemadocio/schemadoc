use actix_web::http::StatusCode;
use actix_web::{error, get, put, web, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::ops::DerefMut;

use crate::models::{DataSourceSource, DataSourceStatus, ProjectSlug};
use crate::web::utils::json_response;
use crate::web::AppStateType;

#[derive(Debug, Deserialize)]
struct UpdateDataSourceStatusBody {
    pub pull_enabled: Option<bool>,
    pub pull_attempt: Option<u32>,

    pub pull_interval_minutes: Option<u32>,
    pub pull_last_at: Option<DateTime<Utc>>,
    pub pull_error: Option<bool>,
    pub pull_error_message: Option<Option<String>>,
}

#[derive(Debug, Serialize)]
pub struct DataSourceBody<'s> {
    pub name: &'s str,
    pub project: &'s ProjectSlug,
    pub source: &'s DataSourceSource,
    pub status: &'s Option<DataSourceStatus>,
}

#[get("")]
async fn get_datasources_endpoint(
    state: web::Data<AppStateType>,
) -> Result<impl Responder, error::Error> {
    let state = state.read().await;

    let ds: Vec<_> = state
        .projects
        .values()
        .filter_map(|project| {
            project.data_source.as_ref().map(|ds| DataSourceBody {
                name: &ds.name,
                project: &project.slug,
                source: &ds.source,
                status: &ds.status,
            })
        })
        .collect();

    Ok(json_response(StatusCode::OK, &ds))
}

#[put("/{project_slug}")]
async fn update_datasource_status_endpoint(
    path: web::Path<(ProjectSlug,)>,
    body: web::Json<UpdateDataSourceStatusBody>,
    state: web::Data<AppStateType>,
) -> Result<impl Responder, error::Error> {
    let (project_slug,) = &path.into_inner();

    let mut lock = state.write().await;

    let state = lock.deref_mut();

    let Some(project) = state.projects.get_mut(project_slug) else {
        return Err(error::ErrorNotFound("Project not found"));
    };

    {
        let Some(data_source) = &mut project.data_source else {
            return Err(error::ErrorNotFound("Datasource not found"));
        };

        let Some(status) = &mut data_source.status else {
            return Err(error::ErrorInternalServerError(
                "Datasource was not properly initialized",
            ));
        };

        if let Some(pull_enabled) = body.pull_enabled {
            status.pull_enabled = pull_enabled;
        }

        if let Some(pull_attempt) = body.pull_attempt {
            status.pull_attempt = pull_attempt;
        }

        if let Some(pull_interval_minutes) = body.pull_interval_minutes {
            status.pull_interval_minutes = pull_interval_minutes;
        }

        if let Some(pull_last_at) = body.pull_last_at {
            status.pull_last_at = Some(pull_last_at);
        }

        if let Some(pull_error) = body.pull_error {
            status.pull_error = pull_error;
        }

        if let Some(pull_error_message) = body.pull_error_message.clone() {
            status.pull_error_message = pull_error_message;
        }
    }

    project
        .persist_datasource(&state.storage)
        .await
        .map_err(|e| error::ErrorInternalServerError(format!("{}", e)))?;

    Ok(json_response(StatusCode::NO_CONTENT, &None::<i32>))
}

pub fn get_datasource_api_scope() -> actix_web::Scope {
    web::scope("datasources")
        .service(get_datasources_endpoint)
        .service(update_datasource_status_endpoint)
}
