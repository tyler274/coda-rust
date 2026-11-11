# Clinical library nightly

How the clinical library refreshes from its sources every night, and what happens when part of a night fails. [physician.md](../../../intent/physician.md#scope) links here. There is no model yet; the [verify skill](../../../.agents/skills/coda-verify-clinical-nightly/SKILL.md) names the tests that hold each rule.

## Words

- **Source:** where the library collects from: PubMed's retraction and correction status, PMC, DailyMed, MMWR or USPSTF.
- **Article:** one item a source has qualified for the library, such as a PMC article or a DailyMed label.
- **Night:** one run of the refresh, from taking the slot to recording its outcome.
- **The slot:** what a night holds while it runs. One night holds it at a time.
- **Ended:** the platform says the night holding the slot finished, crashed or was cancelled.
- **Unreadable:** the platform can't say whether the night holding the slot is alive or ended.
- **Outside failure:** a failure that may not repeat, such as a timeout or a server error.
- **Failure code:** a short name for a failure's cause, the same each time that cause happens.
- **Moved nothing:** a night that had work due and finished none of it.
- **Set aside:** an article a night couldn't admit, kept with its reason while the rest publish.
- **Version:** one built copy of the search index.
- **A fresh start:** a night that succeeded, or a person recording that they resolved what stopped the nightly.
- **Due tonight:** an article that was waiting to retry when the night took the slot, one the night newly qualified, or an admitted one whose source sent new content tonight.
- **Awaiting a version:** admitted content of a source that search reads through versions, which no activated version has published yet.

## Journey

An article's states:

- Every article is in exactly one of four states: admitted, waiting to retry, waiting for a change, or needs a person.
- A source qualifies an article deliberately, and only a qualified article takes moves.
- A new article is waiting to retry, so the next night tries it.
- An admitted article stays admitted until its source publishes new content, which makes it due again.
- An outside failure leaves the article waiting to retry, and the third in a row means it needs a person.
- An article whose input would fail the same way again waits for a change: no readable text, our own defect, or a reader since replaced.
- One our own code couldn't admit tells a person at once.
- New content from its source, or a new reader, makes a waiting article due again.
- A person resolving an article that needs a person makes it due again, from the next night on.
- The source keeps the article's last good content while it waits.

A night:

- A failure sets aside only the article it touches, and the other articles continue.
- A source that can't be prepared holds back only its own new content.
- Only an integrity failure of the whole library holds back what every source publishes.
- Every source runs its night through one runner, not a loop of its own.
- A night runs only when the stop rule allows it. One it keeps from running is deferred as stopped, and records which rule stopped it.
- A night ends in exactly one outcome: succeeded, deferred or failed.
- The outcome is counted from what the night recorded doing with each article due tonight, never from the night's own word.
- It succeeds when nothing was due, or when it admitted everything due, set nothing aside, and left no content awaiting a version, whichever night admitted it. An activation publishes only what was admitted before it.
- It's deferred when it didn't finish what was due for a reason other than a fault in its own work: another night held the slot, the slot's holder was unreadable, the nightly had stopped, no build capacity was free all night, it published while setting articles aside, or it set aside everything due and so moved nothing.
- It fails when its own work failed, and records the failure code and the words that say what failed. A night that says it finished but left an article due tonight neither admitted nor set aside, or left content awaiting a version without activating one, fails with `due_work_left_undone`.
- A failed night's work resumes the next night from its saved progress: what it admitted stays admitted, and what it left is due again.
- The history of nights and resolutions is kept as it happened and never rewritten.

The slot:

- A night takes the slot when it's free, or when the night holding it has ended.
- Whether the holder ended is what the platform running the nightly says; a platform that can't tell says it's unreadable.
- A night that ended without recording an outcome is recorded as failed with the code `ended_without_outcome`.
- While the holder is alive, or unreadable, the next night waits and records why.
- Each new holder of the slot gets a higher token, and every write a night makes checks its token is still current.

PubMed's status:

- An article's status is NLM's words: its publication types and its comments-and-corrections links, kept as NLM sent them, with the record they came from exactly as NLM sent it.
- A book record, such as a chapter, is read the same way; NLM's format gives books publication types but no comments-and-corrections links.
- An article is qualified by being on the night's list, and its key is a PMID.
- A night checks every PubMed article due tonight, whether or not its list still names it, and re-confirms the status of every admitted article on its list. An article a person made due after the night took the slot waits for the next night.
- A re-confirmation NLM answered counts as work the night did. One that went unanswered leaves the article unconfirmed and doesn't count against the night.
- A record NLM sent unchanged re-confirms the stored status, and a changed one is new content that replaces it.
- A record the reader can't read, including one without the publication types NLM's format requires, is our own defect, and the article keeps its last good status.
- No record from NLM is an outside failure, never a sign the article wasn't retracted.
- An admitted article whose check fails keeps its status and state; it goes unconfirmed until a later night, which the freshness report shows.
- A request NCBI refuses outright, such as for an invalid key, fails the night, and the key never appears in what it reports.
- Requests overlap, so a night's length is set by NCBI's rate rather than its latency, and one limiter shared by every request in flight spaces their starts to that rate, counting each interval from when a request actually starts.
- Requests stay within NCBI's rate, and back off when NCBI throttles or fails on its side.

Publishing:

- A version is built, then checked, then activated, in that order.
- Each night builds its candidate into storage only it uses, and each file is written once and never replaced, so only activation needs the slot.
- Activation publishes the content of the night's sources that awaited a version.
- A night finishes only with an activation it made itself, of the version still active.
- Activation switches the version search serves in one step.
- A version that fails its checks never becomes active, and the current one keeps serving.

Telling people:

- One alert reads the records: something needs a person, or the library should have updated and didn't.
- Something needs a person when the nightly stopped, when the slot's holder was unreadable two nights in a row, or when an article needs a person or waits on our own defect.
- The library should have updated and didn't when three nights in a row neither succeeded nor published part of what was due, including nights that waited for a live holder.
- Reminders repeat daily until the records show a resolution.

## Rules

1. Only a person moves an article out of needs a person, and only new content or a new reader moves one out of waiting for a change.
2. A night tries only articles that are waiting to retry.
3. Only a night that had nothing due, or published everything due with nothing set aside, is a success, as counted from what it recorded doing.
4. Automatic attempts stop when, since the last fresh start, the same failure code was recorded on two nights.
5. Automatic attempts stop when, since the last fresh start, three nights in a row failed or moved nothing, where a night that only waited for a live holder, or that the stop rule kept from running, neither counts nor breaks the streak, and a night that published while setting articles aside breaks it.
6. A night takes the slot from another only when the platform says that night ended.
7. A night whose holder has been unreadable two nights in a row tells a person.
8. A night that no longer holds the slot changes nothing the night holding it wrote.
9. At most one version is active, and only a version that passed its checks.

## Outside this spec

Which sources the library collects, its freshness targets and how search reads it follow [physician.md](../../../intent/physician.md#scope). Where alerts go, how often reminders repeat, and alerting on the same failure code repeating across fresh starts belong to the alerts that read these records.
