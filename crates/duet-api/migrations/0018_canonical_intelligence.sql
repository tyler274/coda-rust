begin;

-- The insight cycle is the only generation lifecycle. Remove the never-routed
-- review scheduler, budget ledger, and evaluation cache instead of preserving a
-- second dormant owner beside it.
alter table public.users
    add column intelligence_enabled boolean not null default true,
    add column intelligence_control_revision bigint not null default 0
        check (intelligence_control_revision >= 0);

-- Review-owned indexes cannot truthfully be attached to an insight cycle.
-- Remove that internal working memory with its retired lifecycle instead of
-- manufacturing provenance or leaving the migration dependent on an empty DB.
delete from public.patient_indexes;

drop trigger canonical_revision_supersede_indexes
    on public.canonical_evidence_revisions;
drop trigger native_snapshot_insert_supersede_indexes
    on public.health_contexts;
drop trigger native_snapshot_update_supersede_indexes
    on public.health_contexts;
drop trigger canonical_revision_invalidate_indexes
    on public.canonical_evidence_revisions;
drop function public.supersede_patient_index_inputs();
drop function public.invalidate_patient_indexes();

alter table public.patient_indexes
    drop constraint if exists patient_indexes_user_id_review_id_fkey,
    drop column review_id,
    add column cycle_id uuid,
    add column context_snapshot_id uuid,
    add column history_cursor bigint not null default 0
        check (history_cursor >= 0),
    add column control_revision bigint not null default 0
        check (control_revision >= 0);
alter table public.patient_indexes rename to operating_indexes;
alter index public.patient_indexes_current_idx rename to operating_indexes_current_idx;
alter index public.patient_indexes_dependencies_idx rename to operating_indexes_dependencies_idx;
alter table public.operating_indexes
    add constraint operating_indexes_cycle_owner_fkey
        foreign key (user_id, cycle_id)
        references public.insight_cycles(user_id, id) on delete cascade,
    alter column cycle_id set not null;

create unique index operating_indexes_canonical_current_idx
    on public.operating_indexes(user_id);

drop table public.patient_reviews;
drop table public.review_request_reservations;
drop table public.review_allowances;
drop table public.review_deployment_spend;
drop table public.insight_fact_evaluations;
alter table public.users
    drop column review_enabled,
    drop column review_control_revision,
    drop column review_join_until,
    drop column review_disclosure_version,
    drop column review_consent_at,
    drop column doctor_claim_token,
    drop column doctor_claim_until;

-- New source material marks the opaque index as superseded working context;
-- source withdrawal deletes it through exact inherited receipt dependencies.
create function public.supersede_operating_index_inputs() returns trigger
language plpgsql set search_path=pg_catalog,public as $$
declare owner_id uuid;
begin
    owner_id := case when TG_OP='DELETE' then old.user_id else new.user_id end;
    perform id from public.users where id=owner_id for update;
    update public.operating_indexes set superseded_at=now()
        where user_id=owner_id and superseded_at is null;
    return null;
end;
$$;
revoke all on function public.supersede_operating_index_inputs() from public;
create trigger canonical_revision_supersede_operating_index after insert
on public.canonical_evidence_revisions for each row
execute function public.supersede_operating_index_inputs();
create trigger native_snapshot_insert_supersede_operating_index after insert or delete
on public.health_contexts for each row
execute function public.supersede_operating_index_inputs();
create trigger native_snapshot_update_supersede_operating_index after update
on public.health_contexts for each row when(old.snapshot_id is distinct from new.snapshot_id)
execute function public.supersede_operating_index_inputs();

create function public.invalidate_operating_indexes() returns trigger
language plpgsql set search_path=pg_catalog,public as $$
declare owner_id uuid; record_id uuid;
begin
    owner_id := old.user_id;
    record_id := coalesce(old.evidence_id,old.note_id);
    perform id from public.users where id=owner_id for update;
    delete from public.operating_indexes where user_id=owner_id
      and source_record_ids @> array[record_id];
    return null;
end;
$$;
revoke all on function public.invalidate_operating_indexes() from public;
create trigger canonical_revision_invalidate_operating_indexes after delete
on public.canonical_evidence_revisions for each row
execute function public.invalidate_operating_indexes();

-- A cycle is the only generation/idempotency owner. A ready cycle may be a
-- successful private-index update with no patient-facing communication and
-- therefore no native supporting facts. Failed provider outcomes are terminal
-- for that presentation slot so an ambiguous response cannot buy another call.
alter table public.insight_cycles
    add column evidence_revision bigint not null default 0
        check (evidence_revision >= 0),
    add column history_cursor bigint not null default 0
        check (history_cursor >= 0),
    add column control_revision bigint not null default 0
        check (control_revision >= 0),
    drop constraint insight_cycles_user_id_candidate_fingerprint_key,
    drop constraint insight_cycles_status_check,
    drop constraint insight_cycles_included_concept_ids_check,
    drop constraint insight_cycles_ready_payload_check,
    add constraint insight_cycles_status_check
        check (status in ('generating', 'ready', 'failed')),
    add constraint insight_cycles_ready_payload_check check (
        status = 'generating'
        or status = 'failed'
        or (
            jsonb_array_length(insights) between 0 and 3
            and observed_through is not null
            and generated_at is not null
            and (
                (jsonb_array_length(insights) = 0
                    and jsonb_array_length(evidence) = 0)
                or (jsonb_array_length(insights) > 0
                    and jsonb_array_length(supporting_facts) > 0
                    and jsonb_array_length(evidence) > 0)
            )
        )
    );

-- This legacy column now fingerprints the exact source/context/control input
-- boundary. Different daily or weekly presentation slots may legitimately use
-- the same boundary; the slot key remains the sole public idempotency key.
comment on column public.insight_cycles.candidate_fingerprint is
    'SHA-256 of the claimed evidence, native context, and control boundary';

insert into public.schema_migrations(version)
values ('0018_canonical_intelligence');
commit;
