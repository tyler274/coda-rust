# synthetic_hospital patient 2087

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-09-29 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-02-15 | 2025-10-22 (Wed) | ed | Emergency Medicine |
| 2 | 2020-03-15 | 2025-11-28 (Fri) | inpatient | Cardiology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Construction laborer (currently unemployed)",
 "alcohol_use": "Social (2-3 drinks per week)",
 "family_history": [
  "Father: Alcohol use disorder with cirrhosis, deceased age 60",
  "Mother: Hypertension, type 2 diabetes mellitus",
  "One brother: Opioid use disorder in sustained remission"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 1 pack per day for 20 years",
 "home_medications": [
  {
   "dose": "500 mg PO every 6 hours PRN pain",
   "name": "Acetaminophen"
  },
  {
   "dose": "25 mg PO at bedtime PRN insomnia",
   "name": "Diphenhydramine"
  }
 ],
 "surgical_history": [
  "Incision and drainage of left forearm abscess (2 years prior)",
  "Knee arthroscopy for meniscal tear (age 38)"
 ],
 "chronic_conditions": [
  "Opioid use disorder, severe, active",
  "Chronic hepatitis C infection (untreated, genotype 1a)",
  "Chronic lower back pain secondary to degenerative disc disease",
  "Tobacco use disorder, severe"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Infective Endocarditis (Aortic Valve) with Complications"]
Comorbidities: ["Sinus tachycardia secondary to burns", "Intravenous Drug Use (Substance Use Disorder)"]

## Encounter 0: source 2020-01-15, practice 2025-09-29, outpatient, Internal Medicine

Chief complaint: Low-grade fever, fatigue, and diffuse myalgias

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Low-grade fever, fatigue, and diffuse myalgias

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 3-week history of low-grade fever, fatigue, and diffuse myalgias. He denies any recent travel or sick contacts.

PAST MEDICAL HISTORY:
Active Problem List:
- Opioid use disorder, severe, active
- Chronic hepatitis C infection (untreated, genotype 1a)
- Chronic lower back pain secondary to degenerative disc disease
- Tobacco use disorder, severe

PAST SURGICAL HISTORY:
- Incision and drainage of left forearm abscess (2 years prior)
- Knee arthroscopy for meniscal tear (age 38)

MEDICATIONS:
Home Medications:
- Acetaminophen 500 mg PO every 6 hours PRN pain
- Diphenhydramine 25 mg PO at bedtime PRN insomnia

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis, deceased age 60
- Mother: Hypertension, type 2 diabetes mellitus
- One brother: Opioid use disorder in sustained remission

SOCIAL HISTORY:
History of intravenous drug use

PHYSICAL EXAMINATION:
On examination, he appears ill and has a low-grade fever of 100.4°F (38°C). A new grade 3/6 systolic murmur is heard at the left lower sternal border. There are no signs of heart failure. His skin examination reveals a few painless erythematous macules on the palms and soles.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 12,000 /µL
- Hemoglobin: 11.2 g/dL
- Platelet count: 250,000 /µL
- Erythrocyte sedimentation rate (ESR): 85 mm/hr
- C-reactive protein (CRP): 25 mg/L
- Serum creatinine: 1.8 mg/dL (baseline 0.9 mg/dL)
- Urinalysis: Red blood cell casts, no bacteria

## Encounter 1: source 2020-02-15, practice 2025-10-22, ed, Emergency Medicine

Chief complaint: 3-week history of fever, night sweats, and a 5-pound weight loss

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. Michael Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
3-week history of fever, night sweats, and a 5-pound weight loss

PAST MEDICAL HISTORY:
Active Problem List:
- Infective Endocarditis (Aortic Valve) with Complications (diagnosed 2020-01-15)
- Opioid use disorder, severe, active
- Chronic hepatitis C infection (untreated, genotype 1a)
- Chronic lower back pain secondary to degenerative disc disease
- Tobacco use disorder, severe

PAST SURGICAL HISTORY:
- Incision and drainage of left forearm abscess (2 years prior)
- Knee arthroscopy for meniscal tear (age 38)

MEDICATIONS:
Home Medications:
- Acetaminophen 500 mg PO every 6 hours PRN pain
- Diphenhydramine 25 mg PO at bedtime PRN insomnia

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis, deceased age 60
- Mother: Hypertension, type 2 diabetes mellitus
- One brother: Opioid use disorder in sustained remission

SOCIAL HISTORY:
History of intravenous drug use

REVIEW OF SYSTEMS:
He denies cough, hemoptysis, or dyspnea.

VITAL SIGNS:
His temperature is 38.5°C, blood pressure is 130/85 mmHg, pulse is 100 bpm, and respiratory rate is 18 breaths per minute.

PHYSICAL EXAMINATION:
Physical examination reveals a grade 2/6 systolic murmur best heard at the left lower sternal border. There are no signs of peripheral edema or jugular venous distention.

LABORATORY DATA:
Laboratory tests are as follows:

- White blood cell count: 12,000/mm³
- Hemoglobin: 13.5 g/dL
- Platelet count: 250,000/mm³
- Erythrocyte sedimentation rate (ESR): 85 mm/hr
- Blood cultures: No growth at 48 hours

IMAGING:
A transthoracic echocardiogram shows small vegetations on the tricuspid valve, but the test is reported as having a low sensitivity for endocarditis in this clinical context.

## Encounter 2: source 2020-03-15, practice 2025-11-28, inpatient, Cardiology

Chief complaint: fever, night sweats, and a 4 kg weight loss

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Elena Vasquez, MD
Department: Cardiology
Visit Type: inpatient

CHIEF COMPLAINT:
Fever, night sweats, and a 4 kg weight loss

HISTORY OF PRESENT ILLNESS:
Patient returns for evaluation following assessments in January and February 2020 and presents to the clinic with a 3-week history of fever, night sweats, and a 4 kg weight loss. He denies any cough or dyspnea.

PAST MEDICAL HISTORY:
Active Problem List:
- Infective Endocarditis (Aortic Valve) with Complications (diagnosed 2020-01-15)
- Opioid use disorder, severe, active
- Chronic hepatitis C infection (untreated, genotype 1a)
- Chronic lower back pain secondary to degenerative disc disease
- Tobacco use disorder, severe

PAST SURGICAL HISTORY:
- Incision and drainage of left forearm abscess (2 years prior)
- Knee arthroscopy for meniscal tear (age 38)

MEDICATIONS:
Home Medications:
- Acetaminophen 500 mg PO every 6 hours PRN pain
- Diphenhydramine 25 mg PO at bedtime PRN insomnia

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis, deceased age 60
- Mother: Hypertension, type 2 diabetes mellitus
- One brother: Opioid use disorder in sustained remission

SOCIAL HISTORY:
History of intravenous drug use

VITAL SIGNS:
His temperature is 38.5°C, blood pressure is 125/75 mmHg, pulse is 110 bpm, and respiratory rate is 18 breaths/min.

PHYSICAL EXAMINATION:
Physical examination reveals a new systolic murmur best heard at the left lower sternal border. His skin examination shows scattered petechiae.

LABORATORY DATA:
White blood cell count: 11,000/µL
- Hemoglobin: 12.5 g/dL
- Platelet count: 150,000/µL
- Erythrocyte sedimentation rate: 85 mm/hr
- Blood cultures: Pending

IMAGING:
An echocardiogram shows vegetations on the tricuspid valve.

ASSESSMENT AND PLAN:
Given the patient's risk factors and clinical presentation, you consider infective endocarditis

Order additional blood cultures. While awaiting these results, you consider the utility of the blood cultures in confirming the diagnosis.
