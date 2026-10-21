-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Account deletion is one cascade from public.users under the runtime's
-- eight-second statement timeout. Each foreign key it passes through looks up
-- the referencing rows of every deleted parent row; without an index on the
-- referencing columns, each lookup scans the whole child table, so deleting a
-- long-used account grows with every account's data. These are the foreign
-- keys the cascade reaches that no index supported (Codex M4); a schema test
-- now fails for any new one.
--
-- Maintenance: building an index blocks writes to its table until it is built.
create index conversation_action_results_run
    on public.conversation_action_results(user_id, run_id);
create index conversation_citations_cycle
    on public.conversation_citations(cycle_id);
create index conversation_citations_record_revision
    on public.conversation_citations(record_revision);
create index conversation_continuations_through_message
    on public.conversation_continuations(user_id, through_message_id);
create index conversation_messages_run
    on public.conversation_messages(run_id);
create index conversation_runs_follow_up
    on public.conversation_runs(user_id, follow_up_id);
create index conversation_runs_selected_follow_up
    on public.conversation_runs(user_id, selected_follow_up_id);
create index conversation_runs_follow_up_occurrence
    on public.conversation_runs(user_id, follow_up_occurrence_id);
create index follow_up_occurrences_follow_up
    on public.follow_up_occurrences(user_id, follow_up_id);
create index follow_up_occurrences_conversation_run
    on public.follow_up_occurrences(user_id, conversation_run_id);
create index follow_up_occurrences_conversation_message
    on public.follow_up_occurrences(user_id, conversation_message_id);
create index follow_up_push_deliveries_user
    on public.follow_up_push_deliveries(user_id);
create index follow_up_push_deliveries_follow_up
    on public.follow_up_push_deliveries(follow_up_id);
create index follow_ups_routine
    on public.follow_ups(user_id, routine_id);
create index health_discovery_embeddings_document
    on public.health_discovery_embeddings(user_id, document_id, generation);
create index insight_cycles_assessment_run
    on public.insight_cycles(user_id, assessment_run_id);
create index operating_index_conversation_reads_index
    on public.operating_index_conversation_reads(user_id, index_id);
create index operating_index_conversation_reads_message
    on public.operating_index_conversation_reads(user_id, message_id);
create index operating_indexes_cycle
    on public.operating_indexes(user_id, cycle_id);
create index operating_indexes_conversation_run
    on public.operating_indexes(conversation_run_id);

insert into public.schema_migrations(version) values('0108_deletion_foreign_key_indexes');
commit;
