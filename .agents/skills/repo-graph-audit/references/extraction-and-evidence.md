# Extraction and evidence

Use the repository's native tools and formats. This is a collector selection
guide, not a required schema or a claim that every repository has these layers.

## Keep the snapshot coherent

For a committed target, Git's tree is the inventory:

```sh
git -C <repo> rev-parse <ref>^{commit}
git -C <repo> ls-tree -r -z --full-tree <commit>
```

Read content from that commit, or from a checkout verified to match it. A file
list from `main` combined with current working-tree content is not a snapshot.
For an explicitly requested working-tree audit, record `git status`, enumerate
tracked plus relevant untracked files, account for deletions, and retain content
hashes for files used as evidence. Commit permalinks do not identify local edits.

Treat Git submodules as pinned external boundaries unless their contents are in
scope. Record symlink targets rather than following links outside the target
silently. Tracked generated and vendor files can be grouped and counted without
deeply reading them; list that limitation. Avoid reading secrets or copying raw
credentials, private records, or large datasets into visualization payloads.

## Choose relationship collectors

| Family | Source to inspect | Common false claim to avoid |
| --- | --- | --- |
| Python imports | AST, actual source roots, relative imports, package metadata | A symbol imported from a module is not necessarily a module of that name. Imports inside functions still exist; imports under type-checking guards do not establish runtime calls. |
| JS/TS dependencies | Parser or existing resolver, tsconfig paths, workspace package exports, bundler config | Regex hits in comments/strings and guessed aliases are not resolved imports. Different export conditions can select different implementations. |
| Rust, Go, Swift, JVM or other languages | Native module/package/build declarations and available tooling | Textual `use`, import, or module names are not a universal filesystem mapping. Report unsupported resolution explicitly. |
| Docs and skills | Links, anchors, command examples, routing and ownership statements | A link to a directory does not prove a workflow edge to every descendant. Code-fence commands are a separate family from prose links. |
| Commands | Package entrypoints, wrappers, task runners, installation manifests | A development command need not be installed in the shipped image or recipient package. A mention is not evidence of execution. |
| Builds and delivery | Docker stages/contexts, copy rules, package includes, exports, image references | Source existing in the repo does not mean it ships. A mutable tag is not an immutable artifact identity. |
| Routes and messages | Registration plus callers, generated clients, schemas and handlers | Literal route or event names are clues; dynamic registration and intermediaries may own the connection. |
| Persistence and artifacts | Writers, schemas, identities, readers, migrations and projections | Same-shaped records do not necessarily have the same authority. Historical snapshots may intentionally differ. |
| Tests, models and CI | Invocations, fixtures, imports, test setup and actual results | A declared test is not a run. A synthetic test or formal model does not automatically exercise production callers. |
| External systems | Adapters, documented contracts, approved readback | A configured integration does not establish current deployment, receipt, acknowledgement or external success. |

For every collector, retain the relation type, source location, target, and
resolution limit. Deduplicate by source, target, relationship and claim; distinct
claims between the same owners remain distinct. Separate membership edges from
dependencies and workflow handoffs so counts do not inflate one another.

Exercise a collector on a few known relationships and a known non-relationship
before trusting its total. Inspect unresolved internal targets rather than
discarding them. List unhandled syntax/configuration and do not count it as
covered. Where no reliable resolver is available, use candidate extraction plus
manual source verification and say so.

## Keep enough data to audit a handoff

Tables or transient JSON are both suitable. A practical record contains:

- **Owner:** ID, name, responsibility, source references, primary file membership,
  and any historical/generated/external classification.
- **Structural edge:** source, target, relation, declaration location, and whether
  it is resolved, declared external, or unresolved.
- **Workflow edge:** owner endpoints, relation, claim, artifact/configuration or
  decision crossing it, identity constraints, disposition, rationale, and evidence.
- **Finding:** affected edges, concrete consequence, supporting sources, the
  existing owner to reconcile, proposed correction, and needed verification.

For an expected but absent edge, retain the source that promises it and what
search or execution failed to find. Do not manufacture a callee reference.
For current local evidence, use path and line references. For a committed
snapshot with an accessible remote, use revision-pinned source links. For a
test result, identify its inputs/setup and inspected result as well as its code.

## Traverse the full handoff

Investigate ownership and truth at transitions, not just inside files:

```text
user or operator intent
  -> instructions and configuration
  -> declared entrypoint and installed implementation
  -> admitted inputs and runtime action
  -> authoritative persisted result
  -> reader, presentation, export or external consumer
  -> acknowledgement, correction or recovery
  -> the owner allowed to change the source
```

Adapt this to the actual product. Follow relevant failure and revision paths,
including partial results and old saved data. An intentionally forbidden arrow
should be marked as a boundary rather than proposed as a feature.

Useful cross-checks include:

- A guide invokes a tool that the target installation lacks: reconcile the
  command with that installation or correct its capability promise.
- A writer omits a role's value and a reader substitutes a neighboring role:
  trace the selection through persisted state; keep missing historical truth
  unknown instead of reconstructing it from a convenient proxy.
- Feedback goes to a derived artifact instead of the source owner: distinguish
  an interpretation change, a source correction, and a mechanical mapping defect.
- Tests validate a manifest's internal consistency while a guide promises a
  different interface: inspect the installed workflow, not just the manifest test.

These are investigation patterns, not preloaded findings. A repository may
legitimately have different installations, independent roles, historical formats,
or multiple outputs. Explain the actual violated contract before recommending work.

## Presentation integrity

Validate node/edge IDs, endpoints, membership accounting, source paths and line
bounds, and counts by relationship and disposition before rendering. Those checks
verify the map's structure, not the correctness of its judgments.

Do not turn unresolved dependencies into a green "all clear" because a renderer
cannot display them. Keep uninspected owners and external boundaries visible.
Escape source text as data, preserve the pinned revision, and avoid embedding
full source contents when a short claim and source reference suffice.
