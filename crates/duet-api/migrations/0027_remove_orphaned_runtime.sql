begin;

-- Google Health no longer has an authorization or refresh runtime. End any
-- authority that predates this removal through the same revision fence used by
-- an explicit disconnect. The authorization trigger clears unfinished
-- acquisition work while the imported evidence and its retained revisions stay
-- patient-owned history.
create temporary table affected_google_health_users (
    user_id uuid primary key
) on commit drop;

insert into affected_google_health_users(user_id)
select distinct user_id
from public.source_connections
where provider = 'googleHealth'
  and (
      status <> 'disconnected'
      or credentials_ciphertext is not null
      or cardinality(granted_scopes) > 0
      or oauth_state_hash is not null
      or oauth_state_expires_at is not null
      or last_failure_kind is not null
      or last_error_code is not null
      or retry_not_before is not null
      or disconnected_at is null
  );

update public.source_connections
set status = 'disconnected',
    credentials_ciphertext = null,
    granted_scopes = '{}'::text[],
    oauth_state_hash = null,
    oauth_state_expires_at = null,
    last_failure_kind = null,
    last_error_code = null,
    retry_not_before = null,
    disconnected_at = coalesce(disconnected_at, now()),
    authorization_revision = authorization_revision + 1,
    updated_at = now()
where provider = 'googleHealth'
  and user_id in (select user_id from affected_google_health_users);

-- Runtime disconnect advances this patient fence once, independently of the
-- number of source rows or retained receipts.
update public.users
set evidence_revision = evidence_revision + 1,
    updated_at = now()
where id in (select user_id from affected_google_health_users);

-- Learning cards and WHO ingestion have no product surface or runtime owner.
-- Remove their database state in dependency order rather than preserving a
-- dormant catalogue beside the patient-owned evidence model.
drop table public.learning_schedule;
drop table public.learning_items;
alter table public.learning_sources
    drop constraint learning_sources_current_revision_fk;
drop table public.learning_source_revisions;
drop table public.learning_sources;
drop table public.learning_ingestion_runs;

insert into public.schema_migrations(version)
values ('0027_remove_orphaned_runtime');

commit;
