-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- cost_controls owns one durable accounting source, replacing aggregation of
-- replaceable execution metadata. No content, prompt or health data is stored.
create table public.model_calls (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references public.users(id) on delete cascade,
    run_id uuid references public.conversation_runs(id) on delete set null,
    purpose text not null,
    started_at timestamptz not null,
    receipt jsonb not null check (jsonb_typeof(receipt)='object')
);
create index model_calls_person_time on public.model_calls(user_id,started_at);
create index model_calls_time on public.model_calls(started_at);
create index model_calls_run on public.model_calls(run_id) where run_id is not null;

-- Carry forward available receipts once. An adopted background receipt can also
-- occur in a run; keep the run's copy. Earlier requests with no stored receipt
-- cannot be reconstructed and are not invented as zero-use calls.
with available as (
    select r.user_id,r.id as run_id,r.created_at,
           case when r.message_id is not null then 'message'
                when r.follow_up_occurrence_id is not null then 'check_in'
                when r.dispatch_lane='interactive' then 'requested_assessment'
                else 'proactive_review' end as purpose, x.value as receipt
    from public.conversation_runs r
    cross join lateral jsonb_array_elements(coalesce(r.generation_metadata->'receipts','[]')) x
    union all
    select w.user_id,null,w.updated_at,'conversation_compaction',x.value
    from public.conversation_continuation_work w
    cross join lateral jsonb_array_elements(coalesce(w.candidate_metadata->'receipts','[]')) x
), unique_receipts as (
    select distinct on (user_id,receipt) * from available
    order by user_id,receipt,run_id nulls last
)
insert into public.model_calls(user_id,run_id,purpose,started_at,receipt)
select user_id,run_id,purpose,coalesce((receipt->>'started_at')::timestamptz,created_at),receipt
from unique_receipts;

alter table public.model_calls enable row level security;
create policy patient_rows on public.model_calls
    using (user_id=public.acting_account()) with check (user_id=public.acting_account());
insert into public.schema_migrations(version) values ('0118_model_call_accounting');
commit;
