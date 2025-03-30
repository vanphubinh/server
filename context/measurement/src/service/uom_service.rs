use anyhow::Result;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder,
};
use utils::{PaginatedResponse, PaginationMeta, PaginationParams};
use uuid::Uuid;

use crate::domain::uom::{ActiveModel, Column, CreateUomInput, Entity as UomEntity, UomDto};
pub struct UomService {
    db: DatabaseConnection,
}

impl UomService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    // Get all UOMs with pagination
    pub async fn find_all(&self, params: PaginationParams) -> Result<PaginatedResponse<UomDto>> {
        let page = params.page.unwrap_or(1);
        let per_page = params.per_page.unwrap_or(10);

        // Create the base query
        let mut query = UomEntity::find();

        // Apply ordering (newest first)
        query = query.order_by_desc(Column::CreatedAt);

        // Get total count
        let total = query.clone().count(&self.db).await?;

        // Apply pagination
        let paginator = query.paginate(&self.db, per_page);
        let models = paginator.fetch_page(page - 1).await?;

        // Convert models to response type
        let data = models.into_iter().map(UomDto::from).collect();

        Ok(PaginatedResponse::new(
            data,
            PaginationMeta::new(total, page, per_page),
        ))
    }

    // Get UOM by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<UomDto>> {
        let uom = UomEntity::find_by_id(id).one(&self.db).await?;
        Ok(uom.map(UomDto::from))
    }

    // Create a new UOM
    pub async fn create(&self, input: CreateUomInput) -> Result<UomDto> {
        // Convert to ActiveModel
        let uom = ActiveModel {
            id: Set(Uuid::now_v7()),
            name: Set(input.name),
            created_at: Set(Utc::now()),
            updated_at: Set(None),
            ..Default::default()
        };

        // Let the ActiveModelBehavior handle the UUID and timestamps
        let model = uom.insert(&self.db).await?;
        Ok(UomDto::from(model))
    }

    // Delete a UOM by ID
    pub async fn delete(&self, id: Uuid) -> Result<bool> {
        let result = UomEntity::delete_by_id(id).exec(&self.db).await?;
        Ok(result.rows_affected > 0)
    }
}
