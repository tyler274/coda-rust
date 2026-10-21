-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Read coverage is metadata of the one current understanding, not another
-- transcript or a claim that the model interpreted every message correctly.
alter table public.operating_indexes
    add constraint operating_indexes_user_id_id_key unique(user_id,id);

create table public.operating_index_conversation_reads (
    user_id uuid not null,
    index_id uuid not null,
    message_id uuid not null,
    text_sha256 text not null check(text_sha256 ~ '^[0-9a-f]{64}$'),
    recorded_at timestamptz not null default now(),
    primary key(index_id,message_id),
    foreign key(user_id,index_id) references public.operating_indexes(user_id,id)
        on delete cascade,
    foreign key(user_id,message_id) references public.conversation_messages(user_id,id)
        on delete cascade
);
revoke all on public.operating_index_conversation_reads from public;

insert into public.schema_migrations(version) values ('0038_conversation_memory_reads');
commit;
