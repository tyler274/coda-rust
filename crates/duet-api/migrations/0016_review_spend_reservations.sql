begin;

-- A provider request must reserve its worst-case cost before dispatch.
-- Non-personal deployment totals intentionally survive account deletion:
-- deleting an account cannot refund money already spent or possibly spent.
create table public.review_deployment_spend (
    spend_day date primary key,
    charged_nanodollars bigint not null default 0 check(charged_nanodollars>=0),
    provider_unavailable_until timestamptz
);
create table public.review_allowances (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references public.users(id) on delete cascade,
    spend_day date not null,
    purpose text not null check(purpose in ('patient_review','doctor_preparation')),
    terms jsonb not null check(jsonb_typeof(terms)='object'),
    created_at timestamptz not null default now(),
    unique(user_id,spend_day,purpose)
);
create table public.review_request_reservations (
    id uuid primary key,
    allowance_id uuid not null references public.review_allowances(id) on delete cascade,
    input_ceiling bigint not null check(input_ceiling>0),
    output_ceiling bigint not null check(output_ceiling>0),
    reserved_nanodollars bigint not null check(reserved_nanodollars>=0),
    charged_nanodollars bigint not null check(charged_nanodollars>=0),
    actual_input bigint check(actual_input>=0),
    actual_output bigint check(actual_output>=0),
    reconciled boolean not null default false,
    created_at timestamptz not null default now(),
    check((actual_input is null)=(actual_output is null))
);
create index review_requests_allowance_idx on public.review_request_reservations(allowance_id);
revoke all on public.review_deployment_spend, public.review_allowances,
    public.review_request_reservations from public;
insert into public.schema_migrations(version) values('0016_review_spend_reservations');
commit;
