-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- One coalesced maintenance row per account. Transcript prose stays in the
-- canonical conversation tables; this row contains only scheduling fences,
-- one bounded attempt, and at most one replaceable ready candidate.
create table public.conversation_continuation_work (
    user_id uuid primary key references public.users(id) on delete cascade,
    id uuid not null unique,
    state text not null check(state in ('pending','queued','running','ready','failed')),
    requested_through bigint not null default 0 check(requested_through >= 0),
    evaluated_through bigint not null default 0 check(evaluated_through >= 0),
    timezone text,
    predecessor_generation uuid,
    predecessor_through bigint check(predecessor_through is null or predecessor_through >= 0),
    transcript_generation bigint check(transcript_generation is null or transcript_generation >= 0),
    observed_through bigint check(observed_through is null or observed_through > 0),
    through_sequence bigint check(through_sequence is null or through_sequence > 0),
    observed_message_id uuid,
    control_revision bigint,
    evidence_revision bigint,
    history_cursor bigint,
    context_snapshot_id uuid,
    claim_token uuid,
    lease_until timestamptz,
    delivery_generation integer not null default 0,
    delivery_until timestamptz,
    delivery_confirmed boolean not null default false,
    candidate_generation uuid,
    candidate_markdown text
        check(candidate_markdown is null or (
            candidate_markdown ~ '[^[:space:]]'
            and octet_length(candidate_markdown) <= 16384)),
    candidate_metadata jsonb check(candidate_metadata is null or (
        jsonb_typeof(candidate_metadata)='object'
        and octet_length(candidate_metadata::text) <= 8192)),
    candidate_updated_at timestamptz,
    input_changed boolean not null default false,
    failure text check(failure in ('changed','unavailable','interrupted')),
    accepted_at timestamptz,
    updated_at timestamptz not null default now(),
    check(evaluated_through <= requested_through),
    check((state='running') = (claim_token is not null and lease_until is not null)),
    check((state='ready') = (
        candidate_generation is not null and candidate_markdown is not null
        and candidate_updated_at is not null)),
    check((state='failed') = (failure is not null)),
    check(state='pending' or (
        timezone is not null and predecessor_through is not null
        and transcript_generation is not null and observed_through is not null
        and through_sequence is not null and through_sequence <= observed_through
        and observed_message_id is not null and control_revision is not null
        and evidence_revision is not null and history_cursor is not null
        and accepted_at is not null))
);
create index conversation_continuation_work_pending
    on public.conversation_continuation_work(state,user_id)
    where requested_through > evaluated_through or state in ('queued','running');

-- Migration 0064 advances the transcript generation. This companion trigger
-- terminalizes an attempt/candidate only when the changed terminal input is in
-- its frozen prefix. Later appends remain compatible. The pending eligibility
-- marker is retained so a meaningful later horizon can be evaluated.
create function public.invalidate_conversation_continuation_work() returns trigger
language plpgsql as $$
declare
    account_id uuid;
    affected_sequence bigint;
    material boolean;
begin
    if tg_op='DELETE' then
        if old.state not in ('complete','interrupted','failed') then return old; end if;
        account_id := old.user_id;
        affected_sequence := old.sequence;
    else
        if old.state not in ('complete','interrupted','failed') then return new; end if;
        material := (new.user_id,new.id,new.role,new.text,new.state,new.sequence,
                     new.sources,new.selected_insight,new.selected_context,
                     new.reply_to_message_id)
            is distinct from
                    (old.user_id,old.id,old.role,old.text,old.state,old.sequence,
                     old.sources,old.selected_insight,old.selected_context,
                     old.reply_to_message_id);
        if not material then return new; end if;
        account_id := old.user_id;
        affected_sequence := least(old.sequence,new.sequence);
    end if;
    if not exists(select 1 from public.users where id=account_id) then
        if tg_op='DELETE' then return old; else return new; end if;
    end if;
    update public.conversation_continuation_work
       set state='failed',failure='changed',claim_token=null,lease_until=null,
           input_changed=true,
           candidate_generation=null,candidate_markdown=null,candidate_metadata=null,
           candidate_updated_at=null,delivery_until=null,delivery_confirmed=false,
           updated_at=now()
     where user_id=account_id and state in ('queued','running','ready')
       and through_sequence >= affected_sequence;
    if tg_op='DELETE' then return old; else return new; end if;
end $$;
create trigger conversation_continuation_work_terminal_mutated
    after update on public.conversation_messages for each row
    execute function public.invalidate_conversation_continuation_work();
create trigger conversation_continuation_work_terminal_deleted
    before delete on public.conversation_messages for each row
    execute function public.invalidate_conversation_continuation_work();

-- Existing conversations receive only an eligibility marker. Recovery later
-- applies current consent/AI gates and measures the actual bounded transcript;
-- this migration performs no inference and creates no runnable claim.
insert into public.conversation_continuation_work(
    user_id,id,state,requested_through)
select transcript.user_id,gen_random_uuid(),'pending',max(transcript.sequence)
  from public.conversation_messages transcript
 where transcript.state in ('complete','interrupted','failed')
   and exists (
       select 1
         from public.conversation_runs run
         join public.conversation_messages reply
           on reply.user_id=run.user_id and reply.id=run.assistant_message_id
        where run.user_id=transcript.user_id
          and run.state='completed'
          and reply.role='assistant' and reply.state='complete'
          and reply.text ~ '[^[:space:]]'
          and run.generation_metadata->>'operation'
              is distinct from 'fixed_reminder'
   )
 group by transcript.user_id;

revoke all on public.conversation_continuation_work from public;
revoke all on function public.invalidate_conversation_continuation_work() from public;
insert into public.schema_migrations(version)
values('0065_conversation_preparation');
commit;
