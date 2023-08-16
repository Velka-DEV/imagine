use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ImageVariant::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ImageVariant::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ImageVariant::Name).string().not_null())
                    .col(ColumnDef::new(ImageVariant::Width).integer().not_null())
                    .col(ColumnDef::new(ImageVariant::Height).integer().not_null())
                    .col(ColumnDef::new(ImageVariant::FitMode).integer().default(0))
                    .col(ColumnDef::new(ImageVariant::StripMode).integer().default(0))
                    .col(ColumnDef::new(ImageVariant::Blur).integer().default(0))
                    .col(ColumnDef::new(ImageVariant::Brighten).integer().default(0))
                    .col(ColumnDef::new(ImageVariant::Contrast).float().default(0.0))
                    .col(ColumnDef::new(ImageVariant::HueRotate).integer().default(0))
                    .col(ColumnDef::new(ImageVariant::HorizontalFlip).boolean().default(false))
                    .col(ColumnDef::new(ImageVariant::VerticalFlip).boolean().default(false))
                    .col(ColumnDef::new(ImageVariant::Invert).boolean().default(false))
                    .col(ColumnDef::new(ImageVariant::Grayscale).boolean().default(false))
                    .col(ColumnDef::new(ImageVariant::Rotate).integer().default(0))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ImageVariant::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ImageVariant {
    Table,
    Id,
    Name,
    Width,
    Height,
    FitMode,
    StripMode,
    Blur,
    Brighten,
    Contrast,
    HueRotate,
    HorizontalFlip,
    VerticalFlip,
    Invert,
    Grayscale,
    Rotate,
}
