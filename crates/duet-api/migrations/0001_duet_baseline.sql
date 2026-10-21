-- Duet Cloud SQL baseline. Generated from the proven ordered schema and
-- intentionally excludes removed chat tables and provider-specific roles.

begin;
--
-- PostgreSQL database dump
--

-- Dumped from database version 16.15 (Homebrew)
-- Dumped by pg_dump version 16.15 (Homebrew)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'SQL_ASCII';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: pgcrypto; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS pgcrypto WITH SCHEMA public;


--
-- Name: erase_account_rate_limits(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.erase_account_rate_limits() RETURNS trigger
    LANGUAGE plpgsql SECURITY DEFINER
    SET search_path TO ''
    AS $$
begin
  delete from public.api_rate_limits
  where principal = old.id::text
     or principal = old.auth_user_id::text;
  return old;
end;
$$;


--
-- Name: lookup_gwas(text[]); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.lookup_gwas(rsids text[]) RETURNS TABLE(rsid text, trait text, p_value double precision, effect_size double precision, risk_allele text, study_pmid text, mapped_gene text, sample_size integer)
    LANGUAGE sql STABLE
    AS $$
  select
    g.rsid,
    g.trait,
    g.p_value,
    g.effect_size,
    g.risk_allele,
    g.study_pmid,
    g.mapped_gene,
    g.sample_size
  from public.gwas g
  where g.rsid = any(rsids)
  order by g.p_value asc;
$$;


--
-- Name: lookup_variants(text[]); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.lookup_variants(rsids text[]) RETURNS TABLE(rsid text, cv_gene text, cv_significance text, cv_condition text, cv_review_status text, cv_significance_simple text, cv_plain_english text, af_global real, af_eur real, af_afr real, af_eas real, af_sas real, af_amr real, freq_rarity text)
    LANGUAGE sql STABLE
    AS $$
  select
    coalesce(c.rsid, f.rsid) as rsid,
    c.gene as cv_gene,
    c.clinical_significance as cv_significance,
    c.condition as cv_condition,
    c.review_status as cv_review_status,
    c.significance_simple as cv_significance_simple,
    c.plain_english as cv_plain_english,
    f.af_global,
    f.af_eur,
    f.af_afr,
    f.af_eas,
    f.af_sas,
    f.af_amr,
    f.rarity as freq_rarity
  from unnest(rsids) as r(rsid)
  left join public.clinvar c on c.rsid = r.rsid
  left join public.frequencies f on f.rsid = r.rsid
  where c.rsid is not null or f.rsid is not null;
$$;


--
-- Name: notify_genome_import_progress(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.notify_genome_import_progress() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  perform pg_notify(
    'genome_imports',
    json_build_object('id', new.id, 'user_id', new.user_id)::text
  );
  return new;
end;
$$;


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: api_rate_limits; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.api_rate_limits (
    principal text NOT NULL,
    route_key text NOT NULL,
    window_started_at timestamp with time zone NOT NULL,
    request_count integer DEFAULT 1 NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT api_rate_limits_request_count_check CHECK ((request_count > 0)),
    CONSTRAINT api_rate_limits_route_key_check CHECK (((char_length(route_key) >= 1) AND (char_length(route_key) <= 80)))
);


--
-- Name: clinical_notes; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.clinical_notes (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    user_id uuid NOT NULL,
    source_id text,
    title text DEFAULT ''::text NOT NULL,
    preview text,
    body text,
    recorded_at timestamp with time zone,
    source_label text DEFAULT 'Health Records'::text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: clinvar; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.clinvar (
    rsid text NOT NULL,
    gene text,
    clinical_significance text NOT NULL,
    condition text,
    review_status text,
    chromosome text,
    variant_type text,
    plain_english text,
    significance_simple text GENERATED ALWAYS AS (
CASE
    WHEN ((clinical_significance ~~* '%pathogenic%'::text) AND (clinical_significance !~~* '%benign%'::text)) THEN 'pathogenic'::text
    WHEN (clinical_significance ~~* '%likely pathogenic%'::text) THEN 'likely_pathogenic'::text
    WHEN (clinical_significance ~~* '%risk%'::text) THEN 'risk_factor'::text
    WHEN (clinical_significance ~~* '%drug%'::text) THEN 'drug_response'::text
    WHEN ((clinical_significance ~~* '%benign%'::text) AND (clinical_significance !~~* '%pathogenic%'::text)) THEN 'benign'::text
    WHEN (clinical_significance ~~* '%likely benign%'::text) THEN 'likely_benign'::text
    ELSE 'uncertain'::text
END) STORED
);


--
-- Name: doctor_collection_items; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.doctor_collection_items (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    user_id uuid NOT NULL,
    kind text NOT NULL,
    source_id text NOT NULL,
    title text NOT NULL,
    detail text NOT NULL,
    range_start timestamp with time zone,
    range_end timestamp with time zone,
    evidence jsonb DEFAULT '[]'::jsonb NOT NULL,
    source_kinds text[] DEFAULT '{}'::text[] NOT NULL,
    "position" integer NOT NULL,
    saved_at timestamp with time zone DEFAULT now() NOT NULL
    ,CONSTRAINT doctor_collection_items_kind_check CHECK ((kind = ANY (ARRAY['insight'::text, 'fact'::text])))
    ,CONSTRAINT doctor_collection_items_position_check CHECK (("position" >= 0))
    ,CONSTRAINT doctor_collection_items_range_check CHECK (((range_start IS NULL) OR (range_end IS NULL) OR (range_end >= range_start)))
    ,CONSTRAINT doctor_collection_items_evidence_check CHECK ((jsonb_typeof(evidence) = 'array'::text AND jsonb_array_length(evidence) > 0))
);


--
-- Name: doctor_preparations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.doctor_preparations (
    user_id uuid NOT NULL,
    generated_draft jsonb NOT NULL,
    edited_draft jsonb NOT NULL,
    collection_fingerprint text NOT NULL,
    revision integer DEFAULT 1 NOT NULL,
    prepared_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT doctor_preparations_generated_draft_check CHECK ((jsonb_typeof(generated_draft) = 'object'::text)),
    CONSTRAINT doctor_preparations_edited_draft_check CHECK ((jsonb_typeof(edited_draft) = 'object'::text)),
    CONSTRAINT doctor_preparations_fingerprint_check CHECK ((collection_fingerprint ~ '^[0-9a-f]{64}$'::text)),
    CONSTRAINT doctor_preparations_revision_check CHECK ((revision > 0))
);


--
-- Name: frequencies; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.frequencies (
    rsid text NOT NULL,
    gene text,
    chromosome text,
    "position" integer,
    ref_allele text,
    alt_allele text,
    af_global real,
    af_eur real,
    af_afr real,
    af_eas real,
    af_sas real,
    af_amr real,
    rarity text GENERATED ALWAYS AS (
CASE
    WHEN (af_global < (0.001)::double precision) THEN 'ultra_rare'::text
    WHEN (af_global < (0.01)::double precision) THEN 'rare'::text
    WHEN (af_global < (0.05)::double precision) THEN 'low_frequency'::text
    ELSE 'common'::text
END) STORED
);


--
-- Name: genome_findings; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.genome_findings (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    import_id uuid NOT NULL,
    user_id uuid NOT NULL,
    rsid text NOT NULL,
    chromosome text,
    category text NOT NULL,
    label text DEFAULT ''::text NOT NULL,
    payload jsonb DEFAULT '{}'::jsonb NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT genome_findings_category_check CHECK ((category = ANY (ARRAY['clinical'::text, 'pharma'::text, 'trait'::text, 'carrier'::text])))
);


--
-- Name: genome_imports; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.genome_imports (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    user_id uuid NOT NULL,
    status text DEFAULT 'uploaded'::text NOT NULL,
    file_name text,
    raw_object_path text,
    raw_deleted_at timestamp with time zone,
    imported_at timestamp with time zone DEFAULT now() NOT NULL,
    completed_at timestamp with time zone,
    failed_at timestamp with time zone,
    failure_message text,
    markers_found integer DEFAULT 0 NOT NULL,
    markers_checked integer DEFAULT 0 NOT NULL,
    findings_found integer DEFAULT 0 NOT NULL,
    completed_chunks integer DEFAULT 0 NOT NULL,
    total_chunks integer DEFAULT 0 NOT NULL,
    summary jsonb DEFAULT '{}'::jsonb NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT genome_imports_ready_interpretation_check CHECK (((status <> 'ready'::text) OR (COALESCE((jsonb_typeof((summary -> 'interpretation'::text)) = 'object'::text), false) AND (COALESCE(btrim((summary #>> '{interpretation,version}'::text[])), ''::text) <> ''::text) AND (COALESCE(btrim((summary #>> '{interpretation,interpreted_at}'::text[])), ''::text) <> ''::text)))),
    CONSTRAINT genome_imports_status_check CHECK ((status = ANY (ARRAY['uploaded'::text, 'reading_file'::text, 'checking_reference_data'::text, 'building_profile'::text, 'ready'::text, 'failed'::text])))
);


--
-- Name: gwas; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.gwas (
    id bigint NOT NULL,
    rsid text NOT NULL,
    trait text NOT NULL,
    p_value double precision,
    effect_size double precision,
    risk_allele text,
    study_pmid text,
    mapped_gene text,
    sample_size integer
);


--
-- Name: gwas_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

ALTER TABLE public.gwas ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.gwas_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: health_contexts; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.health_contexts (
    user_id uuid NOT NULL,
    snapshot_id uuid NOT NULL,
    schema_version integer DEFAULT 1 NOT NULL,
    revision bigint DEFAULT 1 NOT NULL,
    generated_at timestamp with time zone NOT NULL,
    timezone text NOT NULL,
    source_generations jsonb DEFAULT '{}'::jsonb NOT NULL,
    facts jsonb DEFAULT '[]'::jsonb NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT health_contexts_revision_check CHECK ((revision > 0)),
    CONSTRAINT health_contexts_schema_version_check CHECK ((schema_version > 0))
);


--
-- Name: health_evidence; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.health_evidence (
    id uuid NOT NULL,
    user_id uuid NOT NULL,
    concept_id text NOT NULL,
    source_kind text NOT NULL,
    source_connection_id text,
    source_record_id text,
    measured_at timestamp with time zone NOT NULL,
    received_at timestamp with time zone NOT NULL,
    payload jsonb NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT health_evidence_source_kind_check CHECK ((source_kind = ANY (ARRAY['appleHealth'::text, 'healthRecords'::text, 'physical'::text, 'manual'::text])))
);


--
-- Name: learning_ingestion_runs; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.learning_ingestion_runs (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    provider text NOT NULL,
    content_mode text NOT NULL,
    status text DEFAULT 'running'::text NOT NULL,
    started_at timestamp with time zone DEFAULT now() NOT NULL,
    completed_at timestamp with time zone,
    discovered_count integer DEFAULT 0 NOT NULL,
    checked_count integer DEFAULT 0 NOT NULL,
    changed_count integer DEFAULT 0 NOT NULL,
    unchanged_count integer DEFAULT 0 NOT NULL,
    failed_count integer DEFAULT 0 NOT NULL,
    diagnostics jsonb DEFAULT '[]'::jsonb NOT NULL,
    CONSTRAINT learning_ingestion_runs_changed_count_check CHECK ((changed_count >= 0)),
    CONSTRAINT learning_ingestion_runs_check CHECK ((((status = 'running'::text) AND (completed_at IS NULL)) OR ((status <> 'running'::text) AND (completed_at IS NOT NULL)))),
    CONSTRAINT learning_ingestion_runs_checked_count_check CHECK ((checked_count >= 0)),
    CONSTRAINT learning_ingestion_runs_content_mode_check CHECK ((content_mode = ANY (ARRAY['metadata'::text, 'structured'::text]))),
    CONSTRAINT learning_ingestion_runs_diagnostics_check CHECK ((jsonb_typeof(diagnostics) = 'array'::text)),
    CONSTRAINT learning_ingestion_runs_discovered_count_check CHECK ((discovered_count >= 0)),
    CONSTRAINT learning_ingestion_runs_failed_count_check CHECK ((failed_count >= 0)),
    CONSTRAINT learning_ingestion_runs_provider_check CHECK ((provider = ANY (ARRAY['catalog'::text, 'nhgri'::text, 'who'::text]))),
    CONSTRAINT learning_ingestion_runs_status_check CHECK ((status = ANY (ARRAY['running'::text, 'succeeded'::text, 'failed'::text, 'skipped'::text]))),
    CONSTRAINT learning_ingestion_runs_unchanged_count_check CHECK ((unchanged_count >= 0))
);


--
-- Name: learning_items; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.learning_items (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    series text NOT NULL,
    stable_key text NOT NULL,
    version integer NOT NULL,
    rotation_order integer NOT NULL,
    source_id uuid NOT NULL,
    source_revision_id uuid,
    title text NOT NULL,
    body text NOT NULL,
    source_name text NOT NULL,
    source_url text NOT NULL,
    origin text NOT NULL,
    status text DEFAULT 'draft'::text NOT NULL,
    approved_at timestamp with time zone,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT learning_items_body_check CHECK (((char_length(body) >= 1) AND (char_length(body) <= 800))),
    CONSTRAINT learning_items_check CHECK (((status <> 'approved'::text) OR (approved_at IS NOT NULL))),
    CONSTRAINT learning_items_origin_check CHECK ((origin = ANY (ARRAY['duet_authored'::text, 'licensed_source_excerpt'::text]))),
    CONSTRAINT learning_items_rotation_order_check CHECK ((rotation_order >= 0)),
    CONSTRAINT learning_items_series_check CHECK ((series = ANY (ARRAY['genomics'::text, 'who_fact_sheet'::text]))),
    CONSTRAINT learning_items_source_name_check CHECK (((char_length(source_name) >= 1) AND (char_length(source_name) <= 160))),
    CONSTRAINT learning_items_source_url_check CHECK ((((char_length(source_url) >= 12) AND (char_length(source_url) <= 1000)) AND (source_url ~~ 'https://%'::text))),
    CONSTRAINT learning_items_stable_key_check CHECK (((char_length(stable_key) >= 1) AND (char_length(stable_key) <= 240))),
    CONSTRAINT learning_items_status_check CHECK ((status = ANY (ARRAY['draft'::text, 'approved'::text, 'retired'::text]))),
    CONSTRAINT learning_items_title_check CHECK (((char_length(title) >= 1) AND (char_length(title) <= 160))),
    CONSTRAINT learning_items_version_check CHECK ((version > 0))
);


--
-- Name: learning_schedule; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.learning_schedule (
    series text NOT NULL,
    locale text DEFAULT 'en'::text NOT NULL,
    day date NOT NULL,
    item_id uuid NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT learning_schedule_locale_check CHECK (((char_length(locale) >= 2) AND (char_length(locale) <= 16))),
    CONSTRAINT learning_schedule_series_check CHECK ((series = ANY (ARRAY['genomics'::text, 'who_fact_sheet'::text])))
);


--
-- Name: learning_source_revisions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.learning_source_revisions (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    source_id uuid NOT NULL,
    content_sha256 text NOT NULL,
    parser_version integer NOT NULL,
    content_mode text NOT NULL,
    source_title text NOT NULL,
    source_published_at timestamp with time zone,
    source_modified_at timestamp with time zone,
    content jsonb DEFAULT '{"sections": [], "key_facts": []}'::jsonb NOT NULL,
    fetched_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT learning_source_revisions_content_check CHECK (((jsonb_typeof(content) = 'object'::text) AND (jsonb_typeof(COALESCE((content -> 'key_facts'::text), '[]'::jsonb)) = 'array'::text) AND (jsonb_typeof(COALESCE((content -> 'sections'::text), '[]'::jsonb)) = 'array'::text))),
    CONSTRAINT learning_source_revisions_content_mode_check CHECK ((content_mode = ANY (ARRAY['metadata'::text, 'structured'::text]))),
    CONSTRAINT learning_source_revisions_content_sha256_check CHECK ((char_length(content_sha256) = 64)),
    CONSTRAINT learning_source_revisions_parser_version_check CHECK ((parser_version > 0)),
    CONSTRAINT learning_source_revisions_source_title_check CHECK (((char_length(source_title) >= 1) AND (char_length(source_title) <= 300)))
);


--
-- Name: learning_sources; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.learning_sources (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    provider text NOT NULL,
    source_key text NOT NULL,
    canonical_url text NOT NULL,
    title text NOT NULL,
    etag text,
    http_last_modified text,
    last_http_status smallint,
    first_seen_at timestamp with time zone DEFAULT now() NOT NULL,
    last_seen_at timestamp with time zone DEFAULT now() NOT NULL,
    last_checked_at timestamp with time zone,
    last_fetched_at timestamp with time zone,
    consecutive_misses integer DEFAULT 0 NOT NULL,
    is_active boolean DEFAULT true NOT NULL,
    current_revision_id uuid,
    CONSTRAINT learning_sources_canonical_url_check CHECK ((((char_length(canonical_url) >= 12) AND (char_length(canonical_url) <= 1000)) AND (canonical_url ~~ 'https://%'::text))),
    CONSTRAINT learning_sources_consecutive_misses_check CHECK ((consecutive_misses >= 0)),
    CONSTRAINT learning_sources_etag_check CHECK (((etag IS NULL) OR (char_length(etag) <= 500))),
    CONSTRAINT learning_sources_http_last_modified_check CHECK (((http_last_modified IS NULL) OR (char_length(http_last_modified) <= 200))),
    CONSTRAINT learning_sources_last_http_status_check CHECK (((last_http_status IS NULL) OR ((last_http_status >= 100) AND (last_http_status <= 599)))),
    CONSTRAINT learning_sources_provider_check CHECK ((provider = ANY (ARRAY['nhgri'::text, 'who'::text]))),
    CONSTRAINT learning_sources_source_key_check CHECK (((char_length(source_key) >= 1) AND (char_length(source_key) <= 240))),
    CONSTRAINT learning_sources_title_check CHECK (((char_length(title) >= 1) AND (char_length(title) <= 300)))
);


--
-- Name: sharing_blocks; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.sharing_blocks (
    blocker_user_id uuid NOT NULL,
    blocked_user_id uuid NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT sharing_blocks_check CHECK ((blocker_user_id <> blocked_user_id))
);


--
-- Name: sharing_connections; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.sharing_connections (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    lower_user_id uuid NOT NULL,
    upper_user_id uuid NOT NULL,
    state text DEFAULT 'active'::text NOT NULL,
    accepted_at timestamp with time zone,
    disconnected_at timestamp with time zone,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT sharing_connections_check CHECK ((lower_user_id < upper_user_id)),
    CONSTRAINT sharing_connections_state_check CHECK ((state = ANY (ARRAY['active'::text, 'reconnect_required'::text, 'disconnected'::text, 'blocked'::text])))
);


--
-- Name: sharing_devices; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.sharing_devices (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    user_id uuid NOT NULL,
    public_key bytea NOT NULL,
    algorithm text DEFAULT 'curve25519'::text NOT NULL,
    key_version integer NOT NULL,
    active_at timestamp with time zone DEFAULT now() NOT NULL,
    revoked_at timestamp with time zone,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    apns_token bytea,
    apns_environment text,
    apns_registered_at timestamp with time zone,
    CONSTRAINT sharing_devices_algorithm_check CHECK ((algorithm = 'curve25519'::text)),
    CONSTRAINT sharing_devices_apns_registration_check CHECK ((((apns_token IS NULL) AND (apns_environment IS NULL) AND (apns_registered_at IS NULL)) OR ((apns_token IS NOT NULL) AND (apns_environment = ANY (ARRAY['sandbox'::text, 'production'::text])) AND (apns_registered_at IS NOT NULL)))),
    CONSTRAINT sharing_devices_apns_token_length_check CHECK (((apns_token IS NULL) OR ((octet_length(apns_token) >= 1) AND (octet_length(apns_token) <= 256)))),
    CONSTRAINT sharing_devices_key_version_check CHECK ((key_version > 0)),
    CONSTRAINT sharing_devices_public_key_check CHECK ((octet_length(public_key) = 32))
);


--
-- Name: sharing_grant_stats; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.sharing_grant_stats (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    grant_id uuid NOT NULL,
    stat_id text NOT NULL,
    history_start date NOT NULL,
    active_at timestamp with time zone DEFAULT now() NOT NULL,
    revoked_at timestamp with time zone,
    purge_after timestamp with time zone,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    ready_at timestamp with time zone,
    CONSTRAINT sharing_grant_stats_stat_id_check CHECK ((stat_id = ANY (ARRAY['steps'::text, 'activeEnergy'::text, 'exerciseMinutes'::text, 'sleepMinutes'::text, 'restingHeartRate'::text, 'distance'::text, 'bodyMass'::text, 'dietaryEnergy'::text, 'water'::text])))
);


--
-- Name: sharing_grants; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.sharing_grants (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    connection_id uuid NOT NULL,
    sender_user_id uuid NOT NULL,
    recipient_user_id uuid NOT NULL,
    state text NOT NULL,
    revision bigint DEFAULT 1 NOT NULL,
    sender_key_version integer NOT NULL,
    recipient_key_version integer NOT NULL,
    offered_at timestamp with time zone,
    accepted_at timestamp with time zone,
    revoked_at timestamp with time zone,
    purge_after timestamp with time zone,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT sharing_grants_check CHECK ((sender_user_id <> recipient_user_id)),
    CONSTRAINT sharing_grants_recipient_key_version_check CHECK ((recipient_key_version > 0)),
    CONSTRAINT sharing_grants_revision_check CHECK ((revision > 0)),
    CONSTRAINT sharing_grants_sender_key_version_check CHECK ((sender_key_version > 0)),
    CONSTRAINT sharing_grants_state_check CHECK ((state = ANY (ARRAY['offered'::text, 'active'::text, 'revoked'::text, 'reconnect_required'::text])))
);


--
-- Name: sharing_invitation_stats; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.sharing_invitation_stats (
    invitation_id uuid NOT NULL,
    stat_id text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT sharing_invitation_stats_stat_id_check CHECK ((stat_id = ANY (ARRAY['steps'::text, 'activeEnergy'::text, 'exerciseMinutes'::text, 'sleepMinutes'::text, 'restingHeartRate'::text, 'distance'::text, 'bodyMass'::text, 'dietaryEnergy'::text, 'water'::text])))
);


--
-- Name: sharing_invitations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.sharing_invitations (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    inviter_user_id uuid NOT NULL,
    qr_secret_hash bytea,
    manual_secret_hash bytea,
    inviter_key_version integer NOT NULL,
    expires_at timestamp with time zone NOT NULL,
    state text DEFAULT 'pending'::text NOT NULL,
    accepted_by_user_id uuid,
    consumed_at timestamp with time zone,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT sharing_invitations_check CHECK ((((state = 'pending'::text) AND (qr_secret_hash IS NOT NULL) AND (manual_secret_hash IS NOT NULL)) OR (state <> 'pending'::text))),
    CONSTRAINT sharing_invitations_inviter_key_version_check CHECK ((inviter_key_version > 0)),
    CONSTRAINT sharing_invitations_manual_secret_hash_check CHECK (((manual_secret_hash IS NULL) OR (octet_length(manual_secret_hash) = 32))),
    CONSTRAINT sharing_invitations_qr_secret_hash_check CHECK (((qr_secret_hash IS NULL) OR (octet_length(qr_secret_hash) = 32))),
    CONSTRAINT sharing_invitations_state_check CHECK ((state = ANY (ARRAY['pending'::text, 'accepted'::text, 'declined'::text, 'cancelled'::text, 'expired'::text])))
);


--
-- Name: sharing_payload_chunks; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.sharing_payload_chunks (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    scope_id uuid NOT NULL,
    month_identity text NOT NULL,
    schema_version integer NOT NULL,
    sender_key_version integer NOT NULL,
    recipient_key_version integer NOT NULL,
    source_revision bigint NOT NULL,
    ciphertext bytea NOT NULL,
    digest bytea NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT sharing_payload_chunks_ciphertext_check CHECK ((octet_length(ciphertext) <= 65536)),
    CONSTRAINT sharing_payload_chunks_digest_check CHECK ((octet_length(digest) = 32)),
    CONSTRAINT sharing_payload_chunks_month_identity_check CHECK ((month_identity ~ '^[0-9]{4}-(0[1-9]|1[0-2])$'::text)),
    CONSTRAINT sharing_payload_chunks_recipient_key_version_check CHECK ((recipient_key_version > 0)),
    CONSTRAINT sharing_payload_chunks_schema_version_check CHECK ((schema_version > 0)),
    CONSTRAINT sharing_payload_chunks_sender_key_version_check CHECK ((sender_key_version > 0)),
    CONSTRAINT sharing_payload_chunks_source_revision_check CHECK ((source_revision > 0))
);


--
-- Name: users; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.users (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    auth_user_id text NOT NULL,
    apple_health_generation integer DEFAULT 0 NOT NULL,
    apple_health_disconnected_at timestamp with time zone,
    health_records_generation integer DEFAULT 0 NOT NULL,
    health_records_disconnected_at timestamp with time zone,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    setup_complete boolean DEFAULT false NOT NULL,
    sharing_label text NOT NULL,
    display_name text,
    doctor_note text,
    CONSTRAINT users_display_name_check CHECK (((display_name IS NULL) OR (((char_length(btrim(display_name)) >= 1) AND (char_length(btrim(display_name)) <= 80)) AND (display_name = btrim(display_name))))),
    CONSTRAINT users_doctor_note_check CHECK (((doctor_note IS NULL) OR (char_length(doctor_note) <= 1000))),
    CONSTRAINT users_sharing_label_check CHECK ((((char_length(btrim(sharing_label)) >= 1) AND (char_length(btrim(sharing_label)) <= 40)) AND (sharing_label = btrim(sharing_label))))
);


--
-- Name: schema_migrations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.schema_migrations (
    version text PRIMARY KEY,
    applied_at timestamp with time zone DEFAULT now() NOT NULL
);

ALTER TABLE public.schema_migrations ENABLE ROW LEVEL SECURITY;

INSERT INTO public.schema_migrations (version) VALUES ('0001_duet_baseline');


--
-- Name: insight_cycles; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.insight_cycles (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    user_id uuid NOT NULL,
    context_snapshot_id uuid NOT NULL,
    kind text NOT NULL,
    window_start timestamp with time zone NOT NULL,
    window_end timestamp with time zone NOT NULL,
    timezone text NOT NULL,
    first_weekday smallint NOT NULL,
    status text DEFAULT 'generating'::text NOT NULL,
    claim_token uuid,
    lease_expires_at timestamp with time zone,
    slot_hash text NOT NULL,
    candidate_fingerprint text NOT NULL,
    included_concept_ids text[] NOT NULL,
    insights jsonb DEFAULT '[]'::jsonb NOT NULL,
    supporting_facts jsonb DEFAULT '[]'::jsonb NOT NULL,
    evidence jsonb DEFAULT '[]'::jsonb NOT NULL,
    source_kinds text[] DEFAULT '{}'::text[] NOT NULL,
    source_generations jsonb DEFAULT '{}'::jsonb NOT NULL,
    generation_metadata jsonb DEFAULT '{}'::jsonb NOT NULL,
    observed_through timestamp with time zone,
    generated_at timestamp with time zone,
    corrected_insight_ids uuid[] DEFAULT '{}'::uuid[] NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT insight_cycles_check CHECK ((window_end > window_start)),
    CONSTRAINT insight_cycles_correction_check CHECK ((cardinality(corrected_insight_ids) <= jsonb_array_length(insights))),
    CONSTRAINT insight_cycles_evidence_check CHECK ((jsonb_typeof(evidence) = 'array'::text)),
    CONSTRAINT insight_cycles_first_weekday_check CHECK (((first_weekday >= 1) AND (first_weekday <= 7))),
    CONSTRAINT insight_cycles_insights_check CHECK ((jsonb_typeof(insights) = 'array'::text)),
    CONSTRAINT insight_cycles_included_concept_ids_check CHECK ((cardinality(included_concept_ids) > 0)),
    CONSTRAINT insight_cycles_ready_payload_check CHECK (((status = 'generating'::text) OR (((jsonb_array_length(insights) >= 0) AND (jsonb_array_length(insights) <= 3)) AND (jsonb_array_length(supporting_facts) > 0) AND (observed_through IS NOT NULL) AND (generated_at IS NOT NULL) AND (((jsonb_array_length(insights) = 0) AND (jsonb_array_length(evidence) = 0)) OR ((jsonb_array_length(insights) > 0) AND (jsonb_array_length(evidence) > 0)))))),
    CONSTRAINT insight_cycles_kind_check CHECK ((kind = ANY (ARRAY['daily'::text, 'weekly'::text]))),
    CONSTRAINT insight_cycles_slot_hash_check CHECK ((char_length(slot_hash) = 64)),
    CONSTRAINT insight_cycles_candidate_fingerprint_check CHECK ((char_length(candidate_fingerprint) = 64)),
    CONSTRAINT insight_cycles_status_check CHECK ((status = ANY (ARRAY['generating'::text, 'ready'::text]))),
    CONSTRAINT insight_cycles_supporting_facts_check CHECK ((jsonb_typeof(supporting_facts) = 'array'::text)),
    CONSTRAINT insight_cycles_timezone_check CHECK (((char_length(timezone) >= 1) AND (char_length(timezone) <= 80)))
);


--
-- Name: api_rate_limits api_rate_limits_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.api_rate_limits
    ADD CONSTRAINT api_rate_limits_pkey PRIMARY KEY (principal, route_key, window_started_at);


--
-- Name: clinical_notes clinical_notes_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.clinical_notes
    ADD CONSTRAINT clinical_notes_pkey PRIMARY KEY (id);


--
-- Name: clinvar clinvar_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.clinvar
    ADD CONSTRAINT clinvar_pkey PRIMARY KEY (rsid);


--
-- Name: doctor_collection_items doctor_collection_items_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.doctor_collection_items
    ADD CONSTRAINT doctor_collection_items_pkey PRIMARY KEY (id);


--
-- Name: doctor_preparations doctor_preparations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.doctor_preparations
    ADD CONSTRAINT doctor_preparations_pkey PRIMARY KEY (user_id);


--
-- Name: doctor_collection_items doctor_collection_items_user_kind_source_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.doctor_collection_items
    ADD CONSTRAINT doctor_collection_items_user_kind_source_key UNIQUE (user_id, kind, source_id);


--
-- Name: doctor_collection_items doctor_collection_items_user_position_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.doctor_collection_items
    ADD CONSTRAINT doctor_collection_items_user_position_key UNIQUE (user_id, "position") DEFERRABLE INITIALLY IMMEDIATE;


--
-- Name: frequencies frequencies_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.frequencies
    ADD CONSTRAINT frequencies_pkey PRIMARY KEY (rsid);


--
-- Name: genome_findings genome_findings_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.genome_findings
    ADD CONSTRAINT genome_findings_pkey PRIMARY KEY (id);


--
-- Name: genome_imports genome_imports_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.genome_imports
    ADD CONSTRAINT genome_imports_pkey PRIMARY KEY (id);


--
-- Name: gwas gwas_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.gwas
    ADD CONSTRAINT gwas_pkey PRIMARY KEY (id);


--
-- Name: health_contexts health_contexts_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.health_contexts
    ADD CONSTRAINT health_contexts_pkey PRIMARY KEY (user_id);


--
-- Name: health_evidence health_evidence_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.health_evidence
    ADD CONSTRAINT health_evidence_pkey PRIMARY KEY (id);


--
-- Name: learning_ingestion_runs learning_ingestion_runs_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.learning_ingestion_runs
    ADD CONSTRAINT learning_ingestion_runs_pkey PRIMARY KEY (id);


--
-- Name: learning_items learning_items_id_series_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.learning_items
    ADD CONSTRAINT learning_items_id_series_key UNIQUE (id, series);


--
-- Name: learning_items learning_items_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.learning_items
    ADD CONSTRAINT learning_items_pkey PRIMARY KEY (id);


--
-- Name: learning_items learning_items_series_rotation_order_version_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.learning_items
    ADD CONSTRAINT learning_items_series_rotation_order_version_key UNIQUE (series, rotation_order, version);


--
-- Name: learning_items learning_items_series_stable_key_version_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.learning_items
    ADD CONSTRAINT learning_items_series_stable_key_version_key UNIQUE (series, stable_key, version);


--
-- Name: learning_schedule learning_schedule_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.learning_schedule
    ADD CONSTRAINT learning_schedule_pkey PRIMARY KEY (series, locale, day);


--
-- Name: learning_source_revisions learning_source_revisions_id_source_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.learning_source_revisions
    ADD CONSTRAINT learning_source_revisions_id_source_id_key UNIQUE (id, source_id);


--
-- Name: learning_source_revisions learning_source_revisions_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.learning_source_revisions
    ADD CONSTRAINT learning_source_revisions_pkey PRIMARY KEY (id);


--
-- Name: learning_source_revisions learning_source_revisions_source_id_content_sha256_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.learning_source_revisions
    ADD CONSTRAINT learning_source_revisions_source_id_content_sha256_key UNIQUE (source_id, content_sha256);


--
-- Name: learning_sources learning_sources_canonical_url_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.learning_sources
    ADD CONSTRAINT learning_sources_canonical_url_key UNIQUE (canonical_url);


--
-- Name: learning_sources learning_sources_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.learning_sources
    ADD CONSTRAINT learning_sources_pkey PRIMARY KEY (id);


--
-- Name: learning_sources learning_sources_provider_source_key_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.learning_sources
    ADD CONSTRAINT learning_sources_provider_source_key_key UNIQUE (provider, source_key);


--
-- Name: sharing_blocks sharing_blocks_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sharing_blocks
    ADD CONSTRAINT sharing_blocks_pkey PRIMARY KEY (blocker_user_id, blocked_user_id);


--
-- Name: sharing_connections sharing_connections_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sharing_connections
    ADD CONSTRAINT sharing_connections_pkey PRIMARY KEY (id);


--
-- Name: sharing_devices sharing_devices_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sharing_devices
    ADD CONSTRAINT sharing_devices_pkey PRIMARY KEY (id);


--
-- Name: sharing_devices sharing_devices_user_id_key_version_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sharing_devices
    ADD CONSTRAINT sharing_devices_user_id_key_version_key UNIQUE (user_id, key_version);


--
-- Name: sharing_grant_stats sharing_grant_stats_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sharing_grant_stats
    ADD CONSTRAINT sharing_grant_stats_pkey PRIMARY KEY (id);


--
-- Name: sharing_grants sharing_grants_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sharing_grants
    ADD CONSTRAINT sharing_grants_pkey PRIMARY KEY (id);


--
-- Name: sharing_invitation_stats sharing_invitation_stats_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sharing_invitation_stats
    ADD CONSTRAINT sharing_invitation_stats_pkey PRIMARY KEY (invitation_id, stat_id);


--
-- Name: sharing_invitations sharing_invitations_manual_secret_hash_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sharing_invitations
    ADD CONSTRAINT sharing_invitations_manual_secret_hash_key UNIQUE (manual_secret_hash);


--
-- Name: sharing_invitations sharing_invitations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sharing_invitations
    ADD CONSTRAINT sharing_invitations_pkey PRIMARY KEY (id);


--
-- Name: sharing_invitations sharing_invitations_qr_secret_hash_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sharing_invitations
    ADD CONSTRAINT sharing_invitations_qr_secret_hash_key UNIQUE (qr_secret_hash);


--
-- Name: sharing_payload_chunks sharing_payload_chunks_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sharing_payload_chunks
    ADD CONSTRAINT sharing_payload_chunks_pkey PRIMARY KEY (id);


--
-- Name: sharing_payload_chunks sharing_payload_chunks_scope_id_month_identity_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sharing_payload_chunks
    ADD CONSTRAINT sharing_payload_chunks_scope_id_month_identity_key UNIQUE (scope_id, month_identity);


--
-- Name: users users_auth_user_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_auth_user_id_key UNIQUE (auth_user_id);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: insight_cycles insight_cycles_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.insight_cycles
    ADD CONSTRAINT insight_cycles_pkey PRIMARY KEY (id);


--
-- Name: insight_cycles insight_cycles_user_id_slot_hash_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.insight_cycles
    ADD CONSTRAINT insight_cycles_user_id_slot_hash_key UNIQUE (user_id, slot_hash);


--
-- Name: insight_cycles insight_cycles_user_id_candidate_fingerprint_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.insight_cycles
    ADD CONSTRAINT insight_cycles_user_id_candidate_fingerprint_key UNIQUE (user_id, candidate_fingerprint);


--
-- Name: api_rate_limits_window_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX api_rate_limits_window_idx ON public.api_rate_limits USING btree (window_started_at);


--
-- Name: clinical_notes_user_recorded_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX clinical_notes_user_recorded_idx ON public.clinical_notes USING btree (user_id, recorded_at DESC);


--
-- Name: clinical_notes_user_source_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX clinical_notes_user_source_idx ON public.clinical_notes USING btree (user_id, source_id) WHERE (source_id IS NOT NULL);


--
-- Name: doctor_collection_items_user_saved_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX doctor_collection_items_user_saved_idx ON public.doctor_collection_items USING btree (user_id, saved_at DESC);


--
-- Name: genome_findings_import_category_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX genome_findings_import_category_idx ON public.genome_findings USING btree (import_id, category, rsid);


--
-- Name: genome_findings_import_chromosome_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX genome_findings_import_chromosome_idx ON public.genome_findings USING btree (import_id, chromosome, rsid);


--
-- Name: genome_imports_user_created_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX genome_imports_user_created_idx ON public.genome_imports USING btree (user_id, created_at DESC);


--
-- Name: genome_imports_user_status_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX genome_imports_user_status_idx ON public.genome_imports USING btree (user_id, status);


--
-- Name: health_evidence_source_record_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX health_evidence_source_record_idx ON public.health_evidence USING btree (user_id, source_kind, source_connection_id, source_record_id) WHERE (source_record_id IS NOT NULL);


--
-- Name: health_evidence_user_concept_measured_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX health_evidence_user_concept_measured_idx ON public.health_evidence USING btree (user_id, concept_id, measured_at);


--
-- Name: health_evidence_user_source_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX health_evidence_user_source_idx ON public.health_evidence USING btree (user_id, source_kind, source_connection_id);


--
-- Name: idx_clinvar_gene; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_clinvar_gene ON public.clinvar USING btree (gene);


--
-- Name: idx_clinvar_significance; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_clinvar_significance ON public.clinvar USING btree (significance_simple);


--
-- Name: idx_freq_chr; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_freq_chr ON public.frequencies USING btree (chromosome);


--
-- Name: idx_freq_rarity; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_freq_rarity ON public.frequencies USING btree (rarity);


--
-- Name: idx_gwas_rsid; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_gwas_rsid ON public.gwas USING btree (rsid);


--
-- Name: idx_gwas_trait; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_gwas_trait ON public.gwas USING btree (trait);


--
-- Name: learning_ingestion_runs_provider_started_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX learning_ingestion_runs_provider_started_idx ON public.learning_ingestion_runs USING btree (provider, started_at DESC);


--
-- Name: learning_items_approved_rotation_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX learning_items_approved_rotation_idx ON public.learning_items USING btree (series, rotation_order, version DESC) WHERE (status = 'approved'::text);


--
-- Name: learning_items_source_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX learning_items_source_idx ON public.learning_items USING btree (source_id);


--
-- Name: learning_items_source_revision_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX learning_items_source_revision_idx ON public.learning_items USING btree (source_revision_id) WHERE (source_revision_id IS NOT NULL);


--
-- Name: learning_schedule_item_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX learning_schedule_item_idx ON public.learning_schedule USING btree (item_id);


--
-- Name: learning_source_revisions_source_fetched_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX learning_source_revisions_source_fetched_idx ON public.learning_source_revisions USING btree (source_id, fetched_at DESC);


--
-- Name: learning_sources_current_revision_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX learning_sources_current_revision_idx ON public.learning_sources USING btree (current_revision_id) WHERE (current_revision_id IS NOT NULL);


--
-- Name: learning_sources_provider_active_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX learning_sources_provider_active_idx ON public.learning_sources USING btree (provider, is_active, source_key);


--
-- Name: sharing_blocks_reverse_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX sharing_blocks_reverse_idx ON public.sharing_blocks USING btree (blocked_user_id, blocker_user_id);


--
-- Name: sharing_connections_lower_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX sharing_connections_lower_idx ON public.sharing_connections USING btree (lower_user_id, updated_at DESC);


--
-- Name: sharing_connections_one_live_pair_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX sharing_connections_one_live_pair_idx ON public.sharing_connections USING btree (lower_user_id, upper_user_id) WHERE (state = ANY (ARRAY['active'::text, 'reconnect_required'::text]));


--
-- Name: sharing_connections_upper_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX sharing_connections_upper_idx ON public.sharing_connections USING btree (upper_user_id, updated_at DESC);


--
-- Name: sharing_devices_one_active_apns_token_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX sharing_devices_one_active_apns_token_idx ON public.sharing_devices USING btree (apns_token) WHERE ((apns_token IS NOT NULL) AND (revoked_at IS NULL));


--
-- Name: sharing_devices_one_active_per_user_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX sharing_devices_one_active_per_user_idx ON public.sharing_devices USING btree (user_id) WHERE (revoked_at IS NULL);


--
-- Name: sharing_grant_stats_grant_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX sharing_grant_stats_grant_idx ON public.sharing_grant_stats USING btree (grant_id, active_at);


--
-- Name: sharing_grant_stats_one_active_scope_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX sharing_grant_stats_one_active_scope_idx ON public.sharing_grant_stats USING btree (grant_id, stat_id) WHERE (revoked_at IS NULL);


--
-- Name: sharing_grants_connection_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX sharing_grants_connection_idx ON public.sharing_grants USING btree (connection_id);


--
-- Name: sharing_grants_one_live_direction_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX sharing_grants_one_live_direction_idx ON public.sharing_grants USING btree (connection_id, sender_user_id, recipient_user_id) WHERE (state = ANY (ARRAY['offered'::text, 'active'::text, 'reconnect_required'::text]));


--
-- Name: sharing_grants_recipient_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX sharing_grants_recipient_idx ON public.sharing_grants USING btree (recipient_user_id, state, updated_at DESC);


--
-- Name: sharing_grants_sender_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX sharing_grants_sender_idx ON public.sharing_grants USING btree (sender_user_id, state, updated_at DESC);


--
-- Name: sharing_invitations_accepted_by_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX sharing_invitations_accepted_by_idx ON public.sharing_invitations USING btree (accepted_by_user_id) WHERE (accepted_by_user_id IS NOT NULL);


--
-- Name: sharing_invitations_inviter_state_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX sharing_invitations_inviter_state_idx ON public.sharing_invitations USING btree (inviter_user_id, state, expires_at DESC);


--
-- Name: sharing_payload_chunks_scope_updated_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX sharing_payload_chunks_scope_updated_idx ON public.sharing_payload_chunks USING btree (scope_id, updated_at, id);


--
-- Name: insight_cycles_user_generated_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX insight_cycles_user_generated_idx ON public.insight_cycles USING btree (user_id, generated_at DESC, id DESC) WHERE (status = 'ready'::text);


--
-- Name: genome_imports genome_imports_notify; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER genome_imports_notify AFTER INSERT OR UPDATE ON public.genome_imports FOR EACH ROW EXECUTE FUNCTION public.notify_genome_import_progress();


--
-- Name: users users_erase_rate_limits; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER users_erase_rate_limits AFTER DELETE ON public.users FOR EACH ROW EXECUTE FUNCTION public.erase_account_rate_limits();


--
-- Name: clinical_notes clinical_notes_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.clinical_notes
    ADD CONSTRAINT clinical_notes_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: doctor_collection_items doctor_collection_items_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.doctor_collection_items
    ADD CONSTRAINT doctor_collection_items_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: doctor_preparations doctor_preparations_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.doctor_preparations
    ADD CONSTRAINT doctor_preparations_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: genome_findings genome_findings_import_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.genome_findings
    ADD CONSTRAINT genome_findings_import_id_fkey FOREIGN KEY (import_id) REFERENCES public.genome_imports(id) ON DELETE CASCADE;


--
-- Name: genome_findings genome_findings_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.genome_findings
    ADD CONSTRAINT genome_findings_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: genome_imports genome_imports_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.genome_imports
    ADD CONSTRAINT genome_imports_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: health_contexts health_contexts_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.health_contexts
    ADD CONSTRAINT health_contexts_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: health_evidence health_evidence_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.health_evidence
    ADD CONSTRAINT health_evidence_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: learning_items learning_items_source_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.learning_items
    ADD CONSTRAINT learning_items_source_id_fkey FOREIGN KEY (source_id) REFERENCES public.learning_sources(id) ON DELETE RESTRICT;


--
-- Name: learning_items learning_items_source_revision_id_source_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.learning_items
    ADD CONSTRAINT learning_items_source_revision_id_source_id_fkey FOREIGN KEY (source_revision_id, source_id) REFERENCES public.learning_source_revisions(id, source_id) ON DELETE RESTRICT;


--
-- Name: learning_schedule learning_schedule_item_id_series_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.learning_schedule
    ADD CONSTRAINT learning_schedule_item_id_series_fkey FOREIGN KEY (item_id, series) REFERENCES public.learning_items(id, series) ON DELETE RESTRICT;


--
-- Name: learning_source_revisions learning_source_revisions_source_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.learning_source_revisions
    ADD CONSTRAINT learning_source_revisions_source_id_fkey FOREIGN KEY (source_id) REFERENCES public.learning_sources(id) ON DELETE CASCADE;


--
-- Name: learning_sources learning_sources_current_revision_fk; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.learning_sources
    ADD CONSTRAINT learning_sources_current_revision_fk FOREIGN KEY (current_revision_id, id) REFERENCES public.learning_source_revisions(id, source_id) ON DELETE SET NULL (current_revision_id);


--
-- Name: sharing_blocks sharing_blocks_blocked_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sharing_blocks
    ADD CONSTRAINT sharing_blocks_blocked_user_id_fkey FOREIGN KEY (blocked_user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: sharing_blocks sharing_blocks_blocker_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sharing_blocks
    ADD CONSTRAINT sharing_blocks_blocker_user_id_fkey FOREIGN KEY (blocker_user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: sharing_connections sharing_connections_lower_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sharing_connections
    ADD CONSTRAINT sharing_connections_lower_user_id_fkey FOREIGN KEY (lower_user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: sharing_connections sharing_connections_upper_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sharing_connections
    ADD CONSTRAINT sharing_connections_upper_user_id_fkey FOREIGN KEY (upper_user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: sharing_devices sharing_devices_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sharing_devices
    ADD CONSTRAINT sharing_devices_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: sharing_grant_stats sharing_grant_stats_grant_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sharing_grant_stats
    ADD CONSTRAINT sharing_grant_stats_grant_id_fkey FOREIGN KEY (grant_id) REFERENCES public.sharing_grants(id) ON DELETE CASCADE;


--
-- Name: sharing_grants sharing_grants_connection_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sharing_grants
    ADD CONSTRAINT sharing_grants_connection_id_fkey FOREIGN KEY (connection_id) REFERENCES public.sharing_connections(id) ON DELETE CASCADE;


--
-- Name: sharing_grants sharing_grants_recipient_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sharing_grants
    ADD CONSTRAINT sharing_grants_recipient_user_id_fkey FOREIGN KEY (recipient_user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: sharing_grants sharing_grants_sender_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sharing_grants
    ADD CONSTRAINT sharing_grants_sender_user_id_fkey FOREIGN KEY (sender_user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: sharing_invitation_stats sharing_invitation_stats_invitation_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sharing_invitation_stats
    ADD CONSTRAINT sharing_invitation_stats_invitation_id_fkey FOREIGN KEY (invitation_id) REFERENCES public.sharing_invitations(id) ON DELETE CASCADE;


--
-- Name: sharing_invitations sharing_invitations_accepted_by_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sharing_invitations
    ADD CONSTRAINT sharing_invitations_accepted_by_user_id_fkey FOREIGN KEY (accepted_by_user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: sharing_invitations sharing_invitations_inviter_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sharing_invitations
    ADD CONSTRAINT sharing_invitations_inviter_user_id_fkey FOREIGN KEY (inviter_user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: sharing_payload_chunks sharing_payload_chunks_scope_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sharing_payload_chunks
    ADD CONSTRAINT sharing_payload_chunks_scope_id_fkey FOREIGN KEY (scope_id) REFERENCES public.sharing_grant_stats(id) ON DELETE CASCADE;


--
-- Name: insight_cycles insight_cycles_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.insight_cycles
    ADD CONSTRAINT insight_cycles_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: api_rate_limits; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.api_rate_limits ENABLE ROW LEVEL SECURITY;

--
-- Name: clinical_notes; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.clinical_notes ENABLE ROW LEVEL SECURITY;

--
-- Name: clinvar; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.clinvar ENABLE ROW LEVEL SECURITY;

--
-- Name: doctor_collection_items; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.doctor_collection_items ENABLE ROW LEVEL SECURITY;

--
-- Name: doctor_preparations; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.doctor_preparations ENABLE ROW LEVEL SECURITY;

--
-- Name: frequencies; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.frequencies ENABLE ROW LEVEL SECURITY;

--
-- Name: genome_findings; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.genome_findings ENABLE ROW LEVEL SECURITY;

--
-- Name: genome_imports; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.genome_imports ENABLE ROW LEVEL SECURITY;

--
-- Name: gwas; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.gwas ENABLE ROW LEVEL SECURITY;

--
-- Name: health_contexts; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.health_contexts ENABLE ROW LEVEL SECURITY;

--
-- Name: health_evidence; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.health_evidence ENABLE ROW LEVEL SECURITY;

--
-- Name: learning_ingestion_runs; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.learning_ingestion_runs ENABLE ROW LEVEL SECURITY;

--
-- Name: learning_items; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.learning_items ENABLE ROW LEVEL SECURITY;

--
-- Name: learning_schedule; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.learning_schedule ENABLE ROW LEVEL SECURITY;

--
-- Name: learning_source_revisions; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.learning_source_revisions ENABLE ROW LEVEL SECURITY;

--
-- Name: learning_sources; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.learning_sources ENABLE ROW LEVEL SECURITY;

--
-- Name: sharing_blocks; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.sharing_blocks ENABLE ROW LEVEL SECURITY;

--
-- Name: sharing_connections; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.sharing_connections ENABLE ROW LEVEL SECURITY;

--
-- Name: sharing_devices; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.sharing_devices ENABLE ROW LEVEL SECURITY;

--
-- Name: sharing_grant_stats; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.sharing_grant_stats ENABLE ROW LEVEL SECURITY;

--
-- Name: sharing_grants; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.sharing_grants ENABLE ROW LEVEL SECURITY;

--
-- Name: sharing_invitation_stats; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.sharing_invitation_stats ENABLE ROW LEVEL SECURITY;

--
-- Name: sharing_invitations; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.sharing_invitations ENABLE ROW LEVEL SECURITY;

--
-- Name: sharing_payload_chunks; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.sharing_payload_chunks ENABLE ROW LEVEL SECURITY;

--
-- Name: users; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.users ENABLE ROW LEVEL SECURITY;

--
-- Name: insight_cycles; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.insight_cycles ENABLE ROW LEVEL SECURITY;

--
-- PostgreSQL database dump complete
--

REVOKE ALL ON ALL TABLES IN SCHEMA public FROM PUBLIC;
REVOKE EXECUTE ON ALL FUNCTIONS IN SCHEMA public FROM PUBLIC;

commit;
