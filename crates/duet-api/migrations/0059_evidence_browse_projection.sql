-- duet-release: online
begin;

-- A replaceable read projection, never another evidence owner. Canonical
-- evidence and notes remain authoritative. The generation changes only when
-- material that can affect clinical-library or lab grouping changes, so daily
-- Apple aggregates do not invalidate this projection.
create table public.evidence_browse_generations (
    user_id uuid primary key references public.users(id) on delete cascade,
    generation bigint not null check (generation >= 0),
    projected_generation bigint not null default 0,
    -- A projection older than this fence may contain evidence that was
    -- corrected or removed. Additive writes leave the fence alone so the last
    -- published view remains usable while its replacement is prepared.
    minimum_readable_generation bigint not null default 0,
    projected_revision bigint not null default 0 check (projected_revision >= 0),
    projected_catalog_revision text not null default '',
    projected_health_records_generation bigint not null default 0
        check (projected_health_records_generation >= 0),
    constraint evidence_browse_projected_generation_check check (
        projected_generation >= -1 and projected_generation <= generation
    ),
    constraint evidence_browse_readable_generation_check check (
        minimum_readable_generation >= 0
        and minimum_readable_generation <= generation
    )
);

create function public.is_evidence_browse_input(concept text, body jsonb)
returns boolean language sql immutable strict
set search_path = pg_catalog,public as $$
    select body->'clinical_record' is not null
        or concept like 'clinical:%'
        or concept like 'medication:%'
        or concept like 'lab:%';
$$;

-- Existing accounts with clinical material need one initial background build.
-- Empty accounts are already complete and become pending only when a relevant
-- source row arrives, avoiding a pointless worker task during account setup.
insert into public.evidence_browse_generations(
    user_id,generation,projected_generation,minimum_readable_generation,
    projected_revision,projected_catalog_revision,
    projected_health_records_generation
)
select u.id,0,case when
    exists(select 1 from public.health_evidence r where r.user_id=u.id
           and public.is_evidence_browse_input(r.concept_id,r.payload))
    or exists(select 1 from public.clinical_notes n where n.user_id=u.id)
    then -1 else 0 end,
    0,u.evidence_revision,
    coalesce((select id from public.clinical_catalog_releases where status='active'),''),
    u.health_records_generation
from public.users u;

create function public.initialize_evidence_browse_generation() returns trigger
language plpgsql set search_path = pg_catalog,public as $$
begin
    insert into public.evidence_browse_generations(
        user_id,generation,projected_generation,minimum_readable_generation,
        projected_revision,projected_catalog_revision,
        projected_health_records_generation
    ) values(
        new.id,0,0,0,new.evidence_revision,
        coalesce((select id from public.clinical_catalog_releases where status='active'),''),
        new.health_records_generation
    ) on conflict do nothing;
    return null;
end;
$$;
create trigger users_evidence_browse_generation after insert on public.users
for each row execute function public.initialize_evidence_browse_generation();

create function public.advance_health_records_browse_generation() returns trigger
language plpgsql set search_path = pg_catalog,public as $$
begin
    if old.health_records_generation is distinct from new.health_records_generation then
        update public.evidence_browse_generations
        set generation=generation+1 where user_id=new.id;
    end if;
    return null;
end;
$$;
create trigger users_health_records_browse_generation
after update of health_records_generation on public.users
for each row execute function public.advance_health_records_browse_generation();

create function public.advance_catalog_browse_generation() returns trigger
language plpgsql set search_path = pg_catalog,public as $$
declare
    affects_active boolean;
begin
    affects_active := case tg_op
        when 'INSERT' then new.status='active'
        when 'DELETE' then old.status='active'
        else old.status='active' or new.status='active'
    end;
    if affects_active then
        update public.evidence_browse_generations generation
        set generation=generation.generation+1
        where exists(select 1 from public.health_evidence record
                     where record.user_id=generation.user_id
                       and public.is_evidence_browse_input(
                           record.concept_id,record.payload))
           or exists(select 1 from public.clinical_notes note
                     where note.user_id=generation.user_id);
    end if;
    return null;
end;
$$;
create trigger clinical_catalog_browse_insert
after insert on public.clinical_catalog_releases
for each row execute function public.advance_catalog_browse_generation();
create trigger clinical_catalog_browse_update
after update of status on public.clinical_catalog_releases
for each row execute function public.advance_catalog_browse_generation();
create trigger clinical_catalog_browse_delete
after delete on public.clinical_catalog_releases
for each row execute function public.advance_catalog_browse_generation();

create table public.evidence_library_projections (
    user_id uuid not null references public.users(id) on delete cascade,
    browse_generation bigint not null,
    catalog_revision text not null,
    health_records_generation bigint not null,
    scope text not null check (scope in ('summary','sourceSummary','completeRecord')),
    source_kind text not null,
    total_count integer not null check (total_count >= 0),
    counts jsonb not null check (jsonb_typeof(counts) = 'object'),
    latest_dates jsonb not null check (jsonb_typeof(latest_dates) = 'object'),
    built_at timestamptz not null default clock_timestamp(),
    primary key (
        user_id,browse_generation,catalog_revision,
        health_records_generation,scope,source_kind
    )
);

create table public.evidence_library_projection_entries (
    user_id uuid not null,
    browse_generation bigint not null,
    catalog_revision text not null,
    health_records_generation bigint not null,
    scope text not null,
    source_kind text not null,
    position integer not null check (position >= 0),
    entry_id text not null,
    category text not null,
    entity text not null check (entity in ('clinical_note','health_evidence')),
    record_id uuid not null,
    entry jsonb not null check (jsonb_typeof(entry) = 'object'),
    search_text text not null,
    primary key (
        user_id,browse_generation,catalog_revision,
        health_records_generation,scope,source_kind,position
    ),
    unique (
        user_id,browse_generation,catalog_revision,
        health_records_generation,scope,source_kind,entry_id
    ),
    foreign key (
        user_id,browse_generation,catalog_revision,
        health_records_generation,scope,source_kind
    ) references public.evidence_library_projections(
        user_id,browse_generation,catalog_revision,
        health_records_generation,scope,source_kind
    ) on delete cascade
);
create index evidence_library_projection_category
    on public.evidence_library_projection_entries(
        user_id,browse_generation,catalog_revision,
        health_records_generation,scope,source_kind,category,position
    );

create table public.evidence_library_projection_members (
    user_id uuid not null,
    browse_generation bigint not null,
    catalog_revision text not null,
    health_records_generation bigint not null,
    scope text not null,
    source_kind text not null,
    entry_id text not null,
    evidence_id uuid not null,
    position integer not null check (position >= 0),
    primary key (
        user_id,browse_generation,catalog_revision,
        health_records_generation,scope,source_kind,entry_id,evidence_id
    ),
    foreign key (
        user_id,browse_generation,catalog_revision,
        health_records_generation,scope,source_kind,entry_id
    ) references public.evidence_library_projection_entries(
        user_id,browse_generation,catalog_revision,
        health_records_generation,scope,source_kind,entry_id
    ) on delete cascade
);
create index evidence_library_projection_member
    on public.evidence_library_projection_members(
        user_id,browse_generation,catalog_revision,
        health_records_generation,scope,source_kind,evidence_id
    );

create table public.evidence_library_projection_codings (
    user_id uuid not null,
    browse_generation bigint not null,
    catalog_revision text not null,
    health_records_generation bigint not null,
    scope text not null,
    source_kind text not null,
    entry_id text not null,
    system text not null,
    code text not null,
    version text not null,
    primary key (
        user_id,browse_generation,catalog_revision,
        health_records_generation,scope,source_kind,
        entry_id,system,code,version
    ),
    foreign key (
        user_id,browse_generation,catalog_revision,
        health_records_generation,scope,source_kind,entry_id
    ) references public.evidence_library_projection_entries(
        user_id,browse_generation,catalog_revision,
        health_records_generation,scope,source_kind,entry_id
    ) on delete cascade
);
create index evidence_library_projection_coding
    on public.evidence_library_projection_codings(
        user_id,browse_generation,catalog_revision,
        health_records_generation,scope,source_kind,system,code,version
    );

create table public.evidence_lab_projections (
    user_id uuid not null references public.users(id) on delete cascade,
    browse_generation bigint not null,
    catalog_revision text not null,
    health_records_generation bigint not null,
    built_at timestamptz not null default clock_timestamp(),
    primary key (
        user_id,browse_generation,catalog_revision,health_records_generation
    )
);

create table public.evidence_lab_projection_histories (
    user_id uuid not null,
    browse_generation bigint not null,
    catalog_revision text not null,
    health_records_generation bigint not null,
    history_id text not null,
    title text not null,
    latest jsonb not null check (jsonb_typeof(latest) = 'object'),
    total_count integer not null check (total_count > 0),
    source_labels text[] not null,
    selected_series jsonb,
    primary key (
        user_id,browse_generation,catalog_revision,
        health_records_generation,history_id
    ),
    foreign key (
        user_id,browse_generation,catalog_revision,health_records_generation
    ) references public.evidence_lab_projections(
        user_id,browse_generation,catalog_revision,health_records_generation
    ) on delete cascade
);

create table public.evidence_lab_projection_rows (
    user_id uuid not null,
    browse_generation bigint not null,
    catalog_revision text not null,
    health_records_generation bigint not null,
    history_id text not null,
    position integer not null check (position >= 0),
    observation_id uuid not null,
    observation jsonb not null check (jsonb_typeof(observation) = 'object'),
    backing_evidence_id uuid not null,
    canonical_record_id uuid not null,
    series_id text,
    primary key (
        user_id,browse_generation,catalog_revision,
        health_records_generation,history_id,position
    ),
    foreign key (
        user_id,browse_generation,catalog_revision,
        health_records_generation,history_id
    ) references public.evidence_lab_projection_histories(
        user_id,browse_generation,catalog_revision,
        health_records_generation,history_id
    ) on delete cascade
);
create index evidence_lab_projection_observation
    on public.evidence_lab_projection_rows(
        user_id,browse_generation,catalog_revision,
        health_records_generation,history_id,observation_id
    );

create function public.advance_evidence_browse_generation() returns trigger
language plpgsql set search_path = pg_catalog,public as $$
begin
    insert into public.evidence_browse_generations(
        user_id,generation,projected_generation,minimum_readable_generation
    )
    select distinct records.user_id,1,-1,0 from changed_records records
    join public.users account on account.id=records.user_id
    on conflict(user_id) do update
        set generation=public.evidence_browse_generations.generation+1;
    return null;
end;
$$;

create function public.invalidate_updated_evidence_browse_generation() returns trigger
language plpgsql set search_path = pg_catalog,public as $$
begin
    insert into public.evidence_browse_generations(
        user_id,generation,projected_generation,minimum_readable_generation
    )
    select distinct user_id,1,-1,1 from (
        select user_id from old_records
        union all select user_id from changed_records
    ) records join public.users account on account.id=records.user_id
    on conflict(user_id) do update
        set generation=public.evidence_browse_generations.generation+1,
            minimum_readable_generation=
                public.evidence_browse_generations.generation+1;
    return null;
end;
$$;

create function public.invalidate_deleted_evidence_browse_generation() returns trigger
language plpgsql set search_path = pg_catalog,public as $$
begin
    insert into public.evidence_browse_generations(
        user_id,generation,projected_generation,minimum_readable_generation
    )
    select distinct records.user_id,1,-1,1 from old_records records
    join public.users account on account.id=records.user_id
    on conflict(user_id) do update
        set generation=public.evidence_browse_generations.generation+1,
            minimum_readable_generation=
                public.evidence_browse_generations.generation+1;
    return null;
end;
$$;

create function public.advance_clinical_evidence_browse_generation() returns trigger
language plpgsql set search_path = pg_catalog,public as $$
begin
    insert into public.evidence_browse_generations(
        user_id,generation,projected_generation,minimum_readable_generation
    )
    select records.user_id,1,-1,
           case when bool_or(
               nullif(records.payload->>'corrects_evidence_id','') is not null
               and coalesce(records.payload->>'verification','')<>'staged'
           ) then 1 else 0 end
    from changed_records records
    join public.users account on account.id=records.user_id
    where public.is_evidence_browse_input(concept_id,payload)
    group by records.user_id
    on conflict(user_id) do update
        set generation=public.evidence_browse_generations.generation+1,
            minimum_readable_generation=case
                when excluded.minimum_readable_generation=1 then
                    public.evidence_browse_generations.generation+1
                else public.evidence_browse_generations.minimum_readable_generation
            end;
    return null;
end;
$$;

create function public.invalidate_updated_clinical_evidence_browse_generation() returns trigger
language plpgsql set search_path = pg_catalog,public as $$
begin
    insert into public.evidence_browse_generations(
        user_id,generation,projected_generation,minimum_readable_generation
    )
    select distinct user_id,1,-1,1 from (
        select user_id,concept_id,payload from old_records
        union all select user_id,concept_id,payload from changed_records
    ) records join public.users account on account.id=records.user_id
    where public.is_evidence_browse_input(concept_id,payload)
    on conflict(user_id) do update
        set generation=public.evidence_browse_generations.generation+1,
            minimum_readable_generation=
                public.evidence_browse_generations.generation+1;
    return null;
end;
$$;

create function public.invalidate_deleted_clinical_evidence_browse_generation() returns trigger
language plpgsql set search_path = pg_catalog,public as $$
begin
    insert into public.evidence_browse_generations(
        user_id,generation,projected_generation,minimum_readable_generation
    )
    select distinct records.user_id,1,-1,1 from old_records records
    join public.users account on account.id=records.user_id
    where public.is_evidence_browse_input(concept_id,payload)
    on conflict(user_id) do update
        set generation=public.evidence_browse_generations.generation+1,
            minimum_readable_generation=
                public.evidence_browse_generations.generation+1;
    return null;
end;
$$;

create trigger health_evidence_browse_insert after insert on public.health_evidence
referencing new table as changed_records for each statement
execute function public.advance_clinical_evidence_browse_generation();
create trigger health_evidence_browse_update after update on public.health_evidence
referencing old table as old_records new table as changed_records for each statement
execute function public.invalidate_updated_clinical_evidence_browse_generation();
create trigger health_evidence_browse_delete after delete on public.health_evidence
referencing old table as old_records for each statement
execute function public.invalidate_deleted_clinical_evidence_browse_generation();
create trigger clinical_notes_browse_insert after insert on public.clinical_notes
referencing new table as changed_records for each statement
execute function public.advance_evidence_browse_generation();
create trigger clinical_notes_browse_update after update on public.clinical_notes
referencing old table as old_records new table as changed_records for each statement
execute function public.invalidate_updated_evidence_browse_generation();
create trigger clinical_notes_browse_delete after delete on public.clinical_notes
referencing old table as old_records for each statement
execute function public.invalidate_deleted_evidence_browse_generation();

revoke all on public.evidence_browse_generations,
    public.evidence_library_projections,
    public.evidence_library_projection_entries,
    public.evidence_library_projection_members,
    public.evidence_library_projection_codings,
    public.evidence_lab_projections,
    public.evidence_lab_projection_histories,
    public.evidence_lab_projection_rows from public;
revoke all on function public.is_evidence_browse_input(text,jsonb),
    public.initialize_evidence_browse_generation(),
    public.advance_health_records_browse_generation(),
    public.advance_catalog_browse_generation(),
    public.advance_evidence_browse_generation(),
    public.invalidate_updated_evidence_browse_generation(),
    public.invalidate_deleted_evidence_browse_generation(),
    public.advance_clinical_evidence_browse_generation(),
    public.invalidate_updated_clinical_evidence_browse_generation(),
    public.invalidate_deleted_clinical_evidence_browse_generation() from public;

insert into public.schema_migrations(version)
values ('0059_evidence_browse_projection');
commit;
