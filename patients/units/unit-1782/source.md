# synthetic_hospital patient 1782

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 33 at first encounter, sex F, African American, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-01-22 (Mon) | ed | Obstetrics & Gynecology |
| 1 | 2020-02-15 | 2024-02-25 (Sun) | inpatient | Obstetrics & Gynecology |
| 2 | 2022-01-15 | 2026-01-09 (Fri) | icu | Obstetrics & Gynecology |
| 3 | 2022-02-15 | 2026-02-11 (Wed) | ed | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Administrative assistant",
 "alcohol_use": "None",
 "family_history": [
  "Mother with preeclampsia requiring early delivery",
  "Father with essential hypertension",
  "Maternal grandmother with cerebrovascular accident"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 tablet daily",
   "name": "Prenatal vitamins with DHA"
  },
  {
   "dose": "200 mg twice daily",
   "name": "Labetalol"
  }
 ],
 "surgical_history": [
  "Cesarean section (2019)",
  "Laparoscopic cholecystectomy (2015)"
 ],
 "chronic_conditions": [
  "Chronic hypertension",
  "Class I obesity"
 ],
 "age_at_first_encounter": 33
}
```

Primary diagnoses: ["Severe preeclampsia", "Hypomagnesemia", "Cervical examination contraindication"]
Comorbidities: ["Pregnancy", "Preeclampsia", "Pregnancy at 32 weeks gestation"]

## Encounter 0: source 2020-01-15, practice 2024-01-22, ed, Obstetrics & Gynecology

Chief complaint: Sudden painless vaginal bleeding that began one hour ago during second trimester of pregnancy

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: ed

CHIEF COMPLAINT:
Sudden painless vaginal bleeding that began one hour ago during second trimester of pregnancy

HISTORY OF PRESENT ILLNESS:
She has had an uneventful pregnancy up to this point, with normal fetal movements and no recent trauma, fever, or abdominal pain. She reports no contractions, leakage of fluid, or recent intercourse.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic hypertension
- Class I obesity

PAST SURGICAL HISTORY:
- Cesarean section (2019)
- Laparoscopic cholecystectomy (2015)

MEDICATIONS:
She take any medications other than prenatal vitamins.

Home Medications:
- Prenatal vitamins with DHA 1 tablet daily
- Labetalol 200 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with preeclampsia requiring early delivery
- Father with essential hypertension
- Maternal grandmother with cerebrovascular accident

SOCIAL HISTORY:
She does not smoke, use alcohol

PHYSICAL EXAMINATION:
On examination, she appears anxious but hemodynamically stable. Her abdomen is gravid, soft, and nontender. Fetal heart monitoring shows a baseline rate of 140/min with moderate variability and no decelerations. Speculum examination reveals blood in the vaginal vault, but digital examination is deferred.

ASSESSMENT AND PLAN:
In evaluating this patient’s presentation, review of

Ultrasound is ordered for further evaluation, and the obstetric team is consulted.

## Encounter 1: source 2020-02-15, practice 2024-02-25, inpatient, Obstetrics & Gynecology

Chief complaint: Shortness of breath and palpitations during prolonged antepartum admission

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. Michael Okonkwo, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Shortness of breath and palpitations during prolonged antepartum admission

PAST MEDICAL HISTORY:
Active Problem List:
- Cervical examination contraindication (diagnosed 2020-01-15)
- Chronic hypertension
- Class I obesity

She has a history of chronic hypertension and was diagnosed with preeclampsia 1 week ago.

PAST SURGICAL HISTORY:
- Cesarean section (2019)
- Laparoscopic cholecystectomy (2015)

MEDICATIONS:
She has been receiving magnesium sulfate therapy for seizure prophylaxis.

Home Medications:
- Prenatal vitamins with DHA 1 tablet daily
- Labetalol 200 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with preeclampsia requiring early delivery
- Father with essential hypertension
- Maternal grandmother with cerebrovascular accident

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative assistant.

VITAL SIGNS:
On examination, her blood pressure is 150/100 mmHg, heart rate is 88 beats per minute, and respiratory rate is 22 breaths per minute.

PHYSICAL EXAMINATION:
She appears lethargic. Her deep tendon reflexes are absent, and her respiratory efforts are shallow.

LABORATORY DATA:
An arterial blood gas shows a pH of 7.35, PaCO2 of 45 mmHg, and PaO2 of 95 mmHg. Serum electrolytes reveal the following:

- Sodium: 138 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 100 mEq/L
- Bicarbonate: 24 mEq/L
- Calcium: 8.9 mg/dL
- Magnesium: 5.2 mg/dL (elevated)

## Encounter 2: source 2022-01-15, practice 2026-01-09, icu, Obstetrics & Gynecology

Chief complaint: New-onset right upper quadrant abdominal pain and persistent nausea at 34 weeks gestation in second pregnancy

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Priya Patel, MD
Department: Obstetrics & Gynecology
Visit Type: icu

CHIEF COMPLAINT:
New-onset right upper quadrant abdominal pain and persistent nausea at 34 weeks gestation in second pregnancy

HISTORY OF PRESENT ILLNESS:
She returns after 2020 admissions with chronic hypertension. Denies history of hypertension but notes recent fatigue and decreased appetite.

PAST MEDICAL HISTORY:
Active Problem List:
- Cervical examination contraindication (diagnosed 2020-01-15)
- Hypomagnesemia (diagnosed 2020-02-15)
- Chronic hypertension
- Class I obesity

PAST SURGICAL HISTORY:
- Cesarean section (2019)
- Laparoscopic cholecystectomy (2015)

MEDICATIONS:
Home Medications:
- Prenatal vitamins with DHA 1 tablet daily
- Labetalol 200 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with preeclampsia requiring early delivery
- Father with essential hypertension
- Maternal grandmother with cerebrovascular accident

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative assistant.

VITAL SIGNS:
On arrival, her blood pressure is 148/96 mm Hg

PHYSICAL EXAMINATION:
Physical exam shows mild scleral icterus and tenderness to palpation in the right upper quadrant. Reflexes are brisk but symmetrical.

LABORATORY DATA:
She has 2+ protein on urinalysis. Laboratory testing shows:
Platelets                       74,000/µL
AST                              290 U/L
ALT                               235 U/L
Total bilirubin                3.1 mg/dL
LDH                              680 U/L
Haptoglobin                  Undetectable

OTHER STUDIES:
Fetal heart rate tracing is normal.

## Encounter 3: source 2022-02-15, practice 2026-02-11, ed, Obstetrics & Gynecology

Chief complaint: Difficulty breathing and generalized weakness one week postpartum

ENCOUNTER NOTE
Date: 2022-02-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: ed

CHIEF COMPLAINT:
Difficulty breathing and generalized weakness one week postpartum

PAST MEDICAL HISTORY:
Active Problem List:
- Cervical examination contraindication (diagnosed 2020-01-15)
- Hypomagnesemia (diagnosed 2020-02-15)
- Severe preeclampsia (diagnosed 2022-01-15)
- Chronic hypertension
- Class I obesity

PAST SURGICAL HISTORY:
- Cesarean section (2019)
- Laparoscopic cholecystectomy (2015)

MEDICATIONS:
She takes receiving magnesium sulfate therapy.

Home Medications:
- Prenatal vitamins with DHA 1 tablet daily
- Labetalol 200 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with preeclampsia requiring early delivery
- Father with essential hypertension
- Maternal grandmother with cerebrovascular accident

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative assistant.

VITAL SIGNS:
Her blood pressure is 150/100 mmHg, heart rate is 88 beats per minute, and respiratory rate is 12 breaths per minute

PHYSICAL EXAMINATION:
She appears lethargic. Her patellar reflexes are absent, and auscultation of the lungs reveals decreased breath sounds bilaterally without wheezes or crackles.

LABORATORY DATA:
An arterial blood gas shows a pH of 7.35, PaCO2 of 50 mmHg, and PaO2 of 70 mmHg. Serum electrolytes are as follows:

- Sodium: 138 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 100 mEq/L
- Bicarbonate: 24 mEq/L
- Calcium: 8.9 mg/dL
- Magnesium: 5.2 mEq/L (elevated)
