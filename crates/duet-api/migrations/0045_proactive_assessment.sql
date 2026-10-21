-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- One source capture can carry both current presentation projections. Source
-- identity is separate from presentation/freshness, so rebuilding a day or
-- week does not manufacture new health information.
alter table public.health_contexts
    add column source_boundary_id uuid,
    add column projections jsonb not null default '[]'::jsonb
        check(jsonb_typeof(projections)='array');
update public.health_contexts
set source_boundary_id=snapshot_id
where source_boundary_id is null;
alter table public.health_contexts
    alter column source_boundary_id set not null;

drop trigger native_snapshot_update_supersede_operating_index
    on public.health_contexts;
create trigger native_snapshot_update_supersede_operating_index after update
on public.health_contexts for each row
when(old.source_boundary_id is distinct from new.source_boundary_id)
execute function public.supersede_operating_index_inputs();

alter table public.conversation_messages
    add column selected_context jsonb;

-- Preserve completed review history while replacing the single-kind trigger
-- with the combined boundary consumed by one proactive assessment.
alter table public.conversation_runs
    drop constraint conversation_runs_assignment_check;

update public.conversation_runs
set review_trigger = jsonb_strip_nulls(jsonb_build_object(
    'schema_version', 1,
    'reasons', case review_trigger->>'kind'
        when 'new_information' then '["new_information"]'::jsonb
        else '["weekly_review"]'::jsonb end,
    'evidence_revision', review_trigger->'evidence_revision',
    'history_cursor', review_trigger->'history_cursor',
    'native_snapshot_id', review_trigger->'native_snapshot_id',
    'source_boundary_id', review_trigger->'native_snapshot_id',
    'weekly_window_start', review_trigger->'window_start',
    'weekly_window_end', review_trigger->'window_end',
    'today_slots', '[]'::jsonb
))
where review_trigger is not null and review_trigger ? 'kind';

alter table public.conversation_runs
    add constraint conversation_runs_assignment_check check(
        (message_id is not null and assistant_message_id is not null
            and follow_up_id is null and follow_up_revision is null
            and follow_up_occurrence_id is null and assignment_time is null
            and review_trigger is null and review_key is null)
        or
        (message_id is null and assistant_message_id is not null
            and follow_up_id is not null and follow_up_revision is not null
            and follow_up_occurrence_id is not null and assignment_time is not null
            and review_trigger is null and review_key is null)
        or
        (message_id is null and follow_up_id is null and follow_up_revision is null
            and follow_up_occurrence_id is null and selected_follow_up_id is null
            and assignment_time is not null and review_key is not null
            and review_trigger is not null and jsonb_typeof(review_trigger)='object'
            and review_trigger->>'schema_version'='1'
            and jsonb_typeof(review_trigger->'reasons')='array'
            and jsonb_array_length(review_trigger->'reasons')>0
            and review_trigger->'reasons' <@
                '["new_information","weekly_review","daily_today","weekly_today"]'::jsonb
            and jsonb_typeof(review_trigger->'today_slots')='array')
    );

-- A conversation run owns inference and may atomically fill both subordinate
-- Today slots. Pre-migration generating cycles have unknowable provider state
-- and are made terminal rather than attached or replayed.
update public.insight_cycles set status='failed',claim_token=null,
    lease_expires_at=null,updated_at=now(),
    generation_metadata=jsonb_build_object(
        'schema_version',2,'provider','openai','outcome','failed',
        'reason','assessment_migration','receipts','[]'::jsonb)
where status='generating';

alter table public.insight_cycles
    alter column context_snapshot_id drop not null,
    add column assessment_run_id uuid,
    add constraint insight_cycles_assessment_run_fkey
        foreign key(user_id,assessment_run_id)
        references public.conversation_runs(user_id,id) on delete set null
        deferrable initially deferred,
    drop constraint insight_cycles_ready_payload_check,
    add constraint insight_cycles_ready_payload_check check (
        status='generating' or status='failed' or (
            jsonb_array_length(insights) between 0 and 3
            and observed_through is not null and generated_at is not null
            and (
                (jsonb_array_length(insights)=0 and jsonb_array_length(evidence)=0)
                or (jsonb_array_length(insights)>0 and jsonb_array_length(evidence)>0)
            )
        )
    );
create unique index insight_cycle_assessment_kind
    on public.insight_cycles(assessment_run_id,kind)
    where assessment_run_id is not null;

-- Citation rows retain the exact successful read receipts selected for the
-- patient-facing claim. Existing citations remain valid historical references.
alter table public.conversation_citations
    add column read_receipts jsonb not null default '[]'::jsonb
        check(jsonb_typeof(read_receipts)='array');

create or replace function public.interrupt_conversation(account_id uuid) returns void
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
      );
    with ended as (
        update public.conversation_runs set state='interrupted',failure='interrupted',
            lease_until=null,revision=change_revision,updated_at=now()
        where user_id=account_id and state in ('queued','running')
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

insert into public.schema_migrations(version) values('0045_proactive_assessment');
commit;
