use crate::models::NewUser;
use crate::startup::AppState;
use crate::utils::SuccessResponse;
use crate::utils::{CustomAppError, CustomAppJson, ErrorContext};
use argon2::password_hash::rand_core::{OsRng, RngCore};
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use sha2::{Digest, Sha256};

#[axum::debug_handler]
#[tracing::instrument(name = "register_user", skip(state, new_user),fields(user_email = new_user.email, user_first_name = new_user.first_name, user_last_name = new_user.last_name))]
pub async fn register_user(
    State(state): State<AppState>,
    CustomAppJson(new_user): CustomAppJson<NewUser>,
) -> Result<impl IntoResponse, CustomAppError> {
    let hashed_password = crate::utils::hash_password(&new_user.password.as_bytes()).await;

    let user = state
        .db_store
        .create_user(
            &new_user.first_name,
            &new_user.last_name,
            &new_user.email,
            &hashed_password,
        )
        .await?;

    // Generate a truly random activation code for the user using argon2::password_hash::rand_core::OsRng
    let activation_code = (OsRng.next_u32() % 900000 + 100000).to_string();
    // Hash the activation code
    let mut hasher = Sha256::new();
    hasher.update(activation_code.as_bytes());
    let hashed_activation_code = format!("{:x}", hasher.finalize());

    // Save activation code in redis
    let mut redis_con = state.redis_store.get().await.map_err(|_| {
        CustomAppError::from((
            "Failed to get redis connection".to_string(),
            ErrorContext::InternalServerError,
        ))
    })?;

    let settings = crate::settings::get_settings().map_err(|_| {
        CustomAppError::from((
            "Failed to read settings".to_string(),
            ErrorContext::InternalServerError,
        ))
    })?;
    let activation_code_expiration_in_seconds = settings.secret.token_expiration * 60;

    bb8_redis::redis::cmd("SET")
        .arg(user.id.to_string())
        .arg(hashed_activation_code)
        .arg("EX")
        .arg(activation_code_expiration_in_seconds)
        .query_async::<_, String>(&mut *redis_con)
        .await
        .map_err(|_| {
            CustomAppError::from((
                "Failed to save activation code".to_string(),
                ErrorContext::InternalServerError,
            ))
        })?;

    // Send activation code to user's email
    crate::utils::send_multipart_email(
        "Welcome to StackOverflow clone with Rust (axum) and SvelteKit".to_string(),
        user,
        state.clone(),
        "user_welcome.html",
        activation_code,
    )
    .await
    .map_err(|_| {
        CustomAppError::from((
            "Failed to send activation email".to_string(),
            ErrorContext::InternalServerError,
        ))
    })?;

    Ok(SuccessResponse {
        message: "Registration complete! Check your email for a verification code to activate your account.".to_string(),
        status_code: StatusCode::CREATED.as_u16(),
    }.into_response())
}
