use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use shared::endpoints::{API, UserUlid};
use shared::{
    endpoints::user_quest_status_endpoints::UserQuestEndpoints,
    errors::AppResult,
    models::{quest_dto::QuestDto, user_quest_status_dto::DailyQuestsResponse},
};
use shared::{
    models::user_quest_status_dto::UserQuestStatusResponse, utils::ulid_validation::validate_ulid,
};

use crate::{AppState, service::user_quest_status_service::UserQuestService};

pub fn user_quest_router() -> Router<AppState> {
    Router::new()
        .route(
            UserQuestEndpoints::GetJournal(UserUlid::default()).template(),
            get(get_journal),
        )
        .route(
            UserQuestEndpoints::GetDailyQuests(UserUlid::default()).template(),
            get(get_daily_quests),
        )
}

async fn get_journal(
    Path(user_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<UserQuestStatusResponse>>> {
    validate_ulid(&user_id)?;

    let journal_data = UserQuestService::get_user_journal(&state.connection, &user_id).await?;

    let response: Vec<UserQuestStatusResponse> = journal_data
        .into_iter()
        .map(|(status, quest)| UserQuestStatusResponse {
            user_ulid: status.user_id,
            quest: QuestDto::from(quest),
            status: status.quest_status,
            current_value: status.current_value,
            is_completed: status.is_completed,
            completed_at: if status.is_completed {
                Some(status.updated_at)
            } else {
                None
            },
        })
        .collect();

    Ok(Json(response))
}

async fn get_daily_quests(
    Path(user_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<DailyQuestsResponse>> {
    let data = UserQuestService::get_daily_quests_with_status(&state.connection, &user_id).await?;

    let quests = data
        .into_iter()
        .map(|(status, quest)| UserQuestStatusResponse {
            user_ulid: user_id.clone(),
            quest: QuestDto::from(quest),
            status: status.quest_status,
            current_value: status.current_value,
            is_completed: status.is_completed,
            completed_at: if status.is_completed {
                Some(status.updated_at)
            } else {
                None
            },
        })
        .collect();

    Ok(Json(DailyQuestsResponse {
        date: chrono::Utc::now().date_naive(),
        quests,
    }))
}
