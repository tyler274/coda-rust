-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Replies and reviews hold the phone snapshot captured by their claim.
-- New uploads do not revoke it. Deleting the context still ends its work.
-- Online: only a trigger and function body change; no data or wire shape
-- changes. Older workers retain their own stricter snapshot checks.
drop trigger conversation_context_authority on public.health_contexts;
create trigger conversation_context_authority
    after delete on public.health_contexts
    for each row execute function public.conversation_context_changed();

create or replace function public.conversation_context_changed() returns trigger
language plpgsql as $$
declare change_revision bigint;
begin
    change_revision := public.conversation_changed(old.user_id);
    if change_revision is not null then
        with ended as (
            update public.conversation_runs set state='failed',failure='unavailable',
                lease_until=null,revision=change_revision,updated_at=now()
            where user_id=old.user_id and state in ('queued','running')
              and message_id is not null
            returning assistant_message_id
        )
        update public.conversation_messages set state='failed',
            revision=change_revision,updated_at=now()
        where id in (select assistant_message_id from ended);
    end if;
    perform public.interrupt_conversation(old.user_id, false);
    return old;
end $$;

insert into public.schema_migrations(version) values('0115_claimed_phone_snapshot');
commit;
