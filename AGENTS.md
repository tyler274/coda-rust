# Coda Rust operating contract

This repository is the Rust workspace for Duet and the physician client. The [intent specs](intent/README.md) own product intent: the principles, the
product specs, stated as current truth. Linear tracks scope, work and acceptance against them; this workspace owns Rust
implementation, operations and proof. Start with the [canonical map](book/src/reading.md).
The user's latest explicit decisions take precedence and land in the specs. Submodules are read-only: behavior that appears only in a
submodule or a generated wire schema is not a specification, and is not restored merely because it is there.

## Writing code

- **Model the problem using the type system.** Start from the domain's meaningful
  distinctions, valid states and allowed operations. Encode the rules the compiler
  can check in types, ownership and interfaces, so pieces cannot be plugged in
  upside down. Use static analysis generously to enforce those rules. Prefer
  guarantees over conventions callers must remember, and choose the simplest
  representation that prevents meaningful mistakes. Common misses: bools or
  `Option`s that together encode one state, where an enum holds each state's
  data in its variant; strings for codes, ids and states, which parse once at
  the boundary into enums or newtypes; one string-named entry point where each
  kind of call deserves its own typed function; error variants no path
  produces; and wildcard arms or `#[serde(other)]` that give up exhaustiveness.
  Precedents: the typed calls in `crates/ui/src/script.rs`, the `Readiness`
  and `Standing` model in `crates/extension-popup/src/face.rs`.
- **Use the typed owner, not a string.** Another language or a structured meaning
  never travels as text inside Rust. Queries go through the product's typed
  database API, browser behaviour lives in its own script files and takes
  arguments, and errors are types. Where the owner truly can't express something,
  say why at that spot and pass values as parameters, never spliced in.
- **State each rule once.** A rule the program depends on, such as which records
  belong to the current work or whether an attempt is still current, lives in one
  function at its owner, and every caller asks it. The compiler checks how a value
  is shaped; it cannot see two functions that answer the same question
  differently. A path that rebuilds state after a crash, a retry or a resume calls
  what the live path calls, so the two cannot drift apart. Common misses: the same
  filter written inline in several functions, and recovery that reconstructs what
  the live path already builds.
- **Errors keep their cause.** An error says what went wrong and carries why.
  Keep the original error as its source instead of replacing it with a sentence,
  and choose the words a person sees at the place that shows them. Extend the
  error type that already owns the path rather than adding a looser one.
- **Failure looks like failure.** Ignore a result only when its failure matters
  to no one. A failed action never reads as done, and a failed read never reads
  as empty.
- **Make control flow easy to follow.** Keep related work together and make
  prerequisites, effects and exits explicit. Handle cases that end the operation
  where they become clear, often with an early return, so the remaining work
  reads in sequence. Do the work, then branch on its result; don't fold an
  action into a condition. Keep each decision with the code that owns it. When
  changing one behavior requires reasoning about unrelated state, separate those
  responsibilities so the change can be understood locally. When control flow is
  hard to follow, use cyclomatic complexity to locate branch-heavy functions and
  check whether their decisions belong together.

Move code toward these as you touch it; don't mass-convert just to comply.
State, fields and variants a change adds meet them from the start. When moving
existing state costs more than the change can carry, such as a persisted
format's migration, say so and why in the PR.
Clippy's rules in `[workspace.lints]` hold what a rule can tell right from wrong;
review holds the rest.

## Development workflow

Iterate locally: implement, exercise the real workflow, review, and correct
before handing off a coherent change through a PR. Investigate failures and
verify corrections before pushing another revision. An early draft can help
when collaboration or remote evidence is needed; it should not replace the
local feedback loop.

Keep investigation and correction proportional to credible consequences.
Distinguish serious rare failures from speculative possibilities, and weigh
the complexity a proposed correction would introduce. Stop when the intended
outcome and appropriate proof are complete. Further review rounds need a
concrete change or unresolved concern, not a search for increasingly speculative
edge cases or a desire to obtain a clean verdict.

Continue implementation, verification, review, and accepted corrections within
the agreed task. Resolve routine choices; bring genuinely open product,
architecture, or scope decisions to the user. A request only to discuss, draft,
or review stays within that scope. Reviewers keep source code read-only; the
implementing agent owns corrections. Findings are evidence, not new requirements.
Follow the latest task scope: a later request to review first or wait suspends
earlier implementation authority.

Use the developer's configured connections and repository setup. Preserve
unrelated work and report access or proof gaps honestly. External publication,
messages, merges, deployment, and destructive actions retain the user's approval
requirements. The swarm's reviewers own pull-request review, and their verdicts
are the review of record. The `review-pr` skill gives an operator an independent
assessment, returned in chat with its retained draft; posting that draft on a PR
is an external publication under the rule above. Local review before a PR
exists remains internal.

Report the actual revision or local change, evidence, and remaining limits;
successful checks or a merge alone do not establish the intended outcome.
Evidence is an artifact a reader can inspect, such as a command with its output or
a screenshot, tied to the revision it ran against. The
[verify skills](verification/README.md#verify-skills) name the proof for each
feature they cover. A physician-site change is not done unless
`cargo test -p physician-sample --test site -- --ignored` passed against a
disposable Postgres (`PHYSICIAN_RUNTIME_TEST_DATABASE_URL`); that Chromium test
fails when the document is blank.

Inspect every diff yourself and verify the affected behavior. Obtain independent
review for non-trivial changes, or when requested. Judge by consequences and
uncertainty, not line count; trivial, mechanical, behavior-preserving edits do
not require a separate reviewer. Reconcile findings and verify corrections before
reporting ready. Opening a PR does not itself require a second review; carry
prior findings and evidence forward.

The engineering skills live in [`.agents/skills`](.agents/skills), and each
skill's description says when to use it. Use them as the work calls for them;
the user need not name each one. Skills supply methods and judgment within these
shared development defaults. `.claude/skills` links to them; resolve skill
references from their real repository paths. The [PStack](verification/README.md#pstack)
skills, `statechart` among them, come from the installed plugin and are not copied
here. A skill asks for the checks its change calls for; a whole-feature
qualification is its own agreed, budgeted task.

Two kinds of agent share this repository. The Antigravity swarm owns ticketed
work from start to finish: it authors the change, reviews it, merges it and
records it in Linear. Operator sessions (Claude Code, Codex, or a developer's
own shell) own ad-hoc work: repository hygiene, infrastructure, documentation
and investigations. An operator session that finds ticket-shaped work hands it
to the swarm by posting on the swarm's board, addressed to its coordinator, and
does not start it. Neither side edits a branch the other created; a change
wanted on the other side's branch is a request on its board or in its PR.

## Where work goes

Work at the owner. Before adding anything, find what already owns it and extend
that, rather than building a parallel one beside it. What more than one place
needs is shared, not copied. Add something new only when nothing existing fits,
and put it where the next agent would look for it. When something replaces an
old path, remove the old one. Stay on the task instead of chasing side problems.
The aim is a repository an agent can read cold, where everything has one obvious
home.

The [canonical map](book/src/reading.md) says where each area is documented, and
the [crate map](book/src/crate-map.md) lists every crate and what it owns.

## Documentation

Repository documentation, including the intent specs, states current truth and
direction, so an agent reading it cold understands what is true now and where
things are going. It reads as a description, never a record: no dated history,
change logs, evidence or decision trails, audit notes, or wording like
"previously" or "we changed X to Y." When something changes, edit the text so it
says what's true now; git history keeps how it got there. Say what something is
for and what must hold, and leave choices that belong to its owner open instead
of prescribing them.

Keep individual Linear ticket IDs, links and status mappings out of repository
documentation. Linear owns that tracking; docs use source and document links to
explain responsibilities, implemented behavior and operating procedures. Live
environment state, rollout notes, prices and implementation gaps stay out too;
they go in the PR, the ticket or the system that holds them.

## Product boundaries

Duet and the physician product are two products joined by
[connected care](intent/connected-care.md). Each keeps its own clinical types,
accounts and stored state, and data crosses only through the connection. They
share code only through the [shared crates](book/src/crate-map.md#shared), and the
[boundaries](crates/boundaries/src/lib.rs) tests keep either product from
importing the other's crates.
[The harness spec](intent/harness.md) says what the shared harness owns.

`eval/` inside `submodules/coda-agents` is sealed. Do not read it or place it in prompts, fixtures, or model-visible context.

## Dev shell

`nix develop` in this repository is the shell for the monorepo. It carries the Rust toolchain, the product tools, Doppler, and `act`. For unattended work, run builds, tests and Git commits with `nix develop .#agent --command <command>`; [unattended development](README.md#unattended-development) describes that profile.

On an Apple silicon Mac where `nix` is not on PATH, each checkout or worktree has its own Apple container: run `dev/apple-session.sh up` once, then the same builds, tests and Git commits with `dev/apple-session.sh exec <command>`, and `dev/apple-session.sh down` when the worktree's work is finished. Files are shared with the Mac, except `target/` outside `target/verification`, and `up` prints the Mac address of each server. Push and publish proof from the Mac. [That route](README.md#apple-silicon-mac-without-nix-apple-container) has the setup. Try it before reporting a missing toolchain as a proof gap, and do not skip the pre-commit hook with `--no-verify` because the host lacks one.

Doppler holds every secret value, and GCP Secret Manager holds only a read-only `DOPPLER_TOKEN` scoped to one config. Never copy Doppler values into Secret Manager or the repository. Tests and the sample server need no Doppler account. [Secrets](book/src/release.md#secrets-doppler) has the configs and commands.

## Infrastructure and CI

`infra/gcp` holds the OpenTofu roots; [GCP deployment](book/src/gcp.md) says which ones this workspace applies. Apply a root only from a reviewed saved plan with the operator's authorization; production applies and DNS changes need the user's explicit authorization. Applying a workspace root grants no production authority over the physician, Duet, clinical-evidence or Gym deployments.

Workspace-wide lint rules live in `[workspace.lints]` in the root [`Cargo.toml`](Cargo.toml); each crate opts in with `[lints] workspace = true`, except the generated `duet-wire`. CI and the pre-commit hook run Clippy with warnings denied on the host build and on the web apps' wasm32 build, so a rule's warning fails either.

Choose checks by affected contracts and dependencies; use hosted CI for shared
integration and environment confirmation. A green routine CI run does not
establish browser qualification: run the ignored browser tests locally or
dispatch CI manually. Report missing local capabilities and remaining proof
honestly. [CI](book/src/release.md#continuous-integration) says what each run
selects and checks, and [local CI checks](README.md#local-ci-checks) how to run
them here.
