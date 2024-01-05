use crate::startup::AppState;
use crate::utils::CustomAppError;
use crate::utils::CustomAppJson;
use axum::{extract::State, response::IntoResponse};
use axum_extra::extract::cookie::PrivateCookieJar;

#[axum::debug_handler]
#[tracing::instrument(name = "get_current_user", skip(cookies, state))]
pub async fn get_current_user(
    cookies: PrivateCookieJar,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, CustomAppError> {
    // Get user_id and session_id from cookie and delete it
    let (user_id, _) =
        crate::utils::get_user_id_from_session(&cookies, &state.redis_store, false).await?;

    // Get user from database
    let user = state.db_store.get_user_by_id(user_id).await?;
    let user_visible: crate::models::UserVisible = user.into();

    Ok(CustomAppJson(user_visible).into_response())
}
