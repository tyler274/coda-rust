-- duet-release: online
begin;

-- Reservation is not acknowledgement: after an uncertain task submission reuse
-- its name, but do not let confirmed queue backlog consume the recovery page.
-- Existing rows start uncertain and are safely confirmed by their next repair.
alter table public.conversation_runs
    add column delivery_confirmed boolean not null default false;
alter table public.doctor_preparation_work
    add column delivery_confirmed boolean not null default false;

insert into public.schema_migrations(version) values ('0054_work_delivery_confirmation');
commit;
