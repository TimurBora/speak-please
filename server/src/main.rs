use axum::Router;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::routes::{refresh_token_routes::refresh_token_router, user_routes::public_user_router};

pub mod middleware;
pub mod routes;
pub mod service;

#[derive(Clone)]
pub struct AppState {
    pub connection: DatabaseConnection,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    dotenv::dotenv().unwrap();

    let db_url = "sqlite://../database.db?mode=rwc";
    let connection = Database::connect(db_url)
        .await
        .expect("Не удалось подключиться к БД");
    Migrator::up(&connection, None)
        .await
        .expect("Migration failed");

    let state = AppState { connection };

    let public_routes = Router::new()
        .merge(refresh_token_router())
        .merge(public_user_router());

    //   let protected_routes = Router::new().layer(axum::middleware::from_fn(
    //middleware::jwt_verify_middleware::check_access_token,
    //));

    let app = Router::new()
        .merge(public_routes)
        //        .merge(protected_routes)
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Сервер запущен на http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
