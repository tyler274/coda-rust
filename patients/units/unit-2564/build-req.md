# Build requirements: unit-2564 (Tyler Wilder, MRN00002564)

Source: synthetic_hospital patient 2564, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself or stops short;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, refills, portal
traffic, forms, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dumps, its
"(diagnosed ...)" stamps or its unfinished assessment.

## Who he is (S)

Tyler Wilder, born 1992-07-02, male, White, not Hispanic or Latino, English, lives in
Florence (01062), employer-sponsored PPO (UnitedHealthcare). Regional sales
representative. Never smoker. Alcohol socially, 1 to 2 drinks a week. No drug use.
No known drug allergies. No surgical history before 2025. Family history: father with
hypertension diagnosed at 55, controlled; mother with hypothyroidism; no family
history of bleeding or clotting disorders. Single (texture; no partner or children
appear in the chart).

Joined the practice 2024-09-10 (A1), a healthy 32-year-old with no problems and no
medications. Height 180 cm, weight about 81 kg, normal blood pressure throughout.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-09-10 | practice | new-patient visit (A1): healthy, no problems, no medications, NKDA; family and social history as above; normal exam. Labs (A2): CBC with **hemoglobin 15.0, hematocrit 45** (the "baseline" the 2025 ED quotes), CMP, lipids, HIV and hepatitis C screens nonreactive |
| 2025-01-03 | outside (regional hospital ED) | high-speed motor vehicle collision, restrained driver, airbags deployed. Severe chest pain and dyspnea. RR 28, BP 90/60, HR 120, SpO2 92% on room air. Anxious, diaphoretic; decreased breath sounds and dullness on the left; neck veins flat; abdomen soft, nontender. Hb 11 (baseline 15), Hct 33, WBC 12.0, platelets 250. Chest x-ray: multiple left rib fractures and a large left hemithorax opacification consistent with fluid. **Traumatic hemothorax with hypovolemic shock, multiple left rib fractures** diagnosed. **Left tube thoracostomy** in the ED drains blood; transfusion started (A3) |
| 2025-01-03 (evening) | outside (same hospital, ICU) | ICU admission after ED care: agitated and confused; BP 90/60, HR 120, RR 22, T 37.2 °C. Multiple contusions over chest and abdomen, **deformed right femur**, oozing from IV sites. Hb 10.2, platelets 55, PT 18, PTT 45, fibrinogen 150, D-dimer elevated. **Disseminated intravascular coagulation** (R1), **acute posthemorrhagic anemia**, **contusion of chest wall / blunt chest trauma**, and **closed fracture of the right femoral shaft** (R2) diagnosed. Managed with blood products (R1, A3) |
| 2025-01-05 | outside (same hospital) | coagulation corrected (A3): platelets 138, PT 13.6, fibrinogen 236. Right femur **intramedullary nail** by orthopedics (A4) |
| 2025-01-07 | outside (same hospital) | chest tube removed after drainage slows; chest x-ray: left lung expanded, small blunting of the left costophrenic angle, no pneumothorax (A3) |
| 2025-01-11 | outside (discharge) | discharged home on day 9: Hb 9.1, platelets 296, PT normal. DIC resolved. **Discharge drugs (A5): oxycodone-acetaminophen 5/325 every 6 hours as needed (short course), ferrous sulfate 325 mg daily.** Weight bearing as tolerated with walker; orthopedics and trauma clinic follow-up. Discharge summary reaches the practice 2025-01-13 |
| 2025-01-16 | practice | post-discharge visit (P, transitional care): pain controlled, walking with walker; Hb 10.6, ferritin 38 (A6). Plan: continue iron, recheck |
| 2025-01-22 | outside | orthopedics letter: incisions healed, staples out; x-ray alignment and hardware satisfactory; outpatient physical therapy (A4) |
| 2025-01-29 | outside | trauma surgery clinic letter: breathing comfortably; chest x-ray 2 views: lungs clear, no effusion, healing left rib fractures (A3). **Traumatic hemothorax resolved** |
| 2025-02-03 | practice (portal) | asks the practice to refill oxycodone; PCP declines and suggests acetaminophen; **oxycodone-acetaminophen stopped**; **acetaminophen 500 mg as needed started** (P) |
| 2025-02-11 | practice | follow-up: off opioids, in physical therapy, walking with a cane; Hb 12.8 (A6) |
| 2025-04-16 | outside | orthopedics letter: fracture united, full weight bearing without aid, cleared to return to work, finish therapy (A4). **Femur fracture and rib fractures resolved** |
| 2025-04-22 | practice (lab) | Hb 14.7, ferritin 64 (A6). **Ferrous sulfate stopped**; **acute posthemorrhagic anemia resolved** |
| 2025-09-23 | practice | annual physical (P): well, back at work full time; normal exam; CBC normal; influenza vaccine |
| 2026-06-22 | outside (regional hospital ED) | second high-speed motor vehicle collision, restrained driver, airbags deployed. BP 90/60, HR 120, RR 28, SpO2 89% on room air. Alert, anxious, diaphoretic; decreased breath sounds and dullness on the left, bruising over the left lateral chest wall, trachea deviated to the right; abdomen soft, nondistended; pelvis stable; no long-bone fracture. Hb 9.8, Hct 29, platelets 250, PT 12, aPTT 30. FAST negative for abdominal fluid. Chest x-ray (R3): large left hemothorax with rightward mediastinal shift, no new rib fracture, healed old left rib fractures. **Traumatic hemothorax with hypovolemic shock, acute posthemorrhagic anemia, contusion of chest wall** diagnosed again. Emergent **left tube thoracostomy** and 2 units of red cells (A7); admitted to trauma surgery |
| 2026-06-26 | outside (discharge) | chest tube out 2026-06-25; chest x-ray: left lung expanded, no pneumothorax, trace blunting of the left costophrenic angle (A7). Discharged on day 4: Hb 10.3. **Discharge drugs (A7): acetaminophen 500 mg and ibuprofen 600 mg as needed, short course.** No coagulopathy this time. Summary reaches the practice 2026-06-29 |
| 2026-07-01 | practice | post-discharge visit (P): sore but breathing comfortably; Hb 11.4, ferritin 71 (A8); no iron started |
| 2026-07-08 | outside | trauma surgery clinic letter: chest x-ray 2 views clear, no effusion (A7). **Traumatic hemothorax resolved**; chest wall contusion settling |
| 2026-07-21 | practice (portal) | back at work, chest wall barely sore, off ibuprofen; return-to-work letter (P). **Chest wall contusion / blunt chest trauma resolved** |
| 2026-08-18 | practice (lab) | Hb 13.9 (A8). **Acute posthemorrhagic anemia resolved** |
| 2026-09-22 | practice (booked) | annual physical |

## Resolutions (R): physician to confirm

- **R1.** The ICU note stops before its assessment and plan. Resolution: the labs
  (platelets 55, PT 18, PTT 45, fibrinogen 150, elevated D-dimer, oozing IV sites)
  are read as trauma-induced DIC and treated with red cells, plasma, platelets and
  cryoprecipitate (A3). No unit counts or product numbers appear beyond A3.
- **R2.** The ED note does not mention the leg; the ICU exam finds a deformed right
  femur. Resolution: the femur is found on the ICU secondary survey and an x-ray
  shows a closed right femoral shaft fracture. The ED record says nothing about the
  leg, as written (interobserver difference kept).
- **R3.** The 2026 ED note orders a chest x-ray and stops. Resolution: the film shows
  what the exam describes (a large left hemothorax with rightward shift) and nothing
  more, plus the healed rib fractures known from 2025.
- **R4.** Same-day encounters 0 and 1 are one hospital stay: the ED record, then the
  ICU admission note the same evening. Hb 11 in the ED and 10.2 in the ICU are both
  carried. The 2026 note's "January 2020" is January 2025.
- **R5.** Vitals in the ED 2025, ICU 2025 and ED 2026 are all BP 90/60 and HR 120;
  all are carried as written for each record.
- **Left as the source has it:** two high-speed collisions as a restrained driver
  eighteen months apart. Nobody in the chart remarks on driving, fatigue, alcohol
  with driving, or a pattern, and no screening or counseling for it is added. No
  hematology referral or workup after the DIC; no one orders one.

## Additions (A): physician to confirm

- **A1.** The 2024-09-10 new-patient visit (the seed has him two years with the
  practice; the source's first encounter is a trauma visit with no practice).
- **A2.** Hb 15.0 and Hct 45 at that visit, so that the ED's "baseline" has a source.
- **A3.** Standard care of the 2025 injuries: left chest tube in the ED; transfusion
  of red cells, plasma, platelets and cryoprecipitate for shock and DIC; coagulation
  corrected by 2025-01-05 (platelets 138, PT 13.6, fibrinogen 236); tube removed
  2025-01-07 with the x-ray as listed; discharge Hb 9.1, platelets 296; trauma clinic
  chest x-ray 2025-01-29 clear with healing rib fractures.
- **A4.** Femur care: intramedullary nail 2025-01-05; orthopedics letters 2025-01-22
  (healed incisions, alignment good, PT) and 2025-04-16 (united, back to work);
  outpatient physical therapy through spring 2025.
- **A5.** Discharge oxycodone-acetaminophen 5/325 short course and ferrous sulfate
  325 mg daily for the posthemorrhagic anemia.
- **A6.** Anemia recovery 2025: Hb 10.6 and ferritin 38 (2025-01-16), Hb 12.8
  (2025-02-11), Hb 14.7 and ferritin 64 (2025-04-22).
- **A7.** Standard care of the 2026 injury: emergent left chest tube, 2 units of red
  cells, admission to trauma surgery, tube out 2026-06-25, discharge 2026-06-26 with
  Hb 10.3 and the x-ray as listed, acetaminophen and ibuprofen as needed; trauma
  clinic chest x-ray 2026-07-08 clear.
- **A8.** Anemia recovery 2026: Hb 11.4 and ferritin 71 (2026-07-01), Hb 13.9
  (2026-08-18).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as the source or this file gives it.
- Records from his prior practice (a physical in 2022 with normal results, Tdap in
  2019) arrive after he joins.
- Post-discharge visits, a yearly physical, a CBC at each post-injury visit and at
  the physical; CMP and lipids once at joining.
- Influenza vaccine each fall (2024, 2025), COVID vaccine each fall (2024, 2025).
  HIV and hepatitis C screening once (2024, nonreactive). PHQ-2 at visits, negative.
- Portal messages about results, refills and scheduling; work and disability forms,
  a return-to-work letter, records requests from his auto insurer, a cancelled visit.
- Booked visit after 2026-08-27: annual physical 2026-09-22.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
