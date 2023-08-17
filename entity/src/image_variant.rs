use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "image_variant")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub width: i16,
    pub height: i16,
    pub fit_mode: i8,
    pub strip_mode: i8,
    pub blur: i32,
    pub brighten: i32,
    pub contrast: f32,
    pub hue_rotate: i32,
    pub horizontal_flip: bool,
    pub vertical_flip: bool,
    pub invert: bool,
    pub grayscale: bool,
    pub rotate: i8,
}

// Not using enum maping for now.
// #[derive(Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Clone)]
// #[sea_orm(rs_type = "i8", db_type = "Integer")]
pub enum Rotation {
    RotateNone = 0,
    Rotate90 = 1,
    Rotate180 = 2,
    Rotate270 = 3,
}

// Not using enum maping for now.
// #[derive(EnumIter, DeriveActiveEnum, Clone, Debug)]
// #[sea_orm(rs_type = "i8", db_type = "Integer")]
pub enum StripMode {
    StripAllExceptCopyright = 0,
    StripAll = 1,
    StripNone = 2,
}

// Not using enum maping for now.
// #[derive(EnumIter, DeriveActiveEnum, Clone, Debug)]
// #[sea_orm(rs_type = "i8", db_type = "Integer")]
pub enum FitMode {
    ScaleDown = 0,
    Contain = 1,
    Cover = 2,
    Crop = 3,
    Pad = 4,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}