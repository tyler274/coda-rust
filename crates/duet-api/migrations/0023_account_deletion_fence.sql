-- Prevent an already-authorized account ensure from reopening product state
-- while the external identity is being irreversibly deleted.

begin;

create table public.account_deletion_fences (
  auth_user_id text primary key,
  started_at timestamptz not null default clock_timestamp(),
  identity_deleted_at timestamptz,
  constraint account_deletion_fences_auth_user_id_check check (
    length(auth_user_id) between 1 and 128
  )
);

insert into public.schema_migrations(version)
values ('0023_account_deletion_fence');

commit;
