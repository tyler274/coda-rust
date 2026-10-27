# Build requirements: unit-1906 (Lori Simard, MRN00001906)

Source: synthetic_hospital patient 1906, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself or stops short;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, refills, portal
traffic, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dumps, its
"(diagnosed ...)" stamps, or its unfinished assessment line.

## Who she is (S)

Born 1995-09-16, female, White, not Hispanic or Latino, English-speaking, Private PPO
(Blue Cross Blue Shield). High school mathematics teacher. Never smoker; social alcohol,
1 to 2 glasses of wine a week. No known drug allergies. Surgical history: tonsillectomy
at age 12. Family history: mother with hypothyroidism, father with hypertension; no
family history of pituitary adenoma or endocrine neoplasia. No chronic conditions and
no medications before the timeline. Height 165 cm, weight about 63 kg, blood pressure
normal throughout.

Joined the practice on 2025-08-19 (A1), a new patient moving her care from a prior
practice in the eastern part of the state. She brings no problems and no medications.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2025-08-19 | practice | new-patient visit (A1): history as above; reports no period since about April and some milky nipple discharge; she already has a gynecology appointment booked for it and the PCP defers the workup to that visit (R1). Urine pregnancy test negative. Baseline CBC, CMP, lipids, HIV and hepatitis C screening (P), all normal |
| 2025-09-05 | outside | prior practice records received (P): routine care only, Pap at 27 normal, tonsillectomy history |
| 2025-10-09 | practice | nurse visit: influenza and updated COVID vaccines (P) |
| 2025-10-28 | outside | gynecology consult (letter received 2025-11-03): 6 months of amenorrhea and galactorrhea; gradual loss of peripheral vision, worse on the right; no headache, dizziness or weight change. Visual field testing suggests bitemporal hemianopsia; neurologic exam otherwise normal. Prolactin 250 ng/mL (ref 4-23), TSH 2.5 mIU/L, FSH 7.2 mIU/mL, LH 6.8 mIU/mL. MRI of the brain with contrast ordered; referral to endocrinology (R2) |
| 2025-11-06 | outside | MRI brain with contrast (report received same day): 1.5 cm pituitary mass with suprasellar extension. **Prolactinoma** (pituitary macroadenoma) recorded on the practice problem list when the endocrinology letter arrives (R2) |
| 2025-11-07 | practice | PCP telephone call with the patient about the MRI result; endocrinology appointment confirmed (P) |
| 2025-11-20 | outside | endocrinology consult (A2; letter received 2025-11-24): macroprolactinoma with chiasmal involvement; **cabergoline 0.25 mg (half of a 0.5 mg tablet) twice weekly started**; repeat prolactin in about 2 to 3 months and repeat MRI in about 6 months |
| 2026-02-10 | outside | endocrinology follow-up (A3; letter received 2026-02-12): prolactin 140 ng/mL on 2026-02-03; galactorrhea less but present; confrontation fields unchanged. **Cabergoline increased to 0.5 mg twice weekly** |
| 2026-03-24 | practice | portal message: patient asks to schedule her physical after the school term's exams (P) |
| 2026-05-28 | practice | **last practice visit**, preventive visit (P): on cabergoline 0.5 mg twice weekly; periods have not returned, some nipple discharge persists; occasional dull headaches over the past few months; endocrinology follow-up with repeat MRI planned for summer; told to call or seek care for worse headache or vision. Exam normal apart from her known field deficit as reported by endocrinology. PHQ-9 2. Pap with HPV cotesting collected (normal, NILM, HPV negative, resulted 2026-06-03) |
| 2026-05-28 | practice | same day, PCP faxes a note to endocrinology about the new headaches, asking whether the MRI should come sooner (P) |
| 2026-06-30 | outside | ED visit (record received 2026-07-02): frequent dull headaches and progressive visual changes over several months, eight months after the gynecology evaluation; spontaneous milky discharge from both nipples, not pregnant or breastfeeding. Neurologic exam normal except peripheral visual field deficits; bilateral galactorrhea. Prolactin markedly elevated at 131 ng/mL (R3); urine pregnancy test negative; BMP and CBC normal. MRI brain: 1.3 cm sellar mass compressing the optic chiasm, no hemorrhage (R4). Discharged home with prompt endocrinology follow-up (A4) |
| 2026-07-01 | practice | portal message from the patient telling the PCP about the ED visit; PCP replies and asks for the record (P) |
| 2026-07-14 | outside | endocrinology follow-up (A5; letter received 2026-07-16): incomplete response to cabergoline with persistent chiasmal compression; **cabergoline increased to 1 mg twice weekly**; formal visual field testing with ophthalmology ordered; referred to neurosurgery |
| 2026-08-11 | outside | neurosurgery consult (A6; letter received 2026-08-14): reviews the June MRI; recommends endoscopic endonasal transsphenoidal resection given chiasmal compression with visual symptoms despite cabergoline; she wants time to consider and to see the formal field results; return visit to decide. Nothing is scheduled in the chart |
| 2026-09-10 | practice | booked visit (after today): follow-up after the ED visit and neurosurgery consult |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Encounter 0 says amenorrhea and galactorrhea for 6 months by late October, so
  they began about April 2025 and were present at joining. The PCP does not run the
  hyperprolactinemia workup because a gynecology visit was already booked; only a
  urine pregnancy test is done. Vision is not discussed at the August visit.
- **R2.** The gynecology assessment stops mid-sentence. Resolution: the plan is the MRI
  and an endocrinology referral. "Prolactinoma" enters the practice list from the
  endocrinology letter of 2025-11-24, with onset 2025-11-06 (the MRI). The source's
  "(diagnosed 2020-01-15)" stamp is not carried.
- **R3.** The ED gives prolactin only as "markedly elevated". Resolution: 131 ng/mL, on
  cabergoline, against the 250 at diagnosis and 140 in February.
- **R4.** The mass is 1.5 cm in November 2025 and 1.3 cm in June 2026. Resolution: partial
  shrinkage on cabergoline; the chiasm is still compressed. Visual symptoms are the
  patient's report of progression; the ED exam carries only "peripheral visual field
  deficits", no new grading.
- Encounter 0 denies headache; the ED reports headaches over several months. Both stand:
  headaches began in early 2026 and are first mentioned at the 2026-05-28 physical.
- **Left as the source has it:** the source gives no formal perimetry; the chart has
  none resulted (the July 2026 order is still open). No other pituitary hormones
  beyond TSH, FSH and LH are measured, and nobody remarks on it.

## Additions (A): physician to confirm

- **A1.** New-patient visit 2025-08-19 (years with the practice: 1; she is a patient
  before encounter 0).
- **A2.** Endocrinology consult starting cabergoline: standard first-line therapy for a
  macroprolactinoma with a field defect, needed because the source asserts the
  diagnosis and never treats it.
- **A3.** Endocrinology follow-up with prolactin 140 ng/mL and the dose increase to
  0.5 mg twice weekly.
- **A4.** ED disposition (the source's ED note has no plan): home, endocrinology follow-up.
- **A5.** Endocrinology follow-up after the ED visit: cabergoline 1 mg twice weekly,
  ophthalmology formal fields ordered, neurosurgery referral. The practice list keeps
  0.5 mg twice weekly (after last seen, nothing is reconciled).
- **A6.** Neurosurgery consult recommending transsphenoidal resection, decision pending.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from a prior practice may mention routine screening done there, with normal results.
- Baseline labs at joining (CBC, CMP, lipids, HIV and hepatitis C screening), normal.
- Cervical screening at 30 with Pap and HPV cotesting, normal. Depression screen normal.
- Influenza and COVID vaccines in fall 2025.
- Portal messages about scheduling and records, records requests, front desk notes.
- Booked visit after 2026-08-27: follow-up after the ED and neurosurgery visits.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001906 on 2026-05-28. Practice contacts after that day do not
happen beyond messages; outside care after it still does, and its records are filed as
received without changing the practice's lists. A visit is booked after today.
