# Build requirements: unit-2129 (Andre Medeiros, MRN00002129)

Source: synthetic_hospital patient 2129, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, refills, portal
traffic, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dumps, its
"(diagnosed 2020-01-15)" stamps, or its quiz text (the clinical trial and the
LPS/cytokine "analysis").

## Who he is (S)

Andre Medeiros, born 1985-09-03, male, Black, not Hispanic, English-speaking, lives in
Amherst (01002), commercial PPO (Blue Cross Blue Shield). High school mathematics
teacher. Never smoker; social alcohol, 1 to 2 drinks on weekends. No known drug
allergies until October 2025 (see R5). No surgical history. Family history: father
with type 2 diabetes and hypertension; mother with seasonal allergies and
hypothyroidism; a paternal uncle with end-stage renal disease of unknown cause.

Joined the practice in May 2021 (A1; new-patient visit 2021-05-20, age 35). He
brought seasonal allergic rhinitis (pollen, spring and fall) and mild acne vulgaris,
and cetirizine 10 mg by mouth daily as needed for the rhinitis. Height 182 cm,
weight about 84 kg.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2021-05-20 | practice | new-patient visit (A1): history as above; normal exam; screening labs (P) |
| 2022-09-12 | practice | telehealth visit for an upper respiratory infection (cough, congestion); supportive care, no antibiotic (A2). He is treated "three weeks" before the nephrology visit |
| 2022-09-26 | practice | about 7 weeks of fatigue and swelling of both legs; URI resolved; 2+ pitting edema both legs, puffiness around the eyes; urinalysis 4+ protein; CMP and lipids drawn (low albumin, high cholesterol, creatinine normal); **urgent nephrology referral** (A3) |
| 2022-10-03 | outside | nephrology consult letter: 2-month history of fatigue and bilateral leg swelling; respiratory infection treated three weeks ago; vital signs normal; periorbital edema and 2+ pitting edema of the lower extremities; urinalysis 4+ protein with oval fat bodies under polarized light; albumin 2.5 g/dL, total cholesterol 300 mg/dL, creatinine 1.0 mg/dL; urine protein/creatinine 6.8 g/g (A4). Nephrotic syndrome; kidney biopsy arranged |
| 2022-10-13 | outside | percutaneous kidney biopsy; pathology: scattered segmental sclerosis and hyalinosis involving some glomeruli; immunofluorescence negative for immunoglobulins and complement; no deposits. **Focal segmental glomerulosclerosis diagnosed** (pathology report received 2022-10-24) |
| 2022-10-24 | outside | nephrology follow-up letter: FSGS explained; **prednisone 60 mg daily started**, **losartan 25 mg daily started**, **furosemide 20 mg daily started** for edema (A5) |
| 2022-10-31 | practice | follow-up after the diagnosis; steroid counseling; flu vaccine (P) |
| 2022-12-12 | outside | nephrology letter: edema resolved, weight back down; **furosemide stopped**; urine protein/creatinine 2.1 g/g (A5) |
| 2023-02-13 | outside | nephrology letter: complete remission, urine protein/creatinine 0.18 g/g, albumin 3.9 g/dL (A5); **prednisone 60 mg stopped and a taper started at 40 mg daily** (prednisone 10 mg tablets), down by 5 to 10 mg every few weeks |
| 2023-08-14 | outside | nephrology letter: **prednisone taper finished**; remission holds; total cholesterol back to normal range (A5) |
| 2023-09-18 | outside | nephrology letter: remission holds, BP normal, **losartan stopped** (A5); yearly nephrology follow-up from here |
| 2024-03-26 | practice | telehealth visit: upper respiratory infection, supportive care (A6) |
| 2024-04-01 | outside | ED record (age 38): fever and malaise after the recent URI, history of FSGS; temperature 38.7 C, heart rate 112 (A7); WBC 13.6 K/uL, C-reactive protein 87 mg/L (A7); urinalysis without protein; chest x-ray clear; blood cultures drawn. **Systemic inflammatory response syndrome** after a viral respiratory infection; IV fluids and acetaminophen; home the same night. ED addendum received 2024-04-08: blood cultures no growth at 5 days |
| 2024-04-04 | practice | post-ED visit: afebrile, feels better; CBC and CRP rechecked, normal; **SIRS resolved** (P) |
| 2024-09-16 | outside | nephrology yearly letter: remission holds (P) |
| 2025-09-08 | outside | nephrology yearly letter: remission holds (P) |
| 2025-10-01 | outside | urgent care note (received 2025-10-03): about a week of feeling unwell with low-grade fevers, body aches and nasal congestion; upper respiratory illness treated as possible sinusitis; **amoxicillin 500 mg three times daily for 7 days started** (R4) |
| 2025-10-09 | outside | admission to an intensive care unit at a regional hospital (age 40), discharge 2025-10-30, summary received 2025-11-03: redness and irritation of face and chest beginning a few days after the urgent care visit, then tender spreading dusky patches with blisters and peeling; severe mouth pain, burning swollen eyes, trouble drinking, painful urination. Febrile 38.9 C, tachycardic 124 (A8); distressed; dusky erythematous patches with flaccid bullae and sloughing on the face and upper trunk, Nikolsky sign present; oral erosions with hemorrhagic crusting; similar lesions around the eyes and genitals. Mild leukopenia (WBC 3.3 K/uL) and elevated liver enzymes (ALT 118, AST 96 U/L) (A8); chest radiograph unremarkable. Skin biopsy: full-thickness epidermal necrosis with sparse lymphocytic infiltrate. **Toxic epidermal necrolysis**, attributed to amoxicillin (R4): **amoxicillin stopped**; supportive ICU care, wound care, fluids, pain control, nutrition, ophthalmology involved for the eyes; creatinine stayed at his baseline; liver enzymes normal by discharge (A8). **Amoxicillin allergy (TEN, severe) recorded** |
| 2025-11-05 | practice | post-discharge visit: skin mostly re-epithelialized, eating normally, eyes comfortable; allergy reviewed, wallet card; CMP and CBC normal (P) |
| 2025-11-19 | outside | ophthalmology letter: eyes healed, no scarring (P) |
| 2025-12-03 | outside | dermatology letter: skin healed; avoid amoxicillin and other penicillins (P) |
| 2026-01-02 | practice | last practice visit: follow-up after TEN; skin fully healed; TEN moved to history (resolved); flu vaccine; CMP, CBC, urinalysis and urine albumin/creatinine drawn, all normal (P) |
| 2026-03-09 | outside | nephrology yearly letter (filed after last_seen): remission holds after the hospital stay (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source's encounter dates and the years in its text ("FSGS (2020)",
  "SIRS (2021)") follow the practice dates: FSGS 2022, SIRS 2024. Stated ages (37,
  38, 40) match the birth date on those dates.
- **R2.** The source lists FSGS and SIRS as active problems later on. Resolution: FSGS
  stays on the list as a chronic diagnosis in remission from February 2023; SIRS is a
  one-time event in April 2024, resolved at the post-ED visit, and appears afterward
  only as past history.
- **R3.** The 2024 ED note's clinical trial of an anti-inflammatory drug and its
  LPS/cytokine "analysis" are quiz text: dropped. No trial, no study drug. The
  "elevated inflammatory markers" are carried as the CRP and WBC in A7.
- **R4.** The source says only that urgent care prescribed "a short course of oral
  treatment" before the TEN. Resolution: the urgent care prescribed amoxicillin
  500 mg three times daily; the hospital team attributes the TEN to it. It is the one
  drug started in the window; cetirizine he had taken for years.
- **R5.** The source lists NKDA even at the TEN admission. Resolution: NKDA through
  the admission; the amoxicillin allergy (TEN, severe, high criticality) is recorded
  from the discharge summary on. Outside letters after that carry it.
- **R6.** The source lists only cetirizine at the 2024 and 2025 encounters, though
  FSGS was treated. Resolution: the FSGS drugs (A5) were all stopped by September
  2023, so cetirizine is his only drug in 2024 and 2025, as the source has it.
- **Left as the source has it:** mild acne is never treated. No statin is started
  for the nephrotic-range cholesterol of 300 (it normalizes with remission). Nobody
  comments on either.

## Additions (A): physician to confirm

- **A1.** The 2021 new-patient visit (five years with the practice).
- **A2.** The 2022-09-12 telehealth visit that "treated" the respiratory infection three
  weeks before the nephrology visit.
- **A3.** The 2022-09-26 practice visit that finds the proteinuria and sends him to
  nephrology.
- **A4.** Urine protein/creatinine 6.8 g/g at the nephrology consult, quantifying the
  4+ protein before the biopsy.
- **A5.** Standard FSGS treatment by nephrology: prednisone 60 mg daily from
  2022-10-24 until remission, then a taper finished 2023-08-14; losartan 25 mg daily
  2022-10-24 to 2023-09-18; furosemide 20 mg daily 2022-10-24 to 2022-12-12. Urine
  protein/creatinine 2.1 g/g in December 2022 and 0.18 g/g in February 2023 with
  albumin 3.9 g/dL (complete remission). After that, urine protein/creatinine stays
  below 0.2 g/g, albumin, creatinine and cholesterol normal, blood pressure normal.
- **A6.** The 2024-03-26 telehealth visit for the URI that precedes the ED visit.
- **A7.** ED numbers that carry the SIRS: temperature 38.7 C, heart rate 112, WBC
  13.6 K/uL, CRP 87 mg/L; blood cultures negative.
- **A8.** ICU numbers that carry "febrile, tachycardic", "mild leukopenia" and
  "elevated liver enzymes": temperature 38.9 C, heart rate 124, WBC 3.3 K/uL, ALT 118
  and AST 96 U/L; normal liver enzymes and baseline creatinine by discharge.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries (a biopsy's core count and needle
  gauge, an imaging study's technique), but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine care done there, with normal results.
- Yearly physicals (2022, 2023, 2024, 2025) with vitals, PHQ-2 or PHQ-9 (negative), CMP,
  CBC, lipids, urinalysis; HIV and hepatitis C screening once (negative) in 2021.
  Weight about 84 to 86 kg, about 88 kg with the nephrotic edema in autumn 2022.
  Blood pressure normal throughout. Glucose normal, including on prednisone.
- Allergic rhinitis flares in spring and fall on cetirizine as needed; cetirizine
  refills.
- Influenza vaccine each fall (none in fall 2025 until January 2026, after the
  admission); COVID vaccines (primary series before joining, updated doses at a
  pharmacy).
- Refills, pharmacy traffic, portal messages about results and scheduling, a
  cancelled physical, reminders, records requests.
- Booked visit after 2026-08-27: a yearly physical in September 2026.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002129 on 2026-01-02. Practice contacts after that day do not
happen except messages, refills and results of that day's draw; outside care after it
still does, and its records are filed as received without changing the practice's lists.
A visit is booked after today.
