-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- A recipient pages its shared chunks by the order they reached it: each
-- upload's chunks get `receive_seq`, taken from `sharing_receive_seq` while
-- the upload holds a transaction lock on its recipient until it commits
-- (`SharingStore.upsert_chunks`). A later value therefore always belongs to a
-- later commit, so a recipient paging on `(receive_seq, id)` never passes a
-- chunk that has yet to commit, and a changed chunk moves to the end. Paging on
-- `updated_at` could, because it is the transaction's start and two senders'
-- uploads to one recipient can commit in the other order.
--
-- Existing chunks are numbered per recipient in their previous `(updated_at,
-- id)` order, and later uploads continue above every number.
--
-- Maintenance: joins the release's window with 0077, 0080-0082 and 0084. The
-- serving runtime inserts chunks without this column.
create sequence public.sharing_receive_seq as bigint;

alter table public.sharing_payload_chunks add column receive_seq bigint;

update public.sharing_payload_chunks payload
set receive_seq = numbered.receive_seq
from (
    select chunk.id,
           row_number() over (
               partition by grant_row.recipient_user_id
               order by chunk.updated_at, chunk.id
           ) as receive_seq
    from public.sharing_payload_chunks chunk
    join public.sharing_grant_stats scope on scope.id = chunk.scope_id
    join public.sharing_grants grant_row on grant_row.id = scope.grant_id
) numbered
where payload.id = numbered.id;

select setval(
    'public.sharing_receive_seq',
    coalesce((select max(receive_seq) from public.sharing_payload_chunks), 0) + 1,
    false
);

alter table public.sharing_payload_chunks alter column receive_seq set not null;

drop index public.sharing_payload_chunks_scope_updated_idx;
create index sharing_payload_chunks_scope_receive_idx
    on public.sharing_payload_chunks (scope_id, receive_seq, id);

insert into public.schema_migrations(version) values('0086_sharing_receive_order');
commit;
