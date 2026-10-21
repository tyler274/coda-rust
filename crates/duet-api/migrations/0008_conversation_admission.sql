-- Admission identity and resolved provenance belong to the accepted command.
-- Retries compare the submission, never re-resolve it against a changed source.
alter table public.duet_commands
 add column submission jsonb not null default '{"timezone":"UTC","selected_context":null,"reply_to_message_id":null}'::jsonb,
 add column admission jsonb not null default '{}'::jsonb;
