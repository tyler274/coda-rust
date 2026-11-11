# Duet UI and design

The [Duet design language](https://linear.app/coda-health/document/duet-design-language-foundations-patterns-and-current-care-e3d0bc28fa6d)
is the design corpus. This page maps that direction to Rust; it does not
maintain a second visual specification.

## Direction and scope

The app has two tabs, You and Shared. Use the corpus's system sans, quieter
solid surfaces and green direction.

Person-to-person sharing is retired from product scope. Shared remains the
practice-connection tab, with no People list or person-to-person notices.
Documented care remains with its existing owner.

The patient-owned doctor workspace is in MVP: preparation, updates,
patient review and saving, including patient-provided and connected-practice
appointment context. Its mockups contain both MVP workspace states and later
sharing states; label those separately. Sharing with physicians outside Coda is
post-MVP, with patient-reviewed browser publication and no PDF download. The
jointly maintained live shared space is a separate post-MVP concept. Physicians
do not approve Duet's wording.

The design corpus links the mockups, flow sheets, doctor workspace and
clinician-preview prototype under “Mockups and interactive prototypes.”

## Rust owners

| Owner | What to inspect |
| --- | --- |
| [`duet-app::roots`](/crates/duet-app/src/roots.rs) | Tab roots, current copy and navigation |
| [`duet-app::account`](/crates/duet-app/src/account.rs), [`duet-app`](/crates/duet-app/src/lib.rs) | Standalone account entry, You and the Shared workspace |
| [`duet-page`](/crates/duet-page/src/lib.rs) | Existing doctor-page layout; retaining this crate does not qualify later browser sharing or restore PDF acceptance |
| [`duet-health`](/crates/duet-health/src/lib.rs), [`duet-notify`](/crates/duet-notify/src/lib.rs) | Native health and notification adapters; verify on installed clients under the owning outcomes |

[Release](release.md) owns platform checks. A rendered screen or sample fixture
does not establish saved state, consent, background delivery or real-model
quality; verify the relevant journey through the backend and installed client.
