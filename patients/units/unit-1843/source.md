# synthetic_hospital patient 1843

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 22 at first encounter, sex F, Southeast Asian (Thai), University student health insurance plan.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2019-05-08 (Wed) | outpatient | Internal Medicine |
| 1 | 2021-07-15 | 2020-11-16 (Mon) | outpatient | Internal Medicine |
| 2 | 2023-07-15 | 2022-11-14 (Mon) | ed | Emergency Medicine |
| 3 | 2026-01-15 | 2025-05-19 (Mon) | outpatient | Medical Genetics |
| 4 | 2026-07-15 | 2025-11-06 (Thu) | outpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "University student health insurance plan",
 "occupation": "College student",
 "alcohol_use": "None",
 "family_history": [
  "Father with beta-thalassemia trait",
  "Paternal grandmother with beta-thalassemia major",
  "Family history of thalassemia in Thailand"
 ],
 "race_ethnicity": "Southeast Asian (Thai)",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 tablet daily",
   "name": "Multivitamin"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Beta-thalassemia minor"
 ],
 "age_at_first_encounter": 22
}
```

Primary diagnoses: ["Thalassemia minor", "Hyponatremia"]
Comorbidities: ["Delayed management of life-threatening respiratory failure"]

## Encounter 0: source 2020-01-15, practice 2019-05-08, outpatient, Internal Medicine

Chief complaint: Evaluation of mild fatigue

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of mild fatigue

PAST MEDICAL HISTORY:
Active Problem List:
- Beta-thalassemia minor

MEDICATIONS:
Home Medications:
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with beta-thalassemia trait
- Paternal grandmother with beta-thalassemia major
- Family history of thalassemia in Thailand

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: College student.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
Laboratory evaluation reveals a hemoglobin level of 11 g/dL with an MVC of 68 fL. Hemoglobin electrophoresis shows increased HbA2 and mildly increased HbF.

OTHER STUDIES:
Genetic analysis reveals a splice-site mutation.

## Encounter 1: source 2021-07-15, practice 2020-11-16, outpatient, Internal Medicine

Chief complaint: Fatigue and mild shortness of breath on exertion for the past 6 months

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue and mild shortness of breath on exertion for the past 6 months

HISTORY OF PRESENT ILLNESS:
Beta-thalassemia minor patient returns. She denies any bleeding gums, epistaxis, or menorrhagia.

PAST MEDICAL HISTORY:
Active Problem List:
- Thalassemia minor (diagnosed 2020-01-15)
- Beta-thalassemia minor

MEDICATIONS:
She takes a multivitamin daily.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with beta-thalassemia trait
- Paternal grandmother with beta-thalassemia major
- Family history of thalassemia in Thailand

SOCIAL HISTORY:
She is a college student and attributes her symptoms to her busy schedule. She eats a balanced diet

PHYSICAL EXAMINATION:
On physical examination, she appears well-nourished and is not in acute distress. Her skin and conjunctivae are not pale. There is no icterus, lymphadenopathy, or hepatosplenomegaly.

LABORATORY DATA:
Her complete blood count reveals:

- Hemoglobin: 10.2 g/dL (Normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 72 fL (Normal: 80-100 fL)
- Red cell distribution width (RDW): 12% (Normal: 11.5-14.5%)
- Platelets: 320 x 10^9/L (Normal: 150-400 x 10^9/L)
- White blood cells: 6.0 x 10^9/L (Normal: 4.0-11.0 x 10^9/L)

Iron studies are ordered and show:

- Serum iron: 90 µg/dL (Normal: 50-170 µg/dL)
- Total iron-binding capacity (TIBC): 300 µg/dL (Normal: 250-450 µg/dL)
- Ferritin: 200 ng/mL (Normal: 15-200 ng/mL)

## Encounter 2: source 2023-07-15, practice 2022-11-14, ed, Emergency Medicine

Chief complaint: Headache, nausea, and confusion after completing first marathon

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Headache, nausea, and confusion after completing first marathon

HISTORY OF PRESENT ILLNESS:
The patient is a 25-year-old female with Beta-thalassemia minor evaluated in 2020 and 2021 for fatigue and exercise intolerance. She presents after completing her first marathon. She reports that she was well-prepared for the race and consumed copious amounts of water before and during the event to stay hydrated. Her friends mention that she took salt tablets sporadically during the race.

PAST MEDICAL HISTORY:
Active Problem List:
- Thalassemia minor (diagnosed 2020-01-15)
- Beta-thalassemia minor

MEDICATIONS:
Home Medications:
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with beta-thalassemia trait
- Paternal grandmother with beta-thalassemia major
- Family history of thalassemia in Thailand

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: College student.

VITAL SIGNS:
Blood pressure 130/85 mmHg, heart rate 88 beats per minute, respiratory rate 16 breaths per minute, and temperature 37.2°C (99.0°F)

PHYSICAL EXAMINATION:
On examination, she is disoriented. She has no focal neurological deficits.

LABORATORY DATA:
Sodium: 125 mEq/L (Normal: 135-145 mEq/L)
- Potassium: 4.2 mEq/L (Normal: 3.5-5.1 mEq/L)
- Chloride: 96 mEq/L (Normal: 98-106 mEq/L)
- Bicarbonate: 24 mEq/L (Normal: 22-29 mEq/L)
- Blood urea nitrogen: 10 mg/dL (Normal: 7-20 mg/dL)
- Creatinine: 0.9 mg/dL (Normal: 0.6-1.1 mg/dL)
- Glucose: 90 mg/dL (Normal: 70-99 mg/dL)
- Serum osmolality: 260 mOsm/kg (Normal: 275-295 mOsm/kg)
- Urine specific gravity: 1.005 (Normal: 1.010-1.025)

## Encounter 3: source 2026-01-15, practice 2025-05-19, outpatient, Medical Genetics

Chief complaint: Genetics counseling prior to attempting conception

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. Priya Patel, MD
Department: Medical Genetics
Visit Type: outpatient

CHIEF COMPLAINT:
Genetics counseling prior to attempting conception

HISTORY OF PRESENT ILLNESS:
The patient returns for preconception genetics counseling. They ask about the potential genetic outcomes for their offspring regarding beta-globin synthesis.

PAST MEDICAL HISTORY:
Active Problem List:
- Thalassemia minor (diagnosed 2020-01-15)
- Hyponatremia (diagnosed 2023-07-15)
- Beta-thalassemia minor

MEDICATIONS:
Home Medications:
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with beta-thalassemia trait
- Paternal grandmother with beta-thalassemia major
- Family history of thalassemia in Thailand

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: College student.

LABORATORY DATA:
Both individuals have elevated HbA2 on hemoglobin electrophoresis

OTHER STUDIES:
Molecular analysis reveals the husband is heterozygous for a null mutation in the beta-globin ( HBB ) gene, resulting in complete absence of beta-globin from that allele. The woman is heterozygous for a different HBB allele, which leads to approximately 50% reduction in beta-globin production due to a promoter region mutation.

## Encounter 4: source 2026-07-15, practice 2025-11-06, outpatient, Obstetrics & Gynecology

Chief complaint: Initial prenatal visit

ENCOUNTER NOTE
Date: 2026-07-15
Provider: Dr. Jennifer Okonkwo, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Initial prenatal visit

HISTORY OF PRESENT ILLNESS:
The patient with Beta-thalassemia minor diagnosed in 2020 presents for initial prenatal care following January genetic counseling. She is excited about her pregnancy and reports feeling well overall. At a follow-up appointment three weeks later, she reports that she has been taking the iron consistently but has noticed no improvement in how she feels.

PAST MEDICAL HISTORY:
Active Problem List:
- Thalassemia minor (diagnosed 2020-01-15)
- Hyponatremia (diagnosed 2023-07-15)
- Beta-thalassemia minor

MEDICATIONS:
She has been taking prenatal vitamins regularly

Home Medications:
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is notable for several relatives who were told they had anemia, though she is unsure of the details.

SOCIAL HISTORY:
She recently immigrated from Thailand. and has a balanced diet.

REVIEW OF SYSTEMS:
No complaints of fatigue, dizziness, or dyspnea

VITAL SIGNS:
Vital signs are normal.

PHYSICAL EXAMINATION:
Physical examination is unremarkable apart from mild conjunctival pallor; there is no hepatosplenomegaly or lymphadenopathy.

LABORATORY DATA:
Routine prenatal labs are collected during the visit. A review of follow-up bloodwork shows no meaningful changes from her initial labs.

OTHER STUDIES:
Hemoglobin electrophoresis reveals findings consistent with α-thalassemia trait.

PLAN:
She is started empirically on oral iron supplementation. Her obstetrician orders further testing. Her partner is referred for screening, and genetic counseling is arranged.
