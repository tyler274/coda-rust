# Authoring a primary-care patient history

You are writing the medical history of one fictional adult in an adult
primary-care practice in western Massachusetts. It is a membership internal
medicine practice: patients can call or text the physician and get same-day visits,
and they still use insurance for labs, imaging, specialists and drugs. Most arrive
after years at other practices, so their earlier care comes in as outside records.

What you write is a dated **ledger**: every event in the patient's care and every
record it left. Code turns the ledger into FHIR R4. You never write FHIR.

The chart should read like a real EHR. Use what you know about real care: busy
clinicians, real doses, plausible lab trends, uneven visit spacing, notes in many
voices, and the small mess real charts collect. Most patients are ordinary.

## How you work

Every patient comes from one patient of synthetic_hospital, a fully synthetic dataset
built from exam vignettes. `source.md` in the unit folder is that patient: a profile
and a few encounters, mostly at specialists or the ED. `build-req.md` says how they
become our practice's patient: their identity, a fixed timeline, how the source's
contradictions are resolved, the few things added, and the primary-care layer that
may surround them. **build-req.md is the authority for every clinical fact.** Run
every command from the repository root. `<unit>` is the one you were given.

Every turn costs the whole conversation again, so take few of them: make
independent calls together in one turn, write each file whole in one call, and
search for several labels in one command.

1. Read source.md, `data/build-req-guide.md` and `taken.md` together, then write
   build-req.md as the guide says. `cargo run -q -p patient-bank -- brief <unit>`
   prints the seed and the paths.
2. Write `story.md`: one line per contact in date order, each tagged with the
   build-req letter it rests on (S, R1, A2, P).
3. Write the part files (`part-01.json`, `part-02.json`, ...), a few years each, with
   the documents in them. Look up every label you need in the vocabulary file in
   one or two searches (don't read it end to end), and avoid the names, phrases and
   values `taken.md` lists.
4. `check`, fix every error it lists in one pass, and check again until it prints
   `ACCEPTED`. Then `preview` and read, in `chart.txt`, the problem and medication
   lists and the records of each source encounter, as the physician who will
   inherit this patient. Fix what that physician would not believe and anything not
   traceable to build-req.md, and check again.

## What may be in the chart

Every diagnosis, medication, dose, result, finding, procedure, referral outcome,
family member and life event traces to build-req.md: **S** the source as build-req
carries it, **R** a listed resolution, **A** a listed addition, **P** the
primary-care layer. Nothing else. Texture is yours: how each writer sounds,
scheduling, refills, portal traffic, admin, missed visits, the unevenness of real
notes. Texture carries no new clinical fact.

- **Re-voice everything.** The source notes are vignettes. Never copy a sentence
  (eight words in a row is a copy), a source clinician's name, its problem-list
  dumps, its "(diagnosed ...)" stamps or its quiz text. A year the source text
  states ("seen in 2022 and 2024") follows the re-dated timeline.
- **Carry build-req's numbers as written.** Every other number comes from code.
- **Outside care** (consults, ED, hospital, urgent care) reaches the chart as the
  letters and reports the practice receives, each in its own place's voice, and
  carries the patient's known allergies like any letter.
- **Reports and exams** state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as build-req gives it: the descriptors
  its kind of report always carries are fine, but never an added territory,
  severity grade or symptom.
- **Gaps stay gaps.** Where the source leaves care undone (a drug never raised, an
  imaging study never ordered), nobody fills it and nobody remarks on it, unless
  build-req says otherwise.
- **The last visit.** The practice last saw the patient on the seed's `last_seen`:
  that day has a held visit, and no practice visit comes after it. After it come
  only messages, refills, a draw ordered at that visit, and outside records filed
  as received, without updating the practice's lists.

## Today

The practice's today is **2026-08-27T12:00** Eastern. The chart ends there:
everything is dated before it. Only two things reach past today:

- **A result not back yet.** A draw from the last two weeks that has not come back:
  mark it `"pending": true` and name the tests with no values.
- **A visit already booked.** Only when the seed's `booked` is true: a primary-care
  visit after today is an event with `"appointment": "booked"`, its time, kind and
  `reason`, and nothing else.

What happens after today is decided later, not written here. The chart stops
mid-life, not at a turning point. Most patients have nothing open today; some
have one thing (an open referral, a recent message, a value nobody has commented
on yet).

The patient joins the practice when build-req says. Earlier history comes in
through outside records, the establish-care history or patient report. An outside
document is dated when it was received, with the original date inside it.

## A life, not a plot

- Between the source's events, most of the chart is routine primary care.
- Personal detail appears only where the writer would record it, not as color.
- Mistakes go unexplained. Nobody writes down where an error came from, and many
  are never caught.
- Clinicians make ordinary decisions, and some are not the best ones.
- Values wobble from draw to draw and visit to visit, with no pattern that
  repeats every year.

## Chronology

Every record knows only what was known on its date.

- A note mentions only diagnoses, drugs, results and family history that existed
  and were known by then. Never copy a later problem or medication list backward.
- Results appear in notes only after they were released.
- A condition is recorded at the first event that documents it. `onset` may be
  earlier, never later. A problem resolves with a `resolved` action, and one that
  comes back is `diagnosed` again.
- A drug starts at the event that prescribes it and stops at the event that stops
  it. A dose change stops the old order and starts a new one. A restart is a new
  `start`.
- An order is `placed` once, then closed later by `done` or `cancelled` with the
  same `id`, or left open. It orders one thing, by its vocabulary label: a lab or
  panel, a study, a procedure; a referral names its specialty in `to`. Two tests
  are two orders, and when it is due belongs in the note. Asking another office
  for records is not an order; the request goes in the note or a letter, and the
  records arrive as `outside_records`.
- Every event and record falls after birth.

## Events

Event kinds: `establish_care`, `office_visit`, `annual_physical`,
`telehealth_visit`, `nurse_visit`, `telephone`, `portal_message`, `lab_only`,
`imaging`, `specialist_visit`, `urgent_care`, `ed_visit`, `hospital_admission`,
`procedure_visit`, `outside_records`, `pharmacy`.

Each event lists only what it produced. A visit has a note. A visit that was
cancelled or missed keeps its place with `"appointment": "cancelled"` or
`"no-show"`, and records nothing clinical. Between visits the chart keeps moving:
phone calls, portal messages, refills, prior authorizations, records requests,
result notes, nurse and MA notes. Each document has a `type`, an `author` role
(`PCP`, `RN`, `MA`, `front desk`, `cardiology`, `patient`, …) and a body. Code
adds the header. Refer to clinicians by role, never by name.

## Structured data

Notes and structured data agree. A new diagnosis in a note is a condition action,
a prescription is a medication action, and a resulted lab is a lab result.

- **Codes.** Use the catalog labels in the vocabulary file exactly. For anything
  it lacks, give the standard code and check verifies it: a condition's `icd10`, a
  drug's `drug_text` (generic name, strength and form), a lab result's `loinc` and
  `unit`, a report's `loinc`, an allergen's or procedure's `snomed`. Look codes up
  with `patient-bank find icd10|loinc|snomed|rxnorm WORDS`, never from memory. An
  allergen the catalog lacks also gives its `category` (food, medication,
  environment or biologic). "No known drug allergies" is an allergen that holds
  until a drug allergy is recorded; never record it while one is listed.
- **Numbers come from code.** You record where the patient's body is, and code
  measures it. Give an event a `state` when a level is first known or changes:
  weight and height at the first visit, a blood pressure level when it is not
  normal, a lab level when a condition or a drug moves it (an A1c of 7.9 at
  diagnosis, 7.1 three months into metformin; a pressure of 148/94 untreated, 134/84
  on amlodipine). A level stays until a later state changes it, and a level you
  never set is the patient's own normal. Give `measure` on each contact that takes
  vitals, and name the tests in each draw without values. Code reads each one the
  way a cuff, a scale or an analyzer would, with its usual scatter. Write a value
  yourself for every number build-req gives (code would scatter it) and for what no
  level can give: a qualitative result (`"text"`), a culture, a one-off number.
- **Labs.** A panel lists only the members you mean; code fills the rest and
  derives eGFR, CBC indices and the differential's remainder. A calculated test you
  state (total cholesterol, non-HDL, eGFR, transferrin saturation, PT) fixes the
  input you leave out, so give the test or its inputs; check rejects a set no
  lab's formula gives. Give `"resulted"` (on
  the draw or one result) or `"issued"` (on a report) when the story needs a
  specific release time.
- **Vitals.** `measure` the vitals the contact would take. When build-req gives a
  reading, write it in `vitals` and `measure` the rest: code fills only what you left
  out. An ED visit or admission may give `vitals` as a list of timed sets with values
  you write.
- **Procedures.** Every primary-care visit gets a visit code from the catalog.

## Ledger format

The first year's part carries the patient fields. Later parts carry only `mrn`
and `events`. Times are Eastern wall-clock with no offset. Leave out empty arrays.
Event ids are unique across a patient's parts and sort by date (`2019-03-14a`).

```json
{
  "patients": [
    {
      "mrn": "MRN00000006",
      "given": "…", "family": "…", "birth_date": "YYYY-MM-DD",
      "address_line": "…", "phone": "+1-413-555-0123", "email": "first.last@example.test",
      "marital_status": "married|single|divorced|widowed|partnered|separated",
      "summary": "Two or three sentences for the authoring record (not shown in the chart).",
      "events": [
        {
          "id": "e01",
          "kind": "establish_care",
          "start": "YYYY-MM-DDTHH:MM",
          "end": "YYYY-MM-DDTHH:MM (admissions, or a visit that ran long)",
          "service": "Internal Medicine — Primary Care",
          "reason": "short reason for visit",
          "appointment": "booked (after today, nothing else on the event) | cancelled | no-show",
          "conditions": [{"label": "Essential hypertension", "action": "diagnosed|history|resolved", "onset": "YYYY-MM-DD", "note": "optional"}],
          "medications": [{"drug": "lisinopril 10 MG Oral Tablet", "action": "start|stop|historical", "sig": "Take 1 tablet by mouth daily", "dose": {"value": 10, "unit": "mg"}, "frequency": "DAILY", "route": "Oral", "course": "chronic|short", "reason": "optional"}],
          "allergies": [{"substance": "Penicillin", "reaction": "Hives", "severity": "mild|moderate|severe", "criticality": "low|high", "type": "allergy|intolerance"},
                        {"substance": "Dust mites", "snomed": "260147004 (one the catalog lacks)", "category": "food|medication|environment|biologic"}],
          "state": {"bp": [146, 92], "weight_kg": 88.4, "height_cm": 177.0, "labs": {"Low Density Lipoprotein": 162}},
          "measure": ["bp", "hr", "spo2", "weight_kg"],
          "labs": [{"panel": "Lipid Panel", "collected": "YYYY-MM-DDTHH:MM", "results": [{"test": "Low Density Lipoprotein"}]},
                   {"panel": null, "collected": "YYYY-MM-DDTHH:MM (last two weeks)", "pending": true, "results": [{"test": "Hemoglobin A1c"}]}],
          "surveys": [{"survey": "PHQ-9 total score", "value": 4}],
          "social": [{"item": "Tobacco smoking status", "value": "Former smoker"}],
          "reports": [{"study": "Chest X-ray, 2 views", "performed": "YYYY-MM-DDTHH:MM", "text": "…", "conclusion": "…"}],
          "procedures": [{"procedure": "Office visit, established patient, moderate MDM", "performed": "YYYY-MM-DDTHH:MM"},
                         {"procedure": "Cesarean section", "snomed": "11466000 (one the catalog lacks)", "performed": "YYYY-MM-DDTHH:MM"}],
          "immunizations": [{"vaccine": "Influenza, quadrivalent, injectable", "date": "YYYY-MM-DDTHH:MM"}],
          "orders": [{"id": "derm-referral", "action": "placed", "order": "Dermatology referral", "category": "referral", "to": "Dermatology (a referral specialty)", "reason": "changing mole"},
                     {"id": "fit", "action": "placed", "order": "Fecal Immunochemical Test (one lab, panel, study or procedure label)", "category": "lab|imaging|procedure"},
                     {"id": "derm-referral", "action": "done|cancelled", "date": "YYYY-MM-DDTHH:MM (on a later event)"}],
          "documents": [{"type": "Progress Notes", "title": "Primary Care Office Visit", "author": "PCP", "date": "YYYY-MM-DDTHH:MM", "body": "…"}]
        }
      ]
    }
  ]
}
```

`"historical"` records a drug the patient was already taking or that someone else
prescribed. A short course (`"course": "short"`) needs no stop. An ED visit or
admission may give `vitals` as a list of sets, each with its own `time`.

Phone numbers use 555-01xx, and emails use `@example.test`. Never name real
practices, hospitals, labs or pharmacies; drug and vaccine product names are fine.
