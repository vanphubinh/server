use anyhow::Result;
use sea_orm::{
  prelude::*,
  sea_query::{Alias, Query},
  ActiveModelTrait,
  ActiveValue::Set,
  ConnectionTrait, FromQueryResult, Order,
};
use utils::{PaginatedResponse, PaginationMeta, PaginationParams};
use uuid::Uuid;

use crate::{
  domain::product::{
    dto::ProductQueryResult,
    entity::{ActiveModel, Column, Entity as ProductEntity, Model as ProductModel},
  },
  product::Product,
};

pub struct ProductService;

impl ProductService {
  pub fn new() -> Self {
    Self {}
  }

  pub async fn create<C>(&self, db: &C, input: CreateProductInput) -> Result<ProductModel>
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

    Ok(product)
  }

  pub async fn list<C>(
    &self,
    db: &C,
    params: PaginationParams,
  ) -> Result<PaginatedResponse<Product>>
  where
    C: ConnectionTrait,
  {
    let page = params.page.unwrap_or(1) - 1; // Convert to 0-based index
    let per_page = params.per_page.unwrap_or(10);

    // Build the query
    let product_query = Query::select()
      .column((ProductEntity, Column::Id))
      .column((ProductEntity, Column::ProductTemplateId))
      .column((ProductEntity, Column::Price))
      .column((
        crate::domain::product_template::Entity,
        crate::domain::product_template::Column::Name,
      ))
      .column((
        crate::domain::product_template::Entity,
        crate::domain::product_template::Column::UomId,
      ))
      .expr_as(
        Expr::col((measurement::uom::Entity, measurement::uom::Column::Name)),
        Alias::new("uom_name"),
      )
      .column((
        crate::domain::product_template::Entity,
        crate::domain::product_template::Column::CategoryId,
      ))
      .expr_as(
        Expr::col((
          crate::domain::category::Entity,
          crate::domain::category::Column::Name,
        )),
        Alias::new("category_name"),
      )
      .expr_as(
        Expr::col((
          crate::domain::product_template::Entity,
          crate::domain::product_template::Column::ProductType,
        ))
        .cast_as(Alias::new("TEXT")),
        Alias::new("product_type"),
      )
      .expr_as(
        Expr::col((
          crate::domain::product_template::Entity,
          crate::domain::product_template::Column::ProductSubtype,
        ))
        .cast_as(Alias::new("TEXT")),
        Alias::new("product_subtype"),
      )
      .from(ProductEntity)
      .left_join(
        crate::domain::product_template::Entity,
        Expr::col((ProductEntity, Column::ProductTemplateId)).equals((
          crate::domain::product_template::Entity,
          crate::domain::product_template::Column::Id,
        )),
      )
      .left_join(
        crate::domain::category::Entity,
        Expr::col((
          crate::domain::product_template::Entity,
          crate::domain::product_template::Column::CategoryId,
        ))
        .equals((
          crate::domain::category::Entity,
          crate::domain::category::Column::Id,
        )),
      )
      .left_join(
        measurement::uom::Entity,
        Expr::col((
          crate::domain::product_template::Entity,
          crate::domain::product_template::Column::UomId,
        ))
        .equals((measurement::uom::Entity, measurement::uom::Column::Id)),
      )
      .order_by((ProductEntity, Column::CreatedAt), Order::Desc)
      .offset(page * per_page)
      .limit(per_page)
      .to_owned();

    let builder = db.get_database_backend();
    let statement = builder.build(&product_query);
    let products = ProductQueryResult::find_by_statement(statement)
      .all(db)
      .await?
      .into_iter()
      .map(Product::from)
      .collect::<Vec<Product>>();

    let total = products.len() as u64;

    Ok(PaginatedResponse::new(
      products,
      PaginationMeta::new(total, page + 1, per_page),
    ))
  }
}

pub struct CreateProductInput {
  pub product_template_id: Uuid,
  pub price: Decimal,
  pub cost: Decimal,
}
