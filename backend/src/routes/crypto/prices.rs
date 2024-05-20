use crate::utils::{CustomAppError, CustomAppJson};
use axum::extract::Query;
use std::collections::HashMap;

#[derive(serde::Deserialize, Debug)]
pub struct CoinMarketDataRequest {
    tags: String,
    currency: String,
    days: i32,
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct CoinMarketData {
    prices: Vec<Vec<f64>>,
    market_caps: Vec<Vec<f64>>,
    total_volumes: Vec<Vec<f64>>,
}

#[axum::debug_handler]
#[tracing::instrument(name = "get_coin_market_data")]
pub async fn get_coin_market_data(
    Query(coin_req): Query<CoinMarketDataRequest>,
) -> Result<CustomAppJson<HashMap<String, CoinMarketData>>, CustomAppError> {
    let tag_ids: Vec<String> = coin_req.tags.split(',').map(|s| s.to_string()).collect();
    let mut responses = HashMap::new();
    for tag_id in tag_ids {
        let url = format!(
            "https://api.coingecko.com/api/v3/coins/{}/market_chart?vs_currency={}&days={}",
            &tag_id, coin_req.currency, coin_req.days
        );
        match reqwest::get(&url).await {
            Ok(response) => match response.json::<CoinMarketData>().await {
                Ok(data) => {
                    responses.insert(tag_id, data);
                }
                Err(e) => {
                    tracing::error!("Failed to parse market data from response: {}", e);
                }
            },
            Err(e) => {
                tracing::error!("Failed to fetch market data from CoinGecko: {}", e);
            }
        }
    }

    Ok(CustomAppJson(responses))
}
