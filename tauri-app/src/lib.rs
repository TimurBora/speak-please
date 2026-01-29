use tauri::Manager;
use tauri_plugin_store::StoreExt;
use tauri_specta::{collect_commands, Builder};

use log::{debug, info};

use crate::auth::service::AuthService;
use crate::auth::{service::AppState, session::UserSession};

pub mod auth;
pub mod commands;

use commands::auth_commands::{
    check_access_token, delete_refresh_token, get_current_session, login, logout, register,
};

use commands::quest_commands::{
    get_daily_quests, get_my_journal, get_proof_details, get_proof_feed, get_someone_journal,
    submit_quest_proof, toggle_proof_belief,
};

use commands::lobby_commands::{
    create_lobby, get_all_lobbies, get_lobby_detail, get_lobby_memebers_count, join_lobby,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let specta_builder = Builder::<tauri::Wry>::new().commands(collect_commands![
        register,
        login,
        logout,
        get_current_session,
        check_access_token,
        delete_refresh_token,
        get_daily_quests,
        submit_quest_proof,
        get_proof_feed,
        get_proof_details,
        toggle_proof_belief,
        get_someone_journal,
        get_my_journal,
        create_lobby,
        get_all_lobbies,
        get_lobby_detail,
        get_lobby_memebers_count,
        join_lobby
    ]);

    #[cfg(all(debug_assertions, not(mobile)))]
    specta_builder
        .export(
            specta_typescript::Typescript::default(),
            "../src/bindings.ts",
        )
        .expect("Failed to export bindings");

    info!("Starting Speak Please application...");

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Warn)
                .level_for("speak_please_lib", log::LevelFilter::Info)
                .level_for("tauri", log::LevelFilter::Info)
                .level_for("zbus", log::LevelFilter::Off)
                .level_for("reqwest", log::LevelFilter::Warn)
                .build(),
        )
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            #[cfg(target_os = "android")]
            {
                debug!("Configuring Android keyring...");
                let _ = android_keyring::set_android_keyring_credential_builder();
            }

            let initial_session = app
                .store("user.json")
                .ok()
                .and_then(|s| s.get("user_conf"))
                .map(|val| {
                    debug!("Loaded user configuration from store for: {}", val["email"]);
                    UserSession {
                        access_token: None,
                        user_ulid: val["user_ulid"].as_str().unwrap_or_default().to_string(),
                        email: val["email"].as_str().unwrap_or_default().to_string(),
                        username: val["username"].as_str().unwrap_or_default().to_string(),
                        level: val["level"].as_u64().unwrap_or(0) as u32,
                        avatar_url: val["avatar_url"].as_str().map(|s| s.to_string()),
                    }
                });

            app.manage(AppState(AuthService::new(initial_session)));
            Ok(())
        })
        .invoke_handler(specta_builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
