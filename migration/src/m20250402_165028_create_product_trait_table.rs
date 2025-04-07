use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(ProductTrait::Table)
          .if_not_exists()
          .col(uuid(ProductTrait::Id).primary_key())
          .col(text(ProductTrait::Name).default(""))
          .col(timestamp_with_time_zone(ProductTrait::CreatedAt).default(Expr::current_timestamp()))
          .col(timestamp_with_time_zone_null(ProductTrait::UpdatedAt))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(ProductTrait::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum ProductTrait {
  Table,
  Id,
  Name,
  CreatedAt,
  UpdatedAt,
}
