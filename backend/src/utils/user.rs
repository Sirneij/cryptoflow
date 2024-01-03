use crate::utils::{CustomAppError, ErrorContext};
use axum_extra::extract::PrivateCookieJar;
use bb8_redis::bb8;
use uuid::Uuid;

#[tracing::instrument(
    name = "get_user_id_from_session",
    skip(cookies, redis_store, is_logout)
)]
pub async fn get_user_id_from_session(
    cookies: &PrivateCookieJar,
    redis_store: &bb8::Pool<bb8_redis::RedisConnectionManager>,
    is_logout: bool,
) -> Result<(Uuid, String), CustomAppError> {
    let session_id = cookies
        .get("sessionid")
        .map(|cookie| cookie.value().to_owned())
        .ok_or_else(|| {
            CustomAppError::from((
                "Session ID not found".to_string(),
                ErrorContext::UnauthorizedAccess,
            ))
        })?;

    let mut redis_con = redis_store.get().await.map_err(|_| {
        CustomAppError::from((
            "Failed to get redis connection".to_string(),
            ErrorContext::InternalServerError,
        ))
    })?;

    let user_id: String = bb8_redis::redis::cmd("GET")
        .arg(&session_id)
        .query_async(&mut *redis_con)
        .await
        .map_err(|_| {
            CustomAppError::from((
                "You are not authorized since you don't seem to have been authenticated"
                    .to_string(),
                ErrorContext::UnauthorizedAccess,
            ))
        })?;

    let user_uuid = Uuid::parse_str(&user_id).map_err(|_| {
        CustomAppError::from((
            "Invalid user ID format".to_string(),
            ErrorContext::InternalServerError,
        ))
    })?;

    if is_logout {
        bb8_redis::redis::cmd("DEL")
            .arg(&session_id)
            .query_async::<_, i64>(&mut *redis_con)
            .await
            .map_err(|_| {
                CustomAppError::from((
                    "Failed to delete session ID from redis".to_string(),
                    ErrorContext::InternalServerError,
                ))
            })?;
    }

    Ok((user_uuid, session_id))
}
