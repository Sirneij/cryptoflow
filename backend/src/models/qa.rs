use serde_json::Value as JsonValue;
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, serde::Serialize, Clone)]
pub struct Question {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub raw_content: String,
    pub author: Uuid,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(serde::Deserialize, Debug)]
pub struct NewQuestion {
    pub title: String,
    pub content: String,
    pub tags: String,
}

#[derive(serde::Serialize, Debug)]
pub struct CreateQuestion {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub raw_content: String,
    pub author: Uuid,
    pub tags: Vec<String>,
}

#[derive(serde::Serialize, Debug)]
pub struct Answer {
    pub id: Uuid,
    pub content: String,
    pub raw_content: String,
    pub author: Uuid,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(serde::Deserialize, Debug)]
pub struct NewAnswer {
    pub content: String,
}

#[derive(serde::Serialize, Debug)]
pub struct CreateAnswer {
    pub content: String,
    pub raw_content: String,
    pub author: Uuid,
    pub question: Uuid,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, FromRow)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub symbol: String,
}

#[derive(serde::Serialize, Debug)]
pub struct QuestionAuthorWithTags {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub raw_content: String,
    pub author: crate::models::UserVisible,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub tags: Vec<Tag>,
}

#[derive(serde::Serialize, Debug)]
pub struct AnswerAuthor {
    pub id: Uuid,
    pub content: String,
    pub raw_content: String,
    pub author: crate::models::UserVisible,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(FromRow, Debug)]
pub struct QuestionAuthorWithTagsQueryResult {
    // Fields from `questions`
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub raw_content: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    // JSON aggregation of tags
    pub tags_json: JsonValue,
    // Fields from `users`
    pub user_id: Uuid,
    pub user_email: String,
    pub user_first_name: String,
    pub user_last_name: String,
    pub user_is_active: Option<bool>,
    pub user_is_staff: Option<bool>,
    pub user_is_superuser: Option<bool>,
    pub user_thumbnail: Option<String>,
    pub user_date_joined: OffsetDateTime,
}

#[derive(FromRow, Debug)]
pub struct AnswerAuthorQueryResult {
    // Fields from `answers`
    pub id: Uuid,
    pub content: String,
    pub raw_content: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    // Fields from `users`
    pub user_id: Uuid,
    pub user_email: String,
    pub user_first_name: String,
    pub user_last_name: String,
    pub user_is_active: Option<bool>,
    pub user_is_staff: Option<bool>,
    pub user_is_superuser: Option<bool>,
    pub user_thumbnail: Option<String>,
    pub user_date_joined: OffsetDateTime,
}
