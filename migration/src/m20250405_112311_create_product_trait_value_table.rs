use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(ProductTraitValue::Table)
          .if_not_exists()
          .col(uuid(ProductTraitValue::Id).primary_key())
          .col(text(ProductTraitValue::Value).default(""))
          .col(uuid(ProductTraitValue::ProductTraitId))
          .foreign_key(
            ForeignKey::create()
              .name("fk-product_trait_value-product_trait_id")
              .from(ProductTraitValue::Table, ProductTraitValue::ProductTraitId)
              .to(ProductTrait::Table, ProductTrait::Id),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(ProductTraitValue::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum ProductTraitValue {
  Table,
  Id,
  Value,
  ProductTraitId,
}

#[derive(DeriveIden)]
enum ProductTrait {
  Table,
  Id,
}
