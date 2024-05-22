use crate::{
    models::ActivateUser,
    startup::AppState,
    utils::{CustomAppError, CustomAppJson, ErrorContext, SuccessResponse},
};
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use sha2::{Digest, Sha256};

#[axum::debug_handler]
#[tracing::instrument(name = "activate_user_account", skip(state, acc_user))]
pub async fn activate_user_account(
    State(state): State<AppState>,
    CustomAppJson(acc_user): CustomAppJson<ActivateUser>,
) -> Result<impl IntoResponse, CustomAppError> {
    let mut redis_con = state.redis_store.get().await.map_err(|_| {
        CustomAppError::from((
            "Failed to get redis connection".to_string(),
            ErrorContext::InternalServerError,
        ))
    })?;

    let mut hasher = Sha256::new();
    hasher.update(acc_user.token.as_bytes());
    let hashed_token = format!("{:x}", hasher.finalize());

    let hashed_activation_code: String = bb8_redis::redis::cmd("GET")
        .arg(&acc_user.id.to_string())
        .query_async(&mut *redis_con)
        .await
        .map_err(|_| {
            CustomAppError::from((
                "This activation has been used or expired".to_string(),
                ErrorContext::BadRequest,
            ))
        })?;

    if hashed_activation_code == hashed_token {
        state.db_store.activate_user(&acc_user.id).await?;

        // Delete activation code from redis
        bb8_redis::redis::cmd("DEL")
            .arg(&acc_user.id.to_string())
            .query_async::<_, i64>(&mut *redis_con)
            .await
            .map_err(|_| {
                CustomAppError::from((
                    "Failed to delete activation code from Redis".to_string(),
                    ErrorContext::InternalServerError,
                ))
            })?;

        Ok(SuccessResponse {
            message: "The activation process was successful. You can now login.".to_string(),
            status_code: StatusCode::OK.as_u16(),
            ..Default::default()
        }
        .into_response())
    } else {
        return Err(CustomAppError::from((
            "Activation code not found or expired".to_string(),
            ErrorContext::BadRequest,
        )));
    }
}
