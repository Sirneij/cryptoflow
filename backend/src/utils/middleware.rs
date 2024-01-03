use crate::startup::AppState;
use crate::utils::get_user_id_from_session;
use axum::{extract::Request, middleware::Next};
use axum::{
    extract::State,
    response::{IntoResponse, Response},
};
use axum_extra::extract::PrivateCookieJar;

#[tracing::instrument(
    name = "validate_authentication_session",
    skip(cookies, state, req, next)
)]
pub async fn validate_authentication_session(
    cookies: PrivateCookieJar,
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, Response> {
    // Use the utility function to get the user ID from the session
    match get_user_id_from_session(&cookies, &state.redis_store, false).await {
        Ok(_user_id) => Ok(next.run(req).await),
        Err(error) => Err(error.into_response()),
    }
}
