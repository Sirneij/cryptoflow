use sqlx::Row;

impl crate::store::Store {
    #[tracing::instrument(name = "get_user_by_id", fields(user_id = id.to_string()))]
    pub async fn get_user_by_id(&self, id: uuid::Uuid) -> Result<crate::models::User, sqlx::Error> {
        let row = sqlx::query(
        r#"
        SELECT 
            id, email, password, first_name, last_name, is_active, is_staff, is_superuser, thumbnail, date_joined
        FROM users
        WHERE id = $1 AND is_active = true
        "#,
    )
    .bind(id)
    .fetch_one(&self.connection)
    .await?;

        Ok(crate::models::User {
            id: row.get("id"),
            email: row.get("email"),
            password: row.get("password"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            is_active: row.get("is_active"),
            is_staff: row.get("is_staff"),
            is_superuser: row.get("is_superuser"),
            thumbnail: row.get("thumbnail"),
            date_joined: row.get("date_joined"),
        })
    }

    #[tracing::instrument(name = "get_user_by_email", fields(user_email = email))]
    pub async fn get_user_by_email(&self, email: &str) -> Result<crate::models::User, sqlx::Error> {
        let row = sqlx::query(
        r#"
        SELECT 
            id, email, password, first_name, last_name, is_active, is_staff, is_superuser, thumbnail, date_joined
        FROM users
        WHERE email = $1 AND is_active = true
        "#,
    )
    .bind(email)
    .fetch_one(&self.connection)
    .await?;

        Ok(crate::models::User {
            id: row.get("id"),
            email: row.get("email"),
            password: row.get("password"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            is_active: row.get("is_active"),
            is_staff: row.get("is_staff"),
            is_superuser: row.get("is_superuser"),
            thumbnail: row.get("thumbnail"),
            date_joined: row.get("date_joined"),
        })
    }

    #[tracing::instrument(name = "create_user", skip(password),fields(user_first_name = first_name, user_last_name = last_name, user_email = email))]
    pub async fn create_user(
        &self,
        first_name: &str,
        last_name: &str,
        email: &str,
        password: &str,
    ) -> Result<crate::models::UserVisible, sqlx::Error> {
        let row = sqlx::query(
        r#"
        INSERT INTO users (first_name, last_name, email, password)
            VALUES ($1, $2, $3, $4)
        RETURNING
            id, email, first_name, last_name, is_active, is_staff, is_superuser, thumbnail, date_joined
        "#,
    )
    .bind(first_name)
    .bind(last_name)
    .bind(email)
    .bind(password)
    .fetch_one(&self.connection)
    .await?;

        Ok(crate::models::UserVisible {
            id: row.get("id"),
            email: row.get("email"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            is_active: row.get("is_active"),
            is_staff: row.get("is_staff"),
            is_superuser: row.get("is_superuser"),
            thumbnail: row.get("thumbnail"),
            date_joined: row.get("date_joined"),
        })
    }

    #[tracing::instrument(name = "activate_user", fields(user_id = id.to_string()))]
    pub async fn activate_user(&self, id: &uuid::Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
        UPDATE users
        SET is_active = true
        WHERE id = $1
        "#,
        )
        .bind(id)
        .execute(&self.connection)
        .await?;

        Ok(())
    }

    #[tracing::instrument(name="create_super_user_in_db.", skip(settings), fields(user_email = settings.superuser.email, user_first_name = settings.superuser.first_name, user_last_name = settings.superuser.last_name))]
    pub async fn create_super_user_in_db(&self, settings: &crate::settings::Settings) {
        let new_super_user = crate::models::NewUser {
            email: settings.superuser.email.clone(),
            password: crate::utils::hash_password(&settings.superuser.password.as_bytes()).await,
            first_name: settings.superuser.first_name.clone(),
            last_name: settings.superuser.last_name.clone(),
        };

        match sqlx::query(
            "INSERT INTO users 
                    (email, password, first_name, last_name, is_active, is_staff, is_superuser) 
                VALUES ($1, $2, $3, $4, true, true, true) 
                ON CONFLICT (email) 
                DO UPDATE 
                    SET 
                        first_name=EXCLUDED.first_name,
                        last_name=EXCLUDED.last_name
                RETURNING id",
        )
        .bind(new_super_user.email)
        .bind(&new_super_user.password)
        .bind(new_super_user.first_name)
        .bind(new_super_user.last_name)
        .map(|row: sqlx::postgres::PgRow| -> uuid::Uuid { row.get("id") })
        .fetch_one(&self.connection)
        .await
        {
            Ok(id) => {
                tracing::info!("Super user created successfully {:#?}.", id);
                id
            }

            Err(e) => {
                tracing::error!("Failed to insert user into DB: {:#?}.", e);
                uuid::Uuid::new_v4()
            }
        };
    }
}
