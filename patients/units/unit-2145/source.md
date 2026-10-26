# synthetic_hospital patient 2145

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 43 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-05-13 (Mon) | ed | Emergency Medicine |
| 1 | 2022-01-15 | 2026-05-18 (Mon) | outpatient | Hematology & Oncology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Warehouse logistics coordinator",
 "alcohol_use": "None",
 "family_history": [
  "Father: Alcohol use disorder, cirrhosis",
  "Mother: Hypertension, hypothyroidism",
  "Younger brother: Hepatitis C infection"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 5 years ago",
 "home_medications": [
  {
   "dose": "50 mg PO daily",
   "name": "Naltrexone"
  },
  {
   "dose": "100 mg PO daily",
   "name": "Sertraline"
  },
  {
   "dose": "2000 units PO daily",
   "name": "Vitamin D3"
  }
 ],
 "surgical_history": [
  "Open reduction and internal fixation of left femur (motor vehicle accident, age 28)",
  "Appendectomy (age 16)"
 ],
 "chronic_conditions": [
  "History of hepatitis C infection",
  "Opioid use disorder, sustained remission",
  "Post-traumatic stress disorder"
 ],
 "age_at_first_encounter": 43
}
```

Primary diagnoses: ["Hepatitis C virus reinfection", "Pancytopenia"]
Comorbidities: ["History of intravenous drug use"]

## Encounter 0: source 2020-01-15, practice 2024-05-13, ed, Emergency Medicine

Chief complaint: 2-week history of fatigue, jaundice, and dark urine

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-week history of fatigue, jaundice, and dark urine

PAST MEDICAL HISTORY:
Active Problem List:
- History of hepatitis C infection
- Opioid use disorder, sustained remission
- Post-traumatic stress disorder

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of left femur (motor vehicle accident, age 28)
- Appendectomy (age 16)

MEDICATIONS:
Home Medications:
- Naltrexone 50 mg PO daily
- Sertraline 100 mg PO daily
- Vitamin D3 2000 units PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder, cirrhosis
- Mother: Hypertension, hypothyroidism
- Younger brother: Hepatitis C infection

SOCIAL HISTORY:
He reports no recent travel, alcohol use, or over-the-counter medication use. He denies any current drug use and states that he has been attending a substance abuse program regularly.

PHYSICAL EXAMINATION:
On physical examination, he is afebrile, with scleral icterus and mild right upper quadrant tenderness.

LABORATORY DATA:
His liver function tests are as follows:

- AST: 120 U/L (Normal: 10-40 U/L)
- ALT: 130 U/L (Normal: 7-56 U/L)
- Alkaline phosphatase: 210 U/L (Normal: 40-130 U/L)
- Total bilirubin: 3.2 mg/dL (Normal: 0.1-1.2 mg/dL)
- Direct bilirubin: 2.1 mg/dL (Normal: 0.0-0.3 mg/dL)

## Encounter 1: source 2022-01-15, practice 2026-05-18, outpatient, Hematology & Oncology

Chief complaint: Increasing fatigue, easy bruising, and frequent nosebleeds

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Marcus Johnson, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing fatigue, easy bruising, and frequent nosebleeds

HISTORY OF PRESENT ILLNESS:
This 45-year-old male with a history of hepatitis C infection, previously evaluated in the emergency department in 2020 for reinfection, presents to the clinic with a 2-month history of increasing fatigue, easy bruising, and frequent nosebleeds. He denies any fever, night sweats, or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Hepatitis C virus reinfection (diagnosed 2020-01-15)
- History of hepatitis C infection
- Opioid use disorder, sustained remission
- Post-traumatic stress disorder

His past medical history is significant for hepatitis C, which was successfully treated with antiviral therapy 5 years ago.

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of left femur (motor vehicle accident, age 28)
- Appendectomy (age 16)

MEDICATIONS:
Home Medications:
- Naltrexone 50 mg PO daily
- Sertraline 100 mg PO daily
- Vitamin D3 2000 units PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder, cirrhosis
- Mother: Hypertension, hypothyroidism
- Younger brother: Hepatitis C infection

SOCIAL HISTORY:
Smoking: Former smoker, quit 5 years ago. Alcohol: None. Occupation: Warehouse logistics coordinator.

PHYSICAL EXAMINATION:
On physical examination, he appears pale and has scattered petechiae on his lower extremities. There is no lymphadenopathy or hepatosplenomegaly.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 8.2 g/dL (Normal: 13.8-17.2 g/dL)
- White blood cell count: 2.1 x 10^9/L (Normal: 4.5-11.0 x 10^9/L)
- Platelets: 20 x 10^9/L (Normal: 150-450 x 10^9/L)
- Reticulocyte count: 0.5% (Normal: 0.5-2.5%)
- Peripheral blood smear: No abnormal cells or blasts, decreased cellularity
