-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

alter table public.conversation_action_results
    drop constraint conversation_action_results_kind_check,
    drop constraint conversation_action_results_target_check,
    add constraint conversation_action_results_kind_check check(kind in(
        'save_doctor_question','prepare_doctor_summary',
        'create_saved_routine','update_saved_routine',
        'create_follow_up','update_follow_up','pause_follow_up',
        'resume_follow_up','cancel_follow_up'
    )),
    add constraint conversation_action_results_target_check check(
        (kind in('save_doctor_question','prepare_doctor_summary')
            and routine_id is null and follow_up_id is null)
        or (kind in('create_saved_routine','update_saved_routine')
            and routine_id is not null and follow_up_id is null)
        or (kind in('create_follow_up','update_follow_up','pause_follow_up',
                    'resume_follow_up','cancel_follow_up')
            and routine_id is null and follow_up_id is not null)
    );

-- Native edits, clearing and privacy-recovery deletion all update the same
-- conversation projection. A removed summary cannot leave an actionable receipt.
create function public.doctor_preparation_action_changed() returns trigger
language plpgsql set search_path=pg_catalog,public as $$
declare message_ids uuid[]; change_revision bigint;
begin
    select array_agg(distinct message_id) into message_ids
        from public.conversation_action_results
        where user_id=old.user_id and kind='prepare_doctor_summary';
    if message_ids is not null then
        change_revision := public.conversation_changed(old.user_id);
        update public.conversation_messages set revision=change_revision,updated_at=now()
            where user_id=old.user_id and id=any(message_ids);
        if TG_OP='DELETE' then
            delete from public.conversation_action_results
                where user_id=old.user_id and kind='prepare_doctor_summary';
        end if;
    end if;
    if TG_OP='DELETE' then return old; end if;
    return new;
end $$;
create trigger doctor_preparation_action_changed before update or delete
    on public.doctor_preparations for each row
    execute function public.doctor_preparation_action_changed();
revoke all on function public.doctor_preparation_action_changed() from public;

insert into public.schema_migrations(version) values('0044_conversation_doctor_preparation');
commit;
