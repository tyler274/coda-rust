# synthetic_hospital patient 2630

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 19 at first encounter, sex M, Non-Hispanic White, Private insurance (parent's employer-sponsored plan).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-11-27 (Wed) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2025-05-19 (Mon) | outpatient | Otolaryngology |
| 2 | 2021-01-15 | 2025-11-30 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Amoxicillin (maculopapular rash)"
 ],
 "insurance": "Private insurance (parent's employer-sponsored plan)",
 "occupation": "College freshman - dormitory resident",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Hypothyroidism",
  "Father: Essential hypertension",
  "No family history of autoimmune disorders or hematologic malignancies"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 19
}
```

Primary diagnoses: ["Infectious mononucleosis"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2024-11-27, outpatient, Internal Medicine

Chief complaint: 1-week history of malaise, sore throat, fever, and puffy neck swelling

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
1-week history of malaise, sore throat, fever, and puffy neck swelling

HISTORY OF PRESENT ILLNESS:
He reports that the symptoms started with a mild headache and fatigue, but have progressively worsened. He now has difficulty swallowing due to throat pain and has noted a "puffy" feeling in his neck.

PAST MEDICAL HISTORY:
His past medical history is unremarkable

MEDICATIONS:
He takes no medications

ALLERGIES:
Amoxicillin (maculopapular rash)

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Essential hypertension
- No family history of autoimmune disorders or hematologic malignancies

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: College freshman - dormitory resident.

REVIEW OF SYSTEMS:
He denies cough, rhinorrhea, or chest pain.

VITAL SIGNS:
The patient is febrile with a temperature of 38.5°C (101.3°F).

PHYSICAL EXAMINATION:
Examination of the oropharynx reveals exudative tonsillitis. There is marked cervical lymphadenopathy, and the spleen is palpable 2 cm below the left costal margin.

LABORATORY DATA:
A peripheral blood smear shows atypical lymphocytes. A rapid strep test is negative.

## Encounter 1: source 2020-07-15, practice 2025-05-19, outpatient, Otolaryngology

Chief complaint: Sore throat, fatigue, and diffuse maculopapular rash after starting amoxicillin two days ago

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Sore throat, fatigue, and diffuse maculopapular rash after starting amoxicillin two days ago

HISTORY OF PRESENT ILLNESS:
The patient returns to the Otolaryngology clinic six months following a prior presentation with similar symptoms. He was previously evaluated in January 2020 for malaise, sore throat, fever, and puffy neck swelling. He presents with sore throat, fatigue, and mild fever for the past week. He reports feeling generally unwell with reduced appetite and malaise. He was prescribed amoxicillin two days ago, after which he developed a diffuse, erythematous maculopapular rash over his trunk and extremities.

PAST MEDICAL HISTORY:
Active Problem List:
- Infectious mononucleosis (diagnosed 2020-01-15)

ALLERGIES:
Amoxicillin (maculopapular rash)

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Essential hypertension
- No family history of autoimmune disorders or hematologic malignancies

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: College freshman - dormitory resident.

REVIEW OF SYSTEMS:
He denies shortness of breath or joint pains.

VITAL SIGNS:
On examination, his temperature is 38.4°C (101.1°F).

PHYSICAL EXAMINATION:
There is bilateral cervical lymphadenopathy and enlarged tonsils with white exudates. Lungs are clear on auscultation, and there is no cough or wheezing.

## Encounter 2: source 2021-01-15, practice 2025-11-30, ed, Emergency Medicine

Chief complaint: 10-day history of high fever up to 102°F, severe sore throat, and difficulty swallowing

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
10-day history of high fever up to 102°F, severe sore throat, and difficulty swallowing

HISTORY OF PRESENT ILLNESS:
The patient presents to the emergency department with recurrent upper respiratory symptoms following two prior episodes of infectious mononucleosis diagnosed in January and July 2020. He reports that the symptoms started with a mild sore throat but have progressively worsened. He now has difficulty swallowing and feels extremely tired, with fevers up to 38.9°C (102°F). He denies cough, rhinorrhea, or rash.

PAST MEDICAL HISTORY:
Active Problem List:
- Infectious mononucleosis (diagnosed 2020-01-15)

MEDICATIONS:
He takes no medications.

ALLERGIES:
Amoxicillin (maculopapular rash)

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Essential hypertension
- No family history of autoimmune disorders or hematologic malignancies

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: College freshman - dormitory resident.

PHYSICAL EXAMINATION:
On physical examination, he has bilateral anterior and posterior cervical lymphadenopathy, and his oropharynx is erythematous with enlarged tonsils and exudates. The spleen is palpable 2 cm below the left costal margin. No hepatomegaly is noted. The remainder of the examination is unremarkable.

LABORATORY DATA:
Laboratory tests reveal the following:

- White blood cell count: 12,000/µL (normal 4,500-11,000/µL)
- Atypical lymphocytes: 20%
- Alanine aminotransferase (ALT): 70 U/L (normal 7-56 U/L)
- Aspartate aminotransferase (AST): 65 U/L (normal 8-48 U/L)
