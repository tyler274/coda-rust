# synthetic_hospital patient 2645

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 34 at first encounter, sex M, White, Non-Hispanic, Private (Employer-sponsored PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-06-23 (Mon) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2025-12-26 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "General anesthetic agents (bronchospasm)",
  "Dust mites"
 ],
 "insurance": "Private (Employer-sponsored PPO)",
 "occupation": "Software Developer",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Asthma",
  "Father: Allergic rhinitis",
  "Sister: Atopic dermatitis"
 ],
 "race_ethnicity": "White, Non-Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "90 mcg, 2 puffs every 4-6 hours as needed for shortness of breath or wheezing",
   "name": "Albuterol HFA inhaler"
  },
  {
   "dose": "300 mg by mouth twice daily (started 1 week prior)",
   "name": "Theophylline extended-release"
  }
 ],
 "surgical_history": [
  "Tonsillectomy and adenoidectomy (age 8, complicated by post-operative bronchospasm)",
  "Arthroscopic meniscal repair, left knee (age 28, complicated by intraoperative bronchospasm)"
 ],
 "chronic_conditions": [
  "Asthma, persistent moderate"
 ],
 "age_at_first_encounter": 34
}
```

Primary diagnoses: ["Theophylline toxicity"]
Comorbidities: ["Asthma"]

## Encounter 0: source 2020-01-15, practice 2025-06-23, ed, Emergency Medicine

Chief complaint: Sudden-onset vomiting, tremors, and restlessness

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset vomiting, tremors, and restlessness

HISTORY OF PRESENT ILLNESS:
She reports that he was recently prescribed a new medication for his asthma after reporting poor symptom control on his usual albuterol inhaler. Earlier today, he complained of a pounding headache and nausea, then began to vomit repeatedly and developed noticeable shaking of his hands. At home, she also noted that he was unusually irritable and had trouble focusing during conversation.

PAST MEDICAL HISTORY:
Active Problem List:
- Asthma, persistent moderate

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy (age 8, complicated by post-operative bronchospasm)
- Arthroscopic meniscal repair, left knee (age 28, complicated by intraoperative bronchospasm)

MEDICATIONS:
Albuterol inhaler, sustained-release theophylline (recently started)

Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 puffs every 4-6 hours as needed for shortness of breath or wheezing
- Theophylline extended-release 300 mg by mouth twice daily (started 1 week prior)

ALLERGIES:
General anesthetic agents (bronchospasm), Dust mites

FAMILY HISTORY:
- Mother: Asthma
- Father: Allergic rhinitis
- Sister: Atopic dermatitis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Software Developer.

VITAL SIGNS:
Blood pressure of 140/90 mmHg, pulse of 122/min, respiratory rate of 24/min, and temperature of 37.6°C (99.7°F)

PHYSICAL EXAMINATION:
He is agitated. Physical examination reveals a thin, diaphoretic man with fine bilateral hand tremor and frequent dry heaving. Cardiac and pulmonary exams are unremarkable.

LABORATORY DATA:
His serum creatinine is normal, and point-of-care glucose is 102 mg/dL

OTHER STUDIES:
ECG shows sinus tachycardia

## Encounter 1: source 2020-07-15, practice 2025-12-26, ed, Emergency Medicine

Chief complaint: Brought to the emergency department after a motor vehicle collision with moderate pain from multiple fractures

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Okonkwo, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought to the emergency department after a motor vehicle collision with moderate pain from multiple fractures

HISTORY OF PRESENT ILLNESS:
This 34-year-old male with persistent moderate asthma on theophylline who previously presented with vomiting and tremors presents following a motor vehicle collision. He is alert but anxious and in moderate pain from multiple fractures.

PAST MEDICAL HISTORY:
Active Problem List:
- Theophylline toxicity (diagnosed 2020-01-15)
- Asthma, persistent moderate

The patient has a history of asthma and reports previous episodes of bronchospasm after receiving general anesthesia.

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy (age 8, complicated by post-operative bronchospasm)
- Arthroscopic meniscal repair, left knee (age 28, complicated by intraoperative bronchospasm)

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 puffs every 4-6 hours as needed for shortness of breath or wheezing
- Theophylline extended-release 300 mg by mouth twice daily (started 1 week prior)

ALLERGIES:
General anesthetic agents (bronchospasm), Dust mites

FAMILY HISTORY:
- Mother: Asthma
- Father: Allergic rhinitis
- Sister: Atopic dermatitis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Software Developer.

VITAL SIGNS:
His vital signs are stable.

PHYSICAL EXAMINATION:
Shortly after induction, the patient develops nystagmus and increased skeletal muscle tone, which are recognized side effects of this medication.

PLAN:
The trauma team plans to perform an urgent orthopedic procedure. The anesthesiologist selects an intravenous agent that provides dissociative anesthesia, preserves spontaneous respirations, maintains airway reflexes, and has a sympathomimetic effect that increases heart rate and blood pressure.
