# Build requirements: unit-2836 (Svetlana Foley, MRN00002836)

Source: synthetic_hospital patient 2836, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text.

## Who she is (S)

Born 1953-07-31, female, White, not Hispanic or Latino, English, Medicare Part A and B,
South Hadley. Retired high school librarian. Former smoker, quit about 20 years ago
(10 pack-years). Wine 1 to 2 glasses a week. Single (P); "her family" comes with her to
palliative care, and the chart names no relative beyond the family history.

- **Allergies:** penicillin (anaphylaxis); iodinated contrast (urticaria/hives).
- **Surgical history:** left modified radical mastectomy with sentinel lymph node
  biopsy in 2014, followed by adjuvant chemotherapy (agents not in the records) and
  letrozole (R1); total abdominal hysterectomy with bilateral salpingo-oophorectomy at
  age 50 (2003); laparoscopic cholecystectomy at age 45 (1998).
- **Family history:** mother invasive breast cancer at 62, deceased; father
  hypertension, chronic gout, recurrent kidney stones; sister severe osteoporosis with
  compression fractures.

**Joining (A1).** New-patient visit 2016-04-12 (age 62), after moving her care from a
prior primary-care office. She brings: essential hypertension on **lisinopril 20 mg
daily** and **hydrochlorothiazide 25 mg daily**; hyperlipidemia on **atorvastatin 20 mg
daily**; history of left breast cancer (2014) on **letrozole 2.5 mg daily**, followed by
an outside oncology group; degenerative osteoarthritis (knees, hands) using
**acetaminophen 650 mg every 6 hours as needed**; osteoporosis (known from a prior
DXA, on no drug); history of a passed calcium oxalate kidney stone. Height 163 cm,
weight 56 kg.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2016-04-12 | practice | new-patient visit (A1), history as above |
| 2019-08-05 | practice | yearly wellness visit; CMP calcium 10.9 mg/dL (A2) |
| 2019-08-12 | practice | lab draw: calcium 11.0, intact PTH 96 pg/mL, phosphorus normal (A2) |
| 2019-08-14 | practice | telephone: **primary hyperparathyroidism diagnosed** (A2); referred to endocrinology |
| 2019-10-02 | outside | endocrinology consult (letter received 2019-10-10): several months of heavy thirst, drinking water all day, waking several times a night to urinate; tiredness and mild muscle weakness. BP 138/82, pulse 80. Thin, no distress, neuro exam normal. Sodium 148 mEq/L, calcium 11.4 mg/dL, serum osmolality 305 mOsm/kg, urine osmolality 120 mOsm/kg. Water deprivation test: urine osmolality essentially unchanged; desmopressin then fails to concentrate the urine. Impression: polyuria from a hypercalcemia-related renal concentrating defect, in primary hyperparathyroidism (R2). Plan: drink to thirst, repeat calcium and PTH, endocrine follow-up (R2) |
| 2019-12-05 | practice | telephone (P): left knee swollen and painful since yesterday; arranged to rheumatology next day |
| 2019-12-06 | outside | rheumatology (letter received late, 2020-01-21): left knee pain and swelling 2 days, no trauma; months of polyuria and constipation. Left knee warm, swollen, tender, motion limited by pain. Calcium 11.8 mg/dL, phosphorus 2.9 mg/dL, PTH elevated (no number in the letter), creatinine 1.2 mg/dL, ESR 30 mm/hr. Knee x-ray: chondrocalcinosis of the articular cartilage. Arthrocentesis: rhomboid, positively birefringent crystals. **Acute pseudogout of the left knee in CPPD disease diagnosed** (R3); intra-articular triamcinolone 40 mg after aspiration (A3) |
| 2020-11-16 | practice | office visit (P): about 6 weeks of low back pain, worse at night, not helped by over-the-counter analgesics; lumbar tenderness, neuro normal; the PCP calls her oncologist, who brings her visit forward and orders a bone scan |
| 2020-12-10 | outside | oncology (letter received 2020-12-22): 2 months of low back pain worse at night, OTC analgesics not helping, no trauma. Lumbar vertebral tenderness, neuro exam normal. Calcium 11.2 mg/dL, albumin 4.0 g/dL, alkaline phosphatase 120 U/L, creatinine 0.9 mg/dL. Bone scan: increased uptake in multiple lumbar vertebrae. **Metastatic carcinoma to bone diagnosed: recurrent breast cancer, now stage IV** (R1). Plan: continue letrozole; **start morphine sulfate extended-release 30 mg every 12 hours and gabapentin 300 mg three times daily** (R4) |
| 2021-03-09 | practice | follow-up (P); constipation on morphine: **polyethylene glycol 17 g daily** (A4) |
| 2021-12-13 | practice | office visit: 68 years old; right great toe pain, swelling and redness for 2 days, sudden onset overnight, worsening; no fever, chills, trauma or earlier joint attacks. T 36.8 C, BP 130/80, pulse 76, RR 16. Right first MTP swollen, red, exquisitely tender. Uric acid 10.2 mg/dL. **Acute gout attributed to hydrochlorothiazide (drug-induced gout) diagnosed** (R5); **colchicine short course** (A5); hydrochlorothiazide continued (Left) |
| 2022-11-28 | outside | ED, then admission to 2022-12-01 (discharge summary received 2022-12-12): 69 years old; 2 weeks of severe throbbing right temporal headache, worse combing her hair; jaw claudication when chewing; low-grade fevers, tiredness; blurred vision in the right eye since that morning. Temporal arteries prominent and tender; right visual acuity decreased; right optic disc pale with blurred margins. ESR 85 mm/hr, CRP 4.5 mg/dL (45 mg/L), hemoglobin 11.2 g/dL, platelets 450,000/uL. **Giant cell arteritis diagnosed** (R6). IV methylprednisolone 1 g daily for 3 days, ophthalmology consult, temporal artery biopsy on 2022-11-30 showing granulomatous arteritis (A6); right-eye vision still reduced at discharge; **discharged on prednisone 60 mg daily** with rheumatology follow-up (A6) |
| 2022-12-07 | practice | post-discharge visit (P): prednisone 60 on the list; rheumatology referral confirmed |
| 2023-01-11 | outside | rheumatology follow-up letter (A7): headache and jaw pain gone; ESR 22 mm/hr, CRP 0.6 mg/dL; prednisone taper over about a year |
| 2023-11-20 | outside | oncology letter (A8, received only 2024-01-08): prednisone finished per rheumatology on 2023-11-01; **dexamethasone 4 mg daily started for bone pain** |
| 2024-04-12 | outside | ED (no ED record ever reaches the practice; the patient reports it on 2024-04-15): a month of worsening confusion, constipation, weakness, tiredness and poor appetite; nausea and vomiting lately. Dry mucous membranes, diminished reflexes, mild abdominal tenderness; no lymph nodes, liver or spleen enlargement, non-focal neuro exam. Skeletal survey: multiple lytic lesions of the vertebrae, pelvis and ribs. Noncontrast head CT: no acute abnormality. ECG: short QT interval. (The imaging and ECG reports arrive 2024-04-22.) **Malignancy-associated hypercalcemia diagnosed** (R7). Treated with IV fluids and IV zoledronic acid, sent home (A9) |
| 2024-05-11 | outside | ED to ICU, admitted to 2024-05-16 (discharge summary received 2024-06-05): 1 week of progressive weakness, constipation and confusion. Lethargic, oriented to person and place, not time; abdomen not distended, hypoactive bowel sounds; weakness in all limbs. Calcium 14.2 mg/dL, creatinine 1.8 mg/dL, sodium 142, potassium 4.0, phosphorus 2.9, albumin 4.0, PTH 10 pg/mL. Recurrent malignancy-associated hypercalcemia. Treated with IV saline, calcitonin and denosumab 120 mg (A10); **hydrochlorothiazide stopped** (A10); calcium 10.1 and creatinine 1.3 at discharge (A10) |
| 2024-05-23 | practice | post-discharge visit (P); hydrochlorothiazide off the practice list |
| 2024-06 | outside | oncology (no letter reaches the practice; a pharmacy fill shows it 2024-07-09): **capecitabine 1500 mg twice daily, 14 days on and 7 off, started; letrozole stopped**; monthly denosumab 120 mg at the infusion center (A11) |
| 2024-08 | outside | first palliative care consult (A12; the letter never arrives; the patient mentions it) |
| 2025-09 | outside | oncology: capecitabine stopped for progression, **eribulin started** (A13; the practice learns of it from the palliative letter) |
| 2025-10-13 | outside | palliative care (letter received 2025-10-16; the palliative physician also phones the PCP on 2025-10-14): 72 years old, comes with her family; several lines of chemotherapy, a new regimen recently started; persistent pain and poor sleep despite analgesics; says she wants to "just end it all" and asks about physician-assisted death; tired, has lost weight. Vitals stable. Alert, distressed; diffuse bony tenderness. Hemoglobin 10.2 g/dL, calcium 10.5 mg/dL, alkaline phosphatase 240 U/L, creatinine 1.1 mg/dL. **Suicidal ideation** recorded. On direct questioning a wish to die without a plan or intent, no firearms at home (A14); safety plan made with her and her family; **mirtazapine 7.5 mg at bedtime** for sleep and mood (A14); psychiatry (psycho-oncology) referral (A14) |
| 2025-10-16 | practice | office visit (P): PCP follows up the palliative call; PHQ-9 14; suicidal ideation on the problem list; safety plan reviewed; psychiatry referral placed |
| 2025-11-20 | outside | psychiatry (psycho-oncology) letter (A14): supportive therapy every 2 weeks, continue mirtazapine 7.5 mg |
| 2026-07-31 | practice | last practice visit (last seen); booked follow-up 2026-09-14 |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source calls the breast cancer stage IV from the start, gives the
  mastectomy as "5 years prior" (2014) with adjuvant chemotherapy, and later says "2
  years ago". Resolution: early-stage left breast cancer treated in 2014 (mastectomy,
  sentinel node biopsy, adjuvant chemotherapy, then letrozole, so hormone-receptor
  positive); "History of breast cancer" until the bone scan of 2020-12-10, when it
  becomes metastatic (stage IV) breast cancer with bone metastases. "2 years ago" is
  dropped.
- **R2.** The source labels the 2019 hypercalcemia "malignancy-associated" although no
  metastasis is known until December 2020 and the PTH is elevated. Resolution: in 2019
  the hypercalcemia is primary hyperparathyroidism ("recently diagnosed" in summer
  2019, A2); the water deprivation and desmopressin results are read as a
  hypercalcemia-related concentrating defect, written in the endocrinology letter
  only, not as a separate problem. Malignancy-associated hypercalcemia begins in 2024
  (R7). Source text "prior hypercalcemia in 2020" follows the re-dated timeline (2019).
- **R3.** CPPD is on the profile from the start. Resolution: CPPD disease with acute
  pseudogout of the left knee is diagnosed at rheumatology on 2019-12-06.
- **R4.** Morphine SR, gabapentin and dexamethasone are listed from 2019, before the
  bone pain they treat. Resolution: morphine SR 30 mg every 12 hours and gabapentin 300
  mg three times daily start at oncology on 2020-12-10; dexamethasone 4 mg daily starts
  at oncology on 2023-11-20 (A8), after the GCA prednisone ends. The 2019 to 2022 lists
  that carry them are not carried. Outside letters later may copy stale lists.
- **R5.** The source says she was "recently started on a diuretic" in December 2021,
  while hydrochlorothiazide is on every list from 2019. Resolution: hydrochlorothiazide
  25 mg has been taken continuously since before joining; "recently started" is
  dropped; the gout is attributed to the thiazide.
- **R6.** "Giant cell arteritis or autoimmune inflammatory condition" resolves to
  giant cell arteritis (M31.6), supported by the biopsy (A6).
- **R7.** Encounter 5's ED record gives no calcium. Resolution: the ED record never
  reaches the practice; the patient reports the high calcium and treatment; the chart
  carries no calcium number for that day. Hypercalcemia (malignancy-associated) is
  recorded from her report on 2024-04-15.
- **R8.** Home medication lists in encounters 5, 6 and 7 drop and re-add letrozole,
  lisinopril and hydrochlorothiazide. Resolution: lisinopril 20 mg continues
  throughout; letrozole runs to June 2024 (A11); hydrochlorothiazide stops 2024-05 (A10).
  The 2025 palliative letter may copy stale entries (letrozole, hydrochlorothiazide),
  as outside lists do.
- **Interobserver/texture:** none.
- **Left as the source has it:** primary hyperparathyroidism is never sent for
  parathyroid surgery and gets no drug; hydrochlorothiazide is continued through the
  2019 hypercalcemia, the 2021 gout and the April 2024 ED visit; osteoporosis gets no
  drug before the 2024 denosumab; no bone-modifying agent after the 2020 bone scan; no
  urate-lowering drug after the gout; letrozole is continued after the metastases are
  found. No note comments on these.

## Additions (A): physician to confirm

- **A1.** 2016-04-12 new-patient visit (ten years with the practice); what she brings.
- **A2.** Primary hyperparathyroidism needs its diagnosis ("recently diagnosed"):
  calcium 10.9 on 2019-08-05, then calcium 11.0 and intact PTH 96 pg/mL on 2019-08-12,
  phosphorus normal; endocrinology referral.
- **A3.** Pseudogout treatment: intra-articular triamcinolone 40 mg into the left knee
  after aspiration; the patient reports it settled within days.
- **A4.** Polyethylene glycol 17 g daily for constipation on morphine (2021-03-09).
- **A5.** Colchicine for the gout flare: 1.2 mg, then 0.6 mg an hour later, then 0.6 mg
  twice daily until the flare settles (short course); the flare settles within a week.
- **A6.** GCA care: IV methylprednisolone 1 g daily for 3 days; temporal artery biopsy
  2022-11-30, granulomatous arteritis; discharge on prednisone 60 mg daily.
- **A7.** Rheumatology follow-up 2023-01-11: symptoms gone, ESR 22 mm/hr, CRP 0.6 mg/dL;
  prednisone tapered over about a year, off on 2023-11-01.
- **A8.** Dexamethasone 4 mg daily (source) starts at oncology on 2023-11-20 for bone pain.
- **A9.** The April 2024 ED treats the hypercalcemia with IV fluids and IV zoledronic acid.
- **A10.** ICU treatment: IV saline, calcitonin, denosumab 120 mg; hydrochlorothiazide
  stopped; discharge calcium 10.1 mg/dL, creatinine 1.3 mg/dL.
- **A11.** First chemotherapy line (source: "multiple lines"): capecitabine 1500 mg twice
  daily, days 1 to 14 of 21, from June 2024; letrozole stopped; denosumab 120 mg monthly.
- **A12.** A first palliative care consult in August 2024 (source: she "returns").
- **A13.** Second line (source: "recently started a new regimen"): eribulin from
  September 2025, after capecitabine.
- **A14.** Palliative plan 2025-10-13: wish to die without plan or intent, no firearms;
  safety plan; mirtazapine 7.5 mg at bedtime; psycho-oncology referral, seen 2025-11-20
  (supportive therapy every 2 weeks, mirtazapine continued).
- **A15.** Ongoing oncology and palliative follow-up through 2026 continues; letters that
  arrive carry no new findings, diagnoses or drugs.

## Levels the chart carries (A, P)

- Weight: 56 kg (2016), 54 kg (2019), 51 kg (2024-05), 47.5 kg (2025-10), 47 kg (2026-07).
- Calcium: normal-high (about 10.2) 2016 to 2018; 10.9 and 11.0 in August 2019 (A2);
  about 10.3 to 11.5 from 2020 through early 2024 at practice draws; 9.6 to 10.3 on denosumab after
  May 2024; 10.5 in October 2025 (S); about 10.3 in 2026.
- Creatinine about 0.9 to 1.0 at practice draws, 1.1 to 1.2 after May 2024.
- Hemoglobin normal until 2022; 11.2 at the GCA visit (S); about 11.0 in 2024; 10.2 in
  October 2025 (S); about 10 to 11 in 2026. Alkaline phosphatase normal until 2024, about 180
  in 2024, 240 in October 2025 (S), about 230 in 2026.
- Uric acid 10.2 at the gout visit (S) only.
- Lipids on atorvastatin 20: LDL about 95 to 110, no change of drug.
- BP normal on lisinopril and hydrochlorothiazide; on lisinopril alone after May 2024,
  around 134/82.
- PHQ-2 0 at yearly screens through 2024; PHQ-9 14 (2025-10-16), 11 (2026-01-20), 8
  (2026-04-14), 7 (2026-07-31).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Records from the prior practice: colonoscopy 2015 normal; mammogram normal.
- Visits for hypertension, lipids and her cancer course every 3 to 6 months; yearly
  wellness visits (Welcome to Medicare in 2018, then annual wellness visits); CMP, CBC,
  lipids yearly.
- Right-breast mammogram yearly 2016 to 2019, normal; after 2020 imaging belongs to
  oncology. Colorectal screening not repeated after the 2015 colonoscopy.
- Influenza each fall; Tdap 2016; Shingrix 2018; PCV13 2018 and PPSV23 2019; COVID
  vaccines from 2021; RSV 2023. Hepatitis C and HIV screening once (2016), negative.
- Refills, pharmacy and prior-authorization traffic, portal messages about results,
  refills and scheduling, a missed visit, reminders; records requests to outside
  offices, with letters arriving late or not at all as the timeline says.
- Booked visit after 2026-08-27: routine follow-up on 2026-09-14.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002836 on 2026-07-31. Practice contacts after that day are
only messages, refills and records; outside care after it still happens, and its records
are filed as received without changing the practice's lists. A visit is booked after
today.
