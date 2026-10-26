# synthetic_hospital patient 1710

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield (PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-04-04 (Fri) | ed | Emergency Medicine |
| 1 | 2020-03-15 | 2025-06-04 (Wed) | inpatient | Obstetrics & Gynecology |
| 2 | 2020-09-15 | 2025-12-08 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield (PPO)",
 "occupation": "High school mathematics teacher",
 "alcohol_use": "None (previous social use, abstinent during reproductive years)",
 "family_history": [
  "Mother: Hypertension, preeclampsia with prior pregnancies",
  "Father: Type 2 Diabetes Mellitus, hyperlipidemia",
  "Maternal aunt: Systemic lupus erythematosus",
  "No family history of early cardiovascular death"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "22 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "1 unit per 10g carbohydrates with meals, correction scale 1:50 for BG >150 mg/dL",
   "name": "Insulin aspart"
  },
  {
   "dose": "200 mg orally twice daily",
   "name": "Labetalol"
  }
 ],
 "surgical_history": [
  "Emergency cesarean section (low transverse incision)",
  "Tonsillectomy (age 8)"
 ],
 "chronic_conditions": [
  "Type 1 Diabetes Mellitus (onset age 22)",
  "Chronic Hypertension (essential, diagnosed age 27)"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Hypertensive emergency in pregnancy", "Severe preeclampsia requiring immediate delivery", "Severe hypoglycemia"]
Comorbidities: ["Anemia in pregnancy", "Thrombocytopenia", "Type 1 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2025-04-04, ed, Emergency Medicine

Chief complaint: Severe headache and blurred vision at 28 weeks gestation

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe headache and blurred vision at 28 weeks gestation

HISTORY OF PRESENT ILLNESS:
She presents to the emergency department with a severe headache and blurred vision. Her prenatal course has been otherwise unremarkable.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 1 Diabetes Mellitus (onset age 22)
- Chronic Hypertension (essential, diagnosed age 27)

PAST SURGICAL HISTORY:
- Emergency cesarean section (low transverse incision)
- Tonsillectomy (age 8)

MEDICATIONS:
She takes labetalol throughout her pregnancy.

Home Medications:
- Insulin glargine 22 units subcutaneous at bedtime
- Insulin aspart 1 unit per 10g carbohydrates with meals, correction scale 1:50 for BG >150 mg/dL

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypertension, preeclampsia with prior pregnancies
- Father: Type 2 Diabetes Mellitus, hyperlipidemia
- Maternal aunt: Systemic lupus erythematosus
- No family history of early cardiovascular death

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (previous social use, abstinent during reproductive years). Occupation: High school mathematics teacher.

VITAL SIGNS:
Blood pressure is 172/116 mm Hg, heart rate is 88 beats per minute, and respiratory rate is 20 beats per minute

PHYSICAL EXAMINATION:
Fundoscopic examination reveals arteriolar narrowing and cotton-wool spots. Deep tendon reflexes are brisk, and there is mild clonus at the ankles.

LABORATORY DATA:
- Hemoglobin: 10.2 g/dL
- Platelet count: 95,000/mm^3
- Serum creatinine: 1.2 mg/dL
- AST: 70 U/L
- ALT: 65 U/L
- Urinalysis: 2+ proteinuria

## Encounter 1: source 2020-03-15, practice 2025-06-04, inpatient, Obstetrics & Gynecology

Chief complaint: Persistent right upper quadrant pain with headache and blurred vision at 36 weeks gestation

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Maria Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Persistent right upper quadrant pain with headache and blurred vision at 36 weeks gestation

HISTORY OF PRESENT ILLNESS:
This 32-year-old female with T1DM and chronic HTN returns eight weeks after a prior ED visit for hypertensive emergency. She presents to the emergency department with a 24-hour history of persistent right upper quadrant pain. She describes the pain as a constant, dull ache that occasionally becomes sharp. She also reports experiencing headaches and blurred vision for the past two days.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertensive emergency in pregnancy (diagnosed 2020-01-15)
- Type 1 Diabetes Mellitus (onset age 22)
- Chronic Hypertension (essential, diagnosed age 27)

Her past medical history is significant for gestational hypertension diagnosed at 28 weeks.

PAST SURGICAL HISTORY:
- Emergency cesarean section (low transverse incision)
- Tonsillectomy (age 8)

MEDICATIONS:
Home Medications:
- Insulin glargine 22 units subcutaneous at bedtime
- Insulin aspart 1 unit per 10g carbohydrates with meals, correction scale 1:50 for BG >150 mg/dL
- Labetalol 200 mg orally twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypertension, preeclampsia with prior pregnancies
- Father: Type 2 Diabetes Mellitus, hyperlipidemia
- Maternal aunt: Systemic lupus erythematosus
- No family history of early cardiovascular death

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (previous social use, abstinent during reproductive years). Occupation: High school mathematics teacher.

VITAL SIGNS:
On examination, her blood pressure is 165/105 mmHg, heart rate is 88 beats per minute, and temperature is 37.2°C (99.0°F).

PHYSICAL EXAMINATION:
Abdominal examination reveals tenderness in the right upper quadrant without rebound or guarding.

LABORATORY DATA:
Laboratory studies are as follows:

- Hemoglobin: 9.8 g/dL (Normal: 12-16 g/dL)
- Platelet count: 55,000/mm³ (Normal: 150,000-400,000/mm³)
- AST: 72 U/L (Normal: 10-40 U/L)
- ALT: 68 U/L (Normal: 7-56 U/L)
- LDH: 600 U/L (Normal: 140-280 U/L)
- Bilirubin, total: 2.1 mg/dL (Normal: 0.1-1.2 mg/dL)
- Peripheral blood smear: schistocytes present

Urinalysis shows 2+ proteinuria.

OTHER STUDIES:
The fetal heart rate tracing is reassuring.

## Encounter 2: source 2020-09-15, practice 2025-12-08, ed, Emergency Medicine

Chief complaint: Confusion and agitation

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. James Okonkwo, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Confusion and agitation

HISTORY OF PRESENT ILLNESS:
This 32-year-old female with Type 1 Diabetes Mellitus and history of hypertensive emergency and severe preeclampsia during her recent pregnancy presents with confusion and agitation. Her husband reports that she has been trying to lose weight and recently started a low-carbohydrate diet. She has also been exercising more frequently. Over the past week, she had several episodes of sweating and palpitations, which she attributed to her new exercise regimen.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertensive emergency in pregnancy (diagnosed 2020-01-15)
- Severe preeclampsia requiring immediate delivery (diagnosed 2020-03-15)
- Type 1 Diabetes Mellitus (onset age 22)
- Chronic Hypertension (essential, diagnosed age 27)

PAST SURGICAL HISTORY:
- Emergency cesarean section (low transverse incision)
- Tonsillectomy (age 8)

MEDICATIONS:
Her home medications include insulin glargine and insulin aspart.

Home Medications:
- Labetalol 200 mg orally twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypertension, preeclampsia with prior pregnancies
- Father: Type 2 Diabetes Mellitus, hyperlipidemia
- Maternal aunt: Systemic lupus erythematosus
- No family history of early cardiovascular death

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (previous social use, abstinent during reproductive years). Occupation: High school mathematics teacher.

VITAL SIGNS:
Her blood glucose level is measured at 45 mg/dL. Her other vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, she is diaphoretic and tremulous.

LABORATORY DATA:
Laboratory values:
- Hemoglobin A1c: 6.2%
- Serum creatinine: 0.9 mg/dL
- Serum potassium: 4.2 mEq/L
- Serum sodium: 140 mEq/L
