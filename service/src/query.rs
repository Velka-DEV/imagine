use ::entity::file::{Model, Entity};
use sea_orm::*;
use uuid::Uuid;

pub struct Query;

impl Query {

    pub async fn list_files(
        db: &DbConn,
    ) -> Result<Vec<Model>, DbErr> {
        Entity::find().all(db).await
    }

    pub async fn get_file(
        db: &DbConn,
        id: Uuid,
    )  -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(db).await
    }
}