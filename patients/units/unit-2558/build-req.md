# Build requirements: unit-2558 (Brandon Hastings, MRN00002558)

Source: synthetic_hospital patient 2558, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself or stops short;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, refills, portal
traffic, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dumps, its
"(diagnosed ...)" stamps, or its unfinished plan text.

## Who he is (S)

Born 1996-06-19, male, White, not Hispanic or Latino, English-speaking, private
insurance through the ACA marketplace (Silver tier). Lives in town, zip 01060.
Independent delivery contractor (gig work, driving and cycling). Never smoker.
Alcohol occasional, 1 to 2 drinks a month. No drugs. Allergies: no known drug
allergies. No surgical history before 2026. No chronic conditions, no home
medications. Family history: father hypertension diagnosed at 52; mother
hypothyroidism; no known family history of bleeding or clotting disorders or
connective tissue disease. Religion: Jehovah's Witness; he declines blood
transfusion (S, encounter 2 social history; recorded by the practice at joining, see R6).

Joined the practice in 2022 (A1: new-patient preventive visit 2022-09-13) with no
problems and no medications. Height 180 cm, weight about 76 kg, steady through 2026-02.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-09-13 | practice | new-patient preventive visit (A1); history as above; normal exam; CBC, CMP, lipid panel, HIV and hepatitis C screening, all normal (P); Tdap (P) |
| 2023-10-12 | practice | preventive visit, normal (P) |
| 2024-10-21 | practice | preventive visit, normal (P) |
| 2025-01-12 to 2025-01-15 | outside | regional hospital admission, trauma surgery (S enc 0). Bicycle crash, helmeted, no loss of consciousness, fell on right side and struck right shoulder on a curb. Vital signs stable on arrival. Tender over right clavicle; decreased breath sounds on the right. Chest x-ray: **right clavicle fracture** and **small right pneumothorax**, no rib fracture, no hemothorax. Then swelling and ecchymosis develop over the right supraclavicular area: **traumatic injury of right subclavian vessels** (R1: CT angiogram, A2). Supplemental oxygen and analgesia; **oxycodone-acetaminophen 5/325 short course** at discharge (R2). Repeat chest x-ray before discharge: pneumothorax resolved (A3). Sling. Discharge summary received 2025-01-16 |
| 2025-01-21 | practice | post-hospital visit (P): pain settling, sling, right radial pulse normal, fading supraclavicular bruise, lungs clear; traumatic pneumothorax resolved on the list |
| 2025-02-05 | outside | orthopedics clinic letter (A4): right shoulder x-ray, midshaft clavicle fracture in acceptable alignment with early callus; nonoperative; sling weaned |
| 2025-02-26 | outside | vascular surgery clinic letter (A2): repeat CT angiogram, supraclavicular hematoma resolved, subclavian artery of normal caliber with smooth wall, no pseudoaneurysm; discharged from vascular follow-up. Subclavian injury resolved |
| 2025-03-20 | outside | orthopedics letter (A4): fracture healed clinically and on x-ray (bridging callus); full activity including cycling. Clavicle fracture resolved |
| 2025-10-12 to 2025-10-16 | outside | regional hospital ICU, critical care (S enc 1). Motor vehicle collision. Age 29 (R3). Hypotensive, tachycardic, severe respiratory distress. Absent breath sounds and hyperresonance on the left, jugular venous distention, trachea deviated to the right: **tension pneumothorax, left**. Immediate needle decompression, then left chest tube (R4); chest x-ray after tube: left lung re-expanded (A5); hemodynamics normalized after decompression. He told the team he declines blood products; none were needed (R6). Tube removed hospital day 3; chest x-ray after removal: no residual pneumothorax (A5). Acetaminophen at discharge. Discharge summary received 2025-10-17 |
| 2025-10-20 | practice | post-hospital visit (P): breathing back to normal, lungs clear; chest x-ray 2 views, lungs fully expanded, no pneumothorax (A5); tension pneumothorax resolved on the list; flu vaccine |
| 2026-02-19 | practice | preventive visit (P), normal, CBC and lipid panel normal |
| 2026-07-06 to 2026-07-22 | outside | regional hospital trauma ICU then surgical floor (S enc 2). Severe motor vehicle collision; arrives unconscious with **multiple traumatic injuries**. BP 80/50, HR 130, RR 24. Pale, diaphoretic, distended abdomen, deformed femur (left, R5). Hb 6.0 g/dL, Hct 18%, platelets 250,000/µL. **Traumatic hemorrhagic shock with acute coagulopathy** (R7: INR 1.9, lactate 6.4 on arrival, A6). A friend who came with him reports he is a Jehovah's Witness who has always refused transfusion; no advance directive; he lacks capacity. Course (R8, A6): no blood products given; tranexamic acid; damage-control laparotomy finds hemoperitoneum from a **liver laceration (moderate)**, controlled with packing; left femur external fixation that night; return to the OR on day 2 for pack removal and closure; **left femoral shaft fracture** converted to intramedullary nail on day 4. Regains consciousness after resuscitation; CT head without contrast normal. Extubated day 3; on waking he confirms he refuses all blood products. **Acute posthemorrhagic anemia**: Hb nadir 5.4 on day 2; IV iron, epoetin alfa and folic acid in hospital; INR 1.2 by day 3; Hb 7.3 at discharge. Discharged home with walker, weight bearing as tolerated, home physical therapy. Discharge medicines: **oxycodone-acetaminophen 5/325 as needed (short)**, **enoxaparin 40 mg daily for 4 weeks (short)**, **ferrous sulfate 325 mg daily**, **folic acid 1 mg daily** (A6). Discharge summary received 2026-07-23 |
| 2026-07-28 | practice | **last seen.** Telehealth transitional-care visit (P): at home with walker; fatigue; incisions dry per patient; medicines reconciled; he confirms he declines blood products and asks that the chart say so. CBC ordered (A7) |
| 2026-07-30 | practice lab | CBC: Hb 8.8 (A7); result sent by portal |
| 2026-08-12 | outside | orthopedics clinic letter (A8): femur x-ray, nail in good position, early callus; weight bearing as tolerated; finish enoxaparin at 4 weeks; outpatient PT |
| 2026-08-14 | outside | trauma surgery clinic letter (A8): incision healed; Hb 10.1; no lifting over 10 lb for 6 weeks; no driving while on opioids |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source names a subclavian vessel injury from the supraclavicular swelling
  and bruising with no imaging. Resolution: a CT angiogram on 2025-01-12 shows a focal
  intimal irregularity of the right subclavian artery with a small surrounding
  supraclavicular hematoma, no active extravasation, no occlusion, distal flow preserved;
  managed without operation by serial pulse checks (see A2). The vessel injury is carried
  as arterial.
- **R2.** "Analgesia" is unnamed. Resolution: oxycodone-acetaminophen 5/325 as needed,
  a short course, and acetaminophen after encounter 1.
- **R3.** The source calls him 28 at the October collision; from the seed birth date he
  is 29 then and 30 in July 2026. Ages follow the seed. The source's "January 2020"
  bicycle crash is January 2025 in this chart.
- **R4.** Encounter 1's plan stops mid-sentence after "chest tube". Resolution: the tube
  is placed, the lung re-expands, the tube comes out on day 3 and he goes home day 4.
  No rib fractures or other injuries are named.
- **R5.** The source gives a deformed femur without a side. Resolution: left.
- **R6.** Jehovah's Witness and his refusal of transfusion appear only in encounter 2,
  which says he "has always refused" in the past. Resolution: the practice records his
  faith and his refusal at joining (2022), and encounter 1's summary records that he
  declined consent for blood products and none were needed. He has no advance directive
  until today, as the source says; the practice never makes one and nobody remarks on it.
- **R7.** "Acute coagulopathy" has no evidence in the source. Resolution: arrival INR 1.9
  (prothrombin time prolonged), lactate 6.4 mmol/L.
- **R8.** Encounter 2 ends at the labs with no decision on blood. Resolution: the team
  honors the friend's report while capacity is absent; bloodless management as in the
  timeline; he confirms the refusal on waking. The bleeding source is a liver laceration
  and the femur fracture; these name the source's "multiple traumatic injuries".
- **Left as the source has it:** the source keeps the subclavian injury and the tension
  pneumothorax on later active lists; outside documents from 2025-10 and 2026-07 may still
  carry them, while the practice list resolves them. No advance directive is completed.

## Additions (A): physician to confirm

- **A1.** 2022 new-patient visit: the seed has him with the practice four years.
- **A2.** CT angiogram on admission (R1) and repeat CT angiogram 2025-02-26 at vascular
  surgery: hematoma resolved, artery normal caliber, smooth wall, no pseudoaneurysm.
- **A3.** Repeat chest x-ray 2025-01-14: right pneumothorax resolved.
- **A4.** Orthopedic follow-up of the clavicle fracture (2025-02-05 early callus,
  acceptable alignment; 2025-03-20 healed, bridging callus).
- **A5.** Chest x-rays after the left chest tube (re-expanded), after tube removal (no
  residual pneumothorax), and 2025-10-20 at the practice (2 views, clear).
- **A6.** Encounter 2 course: INR 1.9, PT prolonged, lactate 6.4 on arrival; tranexamic
  acid; damage-control laparotomy with packing of a moderate liver laceration; external
  fixation then intramedullary nail of the left femur (displaced transverse midshaft
  fracture on x-ray); CT head normal; Hb nadir 5.4 day 2, Hb 7.3 at discharge; INR 1.2 by
  day 3; IV iron, epoetin alfa, folic acid; discharge medicines as listed. Standard
  bloodless care for a Jehovah's Witness in hemorrhagic shock.
- **A7.** Practice CBC 2026-07-30: Hb 8.8.
- **A8.** Orthopedic and trauma clinic letters in August 2026 (Hb 10.1). Filed after
  last_seen; the practice lists are not updated from them.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from a prior primary-care office may mention routine care there, normal.
- Preventive visits about yearly (2022, 2023, 2024, 2026-02; the late-2025 physical was
  cancelled by him and redone in February). Influenza vaccine most falls, COVID vaccine
  some falls, Tdap 2022. HIV and hepatitis C screening once (2022), negative. PHQ-2
  screens 0 or 1. Lipids and CBC at times, normal.
- Post-hospital visits after each admission, transitional care calls.
- Portal messages about results, forms, scheduling, work questions, insurance renewal.
- Booked visit after 2026-08-27: an in-person follow-up after the July admission
  (2026-09-08).

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002558 on 2026-07-28 (telehealth). Practice contacts after
that day are only messages, the draw ordered that day and records filed as received;
the practice's lists stay as they were on 2026-07-28. A visit is booked after today.
