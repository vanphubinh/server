use anyhow::Result;
use sea_orm::{prelude::Decimal, ActiveModelTrait, ActiveValue::Set, ConnectionTrait};
use uuid::Uuid;

use crate::domain::product::{dto::Product, entity::ActiveModel};

pub struct ProductService;

impl ProductService {
  pub fn new() -> Self {
    Self {}
  }

  pub async fn create<C>(&self, db: &C, input: CreateProductInput) -> Result<Product>
  where
    C: ConnectionTrait,
  {
    let product = ActiveModel {
      product_template_id: Set(input.product_template_id),
      price: Set(input.price),
      cost: Set(input.cost),
      ..Default::default()
    };

    let product = product.insert(db).await?;

    Ok(product.into())
  }
}

pub struct CreateProductInput {
  pub product_template_id: Uuid,
  pub price: Decimal,
  pub cost: Decimal,
}
