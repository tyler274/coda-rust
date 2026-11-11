# Duet UI and design

The [Duet design language](https://linear.app/coda-health/document/duet-design-language-foundations-patterns-and-current-care-e3d0bc28fa6d)
is the design corpus. This page maps that direction to Rust; it does not
maintain a second visual specification.

## Direction and scope

The <a href="/intent/duet%2Emd">Duet spec</a> owns what the app does. The app
has two tabs, You and Shared, and opens on You. Shared holds For my doctor and
the practice connection, with no People list or person-to-person notices. Use
the corpus's system sans, quieter solid surfaces and green direction.

The corpus's mockups include states the spec lists as not decided, such as AI
preparation, appointment context and sharing with physicians outside Coda.
Where a mockup and the spec disagree, the spec is right. Physicians do not
approve Duet's wording.

The design corpus links the mockups, flow sheets and prototypes under
“Mockups and interactive prototypes.”

## Rust owners

| Owner | What to inspect |
| --- | --- |
| [`duet-app::roots`](/crates/duet-app/src/roots.rs) | Tab roots, current copy and navigation |
| [`duet-app::account`](/crates/duet-app/src/account.rs), [`duet-app`](/crates/duet-app/src/lib.rs) | Standalone account entry, You and Shared |
| [`duet-page`](/crates/duet-page/src/lib.rs) | Existing doctor-page layout; retaining this crate does not qualify later browser sharing or restore PDF acceptance |
| [`duet-health`](/crates/duet-health/src/lib.rs), [`duet-notify`](/crates/duet-notify/src/lib.rs) | Native health and notification adapters; verify on installed clients under the owning outcomes |

[Release](release.md) owns platform checks. A rendered screen or sample fixture
does not establish saved state, consent, background delivery or real-model
quality; verify the relevant journey through the backend and installed client.
