# synthetic_hospital patient 2602

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex M, Non-Hispanic White, Private commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-09-30 (Mon) | outpatient | Ophthalmology |
| 1 | 2020-09-15 | 2025-05-23 (Fri) | outpatient | Gastroenterology |
| 2 | 2021-09-15 | 2026-05-26 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Private commercial PPO",
 "occupation": "Software engineer",
 "alcohol_use": "Social, 2-3 drinks per week",
 "family_history": [
  "Father with Crohn's disease",
  "Maternal uncle with ankylosing spondylitis",
  "Mother with hypothyroidism"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "40 mg subcutaneous every 14 days",
   "name": "Adalimumab"
  },
  {
   "dose": "1000 mg orally twice daily",
   "name": "Sulfasalazine"
  },
  {
   "dose": "2000 IU orally daily",
   "name": "Vitamin D3"
  }
 ],
 "surgical_history": [
  "Tonsillectomy at age 12"
 ],
 "chronic_conditions": [
  "Crohn's disease",
  "Ankylosing spondylitis",
  "Recurrent aphthous stomatitis",
  "Chronic constipation"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Crohn's disease", "Perianal abscess", "Ankylosing Spondylitis"]
Comorbidities: ["Recurrent Oral Aphthae"]

## Encounter 0: source 2020-01-15, practice 2024-09-30, outpatient, Ophthalmology

Chief complaint: Painful red eyes and blurred vision for 2 days, with persistent lower back pain for 3 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Patel, MD
Department: Ophthalmology
Visit Type: outpatient

CHIEF COMPLAINT:
Painful red eyes and blurred vision for 2 days, with persistent lower back pain for 3 months

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 2-day history of painful red eyes and blurred vision. He also complains of lower back pain that has been persistent for the past 3 months, which is worse in the morning and improves with exercise. He denies any recent infections or trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Crohn's disease
- Ankylosing spondylitis
- Recurrent aphthous stomatitis
- Chronic constipation

His medical history is significant for recurrent oral ulcers and a previous episode of acute anterior uveitis one year ago.

PAST SURGICAL HISTORY:
- Tonsillectomy at age 12

MEDICATIONS:
Home Medications:
- Adalimumab 40 mg subcutaneous every 14 days
- Sulfasalazine 1000 mg orally twice daily
- Vitamin D3 2000 IU orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with Crohn's disease
- Maternal uncle with ankylosing spondylitis
- Mother with hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 2-3 drinks per week. Occupation: Software engineer.

PHYSICAL EXAMINATION:
On physical examination, his visual acuity is decreased, and slit-lamp examination reveals ciliary flush and cells in the anterior chamber. His lumbar spine is tender to palpation, and he has limited range of motion in the lumbar region.

LABORATORY DATA:
Laboratory values are as follows:

- Erythrocyte sedimentation rate (ESR): 45 mm/hr (Normal: 0-22 mm/hr)
- C-reactive protein (CRP): 3.0 mg/dL (Normal: <0.3 mg/dL)
- HLA-B27 antigen: Positive

IMAGING:
An x-ray of the pelvis is performed, showing bilateral sacroiliitis.

## Encounter 1: source 2020-09-15, practice 2025-05-23, outpatient, Gastroenterology

Chief complaint: Pre-operative evaluation for planned drainage of perianal fluid collection

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. James Morrison, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Pre-operative evaluation for planned drainage of perianal fluid collection

HISTORY OF PRESENT ILLNESS:
The patient with Crohn's disease and ankylosing spondylitis diagnosed in January returns for pre-operative evaluation for drainage of a perianal fluid collection. He reports experiencing loose stools for several months, occasionally with abdominal discomfort that fluctuates in intensity. He denies recent travel, antibiotic use, or dietary changes. His weight has been slowly decreasing over the last year, which he attributes to poor appetite and irregular meals.

PAST MEDICAL HISTORY:
Active Problem List:
- Ankylosing Spondylitis (diagnosed 2020-01-15)
- Crohn's disease
- Recurrent aphthous stomatitis
- Chronic constipation

PAST SURGICAL HISTORY:
- Tonsillectomy at age 12

MEDICATIONS:
Home Medications:
- Adalimumab 40 mg subcutaneous every 14 days
- Sulfasalazine 1000 mg orally twice daily
- Vitamin D3 2000 IU orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with Crohn's disease
- Maternal uncle with ankylosing spondylitis
- Mother with hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 2-3 drinks per week. Occupation: Software engineer.

REVIEW OF SYSTEMS:
Review of systems is notable for fatigue, intermittent low-grade fevers, and occasional soreness in the mouth that resolves on its own.

PHYSICAL EXAMINATION:
Physical examination shows a thin male with mild tenderness in the lower abdomen and a small, tender area of fluctuance near the anal verge. Digital rectal examination elicits discomfort.

LABORATORY DATA:
Initial laboratory tests show mild anemia and a slightly elevated inflammatory marker.

IMAGING:
Imaging reveals a perirectal fluid collection suggestive of an abscess.

PLAN:
Further evaluation is planned to clarify the underlying diagnosis and determine the most appropriate site for biopsy.

## Encounter 2: source 2021-09-15, practice 2026-05-26, ed, Emergency Medicine

Chief complaint: Severe pain and swelling near the anus for 3 days

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe pain and swelling near the anus for 3 days

HISTORY OF PRESENT ILLNESS:
This patient with Crohn's disease, ankylosing spondylitis, and prior perianal fluid collection presents to the emergency department with a 3-day history of severe pain and swelling near the anus. He reports the pain worsens during bowel movements and while sitting.

PAST MEDICAL HISTORY:
Active Problem List:
- Ankylosing Spondylitis (diagnosed 2020-01-15)
- Crohn's disease (diagnosed 2020-09-15)
- Recurrent aphthous stomatitis
- Chronic constipation

He has a history of chronic constipation.

PAST SURGICAL HISTORY:
- Tonsillectomy at age 12

MEDICATIONS:
Home Medications:
- Adalimumab 40 mg subcutaneous every 14 days
- Sulfasalazine 1000 mg orally twice daily
- Vitamin D3 2000 IU orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with Crohn's disease
- Maternal uncle with ankylosing spondylitis
- Mother with hypothyroidism

SOCIAL HISTORY:
Engages in anoreceptive intercourse.

VITAL SIGNS:
The patient has a fever of 38.5°C (101.3°F).

PHYSICAL EXAMINATION:
On physical examination, there is a tender, fluctuant, erythematous mass located at the anal verge. Rectal examination is extremely painful and therefore limited.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 15,000 /µL (normal: 4,500-11,000 /µL)
- Hemoglobin: 13.5 g/dL (normal: 13.8-17.2 g/dL)
- C-reactive protein: 24 mg/L (normal: <5 mg/L)
