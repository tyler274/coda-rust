BEGIN;

DROP TABLE IF EXISTS public.lab_results;

INSERT INTO public.schema_migrations(version)
VALUES ('0006_remove_parallel_lab_results')
ON CONFLICT (version) DO NOTHING;

COMMIT;
