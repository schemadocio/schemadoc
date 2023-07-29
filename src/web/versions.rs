use actix_web::http::StatusCode;
use actix_web::web::Bytes;
use actix_web::{error, get, post, web, HttpRequest, Responder};
use std::ops::DerefMut;

use crate::settings::Settings;
use crate::models::ProjectSlug;
use crate::storage::Storer;
use crate::versions::{crud, services};
use crate::web::schema::VersionOut;
use crate::web::utils::json_response;
use crate::web::AppStateType;

#[get("")]
async fn list_versions_endpoint(
    path: web::Path<ProjectSlug>,
    state: web::Data<AppStateType>,
) -> Result<impl Responder, error::Error> {
    let state = state.read().await;

    let versions = crud::get_versions(&state, &path)
        .await
        .ok_or(error::ErrorInternalServerError("Versions not found."))?;

    let results: Vec<_> = versions.iter().map(VersionOut::from).rev().collect();

    Ok(json_response(StatusCode::OK, &results))
}

#[post("")]
async fn add_version_endpoint(
    path: web::Path<ProjectSlug>,
    bytes: Bytes,
    req: HttpRequest,
    state: web::Data<AppStateType>,
    settings: web::Data<Settings>,
) -> Result<impl Responder, error::Error> {
    let mut state = state.write().await;

    let message = req
        .headers()
        .get("X-Message")
        .map(|m| m.to_str().map(|s| s.to_owned()))
        .transpose()
        .map_err(error::ErrorBadRequest)?;

    let content = String::from_utf8_lossy(&bytes);

    let version = services::create_version(&settings, state.deref_mut(), &path, message, &content)
        .await
        .map_err(|e| error::ErrorInternalServerError(format!("Error creating version: {}", e)))?;

    let result = version.as_ref().map(VersionOut::from);

    Ok(json_response(StatusCode::CREATED, &result))
}

#[get("/{id}")]
async fn get_version_by_id_endpoint(
    path: web::Path<(ProjectSlug, u32)>,
    state: web::Data<AppStateType>,
) -> Result<impl Responder, error::Error> {
    let (project_slug, id) = &path.into_inner();

    let state = state.read().await;

    let version = crud::get_version(&state, project_slug, *id)
        .await
        .ok_or(error::ErrorNotFound("Version not found"))?;

    let result = VersionOut::from(version);

    Ok(json_response(StatusCode::OK, &result))
}

#[get("/{id}/compare/{tgtId}")]
async fn compare_two_versions_endpoint(
    path: web::Path<(ProjectSlug, u32, u32)>,
    state: web::Data<AppStateType>,
) -> Result<impl Responder, error::Error> {
    let (project, src_id, tgt_id) = path.into_inner();

    let state = state.read().await;

    let compare_result = services::compare_versions(&state, &project, src_id, tgt_id)
        .await
        .map_err(|e| {
            error::ErrorInternalServerError(format!("Error while comparing two versions: {}", e))
        })?;

    let diff = compare_result.get();

    Ok(json_response(StatusCode::OK, &diff))
}

#[get("/{id}/diff")]
async fn get_version_diff_content_endpoint(
    path: web::Path<(ProjectSlug, u32)>,
    state: web::Data<AppStateType>,
) -> Result<impl Responder, error::Error> {
    let (project_slug, id) = &path.into_inner();

    let state = state.read().await;

    let version = crud::get_version(&state, project_slug, *id)
        .await
        .ok_or(error::ErrorNotFound("Version not found"))?;

    let content = state.storage.read_file(&version.diff_file_path).await?;
    Ok(content)
}

pub fn get_versions_api_scope() -> actix_web::Scope {
    web::scope("projects/{project_slug}/versions")
        .service(add_version_endpoint)
        .service(list_versions_endpoint)
        .service(get_version_by_id_endpoint)
        .service(compare_two_versions_endpoint)
        .service(get_version_diff_content_endpoint)
}
