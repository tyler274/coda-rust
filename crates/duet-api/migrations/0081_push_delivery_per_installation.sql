-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Follow-up notice delivery per iPhone installation. A notice
-- (follow_up_push_deliveries) keeps only what the result owner recorded: the
-- reply, its account, follow-up and revision, and when it was saved. Delivery
-- state belongs to each iPhone the notice is for: one row per (notice,
-- installation), created with the notice for each of the account's iPhones
-- registered with alerts on. Each row has its own attempt, outcome and retry,
-- and the notice's state is derived from its rows, never stored. A row
-- references its iPhone's registration and is deleted with it, so a
-- withdrawal, a refused token, a replacing bind or account deletion ends
-- what that iPhone was owed and nothing else.
--
-- Maintenance: the serving runtime reads and writes each notice's own state
-- columns, which this removes. It joins the release's existing window with
-- 0077 and 0080. A notice still owed at release is owed to each registered,
-- alert-enabled iPhone of its account, at the same retry time; a settled one
-- is owed to none. After 0080 in the same window no iPhone is registered yet,
-- so no notice is owed then.
create table public.follow_up_push_installations (
    message_id uuid not null
        references public.follow_up_push_deliveries(message_id) on delete cascade,
    installation_id uuid not null
        references public.account_devices(installation_id) on delete cascade,
    state text not null default 'pending' check (state in (
        'pending','dispatching','accepted','rejected','uncertain','unavailable',
        'suppressed','superseded','expired')),
    attempt_id uuid,
    attempted_at timestamptz,
    settled_at timestamptz,
    next_attempt_at timestamptz,
    primary key (message_id, installation_id)
);
create index follow_up_push_installations_installation
    on public.follow_up_push_installations(installation_id);
create index follow_up_push_installations_pending
    on public.follow_up_push_installations(installation_id, message_id) where state='pending';

insert into public.follow_up_push_installations(message_id,installation_id,next_attempt_at)
select notice.message_id,device.installation_id,notice.next_attempt_at
from public.follow_up_push_deliveries notice
join public.account_devices device on device.user_id=notice.user_id
    and device.push_token is not null and device.alerts_enabled
where notice.state='pending';

drop index public.follow_up_push_pending;
alter table public.follow_up_push_deliveries
    drop column state, drop column attempt_id, drop column attempted_at,
    drop column settled_at, drop column next_attempt_at;

revoke all on public.follow_up_push_installations from public;
insert into public.schema_migrations(version) values('0081_push_delivery_per_installation');
commit;
