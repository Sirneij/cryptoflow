#[derive(serde::Serialize, Debug, sqlx::FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: Option<bool>,
    pub is_staff: Option<bool>,
    pub is_superuser: Option<bool>,
    pub thumbnail: Option<String>,
    pub date_joined: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, sqlx::FromRow, Clone)]
pub struct UserVisible {
    pub id: uuid::Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: Option<bool>,
    pub is_staff: Option<bool>,
    pub is_superuser: Option<bool>,
    pub thumbnail: Option<String>,
    pub date_joined: chrono::DateTime<chrono::Utc>,
}

impl From<User> for UserVisible {
    fn from(user: User) -> Self {
        UserVisible {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            is_active: user.is_active,
            is_staff: user.is_staff,
            is_superuser: user.is_superuser,
            thumbnail: user.thumbnail,
            date_joined: user.date_joined,
        }
    }
}

#[derive(serde::Serialize)]
pub struct LoggedInUser {
    pub id: uuid::Uuid,
    pub email: String,
    pub password: String,
    pub is_staff: bool,
    pub is_superuser: bool,
}
#[derive(serde::Deserialize, Debug)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct ActivateUser {
    pub id: uuid::Uuid,
    pub token: String,
}
