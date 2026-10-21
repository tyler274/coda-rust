-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Predefined reviews use the conversation execution owner. An unpublished
-- review has no assistant message; quiet completion never creates a placeholder.
alter table public.conversation_runs
    alter column assistant_message_id drop not null,
    add column review_trigger jsonb,
    add column review_key text,
    drop constraint conversation_runs_assignment_check,
    add constraint conversation_runs_assignment_check check(
        (message_id is not null and assistant_message_id is not null
            and follow_up_id is null and follow_up_revision is null
            and follow_up_occurrence_id is null and assignment_time is null
            and review_trigger is null and review_key is null)
        or
        (message_id is null and assistant_message_id is not null
            and follow_up_id is not null and follow_up_revision is not null
            and follow_up_occurrence_id is not null and assignment_time is not null
            and review_trigger is null and review_key is null)
        or
        (message_id is null and follow_up_id is null and follow_up_revision is null
            and follow_up_occurrence_id is null and selected_follow_up_id is null
            and assignment_time is not null and review_key is not null
            and review_trigger is not null and jsonb_typeof(review_trigger)='object'
            and review_trigger ? 'kind'
            and review_trigger->>'kind' in ('new_information','weekly'))
    );
create unique index conversation_review_identity
    on public.conversation_runs(user_id,review_key) where review_key is not null;

insert into public.schema_migrations(version) values('0041_proactive_conversation_reviews');
commit;
