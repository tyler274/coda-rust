# synthetic_hospital patient 2174

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 42 at first encounter, sex M, Non-Hispanic White, Blue Cross Blue Shield PPO (employer-sponsored).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-07-15 | 2026-06-09 (Tue) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2026-06-09 (Tue) | inpatient | Neurology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Penicillin (hives)",
  "Dust mites"
 ],
 "insurance": "Blue Cross Blue Shield PPO (employer-sponsored)",
 "occupation": "Construction site supervisor",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Father: Hypertension, Type 2 diabetes",
  "Mother: Hypothyroidism",
  "Paternal grandfather: Myocardial infarction at age 68"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "10 mg PO daily PRN",
   "name": "Cetirizine"
  }
 ],
 "surgical_history": [
  "Appendectomy (age 12)",
  "Left knee arthroscopy (age 35)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Seasonal allergic rhinitis"
 ],
 "age_at_first_encounter": 42
}
```

Primary diagnoses: ["Guillain-Barr\u00e9 syndrome"]
Comorbidities: ["Infectious colitis"]

## Encounter 0: source 2020-07-15, practice 2026-06-09, ed, Emergency Medicine

Chief complaint: progressive weakness over the past 5 days

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive weakness over the past 5 days

HISTORY OF PRESENT ILLNESS:
He is brought to the emergency department by his spouse because of progressive weakness over the past 5 days. One week ago, he developed low-grade fever, abdominal cramps, and watery diarrhea that resolved spontaneously after 2 days. Two days later, he began to notice tingling in his toes and difficulty climbing stairs. His weakness has since progressed upward to involve his thighs, and he now has trouble rising from a chair without using his arms. He reports no changes in vision, speech, or swallowing, but says his legs feel "heavy" and "sluggish." He denies urinary incontinence or retention but has noticed mild shortness of breath when lying flat at night.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- Appendectomy (age 12)
- Left knee arthroscopy (age 35)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Cetirizine 10 mg PO daily PRN

ALLERGIES:
Penicillin (hives), Dust mites

FAMILY HISTORY:
- Father: Hypertension, Type 2 diabetes
- Mother: Hypothyroidism
- Paternal grandfather: Myocardial infarction at age 68

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Construction site supervisor.

VITAL SIGNS:
On examination, his temperature is 37.0°C (98.6°F), blood pressure is 130/78 mm Hg, pulse is 88/min, and respirations are 20/min.

PHYSICAL EXAMINATION:
Neurologic exam shows symmetric weakness in the lower extremities, diminished patellar and Achilles reflexes bilaterally, and decreased vibration sense in the toes. Upper extremity strength is normal, but deep tendon reflexes are absent in the arms as well. Cranial nerve exam is normal. Sensation to pinprick is preserved throughout.

OTHER STUDIES:
Nerve conduction studies are ordered, which reveal markedly slowed conduction velocities in peripheral nerves.

## Encounter 1: source 2020-07-15, practice 2026-06-09, inpatient, Neurology

Chief complaint: presents with progressive weakness and numbness in his lower limbs over the past several days

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Neurology
Visit Type: inpatient

CHIEF COMPLAINT:
He presents with progressive weakness and numbness in his lower limbs over the past several days

HISTORY OF PRESENT ILLNESS:
This 42-year-old with hypertension and seasonal allergic rhinitis returns after ED evaluation. He recently recovered from a diarrheal illness that resolved without treatment. He reports increasing difficulty climbing stairs and now notices mild shortness of breath.

PAST MEDICAL HISTORY:
Active Problem List:
- Guillain-Barré syndrome (diagnosed 2020-07-15)
- Hypertension
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- Appendectomy (age 12)
- Left knee arthroscopy (age 35)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Cetirizine 10 mg PO daily PRN

ALLERGIES:
Penicillin (hives), Dust mites

FAMILY HISTORY:
- Father: Hypertension, Type 2 diabetes
- Mother: Hypothyroidism
- Paternal grandfather: Myocardial infarction at age 68

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Construction site supervisor.

VITAL SIGNS:
Vital signs are stable.

PHYSICAL EXAMINATION:
On examination, the patient has decreased strength symmetrically in both legs, with areflexia. Sensory examination shows decreased vibration sense in the toes. Cranial nerves are intact.
Chest examination reveals shallow respirations.
