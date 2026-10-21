-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Input arrival is not cancellation. Keep live foreground tasks admitted so
-- their worker can replace stale context under the same action identity/lease.
-- Background work retains its frozen admission and is still interrupted.
-- Prior workers remain safe: their unchanged source-currentness checks end
-- stale work instead of refreshing it. Existing one-argument callers retain
-- full interruption through the default; no stored row shape changes.
drop function public.interrupt_conversation(uuid);
create function public.interrupt_conversation(account_id uuid, background_only boolean default false) returns void
language plpgsql as $$
declare change_revision bigint;
begin
    change_revision := public.conversation_changed(account_id);
    if change_revision is null then return; end if;
    update public.insight_cycles set status='failed',claim_token=null,
        lease_expires_at=null,updated_at=now(),
        generation_metadata=jsonb_build_object(
            'schema_version',2,'provider','openai','outcome','failed',
            'reason','conversation_interrupted','receipts','[]'::jsonb)
    where user_id=account_id and status='generating'
      and assessment_run_id in (
          select id from public.conversation_runs
          where user_id=account_id and state in ('queued','running')
            and (not background_only or message_id is null
                 or follow_up_occurrence_id is not null or review_trigger is not null)
      );
    with ended as (
        update public.conversation_runs set state='interrupted',failure='interrupted',
            lease_until=null,revision=change_revision,updated_at=now()
        where user_id=account_id and state in ('queued','running')
          and (not background_only or message_id is null
               or follow_up_occurrence_id is not null or review_trigger is not null)
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
        perform public.interrupt_conversation(new.id);
    elsif new.evidence_revision is distinct from old.evidence_revision then
        perform public.interrupt_conversation(new.id, true);
    end if;
    return new;
end $$;

create or replace function public.conversation_context_changed() returns trigger
language plpgsql as $$
begin
    if tg_op='DELETE' then
        perform public.interrupt_conversation(old.user_id);
        return old;
    end if;
    if tg_op='INSERT' or new.snapshot_id is distinct from old.snapshot_id then
        perform public.interrupt_conversation(new.user_id, true);
    end if;
    return new;
end $$;

revoke all on function public.interrupt_conversation(uuid,boolean) from public;
insert into public.schema_migrations(version) values('0060_conversation_source_refresh');
commit;
