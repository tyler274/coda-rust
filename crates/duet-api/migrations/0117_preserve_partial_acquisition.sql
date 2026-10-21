-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Reuse the existing page receipts to revisit incomplete pages. Patient URLs
-- stay encrypted; failures contain only source identities and fixed reasons.
alter table public.source_acquisition_pages
    add column request_ciphertext bytea,
    add column retried_at timestamptz,
    add column page_number integer not null default 0,
    add column failed_records jsonb not null default '[]'::jsonb
        check (jsonb_typeof(failed_records)='array');

alter table public.source_acquisition_units
    add column last_failure_kind text,
    add column last_failure_page integer check (last_failure_page > 0);

-- Existing attempts and receipts retain their state. Replaying historical
-- failures is a separate provider operation, never a schema-upgrade side effect.

insert into public.schema_migrations(version) values ('0117_preserve_partial_acquisition');
commit;
