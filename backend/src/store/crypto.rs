#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Coin {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub image: String,
    pub current_price: f64,
    pub market_cap: f64,
    pub market_cap_rank: Option<i32>,
    pub fully_diluted_valuation: Option<f64>,
    pub total_volume: f64,
    pub high_24h: f64,
    pub low_24h: f64,
    pub price_change_24h: f64,
    pub price_change_percentage_24h: f64,
    pub market_cap_change_24h: f64,
    pub market_cap_change_percentage_24h: f64,
    pub circulating_supply: f64,
    pub total_supply: Option<f64>,
    pub max_supply: Option<f64>,
    pub ath: f64,
    pub ath_change_percentage: f64,
    pub ath_date: String,
    pub atl: f64,
    pub atl_change_percentage: f64,
    pub atl_date: String,
    pub roi: Option<ROI>,
    pub last_updated: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ROI {
    pub times: f64,
    pub currency: String,
    pub percentage: f64,
}

impl crate::store::Store {
    #[tracing::instrument(name = "get_all_coins_from_db", skip(self))]
    pub async fn get_all_coins_from_db(&self) -> Result<Vec<crate::models::Tag>, sqlx::Error> {
        let tags = sqlx::query_as::<_, crate::models::Tag>("SELECT * FROM tags")
            .fetch_all(&self.connection)
            .await?;

        Ok(tags)
    }
    pub async fn update_coins(&self) {
        let settings = crate::settings::get_settings().expect("Failed to get settings");
        let url = format!(
            "{}/coins/markets?vs_currency=USD&days=30",
            &settings.coingecko.api_url
        );

        let client = reqwest::Client::new();

        match client
            .get(url)
            .header("x-cg-demo-api-key", &settings.coingecko.api_key)
            .send()
            .await
        {
            Ok(response) => match response.json::<Vec<Coin>>().await {
                Ok(coins) => {
                    let ids: Vec<String> = coins.iter().map(|c| c.id.clone()).collect();
                    let names: Vec<String> = coins.iter().map(|c| c.name.clone()).collect();
                    let symbols: Vec<String> = coins.iter().map(|c| c.symbol.clone()).collect();
                    let images: Vec<String> = coins.iter().map(|c| c.image.clone()).collect();
                    let market_cap_ranks: Vec<i32> = coins
                        .iter()
                        .map(|c| {
                            c.market_cap_rank
                                .map(|rank| rank as i32)
                                .unwrap_or_else(|| -1)
                        })
                        .collect();

                    let query = sqlx::query(
                        "INSERT INTO tags (id, name, symbol, image, market_cap_rank) SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[], $5::int[]) ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name, symbol = EXCLUDED.symbol, image = EXCLUDED.image, market_cap_rank = EXCLUDED.market_cap_rank",
                    )
                    .bind(&ids)
                    .bind(&names)
                    .bind(&symbols)
                    .bind(&images)
                    .bind(&market_cap_ranks);

                    if let Err(e) = query.execute(&self.connection).await {
                        tracing::error!("Failed to update coins: {}", e);
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to parse coins from response: {:?}", e);
                }
            },
            Err(e) => tracing::error!("Failed to fetch coins from CoinGecko: {:?}", e),
        }
    }
}
