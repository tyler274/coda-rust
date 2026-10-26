# synthetic_hospital patient 2291

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 72 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-02-18 (Tue) | outpatient | Cardiology |
| 1 | 2020-04-15 | 2025-06-02 (Mon) | outpatient | Internal Medicine |
| 2 | 2020-10-15 | 2025-11-21 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired high school mathematics teacher",
 "alcohol_use": "Social (1-2 glasses of wine weekly)",
 "family_history": [
  "Father: Atrial fibrillation, ischemic stroke at age 78",
  "Mother: Hypertension, type 2 diabetes",
  "One sibling: Coronary artery disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 20 years ago (30 pack-year history)",
 "home_medications": [
  {
   "dose": "5 mg daily (dose adjusted based on INR)",
   "name": "Warfarin"
  },
  {
   "dose": "50 mg daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "20 mg daily",
   "name": "Simvastatin"
  }
 ],
 "surgical_history": [
  "Cataract extraction with intraocular lens placement (right eye, 3 years ago)"
 ],
 "chronic_conditions": [
  "Chronic atrial fibrillation (permanent)",
  "Hypertension",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 72
}
```

Primary diagnoses: ["Supratherapeutic INR", "Paralytic ileus", "Anticoagulation-related hemorrhage"]
Comorbidities: ["Chronic atrial fibrillation", "Long-term anticoagulant therapy"]

## Encounter 0: source 2020-01-15, practice 2025-02-18, outpatient, Cardiology

Chief complaint: Routine follow-up for chronic atrial fibrillation; asymptomatic but reports recent acetaminophen use for headache

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for chronic atrial fibrillation; asymptomatic but reports recent acetaminophen use for headache

HISTORY OF PRESENT ILLNESS:
He reports no new symptoms but mentions that he has been taking over-the-counter acetaminophen for the past week due to a mild headache. His diet has remained consistent, and he denies any recent antibiotic use or changes in his medication regimen.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic atrial fibrillation (permanent)
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement (right eye, 3 years ago)

MEDICATIONS:
His current medications include warfarin, metoprolol, and simvastatin.

Home Medications:
- Metoprolol succinate 50 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Atrial fibrillation, ischemic stroke at age 78
- Mother: Hypertension, type 2 diabetes
- One sibling: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (30 pack-year history). Alcohol: Social (1-2 glasses of wine weekly). Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
His vital signs are stable

PHYSICAL EXAMINATION:
On physical examination, no abnormalities are noted

LABORATORY DATA:
Laboratory values are as follows:

- INR: 3.8 (target range 2-3)
- PT: 21 seconds (control 11-13.5 seconds)
- aPTT: 30 seconds (control 30-40 seconds)
- Hemoglobin: 14 g/dL
- Platelet count: 250 x 10^9/L

## Encounter 1: source 2020-04-15, practice 2025-06-02, outpatient, Internal Medicine

Chief complaint: Scheduled routine follow-up visit; denies new symptoms

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Kwame Okafor, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Scheduled routine follow-up visit; denies new symptoms

HISTORY OF PRESENT ILLNESS:
Patient returns for follow-up of atrial fibrillation after supratherapeutic INR. He reports no new symptoms and is compliant with his medications. His INR is typically maintained between 2.0 and 3.0.

PAST MEDICAL HISTORY:
Active Problem List:
- Supratherapeutic INR (diagnosed 2020-01-15)
- Chronic atrial fibrillation (permanent)
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement (right eye, 3 years ago)

MEDICATIONS:
He has been on warfarin therapy for stroke prevention for the past 5 years.

Home Medications:
- Metoprolol succinate 50 mg daily
- Simvastatin 20 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Atrial fibrillation, ischemic stroke at age 78
- Mother: Hypertension, type 2 diabetes
- One sibling: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (30 pack-year history). Alcohol: Social (1-2 glasses of wine weekly). Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
On examination, his vital signs are stable,

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

LABORATORY DATA:
However, his recent blood work shows the following:

- Hemoglobin: 13.2 g/dL (Normal: 13.8-17.2 g/dL)
- Platelet count: 250,000/µL (Normal: 150,000-450,000/µL)
- INR: 4.5
- Partial thromboplastin time (PTT): 35 seconds (Normal: 25-35 seconds)
- Prothrombin time (PT): 42 seconds (Normal: 11-13.5 seconds)

ASSESSMENT AND PLAN:
Given the patient's elevated INR,

## Encounter 2: source 2020-10-15, practice 2025-11-21, ed, Emergency Medicine

Chief complaint: Nausea, vomiting, and abdominal distension for 2 days without passage of stool or flatus

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Elena Vasquez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Nausea, vomiting, and abdominal distension for 2 days without passage of stool or flatus

HISTORY OF PRESENT ILLNESS:
This 72-year-old male with permanent atrial fibrillation on warfarin, with history of supratherapeutic INR and anticoagulation-related hemorrhage, presents to the emergency department with a 2-day history of nausea, vomiting, and abdominal distension. He reports no passage of stool or flatus for the past 48 hours.

PAST MEDICAL HISTORY:
Active Problem List:
- Supratherapeutic INR (diagnosed 2020-01-15)
- Anticoagulation-related hemorrhage (diagnosed 2020-04-15)
- Chronic atrial fibrillation (permanent)
- Hypertension
- Hyperlipidemia

His medical history is significant for chronic atrial fibrillation and a recent episode of acute pancreatitis one month ago.

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement (right eye, 3 years ago)

MEDICATIONS:
He takes warfarin.

Home Medications:
- Metoprolol succinate 50 mg daily
- Simvastatin 20 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Atrial fibrillation, ischemic stroke at age 78
- Mother: Hypertension, type 2 diabetes
- One sibling: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (30 pack-year history). Alcohol: Social (1-2 glasses of wine weekly). Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
His vital signs are stable.

PHYSICAL EXAMINATION:
On examination, his abdomen is distended and tympanic to percussion, with hypoactive bowel sounds.

LABORATORY DATA:
Hemoglobin: 13.2 g/dL (normal: 13.8-17.2 g/dL)
- Hematocrit: 39.5% (normal: 40.7-50.3%)
- Platelets: 225 x 10^3/µL (normal: 150-450 x 10^3/µL)
- INR: 3.2 (normal: 0.8-1.1, therapeutic range for warfarin: 2-3)
- Serum potassium: 3.5 mEq/L (normal: 3.5-5.0 mEq/L)
- Serum creatinine: 1.0 mg/dL (normal: 0.6-1.2 mg/dL)
- Amylase: 90 U/L (normal: 30-110 U/L)
- Lipase: 60 U/L (normal: 0-160 U/L)

IMAGING:
An abdominal radiograph shows uniformly distended, gas-filled loops of both the small and large intestines without air-fluid levels.
