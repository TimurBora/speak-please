use sea_orm::*;
use shared::entities::{prelude::*, users};

use crate::utils::hashing::{hash, verify_hash};

//server/src/service/
//└── user/
//    ├── mod.rs           # Объединяет все части в единый интерфейс
//    ├── auth.rs          # Только регистрация, логин и пароли (то, что ты уже написал)
//    ├── progression.rs   # Опыт (XP), уровни, расчет прогресса
//    └── profile.rs       # Получение данных профиля, смена аватара, статуса
// TODO: THINK ABOUT IT

pub struct UserService;

#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,
    DatabaseError(DbErr),
}

impl UserService {
    pub async fn create_user(
        db: &DatabaseConnection,
        username: String,
        email: String,
        password: String,
    ) -> Result<users::Model, anyhow::Error> {
        let password_hash = hash(&password)?;
        let new_user = users::ActiveModel::new_user(username, email, password_hash);

        let model = new_user.insert(db).await?;
        Ok(model)
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<users::Model>, DbErr> {
        User::find_by_id(id).one(db).await
    }

    pub async fn find_by_username(
        db: &DatabaseConnection,
        username: &str,
    ) -> Result<Option<users::Model>, DbErr> {
        User::find()
            .filter(users::Column::Username.eq(username))
            .one(db)
            .await
    }

    pub async fn login_by_username_and_password(
        db: &DatabaseConnection,
        username: &str,
        password: &str,
    ) -> Result<users::Model, AuthError> {
        let user = Self::find_by_username(db, username)
            .await
            .map_err(AuthError::DatabaseError)?
            .ok_or(AuthError::InvalidCredentials)?;

        let is_valid = verify_hash(password, &user.password_hash)
            .map_err(|_| AuthError::InvalidCredentials)?;

        if is_valid {
            Ok(user)
        } else {
            Err(AuthError::InvalidCredentials)
        }
    }

    pub async fn add_xp(
        db: &DatabaseConnection,
        user_id: i32,
        xp_to_add: i32,
    ) -> Result<users::Model, String> {
        let user_model = User::find_by_id(user_id)
            .one(db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("User not found")?;

        let mut user_active: users::ActiveModel = user_model.into();

        let current_xp = user_active.total_xp_accumulated.unwrap();
        let new_total_xp = current_xp + xp_to_add;
        let new_balance = user_active.xp_balance.unwrap() + xp_to_add;

        let new_level = (new_total_xp / 1000) + 1;

        user_active.total_xp_accumulated = Set(new_total_xp);
        user_active.xp_balance = Set(new_balance);
        user_active.level = Set(new_level);

        user_active.update(db).await.map_err(|e| e.to_string())
    }
}
