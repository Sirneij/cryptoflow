use sqlx::Row;

impl crate::store::Store {
    #[tracing::instrument(name = "get_an_answer_from_db", skip(transaction, answer_id))]
    pub async fn get_an_answer_from_db(
        &self,
        transaction: Option<&mut sqlx::Transaction<'_, sqlx::Postgres>>,
        answer_id: uuid::Uuid,
    ) -> Result<crate::models::AnswerAuthor, sqlx::Error> {
        let query = sqlx::query_as::<_, crate::models::AnswerAuthorQueryResult>(
            crate::utils::ANSWER_AUTHOR_QUERY,
        )
        .bind(answer_id);

        let query_result = match transaction {
            Some(t) => query.fetch_one(&mut **t).await?,
            None => query.fetch_one(&self.connection).await?,
        };

        Ok(query_result.into())
    }

    #[tracing::instrument(name = "create_answer_in_db", skip(create_answer))]
    pub async fn create_answer_in_db(
        &self,
        create_answer: crate::models::CreateAnswer,
    ) -> Result<crate::models::AnswerAuthor, sqlx::Error> {
        let mut transaction = self.connection.begin().await?;
        let a_id = match sqlx::query(
            "INSERT INTO answers (content, raw_content, author, question) VALUES ($1, $2, $3, $4) RETURNING id",
        )
        .bind(&create_answer.content)
        .bind(&create_answer.raw_content)
        .bind(&create_answer.author)
        .bind(&create_answer.question)
        .map(|row: sqlx::postgres::PgRow| -> uuid::Uuid { row.get("id") })
        .fetch_one(&mut *transaction)
        .await {
            Ok(id) => id,
            Err(e) => return Err(e),
        };

        let answer_author = self
            .get_an_answer_from_db(Some(&mut transaction), a_id)
            .await?;

        transaction.commit().await?;

        Ok(answer_author)
    }

    #[tracing::instrument(name = "get_answers_from_db")]
    pub async fn get_answers_from_db(
        &self,
        transaction: Option<&mut sqlx::Transaction<'_, sqlx::Postgres>>,
        question_id: uuid::Uuid,
    ) -> Result<Vec<crate::models::AnswerAuthor>, sqlx::Error> {
        let query = sqlx::query_as::<_, crate::models::AnswerAuthorQueryResult>(
            crate::utils::ANSWER_AUTHOR_QUERY_VIA_QUESTION_ID,
        )
        .bind(question_id);

        let results = match transaction {
            Some(t) => query.fetch_all(&mut **t).await?,
            None => query.fetch_all(&self.connection).await?,
        };

        let answers = results.into_iter().map(|result| result.into()).collect();

        Ok(answers)
    }

    #[tracing::instrument(name = "delete_answer_from_db")]
    pub async fn delete_answer_from_db(
        &self,
        author_id: uuid::Uuid,
        answer_id: uuid::Uuid,
    ) -> Result<(), sqlx::Error> {
        let deleted = sqlx::query("DELETE FROM answers WHERE id = $1 AND author = $2 RETURNING id")
            .bind(answer_id)
            .bind(author_id)
            .fetch_optional(&self.connection)
            .await?;

        if deleted.is_none() {
            tracing::warn!(
                "Attempt to delete question with id {} by non-author {}",
                answer_id,
                author_id
            );
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }

    #[tracing::instrument(name = "update_answer_in_db", skip(update_answer))]
    pub async fn update_answer_in_db(
        &self,
        update_answer: crate::models::UpdateAnswer,
    ) -> Result<crate::models::AnswerAuthor, sqlx::Error> {
        let a_id = match sqlx::query(
            "UPDATE answers SET content = $1, raw_content = $2 WHERE id = $3 AND author = $4 RETURNING id",
        )
        .bind(&update_answer.content)
        .bind(&update_answer.raw_content)
        .bind(&update_answer.answer_id)
        .bind(&update_answer.author)
        .map(|row: sqlx::postgres::PgRow| -> uuid::Uuid { row.get("id") })
        .fetch_one(&self.connection)
        .await
        {
            Ok(id) => id,
            Err(e) => return Err(e),
        };

        let answer_author = self.get_an_answer_from_db(None, a_id).await?;

        Ok(answer_author)
    }
}
