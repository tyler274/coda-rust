-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- A time a later decision is computed from (a deadline, an expiry, a backoff
-- or a window) is an anchor, and its writer passes the decision time read
-- after its locks (`app/services/decision_time.py`), or the patient
-- calendar's time for a calendar decision. A clock default would write the
-- transaction's start instead, so an anchor has none, and the database
-- refuses a writer that does not supply it:
--
-- - users.created_at starts the calendar's inactivity windows until the
--   first foreground;
-- - conversation_runs.created_at is counted by the daily allowance;
-- - source_connections.created_at and source_acquisition_units.expires_at
--   start the acquisition's freshness and its day-long attempt;
-- - follow_up_push_deliveries.created_at starts a notice's expiry and backoff;
-- - conversation_reply_feedback.updated_at files a rating under its UTC day.
--
-- `tests/test_decision_time_rule_db.py` holds every clock default to this.
--
-- Maintenance: joins the release's window with 0077 and 0080-0082. The
-- serving runtime inserts these rows without these columns.
alter table public.users alter column created_at drop default;
alter table public.conversation_runs alter column created_at drop default;
alter table public.source_connections alter column created_at drop default;
alter table public.source_acquisition_units alter column expires_at drop default;
alter table public.follow_up_push_deliveries alter column created_at drop default;
alter table public.conversation_reply_feedback alter column updated_at drop default;

insert into public.schema_migrations(version) values('0084_decision_time_anchors');
commit;
