-- duet-release: online
begin;

-- Acquire every DDL lock before touching the outbox. In-flight privacy writers
-- can hold these tables before emitting a receipt; waiting in reverse order
-- would risk aborting a user transaction in a deadlock. A busy release stops
-- immediately instead, leaving the old API and schema intact.
lock table public.users in share row exclusive mode nowait;
lock table public.account_deletion_fences in access exclusive mode nowait;
lock table public.doctor_collection_items in access exclusive mode nowait;
lock table public.privacy_recovery_events in access exclusive mode nowait;

-- Additive delivery metadata; v1 immutable receipt bytes are unchanged. Old
-- writers still emit events, and the insert trigger below enriches them too.
alter table public.privacy_recovery_events
    add column related_account_id uuid,
    add column parent_scope_id uuid,
    add column doctor_scope_recorded boolean not null default false,
    add column delivery_token uuid,
    add column delivery_lease_until timestamptz,
    add column delivery_attempts integer not null default 0,
    add column next_attempt_at timestamptz not null default now();

create function public.privacy_delivery_scope() returns trigger
language plpgsql set search_path=pg_catalog,public as $$
begin
    if new.operation = 'sharing_withdraw' then
        select recipient_user_id, connection_id
        into new.related_account_id, new.parent_scope_id
        from public.sharing_grants where id = new.scope_id;
    end if;
    return new;
end;
$$;
create trigger privacy_delivery_scope before insert on public.privacy_recovery_events
    for each row execute function public.privacy_delivery_scope();
update public.privacy_recovery_events e
set related_account_id = g.recipient_user_id, parent_scope_id = g.connection_id
from public.sharing_grants g
where e.operation = 'sharing_withdraw' and e.scope_id = g.id;

-- The deleted row owns the insight -> item-incarnation relationship. Preserve
-- it in SQL delivery metadata without changing the archived v1 receipt. The
-- replacement trigger captures deletions from both old and new API writers.
create function public.privacy_doctor_item_scope() returns trigger
language plpgsql set search_path=pg_catalog,public as $$
begin
    insert into public.privacy_recovery_events(account_id, operation, scope_id, parent_scope_id, doctor_scope_recorded)
    values (old.user_id, 'doctor_item_delete', old.id,
            case when old.kind = 'insight' then old.source_id::uuid else null end, true);
    return null;
end;
$$;
drop trigger privacy_doctor_item on public.doctor_collection_items;
create trigger privacy_doctor_item after delete on public.doctor_collection_items
    for each row execute function public.privacy_doctor_item_scope();

create index privacy_recovery_events_account_scope
    on public.privacy_recovery_events(account_id, operation, scope_id);
create index privacy_recovery_events_account_parent_scope
    on public.privacy_recovery_events(account_id, operation, parent_scope_id)
    where journaled_at is null;
create index privacy_recovery_events_related_scope
    on public.privacy_recovery_events(related_account_id, operation, parent_scope_id)
    where journaled_at is null;
create index privacy_recovery_events_delivery_due
    on public.privacy_recovery_events(next_attempt_at, occurred_at, id)
    where journaled_at is null;

-- Preserve the deleted incarnation in the existing identity fence, including
-- deletions by the previous API during online rollout. A pre-migration fence
-- with no surviving account cannot be safely correlated by time or guessed.
alter table public.account_deletion_fences
    add column privacy_account_id uuid,
    add column privacy_scope_recorded boolean not null default false;
create function public.privacy_deletion_scope() returns trigger
language plpgsql set search_path=pg_catalog,public as $$
begin
    select id into new.privacy_account_id from public.users
    where auth_user_id = new.auth_user_id;
    new.privacy_scope_recorded := true;
    return new;
end;
$$;
create trigger privacy_deletion_scope before insert on public.account_deletion_fences
    for each row execute function public.privacy_deletion_scope();
create function public.privacy_deleted_account_scope() returns trigger
language plpgsql set search_path=pg_catalog,public as $$
begin
    update public.account_deletion_fences
    set privacy_account_id = old.id, privacy_scope_recorded = true
    where auth_user_id = old.auth_user_id;
    return null;
end;
$$;
create trigger privacy_deleted_account_scope after delete on public.users
    for each row execute function public.privacy_deleted_account_scope();

revoke all on function public.privacy_delivery_scope() from public;
revoke all on function public.privacy_doctor_item_scope() from public;
revoke all on function public.privacy_deletion_scope() from public;
revoke all on function public.privacy_deleted_account_scope() from public;
insert into public.schema_migrations(version) values ('0030_scoped_privacy_delivery');
commit;
