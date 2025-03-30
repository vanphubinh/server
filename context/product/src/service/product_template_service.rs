use anyhow::Result;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ConnectionTrait, DatabaseConnection, DbErr,
    TransactionTrait,
};

use crate::domain::product_template::{
    dto::{CreateProductTemplateInput, ProductTemplateDto},
    entity::ActiveModel,
};

pub struct ProductTemplateService;

impl ProductTemplateService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn create<C>(
        &self,
        db: &C,
        input: CreateProductTemplateInput,
    ) -> Result<ProductTemplateDto>
    where
        C: ConnectionTrait + TransactionTrait,
    {
        let product_template = db
            .transaction::<_, ProductTemplateDto, DbErr>(|txn| {
                Box::pin(async move {
                    let name = input.name;
                    let description = input.description;
                    let product_type = input.product_type;
                    let category_id = input.category_id;

                    let product_template = ActiveModel {
                        name: Set(name),
                        description: Set(description.unwrap_or_default()),
                        product_type: Set(product_type),
                        category_id: Set(category_id),
                        ..Default::default()
                    };

                    let product_template = product_template.insert(txn).await?;

                    Ok(product_template.into())
                })
            })
            .await?;

        Ok(product_template)
    }
}
