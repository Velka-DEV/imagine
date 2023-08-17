use ::entity::file::{Model as FileModel, Entity as FileEntity};
use ::entity::image::{Model as ImageModel, Entity as ImageEntity};
use ::entity::image_variant::{Model as ImageVariantModel, Entity as ImageVariantEntity};
use sea_orm::*;
use uuid::Uuid;

pub struct Query;

impl Query {

    pub async fn list_files(
        db: &DbConn,
    ) -> Result<Vec<FileModel>, DbErr> {
        FileEntity::find().all(db).await
    }

    pub async fn get_file(
        db: &DbConn,
        id: Uuid,
    )  -> Result<Option<FileModel>, DbErr> {
        FileEntity::find_by_id(id).one(db).await
    }

    pub async fn list_images(
        db: &DbConn,
    ) -> Result<Vec<ImageModel>, DbErr> {
        ImageEntity::find().all(db).await
    }

    pub async fn get_image(
        db: &DbConn,
        id: Uuid,
    )  -> Result<Option<ImageModel>, DbErr> {
        ImageEntity::find_by_id(id).one(db).await
    }

    pub async fn list_image_variants(
        db: &DbConn,
    ) -> Result<Vec<ImageVariantModel>, DbErr> {
        ImageVariantEntity::find().all(db).await
    }

    pub async fn get_image_variant(
        db: &DbConn,
        name: &str,
    )  -> Result<Option<ImageVariantModel>, DbErr> {
        ImageVariantEntity::find().filter(::entity::image_variant::Column::Name.eq(name)).one(db).await
    }
}