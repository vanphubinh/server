use chrono::{DateTime, Utc};
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};
use uuid::Uuid;

use crate::{ProductSubtype, ProductType};

use super::entity::Model;

/// Input type for creating a new product
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductInput {
    pub product_template_id: Uuid,
    pub price: Decimal,
    pub cost: Decimal,
}

/// Response type for a single product
#[derive(Debug, Serialize, ToSchema, ToResponse)]
#[serde(rename_all = "camelCase")]
pub struct ProductDto {
    pub id: Uuid,
    pub product_template_id: Uuid,
    pub price: f64,
    pub cost: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<Model> for ProductDto {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            product_template_id: model.product_template_id,
            price: model.price.to_string().parse::<f64>().unwrap_or(0.0),
            cost: model.cost.to_string().parse::<f64>().unwrap_or(0.0),
            created_at: model.created_at.to_utc(),
            updated_at: model.updated_at.map(|dt| dt.to_utc()),
        }
    }
}
