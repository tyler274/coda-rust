# Authoring a primary-care patient history

You are authoring the complete medical history of a fictional adult patient (or a
small family) in one adult longitudinal primary-care practice in western
Massachusetts: a membership-based internal medicine practice. Membership buys
access (same-day and longer visits, calls and texts that reach the physician);
patients still carry insurance for labs, imaging, specialists, hospitals and
drugs, and most arrive after years at fee-for-service practices whose diagnoses,
claims and records follow them in as outside history. The output is the patient's **source of truth**: a dated ledger of
everything that happened and every record the chart holds. Code translates it
into FHIR R4; you never write FHIR yourself.

The chart should read like a real EHR that a physician would recognize: an
ordinary life with its own pace, uneven visit spacing, notes in many clinical
voices, structured data that agrees with the notes, and the small imperfections
real charts accumulate. It is not an exam vignette, a hospital simulation or a
teaching case. Most patients are ordinary. Rare disease only appears when the
seed asks for it.

The charts are used to watch a physician's assistant work over a practice:
prepare visits, review what is new, carry follow-through. So a chart does not end
tidied up. At the bank's "now" some things are still in motion (a result not yet
back, a referral not yet scheduled, a message from two days ago, an abnormal value
nobody commented on, a visit already booked). What happens next is decided later,
not written here. Most patients, most weeks, need nothing.

Real charts are **dense**. Years of care leave hundreds of records: visit notes,
and around them a steady stream of short ones (nursing and MA notes, phone calls,
portal messages, refill and prior-authorization notes, records requests, care
coordination, patient education, result notes, outside records), vitals at every
in-person contact, and years of monitoring labs. Hit the seed's
`target_documents` and `target_observations`; they were measured from real
reference charts.

## How you work

You have tools. Work in stages, in files, and let the checker catch mistakes.
`<unit>` is the authoring unit id you were given; every command runs from the
repository root.

1. `cargo run -q -p patient-bank -- brief <unit>` prints the seed, the targets
   and the paths you write to.
2. **Story first.** Write `story.md` in the unit's draft folder: who this person
   is and how their life goes over the years; family history and when each item
   became known; a dated problem timeline (onset, recognition, changes,
   resolution); a dated medication timeline (start, dose changes, stops, who
   changed it and why); specialists and outside care; and a year-by-year
   calendar of every contact with the practice and the records each leaves.
   Count the calendar against the targets before writing any events. This file
   is the plan the ledger must agree with.
3. **Ledger in parts.** Write the events in chronological parts of about two to
   three years each: `part-01.json`, `part-02.json`, … (format below). Write one
   part, then the next; re-read the story and the end of the previous part before
   each so state carries forward: current medications, open problems, pending
   follow-ups, the last values of weight and labs.
4. `cargo run -q -p patient-bank -- check <unit>` merges the parts and reports
   errors and counts against the targets. Fix every error by editing the
   parts and re-run until it prints `ACCEPTED`.
5. `cargo run -q -p patient-bank -- preview <unit>` renders the chart as of
   today and writes a chronological reading copy, `chart.txt`, ending with the
   visits already booked. Read it start
   to end the way a physician reviewing the chart would, against "Before you
   finish" at the end of this guide. Fix what you find; then check again.

An independent reviewer then reads the chart against the same list, and the unit
is finished only after their findings are fixed.

## The seed

You receive one seed per patient. It fixes sex, birth year, town, preferred
language, insurance, an occupation hint, complexity, years with the practice,
target document and observation counts, clinical anchors (the main threads of the history),
episodes, sometimes a chart-texture item, and often a life context. Honor every
field. Anchors are the backbone. Episodes and texture are things that happened
somewhere in the history; place them where they make sense, in an order that lets
them all be true of one person (the tendon rupture years before the lung disease
needed oxygen). If one truly cannot fit this person, keep the anchors, leave that
item out and say why in story.md.

Most seeds carry `"coded_anchors": true`. Their anchors are exact catalog labels
and are the patient's problem list: record every one, and do not add chronic
diagnoses, injuries or procedures the seed did not plan. The only problems you
may add are self-limited illnesses (a cold, a bladder infection, COVID-19). A
real list picks up a stray entry, so at most two other problems may still be
active at the end; resolve acute ones when they resolve. The episodes are concrete: write
the injury, finding or result the seed names, not the most typical one. The life
context (a veteran, a Deaf patient who uses an interpreter, a caregiver) shapes
the visits, the social history and how the patient talks; do not make it the
subject of every note. Patients differ from one another; the seed is what makes
this one who they are. Family members in one request
share a household or family and their charts must agree with each other (shared
address when living together, consistent family history, a spouse may be
mentioned in social history, nothing contradictory).

The practice "today" is **2026-08-27T12:00** Eastern, and the chart ends there:
every event and record is strictly before it. Write nothing that happens after
today. Two things reach past it, as they do in a real chart on any day:

- **A result not back yet.** A draw from the last few days whose results are not
  in: mark the draw `"pending": true` and name the tests with no values. The check
  tells you when a draw you gave values for would not be back by today.
- **A visit already booked.** A primary-care visit on the calendar after today is
  an event with `"appointment": "booked"`, its time, its kind and a `reason`, and
  nothing else.

The patient joined the practice about
`years_with_practice` years before that (establish-care visit). Earlier history
reaches the chart through outside records, the establish-care history, or
patient report, never as notes that predate when the practice could have held them,
except outside documents that arrive later (date the outside document by when it
was received, and state the original date inside it).

## Chronology is the hard rule

Every record knows only what was known on its date.

- A note written on a date mentions only diagnoses, medications, results, events
  and family history that existed and were known by then. Never copy a later
  problem list or medication list backward. A 2019 note cannot mention a 2022
  diagnosis; a medication appears in notes only from when it was started (or as
  "discussed" before it was started), and not as current after it was stopped.
- Family history and social history evolve: a parent's later heart attack, a new
  job, quitting smoking, a divorce appear only from when they happened or were
  learned.
- Ages: every age stated in a note body must equal the patient's age on that
  date from the birth date you choose. Prefer not to state ages in the body; the
  note header (added by code) already shows age.
- Results appear in notes only after they were resulted.
- Conditions are recorded on the problem list at the first event that documents
  them; `onset` may be earlier (patient-reported history) but never later than the
  recording event. Resolved problems get a `resolved` action at the event that
  resolves them. A resolved problem that comes back is `diagnosed` again at the
  event that records its return; the problem list shows it as a recurrence.
- Medications start at the event that prescribes them, change by stopping the old
  order and starting a new one, and stop at the event that documents the stop. A
  drug stopped and later restarted is a new `start`, and its next `stop` ends it.

## Encounters and the records they leave

Use these event kinds:

`establish_care`, `office_visit`, `annual_physical`, `telehealth_visit`,
`nurse_visit`, `telephone`, `portal_message`, `lab_only`, `imaging`,
`specialist_visit`, `urgent_care`, `ed_visit`, `hospital_admission`,
`procedure_visit`, `outside_records`, `pharmacy`.

Each event lists only what it produced. A primary-care visit usually has vitals,
a `Progress Notes` document, a CPT visit code, maybe labs and orders. Labs drawn
and resulted at the visit are `labs`; a referral, imaging or test sent elsewhere is
an `orders` item placed at the visit and closed later by a `done` or `cancelled`
item with the same `id` (the referral seen, the scan read, the order dropped), or
left open. Some referrals never get scheduled, and some orders are simply never
done; leave them open or cancel them where the chart would.

A booked visit that did not happen keeps its place on the calendar: give the event
`"appointment": "cancelled"` or `"no-show"`, no clinical records, and at most an
administrative note about it. A
telephone call has a `Telephone Encounter` note and nothing else unless an order
came from it. A portal message has a `Patient Message` (the patient's own words,
first person, informal, sometimes with typos) and often a clinician reply in a
separate `Portal Message Reply` document. Lab-only events have results and maybe a
`Result Note`. An ED visit has an `ED Provider Notes` document and its own vitals
and labs. A hospital admission spans days: H&P (`History and Physical`), daily
`Progress Notes` and `Nursing Notes`, labs, a `Discharge Summary`, then a
primary-care post-discharge follow-up. Specialist visits produce `Consults`
letters or specialty progress notes (`Cardiology Progress Notes`,
`Dermatology Progress Notes`, …). Outside records produce `Outside Records`
documents summarizing what was received.

Between visits the chart keeps moving, and these short documents carry most of
the volume (each 30–150 words, clipped, in the voice of whoever wrote it):

- `Nursing Notes`: MA rooming, injections, BP checks, wound checks, vaccine
  administration, pre-visit planning.
- `Telephone Encounter`: triage calls, callbacks, results by phone, pharmacy calls.
- `Patient Message` / `Portal Message Reply`: questions, refill requests, forms,
  home readings, photos described in words.
- `Administrative Note`: records requests, prior authorizations, insurance
  denials, refill authorizations, form completion (FMLA, work, DMV, school),
  no-shows and rescheduling, chart corrections.
- `Care Coordination Note`: referrals placed and tracked, home health, DME,
  transitions after discharge, social work and community resources.
- `Patient Education`: what was taught and how it landed (inhaler technique,
  glucose meter, diet, a new medication).
- `Pharmacy Notes`: medication reconciliation, interaction checks, formulary
  switches.
- `Result Note`: a clinician's comment on a resulted lab or study and what the
  patient was told.

Use the event kinds that fit (`telephone`, `portal_message`, `nurse_visit`,
`pharmacy`, `outside_records`, `lab_only`); an administrative or coordination
note rides on the event it belongs to, or on a `telephone` event of its own. In
this practice a patient's text or call to the physician is a `telephone` or
`portal_message` event with the physician's short reply.

Not every visit has every vital or lab. Do not run a full panel at every visit.
Order labs the way a primary-care physician would for this patient: annual
lipids for a statin user, A1c every 3–6 months for diabetes, BMP after starting
an ACE inhibitor, TSH after a levothyroxine change, CBC when anemic, urinalysis
for urinary symptoms. Screening follows age and sex (colonoscopy or FIT,
mammography, cervical cytology, lung CT for eligible smokers, DXA, hepatitis C
and HIV once, AAA ultrasound for male ever-smokers 65–75). Immunizations happen
the way they do for real adults: influenza in about half of adults' autumns (more
often after 65, rarely every single year), COVID-19 boosters thinning out after
2022, Tdap every ten years or with a wound, shingles and pneumococcal when due and
accepted. Record them as `immunizations` with a label from the vaccine list in the
vocabulary; a dose given elsewhere (a pharmacy, a prior practice) goes on the
event that documents it, usually `outside_records` or `pharmacy`. Most screening
results are normal, and most PHQ-9s are low.

Visit spacing is irregular: delayed follow-ups, missed years, a flurry around an
illness, telehealth during 2020–2021. Annual visits drift by weeks or months from
year to year, and screening dates slip; nothing recurs like clockwork. Visits fall
on every weekday (the preview shows each date's weekday, and the check rejects a
chart where one weekday carries more than a third of them); patients message and
call any day, weekends included. Roughly one booked visit in twelve is cancelled
or missed, and most patients land in urgent care or call with a minor illness now
and then.

**The office layer.** A real chart is thick with small operational notes, and they
are much of its everyday density: refill authorizations, result routing ("reviewed,
RN to call"), recall letters for overdue screening, records requests and the faxes
that answer them (late, partial or twice), check-in and insurance notes, forms,
prior authorizations, scheduling and reminder calls. Give them to this patient at
the pace their care would generate. The medication list also carries what the
patient really takes: over-the-counter drugs, supplements and supplies, not only
prescriptions.

**Medicine of its time.** Every note practices the medicine of its own date:
the guidelines, screening ages, vaccine recommendations, drug approvals and
labeling, and disease names in force then (for example colorectal screening from
45 only after May 2021, recombinant zoster vaccine from late 2017, MASLD only from
mid-2023, adult RSV vaccine from mid-2023). When unsure of a date, write around it. Visit times are realistic Eastern clock
times (clinic 08:00–17:00 with non-round minutes; ED any hour).

## Notes

Write the **body** of each document only; code adds the title, date, patient,
MRN, age and service header. Every document names its `author` by role: `PCP`,
`covering NP`, `RN`, `MA`, `front desk`, `practice pharmacist`, `cardiology`,
`ED physician`, `radiology`, `patient`, `patient's daughter`. Two people in one
role are told apart the way a chart would ("PCP" and "prior PCP, retired 2021").
Bodies vary by author and type, and most are short:

- Physician/NP progress notes: subjective or interval history, pertinent ROS,
  medications reviewed (only those current then), exam, data reviewed,
  assessment and plan by problem. Most 120–300 words; establish-care, annual and
  decision-point notes longer; a quick recheck 60 words. Exam and ROS are often
  templated and carried forward from the last visit with small edits, as real
  EHRs do.
- Nursing, MA and triage notes: 20–80 words, task-focused, clipped, fragments.
- Telephone encounters and portal threads: short and conversational. A patient
  writes the way they write, greeting no one or by "Hi," and the physician
  answers in a line or two.
- Consult letters: addressed back to the primary-care physician by role ("Thank
  you for referring…"), with the specialist's impression and recommendations.
- Discharge summaries: hospital course, discharge medications (list changes),
  follow-up.
- Outside records: a summary of what was received and from where (generic, e.g.
  "prior primary-care practice in Worcester").

The event's `service` is the department of whoever wrote its documents: a PCP's
result note or portal reply is `Internal Medicine — Primary Care` even when it
discusses an MRI. Notes never describe their own tone or the author's intentions
("explained without lecturing"); they record what happened. A patient's verbal
tic or catchphrase may appear once or twice, not in every note.

Notes are written by busy clinicians, not for a textbook. No guideline citations
or reasoning essays in a progress note ("per USPSTF…", "given the ACC/AHA risk
calculator…"); the plan says what was decided. No em dashes (the check rejects
them), no tidy groups of three, no "notably". Real charts carry small
imperfections and nobody fixes all of them: a med list a visit behind, a problem
list that lags, a self-reported drink count that changes between notes, an
abnormal value no note mentions, a result filed to the wrong visit. Things go
wrong around care the way they do: a prior authorization denied and appealed, a
fax that arrives twice, a message answered three days later, a referral that
never gets booked.

Everything about this patient is their own: their people, jobs, pets, phrases and
mishaps. Invent them fresh from the seed; avoid the first stock choice that comes
to mind.

Vary sentence structure, abbreviations and length across authors. Use real
clinical shorthand where clinicians would. Do not reuse paragraphs between notes;
no two documents may share a body. Copy-forward is allowed in moderation (a
history paragraph carried with small edits), as real EHRs do.

Never name clinicians or staff, real or invented ("Dr. Okafor", "M. Lindqvist,
MD"), and never name real practices, hospitals, labs, pharmacies, stores or EHR
and portal products (say "the portal"). The check rejects a note that names one. Drug, vaccine and assay product names
are fine, as in real charts. Refer
to people by role ("PCP", "cardiology", "the covering NP", "the ED physician")
and to places generically ("the regional hospital", "an urgent care in
Holyoke"). Patient-facing family members may be named with fictional first names.
Never use the words synthetic, generated, fictional, template, simulation, test
patient, or similar in any record text.

## Structured data

Every structured item must be stated by or consistent with the notes of the same
event, and every clinically important fact in a note should have its structured
counterpart (a new diagnosis is a condition action; a prescription is a
medication action; a resulted lab is a lab result).

**Vocabulary.** Use the catalog labels below exactly. Conditions, labs, panels,
surveys, reports, procedures and allergens must come from the catalog. If a
condition the story genuinely needs is missing, use `"label"` with your clinical
wording and add `"icd10"` with the ICD-10-CM code you believe is correct; it will
be verified. Medications should use a catalog RxNorm name exactly; if the drug or
strength you need is missing, give `"drug_text"` (generic name, strength, form)
instead of `"drug"` and it will be looked up.

**Labs.** A panel result lists the panel name and the members you have a reason
to state (anything abnormal or clinically relevant to this patient: A1c, LDL,
creatinine, potassium, TSH, hemoglobin and so on). Unstated members of a panel are
filled with normal values by code, so do not state values merely to be complete,
and never let a stated value contradict the story. Give eGFR only if you want a
specific value; otherwise code calculates it from creatinine (CKD-EPI 2021).
For a CBC, state hemoglobin (and MCV whenever red-cell size matters, as in iron
deficiency or B12 deficiency), WBC and platelets as needed, and only the
differential members you mean: code derives hematocrit, RBC, MCH and MCHC from
hemoglobin and MCV, and fills the unstated differential members so the total is
exactly 100%. Stated differential members must leave room for the rest (in sepsis,
neutrophils up and lymphocytes down). Unstated values vary from draw to draw.

**Result timing.** Results are final when the lab or reader releases them, not
when collected. Code allows about two days for cultures and several days for
pathology and cytology; give `"resulted": "YYYY-MM-DDTHH:MM"` on a lab item, or on one
result when it comes back later than the rest of the draw (an HPV test after the
A1c), or `"issued"` on a report, when the story needs a specific time, such as a
send-out taking a week. Until then the result shows as pending. Notes discuss a
result only after it was released, and a result note follows the release.
Qualitative results use `"text"` (e.g. `"Negative"`, `"No growth"`,
`">100,000 CFU/mL E. coli"`). Units default to the catalog unit.

**Vitals.** Rooming records blood pressure, heart rate, SpO2 and weight at nearly
every in-person visit; temperature and respiratory rate at sick visits and when
the complaint calls for them; height at the first visit and about yearly. Nurse
visits record what was checked. ED visits and admissions record several sets:
give `vitals` as a list of sets each with its own `time` (triage and repeat sets
in the ED; every 4–8 hours on the ward). Values come from automated cuffs and
oximeters: any final digit, odd as often as even (137/83, 71), a spread from
visit to visit, rarely the same pair twice. They follow the illness. The check
rejects a series that is nearly all even or keeps repeating.

**Observation count.** Each vital sign is one observation (blood pressure counts
once); each lab panel is one observation per member plus one for the panel; each
standalone lab, survey and social-history item is one. Reach the target with the
contacts and monitoring this patient would really have, never with filler.

**Allergies.** Record each allergy once at the event that first documents it,
with reaction and severity. "No known drug allergies" is the absence of entries,
not an entry.

**Reports.** Imaging, ECG, echo, PFT, sleep studies, cervical cytology and
surgical pathology are `reports` with the full report text (findings and
impression in the voice of the reading physician) and an optional one-line
`conclusion`.

**Procedures.** Performed interventions (colonoscopy, biopsy, joint injection,
surgery) and visit CPT codes are `procedures`. Every in-person or telehealth
primary-care visit gets one visit code from the catalog: telehealth visits use the
telehealth codes, Medicare patients' wellness visits use the annual wellness visit
or Welcome to Medicare codes, post-discharge visits within 30 days may use
transitional care management, and screening colonoscopy uses the screening codes.

## Ledger format

Each part file is one JSON object of this shape. `part-01.json` carries the
patient fields (`given` through `summary`); later parts carry only `mrn` and
`events`. For a family, every part lists each member who has events in it.

```json
{
  "patients": [
    {
      "mrn": "MRN00000006",
      "given": "…", "family": "…", "birth_date": "YYYY-MM-DD",
      "address_line": "…",
      "phone": "+1-413-555-0123",
      "email": "first.last@example.test",
      "marital_status": "married|single|divorced|widowed|partnered|separated",
      "summary": "Two or three sentences for the authoring record (not shown in the chart).",
      "events": [
        {
          "id": "e01",
          "kind": "establish_care",
          "start": "YYYY-MM-DDTHH:MM",
          "end": "YYYY-MM-DDTHH:MM (admissions only)",
          "service": "Internal Medicine — Primary Care",
          "reason": "short reason for visit",
          "appointment": "booked (after today, nothing else on the event) | cancelled | no-show",
          "conditions": [{"label": "Essential hypertension", "action": "diagnosed|history|resolved", "onset": "YYYY-MM-DD", "note": "optional qualifier"}],
          "medications": [{"drug": "lisinopril 10 MG Oral Tablet", "action": "start|stop|historical", "sig": "Take 1 tablet by mouth daily", "dose": {"value": 10, "unit": "mg"}, "frequency": "DAILY", "route": "Oral", "course": "chronic|short", "reason": "optional"}],
          "allergies": [{"substance": "Penicillin", "reaction": "Hives", "severity": "mild|moderate|severe", "criticality": "low|high", "type": "allergy|intolerance"}],
          "vitals": {"bp": [128, 82], "hr": 74, "rr": 14, "temp_c": 36.8, "spo2": 98, "weight_kg": 81.4, "height_cm": 172.0, "peak_flow": 420},
          "vitals (ED or admission)": [{"time": "YYYY-MM-DDTHH:MM", "bp": [148, 90], "hr": 104, "rr": 20, "temp_c": 38.4, "spo2": 93}, {"time": "…"}],
          "labs": [{"panel": "Lipid Panel", "collected": "YYYY-MM-DDTHH:MM", "results": [{"test": "Low Density Lipoprotein", "value": 162}, {"test": "Total Cholesterol", "value": 241}]},
                   {"panel": null, "collected": "YYYY-MM-DDTHH:MM", "results": [{"test": "Hemoglobin A1c", "value": 7.4}]},
                   {"panel": null, "collected": "YYYY-MM-DDTHH:MM (last few days)", "pending": true, "results": [{"test": "High-risk HPV DNA"}]}],
          "surveys": [{"survey": "PHQ-9 total score", "value": 11}],
          "social": [{"item": "Tobacco smoking status", "value": "Former smoker"}, {"item": "History of alcohol use", "value": "2-3 drinks per week"}],
          "reports": [{"study": "Chest X-ray, 2 views", "performed": "YYYY-MM-DDTHH:MM", "text": "…", "conclusion": "…"}],
          "procedures": [{"procedure": "Office visit, established patient, moderate MDM", "performed": "YYYY-MM-DDTHH:MM"}],
          "immunizations": [{"vaccine": "Influenza, quadrivalent, injectable", "date": "YYYY-MM-DDTHH:MM"}],
          "orders": [{"id": "derm-referral", "action": "placed", "order": "Dermatology referral", "category": "referral|lab|imaging|procedure", "to": "Dermatology", "reason": "changing mole, left shoulder"},
                     {"id": "derm-referral", "action": "done|cancelled", "date": "YYYY-MM-DDTHH:MM (on a later event)"}],
          "documents": [{"type": "Progress Notes", "title": "Primary Care Office Visit", "author": "PCP", "date": "YYYY-MM-DDTHH:MM", "status": "final|preliminary", "body": "…"}]
        }
      ]
    }
  ]
}
```

`medications[].action: "historical"` records a medication the patient took
before joining the practice or that another clinician prescribed (reported, not
ordered here); give it on the event where it was first documented. Short courses
(antibiotics, steroid bursts) use `"course": "short"` and need no stop action.
Times are Eastern wall-clock without offset. Omit empty arrays. Event `id`s are
unique across all parts of a patient and sort chronologically; use the date and a
letter, e.g. `2019-03-14a`. (The key is always `vitals`; the second example only
shows the list form.)

An order's `order` text names what was ordered; an imaging or procedure order may
use the matching report or procedure label from the catalog. Visits on the
practice calendar (establish care, office, annual, telehealth and nurse visits in
primary care) appear as appointments; give `end` when a visit ran long.

Phone numbers use the 413-555-01xx range; emails use `@example.test`; street
addresses are plausible for the seed's town; names fit the patient's background
and generation, and no two patients share a full name.

## Before you finish

Read `chart.txt` as the physician who will inherit this patient, and fix what a
physician would notice:

- **Today.** The chart ends mid-life: something is plausibly open for this
  patient (or, for a quiet patient, nothing is, and that is fine), and nothing is
  written after today but pending results and booked visits.
- **Truth.** No note knows something before its date; notes and structured data
  agree; every result, drug and diagnosis in a note exists in the data.
- **Voice.** Each document sounds like its author. Notes are short unless it is a
  decision point. No teaching voice, no guideline citations, no em dashes, no
  greeting addressed to a role ("Hi the PCP").
- **Texture.** Copy-forward and templated exams, small unreconciled
  discrepancies, some friction, at least one thing nobody followed up.
- **Numbers.** Vitals read like a device produced them; lab trends wobble.
- **This patient.** Nothing generic: the people, events and phrases belong to
  this life, and nothing repeats from note to note.

