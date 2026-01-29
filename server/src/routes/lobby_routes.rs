use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use shared::{
    endpoints::{API, LobbyUlid, UserUlid, lobby_endpoints::LobbyEndpoints},
    errors::{AppError, AppResult},
    models::lobby_dto::{
        CreateLobbyRequest, LobbyDetailsResponse, LobbyDto, LobbyFeedItem, LobbyFeedResponse,
        LobbyMemberDto,
    },
    utils::ulid_validation::validate_ulid,
};
use validator::Validate;

use crate::AppState;
use crate::service::lobby_member_service::LobbyMemberService;
use crate::service::lobby_service::LobbyService;

pub fn lobby_router() -> Router<AppState> {
    Router::new()
        .route(LobbyEndpoints::Create.template(), post(create_lobby))
        .route(
            LobbyEndpoints::GetAll(UserUlid::default()).template(),
            get(get_lobby_feed),
        )
        .route(
            LobbyEndpoints::GetDetails(LobbyUlid::default()).template(),
            get(get_lobby_details),
        )
        .route(
            LobbyEndpoints::Join(LobbyUlid::default(), UserUlid::default()).template(),
            post(join_lobby),
        )
        .route(
            LobbyEndpoints::GetMembersCount(LobbyUlid::default()).template(),
            get(get_lobby_members_count),
        )
}

async fn create_lobby(
    State(state): State<AppState>,
    Json(payload): Json<CreateLobbyRequest>,
) -> AppResult<Json<LobbyDto>> {
    payload.validate().map_err(AppError::Validation)?;
    validate_ulid(&payload.owner_id)?;

    let lobby_model = LobbyService::create_lobby(
        &state.connection,
        payload.owner_id,
        payload.name,
        payload.topic,
        payload.description,
    )
    .await?;

    Ok(Json(LobbyDto::from(lobby_model)))
}

#[expect(dead_code)]
async fn get_all_lobbies(State(state): State<AppState>) -> AppResult<Json<Vec<LobbyDto>>> {
    let lobbies = LobbyService::get_all_lobbies(&state.connection).await?;

    let dtos = lobbies.into_iter().map(LobbyDto::from).collect();
    Ok(Json(dtos))
}

async fn get_lobby_feed(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> AppResult<Json<LobbyFeedResponse>> {
    let data =
        LobbyService::get_lobbies_with_membership_status(&state.connection, &user_id).await?;

    let lobby_items = data
        .into_iter()
        .map(|(model, is_member)| LobbyFeedItem {
            lobby: LobbyDto::from(model),
            is_member,
        })
        .collect();

    Ok(Json(LobbyFeedResponse { items: lobby_items }))
}

async fn get_lobby_details(
    State(state): State<AppState>,
    Path(lobby_id): Path<String>,
) -> AppResult<Json<LobbyDetailsResponse>> {
    validate_ulid(&lobby_id)?;

    let lobby = LobbyService::find_by_id(&state.connection, &lobby_id).await?;
    let count = LobbyMemberService::get_lobby_members_count(&state.connection, &lobby_id).await?;

    Ok(Json(LobbyDetailsResponse {
        lobby: LobbyDto::from(lobby),
        members_count: count as u32,
    }))
}

async fn join_lobby(
    State(state): State<AppState>,
    Path((lobby_id, user_id)): Path<(String, String)>,
) -> AppResult<Json<LobbyMemberDto>> {
    validate_ulid(&lobby_id)?;
    validate_ulid(&user_id)?;

    let membership = LobbyMemberService::join_lobby(&state.connection, lobby_id, user_id).await?;

    Ok(Json(LobbyMemberDto::from(membership)))
}

async fn get_lobby_members_count(
    State(state): State<AppState>,
    Path(lobby_id): Path<String>,
) -> AppResult<Json<u32>> {
    validate_ulid(&lobby_id)?;

    let count = LobbyMemberService::get_lobby_members_count(&state.connection, &lobby_id).await?;

    Ok(Json(count as u32))
}
