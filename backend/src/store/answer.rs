use sqlx::Row;

impl crate::store::Store {
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

        let query_result = sqlx::query_as::<_, crate::models::AnswerAuthorQueryResult>(
            crate::utils::ANSWER_AUTHOR_QUERY,
        )
        .bind(a_id)
        .fetch_one(&mut *transaction)
        .await?;

        let answer_author = crate::models::AnswerAuthor {
            id: query_result.id,
            content: query_result.content,
            raw_content: query_result.raw_content,
            created_at: query_result.created_at,
            updated_at: query_result.updated_at,
            author: crate::models::UserVisible {
                id: query_result.user_id,
                email: query_result.user_email,
                first_name: query_result.user_first_name,
                last_name: query_result.user_last_name,
                is_active: query_result.user_is_active,
                is_staff: query_result.user_is_staff,
                is_superuser: query_result.user_is_superuser,
                thumbnail: query_result.user_thumbnail,
                date_joined: query_result.user_date_joined,
            },
        };

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

        let answers = results
            .into_iter()
            .map(|result| crate::models::AnswerAuthor {
                id: result.id,
                content: result.content,
                raw_content: result.raw_content,
                created_at: result.created_at,
                updated_at: result.updated_at,
                author: crate::models::UserVisible {
                    id: result.user_id,
                    email: result.user_email,
                    first_name: result.user_first_name,
                    last_name: result.user_last_name,
                    is_active: result.user_is_active,
                    is_staff: result.user_is_staff,
                    is_superuser: result.user_is_superuser,
                    thumbnail: result.user_thumbnail,
                    date_joined: result.user_date_joined,
                },
            })
            .collect();

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
}
