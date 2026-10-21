-- duet-release: maintenance
begin;

-- One bounded execution record owned by each existing doctor workspace.
-- Inputs stay in their canonical tables: only their fingerprint is retained.
create table public.doctor_preparation_work (
  user_id uuid primary key references public.users(id) on delete cascade,
  id uuid not null unique,
  state text not null check (state in ('queued', 'running', 'completed', 'failed')),
  collection_fingerprint text not null,
  control_revision bigint not null,
  expected_revision bigint,
  failure text check (failure in ('changed', 'unavailable', 'interrupted')),
  claim_token uuid,
  lease_until timestamptz,
  delivery_generation integer not null default 0,
  delivery_until timestamptz,
  accepted_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  check ((state = 'running') = (claim_token is not null and lease_until is not null)),
  check ((state = 'failed') = (failure is not null))
);
create index doctor_preparation_work_recovery
  on public.doctor_preparation_work(state, updated_at)
  where state in ('queued', 'running');

-- Invalidate at the mutation, including change-then-revert and privacy replay.
create function public.invalidate_doctor_preparation_work() returns trigger
language plpgsql as $$
declare account_id uuid;
begin
  if TG_TABLE_NAME = 'users' then
    if (old.doctor_note, old.doctor_note_generation, old.intelligence_control_revision,
        old.intelligence_enabled, old.terms_version, old.terms_accepted_at,
        old.luna_consent_version, old.luna_consent_decision)
       is not distinct from
       (new.doctor_note, new.doctor_note_generation, new.intelligence_control_revision,
        new.intelligence_enabled, new.terms_version, new.terms_accepted_at,
        new.luna_consent_version, new.luna_consent_decision) then
      return new;
    end if;
    account_id := new.id;
  elsif TG_OP = 'DELETE' then
    account_id := old.user_id;
  else
    account_id := new.user_id;
  end if;
  update public.doctor_preparation_work set state='failed', failure='changed',
    claim_token=null, lease_until=null, updated_at=now()
    where user_id=account_id and state in ('queued', 'running');
  return null;
end;
$$;
create trigger doctor_work_account_changed after update on public.users
  for each row execute function public.invalidate_doctor_preparation_work();
create trigger doctor_work_selection_changed after insert or update or delete
  on public.doctor_collection_items for each row
  execute function public.invalidate_doctor_preparation_work();
create trigger doctor_work_summary_changed after insert or update or delete
  on public.doctor_preparations for each row
  execute function public.invalidate_doctor_preparation_work();

create function public.notify_doctor_preparation_work() returns trigger
language plpgsql as $$
begin
  if TG_OP = 'UPDATE' and (old.id, old.state) is not distinct from (new.id, new.state) then
    return null;
  end if;
  -- Reuse the account wakeup channel; notifications never contain health data.
  if TG_OP = 'DELETE' then
    perform pg_notify('duet_conversation', old.user_id::text);
  else
    perform pg_notify('duet_conversation', new.user_id::text);
  end if;
  return null;
end;
$$;
create trigger doctor_work_changed after insert or update or delete
  on public.doctor_preparation_work for each row
  execute function public.notify_doctor_preparation_work();

insert into public.schema_migrations(version) values ('0051_doctor_preparation_work');
commit;
