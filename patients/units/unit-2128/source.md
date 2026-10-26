# synthetic_hospital patient 2128

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, African American, Private commercial insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-10-05 (Thu) | outpatient | Pulmonology |
| 1 | 2021-01-15 | 2024-09-30 (Mon) | outpatient | Endocrinology |
| 2 | 2022-01-15 | 2025-10-14 (Tue) | outpatient | Dermatology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private commercial insurance",
 "occupation": "Information technology specialist",
 "alcohol_use": "Social, 1-2 drinks per week",
 "family_history": [
  "Father: Type 2 diabetes mellitus",
  "Mother: Autoimmune thyroiditis",
  "Sister: Pulmonary sarcoidosis"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "Appendectomy (age 16)"
 ],
 "chronic_conditions": [],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Pancreatic neuroendocrine tumor localization", "Primary adrenal insufficiency", "Sarcoidosis"]
Comorbidities: ["Chronic anemia"]

## Encounter 0: source 2020-01-15, practice 2023-10-05, outpatient, Pulmonology

Chief complaint: Persistent dry cough for six months and generalized fatigue with new tender red bumps on shins

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent dry cough for six months and generalized fatigue with new tender red bumps on shins

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a six-month history of persistent dry cough and generalized fatigue. He denies fever, hemoptysis, chest pain, or recent infections. He has also noticed new tender red bumps on both of his shins that have developed over the past two weeks. He has no known history of tuberculosis, fungal exposures, or recent travel.

PAST MEDICAL HISTORY:
His past medical history is unremarkable

PAST SURGICAL HISTORY:
- Appendectomy (age 16)

MEDICATIONS:
He takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus
- Mother: Autoimmune thyroiditis
- Sister: Pulmonary sarcoidosis

SOCIAL HISTORY:
He is a nonsmoker

VITAL SIGNS:
Vital signs are within normal limits

PHYSICAL EXAMINATION:
Physical exam reveals multiple erythematous nodules over the anterior tibial surfaces bilaterally. Lung auscultation reveals faint bibasilar crackles without wheezing.

LABORATORY DATA:
His hemoglobin is 10.5 g/dL, and mean corpuscular volume is 82 fL

IMAGING:
A chest x-ray reveals bilateral hilar fullness

PATHOLOGY:
He undergoes a bronchoscopy with transbronchial biopsy of the hilar nodes, which shows non-caseating granulomas.

## Encounter 1: source 2021-01-15, practice 2024-09-30, outpatient, Endocrinology

Chief complaint: Progressive fatigue and diffuse muscle weakness over one year with skin darkening on forearms and elbows

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Okafor, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue and diffuse muscle weakness over one year with skin darkening on forearms and elbows

HISTORY OF PRESENT ILLNESS:
The patient returns one year after his Pulmonology visit for cough and shin nodules. He reports a 1-year history of progressive fatigue and diffuse muscle weakness. He also reports unintended weight loss and darkening of forearms and elbows over past several months.

PAST MEDICAL HISTORY:
Active Problem List:
- Sarcoidosis (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- Appendectomy (age 16)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus
- Mother: Autoimmune thyroiditis
- Sister: Pulmonary sarcoidosis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per week. Occupation: Information technology specialist.

VITAL SIGNS:
His blood pressure is 100/60 mm Hg, and heart rate is 82/min.

PHYSICAL EXAMINATION:
Physical exam reveals increased pigmentation in the palmar creases and buccal mucosa.

LABORATORY DATA:
Complete blood count reveals a mild normocytic anemia and an eosinophil percentage of 14%.

OTHER STUDIES:
An ACTH stimulation test is performed, which shows a very minimal increase in serum cortisol.

## Encounter 2: source 2022-01-15, practice 2025-10-14, outpatient, Dermatology

Chief complaint: Chronic painful rash on lower abdomen spreading to groin and perineum over four months with 8kg unintentional weight loss

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Chronic painful rash on lower abdomen spreading to groin and perineum over four months with 8kg unintentional weight loss

HISTORY OF PRESENT ILLNESS:
The patient is a 47-year-old man with a history of sarcoidosis and primary adrenal insufficiency who returns for evaluation of a chronic painful rash. He reports the rash started 4 months ago on his lower abdomen and has now spread to his groin and perineum. He has tried multiple topical antifungals and steroids with no sustained relief. He reports unintended weight loss of 8 kg over the past 3 months, increasing fatigue, and mild abdominal discomfort. He also notes new-onset dry mouth and increased thirst, which he attributes to stress.

PAST MEDICAL HISTORY:
Active Problem List:
- Sarcoidosis (diagnosed 2020-01-15)
- Primary adrenal insufficiency (diagnosed 2021-01-15)

PAST SURGICAL HISTORY:
- Appendectomy (age 16)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus
- Mother: Autoimmune thyroiditis
- Sister: Pulmonary sarcoidosis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per week. Occupation: Information technology specialist.

PHYSICAL EXAMINATION:
Physical examination reveals a large erythematous, scaly plaque with areas of crusting and central clearing on the lower abdomen and medial thighs. Mild conjunctival pallor is noted.

LABORATORY DATA:
Laboratory evaluation shows a hemoglobin of 10.2 g/dL, glucose of 212 mg/dL, and normocytic indices. ALT and AST are mildly elevated. HIV test is negative.

PATHOLOGY:
A skin biopsy reveals necrotic keratinocytes and perivascular lymphocytic infiltrates.
