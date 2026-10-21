-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Upgrade already-applied 0064/0065 without replaying or rewriting them. One
-- terminal-input policy now owns both active-note and prepared-work invalidation.
drop trigger conversation_continuation_work_terminal_mutated on public.conversation_messages;
drop trigger conversation_continuation_work_terminal_deleted on public.conversation_messages;
drop function public.invalidate_conversation_continuation_work();

create or replace function public.conversation_terminal_input_changed() returns trigger
language plpgsql as $$
declare
    account_id uuid;
    affected_sequence bigint;
begin
    if old.state not in ('complete', 'interrupted', 'failed') then
        if tg_op = 'DELETE' then return old; else return new; end if;
    end if;
    account_id := old.user_id;
    affected_sequence := old.sequence;
    if tg_op = 'UPDATE' then
        if (new.user_id, new.id, new.role, new.text, new.state, new.sequence,
            new.sources, new.selected_insight, new.selected_context, new.reply_to_message_id)
            is not distinct from
           (old.user_id, old.id, old.role, old.text, old.state, old.sequence,
            old.sources, old.selected_insight, old.selected_context, old.reply_to_message_id) then
            return new;
        end if;
        affected_sequence := least(old.sequence, new.sequence);
    end if;

    -- Canonical writers already lock the account before its children. Keep this
    -- order and skip parent cascades: erasure must not create surviving work.
    perform 1 from public.users where id = account_id for update;
    if not found then
        if tg_op = 'DELETE' then return old; else return new; end if;
    end if;
    update public.conversation_state
       set transcript_generation = transcript_generation + 1
     where user_id = account_id;
    delete from public.conversation_continuations
     where user_id = account_id and through_sequence >= affected_sequence;
    update public.conversation_continuation_work
       set state='failed',failure='changed',claim_token=null,lease_until=null,
           input_changed=true,
           candidate_generation=null,candidate_markdown=null,candidate_metadata=null,
           candidate_updated_at=null,delivery_until=null,delivery_confirmed=false,
           updated_at=now()
     where user_id=account_id and state in ('queued','running','ready')
       and through_sequence >= affected_sequence;
    -- A previously evaluated small window may grow or shrink on a correction.
    -- Make the existing marker eligible again without inventing another job.
    update public.conversation_continuation_work
       set evaluated_through=least(evaluated_through,greatest(0,affected_sequence-1)),
           input_changed=input_changed or coalesce(through_sequence>=affected_sequence,false),
           delivery_until=null,delivery_confirmed=false,updated_at=now()
     where user_id=account_id and state in ('pending','failed');
    if tg_op = 'DELETE' then return old; else return new; end if;
end $$;

revoke all on function public.conversation_terminal_input_changed() from public;
insert into public.schema_migrations(version)
values ('0066_unified_conversation_invalidation');
commit;
