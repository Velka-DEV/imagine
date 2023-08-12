use salvo::prelude::*;
use uuid::Uuid;

use ::entity::file::Model as File;

use ::service::storage::{store_file, get_named_file};
use ::service::query::Query;
use ::service::mutation::Mutation;

use crate::AppState;

#[handler]
pub async fn serve_file(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<(), StatusError> {
    let id = req
    .params()
    .get("id")
    .cloned()
    .ok_or_else(StatusError::bad_request)?;

    let state = depot
        .obtain::<AppState>()
        .ok_or_else(StatusError::internal_server_error)?;

    let conn = &state.conn;


    let file = Query::get_file(conn, &id)
        .await
        .map_err(|_| StatusError::internal_server_error())?
        .ok_or_else(StatusError::not_found)?;

    let named_file = get_named_file(&file)
        .await
        .ok_or_else(StatusError::not_found)?;

    named_file.send(req.headers(), res).await;
    Ok(())
}

#[handler]
pub async fn upload_file(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<(), StatusError> {
    let state = depot
        .obtain::<AppState>()
        .ok_or_else(StatusError::internal_server_error)?;

    let conn = &state.conn;

    let file = req.file("file")
        .await
        .ok_or_else(StatusError::bad_request)?;

    let file_id = Uuid::new_v4();
    let file_path = store_file(file, &file_id.to_string())
        .map_err(|err| StatusError::internal_server_error().brief(err.to_string()))?;

    let file = File {
        id: Uuid::new_v4(),
        size: file.size(),
        extension: file_path.extension().unwrap_or_default().to_str().unwrap_or_default().to_string(),
        file_name: file_path.file_name().unwrap_or_default().to_str().unwrap_or_default().to_string(),
    };

    Mutation::create_file(conn, file.clone())
        .await
        .map_err(|err| StatusError::internal_server_error().brief(err.to_string()))?;

    res.render(Json(file));
    Ok(())
}

