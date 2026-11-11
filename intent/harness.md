# The harness

The harness is the agent loop that Chords and Duet both run on. It isn't a product surface, and it stays light for the model.

The models are smart. The harness gives them the right tools, the right context at the right time and skills they can find, then gets out of the way.

- **References.** The harness is built from four reference harnesses: Codex, Hermes, OpenClaw and Pi. Each part below names the reference it follows and says where we differ.
- **Open questions** are in DECISIONS.md. Git keeps the history.

## Terms

Each term has one meaning in this spec.

- **Product:** Chords or Duet.
- **Job:** a kind of work a product runs, such as chat, a revision, a Check-in or Today.
- **Run:** one piece of accepted work, such as the answer to one message, one revision or one Check-in.
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

- **One entry point** starts or continues a run from its saved events: a fresh start, a resume after a crash, Try again, the final request, an interruption or a steer.
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
  - an unknown tool;
  - refused access;
  - a result that's too large;
  - a product refusing a write or a finish;
  - an empty turn;
  - output cut off at the provider's limit.
- Successful results of other calls stay available.
- **Repeated failures.** When the same call keeps failing the same way, its result adds a note telling the model to reassess, as Hermes does. The note never stops the run. OpenClaw blocks the call at that point; we don't.

### Parallel calls

- Consecutive read-only calls run together, up to 8 at a time, following Hermes's ordered batching.
- Every call that isn't read-only is a barrier, and no call moves across it. Hermes also lets writes to different files run together; we don't.
- Each result is saved when it returns, and the model gets the results in call order, as in all four references.
- If one read fails, the other reads keep their results.

### Large results

Each tool result has a size limit, as Codex limits tool output.

- **A large read**, or the receipt of an action that already committed, comes back as a preview. The model can read the rest page by page under current authority, and the product keeps the exact original.
- **A new write whose receipt is too large** is rolled back, and the model is told it can try again with a smaller request.
- **Nothing is done again** just to get its result back.
- **When a batch of results is larger than the space left,** the newest results become first pages that fit. Every call keeps its result.

### Skills

- **Each product supplies** its system prompt, its skills folder and its visibility rule.
- **One shared loader** lists a short index and opens a skill by name. The same visibility rule controls both listing and opening.
- **Reading a skill gives no permission.**
- **The skill package is frozen with the accepted work,** so recovery reads the same version.
- **Chat** starts small and finds what it needs.
- **Background work** starts from its assignment, gets all the product's tools, and can preload the skills its assignment always needs, as Hermes's scheduled jobs do.
- **Text the model reads** has no ticket numbers or porting notes.
- **Each piece of guidance lives in one place:** the system prompt or a skill, not both.
- **Skills improve from traces.**
  - Traces show which skills load, what they cost in context, and what goes wrong after a load.
  - A skill that runs long gets shorter, with details moved into linked references.
  - What nearly every run needs moves into the system prompt.
  - A failing tool gets fixed at the tool, not patched in a skill.

### Context

- The model pulls records through source tools.
- Within a window, the provider adapter keeps the model's reasoning between calls and carries what a tool exchange needs to continue.
- **Measurement:** the provider's reported usage, plus an estimate for everything added since.
- **Working window:** each job has one, set in product settings and kept separate from the model's input limit, as OpenClaw keeps a working limit separate from the model's maximum.

### Working notes and fresh windows

- **Notes at any time.** The model can write a working note whenever it finds one useful, and can ask for a fresh window, as Codex's new-context tool allows.
- **Near the end of the window**, the harness keeps room for the reply and asks the model to answer if the work is done, or to save a handoff if more remains, as Codex does.
- **Every request has the same tools.**
- **A fresh window opens only after the product confirms the note is saved.** It contains:
  - the assignment;
  - the original request and accepted steering;
  - pending work and results;
  - the exact handoff text.
- **At the start of a run**, the same handoff point applies. A conversation already past it opens like a fresh window instead of sending the full history.
- **Context pressure never causes a failure,** including a provider refusing a request as too long. The result is a fresh window.
- **A note belongs to the conversation or the piece of work it continues,** and the product stores it.
- **A note can point to a source.** When the model needs the source, it reads it again.

### Earlier conversation

Three history tools follow Codex's design. The harness defines them once, and each product supplies its records and permission checks.

- **List** earlier messages, newest or oldest first, with long messages shortened.
- **Read** one full message, in parts.
- **Search** messages by text.

Rules:

- Shortening never changes the saved original.
- A follow-up can depend only on a message the model received in full, and that stays true after a restart.
- Each product builds the history from its own conversation when the work starts.

### Workers

The lead can give parts of the work to workers, following Codex's design.

- **When to delegate.** The spawn tool's description says when delegation helps. Each job's instructions say whether that job can delegate.
- **What a worker starts with.** Its assignment and the context the lead selects, never the lead's conversation. The worker can open more evidence it's allowed to read.
- **How a worker runs.** The same loop, with the product's worker settings, instructions made from its assignment, and only the tools the product allows.
- **Limits.**
  - Workers don't start workers, and up to 3 run at a time, as in Codex.
  - Workers share the run's one count of requests, tokens and time. No part of the budget is reserved for a single worker.
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
- **Each part of a request is saved once.** The parts are the frozen recipe, the conversation and tool items, and what was added at send time, such as a warning, the handoff text, the final-request text and the previews the model got. A request points to these parts, the way Codex rebuilds its requests from saved items.
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
  - Saved side effects decide whether a step can run again, not whether text was shown, as in OpenClaw.
  - Once output is visible, the transport never retries.
- **Drafts are replaced.** After an interruption or a steer, the harness discards an unpublished draft and runs the step again without it, even if its text was shown live. A published partial answer is part of the conversation, not a draft.
- **Steering.**
  - New input, whether a steer now or a queued message later, is a saved item with its own ID. The harness puts it into a request exactly once, as Codex's input queue does.
  - A steer cancels the model request in progress, and the harness redoes that step with the new input. Finished tool work is kept and reused, never repeated.
  - One answer covers all the messages that steered it, and late output never replaces newer direction.
  - The loop supports both products: in Chords, Stop holds the work until Resume; in Duet, a new message steers the reply in progress.
- **Honest endings.** A run ends exactly once. The product records the outcome when it accepts a finish or ends the attempt. A run that reaches a limit is never recorded as a failure.
- **Isolation.** One account's or practice's work never reaches another's context, tools or results.
- **Rate limits.**
  - Each product has one saved rate-limit gate for its provider quota.
  - If the provider refuses or drops a request before it produced anything, the request waits, follows the provider's retry hint and tries again with backoff, within the run's deadline.
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
| Model request | The recipe and model of one request. |
| Tool call | The call and the exact evidence it read. A product's chart meets the harness here. |
| Reply text | The reply text while the model writes it. Live only. |
| Worker result | One worker's result, for its parent. |
| Finish proposed | The finish the model proposes. |
| Window change | A move to a fresh window, with its timing. |
| Run outcome | How the run ended. |
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
  - **The backstops.** A run has three: 100 model requests, a time allowance and a token count. Each tool result also has a size limit, described in [Large results](#large-results).
  - **Tokens** are output plus uncached input, the way Codex counts its budget. Cached input is cheap and doesn't count.
  - **Owner.** The harness keeps the shared backstops in one place. Each product sets its time allowance, and the harness calculates the run's deadline and the time it keeps for the final request.
  - **Across attempts.** The counts and the clock continue across a run's attempts. Try again resets them, and so does a new message on held or failed work.
  - **Provider retries** don't count toward the request limit. Codex and OpenClaw also keep retries separate.
  - **Near a limit.** The model gets one warning, added to the newest tool result so the cached prefix stays the same, telling it to save its progress.
  - **At the limit.** The final request keeps the full tool list and asks for no tool calls. The harness takes only the text and runs nothing. Hermes ends a run this way on its main path, keeping the tools so the cached prefix stays the same.
  - **After the limit.** The run is held.
    - Chat keeps an honest partial answer, which Try again continues.
    - A draft revision, Check-in or Today run keeps its handoff in the run record, and only its explicit finish publishes.
  - **Values** are adjusted from traces.
- **Provider.** Every product streams from the provider.
  - If a stream goes silent, the harness tries again after the same idle timeout Codex uses.
  - Provider problems before output are retried in the run and don't count as a failed attempt.
  - If saving a paid answer fails, the harness keeps the answer and uses it in the retry instead of calling the model again.
  - If a source is down, the work stays resumable.
- **Recovery.**
  - Automatic recovery stops after 5 failed attempts, as Pi stops at its retry maximum. The work stays resumable through Try again.
  - A steer or a correctable tool error doesn't count as a failed attempt.
  - One owner decides what counts as a failed attempt.
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
  - It covers a chart question, a long conversation that changes windows, and a revision.
  - It reports tokens and cached tokens for each request, window changes, the tools and skills chosen, and what went wrong.
  - Scripted tests cover only what dogfooding can't cause: a crash between an effect and its record, a duplicate delivery, a restart during a run, and the consent check.
