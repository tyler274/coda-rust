-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

create table public.conversation_state (
    user_id uuid primary key references public.users(id) on delete cascade,
    revision bigint not null default 0 check (revision >= 0),
    next_sequence bigint not null default 1 check (next_sequence > 0)
);
create table public.conversation_messages (
    id uuid primary key,
    user_id uuid not null references public.users(id) on delete cascade,
    role text not null check (role in ('user','assistant')),
    text text not null,
    state text not null check (state in ('pending','streaming','complete','interrupted','failed')),
    sequence bigint not null check (sequence > 0),
    revision bigint not null check (revision > 0),
    run_id uuid,
    sources jsonb not null default '[]'::jsonb,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    unique (user_id, sequence),
    unique (user_id, id)
);
create index conversation_message_changes on public.conversation_messages(user_id, revision);
create table public.conversation_runs (
    id uuid primary key,
    user_id uuid not null references public.users(id) on delete cascade,
    message_id uuid not null,
    assistant_message_id uuid not null,
    state text not null check (state in ('queued','running','completed','interrupted','failed','cancelled')),
    revision bigint not null check (revision > 0),
    timezone text not null,
    failure text check (failure in ('unavailable','interrupted','execution_failed')),
    claim_token uuid,
    lease_until timestamptz,
    delivery_generation bigint not null default 0,
    delivery_until timestamptz,
    control_revision bigint,
    evidence_revision bigint,
    history_cursor bigint,
    context_snapshot_id uuid,
    generation_metadata jsonb not null default '{}'::jsonb,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);
alter table public.conversation_messages add constraint conversation_message_run
    foreign key (run_id) references public.conversation_runs(id)
    on delete set null deferrable initially deferred;
create unique index conversation_one_active_run on public.conversation_runs(user_id)
    where state in ('queued','running');
create index conversation_run_changes on public.conversation_runs(user_id,revision);
create index conversation_delivery_pending on public.conversation_runs(delivery_until)
    where state='queued';

-- One current transcript, with revision notifications as wakeups only. No
-- second event-log copy of patient prose is needed to reconnect.
create function public.conversation_changed(account_id uuid) returns bigint
language plpgsql as $$
declare new_revision bigint;
begin
    update public.conversation_state set revision=revision+1
        where user_id=account_id returning revision into new_revision;
    if new_revision is not null then
        perform pg_notify('duet_conversation', account_id::text);
    end if;
    return new_revision;
end $$;

create function public.interrupt_conversation(account_id uuid) returns void
language plpgsql as $$
declare change_revision bigint;
begin
    change_revision := public.conversation_changed(account_id);
    if change_revision is null then return; end if;
    with ended as (
        update public.conversation_runs set state='interrupted',failure='interrupted',
            lease_until=null,revision=change_revision,updated_at=now()
        where user_id=account_id and state in ('queued','running')
        returning assistant_message_id
    ) update public.conversation_messages set state='interrupted',
        revision=change_revision,updated_at=now()
        where id in (select assistant_message_id from ended);
end $$;

-- Every account/source authority owner already increments these revisions.
-- Interrupting in that transaction fences publication and wakes idle streams
-- even if the provider has not produced its first token.
create function public.conversation_account_authority_changed() returns trigger
language plpgsql as $$
begin
    if tg_op='DELETE' then
        perform pg_notify('duet_conversation',old.id::text);
        return old;
    end if;
    if (new.intelligence_control_revision,new.evidence_revision,new.terms_version)
        is distinct from
       (old.intelligence_control_revision,old.evidence_revision,old.terms_version) then
        perform public.interrupt_conversation(new.id);
    end if;
    return new;
end $$;
create trigger conversation_account_authority
    after update of intelligence_control_revision,evidence_revision,terms_version
    on public.users for each row execute function public.conversation_account_authority_changed();
create trigger conversation_account_deleted
    before delete on public.users for each row
    execute function public.conversation_account_authority_changed();
create function public.conversation_context_changed() returns trigger
language plpgsql as $$
begin
    if tg_op='DELETE' then
        perform public.interrupt_conversation(old.user_id);
        return old;
    end if;
    if tg_op='INSERT' or new.snapshot_id is distinct from old.snapshot_id then
        perform public.interrupt_conversation(new.user_id);
    end if;
    return new;
end $$;
create trigger conversation_context_authority
    after insert or update or delete on public.health_contexts
    for each row execute function public.conversation_context_changed();

-- One mutable understanding owner for both agent assignments. Revision is a
-- compare-and-replace fence against a concurrent assignment's newer context.
alter table public.operating_indexes
    alter column cycle_id drop not null,
    add column conversation_run_id uuid references public.conversation_runs(id) on delete set null,
    add column revision bigint not null default 1 check(revision>0),
    add column conversation_message_ids uuid[] not null default '{}';

create table public.conversation_citations (
    message_id uuid not null references public.conversation_messages(id) on delete cascade,
    source_id text not null,
    kind text not null check(kind in ('record','native_fact')),
    record_revision bigint references public.canonical_evidence_revisions(revision) on delete cascade,
    native_fact jsonb,
    primary key(message_id,source_id),
    check ((kind='record' and record_revision is not null and native_fact is null)
        or (kind='native_fact' and record_revision is null and native_fact is not null))
);
revoke all on public.conversation_state, public.conversation_messages,
    public.conversation_runs, public.conversation_citations from public;
revoke all on function public.conversation_changed(uuid),
    public.interrupt_conversation(uuid), public.conversation_account_authority_changed(),
    public.conversation_context_changed() from public;

insert into public.schema_migrations(version) values ('0032_persistent_conversation');
commit;
