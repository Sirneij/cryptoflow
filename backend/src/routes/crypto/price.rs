use crate::utils::{get_crypto_prices, CryptoPrices, CustomAppError, CustomAppJson};

#[derive(serde::Deserialize, Debug)]
pub struct CryptoPriceRequest {
    tags: Vec<String>,
    currency: String,
}

#[axum::debug_handler]
#[tracing::instrument(name = "crypto_price_handler")]
pub async fn crypto_price_handler(
    CustomAppJson(crypto_req): CustomAppJson<CryptoPriceRequest>,
) -> Result<CustomAppJson<CryptoPrices>, CustomAppError> {
    // Call the get_crypto_prices function with the tags
    let prices = get_crypto_prices(crypto_req.tags, &crypto_req.currency)
        .await
        .map_err(CustomAppError::from)?;

    // Return the prices wrapped in CustomAppJson
    Ok(CustomAppJson(prices))
}
