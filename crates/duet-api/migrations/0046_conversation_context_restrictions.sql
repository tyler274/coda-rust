-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- A restriction is one reversible patient instruction. The original transcript
-- remains the visible owner; exact spans below only govern future AI reads.
create table public.conversation_context_restrictions (
    user_id uuid not null references public.users(id) on delete cascade,
    id uuid not null,
    generation bigint not null default 1 check(generation > 0),
    state text not null check(state in ('active','restored')),
    source_message_id uuid,
    created_at timestamptz not null default clock_timestamp(),
    updated_at timestamptz not null default clock_timestamp(),
    primary key(user_id,id),
    foreign key(user_id,source_message_id)
        references public.conversation_messages(user_id,id)
        on delete set null (source_message_id),
    check(updated_at >= created_at)
);

create table public.conversation_context_restriction_spans (
    user_id uuid not null,
    restriction_id uuid not null,
    message_id uuid not null,
    text_sha256 text not null check(text_sha256 ~ '^[0-9a-f]{64}$'),
    start_offset integer not null check(start_offset >= 0),
    end_offset integer not null check(end_offset > start_offset),
    primary key(user_id,restriction_id,message_id,start_offset,end_offset),
    foreign key(user_id,restriction_id)
        references public.conversation_context_restrictions(user_id,id)
        on delete cascade,
    foreign key(user_id,message_id)
        references public.conversation_messages(user_id,id)
        on delete cascade
);
create index conversation_context_active
    on public.conversation_context_restrictions(user_id,id)
    where state='active';
create index conversation_context_spans_message
    on public.conversation_context_restriction_spans(user_id,message_id,start_offset,end_offset);

-- Closed model-reader units outside message prose. Metadata units are removed
-- whole; published prose keeps exact field ranges on its immutable owner.
create table public.conversation_context_restriction_items (
    user_id uuid not null,
    restriction_id uuid not null,
    kind text not null check(kind in(
        'conversation_source','conversation_action','selected_context',
        'published_insight_field'
    )),
    message_id uuid,
    item_id text,
    cycle_id uuid,
    insight_id uuid,
    field_name text,
    text_sha256 text,
    start_offset integer,
    end_offset integer,
    foreign key(user_id,restriction_id)
        references public.conversation_context_restrictions(user_id,id)
        on delete cascade,
    foreign key(user_id,message_id)
        references public.conversation_messages(user_id,id)
        on delete cascade,
    check(
        (kind='conversation_source' and message_id is not null
            and item_id is not null and cycle_id is null and insight_id is null
            and field_name is null and text_sha256 is null
            and start_offset is null and end_offset is null)
        or (kind='conversation_action' and message_id is not null
            and item_id is not null and cycle_id is null and insight_id is null
            and field_name is null and text_sha256 is null
            and start_offset is null and end_offset is null)
        or (kind='selected_context' and message_id is not null
            and item_id is null and cycle_id is null and insight_id is null
            and field_name is null and text_sha256 is null
            and start_offset is null and end_offset is null)
        or (kind='published_insight_field' and message_id is null
            and item_id is null and cycle_id is not null and insight_id is not null
            and field_name in('title','detail')
            and text_sha256 ~ '^[0-9a-f]{64}$'
            and start_offset >= 0 and end_offset > start_offset)
    )
);
create unique index conversation_context_restriction_items_identity
    on public.conversation_context_restriction_items(
        user_id,restriction_id,kind,
        coalesce(message_id,'00000000-0000-0000-0000-000000000000'::uuid),
        coalesce(item_id,''),
        coalesce(cycle_id,'00000000-0000-0000-0000-000000000000'::uuid),
        coalesce(insight_id,'00000000-0000-0000-0000-000000000000'::uuid),
        coalesce(field_name,''),coalesce(start_offset,-1),coalesce(end_offset,-1)
    );

-- Exact eligible read coverage replaces whole-message-only receipts. Existing
-- rows remain empty until their actual eligible spans are reassessed; a message
-- hash alone cannot manufacture semantic lineage.
alter table public.operating_index_conversation_reads
    add column spans jsonb not null default '[]'::jsonb
        check(jsonb_typeof(spans)='array'),
    add column context_policy_revision bigint
        check(context_policy_revision >= 0);

-- Follow-up provenance uses exact jointly-required request/acceptance spans.
-- Keeping message UUIDs separately preserves the existing public projection.
alter table public.follow_ups
    add column source_spans jsonb not null default '[]'::jsonb
        check(jsonb_typeof(source_spans)='array');

create table public.follow_up_context_restrictions (
    user_id uuid not null,
    follow_up_id uuid not null,
    restriction_id uuid not null,
    created_at timestamptz not null default clock_timestamp(),
    primary key(user_id,follow_up_id,restriction_id),
    foreign key(user_id,follow_up_id)
        references public.follow_ups(user_id,id) on delete cascade,
    foreign key(user_id,restriction_id)
        references public.conversation_context_restrictions(user_id,id)
        on delete cascade
);
create table public.conversation_context_restriction_follow_ups (
    user_id uuid not null,
    restriction_id uuid not null,
    follow_up_id uuid not null,
    assessed_revision bigint not null check(assessed_revision > 0),
    outcome text not null check(outcome in('retain','pause_source_restricted')),
    primary key(user_id,restriction_id,follow_up_id),
    foreign key(user_id,restriction_id)
        references public.conversation_context_restrictions(user_id,id) on delete cascade,
    foreign key(user_id,follow_up_id)
        references public.follow_ups(user_id,id) on delete cascade
);
-- Restriction actions project their confirmed durable outcome into the same
-- initiating message as every other conversation-owned action.
alter table public.conversation_action_results
    add column restriction_id uuid,
    drop constraint conversation_action_results_kind_check,
    drop constraint conversation_action_results_target_check,
    add constraint conversation_action_results_kind_check check(kind in(
        'save_doctor_question','prepare_doctor_summary',
        'create_saved_routine','update_saved_routine',
        'create_follow_up','update_follow_up','pause_follow_up',
        'resume_follow_up','cancel_follow_up',
        'forget_conversation_detail','restore_conversation_detail'
    )),
    add constraint conversation_action_results_target_check check(
        (kind in('save_doctor_question','prepare_doctor_summary')
            and routine_id is null and follow_up_id is null and restriction_id is null)
        or (kind in('create_saved_routine','update_saved_routine')
            and routine_id is not null and follow_up_id is null and restriction_id is null)
        or (kind in('create_follow_up','update_follow_up','pause_follow_up',
                    'resume_follow_up','cancel_follow_up')
            and routine_id is null and follow_up_id is not null and restriction_id is null)
        or (kind in('forget_conversation_detail','restore_conversation_detail')
            and routine_id is null and follow_up_id is null and restriction_id is not null)
    ),
    add constraint conversation_action_results_restriction_fkey
        foreign key(user_id,restriction_id)
        references public.conversation_context_restrictions(user_id,id)
        on delete cascade;
create index conversation_action_results_restriction
    on public.conversation_action_results(user_id,restriction_id,result_generation,conversation_revision)
    where restriction_id is not null;

revoke all on public.conversation_context_restrictions,
    public.conversation_context_restriction_spans,
    public.conversation_context_restriction_items,
    public.conversation_context_restriction_follow_ups,
    public.follow_up_context_restrictions from public;

insert into public.schema_migrations(version)
values('0046_conversation_context_restrictions');
commit;
