# Build requirements: unit-2563 (Lucas Wisniewski, MRN00002563)

Source: synthetic_hospital patient 2563, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, its "CT scan is shown" quiz text or its unfinished plans.

## Who he is (S)

Born 1996-05-06, male, White, not Hispanic or Latino, English-speaking, lives in South
Hadley, employer-sponsored private insurance. Regional sales representative (drives a
territory for work). Never smoker. Social drinker, 1 to 2 drinks a week. No known drug
allergies. No surgeries before 2024-10-27. Family history non-contributory; no family
history of bleeding disorders or chronic liver disease. Takes no medications.

Joined the practice in September 2023 (A1): a healthy 27-year-old with no problems and
no medications. Height 180 cm; weight about 82 kg, steady (P), about 78 kg at the visit after
the January 2026 surgery (A5).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-09-12 | practice | new-patient preventive visit (A1); history as above; normal exam; screening labs normal; influenza vaccine (P) |
| 2023-09-19 | outside | prior primary care records received: routine physicals, normal labs, Tdap 2019, childhood hepatitis B series complete (P) |
| 2024-09-17 | practice | annual physical, normal (P) |
| 2024-10-27 (Sun) | outside | regional trauma center ED, age 28: brought in after a high-speed motor vehicle collision. Hemodynamically stable on arrival, then hypotensive and tachycardic about 30 minutes later: BP 82/46, HR 128. Pale, diaphoretic, left upper quadrant pain radiating to the left shoulder; abdomen tender with guarding and rebound; no external injuries or fractures. FAST: free intraperitoneal fluid. CT abdomen and pelvis done while still stable (R3, A2). Fluid resuscitation, then emergency exploratory laparotomy. **Traumatic splenic rupture** and **traumatic hemorrhagic shock with acute coagulopathy** diagnosed. Admission through 2024-11-01 (A2) |
| 2024-11-04 | outside | ED record, operative report and discharge summary received (P) |
| 2024-11-06 | practice | post-discharge visit (P): recovering; **acquired absence of spleen** recorded (A3); **splenic rupture and hemorrhagic shock with coagulopathy resolved** (R2); hemoglobin 10.6 (A2); asplenia counseling; vaccines planned at 14 days after surgery; short opioid course finished, **acetaminophen-oxycodone stopped** |
| 2024-11-12 | practice | nurse visit: PCV20, MenACWY dose 1, MenB-4C dose 1, Hib (A3); influenza vaccine (P) |
| 2024-11-13 | outside | trauma surgery clinic letter: incision healing, staples removed, light duty, no lifting over 10 lb for 6 weeks (A2) |
| 2024-12-10 | outside + practice | trauma surgery letter: cleared for full activity (A2); practice CBC hemoglobin 13.6 (A2) |
| 2025-01-14 | practice | nurse visit: MenACWY dose 2, MenB-4C dose 2 (A3) |
| 2025-05-17 (Sat) | outside | regional trauma center ED, about seven months after the splenic rupture: restrained driver in a head-on collision. Alert and oriented, anxious. BP 130/85, HR 95, RR 18, SpO2 98% on room air. Right upper quadrant tenderness with guarding, no rebound; no external trauma. Hemoglobin 13.2 g/dL, hematocrit 39.5%, platelets 250,000/uL; liver function tests normal. FAST: free fluid in the hepatorenal recess. **Blunt abdominal trauma with suspected hepatic injury** diagnosed. CT and observation admission through 2025-05-19 (R4, A4) |
| 2025-05-21 | outside | ED record, CT report and discharge summary received (P) |
| 2025-05-23 | practice | post-ED visit (P): right upper quadrant soreness easing; exam benign; **acetaminophen as needed stopped** when no longer needed |
| 2025-06-11 | outside | trauma surgery clinic letter: symptoms resolved, no repeat imaging, back to full activity (A4) |
| 2025-09-09 | practice | annual physical: no-show (P) |
| 2025-09-16 | practice | annual physical, normal labs; influenza vaccine (P). **Suspected hepatic injury resolved** on the problem list (R2) |
| 2026-01-13 (Tue) | outside | community hospital ED after a motor vehicle collision: seatbelt bruising across the lower abdomen; initial workup (CBC normal, CT abdomen and pelvis with contrast without acute abnormality) (R5); discharged |
| 2026-01-15 (Thu) | outside | regional trauma center ED, age 29: severe lower abdominal pain beginning suddenly 4 hours before, sharp and constant, steadily worse; no nausea, vomiting or change in bowel habits. BP 100/60, HR 120, RR 22, T 37.8 C. Diffuse tenderness, rebound and guarding; hypoactive bowel sounds. WBC 15,000/uL, hemoglobin 13.2 g/dL, lactate 3.5 mmol/L. Upright chest X-ray: free air under the diaphragm. **Traumatic perforation of small intestine with secondary peritonitis** diagnosed. Repeat laparotomy and admission through 2026-01-21 (A5) |
| 2026-01-23 | outside | ED records (both visits), operative report, pathology and discharge summary received (P) |
| 2026-01-27 | practice | post-discharge visit (P): eating, bowels moving, incision healing; CBC WBC 8.4, hemoglobin 12.6 (A5); weight about 78 kg; **acetaminophen-oxycodone stopped** |
| 2026-02-04 | outside | general surgery clinic letter: healing well, pathology reviewed, no lifting over 10 lb until early March (A5) |
| 2026-03-10 | practice | nurse visit: MenB-4C booster, delayed from January by the admission (A3) |
| 2026-03-11 | outside | general surgery letter: cleared for full activity, discharged from surgical follow-up (A5); the practice files it and **resolves the small-bowel perforation with peritonitis** |
| 2026-09-17 | practice | booked annual physical (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The 2025 and 2026 ED notes say "no prior surgeries" after the emergency
  laparotomy of 2024. Resolution: the 2024 laparotomy was a splenectomy (A2), and every
  record after it lists exploratory laparotomy with splenectomy (October 2024); the
  2026 record also lists the 2025 admission. "No prior surgeries" is true only before
  2024-10-27.
- **R2.** The source keeps "traumatic splenic rupture" active seven and fifteen months
  later, and "suspected hepatic injury" active nine months after its symptoms ended.
  Resolution: outside ED notes may carry them as history, as outside notes do. On the
  practice list the splenic rupture and the hemorrhagic shock with coagulopathy are
  resolved at the 2024-11-06 post-discharge visit, where acquired absence of spleen
  replaces them and stays; the hepatic injury is resolved at the September 2025
  physical. It keeps the source's label ("suspected") because imaging never proved it (R4).
- **R3.** "The CT scan of the abdomen is shown" gives no findings, and a CT does not fit
  a patient who goes from shock straight to the operating room. Resolution: the CT was
  done during the first stable half hour; its findings are in A2. The FAST stays as the
  source gives it.
- **R4.** The 2025 ED note stops before any plan. Resolution: stable with a positive
  FAST, he gets a CT abdomen and pelvis with contrast and an observation admission to
  trauma surgery (A4). Nonoperative.
- **R5.** "An initial workup that showed no acute abnormalities" names no tests.
  Resolution: at the 2026-01-13 community ED, a CBC (normal values) and a CT abdomen and
  pelvis with contrast without acute abnormality; normal vital signs; discharged with
  return precautions. The bowel injury was not seen; no record explains that.
- **R6.** "Motor vehicle accident" is listed as a comorbidity. Resolution: it is the
  mechanism of each injury, carried in the narrative of each record, not as a problem-list
  entry.
- **R7.** The 2026 HPI calls the earlier events "previous motor vehicle collisions";
  the dates make the collisions 2024-10-27, 2025-05-17 and 2026-01-13. Ages: 28 in
  October 2024, 29 in May 2025 (after his May 6 birthday) and January 2026.
- **Left as the source has it:** three collisions in 15 months. No note asks about
  driving safety, sleep, distraction or mood after the crashes, and nobody refers him
  anywhere for it. His anxiety in the 2025 ED stays in that ED note only.

## Additions (A): physician to confirm

- **A1.** Joining in September 2023 (seed: about 3 years with the practice): a
  new-patient preventive visit for a healthy man with no problems or medications.
- **A2.** The 2024 trauma admission, so the source's laparotomy, shock and coagulopathy
  have their course and evidence. CT (before the drop in pressure): high-grade splenic
  laceration through the hilum with large-volume hemoperitoneum and active contrast
  extravasation; liver, kidneys, pancreas and bowel without injury; no fracture.
  Arrival labs: hemoglobin 10.4 g/dL, prothrombin time 19.6 s with INR 1.7, platelets 142,000/uL. Massive transfusion
  protocol: 6 units red cells, 6 units plasma, 1 apheresis platelet unit. Operation:
  exploratory laparotomy, splenectomy, about 2.5 L of hemoperitoneum evacuated, bowel run
  without injury, liver intact. One night in the surgical ICU; post-operative hemoglobin
  low point 7.6 g/dL, 9.2 g/dL at discharge on 2024-11-01. Discharge medication:
  acetaminophen-oxycodone 325/5 mg, 1 tablet every 6 hours as needed, 12 tablets.
  Practice hemoglobin 10.6 on 2024-11-06 and 13.6 on 2024-12-10. Trauma surgery clinic:
  staples out 2024-11-13 with light duty and a 10 lb lifting limit for 6 weeks; cleared
  for full activity 2024-12-10.
- **A3.** Splenectomy needs the standard asplenia care: acquired absence of spleen on the
  problem list; PCV20, MenACWY, MenB-4C and Hib given at day 16 after surgery; second
  MenACWY and MenB-4C doses 2025-01-14; MenB-4C booster one year after the series
  (given 2026-03-10, late because of the January admission); MenACWY booster due every
  5 years (next early 2030); influenza yearly. Counseling: a fever is an emergency, seek
  care the same day; wear a medical alert ID.
- **A4.** The 2025 admission, so the "suspected hepatic injury" has its workup. CT
  abdomen and pelvis with contrast: small hypodensity at the edge of the right hepatic
  lobe (segment VI), possibly a small contusion or laceration; small amount of fluid
  around the liver and in the hepatorenal recess; no active extravasation; spleen
  surgically absent; no other injury. Observation on the trauma service; repeat
  hemoglobin 12.9 and then 13.0 g/dL; diet resumed; discharged 2025-05-19 with
  acetaminophen 500 mg as needed (no more than 3 g a day) and no heavy lifting or contact
  for 4 weeks. Trauma clinic 2025-06-11: pain gone, no repeat imaging, full activity.
- **A5.** The 2026 operation and recovery. Emergency repeat laparotomy through the old
  midline incision on 2026-01-15: full-thickness perforation of the ileum about 60 cm from
  the ileocecal valve with enteric spillage and fibrinous exudate; segmental resection
  of about 12 cm with stapled primary anastomosis; washout of the abdomen. Pathology:
  small-intestine segment with transmural perforation and acute serositis; margins viable.
  Intravenous piperacillin-tazobactam for 4 days after the operation. Diet advanced,
  discharged 2026-01-21 with acetaminophen-oxycodone 325/5 mg, 1 tablet every 6 hours as
  needed, 12 tablets. Practice 2026-01-27: WBC 8.4, hemoglobin 12.6, weight about 78 kg.
  Surgery clinic 2026-02-04: healing, 10 lb lifting limit until early March; 2026-03-11:
  full activity, discharged from surgical follow-up.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries (how a measurement was made, the incision
  and closure of an operation), but nothing that adds a territory, a severity grade or a
  symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening and immunizations done
  there, with normal results.
- Yearly physicals for a healthy man in his late twenties: vitals, PHQ screens with low
  scores, CBC, metabolic panel and lipids now and then, all normal. HIV and hepatitis C
  screening once (negative). Influenza vaccine each fall; updated COVID vaccines at a
  pharmacy.
- A post-discharge or post-ED practice visit after each outside episode; records
  received from each outside contact (the chart is complete: letters, reports and
  summaries arrive and are filed, and the lists are kept current).
- Work notes, portal messages about results, scheduling and vaccines, a missed visit,
  reminders.
- Booked visit after 2026-08-27: annual physical on 2026-09-17.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
