use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_auto(Users::Id))
                    .col(string(Users::Email).unique_key())
                    .col(string(Users::Password).not_null())
                    .col(
                        timestamp(Users::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(timestamp_null(Users::UpdatedAt))
                    .col(timestamp_null(Users::DeletedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Email,
    Password,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
