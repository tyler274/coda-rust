# Build requirements: unit-2529 (August Mulcahy, MRN00002529)

Source: synthetic_hospital patient 2529, in `source.md` beside this file. This file and
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

August Mulcahy, born 1996-11-17, male, White, not Hispanic or Latino, English,
employer-sponsored private PPO, lives in Easthampton MA 01027, single. Software
developer and endurance athlete (distance running races). Never smoker. Alcohol:
social, 1 to 2 drinks a month. No known drug allergies. No surgical history. Family
history: father hypertension, mother hypothyroidism, paternal uncle type 2 diabetes.
No chronic conditions and no medications.

Joined the practice 2023-09-13 (A1), healthy, bringing no problems and no drugs.
Height 180 cm, weight about 71 kg throughout (lean runner), BP and labs normal at
baseline. Records: complete; every outside ED and hospital record reaches the
practice within days and the lists are kept current.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-09-13 | practice | new-patient preventive visit (A1): history as above; normal exam; baseline CMP, CBC, lipid panel, HIV and hepatitis C screening, all normal (P); Tdap given (P) |
| 2024-08-21 | outside | regional hospital ED (age 27): decreased urine output over the past 24 hours after a long-distance endurance race on a hot summer day; weak and lightheaded afterward; no past history, no medications, NKDA. BP 92/58, pulse 116, temperature 38.0 °C. Fatigued; skin warm and dry; mucous membranes parched. Creatinine and potassium elevated (values in A2); urinalysis dark urine, dipstick positive for blood, microscopy few or no red cells; CK markedly elevated (A2). **Exertional rhabdomyolysis, acute kidney injury and dehydration diagnosed.** Aggressive IV fluid resuscitation; admitted to medicine (A2) |
| 2024-08-23 | outside | discharged after 2 days of IV fluids (A2); discharge summary received by the practice the same day |
| 2024-08-23 | practice | RN post-discharge call (P) |
| 2024-08-28 | practice | post-hospital follow-up (P): feels well, urine normal color, not yet running; CK and BMP drawn (A3) |
| 2024-09-11 | practice | lab recheck (A3): CK and creatinine normal. **Rhabdomyolysis, acute kidney injury and dehydration resolved.** Graded return to running, hydration counselling (P) |
| 2025-02-27 | practice | same-week visit (age 28, R2): 2 days back from a week's vacation in southern Mexico; abrupt watery diarrhea from his last day there, worsening; non-bloody, very watery, high volume, up to about a dozen stools a day; mild nausea, leg cramps; no fever, vomiting or abdominal pain; no recent antibiotics, nothing taken for it; had no pre-travel vaccines; ate widely at food stalls, drank tap water when bottled was not available. History of the August 2024 exertional rhabdomyolysis (R1). Vitals normal except heart rate 102. Tired; mucous membranes slightly dry; hyperactive bowel sounds, abdomen nontender. Sodium 133, potassium 3.1, bicarbonate 18, creatinine 1.3; stool microscopy no fecal leukocytes and no blood. **Cholera diagnosed presumptively** (stool culture sent, A4). **Doxycycline 300 mg once, oral rehydration solution, and potassium chloride 20 mEq twice daily for 3 days started** (A4, R3) |
| 2025-02-28 | practice | RN phone check (P): fewer stools, drinking well |
| 2025-03-03 | practice | stool culture result (A4): Vibrio cholerae O1 isolated; result reported to the state health department as a reportable infection (A4) |
| 2025-03-04 | practice | recheck BMP (A4): normal; diarrhea resolved. **Cholera resolved** |
| 2026-02-21 | outside | regional hospital ED (age 29, R2), Saturday: back from a 2-week trip volunteering with a disaster relief organization in a flood-affected rural coastal village in Bangladesh; more than 15 thin, colorless, odorless "rice-water" stools; no abdominal pain, fever or blood; lightheaded on standing; minimal urine output; ate street food and drank untreated water. History of cholera after the Mexico trip in February 2025 and exertional rhabdomyolysis in August 2024 (R1). Skin tented, mucous membranes dry, tachycardic and hypotensive (vitals in A5). **Cholera diagnosed again; dehydration and acute kidney injury diagnosed** (labs in A5). Immediate IV fluids and electrolytes; azithromycin 1 g once (A5); stool culture sent (A5); kept overnight in ED observation, discharged 2026-02-22 (A5). ED record received 2026-02-23 |
| 2026-02-23 | practice | RN post-ED call (P) |
| 2026-02-25 | practice | post-ED follow-up (P): stools formed, eating and drinking normally; BMP normal (A5). Outside stool culture (Vibrio cholerae O1) noted (A5). **Cholera, dehydration and acute kidney injury resolved** |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source's years follow its own dates (rhabdomyolysis "in January 2020",
  cholera "in July 2020"). Resolution: the chart uses the re-dated timeline, so the
  race is August 2024 (the source's "hot summer day" now fits) and the Mexico cholera
  is February 2025.
- **R2.** The source gives ages 27 and 28 at the second and third encounters. With
  birth date 1996-11-17 he is 27 in August 2024, 28 in February 2025 and 29 in
  February 2026. The chart uses those ages.
- **R3.** The source stops at "A presumptive diagnosis is made" and "Treatment is
  initiated" without naming either. Resolution: the diagnosis is cholera (the third
  encounter's history says so), and the treatment is oral rehydration with a single
  dose of doxycycline 300 mg and a short potassium course (A4).
- The source's problem-list dumps are not carried; each condition is recorded at the
  encounter that documents it and resolved when the patient recovers.
- **Left as the source has it:** he had no pre-travel vaccines before Mexico, and the
  chart shows no pre-travel visit, no cholera, typhoid or hepatitis A vaccine and no
  travel advice before the Bangladesh trip. Nobody comments on it.

## Additions (A): physician to confirm

- **A1.** New-patient visit 2023-09-13 (the source gives no practice start; the seed
  gives 3 years with the practice).
- **A2.** The source gives "elevated" with no numbers and no CK. ED 2024-08-21:
  creatinine 2.4 mg/dL, potassium 5.8 mmol/L, BUN 34 mg/dL, CK 38,600 U/L. Admitted to
  medicine for IV fluids (rhabdomyolysis with AKI and hyperkalemia); discharged
  2024-08-23 with CK 4,900 U/L, creatinine 1.2 mg/dL, potassium 4.3 mmol/L.
- **A3.** Recovery labs: 2024-08-28 CK 620 U/L, creatinine 1.0 mg/dL; 2024-09-11 CK
  210 U/L, creatinine 0.96 mg/dL.
- **A4.** Cholera 2025: doxycycline 300 mg single dose (three 100 mg capsules),
  oral rehydration solution, potassium chloride 20 mEq ER twice daily for 3 days for
  the potassium of 3.1. Stool culture (sent 2025-02-27, resulted 2025-03-03): Vibrio
  cholerae O1 isolated; reported to the state health department. Recheck 2025-03-04:
  sodium 139, potassium 4.0, bicarbonate 24, creatinine 0.98.
- **A5.** ED 2026-02-21: BP 86/52, pulse 128, temperature 37.1 °C, respiratory rate
  20, SpO2 98%; sodium 134, potassium 2.9, bicarbonate 15, BUN 38, creatinine 1.9.
  IV balanced crystalloid boluses with IV and oral potassium; azithromycin 1 g once;
  stool culture Vibrio cholerae O1. Discharged 2026-02-22 with creatinine 1.1,
  potassium 3.6, BP 112/70, pulse 84. Practice BMP 2026-02-25: sodium 140, potassium
  4.1, bicarbonate 23, creatinine 1.0.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from a prior practice may mention routine childhood vaccines and normal
  screening done there.
- Preventive visits about yearly (2023 new patient, late 2024, 2025), with vitals,
  PHQ-2, alcohol and tobacco history; one lipid panel and one HIV and hepatitis C
  screen (2023), all normal.
- Influenza vaccine each fall; COVID vaccine each fall; Tdap once (2023).
- Post-discharge calls and follow-up visits after each outside event; result notes
  and portal messages about results, scheduling, forms and immunization records; a
  cancelled or missed visit.
- Booked visit after 2026-08-27: a routine preventive visit.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00002529 on 2026-02-25. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. Nothing is booked after today.
