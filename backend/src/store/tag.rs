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
    #[tracing::instrument(name = "validate_tags", skip(tag_ids))]
    pub async fn validate_tags(&self, tag_ids: Vec<String>) -> Result<(), sqlx::Error> {
        if tag_ids.is_empty() {
            return Err(sqlx::Error::RowNotFound);
        }
        let rows = sqlx::query("SELECT id FROM tags WHERE id = ANY($1)")
            .bind(&tag_ids)
            .fetch_all(&self.connection)
            .await?;

        if rows.len() == tag_ids.len() {
            Ok(())
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }
}
