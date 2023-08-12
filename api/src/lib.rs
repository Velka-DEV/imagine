use std::env;
use salvo::prelude::*;
use migration::{Migrator, MigratorTrait};
use std::path::Path;
use service::sea_orm::{Database, DatabaseConnection};

mod router;
mod handlers;

const STORAGE_PATH: &str = "storage";

#[derive(Debug, Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}

#[allow(dead_code)]
#[tokio::main]
pub async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    // get env vars
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");


    let conn = Database::connect(&db_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();
    let state = AppState { conn };

    let storage_path = Path::new(STORAGE_PATH);
    if !storage_path.exists() {
        std::fs::create_dir(&storage_path).unwrap();
    }

    let acceptor = TcpListener::new(server_url).bind().await;
    Server::new(acceptor).serve(router::create_router().hoop(affix::inject(state))).await;
}