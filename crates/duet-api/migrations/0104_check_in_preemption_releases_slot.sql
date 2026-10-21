-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- The person's preemption never uses up a Check-in (North Star, October 1,
-- extending September 29). Admission advances a Check-in past its slot, and a
-- one-time Check-in becomes completed, before the attempt runs. An attempt the
-- person's newer message or their proactive pause interrupted decided nothing,
-- so its slot returns to the Check-in and admission owes it again once the
-- conversation is quiet. Authority changes (AI permission, consent, terms) and
-- their privacy-restore replays interrupt without releasing: a restore never
-- reopens work. Endings Duet reaches itself stay terminal.
--
-- An interrupted attempt therefore no longer holds its slot's identity: the
-- next attempt is a new occurrence at the same scheduled time, while the
-- interrupted occurrence and its run remain the record of what happened.
--
-- Maintenance: replacing the identity constraint builds a new unique index on
-- an existing table.
alter table public.follow_up_occurrences
    drop constraint follow_up_occurrence_identity;
create unique index follow_up_occurrence_identity
    on public.follow_up_occurrences(follow_up_id,follow_up_revision,scheduled_for)
    where status<>'interrupted';

-- Only a Check-in still at the interrupted revision and still active, or made
-- completed by that admission, gets its slot back; the person's own edit,
-- pause or cancel already decided its next state. Reopening a one-time
-- Check-in cannot exceed the open-follow-up cap: its admission closed it, and
-- no run without the person's message can open another.
create function public.release_interrupted_follow_ups(
    account_id uuid, occurrence_ids uuid[]
) returns void
language plpgsql as $$
begin
    update public.follow_ups f set state='active',
        next_occurrence_at=least(coalesce(f.next_occurrence_at,o.scheduled_for),o.scheduled_for),
        updated_at=now()
    from public.follow_up_occurrences o
    where o.user_id=account_id and o.id=any(occurrence_ids) and o.status='interrupted'
      and f.user_id=o.user_id and f.id=o.follow_up_id
      and f.revision=o.follow_up_revision and f.state in ('active','completed');
end $$;

-- Only the person's message asks for the release; the authority triggers keep
-- calling with the defaults.
drop function public.interrupt_conversation(uuid,boolean);
create function public.interrupt_conversation(
    account_id uuid, reviews_only boolean default false,
    release_follow_ups boolean default false
) returns void
language plpgsql as $$
declare change_revision bigint;
declare released uuid[];
begin
    change_revision := public.conversation_changed(account_id);
    if change_revision is null then return; end if;
    delete from public.insight_cycles
    where user_id=account_id and status='generating'
      and assessment_run_id in (
          select id from public.conversation_runs
          where user_id=account_id and state in ('queued','running')
            and (not reviews_only or review_trigger is not null)
      );
    with ended as (
        update public.conversation_runs set state='interrupted',failure='interrupted',
            preempted_at=now(),lease_until=null,revision=change_revision,updated_at=now()
        where user_id=account_id and state in ('queued','running')
          and (not reviews_only or review_trigger is not null)
        returning assistant_message_id,follow_up_occurrence_id
    ), messages as (
        update public.conversation_messages set state='interrupted',
            revision=change_revision,updated_at=now()
        where id in (select assistant_message_id from ended)
    ), occurrences as (
        update public.follow_up_occurrences set status='interrupted',
            completed_at=now(),updated_at=now()
        where id in (select follow_up_occurrence_id from ended)
          and status in('queued','running')
        returning id
    )
    select coalesce(array_agg(id),'{}') into released from occurrences;
    if release_follow_ups then
        perform public.release_interrupted_follow_ups(account_id, released);
    end if;
end $$;

insert into public.schema_migrations(version) values('0104_check_in_preemption_releases_slot');
commit;
