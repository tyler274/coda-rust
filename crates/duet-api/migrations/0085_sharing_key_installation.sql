-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- The iPhone installation holding the account's active sharing key: the one
-- iPhone Sharing wakes (`app/services/sharing_push.py`). The iPhone sends its
-- installation id (its push registration's id) when it registers, takes over
-- or registers again the key, and the key's holder registers it again at its
-- next sharing sync while this is not its own installation, so a key made
-- before this release is linked then.
--
-- Online: one nullable column without a default, which the serving runtime
-- never reads or writes; its inserts leave it empty.
alter table public.sharing_devices add column installation_id uuid;

insert into public.schema_migrations(version) values('0085_sharing_key_installation');
commit;
