//! The Chords runtime store's migration chain.

use sea_orm_migration::prelude::*;

mod m20261111_init;
mod m20261111_response_feedback;
mod m20261112_chat_memory;
mod m20261112_concerns;
mod m20261112_conversation_activity;
mod m20261112_patient_reviews;
mod m20261112_patient_understanding;
mod m20261112_practice_jobs_dispatch;
mod m20261112_practice_schedule;
mod m20261112_source_refresh;
mod m20261112_today_updates_seen;
mod m20261112_visit_briefings;
mod m20261113_01_spruce_matching;
mod m20261113_02_reply_proposals;
mod m20261113_03_follow_ups_withdrawal;
mod m20261114_02_message_followup_dispatch;

/// The table this chain records applied migrations in. The runtime shares the
/// physician database with other chains, and each chain owns its own ledger.
pub(super) const MIGRATION_TABLE: &str = "physician_runtime_migrations";

/// Every migration, oldest first.
pub(super) struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migration_table_name() -> DynIden {
        Alias::new(MIGRATION_TABLE).into_iden()
    }

    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20261111_init::Migration),
            Box::new(m20261111_response_feedback::Migration),
            Box::new(m20261112_concerns::Migration),
            Box::new(m20261112_practice_jobs_dispatch::Migration),
            Box::new(m20261112_source_refresh::Migration),
            Box::new(m20261112_practice_schedule::Migration),
            Box::new(m20261112_chat_memory::Migration),
            Box::new(m20261112_patient_understanding::Migration),
            Box::new(m20261112_patient_reviews::Migration),
            Box::new(m20261112_visit_briefings::Migration),
            Box::new(m20261112_today_updates_seen::Migration),
            Box::new(m20261112_conversation_activity::Migration),
            Box::new(m20261113_01_spruce_matching::Migration),
            Box::new(m20261113_02_reply_proposals::Migration),
            Box::new(m20261113_03_follow_ups_withdrawal::Migration),
            Box::new(m20261114_02_message_followup_dispatch::Migration),
        ]
    }
}
