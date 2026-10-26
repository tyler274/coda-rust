# synthetic_hospital patient 2152

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 42 at first encounter, sex M, Middle Eastern, Employer-sponsored commercial health insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-07-03 (Wed) | ed | Emergency Medicine |
| 1 | 2022-01-15 | 2026-07-02 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Employer-sponsored commercial health insurance",
 "occupation": "Chemical manufacturing technician at agricultural products plant",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Maternal uncle: G6PD deficiency",
  "Mother: Carrier for G6PD deficiency",
  "Father: Hypertension"
 ],
 "race_ethnicity": "Middle Eastern",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "Glucose-6-phosphate dehydrogenase (G6PD) deficiency (diagnosed age 43)"
 ],
 "age_at_first_encounter": 42
}
```

Primary diagnoses: ["Acquired methemoglobinemia"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2024-07-03, ed, Emergency Medicine

Chief complaint: progressive shortness of breath and bluish discoloration of the lips and fingertips

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive shortness of breath and bluish discoloration of the lips and fingertips

HISTORY OF PRESENT ILLNESS:
He is brought to the emergency department by coworkers because of progressive shortness of breath and bluish discoloration of the lips and fingertips that developed suddenly during a night shift. He works at an industrial plant that manufactures agricultural chemicals, where he is frequently exposed to fumes and compounds containing nitrogen-based oxidants. Over the past several hours he has become increasingly fatigued and lightheaded but has not experienced chest pain, palpitations, fever, or cough.

PAST MEDICAL HISTORY:
Active Problem List:
- Glucose-6-phosphate dehydrogenase (G6PD) deficiency (diagnosed age 43)

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes no regular medications

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Family history is unremarkable.

SOCIAL HISTORY:
He does not smoke

VITAL SIGNS:
Pulse oximetry shows an oxygen saturation of 84% on room air that does not improve with high-flow supplemental oxygen. Pulse is 108/min, blood pressure is 116/72 mmHg, and respirations are 22/min.

PHYSICAL EXAMINATION:
On examination, he is alert but anxious and tachypneic. The skin and mucous membranes have a gray-blue hue, and the cardiopulmonary exam is otherwise normal.

## Encounter 1: source 2022-01-15, practice 2026-07-02, ed, Emergency Medicine

Chief complaint: cyanosis and shortness of breath

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Cyanosis and shortness of breath

HISTORY OF PRESENT ILLNESS:
The patient is a 44-year-old male with glucose-6-phosphate dehydrogenase deficiency diagnosed at age 43, returning to the emergency department two years following a previous episode of acquired methemoglobinemia. He presents with a 2-hour history of cyanosis and shortness of breath. He reports that his symptoms began shortly after using a topical anesthetic cream for a minor skin procedure at home.

PAST MEDICAL HISTORY:
Active Problem List:
- Acquired methemoglobinemia (diagnosed 2020-01-15)
- Glucose-6-phosphate dehydrogenase (G6PD) deficiency (diagnosed age 43)

His past medical history is significant for glucose-6-phosphate dehydrogenase (G6PD) deficiency.

PAST SURGICAL HISTORY:
- No prior surgeries

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal uncle: G6PD deficiency
- Mother: Carrier for G6PD deficiency
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Chemical manufacturing technician at agricultural products plant.

VITAL SIGNS:
His vital signs are as follows: blood pressure 130/85 mmHg, heart rate 102 beats per minute, respiratory rate 22 breaths per minute, and oxygen saturation of 85% on room air, which does not improve with supplemental oxygen.

PHYSICAL EXAMINATION:
On examination, he appears anxious, with a noticeable bluish discoloration of his lips and fingers.

LABORATORY DATA:
Arterial blood gas analysis reveals a PaO2 of 98 mmHg. His complete blood count and basic metabolic panel are within normal limits. A co-oximetry test shows a methemoglobin level of 30%.
