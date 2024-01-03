use crate::{
    startup::AppState,
    utils::{CustomAppError, ErrorContext},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::PrivateCookieJar;

#[axum::debug_handler]
#[tracing::instrument(name = "all_question", skip(state))]
pub async fn all_questions(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, CustomAppError> {
    let questions = state.db_store.get_all_questions_from_db().await?;

    Ok(axum::Json(questions).into_response())
}

#[axum::debug_handler]
#[tracing::instrument(name = "get_question", skip(state))]
pub async fn get_question(
    Path(question_id): Path<uuid::Uuid>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, CustomAppError> {
    let question = state
        .db_store
        .get_question_from_db(None, question_id)
        .await?;

    Ok(axum::Json(question).into_response())
}

#[axum::debug_handler]
#[tracing::instrument(name = "delete_a_question", skip(state))]
pub async fn delete_a_question(
    Path(question_id): Path<uuid::Uuid>,
    State(state): State<AppState>,
    cookies: PrivateCookieJar,
) -> Result<impl IntoResponse, CustomAppError> {
    // Get author id from session
    let (user_uuid, _) =
        crate::utils::get_user_id_from_session(&cookies, &state.redis_store, false).await?;

    state
        .db_store
        .delete_question_from_db(user_uuid, question_id)
        .await
        .map_err(|_| {
            CustomAppError::from((
                "Failed to delete question and it's most probably due to not being authorized"
                    .to_string(),
                ErrorContext::UnauthorizedAccess,
            ))
        })?;

    let response = crate::utils::SuccessResponse {
        message: "Question deleted successfully".to_string(),
        status_code: StatusCode::NO_CONTENT.as_u16(),
    };

    Ok(response.into_response())
}
