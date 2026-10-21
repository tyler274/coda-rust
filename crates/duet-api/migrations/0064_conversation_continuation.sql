-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- A continuation is one replaceable, lossy model note over an exact terminal
-- transcript prefix. The transcript remains canonical and independently
-- retained; this row is neither a source record nor an action/read receipt.
alter table public.conversation_state
    add column transcript_generation bigint not null default 0
        check (transcript_generation >= 0);

create table public.conversation_continuations (
    user_id uuid primary key references public.users(id) on delete cascade,
    generation uuid not null,
    markdown text not null
        check (markdown ~ '[^[:space:]]')
        check (octet_length(markdown) <= 16384),
    through_sequence bigint not null check (through_sequence > 0),
    through_message_id uuid not null,
    updated_at timestamptz not null default now(),
    foreign key (user_id, through_message_id)
        references public.conversation_messages(user_id, id)
        on delete cascade
);

-- Inserts append transcript input and do not invalidate an existing prefix.
-- Streaming/pending publication is likewise outside the stable prefix. Once a
-- row is terminal, however, changing its model-visible identity or content is
-- a new transcript generation. Deletion is always material. The generation
-- also fences the initial no-checkpoint snapshot against edit/delete ABA.
create function public.conversation_terminal_input_changed() returns trigger
language plpgsql as $$
declare
    account_id uuid;
    affected_sequence bigint;
    material boolean;
begin
    if tg_op = 'DELETE' then
        if old.state not in ('complete', 'interrupted', 'failed') then
            return old;
        end if;
        account_id := old.user_id;
        affected_sequence := old.sequence;
        material := true;
    else
        if old.state not in ('complete', 'interrupted', 'failed') then
            return new;
        end if;
        material := (new.user_id, new.id, new.role, new.text, new.state,
                     new.sequence, new.sources, new.selected_insight,
                     new.selected_context, new.reply_to_message_id)
            is distinct from
                    (old.user_id, old.id, old.role, old.text, old.state,
                     old.sequence, old.sources, old.selected_insight,
                     old.selected_context, old.reply_to_message_id);
        if not material then
            return new;
        end if;
        account_id := old.user_id;
        affected_sequence := least(old.sequence, new.sequence);
    end if;

    -- A parent-account cascade may delete message children after the user is
    -- already invisible in this transaction (including restored schemas whose
    -- FK triggers were recreated in a different order). The parent cascade
    -- owns all child erasure; never write a surviving child in that path.
    if not exists (
        select 1 from public.users where id = account_id
    ) then
        if tg_op = 'DELETE' then
            return old;
        end if;
        return new;
    end if;

    update public.conversation_state
       set transcript_generation = transcript_generation + 1
     where user_id = account_id;
    delete from public.conversation_continuations
     where user_id = account_id and through_sequence >= affected_sequence;
    if tg_op = 'DELETE' then
        return old;
    end if;
    return new;
end $$;

create trigger conversation_terminal_input_mutated
    after update on public.conversation_messages
    for each row execute function public.conversation_terminal_input_changed();
create trigger conversation_terminal_input_deleted
    before delete on public.conversation_messages
    for each row execute function public.conversation_terminal_input_changed();

revoke all on public.conversation_continuations from public;
revoke all on function public.conversation_terminal_input_changed() from public;

insert into public.schema_migrations(version)
values ('0064_conversation_continuation');
commit;
