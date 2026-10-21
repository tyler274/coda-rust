//! Additive Duet attempt/event ownership; no shared runtime tables.
use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(
            "ALTER TABLE duet_conversation_inputs DROP CONSTRAINT duet_conversation_inputs_state_check;
             ALTER TABLE duet_conversation_inputs ADD CHECK (state IN ('queued','running','paused','interrupted','failed','completed'));
             ALTER TABLE duet_conversation_messages DROP CONSTRAINT duet_conversation_messages_state_check;
             ALTER TABLE duet_conversation_messages ADD CHECK (state IN ('complete','pending','streaming','interrupted','failed'));
             ALTER TABLE duet_conversation_inputs ADD COLUMN origin_project text,
               ADD COLUMN origin_subject text, ADD COLUMN origin_authenticated_at bigint,
               ADD COLUMN attempt_count integer NOT NULL DEFAULT 0 CHECK (attempt_count BETWEEN 0 AND 3),
               ADD COLUMN claim_token uuid, ADD COLUMN lease_until timestamptz,
               ADD COLUMN retry_deadline timestamptz, ADD COLUMN failure text;
             CREATE TABLE duet_conversation_attempts (
               id uuid PRIMARY KEY, run_id uuid NOT NULL REFERENCES duet_conversation_inputs(run_id) ON DELETE CASCADE,
               ordinal integer NOT NULL CHECK (ordinal BETWEEN 1 AND 3),
               execution_started boolean NOT NULL DEFAULT false,
               state text NOT NULL CHECK (state IN ('running','expired','interrupted','failed','completed')),
               started_at timestamptz NOT NULL DEFAULT clock_timestamp(),
               UNIQUE(run_id,ordinal)
             );
             CREATE TABLE duet_conversation_events (
               attempt_id uuid NOT NULL REFERENCES duet_conversation_attempts(id) ON DELETE CASCADE,
               sequence bigint NOT NULL CHECK (sequence BETWEEN 1 AND 4096),
               body text NOT NULL CHECK (octet_length(body)<=262144),
               delta text, candidate text,
               PRIMARY KEY(attempt_id,sequence),
               CHECK (delta IS NULL OR candidate IS NULL)
             );
             CREATE INDEX duet_conversation_recovery ON duet_conversation_inputs(account_id,created_at,run_id) WHERE state IN ('queued','running');
             CREATE OR REPLACE FUNCTION duet_interrupt_accepted(target uuid) RETURNS void LANGUAGE plpgsql AS $$
             DECLARE rev bigint;
             BEGIN
               IF NOT EXISTS (SELECT 1 FROM duet_conversation_inputs WHERE account_id=target AND state IN ('queued','running','paused')) THEN RETURN; END IF;
               UPDATE duet_conversation_state SET revision=revision+1 WHERE account_id=target RETURNING revision INTO rev;
               UPDATE duet_conversation_attempts SET state='interrupted' WHERE state='running'
                 AND run_id IN (SELECT run_id FROM duet_conversation_inputs WHERE account_id=target);
               UPDATE duet_conversation_messages SET state='interrupted',revision=rev
                 WHERE account_id=target AND role='assistant' AND state IN ('pending','streaming');
               UPDATE duet_conversation_inputs SET state='interrupted',failure='interrupted',lease_until=NULL,revision=rev
                 WHERE account_id=target AND state IN ('queued','running','paused');
             END $$;"
        ).await?;
        Ok(())
    }
    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Err(DbErr::Custom(
            "Durable Duet execution history cannot be rolled back".into(),
        ))
    }
}
