use salvo::prelude::Extractible;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Extractible, Deserialize, Serialize)]
#[sea_orm(table_name = "files")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub file_name: String,
    pub extension: String,
    pub size: u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            ..ActiveModelTrait::default()
        }
    }

}