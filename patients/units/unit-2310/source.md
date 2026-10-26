# synthetic_hospital patient 2310

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-07-25 (Mon) | outpatient | Cardiology |
| 1 | 2020-09-15 | 2023-03-20 (Mon) | outpatient | Internal Medicine |
| 2 | 2023-07-15 | 2026-01-22 (Thu) | ed | Emergency Medicine |
| 3 | 2023-07-15 | 2026-01-23 (Fri) | icu | Pulmonary & Critical Care |
| 4 | 2023-09-15 | 2026-03-16 (Mon) | follow_up | Otolaryngology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Aminoglycoside antibiotics (ototoxicity)",
  "Anticholinergic medications (digital ischemia)"
 ],
 "insurance": "Medicare",
 "occupation": "Retired postal service worker",
 "alcohol_use": "Occasional, 1-2 drinks per week",
 "family_history": [
  "Father: Myocardial infarction at age 62, hypertension",
  "Mother: Stroke at age 70, type 2 diabetes mellitus",
  "Older brother: Coronary artery disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 30 pack-years, quit 5 years ago",
 "home_medications": [
  {
   "dose": "81 mg daily",
   "name": "Aspirin"
  },
  {
   "dose": "40 mg daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "10 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "12.5 mg twice daily",
   "name": "Carvedilol"
  },
  {
   "dose": "25 mg daily",
   "name": "Spironolactone"
  },
  {
   "dose": "40 mg twice daily",
   "name": "Furosemide"
  }
 ],
 "surgical_history": [
  "Implantable cardioverter-defibrillator placement",
  "Intra-aortic balloon pump placement",
  "Mechanical ventilation via endotracheal intubation"
 ],
 "chronic_conditions": [
  "Ischemic cardiomyopathy with reduced ejection fraction",
  "Chronic heart failure, NYHA Class III",
  "Chronic kidney disease stage 3",
  "Status post myocardial infarction",
  "Tricuspid regurgitation, severe secondary",
  "Coronary artery disease",
  "Hypertension",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Adverse effect of anticholinergic medication", "Aminoglycoside-induced ototoxicity screening", "Tricuspid Regurgitation", "Cardiogenic Shock", "Uncomplicated parapneumonic effusion"]
Comorbidities: ["Acute decompensated heart failure"]

## Encounter 0: source 2020-01-15, practice 2022-07-25, outpatient, Cardiology

Chief complaint: Progressive dyspnea on exertion, fatigue, and lower extremity edema over the past two months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive dyspnea on exertion, fatigue, and lower extremity edema over the past two months

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with progressive dyspnea on exertion, fatigue, and lower extremity edema over the past two months. He denies chest pain, palpitations, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Ischemic cardiomyopathy with reduced ejection fraction
- Chronic heart failure, NYHA Class III
- Chronic kidney disease stage 3
- Status post myocardial infarction
- Tricuspid regurgitation, severe secondary
- Coronary artery disease
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Implantable cardioverter-defibrillator placement
- Intra-aortic balloon pump placement
- Mechanical ventilation via endotracheal intubation

MEDICATIONS:
He takes aspirin, lisinopril, carvedilol, and atorvastatin.

Home Medications:
- Spironolactone 25 mg daily
- Furosemide 40 mg twice daily

ALLERGIES:
Aminoglycoside antibiotics (ototoxicity), Anticholinergic medications (digital ischemia)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Stroke at age 70, type 2 diabetes mellitus
- Older brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 5 years ago. Alcohol: Occasional, 1-2 drinks per week. Occupation: Retired postal service worker.

VITAL SIGNS:
Blood pressure is 130/80 mmHg, heart rate is 70 beats per minute, and oxygen saturation is 96% on room air

PHYSICAL EXAMINATION:
Jugular venous pressure is elevated at 10 cm H2O. Cardiac auscultation reveals a holosystolic murmur best heard at the left lower sternal border that increases with inspiration. There is 2+ pitting edema in both lower extremities up to the mid-shins.

LABORATORY DATA:
- Hemoglobin: 13.5 g/dL
- Serum creatinine: 1.2 mg/dL
- B-type natriuretic peptide (BNP): 550 pg/mL (elevated)
- Liver function tests: Normal

IMAGING:
A transthoracic echocardiogram shows normal left ventricular ejection fraction, dilated right atrium and ventricle, and severe tricuspid regurgitation with the presence of a transvenous ICD lead crossing the tricuspid valve.

## Encounter 1: source 2020-09-15, practice 2023-03-20, outpatient, Internal Medicine

Chief complaint: Progressive shortness of breath and swelling in both legs with difficulty lying flat over two months

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Marcus Johnson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath and swelling in both legs with difficulty lying flat over two months

HISTORY OF PRESENT ILLNESS:
The patient with ischemic cardiomyopathy returns for follow-up eight months after prior cardiology assessment for similar symptoms. He presents with progressive shortness of breath and swelling in both legs over the past two months. He reports difficulty lying flat and occasional awakening at night due to breathlessness.

PAST MEDICAL HISTORY:
Active Problem List:
- Tricuspid Regurgitation (diagnosed 2020-01-15)
- Ischemic cardiomyopathy with reduced ejection fraction
- Chronic heart failure, NYHA Class III
- Chronic kidney disease stage 3
- Status post myocardial infarction
- Tricuspid regurgitation, severe secondary
- Coronary artery disease
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Implantable cardioverter-defibrillator placement
- Intra-aortic balloon pump placement
- Mechanical ventilation via endotracheal intubation

MEDICATIONS:
Home Medications:
- Aspirin 81 mg daily
- Atorvastatin 40 mg daily
- Lisinopril 10 mg daily
- Carvedilol 12.5 mg twice daily
- Spironolactone 25 mg daily
- Furosemide 40 mg twice daily

ALLERGIES:
Aminoglycoside antibiotics (ototoxicity), Anticholinergic medications (digital ischemia)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Stroke at age 70, type 2 diabetes mellitus
- Older brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 5 years ago. Alcohol: Occasional, 1-2 drinks per week. Occupation: Retired postal service worker.

PHYSICAL EXAMINATION:
Physical examination shows elevated jugular venous pressure, bilateral pitting edema, and decreased breath sounds with dullness to percussion at the lung bases bilaterally.

IMAGING:
A chest X-ray is shown below and a diagnostic procedure is performed.

## Encounter 2: source 2023-07-15, practice 2026-01-22, ed, Emergency Medicine

Chief complaint: Sudden-onset, crushing substernal chest pain radiating to the left arm beginning 45 minutes ago while resting

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Emily Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset, crushing substernal chest pain radiating to the left arm beginning 45 minutes ago while resting

HISTORY OF PRESENT ILLNESS:
The patient is a 70-year-old male with known ischemic cardiomyopathy with reduced ejection fraction, status post myocardial infarction, and severe secondary tricuspid regurgitation who was previously evaluated in 2020 for progressive dyspnea and lower extremity edema. He presents with sudden-onset, crushing substernal chest pain radiating to the left arm that began 45 minutes ago while he was resting.

PAST MEDICAL HISTORY:
Active Problem List:
- Tricuspid Regurgitation (diagnosed 2020-01-15)
- Uncomplicated parapneumonic effusion (diagnosed 2020-09-15)
- Ischemic cardiomyopathy with reduced ejection fraction
- Chronic heart failure, NYHA Class III
- Chronic kidney disease stage 3
- Status post myocardial infarction
- Tricuspid regurgitation, severe secondary
- Coronary artery disease
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Implantable cardioverter-defibrillator placement
- Intra-aortic balloon pump placement
- Mechanical ventilation via endotracheal intubation

MEDICATIONS:
Home Medications:
- Aspirin 81 mg daily
- Atorvastatin 40 mg daily
- Lisinopril 10 mg daily
- Carvedilol 12.5 mg twice daily
- Spironolactone 25 mg daily
- Furosemide 40 mg twice daily

ALLERGIES:
Aminoglycoside antibiotics (ototoxicity), Anticholinergic medications (digital ischemia)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Stroke at age 70, type 2 diabetes mellitus
- Older brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 5 years ago. Alcohol: Occasional, 1-2 drinks per week. Occupation: Retired postal service worker.

VITAL SIGNS:
Vital signs show a blood pressure of 85/50 mm Hg and heart rate of 112/min.

PHYSICAL EXAMINATION:
He appears pale and diaphoretic. Auscultation reveals crackles at the lung bases.

OTHER STUDIES:
ECG reveals ST-segment elevations in leads V2–V5.

PLAN:
He is started on oxygen and IV access is established.

## Encounter 3: source 2023-07-15, practice 2026-01-23, icu, Pulmonary & Critical Care

Chief complaint: Black discoloration on the tips of fingers on both hands while recovering from cardiogenic shock

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. James Park, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Black discoloration on the tips of fingers on both hands while recovering from cardiogenic shock

HISTORY OF PRESENT ILLNESS:
This 70-year-old man with ischemic cardiomyopathy with reduced ejection fraction, chronic heart failure NYHA Class III, and severe secondary tricuspid regurgitation, previously evaluated in 2020 for progressive dyspnea and parapneumonic effusion, is admitted to the intensive care unit for management of cardiogenic shock in the setting of chronic heart failure and recent myocardial infarction. He is currently on mechanical ventilation and an intra-aortic balloon pump has been placed. Despite these measures, his blood pressure remains critically low, and he is started on a norepinephrine infusion. Three days later, the nurse reports that the patient has developed black discoloration on the tips of his fingers on both hands.

PAST MEDICAL HISTORY:
Active Problem List:
- Tricuspid Regurgitation (diagnosed 2020-01-15)
- Uncomplicated parapneumonic effusion (diagnosed 2020-09-15)
- Cardiogenic Shock (diagnosed 2023-07-15)
- Ischemic cardiomyopathy with reduced ejection fraction
- Chronic heart failure, NYHA Class III
- Chronic kidney disease stage 3
- Status post myocardial infarction
- Tricuspid regurgitation, severe secondary
- Coronary artery disease
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Implantable cardioverter-defibrillator placement
- Intra-aortic balloon pump placement
- Mechanical ventilation via endotracheal intubation

MEDICATIONS:
His current medications include aspirin, atorvastatin, furosemide, and metoprolol.

Home Medications:
- Lisinopril 10 mg daily
- Carvedilol 12.5 mg twice daily
- Spironolactone 25 mg daily

ALLERGIES:
Aminoglycoside antibiotics (ototoxicity), Anticholinergic medications (digital ischemia)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Stroke at age 70, type 2 diabetes mellitus
- Older brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 5 years ago. Alcohol: Occasional, 1-2 drinks per week. Occupation: Retired postal service worker.

PHYSICAL EXAMINATION:
On examination, the patient's distal digits are cold, with clear demarcation between healthy tissue and the areas of black eschar. Pulses are not palpable in the affected digits. The remainder of the cardiovascular examination is unremarkable.

## Encounter 4: source 2023-09-15, practice 2026-03-16, follow_up, Otolaryngology

Chief complaint: Progressive bilateral hearing loss over the past two weeks following recent hospitalization

ENCOUNTER NOTE
Date: 2023-09-15
Provider: Dr. Aisha Patel, MD
Department: Otolaryngology
Visit Type: follow_up

CHIEF COMPLAINT:
Progressive bilateral hearing loss over the past two weeks following recent hospitalization

HISTORY OF PRESENT ILLNESS:
Patient with ischemic cardiomyopathy returns for hearing loss following recent hospitalization. He denies any ear pain, discharge, or recent infections. He was recently hospitalized for acute decompensated heart failure and received intravenous furosemide.

PAST MEDICAL HISTORY:
Active Problem List:
- Tricuspid Regurgitation (diagnosed 2020-01-15)
- Uncomplicated parapneumonic effusion (diagnosed 2020-09-15)
- Cardiogenic Shock (diagnosed 2023-07-15)
- Adverse effect of anticholinergic medication (diagnosed 2023-07-15)
- Ischemic cardiomyopathy with reduced ejection fraction
- Chronic heart failure, NYHA Class III
- Chronic kidney disease stage 3
- Status post myocardial infarction
- Tricuspid regurgitation, severe secondary
- Coronary artery disease
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Implantable cardioverter-defibrillator placement
- Intra-aortic balloon pump placement
- Mechanical ventilation via endotracheal intubation

MEDICATIONS:
His current medications include lisinopril, carvedilol, spironolactone, and furosemide.

Home Medications:
- Aspirin 81 mg daily
- Atorvastatin 40 mg daily

ALLERGIES:
Aminoglycoside antibiotics (ototoxicity), Anticholinergic medications (digital ischemia)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Stroke at age 70, type 2 diabetes mellitus
- Older brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 5 years ago. Alcohol: Occasional, 1-2 drinks per week. Occupation: Retired postal service worker.

VITAL SIGNS:
His blood pressure is 142/88 mmHg, heart rate is 78 bpm, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
Physical examination reveals bilateral decreased air entry at the lung bases and 2+ pitting edema in the lower extremities.

LABORATORY DATA:
Laboratory values are as follows:

- Serum creatinine: 2.1 mg/dL (increased from baseline of 1.5 mg/dL)
- Blood urea nitrogen (BUN): 44 mg/dL
- Potassium: 4.8 mEq/L
- Sodium: 138 mEq/L
- Glomerular filtration rate (GFR): 35 mL/min/1.73m^2
