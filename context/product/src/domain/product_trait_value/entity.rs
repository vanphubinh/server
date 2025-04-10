//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use async_trait::async_trait;
use sea_orm::{entity::prelude::*, ActiveModelBehavior, ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "product_trait_value")]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: Uuid,
  #[sea_orm(column_type = "Text")]
  pub value: String,
  #[sea_orm(column_type = "Uuid")]
  pub product_trait_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "crate::product_trait::Entity",
    from = "Column::ProductTraitId",
    to = "crate::product_trait::entity::Column::Id"
  )]
  ProductTrait,
}

impl Related<crate::product_trait::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::ProductTrait.def()
  }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
  fn new() -> Self {
    Self {
      id: Set(Uuid::now_v7()),
      ..ActiveModelTrait::default()
    }
  }
}
