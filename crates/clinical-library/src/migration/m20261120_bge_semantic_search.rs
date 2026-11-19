//! Adds pgvector and the 384-dimensional BGE semantic embedding column + HNSW index
//! to `clinical_library_units`.

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "DO $$
                 DECLARE
                     vec_schema text;
                 BEGIN
                     PERFORM pg_advisory_xact_lock(hashtext('clinical_library_install_pgvector'));

                     SELECT n.nspname INTO vec_schema
                     FROM pg_extension e
                     JOIN pg_namespace n ON n.oid = e.extnamespace
                     WHERE e.extname = 'vector';

                     IF vec_schema IS NULL THEN
                         CREATE EXTENSION IF NOT EXISTS vector WITH SCHEMA public;
                         SELECT n.nspname INTO vec_schema
                         FROM pg_extension e
                         JOIN pg_namespace n ON n.oid = e.extnamespace
                         WHERE e.extname = 'vector';
                     END IF;

                     EXECUTE format(
                         'ALTER TABLE clinical_library_units ADD COLUMN embedding %I.vector(384)',
                         vec_schema
                     );
                     EXECUTE format(
                         'CREATE INDEX clinical_library_units_embedding_hnsw_idx ON clinical_library_units USING hnsw (embedding %I.vector_cosine_ops)',
                         vec_schema
                     );
                 END
                 $$;",
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
