-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

alter table public.doctor_action_results
    rename to conversation_action_results;
alter table public.conversation_action_results
    rename column result_note_hash to result_content_hash;
alter table public.conversation_action_results
    add column kind text not null default 'save_doctor_question',
    add column routine_id uuid;
alter table public.conversation_action_results
    alter column kind drop default,
    add constraint conversation_action_results_kind_check check (
        kind in (
            'save_doctor_question',
            'create_saved_routine',
            'update_saved_routine'
        )
    ),
    add constraint conversation_action_results_target_check check (
        (kind = 'save_doctor_question' and routine_id is null)
        or (
            kind in ('create_saved_routine', 'update_saved_routine')
            and routine_id is not null
        )
    ),
    add constraint conversation_action_results_user_id_routine_id_fkey
        foreign key (user_id, routine_id)
        references public.saved_routines(user_id, id) on delete cascade;

alter table public.conversation_action_results
    rename constraint doctor_action_results_pkey
    to conversation_action_results_pkey;
alter table public.conversation_action_results
    rename constraint doctor_action_results_user_id_fkey
    to conversation_action_results_user_id_fkey;
alter table public.conversation_action_results
    rename constraint doctor_action_results_user_id_run_id_fkey
    to conversation_action_results_user_id_run_id_fkey;
alter table public.conversation_action_results
    rename constraint doctor_action_results_user_id_message_id_fkey
    to conversation_action_results_user_id_message_id_fkey;
alter table public.conversation_action_results
    rename constraint doctor_action_results_base_generation_check
    to conversation_action_results_base_generation_check;
alter table public.conversation_action_results
    rename constraint doctor_action_results_check
    to conversation_action_results_generation_check;
alter table public.conversation_action_results
    rename constraint doctor_action_results_conversation_revision_check
    to conversation_action_results_conversation_revision_check;
alter table public.conversation_action_results
    rename constraint doctor_action_results_input_hash_check
    to conversation_action_results_input_hash_check;
alter table public.conversation_action_results
    rename constraint doctor_action_results_result_note_hash_check
    to conversation_action_results_result_content_hash_check;
alter index public.doctor_action_results_message
    rename to conversation_action_results_message;
alter index public.doctor_action_results_generation
    rename to conversation_action_results_generation;

create index conversation_action_results_routine
    on public.conversation_action_results(
        user_id, routine_id, result_generation, conversation_revision
    ) where routine_id is not null;

insert into public.schema_migrations(version)
values ('0035_conversation_action_results');
commit;
