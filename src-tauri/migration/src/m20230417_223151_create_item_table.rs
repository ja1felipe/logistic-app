use sea_orm_migration::prelude::*;

use crate::m20230417_223140_create_category_table::Category;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Item::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Item::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Item::Code).string().not_null().unique_key())
                    .col(ColumnDef::new(Item::Description).string().not_null())
                    .col(ColumnDef::new(Item::TotalQtd).integer().not_null())
                    .col(
                        ColumnDef::new(Item::Category)
                            .string()
                            .default(Value::String(None)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_category_code_item")
                            .from(Item::Table, Item::Category)
                            .to(Category::Table, Category::Code)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Item::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Item {
    Table,
    Id,
    Code,
    Description,
    TotalQtd,
    Category,
}
