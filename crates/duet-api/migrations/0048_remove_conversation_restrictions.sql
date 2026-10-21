-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Scoped conversation forgetting is removed rather than repaired. The visible
-- transcript, the account-level privacy recovery journal and every AI-permission
-- fence are unchanged; only the restriction projection and its storage go.
-- Maintenance mode: the previous image still writes the columns dropped here.

-- History of the removed feature is disposed of explicitly before the
-- constraints that forbid it. Its action receipts and journal events go with
-- the restriction rows; the visible transcript and every other journal
-- operation stay.
delete from public.conversation_action_results
    where kind in ('forget_conversation_detail','restore_conversation_detail');
delete from public.privacy_recovery_events
    where operation in ('conversation_forget','conversation_forget_restore');

alter table public.conversation_action_results
    drop constraint conversation_action_results_kind_check,
    drop constraint conversation_action_results_target_check,
    drop constraint conversation_action_results_restriction_fkey,
    drop column restriction_id,
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

drop table public.conversation_context_restriction_follow_ups;
drop table public.follow_up_context_restrictions;
drop table public.conversation_context_restriction_items;
drop table public.conversation_context_restriction_spans;
drop table public.conversation_context_restrictions;

-- Those tables carried the only triggers for the restriction-driven discovery
-- invalidation. The per-source invalidation function is untouched.
drop function public.health_discovery_restriction_changed();
drop function public.invalidate_health_discovery_restriction(uuid,uuid);

-- A receipt meant "all text eligible under the restriction policy was read",
-- which could be part of a message. Only a receipt covering the whole retained
-- text keeps its meaning under the new reader; every other receipt is
-- discarded, so that input returns to the unreviewed window once. The
-- source-text hash stays as the receipt's integrity check.
delete from public.operating_index_conversation_reads reads
    using public.conversation_messages message
    where message.user_id=reads.user_id and message.id=reads.message_id
      and reads.spans <> jsonb_build_array(
            jsonb_build_object('start',0,'end',length(message.text)));
alter table public.operating_index_conversation_reads
    drop column spans,
    drop column context_policy_revision;

alter table public.follow_ups drop column source_spans;

-- Conversation message identities were accumulated only for restriction
-- invalidation. Source, routine and published-cycle dependencies stay.
create or replace function public.remove_deleted_conversation_source() returns trigger
language plpgsql set search_path = public as $$
declare account_id uuid; change_revision bigint;
begin
    select user_id into account_id from public.conversation_messages
    where id=old.message_id;
    if account_id is null then return old; end if;
    change_revision := public.conversation_changed(account_id);
    update public.conversation_messages set
        sources = coalesce((select jsonb_agg(value) from jsonb_array_elements(sources)
                            where value->>'id' <> old.source_id), '[]'::jsonb),
        revision = change_revision, updated_at = now()
    where id=old.message_id and change_revision is not null;
    return old;
end;
$$;
revoke all on function public.remove_deleted_conversation_source() from public;

alter table public.operating_indexes drop column conversation_message_ids;

insert into public.schema_migrations(version)
values('0048_remove_conversation_restrictions');
commit;
