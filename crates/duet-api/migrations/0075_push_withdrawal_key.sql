-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- The SHA-256 of a device-held withdrawal key. Presenting the key clears only
-- the registration it was sent with, without an account session.
alter table public.account_devices
    add column withdrawal_key_hash bytea
        constraint account_devices_withdrawal_key_check
        check(withdrawal_key_hash is null or octet_length(withdrawal_key_hash) = 32);
create index account_devices_withdrawal_key on public.account_devices(withdrawal_key_hash)
    where withdrawal_key_hash is not null;

insert into public.schema_migrations(version) values('0075_push_withdrawal_key');
commit;
