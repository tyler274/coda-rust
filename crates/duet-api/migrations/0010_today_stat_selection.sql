begin;

alter table public.users
  add column today_stat_ids text[];

alter table public.users
  add constraint users_today_stat_ids_count_check
  check (
    today_stat_ids is null
    or cardinality(today_stat_ids) between 1 and 4
  );

alter table public.users
  add constraint users_today_stat_ids_shape_check
  check (
    today_stat_ids is null
    or array_to_string(today_stat_ids, '|') ~
      '^(metric:(steps|active_energy|exercise_minutes|sleep_minutes|resting_heart_rate|distance|body_mass|dietary_energy|water)|signal:[a-z0-9_]+)(\|(metric:(steps|active_energy|exercise_minutes|sleep_minutes|resting_heart_rate|distance|body_mass|dietary_energy|water)|signal:[a-z0-9_]+))*$'
  );

insert into public.schema_migrations (version)
values ('0010_today_stat_selection');

commit;
