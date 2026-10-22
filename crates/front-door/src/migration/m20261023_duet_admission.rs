//! Additive Duet admission controls: the append-only operator AI switch and the
//! exact database instant each accepted input was counted against the allowance.
use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(
            "CREATE TABLE duet_model_execution_switch (
               id bigserial PRIMARY KEY,
               enabled boolean NOT NULL,
               changed_by text NOT NULL CHECK (length(btrim(changed_by)) BETWEEN 1 AND 200),
               reason text NOT NULL CHECK (length(btrim(reason)) BETWEEN 1 AND 1000),
               changed_at timestamptz NOT NULL DEFAULT clock_timestamp()
             );
             CREATE FUNCTION duet_switch_append_only() RETURNS trigger LANGUAGE plpgsql AS $$
             BEGIN RAISE EXCEPTION 'Duet model execution switch history is append-only'; END $$;
             CREATE TRIGGER duet_switch_append_only BEFORE UPDATE OR DELETE ON duet_model_execution_switch
               FOR EACH ROW EXECUTE FUNCTION duet_switch_append_only();
             CREATE TRIGGER duet_switch_no_truncate BEFORE TRUNCATE ON duet_model_execution_switch
               FOR EACH STATEMENT EXECUTE FUNCTION duet_switch_append_only();
             ALTER TABLE duet_conversation_inputs ADD COLUMN admitted_at timestamptz;
             UPDATE duet_conversation_inputs SET admitted_at=to_timestamp(created_at);
             ALTER TABLE duet_conversation_inputs ALTER COLUMN admitted_at SET DEFAULT clock_timestamp(),
               ALTER COLUMN admitted_at SET NOT NULL;
             CREATE INDEX duet_conversation_allowance ON duet_conversation_inputs(account_id,admitted_at)
               WHERE state<>'failed';",
        ).await?;
        Ok(())
    }
    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Err(DbErr::Custom(
            "Duet operator switch audit history cannot be rolled back".into(),
        ))
    }
}
