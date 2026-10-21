-- Adopt original canonical identities and payloads; no destructive beta reset.
create table if not exists public.health_evidence (
 id uuid primary key, user_id uuid not null references public.users(id) on delete cascade,
 concept_id text not null, source_kind text not null, source_connection_id text,
 source_record_id text, measured_at timestamptz not null, received_at timestamptz not null,
 payload jsonb not null, created_at timestamptz not null default now(), updated_at timestamptz not null default now(),
 unique(user_id,id)
);
create table if not exists public.clinical_notes (
 id uuid primary key default gen_random_uuid(), user_id uuid not null references public.users(id) on delete cascade,
 source_id text, title text not null default '', preview text, body text, recorded_at timestamptz,
 source_label text not null default 'Health Records', created_at timestamptz not null default now(), unique(user_id,id)
);
-- Unknown source fields on notes survive round-trip without coercion into prose.
alter table public.clinical_notes add column if not exists duet_source_payload jsonb;
create table if not exists public.canonical_evidence_revisions (
 revision bigint generated always as identity primary key,
 user_id uuid not null references public.users(id) on delete cascade,
 evidence_id uuid, note_id uuid, payload jsonb not null check(jsonb_typeof(payload)='object'),
 content_sha256 text not null, retained_at timestamptz not null default now(),
 check(num_nonnulls(evidence_id,note_id)=1),
 foreign key(user_id,evidence_id) references public.health_evidence(user_id,id) on delete cascade,
 foreign key(user_id,note_id) references public.clinical_notes(user_id,id) on delete cascade
);
create index if not exists duet_revision_account on public.canonical_evidence_revisions(user_id,revision);
create table if not exists public.evidence_sync_changes (
 user_id uuid not null references public.users(id) on delete cascade,
 entity text not null check(entity in ('clinical_note','health_evidence')),
 record_id uuid not null, client_record_id text not null, revision bigint not null,
 deleted boolean not null, primary key(user_id,entity,record_id)
);
create index if not exists duet_changes_cursor on public.evidence_sync_changes(user_id,revision);
-- Preserve the current original bodies when adopting a pre-history installation.
-- Never invent older revisions or discard already-retained history.
insert into public.canonical_evidence_revisions(user_id,evidence_id,payload,content_sha256)
 select e.user_id,e.id,e.payload,encode(sha256(convert_to(e.payload::text,'UTF8')),'hex') from public.health_evidence e
 where not exists(select 1 from public.canonical_evidence_revisions r where r.user_id=e.user_id and r.evidence_id=e.id);
insert into public.canonical_evidence_revisions(user_id,note_id,payload,content_sha256)
 select n.user_id,n.id,coalesce(n.duet_source_payload,to_jsonb(n)-array['user_id','created_at','duet_source_payload']),
 encode(sha256(convert_to(coalesce(n.duet_source_payload,to_jsonb(n)-array['user_id','created_at','duet_source_payload'])::text,'UTF8')),'hex') from public.clinical_notes n
 where not exists(select 1 from public.canonical_evidence_revisions r where r.user_id=n.user_id and r.note_id=n.id);
update public.users u set evidence_revision=greatest(evidence_revision,1)
 where exists(select 1 from public.canonical_evidence_revisions r where r.user_id=u.id);
insert into public.evidence_sync_changes(user_id,entity,record_id,client_record_id,revision,deleted)
 select e.user_id,'health_evidence',e.id,coalesce(e.payload->>'id',e.id::text),u.evidence_revision,false
 from public.health_evidence e join public.users u on u.id=e.user_id
 union all select n.user_id,'clinical_note',n.id,n.id::text,u.evidence_revision,false from public.clinical_notes n join public.users u on u.id=n.user_id
 on conflict(user_id,entity,record_id) do nothing;
create table public.duet_evidence_ingestions (
 user_id uuid not null references public.users(id) on delete cascade,
 operation_id uuid not null, request_sha256 text not null, address jsonb not null,
 primary key(user_id,operation_id)
);
-- Only the canonical mutation owner appends revisions. Older trigger-based
-- writers are not supported concurrently with this workspace release.
drop trigger if exists health_evidence_retain_revision on public.health_evidence;
drop trigger if exists clinical_notes_retain_revision on public.clinical_notes;
-- Remove only the replaced legacy revision/change triggers, leaving unrelated
-- lifecycle/invalidation triggers intact. The new owner writes both atomically.
do $$ declare r record; begin
 for r in select c.relname,t.tgname from pg_trigger t join pg_class c on c.oid=t.tgrelid join pg_proc p on p.oid=t.tgfoid
 where c.oid in ('public.health_evidence'::regclass,'public.clinical_notes'::regclass)
 and p.proname in ('advance_evidence_revision','advance_updated_evidence_revision') loop
  execute format('drop trigger %I on public.%I',r.tgname,r.relname);
 end loop;
end $$;
do $$ declare t text; begin
 foreach t in array array['health_evidence','clinical_notes','canonical_evidence_revisions','evidence_sync_changes','duet_evidence_ingestions'] loop
  execute format('alter table public.%I enable row level security',t);
  execute format('create policy duet_owner_guard on public.%I as restrictive using(user_id=public.acting_account()) with check(user_id=public.acting_account())',t);
  execute format('create policy duet_owner_access on public.%I using(user_id=public.acting_account()) with check(user_id=public.acting_account())',t);
 end loop;
end $$;
