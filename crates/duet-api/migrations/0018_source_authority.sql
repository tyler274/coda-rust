-- ENG-162: source-faithful ingestion. Additive and in place; no row is rewritten.
-- A source that does not declare a date is stored as unknown, never as receipt time.
alter table public.health_evidence
    alter column measured_at drop not null;

-- Explicit source removal (the source itself deleted or retracted the record)
-- leaves current evidence but keeps retained revisions readable by prior
-- citations. Privacy erasure still deletes the rows and their revisions.
alter table public.evidence_sync_changes
    add column if not exists source_removed boolean not null default false;

alter table public.evidence_sync_changes
    drop constraint if exists duet_source_removed_is_deleted;

alter table public.evidence_sync_changes
    add constraint duet_source_removed_is_deleted check (not source_removed or deleted);
