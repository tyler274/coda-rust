BEGIN;

ALTER TABLE public.health_evidence
    DROP CONSTRAINT health_evidence_source_kind_check;

ALTER TABLE public.health_evidence
    ADD CONSTRAINT health_evidence_source_kind_check
    CHECK (source_kind = ANY (ARRAY[
        'appleHealth'::text,
        'googleHealth'::text,
        'epic'::text,
        'healthRecords'::text,
        'physical'::text,
        'manual'::text
    ]));

ALTER TABLE public.source_connections
    DROP CONSTRAINT source_connections_provider_check;

ALTER TABLE public.source_connections
    ADD CONSTRAINT source_connections_provider_check
    CHECK (provider = ANY (ARRAY[
        'googleHealth'::text,
        'epic'::text
    ]));

INSERT INTO public.schema_migrations (version)
VALUES ('0003_epic_connector');

COMMIT;
