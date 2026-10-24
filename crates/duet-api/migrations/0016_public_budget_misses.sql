-- A second window for failed public lookups. The hit bucket stays row id = 1
-- and duet_admit_public_retrieval, defined in 0006. Raw SQL is this existing
-- plpgsql bucket, not ordinary CRUD.
alter table public.duet_public_budget
    drop constraint duet_public_budget_id_check;

alter table public.duet_public_budget
    add constraint duet_public_budget_id_check check (id in (1, 2));

insert into public.duet_public_budget (id, window_start, used)
    values (2, now(), 0);

create function public.duet_admit_public_retrieval_miss ()
    returns boolean
    language plpgsql
    security definer
    set search_path = pg_catalog, public
    as $$
declare
    admitted boolean;
begin
    update
        public.duet_public_budget
    set
        used = case when window_start <= now() - interval '60 seconds' then
            1
        else
            used + 1
        end,
        window_start = case when window_start <= now() - interval '60 seconds' then
            now()
        else
            window_start
        end
    where
        id = 2
        and (window_start <= now() - interval '60 seconds'
            or used < 600)
    returning
        true
    into
        admitted;
    return coalesce(admitted, false);
end
$$;

revoke all on function public.duet_admit_public_retrieval_miss () from public;
