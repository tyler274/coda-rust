-- duet-release: maintenance
begin;

-- Runtime owners are stopped by the maintenance release. Preserve the existing
-- explicit-retry rule for interrupted work; only unstarted live uploads queue.
update public.genome_imports set status='failed',
  failure_message='Genome processing was interrupted. Try again.', failed_at=now(),
  attempt_token=null, attempt_expires_at=null, updated_at=now()
where status in ('reading_file','checking_reference_data','building_profile')
   or (status='uploaded' and attempt_expires_at<=now());

alter table public.genome_imports drop constraint genome_imports_active_attempt_check;
alter table public.genome_imports
  alter column attempt_expires_at drop default,
  add column delivery_generation integer not null default 0 check (delivery_generation>=0),
  add column delivery_until timestamptz;
update public.genome_imports set attempt_expires_at=null where status='uploaded';
alter table public.genome_imports add constraint genome_imports_active_attempt_check check (
  (status='uploaded' and attempt_token is not null and attempt_expires_at is null)
  or (status in ('reading_file','checking_reference_data','building_profile')
      and attempt_token is not null and attempt_expires_at is not null)
  or status in ('ready','failed')
);
create index genome_import_delivery_pending on public.genome_imports(updated_at)
  where status='uploaded';

insert into public.schema_migrations(version) values ('0052_genome_queued_delivery');
commit;
