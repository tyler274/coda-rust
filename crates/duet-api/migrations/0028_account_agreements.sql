begin;

-- Existing accounts intentionally remain unaccepted so the public release
-- obtains fresh, versioned assent from beta users as well as new users.
alter table public.users
    add column terms_version text,
    add column terms_accepted_at timestamptz,
    add column luna_consent_version text,
    add column luna_consent_decision text,
    add column luna_consent_at timestamptz,
    add constraint users_terms_acceptance_pair_check check (
        (terms_version is null) = (terms_accepted_at is null)
    ),
    add constraint users_luna_consent_complete_check check (
        (luna_consent_version is null)
        = (luna_consent_decision is null)
        and (luna_consent_version is null) = (luna_consent_at is null)
    ),
    add constraint users_luna_consent_decision_check check (
        luna_consent_decision is null
        or luna_consent_decision in ('accepted', 'declined')
    );

insert into public.schema_migrations(version)
values ('0028_account_agreements');

commit;
