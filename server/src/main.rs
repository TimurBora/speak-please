use aws_config::Region;
use axum::Router;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    entities::quests::seed_quests,
    routes::{
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
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    dotenv::dotenv().ok();

    let region = Region::new(env::var("AWS_REGION").unwrap());
    let endpoint_url = env::var("AWS_ENDPOINT").unwrap();
    let sdk_config = aws_config::from_env().region(region).load().await;
    let s3_config = aws_sdk_s3::config::Builder::from(&sdk_config)
        .endpoint_url(endpoint_url)
        .force_path_style(true)
        .build();
    let client = aws_sdk_s3::Client::from_conf(s3_config);

    // List buckets and print their names
    let resp = client.list_buckets().send().await.unwrap();
    for bucket in resp.buckets() {
        println!("Bucket: {}", bucket.name().unwrap_or_default());
    }

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
        ) // Делает вывод многострочным и понятным
        .init();

    let db_url = "sqlite://../database.db?mode=rwc";
    let mut opt = ConnectOptions::new(db_url);
    opt.sqlx_logging(false);
    let connection = Database::connect(opt)
        .await
        .expect("Не удалось подключиться к БД");

    Migrator::up(&connection, None)
        .await
        .expect("Migration failed");

    seed_quests(&connection).await.unwrap();
    let state = AppState { connection };

    let public_routes = Router::new()
        .merge(refresh_token_router())
        .merge(public_user_router())
        .merge(user_quest_router());

    //   let protected_routes = Router::new().layer(axum::middleware::from_fn(
    //middleware::jwt_verify_middleware::check_access_token,
    //));

    let app = Router::new()
        .merge(public_routes)
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
        //        .merge(protected_routes)
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server is up: http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
