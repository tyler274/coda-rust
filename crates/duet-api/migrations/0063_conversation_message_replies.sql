-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- The UUID is deliberately retained even if a later product operation removes
-- the target message. ConversationStore validates account ownership and order
-- at each write; readers then project a missing target as unavailable without
-- erasing which message was addressed.
alter table public.conversation_messages
    add column reply_to_message_id uuid,
    add constraint conversation_reply_not_self
        check (reply_to_message_id is null or reply_to_message_id <> id);

insert into public.schema_migrations(version)
values ('0063_conversation_message_replies');
commit;
