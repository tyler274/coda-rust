# synthetic_hospital patient 1720

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Non-Hispanic Black, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-08-06 (Wed) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2026-02-18 (Wed) | ed | Emergency Medicine |
| 2 | 2020-07-15 | 2026-02-18 (Wed) | inpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Administrative Coordinator",
 "alcohol_use": "None (abstains due to pregnancy)",
 "family_history": [
  "Mother: Preeclampsia during pregnancy",
  "Father: Essential hypertension",
  "Maternal aunt: Gestational hypertension"
 ],
 "race_ethnicity": "Non-Hispanic Black",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "200 mg PO twice daily",
   "name": "Labetalol"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Prenatal vitamin"
  }
 ],
 "surgical_history": [
  "Cesarean section (age 29)"
 ],
 "chronic_conditions": [
  "Chronic hypertension",
  "History of gestational hypertension"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Magnesium sulfate toxicity", "Severe preeclampsia requiring immediate delivery", "Eclampsia"]
Comorbidities: ["Essential hypertension", "Severe preeclampsia", "Pregnancy (34 weeks gestation)", "Acute kidney injury"]

## Encounter 0: source 2020-01-15, practice 2025-08-06, ed, Emergency Medicine

Chief complaint: Severe right upper quadrant pain, nausea, and vomiting

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe right upper quadrant pain, nausea, and vomiting

HISTORY OF PRESENT ILLNESS:
She presents to the emergency department with a 24-hour history of severe right upper quadrant pain, nausea, and vomiting. She reports that the pain is constant and radiates to her back.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic hypertension
- History of gestational hypertension

She has a history of chronic hypertension, for which she takes labetalol, and her prenatal course has been complicated by mild preeclampsia diagnosed 2 weeks ago.

PAST SURGICAL HISTORY:
- Cesarean section (age 29)

MEDICATIONS:
She takes labetalol.

Home Medications:
- Prenatal vitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Preeclampsia during pregnancy
- Father: Essential hypertension
- Maternal aunt: Gestational hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (abstains due to pregnancy). Occupation: Administrative Coordinator.

VITAL SIGNS:
Her blood pressure is 160/100 mmHg, heart rate is 92 beats per minute, and temperature is 37.2°C (99.0°F).

PHYSICAL EXAMINATION:
Physical examination reveals right upper quadrant tenderness without rebound or guarding. Reflexes are brisk and clonus is present.

LABORATORY DATA:
Laboratory studies are as follows:

- Hemoglobin: 9.8 g/dL (Normal: 12-16 g/dL)
- Platelet count: 55,000/mm³ (Normal: 150,000-400,000/mm³)
- Aspartate aminotransferase (AST): 210 U/L (Normal: 10-40 U/L)
- Alanine aminotransferase (ALT): 190 U/L (Normal: 7-56 U/L)
- Lactate dehydrogenase (LDH): 600 U/L (Normal: 140-280 U/L)
- Bilirubin, total: 2.2 mg/dL (Normal: 0.1-1.2 mg/dL)
- Peripheral blood smear: schistocytes present

## Encounter 1: source 2020-07-15, practice 2026-02-18, ed, Emergency Medicine

Chief complaint: Sudden onset of tonic-clonic seizures

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of tonic-clonic seizures

HISTORY OF PRESENT ILLNESS:
The patient is a 32-year-old female with chronic hypertension and gestational hypertension history who required delivery five months ago. She presents to the emergency department with a sudden onset of tonic-clonic seizures. Her husband reports that she had been complaining of a severe headache and blurred vision for the past two days.

PAST MEDICAL HISTORY:
Active Problem List:
- Severe preeclampsia requiring immediate delivery (diagnosed 2020-01-15)
- Chronic hypertension
- History of gestational hypertension

Her past medical history is significant for gestational hypertension in her previous pregnancy.

PAST SURGICAL HISTORY:
- Cesarean section (age 29)

MEDICATIONS:
Home Medications:
- Labetalol 200 mg PO twice daily
- Prenatal vitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Preeclampsia during pregnancy
- Father: Essential hypertension
- Maternal aunt: Gestational hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (abstains due to pregnancy). Occupation: Administrative Coordinator.

VITAL SIGNS:
On examination, her blood pressure is 160/110 mmHg

PHYSICAL EXAMINATION:
She has 3+ pitting edema in her lower extremities. Reflexes are brisk, and there is clonus at the ankles.

LABORATORY DATA:
Laboratory values are as follows:

- Platelet count: 110,000/µL
- Serum creatinine: 1.2 mg/dL
- AST: 70 U/L
- ALT: 50 U/L
- Urinalysis: 3+ proteinuria

## Encounter 2: source 2020-07-15, practice 2026-02-18, inpatient, Obstetrics & Gynecology

Chief complaint: Severe headache and visual disturbances during magnesium therapy

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Aisha Johnson, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Severe headache and visual disturbances during magnesium therapy

HISTORY OF PRESENT ILLNESS:
This 32-year-old female with chronic hypertension and gestational hypertension presented earlier today with eclampsia following a January admission for severe preeclampsia. She presents to the emergency department with a severe headache and visual disturbances.

PAST MEDICAL HISTORY:
Active Problem List:
- Severe preeclampsia requiring immediate delivery (diagnosed 2020-01-15)
- Eclampsia (diagnosed 2020-07-15)
- Chronic hypertension
- History of gestational hypertension

She has a history of chronic hypertension, which has been poorly controlled during pregnancy.

PAST SURGICAL HISTORY:
- Cesarean section (age 29)

MEDICATIONS:
The patient is currently receiving intravenous magnesium sulfate for seizure prophylaxis.

Home Medications:
- Labetalol 200 mg PO twice daily
- Prenatal vitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Preeclampsia during pregnancy
- Father: Essential hypertension
- Maternal aunt: Gestational hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (abstains due to pregnancy). Occupation: Administrative Coordinator.

VITAL SIGNS:
Her blood pressure is 160/110 mmHg.

PHYSICAL EXAMINATION:
Deep tendon reflexes are 1+ and she appears drowsy.

LABORATORY DATA:
Laboratory studies are as follows:

- Serum creatinine: 2.2 mg/dL (increased)
- Blood urea nitrogen (BUN): 35 mg/dL (increased)
- Serum potassium: 4.5 mEq/L (normal)
- Serum magnesium: 3.5 mg/dL (increased)
- Urinalysis: Proteinuria 2+
