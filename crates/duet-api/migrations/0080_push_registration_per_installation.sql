-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- One push registration per iPhone installation, not per account. Every
-- signed-in iPhone on an account receives notifications, and signing out
-- removes only this iPhone's registration. A row is keyed by the installation
-- id the iPhone keeps in its Keychain and owned by the account signed in on
-- it; its withdrawal key's hash (unique) is what sign-out and the session-free
-- withdrawal match. Revisions come from one sequence, so a row created again
-- after a delete never matches a revision an iPhone read before it.
--
-- Maintenance: the serving runtime writes one row per account
-- (`on conflict(user_id)`), which needs user_id to be unique, and reads the
-- account's row as the only registration. Neither holds once an account has
-- a row per iPhone, so the old runtime must be drained before this applies.
-- Existing rows cannot be matched to an installation. They are removed, and
-- 0040's trigger, still in place when they are, records each registration
-- they held as an account-scoped push withdrawal. The trigger below then
-- replaces it and records each later withdrawal for its installation. Each
-- iPhone binds and registers again at its next launch.

create function public.record_push_restriction() returns trigger
language plpgsql set search_path=pg_catalog,public as $$
declare ended boolean;
begin
    -- A registration ends when its row is deleted, its token is cleared (a
    -- bind, including another account's on this installation, clears it) or
    -- its alerts are turned off. The receipt is the owner's at that time.
    if tg_op = 'DELETE' then
        -- Account deletion's cascade has its own receipt.
        ended := (old.push_token is not null or old.alerts_enabled)
            and exists(select 1 from public.users where id = old.user_id);
    else
        ended := (old.push_token is not null and new.push_token is null)
            or (old.alerts_enabled and not new.alerts_enabled);
    end if;
    if ended then
        insert into public.privacy_recovery_events(account_id,operation,scope_id)
        values(old.user_id,'push_withdraw',old.installation_id);
    end if;
    return null;
end $$;

-- The account's current rows are withdrawn under the 0040 trigger, whose
-- receipt scope is the account.
update public.account_devices
set revision=revision+1,push_token=null,push_environment=null,alerts_enabled=false,
    registered_at=null,updated_at=now()
where push_token is not null or alerts_enabled;
delete from public.account_devices;

drop trigger push_privacy_restriction on public.account_devices;
alter table public.account_devices drop constraint account_devices_pkey;
drop index public.account_devices_withdrawal_key;
alter table public.account_devices
    add column installation_id uuid not null,
    add constraint account_devices_pkey primary key (installation_id),
    alter column revision drop default;
create index account_devices_user on public.account_devices(user_id);
create unique index account_devices_withdrawal_key on public.account_devices(withdrawal_key_hash)
    where withdrawal_key_hash is not null;
create sequence public.account_device_revisions;

create trigger push_privacy_restriction after update or delete on public.account_devices
    for each row execute function public.record_push_restriction();

-- 0040's function now serves only follow-ups and follow-up settings; its
-- account_devices branch had no trigger left to reach it.
create or replace function public.record_follow_up_restriction() returns trigger
language plpgsql set search_path=pg_catalog,public as $$
declare operation_name text; scope uuid;
begin
    if tg_table_name='follow_ups' then
        scope:=new.id;
        if old.state is distinct from new.state and new.state='cancelled' then
            operation_name:='follow_up_cancel';
        elsif old.state is distinct from new.state and new.state='paused' then
            operation_name:='follow_up_pause';
        end if;
    else
        scope:=new.user_id;
        if not new.enabled and (tg_op='INSERT' or old.enabled) then
            operation_name:='proactive_withdraw';
        end if;
    end if;
    if operation_name is not null then
        insert into public.privacy_recovery_events(account_id,operation,scope_id)
        values(new.user_id,operation_name,scope);
    end if;
    return null;
end $$;

insert into public.schema_migrations(version) values('0080_push_registration_per_installation');
commit;
