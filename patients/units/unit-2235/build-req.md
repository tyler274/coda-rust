# Build requirements: unit-2235 (Francis Kerrigan, MRN00002235)

Source: synthetic_hospital patient 2235, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its unfinished plan text.

## Who he is (S)

Francis Kerrigan, born 1955-07-23, male, White, not Hispanic or Latino, English,
Medicare, lives in Goshen MA 01032, phone +1-518-555-0134. Retired high school
mathematics teacher. Never smoker, no alcohol. No known drug allergies. Married; his
wife is the person who reports his sleep behavior (R3) and, in 2025, his cognitive
decline. Surgical history: cataract extraction with lens implant, right eye (R2).
Family history: father with type 2 diabetes and prostate cancer; mother with
hypertension and hyperlipidemia; a maternal uncle with chronic lymphocytic leukemia.
Wears sandals at home and sometimes goes barefoot indoors (S, recorded at the foot
ulcer visit).

Joined the practice on **2019-09-10** (A1), age 64, from a prior practice. Brought:
type 2 diabetes since about 2001 (20 years' duration by 2021), diabetic retinopathy
followed by an outside ophthalmologist, essential hypertension, hyperlipidemia (never
on a drug). Medications on joining: **metformin 1000 mg twice daily, lisinopril 20 mg
daily, hydrochlorothiazide 12.5 mg daily** (S), continued unchanged through the last
visit.

Chart texture (seed): records have gaps. Some outside letters never come or come late;
the practice sometimes learns of outside care from the patient, his wife or a pharmacy
fill first, and its lists lag.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2019-09-10 | practice | **new patient visit (A1)**; history as above; BP 140s/80s; labs; prior records requested |
| 2021-03-09 | outside | **right cataract extraction with lens implant** at an outside ophthalmology surgery center (R2); op note received 2021-03-23 |
| 2021-05-11 | practice | routine diabetes follow-up (age 65). A few weeks of foul odor from the right foot and occasional fluid leaking; no fever, no pain. Walks barefoot or in sandals at home. Temp 37.1 C, BP 142/88, pulse 84, RR 14. Shallow ulcer on the right plantar foot ringed by callus with minimal erythema; diminished sensation in both feet. HbA1c 8.9%, WBC 8.4. **Diabetic foot ulcer, right foot, diagnosed** (healing, proliferative phase). Right foot x-ray and **podiatry referral (A2)**; no change to diabetes drugs (left as the source has it) |
| 2021-05-18 to 2021-08-10 | outside | podiatry (A2): letters received 2021-05-25, 2021-06-30 and 2021-08-17 |
| 2021-08-24 | practice | diabetes follow-up; **foot ulcer resolved**; wife describes about three months of him punching and kicking in his sleep with falls from bed; **neurology referral (A3)** |
| 2021-11-01 | outside | neurology consult (S, enc 1): about six months of dream enactment (punching, kicking, falls from bed), embarrassed, vivid dreams that match the movements, no sleep problems before. Alert and oriented; neurological exam normal, no rigidity, tremor or slowness; normal tone awake. Polysomnography that night: increased muscle activity in REM sleep (REM sleep without atonia). **REM sleep behavior disorder diagnosed**; **melatonin 3 mg at bedtime** and bedroom safety (A4). Letter with the sleep study received 2021-11-09 |
| 2022-03-22 | practice | about four months of low back pain spreading to both thighs with walking (A5); **orthopedics referral (A5)** |
| 2022-05-02 | outside | orthopedics (S, enc 2), age 66: six months of low back pain radiating to the thighs, worse after a few minutes of walking, eased by leaning forward or sitting; no trauma. Pain on lumbar extension; no dermatomal sensory or motor loss (R7); straight leg raise negative both sides; leg vascular exam normal. MRI the same day: canal narrowing with ligamentum flavum thickening (R6). **Thoracic spinal stenosis.** Physical therapy and acetaminophen advised (A6). **No orthopedic letter ever arrives**; the practice learns of the visit from the patient on 2022-06-07 and gets the MRI report on request 2022-06-21 |
| 2022-10-31 | outside | neurology follow-up (S, enc 3): dream enactment worse, wife describes punching and kicking and falls from bed with minor injuries (bruises); he is unaware of the episodes but recalls vivid action-filled dreams. Alert, oriented, normal cognition; neurological exam normal, no rigidity, slowness or tremor. The 2021 study is cited (R10). **Melatonin raised to 6 mg at bedtime** (A4). Letter arrives late, 2023-01-17 |
| 2023-04-11 | practice | about seven weeks of fatigue, night sweats, poorer appetite, abdominal fullness, about 10 lb weight loss; enlarged neck and armpit nodes; CBC WBC 13.6, Hgb 9.4, platelets 156, lymphocytes 71% (A7); **urgent hematology referral (A7)** |
| 2023-04-27 | outside | hematology-oncology (S, enc 4), age 67: two months of fatigue, night sweats and 10 lb unintentional weight loss, abdominal fullness, less appetite. Pale; bilateral cervical and axillary lymphadenopathy; spleen 3 cm below the left costal margin. Hgb 9.2, MCV 85, WBC 14.0, platelets 150, LDH elevated, smear with smudge cells. Peripheral blood flow cytometry (A8). **Chronic lymphocytic leukemia diagnosed**, Rai stage III. Letter received 2023-05-05 |
| 2023-05-17 | outside | **acalabrutinib 100 mg twice daily started** by hematology-oncology (A8); the practice learns from the specialty pharmacy notice 2023-05-19 |
| 2023-11-02 | outside | oncology (S, enc 5), six months on: constant dull low back pain, not eased by rest, worse at night; no trauma; 10 lb unintentional weight loss over two months; occasional night sweats. Lumbar spine tender, no deformity; straight leg raise negative both sides; neurological exam normal. Hgb 12.5, ESR 70, calcium 10.2, alkaline phosphatase 120. Imaging and PSA ordered (R12, A9). Letter received 2023-11-10 |
| 2023-11-08 to 2023-12-06 | outside | workup and treatment (A9, A10); **metastatic carcinoma to bone** from a **prostate adenocarcinoma**; **leuprolide, abiraterone and prednisone started**. The practice hears from the patient and a pharmacy fill first (2023-12-11, 2023-12-12); the oncology letter with the pathology report arrives 2023-12-19 |
| 2025-04-25 | outside | emergency department (S, enc 6), age 69: three weeks of worsening breathlessness on exertion and dry cough; no chest pain, fever or weight loss. BP 145/90, HR 88, RR 20, SpO2 94% on room air. Decreased breath sounds at the right base; no edema, no JVD, no murmur. Creatinine 1.2, BUN 18, albumin 4.0, total protein 7.2, glucose 160. Chest x-ray: moderate right pleural effusion, no cardiomegaly, no pulmonary edema. Diagnostic thoracentesis (A11). **Uncomplicated parapneumonic effusion**; **levofloxacin 750 mg daily for 7 days** (A11); discharged home. The practice hears from his wife 2025-04-28; the ED record arrives on request 2025-05-20 |
| 2025-05-02 | practice | post-ED visit (A12); follow-up chest x-ray ordered |
| 2025-06-03 | practice imaging | chest x-ray: effusion resolved (A12) |
| 2025-09-23 | practice | wife reports about six weeks of memory loss, confusion and sudden jerks (A13); workup and urgent neurology referral |
| 2025-11-03 | outside | neurology (S, enc 7): rapidly progressing memory loss, confusion, sudden muscle jerks; wife reports several episodes of jerking, more irritable and withdrawn, sleeping more, hard to keep awake in the day. Disoriented to time and place; generalized myoclonus. Routine blood tests normal (R15). Lumbar puncture: CSF WBC 3/mm3, protein 45 mg/dL, glucose 60 mg/dL, 14-3-3 protein positive. EEG: generalized periodic sharp wave complexes. **Creutzfeldt-Jakob disease (probable sporadic)**; **levetiracetam 500 mg twice daily** for myoclonus; MRI and CSF RT-QuIC sent (A14). Letter received 2025-11-10 |
| 2025-11-14 | practice | **last visit** (seed `last_seen`), with his wife: advance care planning and palliative care referral (A15) |
| 2025-11-20 to 2026-07 | outside | MRI and RT-QuIC results, palliative care, hospice (A14, A16); filed as received, practice lists unchanged |
| 2026-09-10 | practice | booked video visit with his wife (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The first source encounter is a routine diabetes follow-up, so he was already
  a patient. The profile's chronic conditions (diabetes, retinopathy, hypertension,
  hyperlipidemia) are known at joining; the seven primary diagnoses start only at the
  encounters that make them.
- **R2.** "Cataract extraction, right eye (2020)" sits in the source's own 2020 and the
  first practice-dated encounter is in May 2021; the year follows the re-dated timeline:
  surgery 2021-03-09. The 2019 and 2020 eye exam letters note a right lens opacity
  becoming visually significant; no cataract is put on the problem list.
- **R3.** The neurology notes say "She describes" what he does in his sleep. The account
  is his wife's (the 2025 note names her as reporter).
- **R4.** The 2022 neurology note calls the hypertension "mild, untreated". He is on
  lisinopril and hydrochlorothiazide throughout; "untreated" applies only to the
  hyperlipidemia, which no one ever treats.
- **R5.** The source's medication lines vary by note ("metformin and lisinopril" with
  hydrochlorothiazide listed separately; "controlled with hydrochlorothiazide"). All
  three drugs at the stated doses run from joining through the last visit.
- **R6.** The diagnosis is thoracic spinal stenosis but the source's MRI is "of the
  lumbar spine". Resolution: the study is an MRI of the lumbar spine whose upper field
  includes the lower thoracic segments; the canal narrowing with ligamentum flavum
  thickening is at T11-T12, and the lumbar levels have no significant canal narrowing.
- **R7.** Orthopedics finds "no loss of sensation" a year after the practice found
  diminished sensation in both feet. Orthopedics examined for dermatomal sensory loss;
  the practice's foot exams keep finding reduced monofilament sensation at the toes and
  soles. No neuropathy diagnosis is added.
- **R8.** Every later source list carries the foot ulcer "in proliferative phase of
  healing". It heals by 2021-08-10 and the practice resolves it 2021-08-24. Outside
  letters may still copy it, as outside lists do.
- **R9.** The ED and 2025 neurology notes call it "metastatic bone disease" and
  "metastatic carcinoma to bone"; the chart carries the prostate primary found in A9
  alongside the bone metastases.
- **R10.** Both neurology notes say polysomnography shows REM muscle activity. One study
  only, 2021-11-01; the 2022 letter refers back to it.
- **R11.** Two separate 10 lb losses (spring 2023 with untreated CLL, autumn 2023 with
  the bone metastases): weight 86 kg at joining, about 81.5 kg in April 2023, back to
  about 85 kg by September 2023 on acalabrutinib, about 80.5 kg in November 2023, about
  82 kg through 2024 and 2025, about 78 kg in November 2025. Height 176 cm.
- **R12.** The 2023-11-02 plan stops mid-sentence. The oncology letter orders a bone
  scan, CT of chest, abdomen and pelvis, and PSA (A9).
- **R13.** The source dates the metastatic carcinoma to 2023-11-02 (practice date); it is
  recorded when the letter with the biopsy arrives (2023-12-19), onset 2023-11-02.
- **R14.** The source's first-visit diabetes note (A1c 8.9) and every later note keep
  metformin alone. The chart keeps that.
- **R15.** "Routine blood tests are normal" in November 2025: CBC and metabolic panel
  normal (the CLL controlled on acalabrutinib).
- **R16.** The source's "unremarkable" neuro exam in 2021 and 2022 and the year "July
  2020" in the 2022 note follow the re-dated timeline (first neurology visit November
  2021).
- **Left as the source has it:** A1c runs 8.2 to 9.0 on metformin alone and no second
  agent is ever added; hyperlipidemia (LDL about 135) is never treated with a statin in
  a diabetic; office pressures in the 140s/80s are never titrated; no aspirin. No note
  comments on any of these. No PSA screening is done in the practice despite the
  father's prostate cancer, and no one remarks on it.

## Additions (A): physician to confirm

- **A1.** 2019-09-10 new-patient visit (the source starts with a routine follow-up).
- **A2.** Foot ulcer care. At the 2021-05-11 visit: right foot x-ray (2021-05-12: no
  cortical erosion, periosteal reaction or soft-tissue gas; no fracture) and podiatry
  referral. Podiatry 2021-05-18: ulcer under the right first metatarsal head 1.4 x 1.0 cm,
  0.2 cm deep, granular base, no probe to bone; sharp debridement of the surrounding
  callus; offloading post-op shoe with a felt aperture pad; saline-moistened dressing
  daily; no antibiotic. 2021-06-22: 0.6 x 0.4 cm, granulating. 2021-08-10: healed,
  epithelialized; diabetic shoes with custom insoles; podiatry every 3 months for callus
  care. Shoes and slippers at home, no barefoot walking.
- **A3.** 2021-08-24: the referral visit to neurology for dream enactment.
- **A4.** REM sleep behavior disorder care: melatonin 3 mg at bedtime from 2021-11-01,
  raised to 6 mg at bedtime 2022-10-31; bedroom safety (mattress on the floor side,
  furniture moved, wife sleeping apart on bad nights). The practice records the dose
  increase from the patient at 2022-12-13 before the letter arrives.
- **A5.** 2022-03-22 practice visit: about four months of low back pain spreading into
  both thighs with walking; orthopedics referral.
- **A6.** Thoracic stenosis care: physical therapy (a course of outside sessions, per
  patient) and acetaminophen 1000 mg up to three times daily as needed, prescribed by the
  practice 2022-06-07.
- **A7.** 2023-04-11 practice visit and CBC (WBC 13.6, Hgb 9.4, platelets 156,
  lymphocytes 71%); urgent hematology referral.
- **A8.** CLL confirmation and therapy: peripheral blood flow cytometry with a clonal
  B-cell population, CD5 and CD23 positive, dim CD20, kappa light chain restricted; Rai
  stage III (anemia). Acalabrutinib 100 mg twice daily from 2023-05-17. Hematology
  follow-up in the oncology letters: Hgb 12.5 by November 2023 (S), CBC normal from 2024.
- **A9.** Metastatic workup: bone scan 2023-11-08 with multiple foci of uptake in the
  thoracic and lumbar spine, pelvis and ribs; CT chest, abdomen and pelvis with contrast
  2023-11-08 with sclerotic lesions in thoracic and lumbar vertebral bodies and both
  iliac bones, mildly enlarged cervical and axillary lymph nodes and spleen (known CLL),
  no lung, liver or adrenal lesion; PSA 64.8 ng/mL; CT-guided left iliac bone biopsy 2023-11-16: metastatic adenocarcinoma,
  NKX3.1 and PSA positive, consistent with prostatic origin. Diagnoses: malignant
  neoplasm of prostate with secondary malignant neoplasm of bone.
- **A10.** Metastatic prostate cancer therapy by oncology: leuprolide 22.5 mg injection
  every 12 weeks from 2023-11-29; abiraterone 1000 mg daily with prednisone 5 mg daily
  from 2023-12-06. PSA 2.1 (2024-03-06), 0.3 (2024-06-12; that letter never arrives, the
  patient reports it), under 0.1 (2024-12-04 and 2025-06-11). Acalabrutinib continues.
- **A11.** ED care: ultrasound-guided diagnostic right thoracentesis, 60 mL clear yellow
  fluid: pH 7.38, glucose 128 mg/dL, protein 4.3 g/dL, LDH 196 U/L, nucleated cells
  2,400/uL, neutrophil predominant; Gram stain no organisms; culture no growth; cytology
  negative for malignant cells. Levofloxacin 750 mg daily for 7 days; discharged home.
- **A12.** 2025-05-02 post-ED practice visit (cough better, breathing near baseline,
  SpO2 97%, faint dullness at the right base) and 2025-06-03 follow-up chest x-ray: no
  effusion, clear lungs, sharp costophrenic angles. Effusion resolved on the list at
  2025-07-22.
- **A13.** 2025-09-23 practice visit: wife reports about six weeks of forgetfulness,
  confusion, irritability and occasional sudden jerks of the arms. MoCA 19/30. CBC,
  metabolic panel, TSH, B12 and RPR normal; CT head without contrast 2025-09-25 with no
  acute abnormality, mild age-appropriate volume loss. Urgent neurology referral.
- **A14.** Neurology after the November visit: levetiracetam 500 mg twice daily for
  myoclonus from 2025-11-03; MRI brain without contrast 2025-11-20: restricted diffusion
  along the cortical ribbon of both cerebral hemispheres and in both caudate heads and
  putamina, consistent with prion disease; CSF RT-QuIC positive (letter received
  2025-12-08). Probable sporadic CJD. Reportable case notified by neurology.
- **A15.** 2025-11-14 last practice visit: disoriented, myoclonic jerks, sleeping much
  of the day, needs help dressing and bathing, weight about 78 kg; advance care planning with his wife (comfort-focused goals, DNR, MOLST
  completed); palliative care referral; current medications continued.
- **A16.** Palliative care home consult 2026-01-06 (letter received 2026-01-13): no longer
  walking unaided, speaks little; levetiracetam continued; hospital bed and home health
  aide arranged. Home hospice from 2026-03-04
  (admission notice received 2026-03-06): bedbound, minimally verbal, total care by his
  wife with hospice aides; with oncology's agreement abiraterone and acalabrutinib
  stopped and no further leuprolide; prednisone continued; metformin stopped;
  levetiracetam, lisinopril and hydrochlorothiazide continued. The practice physician
  agrees to stay attending of record (2026-03-10). These changes stay in the hospice
  records; the practice lists are not updated.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level here says
otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Prior-practice records (received 2019-09-30): colonoscopy 2017 normal, repeat in 10
  years; Tdap 2016; diabetes and blood pressure care on the three drugs above.
- Diabetes and blood pressure visits about every 3 to 5 months, with vitals, A1c, basic
  or comprehensive metabolic panel, lipids yearly, urine albumin/creatinine yearly (normal),
  foot exams (diminished monofilament sensation at the toes and soles, R7; good pulses).
  Levels: A1c 8.2 to 9.0 (8.9 on 2021-05-11); office BP 136 to 146 / 82 to 90; LDL about
  135, HDL about 42, triglycerides about 170; creatinine 1.0 to 1.2. CBC normal before
  2023 and from 2024.
- Yearly diabetic eye exam letters from an outside ophthalmologist: diabetic retinopathy
  stable, no macular edema; right lens opacity in 2019 and 2020 (R2); pseudophakic right
  eye from 2021.
- Hepatitis C and HIV screening once (2019, negative). No lung, AAA or PSA screening.
- Welcome-to-Medicare visit 2020 and yearly wellness visits 2022 to 2024 with PHQ-2
  negative; influenza vaccine each fall (high-dose), COVID primary series 2021 and
  boosters each fall 2021 to 2024, shingles series 2020-2021, PCV20 2022, RSV 2023.
- Refills, pharmacy and specialty-pharmacy notices, portal messages about results and
  scheduling, records requests that go unanswered, one cancelled visit.
- After 2025-11-14: only messages from his wife, refills, hospice paperwork and outside
  records filed as received.
- Booked visit after 2026-08-27: a video visit with his wife (2026-09-10).

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002235 on 2025-11-14. Practice contacts after that day do
not happen; outside care after it still does, and its records are filed as received
without changing the practice's lists. A visit is booked after today.
