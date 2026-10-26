# synthetic_hospital patient 2859

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 72 at first encounter, sex F, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-05-07 (Wed) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2025-08-12 (Tue) | ed | Emergency Medicine |
| 2 | 2020-07-15 | 2025-11-04 (Tue) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired elementary school teacher",
 "alcohol_use": "None",
 "family_history": [
  "Mother with bipolar disorder",
  "Father with rheumatoid arthritis",
  "Sister with hypothyroidism"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 15 years ago",
 "home_medications": [
  {
   "dose": "300 mg PO BID",
   "name": "Lithium carbonate"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Prednisone"
  },
  {
   "dose": "325 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "600 mg PO BID",
   "name": "Calcium carbonate"
  },
  {
   "dose": "500 mg PO PRN pain",
   "name": "Acetaminophen"
  },
  {
   "dose": "2000 units PO daily",
   "name": "Cholecalciferol"
  }
 ],
 "surgical_history": [
  "Total knee arthroplasty (2019)",
  "Cholecystectomy (2005)"
 ],
 "chronic_conditions": [
  "Bipolar disorder",
  "Rheumatoid arthritis",
  "Osteoarthritis",
  "Chronic kidney disease stage 3",
  "Osteoporosis"
 ],
 "age_at_first_encounter": 72
}
```

Primary diagnoses: ["Severe lithium toxicity", "Acute salicylate intoxication", "Dehydration"]
Comorbidities: ["Dehydration", "Osteoarthritis"]

## Encounter 0: source 2020-01-15, practice 2025-05-07, outpatient, Internal Medicine

Chief complaint: Generalized weakness and mild confusion over the past week

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Generalized weakness and mild confusion over the past week

HISTORY OF PRESENT ILLNESS:
Over the past week

PAST MEDICAL HISTORY:
Active Problem List:
- Bipolar disorder
- Rheumatoid arthritis
- Osteoarthritis
- Chronic kidney disease stage 3
- Osteoporosis

PAST SURGICAL HISTORY:
- Total knee arthroplasty (2019)
- Cholecystectomy (2005)

MEDICATIONS:
She reports adherence to her medications, which include prednisone and calcium supplements.

Home Medications:
- Lithium carbonate 300 mg PO BID
- Aspirin 325 mg PO daily
- Calcium carbonate 600 mg PO BID
- Acetaminophen 500 mg PO PRN pain
- Cholecalciferol 2000 units PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with bipolar disorder
- Father with rheumatoid arthritis
- Sister with hypothyroidism

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago. Alcohol: None. Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
On examination, she appears lethargic but is oriented to person, place, and time. Her mucous membranes are dry, and she has mild pedal edema.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum calcium: 7.8 mg/dL (Normal: 8.5-10.2 mg/dL)
- Serum albumin: 2.5 g/dL (Normal: 3.5-5.0 g/dL)
- Serum creatinine: 2.8 mg/dL (Normal: 0.6-1.2 mg/dL)
- Blood urea nitrogen (BUN): 45 mg/dL (Normal: 7-20 mg/dL)
- Ionized calcium: 5.0 mg/dL (Normal: 4.5-5.6 mg/dL)

OTHER STUDIES:
The patient's ECG shows a normal sinus rhythm with no abnormalities. Given the patient's symptoms and laboratory findings,

## Encounter 1: source 2020-04-15, practice 2025-08-12, ed, Emergency Medicine

Chief complaint: Progressive fatigue, nausea, and persistent 'buzzing' in ears

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive fatigue, nausea, and persistent 'buzzing' in ears

HISTORY OF PRESENT ILLNESS:
This 72-year-old female with CKD returns three months after a visit for weakness and confusion, presenting with progressive fatigue, nausea, and a persistent "buzzing" in her ears that has been worsening over the past month. She also reports poor appetite and shortness of breath at rest. She denies fever, cough, or vomiting.

PAST MEDICAL HISTORY:
Active Problem List:
- Dehydration (diagnosed 2020-01-15)
- Bipolar disorder
- Rheumatoid arthritis
- Osteoarthritis
- Chronic kidney disease stage 3
- Osteoporosis

She has no known kidney or lung disease.

PAST SURGICAL HISTORY:
- Total knee arthroplasty (2019)
- Cholecystectomy (2005)

MEDICATIONS:
She takes aspirin and acetaminophen almost daily.

Home Medications:
- Lithium carbonate 300 mg PO BID
- Prednisone 5 mg PO daily
- Calcium carbonate 600 mg PO BID
- Cholecalciferol 2000 units PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with bipolar disorder
- Father with rheumatoid arthritis
- Sister with hypothyroidism

SOCIAL HISTORY:
She lives alone and reports drinking minimal water.

VITAL SIGNS:
Temperature 36.9°C (98.4°F), blood pressure 134/82 mmHg, heart rate 92/min, respiratory rate 24/min, oxygen saturation 97% on room air

PHYSICAL EXAMINATION:
She appears alert and oriented. There is no hepatosplenomegaly or edema.

LABORATORY DATA:
Na⁺ 134 mEq/L
K⁺ 4.2 mEq/L
Cl⁻ 96 mEq/L
HCO₃⁻ 19 mEq/L
PaCO₂ 30 mmHg
pH 7.37
BUN 22 mg/dL
Creatinine 1.1 mg/dL

## Encounter 2: source 2020-07-15, practice 2025-11-04, icu, Emergency Medicine

Chief complaint: Confusion, hand tremors, vomiting and diarrhea for 2 days

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Confusion, hand tremors, vomiting and diarrhea for 2 days

HISTORY OF PRESENT ILLNESS:
This 72-year-old female with bipolar disorder was previously evaluated in January for dehydration and in April for acute salicylate intoxication. She presents to the emergency department with a 2-day history of confusion, hand tremors, and repeated episodes of vomiting and diarrhea. Her daughter reports that the patient has been drinking less water than usual due to nausea.

PAST MEDICAL HISTORY:
Active Problem List:
- Dehydration (diagnosed 2020-01-15)
- Acute salicylate intoxication (diagnosed 2020-04-15)
- Bipolar disorder
- Rheumatoid arthritis
- Osteoarthritis
- Chronic kidney disease stage 3
- Osteoporosis

PAST SURGICAL HISTORY:
- Total knee arthroplasty (2019)
- Cholecystectomy (2005)

MEDICATIONS:
She takes lithium for several years, non-steroidal anti-inflammatory drug (NSAID).

Home Medications:
- Lithium carbonate 300 mg PO BID
- Prednisone 5 mg PO daily
- Aspirin 325 mg PO daily
- Calcium carbonate 600 mg PO BID
- Acetaminophen 500 mg PO PRN pain
- Cholecalciferol 2000 units PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with bipolar disorder
- Father with rheumatoid arthritis
- Sister with hypothyroidism

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago. Alcohol: None. Occupation: Retired elementary school teacher.

VITAL SIGNS:
BP 130/80 mmHg, HR 88 bpm, RR 16/min, and T 37.1°C

PHYSICAL EXAMINATION:
Patient is drowsy but arousable, has a coarse tremor in both hands, and her speech is slurred

LABORATORY DATA:
Serum lithium: 3.5 mEq/L (therapeutic range: 0.6-1.2 mEq/L)
- Serum creatinine: 1.8 mg/dL (baseline 1.0 mg/dL)
- Blood urea nitrogen (BUN): 40 mg/dL
- Serum sodium: 142 mEq/L
- Serum potassium: 4.2 mEq/L
- Serum calcium: 9.2 mg/dL
- Glucose: 110 mg/dL
