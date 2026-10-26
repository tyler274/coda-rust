# synthetic_hospital patient 2323

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 65 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-05-16 (Thu) | outpatient | Pulmonology |
| 1 | 2021-07-15 | 2025-11-25 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare",
 "occupation": "Retired machinist",
 "alcohol_use": "None",
 "family_history": [
  "Father with idiopathic pulmonary fibrosis",
  "Mother with ischemic stroke at age 72",
  "Brother with coronary artery disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 35 pack-years, quit 3 years ago",
 "home_medications": [
  {
   "dose": "150 mg orally twice daily",
   "name": "Nintedanib"
  },
  {
   "dose": "81 mg orally daily",
   "name": "Aspirin"
  },
  {
   "dose": "40 mg orally at bedtime",
   "name": "Atorvastatin"
  },
  {
   "dose": "5 mg orally daily",
   "name": "Amlodipine"
  },
  {
   "dose": "20 mg orally daily",
   "name": "Omeprazole"
  }
 ],
 "surgical_history": [
  "No significant prior surgeries"
 ],
 "chronic_conditions": [
  "Idiopathic pulmonary fibrosis",
  "Ischemic stroke with persistent oropharyngeal dysphagia",
  "History of aspiration lung abscess (resolved)",
  "Hypertension",
  "Gastroesophageal reflux disease"
 ],
 "age_at_first_encounter": 65
}
```

Primary diagnoses: ["Expressive aphasia", "Idiopathic pulmonary fibrosis"]
Comorbidities: ["Acute ischemic stroke"]

## Encounter 0: source 2020-01-15, practice 2024-05-16, outpatient, Pulmonology

Chief complaint: Follow-up visit after completing antibiotic treatment for severe lung infection one year ago

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Amara Okafor, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit after completing antibiotic treatment for severe lung infection one year ago

HISTORY OF PRESENT ILLNESS:
At that time, he had presented with high-grade fever, foul-smelling sputum, and confusion. Imaging had revealed a cavitary lesion in the right lower lung field with a visible air-fluid level. He completed a prolonged course of clindamycin and gradually improved over weeks, with resolution of fever and normalization of white blood cell count. Today, he reports that his cough has resolved and he has regained his baseline energy. He denies hemoptysis, weight loss, or night sweats.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic pulmonary fibrosis
- Ischemic stroke with persistent oropharyngeal dysphagia
- History of aspiration lung abscess (resolved)
- Hypertension
- Gastroesophageal reflux disease

His past medical history is notable for an ischemic stroke 18 months ago that resulted in persistent oropharyngeal dysphagia.

PAST SURGICAL HISTORY:
- No significant prior surgeries

MEDICATIONS:
Home Medications:
- Nintedanib 150 mg orally twice daily
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally at bedtime
- Amlodipine 5 mg orally daily
- Omeprazole 20 mg orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with idiopathic pulmonary fibrosis
- Mother with ischemic stroke at age 72
- Brother with coronary artery disease

SOCIAL HISTORY:
He now lives in a nursing home and uses a modified diet

PHYSICAL EXAMINATION:
Physical examination is unremarkable

LABORATORY DATA:
Sputum cultures and inflammatory markers are negative

IMAGING:
However, a surveillance chest x-ray obtained for follow-up shows a persistent rounded opacity in the same region as before

## Encounter 1: source 2021-07-15, practice 2025-11-25, ed, Emergency Medicine

Chief complaint: Brought to the emergency department after being found disoriented and speaking nonsensically

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought to the emergency department after being found disoriented and speaking nonsensically

HISTORY OF PRESENT ILLNESS:
The patient with idiopathic pulmonary fibrosis, prior stroke with dysphagia, and resolved lung abscess presents to the emergency department. According to his wife, he was watching TV when he began talking "in gibberish" and seemed confused. He was unable to follow simple commands, and he became increasingly frustrated when people did not understand him.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic pulmonary fibrosis (diagnosed 2020-01-15)
- Ischemic stroke with persistent oropharyngeal dysphagia
- History of aspiration lung abscess (resolved)
- Hypertension
- Gastroesophageal reflux disease

PAST SURGICAL HISTORY:
- No significant prior surgeries

MEDICATIONS:
Home Medications:
- Nintedanib 150 mg orally twice daily
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally at bedtime
- Amlodipine 5 mg orally daily
- Omeprazole 20 mg orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with idiopathic pulmonary fibrosis
- Mother with ischemic stroke at age 72
- Brother with coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, 35 pack-years, quit 3 years ago. Alcohol: None. Occupation: Retired machinist.

PHYSICAL EXAMINATION:
On exam, he speaks in long, fluent sentences with intact grammar and prosody, but his words lack meaning and are often nonsensical. He appears unaware of his language deficit. Motor and sensory strength are intact in all extremities.

IMAGING:
The diagram belows show the location of the lesion.
