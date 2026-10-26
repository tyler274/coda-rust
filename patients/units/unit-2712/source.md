# synthetic_hospital patient 2712

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 52 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-12-06 (Tue) | outpatient | Endocrinology |
| 1 | 2020-04-15 | 2023-03-07 (Tue) | outpatient | Cardiology |
| 2 | 2020-07-15 | 2023-06-09 (Fri) | outpatient | Urology |
| 3 | 2020-10-15 | 2023-09-12 (Tue) | outpatient | Neurology |
| 4 | 2021-01-15 | 2023-12-08 (Fri) | outpatient | Internal Medicine |
| 5 | 2022-01-15 | 2024-12-02 (Mon) | ed | Emergency Medicine |
| 6 | 2022-03-15 | 2025-01-29 (Wed) | outpatient | Dermatology |
| 7 | 2023-01-15 | 2025-12-06 (Sat) | inpatient | Nephrology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Hydralazine (drug-induced systemic lupus erythematosus)",
  "Contrast dye (anaphylaxis)"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Administrative assistant",
 "alcohol_use": "Social, 1-2 drinks monthly",
 "family_history": [
  "Mother: Hypertension, osteoporosis, primary hyperparathyroidism",
  "Father: Nephrolithiasis",
  "Sister: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg oral daily",
   "name": "Amlodipine"
  },
  {
   "dose": "25 mg oral daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "20 mg oral daily",
   "name": "Lisinopril"
  }
 ],
 "surgical_history": [
  "Left nephrectomy (age 50, complications of chronic pyelonephritis)",
  "Vaginal delivery x2 (ages 32 and 34)",
  "Laparoscopic cholecystectomy (age 45)"
 ],
 "chronic_conditions": [
  "Resistant hypertension",
  "Primary hyperparathyroidism",
  "History of recurrent nephrolithiasis",
  "History of recurrent urinary tract infections",
  "Status post left nephrectomy",
  "Stress urinary incontinence",
  "Benign essential blepharospasm"
 ],
 "age_at_first_encounter": 52
}
```

Primary diagnoses: ["Primary hyperparathyroidism", "Hypocalciuria", "Stress urinary incontinence", "Proteus mirabilis urinary infection", "Lichenoid drug eruption or cutaneous lymphoma (requiring biopsy)", "Blepharospasm", "Drug-induced systemic lupus erythematosus"]
Comorbidities: ["Urolithiasis (Renal Colic)", "History of Pelvic Fracture", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2022-12-06, outpatient, Endocrinology

Chief complaint: Fatigue, depression, and difficulty concentrating

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, depression, and difficulty concentrating

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with a 6-month history of fatigue, depression, and difficulty concentrating. She also reports a recent onset of constipation and polyuria.

PAST MEDICAL HISTORY:
Active Problem List:
- Resistant hypertension
- Primary hyperparathyroidism
- History of recurrent nephrolithiasis
- History of recurrent urinary tract infections
- Status post left nephrectomy
- Stress urinary incontinence
- Benign essential blepharospasm

She has a past medical history of hypertension and recurrent kidney stones.

PAST SURGICAL HISTORY:
- Left nephrectomy (age 50, complications of chronic pyelonephritis)
- Vaginal delivery x2 (ages 32 and 34)
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
Her medications include hydrochlorothiazide for blood pressure control.

Home Medications:
- Amlodipine 10 mg oral daily
- Lisinopril 20 mg oral daily

ALLERGIES:
Hydralazine (drug-induced systemic lupus erythematosus), Contrast dye (anaphylaxis)

FAMILY HISTORY:
- Mother: Hypertension, osteoporosis, primary hyperparathyroidism
- Father: Nephrolithiasis
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks monthly. Occupation: Administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, her blood pressure is 145/90 mmHg, and she has mild tenderness in the flank region bilaterally. No other abnormalities are noted.

LABORATORY DATA:
Laboratory results are as follows:

- Serum calcium: 11.8 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum phosphorus: 2.4 mg/dL (normal: 2.5-4.5 mg/dL)
- Serum creatinine: 1.0 mg/dL (normal: 0.6-1.2 mg/dL)
- 24-hour urinary calcium: 400 mg (normal: 100-300 mg)
- Parathyroid hormone (PTH): 95 pg/mL (normal: 10-65 pg/mL)

## Encounter 1: source 2020-04-15, practice 2023-03-07, outpatient, Cardiology

Chief complaint: Intermittent headaches, fatigue, and palpitations

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent headaches, fatigue, and palpitations

HISTORY OF PRESENT ILLNESS:
Patient returns for follow-up of primary hyperparathyroidism seen in Endocrinology three months prior and presents with a 6-month history of intermittent headaches, fatigue, and palpitations. She reports a recent history of passing a kidney stone and has been experiencing diffuse abdominal pain for the past two months.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary hyperparathyroidism (diagnosed 2020-01-15)
- Resistant hypertension
- History of recurrent nephrolithiasis
- History of recurrent urinary tract infections
- Status post left nephrectomy
- Stress urinary incontinence
- Benign essential blepharospasm

Her past medical history is significant for hypertension, which has been poorly controlled despite adherence to a three-drug regimen including amlodipine, hydrochlorothiazide, and lisinopril.

PAST SURGICAL HISTORY:
- Left nephrectomy (age 50, complications of chronic pyelonephritis)
- Vaginal delivery x2 (ages 32 and 34)
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
Amlodipine, hydrochlorothiazide, and lisinopril

ALLERGIES:
Hydralazine (drug-induced systemic lupus erythematosus), Contrast dye (anaphylaxis)

FAMILY HISTORY:
- Mother: Hypertension, osteoporosis, primary hyperparathyroidism
- Father: Nephrolithiasis
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks monthly. Occupation: Administrative assistant.

VITAL SIGNS:
Blood pressure is 162/98 mmHg

PHYSICAL EXAMINATION:
On physical examination, she has mild tenderness in the epigastric region. No abdominal masses are palpable.

LABORATORY DATA:
Laboratory results are as follows:

- Serum calcium: 11.2 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum creatinine: 1.1 mg/dL (normal: 0.6-1.2 mg/dL)
- Serum phosphorus: 2.4 mg/dL (normal: 2.5-4.5 mg/dL)
- 24-hour urinary calcium: 400 mg (normal: 100-300 mg)
- Parathyroid hormone (PTH): Elevated

## Encounter 2: source 2020-07-15, practice 2023-06-09, outpatient, Urology

Chief complaint: Urinary leakage over the past several months

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Emily Watson, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Urinary leakage over the past several months

HISTORY OF PRESENT ILLNESS:
The patient returns for urinary symptoms following recent encounters for fatigue, depression, and palpitations associated with primary hyperparathyroidism. She states that the leakage occurs when she coughs, laughs, or exercises, and it has become more frequent and embarrassing, especially during group fitness classes. She denies urgency, frequency, dysuria, or nocturia. She had two vaginal deliveries in her early 30s and went through menopause one year ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary hyperparathyroidism (diagnosed 2020-01-15)
- Resistant hypertension
- History of recurrent nephrolithiasis
- History of recurrent urinary tract infections
- Status post left nephrectomy
- Stress urinary incontinence
- Benign essential blepharospasm

Her medical history includes mild hypertension, for which she takes hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Left nephrectomy (age 50, complications of chronic pyelonephritis)
- Vaginal delivery x2 (ages 32 and 34)
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Amlodipine 10 mg oral daily
- Lisinopril 20 mg oral daily

ALLERGIES:
Hydralazine (drug-induced systemic lupus erythematosus), Contrast dye (anaphylaxis)

FAMILY HISTORY:
- Mother: Hypertension, osteoporosis, primary hyperparathyroidism
- Father: Nephrolithiasis
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks monthly. Occupation: Administrative assistant.

PHYSICAL EXAMINATION:
Physical examination reveals a normal external genitalia. On speculum examination, no pelvic organ prolapse is noted, but there is urethral hypermobility with Valsalva. A cough stress test in the supine position reproduces the leakage. Neurologic examination is unremarkable.

LABORATORY DATA:
Urinalysis is normal.

OTHER STUDIES:
A post-void residual measured by bladder scan is 35 mL.

## Encounter 3: source 2020-10-15, practice 2023-09-12, outpatient, Neurology

Chief complaint: Involuntary closure of her eyes

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. David Kim, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Involuntary closure of her eyes

HISTORY OF PRESENT ILLNESS:
The patient presents for outpatient Neurology evaluation with involuntary closure of her eyes, having recently been evaluated by Endocrinology and Urology for primary hyperparathyroidism and stress urinary incontinence. She reports that this has been progressively worsening over the past 6 months. She reports that these episodes occur multiple times a day and seem to be triggered by her exposure to the bright lights in her workplace. She also notes that touching the skin around her eyes can sometimes stop the spasms.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary hyperparathyroidism (diagnosed 2020-01-15)
- Stress urinary incontinence (diagnosed 2020-07-15)
- Resistant hypertension
- History of recurrent nephrolithiasis
- History of recurrent urinary tract infections
- Status post left nephrectomy
- Benign essential blepharospasm

PAST SURGICAL HISTORY:
- Left nephrectomy (age 50, complications of chronic pyelonephritis)
- Vaginal delivery x2 (ages 32 and 34)
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Amlodipine 10 mg oral daily
- Lisinopril 20 mg oral daily

ALLERGIES:
Hydralazine (drug-induced systemic lupus erythematosus), Contrast dye (anaphylaxis)

FAMILY HISTORY:
- Mother: Hypertension, osteoporosis, primary hyperparathyroidism
- Father: Nephrolithiasis
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks monthly. Occupation: Administrative assistant.

PHYSICAL EXAMINATION:
Forceful contractions of the orbicularis oculi muscles bilaterally. Neurological examination is otherwise unremarkable. Visual acuity and intraocular pressures are within normal limits.

## Encounter 4: source 2021-01-15, practice 2023-12-08, outpatient, Internal Medicine

Chief complaint: Persistent fatigue and occasional dizziness

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Aisha Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue and occasional dizziness

HISTORY OF PRESENT ILLNESS:
The patient is a 53-year-old female with primary hyperparathyroidism, resistant hypertension, recurrent nephrolithiasis status post left nephrectomy, stress urinary incontinence, and benign essential blepharospasm who has undergone recent evaluations in Endocrinology, Cardiology, Urology, and Neurology. She returns for follow-up over the past few weeks.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary hyperparathyroidism (diagnosed 2020-01-15)
- Stress urinary incontinence (diagnosed 2020-07-15)
- Blepharospasm (diagnosed 2020-10-15)
- Resistant hypertension
- History of recurrent nephrolithiasis
- History of recurrent urinary tract infections
- Status post left nephrectomy
- Benign essential blepharospasm

She has a history of hypertension.

PAST SURGICAL HISTORY:
- Left nephrectomy (age 50, complications of chronic pyelonephritis)
- Vaginal delivery x2 (ages 32 and 34)
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
She was started on hydrochlorothiazide at her last visit.

Home Medications:
- Amlodipine 10 mg oral daily
- Lisinopril 20 mg oral daily

ALLERGIES:
Hydralazine (drug-induced systemic lupus erythematosus), Contrast dye (anaphylaxis)

FAMILY HISTORY:
- Mother: Hypertension, osteoporosis, primary hyperparathyroidism
- Father: Nephrolithiasis
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks monthly. Occupation: Administrative assistant.

REVIEW OF SYSTEMS:
She denies chest pain, palpitations, or shortness of breath

VITAL SIGNS:
Her blood pressure today is 130/80 mm Hg

PHYSICAL EXAMINATION:
Physical examination is unremarkable

ASSESSMENT AND PLAN:
The physician notes a few expected laboratory changes associated with her medication

Basic metabolic panel is ordered

## Encounter 5: source 2022-01-15, practice 2024-12-02, ed, Emergency Medicine

Chief complaint: Fever, fatigue, and joint pain

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, fatigue, and joint pain

HISTORY OF PRESENT ILLNESS:
This 54-year-old female with resistant hypertension, primary hyperparathyroidism, and status post left nephrectomy presents to the emergency department with a 2-week history of fever, fatigue, and joint pain. She describes the pain as symmetrical and involving her hands and knees. She also reports a recent onset of a rash on her face, which worsens with sun exposure.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary hyperparathyroidism (diagnosed 2020-01-15)
- Stress urinary incontinence (diagnosed 2020-07-15)
- Blepharospasm (diagnosed 2020-10-15)
- Hypocalciuria (diagnosed 2021-01-15)
- Resistant hypertension
- History of recurrent nephrolithiasis
- History of recurrent urinary tract infections
- Status post left nephrectomy
- Benign essential blepharospasm

Her medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Left nephrectomy (age 50, complications of chronic pyelonephritis)
- Vaginal delivery x2 (ages 32 and 34)
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
She takes for which she started taking hydralazine 3 months ago.

Home Medications:
- Amlodipine 10 mg oral daily
- Hydrochlorothiazide 25 mg oral daily
- Lisinopril 20 mg oral daily

ALLERGIES:
Hydralazine (drug-induced systemic lupus erythematosus), Contrast dye (anaphylaxis)

FAMILY HISTORY:
- Mother: Hypertension, osteoporosis, primary hyperparathyroidism
- Father: Nephrolithiasis
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks monthly. Occupation: Administrative assistant.

VITAL SIGNS:
Temperature of 38.2°C (100.8°F)

PHYSICAL EXAMINATION:
On physical examination, she has a temperature of 38.2°C (100.8°F), and a malar rash is noted. Joint examination reveals tenderness and swelling in the proximal interphalangeal and metacarpophalangeal joints bilaterally.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 12.1 g/dL
- White blood cell count: 4,800/mm³
- Platelet count: 220,000/mm³
- Erythrocyte sedimentation rate (ESR): 60 mm/hr
- Antinuclear antibodies (ANA): Positive
- Anti-histone antibodies: Positive
- Anti-double-stranded DNA (anti-dsDNA) antibodies: Negative
- Complement levels (C3, C4): Normal

## Encounter 6: source 2022-03-15, practice 2025-01-29, outpatient, Dermatology

Chief complaint: Pruritic, violaceous, flat-topped papules on trunk and extremities

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Lisa Thompson, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Pruritic, violaceous, flat-topped papules on trunk and extremities

HISTORY OF PRESENT ILLNESS:
The patient returns two months after her emergency department visit. With her history of resistant hypertension, she presents with new dermatologic concerns. She reports that the rash initially appeared on her lower back and has since become more widespread. She denies any recent infections or contact with irritants.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary hyperparathyroidism (diagnosed 2020-01-15)
- Stress urinary incontinence (diagnosed 2020-07-15)
- Blepharospasm (diagnosed 2020-10-15)
- Hypocalciuria (diagnosed 2021-01-15)
- Drug-induced systemic lupus erythematosus (diagnosed 2022-01-15)
- Resistant hypertension
- History of recurrent nephrolithiasis
- History of recurrent urinary tract infections
- Status post left nephrectomy
- Benign essential blepharospasm

Her past medical history is significant for hypertension,.

PAST SURGICAL HISTORY:
- Left nephrectomy (age 50, complications of chronic pyelonephritis)
- Vaginal delivery x2 (ages 32 and 34)
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
She takes for which she takes lisinopril and hydrochlorothiazide.

Home Medications:
- Amlodipine 10 mg oral daily

ALLERGIES:
Hydralazine (drug-induced systemic lupus erythematosus), Contrast dye (anaphylaxis)

FAMILY HISTORY:
- Mother: Hypertension, osteoporosis, primary hyperparathyroidism
- Father: Nephrolithiasis
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks monthly. Occupation: Administrative assistant.

PHYSICAL EXAMINATION:
Physical examination reveals numerous 3-5 mm shiny papules with fine white lines on the surface, distributed over her trunk, arms, and legs. Some of the papules have coalesced into plaques. No oral or genital lesions are noted.

LABORATORY DATA:
Laboratory studies show:
- Complete blood count: within normal limits
- Comprehensive metabolic panel: within normal limits
- Hepatitis panel: negative
- Antinuclear antibodies (ANA): negative
- Rapid plasma reagin (RPR): non-reactive

## Encounter 7: source 2023-01-15, practice 2025-12-06, inpatient, Nephrology

Chief complaint: Hematuria, fever, and left-sided flank pain

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Robert Johnson, MD
Department: Nephrology
Visit Type: inpatient

CHIEF COMPLAINT:
Hematuria, fever, and left-sided flank pain

HISTORY OF PRESENT ILLNESS:
The patient has recurrent nephrolithiasis and prior left nephrectomy. Despite antibiotic therapy, her symptoms persist, and she undergoes nephrectomy.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary hyperparathyroidism (diagnosed 2020-01-15)
- Stress urinary incontinence (diagnosed 2020-07-15)
- Blepharospasm (diagnosed 2020-10-15)
- Hypocalciuria (diagnosed 2021-01-15)
- Drug-induced systemic lupus erythematosus (diagnosed 2022-01-15)
- Lichenoid drug eruption or cutaneous lymphoma (requiring biopsy) (diagnosed 2022-03-15)
- Resistant hypertension
- History of recurrent nephrolithiasis
- History of recurrent urinary tract infections
- Status post left nephrectomy
- Benign essential blepharospasm

PAST SURGICAL HISTORY:
- Left nephrectomy (age 50, complications of chronic pyelonephritis)
- Vaginal delivery x2 (ages 32 and 34)
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
Home Medications:
- Amlodipine 10 mg oral daily
- Hydrochlorothiazide 25 mg oral daily
- Lisinopril 20 mg oral daily

ALLERGIES:
Hydralazine (drug-induced systemic lupus erythematosus), Contrast dye (anaphylaxis)

FAMILY HISTORY:
- Mother: Hypertension, osteoporosis, primary hyperparathyroidism
- Father: Nephrolithiasis
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks monthly. Occupation: Administrative assistant.

LABORATORY DATA:
Urinalysis reveals a pH of 7.6, specific gravity 1.019, and 1+ blood. Microscopic analysis shows numerous leukocytes, erythrocytes, and characteristic coffin-lid-shaped crystals

IMAGING:
Imaging reveals a large staghorn calculus in the left renal pelvis

PATHOLOGY:
Histopathology confirms chronic pyelonephritis with suppurative destruction of renal parenchyma
