# Duet product north star

Shipped Duet tabs are [`DuetRoot`](/crates/duet-app/src/lib.rs). Today, the library, and sharing are `today_root`, `library_root`, and `shared_root` in [`roots.rs`](/crates/duet-app/src/roots.rs). The physician shell is the other side of the Coda Agents boundary, in [`physician-app`](/crates/physician-app/src/lib.rs). SwiftUI remains the shipping iPhone app.

## Doctor page

[ENG-46](https://linear.app/coda-health/issue/ENG-46). The saved page is one approved snapshot: the page generation, the selected evidence, and the rendered PDF. Later edits do not change a publication. One publication is active per patient and expires (seven days unless a shorter bound is set). Republishing replaces it.

HTML, the PDF, and a SMART Health Link recheck authorization on every retrieval. Revocation stops later retrieval. Account deletion and starting fresh invalidate the publication. Withdrawing, deleting, or correcting a source blocks retrieval until the person approves a new snapshot.

Luna may propose wording. Opening or reviewing a proposal does not save it, and a stale proposal does not overwrite newer wording. Editing, preview, and sharing work when the model is off. The PDF is one US Letter page; overflow blocks export, not saving. The cover note stays off the PDF.

The SMART Health Link carries that same PDF, encrypted, as a FHIR DocumentReference attachment. It is not a second reading of the record. The link secret has at least 256 bits of entropy. File locations expire within an hour and never outlive the publication.

`harness` is shared execution mechanics. It does not hold clinical state, accounts, or a second run ledger. This is the workspace rule. It is not a production cutover.

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
