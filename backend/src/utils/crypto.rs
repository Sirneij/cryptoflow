use reqwest;
use serde_json::Value;
use std::collections::HashMap;

pub type CryptoPrices = HashMap<String, f64>;

#[tracing::instrument(name = "get_crypto_prices")]
pub async fn get_crypto_prices(
    cryptos: Vec<String>,
    currency: &str,
) -> Result<CryptoPrices, reqwest::Error> {
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}",
        cryptos.join(","),
        currency.to_lowercase()
    );

    let response: HashMap<String, Value> = reqwest::get(&url)
        .await?
        .json::<HashMap<String, Value>>()
        .await?;

    tracing::info!("Got response: {:?}", response);

    // Transform the response into a HashMap of cryptocurrency names to their USD prices
    let mut prices = CryptoPrices::new();
    for (crypto, data) in response {
        if let Some(price) = data.get("usd").and_then(|v| v.as_f64()) {
            prices.insert(crypto, price);
        }
    }

    Ok(prices)
}
