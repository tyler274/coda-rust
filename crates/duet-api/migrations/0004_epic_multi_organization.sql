BEGIN;

ALTER TABLE public.source_connections
    ADD COLUMN instance_key text NOT NULL DEFAULT 'default',
    ADD COLUMN display_name text,
    ADD COLUMN provider_context jsonb NOT NULL DEFAULT '{}'::jsonb;

UPDATE public.source_connections
SET instance_key = 'epic-sandbox', display_name = 'Epic sandbox'
WHERE provider = 'epic';

ALTER TABLE public.source_connections
    DROP CONSTRAINT source_connections_user_provider_key;

ALTER TABLE public.source_connections
    ADD CONSTRAINT source_connections_user_provider_instance_key
    UNIQUE (user_id, provider, instance_key);

CREATE INDEX source_connections_user_provider_status_idx
    ON public.source_connections (user_id, provider, status);

-- Derived artifacts retain exact connector provenance. This lets removing one
-- MyChart organization invalidate only artifacts that actually used it.
ALTER TABLE public.insight_cycles
    ADD COLUMN source_connection_ids text[] DEFAULT '{}'::text[] NOT NULL;

ALTER TABLE public.doctor_collection_items
    ADD COLUMN source_connection_ids text[] DEFAULT '{}'::text[] NOT NULL;

CREATE INDEX insight_cycles_source_connection_ids_idx
    ON public.insight_cycles USING gin (source_connection_ids);

CREATE INDEX doctor_collection_source_connection_ids_idx
    ON public.doctor_collection_items USING gin (source_connection_ids);

CREATE TABLE public.epic_organizations (
    id text PRIMARY KEY,
    name text NOT NULL,
    fhir_base_url text NOT NULL,
    search_terms text[] DEFAULT '{}'::text[] NOT NULL,
    active boolean DEFAULT true NOT NULL,
    source_updated_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT epic_organizations_https_check
        CHECK (fhir_base_url ~ '^https://')
);

ALTER TABLE public.epic_organizations ENABLE ROW LEVEL SECURITY;

CREATE INDEX epic_organizations_name_idx
    ON public.epic_organizations (lower(name));

INSERT INTO public.schema_migrations (version)
VALUES ('0004_epic_multi_organization');

COMMIT;
