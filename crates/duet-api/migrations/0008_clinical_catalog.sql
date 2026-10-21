BEGIN;

CREATE TABLE public.clinical_catalog_releases (
    id text PRIMARY KEY,
    status text NOT NULL DEFAULT 'inactive',
    source_manifest jsonb NOT NULL,
    source_sha256 text NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT now(),
    activated_at timestamp with time zone,
    CONSTRAINT clinical_catalog_releases_status_check
      CHECK (status = ANY (ARRAY['inactive'::text, 'active'::text, 'retired'::text])),
    CONSTRAINT clinical_catalog_releases_manifest_check
      CHECK (jsonb_typeof(source_manifest) = 'object'),
    CONSTRAINT clinical_catalog_releases_sha_check
      CHECK (source_sha256 ~ '^[0-9a-f]{64}$'),
    CONSTRAINT clinical_catalog_releases_activation_check
      CHECK (
        (status = 'inactive' AND activated_at IS NULL)
        OR (status = 'active' AND activated_at IS NOT NULL)
        OR status = 'retired'
      )
);

CREATE UNIQUE INDEX clinical_catalog_one_active_release_idx
    ON public.clinical_catalog_releases ((status))
    WHERE status = 'active';

CREATE TABLE public.clinical_catalog_concepts (
    release_id text NOT NULL REFERENCES public.clinical_catalog_releases(id)
      ON DELETE CASCADE,
    id text NOT NULL,
    domains text[] NOT NULL,
    preferred_label text NOT NULL,
    source_receipts jsonb NOT NULL DEFAULT '[]'::jsonb,
    PRIMARY KEY (release_id, id),
    CONSTRAINT clinical_catalog_concepts_id_check CHECK (btrim(id) <> ''),
    CONSTRAINT clinical_catalog_concepts_domains_check
      CHECK (cardinality(domains) > 0 AND NOT domains @> ARRAY['']::text[]),
    CONSTRAINT clinical_catalog_concepts_label_check
      CHECK (btrim(preferred_label) <> ''),
    CONSTRAINT clinical_catalog_concepts_receipts_check
      CHECK (jsonb_typeof(source_receipts) = 'array')
);

CREATE TABLE public.clinical_catalog_codings (
    release_id text NOT NULL,
    concept_id text NOT NULL,
    system text NOT NULL,
    code text NOT NULL,
    display text,
    source_version text,
    source_id text NOT NULL,
    roles text[] NOT NULL DEFAULT ARRAY['identity']::text[],
    FOREIGN KEY (release_id, concept_id)
      REFERENCES public.clinical_catalog_concepts(release_id, id)
      ON DELETE CASCADE,
    PRIMARY KEY (release_id, system, code),
    CONSTRAINT clinical_catalog_codings_system_check CHECK (btrim(system) <> ''),
    CONSTRAINT clinical_catalog_codings_code_check CHECK (btrim(code) <> ''),
    CONSTRAINT clinical_catalog_codings_source_check CHECK (btrim(source_id) <> ''),
    CONSTRAINT clinical_catalog_codings_roles_check
      CHECK (cardinality(roles) > 0 AND NOT roles @> ARRAY['']::text[])
);

CREATE INDEX clinical_catalog_codings_concept_idx
    ON public.clinical_catalog_codings (release_id, concept_id);

CREATE TABLE public.clinical_catalog_aliases (
    release_id text NOT NULL,
    concept_id text NOT NULL,
    alias text NOT NULL,
    normalized_alias text NOT NULL,
    source_id text NOT NULL,
    provenance_kind text NOT NULL,
    rank_weight integer NOT NULL DEFAULT 0,
    FOREIGN KEY (release_id, concept_id)
      REFERENCES public.clinical_catalog_concepts(release_id, id)
      ON DELETE CASCADE,
    PRIMARY KEY (release_id, concept_id, normalized_alias, source_id),
    CONSTRAINT clinical_catalog_aliases_alias_check CHECK (btrim(alias) <> ''),
    CONSTRAINT clinical_catalog_aliases_normalized_check
      CHECK (btrim(normalized_alias) <> ''),
    CONSTRAINT clinical_catalog_aliases_source_check CHECK (btrim(source_id) <> ''),
    CONSTRAINT clinical_catalog_aliases_provenance_check
      CHECK (btrim(provenance_kind) <> '')
);

CREATE INDEX clinical_catalog_aliases_lookup_idx
    ON public.clinical_catalog_aliases (release_id, normalized_alias);

CREATE TABLE public.clinical_catalog_facets (
    release_id text NOT NULL REFERENCES public.clinical_catalog_releases(id)
      ON DELETE CASCADE,
    id text NOT NULL,
    parent_id text,
    label text NOT NULL,
    display_order integer NOT NULL,
    source_receipt jsonb NOT NULL,
    PRIMARY KEY (release_id, id),
    FOREIGN KEY (release_id, parent_id)
      REFERENCES public.clinical_catalog_facets(release_id, id)
      DEFERRABLE INITIALLY DEFERRED,
    CONSTRAINT clinical_catalog_facets_id_check CHECK (btrim(id) <> ''),
    CONSTRAINT clinical_catalog_facets_label_check CHECK (btrim(label) <> ''),
    CONSTRAINT clinical_catalog_facets_order_check CHECK (display_order >= 0),
    CONSTRAINT clinical_catalog_facets_receipt_check
      CHECK (jsonb_typeof(source_receipt) = 'object')
);

CREATE TABLE public.clinical_catalog_concept_facets (
    release_id text NOT NULL,
    concept_id text NOT NULL,
    facet_id text NOT NULL,
    FOREIGN KEY (release_id, concept_id)
      REFERENCES public.clinical_catalog_concepts(release_id, id)
      ON DELETE CASCADE,
    FOREIGN KEY (release_id, facet_id)
      REFERENCES public.clinical_catalog_facets(release_id, id)
      ON DELETE CASCADE,
    PRIMARY KEY (release_id, concept_id, facet_id)
);

CREATE TABLE public.clinical_catalog_relationships (
    release_id text NOT NULL REFERENCES public.clinical_catalog_releases(id)
      ON DELETE CASCADE,
    id text NOT NULL,
    relationship_kind text NOT NULL,
    source_system text NOT NULL,
    source_code text NOT NULL,
    target_system text NOT NULL,
    target_code text NOT NULL,
    source_id text NOT NULL,
    source_version text,
    metadata jsonb NOT NULL DEFAULT '{}'::jsonb,
    PRIMARY KEY (release_id, id),
    CONSTRAINT clinical_catalog_relationships_kind_check
      CHECK (btrim(relationship_kind) <> ''),
    CONSTRAINT clinical_catalog_relationships_source_system_check
      CHECK (btrim(source_system) <> ''),
    CONSTRAINT clinical_catalog_relationships_source_code_check
      CHECK (btrim(source_code) <> ''),
    CONSTRAINT clinical_catalog_relationships_target_system_check
      CHECK (btrim(target_system) <> ''),
    CONSTRAINT clinical_catalog_relationships_target_code_check
      CHECK (btrim(target_code) <> ''),
    CONSTRAINT clinical_catalog_relationships_source_id_check
      CHECK (btrim(source_id) <> ''),
    CONSTRAINT clinical_catalog_relationships_metadata_check
      CHECK (jsonb_typeof(metadata) = 'object')
);

CREATE TABLE public.clinical_catalog_measurement_series (
    release_id text NOT NULL REFERENCES public.clinical_catalog_releases(id)
      ON DELETE CASCADE,
    id text NOT NULL,
    domain text NOT NULL,
    preferred_label text NOT NULL,
    source_receipt jsonb NOT NULL,
    PRIMARY KEY (release_id, id),
    CONSTRAINT clinical_catalog_measurement_series_id_check CHECK (btrim(id) <> ''),
    CONSTRAINT clinical_catalog_measurement_series_domain_check
      CHECK (domain = ANY (ARRAY['lab'::text, 'vital'::text])),
    CONSTRAINT clinical_catalog_measurement_series_label_check
      CHECK (btrim(preferred_label) <> ''),
    CONSTRAINT clinical_catalog_measurement_series_receipt_check
      CHECK (jsonb_typeof(source_receipt) = 'object')
);

CREATE TABLE public.clinical_catalog_measurement_bindings (
    release_id text NOT NULL,
    series_id text NOT NULL,
    source_system text NOT NULL,
    source_code text NOT NULL,
    source_id text NOT NULL,
    source_version text,
    comparability jsonb NOT NULL DEFAULT '{}'::jsonb,
    FOREIGN KEY (release_id, series_id)
      REFERENCES public.clinical_catalog_measurement_series(release_id, id)
      ON DELETE CASCADE,
    PRIMARY KEY (release_id, source_system, source_code),
    CONSTRAINT clinical_catalog_measurement_bindings_system_check
      CHECK (btrim(source_system) <> ''),
    CONSTRAINT clinical_catalog_measurement_bindings_code_check
      CHECK (btrim(source_code) <> ''),
    CONSTRAINT clinical_catalog_measurement_bindings_source_check
      CHECK (btrim(source_id) <> ''),
    CONSTRAINT clinical_catalog_measurement_bindings_comparability_check
      CHECK (jsonb_typeof(comparability) = 'object')
);

CREATE INDEX clinical_catalog_measurement_bindings_series_idx
    ON public.clinical_catalog_measurement_bindings (release_id, series_id);

CREATE TABLE public.clinical_catalog_measurement_series_facets (
    release_id text NOT NULL,
    series_id text NOT NULL,
    facet_id text NOT NULL,
    FOREIGN KEY (release_id, series_id)
      REFERENCES public.clinical_catalog_measurement_series(release_id, id)
      ON DELETE CASCADE,
    FOREIGN KEY (release_id, facet_id)
      REFERENCES public.clinical_catalog_facets(release_id, id)
      ON DELETE CASCADE,
    PRIMARY KEY (release_id, series_id, facet_id)
);

ALTER TABLE public.clinical_catalog_releases ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.clinical_catalog_concepts ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.clinical_catalog_codings ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.clinical_catalog_aliases ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.clinical_catalog_facets ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.clinical_catalog_concept_facets ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.clinical_catalog_relationships ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.clinical_catalog_measurement_series ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.clinical_catalog_measurement_bindings ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.clinical_catalog_measurement_series_facets ENABLE ROW LEVEL SECURITY;

INSERT INTO public.schema_migrations (version)
VALUES ('0008_clinical_catalog')
ON CONFLICT (version) DO NOTHING;

COMMIT;
