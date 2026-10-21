BEGIN;

-- This cutover deliberately has no interpretation compatibility path. The
-- deployed production audit found no retained rows owned by the retired
-- physical-document or Genome-interpretation contracts. Refuse to guess or
-- discard if another environment does contain them.
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM public.health_evidence WHERE source_kind = 'physical'
    ) THEN
        RAISE EXCEPTION
            '0009 requires physical health evidence to be explicitly removed before migration';
    END IF;

    IF EXISTS (SELECT 1 FROM public.genome_imports)
       OR EXISTS (SELECT 1 FROM public.genome_findings) THEN
        RAISE EXCEPTION
            '0009 requires Genome imports to be explicitly removed before migration';
    END IF;

    IF EXISTS (SELECT 1 FROM public.insight_cycles) THEN
        RAISE EXCEPTION
            '0009 requires pre-snapshot-binding insight cycles to be explicitly removed before migration';
    END IF;
END
$$;

DROP FUNCTION public.lookup_variants(text[]);

CREATE FUNCTION public.lookup_variants(rsids text[])
RETURNS TABLE(
    rsid text,
    cv_gene text,
    cv_significance text,
    cv_condition text,
    cv_review_status text,
    af_global real,
    af_eur real,
    af_afr real,
    af_eas real,
    af_sas real,
    af_amr real
)
LANGUAGE sql STABLE
AS $$
    SELECT
        coalesce(c.rsid, f.rsid) AS rsid,
        c.gene AS cv_gene,
        c.clinical_significance AS cv_significance,
        c.condition AS cv_condition,
        c.review_status AS cv_review_status,
        f.af_global,
        f.af_eur,
        f.af_afr,
        f.af_eas,
        f.af_sas,
        f.af_amr
    FROM unnest(rsids) AS requested(rsid)
    LEFT JOIN public.clinvar c ON c.rsid = requested.rsid
    LEFT JOIN public.frequencies f ON f.rsid = requested.rsid
    WHERE c.rsid IS NOT NULL OR f.rsid IS NOT NULL;
$$;

REVOKE EXECUTE ON FUNCTION public.lookup_variants(text[]) FROM PUBLIC;

DROP INDEX public.idx_clinvar_significance;
ALTER TABLE public.clinvar
    DROP COLUMN significance_simple,
    DROP COLUMN plain_english;

DROP INDEX public.idx_freq_rarity;
ALTER TABLE public.frequencies
    DROP COLUMN rarity;

DROP INDEX public.genome_findings_import_category_idx;
ALTER TABLE public.genome_findings
    DROP CONSTRAINT genome_findings_category_check,
    DROP COLUMN category;

ALTER TABLE public.genome_imports
    DROP CONSTRAINT genome_imports_ready_interpretation_check,
    ADD CONSTRAINT genome_imports_ready_reference_check
    CHECK (
        status <> 'ready'::text
        OR (
            coalesce(jsonb_typeof(summary -> 'reference') = 'object', false)
            AND coalesce(btrim(summary #>> '{reference,version}'), '') <> ''
            AND coalesce(btrim(summary #>> '{reference,matched_at}'), '') <> ''
        )
    );

ALTER TABLE public.health_evidence
    DROP CONSTRAINT health_evidence_source_kind_check,
    ADD CONSTRAINT health_evidence_source_kind_check
    CHECK (source_kind = ANY (ARRAY[
        'appleHealth'::text,
        'googleHealth'::text,
        'epic'::text,
        'healthRecords'::text,
        'manual'::text
    ]));

ALTER TABLE public.insight_cycles
    ADD COLUMN IF NOT EXISTS context_snapshot_id uuid NOT NULL;

ALTER TABLE public.insight_cycles
    ADD CONSTRAINT insight_cycles_user_id_id_key UNIQUE (user_id, id);

CREATE TABLE public.insight_fact_evaluations (
    user_id uuid NOT NULL,
    fact_fingerprint text NOT NULL,
    cycle_id uuid NOT NULL,
    evaluated_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT insight_fact_evaluations_pkey
        PRIMARY KEY (user_id, fact_fingerprint),
    CONSTRAINT insight_fact_evaluations_fingerprint_check
        CHECK (char_length(fact_fingerprint) = 64),
    CONSTRAINT insight_fact_evaluations_cycle_fkey
        FOREIGN KEY (user_id, cycle_id)
        REFERENCES public.insight_cycles(user_id, id)
        ON DELETE CASCADE
);

CREATE INDEX insight_fact_evaluations_cycle_idx
    ON public.insight_fact_evaluations (user_id, cycle_id);

ALTER TABLE public.insight_fact_evaluations ENABLE ROW LEVEL SECURITY;

INSERT INTO public.schema_migrations (version)
VALUES ('0009_intelligence_boundary_subtraction')
ON CONFLICT (version) DO NOTHING;

COMMIT;
