# Duet product north star

Shipped Duet tabs are [`DuetRoot`](/crates/duet-app/src/lib.rs). Today, the library, and sharing are `today_root`, `library_root`, and `shared_root` in [`roots.rs`](/crates/duet-app/src/roots.rs). The physician shell is the other side of the Coda Agents boundary, in [`physician-app`](/crates/physician-app/src/lib.rs). SwiftUI remains the shipping iPhone app.

## Approved Rust doctor-page delivery delta

[ENG-46](https://linear.app/coda-health/issue/ENG-46) authorizes durable saved
pages and two account-free delivery formats: ordinary web/HTML with the exact
PDF, and SMART Health Links v1. Publication pins an explicitly approved page
generation, exact selected evidence and rendered PDF; later edits never change
that snapshot. One active publication per patient has mandatory bounded expiry
(seven days by default); explicit republishing revokes/replaces the old one.
Every HTML, PDF, SHL manifest and SHL file retrieval rechecks authorization.
Revocation stops future retrieval, but cannot erase already downloaded copies.

Start fresh and account deletion invalidate publication. Source withdrawal,
deletion, or correction requiring review blocks retrieval until reviewed
republication, never silently rewriting the snapshot. SHL carries the same PDF
as an embedded FHIR R4 DocumentReference attachment, encrypted using compact
JWE `dir`/`A256GCM`; it is not a second clinical interpretation. Manifest
capabilities have at least 256 bits of entropy. File locations expire within
one hour and never outlive publication. No unimplemented dynamic/passcode flags
or EHR compatibility claim accompanies standards conformance.

Luna proposes; opening/reviewing is not saving. Only explicit patient approval
changes the page, and stale proposals cannot overwrite newer patient wording.
Manual editing, preview, and eligible sharing do not depend on AI availability
or permission. PDF and preview share measured Unicode layout, the full selected
evidence and one US Letter page at 10.5/10/9.5-point steps. Overflow blocks
export/publication, not saving; the cover note stays outside the PDF.

The approved neutral harness is shared execution mechanics, not shared clinical
state, accounts, servers, instructions, effect authority, recovery policy or a
second run ledger. Physician and Duet adapters retain those decisions. Design,
implementation and interruption validation remain separate from production,
paid calls, live-model quality and installed-device qualification.

**Canonical owner follow-up:** the `coda-app` North Star owner must fold this
approved delta into the product repository. This workspace record neither edits
the read-only submodule nor claims a shipping cutover. No production deploy,
EHR write-back, physician reply, or premium plan delivery is authorized here.

## Canonical product references

- <a href="/submodules/coda-app/docs/north-star%2Emd#duet-product-north-star">Duet product north star</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#north-star">North star</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#product-structure">Product structure</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#background-work-and-continuity">Background work and continuity</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#conversation-memory-and-follow-through">Conversation, memory, and follow-through</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#requested-follow-through-and-unsolicited-reviews">Requested follow-through and unsolicited reviews</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#identity-and-setup">Identity and setup</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#today">Today</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#living-portrait">Living portrait</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#a-bounded-daily-and-weekly-composition-or-silence">A bounded daily and weekly composition or silence</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#health-information-and-browsing">Health information and browsing</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#insight-evidence-and-sufficiency">Insight evidence and sufficiency</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#insight-automation">Insight automation</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#effective-source-resolution">Effective source resolution</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#connector-delivery-program">Connector delivery program</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#for-my-doctor">For my doctor</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#the-page">The page</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#lunas-part">Luna's part</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#exact-content-and-sharing">Exact content and sharing</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#editing-availability-and-clearing">Editing, availability, and clearing</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#retired-moments">Retired Moments</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#you">You</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#shared">Shared</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#backend-and-ownership">Backend and ownership</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#model-boundary">Model boundary</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#consumer-health-boundary">Consumer-health boundary</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#privacy-consent-and-learning">Privacy, consent, and learning</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#coda-agents-boundary">Coda Agents boundary</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#notifications">Notifications</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#source-access-export-and-deletion">Source access, export, and deletion</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#explicit-non-goals">Explicit non-goals</a>
- <a href="/submodules/coda-app/docs/north-star%2Emd#release-invariant">Release invariant</a>
