# synthetic_hospital patient 2527

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex M, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-03-03 (Mon) | ed | Emergency Medicine |
| 1 | 2020-09-15 | 2025-11-02 (Sun) | ed | Emergency Medicine |
| 2 | 2021-04-15 | 2026-05-25 (Mon) | icu | Trauma Surgery |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Field Sales Representative",
 "alcohol_use": "Occasional social use (1-2 drinks per week)",
 "family_history": [
  "Maternal asthma",
  "Paternal hypertension",
  "Maternal grandfather with coronary artery disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "90 mcg (2 puffs) inhaled every 4-6 hours as needed for wheezing or shortness of breath",
   "name": "Albuterol HFA inhaler"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Asthma, intermittent"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Thermal burn involving 30% body surface area with multiple traumatic injuries and contraindication to succinylcholine", "Tracheobronchial injury", "Blunt thoracic aortic injury"]
Comorbidities: ["Sinus tachycardia secondary to burns", "Hypoxemia"]

## Encounter 0: source 2020-01-15, practice 2025-03-03, ed, Emergency Medicine

Chief complaint: High-speed motor vehicle collision

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
High-speed motor vehicle collision

HISTORY OF PRESENT ILLNESS:
A 28-year-old male presents to the emergency department following a high-speed motor vehicle collision. He was the restrained driver, and airbags were deployed. On arrival, he is alert but appears anxious and dyspneic. A chest tube is placed, resulting in immediate improvement in oxygen saturation to 98% on room air. However, the chest tube continues to have a persistent large air leak after 24 hours.

PAST MEDICAL HISTORY:
Active Problem List:
- Asthma, intermittent

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal asthma
- Paternal hypertension
- Maternal grandfather with coronary artery disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Field Sales Representative.

VITAL SIGNS:
Blood pressure 110/70 mmHg, heart rate 122 beats/min, respiratory rate 28 breaths/min, and oxygen saturation 92% on room air

PHYSICAL EXAMINATION:
Physical examination reveals subcutaneous emphysema in the neck and anterior chest wall. Auscultation of the chest shows decreased breath sounds on the left side.

IMAGING:
A chest x-ray demonstrates a moderate left pneumothorax and extensive subcutaneous air in the chest wall.

## Encounter 1: source 2020-09-15, practice 2025-11-02, ed, Emergency Medicine

Chief complaint: Chest pain that worsens with deep breaths after high-speed motor vehicle collision

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Marcus Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Chest pain that worsens with deep breaths after high-speed motor vehicle collision

HISTORY OF PRESENT ILLNESS:
The patient is a 28-year-old male with intermittent asthma who presented to the ED in January 2020 following a high-speed motor vehicle collision, and who returns following another high-speed motor vehicle collision. He was the restrained driver, and the airbags were deployed. On arrival, he is alert but appears anxious and complains of chest pain that worsens with deep breaths.

PAST MEDICAL HISTORY:
Active Problem List:
- Tracheobronchial injury (diagnosed 2020-01-15)
- Asthma, intermittent

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal asthma
- Paternal hypertension
- Maternal grandfather with coronary artery disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Field Sales Representative.

VITAL SIGNS:
His vital signs are as follows: blood pressure 110/70 mmHg, heart rate 105 beats per minute, respiratory rate 22 breaths per minute, and oxygen saturation 94% on room air.

PHYSICAL EXAMINATION:
Physical examination reveals multiple abrasions over his chest and abdomen. His breath sounds are decreased on the left side.

IMAGING:
A chest x-ray is performed and shows a widened mediastinum, a small left-sided pleural effusion, and an abnormal aortic contour. No rib fractures are noted.

OTHER STUDIES:
The patient's ECG shows sinus tachycardia without ST changes.

## Encounter 2: source 2021-04-15, practice 2026-05-25, icu, Trauma Surgery

Chief complaint: Thermal burns covering 30% body surface area and multiple traumatic injuries following high-speed motor vehicle collision

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. Aisha Patel, MD
Department: Trauma Surgery
Visit Type: icu

CHIEF COMPLAINT:
Thermal burns covering 30% body surface area and multiple traumatic injuries following high-speed motor vehicle collision

HISTORY OF PRESENT ILLNESS:
Returning following another high-speed collision, the 29-year-old male with intermittent asthma and prior tracheobronchial and aortic injuries sustained multiple fractures including a femur fracture and has extensive burns covering 30% of his body surface area. The patient is intubated due to respiratory distress and is scheduled for emergency surgery.

PAST MEDICAL HISTORY:
Active Problem List:
- Tracheobronchial injury (diagnosed 2020-01-15)
- Blunt thoracic aortic injury (diagnosed 2020-09-15)
- Asthma, intermittent

His medical history is significant for asthma,.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes for which he uses an albuterol inhaler as needed.

Home Medications:
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal asthma
- Paternal hypertension
- Maternal grandfather with coronary artery disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Field Sales Representative.

VITAL SIGNS:
On examination, his heart rate is 110 bpm, blood pressure is 135/85 mmHg, and oxygen saturation is 92% on 100% oxygen.
