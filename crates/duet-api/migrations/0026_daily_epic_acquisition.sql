begin;

-- Manual refresh never consumes the independent once-per-UTC-day automatic
-- acquisition claim. Existing values came only from manual/foreground calls
-- because no production daily sweep existed, so they carry no daily authority.
alter table public.source_connections
    drop column last_refresh_day,
    add column last_automatic_refresh_day date;

insert into public.schema_migrations(version)
values ('0026_daily_epic_acquisition');

commit;
