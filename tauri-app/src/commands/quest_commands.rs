use reqwest::Method;
use shared::{
    endpoints::{
        quest_proof_endpoints::QuestProofEndpoints,
        user_quest_status_endpoints::UserQuestEndpoints, QuestProofUlid, QuestUlid, UserUlid,
    },
    errors::{AppError, FrontendRepresentation},
    models::{
        quest_proof_dto::{ProofDetailsResponse, SubmitProofRequest, SubmitProofResponse},
        Pagination,
    },
};
use tauri::State;

use log::{error, info};

use crate::auth::service::AppState;

#[tauri::command]
#[specta::specta]
pub async fn get_daily_quests(
    state: State<'_, AppState>,
) -> FrontendRepresentation<Vec<shared::models::user_quest_status_dto::UserQuestStatusResponse>> {
    let service = &state.0;
    let ulid = service.get_current_user_ulid().await?;

    let response: shared::models::user_quest_status_dto::DailyQuestsResponse = service
        .perform_request(
            Method::GET,
            None::<&()>,
            None,
            UserQuestEndpoints::GetDailyQuests(UserUlid(ulid)),
        )
        .await?;

    Ok(response.quests)
}

#[tauri::command]
#[specta::specta]
pub async fn submit_quest_proof(
    state: State<'_, AppState>,
    quest_ulid: String,
    payload: SubmitProofRequest,
    image_list: Option<Vec<Vec<u8>>>,
    audio_list: Option<Vec<Vec<u8>>>,
) -> FrontendRepresentation<SubmitProofResponse> {
    let service = &state.0;
    let ulid = service.get_current_user_ulid().await?;

    let response: SubmitProofResponse = service
        .perform_request(
            Method::POST,
            Some(&payload),
            None,
            QuestProofEndpoints::InitSubmission(UserUlid(ulid.clone()), QuestUlid(quest_ulid)),
        )
        .await?;

    dbg!(&response);

    let client = reqwest::Client::new();

    if let (Some(images), urls) = (image_list, &response.photo_upload_urls) {
        for (data, url) in images.into_iter().zip(urls) {
            let res = client
                .put(url)
                .header("Content-Type", "image/jpeg")
                .body(data)
                .send()
                .await
                .map_err(|e| AppError::Custom(format!("S3 Network Error: {}", e)))?;

            if !res.status().is_success() {
                let err_body = res.text().await.unwrap_or_default();
                error!("S3 Photo Upload Failed: {}", err_body);
                return Err(AppError::Custom("Failed to upload photo to S3".into()).into());
            }
            info!("Photo uploaded to S3 successfully");
        }
    }

    if let (Some(audios), urls) = (audio_list, &response.voice_upload_urls) {
        for (data, url) in audios.into_iter().zip(urls) {
            client
                .put(url)
                .header("Content-Type", "audio/ogg")
                .body(data)
                .send()
                .await
                .map_err(|e| AppError::Custom(format!("S3 Audio Upload Error: {}", e)))?;
        }
    }

    let _: () = service
        .perform_request(
            Method::POST,
            None::<&()>,
            None,
            QuestProofEndpoints::ConfirmSubmission(QuestProofUlid(response.proof_ulid.clone())),
        )
        .await?;

    info!("Proof {} confirmed successfully", response.proof_ulid);

    Ok(response)
}

#[tauri::command]
#[specta::specta]
pub async fn get_proof_details(
    state: State<'_, AppState>,
    proof_ulid: String,
) -> FrontendRepresentation<ProofDetailsResponse> {
    let service = &state.0;

    info!("Fetching details for proof: {}", proof_ulid);

    let response: ProofDetailsResponse = service
        .perform_request(
            Method::GET,
            None::<&()>,
            None,
            QuestProofEndpoints::GetDetails(QuestProofUlid(proof_ulid)),
        )
        .await?;

    Ok(response)
}

#[tauri::command]
#[specta::specta]
pub async fn get_proof_feed(
    state: State<'_, AppState>,
    limit: Option<u32>,
    offset: Option<u32>,
) -> FrontendRepresentation<shared::models::quest_proof_dto::ProofFeedResponse> {
    let service = &state.0;

    let ulid = service.get_current_user_ulid().await?;

    info!(
        "Fetching proof feed for user: {} (limit: {:?}, offset: {:?})",
        ulid, limit, offset
    );

    let pagination = Pagination {
        limit: limit.unwrap_or(10),
        offset: offset.unwrap_or(0),
    };

    let response: shared::models::quest_proof_dto::ProofFeedResponse = service
        .perform_request(
            Method::GET,
            None::<&()>,
            Some(&pagination),
            QuestProofEndpoints::GetFeed(UserUlid(ulid)),
        )
        .await?;

    Ok(response)
}

#[tauri::command]
#[specta::specta]
pub async fn toggle_proof_belief(
    state: State<'_, AppState>,
    proof_ulid: String,
) -> FrontendRepresentation<()> {
    let service = &state.0;

    let user_ulid = {
        let session_lock = service.session.read().await;
        let session = session_lock.as_ref().ok_or_else(|| {
            error!("Toggle belief failed: No active session");
            AppError::NotFound
        })?;
        session.user_ulid.clone()
    };

    info!(
        "User {} toggling belief for proof {}",
        user_ulid, proof_ulid
    );

    service
        .perform_request::<_, ()>(
            Method::POST,
            None::<&()>,
            None,
            QuestProofEndpoints::BeliefProof(QuestProofUlid(proof_ulid), UserUlid(user_ulid)),
        )
        .await?;

    Ok(())
}

async fn fetch_journal_by_id(
    service: &crate::auth::service::AuthService,
    target_user_ulid: String,
) -> FrontendRepresentation<Vec<ProofDetailsResponse>> {
    info!("Executing journal fetch for: {}", target_user_ulid);

    let response: Vec<ProofDetailsResponse> = service
        .perform_request(
            Method::GET,
            None::<&()>,
            None,
            QuestProofEndpoints::GetUserJournal(UserUlid(target_user_ulid)),
        )
        .await?;

    Ok(response)
}

#[tauri::command]
#[specta::specta]
pub async fn get_my_journal(
    state: State<'_, AppState>,
) -> FrontendRepresentation<Vec<ProofDetailsResponse>> {
    let service = &state.0;
    let ulid = service.get_current_user_ulid().await?;

    fetch_journal_by_id(service, ulid).await
}

#[tauri::command]
#[specta::specta]
pub async fn get_someone_journal(
    state: State<'_, AppState>,
    user_ulid: String,
) -> FrontendRepresentation<Vec<ProofDetailsResponse>> {
    fetch_journal_by_id(&state.0, user_ulid).await
}
