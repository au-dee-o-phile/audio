use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261107_013703_create_content_part"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("content_part")
                    .primary_key(Index::create().col("part_type").col("content_id"))
                    .if_not_exists()
                    .col(uuid("content_id"))
                    .col(string("part_type"))
                    .col(string("mime_type"))
                    .col(boolean("published").default(false))
                    .col(uuid("part_id"))
                    .take(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from_tbl("content_part")
                    .from_col("content_id")
                    .to_tbl("content")
                    .to_col("id")
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Restrict)
                    .take(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("content_part").take())
            .await?;
        Ok(())
    }
}
