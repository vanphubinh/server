use anyhow::Result;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ConnectionTrait, TransactionTrait};

use crate::domain::product::{
    dto::{CreateProductInput, ProductDto},
    entity::ActiveModel,
};

pub struct ProductService;

impl ProductService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn create<C>(&self, db: &C, input: CreateProductInput) -> Result<ProductDto>
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
