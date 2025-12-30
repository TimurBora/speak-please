use sea_orm::{DeleteResult, TransactionTrait, entity::prelude::*, sqlx::types::chrono};
use shared::{
    entities::{prelude::*, refresh_tokens},
    errors::{AppError, AppResult, auth_errors::AuthError, jwt_errors::JwtError},
    utils::hashing::hash,
};
use ulid::Ulid;

pub struct RefreshTokenService;

impl RefreshTokenService {
    #[tracing::instrument(skip(db, refresh_token), fields(user_id = %user_id))]
    pub async fn create_refresh_token(
        db: &DatabaseConnection,
        refresh_token: &str,
        user_id: &str,
    ) -> AppResult<refresh_tokens::Model> {
        tracing::info!("Creating new refresh token for user");

        let hashed_token = hash(refresh_token).map_err(|e| {
            tracing::error!(error = %e, "Failed to hash refresh token");
            JwtError::CreationFailed
        })?;

        let new_refresh_token =
            refresh_tokens::ActiveModel::new_refresh_token(user_id, &hashed_token);

        let model = new_refresh_token.insert(db).await.map_err(|e| {
            tracing::error!(error = %e, "Database error while inserting refresh token");
            e
        })?;

        tracing::debug!(token_id = %model.ulid, "Refresh token successfully stored");
        Ok(model)
    }

    #[tracing::instrument(skip(db, raw_token))]
    pub async fn find_by_token(
        db: &DatabaseConnection,
        raw_token: &str,
    ) -> AppResult<Option<refresh_tokens::Model>> {
        tracing::debug!("Searching for refresh token record");

        let hashed_token = hash(raw_token).map_err(|_| JwtError::CreationFailed)?;

        Ok(refresh_tokens::Entity::find()
            .filter(refresh_tokens::Column::TokenHash.eq(hashed_token))
            .one(db)
            .await?)
    }

    #[tracing::instrument(skip(db, raw_token))]
    pub async fn delete_by_token(
        db: &DatabaseConnection,
        raw_token: &str,
    ) -> AppResult<DeleteResult> {
        tracing::info!("Deleting refresh token by hash");
        let hashed_token = hash(raw_token).unwrap();

        refresh_tokens::Entity::delete_many()
            .filter(refresh_tokens::Column::TokenHash.eq(hashed_token))
            .exec(db)
            .await
            .map_err(AppError::from)
    }

    #[tracing::instrument(skip(db, old_refresh_token_str))]
    pub async fn rotate_refresh_token(
        db: &DatabaseConnection,
        old_refresh_token_str: &str,
    ) -> AppResult<(String, String)> {
        tracing::info!("Starting refresh token rotation");

        let old_token = Self::find_by_token(db, old_refresh_token_str)
            .await?
            .ok_or_else(|| {
                tracing::warn!("Token rotation failed: old token not found in database");
                JwtError::InvalidToken
            })?;

        if old_token.expires_at < chrono::Utc::now() {
            tracing::warn!(token_id = %old_token.ulid, "Token rotation failed: token expired");
            old_token.delete(db).await?;
            return Err(JwtError::InvalidToken.into());
        }

        let user_id = old_token.user_id.clone();
        let raw_new_token = Ulid::new().to_string();
        let hashed_new_token = hash(&raw_new_token).map_err(|_| AuthError::HashError)?;

        tracing::debug!(user_id = %user_id, "Executing rotation transaction");

        db.transaction::<_, (), AppError>(|txn| {
            let user_id = user_id.clone();
            let hashed_token = hashed_new_token.clone();
            let old_token_to_delete = old_token.clone();

            Box::pin(async move {
                old_token_to_delete.delete(txn).await?;
                tracing::debug!("Old token deleted within transaction");

                let new_active_model =
                    refresh_tokens::ActiveModel::new_refresh_token(&user_id, &hashed_token);

                new_active_model.insert(txn).await?;
                tracing::debug!("New token inserted within transaction");
                Ok(())
            })
        })
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Token rotation transaction failed");
            e
        })?;

        tracing::info!(user_id = %user_id, "Token rotation completed successfully");
        Ok((user_id, raw_new_token))
    }

    #[tracing::instrument(skip(db))]
    pub async fn find_by_id(
        db: &DatabaseConnection,
        ulid: &str,
    ) -> AppResult<Option<refresh_tokens::Model>> {
        tracing::debug!(token_id = %ulid, "Finding refresh token by ID");
        RefreshToken::find_by_id(ulid)
            .one(db)
            .await
            .map_err(AppError::from)
    }

    #[tracing::instrument(skip(db))]
    pub async fn delete_by_id(db: &DatabaseConnection, ulid: &str) -> AppResult<DeleteResult> {
        tracing::info!(token_id = %ulid, "Deleting refresh token by ID");
        let res: DeleteResult = RefreshToken::delete_by_id(ulid).exec(db).await?;

        tracing::debug!(
            rows_affected = res.rows_affected,
            "Delete operation finished"
        );
        Ok(res)
    }
}
