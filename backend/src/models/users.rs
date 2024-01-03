#[derive(serde::Serialize, Debug)]
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
    pub date_joined: time::OffsetDateTime,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UserVisible {
    pub id: uuid::Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: Option<bool>,
    pub is_staff: Option<bool>,
    pub is_superuser: Option<bool>,
    pub thumbnail: Option<String>,
    pub date_joined: time::OffsetDateTime,
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
