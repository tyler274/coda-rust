# synthetic_hospital patient 2869

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 72 at first encounter, sex F, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-11-03 (Fri) | outpatient | Internal Medicine |
| 1 | 2021-01-15 | 2024-11-11 (Mon) | outpatient | Internal Medicine |
| 2 | 2022-07-15 | 2026-05-07 (Thu) | outpatient | Neurology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired elementary school teacher",
 "alcohol_use": "Occasional social use (1-2 glasses wine per month)",
 "family_history": [
  "Mother: Major depressive disorder, osteoporosis",
  "Father: Degenerative arthritis, hypertension",
  "Maternal aunt: Primary open-angle glaucoma"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "75 mg by mouth at bedtime",
   "name": "Amitriptyline"
  },
  {
   "dose": "5 mg by mouth twice daily",
   "name": "Oxybutynin"
  },
  {
   "dose": "0.5%, 1 drop in each eye twice daily",
   "name": "Timolol ophthalmic solution"
  },
  {
   "dose": "400 mg by mouth every 6 hours as needed for back pain",
   "name": "Ibuprofen"
  }
 ],
 "surgical_history": [
  "Cesarean section, 1978",
  "Cataract extraction with intraocular lens placement, right eye, 2019"
 ],
 "chronic_conditions": [
  "Major depressive disorder",
  "Spinal osteoarthritis with chronic lower back pain",
  "Primary open-angle glaucoma",
  "Overactive bladder syndrome"
 ],
 "age_at_first_encounter": 72
}
```

Primary diagnoses: ["Orthostatic hypotension", "Adverse effect of anticholinergic medication", "Uncomplicated lumbar radiculopathy"]
Comorbidities: ["Major depressive disorder", "Chronic low back pain"]

## Encounter 0: source 2020-01-15, practice 2023-11-03, outpatient, Internal Medicine

Chief complaint: Dizziness and falls over the past week

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Dizziness and falls over the past week

HISTORY OF PRESENT ILLNESS:
She reports that the episodes occur when she stands up from a sitting position and occasionally she feels like she might pass out. She also mentions experiencing dry mouth and constipation.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder
- Spinal osteoarthritis with chronic lower back pain
- Primary open-angle glaucoma
- Overactive bladder syndrome

PAST SURGICAL HISTORY:
- Cesarean section, 1978
- Cataract extraction with intraocular lens placement, right eye, 2019

MEDICATIONS:
She takes amitriptyline 75 mg at bedtime, started 3 months ago, and ibuprofen as needed for back pain.

Home Medications:
- Oxybutynin 5 mg by mouth twice daily
- Timolol ophthalmic solution 0.5%, 1 drop in each eye twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Major depressive disorder, osteoporosis
- Father: Degenerative arthritis, hypertension
- Maternal aunt: Primary open-angle glaucoma

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 glasses wine per month). Occupation: Retired elementary school teacher.

VITAL SIGNS:
Her blood pressure is 110/70 mmHg while seated and 90/60 mmHg upon standing. Her heart rate is 78 bpm seated and 88 bpm standing.

PHYSICAL EXAMINATION:
Physical examination is otherwise unremarkable.

LABORATORY DATA:
No abnormalities are noted on complete blood count and basic metabolic panel.

## Encounter 1: source 2021-01-15, practice 2024-11-11, outpatient, Internal Medicine

Chief complaint: Increasing memory difficulties, confusion, and occasional urinary incontinence over the past three months

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing memory difficulties, confusion, and occasional urinary incontinence over the past three months

HISTORY OF PRESENT ILLNESS:
This 73-year-old female with major depressive disorder, spinal osteoarthritis, glaucoma, and overactive bladder syndrome returns one year after her prior visit for orthostatic hypotension. She presents with her daughter, who reports the patient has increasing memory difficulties, confusion, and occasional urinary incontinence for three months. The patient also complains of dry mouth and constipation.

PAST MEDICAL HISTORY:
Active Problem List:
- Orthostatic hypotension (diagnosed 2020-01-15)
- Major depressive disorder
- Spinal osteoarthritis with chronic lower back pain
- Primary open-angle glaucoma
- Overactive bladder syndrome

PAST SURGICAL HISTORY:
- Cesarean section, 1978
- Cataract extraction with intraocular lens placement, right eye, 2019

MEDICATIONS:
Her current medications include amitriptyline for depression, oxybutynin for overactive bladder, and timolol eye drops for glaucoma.

Home Medications:
- Timolol ophthalmic solution 0.5%, 1 drop in each eye twice daily
- Ibuprofen 400 mg by mouth every 6 hours as needed for back pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Major depressive disorder, osteoporosis
- Father: Degenerative arthritis, hypertension
- Maternal aunt: Primary open-angle glaucoma

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 glasses wine per month). Occupation: Retired elementary school teacher.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, the patient is alert but appears lethargic. Her pupils are 4 mm and minimally reactive to light. Bowel sounds are diminished, and her skin is warm and dry. Neurological examination is nonfocal.

LABORATORY DATA:
Laboratory values:
- Serum sodium: 140 mEq/L
- Serum potassium: 4.2 mEq/L
- Serum creatinine: 1.0 mg/dL
- Serum calcium: 9.6 mg/dL
- Complete blood count: within normal limits

## Encounter 2: source 2022-07-15, practice 2026-05-07, outpatient, Neurology

Chief complaint: Progressive numbness in lower abdomen and upper thigh region with burning groin pain

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. James O'Brien, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive numbness in lower abdomen and upper thigh region with burning groin pain

HISTORY OF PRESENT ILLNESS:
The patient is a 74-year-old female with longstanding spinal osteoarthritis and chronic lower back pain who presents with progressive neurological symptoms. She has a history of prior anticholinergic medication adverse effects and orthostatic hypotension. She reports a recent flare-up of lower back pain after helping her grandson move furniture. Over the past 4 days, she has developed a burning sensation across the right side of her groin, which is worse with sitting and improves slightly when lying supine. She also reports mild weakness when attempting to flex her hip. She denies saddle anesthesia, bowel or bladder dysfunction, or leg paresthesias. Her gait remains stable, and she does not feel unsteady while walking.

PAST MEDICAL HISTORY:
Active Problem List:
- Orthostatic hypotension (diagnosed 2020-01-15)
- Adverse effect of anticholinergic medication (diagnosed 2021-01-15)
- Major depressive disorder
- Spinal osteoarthritis with chronic lower back pain
- Primary open-angle glaucoma
- Overactive bladder syndrome

She has a history of spinal osteoarthritis.

PAST SURGICAL HISTORY:
- Cesarean section, 1978
- Cataract extraction with intraocular lens placement, right eye, 2019

MEDICATIONS:
Home Medications:
- Amitriptyline 75 mg by mouth at bedtime
- Oxybutynin 5 mg by mouth twice daily
- Timolol ophthalmic solution 0.5%, 1 drop in each eye twice daily
- Ibuprofen 400 mg by mouth every 6 hours as needed for back pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Major depressive disorder, osteoporosis
- Father: Degenerative arthritis, hypertension
- Maternal aunt: Primary open-angle glaucoma

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 glasses wine per month). Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
On examination, motor strength is 5/5 throughout, except for slight weakness with hip flexion on the right. Deep tendon reflexes are symmetric and normal. Sensation to light touch and pinprick is diminished over the inguinal region and upper medial thigh on the right, with no deficits in the lower leg or foot. Rectal tone is normal, and the remainder of the neurologic exam is unremarkable.

ASSESSMENT AND PLAN:
Injury or compression to

A focused neurologic workup is ordered, and lumbosacral imaging is planned to assess for foraminal narrowing.
