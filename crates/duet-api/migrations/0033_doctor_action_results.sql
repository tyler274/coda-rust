-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

alter table public.conversation_runs
    add constraint conversation_runs_user_id_id_key unique (user_id, id);

create table public.doctor_action_results (
    id uuid primary key,
    user_id uuid not null references public.users(id) on delete cascade,
    run_id uuid not null,
    message_id uuid not null,
    input_hash text not null check (input_hash ~ '^[0-9a-f]{64}$'),
    base_generation bigint not null check (base_generation >= 0),
    result_generation bigint not null check (result_generation > base_generation),
    result_note_hash text not null check (result_note_hash ~ '^[0-9a-f]{64}$'),
    confirmed_at timestamptz not null default clock_timestamp(),
    conversation_revision bigint not null check (conversation_revision > 0),
    foreign key (user_id, run_id)
        references public.conversation_runs(user_id, id) on delete cascade,
    foreign key (user_id, message_id)
        references public.conversation_messages(user_id, id) on delete cascade
);
create index doctor_action_results_message
    on public.doctor_action_results(user_id, message_id, conversation_revision);
create index doctor_action_results_generation
    on public.doctor_action_results(user_id, result_generation);

revoke all on public.doctor_action_results from public;

insert into public.schema_migrations(version) values ('0033_doctor_action_results');
commit;
