begin;

-- Disconnect ends future source access without erasing the person's imported
-- history. The connection row remains the lifecycle owner for frozen source
-- provenance and for a later authorization to the same source identity.
alter table public.source_connections
    add column disconnected_at timestamptz;

alter table public.genome_imports
    add column disconnected_at timestamptz;

alter table public.source_connections
    drop constraint source_connections_status_check;
alter table public.source_connections
    add constraint source_connections_status_check
    check (status in (
        'pending', 'connected', 'reauthorization_required', 'disconnected'
    ));

-- Canonical revisions belong to the account, not to the mutable current-row
-- projection. Current source replacement may remove a live receipt while its
-- literal prior versions remain available until account deletion.
alter table public.canonical_evidence_revisions
    drop constraint canonical_evidence_revisions_user_id_evidence_id_fkey;
alter table public.canonical_evidence_revisions
    drop constraint canonical_evidence_revisions_user_id_note_id_fkey;

-- Authorization changes still fence and clear unfinished provider work. They
-- no longer delete imported evidence or any patient-owned derived artifacts.
create or replace function public.invalidate_acquisition_authorization()
returns trigger
language plpgsql set search_path=pg_catalog,public as $$
begin
    update public.source_connections
    set acquisition_claim_unit=null,
        acquisition_claim_version=null,
        acquisition_claim_until=null
    where id=new.id;

    delete from public.source_acquisition_staging where unit_id in (
        select id from public.source_acquisition_units where connection_id=new.id
    );
    delete from public.source_acquisition_pages where unit_id in (
        select id from public.source_acquisition_units where connection_id=new.id
    );
    update public.source_acquisition_units
    set state='unavailable',
        claim_until=null,
        continuation_ciphertext=null,
        pages=0,
        terminal=false,
        last_completed_at=null,
        published_count=0,
        retry_count=0,
        continuation_restarts=0,
        not_before=now(),
        last_failure_code='authorization_changed',
        updated_at=now()
    where connection_id=new.id;
    return null;
end;
$$;

revoke all on function public.invalidate_acquisition_authorization() from public;

insert into public.schema_migrations(version)
values ('0024_retained_source_history');

commit;
