use std::ops::{Deref, DerefMut};
use actix_web::{web, error, delete, post, HttpResponse};
use actix_web::http::StatusCode;
use serde::Deserialize;

use crate::branches;
use crate::web::utils::json_response;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CreateBranchBody {
    name: String,
    base_name: Option<String>,
    base_version_id: Option<u32>,
}

#[post("")]
async fn create_branch_endpoint(
    body: web::Json<CreateBranchBody>,
    path: web::Path<crate::models::ProjectSlug>,
    state: web::Data<crate::web::AppStateType>,
) -> Result<impl actix_web::Responder, error::Error> {
    let body = body.into_inner();
    let project_slug = path.as_ref();

    let mut state = state.write().await;

    let base = branches::get_branch_base(
        state.deref(),
        project_slug,
        body.base_name,
        body.base_version_id,
    ).await
        .map_err(|e| error::ErrorBadRequest(e))?;

    let branch = branches::create_branch(state.deref_mut(), project_slug, &body.name, base)
        .await
        .map_err(|e| error::ErrorBadRequest(e))?;

    Ok(json_response(StatusCode::CREATED, &branch))
}

#[derive(Deserialize, Debug)]
struct DeleteBranchQuery {
    force: Option<bool>,
}

#[delete("/{branch_name}")]
async fn delete_branch_endpoint(
    path: web::Path<(crate::models::ProjectSlug, String)>,
    query: web::Query<DeleteBranchQuery>,
    state: web::Data<crate::web::AppStateType>,
) -> Result<impl actix_web::Responder, error::Error> {
    let (project_slug, branch_name) = &path.into_inner();

    let mut state = state.write().await;

    let force = query.force.unwrap_or(false);

    branches::delete_branch(
        state.deref_mut(), project_slug, branch_name, force, true,
    ).await.map_err(|e| error::ErrorBadRequest(e))?;

    Ok(HttpResponse::NoContent())
}

pub fn get_branches_api_scope() -> actix_web::Scope {
    web::scope("projects/{project_slug}/branches")
        .service(create_branch_endpoint)
        .service(delete_branch_endpoint)
}