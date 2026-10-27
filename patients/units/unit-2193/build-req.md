# Build requirements: unit-2193 (Joseph Keane, MRN00002193)

Source: synthetic_hospital patient 2193, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, its "you note" quiz voice, or its unfinished plan
("According to the USPSTF guidelines,").

## Who he is (S)

Joseph Keane, born 1955-04-21, male, White, not Hispanic or Latino, English-speaking,
lives in Florence (01062). Medicare with a supplemental private plan. Retired long-haul
truck driver. Current cigarette smoker, about 1 pack a day for 40 years, 40 pack-years
(R1). Occasional social alcohol. No known drug allergies. No prior surgeries. Family
history: father had a myocardial infarction at 68, mother a stroke at 72, a brother with
type 2 diabetes. Marital status divorced (texture only; no family member appears).

Joined the practice on 2021-09-21 (A1, new-patient visit). He brought hypertension and
hyperlipidemia, on lisinopril 20 mg daily, hydrochlorothiazide 12.5 mg daily,
atorvastatin 40 mg daily and aspirin 81 mg daily, all continued unchanged to today.
Colonoscopy at his prior practice in 2015 was normal (S: "7 years ago" from 2022). He has
never had a PSA test. Height 178 cm, weight about 89 kg on joining (overweight).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2021-09-21 | practice | new-patient visit (A1): history above; BP 132/84 range; smoking counseled; baseline labs normal (P); records requested from prior practice |
| 2022-06-13 | practice | annual check-up, no complaints, adherent. BP 132/84; exam unremarkable. Hemoglobin 14.2, fasting glucose 98, TC 180, LDL 90, HDL 45, TG 150, creatinine 1.0. **Tobacco use disorder (nicotine dependence, cigarettes) put on the problem list.** Colonoscopy 2015 normal, next due 2025. PSA screening discussed as an individual choice at 67; he declines (R2) |
| 2022-11-08 | practice | referral visit (A2): breathlessness creeping up over a few years and a productive cough most mornings; referred to pulmonology for lung function testing. No chest imaging ordered (R7) |
| 2022-12-08 | outside | pulmonology consult letter: chronic productive morning cough, no recent infection or fever; SpO2 92% on room air; overweight, in no distress; prolonged expiratory phase with wheeze, decreased breath sounds at both bases; regular rhythm; no edema. PFTs: FEV1 1.2 L (45% predicted), FVC 2.0 L (60% predicted), FEV1/FVC 60%, DLCO 80% predicted. **COPD diagnosed.** **Tiotropium 2.5 mcg, 2 inhalations daily, and albuterol inhaler as needed started** (A3). Cessation urged |
| 2023-04-25 | practice | referral visit (A4): cramping calf pain after about two blocks of walking, eased by rest; weak pedal pulses; referred to cardiology |
| 2023-06-12 | outside | cardiology consult letter (received late, 2023-08-08, after a records request): calf cramping after two blocks, relieved by rest; no chest pain, palpitations or dyspnea on review. BP 142/88, HR 78. Diminished hair growth on the legs, no edema, weak pedal pulses both sides. Fasting lipids TC 210, HDL 40, LDL 140, TG 150. ABI 0.65 right, 0.70 left. **Peripheral arterial disease diagnosed.** Walking program, cessation; no drug change |
| 2023-09-19 | practice | annual wellness visit (P); mentions several months of trouble reading and recognizing faces; referred to ophthalmology (A5) |
| 2023-12-27 | outside | ophthalmology consult (R3 date; letter received 2024-03-05): gradual trouble reading and recognizing faces over months; no eye pain, redness or discharge. Decreased central acuity both eyes, 20/60 right and 20/50 left; peripheral fields full to confrontation. Fundus: drusen and pigment change in both maculae, no hemorrhage or fluid; Amsler grid with central distortion both eyes (R4). **Age-related macular degeneration, dry (nonexudative), both eyes.** Home Amsler grid, return in 6 months |
| 2024-02-20 | practice | follow-up; he reports the eye doctor's diagnosis before the letter arrives; AMD added from his report |
| 2024-06-12 | outside | cardiology follow-up letter: calf pain still after two blocks of walking and relieved by rest, worse in how it limits him; new erectile dysfunction. BP 142/88, HR 78. Hair loss on the legs, pallor on elevation, delayed capillary refill, diminished pulses in both feet. Fasting lipids TC 220, HDL 35, LDL 160, TG 150. ABI 0.65 right, 0.70 left, unchanged. **PAD with intermittent claudication (bilateral legs)** (R5); **erectile dysfunction** recorded, attributed to vascular disease. No drug change |
| 2024-07-23 | practice | follow-up after the cardiology letter (P): walking, BP, cessation counseling |
| 2024-12-10 | outside | pulmonology follow-up letter (received late, 2025-01-14): breathlessness worse over months; chronic cough with white sputum; no chest pain, fever or weight loss. SpO2 92% on room air. Increased AP chest diameter, decreased breath sounds, prolonged expiration with wheezes. Spirometry: FEV1 1.0 L (40% predicted), FVC 2.0 L (60% predicted), FEV1/FVC 50% (R6). Continue tiotropium and albuterol; cessation urged |
| 2025-03-04 | practice | COPD/HTN follow-up (P); FIT kit given (P) |
| 2025-06-09 | practice | routine check-up; follow-up of COPD, PAD and tobacco use; **unintentional loss of 10 pounds over 3 months**; no hemoptysis, fever or night sweats, and no change in his usual morning cough (R8). Afebrile, BP 132/84, HR 78, RR 16. Lungs clear to auscultation that day (R9). **Chest X-ray: solitary 1.2 cm right upper lobe nodule with spiculated margins**; no prior imaging for comparison. CT chest ordered. Labs for the weight loss (CBC, CMP, TSH) normal (P) |
| 2025-06-13 | outside imaging | CT chest: the 1.2 cm spiculated right upper lobe nodule confirmed, no other abnormality, no adenopathy. **Solitary pulmonary nodule, high suspicion for malignancy** |
| 2025-06-16 | practice (phone) | PCP gives the result; referral to pulmonology for PET-CT and tissue diagnosis (A6) |
| 2025-07 to 2025-10 | outside / practice | he cancels the pulmonology appointment and on a call with the PCP says he wants to wait; work-up not done (R10) |
| 2025-11-18 | practice | referral visit (A7): about 7 weeks of trouble climbing stairs and getting out of a chair, dry mouth, double vision now and then that settles with rest; no bowel or bladder change. Symmetric proximal weakness (hips and shoulders 4/5) on exam, reflexes reduced. CBC, BMP, CK and TSH normal (P; the neurology note's normal BMP and CBC). Urgent neurology referral; pulmonology re-referred and he agrees to go |
| 2025-12-22 | outside | neurology consult (age 70; letter received late, 2026-01-21): 3 months of difficulty with stairs and rising from a chair, dry mouth, intermittent diplopia improving with rest, no bowel or bladder trouble. Symmetric proximal weakness; deep tendon reflexes diminished, improving after sustained contraction. BMP and CBC normal. Chest X-ray: right hilar mass. **Lambert-Eaton myasthenic syndrome**, likely paraneoplastic. Antibody testing, EMG with repetitive stimulation, urgent PET-CT and pulmonology |
| 2025-12-30 | outside | EMG/NCS with repetitive nerve stimulation (A8): low CMAP amplitudes in all motor nerves tested (ulnar to abductor digiti minimi 1.9 mV); 3 Hz stimulation decrement 24%; increment 210% after 10 seconds of exercise; sensory responses normal; no myopathic units. P/Q-type VGCC antibody 0.41 nmol/L (reference 0.02 or less), positive (A8). **Amifampridine 10 mg three times daily started by neurology** (A8). Practice learns of it from a prior-authorization request on 2026-01-07 |
| 2026-01-09 | outside | PET-CT (A9): right upper lobe nodule now 1.9 cm, SUV max 8.6; right hilar (10R) nodal mass 3.1 cm, SUV max 11.4; right lower paratracheal (4R) node 1.5 cm, SUV max 6.2; no distant hypermetabolic disease |
| 2026-01-12 | outside | MRI brain with gadolinium (A9): no metastasis |
| 2026-01-16 | outside | EBUS-guided needle sampling (A10) of 10R and 4R: small cell carcinoma in both (synaptophysin and TTF-1 positive, Ki-67 about 85%); station 7 negative. **Small cell lung cancer, right upper lobe, limited stage (cT1b N2 M0, stage IIIA)** replaces the nodule entry |
| 2026-01-27 | practice | visit after the diagnosis (P): LEMS, lung cancer, amifampridine added to the lists; advance care planning discussed (P); he names his wishes, full treatment |
| 2026-01-29 | outside | medical and radiation oncology consult letter (A11): limited-stage SCLC; plan carboplatin AUC 5 day 1 with etoposide 100 mg/m2 days 1 to 3, every 21 days for 4 cycles, with concurrent once-daily thoracic radiation 60 Gy in 30 fractions; then durvalumab consolidation; brain MRI surveillance every 3 months instead of prophylactic cranial irradiation. Ondansetron 4 mg as needed for nausea |
| 2026-02-02 to 2026-04-06 | outside | 4 cycles of carboplatin/etoposide (2026-02-02, 02-23, 03-16, 04-06); thoracic radiation 2026-02-09 to 2026-03-20 (A11). Few cycle notes reach the practice; radiation completion summary received 2026-04-02 |
| 2026-04-14 | practice | telehealth visit (P): tired after treatment, eating; strength better, double vision gone |
| 2026-05-05 | outside | restaging CT chest (A12): right upper lobe primary 0.8 cm, 10R node 1.1 cm, 4R node 0.9 cm; post-radiation change in the right paramediastinal lung; no new disease. Partial response |
| 2026-05-18 | outside | **durvalumab 1500 mg IV every 4 weeks started** by oncology (A12); practice not notified |
| 2026-06-29 | outside | surveillance brain MRI (A12): no metastasis; report received |
| 2026-07-24 | practice | last visit (P): on durvalumab (learned from him; added), amifampridine continued, LEMS improved; CBC, CMP, TSH normal |
| 2026-08-10 / 08-11 | outside | surveillance CT chest stable, no new disease; oncology follow-up note, durvalumab continuing (A12). Filed on receipt 2026-08-19; practice lists not changed |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Pack-years are 30 (2022), 40 (most notes and the profile) and 45 (2025).
  Resolution: 40 pack-years throughout; 30 and 45 are not carried.
- **R2.** The 2022 plan stops mid-sentence after naming the USPSTF. Resolution: PSA
  screening is offered as an individual decision at 67 and he declines; the colonoscopy
  from 2015 is noted as due again in 2025. No quiz text is carried.
- **R3.** The ophthalmology visit falls on 2023-12-25, a holiday. Resolution: moved to
  Wednesday 2023-12-27 (age 68 either way).
- **R4.** The ophthalmology note's fundus and Amsler findings are "shown below" and never
  given. Resolution: the letter gives acuity 20/60 right and 20/50 left, macular drusen and
  pigment change without hemorrhage or fluid in both eyes, and central Amsler distortion
  both eyes: dry AMD, both eyes. No injections and no supplement prescription.
- **R5.** The source carries both "peripheral arterial disease" (2023) and "PAD with
  intermittent claudication" (2024), though claudication is present at both. Resolution:
  PAD goes on the list from the 2023 letter; the 2024 letter names atherosclerosis of the
  native leg arteries with intermittent claudication, and the practice adds that entry
  and resolves the general one at the 2024-07-23 visit.
- **R6.** The 2024 spirometry gives FVC 3.2 L (80%) with FEV1 1.0 L and a ratio of 31%,
  which cannot follow an FVC of 2.0 L (60%) two years earlier in worsening COPD.
  Resolution: FEV1 1.0 L (40% predicted) is carried; FVC is 2.0 L (60% predicted) and
  FEV1/FVC 50%. FVC 3.2 L, 80% and 31% are not carried.
- **R7.** The 2025 chest X-ray has "no previous imaging for comparison". Resolution: no
  chest imaging of any kind before 2025-06-09, including at the 2022 referral visit.
- **R8.** The 2025 note has him denying cough although his COPD cough is chronic.
  Resolution: he denies any new or changed cough and any hemoptysis.
- **R9.** Lungs clear in June 2025 after wheeze at both pulmonology visits. Resolution:
  different examiners on different days; kept as written.
- **R10.** Six months after the CT the neurologist still calls the nodule only "concerning
  for malignancy". Resolution: he cancels the July 2025 pulmonology appointment and tells
  the PCP he wants to wait; the work-up stalls until the weakness brings him back in
  November.
- **R11.** The source's medication sections split the same four drugs between prose and
  lists, sometimes garbled ("He takes for which he takes hydrochlorothiazide").
  Resolution: all four drugs at the same doses throughout. Weight runs about 88 to 90 kg
  to early 2025, 84 kg on 2025-06-09 (10 lb below), about 82 kg in November 2025, 80 kg in
  January 2026 and 78 to 79 kg by July 2026.
- **Left as the source has it:** no low-dose CT lung cancer screening was ever ordered in a
  40 pack-year smoker; LDL rises from 90 to 140 to 160 on atorvastatin 40 mg and no one
  intensifies it or rechecks lipids after 2024; cardiology BPs of 142/88 lead to no change;
  no smoking-cessation medication (counseling only); no inhaler change at the 2024
  pulmonology visit despite worse breathlessness; no treatment for erectile dysfunction.
  No note comments on these.

## Additions (A): physician to confirm

- **A1.** 2021-09-21 new-patient visit (the 2022 source visit is an established
  patient's annual check-up). Five years with the practice.
- **A2.** 2022-11-08 practice visit that sends him to pulmonology.
- **A3.** Tiotropium 2.5 mcg 2 inhalations daily and albuterol as needed, started by
  pulmonology for COPD the source never treats.
- **A4.** 2023-04-25 practice visit that sends him to cardiology for calf pain.
- **A5.** Ophthalmology referral at the 2023-09-19 wellness visit.
- **A6.** 2025-06-16 call and pulmonology referral for the nodule.
- **A7.** 2025-11-18 practice visit that sends him to neurology.
- **A8.** EMG with repetitive stimulation, VGCC antibody and amifampridine: the standard
  confirmation and treatment of the source's LEMS.
- **A9.** PET-CT and brain MRI staging for the hilar mass.
- **A10.** EBUS sampling naming the nodule's specific form: small cell carcinoma.
- **A11.** Standard limited-stage SCLC treatment: carboplatin/etoposide with concurrent
  thoracic radiation, ondansetron for nausea.
- **A12.** Restaging CT (partial response), durvalumab consolidation, brain MRI
  surveillance, August 2026 CT stable.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from the prior practice may mention routine screening done there, with normal
  results (the 2015 colonoscopy).
- Practice visits every 4 to 8 months, Medicare wellness visits, BMP or CMP yearly on the
  ACE inhibitor and thiazide, CBC; lipids only at joining (normal-range on statin) and in
  June 2022 as above. Blood pressure about 132/84 in the office. SpO2 about 93% after 2022.
- Prevention: high-dose influenza each fall 2021 to 2025, updated COVID vaccines each fall,
  PCV20 (2021), Shingrix two doses (2022), Tdap (2022), RSV (2023). Hepatitis C antibody
  once, nonreactive. One-time abdominal aortic ultrasound (ever-smoker, 65 to 75), normal
  caliber. FIT in 2025, negative. PHQ-2 screens negative; AUDIT-C low.
- Smoking-cessation counseling at visits; he is not ready to quit.
- Refills, prior authorizations (tiotropium, amifampridine), portal messages, a records
  request when a letter is late, a missed visit.
- Nothing booked after 2026-08-27.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002193 on 2026-07-24. Practice contacts after that day do not
happen beyond messages and refills; outside care after it still does, and its records are
filed as received without changing the practice's lists. Nothing is booked.
