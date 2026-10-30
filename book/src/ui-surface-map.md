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
| [`ui`](/crates/ui/src/lib.rs) | Shared tokens (light and dark: one type scale, colour roles including data, space, shape and motion with springs), icons, buttons with their busy spinner, navigation links, list rows and row groups, section headers, screen headings, the top bar, the text field and the labelled field, selection and setting rows, the dialog and the sheet that drags away, the disclosure, the composer with its context chip, messages, the conversation viewport with Jump to new response, feedback ratings and toggle chips, the undo notice, the date separator, suggestions, receipts, the working mark, citations with their source preview, the source reading pane and the Sources control, cards with overlines that open their object or lead back to where it came from, plan steps, quotes and later context, measurement values, comparison bars, sparklines, trend charts and measurement tiles, the document page, notices, tags, footer actions, state pills, the status line, placeholder and Try again, read and command notices, and tabs; product-specific layout remains in each app |
| [`duet-app::roots`](/crates/duet-app/src/roots.rs) | Tab roots, current copy and navigation; the existing four-tab/Shared surface still needs removal of person-to-person sharing |
| [`duet-app::account`](/crates/duet-app/src/account.rs), [`duet-app`](/crates/duet-app/src/lib.rs) | Standalone account entry and conversation/connection screens; the old join gate is disabled, while the retained sharing toggle still conflicts with fixed consent |
| [`duet-page`](/crates/duet-page/src/lib.rs) | Existing doctor-page layout; retaining this crate does not qualify later browser sharing or restore PDF acceptance |
| [`duet-health`](/crates/duet-health/src/lib.rs), [`duet-notify`](/crates/duet-notify/src/lib.rs), [`duet-widget`](/crates/duet-widget/src/lib.rs) | Native health, notification and widget adapters; verify on installed clients under the owning outcomes |

## Seeing the shared components

[`ui-gallery`](/crates/ui-gallery/src/main.rs) renders every `ui` component with
the shared stylesheet, apart from either app: the type and colour roles first, then
each pattern from the October 21 mockups. The library is the ideal state the rebuilt
physician web app and Duet on iOS and Android build from; a mockup's raw values are
mapped onto the shared roles rather than added as another palette. Its examples are
fictional and scripted, and nothing in it is product behavior. Each app restyles some components, so check
a change in the app it serves as well. To build and serve it:

```sh
cargo build -p ui-gallery --target wasm32-unknown-unknown --features web
wasm-bindgen --target web --out-dir target/ui-gallery-web target/wasm32-unknown-unknown/debug/ui-gallery.wasm
cp crates/ui-gallery/web/index.html target/ui-gallery-web/
```

Then serve `target/ui-gallery-web` with any static file server. **Slow
motion** plays every animation at quarter speed, and **Replay entries**
remounts the sections so their entry motion runs again.

The assistant-ui study in the corpus is a conversation-pattern reference. No
React library adoption is implied for the Rust/Dioxus client. Source snapshots
and the October 21 review describe their stated revisions; verify the current
owner before turning an observation into implementation work.

[Release](release.md) owns platform checks. A rendered screen or sample fixture
does not establish saved state, consent, background delivery or real-model
quality; verify the relevant journey through the backend and installed client.
