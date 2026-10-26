# Build requirements: unit-2122 (Viktor Michaud, MRN00002122)

Source: synthetic_hospital patient 2122, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its unfinished plan wording.

## Who he is (S)

Viktor Michaud, born 1975-09-10, male, White, not Hispanic or Latino, English,
Northampton MA 01060, commercial PPO (Blue Cross Blue Shield). Senior financial analyst.
Never smoker; lifetime abstinent from alcohol. Single (texture; no partner appears).

- **Allergies:** atorvastatin (diffuse muscle pain with CK elevation, an intolerance);
  sulfonamide antibiotics (rash).
- **Surgical history:** cholecystectomy at age 42 (2018, before the practice; R1).
- **Family history:** father with premature coronary artery disease (MI at 42) and
  hyperlipidemia; mother with type 2 diabetes; brother with cirrhosis of unknown cause.
- **Joined the practice** 2023-09-12 (A1), age 48, from a prior practice. He brought:
  chronic liver disease of unknown cause (raised liver enzymes known from the prior
  practice), severe hypercholesterolemia, history of statin-associated myopathy on
  atorvastatin at the prior practice (R1), inflammatory arthropathy (joint pain he treats
  with acetaminophen), status post cholecystectomy. Medications on joining: ezetimibe
  10 mg daily, acetaminophen 500 mg every 6 hours as needed for joint pain, vitamin D3
  2000 units daily. No diabetes yet (R2).
- Records texture is **sparse**: the prior practice sends a short summary with no lab
  values; cardiology reaches the chart as his own account, a fill and a brief late letter;
  gastroenterology as his account and a consult letter weeks later; phlebotomy only as
  his account.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-09-12 | practice | **new-patient visit** (A1): history as above; height 180 cm, weight 84 kg, BP normal; exam normal. Labs (P): CMP with AST 66, ALT 79 (A3), lipids on ezetimibe TC 268, LDL 192, HDL 44, TG 140 (A4), A1c 5.5, CBC normal, HCV and HIV nonreactive. Records requested. Cardiology (lipid) referral placed after results (A5) |
| 2023-10 | outside → practice | prior-practice summary received: problem and medication lists as above; atorvastatin stopped in 2021 for diffuse myalgia with raised CK (no value sent); ezetimibe since; screening colonoscopy 2021 normal; Tdap 2019 |
| 2024-03-19 | practice | follow-up: liver enzymes, lipids, joints. AST and ALT about where they were (A3); LDL about 190 (A4). Has not scheduled cardiology |
| 2024-05 | practice | cardiology referral closed unscheduled; he prefers to wait (A5) |
| 2024-09-17 | practice | annual physical (P): A1c 5.6, LDL about 188, AST/ALT about the same |
| 2025-04-08 | practice | follow-up (a March slot was cancelled): liver enzymes about the same; joints unchanged on acetaminophen |
| 2025-09-16 | practice | annual physical: **fasting lipid panel TC 260, LDL 185, HDL 42, TG 135** (S, R3); A1c 7.0 and fasting glucose 138 (A2) |
| 2025-09-18 | practice | results call: **type 2 diabetes diagnosed; metformin 500 mg twice daily started** (A2); cardiology referral placed again (A5) |
| 2025-10-16 | outside | cardiology consult for lipid management after statin intolerance: reviews the September panel; **familial hypercholesterolemia diagnosed**; **bile acid sequestrant started: colesevelam 1875 mg (three 625 mg tablets) twice daily with meals** (R4). Reaches the practice as his portal account (next day), a pharmacy fill, and a brief cardiology letter received 2025-12 |
| 2026-01-13 | practice | diabetes follow-up (A6): about three months of fatigue, abdominal pain and joint stiffness (S, started about October 2025). A1c 6.6, LDL 158 on colesevelam (A6), AST 92, ALT 105 (A3), CBC and TSH normal. **Gastroenterology referral** for the rising liver enzymes with the new symptoms (A6) |
| 2026-04-21 | outside | gastroenterology consult: six months of fatigue, abdominal pain and joint stiffness; recent-onset diabetes; exam with **bronze skin discoloration and hepatomegaly**; **ferritin 2500 ng/mL (ref 24-336), transferrin saturation 85% (ref 20-50), AST 98, ALT 110, fasting glucose 160**. Impression **hereditary hemochromatosis with arthropathy** (S, R5); HFE genotyping and liver ultrasound ordered (A7). Reaches the practice as his portal account 2026-04-22 and the consult letter 2026-05-14 |
| 2026-04-30 | outside | abdominal ultrasound (A7): liver enlarged at 19.5 cm, smooth contour, no focal lesion; spleen normal size; no ascites; gallbladder surgically absent. Report copied to the practice |
| 2026-05-04 | outside | HFE genotype (A7): **C282Y homozygous**, H63D not detected. He reports it by portal 2026-05-06; the practice adds hereditary hemochromatosis with arthropathy to the list |
| 2026-05-12 | outside | **weekly therapeutic phlebotomy (500 mL) begins** at an outside infusion center under gastroenterology (A8); goal ferritin 50 to 100; known to the practice from his account and the GI letter |
| 2026-05-19 | practice | follow-up after GI: problem list updated (R5, R6); bronze skin and a palpable liver edge on exam (S as GI found them); metformin, ezetimibe, colesevelam continued; acetaminophen kept to no more than 2 g a day (P); no iron or vitamin C supplements |
| 2026-07-21 | practice | labs (A8): ferritin 1710, transferrin saturation 78%, hemoglobin 13.9, AST 74, ALT 83; A1c 6.9 |
| 2026-09-22 | practice | booked: annual physical with diabetes and hemochromatosis follow-up |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source puts the atorvastatin myopathy and the cholecystectomy before both
  encounters without dates. Resolution: both happened at the prior practice, the
  cholecystectomy in 2018 (age 42) and the atorvastatin stop in 2021, so the allergy and
  ezetimibe are on file from joining. The CK value is never received.
- **R2.** The profile lists type 2 diabetes from the start, but both encounters call it
  recent. Resolution: diabetes is diagnosed at the practice in September 2025, a month
  before cardiology, and metformin starts then (A2). The 2023 and 2024 notes never
  mention it.
- **R3.** The source's lipid numbers sit in the cardiology note with no draw. Resolution:
  they are the practice's fasting panel of 2025-09-16, which the cardiologist reviews.
  Cardiology draws nothing of its own in the chart.
- **R4.** The source's plan names only a drug that binds bile acids. Resolution:
  colesevelam 625 mg, three tablets twice daily with meals (3.75 g a day), chosen over
  cholestyramine; it is on the practice list from his October 2025 report. The source's
  "Familial Hypercholesterolemia (diagnosed 2020-01-15)" stamp maps to this visit.
- **R5.** The gastroenterology note says diabetes arose "despite no significant family
  history", yet his mother has type 2 diabetes. Resolution: that phrase is dropped; the
  mother's diabetes stands. The note's diagnosis of hemochromatosis rests on the exam and
  iron studies; the genotype (A7) confirms it. The six-month interval and the six months
  of symptoms run from October 2025.
- **R6.** The source keeps "severe hypercholesterolemia" beside familial
  hypercholesterolemia, "inflammatory arthropathy" beside hemochromatosis arthropathy,
  and "chronic liver disease of unknown etiology" after the cause is found. Resolution:
  severe (pure) hypercholesterolemia is replaced on the practice list by heterozygous
  familial hypercholesterolemia when cardiology's diagnosis is known (October 2025);
  inflammatory arthropathy is resolved into hereditary hemochromatosis with arthropathy
  on 2026-05-19; chronic liver disease stays on the list, its cause written as unknown
  until April 2026 and as iron overload after. History of statin-associated myopathy
  stays as history throughout. Outside letters may still copy the older labels.
- **Left as the source has it:** nobody checks iron studies before the gastroenterology
  visit, despite years of raised liver enzymes, a brother with cirrhosis of unknown cause,
  joint symptoms and new diabetes, and nobody notes the skin color before April 2026.
  Nobody refers the arthropathy to rheumatology. No statin rechallenge, no other statin
  and no PCSK9 inhibitor; LDL stays in the 150s to 160s after colesevelam and no note
  presses it. Metformin stays 500 mg twice daily. Acetaminophen continues in liver
  disease (with the 2 g ceiling from May 2026). No note comments on any of these gaps.

## Additions (A): physician to confirm

- **A1.** 2023-09-12 new-patient visit (the seed gives about three years with the
  practice; both source encounters are outside care).
- **A2.** Diabetes diagnosis on the 2025-09-16 draw: A1c 7.0, fasting glucose 138 (same
  sample confirms); metformin 500 mg twice daily started 2025-09-18 (the source's home
  dose must start somewhere). Earlier A1c values 5.5 (2023) and 5.6 (2024) are normal.
- **A3.** Liver enzyme levels for the listed chronic liver disease: AST about 66 and ALT
  about 79 from 2023 to mid-2025, 92 and 105 in January 2026, the source's 98 and 110 in
  April 2026, 74 and 83 in July 2026 on phlebotomy. Albumin, bilirubin, platelets and INR
  normal throughout.
- **A4.** Lipid levels on ezetimibe before the source's panel: TC 268, LDL 192, HDL 44,
  TG 140 in 2023, LDL about 188 to 190 in 2024.
- **A5.** Cardiology referral placed in 2023, never scheduled and closed in May 2024,
  then placed again in September 2025 after the diabetes diagnosis (the visit that sent
  him to the source's cardiology encounter).
- **A6.** The 2026-01-13 practice visit that sends him to gastroenterology, with A1c 6.6
  and LDL 158 (monitoring of the started drugs).
- **A7.** Gastroenterology's confirmatory steps: HFE genotype C282Y homozygous with H63D
  not detected (resulted 2026-05-04), and the 2026-04-30 abdominal ultrasound as in the
  timeline (supports the source's hepatomegaly). Fibrosis assessment is deferred by GI to
  follow-up; nothing else is ordered.
- **A8.** Standard treatment of the source's hemochromatosis: weekly 500 mL therapeutic
  phlebotomy from 2026-05-12 at an outside infusion center run by gastroenterology, target
  ferritin 50 to 100, with GI advice to avoid iron and vitamin C supplements and raw
  shellfish and to have first-degree relatives screened. July 2026 practice labs as in
  the timeline. Serum iron and TIBC sit consistent with the stated saturations: iron
  232 and TIBC 273 in April, iron 208 and TIBC 267 in July.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Height 180 cm, weight about 84 kg throughout; blood pressure, pulse and oxygen
  saturation normal at every visit.
- Records from the prior practice may mention routine screening done there, with normal
  results (the 2021 colonoscopy, Tdap 2019).
- Visits every 6 to 12 months before the diabetes, about every 3 to 4 months after; CMP
  or hepatic panel at most visits, lipids yearly, A1c, urine albumin/creatinine yearly
  and a foot exam once diabetic; CBC and TSH normal when drawn.
- A diabetic eye exam letter from an optometrist in 2026: no retinopathy.
- Prevention: influenza each fall (2023, 2024, 2025); updated COVID vaccine each fall;
  hepatitis B (Heplisav-B, two doses 2024) and hepatitis A (two doses 2024) for chronic
  liver disease; PCV20 in 2025 (diabetes, chronic liver disease); Shingrix at 50 (October
  2025 and January 2026); PHQ-2 at physicals, negative; hepatitis C and HIV screening
  once (2023), nonreactive.
- Refills, pharmacy traffic, portal messages about results, refills and scheduling, a
  cancelled visit, referral scheduling calls, records requests.
- Booked visit after 2026-08-27: the 2026-09-22 annual physical.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00002122 on 2026-05-19. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. Nothing is booked after today.
