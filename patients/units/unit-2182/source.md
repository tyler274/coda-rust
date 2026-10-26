# synthetic_hospital patient 2182

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 36 at first encounter, sex M, Non-Hispanic White, Commercial insurance (Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 2 | 2020-10-15 | 2025-11-01 (Sat) | ed | Emergency Medicine |
| 3 | 2021-07-15 | 2026-08-08 (Sat) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial insurance (Blue Cross Blue Shield)",
 "occupation": "Construction laborer",
 "alcohol_use": "Social, 2-3 drinks per week",
 "family_history": [
  "Father: Hypertension, coronary artery disease",
  "Mother: Type 2 diabetes mellitus",
  "Brother: Healthy"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 2 years ago (10 pack-year history)",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Sertraline"
  }
 ],
 "surgical_history": [
  "Tonsillectomy, age 8",
  "Left knee arthroscopy for meniscal tear, age 28"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Generalized anxiety disorder"
 ],
 "age_at_first_encounter": 36
}
```

Primary diagnoses: ["Oculomotor nerve injury (CN III palsy)", "Tension pneumothorax"]
Comorbidities: ["Persistent decision-making incapacity without reassessment"]

## Encounter 2: source 2020-10-15, practice 2025-11-01, ed, Emergency Medicine

Chief complaint: High-speed motor vehicle collision with progressive lethargy and respiratory distress

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
High-speed motor vehicle collision with progressive lethargy and respiratory distress

HISTORY OF PRESENT ILLNESS:
This 36-year-old male with hypertension and generalized anxiety disorder presents following a high-speed motor vehicle collision. He was the unrestrained driver and struck a guardrail. Emergency medical services report that he was conscious at the scene but became progressively lethargic en route. Despite oxygen administration, his mental status continues to decline.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Generalized anxiety disorder

PAST SURGICAL HISTORY:
- Tonsillectomy, age 8
- Left knee arthroscopy for meniscal tear, age 28

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Sertraline 50 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, coronary artery disease
- Mother: Type 2 diabetes mellitus
- Brother: Healthy

SOCIAL HISTORY:
Smoking: Former smoker, quit 2 years ago (10 pack-year history). Alcohol: Social, 2-3 drinks per week. Occupation: Construction laborer.

PHYSICAL EXAMINATION:
On arrival, he is awake but confused and minimally responsive to verbal stimuli. He is tachypneic and appears to be in significant respiratory distress. Physical examination reveals decreased breath sounds over the entire left hemithorax, and the left chest wall is dull to percussion. Chest wall movement on the left is reduced, and his trachea is midline. He has multiple abrasions and ecchymoses on his anterior chest and left flank. Peripheral pulses are weak, and his extremities are cool to touch. He has no obvious external bleeding, but there is bruising over his lower ribs on the left side.

IMAGING:
Point-of-care ultrasound reveals significant free fluid in the left pleural cavity. No pericardial fluid is seen. Pelvis and long bones are intact.

## Encounter 3: source 2021-07-15, practice 2026-08-08, icu, Emergency Medicine

Chief complaint: High-speed motor vehicle collision with decreased consciousness requiring airway protection

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Emily Johnson, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
High-speed motor vehicle collision with decreased consciousness requiring airway protection

HISTORY OF PRESENT ILLNESS:
This 37-year-old male with hypertension and generalized anxiety, previously treated for tension pneumothorax, is brought to the emergency department after being involved in a high-speed motor vehicle collision. He is intubated for airway protection due to decreased level of consciousness.

PAST MEDICAL HISTORY:
Active Problem List:
- Tension pneumothorax (diagnosed 2020-10-15)
- Hypertension
- Generalized anxiety disorder

PAST SURGICAL HISTORY:
- Tonsillectomy, age 8
- Left knee arthroscopy for meniscal tear, age 28

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Sertraline 50 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, coronary artery disease
- Mother: Type 2 diabetes mellitus
- Brother: Healthy

SOCIAL HISTORY:
Smoking: Former smoker, quit 2 years ago (10 pack-year history). Alcohol: Social, 2-3 drinks per week. Occupation: Construction laborer.

PHYSICAL EXAMINATION:
Neurological examination reveals his right pupil is dilated and nonreactive to light, and his right eye is deviated laterally and inferiorly. The patient is unable to adduct, elevate, or depress the right eye, and ptosis is noted on the same side. Corneal reflexes are preserved bilaterally, and no facial asymmetry is observed. He does not demonstrate limb weakness or altered sensation in the extremities. No other cranial nerves appear to be involved.

IMAGING:
A noncontrast CT scan of the head shows no acute hemorrhage, but a focal area of contusion is seen near the base of the midbrain.
