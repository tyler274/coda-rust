# Coda principles

These govern everything below them: physician.md, duet.md, connected-care.md, harness.md and the feature models. Each product spec keeps only its product-specific principles and defers to these. When a spec line conflicts with a principle, the principle wins and the line changes. A cap, gate, pause, dialog, toggle, fixed count or code-side judgment is ruled out by default. Questions the principles don't settle, such as what's in scope, consent and data choices, and real product trade-offs, go to DECISIONS.md.

## The principles

**1. Powerful models. Never restrict their intelligence or judgment.** The interface exists to enable the model's intelligence. We add guidance, capability and structure in focused, additive ways, and we never take power away from the model.

**2. A thin harness: context management and safety.** A good harness is about context management and safety, and that's really it. It guides and empowers the model with clear system prompts, salient skills and the right tools, and it is never prescriptive. Context comes through progressive disclosure and discoverable skills instead of stuffing everything in up front, and subagents are used only when they actually help. Skills improve from traces, and what nearly every run needs gets promoted into the resident prompt. Code guardrails stay lightweight, and what genuinely must be deterministic stays deterministic without becoming brittle.

**3. Capabilities are additive.** A product capability is a promise plus a set of tools and skills. It is never the boundary of what the model may do, and the models can do far more than the capabilities we name.

**4. The model keeps judgment; code is lightweight guardrails.** Code keeps truth, access, provenance and what genuinely must be deterministic. It never decides meaning or judgment: no brittle parsers, no lexical rules, no code making a clinical call.

**5. Freedom within real bounds only.** The hard limits are access, consent, patient data, communication (including physician approval of any message the practice sends to a patient), and the fixed safety lines. Inside those bounds, everything is the model's judgment.

**6. No brittle machinery.** No arbitrary limits, fixed counts, labels, parsers, lexical rules, rigid schemas or speculative machinery. When something contradicts the product, or is brittle or arbitrary, we take the clean path to removing it.

**7. Real data is messy.** Nothing assumes clean input.

**8. Learn from the best open agents.** Codex, Hermes, OpenClaw, Muse, Instinct and Grok are the references for the harness and the experience. The harness keeps evolving with them, which is why it stays out of the feature models.

**9. Few controls, nothing internal on screen.** People steer by talking to the agent and with a few direct controls. There are no extra buttons, modals, toggles or exposed machinery, and every extra click takes something away from the physician's own intuition.

**10. Subtract stale and brittle; never give up capability.** Removing stale or brittle behavior is always good, as long as we don't give up capability or treat local as a substitute for production.

**11. One fact, one owner.** Every screen, job and product reads the same owner for a fact, and nothing keeps a parallel copy of state.

## UI principles

These apply to the physician app and Duet alike, on the web, iOS and Android.

**U1. Salient, with the least burden.** Show the most salient information with the fewest clicks and the least scrolling. Salient doesn't mean cutting information.

**U2. Compact, and expands on demand.** Everything is compact and dense, headers included, and expands with a click or reveals more on hover.

**U3. Nothing invented.** Every label, control and concept comes from a real state or a real capability. There are no fallback or hardcoded labels.

**U4. Plain words people trust.** No technical jargon, internal language, unnecessary labels or metadata that could disorient someone (principle 9).

**U5. One shared component set.** The physician app and Duet are built from one set of shared Dioxus components, so every page, modal and element is consistent. The shared `ui` crate owns reusable presentation, and type, spacing, color and motion map to its roles, with no third palette or separate animation system.

**U6. Keep what already works.** The React physician app and the Coda Agents work are the base. Porting keeps their successful design and interactions, and any intentional difference is agreed before it's built.

**U7. Liquid motion and real state.** Clean states, clean transitions and clean boundaries. Motion, such as pulsing or loading indicators, shows progress instead of words. New information fades in gracefully, refreshed from the server, without blocking whatever the person is doing. Motion follows the actual run state and respects reduced motion.

**U8. Native and practical on every device.** It feels native, like iMessage in the browser, works across real screen sizes without hard-coding resolutions, and chooses what is practical over what only looks ideal.

**U9. Learn from the best AI interfaces.** The Codex desktop app, shadcn AI Elements and assistant-ui are the references. We adapt their useful interactions to Dioxus and our motion roles; they are references, not dependencies.

## What each product is

- **Coda Agents (physician).** An ambient Codex for physicians: an AI care team that does the work between visits, under the physician's clinical authority.
- **Duet (patient).** A managed personal agent for health, like Instinct, Muse, Grok, Hermes or OpenClaw, run and maintained for you. Today and the other screens are views of what it keeps, written by its tool calls.
- **Connected care.** The patient's agent and the practice's AI care team working together, inside the hard limits of principle 5.

## Testing a rule against these principles

For any rule in a spec or model, ask:

- Does it limit what the model may think, investigate or take on? Then it goes, unless it's a principle 5 limit.
- Does it put judgment in code, such as a fixed count, threshold, label or parser? Then it goes. The model decides; code checks truth and access.
- Does it add a control, dialog, pause or toggle for the person? Then it goes, unless people need it to steer or it's a consent choice.
- Is it a cost or capacity limit? Then it's an engineering decision, not product intent.
