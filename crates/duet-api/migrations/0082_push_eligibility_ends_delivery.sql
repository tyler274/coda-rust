-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- A notice goes to an iPhone only while that iPhone stays eligible (signed in
-- on the account, alerts on) from when its reply was saved until its attempt.
-- The registration's trigger is the one owner of "this registration ended":
-- it records the withdrawal receipt, and now also ends every notice still
-- owed to that iPhone, in the same transaction as the change that ended it.
-- A deleted row already takes them by the foreign key; an update that clears
-- the token (a bind elsewhere took it) or turns alerts off ends them here.
-- Turning alerts back on brings only later notices, so no attempt settles as
-- `unavailable` any more and that state is removed.
--
-- Maintenance: joins the release's window with 0077, 0080 and 0081. The
-- serving runtime has no per-installation delivery; this changes a trigger and
-- a table created in 0081.
create or replace function public.record_push_restriction() returns trigger
language plpgsql set search_path=pg_catalog,public as $$
declare ended boolean;
begin
    if tg_op = 'DELETE' then
        -- Account deletion's cascade has its own receipt. The row's owed
        -- notices go with it by the foreign key.
        ended := (old.push_token is not null or old.alerts_enabled)
            and exists(select 1 from public.users where id = old.user_id);
    else
        ended := (old.push_token is not null and new.push_token is null)
            or (old.alerts_enabled and not new.alerts_enabled);
        if ended then
            delete from public.follow_up_push_installations
            where installation_id = old.installation_id;
        end if;
    end if;
    if ended then
        insert into public.privacy_recovery_events(account_id,operation,scope_id)
        values(old.user_id,'push_withdraw',old.installation_id);
    end if;
    return null;
end $$;

delete from public.follow_up_push_installations where state = 'unavailable';
alter table public.follow_up_push_installations
    drop constraint follow_up_push_installations_state_check,
    add constraint follow_up_push_installations_state_check check (state in (
        'pending','dispatching','accepted','rejected','uncertain',
        'suppressed','superseded','expired'));

insert into public.schema_migrations(version) values('0082_push_eligibility_ends_delivery');
commit;
