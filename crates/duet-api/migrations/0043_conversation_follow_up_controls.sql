-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

alter table public.conversation_action_results
    drop constraint conversation_action_results_kind_check,
    drop constraint conversation_action_results_target_check,
    add constraint conversation_action_results_kind_check check(kind in(
        'save_doctor_question','create_saved_routine','update_saved_routine',
        'create_follow_up','update_follow_up','pause_follow_up',
        'resume_follow_up','cancel_follow_up'
    )),
    add constraint conversation_action_results_target_check check(
        (kind='save_doctor_question' and routine_id is null and follow_up_id is null)
        or (kind in('create_saved_routine','update_saved_routine')
            and routine_id is not null and follow_up_id is null)
        or (kind in('create_follow_up','update_follow_up','pause_follow_up',
                    'resume_follow_up','cancel_follow_up')
            and routine_id is null and follow_up_id is not null)
    );

insert into public.schema_migrations(version) values('0043_conversation_follow_up_controls');
commit;
