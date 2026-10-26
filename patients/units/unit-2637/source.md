# synthetic_hospital patient 2637

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 25 at first encounter, sex M, African American, Private insurance (employer-sponsored).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-03-08 (Wed) | outpatient | Internal Medicine |
| 1 | 2021-07-15 | 2024-08-27 (Tue) | outpatient | Nephrology |
| 2 | 2023-01-15 | 2026-03-09 (Mon) | outpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Private insurance (employer-sponsored)",
 "occupation": "Software developer",
 "alcohol_use": "None",
 "family_history": [
  "Father with sickle cell trait",
  "Maternal cousin with sickle cell disease",
  "Mother with hypertension"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg orally daily",
   "name": "Hydroxyurea"
  },
  {
   "dose": "1 mg orally daily",
   "name": "Folic acid"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Sickle cell trait",
  "Recurrent vaso-occlusive pain crises",
  "Hyposthenuria secondary to sickle cell trait",
  "Chronic kidney disease, stage 2"
 ],
 "age_at_first_encounter": 25
}
```

Primary diagnoses: ["Sickle cell trait", "Folate deficiency due to increased requirement from erythroid hyperplasia"]
Comorbidities: ["Sickle cell anemia without crisis"]

## Encounter 0: source 2020-01-15, practice 2023-03-08, outpatient, Internal Medicine

Chief complaint: Persistent fatigue and generalized weakness over the past several months with decreased exercise tolerance and occasional lightheadedness on standing.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Jennifer Walsh, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue and generalized weakness over the past several months with decreased exercise tolerance and occasional lightheadedness on standing.

HISTORY OF PRESENT ILLNESS:
He reports that he tires more quickly while climbing stairs and occasionally feels lightheaded when standing up too quickly. He denies bleeding, fever, or recent illness. His appetite is fair, and he consumes a mixed diet.

PAST MEDICAL HISTORY:
Active Problem List:
- Sickle cell trait
- Recurrent vaso-occlusive pain crises
- Hyposthenuria secondary to sickle cell trait
- Chronic kidney disease, stage 2

He notes that he has had intermittent episodes of severe limb and back pain since childhood, which required hospitalization and intravenous pain management. He has not had any hospitalizations in the past year but continues to experience milder episodes of pain every few weeks that resolve with rest and oral hydration. He is up to date on his vaccinations.

MEDICATIONS:
His only medications are daily hydroxyurea and folic acid supplements, though he admits he sometimes forgets to take the folate.

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with sickle cell trait
- Maternal cousin with sickle cell disease
- Mother with hypertension

SOCIAL HISTORY:
He does not drink alcohol or smoke.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
Physical examination shows pale conjunctivae and a systolic flow murmur. No icterus or lymphadenopathy is noted. Abdominal examination is benign with no palpable spleen. Neurologic exam is unremarkable.

LABORATORY DATA:
Laboratory studies reveal:
Hemoglobin                                    8.8 g/dL
Mean corpuscular volume                           112 fL
Reticulocyte count                                  8%
WBC count                                     7,200/mm³
Platelet count                                    430,000/mm³
Total bilirubin                                     2.3 mg/dL
LDH                                      418 U/L

## Encounter 1: source 2021-07-15, practice 2024-08-27, outpatient, Nephrology

Chief complaint: Six-month history of increased urination, particularly at night, accompanied by recent onset of generalized fatigue and nocturia.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Robert Chen, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of increased urination, particularly at night, accompanied by recent onset of generalized fatigue and nocturia.

HISTORY OF PRESENT ILLNESS:
The patient is a 26-year-old male with sickle cell trait and associated hyposthenuria who presents to the Nephrology clinic with a six-month history of increased urination, nocturia, and generalized fatigue. He has a history of recurrent vaso-occlusive pain crises and previously presented with fatigue in January 2020. He denies any pain on urination, fever, or back pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Folate deficiency due to increased requirement from erythroid hyperplasia (diagnosed 2020-01-15)
- Sickle cell trait
- Recurrent vaso-occlusive pain crises
- Hyposthenuria secondary to sickle cell trait
- Chronic kidney disease, stage 2

His past medical history is significant for sickle cell trait, which was diagnosed during newborn screening.

MEDICATIONS:
He does not take any medications

Home Medications:
- Hydroxyurea 1000 mg orally daily
- Folic acid 1 mg orally daily

ALLERGIES:
He has no known drug allergies.

FAMILY HISTORY:
- Father with sickle cell trait
- Maternal cousin with sickle cell disease
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Software developer.

VITAL SIGNS:
On physical examination, his blood pressure is 130/80 mmHg, heart rate is 78 beats per minute, and he is afebrile.

PHYSICAL EXAMINATION:
His abdomen is soft, non-tender, and there is no costovertebral angle tenderness.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum sodium: 140 mEq/L (normal range: 135-145 mEq/L)
- Serum potassium: 4.2 mEq/L (normal range: 3.5-5.1 mEq/L)
- Serum creatinine: 0.9 mg/dL (normal range: 0.6-1.2 mg/dL)
- Urine specific gravity: 1.005 (normal range: 1.010-1.030)

Given the patient's history and laboratory findings,

## Encounter 2: source 2023-01-15, practice 2026-03-09, outpatient, Obstetrics & Gynecology

Chief complaint: Preconception counseling visit regarding genetic risk assessment and inheritance patterns for sickle cell trait in potential offspring.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Amara Okafor, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Preconception counseling visit regarding genetic risk assessment and inheritance patterns for sickle cell trait in potential offspring.

HISTORY OF PRESENT ILLNESS:
The patient with sickle cell trait and history of vaso-occlusive crises, nocturia, and folate deficiency presents for preconception counseling. The couple expresses concern regarding the risk of having a son who carries the sickle cell mutation without disease manifestation.

PAST MEDICAL HISTORY:
Active Problem List:
- Folate deficiency due to increased requirement from erythroid hyperplasia (diagnosed 2020-01-15)
- Sickle cell trait (diagnosed 2021-07-15)
- Recurrent vaso-occlusive pain crises
- Hyposthenuria secondary to sickle cell trait
- Chronic kidney disease, stage 2

MEDICATIONS:
Home Medications:
- Hydroxyurea 1000 mg orally daily
- Folic acid 1 mg orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with sickle cell trait
- Maternal cousin with sickle cell disease
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Software developer.

LABORATORY DATA:
The man's previous hemoglobin electrophoresis revealed only HbA bands. His wife's hemoglobin electrophoresis has not yet been performed.

ASSESSMENT AND PLAN:
The physician notes that sickle cell anemia and sickle cell trait are both caused by a missense mutation in the beta-globin gene, but clinical severity differs depending on zygosity. In the general African American population, approximately 1 in 13 individuals carry the sickle cell allele.
