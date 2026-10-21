-- Fence Genome workers so an interrupted or superseded attempt cannot publish.

begin;

alter table public.genome_imports
  add column attempt_token uuid default gen_random_uuid(),
  add column attempt_expires_at timestamptz default (now() + interval '10 minutes');

update public.genome_imports
set status = 'failed',
    failure_message = 'Genome processing was interrupted. Try again.',
    failed_at = now(),
    attempt_token = null,
    attempt_expires_at = null,
    updated_at = now()
where status in (
  'uploaded',
  'reading_file',
  'checking_reference_data',
  'building_profile'
);

update public.genome_imports
set attempt_token = null, attempt_expires_at = null
where status in ('ready', 'failed');

alter table public.genome_imports
  add constraint genome_imports_active_attempt_check check (
    status not in (
      'uploaded',
      'reading_file',
      'checking_reference_data',
      'building_profile'
    ) or (attempt_token is not null and attempt_expires_at is not null)
  );

drop trigger if exists genome_imports_notify on public.genome_imports;
drop function if exists public.notify_genome_import_progress();

insert into public.schema_migrations(version)
values ('0022_genome_attempt_fencing');

commit;
