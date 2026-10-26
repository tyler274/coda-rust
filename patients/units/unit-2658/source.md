# synthetic_hospital patient 2658

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 35 at first encounter, sex M, Non-Hispanic White, Aetna PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-01-22 (Wed) | outpatient | Internal Medicine |
| 1 | 2020-03-15 | 2025-03-24 (Mon) | outpatient | Hematology & Oncology |
| 2 | 2021-03-15 | 2026-03-17 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Aetna PPO",
 "occupation": "Financial Analyst",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "No family history of hematologic disorders, clotting disorders, or malignancy",
  "Father: Hypertension, hyperlipidemia",
  "Mother: Hypothyroidism"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [
  "History of idiopathic deep vein thrombosis (left lower extremity, diagnosed age 33)"
 ],
 "age_at_first_encounter": 35
}
```

Primary diagnoses: ["Paroxysmal nocturnal hemoglobinuria"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2025-01-22, outpatient, Internal Medicine

Chief complaint: Several months of fatigue and dark-colored urine noticed predominantly in the mornings

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Several months of fatigue and dark-colored urine noticed predominantly in the mornings

HISTORY OF PRESENT ILLNESS:
He has also noticed dark-colored urine in the mornings that seems to clear later in the day. He denies dysuria, urinary frequency, fever, or flank pain.

PAST MEDICAL HISTORY:
Active Problem List:
- History of idiopathic deep vein thrombosis (left lower extremity, diagnosed age 33)

ALLERGIES:
NKDA

FAMILY HISTORY:
- No family history of hematologic disorders, clotting disorders, or malignancy
- Father: Hypertension, hyperlipidemia
- Mother: Hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Financial Analyst.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
Laboratory evaluation shows: Hemoglobin 10.2 g/dL Mean corpuscular volume 90 fL Reticulocyte count Elevated LDH Elevated Haptoglobin Decreased Flow cytometry reveals deficiency of CD55 and CD59 on red blood cells.

## Encounter 1: source 2020-03-15, practice 2025-03-24, outpatient, Hematology & Oncology

Chief complaint: Progressive fatigue and exertional shortness of breath with continued episodes of morning dark urine

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Michael Rodriguez, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue and exertional shortness of breath with continued episodes of morning dark urine

HISTORY OF PRESENT ILLNESS:
This 35-year-old male with history of idiopathic left lower extremity DVT diagnosed age 33 returns for follow-up two months after initial presentation. He describes urine as reddish-brown upon waking, which gradually clears throughout the day. He has also been experiencing progressive fatigue and mild exertional shortness of breath over past few months. He denies dysuria, urgency, flank pain, fevers, weight loss, or recent infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Paroxysmal nocturnal hemoglobinuria (diagnosed 2020-01-15)
- History of idiopathic deep vein thrombosis (left lower extremity, diagnosed age 33)

MEDICATIONS:
He takes no medications.

ALLERGIES:
NKDA

FAMILY HISTORY:
There is no family history of hematologic disorders or kidney disease.

SOCIAL HISTORY:
He has no history of recent travel, no known toxin or drug exposures,

VITAL SIGNS:
His vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, he appears pale but is in no acute distress.
Cardiopulmonary examination is normal, and there is no hepatosplenomegaly or lymphadenopathy.

LABORATORY DATA:
Laboratory results demonstrate normocytic anemia with an elevated reticulocyte count. Serum lactate dehydrogenase is markedly elevated, and haptoglobin is undetectable. A direct antiglobulin (Coombs) test is negative. Urinalysis reveals the presence of free hemoglobin but no red blood cells.

OTHER STUDIES:
Further testing with flow cytometry reveals a deficiency of glycosylphosphatidylinositol (GPI)-anchored proteins on red blood cells, granulocytes, and monocytes.

## Encounter 2: source 2021-03-15, practice 2026-03-17, ed, Emergency Medicine

Chief complaint: Acute severe abdominal pain and severe headaches with difficulty concentrating

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute severe abdominal pain and severe headaches with difficulty concentrating

HISTORY OF PRESENT ILLNESS:
This 36-year-old male with history of idiopathic left lower extremity deep vein thrombosis diagnosed age 33 and evaluation for fatigue and dark urine presents with 6-month history of fatigue, intermittent dark urine, and episode of unexplained abdominal pain that resolved spontaneously. He also reports recent history of headaches and difficulty concentrating.

PAST MEDICAL HISTORY:
Active Problem List:
- Paroxysmal nocturnal hemoglobinuria (diagnosed 2020-01-15)
- History of idiopathic deep vein thrombosis (left lower extremity, diagnosed age 33)

His past medical history is significant for an idiopathic deep vein thrombosis treated with warfarin two years ago.

ALLERGIES:
NKDA

FAMILY HISTORY:
- No family history of hematologic disorders, clotting disorders, or malignancy
- Father: Hypertension, hyperlipidemia
- Mother: Hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Financial Analyst.

PHYSICAL EXAMINATION:
On physical examination, he appears pale and jaundiced. There is no hepatosplenomegaly or lymphadenopathy. His neurological examination is normal.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 8.5 g/dL (Normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 96 fL (Normal: 80-96 fL)
- Lactate dehydrogenase (LDH): 450 U/L (Normal: 140-280 U/L)
- Total bilirubin: 3.2 mg/dL (Normal: 0.3-1.2 mg/dL), with indirect bilirubin of 2.5 mg/dL
- Haptoglobin: <10 mg/dL (Normal: 30-200 mg/dL)
- Reticulocyte count: 4% (Normal: 0.5-1.5%)
- Platelet count: 110,000/µL (Normal: 150,000-450,000/µL)
- Prothrombin time (PT): 12 seconds (Normal: 9-12 seconds)
- Activated partial thromboplastin time (aPTT): 28 seconds (Normal: 25-35 seconds)
- Direct Coombs test: Negative

ASSESSMENT AND PLAN:
Given the patient's symptoms, medical history, and laboratory findings,
