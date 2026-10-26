# synthetic_hospital patient 2269

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-12-08 (Mon) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2026-06-10 (Wed) | ed | Emergency Medicine |
| 2 | 2020-07-15 | 2026-06-12 (Fri) | icu | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired long-haul truck driver",
 "alcohol_use": "None",
 "family_history": [
  "Father: Myocardial infarction at age 61",
  "Mother: Intracerebral hemorrhage at age 74",
  "Sibling: Hypertension and type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 40 pack-years, quit 10 years ago",
 "home_medications": [
  {
   "dose": "81 mg oral daily",
   "name": "Aspirin"
  },
  {
   "dose": "40 mg oral daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "50 mg oral daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "10 mg oral daily",
   "name": "Lisinopril"
  },
  {
   "dose": "18 mcg inhaled daily",
   "name": "Tiotropium"
  },
  {
   "dose": "90 mcg inhaled every 4 hours as needed for shortness of breath",
   "name": "Albuterol sulfate"
  }
 ],
 "surgical_history": [
  "Percutaneous coronary intervention with drug-eluting stent placement (3 weeks prior to first encounter)",
  "Left femoral-popliteal thromboembolectomy with fasciotomy"
 ],
 "chronic_conditions": [
  "Hypertension, poorly controlled",
  "Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) Group B",
  "Coronary artery disease, status post myocardial infarction",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Acute peripheral arterial thrombosis/embolism", "Ventilator-associated pneumonia", "Intracerebral hemorrhage with airway compromise"]
Comorbidities: ["Acute myocardial infarction", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2025-12-08, ed, Emergency Medicine

Chief complaint: presents to the emergency department with sudden onset of severe pain in his left lower extremity

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
He presents to the emergency department with sudden onset of severe pain in his left lower extremity

HISTORY OF PRESENT ILLNESS:
He describes the pain as sharp and constant, and he notes that his foot appears pale and feels cold to the touch. He denies any trauma or injury to the leg.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension, poorly controlled
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) Group B
- Coronary artery disease, status post myocardial infarction
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (3 weeks prior to first encounter)
- Left femoral-popliteal thromboembolectomy with fasciotomy

MEDICATIONS:
The patient's current medications include aspirin, atorvastatin, and metoprolol.

Home Medications:
- Metoprolol succinate 50 mg oral daily
- Lisinopril 10 mg oral daily
- Tiotropium 18 mcg inhaled daily
- Albuterol sulfate 90 mcg inhaled every 4 hours as needed for shortness of breath

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 61
- Mother: Intracerebral hemorrhage at age 74
- Sibling: Hypertension and type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 10 years ago. Alcohol: None. Occupation: Retired long-haul truck driver.

VITAL SIGNS:
His vital signs are stable, and he is afebrile.

PHYSICAL EXAMINATION:
On physical examination, the left foot has a diminished pulse compared to the right, and is cool with delayed capillary refill. Sensation is decreased, and motor strength is 3/5 in the left foot dorsiflexors and plantar flexors.

LABORATORY DATA:
Complete Blood Count (CBC): Within normal limits
- Serum Creatinine: 1.1 mg/dL
- Troponin: Not elevated
- D-dimer: Elevated

IMAGING:
A Doppler ultrasound of the left lower extremity shows an abrupt cutoff of arterial flow in the left popliteal artery.

OTHER STUDIES:
Ankle-Brachial Index (ABI) on the left is 0.5.

ASSESSMENT AND PLAN:
Given the patient's recent MI and current presentation,

## Encounter 1: source 2020-07-15, practice 2026-06-10, ed, Emergency Medicine

Chief complaint: brought to the emergency department after being found unresponsive

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought to the emergency department after being found unresponsive

HISTORY OF PRESENT ILLNESS:
This 68-year-old male with poorly controlled hypertension, COPD, coronary artery disease status post myocardial infarction, and hyperlipidemia, who presented six months prior with acute peripheral arterial thrombosis of the left lower extremity, returns to the emergency department after being found unresponsive. No known trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute peripheral arterial thrombosis/embolism (diagnosed 2020-01-15)
- Hypertension, poorly controlled
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) Group B
- Coronary artery disease, status post myocardial infarction
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (3 weeks prior to first encounter)
- Left femoral-popliteal thromboembolectomy with fasciotomy

MEDICATIONS:
Home Medications:
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Lisinopril 10 mg oral daily
- Tiotropium 18 mcg inhaled daily
- Albuterol sulfate 90 mcg inhaled every 4 hours as needed for shortness of breath

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 61
- Mother: Intracerebral hemorrhage at age 74
- Sibling: Hypertension and type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 10 years ago. Alcohol: None. Occupation: Retired long-haul truck driver.

VITAL SIGNS:
His blood pressure is 220/120 mmHg

PHYSICAL EXAMINATION:
On arrival, he is obtunded with a Glasgow Coma Scale score of 6. Pupils are reactive and symmetric.

IMAGING:
A non-contrast CT scan of the head reveals a hyperdense collection within the basal ganglia extending into the internal capsule, with surrounding edema and no midline shift. There is no crescent- or lens-shaped hematoma.

## Encounter 2: source 2020-07-15, practice 2026-06-12, icu, Internal Medicine

Chief complaint: respiratory failure and septic shock secondary to community-acquired pneumonia

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: icu

CHIEF COMPLAINT:
Respiratory failure and septic shock secondary to community-acquired pneumonia

HISTORY OF PRESENT ILLNESS:
This 68-year-old male with hypertension, CAD, and COPD returns following intracerebral hemorrhage with airway compromise. He was intubated and started on mechanical ventilation 5 days ago. Over the past 24 hours, the patient has developed a new fever of 38.9°C, increased purulent tracheal secretions, and worsening hypoxemia.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute peripheral arterial thrombosis/embolism (diagnosed 2020-01-15)
- Intracerebral hemorrhage with airway compromise (diagnosed 2020-07-15)
- Hypertension, poorly controlled
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) Group B
- Coronary artery disease, status post myocardial infarction
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (3 weeks prior to first encounter)
- Left femoral-popliteal thromboembolectomy with fasciotomy

MEDICATIONS:
He takes on norepinephrine.

Home Medications:
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Lisinopril 10 mg oral daily
- Tiotropium 18 mcg inhaled daily
- Albuterol sulfate 90 mcg inhaled every 4 hours as needed for shortness of breath

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 61
- Mother: Intracerebral hemorrhage at age 74
- Sibling: Hypertension and type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 10 years ago. Alcohol: None. Occupation: Retired long-haul truck driver.

VITAL SIGNS:
Hemodynamically stable

PHYSICAL EXAMINATION:
Currently sedated. Auscultation of the lungs reveals diffuse crackles bilaterally. The rest of the physical examination is unremarkable.

LABORATORY DATA:
White blood cell count has risen from 12,000 to 18,000 cells/µL

IMAGING:
Chest x-ray reveals new bilateral infiltrates

ASSESSMENT AND PLAN:
Given the clinical presentation and recent intubation,
