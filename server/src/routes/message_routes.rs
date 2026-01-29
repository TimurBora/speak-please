use axum::{
    extract::{
        Path, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::broadcast;

use crate::AppState;

pub async fn lobby_chat_ws(
    Path(lobby_id): Path<String>,
    State(state): State<AppState>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_lobby_socket(socket, lobby_id, state))
}

async fn handle_lobby_socket(socket: WebSocket, lobby_id: String, state: AppState) {
    let (mut sender, mut receiver) = socket.split();

    let mut rx = {
        let mut channels = state.lobby_channels.lock().await;
        let tx = channels.entry(lobby_id.clone()).or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        });
        tx.subscribe()
    };

    let mut send_task = tokio::spawn(async move {
        while let Ok(event) = rx.recv().await {
            let msg = serde_json::to_string(&event).unwrap();
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    let lobby_id_clone = lobby_id.clone();
    let state_clone = state.clone();
    let mut recv_task =
        tokio::spawn(
            async move { while let Some(Ok(Message::Text(text))) = receiver.next().await {} },
        );

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    clean_up_channels(state, lobby_id).await;
}

async fn clean_up_channels(state: AppState, lobby_id: String) {
    let mut channels = state.lobby_channels.lock().await;
    if let Some(tx) = channels.get(&lobby_id)
        && tx.receiver_count() == 0
    {
        channels.remove(&lobby_id);
    }
}
