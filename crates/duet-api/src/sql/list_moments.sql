select
    id,
    kind,
    occurred_at
from
    public.account_moments
where
    user_id = $1
order by
    occurred_at desc,
    id desc
