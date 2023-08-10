use salvo::prelude::*;
use salvo::affix;
use crate::{handlers::file, AppState};
use salvo::routing::filters::PathFilter;
use regex::Regex;

pub fn create_router() -> Router {

    let guid = Regex::new("[0-9a-fA-F]{8}-([0-9a-fA-F]{4}-){3}[0-9a-fA-F]{12}").unwrap();
    PathFilter::register_wisp_regex("guid", guid);

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
        .post(file::upload_file)
        .push(
            Router::with_path("<id:guid>")
                .get(file::serve_file)
        )
}

fn create_image_router() -> Router {
    Router::with_path("image")
        .get(file::serve_file)
}

fn create_video_router() -> Router {
    Router::with_path("video")
        .get(file::serve_file)
}