use measurement::uom::dto::Uom;
use sea_orm::{prelude::Decimal, FromQueryResult};
use serde::Serialize;
use utoipa::{ToResponse, ToSchema};
use uuid::Uuid;

use crate::{
  category::Category,
  product_template::{ProductSubtype, ProductType},
};

/// Response type for a single product
#[derive(Debug, Serialize, ToSchema, ToResponse)]
#[serde(rename_all = "camelCase")]
pub struct Product {
  pub id: Uuid,
  pub name: String,
  #[schema(rename = "productTemplateId")]
  pub product_template_id: Uuid,
  pub category: Option<Category>,
  pub uom: Uom,
  #[schema(rename = "productType")]
  pub product_type: ProductType,
  #[schema(rename = "productSubtype")]
  pub product_subtype: ProductSubtype,
  #[schema(rename = "price")]
  pub price: Decimal,
}

#[derive(Debug, FromQueryResult)]
pub struct ProductQueryResult {
  pub id: Uuid,
  pub name: String,
  pub product_template_id: Uuid,
  pub category_id: Option<Uuid>,
  pub category_name: Option<String>,
  pub uom_id: Uuid,
  pub uom_name: String,
  pub product_type: ProductType,
  pub product_subtype: ProductSubtype,
  pub price: Decimal,
}

impl From<ProductQueryResult> for Product {
  fn from(result: ProductQueryResult) -> Self {
    Self {
      id: result.id,
      name: result.name,
      product_template_id: result.product_template_id,
      category: result.category_id.map(|id| Category {
        id,
        name: result.category_name.unwrap_or_default(),
        parent_category_id: None,
      }),
      uom: Uom {
        id: result.uom_id,
        name: result.uom_name,
      },
      product_type: result.product_type,
      product_subtype: result.product_subtype,
      price: result.price,
    }
  }
}
