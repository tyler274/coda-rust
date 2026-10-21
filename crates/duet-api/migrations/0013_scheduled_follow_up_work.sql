-- Scheduled accepted work rides the same command lease/effect owner, with no
-- fabricated user message. Source accepted authority remains separately frozen.
alter table public.duet_commands
    drop constraint duet_commands_work_kind_check;

alter table public.duet_commands
    add constraint duet_commands_work_kind_check check (work_kind in ('message', 'today', 'follow_up'));

alter table public.duet_commands
    drop constraint duet_today_assignment_shape;

alter table public.duet_commands
    add constraint duet_today_assignment_shape check ((work_kind in ('message', 'follow_up') and today_assignment is null) or (work_kind = 'today' and input = '' and today_assignment is not null));
