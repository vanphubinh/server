use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "category")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: Uuid,
  pub name: String,
  pub parent_category_id: Option<Uuid>,
  pub created_at: DateTime<Utc>,
  pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "Entity",
    from = "Column::ParentCategoryId",
    to = "Column::Id"
  )]
  ParentCategory,
}

impl ActiveModelBehavior for ActiveModel {}
