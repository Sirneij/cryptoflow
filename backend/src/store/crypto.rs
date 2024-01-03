impl crate::store::Store {
    #[tracing::instrument(name = "get_all_coins_from_db", skip(self))]
    pub async fn get_all_coins_from_db(&self) -> Result<Vec<crate::models::Tag>, sqlx::Error> {
        let tags = sqlx::query_as::<_, crate::models::Tag>("SELECT * FROM tags")
            .fetch_all(&self.connection)
            .await?;

        Ok(tags)
    }
    pub async fn update_coins(&self) {
        let url = "https://api.coingecko.com/api/v3/coins/list";

        match reqwest::get(url).await {
            Ok(response) => match response.json::<Vec<crate::models::Tag>>().await {
                Ok(coins) => {
                    let ids: Vec<String> = coins.iter().map(|c| c.id.clone()).collect();
                    let names: Vec<String> = coins.iter().map(|c| c.name.clone()).collect();
                    let symbols: Vec<String> = coins.iter().map(|c| c.symbol.clone()).collect();

                    let query = sqlx::query(
                        "INSERT INTO tags (id, name, symbol) SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[]) ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name, symbol = EXCLUDED.symbol",
                    )
                    .bind(&ids)
                    .bind(&names)
                    .bind(&symbols);

                    if let Err(e) = query.execute(&self.connection).await {
                        tracing::error!("Failed to update coins: {}", e);
                    }
                }
                Err(e) => tracing::error!("Failed to parse coins from response: {}", e),
            },
            Err(e) => tracing::error!("Failed to fetch coins from CoinGecko: {}", e),
        }
    }
}
