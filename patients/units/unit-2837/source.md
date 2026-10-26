# synthetic_hospital patient 2837

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 72 at first encounter, sex F, White, non-Hispanic, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-05-09 (Fri) | ed | Emergency Medicine |
| 1 | 2020-05-15 | 2025-09-18 (Thu) | ed | Emergency Medicine |
| 2 | 2020-11-15 | 2026-03-18 (Wed) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired elementary school teacher",
 "alcohol_use": "Social, 1-2 glasses of wine weekly",
 "family_history": [
  "Father: Myocardial infarction, age 68",
  "Mother: Ischemic stroke, age 75",
  "Sister: Hypertension"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Former smoker, 40 pack-years, quit 5 years ago",
 "home_medications": [
  {
   "dose": "5 mg daily",
   "name": "Warfarin"
  },
  {
   "dose": "50 mg twice daily",
   "name": "Metoprolol"
  },
  {
   "dose": "10 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "90 mcg, 2 puffs every 4-6 hours as needed",
   "name": "Albuterol HFA inhaler"
  },
  {
   "dose": "18 mcg inhaled daily",
   "name": "Tiotropium"
  }
 ],
 "surgical_history": [
  "Total abdominal hysterectomy, age 45",
  "Laparoscopic cholecystectomy, age 62"
 ],
 "chronic_conditions": [
  "Atrial fibrillation",
  "Hypertension",
  "Chronic obstructive pulmonary disease"
 ],
 "age_at_first_encounter": 72
}
```

Primary diagnoses: ["Acute mesenteric ischemia", "Contusion of left flank"]
Comorbidities: ["Atrial fibrillation", "Lactic acidosis", "Long-term anticoagulant therapy", "Sinus tachycardia secondary to burns", "Essential hypertension", "Acute posthemorrhagic anemia"]

## Encounter 0: source 2020-01-15, practice 2025-05-09, ed, Emergency Medicine

Chief complaint: Sudden onset of severe, sharp pain in the left lower abdomen that began 2 hours ago after a vigorous coughing episode.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe, sharp pain in the left lower abdomen that began 2 hours ago after a vigorous coughing episode.

HISTORY OF PRESENT ILLNESS:
That began 2 hours ago after a vigorous coughing episode.

PAST MEDICAL HISTORY:
Active Problem List:
- Atrial fibrillation
- Hypertension
- Chronic obstructive pulmonary disease

She has a history of chronic obstructive pulmonary disease (COPD) and has been on warfarin for atrial fibrillation.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy, age 45
- Laparoscopic cholecystectomy, age 62

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily
- Metoprolol 50 mg twice daily
- Lisinopril 10 mg daily
- Albuterol HFA inhaler 90 mcg, 2 puffs every 4-6 hours as needed
- Tiotropium 18 mcg inhaled daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction, age 68
- Mother: Ischemic stroke, age 75
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 5 years ago. Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Retired elementary school teacher.

VITAL SIGNS:
Her vital signs are as follows: blood pressure 100/60 mmHg, heart rate 110 beats per minute, respiratory rate 22 breaths per minute, and temperature 37.2°C.

PHYSICAL EXAMINATION:
On examination, her abdomen is distended with a palpable, tender mass in the right lower quadrant. Her skin is cool and clammy.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 10.2 g/dL (baseline 13.5 g/dL)
- Hematocrit: 30.5%
- White blood cell count: 14,000/mm³
- INR: 3.5

## Encounter 1: source 2020-05-15, practice 2025-09-18, ed, Emergency Medicine

Chief complaint: Six-hour history of sudden-onset, severe, diffuse cramping abdominal pain that is out of proportion to examination findings.

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Six-hour history of sudden-onset, severe, diffuse cramping abdominal pain that is out of proportion to examination findings.

HISTORY OF PRESENT ILLNESS:
The patient with atrial fibrillation on warfarin returns four months after her prior flank contusion. She describes the pain as diffuse and cramping, and it is out of proportion to the mild tenderness noted on examination. She denies any nausea, vomiting, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Contusion of left flank (diagnosed 2020-01-15)
- Atrial fibrillation
- Hypertension
- Chronic obstructive pulmonary disease

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy, age 45
- Laparoscopic cholecystectomy, age 62

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily
- Metoprolol 50 mg twice daily
- Lisinopril 10 mg daily
- Albuterol HFA inhaler 90 mcg, 2 puffs every 4-6 hours as needed
- Tiotropium 18 mcg inhaled daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction, age 68
- Mother: Ischemic stroke, age 75
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 5 years ago. Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Retired elementary school teacher.

VITAL SIGNS:
Her vital signs are as follows: blood pressure 150/90 mmHg, heart rate 110 bpm, respiratory rate 20 breaths/min, and temperature 37.2°C (99.0°F).

PHYSICAL EXAMINATION:
Her abdomen is soft with mild diffuse tenderness but no rebound or guarding. Bowel sounds are diminished.

LABORATORY DATA:
Laboratory studies show:

- White blood cell count: 15,000/mm³
- Hemoglobin: 12 g/dL
- Lactate: 4.2 mmol/L (normal 0.5-2.2 mmol/L)
- Arterial blood gas: pH 7.32, PaCO2 32 mmHg, HCO3- 18 mEq/L

IMAGING:
An abdominal plain film is unremarkable.

## Encounter 2: source 2020-11-15, practice 2026-03-18, icu, Emergency Medicine

Chief complaint: Acute onset of left lower quadrant abdominal pain with bloody diarrhea and syncope that started 3 hours ago.

ENCOUNTER NOTE
Date: 2020-11-15
Provider: Dr. Elena Rodriguez, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Acute onset of left lower quadrant abdominal pain with bloody diarrhea and syncope that started 3 hours ago.

HISTORY OF PRESENT ILLNESS:
The patient with atrial fibrillation and prior mesenteric ischemia returns, reporting lightheadedness, fatigue, and syncope earlier today.

PAST MEDICAL HISTORY:
Active Problem List:
- Contusion of left flank (diagnosed 2020-01-15)
- Acute mesenteric ischemia (diagnosed 2020-05-15)
- Atrial fibrillation
- Hypertension
- Chronic obstructive pulmonary disease

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy, age 45
- Laparoscopic cholecystectomy, age 62

MEDICATIONS:
She takes metoprolol, warfarin, and lisinopril.

Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 puffs every 4-6 hours as needed
- Tiotropium 18 mcg inhaled daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction, age 68
- Mother: Ischemic stroke, age 75
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 5 years ago. Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Retired elementary school teacher.

VITAL SIGNS:
Blood pressure is 90/60 mmHg, heart rate is 110 bpm

PHYSICAL EXAMINATION:
She appears pale and diaphoretic. Abdominal examination reveals tenderness in the left lower quadrant with no rebound or guarding. Rectal examination shows maroon-colored stool.

LABORATORY DATA:
- Hemoglobin: 10.2 g/dL (baseline 13.5 g/dL)
- Hematocrit: 30.5%
- INR: 3.8
- Creatinine: 1.2 mg/dL
- Lactic acid: 4.5 mmol/L

IMAGING:
A CT scan of the abdomen and pelvis with intravenous contrast shows bowel wall thickening at the splenic flexure.
