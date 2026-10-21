begin;

-- A mechanical snapshot fence, not a second evidence store. Statement-level
-- triggers cover every canonical writer, including source/account deletion.
alter table public.users add column evidence_revision bigint not null default 0;
alter table public.users add constraint users_evidence_revision_nonnegative
    check (evidence_revision >= 0);

create function public.advance_evidence_revision() returns trigger
language plpgsql set search_path = pg_catalog, public as $$
begin
    update public.users set evidence_revision = evidence_revision + 1
    where id in (select distinct user_id from changed_evidence);
    return null;
end;
$$;

create function public.advance_updated_evidence_revision() returns trigger
language plpgsql set search_path = pg_catalog, public as $$
begin
    update public.users set evidence_revision = evidence_revision + 1
    where id in (
        select user_id from old_evidence union select user_id from changed_evidence
    );
    return null;
end;
$$;

revoke all on function public.advance_evidence_revision() from public;
revoke all on function public.advance_updated_evidence_revision() from public;

create trigger health_evidence_insert_revision after insert on public.health_evidence
referencing new table as changed_evidence for each statement
execute function public.advance_evidence_revision();
create trigger health_evidence_update_revision after update on public.health_evidence
referencing old table as old_evidence new table as changed_evidence for each statement
execute function public.advance_updated_evidence_revision();
create trigger health_evidence_delete_revision after delete on public.health_evidence
referencing old table as changed_evidence for each statement
execute function public.advance_evidence_revision();
create trigger clinical_notes_insert_revision after insert on public.clinical_notes
referencing new table as changed_evidence for each statement
execute function public.advance_evidence_revision();
create trigger clinical_notes_update_revision after update on public.clinical_notes
referencing old table as old_evidence new table as changed_evidence for each statement
execute function public.advance_updated_evidence_revision();
create trigger clinical_notes_delete_revision after delete on public.clinical_notes
referencing old table as changed_evidence for each statement
execute function public.advance_evidence_revision();

create index health_evidence_user_id_page_idx on public.health_evidence(user_id, id);
create index clinical_notes_user_id_page_idx on public.clinical_notes(user_id, id);

insert into public.schema_migrations(version) values ('0012_evidence_read_revision');
commit;
