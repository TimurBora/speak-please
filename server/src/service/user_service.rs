//server/src/service/
//└── user/
//    ├── mod.rs           # Объединяет все части в единый интерфейс
//    ├── auth.rs          # Только регистрация, логин и пароли (то, что ты уже написал)
//    ├── progression.rs   # Опыт (XP), уровни, расчет прогресса
//    └── profile.rs       # Получение данных профиля, смена аватара, статуса
// TODO: THINK ABOUT IT

use sea_orm::entity::prelude::*;
use shared::{
    entities::{prelude::*, users},
    errors::{AppError, AppResult, DbResultExt, auth_errors::AuthError},
    utils::hashing::{hash, verify_hash},
};

pub struct UserService;

impl UserService {
    #[tracing::instrument(skip(db, password), fields(user.email = %email))]
    pub async fn create_user(
        db: &DatabaseConnection,
        username: String,
        email: String,
        password: String,
    ) -> AppResult<users::Model> {
        tracing::info!("Creating a new user account");

        let password_hash = hash(&password).map_err(|_| {
            tracing::error!("Failed to hash password during user creation");
            AuthError::HashError
        })?;

        let new_user = users::ActiveModel::new_user(username, email, password_hash);

        let model = new_user.insert(db).await.map_db_error().map_err(|e| {
            tracing::error!(error = %e, "Failed to insert user into database");
            e
        })?;

        tracing::info!(user.id = %model.ulid, "User successfully created");
        Ok(model)
    }

    #[tracing::instrument(skip(db))]
    pub async fn find_by_id(db: &DatabaseConnection, id: &str) -> AppResult<Option<users::Model>> {
        tracing::debug!("Searching for user by ID");
        User::find_by_id(id).one(db).await.map_err(AppError::from)
    }

    #[tracing::instrument(skip(db))]
    pub async fn find_by_username(
        db: &DatabaseConnection,
        username: &str,
    ) -> AppResult<Option<users::Model>> {
        tracing::debug!("Searching for user by username");
        User::find()
            .filter(users::Column::Username.eq(username))
            .one(db)
            .await
            .map_err(AppError::from)
    }

    #[tracing::instrument(skip(db))]
    pub async fn find_by_email(
        db: &DatabaseConnection,
        email: &str,
    ) -> AppResult<Option<users::Model>> {
        tracing::debug!("Searching for user by email");
        User::find()
            .filter(users::Column::Email.eq(email))
            .one(db)
            .await
            .map_err(AppError::from)
    }

    #[tracing::instrument(skip(db, password))]
    pub async fn login_by_email_and_password(
        db: &DatabaseConnection,
        email: &str,
        password: &str,
    ) -> AppResult<users::Model> {
        tracing::info!("Attempting login for user");

        let user = Self::find_by_email(db, email).await?.ok_or_else(|| {
            tracing::warn!("Login failed: user not found");
            AuthError::InvalidCredentials
        })?;

        let is_valid = verify_hash(password, &user.password_hash).map_err(|_| {
            tracing::error!("Password verification internal error");
            AuthError::InvalidCredentials
        })?;

        if !is_valid {
            tracing::warn!(user.id = %user.ulid, "Login failed: incorrect password");
            return Err(AuthError::InvalidCredentials.into());
        }

        tracing::info!(user.id = %user.ulid, "User authenticated successfully");
        Ok(user)
    }
}
