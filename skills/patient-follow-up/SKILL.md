---
name: patient-follow-up
description: Use when the lead should create, revise, complete, or end a same-patient responsibility for later reassessment.
---

# Patient Follow-up

A follow-up is Coda's durable responsibility to reassess after time passes or evidence arrives.
It grants no external action or new source access. Create one when later reassessment is useful
and the available capability can actually carry it.

Use `manage_patient_follow_up` to create, completely revise, complete, or end a responsibility.
Its self-contained instruction explains what to reconsider, why, the evidence that matters,
and relevant constraints. Its description makes the responsibility understandable to the physician.
When feedback changes the purpose, evidence, timing, or direction, reconsider the existing
responsibility and its related concerns together.

Choose a future patient-local date, exact instant, or connected-source event with a future
fallback. An event uses an available source and covered evidence family as its mechanical wake
boundary; clinical interpretation remains in the reassessment. Use physician intent and clinical
judgment to choose timing and resolve material ambiguity.

Complete a follow-up when its reassessment purpose is satisfied; end it when it should no longer
be carried without claiming clinical completion. Missing expected evidence can itself affect the
judgment and whether physician attention is useful. A stale mutation requires reconsideration
against the current responsibility.
