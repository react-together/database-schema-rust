use entity::{photo_reactions::*, photos, users};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Entity)
                    .if_not_exists()
                    .col(big_unsigned(Column::PhotoId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Entity, Column::PhotoId)
                            .to(photos::Entity, photos::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(big_unsigned(Column::UserId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Entity, Column::UserId)
                            .to(users::Entity, users::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(Index::create().col(Column::PhotoId).col(Column::UserId))
                    .col(boolean(Column::IsRecommended).not_null())
                    .col(text(Column::Comment).null())
                    .col(date_time(Column::CreatedAt).default(Expr::current_timestamp()))
                    .col(
                        date_time(Column::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .extra("on update CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Entity.into_table_ref()).to_owned())
            .await
    }
}
