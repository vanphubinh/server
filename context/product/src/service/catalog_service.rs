use anyhow::Result;
use chrono::Utc;
use sea_orm::{
  ActiveModelTrait, ActiveValue::Set, ConnectionTrait, EntityTrait, PaginatorTrait, QueryOrder,
};
use serde::Deserialize;
use utils::{PaginatedResponse, PaginationMeta, PaginationParams};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
  domain::category::dto::Category,
  domain::category::{ActiveModel, Column, Entity as CategoryEntity},
};

// Define CreateCategoryInput here
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateCategoryInput {
  /// Category name
  pub name: String,
  /// Optional parent category ID
  pub parent_category_id: Option<Uuid>,
}

// Remove db field
pub struct CatalogService;

impl CatalogService {
  // Remove db parameter from new
  pub fn new() -> Self {
    Self {}
  }

  // Add db parameter: db: &C where C: ConnectionTrait
  pub async fn find_all<C>(
    &self,
    db: &C,
    params: PaginationParams,
  ) -> Result<PaginatedResponse<Category>>
  where
    C: ConnectionTrait,
  {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);

    let mut query = CategoryEntity::find();
    query = query.order_by_desc(Column::CreatedAt);
    let total = query.clone().count(db).await?; // Use passed db
    let paginator = query
      .into_partial_model::<Category>()
      .paginate(db, per_page); // Use passed db
    let categories = paginator.fetch_page(page - 1).await?;

    Ok(PaginatedResponse::new(
      categories,
      PaginationMeta::new(total, page, per_page),
    ))
  }

  // Add db parameter: db: &C where C: ConnectionTrait
  pub async fn find_by_id<C>(&self, db: &C, id: Uuid) -> Result<Option<Category>>
  where
    C: ConnectionTrait,
  {
    let category = CategoryEntity::find_by_id(id).one(db).await?; // Use passed db
    Ok(category.map(Category::from))
  }

  // Add db parameter: db: &C where C: ConnectionTrait
  pub async fn create<C>(&self, db: &C, input: CreateCategoryInput) -> Result<Category>
  where
    C: ConnectionTrait,
  {
    let category = ActiveModel {
      name: Set(input.name),
      parent_category_id: Set(input.parent_category_id),
      created_at: Set(Utc::now()),
      updated_at: Set(None),
      ..Default::default()
    };

    let model = category.insert(db).await?; // Use passed db
    Ok(Category::from(model))
  }

  // Add db parameter: db: &C where C: ConnectionTrait
  pub async fn delete<C>(&self, db: &C, id: Uuid) -> Result<bool>
  where
    C: ConnectionTrait,
  {
    let result = CategoryEntity::delete_by_id(id).exec(db).await?; // Use passed db
    Ok(result.rows_affected > 0)
  }
}
