-- duet-release: online
begin;

set local lock_timeout = '5s';
set local statement_timeout = '60s';

alter table public.source_acquisition_units
    add column delivery_generation bigint not null default 0
        check (delivery_generation >= 0),
    add column delivery_until timestamptz;

insert into public.schema_migrations(version)
values ('0031_acquisition_delivery_generation');

commit;
