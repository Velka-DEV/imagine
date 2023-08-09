use salvo::prelude::Extractible;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Extractible, Deserialize, Serialize)]
#[sea_orm(table_name = "files")]
pub struct File {
    pub id: String,
    pub filename: String,
    pub size: u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}