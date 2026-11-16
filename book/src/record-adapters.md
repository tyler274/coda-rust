# Recorded Hint and FHIR adapter verification

This proof starts at acquired source data. Hint sections contain `items`, section
`failed_parts`, and each item's native `body` and `unavailable_parts`. FHIR inputs
are native NDJSON records. Acquisition credentials and recorded sandbox/corpus inputs stay
outside Git. A small synthetic capture in test support guards malformed siblings. The acquisition owner prepares the Hint envelope and applies the
explicit exclusions in <a href="/intent/record-mapping%2Emd">the mapping contract</a>.

Run in the development shell against a disposable Postgres server:

```sh
PHYSICIAN_RUNTIME_TEST_DATABASE_URL=postgresql://postgres@127.0.0.1:5544/postgres \
SOURCE_REPLAY_ROOT=/absolute/path/archived-selection \
HINT_CAPTURE_ROOT=/absolute/path/hint-captures \
scripts/check-record-adapters.sh
```

Input contracts:

- `SOURCE_REPLAY_ROOT`: `hint-bodies.json`, `hint-replayed.json`, `mimic/`,
  `mimic-expected.json` and `input-provenance.json`. The preparation script at
  `crates/physician-sample/tests/support/prepare_recorded_sources.py --help`
  describes its external capture, corpus and research-manifest inputs. It selects
  the archived twelve-section, thirteen-record Hint selection and the fixed
  57 MIMIC research cases plus their patient and referenced supporting records.
- `HINT_CAPTURE_ROOT`: `live-proof.json`, `live-proof-multipage.json`, and
  `worker-0-{0,1,2,3}.json` / `worker-3-{0,1,2,3}.json`. These are acquisition
  receipts and paired reader outputs for the ordinary and repeated-identity
  captures. The adapter tests consume them without starting an extension.

The command assembles a fresh `target/verification/record-adapter-inputs.*`
directory from the four `worker-0-*` captures and the prepared MIMIC selection,
recording input digests for inspection. Input directories are never modified;
readback artifacts go to `target/verification/record-adapters`.

The filer requires this acquired-envelope contract. The checked-in extension's
raw bridge responses are not this envelope; the acquisition lane must supply
it before connecting that producer to the filer. This unit does not implement
that transport or the source exclusion policy.

The app test implements only acquisition from these captures. It invokes the
existing application loader for patient linking, enrollment and transactional
record delivery, then refreshes through authenticated HTTP source-check routes.
It reconnects to Postgres and reads stored evidence through the shared tools.
No test writes source records directly along that app path. Missing inputs fail
the command rather than silently skipping qualification.

The other suites cover mapping precedence, malformed siblings, unavailable
attachments, Other, retained revisions and persisted citation identity. The
citation provider is scripted. Chromium qualifies the baseline physician site;
opening citations in a browser additionally requires the citation UI implementation.
Real-model behavior and that UI integration are separate proof targets.

The bounded Hint appointment window remains a partial acquisition under the
current source contract. The adapter retains that limit; this proof does not
qualify connection health, Chrome reconnect/retry behavior, or hosted transport.
