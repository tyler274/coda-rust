-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

create table public.follow_up_settings (
    user_id uuid primary key references public.users(id) on delete cascade,
    enabled boolean not null default true,
    revision bigint not null default 1 check (revision > 0),
    updated_at timestamptz not null default now()
);
insert into public.follow_up_settings(user_id)
select id from public.users;

create function public.create_follow_up_settings() returns trigger
language plpgsql set search_path=pg_catalog,public as $$
begin
    insert into public.follow_up_settings(user_id) values(new.id);
    return new;
end $$;
create trigger create_follow_up_settings
    after insert on public.users for each row
    execute function public.create_follow_up_settings();

create table public.follow_ups (
    id uuid primary key,
    user_id uuid not null references public.users(id) on delete cascade,
    accepted_instruction text not null check(length(btrim(accepted_instruction)) > 0),
    schedule jsonb not null check(jsonb_typeof(schedule) = 'object'),
    schedule_summary text not null check(length(btrim(schedule_summary)) > 0),
    revision bigint not null default 1 check(revision > 0),
    state text not null check(state in ('active','paused','cancelled','completed')),
    next_occurrence_at timestamptz,
    source_message_ids uuid[] not null check(cardinality(source_message_ids) > 0),
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    unique(user_id,id),
    check((state='active' and next_occurrence_at is not null)
       or (state<>'active' and next_occurrence_at is null))
);
create index follow_ups_list on public.follow_ups(user_id,updated_at desc,id desc);
create index follow_ups_due on public.follow_ups(next_occurrence_at,id)
    where state='active';

create table public.follow_up_occurrences (
    id uuid primary key,
    user_id uuid not null references public.users(id) on delete cascade,
    follow_up_id uuid not null,
    follow_up_revision bigint not null check(follow_up_revision > 0),
    scheduled_for timestamptz not null,
    status text not null check(status in
        ('queued','running','completed','unavailable','failed','interrupted')),
    claim_token uuid,
    claim_until timestamptz,
    conversation_run_id uuid,
    conversation_message_id uuid,
    completed_at timestamptz,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    unique(user_id,id),
    constraint follow_up_occurrence_identity
        unique(follow_up_id,follow_up_revision,scheduled_for),
    foreign key(user_id,follow_up_id)
        references public.follow_ups(user_id,id) on delete cascade,
    check((status in ('queued','running') and completed_at is null)
       or (status in ('completed','unavailable','failed','interrupted')
           and completed_at is not null))
);
create index follow_up_occurrences_account
    on public.follow_up_occurrences(user_id,scheduled_for,id);
create index follow_up_occurrences_recovery
    on public.follow_up_occurrences(status,claim_until,created_at)
    where status in ('queued','running');

alter table public.conversation_runs
    alter column message_id drop not null,
    add column follow_up_id uuid,
    add column follow_up_revision bigint,
    add column follow_up_occurrence_id uuid,
    add column assignment_time timestamptz,
    add column selected_follow_up_id uuid,
    add column selected_follow_up_revision bigint,
    add constraint conversation_runs_follow_up_fkey
        foreign key(user_id,follow_up_id)
        references public.follow_ups(user_id,id) on delete cascade,
    add constraint conversation_runs_follow_up_occurrence_fkey
        foreign key(user_id,follow_up_occurrence_id)
        references public.follow_up_occurrences(user_id,id) on delete cascade,
    add constraint conversation_runs_selected_follow_up_fkey
        foreign key(user_id,selected_follow_up_id)
        references public.follow_ups(user_id,id) on delete set null,
    add constraint conversation_runs_assignment_check check(
        (message_id is not null and follow_up_id is null
            and follow_up_revision is null and follow_up_occurrence_id is null
            and assignment_time is null)
        or
        (message_id is null and follow_up_id is not null
            and follow_up_revision is not null and follow_up_occurrence_id is not null
            and assignment_time is not null)
    ),
    add constraint conversation_runs_selected_follow_up_check check(
        (selected_follow_up_id is null) = (selected_follow_up_revision is null)
    );
alter table public.follow_up_occurrences
    add constraint follow_up_occurrences_run_fkey
        foreign key(user_id,conversation_run_id)
        references public.conversation_runs(user_id,id) on delete set null
        deferrable initially deferred,
    add constraint follow_up_occurrences_message_fkey
        foreign key(user_id,conversation_message_id)
        references public.conversation_messages(user_id,id) on delete set null
        deferrable initially deferred;

-- Every interruption path, including new user input and AI withdrawal, settles
-- queued scheduled work as interrupted. The assistant message remains the one
-- visible honest result; no second scheduler ledger can strand or replay it.
create or replace function public.interrupt_conversation(account_id uuid) returns void
language plpgsql as $$
declare change_revision bigint;
begin
    change_revision := public.conversation_changed(account_id);
    if change_revision is null then return; end if;
    with ended as (
        update public.conversation_runs set state='interrupted',failure='interrupted',
            lease_until=null,revision=change_revision,updated_at=now()
        where user_id=account_id and state in ('queued','running')
        returning assistant_message_id,follow_up_occurrence_id
    ), messages as (
        update public.conversation_messages set state='interrupted',
            revision=change_revision,updated_at=now()
        where id in (select assistant_message_id from ended)
    )
    update public.follow_up_occurrences set status='interrupted',
        completed_at=now(),updated_at=now()
    where id in (select follow_up_occurrence_id from ended)
      and status in('queued','running');
end $$;

alter table public.conversation_action_results
    drop constraint conversation_action_results_kind_check,
    drop constraint conversation_action_results_target_check,
    add column follow_up_id uuid,
    add constraint conversation_action_results_kind_check check(kind in(
        'save_doctor_question','create_saved_routine','update_saved_routine',
        'create_follow_up','update_follow_up'
    )),
    add constraint conversation_action_results_target_check check(
        (kind='save_doctor_question' and routine_id is null and follow_up_id is null)
        or (kind in('create_saved_routine','update_saved_routine')
            and routine_id is not null and follow_up_id is null)
        or (kind in('create_follow_up','update_follow_up')
            and routine_id is null and follow_up_id is not null)
    ),
    add constraint conversation_action_results_user_id_follow_up_id_fkey
        foreign key(user_id,follow_up_id)
        references public.follow_ups(user_id,id) on delete cascade;
create index conversation_action_results_follow_up
    on public.conversation_action_results(user_id,follow_up_id,result_generation,conversation_revision)
    where follow_up_id is not null;

revoke all on public.follow_up_settings,public.follow_ups,
    public.follow_up_occurrences from public;
revoke all on function public.create_follow_up_settings() from public;

insert into public.schema_migrations(version) values('0039_follow_ups');
commit;
