begin;

-- Duet currently enforces account ownership in its typed application stores.
-- Earlier migrations enabled RLS without policies while the application role
-- owned every table, so PostgreSQL did not provide a tenant boundary. Remove
-- that misleading configuration before splitting schema ownership from the
-- runtime credential. A future per-account RLS design requires its own settled
-- request-scoped identity contract and policies; it must not be approximated by
-- permissive policies or BYPASSRLS.
alter table public.account_moments disable row level security;
alter table public.api_rate_limits disable row level security;
alter table public.clinical_catalog_aliases disable row level security;
alter table public.clinical_catalog_codings disable row level security;
alter table public.clinical_catalog_concept_facets disable row level security;
alter table public.clinical_catalog_concepts disable row level security;
alter table public.clinical_catalog_facets disable row level security;
alter table public.clinical_catalog_measurement_bindings disable row level security;
alter table public.clinical_catalog_measurement_series disable row level security;
alter table public.clinical_catalog_measurement_series_facets disable row level security;
alter table public.clinical_catalog_relationships disable row level security;
alter table public.clinical_catalog_releases disable row level security;
alter table public.clinical_notes disable row level security;
alter table public.clinvar disable row level security;
alter table public.doctor_collection_items disable row level security;
alter table public.doctor_preparations disable row level security;
alter table public.epic_organizations disable row level security;
alter table public.frequencies disable row level security;
alter table public.genome_findings disable row level security;
alter table public.genome_imports disable row level security;
alter table public.gwas disable row level security;
alter table public.health_contexts disable row level security;
alter table public.health_evidence disable row level security;
alter table public.insight_cycles disable row level security;
alter table public.insight_fact_evaluations disable row level security;
alter table public.learning_ingestion_runs disable row level security;
alter table public.learning_items disable row level security;
alter table public.learning_schedule disable row level security;
alter table public.learning_source_revisions disable row level security;
alter table public.learning_sources disable row level security;
alter table public.schema_migrations disable row level security;
alter table public.sharing_blocks disable row level security;
alter table public.sharing_connections disable row level security;
alter table public.sharing_devices disable row level security;
alter table public.sharing_grant_stats disable row level security;
alter table public.sharing_grants disable row level security;
alter table public.sharing_invitation_stats disable row level security;
alter table public.sharing_invitations disable row level security;
alter table public.sharing_payload_chunks disable row level security;
alter table public.source_connections disable row level security;
alter table public.users disable row level security;

insert into public.schema_migrations (version)
values ('0011_remove_decorative_rls');

commit;
