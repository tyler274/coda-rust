-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Push withdrawal, key first. The iPhone binds its withdrawal key's SHA-256 to
-- the account's row (`withdrawal_key_hash`, migration 0075) before it sends a
-- token, and a token is registered only with the key the row holds. A
-- registration without a key cannot be withdrawn by a signed-out iPhone, so
-- none remains: the registrations made before withdrawal keys stop receiving
-- pushes here, and the updated app binds a key and registers again. The
-- trigger from 0040 records each as a push withdrawal. Every later token needs
-- a key.
--
-- Maintenance: the backfill and the validated CHECK on an existing table run
-- while writers are paused and the old runtime, which still accepts keyless
-- registrations from builds the new minimum refuses, is drained.
update public.account_devices
set revision=revision+1,push_token=null,push_environment=null,alerts_enabled=false,
    registered_at=null,updated_at=now()
where push_token is not null and withdrawal_key_hash is null;

alter table public.account_devices
    add constraint account_devices_registration_key_check
        check(push_token is null or withdrawal_key_hash is not null);

insert into public.schema_migrations(version) values('0077_push_withdrawal_key_first');
commit;
