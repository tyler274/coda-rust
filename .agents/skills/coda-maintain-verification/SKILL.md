---
name: coda-maintain-verification
description: "Audit and repair a Coda app verification recipe through source inspection and live feature checks. Invoke explicitly for an app verification maintenance pass."
disable-model-invocation: true
---

# Maintain app verification

Follow the [shared principles](../../../verification/README.md#principles).
Edit verification instructions, feature maps and their harnesses. Product fixes
require implementation authority from the active task; otherwise report them.

## Inspect and drive the mapped features

1. Locate the requested `.agents/skills/coda-verify-*/` recipe and read its agreed
   journey. Resolve an ambiguous target; if none exists, use
   [app creation](../coda-create-verification/SKILL.md).
2. Reconcile the feature index with its files. Inspect each feature's source,
   actual entry points and recent changes against intended behavior. Collect a
   concise drive recipe and evidence of drift; avoid a generated inventory.
   Use read-only parallel source review only when delegation is authorized.
3. Merge overlapping recipes into practical app states. Check source-cited drift
   and newly exposed user paths missing from the map. Coordinate changed diagram
   or proof connections with [formal maintenance](../coda-maintain-formal-verification/SKILL.md).
4. Follow the recipe's launch model and drive every mapped feature, even when the
   source looks clean. One owner drives a shared instance serially; short-lived
   commands use fresh sessions. Doctor before driving and after surprising results.
   A healthy process does not resolve a wedged UI: reset or relaunch when necessary.
5. Capture actions, visible outcomes, visual readback and durable effects. Record
   an unreachable feature only with its concrete prerequisite and attempted route.
   Missing prerequisites in the recipe are drift. Preserve evidence through every
   cleanup and clean failed-iteration residue without killing shared resources.
6. Triage: repair incorrect instructions or harness gaps at their owner; retain
   actual product failures against unchanged expectations. If doctor failure is
   instruction drift, repair it and retry once, restarting what the fix invalidated.
   Re-drive every harness correction before handing it over.
7. Tear down owned resources after the final drive. Verify retained evidence,
   reread changed instructions and report coverage and remaining limits.

## Report the outcome

- **clean:** every feature received source and live coverage; no useful change.
- **changed:** verified instruction, map or harness corrections are ready.
- **blocked:** coverage or verification remains incomplete; state the exact blocker.

Keep run notes in scratch evidence, not a new tracked report. Commit or publish
only within task authority. A maintenance pass does not require a separate PR,
new schedule or product redesign.

Adapted from [PStack maintain-verification-skill](https://github.com/cursor/plugins/blob/eb4db2b55b2b7dec3d9282754cddf0a56bf4ff10/pstack/skills/maintain-verification-skill/SKILL.md).
