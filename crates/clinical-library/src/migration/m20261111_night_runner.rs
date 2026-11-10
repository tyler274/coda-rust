//! What the night runner records: why a stopped night didn't run, a failed
//! night's words, and each night's ledger of the items it was due to do.
//!
//! - A night the stop rule kept from running is deferred as `stopped`, with
//!   the rule that stopped it and, for the same failure twice, that
//!   failure's code.
//! - A failed night keeps the head of its failure's words, so the line that
//!   says what failed survives.
//! - `clinical_library_night_items` lists, for each run, the items due when
//!   it took the slot or that it newly qualified, and what it did with each.
//!   The store counts a night's outcome from these rows.
//! - `clinical_library_unpublished` lists admitted content that only a new
//!   version publishes and no version has published yet. It outlives the
//!   night that admitted it, so a night that crashed before activating
//!   leaves the next one the work of publishing it.
//!
//! The schema is SQL text for the same reason as the first migration: it
//! takes no values, so nothing is spliced in.
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE clinical_library_nights
                   ADD COLUMN stop_reason text
                     CHECK (stop_reason IN ('same_failure_twice', 'failed_nights_in_a_row')),
                   ADD COLUMN failure_detail text
                     CHECK (length(failure_detail) BETWEEN 1 AND 2000),
                   DROP CONSTRAINT clinical_library_nights_deferral_reason_check,
                   ADD CONSTRAINT clinical_library_nights_deferral_reason_check
                     CHECK (deferral_reason IN (
                       'slot_held_by_live_run', 'owner_unreadable', 'waiting_for_capacity',
                       'published_with_set_aside', 'moved_nothing', 'stopped')),
                   DROP CONSTRAINT clinical_library_nights_check,
                   ADD CONSTRAINT clinical_library_nights_outcome_fields CHECK (CASE outcome
                     WHEN 'succeeded' THEN deferral_reason IS NULL AND failure_code IS NULL
                       AND stop_reason IS NULL AND failure_detail IS NULL
                     WHEN 'deferred' THEN deferral_reason IS NOT NULL
                       AND activated_version IS NULL AND failure_detail IS NULL
                       AND (deferral_reason = 'stopped') = (stop_reason IS NOT NULL)
                       AND (stop_reason IS NOT DISTINCT FROM 'same_failure_twice')
                         = (failure_code IS NOT NULL)
                     WHEN 'failed' THEN failure_code IS NOT NULL
                       AND deferral_reason IS NULL AND activated_version IS NULL
                       AND stop_reason IS NULL
                   END);

                 CREATE TABLE clinical_library_night_items (
                   run_id uuid NOT NULL,
                   source text NOT NULL
                     CHECK (source IN ('pubmed', 'pmc', 'dailymed', 'mmwr', 'uspstf')),
                   item_key text NOT NULL,
                   result text NOT NULL CHECK (result IN ('due', 'done', 'set_aside')),
                   PRIMARY KEY (run_id, source, item_key),
                   FOREIGN KEY (source, item_key) REFERENCES clinical_library_items
                 );

                 CREATE TABLE clinical_library_unpublished (
                   source text NOT NULL CHECK (source IN ('pmc', 'dailymed', 'mmwr', 'uspstf')),
                   item_key text NOT NULL,
                   PRIMARY KEY (source, item_key),
                   FOREIGN KEY (source, item_key) REFERENCES clinical_library_items
                 );",
            )
            .await?;
        Ok(())
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Err(DbErr::Custom(
            "the clinical library's history is append-only and can't be rolled back".into(),
        ))
    }
}
