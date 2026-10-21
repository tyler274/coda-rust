BEGIN;

CREATE TABLE public.account_moments (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    user_id uuid NOT NULL,
    kind text NOT NULL,
    occurred_at timestamp with time zone NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT account_moments_kind_check
        CHECK (kind = 'first_summary_prepared'),
    CONSTRAINT account_moments_user_kind_key UNIQUE (user_id, kind),
    CONSTRAINT account_moments_pkey PRIMARY KEY (id),
    CONSTRAINT account_moments_user_id_fkey
        FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE
);

CREATE INDEX account_moments_user_occurred_idx
    ON public.account_moments (user_id, occurred_at DESC);

ALTER TABLE public.account_moments ENABLE ROW LEVEL SECURITY;

INSERT INTO public.schema_migrations (version)
VALUES ('0005_account_moments');

COMMIT;
