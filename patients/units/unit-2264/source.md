# synthetic_hospital patient 2264

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 72 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-06-01 (Mon) | outpatient | Endocrinology |
| 1 | 2021-01-15 | 2021-06-11 (Fri) | outpatient | Urology |
| 2 | 2022-01-15 | 2022-06-07 (Tue) | outpatient | Internal Medicine |
| 3 | 2023-01-15 | 2023-06-05 (Mon) | outpatient | Dermatology |
| 4 | 2024-01-15 | 2024-06-10 (Mon) | outpatient | General Surgery |
| 5 | 2025-01-15 | 2025-06-03 (Tue) | outpatient | Urology |
| 6 | 2025-11-15 | 2026-04-01 (Wed) | outpatient | Cardiology |
| 7 | 2026-01-15 | 2026-06-04 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired high school mathematics teacher",
 "alcohol_use": "Occasional (1-2 drinks per month)",
 "family_history": [
  "Father: Hypertension, benign prostatic hyperplasia",
  "Mother: Osteoporosis, age-related macular degeneration",
  "Sister: Hashimoto thyroiditis (autoimmune)",
  "Paternal uncle: Type 1 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 15 years ago (30 pack-year history)",
 "home_medications": [
  {
   "dose": "200 mg PO daily at bedtime",
   "name": "Quetiapine"
  },
  {
   "dose": "0.4 mg PO daily after evening meal",
   "name": "Tamsulosin"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "20 mg PO every morning, 10 mg PO every evening",
   "name": "Hydrocortisone"
  },
  {
   "dose": "0.1 mg PO daily",
   "name": "Fludrocortisone"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Transurethral resection of the prostate (TURP) 3 years prior"
 ],
 "chronic_conditions": [
  "Benign prostatic hyperplasia",
  "Hypertension",
  "Schizophrenia",
  "Osteoarthritis",
  "Primary adrenal insufficiency (Addison disease)"
 ],
 "age_at_first_encounter": 72
}
```

Primary diagnoses: ["Orthostatic hypotension", "Post-renal acute kidney injury (obstructive uropathy)", "Recurrent benign prostatic hyperplasia", "Senile purpura", "Primary adrenal insufficiency", "Spigelian hernia", "Calcium channel blocker-induced peripheral edema"]
Comorbidities: ["Recurrent benign prostatic hyperplasia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2020-06-01, outpatient, Endocrinology

Chief complaint: Progressive fatigue, decreased appetite, unintentional weight loss, and hyperpigmentation of forearms

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue, decreased appetite, unintentional weight loss, and hyperpigmentation of forearms

HISTORY OF PRESENT ILLNESS:
He reports feeling lightheaded when standing up and has recently noticed that the skin on his forearms has become noticeably darker despite limited sun exposure.

PAST MEDICAL HISTORY:
Active Problem List:
- Benign prostatic hyperplasia
- Hypertension
- Schizophrenia
- Osteoarthritis
- Primary adrenal insufficiency (Addison disease)

PAST SURGICAL HISTORY:
- Transurethral resection of the prostate (TURP) 3 years prior

MEDICATIONS:
Home Medications:
- Quetiapine 200 mg PO daily at bedtime
- Tamsulosin 0.4 mg PO daily after evening meal
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Hydrocortisone 20 mg PO every morning, 10 mg PO every evening
- Fludrocortisone 0.1 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, benign prostatic hyperplasia
- Mother: Osteoporosis, age-related macular degeneration
- Sister: Hashimoto thyroiditis (autoimmune)
- Paternal uncle: Type 1 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (30 pack-year history). Alcohol: Occasional (1-2 drinks per month). Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
His blood pressure is 100/60 mm Hg

PHYSICAL EXAMINATION:
Physical exam shows diffuse hyperpigmentation over the forearms and buccal mucosa

LABORATORY DATA:
Complete blood count reveals an eosinophil percentage of 23%.

## Encounter 1: source 2021-01-15, practice 2021-06-11, outpatient, Urology

Chief complaint: Difficulty initiating urination, weak urinary stream, and sensation of incomplete bladder emptying

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Difficulty initiating urination, weak urinary stream, and sensation of incomplete bladder emptying

HISTORY OF PRESENT ILLNESS:
The patient with benign prostatic hyperplasia and primary adrenal insufficiency previously evaluated in Endocrinology presents to the clinic with a 6-month history of difficulty initiating urination, a weak urinary stream, and a sensation of incomplete bladder emptying. He reports no pain or burning during urination.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary adrenal insufficiency (diagnosed 2020-01-15)
- Benign prostatic hyperplasia
- Hypertension
- Schizophrenia
- Osteoarthritis
- Primary adrenal insufficiency (Addison disease)

His medical history is significant for benign prostatic hyperplasia (BPH), for which he underwent transurethral resection of the prostate (TURP) 3 years ago.

PAST SURGICAL HISTORY:
- Transurethral resection of the prostate (TURP) 3 years prior

MEDICATIONS:
Home Medications:
- Quetiapine 200 mg PO daily at bedtime
- Tamsulosin 0.4 mg PO daily after evening meal
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Hydrocortisone 20 mg PO every morning, 10 mg PO every evening
- Fludrocortisone 0.1 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, benign prostatic hyperplasia
- Mother: Osteoporosis, age-related macular degeneration
- Sister: Hashimoto thyroiditis (autoimmune)
- Paternal uncle: Type 1 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (30 pack-year history). Alcohol: Occasional (1-2 drinks per month). Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, his abdomen is soft and non-tender, with no palpable bladder distention. Digital rectal examination reveals a moderately enlarged, firm, and non-tender prostate.

LABORATORY DATA:
Urinalysis is unremarkable.

OTHER STUDIES:
Post-void residual volume is 150 mL.

## Encounter 2: source 2022-01-15, practice 2022-06-07, outpatient, Internal Medicine

Chief complaint: Painless bilateral ankle swelling following addition of amlodipine for blood pressure control

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Painless bilateral ankle swelling following addition of amlodipine for blood pressure control

HISTORY OF PRESENT ILLNESS:
This 74-year-old male with primary adrenal insufficiency and benign prostatic hyperplasia, previously evaluated in Endocrinology and Urology, returns with concerns regarding bilateral lower extremity swelling. His blood pressure was poorly controlled on hydrochlorothiazide alone, so amlodipine was added to his regimen 6 weeks ago. Since then, his systolic pressure has improved from 168 mm Hg to 132 mm Hg, and he reports generally feeling better. However, he has developed painless bilateral ankle swelling that worsens by the end of the day and improves slightly overnight.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary adrenal insufficiency (diagnosed 2020-01-15)
- Recurrent benign prostatic hyperplasia (diagnosed 2021-01-15)
- Benign prostatic hyperplasia
- Hypertension
- Schizophrenia
- Osteoarthritis
- Primary adrenal insufficiency (Addison disease)

PAST SURGICAL HISTORY:
- Transurethral resection of the prostate (TURP) 3 years prior

MEDICATIONS:
His medications include amlodipine, hydrochlorothiazide, and aspirin. He does not use NSAIDs or steroids.

Home Medications:
- Quetiapine 200 mg PO daily at bedtime
- Tamsulosin 0.4 mg PO daily after evening meal
- Hydrocortisone 20 mg PO every morning, 10 mg PO every evening
- Fludrocortisone 0.1 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, benign prostatic hyperplasia
- Mother: Osteoporosis, age-related macular degeneration
- Sister: Hashimoto thyroiditis (autoimmune)
- Paternal uncle: Type 1 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (30 pack-year history). Alcohol: Occasional (1-2 drinks per month). Occupation: Retired high school mathematics teacher.

REVIEW OF SYSTEMS:
He denies dyspnea, chest pain, orthopnea, or paroxysmal nocturnal dyspnea.

VITAL SIGNS:
On exam, vital signs are normal

PHYSICAL EXAMINATION:
There is 2+ pitting edema bilaterally in the lower extremities. Jugular venous pressure is normal, and heart and lung exams are unremarkable. There is no hepatomegaly or ascites.

LABORATORY DATA:
Labs including renal function, albumin, and BNP are all within normal limits.

IMAGING:
Echocardiogram from 3 months ago showed normal left ventricular function.

OTHER STUDIES:
ECG shows normal sinus rhythm.

ASSESSMENT AND PLAN:
The clinician suspects the edema is related to the patient's antihypertensive medication.

## Encounter 3: source 2023-01-15, practice 2023-06-05, outpatient, Dermatology

Chief complaint: Multiple purple patches on skin developing after minor bumps

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James Wilson, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Multiple purple patches on skin developing after minor bumps

HISTORY OF PRESENT ILLNESS:
The patient is a 75-year-old male with primary adrenal insufficiency diagnosed in 2020 who presents to the dermatology clinic with his daughter, who is concerned about multiple purple patches on his skin that have been appearing over the past few months. He reports that the lesions develop after minor bumps and resolve over weeks to months. He denies pain, itching, or bleeding associated with these patches.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary adrenal insufficiency (diagnosed 2020-01-15)
- Recurrent benign prostatic hyperplasia (diagnosed 2021-01-15)
- Calcium channel blocker-induced peripheral edema (diagnosed 2022-01-15)
- Benign prostatic hyperplasia
- Hypertension
- Schizophrenia
- Osteoarthritis
- Primary adrenal insufficiency (Addison disease)

His medical history includes hypertension, for which he takes hydrochlorothiazide, and benign prostatic hyperplasia.

PAST SURGICAL HISTORY:
- Transurethral resection of the prostate (TURP) 3 years prior

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Quetiapine 200 mg PO daily at bedtime
- Tamsulosin 0.4 mg PO daily after evening meal
- Amlodipine 5 mg PO daily
- Hydrocortisone 20 mg PO every morning, 10 mg PO every evening
- Fludrocortisone 0.1 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, benign prostatic hyperplasia
- Mother: Osteoporosis, age-related macular degeneration
- Sister: Hashimoto thyroiditis (autoimmune)
- Paternal uncle: Type 1 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (30 pack-year history). Alcohol: Occasional (1-2 drinks per month). Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On examination, you note several non-blanching, purple ecchymoses on the extensor surfaces of his hands and forearms. There is no associated warmth, edema, or tenderness. His remaining physical examination is unremarkable.

LABORATORY DATA:
His complete blood count, coagulation profile, and platelet count are within normal limits.

## Encounter 4: source 2024-01-15, practice 2024-06-10, outpatient, General Surgery

Chief complaint: Bulge on lower left side of abdomen noticed when standing or straining

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Aisha Johnson, MD
Department: General Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Bulge on lower left side of abdomen noticed when standing or straining

HISTORY OF PRESENT ILLNESS:
The patient is a 76-year-old male with a history of primary adrenal insufficiency and benign prostatic hyperplasia who has received longitudinal care within the system since 2020. He presents with a new abdominal wall concern. He first noticed the swelling a few months ago while getting out of bed, and since then, it has gradually become more noticeable, particularly when standing or straining. He denies any trauma, prior abdominal surgery, or heavy lifting. He reports no abdominal pain, changes in bowel habits, nausea, or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary adrenal insufficiency (diagnosed 2020-01-15)
- Recurrent benign prostatic hyperplasia (diagnosed 2021-01-15)
- Calcium channel blocker-induced peripheral edema (diagnosed 2022-01-15)
- Senile purpura (diagnosed 2023-01-15)
- Benign prostatic hyperplasia
- Hypertension
- Schizophrenia
- Osteoarthritis
- Primary adrenal insufficiency (Addison disease)

His medical history includes hypertension and osteoarthritis.

PAST SURGICAL HISTORY:
- Transurethral resection of the prostate (TURP) 3 years prior

MEDICATIONS:
Home Medications:
- Quetiapine 200 mg PO daily at bedtime
- Tamsulosin 0.4 mg PO daily after evening meal
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Hydrocortisone 20 mg PO every morning, 10 mg PO every evening
- Fludrocortisone 0.1 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, benign prostatic hyperplasia
- Mother: Osteoporosis, age-related macular degeneration
- Sister: Hashimoto thyroiditis (autoimmune)
- Paternal uncle: Type 1 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (30 pack-year history). Alcohol: Occasional (1-2 drinks per month). Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, there is a soft, reducible mass in the left lower quadrant of the abdominal wall, located several centimeters lateral to the lateral border of the rectus abdominis. The swelling increases with Valsalva maneuver and reduces with gentle pressure. There is no overlying erythema, tenderness, or signs of bowel obstruction.

## Encounter 5: source 2025-01-15, practice 2025-06-03, outpatient, Urology

Chief complaint: Increased urinary frequency, nocturia, weak stream, and sensation of incomplete emptying

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Increased urinary frequency, nocturia, weak stream, and sensation of incomplete emptying

HISTORY OF PRESENT ILLNESS:
The patient returns to urology with recurrent symptoms over the last 4 months. He also reports a sensation of incomplete bladder emptying. The patient denies hematuria or dysuria.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary adrenal insufficiency (diagnosed 2020-01-15)
- Recurrent benign prostatic hyperplasia (diagnosed 2021-01-15)
- Calcium channel blocker-induced peripheral edema (diagnosed 2022-01-15)
- Senile purpura (diagnosed 2023-01-15)
- Spigelian hernia (diagnosed 2024-01-15)
- Benign prostatic hyperplasia
- Hypertension
- Schizophrenia
- Osteoarthritis
- Primary adrenal insufficiency (Addison disease)

His medical history is significant for hypertension, which is currently poorly controlled.

PAST SURGICAL HISTORY:
- Transurethral resection of the prostate (TURP) 3 years prior

MEDICATIONS:
He takes amlodipine.

Home Medications:
- Quetiapine 200 mg PO daily at bedtime
- Tamsulosin 0.4 mg PO daily after evening meal
- Hydrochlorothiazide 25 mg PO daily
- Hydrocortisone 20 mg PO every morning, 10 mg PO every evening
- Fludrocortisone 0.1 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, benign prostatic hyperplasia
- Mother: Osteoporosis, age-related macular degeneration
- Sister: Hashimoto thyroiditis (autoimmune)
- Paternal uncle: Type 1 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (30 pack-year history). Alcohol: Occasional (1-2 drinks per month). Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
His blood pressure today is 160/94 mm Hg.

PHYSICAL EXAMINATION:
On digital rectal examination, the prostate is symmetrically enlarged, smooth, and non-tender.

LABORATORY DATA:
His urinalysis is normal, and serum creatinine is 1.0 mg/dL.

ASSESSMENT AND PLAN:
Benign prostatic hyperplasia (BPH)

Initiates pharmacologic therapy

## Encounter 6: source 2025-11-15, practice 2026-04-01, outpatient, Cardiology

Chief complaint: Episodes of dizziness and near-fainting when standing with blurred vision and palpitations

ENCOUNTER NOTE
Date: 2025-11-15
Provider: Dr. Robert Kim, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Episodes of dizziness and near-fainting when standing with blurred vision and palpitations

HISTORY OF PRESENT ILLNESS:
The patient with primary adrenal insufficiency diagnosed in 2020 presents with episodes of dizziness and near-fainting upon standing. These symptoms have been occurring for the past two weeks and are associated with blurred vision and palpitations.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary adrenal insufficiency (diagnosed 2020-01-15)
- Recurrent benign prostatic hyperplasia (diagnosed 2021-01-15)
- Calcium channel blocker-induced peripheral edema (diagnosed 2022-01-15)
- Senile purpura (diagnosed 2023-01-15)
- Spigelian hernia (diagnosed 2024-01-15)
- Benign prostatic hyperplasia
- Hypertension
- Schizophrenia
- Osteoarthritis
- Primary adrenal insufficiency (Addison disease)

PAST SURGICAL HISTORY:
- Transurethral resection of the prostate (TURP) 3 years prior

MEDICATIONS:
His current medications include quetiapine 200 mg daily and tamsulosin 0.4 mg daily.

Home Medications:
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Hydrocortisone 20 mg PO every morning, 10 mg PO every evening
- Fludrocortisone 0.1 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, benign prostatic hyperplasia
- Mother: Osteoporosis, age-related macular degeneration
- Sister: Hashimoto thyroiditis (autoimmune)
- Paternal uncle: Type 1 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (30 pack-year history). Alcohol: Occasional (1-2 drinks per month). Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
His blood pressure is 145/90 mmHg while seated and drops to 110/70 mmHg upon standing. The heart rate increases from 78 bpm seated to 98 bpm standing.

PHYSICAL EXAMINATION:
Physical examination is otherwise unremarkable.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 14.2 g/dL
- Sodium: 140 mEq/L
- Potassium: 4.1 mEq/L
- Creatinine: 1.0 mg/dL
- Fasting glucose: 98 mg/dL

## Encounter 7: source 2026-01-15, practice 2026-06-04, ed, Emergency Medicine

Chief complaint: Confusion, fatigue, and decreased urine output

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. Emily Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Confusion, fatigue, and decreased urine output

HISTORY OF PRESENT ILLNESS:
The patient with recurrent benign prostatic hyperplasia (January 2025) is brought to the ED by his daughter due to confusion, fatigue, and decreased urine output over the past two days.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary adrenal insufficiency (diagnosed 2020-01-15)
- Recurrent benign prostatic hyperplasia (diagnosed 2021-01-15)
- Calcium channel blocker-induced peripheral edema (diagnosed 2022-01-15)
- Senile purpura (diagnosed 2023-01-15)
- Spigelian hernia (diagnosed 2024-01-15)
- Orthostatic hypotension (diagnosed 2025-11-15)
- Benign prostatic hyperplasia
- Hypertension
- Schizophrenia
- Osteoarthritis
- Primary adrenal insufficiency (Addison disease)

He has a history of hypertension and benign prostatic hyperplasia,.

PAST SURGICAL HISTORY:
- Transurethral resection of the prostate (TURP) 3 years prior

MEDICATIONS:
He takes for which he takes tamsulosin.

Home Medications:
- Quetiapine 200 mg PO daily at bedtime
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Hydrocortisone 20 mg PO every morning, 10 mg PO every evening
- Fludrocortisone 0.1 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, benign prostatic hyperplasia
- Mother: Osteoporosis, age-related macular degeneration
- Sister: Hashimoto thyroiditis (autoimmune)
- Paternal uncle: Type 1 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (30 pack-year history). Alcohol: Occasional (1-2 drinks per month). Occupation: Retired high school mathematics teacher.

REVIEW OF SYSTEMS:
His last bowel movement was 3 days ago.

VITAL SIGNS:
Vital signs are normal.

PHYSICAL EXAMINATION:
On exam, he appears disoriented and has a distended, palpable bladder.

LABORATORY DATA:
Laboratory results show :
Serum creatinine                               4.2 mg/dL (baseline 1.0)
BUN                                                   82 mg/dL
Urinalysis                                           Bland, no hematuria or proteinuria
FENa                                                  0.8%

OTHER STUDIES:
Bedside bladder scan                        700 mL of retained urine
