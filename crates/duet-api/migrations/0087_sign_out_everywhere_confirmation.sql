-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- The fence Identity Platform has been read back holding: every session that
-- signed in before it is refused a new token there. Sign out everywhere is
-- confirmed once this reaches `sessions_revoked_before` and pending until
-- then; the backend finishes a pending one itself, from a later sign out
-- everywhere or the recovery job (`app/services/account_sessions.py`). It
-- only moves forward.
--
-- When the recovery job last began finishing this account's pending sign out
-- everywhere. Each run gives every pending account its own bounded attempt,
-- oldest attempt first, so one that keeps failing goes behind the others and
-- none is starved.
--
-- Online: two nullable columns without a default, which the serving runtime
-- never reads or writes. A fence the serving runtime confirms leaves the
-- confirmation empty, so the new recovery job reads Identity Platform back and
-- records it, writing nothing there when it already holds the fence.
alter table public.users
  add column sessions_revoked_confirmed_before timestamptz,
  add column sessions_revocation_attempted_at timestamptz;

insert into public.schema_migrations(version) values('0087_sign_out_everywhere_confirmation');
commit;
