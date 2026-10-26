# synthetic_hospital patient 2796

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 43 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-04-11 (Thu) | ed | Emergency Medicine |
| 1 | 2022-01-15 | 2026-04-18 (Sat) | icu | Pulmonary & Critical Care |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "No alcohol use",
 "family_history": [
  "Father with nephrolithiasis",
  "Mother with hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [
  "Recurrent nephrolithiasis",
  "Recurrent urinary tract infections"
 ],
 "age_at_first_encounter": 43
}
```

Primary diagnoses: ["Iatrogenic Vascular Trauma from Injection", "Urolithiasis (Renal Colic)"]
Comorbidities: ["History of septic shock"]

## Encounter 0: source 2020-01-15, practice 2024-04-11, ed, Emergency Medicine

Chief complaint: Intermittent left-sided flank pain and fatigue over the past few months with occasional low-grade fevers and cloudy, foul-smelling urine

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Intermittent left-sided flank pain and fatigue over the past few months with occasional low-grade fevers and cloudy, foul-smelling urine

HISTORY OF PRESENT ILLNESS:
She also reports occasional low-grade fevers, malaise, and cloudy, foul-smelling urine. She denies dysuria or hematuria.

PAST MEDICAL HISTORY:
Active Problem List:
- Recurrent nephrolithiasis
- Recurrent urinary tract infections

Her past medical history is significant for recurrent urinary tract infections, often treated with short courses of antibiotics prescribed by urgent care clinics.

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with nephrolithiasis
- Mother with hypertension

SOCIAL HISTORY:
She does not smoke or drink alcohol and works as a school teacher.

VITAL SIGNS:
On physical examination, she is afebrile, with normal vital signs.

PHYSICAL EXAMINATION:
Mild left costovertebral angle tenderness is noted.

LABORATORY DATA:
A urinalysis reveals an alkaline pH of 8.5 and numerous white blood cells. Urine culture is pending. Serum creatinine is 1.0 mg/dL.

IMAGING:
Due to the chronicity of her symptoms, a non-contrast CT scan of the abdomen is performed and a radiograph is also obtained (shown below).

PLAN:
The patient is started on intravenous fluids and pain control.

## Encounter 1: source 2022-01-15, practice 2026-04-18, icu, Pulmonary & Critical Care

Chief complaint: Sudden-onset shortness of breath and hypoxemia immediately following removal of central venous catheter

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Robert Kim, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Sudden-onset shortness of breath and hypoxemia immediately following removal of central venous catheter

HISTORY OF PRESENT ILLNESS:
The 45-year-old female with recurrent nephrolithiasis and UTIs and prior 2020 flank pain is brought to the emergency department with sudden-onset shortness of breath and hypoxemia immediately following the removal of a central venous catheter in the intensive care unit.

PAST MEDICAL HISTORY:
Active Problem List:
- Urolithiasis (Renal Colic) (diagnosed 2020-01-15)
- Recurrent nephrolithiasis
- Recurrent urinary tract infections

She has a history of septic shock secondary to a complicated urinary tract infection, for which she was receiving intravenous antibiotics through the catheter.

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with nephrolithiasis
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Elementary school teacher.

VITAL SIGNS:
Tachypneic with a respiratory rate of 28 breaths per minute, and her oxygen saturation is 88% on room air. Her blood pressure is 110/70 mmHg, and her heart rate is 110 beats per minute.

PHYSICAL EXAMINATION:
On examination, she is anxious. Auscultation of the chest reveals diffuse bilateral wheezes.

IMAGING:
A bedside transthoracic echocardiogram shows no evidence of intracardiac shunt but there is a hyperechoic area in the right ventricle suggestive of air.
