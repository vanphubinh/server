use anyhow::Result;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder,
};
use utils::{PaginatedResponse, PaginationMeta, PaginationParams};
use uuid::Uuid;

use crate::domain::category::{
    ActiveModel, CategoryDto, Column, CreateCategoryInput, Entity as CategoryEntity,
};

pub struct CatalogService {
    db: DatabaseConnection,
}

impl CatalogService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    // Get all categories with pagination
    pub async fn find_all(
        &self,
        params: PaginationParams,
    ) -> Result<PaginatedResponse<CategoryDto>> {
        let page = params.page.unwrap_or(1);
        let per_page = params.per_page.unwrap_or(10);

        // Create the base query
        let mut query = CategoryEntity::find();

        // Apply ordering (newest first)
        query = query.order_by_desc(Column::CreatedAt);

        // Get total count
        let total = query.clone().count(&self.db).await?;

        // Apply pagination
        let paginator = query.paginate(&self.db, per_page);
        let models = paginator.fetch_page(page - 1).await?;

        // Convert models to response type
        let data = models.into_iter().map(CategoryDto::from).collect();

        Ok(PaginatedResponse::new(
            data,
            PaginationMeta::new(total, page, per_page),
        ))
    }

    // Get category by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<CategoryDto>> {
        let category = CategoryEntity::find_by_id(id).one(&self.db).await?;
        Ok(category.map(CategoryDto::from))
    }

    // Create a new category
    pub async fn create(&self, input: CreateCategoryInput) -> Result<CategoryDto> {
        // Convert to ActiveModel
        let category = ActiveModel {
            name: Set(input.name),
            parent_category_id: Set(input.parent_category_id),
            created_at: Set(Utc::now()),
            updated_at: Set(None),
            ..Default::default()
        };

        // Let the ActiveModelBehavior handle the UUID and timestamps
        let model = category.insert(&self.db).await?;
        Ok(CategoryDto::from(model))
    }

    // Delete a category by ID
    pub async fn delete(&self, id: Uuid) -> Result<bool> {
        let result = CategoryEntity::delete_by_id(id).exec(&self.db).await?;
        Ok(result.rows_affected > 0)
    }
}
