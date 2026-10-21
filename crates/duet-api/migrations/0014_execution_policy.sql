-- Frozen admission and content-free accounting belong to the existing command.
alter table public.duet_commands
    add column skill_catalog jsonb not null default '{}';

alter table public.duet_commands
    add column execution_started_at timestamptz;

create table public.duet_model_calls (
    user_id uuid not null references public.users (id) on delete cascade,
    command_id uuid not null,
    sequence bigint
        not null check (sequence >
            0),
            claim bigint not null,
            model text not null,
            purpose text not null,
            started_at timestamptz default clock_timestamp(),
            outcome text not null default 'unknown' check (outcome in ('unknown', 'observed', 'completed', 'failed')),
            input_tokens bigint check (input_tokens >= 0),
            output_tokens bigint check (output_tokens >= 0),
            cached_input_tokens bigint check (cached_input_tokens >= 0),
            response_id text,
            primary key (user_id, command_id, sequence),
            foreign key (user_id, command_id) references public.duet_commands (user_id, id) on delete cascade
);

-- Adopt pre-ledger dispatches once. Their actual dispatch time is unavailable;
-- do not invent a timestamp or price. Merge existing cumulative observations.
insert into public.duet_model_calls (user_id, command_id, sequence,
    claim,
    model,
    purpose,
    started_at
)
select distinct on (r.user_id, r.command_id, (r.record -> 'Dispatch' ->> 'sequence')
::bigint) r.user_id,
r.command_id,
(r.record -> 'Dispatch' ->> 'sequence')::bigint,
r.claim,
r.record -> 'Dispatch' -> 'request' ->> 'model',
d.work_kind,
null
from
    public.duet_execution_records r
    join public.duet_commands d on d.user_id = r.user_id
        and d.id = r.command_id
where
    r.record ? 'Dispatch'
order by
    r.user_id,
    r.command_id,
    (r.record -> 'Dispatch' ->> 'sequence')::bigint,
    r.ordinal;

do $$
declare
    r record;
    body jsonb;
    usage jsonb;
    provenance jsonb;
    seq bigint;
begin
    for r in
    select
        *
    from
        public.duet_execution_records
    where
        record ? 'Exchange'
        or record ? 'Observation'
    order by
        ordinal loop
            body := coalesce(r.record -> 'Exchange', r.record -> 'Observation');
            seq := (body ->> 'sequence')::bigint;
            usage := coalesce(body -> 'response' -> 'usage', body -> 'usage');
            provenance := coalesce(body -> 'response' -> 'provenance', body -> 'provenance');
            update
                public.duet_model_calls
            set
                input_tokens = coalesce((usage ->> 'input_tokens')::bigint, input_tokens),
                output_tokens = coalesce((usage ->> 'output_tokens')::bigint, output_tokens),
                cached_input_tokens = coalesce((provenance ->> 'cached_input_tokens')::bigint, cached_input_tokens),
                response_id = coalesce(provenance ->> 'response_id', response_id),
                outcome = case when r.record ? 'Exchange' then
                    'completed'
                else
                    outcome
                end
            where
                user_id = r.user_id
                and command_id = r.command_id
                and sequence =
                    seq
                    and claim = r.claim;
        end loop;
end
$$;

alter table public.duet_model_calls enable row level security;

alter table public.duet_model_calls force row level security;

create policy duet_account on public.duet_model_calls
    using (user_id = public.acting_account ())
    with check (user_id = public.acting_account ());

revoke all on public.duet_model_calls from public;
