-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- An authorization code the public OAuth callback received and has not yet
-- exchanged. The callback carries no Duet session, so it only consumes the
-- connection's state and holds `{state, code}` here, encrypted with the
-- connector credential key and bound to this connection, behind a random
-- one-time completion handle (stored only as its SHA-256). The app's own
-- signed-in session then redeems the handle; the row is deleted on first use,
-- whoever presents it, and the code is exchanged only for the account that
-- owns the connection (`ConnectorStore.redeem_authorization`).
--
-- One held code per connection: a later callback replaces it. `expires_at` is
-- a decision-time anchor written from the database clock, about two minutes
-- after the callback. `authorization_revision` is the connection's revision
-- when the state was consumed; a disconnect in between advances it and the
-- held code can no longer be redeemed. Deleting the connection or the account
-- removes the row.
--
-- Online: a new empty table that the serving image never reads or writes. Its
-- key, unique index and foreign key build without scanning anything; the
-- foreign key briefly locks source_connections against writes while it is
-- created.
create table public.connector_authorization_completions (
    source_connection_id uuid primary key
        references public.source_connections(id) on delete cascade,
    handle_hash text not null unique,
    authorization_revision bigint not null,
    ciphertext bytea not null,
    expires_at timestamptz not null
);

revoke all on public.connector_authorization_completions from public;
insert into public.schema_migrations(version) values('0089_connector_authorization_completion');
commit;
