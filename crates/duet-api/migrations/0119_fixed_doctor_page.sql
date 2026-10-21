-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Announced beta cutover: existing workspaces reset. This migration must ship
-- only after Dev has told the beta group to save any old PDF they want to keep.
-- New content stays with the existing users/doctor_collection_items owner.
delete from public.doctor_preparations;
delete from public.doctor_collection_items;
update public.users set doctor_note=null,doctor_note_generation=doctor_note_generation+1;
delete from public.conversation_action_results where kind in ('save_doctor_question','prepare_doctor_summary');

drop function public.invalidate_doctor_preparation_work() cascade;
drop function public.notify_doctor_preparation_work() cascade;
drop function public.doctor_preparation_action_changed() cascade;
drop table public.doctor_preparation_work cascade;
drop table public.doctor_preparations cascade;

alter table public.users drop constraint users_doctor_note_check;

alter table public.users
    add column doctor_page jsonb not null default '{}'::jsonb check(jsonb_typeof(doctor_page)='object'),
    add column doctor_proposal jsonb check(doctor_proposal is null or jsonb_typeof(doctor_proposal)='object'),
    add column doctor_page_updated_at timestamptz not null default clock_timestamp();
alter table public.doctor_collection_items
    add column fact_snapshot jsonb,
    add column fact_snapshot_id uuid,
    add column fact_timezone text not null default 'UTC',
    add column page_entry jsonb,
    add column medication_confirmed boolean not null default false;

-- Extend the existing generation: one version for the whole workspace.
create or replace function public.privacy_note_generation() returns trigger
language plpgsql set search_path=pg_catalog,public as $$
begin
    if (new.doctor_note,new.doctor_page) is distinct from (old.doctor_note,old.doctor_page) then
        new.doctor_note_generation := old.doctor_note_generation+1;
    end if;
    if new.doctor_note_generation<>old.doctor_note_generation then
        new.doctor_page_updated_at := new.updated_at;
    end if;
    return new;
end $$;
drop trigger privacy_note_generation on public.users;
create trigger privacy_note_generation before update of doctor_note,doctor_page,doctor_note_generation on public.users
    for each row execute function public.privacy_note_generation();

create function public.doctor_items_changed() returns trigger
language plpgsql set search_path=pg_catalog,public as $$
declare account uuid;
begin
    if tg_op='UPDATE' and to_jsonb(old)=to_jsonb(new) then return null; end if;
    account := case when tg_op='DELETE' then old.user_id else new.user_id end;
    update public.users set doctor_note_generation=doctor_note_generation+1,updated_at=now() where id=account;
    return null;
end $$;
create trigger doctor_items_changed after insert or update or delete on public.doctor_collection_items
    for each row execute function public.doctor_items_changed();
revoke all on function public.doctor_items_changed() from public;

drop trigger health_discovery_doctor_note on public.users;
create trigger health_discovery_doctor_note after update of doctor_note,doctor_page,doctor_note_generation,doctor_proposal on public.users
    for each row execute function public.health_discovery_doctor_note_changed();

-- Preserve the existing discovery and action projection at their owner.
create or replace function public.health_discovery_doctor_note_changed() returns trigger
language plpgsql set search_path=pg_catalog,public as $$
declare message_ids uuid[]; change_revision bigint;
begin
    if new.doctor_note_generation is distinct from old.doctor_note_generation
       or new.doctor_proposal is distinct from old.doctor_proposal then
        perform public.invalidate_health_discovery(new.id,'doctor_workspace',new.id::text);
        select array_agg(distinct message_id) into message_ids from public.conversation_action_results
            where user_id=new.id and kind in ('save_doctor_question','prepare_doctor_summary');
        message_ids := coalesce(message_ids,'{}'::uuid[]) || array_remove(array[
            (old.doctor_proposal->>'message_id')::uuid,(new.doctor_proposal->>'message_id')::uuid],null);
        if cardinality(message_ids)>0 then
            change_revision := public.conversation_changed(new.id);
            if change_revision is not null then
                update public.conversation_messages set revision=change_revision,updated_at=now()
                    where user_id=new.id and id=any(message_ids);
            end if;
        end if;
    end if;
    return null;
end $$;

create or replace function public.record_privacy_restriction() returns trigger
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
            -- A whole-page clear emits one generation-fenced receipt. Emitting
            -- a note receipt first would advance that same generation during
            -- restore and make the page receipt miss its original incarnation.
            if new.doctor_note is null and new.doctor_page='{}'::jsonb and new.doctor_proposal is null
               and (old.doctor_page<>'{}'::jsonb or old.doctor_proposal is not null) then
                insert into public.privacy_recovery_events(account_id, operation, scope_id, scope_version)
                values(account, 'doctor_page_delete', scope, old.doctor_note_generation::text);
            elsif old.doctor_note is not null and new.doctor_note is null then
                insert into public.privacy_recovery_events(account_id,operation,scope_id,scope_version)
                values(account,'doctor_note_delete',scope,old.doctor_note_generation::text);
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
    if operation_name is not null then
        insert into public.privacy_recovery_events(account_id, operation, scope_id, scope_version)
        values(account, operation_name, scope, version);
    end if;
    return null;
end;
$$;


insert into public.schema_migrations(version) values ('0119_fixed_doctor_page');
commit;
