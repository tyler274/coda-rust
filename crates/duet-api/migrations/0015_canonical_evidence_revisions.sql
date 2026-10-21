begin;

-- Revisions are subordinate to the live canonical receipt, never an archive
-- that can survive source withdrawal. One owner covers both retained entities.
alter table public.health_evidence add constraint health_evidence_owner_identity unique(user_id,id);
alter table public.clinical_notes add constraint clinical_notes_owner_identity unique(user_id,id);
create table public.canonical_evidence_revisions (
    revision bigint generated always as identity primary key,
    user_id uuid not null references public.users(id) on delete cascade,
    evidence_id uuid,
    note_id uuid,
    payload jsonb not null check(jsonb_typeof(payload)='object'),
    content_sha256 text not null check(length(content_sha256)=64),
    retained_at timestamptz not null default now(),
    constraint canonical_revision_one_entity check(num_nonnulls(evidence_id,note_id)=1),
    foreign key(user_id,evidence_id) references public.health_evidence(user_id,id) on delete cascade,
    foreign key(user_id,note_id) references public.clinical_notes(user_id,id) on delete cascade
);
create index canonical_revision_evidence_idx on public.canonical_evidence_revisions(user_id,evidence_id,revision desc)
    where evidence_id is not null;
create index canonical_revision_note_idx on public.canonical_evidence_revisions(user_id,note_id,revision desc)
    where note_id is not null;
create index canonical_revision_inventory_idx on public.canonical_evidence_revisions(user_id,revision);
-- Follow the established GCP privilege split. No policy-free RLS or new
-- SECURITY DEFINER boundary; account scope is mandatory in the typed reader.
revoke all on public.canonical_evidence_revisions from public;

-- Only receipt/projection fields are excluded. No code/name/clinical-value
-- normalization, no semantic similarity, and no global content deduplication.
create function public.canonical_evidence_content(value jsonb) returns jsonb
language sql immutable strict set search_path=pg_catalog,public as $$
    select value - array['received_at','source_synced_at','catalog']::text[];
$$;
create function public.retain_health_evidence_revision() returns trigger
language plpgsql set search_path=pg_catalog,public as $$
declare content jsonb;
begin
    content := public.canonical_evidence_content(new.payload);
    if TG_OP='UPDATE' and content=public.canonical_evidence_content(old.payload) then
        return null;
    end if;
    insert into public.canonical_evidence_revisions(user_id,evidence_id,payload,content_sha256)
    values(new.user_id,new.id,new.payload,encode(public.digest(content::text,'sha256'),'hex'));
    return null;
end;
$$;
create function public.retain_clinical_note_revision() returns trigger
language plpgsql set search_path=pg_catalog,public as $$
declare content jsonb;
begin
    content := to_jsonb(new)-array['user_id','created_at']::text[];
    if TG_OP='UPDATE' and content=(to_jsonb(old)-array['user_id','created_at']::text[]) then
        return null;
    end if;
    insert into public.canonical_evidence_revisions(user_id,note_id,payload,content_sha256)
    values(new.user_id,new.id,content,encode(public.digest(content::text,'sha256'),'hex'));
    return null;
end;
$$;
revoke all on function public.canonical_evidence_content(jsonb) from public;
revoke all on function public.retain_health_evidence_revision() from public;
revoke all on function public.retain_clinical_note_revision() from public;

-- Preserve only what exists now. Do not invent older versions or source dates.
insert into public.canonical_evidence_revisions(user_id,evidence_id,payload,content_sha256)
select user_id,id,payload,encode(public.digest(public.canonical_evidence_content(payload)::text,'sha256'),'hex')
from public.health_evidence order by id;
insert into public.canonical_evidence_revisions(user_id,note_id,payload,content_sha256)
select user_id,id,to_jsonb(n)-array['user_id','created_at']::text[],
    encode(public.digest((to_jsonb(n)-array['user_id','created_at']::text[])::text,'sha256'),'hex')
from public.clinical_notes n order by id;
create trigger health_evidence_retain_revision after insert or update on public.health_evidence
for each row execute function public.retain_health_evidence_revision();
create trigger clinical_notes_retain_revision after insert or update on public.clinical_notes
for each row execute function public.retain_clinical_note_revision();

insert into public.schema_migrations(version) values('0015_canonical_evidence_revisions');
commit;
