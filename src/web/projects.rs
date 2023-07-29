use actix_web::http::StatusCode;
use actix_web::{error, get, post, web, Responder};
use serde::Deserialize;
use std::ops::DerefMut;

use crate::settings::Settings;
use crate::models::{Dependency, ProjectSlug};
use crate::projects;
use crate::web::schema::{DependencyOut, ProjectOut};
use crate::web::utils::json_response;
use crate::web::AppStateType;

#[get("")]
async fn list_projects_endpoint(state: web::Data<AppStateType>) -> impl Responder {
    let state = state.read().await;

    let projects = state.projects.values().collect::<Vec<_>>();

    let out: Vec<_> = projects.into_iter().map(ProjectOut::from).collect();

    json_response(StatusCode::OK, &out)
}

#[get("/{slug}")]
async fn get_project_by_id_endpoint(
    path: web::Path<ProjectSlug>,
    state: web::Data<AppStateType>,
) -> impl Responder {
    let state = state.read().await;
    let project = state.projects.get(&path.into_inner());

    json_response(StatusCode::OK, &project.map(ProjectOut::from))
}

#[get("/{slug}/dependents")]
async fn get_dependents_project_by_id_endpoint(
    path: web::Path<ProjectSlug>,
    state: web::Data<AppStateType>,
) -> Option<impl Responder> {
    let state = state.read().await;

    let project = state.projects.get(&path.into_inner())?;

    let dependents = state
        .projects
        .values()
        .filter_map(|p| {
            p.dependencies.as_ref().map(|deps| {
                deps.iter()
                    .filter(|d| d.project == project.slug)
                    .map(|d| Dependency {
                        project: p.slug.clone(),
                        version: d.version,
                        breaking: d.breaking,
                        outdated: d.outdated,
                    })
            })
        })
        .flatten()
        .collect::<Vec<_>>();

    let body: Vec<_> = dependents.iter().map(DependencyOut::from).collect();

    Some(json_response(StatusCode::OK, &body))
}

#[derive(Deserialize)]
struct PullQueryParams {
    force: bool,
}

#[post("/{slug}/pull")]
async fn pull_project_datasource_endpoint(
    path: web::Path<ProjectSlug>,
    settings: web::Data<Settings>,
    state: web::Data<AppStateType>,
    query: web::Query<PullQueryParams>,
) -> Result<impl Responder, error::Error> {
    let mut lock = state.write().await;

    let state = lock.deref_mut();

    projects::pull_project_datasource(settings.as_ref(), state, path.as_ref(), query.force)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok("Pulled")
}

pub fn get_projects_api_scope() -> actix_web::Scope {
    web::scope("projects")
        .service(list_projects_endpoint)
        .service(get_project_by_id_endpoint)
        .service(pull_project_datasource_endpoint)
        .service(get_dependents_project_by_id_endpoint)
}
