use ::entity::file::{Model, Entity};
use sea_orm::*;
use uuid::Uuid;

pub struct Query;

impl Query {
    pub async fn get_file(
        db: &DbConn,
        id: &str,
    )  -> Result<Option<Model>, DbErr> {
        let uuid = Uuid::parse_str(id)
            .map_err(|_| DbErr::Custom(String::from("Invalid UUID")))?;

        Entity::find_by_id(uuid).one(db).await
    }
}