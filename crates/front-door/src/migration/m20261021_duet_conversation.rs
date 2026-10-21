//! Additive Duet-only accepted-work storage. No physician runtime types.
use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(
            "CREATE TABLE duet_conversation_state (
                account_id uuid PRIMARY KEY REFERENCES front_door_accounts(id) ON DELETE CASCADE,
                revision bigint NOT NULL DEFAULT 0 CHECK (revision >= 0),
                next_sequence bigint NOT NULL DEFAULT 1 CHECK (next_sequence > 0)
            );
            CREATE TABLE duet_conversation_inputs (
                account_id uuid NOT NULL REFERENCES front_door_accounts(id) ON DELETE CASCADE,
                message_id uuid NOT NULL,
                run_id uuid NOT NULL UNIQUE,
                reply_id uuid NOT NULL UNIQUE,
                timezone text NOT NULL CHECK (length(timezone) BETWEEN 1 AND 100),
                state text NOT NULL CHECK (state IN ('queued','interrupted')),
                permission_revision bigint NOT NULL,
                revision bigint NOT NULL,
                created_at bigint NOT NULL DEFAULT EXTRACT(EPOCH FROM now())::bigint,
                PRIMARY KEY(account_id,message_id)
            );
            CREATE TABLE duet_conversation_messages (
                account_id uuid NOT NULL REFERENCES front_door_accounts(id) ON DELETE CASCADE,
                id uuid NOT NULL,
                input_id uuid NOT NULL,
                sequence bigint NOT NULL CHECK (sequence > 0),
                revision bigint NOT NULL,
                role text NOT NULL CHECK (role IN ('user','assistant')),
                text text NOT NULL CHECK (length(text) <= 32768),
                state text NOT NULL CHECK (state IN ('complete','pending','interrupted')),
                PRIMARY KEY(account_id,id),
                UNIQUE(account_id,sequence),
                FOREIGN KEY(account_id,input_id) REFERENCES duet_conversation_inputs(account_id,message_id) ON DELETE CASCADE
            );
            CREATE INDEX duet_conversation_queued ON duet_conversation_inputs(account_id) WHERE state='queued';
            CREATE FUNCTION duet_interrupt_accepted(target uuid) RETURNS void LANGUAGE plpgsql AS $$
            DECLARE rev bigint;
            BEGIN
                IF NOT EXISTS (SELECT 1 FROM duet_conversation_inputs WHERE account_id=target AND state='queued') THEN RETURN; END IF;
                UPDATE duet_conversation_state SET revision=revision+1 WHERE account_id=target RETURNING revision INTO rev;
                UPDATE duet_conversation_messages SET state='interrupted',revision=rev
                  WHERE account_id=target AND role='assistant' AND state='pending';
                UPDATE duet_conversation_inputs SET state='interrupted',revision=rev
                  WHERE account_id=target AND state='queued';
            END $$;
            CREATE FUNCTION duet_permission_interrupt() RETURNS trigger LANGUAGE plpgsql AS $$
            BEGIN
                IF (OLD.luna_allowed AND NOT NEW.luna_allowed)
                   OR OLD.luna_version IS DISTINCT FROM NEW.luna_version
                   OR (NOT OLD.restriction_pending AND NEW.restriction_pending) THEN
                    PERFORM duet_interrupt_accepted(NEW.account_id);
                END IF;
                RETURN NEW;
            END $$;
            CREATE TRIGGER duet_permission_interrupt AFTER UPDATE ON front_door_permissions
              FOR EACH ROW EXECUTE FUNCTION duet_permission_interrupt();"
        ).await?;
        Ok(())
    }
    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Err(DbErr::Custom(
            "Accepted conversation history cannot be rolled back".into(),
        ))
    }
}
