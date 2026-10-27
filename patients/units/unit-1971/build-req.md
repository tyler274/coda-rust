# Build requirements: unit-1971 (Steven Wilder, MRN00001971)

Source: synthetic_hospital patient 1971, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or a year its text states without re-dating it.

## Who he is (S)

Born 1973-11-05, male, White, not Hispanic or Latino, English, commercial PPO,
automotive parts manager. Divorced; lives alone (no other person appears in the chart).
Former smoker: quit in 2022 (about 3 years before November 2025), 15 pack-years.
Alcohol: heavy episodic drinking, 5 to 6 drinks on weekends and holidays.
Allergies: sulfonamide antibiotics (rash); latex (contact dermatitis, recorded with
the reaction "Rash").
Surgical history: inguinal hernia repair at 44 (2018), tonsillectomy at 8 (1982).
Family history: father with hypertension and a myocardial infarction at 65; mother
with GERD and type 2 diabetes (78); an older brother with alcohol use disorder.

Joined the practice on 2024-09-10 (A1, a new-patient visit), transferring from a prior
primary care office. Brought: GERD (years, on omeprazole 20 mg daily), essential
hypertension (on amlodipine 5 mg daily), obesity (BMI 31; height 178 cm, weight 98 kg),
and aspirin 81 mg daily that a prior physician started. Former smoker on joining.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-09-10 | practice | new-patient visit (A1): history as above; BP above goal on amlodipine 5 mg (P level, see R4); BMI 31; screening labs (P) |
| 2025-11-20 | practice | routine follow-up for GERD and hypertension (R1); reflux controlled on omeprazole 20 mg daily; **BP 145/90**; ordinary normal exam; BMI 31. No drug change (left as the source has it) |
| 2026-05-12 | outside: ED | sharp chest pain radiating to the back and shortness of breath, sudden, after intense vomiting following a large meal and alcohol. **BP 150/90, HR 110, T 37.8 °C**; in distress; breath sounds diminished on the left; crepitus in the left supraclavicular area. WBC 18.0 (18,000/µL), Hgb 13.5, platelets 250; serum amylase 90 U/L (normal 30-110); arterial gas pH 7.45, PaCO2 32, PaO2 60, bicarbonate 24. Chest X-ray: large left pleural effusion and pneumomediastinum. CT chest with oral contrast (A2). Admitted to thoracic surgery the same day (R2) |
| 2026-05-12 to 2026-05-23 | outside: hospital, thoracic surgery | admission note: in distress, tachypneic and tachycardic; decreased breath sounds in the left lower lung field; subcutaneous emphysema in the neck; **BP 145/90**; WBC 18.0, Hgb 13.5, BUN 21, creatinine 1.0; **pleural fluid amylase 3000 IU/L** (R3); **pleural fluid pH 6.0**; chest X-ray: mediastinal widening and left pleural effusion. **Esophageal rupture with pleural effusion diagnosed.** Course (A3). Discharged home 2026-05-23 |
| 2026-06-09 | outside: thoracic surgery clinic | postoperative visit (A4) |
| 2026-06-18 | practice | post-hospital follow-up, last seen (P, with A5 and A6) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Encounter 0 is labeled a routine follow-up for GERD and hypertension at
  internal medicine, but its history, exam, labs (WBC 18,000, an arterial gas of pH 7.35,
  PaCO2 45, PaO2 60) and chest X-ray describe the acute rupture that the ED and the
  admission describe. Resolution: 2025-11-20 is the routine office follow-up its chief
  complaint states, with BP 145/90 and an ordinary exam. Its acute history, exam,
  arterial gas, blood counts, amylase and chest X-ray are not carried on that date; the
  acute picture belongs to May 2026 as the ED and the admission give it. The
  encounter-0 arterial gas values (7.35/45) are not carried anywhere.
- **R2.** The ED visit and the thoracic surgery admission share one source date and one
  presentation, but sit eleven days apart on the practice calendar; a ruptured
  esophagus cannot wait eleven days. Resolution: he is admitted from the ED on
  2026-05-12, and 2026-05-23 is the discharge date.
- **R3.** The source gives serum amylase 90 at the ED and "serum amylase 3000" on
  admission, with a pleural fluid pH of 6.0. Resolution: 90 U/L is the serum value; 3000
  IU/L is the pleural fluid amylase from the chest tube drainage, beside the pleural
  fluid pH 6.0.
- **R4.** The source has esophageal rupture "diagnosed" in January and on every list.
  Resolution: it is diagnosed on 2026-05-12 and appears on no earlier record.
- **R5.** The source lists the blood pressure at 145/90 and 150/90 with no plan. Before
  the source, his office pressures run about 138 to 146 over 86 to 92 on amlodipine 5 mg;
  the physician advises home readings, less alcohol and weight loss, and does not change
  the drug. The ED and inpatient readings are acute.
- **R6.** The source's two chest X-ray descriptions (large effusion and
  pneumomediastinum in the ED; mediastinal widening and effusion on the admission film)
  stay as two readings of the same day, as written.
- **Left as the source has it:** amlodipine stays 5 mg at 145/90 on 2025-11-20 with no
  comment. Aspirin 81 mg for primary prevention in a 52-year-old is continued and never
  reconsidered. Heavy episodic drinking is noted and counseled but no one treats it as
  a disorder or refers him.

## Additions (A): physician to confirm

- **A1.** The 2024-09-10 new-patient visit (the first source encounter is a follow-up
  and the seed gives about two years with the practice).
- **A2.** ED CT chest with oral contrast, needed to confirm the diagnosis: oral contrast
  extravasating from the distal esophagus into the left pleural space, pneumomediastinum,
  large left pleural effusion. Nothing else abnormal.
- **A3.** Standard inpatient care of the rupture, run by thoracic surgery: a left chest
  tube placed on arrival (the drainage gives the pleural fluid results); left
  thoracotomy the night of 2026-05-12 with primary repair of the distal esophageal tear
  and pleural washout and drainage, done latex-free; NPO with a nasogastric tube; IV
  piperacillin-tazobactam 4.5 g every 8 hours and IV fluconazole 400 mg daily (no sulfa
  drugs); aspirin held and resumed at discharge; contrast esophagram on 2026-05-19 with no
  leak; diet advanced to clear liquids and then soft; chest tubes out before discharge.
  Discharged 2026-05-23 on his home omeprazole 20 mg daily, amlodipine 5 mg daily and
  aspirin 81 mg daily, plus a short course of oxycodone-acetaminophen 5/325 one tablet
  every 6 hours as needed (20 tablets). Advised to avoid alcohol and keep to soft foods.
- **A4.** Thoracic surgery postoperative visit 2026-06-09: incision healing, eating soft
  food without trouble swallowing, off the oxycodone; chest X-ray with the left lung
  expanded and postoperative changes only. Plan: advance diet, no alcohol, clinic return
  in three months.
- **A5.** 2026-06-18: weight 94 kg (down from about 98 after the admission). BP 146/90:
  **amlodipine increased to 10 mg daily** (5 mg stopped), the usual next step after
  readings above goal at every visit. Esophageal rupture added to the practice's
  problem list from the patient's account and the ED record.
- **A6.** AUDIT-C 6 at the 2025 physical and 6 again on 2026-06-18; alcohol counseling at
  both. PHQ-2 0 at each screen.

## Records (chart texture: gaps)

- Prior primary care records arrive 2024-09-26 (P below).
- The ED record arrives by fax 2026-05-14.
- No discharge summary reaches the practice before the June visit; the patient's
  portal message (2026-05-26) and the nurse's call (2026-05-27) are how the practice
  learns of the operation, and the June visit works from the ED record and the
  patient's account. The discharge summary arrives 2026-06-26, after the last visit,
  and is filed without changing the lists.
- The thoracic surgery postoperative letter (2026-06-09) arrives 2026-07-21, filed.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from the prior practice: GERD and hypertension for years, a normal screening
  colonoscopy at 50 (January 2024, repeat in ten years), tetanus booster in 2019.
- Visits for hypertension and GERD about every 6 to 12 months, a yearly physical (April
  2025), BMP or CMP, CBC, lipids and A1c at joining and yearly, all normal.
- Hepatitis C and HIV screening once (2024), negative.
- Influenza and COVID vaccines each fall (2024, 2025); Shingrix two doses (2024-10 and
  2025-02); PCV20 once (2025 physical). No lung screening (15 pack-years).
- Refills, pharmacy traffic, portal messages about results, refills and scheduling, a
  cancelled visit.
- Booked visit after 2026-08-27: blood pressure follow-up on the higher amlodipine dose.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001971 on 2026-06-18. Practice contacts after that day do not
happen; outside care after it still does, and its records are filed as received without
changing the practice's lists. A visit is booked after today.
