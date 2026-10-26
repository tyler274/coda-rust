# synthetic_hospital patient 2139

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 57 at first encounter, sex M, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-02-26 (Wed) | inpatient | Internal Medicine |
| 1 | 2020-09-15 | 2025-11-07 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Penicillin (anaphylaxis)",
  "Shellfish (pruritus)"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Civil service administrator",
 "alcohol_use": "Occasional, 1-2 beers on weekends",
 "family_history": [
  "Father: Myocardial infarction at age 62, hypertension",
  "Mother: Atrial fibrillation, type 2 diabetes",
  "Older brother: Coronary artery disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 3 years ago, 25 pack-year history",
 "home_medications": [
  {
   "dose": "Variable dose daily (target INR 2.0-3.0)",
   "name": "Warfarin"
  },
  {
   "dose": "100 mg daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "81 mg daily",
   "name": "Aspirin"
  },
  {
   "dose": "40 mg daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg daily",
   "name": "Pantoprazole"
  }
 ],
 "surgical_history": [
  "Cardiac catheterization with femoral artery access (complicated by retroperitoneal hematoma)",
  "Laparoscopic cholecystectomy (age 45)"
 ],
 "chronic_conditions": [
  "Nonvalvular atrial fibrillation",
  "Coronary artery disease",
  "Hypertension",
  "Hyperlipidemia",
  "Gastroesophageal reflux disease"
 ],
 "age_at_first_encounter": 57
}
```

Primary diagnoses: ["Coagulopathy due to vitamin K deficiency", "Retroperitoneal hematoma following cardiac catheterization"]
Comorbidities: ["Long-term anticoagulant therapy", "Status post mechanical mitral valve replacement"]

## Encounter 0: source 2020-01-15, practice 2025-02-26, inpatient, Internal Medicine

Chief complaint: Worsening shortness of breath and fatigue

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Worsening shortness of breath and fatigue

HISTORY OF PRESENT ILLNESS:
The patient reports poor appetite over the past 2 weeks due to a recent gastrointestinal illness with prolonged diarrhea. He has also been taking broad-spectrum antibiotics prescribed by his primary physician.

PAST MEDICAL HISTORY:
Active Problem List:
- Nonvalvular atrial fibrillation
- Coronary artery disease
- Hypertension
- Hyperlipidemia
- Gastroesophageal reflux disease

He has a history of nonvalvular atrial fibrillation and.

PAST SURGICAL HISTORY:
He underwent mechanical mitral valve replacement 8 months ago.

MEDICATIONS:
His medications include a beta-blocker and an anticoagulant.

Home Medications:
- Warfarin Variable dose daily (target INR 2.0-3.0)
- Metoprolol succinate 100 mg daily
- Aspirin 81 mg daily
- Atorvastatin 40 mg daily
- Lisinopril 20 mg daily
- Pantoprazole 40 mg daily

ALLERGIES:
Penicillin (anaphylaxis), Shellfish (pruritus)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Atrial fibrillation, type 2 diabetes
- Older brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 3 years ago, 25 pack-year history. Alcohol: Occasional, 1-2 beers on weekends. Occupation: Civil service administrator.

PHYSICAL EXAMINATION:
On examination, there are multiple ecchymoses over the arms and legs, and scattered petechiae on the oral mucosa.

LABORATORY DATA:
Laboratory studies show prothrombin time of 29 seconds, partial thromboplastin time of 48 seconds, and platelet count of 210,000 /mm³.

## Encounter 1: source 2020-09-15, practice 2025-11-07, ed, Emergency Medicine

Chief complaint: 3-day history of fatigue and dark urine

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
3-day history of fatigue and dark urine

HISTORY OF PRESENT ILLNESS:
This 57-year-old male with nonvalvular atrial fibrillation on warfarin and history of coagulopathy presents with three-day fatigue and dark urine. He denies any recent trauma, medication changes, or infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Coagulopathy due to vitamin K deficiency (diagnosed 2020-01-15)
- Nonvalvular atrial fibrillation
- Coronary artery disease
- Hypertension
- Hyperlipidemia
- Gastroesophageal reflux disease

PAST SURGICAL HISTORY:
Aortic valve replacement with a mechanical valve 2 years ago

MEDICATIONS:
He takes warfarin and aspirin.

Home Medications:
- Metoprolol succinate 100 mg daily
- Atorvastatin 40 mg daily
- Lisinopril 20 mg daily
- Pantoprazole 40 mg daily

ALLERGIES:
Penicillin (anaphylaxis), Shellfish (pruritus)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Atrial fibrillation, type 2 diabetes
- Older brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 3 years ago, 25 pack-year history. Alcohol: Occasional, 1-2 beers on weekends. Occupation: Civil service administrator.

VITAL SIGNS:
Temperature is 37.2°C (99°F), blood pressure is 130/80 mmHg, pulse is 90/min, and respiratory rate is 16/min

PHYSICAL EXAMINATION:
His skin and sclerae are anicteric. Cardiac examination reveals a mechanical valve click, but no murmurs, rubs, or gallops. Abdominal examination is unremarkable.

LABORATORY DATA:
Urinalysis shows a red/brown color, is positive for heme, but has no red blood cells on microscopy. Laboratory values are as follows:

- Hemoglobin: 9.2 g/dL (Normal: 13.8-17.2 g/dL)
- Hematocrit: 27.8% (Normal: 40.7-50.3%)
- Lactate dehydrogenase (LDH): Elevated
- Haptoglobin: Decreased
- Direct Coombs test: Negative
- Platelet count: 250,000/µL (Normal: 150,000-450,000/µL)
- INR: 3.5 (Target range for mechanical valve: 2.0-3.0)
