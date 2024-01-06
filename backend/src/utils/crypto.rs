use reqwest;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CryptoPrice {
    name: String,
    price: f64,
}

pub type CryptoPrices = Vec<CryptoPrice>;

#[tracing::instrument(name = "get_crypto_prices")]
pub async fn get_crypto_prices(
    cryptos: String,
    currency: &str,
) -> Result<CryptoPrices, reqwest::Error> {
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}",
        cryptos,
        currency.to_lowercase()
    );

    let response: HashMap<String, Value> = reqwest::get(&url)
        .await?
        .json::<HashMap<String, Value>>()
        .await?;

    let mut prices = CryptoPrices::new();
    for (name, data) in response {
        if let Some(price) = data.get("usd").and_then(|v| v.as_f64()) {
            prices.push(CryptoPrice { name, price });
        }
    }

    Ok(prices)
}
