-- The runtime chain's first migration, m20261111_init. A later schema change
-- is a new migration in the chain; these table definitions remain unchanged.
CREATE TABLE physician_runtime_chat_attempts (
    practice character varying NOT NULL,
    physician character varying NOT NULL,
    workspace character varying NOT NULL,
    chat character varying NOT NULL,
    id character varying NOT NULL,
    "position" bigint NOT NULL,
    predecessor character varying,
    work character varying NOT NULL,
    revision bigint NOT NULL,
    status character varying NOT NULL,
    expires bigint NOT NULL,
    CONSTRAINT "pk-physician_runtime_chat_attempts" PRIMARY KEY (practice, physician, workspace, chat, id)
);

CREATE TABLE physician_runtime_chat_drafts (
    practice text NOT NULL,
    physician text NOT NULL,
    workspace text NOT NULL,
    chat text NOT NULL,
    text text NOT NULL,
    revision bigint NOT NULL,
    created_at bigint NOT NULL,
    updated_at bigint NOT NULL,
    CONSTRAINT "pk-physician_runtime_chat_drafts" PRIMARY KEY (practice, physician, workspace, chat)
);

CREATE TABLE physician_runtime_commands (
    practice character varying NOT NULL,
    physician character varying NOT NULL,
    key character varying NOT NULL,
    command json NOT NULL,
    accepted_at bigint NOT NULL,
    CONSTRAINT "pk-physician_runtime_commands" PRIMARY KEY (practice, physician, key)
);

CREATE TABLE physician_runtime_conversations (
    practice character varying NOT NULL,
    physician character varying NOT NULL,
    workspace character varying NOT NULL,
    chat character varying NOT NULL,
    status character varying NOT NULL,
    revision bigint NOT NULL,
    session_hash bytea,
    state json NOT NULL,
    CONSTRAINT "pk-physician_runtime_conversations" PRIMARY KEY (practice, physician, workspace, chat)
);

CREATE TABLE physician_runtime_events (
    practice character varying NOT NULL,
    physician character varying NOT NULL,
    workspace character varying NOT NULL,
    target character varying NOT NULL,
    owner character varying NOT NULL,
    attempt character varying NOT NULL,
    sequence bigint
        NOT NULL,
        ordinal bigint NOT NULL,
        admissible boolean NOT NULL,
        record jsonb NOT NULL,
        CONSTRAINT "pk-physician_runtime_events" PRIMARY KEY (practice, physician, workspace, target, owner, attempt, sequence)
);

CREATE TABLE physician_runtime_provider_quota (
    singleton boolean NOT NULL,
    not_before_ms bigint NOT NULL,
    CONSTRAINT physician_runtime_provider_quota_singleton_check CHECK ((singleton = true)),
    CONSTRAINT physician_runtime_provider_quota_pkey PRIMARY KEY (singleton)
);

CREATE TABLE physician_runtime_reply_drafts (
    practice text NOT NULL,
    physician text NOT NULL,
    workspace text NOT NULL,
    reply text NOT NULL,
    status character varying NOT NULL,
    revision bigint NOT NULL,
    session_hash bytea,
    state jsonb NOT NULL,
    CONSTRAINT "pk-physician_runtime_reply_drafts" PRIMARY KEY (practice, physician, workspace, reply)
);

CREATE TABLE physician_runtime_reply_revisions (
    practice text NOT NULL,
    physician text NOT NULL,
    workspace text NOT NULL,
    reply text NOT NULL,
    key text NOT NULL,
    archived_at bigint NOT NULL,
    ordinal bigint NOT NULL,
    state jsonb NOT NULL,
    CONSTRAINT "pk-physician_runtime_reply_revisions" PRIMARY KEY (practice, physician, workspace, reply, key)
);

CREATE TABLE physician_runtime_revision_attempts (
    practice character varying NOT NULL,
    physician character varying NOT NULL,
    workspace character varying NOT NULL,
    reply character varying NOT NULL,
    id character varying NOT NULL,
    input_id character varying NOT NULL,
    "position" bigint NOT NULL,
    status character varying NOT NULL,
    expires bigint NOT NULL,
    sources_version bigint NOT NULL,
    sources jsonb NOT NULL,
    refreshes jsonb NOT NULL,
    CONSTRAINT "pk-physician_runtime_revision_attempts" PRIMARY KEY (practice, physician, workspace, reply, id)
);

CREATE TABLE physician_runtime_source_pulls (
    workspace text NOT NULL,
    source_id text NOT NULL,
    source_label text NOT NULL,
    patient jsonb NOT NULL,
    pulled_at bigint NOT NULL,
    CONSTRAINT "pk-physician_runtime_source_pulls" PRIMARY KEY (workspace, source_id)
);

CREATE TABLE physician_runtime_source_versions (
    workspace text NOT NULL,
    version bigint NOT NULL,
    CONSTRAINT physician_runtime_source_versions_pkey PRIMARY KEY (workspace)
);

CREATE TABLE physician_runtime_sources (
    workspace text NOT NULL,
    candidate text NOT NULL,
    observed_at_version bigint NOT NULL,
    revision jsonb,
    CONSTRAINT "pk-physician_runtime_sources" PRIMARY KEY (workspace, candidate, observed_at_version)
);

CREATE UNIQUE INDEX "idx-physician_runtime_reply_revisions-draft_ordinal" ON physician_runtime_reply_revisions USING btree (practice, physician, workspace, reply, ordinal);

CREATE TABLE physician_runtime_recipes (
    practice character varying NOT NULL,
    physician character varying NOT NULL,
    id character varying NOT NULL,
    body jsonb NOT NULL,
    PRIMARY KEY (practice, physician, id)
);

CREATE TABLE physician_runtime_skill_packages (
    practice character varying NOT NULL,
    physician character varying NOT NULL,
    id character varying NOT NULL,
    body jsonb NOT NULL,
    PRIMARY KEY (practice, physician, id)
);
