# Coda principles

These govern everything below them: physician.md, duet.md, connected-care.md, harness.md, the behavior charts and the proofs. Every principle is in Dev's words, which he has repeated since September; the UI principles also quote the Linear tickets that carry them. Each product spec keeps only its product-specific principles and defers to these.

## Order of authority

1. **These principles.**
2. **Dev's later explicit decisions** (DECISIONS.md, "Decided by Dev").
3. **Dev's feature choices and approvals** in sessions. These include multiple-choice picks and "I agree" replies to proposals.
4. **Linear.**
5. **Old docs and the website.**

If a lower source adds machinery that a principle rules out, the principle wins and the spec follows it without asking. A cap, gate, pause, dialog, toggle, fixed count or code-side judgment is ruled out by default, even if Dev approved it in passing or "for now". Dev is asked only about questions the principles don't settle: what is in scope, consent and data choices, and real product trade-offs.

## The principles

**1. Powerful models. Never restrict their intelligence or judgment.**
"I never want to restrict the models from their power and potential and protectiveness." [codex:rollout-2026-09-11T15-36-57-01a091f9-1e58-7892-9c38-90e31364@2026-09-14T01:51:53.620Z]
"The interface should enable that intelligence. We are never restricting intelligence. We are only providing more guidance and capability and structure in focus ways, in additivie ways" [claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T17:57:29.402Z]

**2. A thin harness: context management and safety.** The harness guides and empowers with clear instructions, skills and tools. It is not prescriptive.
"a good harness is all about context management and, safety, and that's really it." [codex:rollout-2026-10-20T23-29-08-01a15c81-6d66-7c00-8ba7-351f7c7e@2026-10-21T18:06:58.475Z]
"keep ou prncipels of powerful models, thin harness, clear system prompts and skills, lightweight code guardrails and keepign what absolutely SHOUDL be detemrinsitic, detemrinsiitc. NOT brittle." [codex:rollout-2026-09-22T11-42-39-01a0c9c8-8f0f-7e51-865c-4f2e9563@2026-09-27T19:37:14.572Z]
"our light our harness is about guiding them and helping them with like capabilities and provenance and authority rather than being prescriptive with them all the time." [claude:220b86fb-906c-44c1-a206-2c4016c51a61.jsonl@2026-09-23T02:39:03.374Z]
"The harness is not meant to restrict capabilities. It's meant to guide and empower With salient skills and tools." [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:24:59.542Z]
"The thing is about skill, disclose, like skills, system prompts, and progressive disclosure, not stuffing context, and not using subagents unnecessarily, but progressive disclosure, discoverability, sometimes will promote skills into main context as traces of all, et cetera." [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:46:26.476Z]

**3. Capabilities are additive.** A product capability is a promise and a set of tools and skills. It is never the boundary of what the model may do.
"the capabilities are additive tools and skills, not, like, restrictive to those." [codex:rollout-2026-10-21T16-08-06-01a16014-0209-7d01-a2bd-e5b72d31@2026-10-23T03:20:02.624Z]
"any capabilities we're building are not limiting it. It's just like more of the product promises or like first class capabilities, but the models could do so much more than even what we think, right?" [codex:rollout-2026-09-09T17-33-57-01a08817-8449-7361-9edf-b221dec5@2026-09-15T15:42:16.541Z]

**4. The model keeps judgment; code is lightweight guardrails.** Code keeps truth, access, provenance and what genuinely must be deterministic. It never decides meaning or judgment.
"code is never substituting for judgement. the model keeps clincial intelligence / opinions. code is just guardials this is a critical distinction I always say" [codex:rollout-2026-09-05T10-36-49-01a07200-2c17-78d0-aac6-b0f0cec0@2026-09-05T14:36:55.028Z]
"Don't ever allow code to separate out meaning, no brittle parsers, lexical shit, etc., and no code making judgment." [codex:rollout-2026-09-08T12-36-08-01a081e0-7f90-79e0-bf5e-8fe5383d@2026-09-09T03:07:10.657Z]

**5. Freedom within real bounds only.** The hard limits are access, consent, patient data, communication (including physician approval of any message the practice sends to a patient), and the fixed safety lines. Everything else is the model's judgment.
"give the model full like empowerment and intelligence within bounds." [codex:rollout-2026-09-01T23-29-58-01a0602a-957f-75d0-88df-c2e7476e@2026-09-02T22:16:38.876Z]
"We're giving him freedom to do stuff within the bounds of the actual task or goal, right?" [codex:rollout-2026-09-05T10-36-49-01a07200-2c17-78d0-aac6-b0f0cec0@2026-09-06T16:35:06.332Z]

**6. No brittle machinery.** No arbitrary limits, fixed counts, labels, parsers, lexical rules, rigid schema or speculative machinery.
"why do we have random limits on what sol flags as attention??" [codex:rollout-2026-09-08T12-36-08-01a081e0-7f90-79e0-bf5e-8fe5383d@2026-09-10T19:28:58.014Z]
"Why are we selecting a three? That's literally insane and so brittle." [codex:rollout-2026-09-01T23-29-58-01a0602a-957f-75d0-88df-c2e7476e@2026-09-04T03:21:20.545Z]
"If it contradicts the contract, or it is brittle, or arbitrary, like random labels, random markdown— sorry, random parsers, random lexical crap, brittle machinery, we need to discuss the clean path to removal." [codex:rollout-2026-09-01T23-29-58-01a0602a-957f-75d0-88df-c2e7476e@2026-09-04T05:36:36.919Z]

**7. Real data is messy.** Nothing assumes clean input.
"I'm really worried about bullshit assuming clean data which is never true from real world data, which would become brittle asismptisons" [codex:rollout-2026-09-22T11-42-39-01a0c9c8-8f0f-7e51-865c-4f2e9563@2026-09-26T23:37:25.861Z]

**8. Learn from the best open agents.** Codex, Hermes, OpenClaw, Muse, Instinct and Grok are the references for the harness and the experience. The harness keeps evolving with them, so it stays out of the behavior charts.
"I want to use best in class harness patterns, obviously not constraining the models in terms of their intelligence." [codex:rollout-2026-09-01T23-29-58-01a0602a-957f-75d0-88df-c2e7476e@2026-09-03T16:00:01.221Z]
"Goal is to be salient, clear, and helpful the way that muses, the way rock bot is, the way instinct is, the way open claude is, the way hermes is, et cetera." [codex:rollout-2026-10-20T23-29-08-01a15c81-6d66-7c00-8ba7-351f7c7e@2026-10-21T16:58:43.155Z]
"the harness will be constantly evolving, especially as we look to Hermes or OpenCloud, etc., and continue to improve on that." [codex:rollout-2026-10-21T16-08-06-01a16014-0209-7d01-a2bd-e5b72d31@2026-10-23T01:21:21.530Z]

**9. Few controls, nothing internal on screen.** People steer by talking to the agent and by a few direct controls. There are no extra buttons, modals, toggles or exposed machinery.
"I just want to make sure we're not, like, offering too many controls or toggles to users where it becomes too hard to maintain per user." [codex:rollout-2026-09-18T12-25-14-01a0b556-1d40-70c3-bbcc-4bc49784@2026-09-20T16:23:59.492Z]
"we're actually penalizing and, and removing physician intuition and physician experience by creating more buttons, more clicks, more modals, more views" [claude:220b86fb-906c-44c1-a206-2c4016c51a61.jsonl@2026-09-23T15:51:49.927Z]
"we don't want to expose internal machinery to the physician." [codex:rollout-2026-09-01T23-29-58-01a0602a-957f-75d0-88df-c2e7476e@2026-09-05T01:33:57.387Z]

**10. Subtract stale and brittle; never give up capability.**
"always good to remove subtractive stuff like sorry, subtract like stale behaviour or like brittle behaviour. But obviously not give up on capability, and also not anchor on local as a substitute for production." [codex:rollout-2026-09-01T23-29-58-01a0602a-957f-75d0-88df-c2e7476e@2026-09-03T18:49:59.177Z]

## UI principles

These apply to the physician app and Duet alike, on the web, iOS and Android. The UI is a full program later ("UI is going to be one full program later, like, down the line." [claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-25T00:05:27.734Z]). Until then they keep every surface pointed the same way. "I think the principles on UI are particularly critical too, when we have a great UI base already from our previous work, especially on the physician web app and CODA agents, as we move to Dioxys and the cross-platform stuff" [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:52:26.501Z]

**U1. Salient, with the least burden.** Show the most salient information with the fewest clicks and the least scrolling. Salient does not mean cutting information.
"We need to minimize clicks, minimize cognitive burden for the physician, and focus on the most salient, intuitive information." [codex:rollout-2026-09-01T23-29-58-01a0602a-957f-75d0-88df-c2e7476e@2026-09-02T20:01:40.033Z]
"I think the point is to be salient and ambient, and I don't want you to take that in a way of, like, oh, let's just cut information." [codex:rollout-2026-09-11T15-36-57-01a091f9-1e58-7892-9c38-90e31364@2026-09-19T17:22:42.497Z]
"We want to minimize scrolling to the extent possible, globally (where reasonable)" [codex:rollout-2026-09-08T12-36-08-01a081e0-7f90-79e0-bf5e-8fe5383d@2026-09-09T03:07:10.657Z]

**U2. Compact, and expands on demand.**
"everything should be compact / high density with ability to expand / with a lcick or soemthing, or a highlight when you hover over. This includes headers, right?" [codex:rollout-2026-09-11T15-36-57-01a091f9-1e58-7892-9c38-90e31364@2026-09-19T18:05:24.780Z]

**U3. Nothing invented.** Every label, control and concept comes from a real state or a real capability. There are no fallback or hardcoded labels.
"I think the point is BY should never create new concepts or controls that we haven't made the right product functionality for or contract functionality for. Does that make sense? That's fucking critical." [codex:rollout-2026-09-01T23-29-58-01a0602a-957f-75d0-88df-c2e7476e@2026-09-04T03:42:43.610Z]
"labels happen BEAUSE of states, not just a random fallback/hardcode bulshi right? what am I missing? I don't ever want invented UI elements" [codex:rollout-2026-09-08T12-36-08-01a081e0-7f90-79e0-bf5e-8fe5383d@2026-09-10T19:21:59.685Z]

**U4. Plain words people trust.** No internal or technical language, unnecessary labels or metadata (principle 9).
"I just need you to make sure this is truly a physician intuitive and delightful experience and something they can trust  THAT DOES NOT MEAN EXPOSE A BUNCH OF TEUCNCIAL JARGON/NONSENSE" [codex:rollout-2026-09-01T23-29-58-01a0602a-957f-75d0-88df-c2e7476e@2026-09-04T03:34:13.980Z]
"we should always be on the lookout for unnecessary information being shown to the physician that could disorient them, like unnecessary labels, unnecessary metadata, et cetera." [claude:220b86fb-906c-44c1-a206-2c4016c51a61.jsonl@2026-09-20T20:46:19.271Z]

**U5. One shared component set.** The physician app and Duet are built from one set of shared components in Dioxus, so every page, modal and element is consistent. The shared `ui` crate owns reusable presentation; there is no third palette or separate animation system.
"It's not just design references for duet though. It's also the physician app, right? I want to build, like, one sort of shared set of components. We're doing Dioxys for that reason." [codex:rollout-2026-10-27T22-35-30-01a1805c-d857-7f71-bf28-c890b0b3@2026-10-28T02:37:47.307Z]
"I think every page and every modal, every element being internally consistent is critical, and we do that by shared components, shared elements, I believe." [codex:rollout-2026-09-11T15-36-57-01a091f9-1e58-7892-9c38-90e31364@2026-09-20T16:57:23.785Z]
"Map type, spacing, color and motion to the shared `ui` roles; do not introduce a third palette or a separate animation system." [linear:ENG-168#Implementation direction for coda-rust]

**U6. Keep what already works.** The React physician app and the Coda Agents work are the base. Porting keeps their successful design and interactions; differences from them are agreed with Dev first.
"preserving its successful design and interactions" [linear:ENG-77#description]
"Review the experience with Dev and make any remaining differences explicit. Agree on intentional behavior changes before implementing them." [linear:ENG-77#Complete when]

**U7. Liquid motion and real state.** Clean states and transitions; motion shows progress instead of words. New information arrives gracefully, refreshed from the server, without blocking. Motion follows the actual run state and respects reduced motion.
"I want clean states, clean transitions, clean boundaries, and something that would be most intuitive for a physician." [claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-23T16:32:15.398Z]
"everything is liquid motion, gracefully fading in or out like liquid" [claude:220b86fb-906c-44c1-a206-2c4016c51a61.jsonl@2026-10-02T05:45:45.201Z]
"instead of using words or verbs, you can always use animations like pulsing or loading icons or bars, or like pulsing orbs to show loading or in progress, etc. instead of vocabulary." [codex:rollout-2026-09-11T15-36-57-01a091f9-1e58-7892-9c38-90e31364@2026-09-20T19:12:41.352Z]
"It gracefully loads, irrespective of what you're doing, right?" [codex:rollout-2026-09-09T17-33-57-01a08817-8449-7361-9edf-b221dec5@2026-09-14T20:45:06.439Z]
"Respect reduced motion and use actual run state." [linear:ENG-168#AI UI and motion reference]

**U8. Native and practical on every device.** It feels native, works across real screen sizes without hard-coding, and chooses what is practical over what only looks ideal.
"it should look nice and native like imessage in the browser. can learn from coda-app design" [claude:220b86fb-906c-44c1-a206-2c4016c51a61.jsonl@2026-09-22T17:50:58.978Z]
"remmeber we need to ensure this works across reaosnable desktop dimensions, not "hard" coding or ove anchroignt o specific reoslutions. right?" [claude:220b86fb-906c-44c1-a206-2c4016c51a61.jsonl@2026-09-29T15:12:38.350Z]
"you always need to think about like not just what would look nice from like a f an ideal state of the world, but what's actually practical" [claude:220b86fb-906c-44c1-a206-2c4016c51a61.jsonl@2026-09-24T15:55:08.601Z]

**U9. Learn from the best AI interfaces.** The references are the Codex desktop app, shadcn AI Elements and assistant-ui (ENG-168, ENG-160). Coda adapts their useful interactions to Dioxus; they are references, not dependencies.
"design with the ocntract and physician intuiveness in mind, + always taking lessons from the codex runtime + animations, smotohness, transitons across states, etc. right?" [codex:rollout-2026-09-01T23-29-58-01a0602a-957f-75d0-88df-c2e7476e@2026-09-07T02:10:22.782Z]
"not takign enough from the amazin react components in the reference studies" [codex:rollout-2026-09-11T15-36-57-01a091f9-1e58-7892-9c38-90e31364@2026-09-19T20:19:33.087Z]
"Adapt the useful interactions to Dioxus and existing motion roles; the React implementation is a reference, not a selected dependency." [linear:ENG-168#AI UI and motion reference]

## What each product is

- **Coda Agents (physician).** An ambient Codex for physicians: an AI care team that does the work between visits, under the physician's clinical authority. "aligned with our nroth star goals of an ambient codex for physicians experience" [claude:220b86fb-906c-44c1-a206-2c4016c51a61.jsonl@2026-09-22T22:46:43.844Z]
- **Duet (patient).** A managed personal agent for health. "I want duet to be like hermes/nstinct/meta muse but for health." [codex:rollout-2026-09-09T17-33-57-01a08817-8449-7361-9edf-b221dec5@2026-09-10T00:49:52.159Z] "the core guiding principle on the product is this is instinct or muse or grockbot or hermez or open claude managed and like maintained for you. For health, right? the, in my view, the today page, all that stuff is just effectively, a layer and interface and, tool calls" [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:08:20.139Z]
- **Connected care.** The patient's agent and the practice's AI care team working together, inside the hard limits of principle 5.

## Testing a rule against these principles

For any rule in a spec, chart or proof, ask:
- Does it limit what the model may think, investigate or take on? Then it goes, unless it's a principle 5 limit.
- Does it put judgment in code, such as a fixed count, threshold, label or parser? Then it goes. The model decides; code checks truth and access.
- Does it add a control, dialog, pause or toggle for the person? Then it goes, unless people need it to steer or it's a consent choice.
- Is it a cost or capacity limit? Then it's a runtime budget owned by Tyler, not product intent, and it stays out of the specs.
