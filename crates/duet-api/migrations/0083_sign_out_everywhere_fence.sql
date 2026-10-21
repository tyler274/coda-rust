-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Sign out everywhere's fence: every session that signed in before this
-- instant is revoked. Identity Platform refuses those sessions a new token,
-- but a token it already issued stays valid for up to an hour, so each write
-- that gives an iPhone's session server state of its own (its push
-- registration, the sharing key it holds) checks this under the account lock
-- in the same transaction (`app/services/account_sessions.py`).
--
-- Online: one nullable column without a default, which the serving runtime
-- never reads or writes.
alter table public.users add column sessions_revoked_before timestamptz;

insert into public.schema_migrations(version) values('0083_sign_out_everywhere_fence');
commit;
