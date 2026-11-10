//! PubMed's status for each article, and the deferral of a night that set
//! everything due aside.
//!
//! An article's status is NLM's words: its publication types and its
//! comments-and-corrections links, in NLM's order, with the record they came
//! from exactly as NLM sent it and its hash, which the database checks
//! agree. `observed_at` is when the library first held this
//! record and `confirmed_at` when NLM last sent it unchanged.
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
                   DROP CONSTRAINT clinical_library_nights_deferral_reason_check,
                   ADD CONSTRAINT clinical_library_nights_deferral_reason_check
                     CHECK (deferral_reason IN (
                       'slot_held_by_live_run', 'owner_unreadable', 'waiting_for_capacity',
                       'published_with_set_aside', 'moved_nothing'));

                 CREATE TABLE clinical_library_pubmed_status (
                   pmid bigint PRIMARY KEY CHECK (pmid > 0),
                   publication_types text[] NOT NULL
                     CHECK (array_position(publication_types, NULL) IS NULL),
                   revised date,
                   record_sha256 bytea NOT NULL CHECK (length(record_sha256) = 32),
                   record_xml text NOT NULL CHECK (length(record_xml) > 0),
                   CHECK (sha256(convert_to(record_xml, 'UTF8')) = record_sha256),
                   observed_at timestamptz NOT NULL DEFAULT clock_timestamp(),
                   confirmed_at timestamptz NOT NULL DEFAULT clock_timestamp(),
                   CHECK (confirmed_at >= observed_at)
                 );

                 CREATE TABLE clinical_library_pubmed_relations (
                   pmid bigint NOT NULL
                     REFERENCES clinical_library_pubmed_status ON DELETE CASCADE,
                   position smallint NOT NULL CHECK (position >= 0),
                   ref_type text NOT NULL CHECK (length(ref_type) BETWEEN 1 AND 64),
                   target_pmid bigint CHECK (target_pmid > 0),
                   source_reference text,
                   note text,
                   PRIMARY KEY (pmid, position)
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
