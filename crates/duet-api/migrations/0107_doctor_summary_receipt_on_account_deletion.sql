-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Removing a doctor preparation revises the replies whose receipts prepared
-- it. During account deletion the cascade reaches the preparation after the
-- account row is gone, conversation_changed (0037) returns no revision, and
-- writing that null into conversation_messages.revision rolled the whole
-- deletion back. Like every other caller, revise only when a conversation
-- remains; the account's messages and receipts cascade away with it.
--
-- Online: a function body. The serving runtime calls nothing new, and its
-- deletions now commit.
create or replace function public.doctor_preparation_action_changed() returns trigger
language plpgsql set search_path=pg_catalog,public as $$
declare message_ids uuid[]; change_revision bigint;
begin
    select array_agg(distinct message_id) into message_ids
        from public.conversation_action_results
        where user_id=old.user_id and kind='prepare_doctor_summary';
    if message_ids is not null then
        change_revision := public.conversation_changed(old.user_id);
        if change_revision is not null then
            update public.conversation_messages set revision=change_revision,updated_at=now()
                where user_id=old.user_id and id=any(message_ids);
            if TG_OP='DELETE' then
                delete from public.conversation_action_results
                    where user_id=old.user_id and kind='prepare_doctor_summary';
            end if;
        end if;
    end if;
    if TG_OP='DELETE' then return old; end if;
    return new;
end $$;

insert into public.schema_migrations(version) values('0107_doctor_summary_receipt_on_account_deletion');
commit;
