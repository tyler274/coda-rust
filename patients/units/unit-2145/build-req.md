# Build requirements: unit-2145 (Brian Barrett, MRN00002145)

Source: synthetic_hospital patient 2145, in `source.md` beside this file. This file and
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

## Who he is (S)

Brian Barrett, born 1981-03-25, male, White, not Hispanic or Latino, English, Medicaid,
lives in the seed's town (01060). Warehouse logistics coordinator. Former smoker, quit
in 2021 (R3). No alcohol. No known drug allergies. Surgical history: appendectomy at
age 16 (1997); open reduction and internal fixation of the left femur after a motor
vehicle accident at age 28 (2009). Family history: father with alcohol use disorder and
cirrhosis; mother with hypertension and hypothyroidism; a younger brother with
hepatitis C infection. Marital status single (texture).

Joined the practice on 2023-07-17 (A1, new patient visit). He brought:

- History of hepatitis C infection, treated with antiviral therapy in 2021 at his prior
  practice and cured (R3).
- Opioid use disorder, in sustained remission; history of intravenous drug use. He
  attends a substance use recovery program regularly and denies current use.
- Post-traumatic stress disorder.
- Medications: naltrexone 50 mg by mouth daily; sertraline 100 mg by mouth daily;
  vitamin D3 2000 units by mouth daily (over the counter). The practice takes over
  prescribing naltrexone and sertraline.

Pancytopenia and hepatitis C reinfection are not known at joining; each starts at its
encounter.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-07-17 | practice | new patient visit (A1): history as above; weight 84 kg, height 180 cm, normal vitals; CMP, CBC, lipid panel normal; HIV-1/2 Ag/Ab nonreactive (P); hepatitis C viral load not detected (A2); PHQ-9 4; Tdap given (P) |
| 2023-07-25 | outside | prior primary care office records received (P): the 2021 hepatitis C treatment course with cure documented there; problem and medication lists as above |
| 2023-10-16 | practice | nurse visit: influenza and COVID vaccines (P) |
| 2023-12-11 | practice | follow-up for OUD in remission and PTSD (P): stable, still attending his program; PHQ-9 3; refills |
| 2024-05-13 | outside (ED) | ED visit; record received 2024-05-14: 2 weeks of fatigue, jaundice and dark urine; no recent travel, no alcohol, no over-the-counter drugs; denies current drug use, attends his program regularly; afebrile, scleral icterus, mild right upper quadrant tenderness. AST 120, ALT 130, alkaline phosphatase 210, total bilirubin 3.2, direct bilirubin 2.1. Hepatitis C antibody reactive and hepatitis C viral load 2,400,000 IU/mL (A3); right upper quadrant ultrasound without biliary dilation or other abnormality (A3). **Hepatitis C virus reinfection diagnosed** (R2). Discharged home with practice follow-up |
| 2024-05-20 | practice | post-ED visit (A4): fatigue and jaundice easing; hepatic panel AST 96, ALT 104, alkaline phosphatase 178, total bilirubin 2.3, direct bilirubin 1.4 (A4); referred to an outside hepatology clinic (Gastroenterology) |
| 2024-06-12 | outside | hepatology consult letter (A5): genotype 1a; viral load 1,150,000 IU/mL; AST 61, ALT 70, alkaline phosphatase 141, total bilirubin 1.2. **Sofosbuvir 400 mg / velpatasvir 100 mg, one tablet daily for 12 weeks, started**. Medicaid prior authorization approved 2024-06-18 (P) |
| 2024-10-14 | practice | nurse visit: influenza and COVID vaccines (P) |
| 2024-12-09 | outside | hepatology follow-up letter (A6): 12 weeks after finishing treatment, viral load not detected; AST 27, ALT 30, alkaline phosphatase 88, total bilirubin 0.7. Cured; yearly viral load at primary care |
| 2024-12-16 | practice | follow-up (P): **hepatitis C virus reinfection resolved** on the problem list after the hepatology letter (A6); PHQ-9 3; refills |
| 2025-07-14 | practice | annual physical (P): CMP, CBC, lipids normal; hepatitis C viral load not detected (A7); FIT kit given; PHQ-9 2 |
| 2025-07-28 | practice | FIT negative (P) |
| 2025-10-20 | practice | nurse visit: influenza and COVID vaccines (P) |
| 2025-12-08 | practice | follow-up visit, no-show (P) |
| 2026-01-12 | practice | rescheduled follow-up (P): stable, refills; PHQ-9 3 |
| 2026-05-01 | practice | last practice visit (A8): about 6 weeks of increasing fatigue, easy bruising and nosebleeds; no fever, night sweats or weight loss; pale, scattered petechiae on the lower legs, no lymphadenopathy, no liver or spleen enlargement. CBC hemoglobin 8.6, WBC 2.4, platelets 24 (A8); CMP normal; hepatitis C viral load not detected (resulted 2026-05-04). **Pancytopenia diagnosed**; urgent hematology referral |
| 2026-05-18 | outside | hematology consult; letter received 2026-05-20: 2 months of increasing fatigue, easy bruising and frequent nosebleeds; no fever, night sweats or weight loss; history of hepatitis C cured with treatment, reinfection in 2024 also cured (R2, R3); pale, scattered petechiae on the lower extremities, no lymphadenopathy or hepatosplenomegaly. Hemoglobin 8.2, WBC 2.1, platelets 20, reticulocytes 0.5%; peripheral smear without blasts or abnormal cells, all lines reduced in number (R4). Vitamin B12 and folate normal (A9). Bone marrow biopsy planned |
| 2026-05-27 | outside | bone marrow biopsy (A10); report received 2026-06-01 |
| 2026-06-08 | outside | hematology follow-up letter (A11) |
| 2026-08-10 | outside | hematology follow-up letter (A12) |
| 2026-09-14 | practice | booked follow-up visit (P) |

Last seen 2026-05-01. Everything after it is records filed as received, messages and
refills; the practice's problem and medication lists stay as they were on 2026-05-01.

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Dates follow source.md's practice dates (ED 2024-05-13, hematology 2026-05-18).
  Ages follow: 43 at the ED visit, 45 at hematology, as the source says.
- **R2.** The hematology note stamps reinfection "diagnosed 2020-01-15" and calls the ED
  visit "in 2020". Resolution: reinfection is diagnosed at the ED visit on 2024-05-13,
  and the hematology letter refers to it as 2024. The source's hematology problem list
  keeps reinfection active; the chart treats it (A5, A6) and the hematology letter
  describes it as a past, cured reinfection.
- **R3.** "Successfully treated with antiviral therapy 5 years ago" (hematology, 2026) and
  "quit 5 years ago" are both read from the 2026 visit: the first hepatitis C course
  and smoking cessation are in 2021, before he joined.
- **R4.** The source's peripheral smear reports "decreased cellularity", a marrow term.
  Resolution: the smear reads as no blasts or abnormal cells with all three lines
  reduced; cellularity is judged on the marrow (A10).
- **Left as the source has it:** naltrexone is continued unchanged through the acute
  hepatitis and on every list; no one holds it or comments on it. How he was reinfected
  is never established; he denies use and no note pursues it.

## Additions (A): physician to confirm

- **A1.** The 2023-07-17 new patient visit (seed: three years with the practice; the ED
  note already carries his problem list).
- **A2.** Hepatitis C viral load not detected at joining, confirming the 2021 cure.
- **A3.** At the ED: hepatitis C antibody reactive and viral load 2,400,000 IU/mL, the
  evidence for the reinfection diagnosis; a right upper quadrant ultrasound with normal
  liver echotexture, no biliary dilation, no gallstones, normal spleen size, no ascites.
- **A4.** The post-ED practice visit and its hepatic panel (values above), and the
  hepatology referral.
- **A5.** Hepatology consult: genotype 1a, viral load 1,150,000 IU/mL, LFTs as above;
  sofosbuvir/velpatasvir 400/100 mg daily for 12 weeks, standard treatment for the
  reinfection the source asserts but never treats.
- **A6.** Sustained virologic response: viral load not detected 2024-12-09, LFTs as above;
  reinfection resolved.
- **A7.** Yearly hepatitis C viral load at primary care (2025-07-14), not detected.
- **A8.** The 2026-05-01 practice visit that sends him to hematology: CBC hemoglobin 8.6,
  WBC 2.4, platelets 24; CMP normal; viral load not detected. Pancytopenia diagnosed.
- **A9.** At hematology: vitamin B12 512 pg/mL and folate 11.8 ng/mL, normal.
- **A10.** Bone marrow biopsy 2026-05-27 (the next step of the source's pancytopenia with
  a bland smear and low reticulocytes): hypocellular marrow, about 15% cellularity, with
  reduced trilineage hematopoiesis; no dysplasia, no increase in blasts, no fibrosis or
  infiltrate; normal male karyotype; no PNH clone by flow cytometry.
- **A11.** Hematology letter 2026-06-08: the marrow names the pancytopenia as aplastic
  anemia, non-severe, felt likely related to his hepatitis; cyclosporine 200 mg by mouth
  twice daily and eltrombopag 150 mg by mouth daily started by hematology, with counts,
  cyclosporine levels, kidney function and LFTs drawn at hematology; platelet
  transfusion for bleeding or platelets under 10. Hemoglobin 8.0, WBC 2.0, platelets 18
  that day. These drugs appear in the letters only, not on the practice list.
- **A12.** Hematology letter 2026-08-10: tolerating both drugs; hemoglobin 9.0, WBC 2.6,
  platelets 31; nosebleeds less frequent; no transfusion needed so far; continue.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from a prior practice may mention routine care done there, with normal results.
- Follow-up for OUD in remission and PTSD about every 6 to 12 months: PHQ-9, refills of
  naltrexone and sertraline, his recovery program attendance.
- A yearly physical with CMP, CBC and lipids; colorectal screening from 45 by FIT
  (negative); HIV screening once (nonreactive); influenza and COVID vaccines each fall;
  Tdap at joining.
- Refills, pharmacy and prior-authorization traffic, portal messages about results,
  refills and scheduling, the missed December 2025 visit.
- Booked visit after 2026-08-27: a follow-up on 2026-09-14.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002145 on 2026-05-01. Practice contacts after that day are
only messages, refills and the result of the draw from that visit; outside care after
it still happens, and its records are filed as received without changing the
practice's lists. A visit is booked after today.
