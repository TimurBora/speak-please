use aws_config::Region;
use axum::Router;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use shared::models::message_dto::LobbyEvent;
use std::{collections::HashMap, env, net::SocketAddr, sync::Arc};
use tokio::{
    net::TcpListener,
    sync::{Mutex, broadcast},
};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    entities::quests::seed_quests,
    file_storage::s3_client::S3Manager,
    routes::{
        lobby_routes::lobby_router, quest_proof_routes::quest_proof_router,
        refresh_token_routes::refresh_token_router, user_quest_status_routes::user_quest_router,
        user_routes::public_user_router,
    },
};

pub mod entities;
pub mod file_storage;
pub mod middleware;
pub mod routes;
pub mod service;

#[derive(Clone)]
pub struct AppState {
    pub connection: DatabaseConnection,
    pub s3_manager: S3Manager,
    pub lobby_channels: Arc<Mutex<HashMap<String, broadcast::Sender<LobbyEvent>>>>, // TODO:
                                                                                    // Implement chat
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    dotenv::dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "server=debug,tower_http=debug,axum::rejection=trace,sea_orm=debug".into()
            }),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .pretty()
                .with_target(false)
                .with_file(false)
                .with_line_number(false),
        )
        .init();

    // This isn't the best practice for DB creation
    let db_url = "sqlite://../database.db?mode=rwc";
    let mut opt = ConnectOptions::new(db_url);
    opt.sqlx_logging(false);
    let connection = Database::connect(opt).await.expect("Cannot connect to DB");

    Migrator::up(&connection, None)
        .await
        .expect("Migration failed");

    // Creating default quests for testing and other stuff
    seed_quests(&connection).await.unwrap();

    let region = Region::new(env::var("AWS_REGION").unwrap());
    let endpoint_url = env::var("AWS_ENDPOINT").unwrap();
    let s3_manager =
        S3Manager::new("speak-please".to_string(), endpoint_url, region.to_string()).await;

    let lobby_channels = Arc::new(Mutex::new(HashMap::new()));
    let state = AppState {
        connection,
        s3_manager,
        lobby_channels,
    };

    // Public routes don't needs access keys
    let public_routes = Router::new()
        .merge(refresh_token_router())
        .merge(public_user_router());

    // Private/Protected routes do needs access keys
    let protected_routes: Router<AppState> = Router::new()
        .merge(user_quest_router())
        .merge(quest_proof_router())
        .merge(lobby_router())
        .layer(axum::middleware::from_fn(
            middleware::jwt_verify_middleware::check_access_token, // Middleware for access key
                                                                   // checking
        ));

    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    DefaultMakeSpan::new()
                        .level(Level::INFO)
                        .include_headers(true),
                )
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(tower_http::LatencyUnit::Millis),
                ),
        )
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Server is up: http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
