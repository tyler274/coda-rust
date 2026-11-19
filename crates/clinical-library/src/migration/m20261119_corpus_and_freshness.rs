//! Admitted evidence documents, searchable content units, version snapshot
//! memberships, per-source refresh status, and search/read latency records.
//!
//! Search serves only units mapped in `clinical_library_version_units` for the
//! currently active row of `clinical_library_versions`, while `clinical_library_units`
//! stores each unit's SHA-256 and enforces agreement in Postgres.
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "CREATE TABLE clinical_library_documents (
                   source text NOT NULL
                     CHECK (source IN ('pubmed', 'pmc', 'dailymed', 'mmwr', 'uspstf')),
                   item_key text NOT NULL CHECK (length(item_key) BETWEEN 1 AND 256),
                   source_code text NOT NULL
                     CHECK (source_code IN ('pubmed', 'pmc', 'dailymed', 'cdc-mmwr', 'uspstf')),
                   source_display_name text NOT NULL
                     CHECK (length(btrim(source_display_name)) BETWEEN 1 AND 128),
                   native_document_id text NOT NULL
                     CHECK (length(btrim(native_document_id)) BETWEEN 1 AND 256),
                   native_version_id text,
                   source_version_label text,
                   title text NOT NULL CHECK (length(btrim(title)) BETWEEN 1 AND 1000),
                   document_type text NOT NULL CHECK (length(btrim(document_type)) BETWEEN 1 AND 128),
                   publication_date text,
                   version_date text,
                   acquired_at timestamptz NOT NULL DEFAULT clock_timestamp(),
                   lifecycle_state text NOT NULL CHECK (lifecycle_state IN
                     ('current', 'corrected', 'retracted', 'superseded', 'withdrawn', 'unknown')),
                   source_status_code text,
                   native_destination_uri text NOT NULL
                     CHECK (length(btrim(native_destination_uri)) BETWEEN 1 AND 2048),
                   version_destination_uri text NOT NULL
                     CHECK (length(btrim(version_destination_uri)) BETWEEN 1 AND 2048),
                   rights_decision_id text NOT NULL
                     CHECK (length(btrim(rights_decision_id)) BETWEEN 1 AND 128),
                   license_identifier text,
                   attribution_text text,
                   may_display_excerpt boolean NOT NULL,
                   identifiers_json text NOT NULL DEFAULT '[]',
                   relationships_json text NOT NULL DEFAULT '[]',
                   PRIMARY KEY (source, item_key),
                   FOREIGN KEY (source, item_key) REFERENCES clinical_library_items
                 );

                 CREATE TABLE clinical_library_units (
                   unit_id uuid PRIMARY KEY,
                   source text NOT NULL,
                   item_key text NOT NULL,
                   native_unit_id text NOT NULL
                     CHECK (length(btrim(native_unit_id)) BETWEEN 1 AND 256),
                   native_locator text NOT NULL
                     CHECK (length(btrim(native_locator)) BETWEEN 1 AND 512),
                   unit_kind text NOT NULL CHECK (length(btrim(unit_kind)) BETWEEN 1 AND 64),
                   ordinal integer NOT NULL CHECK (ordinal >= 0),
                   label text,
                   content text NOT NULL CHECK (length(content) > 0),
                   content_sha256 text NOT NULL CHECK (content_sha256 ~ '^[0-9a-f]{64}$'),
                   CHECK (encode(sha256(convert_to(content, 'UTF8')), 'hex') = content_sha256),
                   native_destination_uri text NOT NULL
                     CHECK (length(btrim(native_destination_uri)) BETWEEN 1 AND 2048),
                   search_tsv tsvector GENERATED ALWAYS AS
                     (to_tsvector('english', coalesce(label, '') || ' ' || content)) STORED,
                   UNIQUE (source, item_key, ordinal),
                   FOREIGN KEY (source, item_key)
                     REFERENCES clinical_library_documents ON DELETE CASCADE
                 );

                 CREATE INDEX clinical_library_units_search_tsv_idx
                   ON clinical_library_units USING GIN (search_tsv);

                 CREATE TABLE clinical_library_version_units (
                   version_id text NOT NULL
                     REFERENCES clinical_library_versions(version) ON DELETE CASCADE,
                   unit_id uuid NOT NULL
                     REFERENCES clinical_library_units(unit_id) ON DELETE CASCADE,
                   PRIMARY KEY (version_id, unit_id)
                 );

                 CREATE TABLE clinical_library_source_status (
                   source text PRIMARY KEY
                     CHECK (source IN ('pubmed', 'pmc', 'dailymed', 'mmwr', 'uspstf')),
                   last_checked_at timestamptz,
                   last_succeeded_at timestamptz,
                   last_run_id uuid,
                   last_error_code text CHECK (last_error_code ~ '^[a-z][a-z0-9_]{0,63}$')
                 );

                 CREATE TABLE clinical_library_search_latencies (
                   id bigserial PRIMARY KEY,
                   recorded_at timestamptz NOT NULL DEFAULT clock_timestamp(),
                   endpoint text NOT NULL CHECK (endpoint IN ('search', 'read')),
                   duration_ms double precision NOT NULL CHECK (duration_ms >= 0),
                   candidate_count integer NOT NULL CHECK (candidate_count >= 0)
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
