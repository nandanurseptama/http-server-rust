use sea_orm_migration::{prelude::*, schema::*};

use crate::m20240823_131140_create_table_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Posts::Table)
                    .if_not_exists()
                    .col(pk_auto(Posts::Id))
                    .col(string(Posts::Title))
                    .col(string(Posts::Text))
                    .col(integer(Posts::AuthorId))
                    .col(
                        timestamp(Posts::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(timestamp_null(Posts::UpdatedAt))
                    .col(timestamp_null(Posts::DeletedAt))
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("fk_posts_author_id")
                            .from(Posts::Table, Posts::AuthorId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Posts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Posts {
    Table,
    Id,
    AuthorId,
    Title,
    Text,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
