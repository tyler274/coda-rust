# synthetic_hospital patient 2234

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-07-08 (Tue) | procedure | General Surgery |
| 1 | 2020-07-15 | 2026-01-06 (Tue) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2026-07-05 (Sun) | icu | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired construction foreman",
 "alcohol_use": "Occasional social use (1-2 drinks per week)",
 "family_history": [
  "Father: Myocardial infarction at age 72, Hypertension",
  "Mother: Type 2 Diabetes Mellitus, Chronic Kidney Disease",
  "Brother: Ischemic stroke at age 70"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 1 pack per day for 45 years",
 "home_medications": [
  {
   "dose": "5 mg daily (adjusted for INR 2.0-3.0)",
   "name": "Warfarin"
  },
  {
   "dose": "50 mg twice daily",
   "name": "Metoprolol tartrate"
  },
  {
   "dose": "20 units subcutaneously at bedtime",
   "name": "Insulin Glargine"
  },
  {
   "dose": "5 mg daily",
   "name": "Amlodipine"
  }
 ],
 "surgical_history": [
  "Cholecystectomy (age 45)",
  "Open reduction and internal fixation of left femoral fracture (age 62)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Chronic Atrial Fibrillation",
  "Chronic Kidney Disease Stage 3"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Cocaine-induced cerebral vasculopathy", "Risk of medication error due to look-alike drug confusion (LASA)", "Anticoagulation-related hemorrhage"]
Comorbidities: ["Chronic atrial fibrillation", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2025-07-08, procedure, General Surgery

Chief complaint: Scheduled for elective hernia repair

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: General Surgery
Visit Type: procedure

CHIEF COMPLAINT:
Scheduled for elective hernia repair

HISTORY OF PRESENT ILLNESS:
In the operating room, the anesthesiologist prepares for induction of anesthesia.
The surgical team requests a bolus of intravenous antihypertensive to optimize the patient&#x27;s blood pressure before the procedure. The anesthesiologist reaches for a vial but hesitates, noticing that the vials of esmolol and ephedrine are similar in appearance and were placed next to each other on the anesthesia cart.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Atrial Fibrillation
- Chronic Kidney Disease Stage 3

PAST SURGICAL HISTORY:
- Cholecystectomy (age 45)
- Open reduction and internal fixation of left femoral fracture (age 62)

MEDICATIONS:
His current medications include warfarin and metoprolol.

Home Medications:
- Metoprolol tartrate 50 mg twice daily
- Insulin Glargine 20 units subcutaneously at bedtime
- Amlodipine 5 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 72, Hypertension
- Mother: Type 2 Diabetes Mellitus, Chronic Kidney Disease
- Brother: Ischemic stroke at age 70

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 45 years. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Retired construction foreman.

VITAL SIGNS:
His blood pressure before induction is 145/90 mmHg.

LABORATORY DATA:
The patient&#x27;s baseline INR was 2.5,

## Encounter 1: source 2020-07-15, practice 2026-01-06, ed, Emergency Medicine

Chief complaint: Sudden onset right-sided weakness and difficulty speaking

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset right-sided weakness and difficulty speaking

HISTORY OF PRESENT ILLNESS:
This 68-year-old male with HTN, T2DM, chronic AF, and CKD stage 3, s/p hernia repair, presents to the emergency department with sudden onset right-sided weakness and difficulty speaking. His wife reports he was in his usual state of health until he awoke from a nap with these symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Risk of medication error due to look-alike drug confusion (LASA) (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Atrial Fibrillation
- Chronic Kidney Disease Stage 3

PAST SURGICAL HISTORY:
- Cholecystectomy (age 45)
- Open reduction and internal fixation of left femoral fracture (age 62)

MEDICATIONS:
He takes warfarin therapy.

Home Medications:
- Metoprolol tartrate 50 mg twice daily
- Insulin Glargine 20 units subcutaneously at bedtime
- Amlodipine 5 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 72, Hypertension
- Mother: Type 2 Diabetes Mellitus, Chronic Kidney Disease
- Brother: Ischemic stroke at age 70

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 45 years. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Retired construction foreman.

VITAL SIGNS:
With a blood pressure of 160/90 mmHg, heart rate of 78 beats per minute, and irregularly irregular rhythm.

PHYSICAL EXAMINATION:
On examination, the patient is alert but anxious,
Neurological examination reveals expressive aphasia and right hemiparesis.

LABORATORY DATA:
Laboratory values are as follows: - INR: 3.8 - Hemoglobin: 13.2 g/dL - Platelet count: 250,000 /μL - Creatinine: 1.1 mg/dL

IMAGING:
A non-contrast head CT scan shows a left intracerebral hemorrhage.

## Encounter 2: source 2021-01-15, practice 2026-07-05, icu, Internal Medicine

Chief complaint: Massive pulmonary embolism

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Sharma, MD
Department: Internal Medicine
Visit Type: icu

CHIEF COMPLAINT:
Massive pulmonary embolism

HISTORY OF PRESENT ILLNESS:
This 69-year-old male with chronic atrial fibrillation on warfarin and cocaine-induced cerebral vasculopathy presents with massive pulmonary embolism. He is admitted for management of massive pulmonary embolism and is started on intravenous anticoagulant. On day 2 of admission, he develops sudden onset hematuria and persistent oozing from his intravenous catheter sites.

PAST MEDICAL HISTORY:
Active Problem List:
- Risk of medication error due to look-alike drug confusion (LASA) (diagnosed 2020-01-15)
- Cocaine-induced cerebral vasculopathy (diagnosed 2020-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Atrial Fibrillation
- Chronic Kidney Disease Stage 3

PAST SURGICAL HISTORY:
- Cholecystectomy (age 45)
- Open reduction and internal fixation of left femoral fracture (age 62)

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily (adjusted for INR 2.0-3.0)
- Metoprolol tartrate 50 mg twice daily
- Insulin Glargine 20 units subcutaneously at bedtime
- Amlodipine 5 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 72, Hypertension
- Mother: Type 2 Diabetes Mellitus, Chronic Kidney Disease
- Brother: Ischemic stroke at age 70

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 45 years. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Retired construction foreman.

VITAL SIGNS:
Blood pressure is 100/58 mmHg, pulse is 88/min, and respiratory rate is 18/min

PHYSICAL EXAMINATION:
Diffuse ecchymoses over his arms and mild abdominal tenderness without distension

LABORATORY DATA:
Hemoglobin of 8.6 g/dL (baseline 12.5), platelet count of 220,000/μL, prolonged aPTT, and normal PT
