-- duet-release: online
begin;
-- Reuse the canonical change membership for exact client identities. No second
-- content store or per-phone inventory is required.
create index evidence_sync_changes_client_identity_idx
    on public.evidence_sync_changes(user_id,entity,lower(client_record_id))
    where not deleted;
create index clinical_notes_account_date_idx
    on public.clinical_notes(user_id,recorded_at desc nulls last,id);
create index health_evidence_account_concept_date_idx
    on public.health_evidence(user_id,concept_id,measured_at desc,id);
create index health_evidence_current_correction_idx
    on public.health_evidence(user_id,lower(payload->>'corrects_evidence_id'))
    where payload->>'verification'<>'staged';
insert into public.schema_migrations(version) values ('0056_evidence_addressed_reads');
commit;
