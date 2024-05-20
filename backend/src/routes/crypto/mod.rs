use axum::{routing::get, Router};

mod coins;
mod price;
mod prices;

pub fn crypto_routes() -> Router<crate::startup::AppState> {
    Router::new()
        .route("/prices", get(price::crypto_price_handler))
        .route("/coins", get(coins::all_coins))
        .route("/coin_prices", get(prices::get_coin_market_data))
}
