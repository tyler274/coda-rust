-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Availability is part of the current conversation even before its first
-- message. Account authority changes must therefore create its revision as
-- well as wake an already open stream. During account deletion the source
-- cascade may call this function after the user row is gone; do not recreate
-- any state in that case.
create or replace function public.conversation_changed(account_id uuid) returns bigint
language plpgsql as $$
declare new_revision bigint;
begin
    insert into public.conversation_state(user_id, revision)
        select id, 1 from public.users where id=account_id
        on conflict (user_id) do update
        set revision=conversation_state.revision+1
        returning revision into new_revision;
    if new_revision is not null then
        perform pg_notify('duet_conversation', account_id::text);
    end if;
    return new_revision;
end $$;

insert into public.schema_migrations(version) values ('0037_empty_conversation_availability');
commit;
