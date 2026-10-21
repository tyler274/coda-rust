-- One shared bounded bucket; no caller-controlled keys or patient content.
create table public.duet_public_budget (id integer primary key check(id=1),window_start timestamptz not null,used integer not null);
insert into public.duet_public_budget values(1,now(),0);
revoke all on public.duet_public_budget from public;
create function public.duet_admit_public_retrieval() returns boolean
language plpgsql security definer set search_path=pg_catalog,public as $$
declare admitted boolean;
begin
 update public.duet_public_budget set
 used=case when window_start<=now()-interval '60 seconds' then 1 else used+1 end,
 window_start=case when window_start<=now()-interval '60 seconds' then now() else window_start end
 where id=1 and (window_start<=now()-interval '60 seconds' or used<600)
 returning true into admitted;
 return coalesce(admitted,false);
end $$;
revoke all on function public.duet_admit_public_retrieval() from public;
