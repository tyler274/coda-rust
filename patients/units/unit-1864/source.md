# synthetic_hospital patient 1864

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 33 at first encounter, sex F, Non-Hispanic White, Employer-sponsored commercial insurance (Blue Cross PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-04-21 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2025-10-10 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Employer-sponsored commercial insurance (Blue Cross PPO)",
 "occupation": "Human resources coordinator",
 "alcohol_use": "Occasional (1-2 glasses of wine per week)",
 "family_history": [
  "Mother: Hashimoto thyroiditis (autoimmune hypothyroidism)",
  "Father: Type 2 diabetes mellitus",
  "Sister: Gestational diabetes"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "200-400 mg PO every 6-8 hours as needed for menstrual cramps",
   "name": "Ibuprofen"
  },
  {
   "dose": "1 tablet PO daily (continued postpartum while breastfeeding)",
   "name": "Prenatal vitamin"
  }
 ],
 "surgical_history": [
  "Uncomplicated vaginal delivery (nulliparous prior)",
  "Wisdom teeth extraction"
 ],
 "chronic_conditions": [
  "History of gestational diabetes mellitus",
  "Primary dysmenorrhea"
 ],
 "age_at_first_encounter": 33
}
```

Primary diagnoses: ["Postpartum thyroiditis, hyperthyroid phase", "Sciatic nerve injury"]
Comorbidities: ["Family history of rheumatoid arthritis"]

## Encounter 0: source 2020-01-15, practice 2025-04-21, outpatient, Internal Medicine

Chief complaint: Fatigue, weight gain, and constipation

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, weight gain, and constipation

HISTORY OF PRESENT ILLNESS:
She presents to her primary care physician 5 months postpartum with complaints of fatigue, weight gain, and constipation. She reports that she felt unusually energetic with palpitations and heat intolerance during the first two months after delivery, but these symptoms have since resolved.

PAST MEDICAL HISTORY:
Active Problem List:
- History of gestational diabetes mellitus
- Primary dysmenorrhea

PAST SURGICAL HISTORY:
- Uncomplicated vaginal delivery (nulliparous prior)
- Wisdom teeth extraction

MEDICATIONS:
Home Medications:
- Ibuprofen 200-400 mg PO every 6-8 hours as needed for menstrual cramps
- Prenatal vitamin 1 tablet PO daily (continued postpartum while breastfeeding)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Her family history is notable for Hashimoto's thyroiditis in her mother.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 glasses of wine per week). Occupation: Human resources coordinator.

VITAL SIGNS:
Her heart rate is 58 bpm, and her blood pressure is 110/70 mmHg.

PHYSICAL EXAMINATION:
On physical examination, her skin is dry, and she appears lethargic. Her thyroid gland is non-tender and of normal size on palpation.

LABORATORY DATA:
Laboratory tests reveal the following:

- Thyroid-stimulating hormone (TSH): 10.5 mIU/L (normal range: 0.4-4.0 mIU/L)
- Free thyroxine (Free T4): 0.7 ng/dL (normal range: 0.9-1.7 ng/dL)
- Anti-thyroid peroxidase (anti-TPO) antibodies: Positive

## Encounter 1: source 2020-07-15, practice 2025-10-10, ed, Emergency Medicine

Chief complaint: Worsening pelvic inflammatory disease symptoms and new right leg numbness

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening pelvic inflammatory disease symptoms and new right leg numbness

HISTORY OF PRESENT ILLNESS:
This 33-year-old female with gestational diabetes mellitus and primary dysmenorrhea, previously evaluated for postpartum thyroiditis in January 2020, presents to the emergency department. She was diagnosed with chlamydial cervicitis one week ago and started oral doxycycline, but has had persistent lower abdominal pain and fever. She receives an intramuscular injection of ceftriaxone in the right gluteal region. Several hours later, she reports new-onset numbness in her right leg and weakness in foot dorsiflexion.

PAST MEDICAL HISTORY:
Active Problem List:
- Postpartum thyroiditis, hyperthyroid phase (diagnosed 2020-01-15)
- History of gestational diabetes mellitus
- Primary dysmenorrhea

PAST SURGICAL HISTORY:
- Uncomplicated vaginal delivery (nulliparous prior)
- Wisdom teeth extraction

MEDICATIONS:
She takes ibuprofen occasionally for menstrual cramps.

Home Medications:
- Prenatal vitamin 1 tablet PO daily (continued postpartum while breastfeeding)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Family history is notable for rheumatoid arthritis in her mother and a sibling with type 1 diabetes.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 glasses of wine per week). Occupation: Human resources coordinator.

VITAL SIGNS:
On arrival, her temperature is 38.4°C (101.1°F)

PHYSICAL EXAMINATION:
Pelvic examination shows cervical motion tenderness. Examination shows decreased strength in the right foot with impaired ankle dorsiflexion and plantarflexion. Sensation is reduced over the posterior thigh, leg, and foot. Reflexes are intact.
