use salvo::prelude::*;
use crate::handlers::file;

pub fn create_router() -> Router {
    Router::new()
    .push(
        create_file_router()
    ).push(
        create_image_router()
    )
    .push(
        create_video_router()
    )
}

fn create_file_router() -> Router {
    Router::with_path("file")
        .get(file::serve_file)
}

fn create_image_router() -> Router {
    Router::with_path("image")
        .get(file::serve_file)
}

fn create_video_router() -> Router {
    Router::with_path("video")
        .get(file::serve_file)
}