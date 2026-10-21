-- Adopt original retained owners without introducing a client-authored publication
-- or projecting doctor-device snapshots into canonical health context. Fresh
-- installations start empty: their producer lifecycles are separate work.
create table if not exists public.health_contexts (
    user_id uuid primary key references public.users (id) on delete cascade,
    snapshot_id uuid not null,
    source_boundary_id uuid not null,
    schema_version integer not null default 1 check (schema_version > 0),
    revision bigint not null default 1 check (revision > 0),
    generated_at timestamptz not null,
    timezone text not null,
    source_generations jsonb not null default '{}',
    facts jsonb not null default '[]',
    projections jsonb not null default '[]',
    updated_at timestamptz not null default now()
);

create table if not exists public.follow_ups (
    id uuid primary key,
    user_id uuid not null references public.users (id) on delete cascade,
    operation text not null default 'check_in',
    accepted_instruction text not null,
    schedule jsonb not null,
    schedule_summary text not null,
    revision bigint not null default 1 check (revision > 0),
    state text not null check (state in ('active', 'paused', 'cancelled', 'completed')),
    next_occurrence_at timestamptz,
    source_message_ids uuid[] not null,
    display_title text,
    confirmation_required boolean not null default false,
    routine_id uuid,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    unique (user_id, id)
);

create table if not exists public.insight_cycles (
    id uuid primary key,
    user_id uuid not null references public.users (id) on delete cascade,
    context_snapshot_id uuid,
    assessment_run_id uuid,
    kind text not null check (kind in ('daily', 'weekly')),
    window_start timestamptz not null,
    window_end timestamptz not null,
    timezone text not null,
    first_weekday smallint not null,
    status text not null check (status = 'ready'),
    insights jsonb not null,
    supporting_facts jsonb not null,
    concept_ids text[] not null default '{}',
    corrected_insight_ids uuid[] not null default '{}',
    observed_through timestamptz not null,
    generated_at timestamptz not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    unique (user_id, id),
    check (window_end > window_start)
);

-- Writers share the account-first fence held by admission and reopening. The
-- runtime has SELECT only; original/import owners hold mutation authority.
create function public.duet_retained_account_lock ()
    returns trigger
    language plpgsql
    as $$
begin
    perform
        id
    from
        public.users
    where
        id = coalesce(new.user_id, old.user_id)
    for update;
    if tg_op = 'UPDATE' then
        if new.user_id <> old.user_id then
            raise exception 'retained owner cannot change account';
        end if;
        if tg_table_name = 'insight_cycles' then
            if old.status = 'ready' and (to_jsonb (new) - 'corrected_insight_ids' - 'updated_at') is distinct from (to_jsonb (old) - 'corrected_insight_ids' - 'updated_at') then
                raise exception 'published observation is immutable';
            end if;
        end if;
    end if;
    if tg_op = 'DELETE' then
        return old;
    end if;
    return new;
end
$$;

-- Existing tables must expose the actual source boundary, not a made-up fallback.
-- An older incompatible import requires its original upgrade before adoption.
do $$
declare
    t text;
begin
    perform
        user_id,
        source_boundary_id,
        revision,
        projections
    from
        public.health_contexts
    limit 0;
    perform
        user_id,
        id,
        revision,
        state,
        accepted_instruction,
        source_message_ids
    from
        public.follow_ups
    limit 0;
    perform
        user_id,
        id,
        status,
        insights,
        supporting_facts,
        corrected_insight_ids
    from
        public.insight_cycles
    limit 0;
    foreach t in array array['health_contexts', 'follow_ups', 'insight_cycles'] loop
        execute format('create trigger duet_retained_lock before insert or update or delete on public.%I for each row execute function public.duet_retained_account_lock()', t);
        execute format('alter table public.%I enable row level security', t);
        execute format('alter table public.%I force row level security', t);
        execute format('create policy duet_retained_guard on public.%I as restrictive using(user_id=public.acting_account()) with check(user_id=public.acting_account())', t);
        execute format('create policy duet_retained_access on public.%I using(user_id=public.acting_account()) with check(user_id=public.acting_account())', t);
    end loop;
end
$$;
