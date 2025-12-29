use sea_orm::{DeleteResult, TransactionTrait, entity::prelude::*, sqlx::types::chrono};
use shared::{
    entities::{prelude::*, refresh_tokens},
    errors::{AppError, AppResult, auth_errors::AuthError, jwt_errors::JwtError},
    utils::hashing::hash,
};
use ulid::Ulid;

pub struct RefreshTokenService;

impl RefreshTokenService {
    pub async fn create_refresh_token(
        db: &DatabaseConnection,
        refresh_token: &str,
        user_id: &str,
    ) -> AppResult<refresh_tokens::Model> {
        let hashed_token = hash(refresh_token).map_err(|_| JwtError::CreationFailed)?;
        let new_refresh_token =
            refresh_tokens::ActiveModel::new_refresh_token(user_id, &hashed_token);

        let model = new_refresh_token.insert(db).await?;
        Ok(model)
    }

    pub async fn find_by_token(
        db: &DatabaseConnection,
        raw_token: &str,
    ) -> AppResult<Option<refresh_tokens::Model>> {
        let hashed_token = hash(raw_token).map_err(|_| JwtError::CreationFailed)?;

        Ok(refresh_tokens::Entity::find()
            .filter(refresh_tokens::Column::TokenHash.eq(hashed_token))
            .one(db)
            .await?)
    }

    pub async fn delete_by_token(
        db: &DatabaseConnection,
        raw_token: &str,
    ) -> AppResult<DeleteResult> {
        let hashed_token = hash(raw_token).unwrap();

        refresh_tokens::Entity::delete_many()
            .filter(refresh_tokens::Column::TokenHash.eq(hashed_token))
            .exec(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn rotate_refresh_token(
        db: &DatabaseConnection,
        old_refresh_token_str: &str,
    ) -> AppResult<(String, String)> {
        let old_token = Self::find_by_token(db, old_refresh_token_str)
            .await?
            .ok_or(JwtError::InvalidToken)?;

        if old_token.expires_at < chrono::Utc::now() {
            old_token.delete(db).await?;
            return Err(JwtError::InvalidToken.into());
        }

        let user_id = old_token.user_id.clone();
        let raw_new_token = Ulid::new().to_string();

        let hashed_new_token = hash(&raw_new_token).map_err(|_| AuthError::HashError)?;

        db.transaction::<_, (), AppError>(|txn| {
            let user_id = user_id.clone();
            let hashed_token = hashed_new_token.clone();
            let old_token_to_delete = old_token.clone();

            Box::pin(async move {
                old_token_to_delete.delete(txn).await?;

                let new_active_model =
                    refresh_tokens::ActiveModel::new_refresh_token(&user_id, &hashed_token);

                new_active_model.insert(txn).await?;
                Ok(())
            })
        })
        .await?;

        Ok((user_id, raw_new_token))
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        ulid: &str,
    ) -> AppResult<Option<refresh_tokens::Model>> {
        RefreshToken::find_by_id(ulid)
            .one(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn delete_by_id(db: &DatabaseConnection, ulid: &str) -> AppResult<DeleteResult> {
        let res: DeleteResult = RefreshToken::delete_by_id(ulid).exec(db).await?;

        Ok(res)
    }
}
