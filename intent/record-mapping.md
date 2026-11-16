# Source record mapping

The [physician spec](physician.md#where-codas-information-comes-from) owns the sixteen buckets and evidence guarantees. These rules interpret the source's declared record identity. They do not infer clinical meaning from prose or values, normalize the whole chart, or establish that a recorded intervention occurred.

## Shared filing

A source record keeps its native family and ID within its connection. Its bucket is independent of that identity. Native content, source versions, named statuses, dates, codes, relationships and unfamiliar fields remain readable. Indexed metadata helps find evidence; it does not stand for every fact in the record.

Unknown kinds and unresolved classifications go to Other. Known kinds retain unfamiliar fields. A safely identifiable record does not need an exhaustive field schema to be admitted. Malformed items, missing identities and contradictory patient bindings are reported individually while usable siblings proceed. Whole-source acquisition failures remain source failures.

A completed acquisition refreshes the source even when retained attachment content cannot be decoded or fetched. Those content limits stay on the record; they do not make the source pull partial or prevent missing-record detection. Failed items, incomplete collections and omitted detail reads remain acquisition failures and produce partial delivery.

Unreadable attachments do not discard their parents or other attachments. Reads expose the retained body, available decoded parts and unavailable part paths with reasons. A saved URL does not mean its target was read. A list response does not mean a full detail response was acquired. Contained resources remain within the native parent; references to other records retain their original targets and do not imply successful acquisition.

The common filer owns retained revisions. A changed native body, filing metadata or decoded part creates a new retained revision of the same source record. Its native-content hash remains separate. The source's explicit version is preserved; where absent, the content hash supplies the envelope's source-content version. Earlier observations and citations keep their original retained revision.

## FHIR R4

Native identity is `resourceType` plus `id` within the actual source connection. `meta.versionId` is the source version. The acquisition owner verifies explicit patient identities against the requested chart. Patient-scoped acquisition supplies scope for records without an explicit patient reference.

| Resource family | Bucket |
|---|---|
| Patient | Patient |
| Condition | Problems |
| MedicationRequest, MedicationStatement, MedicationDispense, MedicationAdministration | Medications |
| AllergyIntolerance | Allergies |
| DiagnosticReport | Results |
| Encounter, Appointment | Visits |
| Procedure | Procedures |
| Immunization | Immunizations |
| ServiceRequest | Orders |
| FamilyMemberHistory | History |
| QuestionnaireResponse | Forms |
| Communication | Messages |
| Other resource families | Other |

Observation classification uses `http://terminology.hl7.org/CodeSystem/observation-category`:

- `laboratory` and `imaging` select Results; `vital-signs` selects Vitals; `social-history` selects History.
- Multiple recognized declarations that agree select that bucket regardless of array order. Recognized declarations selecting different buckets select Other.
- With no category declarations, the exact `http://loinc.org` codes `9279-1`, `8867-4`, `8310-5`, `8302-2`, `9843-4`, `29463-7`, `39156-5`, `85353-1`, `85354-9`, `8480-6` and `8462-4` select Vitals. `2823-3` selects Results. Contradictory supported coded identities select Other.
- `2708-6` alone does not select a bucket: oxygen saturation can occur in laboratory or bedside contexts. A recognized category supplies that context.
- Survey, other broad categories, unknown namespaces and unsupported coded identities select Other unless another declared mapping resolves them. A category named survey does not by itself establish a filled form. Text labels never substitute for a missing coding system.

DocumentReference selects Notes for `clinical-note` in `http://hl7.org/fhir/us/core/CodeSystem/us-core-documentreference-category`. Otherwise it selects Documents. MIME type, filename and decoded prose do not select a clinical bucket.

An order remains an order even when its code also identifies a result. Not-done procedures, not-done immunizations, stopped medications and refuted diagnoses retain their family and explicit source state. The bucket alone never establishes completion, current treatment or a confirmed problem.

Scalar dates are indexed with their declared role and field name. Structured timing, partial precision, components, value choices, contained resources, additional status fields and all category/coding alternatives remain in the native record. A missing scalar date or top-level observation value does not mean the source lacks timing or measurements.

## Hint Chrome responses

Hint acquisition uses the physician's Chrome session. The acquired browser responses govern these mappings; public API documentation does not define a different integration or supply unobserved field rules.

| Acquired family | Native ID | Bucket and indexed source meaning |
|---|---|---|
| patient | `id` | Patient; native name, `dob`, and independently retained sex/gender fields |
| diagnoses | `ident` | Problems; native `name` and `icd10CodeId.code`/`.description` |
| allergies | `ident` | Allergies; native name, start/end fields and `icd10CodeId` |
| medications | `ident` | Medications; `productName`, `status`, `start` and `end` |
| social-history | `ident` | History; parent question and all nested answers remain together |
| family-history | `ident` | History; native diagnosis and `icd10Code`; relatives' diagnoses do not become patient Problems |
| vaccines | `ident` | Immunizations; native name and date retained; date has no indexed clinical role without an established source meaning |
| notes | `noteId` | `noteType=NOTE` selects Notes; `noteType=VITALS` selects Vitals; unsupported subtypes select Other |
| freeform-medications | Unqualified | Populated items report unavailable identity; an empty collection supplies no identity contract |
| surgeries, general, tasks | `ident` | Other; preserve native content without inferring a performed procedure, problem or clinical order |
| appointments | `ident` | Other; retain the scheduled source event, attendees and start/end fields without inferring an encounter |
| files | `Key` | Other; patient-scoped native file metadata, with file-body limits stated |

Patient IDs supplied by items must match the requested patient. A collection with no established item identity contract reports those items as unavailable; it cannot manufacture identities. An empty collection does not establish its populated schema.

Notes retain nested `data`, `noteVersion`, `recordVersion`, status and timing fields distinctly. `data.text` remains readable native evidence. `eventTimestamp` and `recordCreationTimestamp` retain their separate roles. No relationship between the two version fields is inferred; retained content versions identify the evidence. Changing a note subtype never changes its `noteId` identity.

Codes are indexed under their observed source field without guessing an absent coding namespace. Lookup identifiers remain distinct from clinical code values. Uncertain date semantics retain the native field/value instead of acquiring an invented clinical role.

The acquisition contract supplies note and task pages, full notes identified by `noteId`, and the four task statuses exposed by Hint. The filer consumes the resulting sections; Chrome transport and scheduling belong to acquisition. Missing sections, failed pages and unavailable note details remain explicit through partial delivery. Appointment acquisition follows the native chart date window from January 2019 through two years from today, with that scope stated; it does not imply all-time coverage. Recurring occurrence identities remain unqualified and are reported as a coverage limit. File metadata and native file references do not imply that file bodies were read. Successful items stay usable. Repeated native identities within one capture retain the first acquired representation and make collection coverage incomplete; they never prevent other records from being delivered. The acquisition owner applies exclusions once, before filing: the named patient fields, the in-house Rx `data.costToDoctor`, `data.priceForPatient`, `data.invoiceId` and `data.chargeItemId`, and appointment `invoice_ident`. They never recursively delete clinical fields with the same names. Other medication fields remain native evidence; an exclusion requires an observed source field with an established nonclinical meaning. The retained representation is the admitted native JSON after this source policy, not a claim to untouched HTTP bytes.

Patient billing exclusions name the observed payment fields and membership-payment relationships: `billing_address`, `autopay`, `credit_expires_soon`, `credit_in_cents`, `exempt_from_credit_card_fees`, `has_payment_method`, `overdue_in_cents`, `owed_in_cents`, `request_pay_info_sent_at`, `should_charge_credit_card_fees`, `stripe_link`, `uses_billing_address`, `payer` and `payees`. Opaque fields such as `default_source` remain native evidence unless their meaning establishes an exclusion. Medication `start`/`end` fields and allergy, social-history and general `startDate`/`endDate` fields select neutral Start/End index roles. Their native values retain their format and precision; these roles do not establish an intervention date or assert that an intervention occurred.

Notes, tasks and appointments link to their native records. Other Hint families link to the patient chart; a chart link is not a record-specific destination. Unsupported note kinds, including lab, document and Rx subtypes, retain their native bodies in Other until their clinical mapping is explicitly qualified.
