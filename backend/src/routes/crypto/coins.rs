use crate::{startup::AppState, utils::CustomAppError};
use axum::{extract::State, response::IntoResponse};

#[axum::debug_handler]
#[tracing::instrument(name = "all_coins", skip(state))]
pub async fn all_coins(State(state): State<AppState>) -> Result<impl IntoResponse, CustomAppError> {
    let coins = state.db_store.get_all_coins_from_db().await?;

    Ok(axum::Json(coins).into_response())
}
