# synthetic_hospital patient 2194

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 72 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-04-16 (Wed) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2025-10-11 (Sat) | ed | Emergency Medicine |
| 2 | 2020-08-15 | 2025-11-15 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare",
 "occupation": "Retired postal worker",
 "alcohol_use": "Social, 1-2 drinks per week",
 "family_history": [
  "Father: Myocardial infarction at age 65",
  "Mother: Hypertension, chronic kidney disease",
  "Brother: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 25 pack-year history, quit 8 years ago",
 "home_medications": [
  {
   "dose": "20 mg oral daily",
   "name": "Lisinopril"
  },
  {
   "dose": "50 mg oral daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "81 mg oral daily",
   "name": "Aspirin"
  },
  {
   "dose": "40 mg oral daily at bedtime",
   "name": "Atorvastatin"
  },
  {
   "dose": "18 mcg inhaled daily",
   "name": "Tiotropium bromide"
  },
  {
   "dose": "90 mcg inhaled every 4-6 hours as needed for wheezing",
   "name": "Albuterol sulfate"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (COPD)",
  "Hypertension",
  "Coronary artery disease (stable)",
  "Chronic kidney disease, Stage 3a (baseline eGFR 48 mL/min/1.73 m\u00b2)"
 ],
 "age_at_first_encounter": 72
}
```

Primary diagnoses: ["Influenza", "Acute deep vein thrombosis of left popliteal vein", "Severe Clostridioides difficile infection without toxic megacolon"]
Comorbidities: ["History of recent pneumonia", "Essential hypertension", "Delayed management of life-threatening respiratory failure"]

## Encounter 0: source 2020-01-15, practice 2025-04-16, outpatient, Internal Medicine

Chief complaint: 3-day history of fever, malaise, myalgias, and nonproductive cough

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
3-day history of fever, malaise, myalgias, and nonproductive cough

HISTORY OF PRESENT ILLNESS:
He reports a severe headache and generalized weakness. He has not received an influenza vaccine this season.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (COPD)
- Hypertension
- Coronary artery disease (stable)
- Chronic kidney disease, Stage 3a (baseline eGFR 48 mL/min/1.73 m²)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral daily at bedtime
- Tiotropium bromide 18 mcg inhaled daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed for wheezing

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 65
- Mother: Hypertension, chronic kidney disease
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 25 pack-year history, quit 8 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired postal worker.

VITAL SIGNS:
His temperature is 38.9°C (102°F), blood pressure is 145/90 mmHg, pulse is 98/min, and respiratory rate is 22/min. Oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
Physical examination reveals mild wheezing on auscultation, but no signs of consolidation.

LABORATORY DATA:
A rapid influenza diagnostic test (RIDT) returns positive for influenza A.

## Encounter 1: source 2020-07-15, practice 2025-10-11, ed, Emergency Medicine

Chief complaint: 2-day history of swelling, redness, and aching pain in left lower leg after 14-hour international flight

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-day history of swelling, redness, and aching pain in left lower leg after 14-hour international flight

HISTORY OF PRESENT ILLNESS:
Patient with COPD, HTN, CAD, CKD 3a and January influenza presents with left leg symptoms, reporting they started gradually after returning from a 14-hour international flight while mostly seated.

PAST MEDICAL HISTORY:
Active Problem List:
- Influenza (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (COPD)
- Hypertension
- Coronary artery disease (stable)
- Chronic kidney disease, Stage 3a (baseline eGFR 48 mL/min/1.73 m²)

His past medical history is significant for hypertension, stable coronary artery disease, and mild chronic kidney disease (estimated GFR 48 mL/min/1.73 m²).

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes lisinopril, metoprolol, aspirin, and atorvastatin.

Home Medications:
- Metoprolol succinate 50 mg oral daily
- Tiotropium bromide 18 mcg inhaled daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed for wheezing

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 65
- Mother: Hypertension, chronic kidney disease
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
He is a former smoker with a 25–pack-year history and drinks alcohol socially.

VITAL SIGNS:
On examination, his temperature is 37.1°C (98.8°F), blood pressure is 132/78 mm Hg, pulse is 84/min, and respirations are 16/min.

PHYSICAL EXAMINATION:
The left calf is visibly swollen and erythematous with mild warmth and tenderness along the posterior aspect; Homans sign is positive. The right leg is normal. Distal pulses are intact bilaterally, and there is no skin ulceration. Cardiac and pulmonary examinations are unremarkable.

IMAGING:
Duplex ultrasonography of the left leg demonstrates an occlusive thrombus in the left popliteal vein.

PLAN:
The patient is admitted for anticoagulation. The team discusses whether to initiate therapy with unfractionated heparin (UFH) or low-molecular-weight heparin (LMWH). Given his stable hemodynamics and ability to receive subcutaneous injections, LMWH is strongly considered.

## Encounter 2: source 2020-08-15, practice 2025-11-15, ed, Emergency Medicine

Chief complaint: Profuse watery diarrhea beginning shortly after discharge from rehabilitation facility

ENCOUNTER NOTE
Date: 2020-08-15
Provider: Dr. Priya Sharma, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Profuse watery diarrhea beginning shortly after discharge from rehabilitation facility

HISTORY OF PRESENT ILLNESS:
The patient is a 72-year-old male with chronic obstructive pulmonary disease, hypertension, coronary artery disease, and chronic kidney disease stage 3a who received treatment for acute deep vein thrombosis one month prior. He presents to the emergency department with profuse watery diarrhea that began shortly after discharge from a rehabilitation facility.

PAST MEDICAL HISTORY:
Active Problem List:
- Influenza (diagnosed 2020-01-15)
- Acute deep vein thrombosis of left popliteal vein (diagnosed 2020-07-15)
- Chronic obstructive pulmonary disease (COPD)
- Hypertension
- Coronary artery disease (stable)
- Chronic kidney disease, Stage 3a (baseline eGFR 48 mL/min/1.73 m²)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes receiving oral vancomycin since admission.

Home Medications:
- Lisinopril 20 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral daily at bedtime
- Tiotropium bromide 18 mcg inhaled daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed for wheezing

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 65
- Mother: Hypertension, chronic kidney disease
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 25 pack-year history, quit 8 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired postal worker.

PHYSICAL EXAMINATION:
Inspect for abdominal distention

LABORATORY DATA:
Stool tested positive for C. difficile toxin
