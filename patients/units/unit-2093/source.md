# synthetic_hospital patient 2093

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 44 at first encounter, sex M, Non-Hispanic White, Medicare (disability).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-02-26 (Wed) | ed | Emergency Medicine |
| 1 | 2020-10-15 | 2025-11-22 (Sat) | icu | Hematology & Oncology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare (disability)",
 "occupation": "Construction foreman (currently on medical disability leave)",
 "alcohol_use": "Chronic heavy daily use, 6-10 standard drinks per day for 15 years",
 "family_history": [
  "Father: Deceased age 61, complications of alcoholism and cirrhosis",
  "Mother: Type 2 diabetes mellitus, hypertension",
  "One brother: Healthy"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 18 pack-years, quit 1 year ago",
 "home_medications": [
  {
   "dose": "20 mg PO daily",
   "name": "Omeprazole"
  },
  {
   "dose": "325 mg PO daily",
   "name": "Ferrous sulfate"
  },
  {
   "dose": "1 mg PO daily",
   "name": "Folic acid"
  },
  {
   "dose": "100 mg PO daily",
   "name": "Thiamine"
  }
 ],
 "surgical_history": [
  "Diagnostic bone marrow biopsy (2022)",
  "Upper endoscopy (2021)"
 ],
 "chronic_conditions": [
  "Myelodysplastic syndrome with multilineage dysplasia",
  "Alcohol use disorder, severe, in sustained remission attempts",
  "Chronic anemia secondary to MDS",
  "Alcohol-related liver disease without cirrhosis"
 ],
 "age_at_first_encounter": 44
}
```

Primary diagnoses: ["Hypomagnesemia", "Hypocalcemia"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2025-02-26, ed, Emergency Medicine

Chief complaint: Muscle cramps and mild perioral numbness

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Marcus Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Muscle cramps and mild perioral numbness

HISTORY OF PRESENT ILLNESS:
He denies recent vomiting or diarrhea.

PAST MEDICAL HISTORY:
Active Problem List:
- Myelodysplastic syndrome with multilineage dysplasia
- Alcohol use disorder, severe, in sustained remission attempts
- Chronic anemia secondary to MDS
- Alcohol-related liver disease without cirrhosis

PAST SURGICAL HISTORY:
- Diagnostic bone marrow biopsy (2022)
- Upper endoscopy (2021)

MEDICATIONS:
Home Medications:
- Omeprazole 20 mg PO daily
- Ferrous sulfate 325 mg PO daily
- Folic acid 1 mg PO daily
- Thiamine 100 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Deceased age 61, complications of alcoholism and cirrhosis
- Mother: Type 2 diabetes mellitus, hypertension
- One brother: Healthy

SOCIAL HISTORY:
Smoking: Former smoker, 18 pack-years, quit 1 year ago. Alcohol: Chronic heavy daily use, 6-10 standard drinks per day for 15 years. Occupation: Construction foreman (currently on medical disability leave).

PHYSICAL EXAMINATION:
His physical exam is notable for mild tremor and diffuse hyperreflexia, but no Chvostek or Trousseau signs are observed. He is alert and oriented but appears disheveled and smells of alcohol.

LABORATORY DATA:
Basic metabolic panel is unremarkable except for a serum calcium level of 7.8 mg/dL (normal range: 8.4–10.2 mg/dL).
and a repeat calcium level one hour later shows a value of 7.9 mg/dL.
The remainder of his labs, including renal and hepatic function, are pending.

OTHER STUDIES:
His ECG is normal.

PLAN:
He is administered intravenous calcium gluconate,

## Encounter 1: source 2020-10-15, practice 2025-11-22, icu, Hematology & Oncology

Chief complaint: Perioral numbness and tingling in hands and feet during massive transfusion for severe gastrointestinal bleed

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Sarah Okafor, MD
Department: Hematology & Oncology
Visit Type: icu

CHIEF COMPLAINT:
Perioral numbness and tingling in hands and feet during massive transfusion for severe gastrointestinal bleed

HISTORY OF PRESENT ILLNESS:
This 44-year-old male with myelodysplastic syndrome and chronic anemia presents with perioral numbness and tingling during massive transfusion, as in January 2020. He reports that these symptoms began shortly after his last transfusion, which was part of a series of transfusions he has received over the past 24 hours due to a severe gastrointestinal bleed. He has received a total of 8 units of packed red blood cells.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypomagnesemia (diagnosed 2020-01-15)
- Myelodysplastic syndrome with multilineage dysplasia
- Alcohol use disorder, severe, in sustained remission attempts
- Chronic anemia secondary to MDS
- Alcohol-related liver disease without cirrhosis

PAST SURGICAL HISTORY:
- Diagnostic bone marrow biopsy (2022)
- Upper endoscopy (2021)

MEDICATIONS:
He takes omeprazole and ferrous sulfate.

Home Medications:
- Folic acid 1 mg PO daily
- Thiamine 100 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Deceased age 61, complications of alcoholism and cirrhosis
- Mother: Type 2 diabetes mellitus, hypertension
- One brother: Healthy

SOCIAL HISTORY:
Smoking: Former smoker, 18 pack-years, quit 1 year ago. Alcohol: Chronic heavy daily use, 6-10 standard drinks per day for 15 years. Occupation: Construction foreman (currently on medical disability leave).

VITAL SIGNS:
His current vital signs are stable

PHYSICAL EXAMINATION:
On examination, the patient appears anxious, and Chvostek's sign is positive. There is no evidence of active bleeding.

LABORATORY DATA:
- Hemoglobin: 9.2 g/dL (normal: 13.8-17.2 g/dL)
- Platelet count: 150,000/mm³ (normal: 150,000-450,000/mm³)
- Serum calcium: 6.8 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum magnesium: 1.4 mg/dL (normal: 1.7-2.2 mg/dL)
- Serum potassium: 4.0 mEq/L (normal: 3.5-5.1 mEq/L)
- Serum creatinine: 1.0 mg/dL (normal: 0.6-1.2 mg/dL)
