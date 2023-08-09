use std::path::PathBuf;

use salvo::prelude::*;
use salvo::core::fs::NamedFile;
use uuid::Uuid;
use serde::Serialize;

use crate::STORAGE_PATH;

const FILE_STORAGE_PATH: &str = "files";

#[derive(Serialize, Debug)]
struct File {
    id: String,
    size: u64,
}

#[handler]
pub async fn serve_file(req: &mut Request, res: &mut Response) {
    let id = req.params().get("id").cloned().unwrap_or_default();

    if id.is_empty() {
        res.render(StatusError::not_found().brief("File not found."));
        return;
    }

    let path = get_file_path(&id);

    if !path.exists() {
        res.status_code(StatusCode::NOT_FOUND);
        return;
    }

    let filename = path.file_name().unwrap_or_default().to_str().unwrap_or_default();

    NamedFile::builder(&path).attached_name(filename).send(req.headers(), res).await;
}

#[handler]
pub async fn upload_file(req: &mut Request, res: &mut Response) {
    let file = req.file("file").await;

    if file.is_none() {
        res.render(StatusError::bad_request().brief("No file uploaded."));
        return;
    }

    let file = file.unwrap();

    let file_extension = file.path().extension().unwrap_or_default();

    let path = get_file_storage_path();
    if !path.exists() {
        std::fs::create_dir(&path).unwrap();
    }

    let file_id = Uuid::new_v4();
    let mut path = path;
    path.push(file_id.to_string());
    path.set_extension(file_extension);

    let file_info = std::fs::copy(&file.path(), path.to_str().unwrap());

    if file_info.is_err() {
        res.render(StatusError::internal_server_error().brief("Failed to store file."));
        return;
    }

    let file = File {
        id: file_id.to_string(),
        size: file.size(),
    };

    res.render(Json(file));
}

fn get_file_path(id: &str) -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    path
        .push(STORAGE_PATH);
    path
        .push(FILE_STORAGE_PATH);
    path
        .push(id);

    path
}

fn get_file_storage_path() -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    path
        .push(STORAGE_PATH);
    path
        .push(FILE_STORAGE_PATH);

    path
}