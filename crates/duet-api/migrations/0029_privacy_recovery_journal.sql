-- duet-release: online
begin;

-- This outbox commits WITH the existing lifecycle restriction. No foreign key:
-- an account cascade must retain the recovery receipt. The independent copy is
-- confirmed before the new API acknowledges a successful mutation.
create table public.privacy_recovery_events (
    id uuid primary key default gen_random_uuid(),
    account_id uuid not null,
    operation text not null,
    scope_id uuid not null,
    scope_version text not null default '',
    occurred_at timestamptz not null default clock_timestamp(),
    journaled_at timestamptz
);
create index privacy_recovery_events_pending
    on public.privacy_recovery_events(occurred_at, id)
    where journaled_at is null;

-- A note has no item UUID. Give each replacement a monotonic incarnation so
-- replaying a clear never erases a subsequently authored note.
alter table public.users add column doctor_note_generation bigint not null default 0;
create function public.privacy_note_generation() returns trigger
language plpgsql set search_path=pg_catalog,public as $$
begin
    if new.doctor_note is distinct from old.doctor_note then
        new.doctor_note_generation := old.doctor_note_generation + 1;
    end if;
    return new;
end;
$$;
create trigger privacy_note_generation before update of doctor_note on public.users
    for each row execute function public.privacy_note_generation();

create function public.record_privacy_restriction() returns trigger
language plpgsql set search_path=pg_catalog,public as $$
declare
    account uuid;
    scope uuid;
    operation_name text;
    version text := '';
begin
    if tg_table_name = 'users' then
        account := old.id;
        scope := old.id;
        if tg_op = 'DELETE' then
            operation_name := 'account_delete';
        else
            if old.doctor_note is not null and new.doctor_note is null then
                insert into public.privacy_recovery_events(account_id, operation, scope_id, scope_version)
                values(account, 'doctor_note_delete', scope, old.doctor_note_generation::text);
            end if;
            if old.apple_health_disconnected_at is null and new.apple_health_disconnected_at is not null then
                insert into public.privacy_recovery_events(account_id, operation, scope_id)
                values(account, 'apple_health_withdraw', scope);
            end if;
            if old.health_records_disconnected_at is null and new.health_records_disconnected_at is not null then
                insert into public.privacy_recovery_events(account_id, operation, scope_id)
                values(account, 'health_records_withdraw', scope);
            end if;
            if (old.intelligence_enabled and not new.intelligence_enabled)
                or (new.luna_consent_decision = 'declined'
                    and old.luna_consent_decision is distinct from new.luna_consent_decision) then
                operation_name := 'luna_withdraw';
            end if;
        end if;
    elsif tg_table_name = 'sharing_grant_stats' then
        if old.revoked_at is null and new.revoked_at is not null then
            select sender_user_id into account from public.sharing_grants where id = old.grant_id;
            scope := old.grant_id;
            operation_name := 'sharing_withdraw';
        end if;
    elsif tg_table_name = 'sharing_grants' then
        if new.state in ('revoked', 'reconnect_required') and old.state is distinct from new.state then
            account := old.sender_user_id;
            scope := old.id;
            operation_name := 'sharing_withdraw';
        end if;
    else
        account := old.user_id;
        if tg_table_name = 'doctor_preparations' then
            scope := old.user_id;
            version := old.prepared_at::text;
            operation_name := 'doctor_preparation_delete';
        else
            scope := old.id;
            if tg_table_name = 'source_connections' then
                if new.status in ('disconnected', 'reauthorization_required')
                    and old.status is distinct from new.status then
                    operation_name := 'source_withdraw';
                end if;
            elsif tg_table_name = 'genome_imports' then
                if old.disconnected_at is null and new.disconnected_at is not null then
                    operation_name := 'genome_withdraw';
                end if;
            elsif tg_table_name = 'doctor_collection_items' then
                operation_name := 'doctor_item_delete';
            elsif tg_table_name = 'account_moments' then
                operation_name := 'moment_delete';
            end if;
        end if;
    end if;
    if operation_name is not null then
        insert into public.privacy_recovery_events(account_id, operation, scope_id, scope_version)
        values(account, operation_name, scope, version);
    end if;
    return null;
end;
$$;

create trigger privacy_users after update or delete on public.users
    for each row execute function public.record_privacy_restriction();
create trigger privacy_source after update on public.source_connections
    for each row execute function public.record_privacy_restriction();
create trigger privacy_genome after update on public.genome_imports
    for each row execute function public.record_privacy_restriction();
create trigger privacy_sharing_grant after update on public.sharing_grants
    for each row execute function public.record_privacy_restriction();
create trigger privacy_sharing_scope after update on public.sharing_grant_stats
    for each row execute function public.record_privacy_restriction();
create trigger privacy_doctor_item after delete on public.doctor_collection_items
    for each row execute function public.record_privacy_restriction();
create trigger privacy_doctor_preparation after delete on public.doctor_preparations
    for each row execute function public.record_privacy_restriction();
create trigger privacy_moment after delete on public.account_moments
    for each row execute function public.record_privacy_restriction();

revoke all on function public.privacy_note_generation() from public;
revoke all on function public.record_privacy_restriction() from public;
insert into public.schema_migrations(version) values ('0029_privacy_recovery_journal');
commit;
