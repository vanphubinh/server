use anyhow::Result;
use sea_orm::{
  ActiveModelTrait, ActiveValue::Set, ConnectionTrait, EntityTrait, PaginatorTrait, QueryOrder,
};
use serde::Deserialize;
use utils::{PaginatedResponse, PaginationMeta, PaginationParams};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::uom::{dto::Uom, ActiveModel, Column, Entity as UomEntity};

// Define CreateUomInput here
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUomInput {
  pub name: String,
}

// Remove db field
pub struct UomService;

impl UomService {
  // Remove db parameter from new
  pub fn new() -> Self {
    Self {}
  }

  // Add db parameter: db: &C where C: ConnectionTrait
  pub async fn find_all<C>(
    &self,
    db: &C,
    params: PaginationParams,
  ) -> Result<PaginatedResponse<Uom>>
  where
    C: ConnectionTrait,
  {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);

    let mut query = UomEntity::find();
    query = query.order_by_desc(Column::CreatedAt);
    let total = query.clone().count(db).await?; // Use passed db
    let paginator = query.into_partial_model::<Uom>().paginate(db, per_page); // Use passed db
    let uoms = paginator.fetch_page(page - 1).await?;

    Ok(PaginatedResponse::new(
      uoms,
      PaginationMeta::new(total, page, per_page),
    ))
  }

  // Add db parameter: db: &C where C: ConnectionTrait
  pub async fn find_by_id<C>(&self, db: &C, id: Uuid) -> Result<Option<Uom>>
  where
    C: ConnectionTrait,
  {
    let uom = UomEntity::find_by_id(id).one(db).await?; // Use passed db
    Ok(uom.map(Uom::from))
  }

  // Add db parameter: db: &C where C: ConnectionTrait
  pub async fn create<C>(&self, db: &C, input: CreateUomInput) -> Result<Uom>
  where
    C: ConnectionTrait,
  {
    let uom = ActiveModel {
      name: Set(input.name),
      // Assuming Uuid and timestamps are handled by ActiveModelBehavior
      ..Default::default()
    };

    let model = uom.insert(db).await?; // Use passed db
    Ok(Uom::from(model))
  }

  // Add db parameter: db: &C where C: ConnectionTrait
  pub async fn delete<C>(&self, db: &C, id: Uuid) -> Result<bool>
  where
    C: ConnectionTrait,
  {
    let result = UomEntity::delete_by_id(id).exec(db).await?; // Use passed db
    Ok(result.rows_affected > 0)
  }
}
