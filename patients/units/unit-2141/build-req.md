# Build requirements: unit-2141 (Christopher Chandler, MRN00002141)

Source: synthetic_hospital patient 2141, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself or is incomplete;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, refills, portal
traffic, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dumps, its
"(diagnosed ...)" stamps, or its quiz text.

## Who he is (S)

Born 1984-11-18, male, White, not Hispanic or Latino, English-speaking, Blue Cross Blue
Shield State Employee Plan, lives in Easthampton (01027). Married (the source's wife).
Crop farmer and licensed agricultural pesticide applicator; a brother works on the farm
with him. Never smoker; alcohol occasional, 2 to 3 drinks a month. No known drug
allergies. Surgical history: tonsillectomy at age 8. Family history: father with
hypertension diagnosed at 65; mother with hypothyroidism; no family history of
neurological or psychiatric illness. No chronic conditions and no home medications.

He had no primary-care physician before May 2025. He joins the practice after his first
hospitalization (A1), bringing no chronic problems and no medications. Records are
complete: every outside record listed below reaches the practice and is filed.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2025-05-07 (Wed) to 2025-05-10 | outside | Regional hospital ED, then medical ICU. Became ill while spraying pesticide in his field that day; difficulty breathing, excessive salivation, abdominal cramping; wife reports profuse sweating and confusion. Arrival vitals RR 28, HR 110, BP 135/85, T 37.2 C. Exam: tachypneic, constricted pupils, diffuse muscle fasciculations, wheezing, moist skin, garlic-like odor. ABG pH 7.30, PaCO2 50 mmHg, PaO2 60 mmHg. **Organophosphate poisoning diagnosed.** Treatment and course as A2. Discharged home 2025-05-10. The practice receives the ED record and discharge summary on 2025-05-20 (requested at joining). |
| 2025-05-21 (Wed) | practice | New-patient visit (A1): post-hospital follow-up and establishing care; symptoms resolved; normal exam; plasma cholinesterase recheck (A3) plus establishing labs (P). Off pesticide work until the recheck normalizes; protective equipment counseling. |
| 2025-06-25 (Wed) | practice | Lab-only cholinesterase recheck (A3), normal; cleared to return to applying with full protective equipment. **Organophosphate poisoning resolved.** |
| 2026-05-07 (Thu) to 2026-05-09 | outside | Same regional hospital ED, about one year after the ICU admission. Brought by his brother after suddenly collapsing on the farm. Had been spraying crops with a chemical pesticide that day without protective equipment, spilled some on his shirt and kept working; the pesticide had a strong, bitter smell. Dizziness, nausea, blurred vision, then within minutes heavy sweating, abdominal cramps, vomiting; then confusion, stumbling several times, and one brief generalized convulsion that stopped on its own. On arrival: drooling heavily, muttering incoherently, clothes damp with sweat, trembling, shallow breathing, intermittent cough with small amounts of clear sputum. No one else on the farm ill that day. Takes no medications; NKDA. Exam: pinpoint pupils sluggishly reactive, cool clammy skin, fasciculations in arms and legs, hyperactive bowel sounds, abdomen not distended or tender. Arrival vitals and labs as A4. **Organophosphate poisoning diagnosed again** (R2). Treatment and course as A4. Discharged home 2026-05-09. ED record and discharge summary reach the practice 2026-05-12. |
| 2026-05-14 (Thu) | practice | Post-hospital visit (P): recovering, no further convulsion; normal exam; cholinesterase recheck (A5); off pesticide handling until it recovers; protective equipment and decontamination counseling again. |
| 2026-06-17 (Wed) | practice | Lab-only cholinesterase recheck (A5), normal. **Organophosphate poisoning resolved.** |

## Resolutions (R): physician to confirm

- **R1.** The source labels encounter 0 "icu" in Emergency Medicine and gives no course or
  outcome. Resolution: ED presentation with ICU admission, three nights, discharged home
  well (A2). The arterial gas belongs to the ED/ICU record, not the practice.
- **R2.** The source's second encounter carries "Organophosphate poisoning (diagnosed
  2020-01-15)" as an active problem a year later. Resolution: the first episode resolved
  in June 2025 (A3); the May 2026 exposure is a new episode, diagnosed again, and resolves
  in June 2026 (A5). The source's 2020 and 2021 dates follow the re-dated timeline.
- **R3.** The second encounter gives no vitals and stops at the exam. Resolution: its
  vitals, labs, treatment and disposition are A4.
- **Left as the source has it:** after the convulsion in May 2026, no head imaging and no
  EEG are done and no one comments on it. He goes back to spraying in 2025 and is exposed
  again without protective equipment; nobody explains why.

## Additions (A): physician to confirm

- **A1.** New-patient visit 2025-05-21: he joins after the discharge summary advises
  primary-care follow-up.
- **A2.** First hospitalization treatment: clothing removed and skin washed; oxygen by
  non-rebreather mask; IV atropine boluses titrated until secretions dried and wheeze
  cleared, then atropine infusion; pralidoxime 2 g IV load then infusion; no intubation.
  Plasma cholinesterase on arrival 1,050 U/L (reference 5,320 to 12,920). Repeat ABG
  after treatment pH 7.39, PaCO2 41 mmHg, PaO2 88 mmHg on 2 L nasal cannula. Weaned to
  room air, atropine stopped day 2, pralidoxime stopped day 3; discharged 2025-05-10 on
  no medications.
- **A3.** Plasma cholinesterase recovery after the first episode: 4,400 U/L on
  2025-05-21; 7,600 U/L on 2025-06-25.
- **A4.** Second episode: arrival vitals HR 56, BP 104/62, RR 24, SpO2 89% on room air,
  T 36.6 C. Plasma cholinesterase 820 U/L. Same decontamination, oxygen, atropine and
  pralidoxime as A2; no further convulsion; no intubation. Observed two nights in the
  ICU; discharged 2026-05-09 on no medications.
- **A5.** Plasma cholinesterase recovery after the second episode: 3,900 U/L on
  2026-05-14; 7,400 U/L on 2026-06-17.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Establishing labs at the new-patient visit: CBC, CMP, lipid panel, A1c, HIV and
  hepatitis C screening once, all normal. BMI normal; blood pressure normal at practice
  visits.
- Influenza and COVID vaccines in fall 2025; Tdap given once if no record of one.
  PHQ-2 negative.
- Portal messages about results, return to work, scheduling, a records request,
  a cancelled visit during the busy planting or harvest season.
- Booked visit after 2026-08-27: annual physical.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
