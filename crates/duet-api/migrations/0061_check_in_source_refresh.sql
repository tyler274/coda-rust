-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Source arrival changes input, not the authority of an accepted Check-in.
-- Keep queued/running accepted work; unrequested reviews still freeze their
-- source/Today admission. All non-source cancellation calls retain full scope.
-- Only the parameter name changes; existing callers use positional arguments.
-- Old workers still reject stale input and settle without replaying work.
drop function public.interrupt_conversation(uuid,boolean);
create function public.interrupt_conversation(account_id uuid, reviews_only boolean default false) returns void
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
            and (not reviews_only or review_trigger is not null)
      );
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

revoke all on function public.interrupt_conversation(uuid,boolean) from public;
insert into public.schema_migrations(version) values('0061_check_in_source_refresh');
commit;
