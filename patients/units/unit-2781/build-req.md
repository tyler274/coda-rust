# Build requirements: unit-2781 (Melissa Friedman, MRN00002781)

Source: synthetic_hospital patient 2781, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, its second-person quiz voice ("you suspect"), or its
cut-off plan ("Before proceeding with thyroidectomy,").

## Who she is (S)

Born 1978-11-24, female, White, not Hispanic or Latino, English-speaking, lives in
Easthampton, employer-sponsored PPO. High school mathematics teacher. Never smoker. One
or two drinks a week, socially. NKDA. Surgical history: tonsillectomy (1995) and a
cesarean section (2015) (R1). Family history: father with medullary thyroid carcinoma
diagnosed at 48, who died in his 40s; a paternal aunt with pheochromocytoma; mother with
essential hypertension (R2).

Joined the practice in 2022 (first visit 2022-08-16, a new-patient visit, A1), age 43:
well, no chronic conditions, no medications, prior screening at her previous practice
normal. Nothing about blood pressure episodes, the adrenal or the thyroid is known at
joining (R3).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-08-16 | practice | new-patient visit (A1); history as above; normal exam and BP; prevention brought up to date (P) |
| 2024-10-22 | practice | referral visit (P): for a few weeks, episodes that come without warning of a fast heartbeat felt in the chest, a throbbing headache and flushing, each over in minutes, with more tiredness than usual and some nights of poor sleep; seated BP normal today, normal exam; referred to cardiology |
| 2024-11-07 | outside | cardiology consult letter: episodes as above, several minutes each; tiredness, sleep sometimes poor; she denies chest pain, any change in vision and weight loss; takes no medications; her father died in his 40s of a "thyroid problem", no records (R2). Seated BP 120/76; standing up from the chair, BP 198/102. **Plasma metanephrines high**; the rest of the screening bloodwork in range; ECG and chest x-ray show nothing abnormal. **Paroxysmal hypertension** recorded, with **pheochromocytoma suspected** on the metanephrines (R3) |
| 2024-11-19 | practice | follow-up after the cardiology letter (P): result reviewed with her; endocrinology referral placed; no medication started (left as the source has it) |
| 2025-04-22 | practice | visit (P) for a lump she has noticed at the front of her neck and diarrhea; a firm thyroid nodule is felt; endocrinology appointment moved up |
| 2025-05-09 | outside | endocrinology consult letter: palpable neck mass and diarrhea. Family history now: father had thyroid cancer, a paternal aunt pheochromocytoma; the family pattern is recorded as MEN 2A (R2). BP 145/90, HR 98. On exam, a firm thyroid nodule, not tender, at the mid-portion of the gland. Calcium 10.2 mg/dL (reference 8.5 to 10.2), calcitonin 500 pg/mL (reference under 10), CEA 10 ng/mL (reference under 5). **Medullary thyroid carcinoma suspected.** Plan: the pheochromocytoma is confirmed and removed first, thyroidectomy after (R4); repeat catecholamine testing, adrenal CT, neck ultrasound and germline RET testing ordered (A2, A3) |
| 2025-06-03 | outside | plasma and 24-hour urine catecholamines and metanephrines, all far above range (S, reported in the November surgical note; the date is this file's, R5). Adrenal CT: a single mass on the right adrenal whose appearance points to pheochromocytoma (S; details per A2) |
| 2025-06-05 | outside | thyroid and neck ultrasound (A3) |
| 2025-06-19 | outside | genetics letter: germline RET result (A4); **multiple endocrine neoplasia type 2A** added to the problem list; genetic counseling given |
| 2025-11-10 | outside | general surgery preoperative visit before removal of the right adrenal mass: a few times each week, throbbing headaches with palpitations and sweating, lasting 20 to 30 minutes (R6); her home cuff has caught readings above 180/100 during them, with normal pressures in between. Admitted to a regional hospital to prepare for surgery and **phenoxybenzamine started** (A5); day by day her pressure comes down, with no meaningful drop on standing, and the palpitations grow milder (R6) |
| 2025-11-14 | outside | **laparoscopic right adrenalectomy** (A5); **phenoxybenzamine stopped**; discharged 2025-11-17; the practice receives the operative note, pathology and discharge summary. **Pheochromocytoma confirmed**; paroxysmal (secondary) hypertension resolves |
| 2025-12-02 | practice | post-discharge visit (P): no further spells, BP normal off all drugs, wounds healing; thyroid surgery planned with the surgical group |
| 2025-12-18 | outside | plasma free metanephrines normal (A5) |
| 2026-01-22 | outside | **total thyroidectomy with central neck dissection** (A6); discharged next day; **levothyroxine started**. **Medullary thyroid carcinoma confirmed** |
| 2026-02-03 | practice | post-operative visit (P): incision healing, voice normal, no tingling or cramps, calcium normal; diarrhea settling |
| 2026-03-05 | practice | TSH in range on levothyroxine (A6) |
| 2026-04-23 | outside | endocrinology follow-up: calcitonin and CEA after surgery (A6); diarrhea gone; next endocrinology visit booked for 2026-10-22 |
| 2026-08-18 | practice | annual physical (P); well; BP normal; no spells |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source dates her tonsillectomy 1990 and cesarean 2010. Those years follow the
  re-dated timeline (about five years later), keeping her ages at each about where the
  source had them: tonsillectomy 1995 (age 16), cesarean 2015 (age 36). The chart names no
  child.
- **R2.** The family history changes across notes: cardiology hears only that her father
  died of a "thyroid problem" in his 40s with no records; endocrinology hears thyroid cancer
  in the father and pheochromocytoma in a sister; surgery lists the father's medullary
  thyroid carcinoma at 48, a paternal aunt with pheochromocytoma and a mother with essential
  hypertension. Resolution: the profile's set is carried (father, paternal aunt, mother).
  The father was diagnosed at 48 and died in his 40s; she learned the details between the
  cardiology and endocrinology visits. **The sister is not carried**: she has no sister with
  pheochromocytoma. "Family history of MEN 2A" at endocrinology means the family pattern
  points to it; no relative's genetic test is documented, and her own RET result (A4) is
  what establishes it.
- **R3.** Cardiology lists paroxysmal hypertension on her problem list while saying she has
  no chronic illnesses, and later notes stamp pheochromocytoma as diagnosed at that first
  visit. Resolution: she has no chronic illness before autumn 2024. Paroxysmal hypertension
  is recorded at the November 2024 cardiology visit from the postural spike. Pheochromocytoma
  is suspected there on the metanephrines, supported by biochemistry and CT in June 2025,
  and confirmed by pathology in November 2025. MEN 2A enters the chart with the RET result.
- **R4.** The endocrinology plan stops at "before proceeding with thyroidectomy". Resolution:
  the plan is to confirm and remove the pheochromocytoma first, then do the thyroidectomy.
  The quiz text is not carried.
- **R5.** The surgical note reports the confirmatory catecholamine and metanephrine results
  (plasma and urine) and the adrenal imaging without dates. The source says only that the
  mass turned up while her paroxysmal hypertension was being worked up, before the
  surgical visit. Resolution: endocrinology orders them in May 2025 (R4), and this file
  dates them 2025-06-03, a few weeks after that order. The date is this file's choice, not
  the source's, and so is the five months it leaves between the CT and the surgical visit;
  no note explains that interval. The results carry no numbers beyond far above range,
  which is all the source gives.
- **R6.** The surgical plan describes the falling blood pressure before it says she is
  admitted and started on phenoxybenzamine. Resolution: admission and phenoxybenzamine come
  first, then the BP falls over the next few days. Spells of a few minutes in November 2024
  and 20 to 30 minutes, several times a week, in November 2025 are both carried as the
  course of the year.
- **Left as the source has it:**
  - About a year passes from the high metanephrines at cardiology (November 2024) to alpha
    blockade at the surgical admission (November 2025). Cardiology records no medication,
    and the surgical note reports home readings above 180/100 with no blood pressure drug
    before phenoxybenzamine. No blood pressure drug is started in that year, and no note
    explains or comments on it. The practice does not receive those home readings (see P).
  - Calcium 10.2 mg/dL, at the top of the reference range, in a woman with MEN 2A; no one
    checks a parathyroid hormone level, and no note comments on it.
  - No beta blocker is added during the alpha blockade.

## Additions (A): physician to confirm

- **A1.** The 2022 new-patient visit. The seed gives her about four years with the practice,
  and every source encounter is outside care, so she needs a primary-care home before them.
- **A2.** Adrenal CT details (the source gives only a one-sided adrenal tumor that looks
  like a pheochromocytoma, and its history places it on the right): a 3.6 cm
  heterogeneous right adrenal mass, unenhanced attenuation 38 HU; left adrenal normal; no
  other abdominal mass.
- **A3.** Thyroid and neck ultrasound before surgery for the palpable nodule: a solid
  hypoechoic nodule at the mid-portion of the right lobe, 1.8 x 1.4 x 1.2 cm; left lobe
  normal; no abnormal cervical lymph nodes.
- **A4.** Germline RET testing, which supports the source's MEN 2 diagnosis: a pathogenic
  variant at codon 634 (p.Cys634Arg). The genetics letter recommends testing for at-risk
  relatives in general terms and names none.
- **A5.** The adrenalectomy the source schedules. Phenoxybenzamine 10 mg twice daily from
  2025-11-10, stopped after surgery. Laparoscopic right adrenalectomy 2025-11-14 at the
  regional hospital, uncomplicated. Pathology: pheochromocytoma, 3.6 cm, confined to the
  adrenal, margins negative. Plasma free metanephrines normal on 2025-12-18. After that,
  normal BP without drugs and no further spells. Yearly plasma metanephrines follow for
  MEN 2A; the first is drawn with the endocrinology labs at the 2026-10-22 visit (A6).
- **A6.** The thyroidectomy the source plans. Total thyroidectomy with central neck
  dissection 2026-01-22 by the same outside surgical group. Parathyroids are identified and
  preserved, and calcium is normal after surgery with no supplement. Pathology: medullary
  thyroid carcinoma, 1.7 cm, right lobe, margins negative, 0 of 9 central nodes involved.
  Levothyroxine 100 mcg daily from discharge; TSH in range on 2026-03-05, with no dose
  change. On 2026-04-23 calcitonin is undetectable (under 2 pg/mL) and CEA is 2.1 ng/mL.
  The diarrhea resolves. Endocrinology plans repeat calcitonin and CEA in six months and
  books that visit for 2026-10-22, with the first yearly plasma metanephrines (A5) drawn
  the same day.
- **Problem list after the surgeries** (follows from S, A4, A5 and A6): multiple endocrine
  neoplasia type 2A (RET codon 634), with yearly plasma metanephrines and calcitonin and
  CEA surveillance; medullary thyroid carcinoma, status post total thyroidectomy with
  central neck dissection 2026-01-22, under calcitonin and CEA surveillance; pheochromocytoma,
  status post right adrenalectomy 2025-11-14; status post total thyroidectomy on
  levothyroxine replacement (no hypothyroidism entered as a separate diagnosis). Paroxysmal
  (secondary) hypertension is marked resolved after the adrenalectomy.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention
  (heart sounds, abdominal exam, contralateral adrenal, lymph nodes). An abnormal finding
  appears only as the source or this file gives it. A report may carry the descriptors its
  kind always carries (how a size was measured, a procedure's access site, the specimen's
  orientation), but nothing that adds a territory, a severity grade or a symptom, or
  changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal results.
- Yearly physicals (2023 to 2026) with vitals and BP normal before autumn 2024 and after
  the adrenalectomy; basic metabolic panel and CBC at physicals; lipids once or twice, normal.
  Practice visits around each outside event as in the timeline; after the surgeries, a
  TSH about yearly on levothyroxine.
- Age- and sex-appropriate prevention: screening mammography every one to two years, normal;
  cervical co-test (2023, normal); colorectal screening from 45 (colonoscopy 2024-03-12, no
  polyps); influenza each fall; COVID vaccines; Tdap if due; HIV and hepatitis C screening
  once; depression screens at physicals.
- Home BP readings sent through the portal only after the adrenalectomy, all normal; no
  home readings reach the practice before it (the spikes above 180/100 appear only in the
  surgical note). Refills; pharmacy traffic; school forms and a work note for her surgical
  leave (she is a teacher); scheduling messages about the outside appointments; a missed
  visit.
- Booked visits after 2026-08-27: the outside endocrinology follow-up on 2026-10-22 with
  calcitonin, CEA and the first yearly plasma metanephrines (A5, A6), and a practice
  follow-up on 2026-11-12 to review them.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00002781 on 2026-02-03. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. Nothing is booked after today.
