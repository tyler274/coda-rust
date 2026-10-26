# Build requirements: unit-2223 (Herbert Clark, MRN00002223)

Source: synthetic_hospital patient 2223, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz-style plan text.

## Who he is (S)

Herbert Clark, born 1952-06-14, male, White, not Hispanic or Latino, English, Medicare,
lives in Southampton MA 01073, married (seed field; no spouse appears in the chart).
Retired accountant. Never smoker. Abstinent from alcohol, formerly an occasional
drinker. No illicit drugs.

Surgical history: diagnostic cardiac catheterization and percutaneous coronary
intervention with a drug-eluting stent to the LAD, both at an outside hospital in 2015
(R1).

Family history: father with myocardial infarction at 62 and hypertension; mother with
type 2 diabetes and a stroke at 70; no family history of amyloidosis or autoimmune
disease.

Allergies: none known on joining. Hydrochlorothiazide (angioedema and electrolyte
abnormalities) is recorded on 2022-01-29 (R5).

Joined the practice 2019-09-10 (A1; seven years with the practice by today), coming
from another primary-care practice whose records follow. He brought:

- Problems: essential hypertension; type 2 diabetes; coronary artery disease with an
  LAD stent (2015); chronic heart failure with reduced ejection fraction (LVEF 40% on an
  outside echo from 2018, A2); chronic kidney disease stage 3b; history of ischemic
  stroke (2017) with mild residual left-sided weakness, confined to the left arm and
  hand (R2).
- Drugs: lisinopril 20 mg daily, amlodipine 5 mg daily, metformin 1000 mg twice daily,
  furosemide 40 mg daily, aspirin 81 mg daily.

Not known on joining: spinal stenosis, AL amyloidosis, dysphagia, polymyositis,
hepatic dysfunction (each starts at its encounter below).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2019-09-10 | practice | new-patient visit (A1): history, problems and drugs as above; NKDA; labs drawn |
| 2019-09-24 | outside records | prior practice records arrive: problem list, 2015 cath/PCI report summary, 2017 stroke discharge summary (mild residual left-sided weakness), 2018 echo LVEF 40% (A2), 2016 colonoscopy without polyps and 2016 Tdap (P) |
| 2020-09-22 | practice | lower back pain radiating down the backs of both thighs for about 6 months, worse with walking and standing, eased by sitting or leaning forward on a shopping cart; MRI lumbar spine ordered; neurology referral (P) |
| 2020-10-13 | outside imaging | MRI lumbar spine: narrowing of the spinal canal with degenerative facet changes and ligamentum flavum hypertrophy (no levels or grades named beyond that) |
| 2020-10-29 | outside | neurology consult letter: the same history; lower-extremity strength full, peripheral pulses intact, straight leg raise negative both sides; MRI reviewed. **Lumbar spinal stenosis with neurogenic claudication diagnosed** (R3). Plan: physical therapy and **acetaminophen 500 mg up to 2 tablets three times daily as needed**; avoid NSAIDs given kidney function (A3) |
| 2020-11-12 | outside | physical therapy evaluation letter; a home program; PT discharge note early 2021 (A3) |
| 2021-01-06 | practice | about two months of increasing fatigue, exertional dyspnea and swelling of both legs; no chest pain, palpitations or syncope; referred to cardiology (P) |
| 2021-01-18 | outside | cardiology consult letter: same symptoms, no chest pain, palpitations or syncope; BP 150/90, HR 88, RR 18; low-pitched diastolic rumble at the apex; JVP elevated; 2+ pitting edema to mid-shins both legs; creatinine 2.1, BUN 45, albumin 3.2, total protein 8.5; urinalysis with protein, no blood or casts. Echo: increased ventricular wall thickness, normal chamber size, impaired diastolic filling, no significant valve disease, LVEF 38% (R4). Concern for an infiltrative cardiomyopathy; serum protein electrophoresis with immunofixation and serum free light chains sent, hematology referral (A4). No drug change |
| 2021-01-26 | outside lab | M-protein 1.8 g/dL, immunofixation IgG lambda; free lambda 186 mg/L, free kappa 11 mg/L, kappa/lambda ratio 0.06 (A4) |
| 2021-02-11 | outside | hematology consult letter: plan fat pad aspirate and bone marrow biopsy; 24-hour urine protein 2.4 g; NT-proBNP 4850 pg/mL; hs-troponin T 62 ng/L (A4) |
| 2021-02-25 | outside | pathology: abdominal fat pad aspirate Congo red positive with apple-green birefringence; mass spectrometry typing AL lambda (A4) |
| 2021-03-04 | outside | pathology: bone marrow biopsy with 12% lambda-restricted plasma cells; Congo red positive vascular deposits (A4) |
| 2021-03-11 | outside | hematology letter: **systemic AL (lambda) amyloidosis with cardiac and renal involvement diagnosed**; not a stem cell transplant candidate (heart and kidney involvement); **daratumumab, cyclophosphamide, bortezomib and dexamethasone (Dara-CyBorD) started 2021-03-15**: daratumumab subcutaneous and bortezomib 1.3 mg/m2 subcutaneous and cyclophosphamide 300 mg/m2 IV weekly at the infusion center; **dexamethasone 20 mg by mouth weekly** and **acyclovir 400 mg twice daily** at home (A5) |
| 2021-03-18 | practice | visit after the diagnosis; glucose monitoring on dexamethasone days; no diabetes drug change (P) |
| 2021-06-08 | practice | diabetes follow-up; A1c 8.6 on weekly dexamethasone; trouble swallowing for a few months, coughing with liquids; referred to gastroenterology (P) |
| 2021-08-02 | outside | GI consult letter: 6 months of difficulty starting a swallow, frequent coughing and choking when eating, worse with liquids, occasional nasal regurgitation; diabetes called poorly controlled (A1c 8.6, R6); stroke history noted (R2); slight slurring of speech, reduced gag reflex. Videofluoroscopic modified barium swallow the same day: delayed swallow initiation, vallecular pooling, laryngeal penetration without aspiration. **Oropharyngeal dysphagia diagnosed**; speech-language pathology referral (A6) |
| 2021-08-10 | outside | hematology letter: six cycles of Dara-CyBorD completed; free lambda 24 mg/L, kappa 12 mg/L (very good partial response); **dexamethasone, bortezomib and cyclophosphamide stop**; daratumumab every 4 weeks continues to two years total (A5) |
| 2021-08-24 | outside | speech-language pathology evaluation: swallow exercises, chin tuck with thin liquids, small sips; discharge to home program after a few sessions (A6) |
| 2021-11-09 | practice | BP 148/88, ankle edema persists; A1c 7.4 off dexamethasone; **hydrochlorothiazide 12.5 mg daily added** (R5) |
| 2021-11-23 | practice lab | BMP on hydrochlorothiazide: sodium 136, potassium 3.8 (P) |
| 2022-01-29 | outside | ED record: one week of progressive dyspnea, orthopnea and leg swelling; upper lip swelling the day before, gone by arrival (R5); T 37.0 C, BP 135/78, pulse 88, RR 20; JVD, S3 gallop, crackles both lungs, 2+ pitting edema of the legs; chest x-ray with pulmonary edema; sodium 129, potassium 3.1 (A7). IV furosemide given, potassium replaced; **hydrochlorothiazide stopped; hydrochlorothiazide adverse reaction and allergy (angioedema, electrolyte abnormalities) recorded**; overnight observation, home on furosemide 40 mg daily (R5) |
| 2022-02-02 | practice | post-ED visit; edema improved; sodium 135, potassium 4.0 (P); the reaction is resolved, the allergy stays |
| 2022-10-04 | practice | about a month of trouble getting up from chairs and climbing stairs, more tired; referred to rheumatology (P) |
| 2022-11-01 | outside | rheumatology consult letter: two months of difficulty rising from chairs and climbing stairs, more fatigue, occasional exertional dyspnea, no trauma; diabetes and blood pressure called well controlled (A1c 7.0, R6); symmetric weakness of deltoids, biceps and quadriceps both sides; no rash, sensory loss or joint swelling; reflexes preserved; CK 3200 U/L (reference 22-198), ESR 72 mm/hr. Inflammatory myopathy suspected; EMG and muscle biopsy ordered (A8) |
| 2022-11-15 | outside | EMG/NCS: myopathic motor units with fibrillation potentials in proximal muscles; normal nerve conduction (A8) |
| 2022-11-29 | outside | pathology: right vastus lateralis biopsy with endomysial inflammation, CD8 T cells invading non-necrotic fibers; Congo red negative (A8) |
| 2022-12-06 | outside | rheumatology letter: **polymyositis diagnosed**; **prednisone 60 mg daily started**; TPMT sent (A8) |
| 2023-01-10 | outside | rheumatology letter: TPMT normal; **azathioprine 100 mg daily started**; **prednisone to 40 mg daily** (A8) |
| 2023-03-09 | outside | hematology letter: two years of daratumumab complete, **daratumumab stops; acyclovir stops**; free lambda 21 mg/L, kappa 12 mg/L; surveillance every 6 months (A5) |
| 2023-02-14, 2023-05-16, 2023-08-15 | outside | rheumatology letters: CK 640 and **prednisone to 20 mg** (February); CK 210 and **prednisone to 10 mg** (May); CK normal and **prednisone to 5 mg daily, ongoing** (August); strength near normal by August (A8) |
| 2023-10-28 | outside | ED record: two days of worsening shortness of breath and swelling of both ankles; a recent episode of severe chest pain that went away on its own; BP 90/60, HR 110, RR 22, SpO2 92% room air; JVD, crackles at the bases, 2+ pitting edema both ankles; AST 2500, ALT 2300, total bilirubin 1.2, alkaline phosphatase 100, INR 1.1; ECG with ST depressions in the anterior leads; hs-troponin T 64 then 67 ng/L (A9). Admitted to cardiology |
| 2023-10-28 to 2023-11-03 | outside | admission (A9): **hepatic dysfunction (ischemic hepatitis from low blood pressure) diagnosed**; echo LVEF 35%, increased wall thickness, no new regional wall motion abnormality; lisinopril, amlodipine, metformin and azathioprine held; IV furosemide once blood pressure recovered; at discharge AST 180, ALT 610; home on furosemide 40 mg, aspirin, prednisone 5 mg |
| 2023-11-07 | practice | post-discharge visit: BP 138/84; AST 62, ALT 140 (A9); **lisinopril 20 mg and amlodipine 5 mg restarted** |
| 2023-12-05 | practice | liver tests normal (A9): **hepatic dysfunction resolved; metformin 1000 mg twice daily restarted**; rheumatology restarts **azathioprine 100 mg daily** (portal message) |
| 2023-12-12 | outside | cardiology follow-up letter after the admission: stable, no changes (A9) |
| 2024-08-27 | practice | chronic care visit: a month of being more winded on hills and evening ankle swelling (the start of the three months the admission describes); daily weights, cardiology visit moved up; no drug change (P) |
| 2024-10-26 to 2024-10-31 | outside | cardiology admission: three months of progressive exertional dyspnea and swelling of both ankles; BP 145/90, HR 88, RR 18; JVD, crackles at the bases, 2+ pitting edema both ankles, S3; sodium 138, potassium 4.5, creatinine 1.9 (R7), BNP 900, fasting glucose 160, A1c 7.2; chest x-ray with cardiomegaly and pulmonary congestion; ECG with left ventricular hypertrophy. **Acute decompensated heart failure diagnosed**; IV diuresis; home on the same drugs (R8); resolved at discharge |
| 2024-11-05 | practice | post-discharge visit (P) |
| 2024-12-10 | outside | cardiology follow-up letter: stable, no changes (A9) |
| 2025-11-04 to 2025-11-10 | outside | ICU then cardiology floor: two days of worse shortness of breath, three-pillow orthopnea, cannot walk more than a block for fatigue and breathlessness; no chest pain, fever or recent illness; "symptoms like a year ago"; T 36.8 C, BP 155/95, pulse 95, RR 22, SpO2 93% room air; normal S1 and S2 with an S3, bibasilar crackles, JVD, no peripheral edema; fundoscopy arteriolar narrowing without papilledema. **Acute decompensated heart failure diagnosed again**; IV diuresis; home on the same drugs; resolved at discharge |
| 2025-11-13 | practice | post-discharge visit (P) |
| 2025-12-09 | outside | cardiology follow-up letter: stable, no changes (A9) |
| 2026-08-26 | practice | routine chronic care visit; A1c and CMP drawn, pending today (P) |
| 2026-11-17 | practice | booked chronic care follow-up (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source gives the cath and the LAD stent without dates. Resolution: both at
  an outside hospital in 2015, known from prior records. No myocardial infarction is
  recorded; the indication is not stated.
- **R2.** The GI note calls the stroke "recent", yet stroke history is on every list from
  the first encounter, and neurology finds full lower-extremity strength while the
  problem list carries mild residual left-sided weakness. Resolution: the stroke was in
  2017, before he joined; the residual weakness is mild and in the left arm and hand, so
  neurology's normal leg strength and the rheumatology exam stand. "Recent" is not
  carried. The slight slurring of speech is the GI examiner's finding only.
- **R3.** The source names the condition "thoracic spinal stenosis" but describes
  neurogenic claudication and a lumbar spine MRI. Resolution: lumbar spinal stenosis
  with neurogenic claudication, diagnosed at the 2020-10-29 neurology visit. "Thoracic"
  is not carried.
- **R4.** Heart failure with reduced ejection fraction is on every list, but the
  cardiology echo gives no ejection fraction. Resolution: the 2021 echo reports LVEF
  38% alongside the source's findings; the 2018 echo (A2) was 40% and the 2023 echo (A9)
  35%. The diastolic rumble stays in the cardiology exam even though the echo shows no
  significant valve disease; no one reconciles it.
- **R5.** The hydrochlorothiazide allergy is on every list from the start, the reaction
  is "diagnosed" at the 2022 ED visit, and the 2024 cardiology note lists
  hydrochlorothiazide among his drugs. The ED plan "started" a loop diuretic though
  furosemide 40 mg is a home drug throughout. Resolution: the practice adds
  hydrochlorothiazide 12.5 mg on 2021-11-09 for blood pressure and edema; at the
  2022-01-29 ED visit he has hyponatremia and hypokalemia (A7) and reports upper lip
  swelling the day before; the ED stops hydrochlorothiazide and records the reaction
  and allergy; IV furosemide is the loop diuretic the ED starts, and he goes home on his
  usual furosemide 40 mg. The ED's "combination of medications" is not carried. The
  2024 admission note's medication list still copies hydrochlorothiazide, as outside
  lists do, beside the same allergy; the practice list never carries it after
  2022-01-29.
- **R6.** Diabetes is "poorly controlled" in August 2021 and "well controlled" in
  November 2022 on the same metformin. Resolution: weekly dexamethasone (March to August
  2021) raises the A1c to 8.6; off it the A1c is 7.4 (November 2021) and 7.0 (2022). High-
  dose prednisone in 2023 raises it to about 7.8, and it settles at 7.2 by October 2024.
- **R7.** Creatinine 2.1 in 2021 fits stage 3b CKD; creatinine 1.2 at the 2024
  admission does not. Resolution: creatinine runs 1.8 to 2.0 from 2019 on (eGFR about
  33 to 40); the 2024 admission value is 1.9. The 1.2 is not carried.
- **R8.** Outside lists move lisinopril, amlodipine, metformin and furosemide between
  "home medications" and prose from note to note. Resolution: all five home drugs
  continue unchanged at the doses above, except the holds and restarts of 2023 (A9).
- **R9.** Systemic AL amyloidosis is on the profile from the start and "diagnosed" at
  the cardiology encounter. Resolution: it is suspected at the 2021-01-18 cardiology
  visit and confirmed on 2021-03-11 (A4). The 2020 neurology letter does not carry it;
  the cardiology letter calls it suspected, not known.
- **R10.** Hepatic dysfunction stays on later outside lists. Resolution: the liver
  injury of October 2023 resolves by 2023-12-05; outside 2024 and 2025 notes may still
  copy it as history.
- **R11.** The source names acute decompensated heart failure as a diagnosis only from
  the 2024 admission. Resolution: the 2022 ED visit and 2023 admission are recorded as
  pulmonary edema and congestion under chronic HFrEF; the separate acute decompensated
  heart failure entry is made at the 2024 and 2025 admissions.
- **Left as the source has it:** no statin despite coronary disease and an LDL around
  100; no beta-blocker, SGLT2 inhibitor or mineralocorticoid antagonist for HFrEF, and
  amlodipine continues; metformin stays 1000 mg twice daily with an eGFR in the 30s;
  lisinopril continues after an angioedema event attributed to hydrochlorothiazide;
  furosemide is not increased after the 2021 cardiology visit or the admissions; no
  catheterization after the 2023 ST depressions. No note comments on any of these.

## Additions (A): physician to confirm

- **A1.** The 2019-09-10 new-patient visit (seven years with the practice; the source
  begins with established disease and home drugs).
- **A2.** The 2018 outside echo, LVEF 40%, so HFrEF has evidence on joining.
- **A3.** Physical therapy and acetaminophen as needed for lumbar stenosis, the
  conservative first step neurology would order; NSAIDs avoided for CKD.
- **A4.** The AL amyloidosis workup that a thickened, stiff heart with high total protein
  and proteinuria calls for: M-protein 1.8 g/dL IgG lambda; free lambda 186, kappa 11,
  ratio 0.06; 24-hour urine protein 2.4 g; NT-proBNP 4850; hs-troponin T 62; fat pad
  aspirate Congo red positive, typed AL lambda; marrow 12% lambda plasma cells.
- **A5.** Standard AL amyloidosis treatment run by hematology: Dara-CyBorD from
  2021-03-15 for six cycles with weekly dexamethasone 20 mg and acyclovir 400 mg twice
  daily; very good partial response by August 2021 (lambda 24, kappa 12);
  daratumumab every 4 weeks until March 2023 (lambda 21, kappa 12); then surveillance
  every 6 months. Every check after August 2021 has lambda 19 to 25 mg/L and kappa 11
  to 14 mg/L, no relapse through today. Hematology letters every 3 to 6 months while on treatment, then every 6 months.
- **A6.** Speech-language pathology swallow therapy for the dysphagia.
- **A7.** ED sodium 129 and potassium 3.1 (chloride at the low end with them), the electrolyte
  abnormalities the allergy names; both improving on the morning recheck before he
  leaves observation.
- **A8.** Standard polymyositis workup and treatment by rheumatology: EMG, muscle biopsy
  (Congo red negative), TPMT normal; prednisone 60 mg tapered to 5 mg daily by August
  2023; azathioprine 100 mg daily from 2023-01-10 (held 2023-10-28 to 2023-12-05); CK
  640 in February and 210 in May 2023, normal after; rheumatology letters every 3 to 6
  months.
- **A9.** The usual course of the 2023 ED visit: hs-troponin T 64 and 67 (flat), admission
  to cardiology to 2023-11-03, echo LVEF 35% with increased wall thickness and no new
  regional wall motion abnormality, ischemic hepatitis named as the liver injury,
  drugs held as listed, AST 180 / ALT 610 at discharge, AST 62 / ALT 140 on 2023-11-07,
  normal on 2023-12-05; cardiology follow-up letters after each admission (2023-12-12,
  2024-12-10, 2025-12-09) that change nothing.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level here says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal
  results (colonoscopy 2016 without polyps; Tdap 2016).
- Chronic care visits every 2 to 4 months (in person, some telehealth in 2020), Medicare
  annual wellness visits yearly, and post-ED and post-discharge visits. Vitals each
  visit; BMP or CMP, A1c every 3 to 6 months; lipids and urine albumin/creatinine yearly;
  CBC yearly; foot exams.
- Levels: height 176 cm; weight about 84 kg, up 2 to 4 kg with edema and on high-dose
  prednisone; office BP 130s/80s apart from the readings above; creatinine 1.8 to 2.0;
  BUN 28 to 40; glucose on chemistry panels 135 to 155, higher on dexamethasone and
  high-dose prednisone; TSH normal; albumin 3.6 to 3.9 before 2021, 3.2 to 3.5 after; total protein 7.6 to
  8.1 before 2021, 7.0 to 7.6 after treatment; A1c 7.2 to 7.4 in 2019 and 2020, then as
  in R6, about 7.1 to 7.3 in 2025 and 2026; LDL 95 to 110, HDL about 42, triglycerides
  about 160; urine albumin/creatinine 150 to 220 mg/g in 2019 and 2020, 600 to 800 mg/g
  from 2022; CBC normal; CK normal before 2022 and after mid-2023.
- Yearly diabetic eye exam letters from an optometrist: no diabetic retinopathy.
- Prevention: high-dose influenza vaccine each fall; Shingrix in early 2020 (two doses);
  COVID mRNA vaccines in 2021 and the yearly updated doses; PCV20 in 2022; RSV (Arexvy)
  in fall 2023; hepatitis C and HIV screening once, negative; PHQ-2 yearly, negative.
  No PSA or further colorectal screening at his age and burden of illness, by his choice.
- Refills, pharmacy and prior-authorization traffic, portal messages about results,
  refills and scheduling, home weight and glucose logs, a missed visit, reminders.
- Booked visit after 2026-08-27: chronic care follow-up on 2026-11-17.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00002223 on 2025-11-13. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. Nothing is booked after today.
