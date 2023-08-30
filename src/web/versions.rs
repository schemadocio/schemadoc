use std::ops::DerefMut;

use actix_web::http::StatusCode;
use actix_web::web::Bytes;
use actix_web::{error, get, post, web, HttpRequest, Responder};
use schemadoc_diff::schema_diff::HttpSchemaDiff;

use crate::models::ProjectSlug;
use crate::settings::Settings;
use crate::storage::Storer;
use crate::versions::{crud, services, statistics};
use crate::web::schema::VersionOut;
use crate::web::utils::ApiResponse;
use crate::web::AppStateType;

#[get("")]
async fn list_versions_endpoint(
    path: web::Path<(ProjectSlug, String)>,
    state: web::Data<AppStateType>,
) -> error::Result<ApiResponse> {
    let state = state.read().await;

    let (project_slug, branch_name) = path.as_ref();

    let Some(versions) = crud::get_versions(&state, project_slug, branch_name) else {
        // Versions could not be initialized, so just return empty vec
        return Ok((Vec::<VersionOut>::new(),).into());
    };

    let result: Vec<_> = versions.iter().map(VersionOut::from).rev().collect();

    Ok((result,).into())
}

#[post("")]
async fn add_version_endpoint(
    path: web::Path<(ProjectSlug, String)>,
    bytes: Bytes,
    req: HttpRequest,
    state: web::Data<AppStateType>,
    settings: web::Data<Settings>,
) -> error::Result<ApiResponse> {
    let (project_slug, branch_name) = path.as_ref();

    let mut state = state.write().await;

    let message = req
        .headers()
        .get("X-Message")
        .map(|m| m.to_str().map(|s| s.to_owned()))
        .transpose()
        .map_err(error::ErrorBadRequest)?;

    let branch_base_name = req
        .headers()
        .get("X-Branch-Base-Name")
        .map(|m| m.to_str().map(|s| s.to_owned()))
        .transpose()
        .map_err(error::ErrorBadRequest)?;

    let branch_base_version_id = req
        .headers()
        .get("X-Branch-Base-Version-Id")
        .map(|m| m.to_str().map(|s| s.to_owned().parse::<u32>()))
        .transpose()
        .map_err(error::ErrorBadRequest)?
        .transpose()
        .map_err(error::ErrorBadRequest)?;

    let version = {
        let state = state.deref_mut();

        crate::branches::create_branch_if_not_exists(
            state,
            project_slug,
            branch_name,
            branch_base_name,
            branch_base_version_id,
        )
        .await
        .map_err(error::ErrorBadRequest)?;

        let content = String::from_utf8_lossy(&bytes);

        services::create_version(
            &settings,
            state,
            project_slug,
            branch_name,
            message,
            &content,
        )
        .await
        .map_err(|e| error::ErrorInternalServerError(format!("Error creating version: {}", e)))?
    };

    let result = version.as_ref().map(VersionOut::from);

    Ok((result, StatusCode::CREATED).into())
}

#[get("/{id}")]
async fn get_version_by_id_endpoint(
    path: web::Path<(ProjectSlug, String, u32)>,
    state: web::Data<AppStateType>,
) -> error::Result<ApiResponse> {
    let (project_slug, branch_name, id) = &path.into_inner();

    let state = state.read().await;

    let version = crud::get_version(&state, project_slug, branch_name, *id)
        .ok_or(error::ErrorNotFound("Version not found"))?;

    let result = VersionOut::from(version);

    Ok((result,).into())
}

#[get("/{id}/compare/{tgt_branch_name}/{tgt_id}")]
async fn compare_two_versions_endpoint(
    path: web::Path<(ProjectSlug, String, u32, String, u32)>,
    state: web::Data<AppStateType>,
) -> error::Result<ApiResponse> {
    let (project_slug, src_branch_name, src_version_id, tgt_branch_name, tgt_version_id) =
        path.as_ref();

    let state = state.read().await;

    let compare_result = services::compare_versions(
        &state,
        project_slug,
        src_branch_name,
        *src_version_id,
        tgt_branch_name,
        *tgt_version_id,
    )
    .await
    .map_err(|e| {
        error::ErrorInternalServerError(format!("Error while comparing two versions: {}", e))
    })?;

    let Some(diff) = compare_result.get() else {
        return Ok((None::<Response>, StatusCode::NO_CONTENT).into());
    };

    let statistics = statistics::get_diff_statistics(diff);

    #[derive(serde::Serialize)]
    struct Response<'s> {
        diff: &'s HttpSchemaDiff,
        statistics: statistics::DiffStatistics,
    }

    Ok((Response { diff, statistics },).into())
}

#[get("/{id}/diff")]
async fn get_version_diff_content_endpoint(
    path: web::Path<(ProjectSlug, String, u32)>,
    state: web::Data<AppStateType>,
) -> error::Result<impl Responder> {
    let (project_slug, branch_name, id) = &path.into_inner();

    let state = state.read().await;

    let version = crud::get_version(&state, project_slug, branch_name, *id)
        .ok_or(error::ErrorNotFound("Version not found"))?;

    let content = state.storage.read_file(&version.diff_file_path).await?;
    Ok(content)
}

pub fn get_versions_api_scope() -> actix_web::Scope {
    web::scope("projects/{project_slug}/branches/{branch_name}/versions")
        .service(add_version_endpoint)
        .service(list_versions_endpoint)
        .service(get_version_by_id_endpoint)
        .service(compare_two_versions_endpoint)
        .service(get_version_diff_content_endpoint)
}
