-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- A run owns its unfinished outputs across bounded attempts. Remove the
-- unused trigger hash (its identity index was retired by 0094).
-- Maintenance: the previous image still writes review_key.
alter table public.conversation_runs
    drop constraint conversation_runs_assignment_check,
    drop column review_key;
alter table public.conversation_runs
    add constraint conversation_runs_assignment_check check(
        (message_id is not null and assistant_message_id is not null
            and follow_up_id is null and follow_up_revision is null
            and follow_up_occurrence_id is null and assignment_time is null
            and review_trigger is null)
        or
        (message_id is null and assistant_message_id is not null
            and follow_up_id is not null and follow_up_revision is not null
            and follow_up_occurrence_id is not null and assignment_time is not null
            and review_trigger is null)
        or
        (message_id is null and follow_up_id is null and follow_up_revision is null
            and follow_up_occurrence_id is null and selected_follow_up_id is null
            and assignment_time is not null
            and review_trigger is not null and jsonb_typeof(review_trigger)='object'
            and review_trigger->>'schema_version'='1'
            and jsonb_typeof(review_trigger->'reasons')='array'
            and jsonb_array_length(review_trigger->'reasons')>0
            and review_trigger->'reasons' <@
                '["new_information","weekly_review","daily_today","weekly_today"]'::jsonb
            and jsonb_typeof(review_trigger->'today_slots')='array')
    );

-- Old workers could finish a review despite an undelivered message or
-- failed Today slot. Keep that history visibly failed, preserving every
-- healthy output. This migration never requeues or replays old work.
update public.conversation_runs r
set state='failed',failure='execution_failed',
    revision=public.conversation_changed(r.user_id),updated_at=now()
where r.state='completed' and r.review_trigger is not null
  and (r.generation_metadata->>'review_outcome'='undelivered'
       or exists(select 1 from public.insight_cycles c
                 where c.assessment_run_id=r.id and c.status='failed'));

-- Coverage belongs to the daily decision that published it. Later attempts
-- may review more records without changing what an earlier Today decision saw.
update public.insight_cycles c
set generation_metadata=c.generation_metadata || jsonb_build_object(
    'reviewed_through',coalesce((r.generation_metadata->>'reviewed_through')::bigint,c.history_cursor))
from public.conversation_runs r
where c.assessment_run_id=r.id and c.status='ready'
  and not c.generation_metadata ? 'reviewed_through';

-- Pending assignments keep their original arrival range without storing an
-- unbounded duplicate list. The existing record reader supplies its pages.
update public.conversation_runs r
set review_trigger=jsonb_set(r.review_trigger,'{today_slots}',(
    select coalesce(jsonb_agg(
        (slot - 'record_revisions' - 'fact_ids') ||
        case when jsonb_array_length(coalesce(slot->'record_revisions','[]'))>0
          then jsonb_build_object('records_since_revision',(
            select min(value::bigint)-1 from jsonb_array_elements_text(slot->'record_revisions')))
          else '{}'::jsonb end order by position),'[]'::jsonb)
    from jsonb_array_elements(r.review_trigger->'today_slots') with ordinality as slots(slot,position)
))
where r.review_trigger is not null and r.state in ('queued','running');

-- All preemption paths use the same transaction: committed outputs stand;
-- unfinished outputs are released for a future authorized assignment.
create function public.interrupt_conversation_runs(
    account_id uuid, run_ids uuid[], release_follow_ups boolean default false
) returns void
language plpgsql as $$
declare change_revision bigint;
declare released uuid[];
begin
    change_revision := public.conversation_changed(account_id);
    if change_revision is null then return; end if;
    delete from public.insight_cycles
    where user_id=account_id and status='generating'
      and assessment_run_id in (
          select id from public.conversation_runs
          where user_id=account_id and state in ('queued','running')
            and id=any(run_ids)
      );
    with ended as (
        update public.conversation_runs set state='interrupted',failure='interrupted',
            preempted_at=now(),lease_until=null,revision=change_revision,updated_at=now()
        where user_id=account_id and state in ('queued','running')
          and id=any(run_ids)
        returning assistant_message_id,follow_up_occurrence_id
    ), messages as (
        update public.conversation_messages set state='interrupted',
            revision=change_revision,updated_at=now()
        where id in (select assistant_message_id from ended) and state<>'complete'
    ), occurrences as (
        update public.follow_up_occurrences set status='interrupted',
            completed_at=now(),updated_at=now()
        where id in (select follow_up_occurrence_id from ended)
          and status in('queued','running')
        returning id
    )
    select coalesce(array_agg(id),'{}') into released from occurrences;
    if release_follow_ups then
        perform public.release_interrupted_follow_ups(account_id, released);
    end if;
end $$;

create or replace function public.interrupt_conversation(
    account_id uuid, reviews_only boolean default false,
    release_follow_ups boolean default false
) returns void
language sql as $$
    select public.interrupt_conversation_runs(account_id,
        array(select id from public.conversation_runs
              where user_id=account_id and state in ('queued','running')
                and (not reviews_only or review_trigger is not null)),
        release_follow_ups);
$$;

insert into public.schema_migrations(version) values('0113_review_run_ownership');
commit;
