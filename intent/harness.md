# The harness

The harness is the agent loop that Coda Agents and Duet both run on. It isn't a product surface, and it stays deliberately thin. The models are smart; the harness's job is to give them the right tools, the right context at the right moment, and skills they can find, then get out of the way. Open questions live in DECISIONS.md, and git keeps the history.

## What it is for

There is one product-neutral loop, in the shared `harness` crate beside the shared `ui` and `http` crates. It carries the provider and tool protocol, runs the model→tool→model cycle, handles scheduling and cancellation mechanics, measures context, and emits neutral, ordered events. There's no second engine and no shared runtime service.

A good harness is about context management and safety, and not much else. Everything that makes a product what it is stays with that product: clinical types, accounts, consent, evidence, context policy, model roles and instructions, which tools and skills are admitted, durable state, scheduling, effects, commit, recovery and how a run ends. The harness carries those through execution without owning them.

The harness stays out of the product behavior charts. It meets them only at events: a tool call is an event, and what a product must supply before a run is a precondition.

## Principles

Coda's shared principles (PRINCIPLES.md) govern this spec. These are the ones specific to the harness.

1. **Context is curated and discoverable, never stuffed.** The model gets salience on what exists, through clear tool and skill names, and discovers the rest. The harness never resends the whole transcript each turn, never breaks the prompt cache, and doesn't reach for subagents where progressive disclosure does the job.
2. **The model owns judgment.** The harness adds tools and guardrails and never restricts what the model may think or do within the product's limits. No intent classifier, routing table or code-side judgment decides for it.
3. **Product-neutral mechanics only.** Each fact has one owner. Separate owners connect through recorded facts, and none infers another's terminal state.
4. **Models are swappable.** The harness isn't built around model names. Which model fills which role is product configuration.
5. **Fail closed on access, consent and patient data; fail gracefully on everything else.** One failed tool read or bad item fails only itself, and independent successes are kept.
6. **Pull, not push.** New information reaches a run because the work pulls current sources. A push only wakes the work.
7. **Production shapes the design.** Local and hosted are two ways to run one engine. Local limits never drive production architecture, and nothing local becomes a product rule.

## How it works

**A run.** A finite sequence of model requests for one responsibility a product has accepted: a command, a message, a scheduled job or a resume. The product supplies identity, permissions, evidence and the tools for the job, and rechecks them. A run is over once the model makes its finish call. Its result is a completion candidate, and the product decides whether it becomes a saved reply.

**Tools.** Tools are how the model reads and acts. The harness enables the right tools for each command, and tools go with the job, not the model. A malformed, unknown or refused tool call returns its reason in the result, and the run continues so the model can redo just that call.

**Skills.** A skill explains a capability and loads only when the model opens it. The model sees a compact index of skill names, lists and views what it needs, and products can preload the skills a scheduled job will want. Reading a skill never grants tools or permissions. The loader is shared; each product has its own catalog.

**Context.** Active context is a view the model can rebuild, never the store. Records come in through source tools, never injected wholesale, and workers don't inherit the lead's whole conversation. When context fills, the harness measures it and offers the transition. Each product owns how its context is preserved, the two products' policies stay separate, and neither moves to one shared compaction policy.

**Skills improve from traces.** Traces show which skills load, what they cost in context, and what goes wrong after a load. A skill that runs long gets slimmed, with detail moved into linked references. What nearly every run needs moves into the resident prompt, and a failing tool gets fixed at the tool instead of patched in a skill.

**Workers.** The lead of a run can hand pieces of work to workers. A worker gets its assignment with only the context and tools it's allowed, and its result returns to the right parent. Whether and how deep to delegate is the model's judgment.

**Lanes.** Interactive and background work run in separate lanes with their own capacity, so a burst of scheduled work never blocks a conversation or eats the quota an answer needs.

**The run record.** Each run keeps its model and tool calls, the exact evidence read, the recipe and model, and the outcome, and it stays inspectable after a restart. Coda's own run record is the source of truth for every tool built on top of it.

**Simulation.** The loop runs the same way under simulated triggers. A simulation supplies triggers, events and actors and never scripts the model.

## Guarantees

These are the promises the products build on.

- **Durable runs.** Run records and control state survive restarts. Scheduled and resumed work runs in hosted infrastructure and never depends on a developer's machine.
- **No duplicate effects.** Each logical change, such as a send or a saved routine, keeps one identity across retries, so interrupted work can resume without doing anything twice. Nothing is retried transparently once its output is visible.
- **Steering.** Stop, queued input and steering each affect the intended work once, and late output never overwrites newer direction. The loop supports both products: a physician's Stop stays held until Resume, and in Duet a new message steers the reply in progress. Runs can run in parallel within one account.
- **Honest endings.** After the finish call the run is over, and its outcome is recorded truthfully.
- **Isolation.** One account's or practice's work never reaches another's context, tools or results.
- **Access and consent at the tools.** A tool refuses what the caller isn't allowed to read or do, and those checks fail closed.
- **Rate limits.** Each provider quota has one rate-limit gate. A request refused before it produced anything waits, honors the provider's retry hint and retries within the run's deadline. The gate exists before any hosted or paid run.
- **Context transitions.** A context transition never replays an effect.
- **Accounting.** Every model call is recorded against its person and purpose, failed attempts and background work included, and unknown usage is recorded as unknown, never zero.

## Events to the products

The harness writes neutral, ordered events into each product's own durable store. Required acknowledgments come before dependent work, and none of this is best-effort telemetry or a new shared journal.

- **Model request**, with the recipe and model.
- **Tool call**, with the exact evidence read. This is where a product's chart meets the harness, and products build live status (current step, purpose, trail) from these events, never from model text.
- **Streamed reply output** while a reply runs; the product keeps it when the reply finishes.
- **Worker result**, routed to its parent run.
- **Completion candidate**, for the product to accept or not.
- **Run outcome**, kept with the run record.
- **Usage**, per model call, attributed to person and purpose.

Before and during a run, the product supplies its preconditions: admitted tools, authority and currency checks, identity, permissions, evidence, persistence, effects, commit and recovery policy. Duet also requires accepted Terms, AI permission and the operator switch for every model call.

## What the harness never does

- Depend on either product's domain, server, authentication, wire or storage crates.
- Own accepted input, schedules, conversations, memory or action results, or share clinical memory between the products.
- Impose one commit or rollback policy on both products.
- Treat its result as a saved reply. Only the product publishes.
- Read model prose as clinical decisions, judge clinical reasoning, or decide in code that valid evidence is too sparse, noisy or unimportant to reach the model.
- Make citations depend on parsing model text. The model sees short labels that map back to the source.
- Invent progress. Status comes only from real events.
- Keep a second run ledger anywhere, including cloud scheduling or delivery.
- Grow cost with the size of the roster instead of with what changed.
- Appear in the product behavior charts.

## Scope

In the MVP:

- One loop, reusing the existing provider adapters and the skills loader.
- A physician proof: a synthetic physician workspace accepts a durable command, runs the loop, gets a real evidence-tool result and commits a grounded reply that reads back.
- A Duet proof: a fictional, consented Duet account accepts a durable message, uses the same loop for a source read or a requested routine, and streams and keeps the reply.
- Then every other model-running journey runs through the loop, with its instructions, skills and tools verified to reach the model.
- Duet's unmounted conversation path is retired once the loop covers it.

Later:

- A formal model of the harness, only when a promise such as steering, restarts or no duplicate effects is worth proving.
- Programmatic tool calling.

## Engineering decisions

- **Model provider.** Production model calls go through the OpenAI API under its BAA. Subscription CLIs are for local testing only. Staging has its own real model path with its own spend cap.
- **Budgets.** The loop's request, tool, byte and deadline limits, retry budgets, cadence and how much work runs at once are engineering decisions, adjusted as we learn.
- **Recovery.** The physician product continues interrupted work through successor attempts; Duet retries the same accepted run.
- **Storage.** Permanent records are never rewritten to add a field. A new field is derived from what they hold when that's exact, or reads unknown, and old runs stay replayable.
- **Build order.** Hosted access and durable conversations, then the physician and Duet proofs together, then the remaining care and connected-patient journeys, then the full Rust cutover.
- **Capacity.** The physician product adds an admission limit and priority classes only once load measurements justify them. Duet has one capacity owner with separate queues for chat, Check-ins and reviews.
