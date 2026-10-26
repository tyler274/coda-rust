# synthetic_hospital patient 2801

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 42 at first encounter, sex F, Hispanic, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-10-11 (Tue) | outpatient | Cardiology |
| 1 | 2021-07-15 | 2024-04-15 (Mon) | outpatient | Cardiology |
| 2 | 2023-07-15 | 2026-03-31 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Administrative assistant",
 "alcohol_use": "None",
 "family_history": [
  "Mother with history of rheumatic fever",
  "Father with type 2 diabetes mellitus",
  "Maternal uncle with ischemic stroke at age 65"
 ],
 "race_ethnicity": "Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "5 mg PO daily (adjusted to INR 2.0-3.0)",
   "name": "Warfarin sodium"
  },
  {
   "dose": "20 mEq PO daily",
   "name": "Potassium chloride"
  }
 ],
 "surgical_history": [
  "Tonsillectomy at age 8"
 ],
 "chronic_conditions": [
  "Rheumatic heart disease",
  "Severe mitral stenosis",
  "Secondary pulmonary hypertension (WHO Group 2)",
  "Chronic heart failure",
  "Paroxysmal atrial fibrillation"
 ],
 "age_at_first_encounter": 42
}
```

Primary diagnoses: ["Acute Decompensated Heart Failure with Pulmonary Edema", "Pulmonary hypertension secondary to mitral stenosis", "Rheumatic mitral stenosis"]
Comorbidities: ["Rheumatic mitral stenosis"]

## Encounter 0: source 2020-01-15, practice 2022-10-11, outpatient, Cardiology

Chief complaint: Progressive shortness of breath and fatigue over the past several months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Vargas, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath and fatigue over the past several months

HISTORY OF PRESENT ILLNESS:
She notes that she becomes winded walking up stairs or doing housework, which never used to be an issue. She also mentions occasional palpitations and has noticed her legs feel a bit swollen by the end of the day. She denies chest pain, cough, or orthopnea.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatic heart disease
- Severe mitral stenosis
- Secondary pulmonary hypertension (WHO Group 2)
- Chronic heart failure
- Paroxysmal atrial fibrillation

Her medical history includes rheumatic fever as a child, but she has not had any follow-up for years.

PAST SURGICAL HISTORY:
- Tonsillectomy at age 8

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Warfarin sodium 5 mg PO daily (adjusted to INR 2.0-3.0)
- Potassium chloride 20 mEq PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with history of rheumatic fever
- Father with type 2 diabetes mellitus
- Maternal uncle with ischemic stroke at age 65

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative assistant.

VITAL SIGNS:
Vital signs show a heart rate of 92/min and a blood pressure of 118/72 mmHg.

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a loud first heart sound and a low-pitched, rumbling diastolic murmur best heard at the cardiac apex with the bell of the stethoscope while the patient is in the left lateral decubitus position. There are bibasilar crackles on lung exam and mild bilateral ankle edema.

IMAGING:
Chest x-ray shows mild pulmonary vascular congestion.

## Encounter 1: source 2021-07-15, practice 2024-04-15, outpatient, Cardiology

Chief complaint: Gradually worsening exertional dyspnea and fatigue over the last several years with new orthopnea

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Elena Vargas, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Gradually worsening exertional dyspnea and fatigue over the last several years with new orthopnea

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up evaluation of her chronic rheumatic valvular disease, having initially presented in January 2020 with progressive shortness of breath and fatigue. Her symptoms initially became pronounced during her most recent pregnancy, necessitating diuretic therapy, dietary sodium restriction, and close monitoring. She has since experienced increasing orthopnea and decreased exercise tolerance despite adherence to her regimen of hydrochlorothiazide.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatic mitral stenosis (diagnosed 2020-01-15)
- Rheumatic heart disease
- Severe mitral stenosis
- Secondary pulmonary hypertension (WHO Group 2)
- Chronic heart failure
- Paroxysmal atrial fibrillation

She was diagnosed with mitral stenosis in adolescence, attributed to a history of untreated Streptococcus pyogenes pharyngitis leading to rheumatic heart disease.

PAST SURGICAL HISTORY:
- Tonsillectomy at age 8

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Warfarin sodium 5 mg PO daily (adjusted to INR 2.0-3.0)
- Potassium chloride 20 mEq PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with history of rheumatic fever
- Father with type 2 diabetes mellitus
- Maternal uncle with ischemic stroke at age 65

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative assistant.

VITAL SIGNS:
Vital signs show mild tachycardia and tachypnea, with a blood pressure of 112/82 mm Hg.

PHYSICAL EXAMINATION:
On cardiac examination, you note a loud S1, a prominent opening snap following S2, and a low-pitched, rumbling late diastolic murmur at the cardiac apex. There is a palpable right ventricular heave at the left sternal border, but no peripheral edema or jugular venous distension.

IMAGING:
Chest radiography shows mild left atrial enlargement and evidence of pulmonary vascular redistribution.

## Encounter 2: source 2023-07-15, practice 2026-03-31, ed, Emergency Medicine

Chief complaint: Acute onset shortness of breath, palpitations, and cough worsening over the past few hours

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Michael Okonkwo, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset shortness of breath, palpitations, and cough worsening over the past few hours

HISTORY OF PRESENT ILLNESS:
Patient with rheumatic mitral stenosis and secondary pulmonary hypertension followed in Cardiology presents to the emergency department with acute onset of shortness of breath, palpitations, and a cough that has worsened over the past few hours.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatic mitral stenosis (diagnosed 2020-01-15)
- Pulmonary hypertension secondary to mitral stenosis (diagnosed 2021-07-15)
- Rheumatic heart disease
- Severe mitral stenosis
- Secondary pulmonary hypertension (WHO Group 2)
- Chronic heart failure
- Paroxysmal atrial fibrillation

She reports a history of rheumatic fever in childhood.

PAST SURGICAL HISTORY:
- Tonsillectomy at age 8

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Warfarin sodium 5 mg PO daily (adjusted to INR 2.0-3.0)
- Potassium chloride 20 mEq PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with history of rheumatic fever
- Father with type 2 diabetes mellitus
- Maternal uncle with ischemic stroke at age 65

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative assistant.

VITAL SIGNS:
On examination, her blood pressure is 130/85 mmHg, heart rate is irregularly irregular at 120 bpm, respiratory rate is 22 breaths/min, and oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
Auscultation of the heart reveals a diastolic rumbling murmur best heard at the apex with the patient in the left lateral decubitus position. Lung examination shows bibasilar crackles.

IMAGING:
A chest x-ray demonstrates an elevation of the left mainstem bronchus and left atrial enlargement.

OTHER STUDIES:
An electrocardiogram confirms atrial fibrillation with no ST changes.
