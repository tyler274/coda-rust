# synthetic_hospital patient 2819

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 44 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-05-10 (Wed) | outpatient | Gastroenterology |
| 1 | 2023-01-15 | 2026-05-11 (Mon) | outpatient | Gastroenterology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Sulfa drugs (anaphylaxis)"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "High school mathematics teacher",
 "alcohol_use": "No alcohol consumption; has been abstinent for 3 years due to elevated liver enzymes noted on prior routine screening",
 "family_history": [
  "Mother with Hashimoto thyroiditis",
  "Father with essential hypertension",
  "Maternal aunt with rheumatoid arthritis",
  "No known family history of liver disease or bleeding disorders"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg daily",
   "name": "Prednisone"
  },
  {
   "dose": "50 mg daily",
   "name": "Azathioprine"
  },
  {
   "dose": "100 mg daily",
   "name": "Spironolactone"
  },
  {
   "dose": "20 mg twice daily",
   "name": "Propranolol"
  },
  {
   "dose": "5 mg daily",
   "name": "Phytonadione (Vitamin K1)"
  },
  {
   "dose": "40 mg daily",
   "name": "Pantoprazole"
  }
 ],
 "surgical_history": [
  "Laparoscopic cholecystectomy (age 39)",
  "Dental extraction (age 42) complicated by prolonged bleeding"
 ],
 "chronic_conditions": [
  "Autoimmune hepatitis",
  "Cirrhosis",
  "Portal hypertension",
  "Thrombocytopenia secondary to hypersplenism",
  "Coagulopathy"
 ],
 "age_at_first_encounter": 44
}
```

Primary diagnoses: ["Chronic liver disease"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2023-05-10, outpatient, Gastroenterology

Chief complaint: Vague abdominal discomfort, unintentional weight loss, and easy bruising

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Vague abdominal discomfort, unintentional weight loss, and easy bruising

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with complaints of vague abdominal discomfort and recent unintentional weight loss. She notes easy bruising over the past few weeks. She also reports occasional nausea and fatigue.

PAST MEDICAL HISTORY:
Active Problem List:
- Autoimmune hepatitis
- Cirrhosis
- Portal hypertension
- Thrombocytopenia secondary to hypersplenism
- Coagulopathy

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 39)
- Dental extraction (age 42) complicated by prolonged bleeding

MEDICATIONS:
Home Medications:
- Prednisone 10 mg daily
- Azathioprine 50 mg daily
- Spironolactone 100 mg daily
- Propranolol 20 mg twice daily
- Phytonadione (Vitamin K1) 5 mg daily
- Pantoprazole 40 mg daily

ALLERGIES:
Sulfa drugs (anaphylaxis)

FAMILY HISTORY:
- Mother with Hashimoto thyroiditis
- Father with essential hypertension
- Maternal aunt with rheumatoid arthritis
- No known family history of liver disease or bleeding disorders

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol consumption; has been abstinent for 3 years due to elevated liver enzymes noted on prior routine screening. Occupation: High school mathematics teacher.

PHYSICAL EXAMINATION:
Physical exam reveals mild jaundice and scattered ecchymoses.

LABORATORY DATA:
Laboratory studies show:
Prothrombin time                                                         20 seconds
Partial thromboplastin time                                          35 seconds
Damage to

## Encounter 1: source 2023-01-15, practice 2026-05-11, outpatient, Gastroenterology

Chief complaint: Fatigue and mild jaundice

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Marcus Johnson, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue and mild jaundice

HISTORY OF PRESENT ILLNESS:
The patient with autoimmune hepatitis and cirrhosis returns for follow-up of chronic liver disease from 2020. She presents with fatigue and mild jaundice. Her physician suspects a rare liver disease and orders a new diagnostic blood test with a sensitivity of 92% and specificity of 88%.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic liver disease (diagnosed 2020-01-15)
- Autoimmune hepatitis
- Cirrhosis
- Portal hypertension
- Thrombocytopenia secondary to hypersplenism
- Coagulopathy

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 39)
- Dental extraction (age 42) complicated by prolonged bleeding

MEDICATIONS:
Home Medications:
- Prednisone 10 mg daily
- Azathioprine 50 mg daily
- Spironolactone 100 mg daily
- Propranolol 20 mg twice daily
- Phytonadione (Vitamin K1) 5 mg daily
- Pantoprazole 40 mg daily

ALLERGIES:
Sulfa drugs (anaphylaxis)

FAMILY HISTORY:
- Mother with Hashimoto thyroiditis
- Father with essential hypertension
- Maternal aunt with rheumatoid arthritis
- No known family history of liver disease or bleeding disorders

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol consumption; has been abstinent for 3 years due to elevated liver enzymes noted on prior routine screening. Occupation: High school mathematics teacher.

OTHER STUDIES:
In a population of 1,200 individuals, the prevalence of this disease is estimated to be 5%. The test results show 55 true positives, 14 false negatives, 124 false positives, and 1007 true negatives.

PLAN:
You are asked to compute the positive predictive value (PPV) for this test in the described population.
