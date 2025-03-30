use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{prelude::Decimal, TransactionTrait};
use serde::Deserialize;
use utils::{AppError, CreateResponse, SharedState};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    product::ProductDto,
    product_template::ProductSubtype,
    product_template::ProductType,
    service::{
        product_service::{CreateProductInput, ProductService},
        product_template_service::{CreateProductTemplateInput, ProductTemplateService},
    },
};

/// Create product request containing all requred fields
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductRequest {
    /// Product name
    pub name: String,
    /// Product type
    #[serde(rename = "productType")]
    pub product_type: ProductType,
    /// Product subtype
    #[serde(rename = "productSubtype")]
    pub product_subtype: ProductSubtype,
    /// Unit of measure ID
    #[serde(rename = "uomId")]
    pub uom_id: Uuid,
    /// Product price
    pub price: Decimal,
    /// Product cost
    pub cost: Decimal,
    /// Optional category ID
    #[serde(rename = "categoryId")]
    pub category_id: Option<Uuid>,
    /// Optional description
    pub description: Option<String>,
}

/// Create product response
pub type CreateProductResponse = CreateResponse;

/// Create a new product with its template
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/products/create_product",
    request_body = CreateProductRequest,
    responses(
        (status = 201, response = CreateProductResponse),
        (status = 400),
        (status = 500),
    ),
    tag = "Product",
    summary = "Create a new product with template",
    description = "Create a new product template and product in a single transaction"
)]
pub async fn create_product(
    State(state): State<SharedState>,
    Json(req): Json<CreateProductRequest>,
) -> Result<(StatusCode, Json<CreateProductResponse>), AppError> {
    println!("Creating product: {:?}", req);
    // Use the transaction helper to manage commit/rollback
    let product = state
        .db
        .transaction::<_, ProductDto, AppError>(|txn| {
            Box::pin(async move {
                // Instantiate services without the transaction connection `txn`
                let product_service = ProductService::new();
                let template_service = ProductTemplateService::new();

                // Prepare template input
                let template_input = CreateProductTemplateInput {
                    name: req.name, // req is moved into the closure
                    product_type: req.product_type,
                    product_subtype: req.product_subtype,
                    uom_id: req.uom_id,
                    category_id: req.category_id,
                    description: req.description,
                };

                // Pass txn to the service method
                let template = template_service
                    .create(txn, template_input) // Pass txn as the first argument
                    .await?;

                // Prepare product input
                let product_input = CreateProductInput {
                    product_template_id: template.id,
                    price: req.price, // req is still available here
                    cost: req.cost,
                };

                // Pass txn to the service method
                let product = product_service
                    .create(txn, product_input) // Pass txn as the first argument
                    .await?;

                // Return the created product from the transaction closure
                Ok(product)
            })
        })
        .await?; // Await the result and use ? for automatic error conversion

    // If the transaction was successful, return the created response
    Ok((
        StatusCode::CREATED,
        Json(CreateProductResponse { id: product.id }),
    ))
}
