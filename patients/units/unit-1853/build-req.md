# Build requirements: unit-1853 (Delaney Martel, MRN00001853)

Source: synthetic_hospital patient 1853, in `source.md` beside this file. This file and
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
"(diagnosed ...)" stamps, or its quiz text ("histologic section of the lung is shown").

## Who she is (S)

Delaney Martel, born 1992-05-16, female, White, not Hispanic or Latino, English,
Blue Cross Blue Shield PPO, lives in the seed's town. Environmental consultant. Never
smoker; social alcohol, 1 to 2 drinks a week (none while pregnant). Allergies: latex
(rash); penicillin (reaction unknown to her, childhood label). Surgical history: knee
arthroscopy at age 24 (2016, before the practice); low transverse cesarean (2024, R2).
Family history: mother with a preterm delivery and seasonal allergies; father with
hypertension and type 2 diabetes; a maternal aunt with susceptibility to high-altitude
pulmonary edema. Marital status married; the spouse is not named or described.

Joined the practice 2022-09-12 (A1, new-patient preventive visit, age 30). Brought:
seasonal allergic rhinitis, on levocetirizine 5 mg daily during allergy season
(over the counter). No pregnancies before 2024 (R2). No other problems or drugs.

## Timeline (dates fixed; S unless marked)

LMP-based dating: first pregnancy LMP 2024-04-19 (32w0d on 2024-11-29); second
pregnancy LMP 2025-12-04 (34w0d on 2026-07-30). Both from R3.

| Date | Where | What happens |
|---|---|---|
| 2022-09-12 | practice | new-patient preventive visit (A1); history as above; seasonal allergic rhinitis on the list, **levocetirizine 5 mg daily in season (historical)**; screening labs (P) |
| 2024-02-26 | practice | preconception visit (A2): planning a pregnancy; **prenatal vitamin with DHA, 1 tablet daily, started** |
| 2024-05-20/22 | practice | positive home pregnancy test, confirmed by urine hCG at a nurse visit (P); **pregnancy on the list**; referred to an outside obstetrics group (P) |
| 2024-06 to 11 | outside | outside obstetric prenatal care; uncomplicated pregnancy (S "received prenatal care ... uncomplicated") |
| 2024-11-29 to 2024-12-04 | outside | obstetric admission, regional hospital (discharge summary received 2024-12-09): presents in preterm labor at 32 weeks; given tocolysis and a full antenatal corticosteroid course, labor continues; **delivered 2024-12-01 by primary low transverse cesarean** (R2; breech, A3). Newborn (sex not stated) to the NICU: respiratory rate 70, heart rate 160, SpO2 85% on room air, decreased breath sounds both sides, chest x-ray reticulogranular pattern with air bronchograms; neonatal respiratory distress syndrome (R1). Mother's recovery routine, discharged 2024-12-04. **Pregnancy closed; history of preterm labor on the list**; cesarean added to surgical history |
| 2025-01-13 | practice | postpartum visit (P); infant home from the NICU (A4); levocetirizine and prenatal vitamin continue |
| 2025-07-21 to 2025-07-22 | outside | emergency department, then overnight observation (record received 2025-07-24): progressive shortness of breath and dry cough after a hiking trip in the Rocky Mountains, ascending to 12,000 feet over two days, now home; no sick contacts or recent illness. Temp 37.2 C (99 F), BP 130/80, pulse 95, RR 26, SpO2 86% on room air, 94% on supplemental oxygen. Bilateral basal crackles. WBC 11.0 (11,000/mm3), hemoglobin 14.5, platelets 250, procalcitonin 0.1 ng/mL (normal under 0.5). Room-air arterial gas pH 7.45, PaCO2 30, PaO2 55, HCO3 24. Chest x-ray bilateral alveolar infiltrates. **High-altitude pulmonary edema** with hypoxemia. Oxygen, weaned to room air, discharged next day (A5) |
| 2025-07-28 | practice | post-ED visit (P): breathing back to normal, SpO2 normal on room air, lungs clear; follow-up chest x-ray ordered (A6); counseled on slow ascent for any future altitude travel |
| 2025-08-25 | practice | follow-up chest x-ray, 2 views: clear lungs (A6); **HAPE resolved** |
| 2026-01-12/14 | practice | positive home pregnancy test, urine hCG positive at a nurse visit (P); **pregnancy on the list** (supervision of pregnancy with history of preterm labor); referred to outside obstetrics / maternal-fetal medicine (P) |
| 2026-05-01 | practice | last practice visit (seed `last_seen`): about 21 weeks, pregnancy uncomplicated per OB; spring allergy symptoms; levocetirizine and prenatal vitamin continue |
| 2026-07-30 to 2026-08-03 | outside | obstetric admission (discharge summary received 2026-08-10): sudden preterm labor at 34 weeks after an uncomplicated pregnancy; **delivered 2026-07-30 by repeat low transverse cesarean** (A7). Newborn boy: warming and tactile stimulation, ongoing respiratory distress; diminished breath sounds both sides with fine rales; arterial gas with hypoxemia and respiratory acidosis; chest x-ray diffuse reticulogranular (ground-glass) pattern with air bronchograms; intubated and admitted to the NICU for oxygen and further care; respiratory arrest and death on 2026-07-31 despite full support (R1, A8). Lung tissue sent for histology; report goes to the obstetric team (R4). Mother discharged 2026-08-03 |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source puts the newborns' findings in the mother's encounters and lists
  "Respiratory distress syndrome", "Respiratory arrest" and "Preterm birth at 27 weeks"
  among her diagnoses. Resolution: respiratory distress syndrome, its vitals, exam,
  gases and x-rays, and the respiratory arrest belong to the infants and appear only in
  the neonatal paragraphs of the obstetric discharge summaries. They are never on the
  mother's problem list or structured data. The ED record's copied history does not
  carry RDS. "Hypoxemia" is the mother's, at the 2025 ED visit, as part of HAPE.
- **R2.** The profile lists a low transverse cesarean and "history of preterm labor" at
  the first encounter, whose HPI describes an uncomplicated pregnancy until preterm
  labor, and the third encounter says "history of preterm labor returns". Resolution:
  the 2024 pregnancy is her first; its preterm labor makes the history, and its delivery
  is the cesarean. Neither exists before 2024-12-01. "Preterm birth at 27 weeks" is not
  carried (it contradicts 32 and 34 weeks).
- **R3.** The source gives gestational ages only. Resolution: LMPs as above, so the
  practice dates fall at 32w0d and 34w0d.
- **R4.** The "histologic section ... is shown" image is quiz text. Resolution: the
  discharge summary records that lung tissue went for histology with results to the
  obstetric team; no histology result appears in the chart.
- **R5.** The ED HPI dates the preterm labor "January 2020"; it follows the re-dated
  timeline (late 2024). The third encounter's "He" for the patient is the newborn.
- **R6.** Encounter 2's text runs the mother's labor into the newborn's resuscitation.
  Resolution: the mother's part is the preterm labor and delivery; everything after
  birth is the newborn's.
- **Left as the source has it:** no altitude prophylaxis drug is prescribed and no
  further cardiopulmonary testing is done after HAPE. The source names no preterm-birth
  prevention for the second pregnancy; none appears.

## Additions (A): physician to confirm

- **A1.** 2022-09-12 new-patient visit: the source starts at an outside admission with
  prenatal care already under way, and she has four years with the practice.
- **A2.** 2024-02-26 preconception visit, where the prenatal vitamin (on every source
  list) starts.
- **A3.** First delivery by cesarean for breech presentation of the preterm fetus: the
  profile's cesarean needs an indication; physician to confirm.
- **A4.** The 2024 infant survives the NICU stay and goes home in early January 2025 (the
  source leaves the outcome open; later notes mention the child only briefly).
- **A5.** ED disposition: overnight observation on oxygen, room air SpO2 at least 94% at
  rest by morning, discharged with a slow-ascent handout.
- **A6.** One follow-up 2-view chest x-ray 2025-08-25, clear, closing the HAPE episode.
- **A7.** Second delivery by repeat low transverse cesarean (active preterm labor with a
  cesarean twenty months earlier).
- **A8.** The newborn's death is dated 2026-07-31.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as the source or this file gives it. A report
  may carry the descriptors its kind always carries, but nothing that adds a territory,
  a severity grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal
  results (Pap with HPV 2020, Tdap 2019).
- Yearly preventive visits in non-pregnant years (2023, 2025) with CBC, metabolic panel,
  lipids, A1c (father with diabetes); HIV and hepatitis C screening once at joining;
  Pap with HPV in 2025, normal; PHQ-2 or PHQ-9 at preventive visits and postpartum,
  low scores.
- Influenza and updated COVID vaccines most falls (2024 during pregnancy at the
  practice).
- Spring allergy contacts, prenatal vitamin refills, portal messages about results and
  scheduling, a cancelled visit, outside obstetric and maternal-fetal medicine letters during each
  pregnancy saying it is progressing normally (normal prenatal labs, dating and anatomy
  ultrasounds, glucose screen, and in 2026 normal cervical length checks).
- After 2026-05-01: only the filed discharge summary and portal messages (a note of
  condolence from the PCP and an offer to talk). No visit is booked.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001853 on 2026-05-01. Practice contacts after that day do
not happen; outside care after it still does, and its records are filed as received
without changing the practice's lists. Nothing is booked after today.
