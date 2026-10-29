# Duet UI and design

The [Duet design language](https://linear.app/coda-health/document/duet-design-language-foundations-patterns-and-current-care-e3d0bc28fa6d)
is the design corpus. This page maps that direction to Rust; it does not
maintain a second visual specification.

## Direction and scope

Keep one continuous conversation. Today is the home for insights and current
care: Plans, reminders and Check-ins move out of the companion card, and the
duplicate care hub is retired. Use the corpus's system sans, quieter solid
surfaces and green direction. The full Today → selected item in chat → accepted
save → same current item and originating conversation path must remain coherent
across the client, backend and model interaction.

Person-to-person Shared is retired from product scope. Practice connection
controls and documented care remain reachable. Their final placement is an
open design question; removing a tab must not silently redesign consent.

The patient-owned doctor workspace is in MVP: preparation, updates,
patient review and saving, including patient-provided and connected-practice
appointment context. Its mockups contain both MVP workspace states and later
sharing states; label those separately. Sharing with physicians outside Coda is
post-MVP, with patient-reviewed browser publication and no PDF download. The
jointly maintained live shared space is a separate post-MVP concept. Physicians
do not approve Duet's wording.

The design corpus links the mockups, flow sheets, doctor workspace and
clinician-preview prototype under “Mockups and interactive prototypes.”

## Rust owners and gaps

| Owner | What to inspect |
| --- | --- |
| [`ui`](/crates/ui/src/lib.rs) | Shared tokens (light and dark), icons, buttons, navigation links, read and command notices, and tabs; product-specific layout remains in each app |
| [`duet-app::roots`](/crates/duet-app/src/roots.rs) | Tab roots, current copy and navigation; the existing four-tab/Shared surface still needs removal of person-to-person sharing |
| [`duet-app::account`](/crates/duet-app/src/account.rs), [`duet-app`](/crates/duet-app/src/lib.rs) | Standalone account entry and conversation/connection screens; the old join gate is disabled, while the retained sharing toggle still conflicts with fixed consent |
| [`duet-page`](/crates/duet-page/src/lib.rs) | Existing doctor-page layout; retaining this crate does not qualify later browser sharing or restore PDF acceptance |
| [`duet-health`](/crates/duet-health/src/lib.rs), [`duet-notify`](/crates/duet-notify/src/lib.rs), [`duet-widget`](/crates/duet-widget/src/lib.rs) | Native health, notification and widget adapters; verify on installed clients under the owning outcomes |

The assistant-ui study in the corpus is a conversation-pattern reference. No
React library adoption is implied for the Rust/Dioxus client. Source snapshots
and the October 21 review describe their stated revisions; verify the current
owner before turning an observation into implementation work.

[Release](release.md) owns platform checks. A rendered screen or sample fixture
does not establish saved state, consent, background delivery or real-model
quality; verify the relevant journey through the backend and installed client.
