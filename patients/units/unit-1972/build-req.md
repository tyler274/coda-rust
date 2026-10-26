# Build requirements: unit-1972 (Keith Dupuis, MRN00001972)

Source: synthetic_hospital patient 1972, in `source.md` beside this file. This file and
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

Keith Dupuis, born 1973-08-27, male, White, not Hispanic or Latino, English,
Medicare (disability). Construction worker. Current smoker, 35 pack-years. Alcohol
occasional. No known drug allergies. Surgical history: appendectomy, remote (no date
or details in any record). Family history: father with COPD, mother with
hypertension. Nothing else in the family history.

Joined the practice on 2024-10-29 (A1), a new-patient visit, after care at another
practice. He brought chronic obstructive pulmonary disease (diagnosed by his prior
practice about 2019) and essential hypertension (about 2017), on tiotropium 18 mcg
inhaled daily, albuterol 90 mcg inhaled as needed and amlodipine 5 mg daily, all
prescribed by the prior practice (historical on joining, then refilled here).
Height 178 cm, weight 84 kg at joining, stable through 2025.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-10-29 | practice | new-patient visit (A1): history as above; BP controlled on amlodipine; lungs with mildly reduced air movement, no wheeze; labs CMP, CBC, lipids, A1c, hepatitis C and HIV screening (normal / negative); creatinine 1.0 (his baseline, S); influenza vaccine; records requested from prior practice |
| 2024-11 | practice | prior practice records received: COPD and hypertension history, the three drugs, Tdap 2019, no colorectal screening on file |
| 2024-12 | practice | FIT returned, negative (P) |
| 2025-01-21 | practice | COPD follow-up; office spirometry (A2): post-bronchodilator FEV1/FVC 0.61, FEV1 64% predicted; COPD kept on tiotropium alone; PCV20 given; low-dose CT ordered for lung screening |
| 2025-02-11 | outside imaging | low-dose CT chest for lung cancer screening (A3): no nodules, Lung-RADS 1; annual repeat advised |
| 2025-04-22 | practice | hypertension follow-up; BP controlled on amlodipine 5 mg; Shingrix dose 1 |
| 2025-06-17 | practice | follow-up booked, no-show |
| 2025-07-08 | practice | telehealth check-in: breathing at baseline, refills |
| 2025-10-24 | practice | **last practice visit (last_seen)**: annual preventive visit; BMP, CBC, lipids normal; influenza, updated COVID and Shingrix dose 2; smoking counselled, he declines medication |
| 2026-07-06 | outside ED | 5 days of rising shortness of breath, cough with yellow sputum, right-sided pleuritic chest pain, low-grade fever at home. T 38.3 °C, BP 135/85, HR 102, RR 22, SpO2 92% on room air. Reduced breath sounds and dullness at the right base. Chest x-ray: right pleural effusion. Thoracentesis: pleural fluid pH 7.1, glucose 40 mg/dL, leukocytes 60,000/mm3 with 90% neutrophils, Gram stain gram-positive cocci in clusters. **Complicated parapneumonic effusion** diagnosed; right chest tube placed (A4); ceftriaxone and azithromycin IV started (S), vancomycin added for the Gram stain (A4); admitted |
| 2026-07-06 | outside ICU | same day, worsening hypoxemic respiratory failure: **intubated and mechanically ventilated**; initial settings tidal volume 10 mL/kg predicted body weight, rate 16, PEEP 8; BP 110/70, HR 102, SpO2 88%; bilateral crackles; ABG pH 7.32, PaCO2 50, PaO2 68, HCO3 24; lactate 2.2; PaO2/FiO2 150; chest x-ray diffuse bilateral infiltrates. **Moderate ARDS** diagnosed; **sepsis** |
| 2026-07-08 | outside ICU | ICU day 2 (R1): FiO2 80%, PEEP 12, tidal volume 6 mL/kg ideal body weight; refractory hypoxemia, PaO2/FiO2 85 (R2); urine output 20 mL/hr over 6 hours; weight up 3 kg since admission; BP 95/60 on norepinephrine, HR 110, RR 28 (ventilator), T 38.3 °C; bilateral crackles, 2+ pitting edema of both legs; creatinine 1.8 (baseline 1.0), BUN 36, sodium 145, potassium 4.5, hemoglobin 9.8, WBC 18,000, platelets 250,000; ABG pH 7.32, PaCO2 45, PaO2 68, HCO3 22; chest x-ray diffuse bilateral infiltrates. **Refractory hypoxemia in ARDS.** On ceftriaxone, azithromycin and norepinephrine (S) |
| 2026-07-07 to 07-21 | outside hospital | course (A4), below; discharged home 2026-07-21 |
| 2026-07-22 | practice | ED record, ICU notes, culture and imaging reports and discharge summary received and filed |
| 2026-07-23 | practice | RN post-discharge telephone call (P); post-hospital visit offered, he will call after the pulmonary visit |
| 2026-08-11 | outside | pulmonary clinic follow-up (A5) |
| 2026-08-13 | practice | pulmonary letter received and filed; portal message asking him to book a post-hospital visit, unanswered at today |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** All three source encounters carry the same date, yet the third says he was
  admitted 2 days ago and has gained 3 kg since. Resolution: the ED visit and the
  first ICU note are 2026-07-06; the second ICU note is 2026-07-08.
- **R2.** The source gives PaO2/FiO2 150 at both ICU notes, but the second gives PaO2
  68 on FiO2 80%, which is 85. Resolution: 150 at the first ICU note (moderate ARDS,
  as the source says), 85 on 2026-07-08 (refractory hypoxemia). The second "150" is
  not carried.
- **R3.** Initial tidal volume 10 mL/kg predicted body weight, then 6 mL/kg ideal body
  weight two days later. Resolution: both are carried; the ICU reduces tidal volume to
  6 mL/kg once ARDS is recognized, as lung-protective ventilation. Nobody comments on
  the first setting.
- **R4.** The comorbidity "opportunistic pulmonary infection requiring diagnostic
  sampling" is a label with no organism. Resolution: bronchoscopy with
  bronchoalveolar lavage on 2026-07-07 (A4) sends fungal culture, Pneumocystis PCR and
  AFB smear and culture, all negative; the lavage bacterial culture grows the same
  Staphylococcus aureus. No opportunistic infection is diagnosed; the label is not
  carried as a condition.
- **R5.** The source's creatinine 1.8 from a baseline of 1.0 with low urine output is
  carried as numbers and described in the hospital record as kidney function that
  fell and recovered; no separate kidney diagnosis is listed.
- **R6.** The first ICU note's social history is the single word "Smoking". The
  profile's current smoker, 35 pack-years is carried throughout.
- **R7.** The unfinished ICU plan ("Given the patient's ARDS diagnosis and current
  condition,") is not carried; the hospital course in A4 stands in its place.
- **R8.** The source's problem lists name the effusion, ARDS, sepsis and the
  respiratory infection with diagnosed stamps. Resolution: these exist only in the
  outside hospital and pulmonary records. They come after the last practice visit,
  so the practice's own problem and medication lists stay as they were on
  2025-10-24 (COPD, hypertension; tiotropium, albuterol, amlodipine). The hospital's
  new drugs appear only in its records.
- **Left as the source has it:** COPD stays on tiotropium alone before the admission,
  with no inhaled steroid or long-acting beta-agonist added; nobody remarks on it. He
  is not on a statin and no one raises one. He keeps smoking through 2025; the
  practice counsels him and he declines medication.

## Additions (A): physician to confirm

- **A1.** The 2024-10-29 new-patient visit and the prior practice's records (one year
  with the practice by the last visit; the source starts in an ED, so he needs a
  primary-care home).
- **A2.** One office spirometry, 2025-01-21, to support the listed COPD:
  post-bronchodilator FEV1/FVC 0.61, FEV1 64% predicted (moderate obstruction).
- **A3.** Low-dose CT lung screening, 2025-02-11 (age 51, 35 pack-years, current
  smoker): no nodules, Lung-RADS 1. The annual repeat due in February 2026 is
  reminded once by portal and not done.
- **A4.** Hospital course after the source ends, as the hospital records give it:
  - right chest tube placed 2026-07-06 for the complicated effusion (pH 7.1, glucose
    40), draining purulent fluid;
  - pleural fluid culture: Staphylococcus aureus, methicillin-susceptible; blood
    cultures from 2026-07-06 no growth;
  - bronchoscopy with lavage 2026-07-07 (see R4);
  - vancomycin from 2026-07-06, stopped with ceftriaxone on 2026-07-08 when
    susceptibilities return, cefazolin IV from 2026-07-08; azithromycin stopped
    2026-07-10;
  - prone positioning 2026-07-08 to 07-10; norepinephrine off 2026-07-10;
    diuresis once off pressors;
  - extubated 2026-07-13; chest tube removed 2026-07-16;
  - creatinine back to 1.1 and hemoglobin 10.4 at discharge;
  - discharged home 2026-07-21 on room air, SpO2 93% at rest; discharge drugs:
    cephalexin 500 mg four times daily through 2026-08-05, nicotine patch 21 mg
    daily, tiotropium and albuterol as before; amlodipine held (blood pressure low
    normal in hospital), for primary care to resume;
  - discharge chest x-ray: improving bilateral infiltrates, small residual right
    pleural fluid after tube removal.
- **A5.** Pulmonary clinic visit 2026-08-11: antibiotics nearly done, breathing
  improving, short of breath on stairs; SpO2 94% on room air; BP 128/78 off
  amlodipine; chest x-ray with largely cleared infiltrates and mild residual right
  pleural thickening; down to a few cigarettes a day on the patch. Plan: repeat
  chest x-ray in 6 to 8 weeks, full pulmonary function tests in 3 months, primary
  care to decide on amlodipine.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention
  (heart sounds, abdomen, no lymphadenopathy). An abnormal finding appears only as the
  source or this file gives it. A report may carry the descriptors its kind always
  carries (a chest tube's side and position, a CT's technique and dose), but nothing
  that adds a territory, a severity grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine care there with normal results.
- COPD and blood pressure visits every 3 to 6 months while he is seen; BP about
  130/80 on amlodipine; SpO2 94 to 96% at rest in the office; yearly BMP, CBC, lipids.
- Prevention: FIT yearly (negative), influenza each fall, PCV20 once, Shingrix series,
  COVID updated vaccine in fall 2025; Tdap up to date from 2019; hepatitis C and HIV
  screening once; PHQ-2 or PHQ-9 screens, negative; smoking counselling.
- Refills, pharmacy traffic, portal messages about results, refills and scheduling, the
  missed visit, reminders.
- After 2025-10-24: no practice visit. Refills of the practice's own drugs, the
  post-discharge RN call, filed records and the unanswered scheduling message only.
- Nothing booked after 2026-08-27.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001972 on 2025-10-24. Practice contacts after that day do
not include visits; outside care after it still happens, and its records are filed as
received without changing the practice's lists. Nothing is booked after today.
