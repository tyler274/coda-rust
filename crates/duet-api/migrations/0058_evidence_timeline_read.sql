-- duet-release: online
begin;
-- Date-keyset reads must not sort an account's full evidence history before
-- returning one visible page. Clinical notes already have the matching index.
create index health_evidence_account_date_idx
    on public.health_evidence(user_id,measured_at desc nulls last,id);
insert into public.schema_migrations(version) values ('0058_evidence_timeline_read');
commit;
