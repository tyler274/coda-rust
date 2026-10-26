# synthetic_hospital patient 2606

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 22 at first encounter, sex M, Non-Hispanic White, Parental private insurance (PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-05-18 (Tue) | outpatient | Medical Genetics |
| 1 | 2022-01-15 | 2023-05-16 (Tue) | outpatient | Cardiology |
| 2 | 2023-01-15 | 2024-05-04 (Sat) | ed | Emergency Medicine |
| 3 | 2023-07-15 | 2024-11-14 (Thu) | ed | Emergency Medicine |
| 4 | 2025-01-15 | 2026-05-19 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Parental private insurance (PPO)",
 "occupation": "College student (Business major, intramural basketball)",
 "alcohol_use": "None",
 "family_history": [
  "Maternal grandfather: End-stage renal disease, deceased at age 45",
  "Maternal uncle: Premature cardiovascular disease with neuropathic pain, deceased at age 38",
  "Mother: Fabry disease carrier, mild acroparesthesias and hypohidrosis",
  "Father: Healthy",
  "No siblings"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 mg/kg IV every 2 weeks",
   "name": "Agalsidase beta"
  },
  {
   "dose": "20 mg PO three times daily",
   "name": "Sildenafil"
  },
  {
   "dose": "62.5 mg PO twice daily",
   "name": "Bosentan"
  },
  {
   "dose": "100 mg PO twice daily",
   "name": "Flecainide"
  },
  {
   "dose": "5 mg PO twice daily",
   "name": "Apixaban"
  },
  {
   "dose": "300 mg PO three times daily",
   "name": "Gabapentin"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Anderson-Fabry disease (alpha-galactosidase A deficiency)",
  "Congenital ventricular septal defect (unrepaired)",
  "Eisenmenger syndrome",
  "Hypertrophic cardiomyopathy with outflow tract obstruction",
  "Wolff-Parkinson-White syndrome",
  "Pulmonary arterial hypertension",
  "Small fiber peripheral neuropathy",
  "Paroxysmal atrial fibrillation"
 ],
 "age_at_first_encounter": 22
}
```

Primary diagnoses: ["Hypertrophic obstructive cardiomyopathy", "Atrial Fibrillation with Wolff-Parkinson-White Syndrome", "Fabry disease", "Eisenmenger syndrome"]
Comorbidities: ["Left ventricular hypertrophy", "Family history of premature coronary artery disease"]

## Encounter 0: source 2020-01-15, practice 2021-05-18, outpatient, Medical Genetics

Chief complaint: Recurrent burning pain in hands and feet with reduced sweating and clustered red papules in groin

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Medical Genetics
Visit Type: outpatient

CHIEF COMPLAINT:
Recurrent burning pain in hands and feet with reduced sweating and clustered red papules in groin

HISTORY OF PRESENT ILLNESS:
He presents with a 7-year history of recurrent burning pain in his hands and feet, exacerbated by exercise and heat. He reports reduced sweating since his teens, causing frequent overheating during minimal activity. For the past 3 years, he has noticed clustered red papules in his groin and lower abdomen shown in the image below. Recently, he developed fatigue, ankle swelling, and shortness of breath climbing stairs.

PAST MEDICAL HISTORY:
Active Problem List:
- Anderson-Fabry disease (alpha-galactosidase A deficiency)
- Congenital ventricular septal defect (unrepaired)
- Eisenmenger syndrome
- Hypertrophic cardiomyopathy with outflow tract obstruction
- Wolff-Parkinson-White syndrome
- Pulmonary arterial hypertension
- Small fiber peripheral neuropathy
- Paroxysmal atrial fibrillation

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Agalsidase beta 1 mg/kg IV every 2 weeks
- Sildenafil 20 mg PO three times daily
- Bosentan 62.5 mg PO twice daily
- Flecainide 100 mg PO twice daily
- Apixaban 5 mg PO twice daily
- Gabapentin 300 mg PO three times daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal grandfather: End-stage renal disease, deceased at age 45
- Maternal uncle: Premature cardiovascular disease with neuropathic pain, deceased at age 38
- Mother: Fabry disease carrier, mild acroparesthesias and hypohidrosis
- Father: Healthy
- No siblings

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: College student (Business major, intramural basketball).

PHYSICAL EXAMINATION:
Examination reveals diminished light touch sensation in gloves-and-stocking distribution, and 2+ pedal edema. Cardiac auscultation discloses an S4 gallop.

LABORATORY DATA:
Urinalysis shows 3+ proteinuria without hematuria. Serum creatinine is elevated at 1.9 mg/dL, but HbA1c and ANA are normal.

IMAGING:
Echocardiography demonstrates left ventricular hypertrophy with diastolic dysfunction

## Encounter 1: source 2022-01-15, practice 2023-05-16, outpatient, Cardiology

Chief complaint: Progressive shortness of breath, fatigue, and bluish discoloration of lips and fingertips

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath, fatigue, and bluish discoloration of lips and fingertips

HISTORY OF PRESENT ILLNESS:
The patient returns for cardiology follow-up of Anderson-Fabry disease and congenital heart disease. Over the past year, he also notes bluish discoloration of his lips and fingertips, worsening with exertion. The patient's symptoms have progressed despite no prior cardiac interventions.

PAST MEDICAL HISTORY:
Active Problem List:
- Fabry disease (diagnosed 2020-01-15)
- Anderson-Fabry disease (alpha-galactosidase A deficiency)
- Congenital ventricular septal defect (unrepaired)
- Eisenmenger syndrome
- Hypertrophic cardiomyopathy with outflow tract obstruction
- Wolff-Parkinson-White syndrome
- Pulmonary arterial hypertension
- Small fiber peripheral neuropathy
- Paroxysmal atrial fibrillation

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Agalsidase beta 1 mg/kg IV every 2 weeks
- Sildenafil 20 mg PO three times daily
- Bosentan 62.5 mg PO twice daily
- Flecainide 100 mg PO twice daily
- Apixaban 5 mg PO twice daily
- Gabapentin 300 mg PO three times daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal grandfather: End-stage renal disease, deceased at age 45
- Maternal uncle: Premature cardiovascular disease with neuropathic pain, deceased at age 38
- Mother: Fabry disease carrier, mild acroparesthesias and hypohidrosis
- Father: Healthy
- No siblings

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: College student (Business major, intramural basketball).

VITAL SIGNS:
On exam, his oxygen saturation is 88% on room air, blood pressure is 110/70 mm Hg, and heart rate is 92/min.

PHYSICAL EXAMINATION:
He has central cyanosis, digital clubbing, and jugular venous distention. Cardiac auscultation reveals a harsh holosystolic murmur at the lower left sternal border and a loud, palpable second heart sound. Lung exam reveals clear breath sounds.

IMAGING:
Chest X-ray is shown below.

OTHER STUDIES:
ECG shows right bundle branch block and right axis deviation.

## Encounter 2: source 2023-01-15, practice 2024-05-04, ed, Emergency Medicine

Chief complaint: Syncopal episode during basketball practice preceded by palpitations and chest pain

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Syncopal episode during basketball practice preceded by palpitations and chest pain

HISTORY OF PRESENT ILLNESS:
This 25-year-old male with Anderson-Fabry disease and Eisenmenger syndrome, last evaluated in 2022 for progressive dyspnea and cyanosis, presents to the emergency department following syncope. He reports that the episode was preceded by palpitations and chest pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Fabry disease (diagnosed 2020-01-15)
- Eisenmenger syndrome (diagnosed 2022-01-15)
- Anderson-Fabry disease (alpha-galactosidase A deficiency)
- Congenital ventricular septal defect (unrepaired)
- Hypertrophic cardiomyopathy with outflow tract obstruction
- Wolff-Parkinson-White syndrome
- Pulmonary arterial hypertension
- Small fiber peripheral neuropathy
- Paroxysmal atrial fibrillation

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Agalsidase beta 1 mg/kg IV every 2 weeks
- Sildenafil 20 mg PO three times daily
- Bosentan 62.5 mg PO twice daily
- Flecainide 100 mg PO twice daily
- Apixaban 5 mg PO twice daily
- Gabapentin 300 mg PO three times daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His father died suddenly at the age of 42 from an unknown cause.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: College student (Business major, intramural basketball).

VITAL SIGNS:
On examination, his blood pressure is 130/85 mmHg, heart rate is 78 bpm

PHYSICAL EXAMINATION:
He is now alert and oriented. Cardiac auscultation reveals a harsh crescendo-decrescendo murmur at the lower left sternal border that increases in intensity with the Valsalva maneuver.

IMAGING:
An echocardiogram demonstrates asymmetric septal hypertrophy and systolic anterior motion of the mitral valve leaflet.

OTHER STUDIES:
An ECG shows left ventricular hypertrophy with deep, narrow Q waves in the lateral leads.

## Encounter 3: source 2023-07-15, practice 2024-11-14, ed, Emergency Medicine

Chief complaint: Sudden palpitations, dizziness, and near-syncope lasting 15 minutes while playing basketball

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden palpitations, dizziness, and near-syncope lasting 15 minutes while playing basketball

HISTORY OF PRESENT ILLNESS:
The patient is a 25-year-old male with Anderson-Fabry disease, Eisenmenger syndrome, hypertrophic cardiomyopathy, and Wolff-Parkinson-White syndrome who is brought to the emergency department six months after a prior basketball-related syncopal episode with a sudden episode of palpitations, dizziness, and near-syncope that began while playing basketball. He describes his heart as "pounding out of his chest" and states the episode lasted approximately 15 minutes before spontaneously resolving.

PAST MEDICAL HISTORY:
Active Problem List:
- Fabry disease (diagnosed 2020-01-15)
- Eisenmenger syndrome (diagnosed 2022-01-15)
- Hypertrophic obstructive cardiomyopathy (diagnosed 2023-01-15)
- Anderson-Fabry disease (alpha-galactosidase A deficiency)
- Congenital ventricular septal defect (unrepaired)
- Hypertrophic cardiomyopathy with outflow tract obstruction
- Wolff-Parkinson-White syndrome
- Pulmonary arterial hypertension
- Small fiber peripheral neuropathy
- Paroxysmal atrial fibrillation

He has no known medical history.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Agalsidase beta 1 mg/kg IV every 2 weeks
- Sildenafil 20 mg PO three times daily
- Bosentan 62.5 mg PO twice daily
- Flecainide 100 mg PO twice daily
- Apixaban 5 mg PO twice daily
- Gabapentin 300 mg PO three times daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His father died suddenly at the age of 35

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: College student (Business major, intramural basketball).

VITAL SIGNS:
Heart rate of 82/min and blood pressure of 124/76 mm Hg

PHYSICAL EXAMINATION:
On exam, he is alert and oriented. Physical exam is unremarkable.

LABORATORY DATA:
Laboratory studies including electrolytes and cardiac enzymes are within normal limits

OTHER STUDIES:
ECG performed during evaluation shows a short PR interval and a slurred upstroke in the QRS complex, which resolves after the episode

## Encounter 4: source 2025-01-15, practice 2026-05-19, ed, Emergency Medicine

Chief complaint: Collapse while running on treadmill at the gym requiring revival

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Emily Watson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Collapse while running on treadmill at the gym requiring revival

HISTORY OF PRESENT ILLNESS:
The patient with known Anderson-Fabry disease and hypertrophic cardiomyopathy presents following exertional collapse similar to 2023 episodes. He is quickly revived and brought to the emergency department. He reports several months of occasional shortness of breath during exercise but has not previously sought care.

PAST MEDICAL HISTORY:
Active Problem List:
- Fabry disease (diagnosed 2020-01-15)
- Eisenmenger syndrome (diagnosed 2022-01-15)
- Hypertrophic obstructive cardiomyopathy (diagnosed 2023-01-15)
- Atrial Fibrillation with Wolff-Parkinson-White Syndrome (diagnosed 2023-07-15)
- Anderson-Fabry disease (alpha-galactosidase A deficiency)
- Congenital ventricular septal defect (unrepaired)
- Hypertrophic cardiomyopathy with outflow tract obstruction
- Wolff-Parkinson-White syndrome
- Pulmonary arterial hypertension
- Small fiber peripheral neuropathy
- Paroxysmal atrial fibrillation

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Agalsidase beta 1 mg/kg IV every 2 weeks
- Sildenafil 20 mg PO three times daily
- Bosentan 62.5 mg PO twice daily
- Flecainide 100 mg PO twice daily
- Apixaban 5 mg PO twice daily
- Gabapentin 300 mg PO three times daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal grandfather: End-stage renal disease, deceased at age 45
- Maternal uncle: Premature cardiovascular disease with neuropathic pain, deceased at age 38
- Mother: Fabry disease carrier, mild acroparesthesias and hypohidrosis
- Father: Healthy
- No siblings

SOCIAL HISTORY:
He does not use alcohol, tobacco, or drugs.

VITAL SIGNS:
On physical examination, his pulse is 86/min, blood pressure is 118/72 mm Hg, respiratory rate is 16/min, and temperature is 36.7°C (98.1°F).

PHYSICAL EXAMINATION:
A crescendo-decrescendo systolic murmur is heard along the left sternal border that becomes softer when the patient squats. There is no radiation of the murmur to the neck. His carotid upstroke is brisk and bounding.
