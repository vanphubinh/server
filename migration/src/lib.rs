pub use sea_orm_migration::prelude::*;

mod m20250320_132610_create_uom_table;
mod m20250324_125055_create_category_table;
mod m20250329_092600_create_product_template_table;
mod m20250329_093335_create_product_table;
mod m20250402_165028_create_product_trait_table;
mod m20250405_112311_create_product_trait_value_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
      Box::new(m20250320_132610_create_uom_table::Migration),
      Box::new(m20250324_125055_create_category_table::Migration),
      Box::new(m20250329_092600_create_product_template_table::Migration),
      Box::new(m20250329_093335_create_product_table::Migration),
      Box::new(m20250402_165028_create_product_trait_table::Migration),
      Box::new(m20250405_112311_create_product_trait_value_table::Migration),
    ]
  }
}
