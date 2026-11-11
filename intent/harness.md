# The harness

The harness is the agent loop that Chords and Duet both run on. It isn't a product surface, and it stays light for the model.

The models are smart. The harness gives them the right tools, the right context at the right time and skills they can find, then gets out of the way.

- **References.** The harness is built from four reference harnesses: Codex, Hermes, OpenClaw and Pi. Each part below names the reference it follows and says where we differ.
- **Open questions** are in OPEN-QUESTIONS.md. Git keeps the history.

## Terms

Each term has one meaning in this spec.

- **Product:** Chords or Duet.
- **Job:** a kind of work a product runs, such as chat, a revision, a Check-in or Today.
- **Run:** one execution of accepted work, such as answering a message, preparing a revision or completing a Check-in. Resume and Try again start a new run linked to the preceding run.
- **Attempt:** one try at a run. After a crash or restart, a new attempt continues the same run.
- **Recipe:** what a product gives the harness for a run: the model settings, system prompt, skill catalog, tools and the job's assignment.
- **System prompt:** the instructions the model always has during a job.
- **Skill:** a package of guidance the model opens by name.
- **Window:** the context the model works in.
- **Fresh window:** a new window that starts from a short, exact starting point instead of the full history.
- **Working note:** text the model saves so it can continue its work later.
- **Handoff:** the working note the model saves before a fresh window.
- **Steer:** a new message that arrives during a run and changes its direction.
- **Held:** stopped until the person acts, for example with Try again or Resume. Nothing continues held work automatically.
- **Lead:** the main loop of a run.
- **Worker:** a separate loop the lead gives a piece of work to.
- **Lane:** a separate stream of work with its own capacity, such as interactive or background work.
- **Event:** a short record the harness sends the product when something happens.
- **Run record:** the saved history of a run.

## What it is for

- **One loop that doesn't depend on a product.** It lives in the shared `harness` crate, beside the shared `ui` and `coda-http` crates. There's no second engine and no shared runtime service.
- **What the harness does:**
  - carries the provider protocol and the tool protocol;
  - runs the cycle of model, then tools, then model;
  - manages scheduling and cancellation;
  - measures context;
  - sends events, in order.
- **What each product owns:** its protocol, UI, clinical types, accounts, consent, evidence, model settings and instructions.
- **Where they meet:** at tool calls and at events. The harness never appears in the feature models.
- **No dependencies on the products.** The harness doesn't depend on either product's domain, server, authentication, wire or storage crates.

## Principles

PRINCIPLES.md holds Coda's shared principles, and they apply here. The principles below are specific to the harness. The first five are its founding principles.

1. **Lightweight, with progressive disclosure.**
   - The model starts with what its job needs and opens more when it wants it: the full text of a skill, the next page of a tool result, an earlier message.
   - "Lightweight" means light for the model to carry, not few lines of code.
2. **Discoverable tools and skills.** The model sees which tools and skills exist, each with a short description, and finds and opens the rest itself.
3. **The right instructions and skills for each job.** Chat, background work and workers each load the instructions and skills their job needs. Context management is everything.
4. **Notes, not compaction.** Working notes and fresh windows manage context, as in Codex. Nothing is summarized, and earlier conversation stays readable.
5. **Caching.** Stable content comes first in each request and stays the same, so the provider can reuse it. The provider reports cache hits; the harness doesn't promise them.
6. **Never restrict the model's intelligence.**
   - Within the product's authority, the model decides what to investigate, which skills and tools to use, whether to delegate, and when it has enough evidence to answer.
   - No intent classifier or fixed sequence chooses the investigation.
   - Code never decides that valid evidence is too sparse, noisy or unimportant to reach the model.
   - The harness never reads model text as a clinical decision, and never judges clinical reasoning.
   - Guardrails are light and only add help: feedback, nudges and runaway backstops. They never reduce what the model can see or do.
7. **Recover, don't fail closed.**
   - When the model makes a mistake it can correct, the harness returns feedback in the same run. The mistake never counts as a failure.
   - Access, consent and patient-data checks are the exception. They fail closed at the tool: the tool refuses, and the run continues.
8. **One shared harness, with as few product differences as possible.**
   - Both products use the same loop, tools, rules and mechanics for every job: chat, Check-ins, Today, revisions, recovery and workers. No job gets a special path.
   - Products differ only in what the person sees and controls. Any other difference has to earn its place.
   - When this rule answers a question, apply it without asking.
9. **The harness is separate from the interface.** The harness sends events and never draws a screen, the way Codex keeps its core separate from its app server.
10. **Built from first principles and the references.**
    - Each mechanism comes from what Codex, Hermes, OpenClaw or Pi actually do. We build something new only where none of them covers the need, and say so.
    - We fix a problem at its owner with a structural change, not a patch around it.
    - We add complexity only for a real need.
11. **One owner for each fact, nothing stale.**
    - Each rule lives in one place.
    - Models are swappable. Product settings hold which model does each job, and the facts about that model.
    - Neither the harness nor the products copy information that goes stale, such as a price list.
12. **Stubs behind the tool.** Features that don't exist yet, and synthetic sources, sit behind the same tools the real ones will use. The model's choices and the evidence it gets stay real, so the harness doesn't change when the real feature arrives.

## How it works

### A run

- **One entry point** starts or continues execution from saved events. Automatic recovery continues the same run through another attempt. Resume and Try again start a new run linked to the preceding run. The final request, an interruption or a steer stays within the active run.
- **Only the harness interprets these events.** Products never assemble recovery.
- **No code reads prompt text as state.**
- **The recipe is frozen with the accepted work,** so a new message doesn't change it.

### The product contract

- **What the product does:**
  - checks authority and currency: is the work still allowed, and are its inputs still current?
  - saves events;
  - runs tools;
  - stores working notes;
  - decides what to publish and what to hold. Only the product publishes; a harness result is never a saved reply by itself;
  - sets its own commit and rollback policy.
- **What the product supplies before and during a run:** admitted tools, authority and currency checks, identity, permissions, evidence, persistence, effects, commit policy and recovery policy.
- **Finishing.** The model proposes a finish. The product accepts it only after its authority and currency checks. Otherwise it returns feedback, and the model continues in the same run.
- **Differences.** Each product keeps only a small policy for its genuine differences.

### Tools

- Tools are how the model reads and acts. They go with the job, not with the model.
- The product marks which of its tools are read-only.
- Before a call runs, it passes its authority check and is recorded as started.
- **Source labels.** The model sees short labels that map back to each source, and citations use those labels. Citations never depend on parsing model text.
- **Correctable errors** come back to the model under the original call ID:
  - bad arguments;
  - an unknown tool, with the names of the tools the job has, as Hermes does;
  - a call with an empty tool name;
  - refused access;
  - a product refusing a write or a finish;
  - an empty turn;
  - output cut off at the provider's limit.
- Successful results of other calls stay available.
- **A tool that fails unexpectedly returns an error result.** The model is told the call failed. If the call may have changed something, it is told the outcome is unknown and to check before repeating it. The run continues, as in Codex, Hermes, OpenClaw and Pi.
- **A call always has a usable ID.** A call with an empty ID, or an ID already used in the run, gets a new one and the turn runs, as OpenClaw and Hermes do.
- **Every request has matched calls and results.** Before a request, a call without a result gets a result that says it was interrupted and its outcome is unknown. A result without a call is left out. Earlier saved records don't change, and the saved request shows what was sent. Codex, OpenClaw and Hermes repair their history the same way.
- **Repeated failures.** When the same call keeps failing the same way, its result adds a note telling the model to reassess, as Hermes does. The note never stops the run. OpenClaw blocks the call at that point; we don't.

### Parallel calls

- Consecutive read-only calls run together, up to 8 at a time, following Hermes's ordered batching.
- Every call that isn't read-only is a barrier, and no call moves across it. Hermes also lets writes to different files run together; we don't.
- Each result is saved when it returns, and the model gets the results in call order, as in all four references.
- If one read fails, the other reads keep their results.

### Large results

Each tool result has an inline limit. A result under it goes to the model whole.

- **A larger result** comes back as a short preview: its size, its start and end, and how to read it. Hermes and Claude Code send a short preview with a path to the full output, and Codex keeps the start and end.
- **The product keeps the exact original under the call ID.** The model reads any range of it, or searches inside it, with `read_tool_result`, under current authority.
- **The bounds follow Hermes.** They're counted in characters, and the working window in characters is its tokens × 4.
  - One result shows whole up to 15% of the window, between 8,000 and 100,000 characters.
  - The results of one turn show whole up to 30% of the window, between 16,000 and 200,000 characters.
  - A preview is 1,500 characters in all, split between the start and the end.
  - The harness owns these bounds, and traces guide their tuning.
- **A requested read is a page, never a preview.** `read_tool_result` and the history read tool return one complete, contiguous page, at most the one-result bound, with the offset to continue. Hermes keeps its reads out of the per-result limit; we also keep them out of previews.
- **Before sending a turn's results,** if they're over their bound, the largest new results that aren't reads become previews first. If they're still over, the read pages among them get shorter, equally, and stay complete. Call order stays the same, and every call keeps its result. A page's length is fixed before its representation is saved.
- **The harness records which characters of each result reached the model.** The product decides whether that covers what its finish needs, such as the current chart revision and passages in Chords, or the authority and evidence a proposal needs in Duet.
- **Presentation size does not decide whether a write commits.** A receipt that can be retained may become a preview. The product owns write acceptance and rollback.
- **Nothing is done again** just to get its result back.
- **Once sent, each representation stays unchanged for the rest of that window** and is replayed exactly after recovery. A fresh window clears space instead of trimming old results.

### Skills

- **Each product supplies** its system prompt, its skills folder and its visibility rule.
- **One shared loader** lists a short index and opens a skill by name. The same visibility rule controls both listing and opening.
- **Reading a skill gives no permission.**
- **The skill package is frozen with the accepted work,** so recovery reads the same version.
- **Chat** starts small and finds what it needs.
- **Background work** starts from its assignment, gets all the product's tools, and can preload the skills its assignment always needs, as Hermes's scheduled jobs do.
- **Text the model reads** has no ticket numbers or porting notes.
- **Each piece of guidance lives in one place:** the system prompt or a skill, not both.
- **An unknown skill name returns the names of the skills the job has,** as Hermes does.
- **Skills improve from traces.**
  - Traces show which skills load, what they cost in context, and what goes wrong after a load.
  - A skill that runs long gets shorter, with details moved into linked references.
  - What nearly every run needs moves into the system prompt.
  - A failing tool gets fixed at the tool, not patched in a skill.

### Context

- The model pulls records through source tools.
- Within a window, the provider adapter keeps the model's reasoning between calls and carries what a tool exchange needs to continue.
- **Measurement:** the provider's reported usage, plus an estimate for everything added since. Tool calls and results count at 3 characters per token, as OpenClaw counts JSON, and other text at 4.
- **Working window:** each job has one, set in product settings and kept separate from the model's input limit, as OpenClaw keeps a working limit separate from the model's maximum.

### Working notes and fresh windows

- **The note is a running checkpoint.** The model saves a working note as its work moves forward, and can ask for a fresh window, as Codex's notes and new-context tools allow. Each save is a complete note, and the newest one is the checkpoint.
- **The model knows this from the start.** Wherever the note tools are offered, the harness's instructions say that long work can outlast one window, that the window is then lost, that only notes and the history tools carry the work across, and what a note should hold. Codex gives the same guidance up front.
- **Near the end of usable input**, at 90% of it, the harness asks the model to answer if the work is done, or to save its note and ask for a fresh window if more remains. The request gives the tokens left and says the next window will not hold this conversation. If the model keeps working, the next request says to stop and save now. Both follow Codex's wording, and both are instructions: every call the model makes still runs. Usable input is the smaller of the model's input limit and the working window minus the reply reserve. Codex also keeps its 90% point below its usable input.
- **Room for the reply.** The harness keeps 16,384 tokens of the window free for the reply, as Pi, OpenClaw and Codex do. This is room in the window, not a limit on output.
- **Every request has the same tools.**
- **A turn's results fit the room left in the window.** What doesn't fit is offloaded like any large result, so the request near the end can always be sent. Result sizes follow Hermes, which acts earlier in the window; this rule joins them to Codex's late request.
- **A fresh window contains:**
  - the assignment;
  - the original request and accepted steering;
  - pending work and results;
  - a short message that this is a fresh window for the same work and that the history tools list earlier results, as Codex tells the model about its window change;
  - the exact text of the newest note the product confirmed saved, and references to earlier notes in the order saved.
- **If the window has no room left, the fresh window opens anyway,** with whatever note exists. Without a note, the model rebuilds from sources and the history tools. The harness never summarizes.
- **At the start of a run**, the same handoff point applies. A conversation already past it opens like a fresh window instead of sending the full history.
- **At the start of a run, the model is told when the conversation has earlier notes,** with their references in the order saved. It reads one if the message continues that work. Codex puts its note hint into the first context of a thread.
- **Context pressure never causes a failure,** including a provider refusing a request as too long. The result is a fresh window.
- **If the provider refuses the fresh window too,** the harness tries once more. Pending results that aren't reads become previews, and pending read pages shorten to at most 8,000 characters, still respecting the turn bound, and stay complete. If it's still refused, the run is held at a limit. A final request is never retried after a refusal.
- **A note belongs to the conversation or the piece of work it continues,** and the product stores it.
- **A note can point to a source.** When the model needs current data, it reads the source again.

### Earlier conversation

Three history tools follow Codex's design. The harness defines them once, and each product supplies its records and permission checks.

- **List** earlier messages and this work's tool results, newest or oldest first, with long items shortened. Tool results can be filtered by tool name and window, as Codex's history lists tool items. A tool-name filter that matches nothing returns the tool names that have results.
- **Read** one full message, in parts. A tool result is read with `read_tool_result`.
- **Search** messages and tool results by text.

Rules:

- Shortening never changes the saved original.
- A follow-up can depend only on a message the model received in full, and that stays true after a restart.
- Each product builds the history from its own conversation when the work starts.
- Tool results come from the run record of the current work.
- A list or search sees the tool results saved before its turn started, so its answer never depends on which call finished first.

### Workers

The lead can give parts of the work to workers, following Codex's design.

- **When to delegate.** The spawn tool's description says when delegation helps. Each job's instructions say whether that job can delegate.
- **What a worker starts with.** Its assignment and the context the lead selects, never the lead's conversation. The worker can open more evidence it's allowed to read.
- **How a worker runs.** The same loop, with the product's worker settings, instructions made from its assignment, and only the tools the product allows.
- **Limits.**
  - Workers don't start workers, and up to 3 run at a time, as in Codex.
  - Workers share the run's one count of requests and tokens. No part of the budget is reserved for a single worker.
  - The count is saved in one place, so neither a later attempt nor a worker can reset it.
- **Results.**
  - Each result is saved when the worker finishes and goes only to the parent that asked for it.
  - Each result is marked as a report to check, not as instructions, as Hermes and OpenClaw do.
  - If one worker fails, the others keep their results.
  - Workers never finish the run. Only the lead proposes the finish.
- **Durability.** Workers are part of the saved work. A later attempt reconnects to its workers instead of starting duplicates. Stop holds unfinished workers, and Resume uses the results of finished ones.

### Lanes

- Interactive and background work run in separate lanes, each with its own capacity, as in OpenClaw.
- A burst of scheduled work never blocks a conversation or uses the quota an answer needs.
- One account can run a chat and background work at the same time.

### The run record

- **What it keeps:** the requests, the exact evidence delivered, the recipe and model, usage, window changes and their timing, and the outcome. It stays available to inspect after a restart.
- **Each part of a request is saved once, across runs too.** The parts are the frozen recipe, the conversation and tool items, and what was added at send time, such as a warning, the handoff text, the final-request text and the previews the model got. A request points to these parts, the way Codex rebuilds its requests from saved items. A run points to the product's messages by their ID instead of copying them, so a message deleted for privacy is gone from every run record that pointed to it.
- **A retry of the same request sends exactly what was sent before.** New steering or changed sources make a new request.
- **It's the one authoritative run record.** No second run record exists anywhere, including in cloud scheduling or delivery. Every tool built on top of it uses it as the source of truth.

### Simulation

- The loop runs the same way with simulated triggers.
- A simulation supplies the triggers, events and actors.
- A stub behind a product tool or the provider can return source data or a failure.
- Nothing reads markers from the person's message, and nothing scripts the model's calls. The model chooses its calls and draws its own conclusions.

## Guarantees

The products depend on these promises.

- **Durable runs.**
  - Run records and control state survive a restart.
  - Scheduled and resumed work runs on hosted infrastructure and never depends on a developer's computer.
  - A retry limit never drops the work, and a skipped run never counts as done.
- **No duplicate effects.**
  - Each change, such as a send or a saved routine, keeps one identity across retries.
  - On recovery, finished tool results are reused by call ID, as Codex does.
  - A call that started but has no recorded outcome is never run again by the harness. The harness checks the product's saved result for that call without running it. If no result is known, the next request names the call and says its outcome is unknown, as OpenClaw and Hermes do. The model is told, checks current state, and decides whether to repeat a call that may have had an effect.
  - Saved side effects decide whether a step can run again, not whether text was shown, as in OpenClaw.
  - Once output is visible, the transport never retries.
- **Drafts are replaced.** After an interruption or a steer, the harness discards an unpublished draft and runs the step again without it, even if its text was shown live. A published partial answer is part of the conversation, not a draft.
- **Steering.**
  - New input, whether a steer now or a queued message later, is a saved item with its own ID. The harness puts it into a request exactly once, as Codex's input queue does.
  - A steer cancels the model request in progress. Tool calls already executing are allowed to finish, and their results are retained; calls that have not started are not executed. The harness discards unpublished draft text. The next request sends the completed calls with their retained results, each call that did not start marked as not run, and the new input, as Codex and Pi answer every call they issued. The model decides which calls to issue again. Finished tool work is never repeated.
  - One answer covers all the messages that steered it, and late output never replaces newer direction.
  - The loop supports both products: in Chords, Stop holds the work until Resume; in Duet, a new message steers the reply in progress.
- **Honest endings.** A run ends exactly once, with one outcome:
  - completed;
  - held, with its reason: stopped by the person, at a limit, or blocked by the provider;
  - failed, when automatic recovery is used up;
  - cancelled, when the work is withdrawn or replaced.

  The product records the outcome when it accepts a finish or ends the attempt. A run that reaches a limit is never recorded as a failure. Resume and Try again start a new run that continues the same work, as Codex continues a thread with a new turn. The hold reason decides what the person sees: Resume after Stop, and the partial answer with Try again after a limit. An attempt can end interrupted while its run continues.
- **Isolation.** One account's or practice's work never reaches another's context, tools or results.
- **Rate limits.**
  - Each product has one saved rate-limit gate for its provider quota.
  - If the provider refuses or drops a request before it produced anything, the request waits, follows the provider's retry hint and tries again with backoff. A completed response resets the count. After five retries in a row without one, the attempt fails, as in Codex. A retry hint longer than 60 seconds ends the attempt the same way; OpenClaw and Pi stop at that delay.
  - When credits run out, the run stops at once with a plain message.
  - The gate exists before any hosted or paid run.
- **Fresh windows never repeat an effect.**
- **Accounting.**
  - Each model call is recorded with its token usage against its person and purpose, including failed attempts and background work.
  - Unknown usage is recorded as unknown, never as zero.
  - Cost comes from the provider's usage records, not from prices kept in the product.

## Events

The harness reports what happens as events, in order.

- **One neutral vocabulary.** Both products get the same events, the way Pi's agent events and Codex's turn items are each one vocabulary for all their clients.
- **Saved by the product.** Each product must save the events in its own durable store. This isn't best-effort telemetry, and it isn't a new shared journal.
- **Live text is the exception.** Reply text is live only; see the reply-text rules below.
- **Saves come before dependent work.** When the next step depends on a saved event, the harness waits for the product to confirm the save.
- **Status comes only from events.** Products build live status (current step, purpose, trail) from events, never from model text. The harness never invents progress.

| Event | What it carries |
|---|---|
| Run start | That a run started, and the run it continues, if any. |
| Model request | The recipe and model of one request. |
| Tool call | The call and the exact evidence it read. A product's chart meets the harness here. |
| Reply text | The reply text while the model writes it. Live only. |
| Worker result | One worker's result, for its parent. |
| Finish proposed | The finish the model proposes. |
| Window change | A move to a fresh window, with its timing and its reason: the model asked, near the end, full or refused, as Pi records its compaction reason. |
| Run outcome | How the run ended: completed, held with its reason, failed or cancelled. |
| Usage | The tokens of one model call, with its person and purpose. |

Reply text:

- The products never save reply text piece by piece.
- The run record saves the reply once, when the message ends, as Codex, Hermes, Pi and OpenClaw do.
- If a product shows text while the model writes, the text reaches the screen over a live connection, the way Codex's app server sends each piece as a notification.

## Boundaries

### The products own these

- Accepted input, schedules, conversations and action results.
- Memory. The products never share clinical memory.
- Evals.
- Chart search and ranking.
- Clinical rules, such as citation rules, privacy deletion and what a held run means. These sit behind the product's tools and its finish.

### Not in the harness, by decision

- Fallback models or providers.
- Hooks or plugins. The product contract is the only place product code meets a tool call, and connectors come in as tools.
- Goals that run work again and again. Each product decides when its work continues.
- A model that writes or edits its own skills.

### Later

- A formal model of the harness, only when a promise such as steering, restarts or no duplicate effects needs proof.
- Programmatic tool calling.
- Image and file tool outputs, with the chart work.

## Engineering decisions

- **Model provider.**
  - Production model calls go through the OpenAI API, under its BAA.
  - For local tests on synthetic data, a developer can opt in with their own ChatGPT plan, signing in the way Codex does. The plan is a model choice on the same product path, with no other harness inside ours.
  - Caching on the plan doesn't measure API caching.
  - Staging has its own real model path, with its own run budgets.
- **Model settings.**
  - For each job, each product sets the model, its reasoning effort, the working window and the model's input limit.
  - There are no hidden defaults: if a setting is missing, startup stops.
  - The working window is 272K tokens, below the point where long inputs cost double.
- **Budgets.** Limits are runaway backstops and never limit an investigation. None of the references stops a run on cost in dollars.
  - **The backstops.** A run has two: 100 model requests and a token count. A run has no time limit: Codex and Pi have none, Hermes's is off by default, and OpenClaw's is 48 hours. Tool-result presentation is bounded as described in [Large results](#large-results).
  - **Tokens** are output plus uncached input, the way Codex counts its budget. Cached input is cheap and doesn't count.
  - **Owner.** The harness keeps the shared backstops in one place.
  - **Across attempts.** The counts continue across a run's attempts. Resume and Try again start a new run and reset them, and so does a new message on held or failed work.
  - **Provider retries** don't count toward the request limit. Codex and OpenClaw also keep retries separate.
  - **Near a limit.** The model gets one notice, added to the newest tool result so the cached prefix stays the same. It tells the model to stop new work and answer from what it has, as Hermes's run-budget notice does.
  - **At the limit.** The final request keeps the full tool list and asks for no tool calls. The harness takes only the text and runs nothing. Hermes ends a run this way on its main path, keeping the tools so the cached prefix stays the same.
  - **After the limit.** The run is held.
    - Chat keeps an honest partial answer, which Try again continues.
    - A draft revision, Check-in or Today run keeps its handoff in the run record, and only its explicit finish publishes.
  - **Values** are adjusted from traces.
- **Provider.** Every product streams from the provider.
  - The completed output item is the only source of a turn's content. Streamed pieces are a live preview, and the harness ignores events it doesn't use, as Codex, Pi, OpenClaw and Hermes do. Output items and content parts it doesn't know are skipped the same way.
  - A refusal from the model is its answer and is shown as text, as Hermes, OpenClaw and Pi do.
  - If a stream goes silent, the harness tries again after the same idle timeout Codex uses.
  - Provider problems before output that another try can fix are retried in the run and don't count as a failed attempt.
  - A reply the provider filtered, or a provider error that another try can't fix, holds the run as blocked by the provider, with its reason. The same request is not sent again. All four references stop and show the error.
  - If saving a paid answer fails, the harness keeps the answer and uses it in the retry instead of calling the model again.
  - If a source is down, the work stays resumable.
- **Recovery.**
  - Automatic recovery stops after 5 failed attempts, as Pi stops at its retry maximum. The work stays resumable through Try again.
  - A steer or a correctable tool error doesn't count as a failed attempt.
  - The harness decides what counts as a failed attempt for both products. Requested cancellation means interrupted, even when another error arrives with it. After a run returns an error, hosting settles the attempt using the harness classification.
- **Caching.**
  - Each request runs in this order: system prompt, tools, the job's start, then the conversation, which only grows.
  - The cache key is per scope, made only from scope IDs and never from message text, as Codex keys by session.
  - We measure how narrow each scope needs to be.
- **Storage.**
  - The cutover is clean, with no compatibility with earlier saved shapes or with coda-agents and coda-app.
  - What must survive is the conversation, its evidence and the ability to resume interrupted work.
  - Defaults that honestly mean "unknown" stay.
- **Proof.**
  - The proof that matters is dogfooding the real model through the ChatGPT plan, on synthetic patients.
  - It reports tokens and cached tokens for each request, window changes, the tools and skills chosen, and what went wrong.
  - Scripted tests cover what dogfooding can't cause.
