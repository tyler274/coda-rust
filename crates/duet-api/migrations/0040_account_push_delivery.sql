-- Requires a maintenance release: moves the sole token owner and removes its old columns.
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

create table public.account_devices (
    user_id uuid primary key references public.users(id) on delete cascade,
    revision bigint not null default 1 check(revision>0),
    push_token bytea unique,
    push_environment text,
    alerts_enabled boolean not null default false,
    registered_at timestamptz,
    updated_at timestamptz not null default now(),
    constraint account_devices_registration_check check(
        (push_token is null and push_environment is null and registered_at is null and not alerts_enabled)
        or (push_token is not null and push_environment in ('sandbox','production')
            and registered_at is not null and octet_length(push_token) between 1 and 256)
    )
);
insert into public.account_devices(user_id,push_token,push_environment,registered_at)
select user_id,apns_token,apns_environment,apns_registered_at
from public.sharing_devices where revoked_at is null and apns_token is not null;
alter table public.sharing_devices
    drop column apns_token,drop column apns_environment,drop column apns_registered_at;

create table public.follow_up_push_deliveries (
    message_id uuid primary key references public.conversation_messages(id) on delete cascade,
    user_id uuid not null references public.users(id) on delete cascade,
    follow_up_id uuid not null references public.follow_ups(id) on delete cascade,
    follow_up_revision bigint not null check(follow_up_revision>0),
    state text not null default 'pending' check(state in
        ('pending','dispatching','accepted','unavailable','suppressed','rejected','uncertain')),
    attempt_id uuid,
    attempted_at timestamptz,
    settled_at timestamptz,
    created_at timestamptz not null default now()
);
create index follow_up_push_pending on public.follow_up_push_deliveries(created_at,message_id)
    where state='pending';

-- Operational withdrawals use the existing content-free restore journal.
create function public.record_follow_up_restriction() returns trigger
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
    elsif tg_table_name='follow_up_settings' then
        scope:=new.user_id;
        if not new.enabled and (tg_op='INSERT' or old.enabled) then
            operation_name:='proactive_withdraw';
        end if;
    else
        scope:=new.user_id;
        if (old.push_token is not null and new.push_token is null)
            or (old.alerts_enabled and not new.alerts_enabled) then
            operation_name:='push_withdraw';
        end if;
    end if;
    if operation_name is not null then
        insert into public.privacy_recovery_events(account_id,operation,scope_id)
        values(new.user_id,operation_name,scope);
    end if;
    return null;
end $$;
create trigger follow_up_privacy_restriction after update of state on public.follow_ups
    for each row execute function public.record_follow_up_restriction();
create trigger proactive_privacy_restriction after insert or update of enabled on public.follow_up_settings
    for each row execute function public.record_follow_up_restriction();
create trigger push_privacy_restriction after update on public.account_devices
    for each row execute function public.record_follow_up_restriction();
revoke all on public.account_devices,public.follow_up_push_deliveries from public;
revoke all on function public.record_follow_up_restriction() from public;
insert into public.schema_migrations(version) values('0040_account_push_delivery');
commit;
