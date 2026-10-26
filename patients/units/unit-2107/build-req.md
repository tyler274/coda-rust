# Build requirements: unit-2107 (Daniel Chen, MRN00002107)

Source: synthetic_hospital patient 2107, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its "shown below" image text.

## Who he is (S)

Born 1982-05-05, male, Asian (Chinese; born in China), Private PPO, information
technology specialist, single (no partner or children recorded). Never smoker; alcohol
occasional, 1 to 2 drinks a month; no illicit drugs. Allergies: sulfamethoxazole
(anaphylaxis), latex (contact dermatitis, recorded as rash). Height 173 cm, weight
about 70 kg at joining.

Family history (S, recorded as the chart learns it, R10): father with multiple endocrine
neoplasia type 2 and medullary thyroid carcinoma; paternal aunt with pheochromocytoma;
mother with type 2 diabetes and essential hypertension; sister a chronic hepatitis B
carrier.

Joined the practice 2019-09-10 (A1) with no known problems and no medications. Every
source condition starts later, at its own event (R1).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2019-09-10 | practice | new patient visit (A1); healthy; BP 136/86; routine labs normal; HCV and HIV screens negative; Tdap |
| 2020-04-07 | practice | telehealth: fatigue and mild right upper quadrant ache for a couple of months (S, GI HPI); labs ordered (A2) |
| 2020-04-09 | practice | draw: ALT 92, AST 61, otherwise normal CMP and CBC, TSH normal (A2) |
| 2020-04-16 | practice | hepatitis serologies: **HBsAg positive**, hepatitis B core antibody positive, hepatitis B surface antibody negative (A2). He now tells the practice his sister is a hepatitis B carrier (R10). Plan: repeat in 6 months |
| 2020-10-13 | practice | annual physical: office BP 148/94, **essential hypertension diagnosed** (R8), diet, exercise and home readings, no drug; repeat HBsAg still positive, ALT 84, AST 57 (A2) |
| 2020-10-16 | practice | result call: HBsAg positive more than 6 months, **chronic hepatitis B diagnosed**; gastroenterology referral (A2) |
| 2020-12-28 | outside | gastroenterology consult: fatigue and mild RUQ discomfort for several months; elevated transaminases and HBsAg positive for the past 8 months; HBV DNA 46,000 IU/mL (A3); liver biopsy arranged (R4) |
| 2021-01-12 | outside | liver biopsy: chronic inflammation with areas of hepatocyte injury (S, R4) |
| 2021-01-21 | outside | gastroenterology follow-up by video: biopsy reviewed; **tenofovir alafenamide 25 mg daily started** (A4) |
| 2021-03-09 | practice | HTN follow-up (A5): episodic headaches with racing heart and heavy sweating, more sluggish, a few bouts of vague abdominal discomfort; appetite normal; BP 158/96, HR 96. Family history clarified: father MEN2 with medullary thyroid carcinoma, paternal aunt pheochromocytoma (R10). Urgent endocrinology referral; antihypertensive deferred to endocrinology |
| 2021-04-05 | outside | endocrinology consult (R5): symptoms as above; BP 166/94, HR 92; cardiac and abdominal exams normal; denies illicit drugs; family history of endocrine surgeries. Labs drawn: calcium 10.6 (borderline high), calcitonin 186 pg/mL (raised), 24-hour urine normetanephrine 1,120 mcg/24 h (reference under 900) and metanephrine 430 mcg/24 h (reference under 400), mildly raised (A6). Germline genetic testing sent |
| 2021-04-08 | outside | CT abdomen (adrenal protocol): left adrenal fullness, right adrenal normal (S, A6). Thyroid ultrasound: 1.4 cm solid hypoechoic nodule, right lobe (S, A6) |
| 2021-04-19 | outside | endocrinology follow-up letter (R5): germline RET point mutation; **multiple endocrine neoplasia type 2 diagnosed**, with **left pheochromocytoma** and a **thyroid nodule** suspicious for medullary carcinoma; **doxazosin 4 mg daily started** for preoperative alpha blockade (A7); surgery and genetic counseling referrals |
| 2021-04-26 | outside | genetic counseling letter: RET result explained, autosomal dominant, sister advised to be tested (S referral; no outcome recorded) |
| 2021-04-29 to 05-01 | outside | **laparoscopic left adrenalectomy** (age 38, S): pathology pheochromocytoma 2.4 cm confined to the gland, margins clear (A8). Pheochromocytoma resolved. Doxazosin continued at discharge because BP stays above normal (R8) |
| 2021-05-11 | practice | post-op visit: incisions healing, BP 138/88 on doxazosin, symptoms gone (P) |
| 2021-06-21 to 06-22 | outside | **total thyroidectomy with central neck dissection** (age 39, S): pathology **medullary thyroid carcinoma**, 1.3 cm right lobe, margins negative, central nodes negative (A8); calcium normal; **levothyroxine 125 mcg daily started** |
| 2021-07-20 | outside | endocrinology post-op letter: calcitonin undetectable (under 2 pg/mL), plasma free metanephrines normal, TSH normal on levothyroxine 125 mcg (A8). Medullary carcinoma becomes history of thyroid cancer |
| 2021-08-03 | outside | gastroenterology letter: HBV DNA not detected, ALT normal on tenofovir (A3) |
| 2021-10-19 | practice | annual physical (P); BP 134/86 on doxazosin |
| 2022-04-12 | practice | HTN follow-up (P) |
| 2022-06-14 | outside | endocrinology yearly letter: calcitonin undetectable, plasma metanephrines normal, TSH normal (A9) |
| 2022-07-12 | outside | liver ultrasound: normal size and echotexture, no focal lesion (A10) |
| 2022-07-19 | outside | gastroenterology letter: HBV DNA not detected, ALT normal (A3, A10) |
| 2022-10-25 | practice | annual physical (P) |
| 2023-04-18 | practice | HTN follow-up (P). Around May 2023 he stops tenofovir on his own and tells no one (R3) |
| 2023-06-20 | outside | endocrinology yearly letter: as 2022 (A9) |
| 2023-07-25 | outside | gastroenterology office notes a missed follow-up (P texture) |
| 2023-10-24 | practice | annual physical: no-show (P) |
| 2023-12-19 to 12-23 | outside | ED, then admission (R6): 2 weeks of worsening fatigue, generalized weakness, vague abdominal discomfort; low-grade fevers; 4 kg weight loss in a month; dark urine. Off tenofovir since about May (R3). BP 168/94, temperature 37.9 °C. No rash or joint deformity; abdomen soft, mild right flank tenderness. ESR 68, creatinine 1.64, hemoglobin 11.6 with MCV 88 (mild normocytic anemia), urinalysis blood 2+ with 15 RBC/hpf (microscopic hematuria), ANCA negative, HBV DNA 1,900,000 IU/mL (A11). Mesenteric CT angiogram 12-20: multiple small aneurysms with segmental narrowing of mesenteric branches, consistent with medium-vessel vasculitis (R6). **Hepatitis B associated polyarteritis nodosa diagnosed.** Tenofovir resumed; **prednisone 40 mg daily started** (A11); discharge creatinine 1.52; rheumatology and GI follow-up |
| 2023-12-28 | practice | post-discharge visit (A12): BP 146/92, creatinine 1.48 |
| 2024-01-09 | outside | rheumatology first visit (A13): **azathioprine 75 mg daily started**; prednisone taper toward 5 mg |
| 2024-02-13 | outside | gastroenterology letter: HBV DNA 640 IU/mL, back on tenofovir (A3) |
| 2024-04-16 | outside | rheumatology letter (A13): **prednisone now 5 mg daily**; creatinine 1.24, ESR 16, hemoglobin 14.0 |
| 2024-05-14 | practice | follow-up (P): BP 140/88; A1c normal on prednisone; PCV20 and first Shingrix (immunosuppressed) |
| 2024-07-16 | practice | nurse visit: second Shingrix (P) |
| 2024-10-08 | outside | rheumatology letter: stable, regimen unchanged (A13) |
| 2024-10-22 | practice | annual physical (P); BP 140/88 |
| 2024-11-12 | outside | endocrinology yearly letter: as 2022 (A9) |
| 2025-01-31 | practice | **last seen.** About 2 months of numbness and tingling in both feet; reduced light touch and pinprick to the ankles; **peripheral sensory neuropathy diagnosed** (R9); **gabapentin 300 mg three times daily started** (titrated up over a week); B12, A1c and TSH normal (A14); BP 142/88; message to rheumatology |
| 2025-03-04 | outside | liver ultrasound: no focal lesion (A10). Gastroenterology letter 2025-03-11: HBV DNA not detected (A3) |
| 2025-06-10 | outside | rheumatology letter: stable, regimen unchanged (A13) |
| 2025-11-18 | outside | endocrinology yearly letter: as 2022 (A9) |
| 2025-12-19 | outside | rheumatology follow-up (S, R11): worsening fatigue, unintentional weight loss (about 3 kg since January, A15), intermittent abdominal pain over the past month, worse numbness and tingling in the lower legs, a recent painful ulcer on the thigh. Hepatomegaly. AST 96, ALT 118 (A15); urinalysis protein 1+, no RBC casts. Chest X-ray 12-19 unremarkable: no infiltrate, nodule or hemorrhage. Renal CT angiogram 12-22: multiple microaneurysms and narrowing of medium-sized arteries. Liver biopsy 12-23: transmural inflammation of medium-sized arteries with fibrinoid necrosis and surrounding inflammatory infiltrate. Letter dated 12-29 |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The profile lists all six drugs and all five conditions at every encounter,
  including the first. Resolution: each starts at its event in the timeline (chronic
  hepatitis B and hypertension 2020-10, tenofovir 2021-01, MEN2 and doxazosin 2021-04,
  levothyroxine 2021-06, PAN and prednisone 2023-12, azathioprine 2024-01, neuropathy
  and gabapentin 2025-01). Outside letters list only what was current on their dates.
- **R2.** The surgical history (adrenalectomy at 38, thyroidectomy at 39) appears at the
  GI visit, before MEN2 is found. Resolution: both operations follow the 2021 workup,
  on 2021-04-29 (age 38) and 2021-06-21 (age 39).
- **R3.** The ED says hepatitis B was "diagnosed 8 months ago" and antiviral treatment
  was not completed. Resolution: the 8-month date is not carried (known since 2020); the
  unfinished treatment is that he stopped tenofovir on his own around May 2023. The
  practice's list keeps tenofovir active throughout, since nobody knew; the hospital
  resumes it.
- **R4.** The GI encounter carries a biopsy result. Resolution: the consult arranges the
  biopsy, done 2021-01-12; tenofovir follows the result.
- **R5.** The endocrinology note carries a whole workup at one visit (labs, imaging,
  germline test). Resolution: consult 2021-04-05 with labs drawn, imaging 04-08,
  results and the RET result in a letter 04-19. "Following his January GI visit" becomes
  following his December GI visit.
- **R6.** The ED shows a mesenteric angiogram "below". Resolution: he is admitted and
  the angiogram, done as CT angiography (2023-12-20), reads small aneurysms with segmental narrowing,
  consistent with medium-vessel vasculitis. PAN is diagnosed there, matching the 2025
  note's "diagnosed in 2023".
- **R7.** MEN2 is listed twice. Resolution: one condition, coded type 2A (E31.22) for
  the pheochromocytoma, medullary carcinoma and borderline calcium phenotype.
- **R8.** Hypertension is on the list from the start and doxazosin is the only
  antihypertensive. Resolution: essential hypertension is diagnosed 2020-10 without a
  drug; doxazosin starts for pheochromocytoma preparation and is kept after surgery for
  the persisting pressure.
- **R9.** Peripheral sensory neuropathy and gabapentin are on every list. Resolution:
  first recorded 2025-01-31; the 2025-12 rheumatology letter describes it as worse.
- **R10.** Family history comes in pieces: mother's diabetes and hypertension and
  father's thyroid cancer at joining; the sister's hepatitis B in 2020-04; the father's
  MEN2 and the aunt's pheochromocytoma in 2021-03.
- **R11.** The 2025 rheumatology note has no plan, and its renal angiogram is done as CT angiography. Resolution: the letter says the
  findings show active disease of the renal and hepatic arteries, current medicines
  continue for now while treatment is discussed with hepatology, and rheumatology
  will see him again. No later letter has arrived by today.
- **Left as the source has it:** no parathyroid workup for the borderline calcium
  (later calciums normal, no comment); blood pressure stays around 140/88 on doxazosin
  alone after the renal involvement, with no second agent; no hepatitis B test at
  joining; azathioprine is used in hepatitis B associated PAN without comment; no
  hepatitis A vaccine.

## Additions (A): physician to confirm

- **A1.** 2019-09-10 new-patient visit (seed: 7 years with the practice).
- **A2.** The practice finds the hepatitis B: ALT 92, AST 61 (2020-04-09); HBsAg positive,
  core antibody positive, surface antibody negative (2020-04-16); HBsAg still positive,
  ALT 84, AST 57 (2020-10-13); GI referral 2020-10-16. Gives the source's "8 months".
- **A3.** HBV DNA: 46,000 IU/mL (2020-12-28), not detected (2021-08-03, 2022-07-19),
  1,900,000 IU/mL (2023-12-19), 640 IU/mL (2024-02-13), not detected (2025-03-11).
- **A4.** Tenofovir alafenamide 25 mg started by GI 2021-01-21 (a source drug must start).
- **A5.** 2021-03-09 practice visit that sends him to endocrinology.
- **A6.** Numbers for the source's qualitative findings: calcium 10.6, calcitonin 186,
  urine normetanephrine 1,120 and metanephrine 430; thyroid nodule 1.4 cm right lobe.
- **A7.** Doxazosin 4 mg started 2021-04-19 (a source drug must start).
- **A8.** Pathology and post-op results: pheochromocytoma 2.4 cm, confined, margins
  clear; medullary carcinoma 1.3 cm right lobe, margins and central nodes negative;
  calcitonin under 2 pg/mL and plasma metanephrines normal 2021-07-20.
- **A9.** Yearly endocrinology surveillance for MEN2 (calcitonin, plasma metanephrines,
  TSH), all normal: 2022-06-14, 2023-06-20, 2024-11-12, 2025-11-18.
- **A10.** Liver ultrasound surveillance for chronic hepatitis B ordered by GI:
  2022-07-12 and 2025-03-04, no focal lesion.
- **A11.** Admission after the ED with its numbers above; prednisone 40 mg daily at
  discharge; tenofovir resumed; discharge creatinine 1.52.
- **A12.** 2023-12-28 post-discharge practice visit: BP 146/92, creatinine 1.48.
- **A13.** Rheumatology course: azathioprine 75 mg daily from 2024-01-09; prednisone 5 mg
  daily by 2024-04-16 with creatinine 1.24, ESR 16, hemoglobin 14.0; stable letters
  2024-10-08 and 2025-06-10.
- **A14.** Neuropathy labs 2025-01-31: B12, A1c and TSH normal.
- **A15.** 2025-12 numbers: AST 96, ALT 118, urine protein 1+, weight about 3 kg below
  January 2025.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Prior-practice records may mention routine care there with normal results.
- Blood pressure visits about every 6 months and yearly physicals through 2024, with
  vitals, CMP, CBC, lipids once or twice, PHQ-2.
- Influenza each fall 2019 to 2024; COVID vaccines (primary series summer 2021, booster
  winter 2021, updated doses each fall 2022 to 2024); Tdap 2019; PCV20 and two Shingrix
  doses in 2024 for immunosuppression. HCV and HIV screening once (2019). No colorectal
  screening (under 45).
- Refills, a prior authorization for tenofovir, portal messages about results, refills
  and scheduling, a cancelled visit and a no-show.
- After 2025-01-31: refills and messages only; outside letters filed as received.
- No visit is booked.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002107 on 2025-01-31. No practice visit comes after that day, only refills and messages; outside care after it still does, and its records are filed as received without changing the practice's lists. No visit is booked after today.
