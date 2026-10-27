# Build requirements: unit-1986 (Scott Dion, MRN00001986)

Source: synthetic_hospital patient 1986, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, its copied surgical-history list, or its quiz text
(trial-design puzzles, "the resident is instructed...").

## Who he is (S)

Born 1966-05-19, male, White, not Hispanic or Latino, English-speaking, commercial PPO,
lives in Granby, married (his wife is named in the source only as "his wife"; she comes
to visits late in the chart). Construction site supervisor, much of the day outdoors.
Never smoker, no alcohol. Allergy: sulfonamide antibiotics (rash), known before joining.
Family history: brother with melanoma, father with type 2 diabetes, mother with
hypertension. No surgery before joining.

Joined the practice on **2019-09-10** (new patient visit, A1), age 53, from a prior
primary-care office. He brought:

- type 2 diabetes, diagnosed about 2013 (S: "eight years ago" in late 2021), on
  **metformin 1000 mg twice daily**; earlier diet alone;
- hypertension, diagnosed about 2016 (S: "five years ago"), managed with diet, no drug
  (R1);
- hyperlipidemia on **atorvastatin 40 mg daily**;
- history of gout: a few flares years ago, none recently, no urate-lowering drug.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2019-09-10 | practice | new patient visit (A1); history as above; height 180 cm, weight 99 kg; screening labs (P) |
| 2021-05-18 | practice | annual physical (A2): wife has watched a mole on his upper back change over about 6 months; PCP sees a pigmented lesion there and refers to dermatology. Urine albumin/creatinine 168 mg/g (A3) |
| 2021-06-14 | outside | dermatology consult letter: changing lesion on upper back for 6 months, worried because of family skin cancer; upper back often uncovered at outdoor work; no fever, weight loss or night sweats; 1 cm pigmented lesion, asymmetric, irregular borders, brown to black; no palpable lymphadenopathy; dermoscopy with irregular streaks and a blue-white veil. Excisional biopsy the same day (A4) |
| 2021-06-24 | outside | dermatopathology received (A4): **nodular melanoma**, tumor thickness 0.7 mm, no ulceration, mitoses 2/mm², peripheral and deep margins clear by under 2 mm; stage IA. Wide local excision advised |
| 2021-07-13 | outside | **wide local excision of the upper back scar**, 1 cm margins, no residual melanoma (A5). The operative and pathology letters never reach the practice; he reports it at the October visit |
| 2021-09-14 | practice | lab draw: urine albumin/creatinine 192 mg/g (A3) |
| 2021-10-19 | practice | diabetes and blood pressure visit (A6): BP 144/90; persistent albuminuria. **Lisinopril 20 mg daily started** for blood pressure and kidney protection (S: started "two months" before the cough visit) |
| 2021-12-20 | practice | dry, irritating cough for 3 weeks, all day and worse at night; no fever, chills, chest pain, wheeze or dyspnea; T 36.8 °C, BP 136/88, pulse 78; heart and lungs normal; urinalysis with moderate protein (2+); creatinine and potassium normal. **Lisinopril adverse effect (ACE-inhibitor cough)**: **lisinopril stopped**, **ACE inhibitor intolerance (cough) recorded**, **hydrochlorothiazide 25 mg daily started** (R2) |
| 2022-03-15 | practice | follow-up: read about a clinical trial of a new blood pressure drug and wants to join; asks about benefits and risks; no chest pain, dyspnea or edema. BP 138/86, HR 78; exam unremarkable. Labs (drawn 2022-03-08): fasting glucose 146, A1c 7.2, creatinine 1.1, LDL 98, potassium 4.5. PCP goes over the consent paperwork with him (R3) |
| 2022-03-29 | outside | trial enrollment letter from the research site: randomized, blinded study drug added to his usual regimen; baseline BP 154/96 at the research site (R4) |
| 2022-09-06 | practice | six-month check while in the trial: adherent to the study drug and his usual medicine; BP 128/82. Labs (drawn 2022-08-30): fasting glucose 98, A1c 6.8, total cholesterol 180, LDL 95, creatinine 1.0, potassium 4.5 |
| 2023-02-21 | outside | trial close-out letter: study drug stopped, completed per protocol |
| 2023-03-07 | practice | follow-up after the trial: feels well, adherent; heard that several participants left because of side effects, group unknown; the study's summary letter reports no meaningful difference in blood pressure between the two drugs. BP 132/84; exam unremarkable. Labs (drawn 2023-02-28): fasting glucose 138, A1c 7.2, creatinine 1.1, LDL 98 |
| 2024-06-09 | outside | ED (Sunday): 2 days of throbbing right great toe pain that began suddenly at night, with redness and swelling; no injury. Right first MTP joint red, warm, exquisitely tender, motion limited by pain; rest of exam unremarkable. Arthrocentesis: synovial WBC 18,000/mm³ with 75% neutrophils, Gram stain without organisms, needle-shaped negatively birefringent crystals. **Acute gout**; **colchicine** (A7). His medicines then include hydrochlorothiazide and metformin. The ED note reaches the practice on 2024-07-02 |
| 2024-06-13 | practice | post-ED visit (P): toe better on colchicine; hydrochlorothiazide continued (left as the source has it) |
| 2025-05-19 | outside | urgent care: left-sided pneumonia, a week of an oral antibiotic, course completed (S: "diagnosed two weeks ago"). No record ever reaches the practice; it learns of it from the ED record and from him |
| 2025-06-02 | outside | ED, then admission: a week of worsening shortness of breath and dry cough, worse lying flat, more tired; never smoker; no other recent infection; no fever, chest pain or hemoptysis. Mildly tachypneic, hemodynamically stable; decreased breath sounds, dullness and reduced tactile fremitus at the left base. Chest x-ray: moderate left pleural effusion, about one third of the hemithorax. Diagnostic thoracentesis at the left midaxillary line (A8 fluid results). **Uncomplicated parapneumonic effusion**; small-bore **chest tube** 2025-06-03 (R5); IV ampicillin-sulbactam; tube out 2025-06-05; discharged 2025-06-06 on **amoxicillin-clavulanate 875/125 mg twice daily for 7 more days** (A9) |
| 2025-06-10 | practice | post-discharge visit (P): breathing near baseline |
| 2025-07-22 | practice-ordered | follow-up chest x-ray (A10): effusion resolved, lungs clear |
| 2026-06-09 | practice | **last seen** (A11): diabetes follow-up with his wife; about 6 weeks of headaches, worse in the morning, with nausea and some morning vomiting; she finds him more irritable. Neurological exam without focal deficit. Urgent MRI of the brain with and without contrast ordered |
| 2026-06-16 | outside | ED: 2 months of worsening headache, nausea and vomiting, spells of confusion, new right-sided weakness; wife reports he has grown lethargic and irritable. Alert but tired; right hemiparesis, right extensor plantar response (Babinski positive). Non-contrast head CT: large left hemispheric mass with midline shift. MRI brain with contrast: heterogeneous mass with serpiginous enhancement and central necrosis crossing the corpus callosum ("butterfly" pattern). Admitted to neurosurgery; dexamethasone (A12) |
| 2026-06-19 | outside | **craniotomy with debulking** of the left hemispheric mass; pathology **glioblastoma, IDH-wildtype, WHO grade 4**, MGMT promoter methylated (A12). Discharged home with his wife 2026-06-25 on a dexamethasone taper (A12) |
| 2026-07-20 | outside | chemoradiation begins (A13): 60 Gy in 30 fractions with daily temozolomide 75 mg/m², planned adjuvant temozolomide afterward; still under way on 2026-08-27 |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The dermatology note (practice date 2021-06) says his hypertension is
  "controlled with lisinopril", but the cough visit says lisinopril began only two months
  before 2021-12-20. Resolution: hypertension is diet-managed until lisinopril starts on
  2021-10-19; the dermatology letter lists metformin and atorvastatin only.
- **R2.** The source names no replacement for lisinopril, and every later list still
  carries lisinopril 20 mg daily; hydrochlorothiazide first appears at the 2024 gout ED
  visit. Resolution: lisinopril stops on 2021-12-20 and hydrochlorothiazide 25 mg daily
  starts that day. The practice's own notes and lists never carry lisinopril after that
  date (the 2022-03, 2022-09 and 2023-03 source mentions are dropped). The outside ED
  records of 2025 and 2026 may still copy lisinopril 20 mg on their medication lists and
  may omit hydrochlorothiazide, as outside lists do. The ACE-inhibitor intolerance (and
  the condition "lisinopril adverse effect") starts on 2021-12-20, not at the first visit.
- **R3.** Encounter 2 ends on a quiz about assignment by preference, while encounter 3
  calls the study a randomized controlled trial. Resolution: the study is randomized and
  blinded; the chart never discusses its design flaws. The source's statements about
  "preliminary results" (encounter 3) and dropouts and final results (encounter 4) stay
  only as what he reports and what the study's summary letter says.
- **R4.** BP 138/86 at the practice on 2022-03-15 and a "starting" BP of 154/96 for the
  study. Resolution: 154/96 is the research site's single baseline reading two weeks
  later; the practice records it only as reported in the enrollment letter.
- **R5.** The surgical history lists a chest tube for a parapneumonic effusion the source
  calls uncomplicated. Resolution: the fluid meets uncomplicated criteria (A8), and the
  tube is placed for drainage of a moderate, symptomatic effusion; it is out in 2 days.
- **R6.** Every source encounter, from 2021 on, lists the wide local excision, the chest
  tube and the craniotomy. Resolution: each happens at its own event (2021-07-13,
  2025-06-03, 2026-06-19) and no record mentions one before it happens. Likewise the
  melanoma, the ACE-inhibitor intolerance, acute gout and the effusion enter at their
  events, and the ACE-inhibitor allergy is absent before 2021-12-20.
- **R7.** Encounter 6 says "no recent infection" and also a pneumonia two weeks earlier.
  Resolution: the pneumonia is the only recent infection.
- **R8.** Encounter 5 ("He takes hydrochlorothiazide, metformin" with lisinopril and
  atorvastatin on the home list) and encounters 6 and 7 (no hydrochlorothiazide):
  the outside lists are copied, per R2; the practice list is metformin,
  hydrochlorothiazide and atorvastatin from 2021-12-20 on.
- **Left as the source has it:** hydrochlorothiazide continues after the gout flare;
  no uric acid level and no urate-lowering drug is ever raised. No ARB is tried after
  the ACE-inhibitor cough despite albuminuria. Hydrochlorothiazide is given with a
  listed sulfonamide-antibiotic allergy, and nobody remarks on it. The problem list
  keeps "Type 2 diabetes mellitus without complications" despite albuminuria.

## Additions (A): physician to confirm

- **A1.** 2019-09-10 new-patient visit (encounter 1 says his primary-care physician started
  his blood pressure drug, so he was already a practice patient; seed: 7 years with the
  practice).
- **A2.** The 2021-05-18 physical where the lesion is found and dermatology referral is made
  (source: noted "during routine skin examination").
- **A3.** Urine albumin/creatinine 168 mg/g (2021-05-18) and 192 mg/g (2021-09-14), giving the
  "kidney protection" reason for lisinopril and fitting the 2021-12 urinalysis protein;
  afterward it runs about 150 to 240 mg/g. Before 2021 it is normal.
- **A4.** Same-day excisional biopsy and its pathology (0.7 mm, no ulceration, mitoses 2/mm²,
  stage IA), so the source's nodular melanoma has a diagnosis.
- **A5.** Wide local excision details (1 cm margins, no residual tumor). Afterward,
  dermatology skin checks every 6 to 12 months, without new lesions; only some letters
  reach the practice.
- **A6.** The 2021-10-19 visit that starts lisinopril (BP 144/90).
- **A7.** ED treatment of the flare: colchicine 0.6 mg, 1.2 mg at once and 0.6 mg an hour
  later, then 0.6 mg twice daily until the flare settles (short course).
- **A8.** ED vitals: respiratory rate 22, SpO2 94% on room air, afebrile, BP and pulse normal. Pleural fluid: pH 7.36, glucose 112 mg/dL, LDH 310 U/L (serum 190), protein
  3.9 g/dL (serum 6.8), nucleated cells 3,200/µL, neutrophil-predominant, Gram stain no
  organisms, culture no growth, cytology negative for malignant cells. About 1,100 mL
  drained by the tube.
- **A9.** Inpatient IV ampicillin-sulbactam and discharge amoxicillin-clavulanate
  875/125 mg twice daily for 7 days.
- **A10.** The follow-up chest x-ray 7 weeks later, normal.
- **A11.** The 2026-06-09 visit where he reports the headaches (source: 2-month history at
  the 2026-06-16 ED) and the PCP orders an urgent MRI; the ED visit comes first, and the
  outpatient MRI order is cancelled on 2026-06-17.
- **A12.** Hospital course of the brain tumor: dexamethasone 10 mg IV in the ED, then 4 mg every 6 hours, then a
  taper at discharge; pathology details (IDH-wildtype, MGMT promoter methylated); hospital
  stay 2026-06-16 to 2026-06-25. Postoperative MRI 2026-06-20: subtotal resection, residual enhancing tumor across the corpus callosum. Right-sided weakness persists at discharge, somewhat better; he goes home with his wife and is not to drive.
- **A13.** Standard first-line therapy after resection: radiation 60 Gy in 30 fractions
  with concurrent temozolomide 75 mg/m² daily, starting 2026-07-20, run by radiation
  oncology and neuro-oncology; adjuvant temozolomide planned. The practice learns of the
  start from his wife on 2026-07-21; the neuro-oncology consult letter arrives 2026-08-18;
  radiation oncology sends nothing. None of it enters the practice's lists.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from the prior primary-care office (received 2019-10): a screening colonoscopy
  in 2017 without polyps (repeat in 10 years), Tdap in 2017.
- Diabetes and blood pressure visits about every 4 to 7 months, with vitals, A1c, basic or
  comprehensive metabolic panel, lipids yearly, urine albumin/creatinine yearly, foot exams
  with normal sensation and pulses. A1c runs 6.8 to 7.4 apart from the source's values.
  Office BP runs about 128 to 138 / 80 to 88 on hydrochlorothiazide. Weight 96 to 100 kg.
- Yearly diabetic eye exam letters from an optometrist, no retinopathy; some years none
  arrives.
- Prevention: hepatitis C and HIV screening once (2019, negative); influenza vaccine each
  fall; COVID-19 vaccines (primary series 2021 elsewhere, then yearly updates at the
  practice); recombinant zoster series 2020-2021; PCV20 in 2022; PHQ-2 yearly, negative.
- Refills, pharmacy and prior-authorization traffic, portal messages about results,
  refills and scheduling, a no-show in March 2026 (busy season at work), records requests.
- After 2026-06-09: his wife's calls, a metformin refill, and outside records filed as
  received. Nothing is booked.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001986 on 2026-06-09. Practice contacts after that day do not
happen; outside care after it still does, and its records are filed as received without
changing the practice's lists. Nothing is booked after today.
