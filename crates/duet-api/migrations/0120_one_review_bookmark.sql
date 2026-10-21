-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Retain credit from every committed output, including a Today decision whose
-- conversation failed. Never infer review progress from a claimed input cursor.
update public.conversation_runs r
set generation_metadata=jsonb_set(r.generation_metadata,'{reviewed_through}',
    to_jsonb(greatest(coalesce((r.generation_metadata->>'reviewed_through')::bigint,0),c.reviewed_through)))
from (
    select assessment_run_id,max((generation_metadata->>'reviewed_through')::bigint) as reviewed_through
    from public.insight_cycles where status='ready'
      and generation_metadata ? 'reviewed_through'
    group by assessment_run_id
) c where r.id=c.assessment_run_id;
update public.insight_cycles set generation_metadata=generation_metadata-'reviewed_through'
where generation_metadata ? 'reviewed_through';

-- Accepted runs keep their original input and pending slot identities. Translate
-- the old per-day input range into calendar context; it is not reviewed credit.
update public.conversation_runs r
set history_cursor=greatest(r.history_cursor,(r.review_trigger->>'history_cursor')::bigint),
    review_trigger=jsonb_set(r.review_trigger,'{today_slots}',(
        select coalesce(jsonb_agg((s-'records_since_revision') ||
            jsonb_build_object('timezone',r.timezone,'first_weekday',7)),'[]'::jsonb)
        from jsonb_array_elements(r.review_trigger->'today_slots') s
    )) || case when exists(
        select 1 from jsonb_array_elements(r.review_trigger->'today_slots') s where s->>'kind'='daily'
    ) then jsonb_build_object('calendar_since_revision',coalesce((
        select min((s->>'records_since_revision')::bigint)
        from jsonb_array_elements(r.review_trigger->'today_slots') s
    ),(r.review_trigger->>'new_since_revision')::bigint,0)) else '{}'::jsonb end
where r.review_trigger is not null;

-- Runs own all execution and interruption. Published slots have no lease.
create or replace function public.interrupt_conversation_runs(
    account_id uuid, run_ids uuid[], release_follow_ups boolean default false
) returns void language plpgsql as $$
declare change_revision bigint; pause_time timestamptz;
begin
    change_revision := public.conversation_changed(account_id);
    if change_revision is null then return; end if;
    perform id from public.conversation_runs
        where user_id=account_id and id=any(run_ids) for update;
    pause_time := clock_timestamp();
    with ended as (
        update public.conversation_runs set state=case when release_follow_ups then 'paused' else 'interrupted' end,
            failure=case when not release_follow_ups then 'interrupted' end,
            paused_at=case when release_follow_ups then coalesce(paused_at,pause_time) end,
            lease_until=null,revision=change_revision,updated_at=now()
        where user_id=account_id and id=any(run_ids)
          and state in ('queued','running','paused')
        returning id,assistant_message_id,follow_up_occurrence_id
    ), messages as (
        update public.conversation_messages set state='interrupted',
            revision=change_revision,updated_at=now()
        where id in (select assistant_message_id from ended) and state<>'complete'
    )
    update public.follow_up_occurrences set
        status=case when release_follow_ups then 'queued' else 'interrupted' end,
        completed_at=case when not release_follow_ups then now() end,updated_at=now()
    where id in (select follow_up_occurrence_id from ended)
      and status in ('queued','running');
end $$;

delete from public.insight_cycles where status<>'ready';
drop view public.insight_cycles_bookkeeping;
alter table public.insight_cycles
    drop column claim_token,
    drop column lease_expires_at,
    alter column status set default 'ready',
    drop constraint insight_cycles_status_check,
    add constraint insight_cycles_status_check check(status='ready');

insert into public.schema_migrations(version) values('0120_one_review_bookmark');
commit;
