use salvo::prelude::*;
use salvo::routing::filter::PathFilter;
use regex::Regex;
use std::path::Path;

mod router;
mod handlers;

const STORAGE_PATH: &str = "storage";

#[handler]
async fn hello() -> &'static str {
    "Hello World"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let guid = Regex::new("[0-9a-fA-F]{8}-([0-9a-fA-F]{4}-){3}[0-9a-fA-F]{12}").unwrap();
    PathFilter::register_wisp_regex("guid", guid);

    let storage_path = Path::new(STORAGE_PATH);
    if !storage_path.exists() {
        std::fs::create_dir(storage_path).unwrap();
    }

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router::create_router()).await;
}