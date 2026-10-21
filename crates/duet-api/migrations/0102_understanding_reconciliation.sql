begin;

-- Planned maintenance: the serving release reads and writes
-- operating_indexes.superseded_at, which this migration drops.

-- The understanding's read receipts already name the exact record versions it
-- relied on, and it already lists the saved routines and published
-- observations it depends on. A completed review records here what it was
-- shown, so later runs are told only what changed after that:
-- {"records": {id: {"revision", "available"}}, "saved_routines": {id: revision},
--  "observations": {insight id: true}}.
alter table public.operating_indexes
    add column reconciled jsonb not null default '{}'::jsonb;

-- The precise list replaces the account-wide superseded mark and its writers.
drop trigger canonical_revision_supersede_operating_index
    on public.canonical_evidence_revisions;
drop trigger native_snapshot_insert_supersede_operating_index
    on public.health_contexts;
drop trigger native_snapshot_update_supersede_operating_index
    on public.health_contexts;
drop function public.supersede_operating_index_inputs();
alter table public.operating_indexes drop column superseded_at;

-- Deleting a saved routine still removes the understanding that relied on it;
-- an edit is now found by comparing the routine with the understanding.
drop trigger saved_routine_understanding on public.saved_routines;
create or replace function public.invalidate_saved_routine_understanding() returns trigger
language plpgsql set search_path = public as $$
begin
    delete from public.operating_indexes
    where user_id = old.user_id and old.id = any(saved_routine_ids);
    return old;
end;
$$;
create trigger saved_routine_understanding
    after delete on public.saved_routines
    for each row execute function public.invalidate_saved_routine_understanding();

-- A correction of a published observation still advances the understanding's
-- revision, which ends work that read it before the correction; the
-- correction itself is found by comparing the observation with it.
create or replace function public.invalidate_published_insight_understanding() returns trigger
language plpgsql set search_path = public as $$
begin
    if tg_op = 'DELETE' then
        delete from public.operating_indexes
        where user_id = old.user_id and old.id = any(published_cycle_ids);
        return old;
    end if;
    if old.corrected_insight_ids is distinct from new.corrected_insight_ids then
        update public.operating_indexes
        set revision = revision + 1
        where user_id = old.user_id and old.id = any(published_cycle_ids);
    end if;
    return new;
end;
$$;

insert into public.schema_migrations(version) values ('0102_understanding_reconciliation');
commit;
