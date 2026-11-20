-- For my doctor (Shared): marks from You, free-text notes (standalone or record-linked),
-- immutable sends to a connected practice via front-door, and source-removal notices.
create table if not exists public.duet_for_my_doctor_marks (
    id uuid primary key,
    user_id uuid not null references public.users (id) on delete cascade,
    target_kind text not null check (target_kind in ('record', 'value', 'reading', 'trend')),
    target_key text not null check (char_length(btrim(target_key)) > 0),
    record_id uuid,
    entity text check (entity is null or entity in ('health_evidence', 'clinical_note')),
    record_revision integer,
    marked_revision integer,
    title text not null check (char_length(btrim(title)) > 0),
    summary_text text not null default '',
    range_label text,
    source_label text,
    source_corrected boolean not null default false,
    correction_date timestamptz,
    payload jsonb not null default '{}'::jsonb,
    marked_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    unique (user_id, target_key)
);

create index if not exists duet_for_my_doctor_marks_user_record on public.duet_for_my_doctor_marks (user_id, record_id)
where
    record_id is not null;

create table if not exists public.duet_for_my_doctor_notes (
    id uuid primary key,
    user_id uuid not null references public.users (id) on delete cascade,
    body text not null check (char_length(btrim(body)) > 0),
    linked_record_id uuid,
    linked_record_title text,
    linked_entity text check (linked_entity is null or linked_entity in ('health_evidence', 'clinical_note')),
    version bigint not null default 1,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create index if not exists duet_for_my_doctor_notes_user_updated on public.duet_for_my_doctor_notes (user_id, updated_at desc, id);

create table if not exists public.duet_for_my_doctor_sends (
    id uuid primary key,
    user_id uuid not null references public.users (id) on delete cascade,
    share_key text not null check (char_length(btrim(share_key)) > 0),
    practice_id text not null check (char_length(btrim(practice_id)) > 0),
    practice_name text not null check (char_length(btrim(practice_name)) > 0),
    patient_id text not null check (char_length(btrim(patient_id)) > 0),
    title text not null check (char_length(btrim(title)) > 0),
    summary_text text not null,
    snapshot jsonb not null default '{}'::jsonb,
    mark_ids uuid[] not null default '{}',
    note_ids uuid[] not null default '{}',
    front_door_share_id text,
    standing text not null default 'sent' check (standing in ('sent', 'received')),
    sent_at timestamptz not null default now(),
    received_at timestamptz,
    connection_ended_at timestamptz,
    unique (user_id, share_key)
);

create index if not exists duet_for_my_doctor_sends_user_sent on public.duet_for_my_doctor_sends (user_id, sent_at desc, id);

create table if not exists public.duet_for_my_doctor_notices (
    id uuid primary key,
    user_id uuid not null references public.users (id) on delete cascade,
    title text not null,
    message text not null,
    dismissed_at timestamptz,
    created_at timestamptz not null default now()
);

create index if not exists duet_for_my_doctor_notices_active on public.duet_for_my_doctor_notices (user_id, created_at desc)
where
    dismissed_at is null;

do $$
declare
    t text;
begin
    foreach t in array array['duet_for_my_doctor_marks', 'duet_for_my_doctor_notes', 'duet_for_my_doctor_sends', 'duet_for_my_doctor_notices'] loop
        execute format('alter table public.%I enable row level security', t);
        execute format('alter table public.%I force row level security', t);
        execute format('create policy account_guard on public.%I as restrictive using (user_id = public.acting_account()) with check (user_id = public.acting_account())', t);
        execute format('create policy account_isolation on public.%I using (user_id = public.acting_account()) with check (user_id = public.acting_account())', t);
    end loop;
end
$$;
