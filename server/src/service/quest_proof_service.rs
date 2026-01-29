use chrono::Utc;
use migration::Expr;
use sea_orm::*;
use serde_json::json;
use shared::{
    errors::{AppError, AppResult},
    models::{
        quest_proof_dto::{ProofDetailsResponse, ProofFeedResponse},
        user_quest_status_dto::QuestStatus,
    },
};
use tracing::info;
use ulid::Ulid;

use crate::{
    entities::{
        prelude::{Quest, QuestProof, QuestProofBeliefs, User, UserQuestStatus},
        quest_proof_beliefs,
        quest_proofs::{self, ActiveModel, Model, ProofStatus},
        user_quest_status,
    },
    file_storage::s3_client::S3Manager,
    service::user_quest_status_service::UserQuestService,
};

pub struct DetailedProof {
    pub proof: Model,
    pub username: String,
    pub avatar_url: Option<String>,
    pub quest_title: String,
    pub quest_description: Option<String>,
    pub xp_reward: u32,
    pub photo_urls: Vec<String>,
    pub voice_urls: Vec<String>,
    pub beliefs_count: u32,
    pub is_believed: bool,
}

impl From<DetailedProof> for ProofDetailsResponse {
    fn from(d: DetailedProof) -> Self {
        Self {
            ulid: d.proof.ulid,
            user_id: d.proof.user_id,
            username: d.username,
            avatar_url: d.avatar_url,
            quest_id: d.proof.quest_id,
            quest_title: d.quest_title,
            quest_description: d.quest_description,
            xp_reward: d.xp_reward,
            proof_text: d.proof.proof_text,
            status: format!("{:?}", d.proof.status),
            photo_urls: d.photo_urls,
            voice_urls: d.voice_urls,
            created_at: d.proof.created_at,

            beliefs_count: d.beliefs_count,
            is_believed: d.is_believed,
        }
    }
}

pub struct QuestProofService;

impl QuestProofService {
    // Initialize proof record with "Uploading" status; files are not yet verified.
    pub async fn init_proof_submition(
        db: &DatabaseConnection,
        s3: &S3Manager,
        user_id: String,
        quest_id: String,
        proof_text: Option<String>,
        photo_count: u32,
        voice_count: u32,
    ) -> Result<(Model, Vec<String>, Vec<String>), DbErr> {
        let has_text = proof_text.as_ref().is_some_and(|t| !t.trim().is_empty());
        if !has_text && photo_count == 0 && voice_count == 0 {
            return Err(DbErr::Custom("Proof must contain something".into()));
        }

        let proof_id = Ulid::new().to_string();

        // Generate pre-signed S3 URLs for photo uploads (expires in 1 hour)
        let mut photo_urls = Vec::new();
        let mut photo_keys = Vec::new();
        for i in 0..photo_count {
            let key = S3Manager::generate_proof_key(&user_id, &proof_id, i, "jpg", false);
            let url = s3
                .get_upload_url(&key, "image/jpeg", 3600)
                .await
                .map_err(|e| DbErr::Custom(e.to_string()))?;
            photo_urls.push(url);
            photo_keys.push(key);
        }

        // Generate pre-signed S3 URLs for voice uploads (expires in 1 hour)
        let mut voice_urls = Vec::new();
        let mut voice_keys = Vec::new();
        for i in 0..voice_count {
            let key = S3Manager::generate_proof_key(&user_id, &proof_id, i, "ogg", true);
            let url = s3
                .get_upload_url(&key, "audio/ogg", 3600)
                .await
                .map_err(|e| DbErr::Custom(e.to_string()))?;
            voice_urls.push(url);
            voice_keys.push(key);
        }

        let active_model = ActiveModel {
            ulid: Set(proof_id),
            user_id: Set(user_id),
            quest_id: Set(quest_id),
            proof_text: Set(proof_text),
            photos: Set(if photo_keys.is_empty() {
                None
            } else {
                Some(json!(photo_keys))
            }),
            voice_notes: Set(if voice_keys.is_empty() {
                None
            } else {
                Some(json!(voice_keys))
            }),
            status: Set(ProofStatus::Uploading),
            beliefs_count: Set(0),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
        };

        let model = active_model.insert(db).await?;
        Ok((model, photo_urls, voice_urls))
    }

    // Confirm proof record with "InPending" status; files are verified and ready to use
    pub async fn confirm_proof_upload(
        db: &DatabaseConnection,
        proof_id: String,
    ) -> Result<Model, DbErr> {
        let txn = db.begin().await?;

        let proof = QuestProof::find_by_id(proof_id)
            .one(&txn)
            .await?
            .ok_or(DbErr::Custom("Proof not found".into()))?;

        let mut active_model: ActiveModel = proof.clone().into();
        active_model.status = Set(ProofStatus::Pending);
        active_model.updated_at = Set(Utc::now());

        let updated_proof = active_model.update(&txn).await?;

        let today = proof.updated_at.date_naive();

        UserQuestStatus::update_many()
            .col_expr(
                crate::entities::user_quest_status::Column::QuestStatus,
                Expr::value(QuestStatus::InPending),
            )
            .filter(crate::entities::user_quest_status::Column::QuestId.eq(&proof.quest_id))
            .filter(crate::entities::user_quest_status::Column::AssignedAt.eq(today))
            .exec(&txn)
            .await?;

        txn.commit().await?;
        Ok(updated_proof)
    }

    pub async fn update_status(
        db: &DatabaseConnection,
        proof_id: &str,
        new_status: ProofStatus,
    ) -> Result<Model, DbErr> {
        let txn = db.begin().await?;

        let proof = QuestProof::find_by_id(proof_id.to_string())
            .one(&txn)
            .await?
            .ok_or(DbErr::RecordNotFound("Proof not found".into()))?;

        let mut active_model: ActiveModel = proof.clone().into();
        active_model.status = Set(new_status.clone());
        let updated_proof = active_model.update(&txn).await?;

        if let ProofStatus::Approved = new_status {
            UserQuestService::complete_quest_internal(
                &txn,
                &proof.user_id,
                &proof.quest_id,
                proof.updated_at.date_naive(),
            )
            .await
            .map_err(|e| DbErr::Custom(e.to_string()))?;
        }

        txn.commit().await?;
        Ok(updated_proof)
    }

    pub async fn get_proof_full_details(
        db: &DatabaseConnection,
        s3: &S3Manager,
        proof_id: &str,
        current_user_id: &str,
    ) -> Result<Option<DetailedProof>, DbErr> {
        let proof_option = QuestProof::find_by_id(proof_id.to_string())
            .find_also_related(User)
            .find_also_related(Quest)
            .one(db)
            .await?;

        if let Some((proof, Some(user), Some(quest))) = proof_option {
            let photo_urls = s3.resolve_urls(proof.photos.as_ref()).await;
            let voice_urls = s3.resolve_urls(proof.voice_notes.as_ref()).await;

            let is_believed = Self::is_believed_by_user(db, current_user_id, &proof.ulid).await;

            return Ok(Some(DetailedProof {
                beliefs_count: proof.beliefs_count,
                proof,
                username: user.username,
                avatar_url: user.avatar_url,
                quest_title: quest.title,
                is_believed,
                quest_description: quest.description,
                xp_reward: quest.xp_reward,
                photo_urls,
                voice_urls,
            }));
        }

        Ok(None)
    }

    pub async fn get_feed(
        db: &DatabaseConnection,
        s3: &S3Manager,
        current_user_id: &str,
        limit: u32,
        offset: u32,
    ) -> AppResult<ProofFeedResponse> {
        let proofs_with_relations = QuestProof::find()
            .filter(quest_proofs::Column::UserId.ne(current_user_id))
            .order_by_desc(crate::entities::quest_proofs::Column::CreatedAt)
            .limit((limit + 1) as u64)
            .offset(offset as u64)
            .find_also_related(User)
            .find_also_related(Quest)
            .all(db)
            .await?;

        let has_more = proofs_with_relations.len() > limit as usize;
        let proofs_to_process = if has_more {
            &proofs_with_relations[..limit as usize]
        } else {
            &proofs_with_relations
        };

        let mut results = Vec::with_capacity(proofs_to_process.len());

        for (proof, user_opt, quest_opt) in proofs_to_process {
            if let (Some(user), Some(quest)) = (user_opt, quest_opt) {
                let is_believed = Self::is_believed_by_user(db, current_user_id, &proof.ulid).await;

                let photo_urls = s3.resolve_urls(proof.photos.as_ref()).await;
                let voice_urls = s3.resolve_urls(proof.voice_notes.as_ref()).await;

                results.push(ProofDetailsResponse {
                    ulid: proof.ulid.clone(),
                    user_id: proof.user_id.clone(),
                    username: user.username.clone(),
                    avatar_url: user.avatar_url.clone(),
                    quest_id: proof.quest_id.clone(),
                    quest_title: quest.title.clone(),
                    quest_description: quest.description.clone(),
                    xp_reward: quest.xp_reward,
                    proof_text: proof.proof_text.clone(),
                    status: format!("{:?}", proof.status),
                    beliefs_count: proof.beliefs_count,
                    is_believed,
                    photo_urls,
                    voice_urls,
                    created_at: proof.created_at,
                });
            }
        }

        Ok(ProofFeedResponse {
            items: results,
            has_more,
            next_offset: offset + limit,
        })
    }

    pub async fn toggle_belief(
        db: &DatabaseConnection,
        proof_ulid: String,
        user_id: String,
    ) -> AppResult<bool> {
        let txn = db.begin().await?;

        let (proof, quest_option, user_option) = QuestProof::find_by_id(&proof_ulid)
            .find_also_related(Quest)
            .find_also_related(User)
            .one(&txn)
            .await?
            .ok_or(AppError::NotFound)?;

        if proof.user_id == user_id {
            return Err(AppError::Custom("Cannot believe in yourself".into())); // Depression ;(
        }

        let existing_belief = QuestProofBeliefs::find_by_id((user_id.clone(), proof_ulid.clone()))
            .one(&txn)
            .await?;

        let diff = if let Some(belief) = existing_belief {
            belief.delete(&txn).await?;

            -1
        } else {
            let new_belief = quest_proof_beliefs::ActiveModel {
                user_id: Set(user_id),
                proof_id: Set(proof_ulid.clone()),
                created_at: Set(chrono::Utc::now()),
            };
            new_belief.insert(&txn).await?;

            1
        };

        let updated_proofs = QuestProof::update_many()
            .col_expr(
                quest_proofs::Column::BeliefsCount,
                Expr::col(quest_proofs::Column::BeliefsCount).add(diff),
            )
            .filter(quest_proofs::Column::Ulid.eq(&proof_ulid))
            .exec_with_returning(&txn)
            .await?;

        let updated_proof = updated_proofs
            .into_iter()
            .next()
            .ok_or(AppError::Custom("Failed to update proof count".into()))?;

        // Check for level up and quest complete
        if diff > 0
            && let Some(quest) = quest_option
            && let Some(user) = user_option
        {
            let current_beliefs = updated_proof.beliefs_count;

            let required_beliefs = quest.complexity.required_beliefs(user.level);

            if current_beliefs >= required_beliefs {
                let user_quest = UserQuestStatus::find()
                    .filter(user_quest_status::Column::UserId.eq(&proof.user_id))
                    .filter(user_quest_status::Column::QuestId.eq(&proof.quest_id))
                    .one(&txn)
                    .await?
                    .ok_or(AppError::Custom("Quest status not found".into()))?;

                if user_quest.quest_status != QuestStatus::Completed {
                    let mut active_user_quest: user_quest_status::ActiveModel = user_quest.into();
                    active_user_quest.quest_status = Set(QuestStatus::Completed);
                    active_user_quest.updated_at = Set(chrono::Utc::now());
                    active_user_quest.is_completed = Set(true);
                    active_user_quest.update(&txn).await?;

                    info!(
                        "Quest {} for user {} COMPLETED by community belief!",
                        proof.quest_id, proof.user_id
                    );
                }
            }
        }

        txn.commit().await?;
        Ok(diff > 0)
    }

    pub async fn is_believed_by_user(
        db: &DatabaseConnection,
        user_id: &str,
        proof_ulid: &str,
    ) -> bool {
        QuestProofBeliefs::find_by_id((user_id.to_string(), proof_ulid.to_string()))
            .one(db)
            .await
            .map(|res| res.is_some())
            .unwrap_or(false)
    }

    pub async fn get_user_history(
        db: &DatabaseConnection,
        s3: &S3Manager,
        user_id: &str,
    ) -> Result<Vec<ProofDetailsResponse>, DbErr> {
        let proofs = QuestProof::find()
            .filter(quest_proofs::Column::UserId.eq(user_id))
            .order_by_desc(quest_proofs::Column::CreatedAt)
            .find_also_related(User)
            .find_also_related(Quest)
            .all(db)
            .await?;

        let mut results = Vec::with_capacity(proofs.len());

        for (proof, user_opt, quest_opt) in proofs {
            if let (Some(user), Some(quest)) = (user_opt, quest_opt) {
                let photo_urls = s3.resolve_urls(proof.photos.as_ref()).await;
                let voice_urls = s3.resolve_urls(proof.voice_notes.as_ref()).await;

                results.push(ProofDetailsResponse {
                    ulid: proof.ulid,
                    user_id: proof.user_id,
                    username: user.username,
                    avatar_url: user.avatar_url,
                    quest_id: proof.quest_id,
                    quest_title: quest.title,
                    quest_description: quest.description,
                    xp_reward: quest.xp_reward,
                    proof_text: proof.proof_text,
                    status: format!("{:?}", proof.status),
                    beliefs_count: proof.beliefs_count,
                    is_believed: false,
                    photo_urls,
                    voice_urls,
                    created_at: proof.created_at,
                });
            }
        }
        Ok(results)
    }
}
