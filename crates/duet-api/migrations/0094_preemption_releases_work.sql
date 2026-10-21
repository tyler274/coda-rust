-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- The person's preemption never uses up work (North Star, September 29). A
-- run ended by the person's newer message, the person's own control, or a
-- change of the account's authority, evidence or native context has decided
-- nothing: its undecided Today slots are released and the run is stamped, so
-- admission treats its slot and review boundary as still owed. Endings Duet
-- reaches itself stay unstamped and terminal, so nothing is retried in a loop.
--
-- The trigger-identity index recorded a second notion of done that only ever
-- matched preempted runs; admission's handled checks and slot rows decide.
--
-- Online: a nullable column without a default, an index drop and function
-- bodies. The serving runtime's one-argument message call resolves to the new
-- two-argument function, and its admission still reads a valid schema.
alter table public.conversation_runs add column preempted_at timestamptz;

drop index public.conversation_review_identity;

drop function public.interrupt_conversation(uuid,boolean,boolean);
create function public.interrupt_conversation(
    account_id uuid, reviews_only boolean default false
) returns void
language plpgsql as $$
declare change_revision bigint;
begin
    change_revision := public.conversation_changed(account_id);
    if change_revision is null then return; end if;
    delete from public.insight_cycles
    where user_id=account_id and status='generating'
      and assessment_run_id in (
          select id from public.conversation_runs
          where user_id=account_id and state in ('queued','running')
            and (not reviews_only or review_trigger is not null)
      );
    with ended as (
        update public.conversation_runs set state='interrupted',failure='interrupted',
            preempted_at=now(),lease_until=null,revision=change_revision,updated_at=now()
        where user_id=account_id and state in ('queued','running')
          and (not reviews_only or review_trigger is not null)
        returning assistant_message_id,follow_up_occurrence_id
    ), messages as (
        update public.conversation_messages set state='interrupted',
            revision=change_revision,updated_at=now()
        where id in (select assistant_message_id from ended)
    )
    update public.follow_up_occurrences set status='interrupted',
        completed_at=now(),updated_at=now()
    where id in (select follow_up_occurrence_id from ended)
      and status in('queued','running');
end $$;

create or replace function public.conversation_account_authority_changed() returns trigger
language plpgsql as $$
begin
    if tg_op='DELETE' then
        perform pg_notify('duet_conversation',old.id::text);
        return old;
    end if;
    if (new.intelligence_control_revision,new.terms_version) is distinct from
       (old.intelligence_control_revision,old.terms_version) then
        perform public.interrupt_conversation(new.id, false);
    elsif new.evidence_revision is distinct from old.evidence_revision then
        perform public.interrupt_conversation(new.id, true);
    end if;
    return new;
end $$;

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
        perform public.interrupt_conversation(old.user_id, false);
        return old;
    end if;
    if tg_op='INSERT' or new.snapshot_id is distinct from old.snapshot_id then
        perform public.interrupt_conversation(new.user_id, true);
    end if;
    return new;
end $$;

insert into public.schema_migrations(version) values('0094_preemption_releases_work');
commit;
