# Clinician directory, care team, and records requests

The product-neutral `npi-registry` crate reads public CMS NPPES API v2.1 records.
It validates NPI check digits, bounds search results, uses request timeouts and no
redirects, and keeps a short in-memory NPI lookup cache. Directory data includes
individuals and organizations, names and credentials, taxonomies, practice and
mailing addresses, phone/fax, endpoints, and the directory update date. NPPES is
not proof of identity, licensure, a patient relationship, or current contact details.

## Separate product owners

Duet stores private care-team membership and release-review templates in its own
database. Physician accounts store their own self-asserted NPI association and
public snapshot in `physician-auth`. Neither product imports the other's account
or clinical types. The public directory crate has no product accounts or durable
application storage. There is no automatic link from a patient's care team to a
Coda physician account.

Duet's `0016_care_team` migration belongs to `duet-migrate`, not API startup or the
reference application's migration ledger. It grants the runtime role access to
`duet_care_team` and `duet_release_authorizations`. The bounded SeaORM adoption
shares the existing SQLx 0.9 runtime pool; ordinary care-team and template CRUD
uses typed ORM queries. Account locking and privacy/session fencing remain with
Duet's existing owners. Account deletion removes both tables' rows; deleting a
member also removes templates for which that member is the discloser.

Physician NPI fields are added by the third named `physician-auth` migration.
The association requires an individual record and a normalized surname match,
unless an authorized administrator overrides the match. Uniqueness prevents two
accounts claiming the same NPI. Clearing the association removes the stored NPI
snapshot. `npi_verified_at` records the directory check time, **not** identity or
credential verification. The UI calls this a self-asserted association.

## Patient journey

In **You → Care team**, patients search by name or organization and optionally
location/specialty, add a directory result, edit private labels, refresh the public
snapshot, or remove the member. Search uses authenticated POST requests so search
terms do not appear in request URLs. Server-side add fetches the directory record;
clients cannot submit an arbitrary clinician snapshot. Responses are private and
not cacheable. All membership and template access is account scoped.

Call links accept only constrained dialable numbers. Fax copying has a manual
fallback. Maps links contain the public address, are percent encoded, and do not
send a referrer. Direct endpoints are displayed as text, not ordinary email links.
Duet does not send messages, faxes, forms, or clinical records to any contact.
A refresh updates membership data, not previously saved template snapshots;
saving edits to a template resolves the currently saved member snapshots again.
Confirm destinations directly before using a form, including after refresh.

**Request my records** selects a discloser and recipient (patient, care-team member,
or manually entered party), then edits patient information, scope, purpose,
expiration date/event, and an optional typed name. Only the patient's display name
can be prefilled from the current profile: DOB, address, and phone are not inferred.
The UI currently requires a patient name, scope, and expiration before preview;
the API can store incomplete drafts. PUT replaces the editable body, including
clearing a previously supplied typed name when omitted. `signed_at` is the server
time of typed-name submission, not authenticated signing evidence.

The saved template has an SVG image preview and an accessible textual review.
Edits invalidate review, and a reviewed current preview is required before the UI
hands the private PDF to download/share. The renderer paginates long text and
refuses PDF export for unsupported text or unfaithful layout. Rendering or download
is not delivery to a recipient.

## Template limitations and recovery

The template includes revocation, non-conditioning, redisclosure, and copy-right
statements, along with patient/discloser/recipient details and expiration. It is a
review aid, not a guarantee of HIPAA or state-law compliance, recipient acceptance,
or an authenticated electronic signature. Psychotherapy notes are excluded and
require separate authorization. Sensitive-category initials do not establish
42 CFR Part 2 or other specialized consent compliance; recipient/legal review and
additional forms may be required.

Care-team mutations reconcile uncertain outcomes through a fresh list read. A
known template ID is retained while the form stays mounted, and subsequent saves
update it rather than create another template. An uncertain initial template POST
blocks further creation **only in the current UI session**. There is currently no
release-list/read recovery endpoint, idempotency key, or durable cross-session
recovery UI. A reload, account switch, or another tab cannot recover that ambiguous
create. Do not blindly resubmit: support reconciliation and durable idempotent
recovery are follow-up work before claiming robust lost-response recovery.

Account-scoped UI is reset on account transitions; private drafts, results, and
previews are not placed in local storage. Downloaded PDFs and copied fax numbers
are outside that UI retention boundary and must be handled carefully.

## Proof and remaining device qualification

Synthetic NPPES fixtures are used by the registry, route, and browser tests; no
patient information or live directory lookup is needed. `duet-api/tests/care_team.rs`
requires explicit loopback `DUET_TEST_DATABASE_URL` and creates a disposable
database/runtime role. It checks membership uniqueness, account isolation,
release replacement, PDF access, account deletion, and deletion during lookup.
`duet-page/tests/release.rs` checks SVG layout and extracts PDF text using
`pdftotext`. The existing `mobile` Chromium durable journey exercises the compiled
patient UI against real routes and disposable PostgreSQL, including directory
search/add, contact affordances, label changes, template edits/review, SVG preview,
and PDF download handoff. It does not place a call, send a fax, or authenticate a
signature.

Phone-sized Chromium is not iOS/Android qualification. Native share sheets,
clipboard permissions, telephone handoff, external maps, assistive technology,
and real-device account transitions still require device testing. Live NPPES
availability is a separate opt-in test, not a requirement for deterministic proof.
