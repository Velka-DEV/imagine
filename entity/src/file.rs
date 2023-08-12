use salvo::prelude::Extractible;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Extractible, Deserialize, Serialize)]
#[sea_orm(table_name = "files")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub file_name: String,
    pub extension: String,
    pub size: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    // Automatic uuid generation
    // fn new() -> Self {
    //     Self {
    //         id: Set(Uuid::new_v4()),
    //         ..ActiveModelTrait::default()
    //     }
    // }
}