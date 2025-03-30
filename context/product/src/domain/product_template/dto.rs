use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};
use uuid::Uuid;

use super::entity::Model;
use crate::product_template::{ProductSubtype, ProductType};

/// Input type for creating a new product template
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductTemplateInput {
    pub name: String,
    pub uom_id: Uuid,
    pub product_subtype: ProductSubtype,
    pub product_type: ProductType,
    pub category_id: Option<Uuid>,
    pub description: Option<String>,
}

/// Response type for a single product template
#[derive(Debug, Serialize, ToSchema, ToResponse)]
#[serde(rename_all = "camelCase")]
pub struct ProductTemplateDto {
    pub id: Uuid,
    pub name: String,
    pub uom_id: Uuid,
    pub product_type: ProductType,
    pub product_subtype: ProductSubtype,
    pub description: String,
    pub category_id: Option<Uuid>,
}

impl From<Model> for ProductTemplateDto {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            uom_id: model.uom_id,
            product_subtype: model.product_subtype,
            description: model.description,
            product_type: model.product_type,
            category_id: model.category_id,
        }
    }
}
