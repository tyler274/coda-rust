# synthetic_hospital patient 2050

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-03-22 (Sat) | inpatient | Internal Medicine |
| 1 | 2020-04-15 | 2025-06-15 (Sun) | ed | Emergency Medicine |
| 2 | 2020-10-15 | 2025-12-13 (Sat) | icu | Intensive Care Unit |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Construction laborer, currently unemployed",
 "alcohol_use": "Occasional (1-2 drinks per month)",
 "family_history": [
  "Father: Myocardial infarction at age 58, hypertension",
  "Mother: Type 2 diabetes mellitus",
  "Brother: Opioid use disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 1 pack per day for 20 years",
 "home_medications": [
  {
   "dose": "8 mg/2 mg sublingual film daily",
   "name": "Buprenorphine/naloxone"
  },
  {
   "dose": "4 mg nasal spray as needed for overdose reversal",
   "name": "Naloxone nasal spray"
  },
  {
   "dose": "5 mg orally daily",
   "name": "Amlodipine"
  },
  {
   "dose": "50 mg orally daily",
   "name": "Sertraline"
  }
 ],
 "surgical_history": [
  "Open reduction internal fixation right femur (2015)",
  "Incision and drainage cutaneous abscess left forearm (2021)"
 ],
 "chronic_conditions": [
  "Opioid use disorder, severe, active",
  "Chronic hepatitis C infection",
  "Hypertension",
  "Major depressive disorder, recurrent, moderate"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Renal infarction", "Infective Endocarditis (Aortic Valve) with Complications"]
Comorbidities: ["Intravenous drug use (Substance use disorder)"]

## Encounter 0: source 2020-01-15, practice 2025-03-22, inpatient, Internal Medicine

Chief complaint: Two-week history of fever, fatigue, and new-onset lower back pain

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Two-week history of fever, fatigue, and new-onset lower back pain

HISTORY OF PRESENT ILLNESS:
He denies any recent trauma or heavy lifting.

PAST MEDICAL HISTORY:
Active Problem List:
- Opioid use disorder, severe, active
- Chronic hepatitis C infection
- Hypertension
- Major depressive disorder, recurrent, moderate

PAST SURGICAL HISTORY:
- Open reduction internal fixation right femur (2015)
- Incision and drainage cutaneous abscess left forearm (2021)

MEDICATIONS:
Home Medications:
- Buprenorphine/naloxone 8 mg/2 mg sublingual film daily
- Naloxone nasal spray 4 mg nasal spray as needed for overdose reversal
- Amlodipine 5 mg orally daily
- Sertraline 50 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 58, hypertension
- Mother: Type 2 diabetes mellitus
- Brother: Opioid use disorder

SOCIAL HISTORY:
History of intravenous drug use

VITAL SIGNS:
His temperature is 38.9°C, blood pressure is 110/70 mmHg, heart rate is 105 bpm, and respiratory rate is 20 breaths per minute.

PHYSICAL EXAMINATION:
Physical examination reveals a holosystolic murmur best heard at the left lower sternal border that radiates to the right sternal border. There are also tender, raised lesions on the pads of his fingers and toes.

LABORATORY DATA:
Laboratory studies show:

- Hemoglobin: 10.2 g/dL (Normocytic anemia)
- White blood cell count: 12,000/mm³
- Erythrocyte sedimentation rate (ESR): 85 mm/hr
- Blood cultures: Pending

## Encounter 1: source 2020-04-15, practice 2025-06-15, ed, Emergency Medicine

Chief complaint: Sudden onset of severe left flank pain with nausea and vomiting for the past 6 hours, associated with intermittent fevers and night sweats over two weeks

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe left flank pain with nausea and vomiting for the past 6 hours, associated with intermittent fevers and night sweats over two weeks

HISTORY OF PRESENT ILLNESS:
This 45-year-old male with severe active opioid use disorder, chronic hepatitis C, and recent hospitalization for infective endocarditis presents for emergency evaluation. He reports intermittent fevers and night sweats over the past two weeks.

PAST MEDICAL HISTORY:
Active Problem List:
- Infective Endocarditis (Aortic Valve) with Complications (diagnosed 2020-01-15)
- Opioid use disorder, severe, active
- Chronic hepatitis C infection
- Hypertension
- Major depressive disorder, recurrent, moderate

PAST SURGICAL HISTORY:
- Open reduction internal fixation right femur (2015)
- Incision and drainage cutaneous abscess left forearm (2021)

MEDICATIONS:
Home Medications:
- Buprenorphine/naloxone 8 mg/2 mg sublingual film daily
- Naloxone nasal spray 4 mg nasal spray as needed for overdose reversal
- Amlodipine 5 mg orally daily
- Sertraline 50 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 58, hypertension
- Mother: Type 2 diabetes mellitus
- Brother: Opioid use disorder

SOCIAL HISTORY:
History of intravenous drug use

VITAL SIGNS:
Temperature is 38.7°C (101.7°F), blood pressure is 145/90 mmHg, and heart rate is 102 beats per minute

PHYSICAL EXAMINATION:
A new grade 3/6 systolic murmur is appreciated at the left lower sternal border. Abdominal examination reveals left costovertebral angle tenderness.

LABORATORY DATA:
Urinalysis shows red blood cells and protein, but no casts.
Laboratory values are as follows:

- White blood cell count: 15,000/mm³
- Hemoglobin: 13.2 g/dL
- Platelet count: 250,000/mm³
- Creatinine: 1.2 mg/dL
- Blood urea nitrogen: 21 mg/dL

IMAGING:
A CT scan of the abdomen and pelvis with contrast demonstrates a wedge-shaped area of decreased enhancement in the left renal cortex.

## Encounter 2: source 2020-10-15, practice 2025-12-13, icu, Intensive Care Unit

Chief complaint: Two-week history of fever, chills, and headache with sudden development of left-sided weakness and slurred speech

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Priya Patel, MD
Department: Intensive Care Unit
Visit Type: icu

CHIEF COMPLAINT:
Two-week history of fever, chills, and headache with sudden development of left-sided weakness and slurred speech

HISTORY OF PRESENT ILLNESS:
The patient is a 45-year-old male with severe active opioid use disorder and a history of intravenous drug use who returns following prior hospitalizations for infective endocarditis and renal infarction. He presents to the ICU with a 2-week history of fever, chills, and a new-onset headache. He also reports a 10-pound weight loss over the same period. The patient suddenly develops left-sided weakness and slurred speech.

PAST MEDICAL HISTORY:
Active Problem List:
- Infective Endocarditis (Aortic Valve) with Complications (diagnosed 2020-01-15)
- Renal infarction (diagnosed 2020-04-15)
- Opioid use disorder, severe, active
- Chronic hepatitis C infection
- Hypertension
- Major depressive disorder, recurrent, moderate

PAST SURGICAL HISTORY:
- Open reduction internal fixation right femur (2015)
- Incision and drainage cutaneous abscess left forearm (2021)

MEDICATIONS:
Home Medications:
- Buprenorphine/naloxone 8 mg/2 mg sublingual film daily
- Naloxone nasal spray 4 mg nasal spray as needed for overdose reversal
- Amlodipine 5 mg orally daily
- Sertraline 50 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 58, hypertension
- Mother: Type 2 diabetes mellitus
- Brother: Opioid use disorder

SOCIAL HISTORY:
History of intravenous drug use

VITAL SIGNS:
On examination, his temperature is 38.9°C, heart rate is 110 bpm, and blood pressure is 125/70 mmHg

PHYSICAL EXAMINATION:
A heart murmur consistent with mitral regurgitation is noted. Fundoscopic examination reveals Roth spots.

LABORATORY DATA:
Blood cultures are drawn. The patient's blood cultures grow Staphylococcus aureus.

IMAGING:
An echocardiogram shows vegetations on the native mitral valve with moderate regurgitation. A non-contrast CT scan of the head shows no hemorrhage.
