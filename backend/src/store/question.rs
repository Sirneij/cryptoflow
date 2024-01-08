use sqlx::Row;
use std::collections::HashMap;

impl crate::store::Store {
    #[tracing::instrument(name = "get_question_from_db", skip(question_id))]
    pub async fn get_question_from_db(
        &self,
        transaction: Option<&mut sqlx::Transaction<'_, sqlx::Postgres>>,
        question_id: uuid::Uuid,
    ) -> Result<crate::models::QuestionAuthorWithTags, sqlx::Error> {
        let query = sqlx::query_as::<_, crate::models::QuestionAuthorWithTagsQueryResult>(
            crate::utils::QUESTION_AUTHOR_WITH_TAGS_QUERY,
        )
        .bind(question_id);

        let result = match transaction {
            Some(t) => query.fetch_one(&mut **t).await?,
            None => query.fetch_one(&self.connection).await?,
        };

        let question_author_with_tags = crate::models::QuestionAuthorWithTags {
            id: result.id,
            title: result.title,
            slug: result.slug,
            content: result.content,
            raw_content: result.raw_content,
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
            created_at: result.created_at,
            updated_at: result.updated_at,
            tags: serde_json::from_value(result.tags_json)
                .map_err(|e| sqlx::Error::Protocol(e.to_string().into()))?,
        };

        Ok(question_author_with_tags)
    }

    #[tracing::instrument(name = "create_question_in_db", skip(create_question))]
    pub async fn create_question_in_db(
        &self,
        create_question: crate::models::CreateQuestion,
    ) -> Result<crate::models::QuestionAuthorWithTags, sqlx::Error> {
        let mut transaction = self.connection.begin().await?;
        let q_id = match sqlx::query(
            "INSERT INTO questions (title, slug, content, raw_content, author) VALUES ($1, $2, $3, $4, $5) RETURNING id",
        )
        .bind(&create_question.title)
        .bind(&create_question.slug)
        .bind(&create_question.content)
        .bind(&create_question.raw_content)
        .bind(&create_question.author).map(|row: sqlx::postgres::PgRow| -> uuid::Uuid { row.get("id") })
        .fetch_one(&mut *transaction)
        .await {
            Ok(id) => id,
            Err(e) => return Err(e),
        };

        match sqlx::query("INSERT INTO question_tags (question, tag) SELECT $1, * FROM UNNEST($2)")
            .bind(q_id)
            .bind(&create_question.tags)
            .execute(&mut *transaction)
            .await
        {
            Ok(_) => {
                tracing::info!("Tag ids inserted successfully");
            }
            Err(e) => return Err(e),
        }

        let question_author_with_tags = self
            .get_question_from_db(Some(&mut transaction), q_id)
            .await?;

        transaction.commit().await?;

        Ok(question_author_with_tags)
    }

    #[tracing::instrument(name = "update_question_in_db", skip(update_question))]
    pub async fn update_question_in_db(
        &self,
        question_id: uuid::Uuid,
        update_question: crate::models::CreateQuestion,
    ) -> Result<crate::models::QuestionAuthorWithTags, sqlx::Error> {
        let mut transaction = self.connection.begin().await?;

        let q_id = match sqlx::query(
            "UPDATE questions SET title = $1, slug = $2, content = $3, raw_content = $4 WHERE id = $5 AND author = $6 RETURNING id",
        )
        .bind(&update_question.title)
        .bind(&update_question.slug)
        .bind(&update_question.content)
        .bind(&update_question.raw_content)
        .bind(question_id)
        .bind(&update_question.author)
        .map(|row: sqlx::postgres::PgRow| -> uuid::Uuid { row.get("id") })
        .fetch_one(&mut *transaction)
        .await {
            Ok(id) => id,
            Err(e) => return Err(e),
        };

        match sqlx::query("DELETE FROM question_tags WHERE question = $1")
            .bind(q_id)
            .execute(&mut *transaction)
            .await
        {
            Ok(_) => {
                tracing::info!("Tag ids deleted successfully");
            }
            Err(e) => return Err(e),
        }

        match sqlx::query("INSERT INTO question_tags (question, tag) SELECT $1, * FROM UNNEST($2)")
            .bind(q_id)
            .bind(&update_question.tags)
            .execute(&mut *transaction)
            .await
        {
            Ok(_) => {
                tracing::info!("Tag ids inserted successfully");
            }
            Err(e) => return Err(e),
        }

        let question_author_with_tags = self
            .get_question_from_db(Some(&mut transaction), q_id)
            .await?;

        transaction.commit().await?;

        Ok(question_author_with_tags)
    }

    #[tracing::instrument(name = "get_all_questions_from_db")]
    pub async fn get_all_questions_from_db(
        &self,
    ) -> Result<Vec<crate::models::QuestionAuthorWithTags>, sqlx::Error> {
        let results = sqlx::query_as::<_, crate::models::QuestionAuthorWithTagsQueryResult>(
            crate::utils::QUESTION_AUTHOR_WITH_TAGS_QUERY_ALL,
        )
        .fetch_all(&self.connection)
        .await?;

        let mut questions_map: HashMap<uuid::Uuid, crate::models::QuestionAuthorWithTags> =
            HashMap::new();

        for result in results {
            let tags: Vec<crate::models::Tag> = serde_json::from_value(result.tags_json)
                .map_err(|e| sqlx::Error::Protocol(e.to_string().into()))?;

            questions_map
                .entry(result.id)
                .or_insert(crate::models::QuestionAuthorWithTags {
                    id: result.id,
                    title: result.title,
                    slug: result.slug,
                    content: result.content,
                    raw_content: result.raw_content,
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
                    created_at: result.created_at,
                    updated_at: result.updated_at,
                    tags,
                });
        }

        Ok(questions_map.into_values().collect())
    }

    #[tracing::instrument(name = "delete_question_from_db")]
    pub async fn delete_question_from_db(
        &self,
        author_id: uuid::Uuid,
        question_id: uuid::Uuid,
    ) -> Result<(), sqlx::Error> {
        let deleted =
            sqlx::query("DELETE FROM questions WHERE id = $1 AND author = $2 RETURNING id")
                .bind(question_id)
                .bind(author_id)
                .fetch_optional(&self.connection)
                .await?;

        if deleted.is_none() {
            tracing::warn!(
                "Attempt to delete question with id {} by non-author {}",
                question_id,
                author_id
            );
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }
}
