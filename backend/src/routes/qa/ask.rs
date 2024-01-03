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
    // Process tags
    let tag_names: Vec<String> = new_question
        .tags
        .split(",")
        .map(|s| s.to_string().to_lowercase())
        .collect();
    let tag_ids = state.db_store.get_tag_ids_from_db(tag_names).await?;

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
