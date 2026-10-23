---
name: coda-create-verification
description: "Create a Coda app verification skill using its real Rust application, existing harnesses and user-visible readbacks. Invoke explicitly to establish app coverage."
disable-model-invocation: true
---

# Create app verification

Follow the [shared principles](../../../verification/README.md#principles).
Write for an agent entering the repository mid-change.

## Discover how to exercise the agreed journey

Read the agreed journey and statecharts; connect existing formal promises without
making a passing model proof a prerequisite for testing the app. Ask only for
missing information that cannot be established from the repository:

- **Surface/run:** actual user entry points, documented build/start commands,
  authentication, data initialization, seeds, ports and readiness.
- **Drive/observe:** existing browser, HTTP, CLI or native harnesses; stable selectors;
  screenshots, accessibility state, responses, logs and durable side effects.
- **Isolate:** whether instances can have separate ports, profiles and data. Identify
  the exact instance before driving it. Fix startup within scope or report the blocker.

Code explains how to exercise behavior; the agreed design determines expectations.

## Write one usable recipe

Create or extend `.agents/skills/coda-verify-<app>/SKILL.md`, with a narrow app/surface
frontmatter description. Use actual commands and paths, leaving no placeholders:

- **Launch:** reach a usable seeded instance, including migrations; name readiness
  and teardown. For short-lived commands use a fresh isolated session per drive.
- **Doctor:** read-only instance identity, readiness, build and authentication checks.
- **Drive:** exercise real entry points with existing harnesses and stable handles.
- **Evidence:** capture action, resulting state and side effects. Name fixture/replay
  boundaries and retain artifacts after cleanup. Verify what dry-run modes skip.
- **Visual readback:** inspect images and accessibility state; DOM presence can hide
  clipping or unusable controls. Report visual failures alongside functional passes.
- **Cleanup/helpers:** stop only owned processes and scratch state; retain evidence.
  Make shipped helpers executable and show their invocation.

Add `features/README.md` and feature recipes using the
[feature-map example](references/feature-map-example/README.md). Start with the agreed
scope, typically 3–5 outcomes. Each feature has four H2s: `Sub-features`,
`How to get to it (user POV)`, `Driving it with <harness>`, and `Gotchas`.
Link intended journey, diagram/data proofs, real owners, entry points and observable
end states. Keep absent seeds, unavailable prerequisites and untested routes visible;
one convenient entry point does not cover all of them.

## Run it before handoff

Follow launch, doctor, one mapped feature, evidence capture and cleanup end to end.
Confirm evidence survives cleanup. Clean failed attempts before retrying. Re-drive
any changed harness. Report setup supplied by a stand-in but absent on the real
target; that target remains unverified. Unexecuted app instructions remain a draft.

Report the demonstrated boundary and remaining coverage. Use
[app maintenance](../coda-maintain-verification/SKILL.md) for drift and
[formal creation](../coda-create-formal-verification/SKILL.md) to add proofs to this
same recipe. Maintenance follows relevant work; no extra schedule is required.

Adapted from [PStack create-verification-skill](https://github.com/cursor/plugins/blob/eb4db2b55b2b7dec3d9282754cddf0a56bf4ff10/pstack/skills/create-verification-skill/SKILL.md).
