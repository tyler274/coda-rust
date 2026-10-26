# synthetic_hospital patient 2119

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 56 at first encounter, sex M, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-05-27 (Mon) | outpatient | Neurology |
| 1 | 2021-07-15 | 2025-11-22 (Sat) | ed | Emergency Medicine |
| 2 | 2021-09-15 | 2026-01-14 (Wed) | outpatient | Cardiology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Industrial machinery mechanic",
 "alcohol_use": "None",
 "family_history": [
  "Father: Bicuspid aortic valve, coronary artery disease",
  "Mother: Atrial fibrillation, stroke at age 72",
  "Brother: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 8 years ago",
 "home_medications": [
  {
   "dose": "Variable dose (target INR 2.0-3.0)",
   "name": "Warfarin"
  },
  {
   "dose": "100 mg daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "10 mg daily",
   "name": "Lisinopril"
  }
 ],
 "surgical_history": [
  "Surgical repair of deep laceration to left upper arm with neurovascular bundle reconstruction (6 months prior)",
  "Dental scaling and root planing (1 month prior)"
 ],
 "chronic_conditions": [
  "Bicuspid aortic valve",
  "Atrial fibrillation",
  "Hypertension"
 ],
 "age_at_first_encounter": 56
}
```

Primary diagnoses: ["Infective Endocarditis (Aortic Valve) with Complications", "Peripheral nerve injury with successful reinnervation", "Coagulopathy due to vitamin K deficiency"]
Comorbidities: ["Status post laparoscopic cholecystectomy"]

## Encounter 0: source 2020-01-15, practice 2024-05-27, outpatient, Neurology

Chief complaint: Routine neurology clinic evaluation of left upper arm recovery following workplace injury with neurovascular bundle reconstruction.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine neurology clinic evaluation of left upper arm recovery following workplace injury with neurovascular bundle reconstruction.

HISTORY OF PRESENT ILLNESS:
Six months ago, he sustained a deep laceration to the upper arm that severed the neurovascular bundle. The wound was surgically repaired the same day. Since then, he has regained full motor strength and sensation in the affected arm and reports no functional limitations.

PAST MEDICAL HISTORY:
Active Problem List:
- Bicuspid aortic valve
- Atrial fibrillation
- Hypertension

PAST SURGICAL HISTORY:
- Surgical repair of deep laceration to left upper arm with neurovascular bundle reconstruction (6 months prior)
- Dental scaling and root planing (1 month prior)

MEDICATIONS:
Home Medications:
- Warfarin Variable dose (target INR 2.0-3.0)
- Metoprolol succinate 100 mg daily
- Lisinopril 10 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Bicuspid aortic valve, coronary artery disease
- Mother: Atrial fibrillation, stroke at age 72
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago. Alcohol: None. Occupation: Industrial machinery mechanic.

PHYSICAL EXAMINATION:
Physical examination shows normal bulk, tone, and strength in all muscle groups, with intact light touch, vibration, and pain sensation. Deep tendon reflexes are symmetric and normal bilaterally. There is no fasciculation, spasticity, or atrophy.

OTHER STUDIES:
Electromyography reveals reinnervation of affected muscles by neighboring motor units.

## Encounter 1: source 2021-07-15, practice 2025-11-22, ed, Emergency Medicine

Chief complaint: Three weeks of progressive fatigue, low-grade fevers, and unintentional weight loss in a patient with a bicuspid aortic valve.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Three weeks of progressive fatigue, low-grade fevers, and unintentional weight loss in a patient with a bicuspid aortic valve.

HISTORY OF PRESENT ILLNESS:
This 57-year-old male with bicuspid aortic valve, atrial fibrillation, and hypertension presents for evaluation. He has a history of a bicuspid aortic valve and underwent dental scaling and root planing about one month ago. He did not receive any prophylactic antibiotics before the procedure.

PAST MEDICAL HISTORY:
Active Problem List:
- Peripheral nerve injury with successful reinnervation (diagnosed 2020-01-15)
- Bicuspid aortic valve
- Atrial fibrillation
- Hypertension

PAST SURGICAL HISTORY:
- Surgical repair of deep laceration to left upper arm with neurovascular bundle reconstruction (6 months prior)
- Dental scaling and root planing (1 month prior)

MEDICATIONS:
Home Medications:
- Warfarin Variable dose (target INR 2.0-3.0)
- Metoprolol succinate 100 mg daily
- Lisinopril 10 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Bicuspid aortic valve, coronary artery disease
- Mother: Atrial fibrillation, stroke at age 72
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago. Alcohol: None. Occupation: Industrial machinery mechanic.

VITAL SIGNS:
His temperature is 38.0°C (100.4°F)

PHYSICAL EXAMINATION:
He has a soft systolic murmur at the right upper sternal border, which he reports is longstanding. Mild splenomegaly is noted. There are no petechiae, Janeway lesions, or Osler nodes.

LABORATORY DATA:
Blood cultures grow Strep sanguinis

IMAGING:
Transthoracic echocardiogram reveals a small vegetation on the aortic valve

## Encounter 2: source 2021-09-15, practice 2026-01-14, outpatient, Cardiology

Chief complaint: Follow-up evaluation after completing 8 weeks of intravenous antibiotic therapy for Enterococcus faecalis infective endocarditis, now with elevated INR.

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Priya Patel, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up evaluation after completing 8 weeks of intravenous antibiotic therapy for Enterococcus faecalis infective endocarditis, now with elevated INR.

HISTORY OF PRESENT ILLNESS:
This 57-year-old male with a history of bicuspid aortic valve and atrial fibrillation on chronic anticoagulation returns for follow-up evaluation after completing an 8-week course of intravenous antibiotic therapy for Enterococcus faecalis infective endocarditis, now presenting with concerns regarding elevated INR. He was initially managed in the hospital with a prolonged course of ampicillin and ceftriaxone, followed by outpatient parenteral therapy through a peripherally inserted central catheter (PICC). He has also been on chronic warfarin therapy for atrial fibrillation, and his warfarin dose had been stable for several months prior to this illness. However, over the past three weeks, his INR has repeatedly increased despite dose reductions, and he recently experienced gum bleeding while brushing his teeth. He denies any new medications, changes in diet, or herbal supplement use. He consumes a regular diet but has had mild nausea and decreased appetite.

PAST MEDICAL HISTORY:
Active Problem List:
- Peripheral nerve injury with successful reinnervation (diagnosed 2020-01-15)
- Infective Endocarditis (Aortic Valve) with Complications (diagnosed 2021-07-15)
- Bicuspid aortic valve
- Atrial fibrillation
- Hypertension

PAST SURGICAL HISTORY:
- Surgical repair of deep laceration to left upper arm with neurovascular bundle reconstruction (6 months prior)
- Dental scaling and root planing (1 month prior)

MEDICATIONS:
Home Medications:
- Warfarin Variable dose (target INR 2.0-3.0)
- Metoprolol succinate 100 mg daily
- Lisinopril 10 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Bicuspid aortic valve, coronary artery disease
- Mother: Atrial fibrillation, stroke at age 72
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago. Alcohol: None. Occupation: Industrial machinery mechanic.

VITAL SIGNS:
His vital signs are normal.

PHYSICAL EXAMINATION:
Physical examination is unremarkable aside from scattered ecchymoses on his forearms.

LABORATORY DATA:
His platelet count is normal. Repeat INR is 5.1.
