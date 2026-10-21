-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Every canonical evidence or note mutation locks its account row
-- (advance_evidence_revision and advance_updated_evidence_revision take it
-- FOR UPDATE) before account-derived state. This is the same account-first
-- order as context publication, assignment claims and account deletion.
-- PostgreSQL fires same-event triggers in name order, so the browse
-- triggers ran first and locked the account's browse generation before the
-- account. A browse build holds the account's foreign-key lock and then
-- publishes that generation, so the two deadlocked (Codex R3). The rename
-- only orders the browse triggers after the account lock; no function changes.
alter trigger health_evidence_browse_insert on public.health_evidence
    rename to health_evidence_z_browse_insert;
alter trigger health_evidence_browse_update on public.health_evidence
    rename to health_evidence_z_browse_update;
alter trigger health_evidence_browse_delete on public.health_evidence
    rename to health_evidence_z_browse_delete;
alter trigger clinical_notes_browse_insert on public.clinical_notes
    rename to clinical_notes_z_browse_insert;
alter trigger clinical_notes_browse_update on public.clinical_notes
    rename to clinical_notes_z_browse_update;
alter trigger clinical_notes_browse_delete on public.clinical_notes
    rename to clinical_notes_z_browse_delete;

insert into public.schema_migrations(version) values('0109_browse_after_account_lock');
commit;
