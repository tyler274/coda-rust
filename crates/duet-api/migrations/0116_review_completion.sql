-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Pausing retains accepted work. Existing terminal history is not revived.
alter table public.conversation_runs
    add column paused_at timestamptz,
    add column retry_deadline_at timestamptz,
    drop constraint conversation_runs_state_check,
    add constraint conversation_runs_state_check
        check (state in ('queued','running','paused','completed','interrupted','failed','cancelled'));
update public.conversation_runs set retry_deadline_at=started_at+interval '30 minutes'
    where started_at is not null and review_trigger is null and state in ('queued','running');
alter table public.model_execution_switch alter column changed_at drop default;

-- This remains the single interruption owner. The third argument now retains
-- the accepted work; it no longer rewinds schedules to create replacements.
create or replace function public.interrupt_conversation_runs(
    account_id uuid, run_ids uuid[], release_follow_ups boolean default false
) returns void language plpgsql as $$
declare change_revision bigint; pause_time timestamptz;
begin
    change_revision := public.conversation_changed(account_id);
    if change_revision is null then return; end if;
    perform id from public.conversation_runs
        where user_id=account_id and id=any(run_ids) for update;
    -- Triggers cannot receive a Python clock sample. Read the database clock
    -- here, after the account and run locks, for every caller of this owner.
    pause_time := clock_timestamp();
    with ended as (
        update public.conversation_runs set state=case when release_follow_ups then 'paused' else 'interrupted' end,
            failure=case when not release_follow_ups then 'interrupted' end,
            paused_at=case when release_follow_ups then coalesce(paused_at,pause_time) end,
            lease_until=null,revision=change_revision,updated_at=now()
        where user_id=account_id and id=any(run_ids)
          and state in ('queued','running','paused')
        returning id,assistant_message_id,follow_up_occurrence_id
    ), messages as (
        update public.conversation_messages set state='interrupted',
            revision=change_revision,updated_at=now()
        where id in (select assistant_message_id from ended) and state<>'complete'
    ), cycles as (
        update public.insight_cycles set
            status=case when release_follow_ups then 'generating' else 'failed' end,
            claim_token=null,lease_expires_at=null,updated_at=now()
        where assessment_run_id in (select id from ended) and status='generating'
    )
    update public.follow_up_occurrences set
        status=case when release_follow_ups then 'queued' else 'interrupted' end,
        completed_at=case when not release_follow_ups then now() end,updated_at=now()
    where id in (select follow_up_occurrence_id from ended)
      and status in ('queued','running');
end $$;

create or replace function public.interrupt_conversation(
    account_id uuid, reviews_only boolean default false,
    release_follow_ups boolean default false
) returns void language plpgsql as $$
begin
    -- A newer message replaces an older reply but holds background work.
    perform public.interrupt_conversation_runs(account_id,
        array(select id from public.conversation_runs where user_id=account_id
              and state in ('queued','running','paused')
              and (not reviews_only or review_trigger is not null)
              and (not release_follow_ups or message_id is not null)), false);
    if release_follow_ups then
        perform public.interrupt_conversation_runs(account_id,
            array(select id from public.conversation_runs where user_id=account_id
                  and state in ('queued','running') and message_id is null
                  and (not reviews_only or review_trigger is not null)), true);
    end if;
end $$;
drop function public.release_interrupted_follow_ups(uuid,uuid[]);

-- Every conversation uses its claimed record cursor. Only a correction to
-- something actually used needs a reread. Consent/deletion still end work.
create or replace function public.conversation_account_authority_changed() returns trigger
language plpgsql as $$
begin
    if tg_op='DELETE' then
        perform pg_notify('duet_conversation',old.id::text);
        return old;
    end if;
    if (new.luna_consent_version,new.luna_consent_decision)
       is distinct from
       (old.luna_consent_version,old.luna_consent_decision) then
        perform public.interrupt_conversation(new.id, false);
    elsif (new.terms_version,new.terms_accepted_at) is distinct from
          (old.terms_version,old.terms_accepted_at) then
        perform public.interrupt_conversation_runs(new.id,
            array(select id from public.conversation_runs where user_id=new.id
                  and state in ('queued','running')), true);
    elsif new.intelligence_enabled is distinct from old.intelligence_enabled then
        if not new.intelligence_enabled then
            perform public.interrupt_conversation_runs(new.id,
                array(select id from public.conversation_runs where user_id=new.id
                      and state in ('queued','running')), true);
        else
            perform public.conversation_changed(new.id);
        end if;
    elsif new.intelligence_control_revision is distinct from old.intelligence_control_revision then
        perform public.interrupt_conversation(new.id, false);
    elsif new.evidence_revision is distinct from old.evidence_revision then
        perform pg_notify('duet_conversation',new.id::text);
    end if;
    return new;
end $$;

create or replace function public.conversation_context_changed() returns trigger
language plpgsql as $$
declare change_revision bigint;
begin
    change_revision := public.conversation_changed(old.user_id);
    if change_revision is not null then
        with ended as (
            update public.conversation_runs set state='failed',failure='unavailable',
                paused_at=null,lease_until=null,revision=change_revision,updated_at=now()
            where user_id=old.user_id and state in ('queued','running','paused')
              and message_id is not null
            returning assistant_message_id
        )
        update public.conversation_messages set state='failed',
            revision=change_revision,updated_at=now()
        where id in (select assistant_message_id from ended) and state<>'complete';
    end if;
    perform public.interrupt_conversation(old.user_id, false);
    return old;
end $$;

-- A maintenance upgrade may begin with the operator switch already off.
select public.interrupt_conversation_runs(r.user_id,array_agg(r.id),true)
from public.conversation_runs r
left join public.follow_ups f on f.id=r.follow_up_id and f.revision=r.follow_up_revision
where r.state in ('queued','running') and f.operation is distinct from 'reminder'
  and not coalesce((select enabled from public.model_execution_switch order by id desc limit 1),false)
group by r.user_id;

insert into public.schema_migrations(version) values('0116_review_completion');
commit;
