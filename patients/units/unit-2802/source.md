# synthetic_hospital patient 2802

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 37 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-05-22 (Mon) | outpatient | Cardiology |
| 1 | 2022-07-15 | 2025-12-01 (Mon) | outpatient | Pulmonology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Penicillin (rash)",
  "Sulfa drugs (hives)"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Social (1-2 glasses of wine per week)",
 "family_history": [
  "Mother: mitral valve prolapse",
  "Paternal aunt: idiopathic pulmonary arterial hypertension",
  "Father: systemic hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 tablet PO daily",
   "name": "Ethinyl estradiol/norethindrone"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Multivitamin"
  }
 ],
 "surgical_history": [
  "None (childhood heart murmur diagnosed, no surgical intervention)"
 ],
 "chronic_conditions": [],
 "age_at_first_encounter": 37
}
```

Primary diagnoses: ["Mitral Valve Prolapse", "Pulmonary Hypertension"]
Comorbidities: ["Left ventricular hypertrophy"]

## Encounter 0: source 2020-01-15, practice 2023-05-22, outpatient, Cardiology

Chief complaint: Progressive dyspnea on exertion and palpitations

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive dyspnea on exertion and palpitations

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with a 6-month history of progressive dyspnea on exertion and palpitations. She denies chest pain, syncope, or leg swelling.

PAST MEDICAL HISTORY:
Her medical history is significant for a "heart murmur" diagnosed during childhood, which was not associated with any intervention.

PAST SURGICAL HISTORY:
- None (childhood heart murmur diagnosed, no surgical intervention)

MEDICATIONS:
Home Medications:
- Ethinyl estradiol/norethindrone 1 tablet PO daily
- Multivitamin 1 tablet PO daily

ALLERGIES:
Penicillin (rash), Sulfa drugs (hives)

FAMILY HISTORY:
- Mother: mitral valve prolapse
- Paternal aunt: idiopathic pulmonary arterial hypertension
- Father: systemic hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Elementary school teacher.

VITAL SIGNS:
On physical examination, her blood pressure is 130/75 mmHg, heart rate is 92 bpm, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a high-pitched, mid-systolic click followed by a late systolic murmur best heard at the apex and radiating to the axilla.

IMAGING:
A transthoracic echocardiogram demonstrates mitral valve leaflets with excessive motion during systole, significant mitral regurgitation, and left atrial enlargement.

OTHER STUDIES:
An electrocardiogram shows atrial fibrillation with a ventricular rate of 92 bpm.

## Encounter 1: source 2022-07-15, practice 2025-12-01, outpatient, Pulmonology

Chief complaint: Worsening dyspnea and fatigue

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Marcus Johnson, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening dyspnea and fatigue

HISTORY OF PRESENT ILLNESS:
The patient returns for further evaluation following her 2020 Cardiology assessment for dyspnea and palpitations. She reports a 6-month history of progressive exertional dyspnea and fatigue.

PAST MEDICAL HISTORY:
Active Problem List:
- Mitral Valve Prolapse (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- None (childhood heart murmur diagnosed, no surgical intervention)

MEDICATIONS:
Home Medications:
- Ethinyl estradiol/norethindrone 1 tablet PO daily
- Multivitamin 1 tablet PO daily

ALLERGIES:
Penicillin (rash), Sulfa drugs (hives)

FAMILY HISTORY:
- Mother: mitral valve prolapse
- Paternal aunt: idiopathic pulmonary arterial hypertension
- Father: systemic hypertension

SOCIAL HISTORY:
She does not smoke

REVIEW OF SYSTEMS:
She denies chest pain, cough, or leg swelling

VITAL SIGNS:
Blood pressure is 125/80 mmHg, heart rate is 88 beats per minute, and oxygen saturation is 95% on room air

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a loud S2 and a systolic murmur along the left sternal border. Lung fields are clear to auscultation.

IMAGING:
A chest x-ray shows enlargement of the pulmonary arteries and rapid tapering of the distal vessels. An echocardiogram demonstrates right ventricular hypertrophy and dilation with normal left ventricular function.
