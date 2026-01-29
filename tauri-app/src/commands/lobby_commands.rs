use reqwest::Method;
use shared::{
    endpoints::{lobby_endpoints::LobbyEndpoints, LobbyUlid, UserUlid},
    errors::FrontendRepresentation,
    models::lobby_dto::{
        CreateLobbyRequest, LobbyDetailsResponse, LobbyDto, LobbyFeedResponse, LobbyMemberDto,
    },
};
use tauri::State;

use log::info;

use crate::auth::service::AppState;

#[tauri::command]
#[specta::specta]
pub async fn create_lobby(
    name: String,
    topic: String,
    description: Option<String>,
    state: State<'_, AppState>,
) -> FrontendRepresentation<LobbyDto> {
    let service = &state.0;

    let ulid = service.get_current_user_ulid().await?;

    let lobby_create_request = CreateLobbyRequest {
        name,
        description,
        topic,
        owner_id: ulid,
    };

    let response: LobbyDto = service
        .perform_request(
            Method::POST,
            Some(&lobby_create_request),
            None,
            LobbyEndpoints::Create,
        )
        .await?;

    Ok(response)
}

#[tauri::command]
#[specta::specta]
pub async fn get_all_lobbies(
    state: State<'_, AppState>,
) -> FrontendRepresentation<LobbyFeedResponse> {
    let service = &state.0;

    let ulid = service.get_current_user_ulid().await?;
    let lobbies: LobbyFeedResponse = service
        .perform_request(
            Method::GET,
            None::<&()>,
            None,
            LobbyEndpoints::GetAll(UserUlid(ulid)),
        )
        .await?;

    Ok(lobbies)
}

#[tauri::command]
#[specta::specta]
pub async fn get_lobby_detail(
    state: State<'_, AppState>,
    lobby_ulid: String,
) -> FrontendRepresentation<LobbyDetailsResponse> {
    let service = &state.0;

    info!("Fetching details for lobby: {}", lobby_ulid);

    let response: LobbyDetailsResponse = service
        .perform_request(
            Method::GET,
            None::<&()>,
            None,
            LobbyEndpoints::GetDetails(LobbyUlid(lobby_ulid)),
        )
        .await?;

    Ok(response)
}

#[tauri::command]
#[specta::specta]
pub async fn join_lobby(
    state: State<'_, AppState>,
    lobby_ulid: String,
) -> FrontendRepresentation<LobbyMemberDto> {
    let service = &state.0;

    let ulid = service.get_current_user_ulid().await?;

    let response: LobbyMemberDto = service
        .perform_request(
            Method::POST,
            None::<&()>,
            None,
            LobbyEndpoints::Join(LobbyUlid(lobby_ulid), UserUlid(ulid)),
        )
        .await?;

    Ok(response)
}

#[tauri::command]
#[specta::specta]
pub async fn get_lobby_memebers_count(
    state: State<'_, AppState>,
    lobby_ulid: String,
) -> FrontendRepresentation<u32> {
    let service = &state.0;

    let response: u32 = service
        .perform_request(
            Method::GET,
            None::<&()>,
            None,
            LobbyEndpoints::GetMembersCount(LobbyUlid(lobby_ulid)),
        )
        .await?;

    Ok(response)
}
