# synthetic_hospital patient 2209

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-08-05 (Tue) | inpatient | Internal Medicine |
| 1 | 2020-04-15 | 2025-11-03 (Mon) | inpatient | Internal Medicine |
| 2 | 2020-10-15 | 2026-05-02 (Sat) | icu | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired postal service supervisor",
 "alcohol_use": "None",
 "family_history": [
  "Father: Myocardial infarction at age 62, Type 2 diabetes mellitus",
  "Mother: Stroke at age 70, Hypertension",
  "Sister: Chronic kidney disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 20 pack-years, quit 8 years ago",
 "home_medications": [
  {
   "dose": "12.5 mg PO BID",
   "name": "Carvedilol"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Furosemide"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "24 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "17 g PO daily as needed for constipation",
   "name": "Polyethylene glycol 3350"
  }
 ],
 "surgical_history": [
  "Coronary artery bypass grafting x3 (5 years ago)",
  "Percutaneous coronary intervention with drug-eluting stent placement",
  "Cholecystectomy"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus",
  "Hypertension",
  "Chronic heart failure with reduced ejection fraction",
  "Chronic kidney disease Stage 3b",
  "Chronic constipation"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Hyperchloremic metabolic acidosis due to excess normal saline infusion", "Hyperosmolar hyperglycemic state"]
Comorbidities: ["Dehydration", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2025-08-05, inpatient, Internal Medicine

Chief complaint: altered mental status

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Altered mental status

HISTORY OF PRESENT ILLNESS:
His family reports that he has been increasingly confused over the past two days. He has a history of poor adherence to his insulin regimen.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus
- Hypertension
- Chronic heart failure with reduced ejection fraction
- Chronic kidney disease Stage 3b
- Chronic constipation

PAST SURGICAL HISTORY:
- Coronary artery bypass grafting x3 (5 years ago)
- Percutaneous coronary intervention with drug-eluting stent placement
- Cholecystectomy

MEDICATIONS:
Home Medications:
- Carvedilol 12.5 mg PO BID
- Lisinopril 20 mg PO daily
- Furosemide 40 mg PO daily
- Amlodipine 5 mg PO daily
- Metformin 1000 mg PO BID
- Insulin glargine 24 units subcutaneous at bedtime
- Polyethylene glycol 3350 17 g PO daily as needed for constipation

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, Type 2 diabetes mellitus
- Mother: Stroke at age 70, Hypertension
- Sister: Chronic kidney disease

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 8 years ago. Alcohol: None. Occupation: Retired postal service supervisor.

VITAL SIGNS:
Temperature is 37.2°C (99°F), blood pressure is 110/70 mmHg, pulse is 100/min, and respirations are 22/min

PHYSICAL EXAMINATION:
On examination, the patient is lethargic but arousable, with dry mucous membranes and decreased skin turgor.

LABORATORY DATA:
Serum glucose: 800 mg/dL
Serum sodium: 145 mEq/L
Serum potassium: 4.5 mEq/L
Serum chloride: 105 mEq/L
Serum bicarbonate: 24 mEq/L
Blood urea nitrogen (BUN): 50 mg/dL
Serum creatinine: 2.5 mg/dL
Serum osmolality: 340 mOsm/kg
Urine ketones: Negative
Arterial blood gas (ABG): pH 7.38, pCO2 40 mmHg, HCO3- 24 mEq/L
The patient's calculated anion gap is within normal limits.

## Encounter 1: source 2020-04-15, practice 2025-11-03, inpatient, Internal Medicine

Chief complaint: progressive fatigue and shortness of breath over the past 3 days

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. James Okafor, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Progressive fatigue and shortness of breath over the past 3 days

HISTORY OF PRESENT ILLNESS:
This 68-year-old male with type 2 diabetes mellitus, chronic heart failure, and chronic kidney disease, who was hospitalized three months prior for hyperosmolar hyperglycemic state, presents with progressive fatigue and shortness of breath. He reports decreased oral intake and poor appetite during this time.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperosmolar hyperglycemic state (diagnosed 2020-01-15)
- Type 2 diabetes mellitus
- Hypertension
- Chronic heart failure with reduced ejection fraction
- Chronic kidney disease Stage 3b
- Chronic constipation

PAST SURGICAL HISTORY:
- Coronary artery bypass grafting x3 (5 years ago)
- Percutaneous coronary intervention with drug-eluting stent placement
- Cholecystectomy

MEDICATIONS:
He takes carvedilol, furosemide, lisinopril, and metformin.

Home Medications:
- Amlodipine 5 mg PO daily
- Insulin glargine 24 units subcutaneous at bedtime
- Polyethylene glycol 3350 17 g PO daily as needed for constipation

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, Type 2 diabetes mellitus
- Mother: Stroke at age 70, Hypertension
- Sister: Chronic kidney disease

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 8 years ago. Alcohol: None. Occupation: Retired postal service supervisor.

PHYSICAL EXAMINATION:
Appears mildly confused and is breathing rapidly. An S3 heart sound is auscultated, as well as a 2/6 holosystolic murmur with soft radiation to the axilla. The lungs are clear bilaterally.

LABORATORY DATA:
Serum bicarbonate concentration of 15 mEq/L, elevated serum lactate, and a low arterial pH

## Encounter 2: source 2020-10-15, practice 2026-05-02, icu, Internal Medicine

Chief complaint: progressive lethargy and decreased responsiveness over the past 36 hours

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Maria Rodriguez, MD
Department: Internal Medicine
Visit Type: icu

CHIEF COMPLAINT:
Progressive lethargy and decreased responsiveness over the past 36 hours

HISTORY OF PRESENT ILLNESS:
This 68-year-old male with type 2 diabetes mellitus and chronic kidney disease, previously hospitalized for hyperosmolar hyperglycemic state in January 2020 and hyperchloremic metabolic acidosis in April 2020, is brought to the emergency department by his home health aide due to progressive lethargy and decreased responsiveness over the past 36 hours. He recently recovered from a mild respiratory illness for which he was treated with oral antibiotics but has had a poor oral intake since then. According to the aide, he has not been taking his usual medications, including metformin and amlodipine, over the past few days.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperosmolar hyperglycemic state (diagnosed 2020-01-15)
- Hyperchloremic metabolic acidosis due to excess normal saline infusion (diagnosed 2020-04-15)
- Type 2 diabetes mellitus
- Hypertension
- Chronic heart failure with reduced ejection fraction
- Chronic kidney disease Stage 3b
- Chronic constipation

He has a history of type 2 diabetes mellitus, hypertension, and chronic constipation.

PAST SURGICAL HISTORY:
- Coronary artery bypass grafting x3 (5 years ago)
- Percutaneous coronary intervention with drug-eluting stent placement
- Cholecystectomy

MEDICATIONS:
He takes metformin and amlodipine.

Home Medications:
- Carvedilol 12.5 mg PO BID
- Lisinopril 20 mg PO daily
- Furosemide 40 mg PO daily
- Insulin glargine 24 units subcutaneous at bedtime
- Polyethylene glycol 3350 17 g PO daily as needed for constipation

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, Type 2 diabetes mellitus
- Mother: Stroke at age 70, Hypertension
- Sister: Chronic kidney disease

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 8 years ago. Alcohol: None. Occupation: Retired postal service supervisor.

VITAL SIGNS:
The patient is tachycardic with a heart rate of 108 beats per minute and hypotensive with a blood pressure of 92/56 mm Hg. His respiratory rate is 22 breaths per minute and oxygen saturation is 97% on room air. Temperature is 36.8°C (98.2°F).

PHYSICAL EXAMINATION:
On presentation, he appears somnolent and only opens his eyes to voice, with incoherent speech and withdrawal to pain. His mucous membranes are dry, and skin turgor is reduced. Lung exam is clear, and no focal neurologic deficits are present.

LABORATORY DATA:
Initial laboratory workup shows a serum glucose of 980 mg/dL and elevated serum sodium. Serum ketones are absent.

IMAGING:
A head CT is normal.
