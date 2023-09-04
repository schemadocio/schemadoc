use actix_web::{error, get, post, web};
use serde::Deserialize;
use std::ops::DerefMut;

use crate::datasources;
use crate::models::{Dependency, ProjectSlug};
use crate::settings::Settings;
use crate::web::auth::BasicAuth;
use crate::web::response::ApiResponse;
use crate::web::schema::{DependencyOut, ProjectOut};
use crate::web::AppStateType;

#[get("")]
async fn list_projects_endpoint(state: web::Data<AppStateType>) -> ApiResponse {
    let state = state.read().await;

    let projects = state.projects.values().collect::<Vec<_>>();

    let out: Vec<_> = projects.into_iter().map(ProjectOut::from).collect();

    (&out,).into()
}

#[get("/{slug}")]
async fn get_project_by_id_endpoint(
    path: web::Path<ProjectSlug>,
    state: web::Data<AppStateType>,
) -> ApiResponse {
    let state = state.read().await;
    let project = state.projects.get(&path.into_inner());

    (&project.map(ProjectOut::from),).into()
}

#[get("/{slug}/dependents")]
async fn get_dependents_project_by_id_endpoint(
    path: web::Path<ProjectSlug>,
    state: web::Data<AppStateType>,
) -> error::Result<ApiResponse> {
    let state = state.read().await;

    let project = state
        .projects
        .get(&path.into_inner())
        .ok_or(error::ErrorNotFound("Project not found"))?;

    let dependents = state
        .projects
        .values()
        .flat_map(|p| {
            p.dependencies
                .iter()
                .filter(|d| d.project == project.slug)
                .map(|d| Dependency {
                    project: p.slug.clone(),
                    branch: d.branch.clone(),
                    version: d.version,
                    breaking: d.breaking,
                    outdated: d.outdated,
                })
        })
        .collect::<Vec<_>>();

    let body: Vec<_> = dependents.iter().map(DependencyOut::from).collect();

    Ok((&body,).into())
}

#[derive(Deserialize)]
struct PullQueryParams {
    force: Option<bool>,
}

#[post("/{slug}/pull")]
async fn pull_project_datasource_endpoint(
    _: BasicAuth,
    path: web::Path<ProjectSlug>,
    settings: web::Data<Settings>,
    state: web::Data<AppStateType>,
    query: web::Query<PullQueryParams>,
) -> error::Result<ApiResponse> {
    let mut lock = state.write().await;

    let state = lock.deref_mut();

    let force = query.force.unwrap_or(false);

    datasources::pull_project_datasources(settings.as_ref(), state, path.as_ref(), force)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(("Pulled",).into())
}

pub fn get_projects_api_scope() -> actix_web::Scope {
    web::scope("projects")
        .service(list_projects_endpoint)
        .service(get_project_by_id_endpoint)
        .service(pull_project_datasource_endpoint)
        .service(get_dependents_project_by_id_endpoint)
}
