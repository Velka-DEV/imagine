use ::entity::{file, file::Entity as File};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn get_file(
        db: &DbConn,
        id: &str,
    )  -> Result<Option<file::Model>, DbErr> {
        File::find_by_id(id).one(db).await
    }
}