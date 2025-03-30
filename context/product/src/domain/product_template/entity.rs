use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, ToSchema)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "product_type")]
pub enum ProductType {
    #[sea_orm(string_value = "goods")]
    Goods,
    #[sea_orm(string_value = "service")]
    Service,
}

#[derive(Clone, Debug, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, ToSchema)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "product_subtype")]
pub enum ProductSubtype {
    #[sea_orm(string_value = "standard")]
    Standard,
    #[sea_orm(string_value = "to_print_packaging")]
    ToPrintPackaging,
    #[sea_orm(string_value = "printing_mould")]
    PrintingMould,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "product_template")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub product_type: ProductType,
    pub product_subtype: ProductSubtype,
    pub description: String,
    pub uom_id: Uuid,
    pub category_id: Option<Uuid>,
    pub created_at: ChronoDateTimeWithTimeZone,
    #[sea_orm(nullable)]
    pub updated_at: Option<ChronoDateTimeWithTimeZone>,
    #[sea_orm(nullable)]
    pub archived_at: Option<ChronoDateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "crate::domain::product::Entity")]
    Product,
}

impl Related<crate::domain::product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
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
