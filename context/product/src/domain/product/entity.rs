use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "product")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub product_template_id: Uuid,
    pub price: Decimal,
    pub cost: Decimal,
    pub created_at: ChronoDateTimeWithTimeZone,
    #[sea_orm(nullable)]
    pub updated_at: Option<ChronoDateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::domain::product_template::Entity",
        from = "Column::ProductTemplateId",
        to = "crate::domain::product_template::Column::Id"
    )]
    ProductTemplate,
}

impl Related<crate::domain::product_template::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProductTemplate.def()
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

    async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let _ = db;
        let mut this = self;
        if !insert {
            this.updated_at = Set(Some(Utc::now().into()));
        }
        Ok(this)
    }
}
