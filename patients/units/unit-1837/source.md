# synthetic_hospital patient 1837

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-06-03 (Tue) | ed | Emergency Medicine |
| 1 | 2020-05-15 | 2025-09-25 (Thu) | ed | Obstetrics & Gynecology |
| 2 | 2020-08-15 | 2025-12-30 (Tue) | icu | General Surgery |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "High school mathematics teacher",
 "alcohol_use": "Never",
 "family_history": [
  "Father: Severe hypertriglyceridemia",
  "Paternal grandfather: Acute pancreatitis",
  "Mother: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "145 mg PO daily",
   "name": "Fenofibrate"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Recurrent urinary tract infections",
  "Severe hypertriglyceridemia"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Acute pyelonephritis in pregnancy", "Acute pyelonephritis", "Infected pancreatic necrosis"]
Comorbidities: ["History of septic shock", "Pregnancy"]

## Encounter 0: source 2020-01-15, practice 2025-06-03, ed, Emergency Medicine

Chief complaint: Five-day history of fever, chills, and right-sided flank pain with dysuria and frequency

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Five-day history of fever, chills, and right-sided flank pain with dysuria and frequency

HISTORY OF PRESENT ILLNESS:
She reports dysuria and frequency but denies nausea, vomiting, or diarrhea.

PAST MEDICAL HISTORY:
Active Problem List:
- Recurrent urinary tract infections
- Severe hypertriglyceridemia

Her past medical history is significant for recurrent urinary tract infections.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Fenofibrate 145 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Severe hypertriglyceridemia
- Paternal grandfather: Acute pancreatitis
- Mother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never. Alcohol: Never. Occupation: High school mathematics teacher.

VITAL SIGNS:
On examination, she is febrile with a temperature of 38.9°C (102°F), heart rate is 110/min, and blood pressure is 130/85 mmHg.

PHYSICAL EXAMINATION:
Costovertebral angle tenderness is noted on the right side.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 15,000/mm³ (Normal: 4,500-11,000/mm³)
- Hemoglobin: 13.2 g/dL (Normal: 12-16 g/dL)
- Platelets: 250,000/mm³ (Normal: 150,000-450,000/mm³)
- Urinalysis: Pyuria, positive leukocyte esterase, and nitrites
- Blood cultures: No growth to date
- Urine cultures: Pending

ASSESSMENT AND PLAN:
Despite 72 hours of antibiotic therapy, her fever persists, and her flank pain has not improved.

She was admitted to the hospital and started on intravenous ceftriaxone.

## Encounter 1: source 2020-05-15, practice 2025-09-25, ed, Obstetrics & Gynecology

Chief complaint: Two-day history of fever, chills, and right-sided flank pain radiating to lower abdomen at 30 weeks gestation

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Priya Sharma, MD
Department: Obstetrics & Gynecology
Visit Type: ed

CHIEF COMPLAINT:
Two-day history of fever, chills, and right-sided flank pain radiating to lower abdomen at 30 weeks gestation

HISTORY OF PRESENT ILLNESS:
The patient is a 28-year-old woman with recurrent urinary tract infections and severe hypertriglyceridemia on fenofibrate therapy who was treated for acute pyelonephritis in January 2020 and now returns to the emergency department at 30 weeks gestation with a 2-day history of fever, chills, and right-sided flank pain. She reports that the pain is constant and radiates to the lower abdomen. She has experienced nausea but no vomiting.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute pyelonephritis (diagnosed 2020-01-15)
- Recurrent urinary tract infections
- Severe hypertriglyceridemia

Her past medical history is significant for recurrent urinary tract infections.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Fenofibrate 145 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Severe hypertriglyceridemia
- Paternal grandfather: Acute pancreatitis
- Mother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never. Alcohol: Never. Occupation: High school mathematics teacher.

VITAL SIGNS:
Her temperature is 101.5°F (38.6°C), blood pressure is 120/78 mmHg, heart rate is 110 bpm, and fetal heart rate is 160 bpm.

PHYSICAL EXAMINATION:
Physical examination reveals right costovertebral angle tenderness.

LABORATORY DATA:
Urinalysis shows pyuria, bacteriuria, and hematuria. A complete blood count reveals leukocytosis with a left shift.

## Encounter 2: source 2020-08-15, practice 2025-12-30, icu, General Surgery

Chief complaint: Severe constant epigastric pain radiating to the back worsening over 24 hours

ENCOUNTER NOTE
Date: 2020-08-15
Provider: Dr. Elena Vasquez, MD
Department: General Surgery
Visit Type: icu

CHIEF COMPLAINT:
Severe constant epigastric pain radiating to the back worsening over 24 hours

HISTORY OF PRESENT ILLNESS:
Patient with recurrent UTIs and severe hypertriglyceridemia presents with pain. She describes the pain as constant and worsening over the past 24 hours. She denies any nausea, vomiting, or fever.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute pyelonephritis (diagnosed 2020-01-15)
- Acute pyelonephritis in pregnancy (diagnosed 2020-05-15)
- Recurrent urinary tract infections
- Severe hypertriglyceridemia

Her past medical history is significant for hypertriglyceridemia, for which she was taking fenofibrate but discontinued it after confirming her pregnancy.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Fenofibrate 145 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Severe hypertriglyceridemia
- Paternal grandfather: Acute pancreatitis
- Mother: Type 2 diabetes mellitus

SOCIAL HISTORY:
She has no history of alcohol use.

VITAL SIGNS:
On examination, her vital signs are stable

PHYSICAL EXAMINATION:
Her abdomen is soft but tender in the epigastric region without rebound or guarding. Bowel sounds are normal. There is no jaundice or scleral icterus.

LABORATORY DATA:
Laboratory tests reveal the following:
- Hemoglobin: 12.5 g/dL
- White blood cell count: 11,000/mm^3
- Platelets: 250,000/mm^3
- Serum amylase: 450 U/L (Normal: 30-110 U/L)
- Serum lipase: 600 U/L (Normal: 0-160 U/L)
- Serum triglycerides: 1,200 mg/dL (Normal: <150 mg/dL)
- Serum calcium: 9.2 mg/dL
- Liver function tests: Within normal limits

IMAGING:
An ultrasound of the abdomen shows no gallstones or biliary ductal dilatation.

ASSESSMENT AND PLAN:
Based on the clinical presentation and laboratory findings,
