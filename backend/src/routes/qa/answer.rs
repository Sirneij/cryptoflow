use crate::{
    models::{CreateAnswer, NewAnswer},
    startup::AppState,
    utils::{CustomAppError, CustomAppJson},
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use axum_extra::extract::PrivateCookieJar;

#[axum::debug_handler]
#[tracing::instrument(name = "answer_question", skip(state))]
pub async fn answer_question(
    Path(question_id): Path<uuid::Uuid>,
    State(state): State<AppState>,
    cookies: PrivateCookieJar,
    CustomAppJson(new_answer): CustomAppJson<NewAnswer>,
) -> Result<impl IntoResponse, CustomAppError> {
    // Get author id from session
    let (user_uuid, _) =
        crate::utils::get_user_id_from_session(&cookies, &state.redis_store, false).await?;

    // Create answer
    let create_answer = CreateAnswer {
        content: crate::utils::convert_markdown_to_html(&new_answer.content).await,
        raw_content: new_answer.content,
        author: user_uuid,
        question: question_id,
    };

    let answer = state.db_store.create_answer_in_db(create_answer).await?;

    Ok(axum::Json(answer).into_response())
}
