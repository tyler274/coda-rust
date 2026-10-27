# Build requirements: unit-2823 (Beverly Fischer, MRN00002823)

Source: synthetic_hospital patient 2823, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text ("considers an additional agent...",
"started on a medication that is taken orally with water...").

## Who she is (S)

Born 1954-02-03, female, White, not Hispanic or Latino, English, Medicare with a
supplemental private plan. Retired administrative assistant. Married; from late 2024
she is caring for an ailing husband (S, ED 2025; his illness is never named). Never
smoker. Occasional wine, 1 to 2 glasses a week. No known drug allergies.

Surgical history: total abdominal hysterectomy at age 52 (2006) for uterine fibroids.
Open reduction and internal fixation of a low-trauma left distal radius fracture, which
happens in January 2024 (R3), not before.

Family history: mother type 2 diabetes and osteoporosis; father hypertension and COPD;
maternal aunt hip fracture from osteoporosis.

**Joins the practice 2019-09-10** (A1, new-patient visit), transferring from a prior
practice. She brings: type 2 diabetes, long-standing and poorly controlled;
hypertension; rheumatoid arthritis; COPD, severe; chronic kidney disease stage 3b,
progressive; GERD; status post hysterectomy. Medications brought:
metformin 1000 mg twice daily, glyburide 5 mg daily, lisinopril 20 mg daily,
amlodipine 5 mg daily, hydrochlorothiazide 12.5 mg daily, omeprazole 20 mg daily,
prednisone 10 mg daily (for RA and COPD), methotrexate 15 mg weekly (six 2.5 mg
tablets), fluticasone-salmeterol 250/50 one inhalation twice daily, albuterol 90 mcg
inhaler as needed, aspirin 81 mg daily, calcium carbonate 600 mg three times daily,
cholecalciferol 2000 units daily. Alendronate is not yet started (R2). Osteoporosis is
not yet diagnosed (R2).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2019-09-10 | practice | **A1** new-patient visit; history as above; height 160 cm, weight 73 kg; BP 138/82; labs drawn (CMP, CBC, A1c, lipids, urine albumin/creatinine, hepatitis C antibody). Referred to nephrology (A2) and to an outside rheumatology group to continue RA care (A3) |
| 2019-09-24 | outside records | prior practice records (A4, P): spirometry 2017, post-bronchodilator FEV1/FVC 0.56, FEV1 41% predicted (severe obstruction); screening colonoscopy 2016 normal, repeat in 10 years; PCV13 2018; Tdap 2015; screening mammogram 2018 normal |
| 2019-10-30 | outside | **A2** nephrology consult letter: CKD stage 3b from diabetes and hypertension, creatinine 1.35, urine albumin/creatinine 210 mg/g; no medication change; yearly follow-up |
| 2019-11-19 | outside | **A3** rheumatology letter: seropositive RA, stable on methotrexate 15 mg weekly and prednisone 10 mg daily; no change |
| 2020-02-17 | practice | annual wellness visit and diabetes visit (S enc 0): diabetes poorly controlled, medication reconciliation. The physician raises adding a DPP-4 inhibitor; she prefers to work on diet first and it is **not started** (R1). Height loss noted; **DXA ordered** |
| 2020-04-28 | outside imaging | DXA lumbar spine and hip: **T-score −2.8**, the lowest site lumbar spine; osteoporosis |
| 2020-05-15 | practice | bone density follow-up (S enc 1): she has had no fractures (R3); about 5 cm (2 in) of height lost over several years; calcium, phosphorus, intact PTH and alkaline phosphatase all normal. **Osteoporosis diagnosed. Alendronate 70 mg weekly started** with the usual instructions (empty stomach, full glass of water, upright 30 minutes) |
| 2021-01-12 | practice | **P** visit for fatigue, generalized weakness and bone pain; normocytic anemia on CBC (hemoglobin 10.8); creatinine 1.6. Referred to endocrinology |
| 2021-02-18 | outside | endocrinology consult (S enc 2): ongoing fatigue, weakness, bone pain despite alendronate; mild lower-leg edema; normocytic anemia; reduced bone density on the 2020 DXA. Creatinine 1.6 (R4); intact PTH 186 pg/mL, calcium 9.3, phosphorus 4.6 (A5). Her nephrologist's view, quoted: hyperparathyroidism from her kidney disease. **Secondary hyperparathyroidism of renal origin** and **anemia in chronic kidney disease** diagnosed (R4). No medication change; continue cholecalciferol and calcium; nephrology to follow PTH |
| 2021-04-19 to 2021-04-28 | outside hospital | **COPD exacerbation, intubated** (S enc 3, "hospitalized twice ... requiring intubation"; R5). Intubated 3 days, intravenous steroids and antibiotics, SARS-CoV-2 PCR negative; discharged on home regimen unchanged |
| 2021-11-08 to 2021-11-18 | outside hospital | **second COPD exacerbation, intubated** (S, R5). Intubated 2 days; same treatment; discharged on home regimen unchanged |
| 2022-02-14 | outside | endocrinology follow-up one year later (S enc 3): progressive fatigue; proximal weakness (stairs, rising from a chair, combing hair); weight gain; fragile skin, easy bruising; wide violaceous abdominal striae. BP **164/92**, pulse **86**, BMI **31**. Rounded face, supraclavicular fullness, dorsocervical fat pad, thin extremities with proximal muscle wasting. No change in appetite or sleep, no fever, chest pain or palpitations. Assessment: glucocorticoid excess from long-term prednisone; **exogenous glucocorticoid use** recorded. Letter recommends discussing steroid-sparing options with rheumatology; prednisone stays 10 mg (Left as the source has it) |
| 2023-01-23 | practice | **A6** portal message asking for a gynecology referral for a private problem; referral placed |
| 2023-02-15 | outside | gynecology consult (S enc 4): months of vulvar itching and irritation, discomfort sitting and walking, a discolored spot slowly enlarging; delayed care from embarrassment; no discharge, bleeding or recent sexual activity; last Pap more than ten years ago (post-hysterectomy); hypertension well controlled. Exam: 1.5 cm pale thickened plaque, slightly irregular borders, right labia majora; no inguinal lymphadenopathy. **Punch biopsy taken** (A7) |
| 2023-02-21 | outside | **A7** biopsy pathology: invasive keratinizing squamous cell carcinoma. **Vulvar squamous cell carcinoma diagnosed**; referred to gynecologic oncology |
| 2023-03-06 | outside | **A7** gynecologic oncology consult: plan radical local excision and right inguinal sentinel lymph node biopsy |
| 2023-03-21 | outside | **A7** radical local excision of the right vulvar lesion and right inguinal sentinel node biopsy. Pathology: invasive SCC, 1.5 cm, depth of invasion 2.1 mm, no lymphovascular invasion, margins negative (closest 11 mm), 0 of 2 sentinel nodes involved; FIGO stage IB. No adjuvant therapy; surveillance every 6 months |
| 2023-09-18, 2024-03-18, 2024-09-16, 2025-09-22, 2026-06-15 | outside | **A7** gynecologic oncology surveillance letters: no evidence of recurrence each time |
| 2024-01-08 | outside ED | **R3** fall from standing on an icy driveway onto the outstretched left hand; displaced left distal radius fracture on X-ray; splinted; orthopedics referral |
| 2024-01-12 | outside | **R3** open reduction and internal fixation, left distal radius, volar plate |
| 2024-02-13 | outside imaging | **R3** repeat DXA: lumbar spine **T-score −2.8** (S enc 5) |
| 2024-02-26 | practice | osteoporosis follow-up after the fragility fracture (S enc 5): BP **138/84**; exam unremarkable apart from the healing left wrist; BMI not 24 (R6). **Postmenopausal osteoporosis with fragility fracture** replaces osteoporosis on the list. Plan: continue alendronate, calcium and vitamin D; fall-prevention advice (the source's unfinished plan, R7) |
| 2024-03-25 | outside | **R3** orthopedics follow-up letter: fracture healed, hardware in good position, discharged from follow-up |
| 2025-02-24 | outside ED | herpes zoster (S enc 6): three days of a painful vesicular rash on the left flank, began as red bumps; stress caring for her ailing husband; age 71. Vitals normal. Band of erythematous papules and vesicles on an erythematous base along left T8, not crossing the midline, no other skin findings. HIV antibody negative, fasting glucose **148**, CBC normal, ESR **20** mm/hr. **Herpes zoster, left T8** diagnosed; **valacyclovir 1 g every 12 hours for 7 days** (renal dosing) (A8) |
| 2025-03-04 | practice | **P** post-ED visit: lesions crusting, pain easing; zoster resolves |
| 2026-02-25 to 2026-03-17 | outside hospital, ICU | sepsis (S enc 7; A9): two days of high fever and chills, then confusion and severe hypotension; age 72. Arrival: temperature 39.4 °C, BP 78/42, heart rate 124, respiratory rate 26, SpO2 92% on room air. Cool extremities, delayed capillary refill, dry mucous membranes. Creatinine 3.9, BUN 68, lactate 5.2, WBC 19.6. Admitted to the ICU: intravenous fluids, broad-spectrum antibiotics (vancomycin and piperacillin-tazobactam), norepinephrine, stress-dose hydrocortisone (A9). Blood and urine cultures without growth; chest X-ray without consolidation; no source identified. Mental status clears. Urine output stays low and creatinine keeps rising; **anuric** by day 4. Renal ultrasound: normal-sized kidneys, no hydronephrosis. **Renal biopsy** (2026-03-03): diffuse necrosis of the renal cortex. **Diffuse cortical necrosis** diagnosed. **Hemodialysis started** (A9) on day 5 through a temporary femoral catheter, then a tunneled right internal jugular catheter (2026-03-10); no renal recovery; discharged on in-center hemodialysis three times weekly. Discharge medication changes (A9): **metformin, glyburide, lisinopril, hydrochlorothiazide, methotrexate and alendronate stopped; insulin glargine 8 units at bedtime started**; prednisone back to 10 mg daily; others continued |
| 2026-03-24 | practice | **P** post-discharge visit: list reconciled to the discharge changes; **end-stage renal disease on hemodialysis** replaces CKD 3b (A9); sepsis resolved |
| 2026-07-08 | practice | last visit (seed `last_seen`): diabetes and post-hospital follow-up; A1c drawn |

## Levels (code reads these; numbers as written)

- Height 160 cm from 2019 (stated height loss is history).
- Weight 73 kg in 2019, rising through 2021 (about 76 in July, 77.6 in November) to about 79 kg (BMI 31) by early 2022, about 77 kg from
  2023 to 2025, about 71 kg after the 2026 admission.
- BP about 138/82 on her three drugs through 2021; about 146/86 at the practice in
  March 2022 after the endocrinology reading, back to about 136/82 by mid-2022; 138/84
  in February 2024; about 126/74 after the 2026 admission off lisinopril and HCTZ.
- A1c: 8.9 (2019), 9.1 (February 2020), 8.6 (2021), 9.3 (2022), 8.8 (2023), 8.7
  (2024), 8.9 (2025); on glargine after the admission 8.3 (March 2026), 7.9 (July 2026).
- Creatinine 1.35 (2019), 1.6 (2021), 1.65 (2023), 1.75 (2025). After the admission
  the practice does not draw a basic panel; dialysis labs come in nephrology letters
  (pre-dialysis creatinine 6.4, potassium 4.8, hemoglobin 9.6 in April 2026; the unit plans permanent access).
- Hemoglobin 12.0 (2019, normocytic), 10.8 from 2021, 9.6 after the 2026 admission.
- Urine albumin/creatinine 210 mg/g (2019), 240 (2022), 260 (2024) (A2).
- Intact PTH 58 (normal, May 2020), 186 (February 2021), 172 (nephrology 2023).
- Serum glucose on metabolic panels runs about 165 to 200 mg/dL through 2025, in keeping
  with the A1c; about 210 on the 2026 admission, about 150 on glargine.
- Everything else is her own normal: lipids, liver tests, TSH, potassium, sodium,
  calcium, vitamin D (25-hydroxy vitamin D 34 in 2021).

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Encounter 0 ends with quiz text about a drug that raises insulin release by
  enzyme inhibition (a DPP-4 inhibitor) and no later list carries one. Resolution: the
  physician raises it, she wants to try diet first, it is not started, and no later
  visit takes it up again.
- **R2.** The profile lists osteoporosis and alendronate from the start, but encounter 1
  makes the diagnosis on the DXA and starts the upright-after-dosing drug (alendronate).
  Resolution: osteoporosis is diagnosed and alendronate starts on 2020-05-15.
- **R3.** Every list carries an ORIF of a low-trauma left distal radius fracture, yet in
  2020 she reports no fracture history, and encounter 5 (2024) follows a "recent
  fragility fracture". Resolution: the wrist fracture is that recent fracture: a fall
  from standing on 2024-01-08 and ORIF on 2024-01-12. Before 2024 no note mentions it;
  outside letters' copied surgical lists after it may carry it. The 2024 DXA (T −2.8,
  lumbar spine) is a repeat study done after the fracture.
- **R4.** Encounter 2 says she was recently started on hemodialysis, gives a creatinine
  of 5.8 and calls the hyperparathyroidism tertiary, while every list says CKD stage 3b,
  metformin and glyburide continue for five more years, and in 2026 she develops
  kidney failure and becomes anuric. Resolution: she is not on dialysis before 2026.
  Creatinine 5.8 is not carried: the 2021 creatinine is 1.6. The hyperparathyroidism is
  secondary, from CKD (normal calcium, raised PTH), followed by nephrology. "Tertiary" is
  not carried. The normocytic anemia is anemia of CKD.
- **R5.** Encounter 3 (February 2022) says she was hospitalized twice "this year" with
  intubation. Resolution: "this year" is the year before that visit: two admissions in
  April and November 2021.
- **R6.** BMI 31 in February 2022 and 24 in February 2024 on the same prednisone dose
  cannot both hold. Resolution: BMI 31 in 2022 is carried (it belongs to the weight
  gain); BMI 24 is not; she weighs about 77 kg (BMI about 30) from 2023.
- **R7.** Encounter 5 ends mid-sentence ("In addition to calcium and vitamin D
  supplementation,"). Resolution: the plan continues alendronate, calcium and vitamin D
  and gives fall-prevention advice. The same note's "no family history of fractures or
  malignancy" and "does not smoke or consume alcohol" are not carried; the profile's
  family history and occasional wine stand.
- **R8.** The source's medication lists drop and restore drugs from visit to visit
  (lisinopril, metformin, amlodipine, prednisone and others vanish from single lists).
  Resolution: all brought drugs continue unchanged until the 2026 discharge; outside
  letters may copy partial lists, as outside lists do.
- **R9.** Encounter 0 and the profile give no reason she is on prednisone; encounter 3
  says it is for RA and severe COPD. Carried as that.
- **R10.** The 2025 ED calls her CBC normal, while she has anemia of CKD from 2021.
  Resolution: the ED CBC shows a normal white count and platelets and her usual
  hemoglobin (about 10.8); the ED record calls it unremarkable for her.
- **Left as the source has it** (no note comments on any of these): no DPP-4 inhibitor,
  SGLT2 inhibitor or statin is ever started; metformin stays 1000 mg twice daily and
  glyburide 5 mg daily through CKD 3b; methotrexate is given without folic acid;
  alendronate continues through eGFR in the low 30s and after the fracture, with no
  switch of agent; prednisone is never tapered despite the endocrinology letter; COPD
  stays on fluticasone-salmeterol and albuterol only after two intubations; the height
  loss is never imaged; no alpha-1 antitrypsin test in a never smoker with severe COPD.

## Additions (A): physician to confirm

- **A1.** 2019-09-10 new-patient visit (the first source encounter is a routine
  diabetes visit and the seed gives about 7 years with the practice).
- **A2.** Nephrology care for the source's progressive CKD and "her nephrologist":
  a consult in 2019 and yearly letters (2020-10-20, 2021-10-19, 2022-10-18,
  2023-10-24, 2024-10-22, 2025-10-21), plus after dialysis starts (2026-04-14,
  2026-08-12). Urine albumin/creatinine 210 to 260 mg/g supports diabetic kidney
  disease. Nephrology letters change no drug before 2026.
- **A3.** Rheumatology care for RA on methotrexate and prednisone: yearly letters
  (2019-11-19, 2020-11-24, 2021-12-14, 2022-11-15, 2023-11-14, 2024-11-12,
  2025-11-18), stable each time, no change; and 2026-04-21 after discharge:
  methotrexate stays off while on dialysis, prednisone 10 mg continues, other options
  to be discussed when she has recovered.
- **A4.** One spirometry (2017, prior practice: FEV1/FVC 0.56, FEV1 41% predicted) to
  support the stated severe COPD.
- **A5.** PTH 186, calcium 9.3, phosphorus 4.6 for the hyperparathyroidism the
  endocrinology visit asserts.
- **A6.** The January 2023 portal message that produces the gynecology referral.
- **A7.** The course of the vulvar lesion after the source's exam: biopsy, gynecologic
  oncology, excision with sentinel nodes, stage IB, surveillance without recurrence
  (findings exactly as in the timeline).
- **A8.** Valacyclovir, renally dosed, for the ED zoster.
- **A9.** The 2026 ICU course around the source's facts: the arrival vital-sign
  numbers, the laboratory numbers, standard septic-shock care, cultures without growth,
  renal ultrasound, the biopsy date, hemodialysis for the anuria, the resulting end-stage
  renal disease, and the discharge medication changes (drugs unsafe on dialysis stopped;
  basal insulin for her diabetes).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Records from the prior practice may mention routine screening done there, with normal
  results (as in the 2019-09-24 row).
- Diabetes, kidney and blood pressure visits every 3 to 6 months, with vitals; A1c,
  metabolic panel, CBC, liver tests for methotrexate, lipids yearly, urine
  albumin/creatinine; foot exams with normal sensation and pulses. Annual wellness visits.
- Yearly diabetic eye exam letters from an outside optometrist: no retinopathy.
- Post-discharge visits after each admission.
- Prevention: screening mammograms 2020, 2022 and 2024, normal; fecal immunochemical test
  2025, negative; hepatitis C antibody once (2019), negative; HIV once (the 2025 ED test);
  high-dose influenza vaccine each fall; PPSV23 in 2019 (after the prior PCV13); COVID
  vaccines from 2021 (two-dose series at a pharmacy, then fall doses); RSV vaccine 2023;
  recombinant zoster vaccine two doses in 2025 after the zoster; Tdap 2025. No live
  vaccines (immunosuppressed). PHQ-2 or PHQ-9 yearly, negative or low.
- Refills, pharmacy and prior-authorization traffic, portal messages about results,
  refills and scheduling, a missed visit, reminders.
- No booked visit after 2026-08-27.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Records

Complete (seed): every outside letter, report and summary above arrives and is filed,
usually within a week, and the practice's lists are kept current until the last visit.

## Last seen

The practice last saw MRN00002823 on 2026-07-08. Practice contacts after that day are
only messages, refills and result notes; outside care after it still happens (the
August nephrology letter), and its records are filed as received without changing the
practice's lists. Nothing is booked after today.
