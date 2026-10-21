-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Provision vector through the database extension administrator before the
-- restricted migrator runs. Local/CI databases may create it as their owner.
create extension if not exists vector with schema public;

-- Disposable retrieval state. Canonical rows and their exact readers remain
-- the only owners of patient evidence, conversation and saved work.
create table public.health_discovery_pending (
    user_id uuid not null references public.users(id) on delete cascade,
    source_kind text not null,
    source_id text not null,
    generation uuid not null default gen_random_uuid(),
    queued_at timestamptz not null default clock_timestamp(),
    changed_at timestamptz not null default clock_timestamp(),
    primary key (user_id,source_kind,source_id)
);
create index health_discovery_pending_fair
    on public.health_discovery_pending(user_id,queued_at,source_kind,source_id);

create table public.health_discovery_documents (
    id uuid primary key,
    user_id uuid not null references public.users(id) on delete cascade,
    source_kind text not null,
    source_id text not null,
    generation uuid not null,
    family text not null,
    reference jsonb not null,
    metadata jsonb not null,
    occurred_at timestamptz,
    conversation_sequence bigint,
    snapshot_id uuid,
    text text not null,
    input_sha256 text not null,
    indexed_at timestamptz not null default clock_timestamp(),
    unique(user_id,id,generation)
);
create index health_discovery_source
    on public.health_discovery_documents(user_id,source_kind,source_id);
create index health_discovery_scope
    on public.health_discovery_documents(user_id,family,occurred_at,id);
create index health_discovery_embedding_pending
    on public.health_discovery_documents(user_id,indexed_at,id);

-- Each lexical passage is only a numerical/text index boundary. It never
-- becomes an evidence unit or a substitute for reading the referenced owner.
create table public.health_discovery_terms (
    document_id uuid not null references public.health_discovery_documents(id) on delete cascade,
    part integer not null,
    text text not null,
    terms tsvector not null,
    embedding_recipe text,
    embedding_claim uuid,
    embedding_claim_until timestamptz,
    primary key(document_id,part)
);
create index health_discovery_terms_search on public.health_discovery_terms using gin(terms);

create table public.health_discovery_embeddings (
    user_id uuid not null,
    document_id uuid not null,
    generation uuid not null,
    passage integer not null,
    part integer not null,
    recipe text not null,
    embedding public.vector(384) not null,
    primary key(document_id,passage,part),
    foreign key(user_id,document_id,generation)
        references public.health_discovery_documents(user_id,id,generation) on delete cascade
);
create index health_discovery_embeddings_account
    on public.health_discovery_embeddings(user_id,recipe,document_id);

-- Invalidation and the durable refresh marker commit with the canonical
-- mutation. A delayed indexer can never resurrect the previous generation.
create function public.invalidate_health_discovery(account uuid,kind text,source text)
returns void language plpgsql set search_path=pg_catalog,public as $$
begin
    delete from public.health_discovery_documents
        where user_id=account and source_kind=kind and source_id=source;
    if not exists(select 1 from public.users where id=account) then return; end if;
    insert into public.health_discovery_pending(user_id,source_kind,source_id)
        values(account,kind,source)
    on conflict(user_id,source_kind,source_id) do update
        set generation=gen_random_uuid(),changed_at=clock_timestamp();
end $$;

create function public.health_discovery_source_changed() returns trigger
language plpgsql set search_path=pg_catalog,public as $$
declare value jsonb; account uuid; source text;
begin
    value := case when tg_op='DELETE' then to_jsonb(old) else to_jsonb(new) end;
    account := (value->>'user_id')::uuid;
    source := case when tg_argv[1]='account' then account::text else value->>tg_argv[1] end;
    if tg_op='UPDATE' and to_jsonb(old)=to_jsonb(new) then return null; end if;
    -- Streaming is not accepted history, and indexing never delays a delta.
    if tg_argv[0]='conversation' and value->>'state' in('pending','streaming') then
        delete from public.health_discovery_documents
            where user_id=account and source_kind='conversation' and source_id=source;
        delete from public.health_discovery_pending
            where user_id=account and source_kind='conversation' and source_id=source;
        return null;
    end if;
    perform public.invalidate_health_discovery(account,tg_argv[0],source);
    return null;
end $$;

create trigger health_discovery_conversation after insert or update or delete
    on public.conversation_messages for each row
    execute function public.health_discovery_source_changed('conversation','id');
create trigger health_discovery_record after insert or update or delete
    on public.canonical_evidence_revisions for each row
    execute function public.health_discovery_source_changed('record','revision');
create trigger health_discovery_understanding after insert or update or delete
    on public.operating_indexes for each row
    execute function public.health_discovery_source_changed('understanding','account');
create trigger health_discovery_routine after insert or update or delete
    on public.saved_routines for each row
    execute function public.health_discovery_source_changed('saved_routine','id');
create trigger health_discovery_follow_up after insert or update or delete
    on public.follow_ups for each row
    execute function public.health_discovery_source_changed('follow_up','id');
create trigger health_discovery_follow_up_restriction after insert or update or delete
    on public.follow_up_context_restrictions for each row
    execute function public.health_discovery_source_changed('follow_up','follow_up_id');
create trigger health_discovery_insight after insert or update or delete
    on public.insight_cycles for each row
    execute function public.health_discovery_source_changed('published_insight','id');
create trigger health_discovery_native after insert or update or delete
    on public.health_contexts for each row
    execute function public.health_discovery_source_changed('native_fact','account');
create trigger health_discovery_doctor_items after insert or update or delete
    on public.doctor_collection_items for each row
    execute function public.health_discovery_source_changed('doctor_workspace','account');
create trigger health_discovery_doctor_preparation after insert or update or delete
    on public.doctor_preparations for each row
    execute function public.health_discovery_source_changed('doctor_workspace','account');

create function public.health_discovery_doctor_note_changed() returns trigger
language plpgsql set search_path=pg_catalog,public as $$
begin
    if new.doctor_note is distinct from old.doctor_note then
        perform public.invalidate_health_discovery(new.id,'doctor_workspace',new.id::text);
    end if;
    return null;
end $$;
create trigger health_discovery_doctor_note after update of doctor_note on public.users
    for each row execute function public.health_discovery_doctor_note_changed();

-- Changes to privacy state, including recovery replay, invalidate only the
-- affected canonical units. Python remains the sole interpretation of offsets.
create function public.invalidate_health_discovery_restriction(account uuid,restriction uuid)
returns void language plpgsql set search_path=pg_catalog,public as $$
declare item record;
begin
    for item in
        select distinct 'conversation' as kind,message_id::text as source
        from public.conversation_context_restriction_spans
        where user_id=account and restriction_id=restriction
        union
        select distinct case when message_id is not null then 'conversation'
                             else 'published_insight' end,
                        coalesce(message_id,cycle_id)::text
        from public.conversation_context_restriction_items
        where user_id=account and restriction_id=restriction
    loop
        perform public.invalidate_health_discovery(account,item.kind,item.source);
    end loop;
end $$;

create function public.health_discovery_restriction_changed() returns trigger
language plpgsql set search_path=pg_catalog,public as $$
declare value jsonb; account uuid; source text; kind text;
begin
    value := case when tg_op='DELETE' then to_jsonb(old) else to_jsonb(new) end;
    account := (value->>'user_id')::uuid;
    if tg_table_name='conversation_context_restrictions' then
        perform public.invalidate_health_discovery_restriction(account,(value->>'id')::uuid);
    else
        kind := case when value->>'message_id' is not null then 'conversation' else 'published_insight' end;
        source := coalesce(value->>'message_id',value->>'cycle_id');
        perform public.invalidate_health_discovery(account,kind,source);
    end if;
    return null;
end $$;
create trigger health_discovery_restriction after insert or update or delete
    on public.conversation_context_restrictions for each row
    execute function public.health_discovery_restriction_changed();
create trigger health_discovery_restricted_span after insert or update or delete
    on public.conversation_context_restriction_spans for each row
    execute function public.health_discovery_restriction_changed();
create trigger health_discovery_restricted_item after insert or update or delete
    on public.conversation_context_restriction_items for each row
    execute function public.health_discovery_restriction_changed();

-- Initial catch-up is resumable and contains only canonical identities.
insert into public.health_discovery_pending(user_id,source_kind,source_id)
select user_id,'conversation',id::text from public.conversation_messages
    where state not in('pending','streaming')
union all select user_id,'record',revision::text from public.canonical_evidence_revisions
union all select user_id,'understanding',user_id::text from public.operating_indexes
union all select user_id,'saved_routine',id::text from public.saved_routines
union all select user_id,'follow_up',id::text from public.follow_ups
union all select user_id,'published_insight',id::text from public.insight_cycles where status='ready'
union all select user_id,'native_fact',user_id::text from public.health_contexts
union all select id,'doctor_workspace',id::text from public.users u
    where coalesce(doctor_note,'')<>''
       or exists(select 1 from public.doctor_collection_items where user_id=u.id)
       or exists(select 1 from public.doctor_preparations where user_id=u.id);

revoke all on public.health_discovery_pending,public.health_discovery_documents,
    public.health_discovery_terms,public.health_discovery_embeddings from public;
revoke all on function public.invalidate_health_discovery(uuid,text,text),
    public.health_discovery_source_changed(),public.health_discovery_doctor_note_changed(),
    public.invalidate_health_discovery_restriction(uuid,uuid),
    public.health_discovery_restriction_changed() from public;

insert into public.schema_migrations(version) values('0047_health_discovery');
commit;
