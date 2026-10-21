-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- A preparation whose attempt ended on a transient fault runs again as the
-- same work, up to three attempts. A constant default adds no rewrite, and
-- serving code that does not know the column leaves it at its first attempt.
alter table public.doctor_preparation_work
    add column attempts integer not null default 1;

insert into public.schema_migrations(version) values('0101_doctor_preparation_attempts');
commit;
