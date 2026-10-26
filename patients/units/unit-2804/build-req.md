# Build requirements: unit-2804 (Jennifer Esposito, MRN00002804)

Source: synthetic_hospital patient 2804, in `source.md` beside this file. This file and
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
"(diagnosed ...)" stamps, or its unfinished text.

## Who she is (S)

Jennifer Esposito, born 1978-10-02, female, White, not Hispanic or Latino, English,
Commercial PPO, lives in Florence (01062). Office administrator. Never smoker, no
alcohol. No known drug allergies. No surgical history. Family history: mother with
diverticulosis, sister with deep vein thrombosis, father with type 2 diabetes.

Joined the practice on 2023-09-12 (new-patient visit, A1), age 44. She brought:
- chronic constipation (for years, managed with over-the-counter regimen);
- diverticulosis of the colon, found at a colonoscopy at her prior practice in March
  2021 (A2), done for constipation and occasional blood on the tissue with hard
  stools: sigmoid diverticulosis, no polyps, repeat in 10 years;
- home medications: polyethylene glycol 3350 17 g by mouth daily, psyllium husk 3.4 g
  by mouth daily, docusate sodium 100 mg by mouth twice daily. These continue
  unchanged through today.

The occasional blood with hard stools (S, encounter 0) is her own long-standing report,
attributed to constipation; it carries no separate diagnosis.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-09-12 | practice | new-patient visit (A1): history as above; weight 68.0 kg, height 165 cm; routine labs normal; hepatitis C and HIV screens nonreactive (P) |
| 2025-06-13 | outside (ED) | 2 days of constant sharp left lower quadrant pain worse with movement, low-grade fever, poor appetite; no travel, sick contacts or prior similar episodes; occasional blood with hard stools in the past. T 38.2 °C, BP 130/85, HR 92, RR 18. LLQ tenderness with voluntary guarding, no rebound, normal bowel sounds. WBC 14,000/µL, hemoglobin 13.2 g/dL, platelets 250,000/µL, CRP 25 mg/L. CT abdomen/pelvis with contrast (A3): sigmoid diverticulitis without abscess, perforation or free air; background diverticulosis; pancreas, portal and splenic veins normal. **Diverticulitis diagnosed**; discharged home on **amoxicillin-clavulanate 875/125 mg twice daily for 10 days** (A3). ED record received 2025-06-16 |
| 2025-06-18 | practice | post-ED visit (P): pain easing, afebrile, finishing antibiotics; colonoscopy 2021 counts as recent enough; no repeat planned |
| 2025-07-01 | practice | telehealth (P): course complete, pain gone, eating normally. **Diverticulitis resolved** |
| 2025-08-11 to 2025-08-28 | outside (hospital) | admission for **acute necrotizing pancreatitis** (R1): severe epigastric pain; lipase 2,140 U/L; triglycerides 142 mg/dL; right upper quadrant ultrasound without gallstones or duct dilation; CT abdomen with contrast: necrosis of under 30% of the pancreas, no gas, no infected collection; portal and splenic veins patent. Supportive care (fluids, analgesia, bowel rest then diet); no surgery, no drainage, no antibiotics. No cause identified (no alcohol, no gallstones, normal triglycerides, no new drugs). Discharge summary received 2025-08-29 |
| 2025-09-03 | practice | post-discharge visit (P): eating small meals, weight 63.6 kg; CBC and CMP normal; lipase normal |
| 2025-10-14 | outside (GI) | gastroenterology follow-up letter (A4): pain resolved, eating normally, weight recovering; lipase normal, A1c 5.4%. **Acute pancreatitis resolved; history of necrotizing pancreatitis** carried on the list. No further imaging planned unless symptoms recur |
| 2026-02-13 | outside (ED, then admission to 2026-02-17) | 2 days of black stools, then vomiting blood, lightheadedness and fatigue; history of diverticulosis and of necrotizing pancreatitis six months earlier treated in hospital without surgery. BP 98/60, HR 108, RR 16, T 37.1 °C. Pale, fatigued; mild left upper quadrant tenderness without guarding or rebound. Hemoglobin 7.6 g/dL, platelets normal, INR 1.1 (A5). CT abdomen/pelvis with contrast (A5): thrombus in the main portal vein; gastric fundal varices; liver normal in contour; no ascites; no peripancreatic collection. Upper endoscopy: **isolated gastric varices with active bleeding from the fundus**, treated with cyanoacrylate injection with hemostasis (A5). 2 units of packed red cells, octreotide infusion, IV pantoprazole, ceftriaxone for 5 days total (inpatient only). **Upper GI hemorrhage, gastric varices, portal vein thrombosis diagnosed.** Discharged on **nadolol 20 mg daily** and **pantoprazole 40 mg daily for 4 weeks** (A5). Discharge hemoglobin 9.4 g/dL. Discharge summary received 2026-02-18 |
| 2026-02-20 | practice | post-discharge visit (P): no further bleeding; hemoglobin 10.6 g/dL |
| 2026-03-11 | outside (GI/hepatology) | consult letter (A6): repeat upper endoscopy 2026-03-10 shows the treated fundal varix obliterated, no bleeding stigmata. Thrombophilia testing negative (JAK2 V617F, factor V Leiden, prothrombin G20210A, antiphospholipid antibodies). **Apixaban 5 mg twice daily started** for portal vein thrombosis, planned for at least 6 months, then repeat CT to decide on duration; nadolol continued; pantoprazole ends on schedule. **Upper GI hemorrhage resolved** |
| 2026-05-19 | practice | follow-up (P): no bleeding, taking apixaban and nadolol; resting HR in the low 60s; weight 66.0 kg; hemoglobin 12.6 g/dL |
| 2026-06-16 | outside (GI/hepatology) | follow-up letter (A6): well; continue apixaban and nadolol; repeat CT abdomen with contrast planned for September 2026 (not yet done) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source lists "history of necrotizing pancreatitis" at the June 2025 ED visit,
  yet the February 2026 ED note places the episode "six months ago". Resolution: the
  pancreatitis admission falls in August 2025, between the two ED visits (about six
  months before February 2026). The June 2025 ED record and every earlier record never
  mention pancreatitis; the profile's listing of it from the start is not carried.
- **R2.** The February 2026 ED note copies diverticulitis as an active problem.
  Resolution: diverticulitis resolved on 2025-07-01; the February record may mention it
  only as past history.
- **R3.** The source names portal vein thrombosis as a primary diagnosis but gives only
  the endoscopy. Resolution: the thrombus is shown on the February 2026 CT (A5); the
  isolated gastric varices are its consequence.
- **R4.** The source's year references follow the re-dated timeline: "eight months
  after her diverticulitis visit" is June 2025 to February 2026.
- **Left as the source has it:** no hemoglobin trend or iron studies after the bleed
  beyond the values listed; no imaging follow-up of the necrosis after the August 2025
  admission (so the portal vein thrombosis is found only at the bleed); no cause found
  for the pancreatitis. Nobody remarks on these.

## Additions (A): physician to confirm

- **A1.** 2023-09-12 new-patient visit (three years with the practice; the source's
  first encounter is an ED visit, so she was already a patient).
- **A2.** The March 2021 prior-practice colonoscopy (sigmoid diverticulosis, no polyps),
  which is where her diverticulosis was found.
- **A3.** The June 2025 ED CT and the 10-day amoxicillin-clavulanate course: the source
  gives diverticulitis with no imaging or plan.
- **A4.** The October 2025 GI follow-up letter after the pancreatitis admission
  (A1c 5.4%, lipase normal).
- **A5.** February 2026 admission details: hemoglobin 7.6 g/dL (discharge 9.4), INR 1.1,
  the CT showing the portal vein thrombus and fundal varices, cyanoacrylate injection,
  transfusion of 2 units, octreotide, IV pantoprazole, ceftriaxone, discharge on nadolol
  20 mg daily and pantoprazole 40 mg daily for 4 weeks.
- **A6.** Hepatology care for the portal vein thrombosis: repeat endoscopy (varix
  obliterated), negative thrombophilia testing (her sister's DVT makes it standard),
  apixaban 5 mg twice daily from 2026-03-11, repeat CT planned for September 2026.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from the prior practice (received 2023-09) may mention routine screening there
  with normal results: Pap with HPV co-test in October 2020 negative, screening mammogram
  2022 normal, Tdap 2019, COVID primary series.
- Preventive visits yearly (2024-09, 2025-11); labs: CBC, CMP, lipids, A1c (normal),
  TSH; hepatitis C and HIV once at joining; PHQ-2 negative.
- Screening mammogram yearly, normal (BI-RADS 1). Pap with HPV co-test at the November
  2025 physical, negative. Colorectal screening covered by the 2021 colonoscopy.
- Influenza vaccine each fall; updated COVID vaccine each fall.
- Hemoglobin values after the bleed as listed (9.4, 10.6, 12.6); otherwise CBC normal.
  Weight 68.0 kg at joining, 63.6 kg after the pancreatitis, 66.0 kg by May 2026.
- Refills, pharmacy and prior-authorization traffic (apixaban), portal messages about
  results, refills and scheduling, a missed or cancelled visit.
- Booked visit after 2026-08-27: annual physical on 2026-09-22.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00002804 on 2026-05-19. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. A visit is booked after today.
