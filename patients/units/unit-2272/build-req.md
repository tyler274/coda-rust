# Build requirements: unit-2272 (Bernard Bennett, MRN00002272)

Source: synthetic_hospital patient 2272, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text ("Findings on blood smear are shown
below").

## Who he is (S)

Bernard Bennett, born 1955-07-03, male, White, not Hispanic or Latino, English,
Medicare (from his 65th birthday, July 2020; commercial coverage before that is
texture). Lives in town with his wife (S: the ICU note's "his wife", the neurology
note's "care partner"); married. Retired construction foreman. Tobacco: 40
pack-years, about a pack a day from his early twenties; still smoking when he joins
and quits in April 2020 (R2). Alcohol: occasional, 1 to 2 drinks a month. No known
drug allergies.

Surgical history: appendectomy at 34 (about 1990); cystoscopy with urethral dilation
at 64 (2019-08-12, outside urology, before he joined); diagnostic bone biopsy of the
left tibia, done 2022-11-08 at 67 (R3).

Family history: father, myocardial infarction at 68; mother, dementia with behavioral
disturbances; older brother, Paget disease of bone. No other relatives.

**Joins the practice 2019-09-17** (A1, new-patient visit; 7 years with the practice
by today), age 64. He brings:

- Essential hypertension, on **hydrochlorothiazide 25 mg daily** and **lisinopril 10 mg
  daily** (both continue unchanged to the 2026 admission, R5).
- Chronic kidney disease, stage 2 (A10: creatinine about 1.1, urine
  albumin/creatinine 40 to 70 mg/g).
- Benign prostatic hyperplasia with lower urinary tract symptoms, on **tamsulosin 0.4 mg
  daily**.
- Lumbar osteoarthritis (lumbar spondylosis) with radiculopathy, long-standing; takes
  **acetaminophen 500 mg as needed for pain** (R14).
- Urethral stricture, dilated cystoscopically 2019-08-12 (R15).
- Nicotine dependence, cigarettes (current smoker; resolves when he quits, R2).

Not known at joining (each starts at its encounter): chronic bronchitis, Paget disease,
right conductive hearing loss, primary myelofibrosis, age-related cognitive decline,
the 2024 lumbar radiculopathy episode, carotid sinus hypersensitivity, the 2026
hypertensive emergency and cyanide toxicity.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2019-09-17 | practice | **new-patient visit** (A1); history as above; prior records filed (urology note of the 2019-08-12 dilation; a normal colonoscopy in 2014 at the prior practice, repeat due 2024). BP 134/84, weight 94 kg, height 178 cm. Labs at joining (P). Smoking cessation advised; **low-dose CT lung screening ordered** (A2) |
| 2019-10-22 | outside imaging | LDCT: Lung-RADS 1 (A2) |
| 2020-03-10 | practice | HTN/CKD follow-up; mentions a morning cough with white phlegm through the winter (S: cough present two years by April 2022, worse in winter); sets a quit date |
| 2020-04-06 | (home) | **quits smoking** (R2); tobacco status former smoker from here; nicotine dependence resolved, history of tobacco dependence recorded |
| 2020-08-11 | practice | Welcome to Medicare visit (P); AAA screening ultrasound ordered (P) |
| 2020-11-17 | outside imaging | LDCT: Lung-RADS 1 (A2) |
| 2021-11-16 | outside imaging | LDCT: Lung-RADS 1 (A2) |
| 2022-03-08 | practice | productive cough for about two years, worse each winter, white sputum, now intermittently blood-streaked for about three months (R9); no fever, weight loss or night sweats; scattered expiratory wheezes; **referred to pulmonology** (A3) |
| 2022-04-18 | outside, pulmonology | consult letter (received 2022-04-25): cough about two years, worse in winter, white sputum with intermittent blood streaking for 4 months; no fever, weight loss or night sweats; BP 135/85, HR 78, SpO2 96% on room air; overweight, no distress; scattered wheezes and prolonged expiration. Chest X-ray: increased bronchovascular markings, no mass, no focal consolidation. **Chronic bronchitis diagnosed.** Plan: stay off tobacco, continue yearly LDCT, call if the blood increases. No inhaler, no spirometry (left as source) |
| 2022-09-13 | practice | annual wellness visit; left hip and shin ache for about five months, worse with walking; new headaches and muffled hearing on the right in recent weeks; **referred to orthopedics** (A3) |
| 2022-10-24 | outside, orthopedics | consult letter (received 2022-11-01): six months of increasing left leg pain felt from the hip down the shin (R4), recent headaches, right-sided hearing difficulty. Left tibia enlarged and warm with mild erythema, no overlying skin changes; hip range of motion full and painless; neurological exam unremarkable except decreased hearing on the right. Calcium 9.8 mg/dL, phosphorus 3.5 mg/dL, **alkaline phosphatase 220 U/L**, PTH 55 pg/mL, 25-OH vitamin D 32 ng/mL. X-ray left tibia and fibula: thickened cortical bone with a cotton-wool appearance. Audiometry: conductive hearing loss, right ear. **Paget disease of bone, left tibia, diagnosed; right conductive hearing loss recorded.** Plan: biopsy to exclude another lesion, then bisphosphonate (A4) |
| 2022-11-08 | outside, orthopedics | **left tibia needle bone biopsy** (R3); pathology (issued 2022-11-16): thickened, disorganized trabeculae with mosaic cement lines, increased osteoclasts and fibrovascular marrow; no malignancy; consistent with Paget disease (A4) |
| 2022-11-17 | outside imaging | LDCT: Lung-RADS 1 (A2) |
| 2022-11-22 | practice (portal) | **cholecalciferol 1,000 units daily started** before the infusion (A4) |
| 2022-12-06 | outside infusion | **zoledronic acid 5 mg IV once** (A4) |
| 2023-02-28 | practice lab | alkaline phosphatase 96 U/L (A4); normal on every later draw |
| 2023-04-11 | practice | three months of fatigue, early satiety and unintentional weight loss (about 93 to 88.5 kg); spleen palpable well below the left costal margin; CBC hemoglobin 9.2 g/dL, WBC 2.6 x10^3/uL, platelets 84 x10^3/uL (A3); **urgent hematology referral** |
| 2023-04-24 | outside, hematology-oncology | consult letter (received 2023-05-03): fatigue, early satiety, weight loss over three months; history of hypertension, no known malignancy; marked splenomegaly below the left costal margin; **hemoglobin 9.0 g/dL, WBC 2,500/uL, platelets 80,000/uL**; smear with teardrop red cells and a leukoerythroblastic picture (R11); marrow aspirate a dry tap, so a trephine core is taken the same day: fibrotic marrow with clustered atypical megakaryocytes, consistent with primary myelofibrosis (issued 2023-05-01); JAK2 V617F detected; abdominal ultrasound 2023-04-27: spleen 19 cm, liver normal size (A5). **Primary myelofibrosis diagnosed.** Not a transplant candidate given age and comorbidities (A5) |
| 2023-05-10 | outside, hematology | **ruxolitinib 5 mg twice daily started** (A5) for symptomatic splenomegaly and constitutional symptoms; CBC every 3 months at hematology from here |
| 2023-08 to 2026-08 | outside, hematology | follow-up letters about every 3 months (A5): early satiety gone by August 2023, weight back to 90 to 92 kg, spleen tip about 4 cm below the costal margin from late 2023; hemoglobin about 9.5 to 10, WBC about 3.0 to 3.5, platelets about 85 to 95; ruxolitinib 5 mg twice daily throughout |
| 2023-11-14 | outside imaging | LDCT: Lung-RADS 2, 3 mm solid nodule right upper lobe (A2) |
| 2024-03-19 | practice | HTN/CKD visit; BP 146/90 (above goal from 2024 on, R16); his wife reports about a year of misplaced keys, trouble with new names and word-finding; he still pays the bills and manages his own pills; delayed recall 2 of 3 words (A3); **referred to neurology**; FIT kit given (P) |
| 2024-04-24 | outside, neurology | consult letter (received 2024-05-01): the history above, waking earlier without daytime fatigue; alert, oriented x3; **MMSE 28/30, points lost only on delayed recall**; neurological exam normal; electrolytes, TSH and vitamin B12 normal; CBC unchanged from his hematology values (R7). **Age-related cognitive decline diagnosed** (R6); neurology follow-up in six months |
| 2024-09-30 | practice (phone) | right thigh pain for a week; already booked with neurology on 10-07; acetaminophen (A3) |
| 2024-10-07 | outside, neurology | letter (received 2024-10-14): two weeks of dull right anterior thigh pain toward the medial knee, worse walking uphill, better with rest; right leg weak rising from a chair or stepping onto a curb; no bowel or bladder incontinence, saddle anesthesia or back trauma. Right knee extension 4/5, hip flexion and ankle strength intact, right patellar reflex reduced compared with the left, reduced pinprick and light touch over the anterior thigh and medial shin, straight leg raise negative. **Lumbar radiculopathy diagnosed** (right L4 territory). **MRI lumbar spine ordered** |
| 2024-10-18 | outside imaging | MRI lumbar spine without contrast (A6): multilevel disc degeneration and facet arthropathy L3 to S1; at L3-L4 a right subarticular disc bulge with facet hypertrophy narrows the right lateral recess and contacts the traversing right L4 root; no significant central canal stenosis; no fracture |
| 2024-10-30 | outside, neurology | letter: MRI reviewed; physical therapy referral; acetaminophen (A6) |
| 2024-11-12 | outside, physical therapy | PT evaluation (A6); about 8 visits |
| 2024-11-19 | outside imaging | LDCT: Lung-RADS 2, right upper lobe 3 mm nodule unchanged (A2) |
| 2025-01-27 | outside, neurology | letter: thigh pain gone, right knee extension 5/5, right patellar reflex still slightly reduced; **lumbar radiculopathy resolved** (A6) |
| 2025-04-17 | outside, ED | ED record (received 2025-04-21): blood in the urine this morning, at the start of the stream and clearing by the end; no trauma or vigorous exercise; BP 145/90, HR 78, afebrile; abdomen unremarkable, no flank tenderness; prostate mildly enlarged and non-tender; no meatal discharge or lesions. Urinalysis: red cells present (blood moderate, 35 RBC/hpf), no casts; **creatinine 1.1 mg/dL**; CBC unchanged from his hematology values (R7); PT/INR normal. Discharged with urology follow-up |
| 2025-04-22 | practice | post-ED visit; no further visible blood; urology referral confirmed (A3) |
| 2025-05-06 | outside imaging | CT urogram (CT abdomen and pelvis without and with contrast): no renal mass, no hydronephrosis, no filling defect in the collecting systems, ureters or bladder; spleen enlarged as known (A7) |
| 2025-05-13 | outside, urology | letter (A7): flexible cystoscopy shows a short bulbar urethral stricture, narrow but passable, with friable mucosa at the stricture; enlarged prostatic lateral lobes; bladder without tumor or stones. Hematuria attributed to the stricture. Observation; urology as needed; repeat urinalysis at the practice |
| 2025-06-03 | practice lab | urinalysis: no blood, 0 to 2 RBC/hpf (A7) |
| 2025-10-10 | outside, ED | ED record (received 2025-10-14): recurrent dizzy spells, and this morning one syncope while shaving after light-headedness and a sense of blacking out; BP 145/90, HR 78; neurological exam normal; ECG normal sinus rhythm (P); troponin normal (P). **Carotid sinus massage under monitoring: 4-second ventricular pause, systolic pressure to 85 mm Hg. Carotid sinus hypersensitivity diagnosed** (R13). Discharged with cardiology referral |
| 2025-10-16 | practice | post-ED visit; no further syncope; electric razor, loose collars, turn the head slowly (A8); cardiology referral placed |
| 2025-11-12 | outside, cardiology | letter (A8): mixed carotid sinus hypersensitivity; transthoracic echo normal (LVEF 60%, no significant valve disease); 14-day ambulatory monitor ordered. Conservative care; pacing to be reconsidered if syncope recurs |
| 2025-12-08 | outside, cardiology | letter (A8): monitor sinus rhythm 52 to 118, no pauses over 2 seconds, no atrial fibrillation; no further syncope; cardiology as needed |
| 2025-11-18 | outside imaging | LDCT: Lung-RADS 2, nodule unchanged (A2) |
| 2026-04-12 | outside, hospital | admitted through the ED for **hypertensive emergency** (A9: BP 226/128, headache, creatinine 2.5 mg/dL, BUN 40 mg/dL); ICU; **sodium nitroprusside infusion** |
| 2026-04-14 | outside, ICU | 48 hours into nitroprusside: increasing agitation and confusion, severe headache, flushing; his wife notes heavier breathing. BP 145/90, HR 102, RR 22, SpO2 94% on room air. ABG pH 7.35, PaCO2 45 mmHg, PaO2 85 mmHg; sodium 140, potassium 4.2, **creatinine 2.8 mg/dL (2.5 on admission, R8)**, BUN 55 mg/dL, **lactate 5 mmol/L**; venous cyanide sent. ECG sinus tachycardia, otherwise normal. **Cyanide toxicity diagnosed** (A9): nitroprusside stopped, **hydroxocobalamin 5 g IV**, **nicardipine infusion** instead |
| 2026-04-15 to 04-18 | outside, hospital | (A9) lactate 1.6 mmol/L on 04-15; mental status back to his baseline within a day; cyanide level resulted 04-16: 1.1 mg/L; creatinine 1.6 mg/dL, BUN 28 at discharge 2026-04-18; **amlodipine 5 mg daily added**, lisinopril, hydrochlorothiazide, tamsulosin and ruxolitinib continued; discharge BP 142/86. Discharge summary received 2026-04-20 |
| 2026-04-24 | practice | post-discharge visit (A3/A9): BP 138/84; creatinine 1.4 mg/dL; feels himself again |
| 2026-06-12 | practice | **last practice visit** (seed `last_seen`): BP 132/80; creatinine 1.2 mg/dL; no further syncope or hematuria; follow-up booked for September |
| after 2026-06-12 | outside | hematology letter(s) filed as received; the practice's lists stay as at 06-12 |
| 2026-09-14 | practice | booked routine follow-up (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The profile lists every condition from the first encounter, and the stamps
  date each to a later encounter. Resolution: only hypertension, CKD, BPH, lumbar
  osteoarthritis with radiculopathy and urethral stricture are known at joining; every
  other condition starts at the encounter that diagnoses it, as in the timeline.
  Outside letters list only what is known by their date.
- **R2.** "Quit 2 years ago" repeats unchanged on every visit from 2022 to 2026.
  Resolution: he quit 2020-04-06, two years before the April 2022 pulmonology visit;
  later letters say "quit in 2020" or "former smoker". 40 pack-years throughout.
- **R3.** The tibia biopsy is listed "at age 65" (2020 to 2021), before Paget disease is
  first diagnosed at 67. Resolution: the biopsy follows the orthopedic visit, on
  2022-11-08 at 67. "Age 65" is not carried.
- **R4.** The orthopedic complaint is left hip pain; the findings are in the left tibia.
  Resolution: he feels the pain from the hip down the shin; the hip examines normally
  and the findings are in the tibia.
- **R5.** Encounter prose drops hydrochlorothiazide or lisinopril and tamsulosin from
  some home-medication lists ("He takes which are managed with tamsulosin and
  lisinopril"). Resolution: hydrochlorothiazide 25 mg, lisinopril 10 mg, tamsulosin
  0.4 mg and acetaminophen as needed continue unchanged from joining through the
  2026 admission. Outside lists may omit one, as outside lists do.
- **R6.** "Mild neurocognitive disorder (age-related)" (profile) and "Age-related
  cognitive decline" (diagnosed at the 2024 neurology visit) are one condition. With
  an MMSE of 28/30 and intact daily function the practice list carries "Age-related
  cognitive decline" from 2024-04-24 only; the hospital in 2026 calls it mild
  neurocognitive disorder in its own words.
- **R7.** A CBC "within normal limits" a year after a pancytopenic myelofibrosis
  diagnosis (neurology 2024, ED 2025) cannot be true. Resolution: those CBCs show his
  stable known cytopenias at the hematology levels; the letters call them unchanged.
- **R8.** Creatinine 1.1 in April 2025 and a "baseline" of 2.5 a year later cannot both
  be baselines. Resolution: his baseline is about 1.1 (CKD stage 2 with
  albuminuria); 2.5 is the admission value of the hypertensive emergency (its renal
  involvement, recorded within that admission, not as a separate problem), 2.8 at 48
  hours, recovering to 1.6, 1.4 and 1.2.
- **R9.** The pulmonology complaint is blood-tinged sputum for 4 months; the history says
  white sputum. Resolution: sputum is white, intermittently streaked with blood for
  about 4 months.
- **R10.** Neurology says he "returns" for cognitive symptoms; there is no earlier
  neurology visit. Resolution: 2024-04-24 is his first neurology consult; 2024-10-07
  is the return.
- **R11.** "Findings on blood smear are shown below" (no image) and a dry tap with no
  biopsy. Resolution: the letter describes the smear (teardrop cells,
  leukoerythroblastic picture) and the same-day trephine core gives the diagnosis
  (A5).
- **R12.** The ICU cyanide level is "pending": it results on 2026-04-16 (A9).
- **R13.** Carotid sinus hypersensitivity is on every list from 2022 and the ED history
  calls him a patient "with carotid sinus hypersensitivity", but carotid sinus massage
  first establishes it on 2025-10-10. Resolution: diagnosed 2025-10-10; the earlier
  dizzy spells were not reported to the practice.
- **R14.** "Lumbar osteoarthritis with radiculopathy" (long-standing) and "Uncomplicated
  lumbar radiculopathy" (2024). Resolution: lumbar spondylosis with radiculopathy is
  long-standing history at joining; the right L4 radiculopathy of October 2024 is a
  new episode, diagnosed then and resolved 2025-01-27.
- **R15.** Urethral stricture is stamped "diagnosed" in 2025 but was dilated at 64.
  Resolution: known at joining from the 2019-08-12 dilation.
- **R16.** Hypertension "controlled" (2022) and "poorly controlled" (2024). Both true:
  BP about 132 to 136 / 82 to 86 through 2023, about 144 to 150 / 88 to 94 from 2024
  to the admission.
- **R17.** The ICU encounter is "48 hours after admission": admitted 2026-04-12, the
  source event 2026-04-14.
- **Left as the source has it:** no statin despite a 40 pack-year history, hypertension
  and a father with MI at 68; no inhaler and no spirometry for chronic bronchitis; no
  CT beyond yearly screening for the blood-streaked sputum; no workup of the new
  headaches and no ENT or skull imaging for the conductive hearing loss; no brain
  imaging for the memory complaint; blood pressure above goal from 2024 to April
  2026 with no drug change (the practice records readings and asks for home
  readings). Nobody comments on any of these.

## Additions (A): physician to confirm

- **A1.** The 2019-09-17 new-patient visit (seven years with the practice), the prior
  records (urology dilation note 2019-08-12; colonoscopy 2014 normal at the prior
  practice).
- **A2.** Yearly low-dose CT lung screening (eligible: 64, 40 pack-years): 2019-10-22,
  2020-11-17, 2021-11-16 and 2022-11-17 Lung-RADS 1; 2023-11-14 Lung-RADS 2 for a
  3 mm solid right upper lobe nodule; 2024-11-19 and 2025-11-18 Lung-RADS 2, unchanged.
- **A3.** The practice contacts that sent him out or followed up: 2022-03-08
  (pulmonology), 2022-09-13 (orthopedics), 2023-04-11 (hematology; CBC hemoglobin 9.2,
  WBC 2.6, platelets 84; weight 88.5 kg), 2024-03-19 (neurology; recall 2 of 3),
  2024-09-30 (phone), 2025-04-22 (post-ED, urology), 2025-10-16 (post-ED, cardiology),
  2026-04-24 (post-discharge).
- **A4.** Paget next steps: biopsy pathology as in the timeline; cholecalciferol 1,000
  units daily from 2022-11-22 (continued); zoledronic acid 5 mg IV once 2022-12-06;
  alkaline phosphatase 96 U/L on 2023-02-28 and normal after.
- **A5.** Myelofibrosis course: smear and core biopsy as above; JAK2 V617F detected;
  spleen 19 cm on ultrasound 2023-04-27; not a transplant candidate; ruxolitinib 5 mg
  twice daily from 2023-05-10; hematology every 3 months with the levels in the
  timeline.
- **A6.** Radiculopathy course: MRI 2024-10-18 as above; PT from 2024-11-12; resolved
  2025-01-27.
- **A7.** Hematuria course: CT urogram 2025-05-06 normal; cystoscopy 2025-05-13 as
  above; urinalysis 2025-06-03 clear.
- **A8.** Carotid sinus hypersensitivity course: avoidance advice; echo LVEF 60%; 14-day
  monitor without pauses; no pacemaker; no further syncope through 2026-06-12.
- **A9.** Admission course: BP 226/128 and creatinine 2.5 / BUN 40 on admission;
  nitroprusside stopped, hydroxocobalamin 5 g IV, nicardipine; lactate 1.6 on 04-15;
  cyanide 1.1 mg/L resulted 04-16; discharge 04-18 creatinine 1.6, BUN 28, BP 142/86;
  amlodipine 5 mg daily added; creatinine 1.4 on 2026-04-24 and 1.2 on 2026-06-12;
  BP 138/84 then 132/80.
- **A10.** CKD stage 2 basis: creatinine about 1.05 to 1.15 (eGFR about 65 to 75) and
  urine albumin/creatinine 40 to 70 mg/g on yearly checks; urinalysis protein negative.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal results.
- Hypertension/CKD visits about every 6 months, with BP, weight, basic metabolic panel,
  urine albumin/creatinine yearly; Welcome to Medicare 2020, annual wellness visits
  yearly from 2021, with PHQ-2 (normal), fall and function questions, lipids (his own
  normal), A1c (normal), PSA (about 2 to 2.5, stable, with BPH). CBC at the practice is
  normal before 2023; after 2023 his CBCs are drawn at hematology.
- Weight 94 kg at joining, about 93 in 2022, 88 in April 2023, 90 to 92 after.
- Prevention: Tdap 2019; influenza (high-dose or adjuvanted) each fall; zoster
  recombinant x2 (2020-10, 2021-02); COVID primary series early 2021 at a pharmacy, then
  boosters and the yearly updated vaccine; PCV20 2022; RSV 2023. Hepatitis C and HIV
  screening once at joining (negative). AAA screening ultrasound 2020 normal. FIT in
  2024 and 2025 negative (colonoscopy due 2024; with his platelets he chose FIT).
- Refills (90-day mail order), specialty-pharmacy and prior-authorization traffic for
  ruxolitinib, portal messages about results and scheduling, a cancelled and a missed
  visit, records requests.
- Booked visit after 2026-08-27: routine follow-up on 2026-09-14.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002272 on 2026-06-12. Practice contacts after that day are
only messages, refills and records; outside care after it still happens, and its
records are filed as received without changing the practice's lists. A visit is booked
after today.
