begin;

-- Privacy/activity authority is account-owned. Foreground activity cannot
-- enable AI; existing accounts remain disabled until the disclosure control.
alter table public.users
    add column review_enabled boolean not null default false,
    add column review_control_revision bigint not null default 0,
    add column last_foreground_at timestamptz,
    add column review_join_until timestamptz,
    add column review_disclosure_version text,
    add column review_consent_at timestamptz,
    add column doctor_claim_token uuid,
    add column doctor_claim_until timestamptz;

create table public.patient_reviews (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references public.users(id) on delete cascade,
    opportunity_day date not null,
    kind text not null check(kind in ('initial','daily')),
    state text not null check(state in ('pending','running','completed','failed','cancelled')),
    evidence_revision bigint not null,
    history_cursor bigint not null default 0,
    input_fingerprint text,
    instruction_version text,
    continuation_number integer not null default 0 check(continuation_number>=0),
    continuation_requested boolean not null default false,
    context_snapshot_id uuid,
    control_revision bigint not null,
    prior_index_id uuid,
    claim_token uuid,
    deadline timestamptz,
    failure text check(failure in ('expired','source_changed','disabled','provider','budget','invalid_output')),
    created_at timestamptz not null default now(),
    completed_at timestamptz,
    unique(user_id,opportunity_day),
    unique(user_id,id)
);
create table public.patient_indexes (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references public.users(id) on delete cascade,
    review_id uuid not null,
    markdown text not null check(length(markdown) between 1 and 24000),
    model text not null,
    instruction_version text not null,
    evidence_revision bigint not null,
    source_record_ids uuid[] not null,
    source_kinds text[] not null,
    source_connection_ids text[] not null,
    read_receipts jsonb not null check(jsonb_typeof(read_receipts)='array'),
    fact_receipts jsonb not null check(jsonb_typeof(fact_receipts)='array'),
    superseded_at timestamptz,
    created_at timestamptz not null default now(),
    foreign key(user_id,review_id) references public.patient_reviews(user_id,id) on delete cascade,
    unique(review_id)
);
create index patient_indexes_current_idx on public.patient_indexes(user_id,created_at desc,id);
create index patient_indexes_dependencies_idx on public.patient_indexes using gin(source_record_ids);
revoke all on public.patient_reviews, public.patient_indexes from public;

-- A mechanical retention timestamp, not a prose promotion/merging lifecycle.
-- Preserve when an input first changed so even a long-lived valid index gets
-- a bounded reconciliation window; receipt-only churn never reaches INSERT.
create function public.supersede_patient_index_inputs() returns trigger
language plpgsql set search_path=pg_catalog,public as $$
declare owner_id uuid;
begin
    owner_id := case when TG_OP='DELETE' then old.user_id else new.user_id end;
    perform id from public.users where id=owner_id for update;
    update public.patient_indexes set superseded_at=now()
        where user_id=owner_id and superseded_at is null;
    return null;
end;
$$;
revoke all on function public.supersede_patient_index_inputs() from public;
create trigger canonical_revision_supersede_indexes after insert
on public.canonical_evidence_revisions for each row
execute function public.supersede_patient_index_inputs();
create trigger native_snapshot_insert_supersede_indexes after insert or delete
on public.health_contexts for each row
execute function public.supersede_patient_index_inputs();
create trigger native_snapshot_update_supersede_indexes after update
on public.health_contexts for each row when(old.snapshot_id is distinct from new.snapshot_id)
execute function public.supersede_patient_index_inputs();

-- Ordinary revisions supersede working context through version identity; only
-- withdrawal removes it. Inherited dependencies preserve deletion authority.
create function public.invalidate_patient_indexes() returns trigger
language plpgsql set search_path=pg_catalog,public as $$
declare owner_id uuid; record_id uuid;
begin
    owner_id := old.user_id;
    record_id := coalesce(old.evidence_id,old.note_id);
    -- Serialize invalidation with adoption. Without this fence an uncommitted
    -- source write could delete old indexes, then miss a newly adopted one.
    perform id from public.users where id=owner_id for update;
    delete from public.patient_indexes where user_id=owner_id
      and source_record_ids @> array[record_id];
    return null;
end;
$$;
revoke all on function public.invalidate_patient_indexes() from public;
create trigger canonical_revision_invalidate_indexes after delete
on public.canonical_evidence_revisions for each row
execute function public.invalidate_patient_indexes();

insert into public.schema_migrations(version) values('0017_patient_review_lifecycle');
commit;
