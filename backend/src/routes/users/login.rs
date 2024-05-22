use crate::models::LoginUser;
use crate::startup::AppState;
use crate::utils::verify_password;
use crate::utils::SuccessResponse;
use crate::utils::{CustomAppError, CustomAppJson, ErrorContext};
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::cookie::{Cookie, PrivateCookieJar, SameSite};
use time::Duration;

#[axum::debug_handler]
#[tracing::instrument(name = "login_user", skip(cookies, state, login), fields(email = %login.email))]
pub async fn login_user(
    cookies: PrivateCookieJar,
    State(state): State<AppState>,
    CustomAppJson(login): CustomAppJson<LoginUser>,
) -> Result<(PrivateCookieJar, impl IntoResponse), CustomAppError> {
    // Get user from db by email
    let user = state
        .db_store
        .get_user_by_email(&login.email)
        .await
        .map_err(|_| {
            CustomAppError::from((
                "Invalid email or password".to_string(),
                ErrorContext::BadRequest,
            ))
        })?;

    // Verify password
    tokio::task::spawn_blocking(move || {
        verify_password(&user.password, &login.password.as_bytes())
    })
    .await
    .map_err(|_| {
        CustomAppError::from((
            "Server error occurred".to_string(),
            ErrorContext::InternalServerError,
        ))
    })?
    .map_err(|_| {
        CustomAppError::from((
            "Invalid email or password".to_string(),
            ErrorContext::BadRequest,
        ))
    })?;

    // Generate a truly random session id for the user
    let session_id = uuid::Uuid::new_v4().to_string();

    // Save session id in redis
    let mut redis_con = state.redis_store.get().await.map_err(|_| {
        CustomAppError::from((
            "Failed to connect to session store".to_string(),
            ErrorContext::InternalServerError,
        ))
    })?;

    let settings = crate::settings::get_settings().map_err(|_| {
        CustomAppError::from((
            "Failed to read settings".to_string(),
            ErrorContext::InternalServerError,
        ))
    })?;
    let cookie_expiration = settings.secret.cookie_expiration;

    bb8_redis::redis::cmd("SET")
        .arg(session_id.clone())
        .arg(user.id.to_string())
        .arg("EX")
        .arg(cookie_expiration * 60)
        .query_async::<_, String>(&mut *redis_con)
        .await
        .map_err(|_| {
            CustomAppError::from((
                "Failed to save session".to_string(),
                ErrorContext::InternalServerError,
            ))
        })?;

    // Create cookie
    let cookie = Cookie::build(("sessionid", session_id))
        .secure(true)
        .same_site(SameSite::Strict)
        .http_only(true)
        .path("/")
        .max_age(Duration::minutes(cookie_expiration));

    Ok((
        cookies.add(cookie),
        SuccessResponse {
            message: "The authentication process was successful.".to_string(),
            status_code: StatusCode::OK.as_u16(),
            ..Default::default()
        }
        .into_response(),
    ))
}
