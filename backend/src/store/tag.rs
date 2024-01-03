use sqlx::Row;

impl crate::store::Store {
    #[tracing::instrument(name = "get_tag_ids_from_db", skip(tag_names))]
    pub async fn get_tag_ids_from_db(
        &self,
        tag_names: Vec<String>,
    ) -> Result<Vec<uuid::Uuid>, sqlx::Error> {
        match sqlx::query("SELECT id FROM tags WHERE name = ANY($1)")
            .bind(&tag_names)
            .map(|row: sqlx::postgres::PgRow| -> uuid::Uuid { row.get("id") })
            .fetch_all(&self.connection)
            .await
        {
            Ok(ids) => Ok(ids),
            Err(e) => Err(e),
        }
    }
}
