# synthetic_hospital patient 2074

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 56 at first encounter, sex M, Non-Hispanic White, Commercial PPO (COBRA continuation).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2026-04-28 (Tue) | outpatient | Cardiology |
| 1 | 2020-03-15 | 2026-07-15 (Wed) | outpatient | Psychiatry |
| 2 | 2020-04-15 | 2026-08-05 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO (COBRA continuation)",
 "occupation": "Former construction foreman, currently on medical disability leave",
 "alcohol_use": "None",
 "family_history": [
  "Father: Myocardial infarction at age 58",
  "Mother: Major depressive disorder",
  "Brother: Hypertension and type 2 diabetes"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 30 pack-years, quit 2 months ago following myocardial infarction",
 "home_medications": [
  {
   "dose": "81 mg oral daily",
   "name": "Aspirin"
  },
  {
   "dose": "40 mg oral nightly",
   "name": "Atorvastatin"
  },
  {
   "dose": "50 mg oral daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "90 mcg, 2 puffs every 4-6 hours as needed for shortness of breath",
   "name": "Albuterol sulfate inhaler"
  },
  {
   "dose": "18 mcg inhaled daily",
   "name": "Tiotropium"
  }
 ],
 "surgical_history": [
  "Percutaneous coronary intervention with drug-eluting stent placement (2 months prior)"
 ],
 "chronic_conditions": [
  "Coronary artery disease",
  "Chronic obstructive pulmonary disease",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 56
}
```

Primary diagnoses: ["Major depressive disorder", "Dressler syndrome", "Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease)"]
Comorbidities: ["Medication noncompliance", "Old myocardial infarction", "Status post percutaneous coronary intervention"]

## Encounter 0: source 2020-01-15, practice 2026-04-28, outpatient, Cardiology

Chief complaint: Increasing shortness of breath, fatigue, and swelling in legs over the past two months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Patel, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing shortness of breath, fatigue, and swelling in legs over the past two months

HISTORY OF PRESENT ILLNESS:
He has been noncompliant with his inhalers.

PAST MEDICAL HISTORY:
Active Problem List:
- Coronary artery disease
- Chronic obstructive pulmonary disease
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (2 months prior)

MEDICATIONS:
Home Medications:
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral nightly
- Metoprolol succinate 50 mg oral daily
- Albuterol sulfate inhaler 90 mcg, 2 puffs every 4-6 hours as needed for shortness of breath
- Tiotropium 18 mcg inhaled daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 58
- Mother: Major depressive disorder
- Brother: Hypertension and type 2 diabetes

SOCIAL HISTORY:
He has a 30-pack-year smoking history

VITAL SIGNS:
On physical examination, his blood pressure is 130/85 mmHg, heart rate is 98 bpm, and respiratory rate is 22 breaths per minute

PHYSICAL EXAMINATION:
He has distended neck veins with a prominent V wave noted. His lung examination reveals prolonged expiration with wheezes. Cardiac examination shows a holosystolic murmur best heard at the left lower sternal border that increases with inspiration. Abdominal examination is unremarkable. Extremities show 2+ pitting edema up to the mid-shins bilaterally.

IMAGING:
Chest X-ray shows hyperinflated lungs and an enlarged cardiac silhouette. Echocardiogram reveals right ventricular dilation and tricuspid valve regurgitation with no significant left heart disease.

## Encounter 1: source 2020-03-15, practice 2026-07-15, outpatient, Psychiatry

Chief complaint: Routine follow-up two months after myocardial infarction discharge with low mood and fatigue

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. James Okafor, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up two months after myocardial infarction discharge with low mood and fatigue

HISTORY OF PRESENT ILLNESS:
This 56-year-old male with coronary artery disease, chronic obstructive pulmonary disease, and hyperlipidemia returns for psychiatric evaluation two months after ST-elevation myocardial infarction discharge following Cardiology evaluation two months ago for dyspnea and leg edema. He was also referred to a cardiac rehabilitation program but has only attended one session. Over the past month, he reports persistent low mood, diminished energy, early morning awakening, and loss of interest in previously enjoyable activities. He denies suicidal ideation.

PAST MEDICAL HISTORY:
Active Problem List:
- Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease) (diagnosed 2020-01-15)
- Coronary artery disease
- Chronic obstructive pulmonary disease
- Hyperlipidemia

PAST SURGICAL HISTORY:
He underwent successful percutaneous coronary intervention

MEDICATIONS:
He was started on atorvastatin, metoprolol succinate, and aspirin.

Home Medications:
- Albuterol sulfate inhaler 90 mcg, 2 puffs every 4-6 hours as needed for shortness of breath
- Tiotropium 18 mcg inhaled daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 58
- Mother: Major depressive disorder
- Brother: Hypertension and type 2 diabetes

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 2 months ago following myocardial infarction. Alcohol: None. Occupation: Former construction foreman, currently on medical disability leave.

VITAL SIGNS:
Vital signs and laboratory values are within normal limits

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
Laboratory values are within normal limits

## Encounter 2: source 2020-04-15, practice 2026-08-05, ed, Emergency Medicine

Chief complaint: Two-week history of pleuritic chest pain and low-grade fevers

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-week history of pleuritic chest pain and low-grade fevers

HISTORY OF PRESENT ILLNESS:
This 56-year-old male with coronary artery disease and chronic obstructive pulmonary disease, recently evaluated for cor pulmonale and depression post-MI, presents with a 2-week history of pleuritic chest pain and low-grade fevers. He reports pain worsens when lying down and improves when leaning forward.

PAST MEDICAL HISTORY:
Active Problem List:
- Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease) (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-03-15)
- Coronary artery disease
- Chronic obstructive pulmonary disease
- Hyperlipidemia

He had a myocardial infarction 6 weeks ago, which was managed with percutaneous coronary intervention.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (2 months prior)

MEDICATIONS:
His current medications include aspirin, atorvastatin, and metoprolol.

Home Medications:
- Metoprolol succinate 50 mg oral daily
- Albuterol sulfate inhaler 90 mcg, 2 puffs every 4-6 hours as needed for shortness of breath
- Tiotropium 18 mcg inhaled daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 58
- Mother: Major depressive disorder
- Brother: Hypertension and type 2 diabetes

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 2 months ago following myocardial infarction. Alcohol: None. Occupation: Former construction foreman, currently on medical disability leave.

PHYSICAL EXAMINATION:
On physical examination, a pericardial friction rub is auscultated.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 12,000 /µL
- Erythrocyte sedimentation rate: 50 mm/hr
- C-reactive protein: 30 mg/L
- Troponin I: <0.04 ng/mL (normal)

IMAGING:
Echocardiography reveals a small pericardial effusion without evidence of tamponade.

OTHER STUDIES:
An ECG shows diffuse ST-segment elevations and PR depressions.
