# Build requirements: unit-2203 (Arthur Roberge, MRN00002203)

Source: synthetic_hospital patient 2203, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself or leaves a hole;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, refills, portal
traffic, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dumps or its
"(diagnosed 2020-01-15)" stamps. The source's "January 2020" stroke is the
2024-06-30 stroke on this timeline.

## Who he is (S)

Arthur Roberge, born 1957-03-07, male, White, not Hispanic or Latino, English,
Amherst MA 01002, Medicare Part A and B. Married; his wife brings him to the ED in
2025 (S) and appears only as the person who calls or accompanies him. Retired postal
clerk. Former smoker, quit 2010, 25 pack-years. Alcohol socially, 1 to 2 drinks a
week. No known drug allergies.

Surgical history: cholecystectomy (2005), inguinal hernia repair (2012).
Family history: father myocardial infarction at 61; mother type 2 diabetes; paternal
grandfather ischemic stroke at 68.

Joined the practice on 2024-05-29 (new patient visit, A1), bringing hypertension,
type 2 diabetes mellitus and hyperlipidemia, on metformin 1000 mg twice daily,
lisinopril 20 mg daily and atorvastatin 40 mg daily (all S), prescribed before by a
prior primary-care office.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-05-29 | practice | new patient visit (A1): history as above; fasting lipid panel drawn today: **TC 190, LDL 100, HDL 45, TG 150** (S: "from one month ago" at the first ED visit); A1c, CMP, urine albumin/creatinine, hepatitis C antibody with it (P). No drug change |
| 2024-06-30 | outside | ED, regional hospital: left-sided weakness and slurred speech starting 2 hours before arrival; taking all three home drugs as prescribed. BP 155/90, HR 78, RR 16. Weakness of left arm and leg, left facial droop, dysarthria. Non-contrast head CT: no hemorrhage. **Acute ischemic stroke diagnosed.** Admitted; discharged home 2024-07-03 (A2). Home medications unchanged |
| 2024-07-10 | practice | post-discharge visit (P): deficits resolved (R2); acute stroke moved to history of stroke without residual deficits; no drug change |
| 2025-02-26 | outside | ED, regional hospital: sudden left-sided weakness and slurred speech starting 2 hours before arrival, last known well 3 hours before arrival; wife reports he takes metformin, lisinopril and atorvastatin as prescribed; prior ischemic stroke June 2024. BP 155/90, HR 78, afebrile. Left facial droop, left arm and leg weakness, dysarthria. **Glucose 180, creatinine 1.2, INR 1.0, platelets 250 (×10^3/µL).** Non-contrast head CT: no hemorrhage. **Recurrent acute ischemic stroke** (R3). Admitted; discharged home 2025-03-01 (A2). Home medications unchanged |
| 2025-03-05 | practice | post-discharge visit (P): deficits resolved (R2); no drug change |
| 2025-12-31 | outside | ED, regional hospital, 10 months after the February stroke: brought by his wife for confusion and slurred speech starting 2 hours before; adherent, no recent illness. T 37.0 °C, BP 160/90, pulse 88, RR 16. Alert, anxious; expressive aphasia and right facial droop. Non-contrast head CT: no hemorrhage or mass effect. **Acute ischemic stroke, third event** (R3). Admitted; discharged home 2026-01-03 (A2). Home medications unchanged at discharge |
| 2026-01-07 | practice | post-discharge visit (P): speech back to baseline, no facial asymmetry (R2); **aspirin 81 mg daily started** (A3) |
| 2026-08-07 | practice | last visit (seed `last_seen`): Medicare annual wellness visit and chronic-care follow-up with labs; resulted before today |

## Resolutions (R): physician to confirm

- **R1.** The source dates the lipid panel "one month ago" at the first ED visit and
  lists no prior primary care. Resolution: it is the fasting panel drawn at his new
  patient visit on 2024-05-29, released before the stroke. The ED record quotes it.
- **R2.** Each ED visit presents with new deficits from a normal baseline, so the
  deficits of each earlier event cannot have persisted. Resolution: deficits resolve
  during each hospital stay or by the post-discharge visit; practice neurological
  exams between events are normal. "History of stroke without residual deficits"
  stands on the problem list between events.
- **R3.** The source's problem lists name only one stroke, "diagnosed" at the first
  encounter, and the 2025-12-31 visit does not name its diagnosis. Resolution: each of
  the three ED visits is a separate acute ischemic stroke (the third with left
  hemisphere signs: expressive aphasia, right facial droop), recorded as diagnosed at
  its ED visit and moved to history afterward. The patient's primary diagnosis stays
  acute ischemic stroke.
- **R4.** The first and second ED visits carry the identical vitals 155/90 and 78.
  Kept as written: two separate measurements that happen to match.
- **Left as the source has it:** no antiplatelet after the first or second stroke
  (each discharge continues the three home drugs unchanged); no brain MRI, vascular
  imaging, echocardiogram or rhythm monitoring at any event; no thrombolysis noted in
  any ED record; atorvastatin stays 40 mg with LDL around 100 and is never
  intensified, even after aspirin is added. No note comments on any of these.

## Additions (A): physician to confirm

- **A1.** The 2024-05-29 new patient visit: the first source encounter is an ED visit
  that quotes a lipid panel from a month before, so he had joined just before.
- **A2.** Each ED visit ends in a short admission (2024-06-30 to 07-03, 2025-02-26 to
  03-01, 2025-12-31 to 2026-01-03), so each source encounter has a discharge. Each
  discharge summary gives: diagnosis acute ischemic stroke, deficits improving, home
  medications continued unchanged, follow up with primary care. No other tests or
  results.
- **A3.** Aspirin 81 mg daily, started at the 2026-01-07 post-discharge visit: the
  standard secondary-prevention step after the source ends, taken at the first
  practice visit after the last stroke.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level here says
otherwise:

- Reports and exams state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as the source or this file gives it.
- Height 178 cm; weight about 91 to 93 kg throughout.
- Office BP on lisinopril runs about 130 to 138 / 78 to 84 between events.
- A1c on metformin alone, without a drug change: 7.3 (2024-05-29), 7.8
  (2024-11-20), 7.2 (2025-05-14), 7.6 (2025-10-01), 7.0 (2026-04-08), 7.3
  (2026-08-07). Practice glucose runs about 130 to 150. Creatinine runs about 1.05 to 1.2 (eGFR mid-60s to high 70s). Urine albumin/creatinine
  normal. LDL about 100 to 110, HDL mid-40s, TG 130 to 160 on later draws.
- Diabetes and blood pressure visits every 3 to 6 months; A1c and a metabolic panel
  at each; lipids and urine albumin yearly; foot exams normal.
- Yearly diabetic eye exam letters from an outside optometrist: no retinopathy.
- Prior primary-care records (received June 2024): screening colonoscopy 2018 with
  no polyps, repeat in 10 years; Tdap 2019; Shingrix series 2021; PCV20 2023.
- Influenza (high-dose) and updated COVID vaccines each fall; hepatitis C antibody
  once, negative. PHQ-2 at wellness visits, negative.
- Refills, pharmacy traffic, portal messages about results and scheduling, one
  missed visit (2025-09-17) and its reschedule.
- Booked visit after 2026-08-27: a routine diabetes and blood pressure follow-up.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002203 on 2026-08-07. Practice contacts after that day are
only messages, refills and outside records filed as received, without changing the
practice's lists. A visit is booked after today.
