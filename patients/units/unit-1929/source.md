# synthetic_hospital patient 1929

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Non-Hispanic White, Private PPO (employer-sponsored).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-11-11 (Mon) | outpatient | Otolaryngology |
| 1 | 2020-07-15 | 2025-05-27 (Tue) | outpatient | Neurology |
| 2 | 2021-07-15 | 2026-05-15 (Fri) | outpatient | Otolaryngology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Private PPO (employer-sponsored)",
 "occupation": "Executive Assistant",
 "alcohol_use": "Social, 1-2 drinks per month",
 "family_history": [
  "Father: Otosclerosis with bilateral hearing aids, age 58",
  "Mother: Myotonic dystrophy type 1 with cataracts and type 2 diabetes",
  "Maternal grandmother: Late-onset muscle weakness (presumed DM1)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "Wisdom teeth extraction"
 ],
 "chronic_conditions": [
  "Otosclerosis",
  "Myotonic dystrophy type 1"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Otosclerosis", "Myotonic dystrophy type 1"]
Comorbidities: ["Family History of Hereditary Hearing Loss"]

## Encounter 0: source 2020-01-15, practice 2024-11-11, outpatient, Otolaryngology

Chief complaint: gradually worsening difficulty hearing over the past several years

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Gradually worsening difficulty hearing over the past several years

HISTORY OF PRESENT ILLNESS:
She says that she often struggles to follow conversations in crowded settings and has to increase the volume on her television. She denies ear pain, tinnitus, or recurrent infections. She has never experienced vertigo, imbalance, or head trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Otosclerosis
- Myotonic dystrophy type 1

PAST SURGICAL HISTORY:
- Wisdom teeth extraction

MEDICATIONS:
She takes no medications

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Family history is notable for a father who required hearing aids in his 40s.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per month. Occupation: Executive Assistant.

PHYSICAL EXAMINATION:
On physical examination, the external ear canals are normal and tympanic membranes appear intact without erythema or effusion. Rinne testing shows bone conduction greater than air conduction in both ears, and Weber testing localizes to the right. Neurologic examination is otherwise normal.

## Encounter 1: source 2020-07-15, practice 2025-05-27, outpatient, Neurology

Chief complaint: Progressive muscle weakness

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Sharma, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive muscle weakness

HISTORY OF PRESENT ILLNESS:
The patient returns for Neurology consultation following her Otolaryngology evaluation in January for gradually worsening hearing difficulty. She presents with a 4-month history of progressive muscle weakness, particularly in her shoulders and hips, making it difficult for her to rise from a seated position or climb stairs. She also reports frequent episodes of difficulty swallowing, especially with solid foods.

PAST MEDICAL HISTORY:
Active Problem List:
- Otosclerosis (diagnosed 2020-01-15)
- Myotonic dystrophy type 1

PAST SURGICAL HISTORY:
- Wisdom teeth extraction

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Her family history is notable for similar symptoms in her mother, who passed away at 50 years of age.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per month. Occupation: Executive Assistant.

PHYSICAL EXAMINATION:
On physical examination, she has muscle wasting and mild ptosis. After contracting facial muscles, she has decreased ability to fully relax the muscles.

LABORATORY DATA:
Laboratory tests show elevated serum creatine kinase and normal thyroid function tests.

OTHER STUDIES:
Genetic testing reveals a trinucleotide repeat expansion in the DMPK gene.

## Encounter 2: source 2021-07-15, practice 2026-05-15, outpatient, Otolaryngology

Chief complaint: progressive difficulty hearing, especially in noisy environments

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Sarah Chen, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty hearing, especially in noisy environments

HISTORY OF PRESENT ILLNESS:
The patient returns to Otolaryngology clinic for ongoing hearing concerns, with known history of otosclerosis and myotonic dystrophy type 1. She presents with a 6-month history of progressive difficulty hearing, especially in noisy environments. She reports that her ability to understand speech has somewhat improved when background noise is present. She denies any pain, discharge, or tinnitus.

PAST MEDICAL HISTORY:
Active Problem List:
- Otosclerosis (diagnosed 2020-01-15)
- Myotonic dystrophy type 1 (diagnosed 2020-07-15)

PAST SURGICAL HISTORY:
- Wisdom teeth extraction

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Her past medical history is significant for a maternal aunt and grandmother with similar hearing problems that began in their thirties.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per month. Occupation: Executive Assistant.

PHYSICAL EXAMINATION:
On physical examination, the tympanic membranes appear normal bilaterally, and there is no evidence of external ear pathology. Weber test lateralizes to the right ear, and Rinne test shows bone conduction to be greater than air conduction in the right ear.

OTHER STUDIES:
Audiometry confirms conductive hearing loss in the right ear.
