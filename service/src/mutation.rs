use ::entity::file::{Model, ActiveModel};
use sea_orm::ActiveValue::Set;
use sea_orm::*;
use uuid::Uuid;

pub struct Mutation;

impl Mutation {
    pub async fn create_file(
        db: &DbConn,
        form_data: Model,
    ) -> Result<ActiveModel, DbErr> {
        ActiveModel {
            file_name: Set(form_data.file_name.to_owned()),
            extension: Set(form_data.extension.to_owned()),
            size: Set(form_data.size.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }
}