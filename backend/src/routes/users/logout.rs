use crate::startup::AppState;
use crate::utils::CustomAppError;
use crate::utils::SuccessResponse;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::cookie::{Cookie, PrivateCookieJar};

#[axum::debug_handler]
#[tracing::instrument(name = "logout_user", skip(cookies, state))]
pub async fn logout_user(
    cookies: PrivateCookieJar,
    State(state): State<AppState>,
) -> Result<(PrivateCookieJar, impl IntoResponse), CustomAppError> {
    // Get user_id and session_id from cookie and delete it
    let (_, _) = crate::utils::get_user_id_from_session(&cookies, &state.redis_store, true).await?;

    Ok((
        cookies.remove(Cookie::from("sessionid")),
        SuccessResponse {
            message: "The unauthentication process was successful.".to_string(),
            status_code: StatusCode::OK.as_u16(),
        }
        .into_response(),
    ))
}
