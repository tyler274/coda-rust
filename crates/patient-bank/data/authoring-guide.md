# Authoring a primary-care patient history

You are writing the medical history of one fictional adult (or a small family)
in an adult primary-care practice in western Massachusetts. It is a membership
internal medicine practice: patients can call or text the physician and get
same-day visits, and they still use insurance for labs, imaging, specialists and
drugs. Most arrive after years at other practices, so their earlier care comes
in as outside records.

What you write is a dated **ledger**: every event in the patient's care and every
record it left. Code turns the ledger into FHIR R4. You never write FHIR.

The chart should read like a real EHR. Use what you know about real care: busy
clinicians, real doses, plausible lab trends, uneven visit spacing, notes in many
voices, and the small mess real charts collect. Most patients are ordinary.

## How you work

Run every command from the repository root. `<unit>` is the unit id you were given.

1. `cargo run -q -p patient-bank -- brief <unit>` prints the seed, the document
   and observation targets, and the paths you write to. It also writes
   `taken.md`: the names, phrases and lab values other charts already use. Give
   this patient their own.
2. Write `story.md` in the draft folder: who this person is, a dated problem
   timeline, a dated medication timeline, outside care, and a year-by-year list
   of contacts with the practice. The ledger must agree with it.
3. Write the events in chronological part files of two to three years each
   (`part-01.json`, `part-02.json`, …). Before each part, reread the story and the
   end of the last part, so current drugs, open problems and last values carry
   forward.
4. `cargo run -q -p patient-bank -- check <unit>` merges the parts and lists
   errors. Fix them and rerun until it prints `ACCEPTED`.
5. `cargo run -q -p patient-bank -- preview <unit>` writes `chart.txt`, the chart
   as of today, oldest first. Read it start to end as the physician inheriting
   this patient, fix what that physician would not believe, and check again.

The targets say how much a life like this leaves in a chart. Reach them with
the contacts this patient would really have, not with filler.

## The seed

The seed fixes sex, birth date, name, town, insurance, an occupation, complexity,
years with the practice and the main clinical threads (anchors), plus episodes
and life context. Honor it. Place each episode where it fits this person; if one
cannot be true of them, leave it out and say why in story.md. Family members
authored together share a household or family, and their charts must agree.

## Today

The practice's today is **2026-08-27T12:00** Eastern. The chart ends there:
everything is dated before it. Only two things reach past today:

- **A result not back yet.** A draw from the last two weeks that has not come back:
  mark it `"pending": true` and name the tests with no values.
- **A visit already booked.** A primary-care visit after today is an event with
  `"appointment": "booked"`, its time, kind and `reason`, and nothing else.

What happens after today is decided later, not written here. The chart stops
mid-life, not at a turning point. Most patients have nothing open today; some
have one thing (an open referral, a recent message, a value nobody has commented
on yet).

The patient joined the practice about `years_with_practice` years before today.
Earlier history comes in through outside records, the establish-care history or
patient report. An outside document is dated when it was received, with the
original date inside it.

## A life, not a plot

- Most years are routine. A life has no arc and no resolution, and most patients
  have a dramatic event once or twice in a decade, at no particular spacing.
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
  same `id`, or left open.
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

- **Codes.** Use the catalog labels in the vocabulary file exactly. For a
  condition the catalog lacks, give your `label` and an `icd10` code. For a drug
  the catalog lacks, give `drug_text` (generic name, strength and form). Both are
  verified at check.
- **Labs.** State the panel and only the members you mean. Code fills the rest
  with normal values, calculates eGFR from creatinine, and derives hematocrit, RBC,
  MCH and MCHC from hemoglobin and MCV. A stated differential must leave room for
  the members you did not state. Qualitative results use `"text"`. Give
  `"resulted"` (on the draw or on one result) or `"issued"` (on a report) when the
  story needs a specific release time.
- **Vitals.** Give the vitals a visit would take. Give ED visits and admissions a
  list of timed sets.
- **Procedures.** Every primary-care visit gets a visit code from the catalog.

## Ledger format

`part-01.json` carries the patient fields. Later parts carry only `mrn` and
`events`. Times are Eastern wall-clock with no offset. Leave out empty arrays.
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
          "allergies": [{"substance": "Penicillin", "reaction": "Hives", "severity": "mild|moderate|severe", "criticality": "low|high", "type": "allergy|intolerance"}],
          "vitals": {"bp": [128, 82], "hr": 74, "rr": 14, "temp_c": 36.8, "spo2": 98, "weight_kg": 81.4, "height_cm": 172.0},
          "labs": [{"panel": "Lipid Panel", "collected": "YYYY-MM-DDTHH:MM", "results": [{"test": "Low Density Lipoprotein", "value": 162}]},
                   {"panel": null, "collected": "YYYY-MM-DDTHH:MM (last two weeks)", "pending": true, "results": [{"test": "Hemoglobin A1c"}]}],
          "surveys": [{"survey": "PHQ-9 total score", "value": 4}],
          "social": [{"item": "Tobacco smoking status", "value": "Former smoker"}],
          "reports": [{"study": "Chest X-ray, 2 views", "performed": "YYYY-MM-DDTHH:MM", "text": "…", "conclusion": "…"}],
          "procedures": [{"procedure": "Office visit, established patient, moderate MDM", "performed": "YYYY-MM-DDTHH:MM"}],
          "immunizations": [{"vaccine": "Influenza, quadrivalent, injectable", "date": "YYYY-MM-DDTHH:MM"}],
          "orders": [{"id": "derm-referral", "action": "placed", "order": "Dermatology referral", "category": "referral|lab|imaging|procedure", "to": "Dermatology", "reason": "changing mole"},
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

Phone numbers use 413-555-01xx, and emails use `@example.test`. Never name real
practices, hospitals, labs or pharmacies; drug and vaccine product names are fine.
