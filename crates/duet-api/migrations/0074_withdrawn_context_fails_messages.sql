-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- When an account's native context is deleted (a source authorization
-- changed), the reply Duet was writing to a person's message can't finish.
-- That is Duet's ending, not the person's: end those runs failed/unavailable,
-- so the person sees that Duet couldn't complete the response and the
-- message doesn't use their daily allowance. Check-ins and reviews are
-- interrupted as before.
--
-- Online: only this trigger function's body changes; no table, column or
-- stored row shape does. An old worker holding such a run sees it ended and
-- keeps that state, as it already does for interrupted runs.
create or replace function public.conversation_context_changed() returns trigger
language plpgsql as $$
declare change_revision bigint;
begin
    if tg_op='DELETE' then
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
        perform public.interrupt_conversation(old.user_id);
        return old;
    end if;
    if tg_op='INSERT' or new.snapshot_id is distinct from old.snapshot_id then
        perform public.interrupt_conversation(new.user_id, true);
    end if;
    return new;
end $$;

insert into public.schema_migrations(version) values('0074_withdrawn_context_fails_messages');
commit;
