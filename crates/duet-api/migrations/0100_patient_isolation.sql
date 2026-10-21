begin;

-- Patient isolation: the database admits only the rows of the patient an
-- operation serves (docs/backend-architecture.md#patient-isolation).
--
-- The application sets app.account_id on every connection it acquires: from
-- the verified account for a request, from the job's stored owner for a worker
-- job. With no patient set, every policy below admits nothing. The runtime role
-- does not own these tables and has no BYPASSRLS, so the policies bind it; the
-- schema owner, which runs migrations, is not bound.
--
-- Planned maintenance: the serving release does not set app.account_id, so it
-- would see no patient rows once these policies exist.

create function public.acting_account() returns uuid
    language sql stable parallel safe
    as $$ select nullif(current_setting('app.account_id', true), '')::uuid $$;

-- The account itself.
alter table public.users enable row level security;
create policy patient_rows on public.users
    using (id = public.acting_account())
    with check (id = public.acting_account());

-- Bookkeeping views are the declared reads across patients: identifiers,
-- states and times, never what a patient wrote or what their records say. A
-- view runs with its owner's rights, so it is outside patient isolation; the
-- runtime role may only read it (scripts/gcp/split_database_roles.py).

-- The account's lifecycle: what the sign-in gate resolves before any patient
-- is being served, and what sweeps check before admitting work. Not the
-- person's name, sharing label, doctor note or chosen stats.
create view public.users_bookkeeping as
    select id, auth_user_id, setup_complete, intelligence_enabled,
           intelligence_control_revision, terms_version, terms_accepted_at,
           luna_consent_version, luna_consent_decision, luna_consent_at,
           evidence_revision, last_foreground_at, created_at, updated_at,
           sessions_revoked_before, sessions_revoked_confirmed_before,
           sessions_revocation_attempted_at
    from public.users;

-- Check-in schedules, without what a check-in asks or is titled.
create view public.follow_ups_bookkeeping as
    select id, user_id, revision, state, operation, next_occurrence_at,
           confirmation_required, created_at, updated_at
    from public.follow_ups;

create view public.follow_up_settings_bookkeeping as
    select user_id, enabled, revision, updated_at from public.follow_up_settings;

create view public.follow_up_occurrences_bookkeeping as
    select id, user_id, follow_up_id, follow_up_revision, scheduled_for, status,
           claim_until, conversation_run_id, completed_at, created_at, updated_at
    from public.follow_up_occurrences;

-- Health record connections: which portal and organization, state, claims
-- and the pending OAuth handshake, never credentials or the portal's patient
-- identifier.
create view public.source_connections_bookkeeping as
    select id, user_id, provider, instance_key, status, authorization_revision,
           granted_scopes, oauth_state_hash, oauth_state_expires_at, acquisition_claim_until,
           retry_not_before, last_synced_at, last_automatic_refresh_day,
           disconnected_at, created_at, updated_at
    from public.source_connections;

-- A held authorization code's one-time handle, without the code.
create view public.connector_authorization_completions_bookkeeping as
    select source_connection_id, handle_hash, authorization_revision, expires_at
    from public.connector_authorization_completions;

create view public.source_acquisition_units_bookkeeping as
    select id, connection_id, attempt_id, query_key, authorization_revision, state,
           not_before, expires_at, claim_until, last_completed_at,
           search_outcome is not null as search_warned, updated_at
    from public.source_acquisition_units;

-- Owed check-in notices per iPhone, without the push token.
create view public.follow_up_push_deliveries_bookkeeping as
    select message_id, user_id, created_at from public.follow_up_push_deliveries;

create view public.follow_up_push_installations_bookkeeping as
    select message_id, installation_id, state, attempted_at, next_attempt_at, settled_at
    from public.follow_up_push_installations;

-- Conversation work states and leases, without receipts or review triggers.
create view public.conversation_runs_bookkeeping as
    select id, user_id, message_id, assistant_message_id, follow_up_occurrence_id,
           review_trigger is not null as is_review, state, dispatch_lane, timezone,
           lease_until, delivery_generation, delivery_until, delivery_confirmed,
           started_at, terminal_reported_at, created_at, updated_at
    from public.conversation_runs;

-- The other job queues: owner, state, leases and delivery, never their output.
create view public.conversation_continuation_work_bookkeeping as
    select id, user_id, state, requested_through, evaluated_through, lease_until,
           delivery_generation, delivery_until, delivery_confirmed, accepted_at,
           updated_at
    from public.conversation_continuation_work;

create view public.doctor_preparation_work_bookkeeping as
    select id, user_id, state, lease_until, delivery_generation, delivery_until,
           delivery_confirmed, accepted_at, updated_at
    from public.doctor_preparation_work;

create view public.health_discovery_work_bookkeeping as
    select id, user_id, claim_until, delivery_generation, delivery_until,
           delivery_confirmed
    from public.health_discovery_work;

-- Whether an account's search index is behind: generations, counts and
-- recipe versions, never the indexed text.
create view public.evidence_browse_generations_bookkeeping as
    select user_id, generation, projected_generation
    from public.evidence_browse_generations;

create view public.health_discovery_pending_bookkeeping as
    select user_id, queued_at from public.health_discovery_pending;

create view public.health_discovery_recipes_bookkeeping as
    select document.user_id, term.embedding_recipe
    from public.health_discovery_documents document
    join public.health_discovery_terms term on term.document_id = document.id;

-- raw_object_path is the server's storage pointer to an upload (the import
-- id and the upload's sanitized file name); file maintenance reconciles
-- interrupted deletions by path, across accounts.
create view public.genome_imports_bookkeeping as
    select id, user_id, status, attempt_token, attempt_expires_at,
           delivery_generation, delivery_until, raw_object_path,
           created_at, updated_at
    from public.genome_imports;

-- The timezone a health context was read in, for scheduling.
create view public.health_contexts_bookkeeping as
    select user_id, timezone from public.health_contexts;

-- Understanding cycles in progress, without what they found.
create view public.insight_cycles_bookkeeping as
    select id, user_id, kind, status, lease_expires_at, assessment_run_id,
           created_at, updated_at
    from public.insight_cycles;

-- Outside isolation, each behind its own access boundary rather than the
-- patient being served: the privacy recovery journal (written by triggers for
-- both parties of a share, delivered across accounts by the recovery job, and
-- replayed after a restore with the schema owner's credentials), the deletion
-- fences (keyed by sign-in identity after the account is gone), an iPhone's
-- push registration (account_devices: the iPhone's row, authorized by its
-- installation and revision or its withdrawal key as it passes between the
-- accounts that sign in on it), and the global catalogs and operations
-- tables. Device tokens and these identifiers are still sensitive.
--
-- A change to an iPhone's registration ends that iPhone's owed notices and
-- records the previous account's receipt, whichever account's sign-in made
-- the change, so the trigger doing it runs with its owner's rights. A trigger
-- function cannot be called directly; only a registration change reaches it.
alter function public.record_push_restriction() security definer;

-- Rows owned directly by one patient.
alter table public.account_moments enable row level security;
create policy patient_rows on public.account_moments
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.canonical_evidence_revisions enable row level security;
create policy patient_rows on public.canonical_evidence_revisions
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.clinical_notes enable row level security;
create policy patient_rows on public.clinical_notes
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.conversation_action_results enable row level security;
create policy patient_rows on public.conversation_action_results
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.conversation_continuation_work enable row level security;
create policy patient_rows on public.conversation_continuation_work
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.conversation_continuations enable row level security;
create policy patient_rows on public.conversation_continuations
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.conversation_messages enable row level security;
create policy patient_rows on public.conversation_messages
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.conversation_reply_feedback enable row level security;
create policy patient_rows on public.conversation_reply_feedback
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.conversation_runs enable row level security;
create policy patient_rows on public.conversation_runs
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.conversation_state enable row level security;
create policy patient_rows on public.conversation_state
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.doctor_collection_items enable row level security;
create policy patient_rows on public.doctor_collection_items
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.doctor_preparation_work enable row level security;
create policy patient_rows on public.doctor_preparation_work
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.doctor_preparations enable row level security;
create policy patient_rows on public.doctor_preparations
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.evidence_browse_generations enable row level security;
create policy patient_rows on public.evidence_browse_generations
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.evidence_lab_projection_histories enable row level security;
create policy patient_rows on public.evidence_lab_projection_histories
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.evidence_lab_projection_rows enable row level security;
create policy patient_rows on public.evidence_lab_projection_rows
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.evidence_lab_projections enable row level security;
create policy patient_rows on public.evidence_lab_projections
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.evidence_library_projection_codings enable row level security;
create policy patient_rows on public.evidence_library_projection_codings
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.evidence_library_projection_entries enable row level security;
create policy patient_rows on public.evidence_library_projection_entries
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.evidence_library_projection_members enable row level security;
create policy patient_rows on public.evidence_library_projection_members
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.evidence_library_projections enable row level security;
create policy patient_rows on public.evidence_library_projections
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.evidence_sync_changes enable row level security;
create policy patient_rows on public.evidence_sync_changes
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.follow_up_occurrences enable row level security;
create policy patient_rows on public.follow_up_occurrences
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.follow_up_push_deliveries enable row level security;
create policy patient_rows on public.follow_up_push_deliveries
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.follow_up_settings enable row level security;
create policy patient_rows on public.follow_up_settings
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.follow_ups enable row level security;
create policy patient_rows on public.follow_ups
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.genome_findings enable row level security;
create policy patient_rows on public.genome_findings
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.genome_imports enable row level security;
create policy patient_rows on public.genome_imports
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.health_contexts enable row level security;
create policy patient_rows on public.health_contexts
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.health_discovery_documents enable row level security;
create policy patient_rows on public.health_discovery_documents
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.health_discovery_embeddings enable row level security;
create policy patient_rows on public.health_discovery_embeddings
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.health_discovery_pending enable row level security;
create policy patient_rows on public.health_discovery_pending
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.health_discovery_work enable row level security;
create policy patient_rows on public.health_discovery_work
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.health_evidence enable row level security;
create policy patient_rows on public.health_evidence
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.insight_cycles enable row level security;
create policy patient_rows on public.insight_cycles
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.operating_index_conversation_reads enable row level security;
create policy patient_rows on public.operating_index_conversation_reads
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.operating_indexes enable row level security;
create policy patient_rows on public.operating_indexes
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.saved_routines enable row level security;
create policy patient_rows on public.saved_routines
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
alter table public.source_connections enable row level security;
create policy patient_rows on public.source_connections
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());

-- Rows owned through their parent: visible exactly when the parent is.
alter table public.conversation_citations enable row level security;
create policy patient_rows on public.conversation_citations
    using (exists (select 1 from public.conversation_messages parent where parent.id = conversation_citations.message_id))
    with check (exists (select 1 from public.conversation_messages parent where parent.id = conversation_citations.message_id));
alter table public.health_discovery_terms enable row level security;
create policy patient_rows on public.health_discovery_terms
    using (exists (select 1 from public.health_discovery_documents parent where parent.id = health_discovery_terms.document_id))
    with check (exists (select 1 from public.health_discovery_documents parent where parent.id = health_discovery_terms.document_id));
alter table public.source_query_memberships enable row level security;
create policy patient_rows on public.source_query_memberships
    using (exists (select 1 from public.health_evidence parent where parent.id = source_query_memberships.evidence_id))
    with check (exists (select 1 from public.health_evidence parent where parent.id = source_query_memberships.evidence_id));
alter table public.connector_authorization_completions enable row level security;
create policy patient_rows on public.connector_authorization_completions
    using (exists (select 1 from public.source_connections parent where parent.id = connector_authorization_completions.source_connection_id))
    with check (exists (select 1 from public.source_connections parent where parent.id = connector_authorization_completions.source_connection_id));
alter table public.source_acquisition_units enable row level security;
create policy patient_rows on public.source_acquisition_units
    using (exists (select 1 from public.source_connections parent where parent.id = source_acquisition_units.connection_id))
    with check (exists (select 1 from public.source_connections parent where parent.id = source_acquisition_units.connection_id));
alter table public.source_acquisition_pages enable row level security;
create policy patient_rows on public.source_acquisition_pages
    using (exists (select 1 from public.source_acquisition_units parent where parent.id = source_acquisition_pages.unit_id))
    with check (exists (select 1 from public.source_acquisition_units parent where parent.id = source_acquisition_pages.unit_id));
alter table public.source_acquisition_staging enable row level security;
create policy patient_rows on public.source_acquisition_staging
    using (exists (select 1 from public.source_acquisition_units parent where parent.id = source_acquisition_staging.unit_id))
    with check (exists (select 1 from public.source_acquisition_units parent where parent.id = source_acquisition_staging.unit_id));
alter table public.follow_up_push_installations enable row level security;
create policy patient_rows on public.follow_up_push_installations
    using (exists (select 1 from public.follow_up_push_deliveries parent where parent.message_id = follow_up_push_installations.message_id))
    with check (exists (select 1 from public.follow_up_push_deliveries parent where parent.message_id = follow_up_push_installations.message_id));

-- Sharing: rows between two patients. Either party reads and may stop or
-- purge; only the sender writes encrypted payloads. Invitations and their
-- offered stats stay outside isolation behind their own boundary: possession
-- of the secret code authorizes preview and acceptance, since the recipient
-- is unknown until then.
alter table public.sharing_connections enable row level security;
create policy members on public.sharing_connections
    using (public.acting_account() in (lower_user_id, upper_user_id))
    with check (public.acting_account() in (lower_user_id, upper_user_id));

alter table public.sharing_blocks enable row level security;
create policy parties_read on public.sharing_blocks for select
    using (public.acting_account() in (blocker_user_id, blocked_user_id));
create policy blocker_inserts on public.sharing_blocks for insert
    with check (blocker_user_id = public.acting_account());
create policy blocker_updates on public.sharing_blocks for update
    using (blocker_user_id = public.acting_account())
    with check (blocker_user_id = public.acting_account());
create policy blocker_deletes on public.sharing_blocks for delete
    using (blocker_user_id = public.acting_account());

alter table public.sharing_grants enable row level security;
create policy parties on public.sharing_grants
    using (public.acting_account() in (sender_user_id, recipient_user_id))
    with check (public.acting_account() in (sender_user_id, recipient_user_id));

-- A grant's scopes are visible exactly when the grant is.
alter table public.sharing_grant_stats enable row level security;
create policy parties on public.sharing_grant_stats
    using (exists (select 1 from public.sharing_grants parent
                   where parent.id = sharing_grant_stats.grant_id))
    with check (exists (select 1 from public.sharing_grants parent
                        where parent.id = sharing_grant_stats.grant_id));

alter table public.sharing_payload_chunks enable row level security;
create policy parties_read on public.sharing_payload_chunks for select
    using (exists (select 1 from public.sharing_grant_stats parent
                   where parent.id = sharing_payload_chunks.scope_id));
create policy parties_purge on public.sharing_payload_chunks for delete
    using (exists (select 1 from public.sharing_grant_stats parent
                   where parent.id = sharing_payload_chunks.scope_id));
create policy sender_inserts on public.sharing_payload_chunks for insert
    with check (exists (select 1 from public.sharing_grant_stats scope
                        join public.sharing_grants grant_row on grant_row.id = scope.grant_id
                        where scope.id = sharing_payload_chunks.scope_id
                          and grant_row.sender_user_id = public.acting_account()));
create policy sender_updates on public.sharing_payload_chunks for update
    using (exists (select 1 from public.sharing_grant_stats scope
                   join public.sharing_grants grant_row on grant_row.id = scope.grant_id
                   where scope.id = sharing_payload_chunks.scope_id
                     and grant_row.sender_user_id = public.acting_account()))
    with check (exists (select 1 from public.sharing_grant_stats scope
                        join public.sharing_grants grant_row on grant_row.id = scope.grant_id
                        where scope.id = sharing_payload_chunks.scope_id
                          and grant_row.sender_user_id = public.acting_account()));

-- Who may see a person's sharing card: themselves, anyone they share a
-- connection or block with, and anyone holding their open invitation's code
-- (the preview shows who invited you). The card is the name, sharing label
-- and public key, never anything else on the account.
create function public.may_see_sharing_card(account uuid) returns boolean
    language sql stable
    as $$
      select account = public.acting_account()
        or exists (select 1 from public.sharing_connections c
                   where (c.lower_user_id = account and c.upper_user_id = public.acting_account())
                      or (c.upper_user_id = account and c.lower_user_id = public.acting_account()))
        or exists (select 1 from public.sharing_blocks b
                   where (b.blocker_user_id = account and b.blocked_user_id = public.acting_account())
                      or (b.blocked_user_id = account and b.blocker_user_id = public.acting_account()))
        or exists (select 1 from public.sharing_invitations i
                   where i.inviter_user_id = account and i.state = 'pending')
    $$;

alter table public.sharing_devices enable row level security;
create policy cards_read on public.sharing_devices for select
    using (public.may_see_sharing_card(user_id));
create policy owner_writes on public.sharing_devices for insert
    with check (user_id = public.acting_account());
create policy owner_updates on public.sharing_devices for update
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
create policy owner_deletes on public.sharing_devices for delete
    using (user_id = public.acting_account());

create view public.sharing_cards as
    select id, display_name, sharing_label from public.users
    where public.may_see_sharing_card(id);

-- Each account's live connection count, and nothing about whom it is with:
-- accepting an invitation checks the inviter's limit, which the accepting
-- patient cannot otherwise see. Sharing writes hold both accounts' sharing
-- locks, so the count cannot change while one is checked.
create view public.sharing_connection_counts as
    select account_id, count(*) as live_connections
    from (select lower_user_id as account_id, state from public.sharing_connections
          union all
          select upper_user_id, state from public.sharing_connections) member
    where state in ('active', 'reconnect_required')
    group by account_id;

-- Retention: revoked grants and scopes past their purge deadline.
create view public.sharing_grants_bookkeeping as
    select id, sender_user_id, recipient_user_id, state, revoked_at, purge_after
    from public.sharing_grants;

create view public.sharing_grant_stats_bookkeeping as
    select id, grant_id, revoked_at, purge_after from public.sharing_grant_stats;

do $$ declare view_name name; begin
  if exists (select 1 from pg_roles where rolname = 'duet_runtime') then
    -- The migration ledger is the owner's; the runtime only reads it.
    revoke all on public.schema_migrations from duet_runtime;
    grant select on public.schema_migrations to duet_runtime;
    for view_name in select c.relname from pg_class c
        join pg_namespace n on n.oid = c.relnamespace
        where n.nspname = 'public' and c.relkind = 'v' loop
      execute format('revoke insert, update, delete, truncate on public.%I from duet_runtime', view_name);
    end loop;
  end if;
end $$;

insert into public.schema_migrations(version) values ('0100_patient_isolation');
commit;
