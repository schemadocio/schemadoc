use crate::app_state::AppState;
use crate::web::AppStateType;
use actix_web::{error, get, web};
use std::ops::DerefMut;

#[get("/invalidate-app-state")]
async fn invalidate_app_state_endpoint(
    state: web::Data<AppStateType>,
) -> Result<&'static str, error::Error> {
    let mut lock = state.write().await;

    let state = lock.deref_mut();

    *state = AppState::read(state.storage.clone())
        .await
        .map_err(|_| error::ErrorInternalServerError("Error reading store"))?;

    Ok("Invalidated")
}

pub fn get_common_api_scope() -> actix_web::Scope {
    web::scope("common").service(invalidate_app_state_endpoint)
}
