CREATE OR REPLACE FUNCTION physician_authority_notify ()
    RETURNS TRIGGER
    LANGUAGE plpgsql
    AS $$
BEGIN
    PERFORM
        pg_notify('physician_authority', TG_TABLE_NAME);
    RETURN NULL;
END;
$$;

CREATE TRIGGER physician_accounts_authority_notify
    AFTER INSERT OR UPDATE OR DELETE OR TRUNCATE ON physician_accounts
    FOR EACH STATEMENT
    EXECUTE FUNCTION physician_authority_notify ();

CREATE TRIGGER physician_sessions_authority_notify
    AFTER UPDATE OR DELETE OR TRUNCATE ON physician_sessions
    FOR EACH STATEMENT
    EXECUTE FUNCTION physician_authority_notify ();

CREATE TRIGGER physician_memberships_authority_notify
    AFTER INSERT OR UPDATE OR DELETE OR TRUNCATE ON physician_memberships
    FOR EACH STATEMENT
    EXECUTE FUNCTION physician_authority_notify ();

CREATE TRIGGER physician_patients_authority_notify
    AFTER INSERT OR UPDATE OR DELETE OR TRUNCATE ON physician_patients
    FOR EACH STATEMENT
    EXECUTE FUNCTION physician_authority_notify ();
