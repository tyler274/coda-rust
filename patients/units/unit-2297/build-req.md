# Build requirements: unit-2297 (Chad Wojcik, MRN00002297)

Source: synthetic_hospital patient 2297, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz-style framing.

## Who he is (S)

Chad Wojcik, born 1957-04-27, male, White, not Hispanic or Latino, English, Medicare,
Sunderland MA 01375. Retired postal worker. Married; his wife is the person who
brings him to the ED in December 2024 (S: "his wife reports"). Former smoker, 35
pack-years, quit in September 2024 at the time of his myocardial infarction (R1).
Alcohol: social, 1 to 2 beers on weekends. Allergies: no known drug allergies.

Surgical history: diagnostic cardiac catheterization and percutaneous coronary
intervention with one drug-eluting stent to the left anterior descending artery,
September 2024 (R1).

Family history: father had a myocardial infarction at 62 and has died; mother has
COPD and is living; one older brother has type 2 diabetes.

**Joining.** Joins the practice on 2024-10-08 (new patient visit, A1), about a month
after his MI, having left a prior primary-care office. He brings:

- Problems: COPD, GOLD stage II; coronary artery disease with myocardial infarction
  September 2024 and an LAD drug-eluting stent; hypertension; hyperlipidemia;
  history of tobacco dependence (35 pack-years, quit September 2024).
- Drugs: aspirin 81 mg daily; atorvastatin 40 mg daily; metoprolol succinate 50 mg
  daily; lisinopril 10 mg daily; tiotropium 18 mcg inhaled daily (dry-powder
  capsule); fluticasone propionate/salmeterol 250/50 mcg one inhalation twice daily;
  albuterol 90 mcg/actuation, inhaled every 4 to 6 hours as needed for wheezing or
  shortness of breath.

These seven drugs continue unchanged through the whole chart.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| about 2024-09-09 to 2024-09-12 | outside | admission at the county medical center for acute myocardial infarction; diagnostic catheterization and one drug-eluting stent to the LAD on 2024-09-10; stops smoking (R1). The practice receives this as the hospital's discharge summary with his prior-practice records after joining (A1) |
| 2024-10-08 | practice | **new patient visit** (A1): establishes care; history and drugs as above; records requested from the prior office and the hospital |
| 2024-12-11 | outside | ED visit at the county medical center: 2 days of severe headache, high fever and confusion; wife reports he has been less responsive and complaining of a stiff neck. T 39.2 °C (102.5 °F), HR 110, BP 130/85. Disoriented to time and place; resistance and pain with neck flexion; decreased breath sounds at the right lower lobe. WBC 15,000/µL; blood glucose 150 mg/dL. CSF: cloudy; WBC 2,000/µL; protein 180 mg/dL; glucose 30 mg/dL; Gram stain gram-positive cocci in pairs. **Bacterial meningitis due to Streptococcus pneumoniae** diagnosed; admitted |
| 2024-12-11 to 2024-12-21 | outside | inpatient stay for the meningitis (A2): CSF culture grows Streptococcus pneumoniae; IV ceftriaxone with vancomycin and dexamethasone at first, vancomycin and dexamethasone stopped once the culture returned; 10 days of IV ceftriaxone completed in hospital; mental status back to baseline; discharged home on his usual seven drugs, no new prescriptions. Discharge summary reaches the practice |
| 2024-12-30 | practice | post-discharge visit (P): meningitis **resolved** on the practice list |
| 2025-04-03 to 2025-04-12 | outside | ICU admission at the county medical center for **acute hypoxemic respiratory failure without hypercapnic encephalopathy**; intubated 2025-04-03 (R2) |
| 2025-04-08 | outside | ICU extubation assessment, ventilator day 5: on pressure support 5 cm H2O with PEEP 5 cm H2O. T 37.2 °C, BP 135/85, HR 88, SpO2 94% on these settings, RR 16. Alert, follows commands, lifts his head off the pillow for 5 seconds, strong cough. ABG 30 minutes earlier on these settings: pH 7.32, PaCO2 45 mmHg, PaO2 78 mmHg on FiO2 40%. Extubated that day (R3). Discharged 2025-04-12 on his usual seven drugs (R2) |
| 2025-04-22 | practice | post-discharge visit (P): respiratory failure **resolved** on the practice list; referred to pulmonology (A4) |
| 2025-06-11 | outside | pulmonology consult (A4): spirometry FEV1/FVC 0.62, FEV1 61% predicted after bronchodilator, consistent with COPD GOLD 2; inhalers continued unchanged |
| 2025-10-01 to 2025-10-11 | outside | ICU admission at the county medical center for **pneumonia with septic shock** (R4) |
| 2025-10-02 | outside | ICU procedure: on norepinephrine for septic shock from pneumonia; central venous catheter placement under ultrasound for multiple infusions. BP 90/60, HR 110, RR 22, SpO2 92% on 2 L nasal cannula. The catheter would not advance and aspiration returned bright red, briskly flowing blood. An arterial gas drawn at once from that sample: pH 7.45, PaCO2 35 mmHg, PaO2 95 mmHg, HCO3- 24 mEq/L, confirming arterial placement (R5). **Accidental arterial cannulation during central venous catheter placement** recorded; needle withdrawn, pressure held, line placed at a second site (A5). Sputum culture sent (R4, A6). Discharged 2025-10-11 on his usual seven drugs |
| 2025-10-21 | practice | post-discharge visit (P): pneumonia, septic shock and the arterial cannulation **resolved** on the practice list; follow-up chest x-ray ordered (A7) |
| 2025-11-18 | outside imaging | follow-up chest x-ray, 2 views: clear lungs (A7) |
| 2026-08-10 | practice | last practice visit (seed `last_seen`); routine follow-up with a draw |
| after 2026-08-27 | practice | booked routine follow-up visit |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Every encounter, across ten months, calls the MI "recent (3 months prior)" and
  says he quit smoking "3 months ago." Resolution: the MI, catheterization, stent and
  smoking quit date are fixed about 3 months before the first encounter, in September
  2024 (stent 2024-09-10). From the April 2025 admission on, records describe the MI
  as September 2024, not as recent; an outside note may still copy the old wording.
- **R2.** The source gives no cause for the April 2025 respiratory failure beyond the
  diagnosis itself. Resolution: the chart names the failure and its care only. "Admitted
  for respiratory failure in May 2020" in the October note is that April 2025 admission.
  Admission 2025-04-03, intubation that day (the source's "intubated 5 days ago"),
  discharge 2025-04-12.
- **R3.** The extubation note stops at the readiness question. Resolution: he passed the
  assessment and was extubated on 2025-04-08 without reintubation. The quiz framing is
  not carried.
- **R4.** The source's comorbidity "opportunistic pulmonary infection requiring diagnostic
  sampling" is carried as the October 2025 pneumonia, sampled with a sputum culture.
  "Opportunistic" is not carried: nothing in the source makes him immunocompromised.
  Pneumonia and septic shock are diagnosed at that admission (2025-10-01). The source's
  "Acute hypoxemic respiratory failure" on the October problem list is the April episode,
  already resolved.
- **R5.** The October oxygen saturation of 92% on 2 L and a PaO2 of 95 are both kept: the
  gas is the sample aspirated through the misplaced needle, drawn to confirm it was
  arterial, not a measure of his oxygenation on the cannula.
- **R6.** The source states his age as 67 in October 2025; by the seed's birth date he is
  68 then (67 in December 2024 and April 2025). Records follow the birth date.
- **R7.** The source lists all three hospital diagnoses on later problem lists as active.
  Resolution: meningitis, respiratory failure, pneumonia, septic shock and the arterial
  cannulation each resolve at the practice visit after its discharge.
- **Left as the source has it:** no P2Y12 inhibitor appears with aspirin after the
  September 2024 drug-eluting stent, and no one adds or remarks on one. Atorvastatin stays
  40 mg after the MI. The ED record carries the decreased right lower lobe breath sounds
  with no chest imaging named. These gaps are kept without comment.

## Additions (A): physician to confirm

- **A1.** The 2024-10-08 new-patient visit (he arrives with every problem and drug in
  place), and the September 2024 MI discharge summary and prior-office records that
  arrive afterward. Prior-office records show a screening colonoscopy in 2019 with no
  polyps and a Tdap in 2019.
- **A2.** The December 2024 inpatient course: a pneumococcal meningitis is admitted from
  the ED and treated. CSF culture Streptococcus pneumoniae; ceftriaxone 10 days with
  initial vancomycin and dexamethasone; discharged 2024-12-21 at baseline.
- **A3.** Pneumococcal conjugate vaccine (PCV20) once, February 2025, after the
  pneumococcal meningitis and with his COPD.
- **A4.** Pulmonology referral after the April 2025 respiratory failure, and the
  June 2025 consult's spirometry (FEV1/FVC 0.62, FEV1 61% predicted post-bronchodilator)
  to support the source's GOLD stage II. A yearly pulmonology follow-up (June 2026) finds
  him stable on the same inhalers.
- **A5.** After the arterial puncture: needle withdrawn without dilation, firm pressure
  held, catheter placed at a second site under ultrasound; no hematoma or other harm.
- **A6.** Sputum culture from the October 2025 pneumonia: normal respiratory flora, no
  predominant pathogen; treated with IV antibiotics in hospital, no antibiotics at
  discharge.
- **A7.** Follow-up chest x-ray 6 to 8 weeks after the pneumonia (2025-11-18): clear.
- **A8.** Lung cancer screening (35 pack-years, quit 2024): low-dose CT chest 2025-01-21
  and 2026-01-20, both Lung-RADS 1.
- **A9.** Cardiology follow-up of his CAD at the outside cardiology group (2024-11-06,
  2025-08-19, 2026-03-17): no angina, no drug changes.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal
  results.
- Visits for COPD, CAD, hypertension and hyperlipidemia every few months; a post-discharge
  visit after each admission; Medicare annual wellness visits (initial 2025, subsequent
  2026); metabolic panel, CBC, lipids and A1c (normal range) as those conditions call for.
  Blood pressure at goal on his drugs; LDL under 70 on atorvastatin 40.
- Prevention: high-dose influenza each fall; COVID vaccines; PCV20 (A3); Shingrix series
  in 2025; RSV vaccine fall 2025; Tdap from 2019 on file. Hepatitis C screening once
  (negative). One-time abdominal aortic ultrasound for a male ever-smoker (normal aortic
  caliber, about 2.0 cm). Depression screens negative.
- Refills, pharmacy and prior-authorization traffic, portal messages about results,
  refills and scheduling, a missed visit, a cancelled visit, calls from his wife while he
  is in hospital.
- Booked visit after 2026-08-27: a routine follow-up.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Records

Complete: outside letters, ED records, ICU notes, discharge summaries and reports arrive
within a few days and are filed; the practice's lists are kept current through the last
visit.

## Last seen

The practice last saw MRN00002297 on 2026-08-10. Practice contacts after that day do not
happen; outside care after it still does, and its records are filed as received without
changing the practice's lists. A visit is booked after today.
