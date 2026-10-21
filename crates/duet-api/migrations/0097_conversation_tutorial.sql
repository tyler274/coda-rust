-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- A Duet reply may carry one known tutorial the assistant chose to show with
-- it. It is presentation only: it records no consent, action or health fact.
--
-- Online: the serving release reads messages through a model that ignores the
-- new nullable column and never writes it. The check is added NOT VALID; every
-- existing row is null.
alter table public.conversation_messages add column tutorial text;
alter table public.conversation_messages
    add constraint conversation_messages_tutorial_check
    check (tutorial in ('wearables_apple_health')) not valid;

insert into public.schema_migrations(version) values ('0097_conversation_tutorial');
commit;
