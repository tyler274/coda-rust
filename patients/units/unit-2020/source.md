# synthetic_hospital patient 2020

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 56 at first encounter, sex M, African American, Medicare (disability) and Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-07-22 (Tue) | outpatient | Cardiology |
| 1 | 2020-07-15 | 2026-01-17 (Sat) | ed | Emergency Medicine |
| 2 | 2020-10-15 | 2026-04-15 (Wed) | icu | Pulmonary & Critical Care |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare (disability) and Medicaid",
 "occupation": "Retired truck driver, currently on disability",
 "alcohol_use": "Occasional (1-2 drinks per month)",
 "family_history": [
  "Father: Myocardial infarction at age 62",
  "Mother: Type 2 diabetes mellitus, hypertension",
  "Brother: Coronary artery disease"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Former smoker, quit 8 years ago, 30 pack-year history",
 "home_medications": [
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Chlorthalidone"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Furosemide"
  }
 ],
 "surgical_history": [
  "Appendectomy (age 22)",
  "Left knee arthroscopy (age 48)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Chronic heart failure with reduced ejection fraction",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 56
}
```

Primary diagnoses: ["Acute decompensated heart failure", "Flash pulmonary edema"]
Comorbidities: ["Acute decompensated heart failure", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2025-07-22, outpatient, Cardiology

Chief complaint: Progressive lower-extremity edema and fatigue over 2 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive lower-extremity edema and fatigue over 2 months

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 2-month history of progressive lower-extremity edema and fatigue. He denies any chest pain, palpitations, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic heart failure with reduced ejection fraction
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Appendectomy (age 22)
- Left knee arthroscopy (age 48)

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Metoprolol succinate 50 mg PO daily
- Chlorthalidone 25 mg PO daily
- Furosemide 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus, hypertension
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 30 pack-year history. Alcohol: Occasional (1-2 drinks per month). Occupation: Retired truck driver, currently on disability.

VITAL SIGNS:
On physical examination, his blood pressure is 142/88 mmHg, heart rate is 88 beats per minute, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
He has 2+ pitting edema up to the shins bilaterally. Auscultation of the heart reveals an S3 gallop, and lung fields are clear to auscultation. Abdominal examination is unremarkable with no hepatosplenomegaly or ascites. Jugular venous pressure is elevated at 8 cm H2O. A hepatojugular reflux test is performed, and there is a sustained increase in jugular venous pressure.

## Encounter 1: source 2020-07-15, practice 2026-01-17, ed, Emergency Medicine

Chief complaint: Worsening shortness of breath over 3 days with orthopnea requiring two pillows

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening shortness of breath over 3 days with orthopnea requiring two pillows

HISTORY OF PRESENT ILLNESS:
The patient is a 56-year-old male with chronic heart failure with reduced ejection fraction returning six months after his prior cardiology encounter. He reports needing to sleep upright on two pillows and notes increasing difficulty breathing while lying flat. He denies cough, fever, chills, or chest pain. He also reports decreased exercise tolerance over the past few weeks.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute decompensated heart failure (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic heart failure with reduced ejection fraction
- Hyperlipidemia

He has a longstanding history of systemic hypertension.

PAST SURGICAL HISTORY:
- Appendectomy (age 22)
- Left knee arthroscopy (age 48)

MEDICATIONS:
He takes a thiazide diuretic and a beta-blocker.

Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Chlorthalidone 25 mg PO daily
- Furosemide 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus, hypertension
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 30 pack-year history. Alcohol: Occasional (1-2 drinks per month). Occupation: Retired truck driver, currently on disability.

VITAL SIGNS:
On examination, he is afebrile with a respiratory rate of 22/min, blood pressure of 168/94 mm Hg, and pulse of 88/min.

PHYSICAL EXAMINATION:
Bibasilar crackles are audible on lung auscultation, and dullness to percussion is noted at the lung bases. Cardiac auscultation reveals a normal S1 and S2 with no murmurs, gallops, or rubs. There is trace bilateral lower extremity pitting edema.

LABORATORY DATA:
Serum creatinine and liver enzymes are within normal limits.

IMAGING:
Chest x-ray shows bilateral pleural effusions and interstitial markings consistent with pulmonary congestion.

OTHER STUDIES:
An ECG demonstrates a widened QRS complex with broad notched R waves in leads I and V6. Thoracentesis is performed, and analysis of the pleural fluid is pending.

PLAN:
The patient is admitted for further management of his respiratory symptoms. Based on this presentation and the most likely underlying cause of the effusion,

## Encounter 2: source 2020-10-15, practice 2026-04-15, icu, Pulmonary & Critical Care

Chief complaint: Acute onset of shortness of breath while lying in bed

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Priya Patel, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Acute onset of shortness of breath while lying in bed

HISTORY OF PRESENT ILLNESS:
The patient with HFrEF returns after January and July 2020 episodes. Symptoms began while he was lying in bed. He reports a 2-week history of increasing lower extremity edema and paroxysmal nocturnal dyspnea.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute decompensated heart failure (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic heart failure with reduced ejection fraction
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Appendectomy (age 22)
- Left knee arthroscopy (age 48)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Chlorthalidone 25 mg PO daily
- Furosemide 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus, hypertension
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 30 pack-year history. Alcohol: Occasional (1-2 drinks per month). Occupation: Retired truck driver, currently on disability.

VITAL SIGNS:
With a respiratory rate of 30 breaths per minute, blood pressure is 170/100 mmHg, heart rate is 110 bpm, and oxygen saturation is 88% on room air.

PHYSICAL EXAMINATION:
On examination, he is tachypneic
Lung auscultation reveals bilateral crackles up to the mid-lung fields. Jugular venous pressure is elevated.

LABORATORY DATA:
Arterial blood gas on 100% oxygen shows:

- pH: 7.45
- PaCO2: 35 mmHg
- PaO2: 60 mmHg
- HCO3-: 24 mEq/L

IMAGING:
Chest X-ray shows diffuse bilateral alveolar infiltrates.
