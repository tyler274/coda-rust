# Build requirements: unit-2687 (Ramon Negron, MRN00002687)

Source: synthetic_hospital patient 2687, in `source.md` beside this file. This file and
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

Ramon Negron, born 1991-10-24, male, Hispanic or Latino (race recorded as other),
English-speaking, Medicaid. Works as an agricultural pest control technician (mixes and
sprays pesticides in fields). Never smoker; alcohol occasionally, 1 to 2 drinks a month;
no drug use. No known drug allergies. Surgical history: tonsillectomy at age 8 (about
2000). Family history: father with type 2 diabetes, mother with essential hypertension.
No chronic conditions and no home medications.

Joined the practice in August 2023 (A1: new-patient visit 2023-08-22, age 31); he
brought no problems and no drugs. Last seen by the practice 2024-09-27; nothing booked.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-08-22 | practice | new-patient preventive visit (A1, P): history as above; healthy; height 172 cm, weight 74 kg; exam normal; screening labs and Tdap (P) |
| 2023-10-17 | practice | nurse visit, influenza and COVID vaccines (P) |
| 2024-08-30 | practice | annual physical, no-show (P) |
| 2024-09-27 | practice | annual physical, the last practice visit (P): still well, no concerns; weight 75 kg; exam normal; routine labs normal; influenza vaccine. Return in a year. He does not book |
| 2025-06-04 | outside | regional hospital ED, age 33 (R1): found confused and sweating in a field after several hours of spraying pesticides; before collapsing he told coworkers of blurred vision, shortness of breath and belly cramps; coworkers moved him out of the spray area and called for help. Arrival BP 92/56, pulse 48, respirations 28, SpO2 88% on room air. Disoriented, drooling heavily; pinpoint reactive pupils; tearing and runny nose; diffuse fasciculations; wheezing from bronchospasm; bradycardia; hyperactive bowel sounds with copious watery diarrhea; cool clammy skin, heavy sweating. **Organophosphate poisoning diagnosed.** ED course and disposition as R2 and A2 |
| 2025-06-11 | practice (portal) | he messages that he was in the ED for a pesticide exposure and is back at work; PCP offers a visit and asks the hospital for the record; he does not book (P) |
| 2025-07-29 | practice | ED record received late, filed (chart texture: gaps) |
| 2025-10 | outside | influenza vaccine at a retail pharmacy, reported to the practice (P) |
| 2026-06-05 to 2026-06-09 | outside | regional hospital ED then ICU, age 34 (R3): a year after the first episode; mixing a liquid insecticide without gloves when coworkers saw heavy sweating and drooling; vomited repeatedly on the way in and grew more short of breath; paramedics found pinpoint pupils and copious secretions hard to suction, took off his soaked shirt and rinsed his skin with water. On arrival anxious, sweating, gasping, with audible bronchial secretions; skin cool and clammy; tearing and runny nose; diffuse wheezes and crackles; fasciculations in calves and forearms; follows simple commands, cannot finish sentences for breathlessness; no sign of trauma. Bradycardic with a weak pulse; SpO2 low despite mask oxygen (numbers as R4). Bedside glucose normal (94, R4). Suctioning and preparation for intubation; course as R5 and A3. **Organophosphate poisoning, recurrent occupational exposure** |
| 2026-06-17 | practice (portal) | he messages that he was hospitalized again and is home; PCP advises a visit and asks the hospital for records (P) |
| 2026-07-21 | practice | hospital discharge summary received and filed |
| 2026-07-23 | practice (phone) | RN calls to schedule the post-hospital visit; voicemail left; nothing booked as of today (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source calls him 32 at the first ED visit; by the seed's birth date and the
  practice date he is 33. Age 33 in 2025 and 34 in 2026. The source's "one year after"
  matches the 2025 and 2026 dates.
- **R2.** The first ED note stops at the exam with no treatment or disposition. Resolution:
  standard care for a cholinergic crisis, written into the ED record: clothing removed and
  skin washed, oxygen, repeated IV atropine until secretions dried and the chest cleared,
  IV pralidoxime. Kept about a day in the ED observation unit and discharged home
  2025-06-05 with no prescriptions, told to stay off pesticide work until cleared and to
  see his PCP in a week. Later ED vital signs: about 3 hours after arrival BP 112/70,
  pulse 92, respirations 18, SpO2 96% on 2 L; at discharge BP 118/74, pulse 78,
  respirations 16, SpO2 98% on room air. The atropine and pralidoxime were given in
  hospital and are not on any medication list.
- **R3.** The source labels the second encounter "icu" under Emergency Medicine. It is one
  outside stay: ED arrival 2026-06-05, ICU 2026-06-05 to 2026-06-08, a night on the
  medical floor, discharged home 2026-06-09.
- **R4.** The second encounter gives no vital sign numbers. Arrival: pulse 44, BP 98/60,
  respirations 30, SpO2 87% on a non-rebreather mask. "Bedside glucose is normal": 94
  mg/dL. Next morning in the ICU: pulse 96, BP 118/72, respirations 18, SpO2 95% on 2 L.
  At discharge: pulse 76, BP 120/76, respirations 16, SpO2 98% on room air.
- **R5.** The source ends while the team "considers endotracheal intubation". Resolution:
  aggressive suctioning and escalating atropine boluses, then an atropine infusion, dried
  the secretions and improved oxygenation, so he was not intubated. Pralidoxime infusion,
  decontamination completed. Atropine weaned off over 2 days in the ICU; no
  complications. Discharged with no prescriptions, advised not to return to pesticide
  mixing or spraying until cleared, to wear gloves and protective equipment, and to see
  his PCP.
- **R6.** The source's problem list at the second encounter dates the poisoning to the
  first. The chart records organophosphate poisoning at the 2025 ED visit and again at
  the 2026 admission as a new exposure. Neither reaches the practice's own problem list:
  both happen after the last practice visit, and their records are filed as received.
- **Left as the source has it:** nobody at the practice sees him after either poisoning,
  no occupational medicine or workplace follow-up is arranged, and no repeat cholinesterase
  is drawn as an outpatient. The chart does not comment on it.

## Additions (A): physician to confirm

- **A1.** The 2023-08-22 new-patient visit (the practice has had him 3 years).
- **A2.** 2025 ED labs to support the diagnosis: red cell acetylcholinesterase 3100 U/L
  (low); basic metabolic panel and CBC otherwise unremarkable (normal values).
- **A3.** 2026 hospital labs: red cell acetylcholinesterase 2400 U/L (low) on arrival,
  2900 U/L at discharge; basic metabolic panel and CBC unremarkable (normal values).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from a prior practice may mention routine care done there (childhood
  vaccines, the tonsillectomy), with normal results.
- New-patient and annual labs for a healthy man in his early thirties with a father with
  type 2 diabetes: CBC, CMP, lipid panel, hemoglobin A1c, all normal; HIV and hepatitis C
  screening once, negative. PHQ-2 at each physical, negative.
- Influenza vaccine each fall he engages (2023 and 2024 at the practice, 2025 at a
  pharmacy), COVID vaccine 2023, Tdap 2023.
- Scheduling calls, portal messages about results and records, a missed visit, a
  reminder that a physical is due.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002687 on 2024-09-27. Practice contacts after that day are
messages and calls only; outside care after it still happens, and its records are filed
as received without changing the practice's lists. Nothing is booked.
