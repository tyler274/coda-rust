//! The practice's Spruce channel: connection, conversations, message versions
//! and saved files, created from their entities.
use sea_orm::Schema;
use sea_orm_migration::prelude::*;

use super::super::entities::{
    channel_connection, channel_conversation, channel_file, channel_message,
};

/// Creates the channel tables.
#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = Schema::new(manager.get_database_backend());
        manager
            .create_table(schema.create_table_from_entity(channel_connection::Entity))
            .await?;
        manager
            .create_table(schema.create_table_from_entity(channel_conversation::Entity))
            .await?;
        manager
            .create_table(schema.create_table_from_entity(channel_message::Entity))
            .await?;
        manager
            .create_table(schema.create_table_from_entity(channel_file::Entity))
            .await
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Err(DbErr::Custom(
            "the physician runtime's records can't be rolled back".into(),
        ))
    }
}
