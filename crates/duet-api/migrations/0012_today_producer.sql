-- Today assignments extend the existing accepted-work ledger. There is no
-- second run/lease/retry owner, and unpublished slots never enter insight_cycles.
alter table public.duet_commands
    add column work_kind text not null default 'message' check (work_kind in ('message', 'today')),
    add column today_assignment jsonb,
    add column today_staged jsonb not null default '{}',
    add column today_rereads integer not null default 0 check (today_rereads between 0 and 1),
    add column today_resume_after bigint not null default 0,
    add constraint duet_today_assignment_shape check ((work_kind = 'message' and today_assignment is null) or (work_kind = 'today' and input = '' and today_assignment is not null));

create index duet_today_commands on public.duet_commands (user_id, created_at)
where
    work_kind = 'today';

-- The five original calendar fields are the identity, not snapshot freshness.
-- An incompatible legacy import with duplicate published slots fails migration
-- rather than silently choosing one patient's retained presentation.
create unique index duet_today_published_slot on public.insight_cycles (user_id, kind, window_start, window_end, timezone, first_weekday);

-- Ordinary API roles retain SELECT only. The migration/grant owner separately
-- grants INSERT and UPDATE(corrected_insight_ids,updated_at) to the non-owner
-- producer role; no SECURITY DEFINER or RLS bypass is introduced here.
revoke insert, update, delete on public.insight_cycles from public;
