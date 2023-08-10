use ::entity::file;
use sea_orm::*;
use uuid::Uuid;

pub struct Mutation;

impl Mutation {
    pub async fn create_file(
        db: &DbConn,
        form_data: file::Model,
    ) -> Result<file::ActiveModel, DbErr> {
        file::ActiveModel {
            id: Set(Uuid::new_v4().to_string().to_owned()),
            file_name: Set(form_data.file_name.to_owned()),
            extension: Set(form_data.extension.to_owned()),
            size: Set(form_data.size.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }
}