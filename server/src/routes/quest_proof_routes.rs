use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use shared::{endpoints::API, errors::AppError};
use shared::{endpoints::QuestProofUlid, utils::ulid_validation::validate_ulid};
use shared::{
    endpoints::quest_proof_endpoints::QuestProofEndpoints, models::quest_proof_dto::PaginationQuery,
};
use shared::{
    endpoints::{QuestUlid, UserUlid},
    models::quest_proof_dto::{ProofDetailsResponse, SubmitProofRequest, SubmitProofResponse},
};
use shared::{errors::AppResult, models::quest_proof_dto::ProofFeedResponse};

use crate::{AppState, service::quest_proof_service::QuestProofService};
use validator::Validate;

pub fn quest_proof_router() -> Router<AppState> {
    Router::new()
        .route(
            QuestProofEndpoints::InitSubmission(UserUlid::default(), QuestUlid::default())
                .template(),
            post(init_proof),
        )
        .route(
            QuestProofEndpoints::ConfirmSubmission(QuestProofUlid::default()).template(),
            post(confirm_proof),
        )
        .route(
            QuestProofEndpoints::GetDetails(QuestProofUlid::default()).template(),
            get(get_proof_details),
        )
        .route(
            QuestProofEndpoints::GetFeed(UserUlid::default()).template(),
            get(get_proof_feed),
        )
        .route(
            QuestProofEndpoints::BeliefProof(QuestProofUlid::default(), UserUlid::default())
                .template(),
            post(toggle_proof_belief),
        )
        .route(
            QuestProofEndpoints::GetUserJournal(UserUlid::default()).template(),
            get(get_user_proof_history),
        )
}

async fn init_proof(
    State(state): State<AppState>,
    Path((user_id, quest_id)): Path<(String, String)>,
    axum::Json(payload): axum::Json<SubmitProofRequest>,
) -> AppResult<axum::Json<SubmitProofResponse>> {
    validate_ulid(&user_id)?;
    validate_ulid(&quest_id)?;
    payload.validate().map_err(AppError::Validation)?;

    let (model, photo_urls, voice_urls) = QuestProofService::init_proof_submition(
        &state.connection,
        &state.s3_manager,
        user_id,
        quest_id,
        payload.proof_text,
        payload.photo_count,
        payload.voice_count,
    )
    .await?;

    Ok(axum::Json(SubmitProofResponse {
        proof_ulid: model.ulid,
        status: format!("{:?}", model.status),
        photo_upload_urls: photo_urls,
        voice_upload_urls: voice_urls,
    }))
}

async fn confirm_proof(
    State(state): State<AppState>,
    Path(proof_id): Path<String>,
) -> AppResult<StatusCode> {
    validate_ulid(&proof_id)?;

    QuestProofService::confirm_proof_upload(&state.connection, proof_id)
        .await
        .map_err(AppError::from)?;

    Ok(StatusCode::OK)
}

async fn get_proof_details(
    State(state): State<AppState>,
    Path(proof_id): Path<String>,
    current_user_id: String,
) -> AppResult<axum::Json<ProofDetailsResponse>> {
    validate_ulid(&proof_id)?;

    let detail = QuestProofService::get_proof_full_details(
        &state.connection,
        &state.s3_manager,
        &proof_id,
        &current_user_id,
    )
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(axum::Json(detail.into()))
}

async fn get_proof_feed(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    axum::extract::Query(pagination): axum::extract::Query<PaginationQuery>,
) -> AppResult<axum::Json<ProofFeedResponse>> {
    validate_ulid(&user_id)?;
    pagination.validate().map_err(AppError::Validation)?;

    let limit = pagination.limit.unwrap_or(20);
    let offset = pagination.offset.unwrap_or(0);

    let results = QuestProofService::get_feed(
        &state.connection,
        &state.s3_manager,
        &user_id,
        limit as u32,
        offset as u32,
    )
    .await?;

    Ok(axum::Json(results))
}

async fn toggle_proof_belief(
    State(state): State<AppState>,
    Path((proof_id, user_id)): Path<(String, String)>,
) -> AppResult<StatusCode> {
    validate_ulid(&proof_id)?;
    validate_ulid(&user_id)?;

    QuestProofService::toggle_belief(&state.connection, proof_id, user_id).await?;

    Ok(StatusCode::OK)
}

async fn get_user_proof_history(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> AppResult<axum::Json<Vec<ProofDetailsResponse>>> {
    validate_ulid(&user_id)?;

    let history =
        QuestProofService::get_user_history(&state.connection, &state.s3_manager, &user_id).await?;

    Ok(axum::Json(history))
}
