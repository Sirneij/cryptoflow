use crate::utils::ErrorContext;
use crate::{startup::AppState, utils::CustomAppError};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::PrivateCookieJar;

#[axum::debug_handler]
#[tracing::instrument(name = "question_answers", skip(state))]
pub async fn question_answers(
    Path(question_id): Path<uuid::Uuid>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, CustomAppError> {
    let answers = state
        .db_store
        .get_answers_from_db(None, question_id)
        .await?;

    Ok(axum::Json(answers).into_response())
}

#[axum::debug_handler]
#[tracing::instrument(name = "delete_an_answer", skip(state))]
pub async fn delete_an_answer(
    Path(answer_id): Path<uuid::Uuid>,
    State(state): State<AppState>,
    cookies: PrivateCookieJar,
) -> Result<impl IntoResponse, CustomAppError> {
    // Get author id from session
    let (user_uuid, _) =
        crate::utils::get_user_id_from_session(&cookies, &state.redis_store, false).await?;

    state
        .db_store
        .delete_answer_from_db(user_uuid, answer_id)
        .await
        .map_err(|_| {
            CustomAppError::from((
                "Failed to delete answer and it's most probably due to not being authorized"
                    .to_string(),
                ErrorContext::UnauthorizedAccess,
            ))
        })?;

    Ok(crate::utils::SuccessResponse {
        message: "Answer deleted successfully".to_string(),
        status_code: StatusCode::NO_CONTENT.as_u16(),
    }
    .into_response())
}
