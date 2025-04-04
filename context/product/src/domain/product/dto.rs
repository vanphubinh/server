use serde::Serialize;
use utoipa::{ToResponse, ToSchema};
use uuid::Uuid;

use super::entity::Model;

/// Response type for a single product
#[derive(Debug, Serialize, ToSchema, ToResponse)]
#[serde(rename_all = "camelCase")]
pub struct ProductDto {
    pub id: Uuid,
    pub product_template_id: Uuid,
    pub price: f64,
    pub cost: f64,
}

impl From<Model> for ProductDto {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            product_template_id: model.product_template_id,
            price: model.price.to_string().parse::<f64>().unwrap_or(0.0),
            cost: model.cost.to_string().parse::<f64>().unwrap_or(0.0),
        }
    }
}
