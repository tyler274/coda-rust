# synthetic_hospital patient 2605

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 26 at first encounter, sex M, Mediterranean (Italian and Greek ancestry), Blue Cross PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-01-23 (Mon) | outpatient | Internal Medicine |
| 1 | 2021-01-15 | 2024-01-29 (Mon) | outpatient | Internal Medicine |
| 2 | 2021-02-15 | 2024-03-07 (Thu) | outpatient | Rheumatology |
| 3 | 2021-03-15 | 2024-04-08 (Mon) | outpatient | Rheumatology |
| 4 | 2023-01-15 | 2026-01-27 (Tue) | outpatient | Rheumatology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Blue Cross PPO",
 "occupation": "Software engineer",
 "alcohol_use": "None",
 "family_history": [
  "Father with ankylosing spondylitis (HLA-B27 positive)",
  "Mother with beta thalassemia minor trait",
  "Paternal uncle with Crohn's disease"
 ],
 "race_ethnicity": "Mediterranean (Italian and Greek ancestry)",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "40 mg subcutaneous every 2 weeks",
   "name": "Adalimumab"
  },
  {
   "dose": "1000 mg orally twice daily",
   "name": "Sulfasalazine"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Ankylosing Spondylitis",
  "Beta Thalassemia Minor",
  "Recurrent Anterior Uveitis"
 ],
 "age_at_first_encounter": 26
}
```

Primary diagnoses: ["Ankylosing Spondylitis", "Thalassemia minor"]
Comorbidities: ["Family history of hereditary bleeding disorder", "Anterior uveitis"]

## Encounter 0: source 2020-01-15, practice 2023-01-23, outpatient, Internal Medicine

Chief complaint: evaluation of anemia discovered incidentally during a workplace screening

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of anemia discovered incidentally during a workplace screening

HISTORY OF PRESENT ILLNESS:
He reports feeling well and denies fatigue, weakness, lightheadedness, palpitations, or shortness of breath. He exercises four times per week, works as a software engineer, and has never experienced limitations in his physical endurance. His bowel movements are regular, and he denies any change in stool color or consistency. Based on those findings, he was empirically started on oral iron by a local clinic. He now returns for follow-up, reporting that he has been taking the iron supplement as directed for six weeks without any changes in how he feels.

PAST MEDICAL HISTORY:
Active Problem List:
- Ankylosing Spondylitis
- Beta Thalassemia Minor
- Recurrent Anterior Uveitis

MEDICATIONS:
Home Medications:
- Adalimumab 40 mg subcutaneous every 2 weeks
- Sulfasalazine 1000 mg orally twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His family history is notable for a father and paternal uncle who were once told they had “small red blood cells” but never required treatment or transfusions.

SOCIAL HISTORY:
His diet is well-balanced and includes meat, grains, and vegetables. He does not drink alcohol, smoke, or use recreational drugs.

VITAL SIGNS:
On examination, his vital signs are normal.

PHYSICAL EXAMINATION:
He is well-appearing and in no distress. There is no pallor, jaundice, or scleral icterus. Cardiopulmonary exam is unremarkable. Abdominal exam reveals no hepatosplenomegaly.

LABORATORY DATA:
A complete blood count ordered by his occupational health provider showed anemia with microcytosis. Repeat labs ordered by his primary care physician again show microcytic anemia with no improvement. Iron studies demonstrate normal serum iron and ferritin levels. Hemoglobin electrophoresis is performed, which shows elevated hemoglobin A2 and a mild increase in hemoglobin F.

## Encounter 1: source 2021-01-15, practice 2024-01-29, outpatient, Internal Medicine

Chief complaint: lower back pain

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Okafor, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Lower back pain

HISTORY OF PRESENT ILLNESS:
The patient returns to the clinic with beta thalassemia minor diagnosed in January 2020 and presents with a 6-month history of lower back pain that is worse in the morning and improves with exercise. He also reports occasional episodes of mild anterior uveitis over the past year. The patient has tried multiple courses of ibuprofen with minimal relief of his symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Thalassemia minor (diagnosed 2020-01-15)
- Ankylosing Spondylitis
- Beta Thalassemia Minor
- Recurrent Anterior Uveitis

MEDICATIONS:
Home Medications:
- Adalimumab 40 mg subcutaneous every 2 weeks
- Sulfasalazine 1000 mg orally twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
He has a family history of psoriasis but denies any personal skin problems.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Software engineer.

PHYSICAL EXAMINATION:
On physical examination, there is decreased lumbar flexion and tenderness over the sacroiliac joints.

LABORATORY DATA:
His inflammatory markers, including C-reactive protein and erythrocyte sedimentation rate, are elevated. HLA-B27 testing is positive.

IMAGING:
Radiographs of the sacroiliac joints show bilateral sacroiliitis.

## Encounter 2: source 2021-02-15, practice 2024-03-07, outpatient, Rheumatology

Chief complaint: lower back pain

ENCOUNTER NOTE
Date: 2021-02-15
Provider: Dr. Elena Vasquez, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Lower back pain

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of ankylosing spondylitis and beta thalassemia minor, on adalimumab and sulfasalazine. He reports a 6-month history of lower back pain that is worse in the morning and improves with exercise. He also reports occasional episodes of mild anterior uveitis over the past year. He denies any recent infections or gastrointestinal symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Thalassemia minor (diagnosed 2020-01-15)
- Ankylosing Spondylitis (diagnosed 2021-01-15)
- Beta Thalassemia Minor
- Recurrent Anterior Uveitis

MEDICATIONS:
Home Medications:
- Adalimumab 40 mg subcutaneous every 2 weeks
- Sulfasalazine 1000 mg orally twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His family history is significant for psoriasis in his father.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Software engineer.

PHYSICAL EXAMINATION:
On physical examination, there is tenderness over the sacroiliac joints, and his lumbar spine range of motion is decreased. His Schober's test result is abnormal.

IMAGING:
Plain x-rays of the pelvis show bilateral sacroiliitis.

## Encounter 3: source 2021-03-15, practice 2024-04-08, outpatient, Rheumatology

Chief complaint: a 6-month history of lower back pain

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Elena Vasquez, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
A 6-month history of lower back pain

HISTORY OF PRESENT ILLNESS:
The patient is a 27-year-old male with Beta Thalassemia Minor and Ankylosing Spondylitis returning for follow-up. He describes pain worse in the morning and improving with exercise. He notes occasional episodes of mild anterior uveitis over the past year. He denies recent infections or GI symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Thalassemia minor (diagnosed 2020-01-15)
- Ankylosing Spondylitis (diagnosed 2021-01-15)
- Beta Thalassemia Minor
- Recurrent Anterior Uveitis

MEDICATIONS:
Home Medications:
- Adalimumab 40 mg subcutaneous every 2 weeks
- Sulfasalazine 1000 mg orally twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His family history is significant for psoriasis in his mother.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Software engineer.

VITAL SIGNS:
His vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, there is decreased lumbar flexion and tenderness over the sacroiliac joints.

LABORATORY DATA:
Laboratory tests reveal an elevated C-reactive protein (CRP) and erythrocyte sedimentation rate (ESR), but HLA-B27 testing is pending.

IMAGING:
Radiographs of the pelvis show bilateral sacroiliitis.

## Encounter 4: source 2023-01-15, practice 2026-01-27, outpatient, Rheumatology

Chief complaint: chronic lower back pain

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Aisha Patel, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Chronic lower back pain

HISTORY OF PRESENT ILLNESS:
The patient is a 29-year-old male with ankylosing spondylitis initially diagnosed in January 2021 and beta thalassemia minor who returns to the rheumatology clinic for ongoing management. He presents with chronic lower back pain that has been progressively worsening over the past 2 years. He describes the pain as dull and deep, centered around his lower spine and hips, and notes that it is worse in the mornings or after prolonged rest but improves with physical activity. He has recently started experiencing stiffness that takes about an hour to resolve upon waking. He denies any trauma or recent physical strain. On further questioning, he mentions occasional eye redness and discomfort over the past few months that resolved spontaneously.

PAST MEDICAL HISTORY:
Active Problem List:
- Thalassemia minor (diagnosed 2020-01-15)
- Ankylosing Spondylitis (diagnosed 2021-01-15)
- Beta Thalassemia Minor
- Recurrent Anterior Uveitis

MEDICATIONS:
Home Medications:
- Adalimumab 40 mg subcutaneous every 2 weeks
- Sulfasalazine 1000 mg orally twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
There is no family history of autoimmune disease, but his father had a “bad back” that required surgery in his 40s.

SOCIAL HISTORY:
He works as a software engineer and leads a sedentary lifestyle.

REVIEW OF SYSTEMS:
He denies fevers, chills, or weight loss.

PHYSICAL EXAMINATION:
Physical examination reveals decreased lumbar spine flexion, reduced chest expansion, and tenderness over the sacroiliac joints. His gait is normal, but Schober’s test is positive.

LABORATORY DATA:
Laboratory studies show elevated inflammatory markers, but rheumatoid factor and ANA are negative.

IMAGING:
Radiographs of the spine demonstrate changes suggestive of sacroiliac joint sclerosis and fusion.
