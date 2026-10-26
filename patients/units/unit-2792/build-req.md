# Build requirements: unit-2792 (Heather Fournier, MRN00002792)

Source: synthetic_hospital patient 2792, in `source.md` beside this file. This file and
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
never carry a source clinician's name, its "Active Problem List" dump, its
"(diagnosed ...)" stamp, its "lesions shown in the image below" or the bare
"Electromyography" line as written.

## Who she is (S)

Born 1970-10-25, female, White, not Hispanic or Latino, English-speaking, lives in
Belchertown (01007), private employer-sponsored PPO. High school mathematics
teacher. Never smoker; social alcohol, 1 to 2 glasses of wine a week; no drugs.
No known drug allergies. Surgical history: cholecystectomy (2014) and tubal ligation
(1999) (R4). Family history: mother with chronic autoimmune (lymphocytic) thyroiditis,
a maternal aunt with rheumatoid arthritis, father with hypertension (R3). Marital
status recorded as married (authoring field only; no spouse appears in the chart).

Joined the practice on 2022-08-10 (A1), a new-patient preventive visit. She brought
no chronic conditions and no regular medications. Height 165 cm, weight 68 kg at
joining, BMI about 25.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-08-10 | practice | new-patient preventive visit (A1): healthy, history as above; prior-practice records requested |
| 2022-08-24 | outside records | prior PCP records received (P): colonoscopy 2021 normal, repeat in 10 years; screening mammogram 2021 normal; Pap with HPV 2020 normal |
| 2024-06-24 | practice | (A2) about two months of a violet-red rash on the upper eyelids and red patches on the backs of the hands, and several months of trouble climbing stairs and keeping her arms up to comb her hair; no fever, no weight loss. Exam: violaceous eyelid rash, erythematous patches on the dorsal hands, mild symmetric proximal weakness (4+/5 deltoids and hip flexors). Labs drawn: **CK 1,840 U/L**, **ANA positive, 1:640 speckled** (R1), AST 58, ALT 44, CBC and TSH normal. Referred to dermatology |
| 2024-07-19 | outside | dermatology consult letter (S, encounter 0): age 53; progressive difficulty climbing stairs and combing her hair over several months, with new reddish skin changes on face and hands; denies fever and weight loss; T 36.8 C, BP 124/76, pulse 80, RR 14, SpO2 99% on room air; violaceous rash on the eyelids and erythematous lesions on the dorsal hands (R2); symmetric decreased strength in proximal upper and lower extremities; elevated CK and positive ANA (the practice's June draw, R1). Impression: **dermatomyositis** (clinical diagnosis; R5). Recommends rheumatology for confirmation and treatment; sun protection. No medication started. Letter received 2024-07-25 |
| 2024-07-30 | practice | telehealth to review the letter (P): **dermatomyositis added to the problem list**; rheumatology referral placed |
| 2024-09-11 | practice | rheumatology referral: the first offered rheumatology date in October 2024 is cancelled by the patient at the start of the school year; weakness mild and stable; referral left open (R5) |
| 2025-08-13 | practice | annual physical (P): over the summer stairs and lifting her arms have become harder again; mild symmetric proximal weakness persists; eyelid and knuckle redness still present. **CK 2,650 U/L**, AST 71, ALT 52. New rheumatology referral placed, marked soon (R5) |
| 2025-11-19 | outside | rheumatology initial consultation letter (A3): proximal weakness worse over several months; CK 3,120 U/L, AST 96, ALT 71; orders myositis antibody panel, EMG, muscle biopsy |
| 2025-12-10 | outside | EMG/nerve conduction report (S "electromyography", findings A3): normal nerve conduction; myopathic motor units (short-duration, low-amplitude, polyphasic) with fibrillation potentials in proximal arm and thigh muscles; distal muscles normal |
| 2026-01-06 | outside | muscle biopsy, left vastus lateralis, pathology report (S "muscle biopsy confirms dermatomyositis", findings A3): perifascicular atrophy with perivascular inflammation; consistent with dermatomyositis |
| 2026-01-21 | outside | rheumatology follow-up letter (S, encounter 1): returns for the symptoms first evaluated in July 2024; over the past 6 months stairs and raising her arms are harder, brushing her hair is tiring, her neck feels weak sitting upright; takes no regular medications; NKDA; family history as above. Exam: symmetric proximal weakness of upper and lower limbs; violaceous eyelid rash and scaly red patches over the knuckles. CK elevated (**3,480 U/L**, AST 104, ALT 78); **anti-Mi-2 antibody positive** (other myositis antibodies negative); EMG myopathic; biopsy confirms. Assessment: **dermatomyositis confirmed**. Plan (A4): **prednisone 60 mg daily**, **methotrexate 15 mg weekly**, **folic acid 1 mg daily**, **cholecalciferol 1000 units daily**; pre-treatment hepatitis B surface antigen, hepatitis C antibody and QuantiFERON negative; CT chest, abdomen and pelvis for cancer screening (A5); follow-up 8 weeks |
| 2026-02-04 | outside | CT chest and CT abdomen/pelvis with contrast (A5): no mass, no adenopathy, no interstitial lung disease; surgically absent gallbladder; otherwise normal |
| 2026-02-11 | practice | follow-up after rheumatology (P): on the regimen above, tolerating it; BP 126/80; glucose normal; PCV20 given; DXA ordered (A6) |
| 2026-02-25 | outside | DXA (A6): lumbar spine T-score -0.9, total hip T-score -0.4; normal bone density |
| 2026-03-18 | outside | rheumatology letter (A4): strength better (stairs easier, can hold arms up), rash fading; CK 910 U/L, AST 40, ALT 38; **prednisone lowered to 40 mg daily**; methotrexate 15 mg weekly continues |
| 2026-05-13 | outside | rheumatology letter (A4): further improvement; CK 260 U/L; LFTs normal; **prednisone lowered to 20 mg daily** |
| 2026-07-08 | practice | last practice visit (P): doing well, back to climbing stairs at school; weight 71.5 kg on prednisone; BP 128/80; CK 170 U/L; CMP, CBC, A1c normal |
| 2026-08-12 | outside | rheumatology letter (A4), received 2026-08-19 and filed: strength near normal; CK 140 U/L; **prednisone lowered to 15 mg daily**; methotrexate 15 mg weekly continues. Practice lists not updated after last_seen |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The dermatology encounter reports "elevated CK" and "positive ANA" without
  values or a draw. Resolution: the practice drew them at the 2024-06-24 visit that
  sent her to dermatology; the dermatology letter quotes them. Values as in the
  timeline (CK 1,840 U/L; ANA 1:640 speckled). Every other CK in the chart is the
  value given in the timeline.
- **R2.** "Some lesions shown in the image below" has no image. Resolution: the
  letter describes erythematous lesions on the backs of the hands (the chief
  complaint's "face and hands"). The knuckle scale is first described at the 2025
  physical and at rheumatology in 2026, as the source describes it there.
- **R3.** Encounter 0 says her mother has rheumatoid arthritis; the profile and
  encounter 1 say mother with autoimmune thyroiditis and a maternal aunt with
  rheumatoid arthritis. Resolution: the profile version everywhere; encounter 0's
  statement is not carried. The thyroid disease is written as "autoimmune
  thyroiditis", never by its eponym.
- **R4.** Source text years follow the re-dated timeline (+4.5 years): "initially
  evaluated in January 2020" becomes July 2024; cholecystectomy 2010 becomes 2014;
  tubal ligation 1995 becomes 1999.
- **R5.** The source stamps dermatomyositis as diagnosed at the dermatology visit,
  yet the rheumatology visit 18 months later makes the diagnosis with antibody, EMG
  and biopsy, and she takes no medication in between. Resolution: dermatology makes
  a clinical diagnosis in July 2024 and the practice lists it then; the first
  rheumatology appointment is cancelled by the patient that fall while symptoms are
  mild; weakness worsens from summer 2025 (the source's "past 6 months" before
  January 2026); a second referral leads to the November 2025 consult (A3), the
  workup, and confirmation on 2026-01-21. Nobody comments on the delay.
- **Left as the source has it:** no treatment for dermatomyositis from July 2024 to
  January 2026, and no cancer screening imaging before January 2026. No note remarks
  on it.

## Additions (A): physician to confirm

- **A1.** 2022-08-10 new-patient visit (seed: about 4 years with the practice).
- **A2.** 2024-06-24 practice visit that draws the labs and refers to dermatology.
- **A3.** The November 2025 rheumatology consultation and the findings of the EMG
  and biopsy the source asserts (standard findings for dermatomyositis, given above).
- **A4.** Standard treatment after confirmation, run by rheumatology: prednisone
  60 mg daily tapered to 40 mg (March), 20 mg (May) and 15 mg (August 2026);
  methotrexate 15 mg weekly with folic acid 1 mg daily; cholecalciferol 1000 units
  daily; pre-methotrexate hepatitis B, hepatitis C and TB screens negative. CK and
  LFT values as in the timeline.
- **A5.** One CT chest, abdomen and pelvis for cancer screening in adult
  dermatomyositis, normal (also no interstitial lung disease).
- **A6.** One DXA for glucocorticoid therapy, normal (values above).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as the source or this file gives it.
- Records from the prior practice may mention routine screening done there, with
  normal results (as in the timeline).
- Yearly preventive visits (2023, 2025 physicals in the timeline span; she skips a
  2024 physical, the June 2024 visit standing in), labs: CBC, CMP, lipids, TSH,
  A1c; hepatitis C and HIV screening once (2022). PHQ-2/PHQ-9 screens negative.
- Screening mammograms yearly each autumn 2022 to 2025, normal. Pap with HPV at the
  2025 physical, negative. Colonoscopy not due.
- Influenza vaccine each fall 2022 to 2025; COVID vaccines in fall 2022 to 2025;
  Tdap at joining; Shingrix two doses in 2023; PCV20 once in February 2026 for
  immunosuppression.
- Weight about 68 kg until 2026, then 70 to 72 kg on prednisone. BP normal
  throughout (high 110s to 120s systolic), a little higher on prednisone.
- Refills via rheumatology, pharmacy traffic, portal messages about results,
  referrals and scheduling, a cancelled visit, records requests.
- No visit booked after today (seed `booked` false).

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002792 on 2026-07-08. Practice contacts after that day do
not happen; outside care after it still does, and its records are filed as received
without changing the practice's lists. Nothing is booked after today.
