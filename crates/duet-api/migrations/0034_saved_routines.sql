-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- A routine UUID is the account-scoped item incarnation. Deletion receipts
-- permanently fence reuse of that UUID; a genuinely new item gets a new UUID.
create table public.saved_routines (
    user_id uuid not null references public.users(id) on delete cascade,
    id uuid not null,
    title text not null check (
        char_length(title) between 1 and 32768
        and title ~ '[^[:space:]]'
    ),
    body text not null check (
        char_length(body) between 1 and 32768
        and body ~ '[^[:space:]]'
    ),
    original_content_hash text not null check (
        original_content_hash ~ '^[0-9a-f]{64}$'
    ),
    revision bigint not null default 1 check (revision > 0),
    created_at timestamptz not null default clock_timestamp(),
    updated_at timestamptz not null default clock_timestamp(),
    primary key (user_id, id),
    check (updated_at >= created_at)
);
create index saved_routines_current
    on public.saved_routines(user_id, updated_at desc, id desc);

create function public.record_saved_routine_delete() returns trigger
language plpgsql set search_path=pg_catalog,public as $$
begin
    insert into public.privacy_recovery_events(
        account_id, operation, scope_id
    ) values (
        old.user_id, 'saved_routine_delete', old.id
    );
    return null;
end;
$$;
create trigger privacy_saved_routine
    after delete on public.saved_routines
    for each row execute function public.record_saved_routine_delete();

revoke all on public.saved_routines from public;
revoke all on function public.record_saved_routine_delete() from public;

insert into public.schema_migrations(version) values ('0034_saved_routines');
commit;
