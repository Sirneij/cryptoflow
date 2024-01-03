use crate::utils::validate_authentication_session;
use axum::{
    routing::{delete, get, post},
    Router,
};

mod answer;
mod answers;
mod ask;
mod questions;

pub fn qa_routes(state: crate::startup::AppState) -> Router<crate::startup::AppState> {
    Router::new()
        .route("/ask", post(ask::ask_question))
        .route("/answer/:question_id", post(answer::answer_question))
        .route(
            "/questions/:question_id",
            delete(questions::delete_a_question),
        )
        .route("/answers/:answer_id", delete(answers::delete_an_answer))
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            validate_authentication_session,
        ))
        .route("/questions", get(questions::all_questions))
        .route("/questions/:question_id", get(questions::get_question))
        .route(
            "/questions/:question_id/answers",
            get(answers::question_answers),
        )
}
