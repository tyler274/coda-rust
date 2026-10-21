-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- A change of the account's authority, evidence or native context interrupts
-- in-flight assessments on purpose: their frozen inputs are no longer current.
-- Their Today slots were never decided at the new inputs, so release them to
-- ordinary admission instead of failing them for the rest of the day. The
-- interrupted run still cannot publish; its slot rows are gone and the
-- released slot is admitted afresh, under a new run, at the new revision.
-- Another release needs another input change. A newly accepted person's
-- message changes no input, so its interruption keeps failing the slot, as
-- does every ending a worker reaches itself.
--
-- Online: function bodies only; no table or stored row shape changes. The
-- serving runtime's one-argument message call keeps failing slots through the
-- default, and every reader already treats an absent slot as not admitted.
drop function public.interrupt_conversation(uuid,boolean);
create function public.interrupt_conversation(
    account_id uuid, reviews_only boolean default false,
    inputs_changed boolean default false
) returns void
language plpgsql as $$
declare change_revision bigint;
begin
    change_revision := public.conversation_changed(account_id);
    if change_revision is null then return; end if;
    if inputs_changed then
        delete from public.insight_cycles
        where user_id=account_id and status='generating'
          and assessment_run_id in (
              select id from public.conversation_runs
              where user_id=account_id and state in ('queued','running')
                and (not reviews_only or review_trigger is not null)
          );
    else
        update public.insight_cycles set status='failed',claim_token=null,
            lease_expires_at=null,updated_at=now(),
            generation_metadata=jsonb_build_object(
                'schema_version',2,'provider','openai','outcome','failed',
                'reason','conversation_interrupted','receipts','[]'::jsonb)
        where user_id=account_id and status='generating'
          and assessment_run_id in (
              select id from public.conversation_runs
              where user_id=account_id and state in ('queued','running')
                and (not reviews_only or review_trigger is not null)
          );
    end if;
    with ended as (
        update public.conversation_runs set state='interrupted',failure='interrupted',
            lease_until=null,revision=change_revision,updated_at=now()
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
        perform public.interrupt_conversation(new.id, false, true);
    elsif new.evidence_revision is distinct from old.evidence_revision then
        perform public.interrupt_conversation(new.id, true, true);
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
        perform public.interrupt_conversation(old.user_id, false, true);
        return old;
    end if;
    if tg_op='INSERT' or new.snapshot_id is distinct from old.snapshot_id then
        perform public.interrupt_conversation(new.user_id, true, true);
    end if;
    return new;
end $$;

insert into public.schema_migrations(version) values('0088_input_change_releases_today_slot');
commit;
