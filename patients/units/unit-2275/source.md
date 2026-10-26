# synthetic_hospital patient 2275

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-06-10 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-09-15 | 2025-02-05 (Wed) | inpatient | Cardiology |
| 2 | 2021-09-15 | 2026-02-08 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired long-haul truck driver",
 "alcohol_use": "Regular alcohol consumption, 2-3 beers nightly",
 "family_history": [
  "Father: Myocardial infarction at age 64",
  "Mother: Hypertension and stroke",
  "Brother: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 40 pack-years, quit 1 year ago",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "0.4 mg SL PRN chest pain",
   "name": "Nitroglycerin"
  }
 ],
 "surgical_history": [
  "Appendectomy, age 25"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Chronic stable angina",
  "Obesity",
  "Coronary artery disease"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Obesity", "Acute hypertensive heart failure", "Acute decompensated heart failure"]
Comorbidities: ["Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2024-06-10, outpatient, Internal Medicine

Chief complaint: presents for routine follow-up

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
He presents for routine follow-up

HISTORY OF PRESENT ILLNESS:
He states he wants to “do something natural” to help his blood pressure.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Chronic stable angina
- Obesity
- Coronary artery disease

He was recently diagnosed with hypertension.

PAST SURGICAL HISTORY:
- Appendectomy, age 25

MEDICATIONS:
He takes currently takes lisinopril.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily
- Nitroglycerin 0.4 mg SL PRN chest pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 64
- Mother: Hypertension and stroke
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
He has a 40-pack-year smoking history and consumes alcohol regularly. He lives alone and prepares most of his meals, which are rich in processed foods and meats.

REVIEW OF SYSTEMS:
He does not experience chest pain, headaches, or dizziness.

VITAL SIGNS:
His BMI is 33 kg/m². His blood pressure today is 150/92 mm Hg, pulse is 82/min, respiratory rate is 16/min, and temperature is 98.7°F (37.1°C).

## Encounter 1: source 2020-09-15, practice 2025-02-05, inpatient, Cardiology

Chief complaint: worsening exertional dyspnea and occasional nocturnal dyspnea

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Marcus Johnson, MD
Department: Cardiology
Visit Type: inpatient

CHIEF COMPLAINT:
Worsening exertional dyspnea and occasional nocturnal dyspnea

HISTORY OF PRESENT ILLNESS:
This 68-year-old male with hypertension, chronic stable angina, obesity, and coronary artery disease returns with worsening exertional dyspnea and occasional nocturnal dyspnea over the past few months. He has been treated with hydrochlorothiazide for his hypertension but admits to inconsistent medication adherence.

PAST MEDICAL HISTORY:
Active Problem List:
- Obesity (diagnosed 2020-01-15)
- Hypertension
- Chronic stable angina
- Coronary artery disease

PAST SURGICAL HISTORY:
- Appendectomy, age 25

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily
- Nitroglycerin 0.4 mg SL PRN chest pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 64
- Mother: Hypertension and stroke
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 1 year ago. Alcohol: Regular alcohol consumption, 2-3 beers nightly. Occupation: Retired long-haul truck driver.

VITAL SIGNS:
Blood pressure is 160/90 mmHg, heart rate is 78 bpm, and BMI is 32 kg/m^2

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a low-frequency sound heard at the apex immediately before the first heart sound, which is more pronounced when the patient is in the left lateral decubitus position. His lungs are clear to auscultation, and there is no peripheral edema.

LABORATORY DATA:
Serum sodium: 140 mEq/L
- Serum potassium: 4.2 mEq/L
- Serum creatinine: 1.1 mg/dL
- Fasting glucose: 98 mg/dL
- Total cholesterol: 210 mg/dL
- LDL cholesterol: 140 mg/dL
- HDL cholesterol: 40 mg/dL

OTHER STUDIES:
An electrocardiogram shows left ventricular hypertrophy.

## Encounter 2: source 2021-09-15, practice 2026-02-08, ed, Emergency Medicine

Chief complaint: gradually worsening difficulty breathing over the past week

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Gradually worsening difficulty breathing over the past week

HISTORY OF PRESENT ILLNESS:
This 69-year-old male with hypertension, chronic stable angina, obesity, and coronary artery disease returns following hospitalization for heart failure. He is brought to the emergency department by his daughter. He has also been increasingly tired and says he sometimes needs to sleep sitting upright in a chair. His appetite has declined, but he denies chest pain, cough, or recent infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Obesity (diagnosed 2020-01-15)
- Acute hypertensive heart failure (diagnosed 2020-09-15)
- Hypertension
- Chronic stable angina
- Coronary artery disease

His medical history includes hypertension and chronic stable angina.

PAST SURGICAL HISTORY:
- Appendectomy, age 25

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily
- Nitroglycerin 0.4 mg SL PRN chest pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 64
- Mother: Hypertension and stroke
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
He lives alone and has not followed up with a physician in several months.

PHYSICAL EXAMINATION:
On exam, he appears fatigued and has mild respiratory distress while speaking in short sentences. Cardiovascular exam reveals a regular rate and rhythm with a displaced apical impulse. Bibasilar breath sounds are diminished with crackles at the bases. No wheezing is noted. There is no leg swelling or jugular venous distension.

IMAGING:
A chest x-ray is obtained and shows a prominent cardiac silhouette with pulmonary vascular markings and subtle perihilar haziness.
