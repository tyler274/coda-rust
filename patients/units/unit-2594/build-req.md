# Build requirements: unit-2594 (Jacob Murphy, MRN00002594)

Source: synthetic_hospital patient 2594, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its unfinished plan text.

## Who he is (S)

Jacob Murphy, born 1996-06-29, male, White, not Hispanic or Latino, English, lives in
Northampton (01060), private employer-sponsored PPO. Field sales representative (drives
for work). Never smoker; alcohol occasional, 1 to 2 drinks a month. No known drug
allergies. No surgical history before 2025-11-07. No chronic conditions, no home
medications. Family history: father with hypertension (controlled), mother with
hypothyroidism, no family history of bleeding disorders or hematologic conditions.

Joined the practice 2024-06-11 (A1), age 27, healthy, on no medications, with nothing on
his problem list. Height 180 cm, weight about 79 kg.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-06-11 | practice | new-patient preventive visit (A1); history as above; normal exam; Tdap, HIV and hepatitis C screening, lipids, CBC, CMP (P); prior-practice records requested |
| 2024-10 | practice | nurse visit: influenza and COVID vaccines (P) |
| 2025-03-17 | (patient report) | bicycle accident: collided with a car, thrown over the handlebars, landed on his upper abdomen against the curb; checked at the scene, no apparent injury, no care sought (R1) |
| 2025-03-22 | outside | regional hospital ED (Saturday): two days of persistent upper abdominal discomfort and nausea, five days after the accident; vital signs normal; abdomen soft, epigastric tenderness, no rebound or guarding; Hgb 13.8 g/dL, WBC 9,000/mm3 (9.0 K/uL), amylase 450 U/L (30 to 110), lipase 210 U/L (0 to 160), liver tests normal; abdominal ultrasound: small peripancreatic fluid collection. **Traumatic pancreatic injury** and **blunt abdominal trauma** diagnosed. Admitted for nonoperative care, discharged 2025-03-25 (R2, A2) |
| 2025-03-26 | practice | portal message from the patient: was in the hospital, home now; records requested (P; gaps: no discharge summary yet) |
| 2025-03-31 | practice | post-hospital visit (A3): discomfort easing, eating; exam: mild epigastric tenderness only; lipase 52 U/L (A3) |
| 2025-04-22 | practice | regional hospital ED record and discharge summary received, a month late (gaps); filed |
| 2025-06-20 | practice | annual physical cancelled by the patient (work travel), not rebooked (P) |
| 2025-10 | practice | nurse visit: influenza vaccine (P) |
| 2025-11-07 | outside | regional hospital ED (Friday), trauma activation: high-speed motor vehicle collision, restrained driver, airbags deployed; alert, anxious, diffuse abdominal pain; BP 110/70, HR 105, RR 22, SpO2 97% on room air; abdominal distension, tenderness in all quadrants, seatbelt sign across the lower abdomen, no rebound or guarding; FAST: free fluid in the hepatorenal recess, no other abnormality. **Intra-abdominal hemorrhage requiring operative management**: **exploratory laparotomy** the same day with control of bleeding (R3, A4). Blunt abdominal trauma again. Discharged 2025-11-12 with **oxycodone 5 mg** every 6 hours as needed (short course) and acetaminophen (A4) |
| 2025-11-12 | practice | pharmacy fill history shows the oxycodone; the practice learns of the admission from it and calls him (gaps) |
| 2025-11-14 | practice | discharge summary received (no operative note ever arrives: gaps) |
| 2025-11-18 | practice | post-discharge visit (A5): midline incision healing, staples in; Hgb 11.9 g/dL; tapering oxycodone |
| 2025-11-21 | outside | trauma surgery clinic: staples removed, incision healed, lifting limit for six weeks (A6); letter received 2026-01-09 (late) |
| 2025-12-16 | practice | follow-up (A7): off oxycodone, back at work on light duty; Hgb 13.4 g/dL; PHQ-9 low (P) |
| 2026-04-17 | practice | annual physical, **last seen**: well; healed midline scar; CBC (Hgb 14.4 g/dL, A8), CMP, lipids (P) |
| 2026-07-16 | outside | regional hospital trauma ICU (Thursday): another high-speed motor vehicle collision, restrained driver, airbags deployed; alert, anxious, diaphoretic; BP 90/60, HR 120, RR 22, SpO2 97% on room air; abdominal distension, guarding, diffuse tenderness; no other injury on secondary survey; FAST: free fluid in the hepatorenal and splenorenal recesses. Hemorrhagic shock; plan in the note: resuscitation and operative exploration (R4) |
| 2026-07-24 | practice | the trauma ICU admission note arrives by fax and is filed as received; nothing further has arrived by today; the practice's lists are not changed (gaps; after last seen) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source dates the bicycle accident "5 days ago" from the ED visit. Resolution:
  the accident is 2025-03-17, the ED visit 2025-03-22, and symptoms began 2025-03-20.
- **R2.** The source's assessment and plan stop mid-sentence. Resolution: the ED admits him
  to the surgical service for nonoperative care (bowel rest, IV fluids, serial abdominal
  exams); diet is advanced and he goes home 2025-03-25. No CT is carried (the source names
  only the ultrasound), and no repeat enzyme values are given in the hospital records.
  The diagnosis is the source's "traumatic pancreatic injury", unspecified part.
- **R3.** The source does not say what was done on 2025-11-07, but the 2026 note calls it
  "intra-abdominal hemorrhage requiring operative management" in the earlier year and the
  problem list dates it to that encounter. Resolution: exploratory laparotomy on
  2025-11-07, bleeding controlled; the records name no bleeding organ and the chart names
  none. The source's age of 28 at that visit follows the birth date: he is 29. The
  source's "January 2020" and "2020" become March 2025 and November 2025.
- **R4.** Encounter 2 is a trauma ICU note with no plan. Resolution: the received note
  ends with resuscitation and operative exploration as the plan. Its outcome is not in the
  chart: no operative note or discharge summary has arrived by 2026-08-27.
- **Left as the source has it:** no CT at the first ED visit, and no follow-up pancreatic
  imaging. Nobody comments on either. Nobody comments on the two collisions in eight months.

## Additions (A): physician to confirm

- **A1.** New-patient visit 2024-06-11 (the source has no primary-care visits; seed gives
  two years with the practice).
- **A2.** Admission 2025-03-22 to 2025-03-25 for nonoperative care (R2).
- **A3.** Post-hospital practice visit 2025-03-31 with lipase 52 U/L (normal).
- **A4.** Exploratory laparotomy 2025-11-07 (R3); discharge 2025-11-12 with oxycodone
  hydrochloride 5 mg every 6 hours as needed for pain, short course, and acetaminophen
  500 mg tablets, 1000 mg every 8 hours as needed.
- **A5.** Post-discharge practice visit 2025-11-18, Hgb 11.9 g/dL.
- **A6.** Trauma surgery clinic visit 2025-11-21: staples removed, incision healed,
  no lifting over 15 pounds for six weeks.
- **A7.** Practice follow-up 2025-12-16, Hgb 13.4 g/dL; oxycodone stopped.
- **A8.** Hgb 14.4 g/dL at the 2026-04-17 physical.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice (received 2024-07) may mention routine care there, with
  normal results.
- Preventive visits for a man in his late twenties: blood pressure, weight, PHQ-2 or
  PHQ-9, AUDIT-C, lipids once, CBC and CMP; HIV and hepatitis C screening once; Tdap
  2024; influenza each fall; COVID vaccine 2024.
- Refills, pharmacy traffic, portal messages about results and scheduling, a cancelled
  physical, work-note requests.
- Weight about 79 kg, down to about 75 kg after the November 2025 laparotomy, back to
  about 78 kg by April 2026. Height 180 cm.
- Nothing booked after 2026-08-27.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Records (chart texture: gaps)

The first admission's records arrive a month late; the practice first hears of it from
the patient. The laparotomy is learned from a pharmacy fill; its discharge summary arrives,
its operative note never does. The surgery clinic letter arrives seven weeks late. The
2026 trauma ICU admission note arrives after the last visit and is filed without changing
the practice's lists; nothing else from that admission has arrived.

## Last seen

The practice last saw MRN00002594 on 2026-04-17. Practice contacts after that day do not
happen beyond messages and filing; outside care after it still does, and its records are
filed as received without changing the practice's lists. Nothing is booked.
