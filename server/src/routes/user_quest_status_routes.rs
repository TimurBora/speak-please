use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use shared::endpoints::API;
use shared::{
    endpoints::user_quest_status_endpoints::UserQuestEndpoints,
    errors::AppResult,
    models::{
        quest_dto::QuestDto,
        user_quest_status_dto::{DailyQuestsResponse, QuestStatus},
    },
};
use shared::{
    models::user_quest_status_dto::UserQuestStatusResponse, utils::ulid_validation::validate_ulid,
};

use crate::{AppState, service::user_quest_status_service::UserQuestService};

pub fn user_quest_router() -> Router<AppState> {
    Router::new()
        .route(
            UserQuestEndpoints::GetJournal("0".to_string()).template(),
            get(get_journal),
        )
        .route(
            UserQuestEndpoints::GetDailyQuests("0".to_string()).template(),
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
        .filter_map(|(status, quest_opt)| {
            quest_opt.map(|quest| UserQuestStatusResponse {
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
        })
        .collect();

    Ok(Json(response))
}

async fn get_daily_quests(
    Path(user_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<DailyQuestsResponse>> {
    validate_ulid(&user_id)?;

    let quest_models = UserQuestService::get_or_assign_quests(&state.connection, &user_id).await?;

    let quests_dto: Vec<UserQuestStatusResponse> = quest_models
        .into_iter()
        .map(|q| UserQuestStatusResponse {
            user_ulid: user_id.clone(),
            quest: QuestDto::from(q),
            status: QuestStatus::InProgress,
            current_value: 0,
            is_completed: false,
            completed_at: None,
        })
        .collect();

    Ok(Json(DailyQuestsResponse {
        date: chrono::Utc::now().date_naive(),
        quests: quests_dto,
    }))
}
