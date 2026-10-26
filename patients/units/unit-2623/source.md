# synthetic_hospital patient 2623

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 29 at first encounter, sex M, Non-Hispanic White, Employer-sponsored private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-07-01 (Mon) | outpatient | Cardiology |
| 1 | 2020-05-15 | 2024-10-31 (Thu) | outpatient | Internal Medicine |
| 2 | 2021-07-15 | 2026-01-12 (Mon) | outpatient | Oncology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Employer-sponsored private insurance",
 "occupation": "Software Engineer",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Father: Familial Hypercholesterolemia",
  "Paternal grandfather: Myocardial infarction at age 42",
  "Mother: No significant medical history"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
 "home_medications": [],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "Familial Hypercholesterolemia",
  "Congenital bicuspid aortic valve with aortic stenosis",
  "Testicular germ cell tumor"
 ],
 "age_at_first_encounter": 29
}
```

Primary diagnoses: ["Familial Hypercholesterolemia", "Aortic valve stenosis", "Malignancy-associated hypercalcemia"]
Comorbidities: ["Cardiovascular disease risk factors"]

## Encounter 0: source 2020-01-15, practice 2024-07-01, outpatient, Cardiology

Chief complaint: Routine cardiovascular assessment for known bicuspid aortic valve

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine cardiovascular assessment for known bicuspid aortic valve

HISTORY OF PRESENT ILLNESS:
He recalls being told during adolescence that he had a heart murmur but has never experienced chest pain, syncope, or dyspnea on exertion.

PAST MEDICAL HISTORY:
Active Problem List:
- Familial Hypercholesterolemia
- Congenital bicuspid aortic valve with aortic stenosis
- Testicular germ cell tumor

PAST SURGICAL HISTORY:
- No prior surgeries

ALLERGIES:
NKDA

FAMILY HISTORY:
The patient's father had a history of aortic valve replacement in his early 40s.

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per week). Occupation: Software Engineer.

PHYSICAL EXAMINATION:
On auscultation, a harsh crescendo-decrescendo systolic murmur is heard best at the right upper sternal border, radiating to the carotids. The murmur does not increase in intensity during Valsalva maneuver. An S4 heart sound is appreciated, and splitting of S2 is noted to be paradoxical, becoming more pronounced with expiration. There are no stigmata of connective tissue disease on physical exam.

IMAGING:
Echocardiography shows left ventricular hypertrophy but no evidence of asymmetric septal hypertrophy or systolic anterior motion of the mitral valve.

## Encounter 1: source 2020-05-15, practice 2024-10-31, outpatient, Internal Medicine

Chief complaint: Follow-up appointment for familial hypercholesterolemia management

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Marcus Johnson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up appointment for familial hypercholesterolemia management

HISTORY OF PRESENT ILLNESS:
The patient returns for familial hypercholesterolemia management, his first Internal Medicine visit since his January 2020 Cardiology assessment for bicuspid aortic valve surveillance. He has a history of testicular germ cell tumor. He is asymptomatic but is concerned because he reports that

PAST MEDICAL HISTORY:
Active Problem List:
- Aortic valve stenosis (diagnosed 2020-01-15)
- Familial Hypercholesterolemia
- Congenital bicuspid aortic valve with aortic stenosis
- Testicular germ cell tumor

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He has never been on lipid-lowering medication.

ALLERGIES:
NKDA

FAMILY HISTORY:
His father had a heart attack at age 38 and his uncle required coronary bypass surgery before age 40.

SOCIAL HISTORY:
The patient does not smoke,

PHYSICAL EXAMINATION:
On examination, the physician notes several yellow papules on the extensor surfaces of the elbows and along the Achilles tendons.
is not obese,

LABORATORY DATA:
Fasting lipid testing shows a total cholesterol of 340 mg/dL (NR <200), LDL cholesterol of 290 mg/dL, HDL cholesterol of 45 mg/dL (NR >40), and triglycerides of 110 mg/dL (NR <150).

## Encounter 2: source 2021-07-15, practice 2026-01-12, outpatient, Oncology

Chief complaint: Evaluation of persistently abnormal laboratory results noted on routine annual health screening

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Elena Rodriguez, MD
Department: Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of persistently abnormal laboratory results noted on routine annual health screening

HISTORY OF PRESENT ILLNESS:
The patient returns for evaluation of persistently abnormal laboratory results with known history of testicular germ cell tumor, familial hypercholesterolemia, and congenital bicuspid aortic valve with stenosis, previously evaluated in Cardiology and Internal Medicine. He reports generally good health and denies any significant fatigue, weakness, constipation, bone pain, kidney stones, abdominal pain, or psychiatric symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Aortic valve stenosis (diagnosed 2020-01-15)
- Familial Hypercholesterolemia (diagnosed 2020-05-15)
- Congenital bicuspid aortic valve with aortic stenosis
- Testicular germ cell tumor

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
Family history is notable for his father having “elevated calcium levels” for many years without treatment.

SOCIAL HISTORY:
He works as a software engineer and exercises regularly at a local gym. He eats a balanced diet without excessive dairy or supplement use.

VITAL SIGNS:
Temperature is 36.7°C (98.1°F), blood pressure is 124/78 mmHg, pulse is 72/min, and BMI is 23 kg/m²

PHYSICAL EXAMINATION:
The rest of the physical examination is normal.
