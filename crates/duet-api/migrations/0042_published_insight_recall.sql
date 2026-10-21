-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

alter table public.operating_indexes
    add column published_cycle_ids uuid[] not null default '{}';

-- Retain the admitted identity for exact retry/recovery even if its source is
-- later deleted. No title, model wording or health evidence is copied here.
alter table public.conversation_messages add column selected_insight jsonb;

create function public.invalidate_published_insight_understanding() returns trigger
language plpgsql set search_path = public as $$
begin
    if tg_op = 'DELETE' then
        delete from public.operating_indexes
        where user_id = old.user_id and old.id = any(published_cycle_ids);
        return old;
    end if;
    if old.corrected_insight_ids is distinct from new.corrected_insight_ids then
        update public.operating_indexes
        set superseded_at = clock_timestamp(), revision = revision + 1
        where user_id = old.user_id and old.id = any(published_cycle_ids);
    end if;
    return new;
end;
$$;
create trigger published_insight_understanding
    after update or delete on public.insight_cycles
    for each row execute function public.invalidate_published_insight_understanding();
revoke all on function public.invalidate_published_insight_understanding() from public;

alter table public.conversation_citations
    drop constraint conversation_citations_kind_check,
    drop constraint conversation_citations_check,
    add constraint conversation_citations_kind_check check (kind in ('record','native_fact','insight')),
    add column cycle_id uuid references public.insight_cycles(id) on delete cascade,
    add column insight_id uuid,
    add constraint conversation_citations_body_check check (
        (kind='record' and record_revision is not null and native_fact is null and cycle_id is null and insight_id is null)
        or (kind='native_fact' and record_revision is null and native_fact is not null and cycle_id is null and insight_id is null)
        or (kind='insight' and record_revision is null and native_fact is null and cycle_id is not null and insight_id is not null)
    );

-- Source cards are a projection. Cascading source removal must also remove
-- their copied labels from recall, exports and connected clients.
create function public.remove_deleted_conversation_source() returns trigger
language plpgsql set search_path = public as $$
declare account_id uuid; change_revision bigint;
begin
    select user_id into account_id from public.conversation_messages
    where id=old.message_id;
    if account_id is null then return old; end if;
    change_revision := public.conversation_changed(account_id);
    update public.conversation_messages set
        sources = coalesce((select jsonb_agg(value) from jsonb_array_elements(sources)
                            where value->>'id' <> old.source_id), '[]'::jsonb),
        revision = change_revision, updated_at = now()
    where id=old.message_id and change_revision is not null;
    delete from public.operating_indexes
    where user_id=account_id and old.message_id=any(conversation_message_ids);
    return old;
end;
$$;
create trigger deleted_conversation_source
    after delete on public.conversation_citations
    for each row execute function public.remove_deleted_conversation_source();
revoke all on function public.remove_deleted_conversation_source() from public;

insert into public.schema_migrations(version) values ('0042_published_insight_recall');
commit;
