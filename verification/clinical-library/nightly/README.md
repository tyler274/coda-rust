# Clinical library nightly

How the clinical library refreshes from its sources every night, and what happens when part of a night fails. [physician.md](../../../intent/physician.md#engineering-decisions) links here. There is no model yet; the [verify skill](../../../.agents/skills/coda-verify-clinical-nightly/SKILL.md) names the tests that hold each rule.

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
- A person resolving an article that needs a person makes it due again.
- The source keeps the article's last good content while it waits.

A night:

- A failure sets aside only the article it touches, and the other articles continue.
- A source that can't be prepared holds back only its own new content.
- Only an integrity failure of the whole library holds back what every source publishes.
- A night ends in exactly one outcome: succeeded, deferred or failed.
- It succeeds when nothing was due, or when it published everything due with nothing set aside.
- It's deferred when it didn't finish what was due for a reason other than a fault in its own work: another night held the slot, the slot's holder was unreadable, no build capacity was free all night, it published while setting articles aside, or it set aside everything due and so moved nothing.
- It fails when its own work failed, and records the failure code.
- A failed night's work resumes the next night from its saved progress.
- The history of nights and resolutions is kept as it happened and never rewritten.

The slot:

- A night takes the slot when it's free, or when the night holding it has ended.
- A night that ended without recording an outcome is recorded as failed with the code `ended_without_outcome`.
- While the holder is alive, or unreadable, the next night waits and records why.
- Each new holder of the slot gets a higher token, and every write a night makes checks its token is still current.

PubMed's status:

- An article's status is NLM's words: its publication types and its comments-and-corrections links, kept as NLM sent them, with the record they came from exactly as NLM sent it.
- A book record, such as a chapter, is read the same way; NLM's format gives books publication types but no comments-and-corrections links.
- An article is qualified by being on the night's list, and its key is a PMID.
- A night checks every article on its list that is new, waiting to retry, or admitted; an admitted article's check re-confirms its status.
- A record NLM sent unchanged re-confirms the stored status, and a changed one is new content that replaces it.
- A record the reader can't read, including one without the publication types NLM's format requires, is our own defect, and the article keeps its last good status.
- No record from NLM is an outside failure, never a sign the article wasn't retracted.
- An admitted article whose check fails keeps its status and state; it goes unconfirmed until a later night, which the freshness report shows.
- A request NCBI refuses outright, such as for an invalid key, fails the night, and the key never appears in what it reports.
- Requests stay within NCBI's rate, and back off when NCBI throttles or fails on its side.

Publishing:

- A version is built, then checked, then activated, in that order.
- Activation switches the version search serves in one step.
- A version that fails its checks never becomes active, and the current one keeps serving.

Telling people:

- One alert reads the records: something needs a person, or the library should have updated and didn't.
- Reminders repeat daily until the records show a resolution.

## Rules

1. Only a person moves an article out of needs a person, and only new content or a new reader moves one out of waiting for a change.
2. A night tries only articles that are waiting to retry.
3. Only a night that had nothing due, or published everything due with nothing set aside, is a success.
4. Automatic attempts stop when, since the last fresh start, the same failure code was recorded on two nights.
5. Automatic attempts stop when, since the last fresh start, three nights in a row failed or moved nothing, where a night that only waited for a live holder neither counts nor breaks the streak and a night that published while setting articles aside breaks it.
6. A night takes the slot from another only when the platform says that night ended.
7. A night whose holder has been unreadable two nights in a row tells a person.
8. A night that no longer holds the slot changes nothing the night holding it wrote.
9. At most one version is active, and only a version that passed its checks.

## Outside this spec

Which sources the library collects, its freshness targets and how search reads it follow [physician.md](../../../intent/physician.md#engineering-decisions). Where alerts go, how often reminders repeat, and alerting on the same failure code repeating across fresh starts belong to the alerts that read these records.
