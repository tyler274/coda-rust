---
name: improve-review-pr
description: Learn from actual review outcomes and user corrections to improve the responsible guidance. Use for retrospectives or demonstrated workflow problems, distinguishing durable lessons from product decisions and one-off preferences.
---

# Improve Review

Improve future judgment from observed outcomes. Follow the shared
[development workflow](../../../AGENTS.md#development-workflow); a retrospective
can conclude that no guidance change is warranted.

## Reconstruct the evidence

Start with the shared GitHub record: PR intent, bot reviews, author replies,
correction commits, reviewer dispositions, and verified outcomes. Link the source
reviews, comments, and revisions when explaining a lesson. Include
local review artifacts or original user decisions where GitHub lacks consequential
context. Establish the target revisions, guidance, and reviewer sessions used;
current instructions do not prove what an earlier session followed.

Trace the sequence across context, review judgment, parent reconciliation,
correction, and verification. Identify what succeeded or failed, including shared
causes across findings and interactions between related PRs. Check claims against
actual results: silence, reactions, thread resolution, and merge are not proof
that a finding was right or wrong.

## Improve the responsible owner

Read the current guidance before proposing a change. Put shared development
defaults in [AGENTS.md](../../../AGENTS.md#development-workflow); local review
handoff and reconciliation in [local-review](../local-review/SKILL.md); PR scope,
coordination, and discussion in [review-pr](../review-pr/SKILL.md); how every
reviewer judges a change in the [review standard](../local-review/references/standard.md);
what is specific to a PR reviewer in its [reviewer guidance](../review-pr/references/reviewer.md); and
execution behavior in the relevant launcher or execution reference.

Explain the observed failure, its cause, the future decision that should change,
and where the lesson does not apply. Prefer replacing or simplifying guidance.
Product defects belong with their product owners; an unusual example or advisory
preference need not become a universal rule or new review gate.

## Check the result

For a requested assessment, present the diagnosis and proposal. For authorized
guidance changes, update the responsible files and their callers, preserving
unrelated work. `.agents/skills` owns the repository copies and `.claude/skills`
links to them.

Check a realistic example and counterexample: shared cause versus local defect,
coupled batch versus unrelated work, or implementation versus review-only scope.
Use independent local review for substantive changes under the repository default.
A historical model replay should receive only evidence available at that revision.

Run appropriate structural or script checks and distinguish them from model
behavioral validation. Report the lesson, change, evidence, and limits in chat.
Judge usefulness by material misses, false alarms, repeated corrections, and
effort to understand intent—not comment count or agent agreement.

## Design reference

[Warp improve-review-pr](https://github.com/warpdotdev-demos/cloud-factory-demo/blob/main/.agents/skills/improve-review-pr/SKILL.md)
informs the feedback-to-guidance approach.
