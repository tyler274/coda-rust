BEGIN;

ALTER TABLE public.health_evidence
    DROP CONSTRAINT health_evidence_source_kind_check;

ALTER TABLE public.health_evidence
    ADD CONSTRAINT health_evidence_source_kind_check
    CHECK (source_kind = ANY (ARRAY[
        'appleHealth'::text,
        'googleHealth'::text,
        'healthRecords'::text,
        'physical'::text,
        'manual'::text
    ]));

CREATE TABLE public.source_connections (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    user_id uuid NOT NULL REFERENCES public.users(id) ON DELETE CASCADE,
    provider text NOT NULL,
    status text DEFAULT 'pending'::text NOT NULL,
    external_user_id text,
    granted_scopes text[] DEFAULT '{}'::text[] NOT NULL,
    credentials_ciphertext bytea,
    oauth_state_hash text,
    oauth_state_expires_at timestamp with time zone,
    last_synced_at timestamp with time zone,
    last_failure_kind text,
    last_error_code text,
    retry_not_before timestamp with time zone,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT source_connections_provider_check
        CHECK (provider = 'googleHealth'::text),
    CONSTRAINT source_connections_status_check
        CHECK (status = ANY (ARRAY[
            'pending'::text,
            'connected'::text,
            'reauthorization_required'::text
        ])),
    CONSTRAINT source_connections_failure_kind_check
        CHECK (
            last_failure_kind IS NULL
            OR last_failure_kind = ANY (ARRAY[
                'temporary'::text,
                'reauthorization_required'::text,
                'account_not_linked'::text,
                'provider_contract_failure'::text
            ])
        ),
    CONSTRAINT source_connections_user_provider_key UNIQUE (user_id, provider)
);

ALTER TABLE public.source_connections ENABLE ROW LEVEL SECURITY;

CREATE UNIQUE INDEX source_connections_oauth_state_idx
    ON public.source_connections (oauth_state_hash)
    WHERE oauth_state_hash IS NOT NULL;

INSERT INTO public.schema_migrations (version)
VALUES ('0002_google_health_connector');

COMMIT;
