# synthetic_hospital patient 2186

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-10-14 (Mon) | ed | Emergency Medicine |
| 1 | 2020-09-15 | 2025-06-20 (Fri) | icu | Cardiology |
| 2 | 2021-05-15 | 2026-02-12 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare",
 "occupation": "Retired long-haul truck driver",
 "alcohol_use": "Occasional (1-2 drinks per month)",
 "family_history": [
  "Father: Deceased at age 61 from acute myocardial infarction",
  "Mother: Hypertension and osteoporosis, age 91",
  "One older brother: Coronary artery disease, status post coronary artery bypass grafting"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 50 pack-year history, quit 2 years ago",
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
   "dose": "25 mg oral daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "18 mcg inhaled daily",
   "name": "Tiotropium"
  },
  {
   "dose": "90 mcg inhaler, 2 puffs every 4-6 hours as needed for wheezing or shortness of breath",
   "name": "Albuterol sulfate"
  }
 ],
 "surgical_history": [
  "Mitral valve repair with bioprosthetic ring annuloplasty (complicated by papillary muscle rupture following acute myocardial infarction)",
  "Cardiac catheterization with percutaneous coronary intervention and drug-eluting stent placement to left anterior descending artery",
  "Diagnostic bronchoscopy with bronchoalveolar lavage (during prior admission)"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) Group C",
  "Hypertension",
  "Hyperlipidemia",
  "Coronary artery disease, status post myocardial infarction"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Beta-blocker toxicity", "Cardiogenic Shock", "Acute mitral regurgitation due to papillary muscle rupture"]
Comorbidities: ["Acute hypoxemic respiratory failure without hypercapnic encephalopathy", "Acute myocardial infarction", "Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2024-10-14, ed, Emergency Medicine

Chief complaint: Acute onset of severe tearing chest pain radiating to the back with shortness of breath

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of severe tearing chest pain radiating to the back with shortness of breath

HISTORY OF PRESENT ILLNESS:
He describes the pain as severe, tearing, and radiating to his back.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) Group C
- Hypertension
- Hyperlipidemia
- Coronary artery disease, status post myocardial infarction

PAST SURGICAL HISTORY:
- Mitral valve repair with bioprosthetic ring annuloplasty (complicated by papillary muscle rupture following acute myocardial infarction)
- Cardiac catheterization with percutaneous coronary intervention and drug-eluting stent placement to left anterior descending artery
- Diagnostic bronchoscopy with bronchoalveolar lavage (during prior admission)

MEDICATIONS:
Home Medications:
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral daily
- Metoprolol succinate 25 mg oral daily
- Tiotropium 18 mcg inhaled daily
- Albuterol sulfate 90 mcg inhaler, 2 puffs every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Deceased at age 61 from acute myocardial infarction
- Mother: Hypertension and osteoporosis, age 91
- One older brother: Coronary artery disease, status post coronary artery bypass grafting

SOCIAL HISTORY:
Smoking: Former smoker, 50 pack-year history, quit 2 years ago. Alcohol: Occasional (1-2 drinks per month). Occupation: Retired long-haul truck driver.

VITAL SIGNS:
On examination, his blood pressure is 90/60 mmHg, heart rate is 120 beats per minute, respiratory rate is 28 breaths per minute, and oxygen saturation is 88% on room air.

PHYSICAL EXAMINATION:
Lung auscultation reveals bilateral rales halfway up both lung fields. Cardiac examination shows a new holosystolic murmur at the apex that radiates to the axilla. The patient appears diaphoretic and in distress.

IMAGING:
Chest X-ray demonstrates pulmonary edema. An urgent transthoracic echocardiogram reveals flail mitral valve leaflets with severe mitral regurgitation and a dilated left atrium.

OTHER STUDIES:
An ECG shows no new changes since his previous myocardial infarction.

## Encounter 1: source 2020-09-15, practice 2025-06-20, icu, Cardiology

Chief complaint: Acute dyspnea and hypotension following elective total hip replacement

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Marcus Johnson, MD
Department: Cardiology
Visit Type: icu

CHIEF COMPLAINT:
Acute dyspnea and hypotension following elective total hip replacement

HISTORY OF PRESENT ILLNESS:
The patient with COPD Group C and papillary muscle rupture is admitted to the ICU following an elective total hip replacement. On postoperative day 2, he becomes He denies chest pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute mitral regurgitation due to papillary muscle rupture (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) Group C
- Hypertension
- Hyperlipidemia
- Coronary artery disease, status post myocardial infarction

PAST SURGICAL HISTORY:
- Mitral valve repair with bioprosthetic ring annuloplasty (complicated by papillary muscle rupture following acute myocardial infarction)
- Cardiac catheterization with percutaneous coronary intervention and drug-eluting stent placement to left anterior descending artery
- Diagnostic bronchoscopy with bronchoalveolar lavage (during prior admission)

MEDICATIONS:
His current medications include aspirin, atorvastatin, and metoprolol.

Home Medications:
- Metoprolol succinate 25 mg oral daily
- Tiotropium 18 mcg inhaled daily
- Albuterol sulfate 90 mcg inhaler, 2 puffs every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Deceased at age 61 from acute myocardial infarction
- Mother: Hypertension and osteoporosis, age 91
- One older brother: Coronary artery disease, status post coronary artery bypass grafting

SOCIAL HISTORY:
Smoking: Former smoker, 50 pack-year history, quit 2 years ago. Alcohol: Occasional (1-2 drinks per month). Occupation: Retired long-haul truck driver.

VITAL SIGNS:
With a blood pressure of 85/50 mmHg
with a respiratory rate of 28 breaths per minute

PHYSICAL EXAMINATION:
On examination, he is tachypneic
jugular venous pressure is elevated, and there are bilateral crackles halfway up both lung fields. Heart sounds include a third heart sound (S3) gallop. Extremities are cool to touch, and there is 1+ pitting edema in both lower legs.

LABORATORY DATA:
Laboratory results are as follows:

- Troponin I: 8 ng/mL (normal <0.04 ng/mL)
- B-type natriuretic peptide (BNP): 900 pg/mL (normal <100 pg/mL)
- Arterial blood gas: pH 7.32, PaCO2 45 mmHg, PaO2 60 mmHg, HCO3- 22 mEq/L
- Complete blood count and basic metabolic panel are within normal limits.

OTHER STUDIES:
An ECG shows no new changes compared to preoperative records.
A pulmonary artery catheterization is performed, revealing a cardiac index of 1.8 L/min/m^2 (normal 2.5-4.0 L/min/m^2) and a pulmonary capillary wedge pressure of 28 mmHg (normal 6-12 mmHg).

## Encounter 2: source 2021-05-15, practice 2026-02-12, ed, Emergency Medicine

Chief complaint: Dizziness and fatigue two days after hospital discharge for COPD exacerbation

ENCOUNTER NOTE
Date: 2021-05-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Dizziness and fatigue two days after hospital discharge for COPD exacerbation

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old male with COPD and coronary artery disease with prior papillary muscle rupture and cardiogenic shock. He is being discharged from the hospital after a 5-day admission for a COPD exacerbation. During his hospital stay, his metoprolol dose was increased due to episodes of tachycardia. His discharge medications are reviewed with him, and he is given prescriptions to fill at his community pharmacy. Two days later, he presents to the emergency department with dizziness and fatigue.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute mitral regurgitation due to papillary muscle rupture (diagnosed 2020-01-15)
- Cardiogenic Shock (diagnosed 2020-09-15)
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) Group C
- Hypertension
- Hyperlipidemia
- Coronary artery disease, status post myocardial infarction

PAST SURGICAL HISTORY:
- Mitral valve repair with bioprosthetic ring annuloplasty (complicated by papillary muscle rupture following acute myocardial infarction)
- Cardiac catheterization with percutaneous coronary intervention and drug-eluting stent placement to left anterior descending artery
- Diagnostic bronchoscopy with bronchoalveolar lavage (during prior admission)

MEDICATIONS:
He has been stabilized on oral prednisone, tiotropium, and short-acting bronchodilators. His cardiac medications include aspirin, atorvastatin, and metoprolol.

Home Medications:
- Metoprolol succinate 25 mg oral daily
- Albuterol sulfate 90 mcg inhaler, 2 puffs every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Deceased at age 61 from acute myocardial infarction
- Mother: Hypertension and osteoporosis, age 91
- One older brother: Coronary artery disease, status post coronary artery bypass grafting

SOCIAL HISTORY:
Smoking: Former smoker, 50 pack-year history, quit 2 years ago. Alcohol: Occasional (1-2 drinks per month). Occupation: Retired long-haul truck driver.

VITAL SIGNS:
Blood pressure is 95/60 mmHg, heart rate is 48 beats per minute, and respiratory rate is 20 breaths per minute

PHYSICAL EXAMINATION:
Physical examination is unremarkable except for bradycardia

LABORATORY DATA:
Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 1.0 mg/dL
- Glucose: 98 mg/dL
- Hemoglobin: 14 g/dL
