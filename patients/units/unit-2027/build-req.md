# Build requirements: unit-2027 (David Donovan, MRN00002027)

Source: synthetic_hospital patient 2027, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text ("you note", "is shown below").

## Who he is (S)

David Donovan, born 1964-02-19, male, White, not Hispanic or Latino, English. Lives
with his wife (she is the only family member who appears in the chart). Project
manager; goes on medical leave in October 2024 (R8). Former smoker, 20 pack-years,
quit in 2014. Alcohol rare, 1 to 2 drinks a month. No known drug allergies. Height
180 cm.

Family history: father with hypertension and an MI at 67; mother with type 2
diabetes and a stroke at 72; one brother with hypertension; no family history of
neurodegenerative disease or carcinoid.

Joined the practice 2021-09-14 (A1, new patient visit). He brought: essential
hypertension (long-standing, variable control) and chronic kidney disease stage 3
from hypertensive nephrosclerosis (named by his prior practice in 2019), on
**lisinopril 10 mg daily** and **amlodipine 5 mg daily**. No surgeries yet.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2021-09-14 | practice | new patient visit (A1): history as above; BP 146/92; weight 86 kg; creatinine 1.74 (eGFR about 44), urine albumin/creatinine 72 mg/g (A12); prior records requested |
| 2021-10-05 | outside | prior practice records arrive: hypertension for years, CKD named 2019, renal ultrasound 2019 with normal-sized kidneys and no hydronephrosis, screening colonoscopy 2019 normal (P) |
| 2021-11-16 | outside | low-dose CT lung screening, Lung-RADS 1 (A11) |
| 2022-01-10 to 2022-01-13 | outside | regional hospital admission: **open appendectomy** through a right lower quadrant incision for acute appendicitis (S: "open appendectomy 2 years ago", R3). The practice learns of it from him at the 2022-02-08 visit; the discharge summary arrives only on request, 2022-04-20 |
| 2022-10-17 | practice | about 2 months of worsening shortness of breath on exertion; BP 158/96; lungs clear; referred to cardiology (A2) |
| 2022-11-04 | outside | cardiology consult (letter and echo report received 2022-11-14): worsening exertional dyspnea; BP 168/96; laterally displaced point of maximal impulse; normal S1 and S2, no murmurs; labs normal (R4). Transthoracic echo: increased LV wall thickness (septum 1.4 cm, posterior wall 1.3 cm), reduced compliance (impaired relaxation), normal chamber size (LVIDd 4.9 cm), LVEF 60%, no valvular disease. **Left ventricular hypertrophy diagnosed.** No drug change (left as source has it) |
| 2022-12-06 | outside | low-dose CT lung screening, Lung-RADS 1 (A11) |
| 2023-04-12 | practice | about 3 months of episodic nausea, abdominal cramping and watery diarrhea, often after meals, sometimes with facial warmth and flushing; 10 lb unintended loss over 2 months (86 kg to 81.5 kg); no travel, antibiotics or sick contacts; CBC normal, CMP with creatinine at baseline; referred to gastroenterology (A3) |
| 2023-05-02 | outside | CT abdomen and pelvis with contrast (ordered by GI): 4.0 cm ileal mass (S); three hypervascular liver lesions, largest 1.8 cm in the right lobe, consistent with metastases (A4) |
| 2023-05-08 | outside | gastroenterology consult (letter received 2023-05-16): history as at 2023-04-12; well-nourished appearance, mild hepatomegaly, stool guaiac negative, no rash; mild niacin deficiency; 24-hour urine 5-HIAA elevated (38 mg/24 h, A4); CT as above. **Carcinoid syndrome from a metastatic well-differentiated neuroendocrine (carcinoid) tumor of the ileum diagnosed** (R1, R2). Referred to surgical oncology and medical oncology. No niacin supplement (left as source has it) |
| 2023-06-19 to 2023-06-24 | outside | **exploratory laparotomy with ileal resection and right hemicolectomy** (R3). The practice learns of it from his wife's phone call 2023-06-26; the operative note and discharge summary never arrive. Surgical pathology (received on request 2023-08-22): well-differentiated neuroendocrine tumor of the ileum, 4.0 cm, grade 1, Ki-67 2%, margins negative, 0 of 14 lymph nodes (A4) |
| 2023-07-06 | practice | post-operative visit: midline incision healing; liver lesions left in place for medical therapy |
| 2023-07-18 | outside | medical oncology starts **octreotide LAR 30 mg IM monthly** (A5), given at the oncology infusion center. The practice learns of it from a specialty pharmacy prior-authorization call; the oncology consult letter arrives 2023-09-12 and the practice list adds octreotide only then |
| 2023-09-19 | practice | follow-up: diarrhea and flushing much less frequent on octreotide (A5) |
| 2023-12-04 | practice | slowly enlarging lower abdominal bulge for about 5 months, more prominent with coughing or straining; abdominal ultrasound ordered, general surgery referral (A6) |
| 2023-12-12 | outside | low-dose CT lung screening, Lung-RADS 1 (A11) |
| 2024-01-10 | outside | abdominal ultrasound: defect in the anterior abdominal wall in the right lower quadrant with protrusion of omental fat (S) |
| 2024-01-24 | outside | general surgery consult (letter received 2024-02-01): 6 months of a slowly enlarging lower abdominal mass, more prominent with cough or strain; soft, non-tender right lower quadrant bulge that enlarges with straining and reduces lying supine; no overlying redness or warmth; history of the open appendectomy 2 years earlier. **Incisional hernia diagnosed.** Reducible and painless: watchful waiting, return for pain or a bulge that will not go back (A6) |
| 2024-06-04 | outside | oncology surveillance CT abdomen and pelvis: post-surgical changes, liver lesions unchanged (largest 1.8 cm), no new lesions (A5) |
| 2024-06-10 | practice | about 6 months of progressive hand weakness (hard to hold objects), tripping over his feet, occasional cramps and twitches in arms and legs; CBC, BMP (electrolytes normal, creatinine at baseline) and TSH normal; creatine kinase 268 U/L, slightly elevated (A7); referred to neurology |
| 2024-07-31 | outside | neurology consult (letter and EMG report received 2024-08-08): weaker grip, frequent tripping, cramps and twitches; atrophy of the intrinsic hand muscles, upper extremity fasciculations, brisk lower extremity reflexes, extensor plantar response on the right, intact sensation, no bowel or bladder dysfunction; reviewed the 2024-06-10 labs. EMG and nerve conduction: acute and chronic denervation. **Amyotrophic lateral sclerosis diagnosed; riluzole 50 mg twice daily started** (A8). Liver tests asked of the practice monthly for 3 months |
| 2024-12-10 | outside | low-dose CT lung screening, Lung-RADS 1 (A11). No later screening scan, without comment |
| 2025-02-07 | outside | neurology follow-up (letter received 2025-02-14): trouble buttoning shirts and climbing stairs; hand atrophy, arm fasciculations, lower extremity hyperreflexia, normal sensation. Spirometry and respiratory muscle testing: FVC 74% predicted, FEV1 75% predicted, FEV1/FVC 0.80 (normal), vital capacity reduced, maximal inspiratory pressure -52 cmH2O and maximal expiratory pressure 68 cmH2O (both reduced), DLCO 88% predicted (normal) (R5) |
| 2025-06-03 | outside | oncology surveillance CT abdomen and pelvis: liver lesions unchanged, no new disease (A5). Report only; no oncology letter that year |
| 2025-07-25 | outside | neurology follow-up: 6 months of progressive trouble swallowing and slurred speech, occasional choking on thin liquids; his wife notes more clumsiness and several stumbles; tongue atrophy and fasciculations, nasal spastic speech, brisk jaw jerk, increased upper extremity tone with hyperreflexia, spastic gait, no sensory deficit, cognition intact; CBC, electrolytes, renal function (at his baseline, R4) and thyroid tests unremarkable. Speech-language pathology swallow evaluation the same day: thin liquids trigger cough, chin-tuck and mildly thick liquids advised; a feeding tube discussed and deferred by him (A9). The practice learns of this visit from his wife at the 2025-08-19 visit; the letter arrives 2025-10-21 after a records request |
| 2026-01-26 | outside | neurology follow-up (letter received 2026-02-03): 6 months of worsening stair climbing and grip, occasional slurred speech; his wife notes shallow breathing, worse at night, and bouts of uncontrolled laughing and crying (left as source has it: no drug started for them); normal sensation, hand atrophy, upper extremity fasciculations, brisk reflexes, extensor plantar responses. Arterial blood gas on room air in the pulmonary function lab (R5): pH 7.42, PaCO2 45, PaO2 70, HCO3 24. FVC 60% predicted. **Nocturnal BiPAP ordered** (A10) |
| 2026-02-10 | practice | follow-up after neurology: BiPAP set up at home; BMP: potassium 5.4, creatinine 1.98 (eGFR about 38). **Hyperkalemia diagnosed**; referred to nephrology for the rising creatinine and potassium (A12) |
| 2026-03-23 | outside | nephrology first consult: a low-potassium diet advised (S: "as advised during his last visit"). No letter ever arrives; he reports the advice by portal message 2026-03-25 |
| 2026-08-10 | outside | nephrology follow-up (letter received 2026-08-18): routine follow-up of CKD stage 3; adherent to lisinopril and amlodipine; on the low-potassium diet; no palpitations and no weakness beyond his known ALS (R6); BP 132/84; examination unremarkable apart from his known ALS findings (R6). Potassium 5.6, creatinine 2.1, eGFR 35, bicarbonate 24, calcium 9.2. Plan in the letter: continue the diet, repeat BMP in 2 weeks, nephrology to decide next steps on that result (A12) |
| 2026-08-24 | practice | BMP drawn for nephrology; pending today |

Booked after today: a primary-care follow-up on 2026-09-14.

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The profile carries every condition, riluzole and octreotide from the first
  (2022) encounter, yet the encounters diagnose LVH in November 2022, carcinoid in
  May 2023, the hernia in January 2024 and ALS in July 2024. Resolution: each
  condition and drug starts at its own event in the timeline. At joining he has only
  hypertension and CKD stage 3. The 2023 gastroenterology note calls him "on
  octreotide" and "returning"; that is dropped, since that visit makes the
  diagnosis. The 2024 surgery note names ALS six months before neurology diagnoses
  it; the surgery letter does not mention ALS.
- **R2.** "Carcinoid syndrome", "Metastatic neuroendocrine tumor" and "enterochromaffin
  cell neoplasm" are one disease. The chart records carcinoid syndrome, a malignant
  carcinoid tumor of the ileum and secondary carcinoid tumors of the liver, all from
  2023-05-08.
- **R3.** Surgical dates: "open appendectomy 2 years prior" (2022 note) and "2 years
  ago" (2024 note), and "exploratory laparotomy with small bowel resection, 18 months
  prior" in a 2022 note, before the tumor was found; the comorbidities say "status
  post right hemicolectomy". Resolution: open appendectomy January 2022; one operation
  in June 2023, an exploratory laparotomy with ileal resection and right hemicolectomy,
  after the May 2023 diagnosis. The hernia sits at the right lower quadrant
  appendectomy incision and is first noticed in mid-2023.
- **R4.** "Laboratory evaluation is normal" (cardiology 2022) and "renal function within
  normal limits" (neurology 2025) in a man with CKD stage 3. Resolution: electrolytes
  and blood counts normal and creatinine at his known baseline; letters say so.
  Creatinine runs 1.7 to 1.8 (eGFR 42 to 46) from 2021 to 2024, 1.85 to 1.95 in 2025,
  1.98 in February 2026, about 2.0 in May 2026 and 2.1 in August 2026 (eGFR 35, the source's value). He is
  stage 3b throughout.
- **R5.** The 2025 spirometry is qualitative in the source; the chart carries the values
  in the timeline, which fit the "decreased/normal" pattern and lead to the 2026 FVC of
  60%. The 2026 arterial gas would not be drawn in an office; it is kept as drawn in
  the hospital pulmonary function lab the same day, since it supports the BiPAP order.
- **R6.** The 2026 nephrology note says he "denies muscle weakness" and that the exam is
  "unremarkable" in a man with ALS. Resolution: no new or different weakness and no
  palpitations; exam without edema, heart and lungs normal, ALS findings unchanged.
  The source's "61-year-old" at that visit follows the birth date: he is 62.
- **R7.** The 2025 neurology note says "He takes hydrochlorothiazide"; no list ever
  carries it and no visit starts it. Not carried.
- **R8.** Occupation "currently on medical leave" and Medicare primary at 58.
  Resolution: he works as a project manager until October 2024, then goes on medical
  leave after the ALS diagnosis; Medicare (disability, through his ALS) becomes
  primary in 2025 with his Blue Cross Blue Shield plan secondary. Before 2025 the Blue
  Cross plan is primary.
- **R9.** "Long-standing, variable control" against "controlled with lisinopril":
  office pressures vary visit to visit, as listed in the levels below. The source's
  168/96 at cardiology and 132/84 at nephrology are carried.
- **Left as the source has it:** lisinopril 10 mg and amlodipine 5 mg never change, not
  after the 168/96 with new LVH and not with the potassium of 5.6; no drug is started
  for the niacin deficiency or for the laughing and crying spells; the hernia is not
  repaired. No note comments on these.

## Additions (A): physician to confirm

- **A1.** The 2021-09-14 new patient visit (the source's first encounter reads as an
  established patient).
- **A2.** The 2022-10-17 visit that sends him to cardiology.
- **A3.** The 2023-04-12 visit that sends him to gastroenterology.
- **A4.** Evidence for "metastatic": the CT's three liver lesions (largest 1.8 cm, right
  lobe); the 24-hour urine 5-HIAA value of 38 mg/24 h; the pathology (grade 1, Ki-67 2%,
  4.0 cm, margins negative, 0 of 14 nodes).
- **A5.** Standard care for metastatic carcinoid: octreotide LAR 30 mg monthly from
  2023-07-18 by medical oncology; symptoms improve; surveillance CT abdomen and pelvis
  2024-06-04 and 2025-06-03, both stable. Oncology visits about every 6 months with
  the monthly injections; only the 2023 consult and a June 2024 follow-up letter arrive.
- **A6.** The 2023-12-04 visit and ultrasound order; the surgeon's watchful-waiting plan.
- **A7.** The 2024-06-10 visit and its labs (the source's "slightly elevated" CK is 268
  U/L), which neurology reviewed.
- **A8.** Riluzole (source home medication) starts at the ALS diagnosis; liver tests
  monthly for 3 months at the practice (normal).
- **A9.** The 2025-07-25 swallow evaluation and the feeding tube discussion.
- **A10.** Nocturnal BiPAP from February 2026 for the FVC of 60% with a PaCO2 of 45.
- **A11.** Lung cancer screening (20 pack-years, quit 2014): low-dose CT 2021-11-16,
  2022-12-06, 2023-12-12 and 2024-12-10, all Lung-RADS 1.
- **A13.** ALS supportive care as the disease advances: the ALS clinic's PT, OT,
  speech therapy and dietitian; a button hook and adaptive utensils (2025), a cane
  outdoors (mid-2025), a walker indoors (late 2025) and a wheelchair for distance
  (2026); a home aide three mornings a week (2026); health care proxy (his wife) in
  August 2024; in May 2026 he wants BiPAP but no tracheostomy or long-term
  ventilation, stays undecided on a feeding tube, and wants to stay home.
- **A12.** Hyperkalemia and nephrology: the 2026-02-10 potassium of 5.4 and creatinine
  1.98, the referral, the March nephrology visit and the repeat BMP drawn 2026-08-24.
  Urine albumin/creatinine runs 60 to 95 mg/g yearly from 2021, as fits hypertensive
  nephrosclerosis.

## Levels

- Weight: 86 kg in 2021 to 2022; 81.5 kg in April 2023; 79 kg after surgery (July
  2023); 80 kg in 2024; 77 kg by August 2025; 74 kg in 2026.
- Office BP: 146/92 (2021), 152/94 (2022, 158/96 at the October visit), 140/86 (2023),
  138/84 (2024), 134/82 (2025), 132/84 (2026).
- Potassium normal (4.3 to 4.8) until 2025; 4.9 in 2025; 5.4 in February 2026; 5.2 in
  May 2026; 5.6 at nephrology in August 2026.
- Blood counts, liver tests, TSH, lipids, glucose normal throughout.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Records from the prior practice may mention routine screening done there, with normal
  results (colonoscopy 2019).
- Hypertension and CKD visits every 3 to 6 months with vitals, BMP; CBC and lipids
  yearly; urine albumin/creatinine yearly.
- Yearly physicals, then from 2025 a Welcome to Medicare visit and annual wellness
  visits; depression screens (PHQ-2/PHQ-9, normal); advance care planning after the ALS diagnosis.
- Prevention: influenza each fall, COVID vaccines, pneumococcal (PCV20, 2021), shingles
  series (2022), RSV (2024, age 60 with chronic conditions), Tdap if due. Hepatitis C
  and HIV screening once.
- Refills, specialty pharmacy and prior-authorization traffic (octreotide, riluzole,
  BiPAP), medical leave and disability paperwork, records requests, portal messages
  about results and scheduling, a missed visit.
- Booked visit after 2026-08-27: primary-care follow-up 2026-09-14.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00002027 on 2026-05-12. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. A visit is booked after today.
