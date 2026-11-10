//! The library's first schema: the nightly's slots, each source item's state,
//! the nightly history, people's resolutions, and the index versions.
//!
//! The checks repeat the Rust types' rules, so a row the types couldn't have
//! produced is refused by the database too.
//!
//! The schema is SQL text because SeaQuery's schema builder can't express
//! all of it: the plpgsql append-only triggers, the CASE checks tying each
//! state to its columns, and the partial unique index on a constant that
//! allows one active version. It takes no values, so nothing is spliced in. The waiting-to-retry bound of 2
//! failures is `article::RETRY_LIMIT - 1`; a test in `tests/postgres.rs`
//! holds the two together.
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "CREATE TABLE clinical_library_slots (
                   slot text PRIMARY KEY CHECK (slot ~ '^[a-z0-9_-]{1,64}$'),
                   holder uuid,
                   token bigint NOT NULL DEFAULT 0 CHECK (token >= 0)
                 );

                 CREATE TABLE clinical_library_items (
                   source text NOT NULL
                     CHECK (source IN ('pubmed', 'pmc', 'dailymed', 'mmwr', 'uspstf')),
                   item_key text NOT NULL CHECK (length(item_key) BETWEEN 1 AND 256),
                   state text NOT NULL CHECK (state IN
                     ('admitted', 'waiting_to_retry', 'waiting_for_change', 'needs_person')),
                   failures smallint NOT NULL DEFAULT 0,
                   wait_reason text CHECK (wait_reason IN
                     ('no_readable_text', 'own_defect', 'superseded_reader')),
                   last_failure text CHECK (last_failure ~ '^[a-z][a-z0-9_]{0,63}$'),
                   PRIMARY KEY (source, item_key),
                   CHECK (CASE state
                     WHEN 'admitted' THEN
                       failures = 0 AND wait_reason IS NULL AND last_failure IS NULL
                     WHEN 'waiting_to_retry' THEN
                       failures BETWEEN 0 AND 2 AND wait_reason IS NULL
                       AND (failures = 0) = (last_failure IS NULL)
                     WHEN 'waiting_for_change' THEN
                       failures = 0 AND wait_reason IS NOT NULL AND last_failure IS NULL
                     WHEN 'needs_person' THEN
                       failures = 0 AND wait_reason IS NULL AND last_failure IS NOT NULL
                   END)
                 );

                 -- Nights and resolutions share one sequence, so the history
                 -- reads in the order things happened.
                 CREATE SEQUENCE clinical_library_history;

                 CREATE TABLE clinical_library_nights (
                   id bigint PRIMARY KEY DEFAULT nextval('clinical_library_history'),
                   slot text NOT NULL CHECK (slot ~ '^[a-z0-9_-]{1,64}$'),
                   run_id uuid NOT NULL,
                   outcome text NOT NULL CHECK (outcome IN ('succeeded', 'deferred', 'failed')),
                   activated_version text,
                   deferral_reason text CHECK (deferral_reason IN (
                     'slot_held_by_live_run', 'owner_unreadable',
                     'waiting_for_capacity', 'published_with_set_aside')),
                   failure_code text CHECK (failure_code ~ '^[a-z][a-z0-9_]{0,63}$'),
                   recorded_at timestamptz NOT NULL DEFAULT clock_timestamp(),
                   -- Each run records exactly one outcome.
                   UNIQUE (slot, run_id),
                   CHECK (CASE outcome
                     WHEN 'succeeded' THEN deferral_reason IS NULL AND failure_code IS NULL
                     WHEN 'deferred' THEN deferral_reason IS NOT NULL
                       AND failure_code IS NULL AND activated_version IS NULL
                     WHEN 'failed' THEN failure_code IS NOT NULL
                       AND deferral_reason IS NULL AND activated_version IS NULL
                   END)
                 );

                 CREATE TABLE clinical_library_resolutions (
                   id bigint PRIMARY KEY DEFAULT nextval('clinical_library_history'),
                   slot text NOT NULL CHECK (slot ~ '^[a-z0-9_-]{1,64}$'),
                   recorded_by text NOT NULL CHECK (length(btrim(recorded_by)) BETWEEN 1 AND 200),
                   reason text NOT NULL CHECK (length(btrim(reason)) BETWEEN 1 AND 1000),
                   recorded_at timestamptz NOT NULL DEFAULT clock_timestamp()
                 );

                 CREATE FUNCTION clinical_library_append_only() RETURNS trigger
                   LANGUAGE plpgsql AS $$
                 BEGIN RAISE EXCEPTION 'clinical library history is append-only'; END $$;
                 CREATE TRIGGER clinical_library_nights_append_only
                   BEFORE UPDATE OR DELETE ON clinical_library_nights
                   FOR EACH ROW EXECUTE FUNCTION clinical_library_append_only();
                 CREATE TRIGGER clinical_library_nights_no_truncate
                   BEFORE TRUNCATE ON clinical_library_nights
                   FOR EACH STATEMENT EXECUTE FUNCTION clinical_library_append_only();
                 CREATE TRIGGER clinical_library_resolutions_append_only
                   BEFORE UPDATE OR DELETE ON clinical_library_resolutions
                   FOR EACH ROW EXECUTE FUNCTION clinical_library_append_only();
                 CREATE TRIGGER clinical_library_resolutions_no_truncate
                   BEFORE TRUNCATE ON clinical_library_resolutions
                   FOR EACH STATEMENT EXECUTE FUNCTION clinical_library_append_only();

                 CREATE TABLE clinical_library_versions (
                   version text PRIMARY KEY CHECK (version ~ '^[!-~]{1,128}$'),
                   checked boolean NOT NULL,
                   active boolean NOT NULL DEFAULT false,
                   activated_by_run uuid,
                   -- Only a checked version can be active.
                   CHECK (NOT active OR (checked AND activated_by_run IS NOT NULL))
                 );
                 -- At most one version is active.
                 CREATE UNIQUE INDEX clinical_library_one_active_version
                   ON clinical_library_versions ((true)) WHERE active;",
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
