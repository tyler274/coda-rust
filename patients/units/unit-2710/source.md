# synthetic_hospital patient 2710

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 52 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-06-11 (Tue) | ed | Emergency Medicine |
| 1 | 2020-09-15 | 2025-02-20 (Thu) | icu | Pulmonary & Critical Care |
| 2 | 2021-07-15 | 2025-12-05 (Fri) | icu | Pulmonary & Critical Care |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "International Sales Manager",
 "alcohol_use": "Occasional social use (1-2 glasses wine per week)",
 "family_history": [
  "Mother: pulmonary embolism at age 68",
  "Father: hypertension, hyperlipidemia",
  "Sister: deep vein thrombosis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  }
 ],
 "surgical_history": [
  "Laparoscopic cholecystectomy (2015)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "History of deep vein thrombosis (remote)"
 ],
 "age_at_first_encounter": 52
}
```

Primary diagnoses: ["Massive pulmonary embolism", "Pulmonary embolism"]
Comorbidities: ["Essential hypertension", "Delayed management of life-threatening respiratory failure"]

## Encounter 0: source 2020-01-15, practice 2024-06-11, ed, Emergency Medicine

Chief complaint: Sudden onset of shortness of breath and pleuritic chest pain

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of shortness of breath and pleuritic chest pain

HISTORY OF PRESENT ILLNESS:
Sudden onset of shortness of breath and pleuritic chest pain that started 2 hours ago. She recently returned from a 12-hour flight 3 days ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- History of deep vein thrombosis (remote)

Her past medical history includes hypertension.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (2015)

MEDICATIONS:
Hydrochlorothiazide

ALLERGIES:
NKDA

FAMILY HISTORY:
No family history of clotting disorders

SOCIAL HISTORY:
She does not smoke

VITAL SIGNS:
Her blood pressure is 135/85 mmHg, heart rate is 110 bpm, respiratory rate is 22 breaths/min, and oxygen saturation is 94% on room air

PHYSICAL EXAMINATION:
Her lungs are clear to auscultation, and there is no leg swelling or tenderness

LABORATORY DATA:
A D-dimer test is ordered, and the result is within normal limits

IMAGING:
Chest radiography is unremarkable

OTHER STUDIES:
An electrocardiogram shows sinus tachycardia with no ST changes

## Encounter 1: source 2020-09-15, practice 2025-02-20, icu, Pulmonary & Critical Care

Chief complaint: Sudden-onset shortness of breath and sharp, pleuritic chest pain

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Marcus Johnson, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Sudden-onset shortness of breath and sharp, pleuritic chest pain

HISTORY OF PRESENT ILLNESS:
This 52-year-old female with hypertension and remote DVT returns with recurrent symptoms following her prior pulmonary embolism. She presents to the emergency department with sudden-onset shortness of breath and sharp, pleuritic chest pain that started 2 hours ago. She reports a recent history of a long car trip lasting over 8 hours without breaks.

PAST MEDICAL HISTORY:
Active Problem List:
- Pulmonary embolism (diagnosed 2020-01-15)
- Hypertension
- History of deep vein thrombosis (remote)

Her past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (2015)

MEDICATIONS:
For which she takes hydrochlorothiazide.

ALLERGIES:
NKDA

FAMILY HISTORY:
She has no family history of cardiac or thromboembolic disease.

SOCIAL HISTORY:
She does not smoke

VITAL SIGNS:
On examination, her blood pressure is 135/85 mmHg, heart rate is 110 bpm, respiratory rate is 22 breaths/min, and oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
Her jugular venous pressure is not elevated. Lung auscultation reveals no wheezes, crackles, or rhonchi. A lower extremity examination shows no signs of deep vein thrombosis.

LABORATORY DATA:
D-dimer level is elevated.

Laboratory Values:
- D-dimer: 0.8 mg/L (normal <0.5 mg/L)

IMAGING:
A chest X-ray is clear with no signs of consolidation, but a small pleural effusion is noted on the right side.

OTHER STUDIES:
An electrocardiogram shows sinus tachycardia with no ST changes.

## Encounter 2: source 2021-07-15, practice 2025-12-05, icu, Pulmonary & Critical Care

Chief complaint: Sudden onset of shortness of breath and left-sided chest pain that worsens with deep breaths

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Priya Patel, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Sudden onset of shortness of breath and left-sided chest pain that worsens with deep breaths

HISTORY OF PRESENT ILLNESS:
The patient is a 53-year-old female with hypertension and remote deep vein thrombosis who presents with sudden onset shortness of breath and left-sided chest pain similar to her prior episodes in January and September 2020. She returned from a transatlantic flight 2 days ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Pulmonary embolism (diagnosed 2020-01-15)
- Massive pulmonary embolism (diagnosed 2020-09-15)
- Hypertension
- History of deep vein thrombosis (remote)

She has a past medical history of hypertension and a remote history of deep vein thrombosis.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (2015)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: pulmonary embolism at age 68
- Father: hypertension, hyperlipidemia
- Sister: deep vein thrombosis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 glasses wine per week). Occupation: International Sales Manager.

VITAL SIGNS:
Her blood pressure is 135/85 mmHg. Oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
On examination, she is anxious, tachypneic with a respiratory rate of 24 breaths/min, and tachycardic with a heart rate of 110 beats/min. Lung auscultation reveals decreased breath sounds at the left lung base with dullness to percussion.

LABORATORY DATA:
A D-dimer test returns elevated. Laboratory Values:
- D-dimer: Elevated
- Troponin: Normal
- B-type natriuretic peptide (BNP): Normal
- Arterial blood gas: Hypoxemia

IMAGING:
A CT pulmonary angiography is performed and shows a filling defect in a peripheral branch of the left pulmonary artery.
