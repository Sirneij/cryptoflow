use crate::{
    startup::AppState,
    utils::{CustomAppError, CustomAppJson, ErrorContext},
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
        ..Default::default()
    };

    Ok(response.into_response())
}

#[axum::debug_handler]
#[tracing::instrument(name = "update_a_question", skip(state))]
pub async fn update_a_question(
    Path(question_id): Path<uuid::Uuid>,
    State(state): State<AppState>,
    cookies: PrivateCookieJar,
    CustomAppJson(update_question): CustomAppJson<crate::models::UpdateQuestion>,
) -> Result<impl IntoResponse, CustomAppError> {
    // Get author id from session
    let (user_uuid, _) =
        crate::utils::get_user_id_from_session(&cookies, &state.redis_store, false).await?;

    // Extract tags from update_question
    let mut tag_ids: Vec<String> = update_question
        .tags
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();

    // Check if tags are more than 4
    if tag_ids.len() > 4 {
        return Err(CustomAppError::from((
            "Tags cannot be more than 4".to_string(),
            crate::utils::ErrorContext::BadRequest,
        )));
    }

    // Sort and deduplicate tags
    tag_ids.sort();
    tag_ids.dedup();

    // Create a question out of update_question
    let new_update_question = crate::models::CreateQuestion {
        slug: crate::utils::slugify(&update_question.title).await,
        title: update_question.title,
        content: crate::utils::convert_markdown_to_html(&update_question.content).await,
        raw_content: update_question.content,
        author: user_uuid,
        tags: tag_ids,
    };

    state
        .db_store
        .update_question_in_db(question_id, new_update_question)
        .await
        .map_err(|_| {
            CustomAppError::from((
                "Failed to update question and it's most probably due to not being authorized"
                    .to_string(),
                ErrorContext::UnauthorizedAccess,
            ))
        })?;

    let response = crate::utils::SuccessResponse {
        message: "Question updated successfully".to_string(),
        status_code: StatusCode::NO_CONTENT.as_u16(),
        ..Default::default()
    };

    Ok(response.into_response())
}
