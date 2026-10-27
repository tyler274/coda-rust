# Build requirements: unit-2149 (Theo Lewandowski, MRN00002149)

Source: synthetic_hospital patient 2149, in `source.md` beside this file. This file and
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
"(diagnosed 2020-07-15)" stamps, or its quiz text ("cells labeled Y").

## Who he is (S)

Born 1986-04-29, male, White, not Hispanic or Latino, English, Blue Cross Blue Shield,
lives in western Massachusetts (seed town). Wildlife biologist. Married (the source's
wife). Never smoker; social alcohol, 2 to 3 drinks a week. No known drug allergies.
Surgical history: tonsillectomy at age 12, open appendectomy at age 25. Family history:
father with epilepsy and hypertension; mother with type 2 diabetes; paternal grandfather
MI at 62.

Joined the practice on 2024-06-11 (new-patient visit, A1), bringing:
generalized tonic-clonic seizure disorder, well controlled, on **levetiracetam 1000 mg
twice daily**; essential hypertension on **lisinopril 10 mg daily**. Both drugs continue
unchanged through the whole chart.

Records texture: sparse. Outside care reaches the practice as the patient's (or his
wife's) account, one short ED note and one short discharge summary, with few numbers.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-06-11 | practice | new-patient visit (A1): history as above; BP on lisinopril at goal; exam normal; establishing labs (P) |
| 2024-06-25 | practice | prior-practice records received (P): routine care, no new facts |
| 2024-12-21 | outside | ED, a hospital in New Mexico (R1), arrives by ambulance: sudden fever, chills and dyspnea; dry cough since the day before, now blood-tinged sputum; wife says he became acutely ill overnight and increasingly confused; took part in a prairie dog eradication project the week before; no international travel. T 39.6 °C, HR 124, BP 92/58, RR 30. Obtunded, coughing frothy pink sputum, crackles throughout both lungs. Sputum gram stain and culture: pleomorphic gram-negative rod. Chest x-ray: bilateral alveolar infiltrates with necrotizing features. **Intubated for hypoxic respiratory failure**, admitted to the ICU for presumed septic shock. **Pneumonic plague** and **acute hypoxemic respiratory failure** diagnosed (R2). The practice receives only the short ED note, on 2025-01-10 |
| 2024-12-23 | practice | wife's portal message that he is in an ICU out of state (P) |
| 2024-12-21 to 2025-01-03 | outside | ICU then ward stay (A2): culture confirmed Y. pestis; IV gentamicin; extubated on hospital day 5; discharged on oral **ciprofloxacin 500 mg twice daily** to finish a 14-day total course. Known to the practice only by the patient's account |
| 2025-01-14 | practice | post-hospital visit (P, A2): his account of the stay; antibiotics finished (ciprofloxacin recorded as a completed outside short course); levetiracetam and lisinopril were continued in hospital; tired, cough nearly gone; lungs clear; SpO2 normal; CBC and BMP; follow-up chest x-ray ordered (A3). **Acute hypoxemic respiratory failure resolved** |
| 2025-02-25 | practice-ordered | chest x-ray, 2 views (A3): lungs clear, infiltrates resolved, no effusion. **Pneumonic plague resolved** on the result note |
| 2025-04-08 | practice | last seen. Preventive visit and follow-up: back at work, no seizures, BP at goal on lisinopril; levetiracetam and lisinopril refilled |
| 2025-12-23 | outside | ICU, a regional hospital, pulmonary and critical care: submerged in cold water for several minutes, rescued, hypoxemic despite supplemental oxygen. Tachypnea, diffuse crackles over both lungs, accessory muscle use. Arterial gas: hypoxemia with a widened alveolar–arterial gradient (no numbers). Chest x-ray: bilateral patchy opacities, no cardiomegaly. **Acute respiratory distress syndrome with refractory hypoxemia** and **acute hypoxemic respiratory failure** (R3). Course (A4): intubated, lung-protective ventilation with prone positioning, extubated on hospital day 6, discharged home 2026-01-05 on his usual two drugs. Practice hears from his wife by portal 2025-12-29; the short discharge summary arrives 2026-01-30 |
| 2026-04-21 | outside | pulmonary clinic follow-up (A5): breathing back to baseline; lungs clear; spirometry within normal limits; chest x-ray clear. Short letter received 2026-05-04 |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source says he "lives in rural New Mexico", but he is a western
  Massachusetts patient of the practice. Resolution: he was in New Mexico for a work
  assignment that included the prairie dog project; the ED and ICU are there, and his
  wife was with him. The source's "works in wildlife control" is read as his wildlife
  biologist job.
- **R2.** The source lists pneumonic plague only as a diagnosis at encounter 1 and stamps
  it on the later list. Resolution: plague is diagnosed at the 2024-12-21 ED visit and
  resolved after the clear February 2025 film; the 2025-12 ICU summary mentions it as
  past history, not an active problem.
- **R3.** The source's primary diagnoses "Refractory hypoxemia in ARDS" and the
  comorbidity "acute hypoxemic respiratory failure without hypercapnic encephalopathy"
  belong to the 2025-12-23 ICU stay; the earlier plague visit has its own hypoxic
  respiratory failure (intubated). Both episodes carry that diagnosis, each resolved.
- **R4.** The source's pathology section is a quiz about a slide from an unrelated case.
  The chart carries no pathology from either stay.
- **R5.** Wife's report of confusion and "obtunded" exam stay as the ED wrote them; no
  seizure is attributed to either illness and levetiracetam is never changed.
- **Left as the source has it:** none beyond the above; the source leaves no care gap
  inside its span.

## Additions (A): physician to confirm

- **A1.** 2024-06-11 new-patient visit (the source's first encounter already lists him on
  two chronic drugs; he needs a primary-care home before it).
- **A2.** The plague hospital course after the ED: culture confirmation, IV gentamicin,
  extubation on hospital day 5, discharge 2025-01-03 on ciprofloxacin 500 mg twice daily
  to complete 14 days. Needed so the source's intubated, shocked patient is alive and
  treated with standard plague therapy.
- **A3.** One follow-up chest x-ray 2025-02-25: clear (documents resolution of the
  necrotizing infiltrates).
- **A4.** The ARDS ICU course: lung-protective ventilation, prone positioning, extubated
  hospital day 6, discharged 2026-01-05.
- **A5.** One pulmonary clinic follow-up 2026-04-21 with normal spirometry and a clear
  chest x-ray (usual post-ARDS check).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from a prior practice may mention routine screening done there, with normal results.
- Hypertension and seizure follow-up at practice visits through 2025-04-08 with vitals;
  BMP for lisinopril; CBC, CMP, lipids and A1c once; HIV and hepatitis C screening once;
  PHQ-2 depression screens.
- Influenza vaccine in the fall (2024 at the practice; 2025 at a pharmacy, reported by
  portal); COVID vaccine fall 2024; Tdap if due.
- Refills, pharmacy traffic, portal messages about results, refills and scheduling, a
  missed visit.
- No practice visit after 2025-04-08; after it only messages, refills and records filed
  as received, without updating the practice's lists.
- Booked visit after 2026-08-27: an overdue follow-up after the ICU stay.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002149 on 2025-04-08. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. A visit is booked after today.
