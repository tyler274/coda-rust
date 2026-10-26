# synthetic_hospital patient 2122

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 50 at first encounter, sex M, Non-Hispanic White (Northern European ancestry), Commercial PPO (Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-10-16 (Thu) | outpatient | Cardiology |
| 1 | 2020-07-15 | 2026-04-21 (Tue) | outpatient | Gastroenterology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Atorvastatin (diffuse myopathy, CK elevation)",
  "Sulfa drugs (rash)"
 ],
 "insurance": "Commercial PPO (Blue Cross Blue Shield)",
 "occupation": "Senior Financial Analyst",
 "alcohol_use": "None (lifetime abstinent)",
 "family_history": [
  "Father: Premature coronary artery disease (MI at age 42), hyperlipidemia",
  "Mother: Type 2 diabetes mellitus",
  "Brother: Cirrhosis of unknown etiology"
 ],
 "race_ethnicity": "Non-Hispanic White (Northern European ancestry)",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg by mouth daily",
   "name": "Ezetimibe"
  },
  {
   "dose": "500 mg by mouth twice daily",
   "name": "Metformin"
  },
  {
   "dose": "500 mg by mouth every 6 hours as needed for joint pain",
   "name": "Acetaminophen"
  },
  {
   "dose": "2000 units by mouth daily",
   "name": "Vitamin D3"
  }
 ],
 "surgical_history": [
  "Cholecystectomy (age 42)"
 ],
 "chronic_conditions": [
  "Chronic liver disease of unknown etiology",
  "Type 2 diabetes mellitus (recent onset)",
  "Severe hypercholesterolemia",
  "History of statin-associated myopathy",
  "Inflammatory arthropathy"
 ],
 "age_at_first_encounter": 50
}
```

Primary diagnoses: ["Familial Hypercholesterolemia", "Hereditary Hemochromatosis with arthropathy"]
Comorbidities: ["Chronic liver disease"]

## Encounter 0: source 2020-01-15, practice 2025-10-16, outpatient, Cardiology

Chief complaint: Lipid management and medication intolerance

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Sharma, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Lipid management and medication intolerance

HISTORY OF PRESENT ILLNESS:
He was previously started on atorvastatin but developed diffuse muscle pain and elevated creatine kinase, prompting discontinuation.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic liver disease of unknown etiology
- Type 2 diabetes mellitus (recent onset)
- Severe hypercholesterolemia
- History of statin-associated myopathy
- Inflammatory arthropathy

PAST SURGICAL HISTORY:
- Cholecystectomy (age 42)

MEDICATIONS:
Home Medications:
- Ezetimibe 10 mg by mouth daily
- Metformin 500 mg by mouth twice daily
- Acetaminophen 500 mg by mouth every 6 hours as needed for joint pain
- Vitamin D3 2000 units by mouth daily

ALLERGIES:
Atorvastatin (diffuse myopathy, CK elevation), Sulfa drugs (rash)

FAMILY HISTORY:
Family history of premature coronary artery disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (lifetime abstinent). Occupation: Senior Financial Analyst.

LABORATORY DATA:
Total cholesterol: 260 mg/dL LDL: 185 mg/dL HDL: 42 mg/dL Triglycerides: 135 mg/dL

PLAN:
The physician starts him on a medication that binds bile acids in the intestine to reduce LDL levels.

## Encounter 1: source 2020-07-15, practice 2026-04-21, outpatient, Gastroenterology

Chief complaint: Persistent fatigue, abdominal pain, and joint stiffness

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. David Kim, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue, abdominal pain, and joint stiffness

HISTORY OF PRESENT ILLNESS:
The patient with chronic liver disease and recent onset diabetes mellitus returns for gastroenterology evaluation, presenting six months after his prior cardiology encounter for lipid management and medication intolerance. He reports persistent fatigue, abdominal pain, and joint stiffness over the past six months. He also reports a recent onset of diabetes mellitus despite no significant family history of the disease.

PAST MEDICAL HISTORY:
Active Problem List:
- Familial Hypercholesterolemia (diagnosed 2020-01-15)
- Chronic liver disease of unknown etiology
- Type 2 diabetes mellitus (recent onset)
- Severe hypercholesterolemia
- History of statin-associated myopathy
- Inflammatory arthropathy

PAST SURGICAL HISTORY:
- Cholecystectomy (age 42)

MEDICATIONS:
Home Medications:
- Ezetimibe 10 mg by mouth daily
- Metformin 500 mg by mouth twice daily
- Acetaminophen 500 mg by mouth every 6 hours as needed for joint pain
- Vitamin D3 2000 units by mouth daily

ALLERGIES:
Atorvastatin (diffuse myopathy, CK elevation), Sulfa drugs (rash)

FAMILY HISTORY:
- Father: Premature coronary artery disease (MI at age 42), hyperlipidemia
- Mother: Type 2 diabetes mellitus
- Brother: Cirrhosis of unknown etiology

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (lifetime abstinent). Occupation: Senior Financial Analyst.

PHYSICAL EXAMINATION:
Bronze discoloration of his skin and hepatomegaly

LABORATORY DATA:
Serum ferritin: 2500 ng/mL (Normal: 24-336 ng/mL)
- Transferrin saturation: 85% (Normal: 20-50%)
- AST: 98 U/L (Normal: 10-40 U/L)
- ALT: 110 U/L (Normal: 7-56 U/L)
- Fasting glucose: 160 mg/dL (Normal: 70-99 mg/dL)
