# Coda intent specs

**What these are.** Dev's product intent, written from primary sources so he can correct it. They hold intent only, not designs or code.

**Read in this order.**
1. `PRINCIPLES.md`: Dev's principles, including the shared UI principles (U1–U9). They govern every spec, chart and proof below them.
2. The specs, one per product:
   - `physician.md`: Coda Agents, the physician web app.
   - `duet.md`: the patient app.
   - `connected-care.md`: how a Duet patient and a Coda practice connect.
   - `harness.md`: the shared agent loop. It stays outside product behavior and meets the products only at events: tools are events, and needs are preconditions.
3. `DECISIONS.md`: the one list of open questions (for Dev, for Tyler, studies, the UI program, after the MVP, before production) and every settled conflict. The specs carry no open questions or conflicts of their own.

**Supporting files.**
- `HISTORY.md`: where Dev replaced an earlier call, and what the principles removed. Only for understanding why a spec line says what it says.
- Linear, doc and site lines that lag these specs are corrected at their source, with Dev's approval of the exact text; the working list stays outside the repository.

**How they were built.** Five primary sources: Dev's own messages in Claude and Codex sessions (`dev_words.jsonl`); his answers to agents' multiple-choice questions (`asks.jsonl`, where a chosen option is binding); Linear projects and tickets (`linear-raw.md`); the original coda-agents and coda-app markdown docs; and codahealth.ai (`site-*.txt`). Agents extracted claims with verbatim quotes. A script (`verify.py`) checks every quote character for character against its source. Writers drafted each spec from the checked evidence. An adversarial reviewer checked it against the raw sources, and a final cross-spec review fixed wording drift and built DECISIONS.md. `check-spec.py` confirms that every bullet cites a pointer that resolves. The source corpus (Dev's transcripts) and both scripts stay outside the repository, because the transcripts are private.

**Authority rules.**
1. `PRINCIPLES.md` comes first, then Dev's later explicit decisions, then his feature choices and approvals, then Linear, then old docs and the site.
2. Any rule that adds machinery a principle rules out loses to the principle, even one Dev approved in passing or "for now". Machinery here means a cap, gate, pause, dialog, toggle, fixed count, or judgment made in code. The spec follows the principle and HISTORY.md records the change; Dev isn't asked.
3. When Dev explicitly changed his mind, the newer position wins and HISTORY.md keeps the old one.
4. Only questions the principles don't settle go to DECISIONS.md: scope, consent and data choices, and real product trade-offs. No agent resolves those.
5. Docs `(doc)` and the site `(site)` fill detail only where Dev and Linear are silent. They are never Dev's word.
6. Agent-written artifacts are never sources: verification graphs, `decisions.md`, memory notes and agent summaries.
7. A missing step is written as `GAP:`, never invented.

**Charts and proofs: the intended chain, not yet in place.** Principles percolate down: specs cite principles, charts cite spec lines, and proofs derive from charts. Today the ENG-185 behavior graphs (physician, Duet, connected care) do not cite these specs. They predate them and cite dates and the agent-written per-graph `decisions.md`, so they lag the Oct 28 decisions. The next step is to give spec lines stable IDs and re-derive the charts per journey from them, so every state, transition and guard cites a spec line. Then the per-graph `decisions.md` retire, and the Verus proofs follow the charts. The harness keeps its own model only when a promise is worth proving, such as steering, restarts or no duplicate effects, and joins the graphs only through the events in `harness.md` section 5. Code is judged against the graphs, never the reverse.

**How to correct them.** Tell an agent the correction in plain words, or edit the line yourself. Your correction is a new dated source: the line is updated to cite it, and the old position moves to HISTORY.md. To answer an open question, reply by ID ("O2: …"). The agent updates every spec the item touches, removes the item from DECISIONS.md and reruns both checks.
