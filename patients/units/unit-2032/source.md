# synthetic_hospital patient 2032

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 56 at first encounter, sex M, African American, Blue Cross Blue Shield PPO (employer-sponsored).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-01-30 (Thu) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2025-07-19 (Sat) | inpatient | Gastroenterology |
| 2 | 2021-01-15 | 2026-01-24 (Sat) | icu | Critical Care Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO (employer-sponsored)",
 "occupation": "Construction site supervisor",
 "alcohol_use": "Heavy daily consumption, approximately 6-8 standard drinks per day for 20+ years",
 "family_history": [
  "Father: Hypertension, alcohol use disorder",
  "Mother: Type 2 diabetes mellitus",
  "Brother: Gout"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Former smoker, quit 10 years ago, 20 pack-years",
 "home_medications": [
  {
   "dose": "40 mg orally daily",
   "name": "Furosemide"
  },
  {
   "dose": "100 mg orally daily",
   "name": "Spironolactone"
  },
  {
   "dose": "5 mg orally daily",
   "name": "Amlodipine"
  }
 ],
 "surgical_history": [
  "Laparoscopic appendectomy (age 24)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Alcohol use disorder",
  "Cirrhosis"
 ],
 "age_at_first_encounter": 56
}
```

Primary diagnoses: ["Gout", "Right-sided pleural effusion", "Hepatorenal syndrome"]
Comorbidities: ["Alcohol consumption", "Ascites"]

## Encounter 0: source 2020-01-15, practice 2025-01-30, ed, Emergency Medicine

Chief complaint: Acute left knee pain and swelling

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute left knee pain and swelling

HISTORY OF PRESENT ILLNESS:
That began overnight. He reports two prior episodes affecting the same knee over the last three years, both of which resolved without intervention.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Alcohol use disorder
- Cirrhosis

His medical history is notable for hypertension.

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 24)

MEDICATIONS:
Home Medications:
- Furosemide 40 mg orally daily
- Spironolactone 100 mg orally daily
- Amlodipine 5 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, alcohol use disorder
- Mother: Type 2 diabetes mellitus
- Brother: Gout

SOCIAL HISTORY:
Daily alcohol consumption.

VITAL SIGNS:
Vital signs are within normal limits except for a temperature of 37.5°C (99.5°F).

PHYSICAL EXAMINATION:
On examination, the left knee is swollen, erythematous, warm, and tender with limited range of motion.

LABORATORY DATA:
Synovial fluid is obtained for analysis.

## Encounter 1: source 2020-07-15, practice 2025-07-19, inpatient, Gastroenterology

Chief complaint: Progressive shortness of breath over the past two weeks

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Gastroenterology
Visit Type: inpatient

CHIEF COMPLAINT:
Progressive shortness of breath over the past two weeks

HISTORY OF PRESENT ILLNESS:
The patient is a 56-year-old male with cirrhosis, hypertension, and alcohol use disorder who presents with progressive shortness of breath over the past two weeks. He was previously evaluated in the emergency department in January 2020 for acute left knee pain. He denies chest pain, fever, or cough.

PAST MEDICAL HISTORY:
Active Problem List:
- Gout (diagnosed 2020-01-15)
- Hypertension
- Alcohol use disorder
- Cirrhosis

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 24)

MEDICATIONS:
He takes furosemide and spironolactone.

Home Medications:
- Amlodipine 5 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, alcohol use disorder
- Mother: Type 2 diabetes mellitus
- Brother: Gout

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago, 20 pack-years. Alcohol: Heavy daily consumption, approximately 6-8 standard drinks per day for 20+ years. Occupation: Construction site supervisor.

VITAL SIGNS:
Afebrile, blood pressure is 130/85 mmHg, heart rate is 88 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 92% on room air

PHYSICAL EXAMINATION:
Chest auscultation reveals decreased breath sounds at the right base without wheezes or crackles. Abdominal examination shows a fluid wave consistent with ascites. There is no peripheral edema.

LABORATORY DATA:
Thoracentesis yields a clear, straw-colored fluid with the following analysis:

- Protein (pleural): 1.8 g/dL
- Protein (serum): 6.0 g/dL
- LDH (pleural): 110 U/L
- LDH (serum): 200 U/L
- WBC count (pleural): 150 cells/µL with 80% lymphocytes

IMAGING:
Chest X-ray demonstrates a right-sided pleural effusion.

## Encounter 2: source 2021-01-15, practice 2026-01-24, icu, Critical Care Medicine

Chief complaint: Worsening abdominal distension and confusion

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Okafor, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Worsening abdominal distension and confusion

HISTORY OF PRESENT ILLNESS:
This 57-year-old male with a history of cirrhosis, alcohol use disorder, and hypertension, who required hospitalization six months ago for a right-sided pleural effusion, presents to the emergency department with a two-day history of worsening abdominal distension and confusion. His wife reports that he has been increasingly lethargic and has not been eating well for the past week. The patient's renal function does not improve after receiving intravenous isotonic saline.

PAST MEDICAL HISTORY:
Active Problem List:
- Gout (diagnosed 2020-01-15)
- Right-sided pleural effusion (diagnosed 2020-07-15)
- Hypertension
- Alcohol use disorder
- Cirrhosis

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 24)

MEDICATIONS:
Home Medications:
- Furosemide 40 mg orally daily
- Spironolactone 100 mg orally daily
- Amlodipine 5 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, alcohol use disorder
- Mother: Type 2 diabetes mellitus
- Brother: Gout

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago, 20 pack-years. Alcohol: Heavy daily consumption, approximately 6-8 standard drinks per day for 20+ years. Occupation: Construction site supervisor.

VITAL SIGNS:
His blood pressure is 90/60 mmHg, heart rate is 102 beats per minute, and respiratory rate is 20 breaths per minute.

PHYSICAL EXAMINATION:
On examination, the patient is jaundiced, disoriented, and has asterixis. His abdomen is distended with a fluid wave consistent with ascites. There is no peripheral edema.

LABORATORY DATA:
Initial laboratory tests reveal:

- Serum creatinine: 2.1 mg/dL
- Blood urea nitrogen (BUN): 42 mg/dL
- Serum sodium: 130 mEq/L
- Urine sodium: 5 mEq/L
- Urine osmolality: 500 mOsm/kg
- Urine analysis: no blood, no casts, no protein
