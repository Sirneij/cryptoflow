use crate::models::{CreateQuestion, NewQuestion};
use crate::startup::AppState;
use crate::utils::{CustomAppError, CustomAppJson};
use axum::{extract::State, response::IntoResponse};
use axum_extra::extract::PrivateCookieJar;

#[axum::debug_handler]
#[tracing::instrument(name = "ask_question", skip(state, cookies, new_question))]
pub async fn ask_question(
    State(state): State<AppState>,
    cookies: PrivateCookieJar,
    CustomAppJson(new_question): CustomAppJson<NewQuestion>,
) -> Result<impl IntoResponse, CustomAppError> {
    if new_question.title.is_empty() {
        return Err(CustomAppError::from((
            "Title cannot be empty".to_string(),
            crate::utils::ErrorContext::BadRequest,
        )));
    }

    if new_question.content.is_empty() {
        return Err(CustomAppError::from((
            "Content cannot be empty".to_string(),
            crate::utils::ErrorContext::BadRequest,
        )));
    }

    if new_question.tags.is_empty() {
        return Err(CustomAppError::from((
            "Tags cannot be empty".to_string(),
            crate::utils::ErrorContext::BadRequest,
        )));
    }

    // Process tags
    let mut tag_ids: Vec<String> = new_question
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

    // Validate tags
    state.db_store.validate_tags(&tag_ids).await?;

    // Get author id from session
    let (user_uuid, _) =
        crate::utils::get_user_id_from_session(&cookies, &state.redis_store, false).await?;

    // Create question
    let create_question = CreateQuestion {
        slug: crate::utils::slugify(&new_question.title).await,
        title: new_question.title,
        content: crate::utils::convert_markdown_to_html(&new_question.content).await,
        raw_content: new_question.content,
        author: user_uuid,
        tags: tag_ids,
    };

    let question = state
        .db_store
        .create_question_in_db(create_question)
        .await?;

    Ok(axum::Json(question).into_response())
}
