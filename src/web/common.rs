use crate::app_state::AppState;
use crate::web::auth::BasicAuth;
use crate::web::AppStateType;
use actix_web::{error, post, web};
use std::ops::DerefMut;

#[post("/invalidate-app-state")]
async fn invalidate_app_state_endpoint(
    _: BasicAuth,
    state: web::Data<AppStateType>,
) -> Result<&'static str, error::Error> {
    let mut lock = state.write().await;

    let state = lock.deref_mut();

    *state = AppState::read(state.storage.clone(), state.config_storage.clone())
        .await
        .map_err(|_| error::ErrorInternalServerError("Error reading store"))?;

    Ok("Invalidated")
}

pub fn get_common_api_scope() -> actix_web::Scope {
    web::scope("common").service(invalidate_app_state_endpoint)
}
