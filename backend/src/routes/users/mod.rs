use crate::utils::validate_authentication_session;
use axum::{
    routing::{get, post},
    Router,
};

mod activate_account;
mod current_user;
mod login;
mod logout;
mod register;

pub fn users_routes(state: crate::startup::AppState) -> Router<crate::startup::AppState> {
    Router::new()
        .route("/logout", post(logout::logout_user))
        .route("/current", get(current_user::get_current_user))
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            validate_authentication_session,
        ))
        .route("/login", post(login::login_user))
        .route("/register", post(register::register_user))
        .route("/activate", post(activate_account::activate_user_account))
}
