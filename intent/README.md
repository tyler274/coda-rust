# Coda intent specs

These specs say what we're building and why. They are the source of truth for what the products do: the journey models and the code follow them, and Linear tracks the work against them. They describe the product as we intend it today, and git keeps the history.

**Read in this order.**

1. `PRINCIPLES.md`: the principles that govern every spec, chart and proof, including the shared UI principles (U1–U9).
2. The specs, one per product:
   - `physician.md`: Coda Agents, the physician web app.
   - `duet.md`: Duet, the patient app.
   - `connected-care.md`: how a Duet patient and a Coda practice connect.
   - `harness.md`: the shared agent loop. It stays outside product behavior and meets the products only through tools and events.
3. `DECISIONS.md`: the open questions, split into product decisions and engineering decisions. The specs themselves hold only what's decided.

Each spec states the product's behavior first and ends with its engineering decisions. Product decisions and engineering decisions are the two kinds of choice; nothing here is owned by a person.

**Models.** Each journey's model cites spec lines by ID, such as `physician A1.3`, and TLC checks that it keeps the journey's rules. Code is judged against the model, never the reverse. Sign-in is the first journey; see [verification](../verification/README.md).

**Changing them.** Edit the line so it says what's true now. When an open question is answered, the answer goes into every spec it touches and the question leaves DECISIONS.md. A Linear ticket, doc or website line that disagrees with a spec gets corrected at its source.
