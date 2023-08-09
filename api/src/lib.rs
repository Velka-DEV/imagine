use salvo::prelude::*;
use std::path::Path;

mod router;
mod handlers;

const STORAGE_PATH: &str = "storage";

struct AppState {
    conn: DatabaseConnection,
}

#[allow(dead_code)]
#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt().init();

    let storage_path = Path::new(STORAGE_PATH);
    if !storage_path.exists() {
        std::fs::create_dir(&storage_path).unwrap();
    }

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router::create_router()).await;
}