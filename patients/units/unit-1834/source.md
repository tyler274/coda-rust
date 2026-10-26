# synthetic_hospital patient 1834

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 34 at first encounter, sex F, African American, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-10-18 (Mon) | outpatient | Endocrinology |
| 1 | 2020-03-15 | 2021-12-20 (Mon) | outpatient | Internal Medicine |
| 2 | 2020-09-15 | 2022-06-27 (Mon) | outpatient | Dermatology |
| 3 | 2021-03-15 | 2022-12-19 (Mon) | outpatient | Pulmonology |
| 4 | 2021-09-15 | 2023-06-21 (Wed) | outpatient | Rheumatology |
| 5 | 2022-03-15 | 2023-12-13 (Wed) | outpatient | Internal Medicine |
| 6 | 2023-03-15 | 2024-12-16 (Mon) | outpatient | Neurology |
| 7 | 2024-03-15 | 2025-12-11 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Maternal aunt with sarcoidosis",
  "Mother with systemic lupus erythematosus",
  "Paternal cousin with ulcerative colitis",
  "Father with type 2 diabetes mellitus"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "1.2 g orally twice daily",
   "name": "Mesalamine"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Sarcoidosis",
  "Ulcerative colitis",
  "Primary hyperparathyroidism"
 ],
 "age_at_first_encounter": 34
}
```

Primary diagnoses: ["Sarcoidosis", "Primary hyperparathyroidism", "Enteropathic arthritis (Peripheral arthritis associated with inflammatory bowel disease)"]
Comorbidities: ["Erythema nodosum (isolated)", "Malignancy-associated hypercalcemia"]

## Encounter 0: source 2020-01-15, practice 2021-10-18, outpatient, Endocrinology

Chief complaint: Six-month history of fatigue, constipation, and depression

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of fatigue, constipation, and depression

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with a 6-month history of fatigue, constipation, and depression.

PAST MEDICAL HISTORY:
Active Problem List:
- Sarcoidosis
- Ulcerative colitis
- Primary hyperparathyroidism

MEDICATIONS:
Home Medications:
- Mesalamine 1.2 g orally twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Mentions that her mother had a "gland problem" that required surgery.

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Her blood pressure is 130/85 mmHg, and her heart rate is 78 bpm.

PHYSICAL EXAMINATION:
On physical examination, she appears well, with no obvious abnormalities.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum calcium: 11.2 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum phosphate: 2.4 mg/dL (normal: 2.5-4.5 mg/dL)
- Serum creatinine: 0.9 mg/dL (normal: 0.6-1.1 mg/dL)
- Parathyroid hormone (PTH): Elevated

A 24-hour urine collection shows hypercalciuria.

IMAGING:
A dual-energy X-ray absorptiometry (DEXA) scan indicates decreased bone mineral density consistent with osteopenia.

## Encounter 1: source 2020-03-15, practice 2021-12-20, outpatient, Internal Medicine

Chief complaint: Two-month history of progressive shortness of breath, dry cough, and painful red nodules on shins

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Michael Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Two-month history of progressive shortness of breath, dry cough, and painful red nodules on shins

HISTORY OF PRESENT ILLNESS:
This 34-year-old African American female with ulcerative colitis and primary hyperparathyroidism diagnosed two months prior presents to the clinic with a 2-month history of progressive shortness of breath, dry cough, and fatigue. She also reports painful red nodules on her shins that appeared a few weeks ago. She denies joint pains, eye symptoms, or recent travel.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary hyperparathyroidism (diagnosed 2020-01-15)
- Sarcoidosis
- Ulcerative colitis

MEDICATIONS:
Home Medications:
- Mesalamine 1.2 g orally twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal aunt with sarcoidosis
- Mother with systemic lupus erythematosus
- Paternal cousin with ulcerative colitis
- Father with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
On physical examination, her vital signs are within normal limits.

PHYSICAL EXAMINATION:
Lung auscultation reveals bilateral crackles at the lung bases. Dermatological examination shows multiple tender, erythematous nodules on the anterior aspects of both lower legs consistent with erythema nodosum.

LABORATORY DATA:
Laboratory results are as follows:

- Serum calcium: 11.8 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum creatinine: 0.9 mg/dL (normal: 0.6-1.1 mg/dL)
- Serum albumin: 4.2 g/dL (normal: 3.5-5.0 g/dL)
- Parathyroid hormone (PTH): Low
- 25-hydroxyvitamin D: Normal
- 1,25-dihydroxyvitamin D (calcitriol): Elevated
- Angiotensin-converting enzyme (ACE): Elevated
- Urinary calcium: Increased

IMAGING:
Chest radiography shows bilateral hilar lymphadenopathy.

## Encounter 2: source 2020-09-15, practice 2022-06-27, outpatient, Dermatology

Chief complaint: Progressive dry cough, mild dyspnea, and fatigue over six months

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Jessica Williams, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive dry cough, mild dyspnea, and fatigue over six months

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of sarcoidosis from March 2020. She presents with a 6-month history of progressive dry cough, mild dyspnea, and fatigue. She has no history of smoking or recent infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary hyperparathyroidism (diagnosed 2020-01-15)
- Sarcoidosis (diagnosed 2020-03-15)
- Ulcerative colitis

MEDICATIONS:
Home Medications:
- Mesalamine 1.2 g orally twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal aunt with sarcoidosis
- Mother with systemic lupus erythematosus
- Paternal cousin with ulcerative colitis
- Father with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On examination, she has mild erythema nodosum on her shins and no wheezing or crackles on lung auscultation.

LABORATORY DATA:
Laboratory testing shows normal serum phosphate and elevated calcium.

IMAGING:
Chest radiograph reveals bilateral hilar lymphadenopathy without focal infiltrates.

PATHOLOGY:
A lung biopsy is performed, and histology is shown below.

## Encounter 3: source 2021-03-15, practice 2022-12-19, outpatient, Pulmonology

Chief complaint: Three-month history of dry cough, mild shortness of breath, and occasional fevers

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Robert Kim, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of dry cough, mild shortness of breath, and occasional fevers

PAST MEDICAL HISTORY:
Active Problem List:
- Primary hyperparathyroidism (diagnosed 2020-01-15)
- Sarcoidosis (diagnosed 2020-03-15)
- Ulcerative colitis

She denies any significant medical history.

MEDICATIONS:
Home Medications:
- Mesalamine 1.2 g orally twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal aunt with sarcoidosis
- Mother with systemic lupus erythematosus
- Paternal cousin with ulcerative colitis
- Father with type 2 diabetes mellitus

SOCIAL HISTORY:
She works as a school teacher and has not traveled recently.

VITAL SIGNS:
On physical examination, her temperature is 37.8°C (100°F), blood pressure is 120/75 mmHg, pulse is 80 beats per minute, and respiratory rate is 18 breaths per minute

PHYSICAL EXAMINATION:
Lung auscultation reveals bilateral crackles at the lung bases. No skin lesions or lymphadenopathy are palpable.

LABORATORY DATA:
Laboratory tests reveal:
- Serum calcium: 10.8 mg/dL (Normal: 8.5-10.2 mg/dL)
- Serum angiotensin-converting enzyme (ACE): Elevated
- Complete blood count: Normal
- Liver function tests: Normal
- Tuberculin skin test: Negative

IMAGING:
A chest x-ray shows bilateral hilar adenopathy and reticular opacities.

## Encounter 4: source 2021-09-15, practice 2023-06-21, outpatient, Rheumatology

Chief complaint: Three-month history of dry cough, fatigue, and ten-pound unintentional weight loss

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Aisha Johnson, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of dry cough, fatigue, and ten-pound unintentional weight loss

HISTORY OF PRESENT ILLNESS:
The patient with known sarcoidosis, ulcerative colitis, and primary hyperparathyroidism returns for rheumatology evaluation six months following her last Pulmonology assessment, presenting with a three-month history of dry cough, fatigue, and ten-pound unintentional weight loss. She denies fever, night sweats, or hemoptysis.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary hyperparathyroidism (diagnosed 2020-01-15)
- Sarcoidosis (diagnosed 2020-03-15)
- Ulcerative colitis

MEDICATIONS:
Home Medications:
- Mesalamine 1.2 g orally twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal aunt with sarcoidosis
- Mother with systemic lupus erythematosus
- Paternal cousin with ulcerative colitis
- Father with type 2 diabetes mellitus

SOCIAL HISTORY:
She works as a school teacher and has not traveled recently.

VITAL SIGNS:
On physical examination, she is afebrile with normal vital signs

PHYSICAL EXAMINATION:
Lung auscultation reveals bilateral crackles at the lung bases. No lymphadenopathy is palpable.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum calcium: 11.2 mg/dL (Normal: 8.5-10.2 mg/dL)
- Serum creatinine: 0.9 mg/dL (Normal: 0.6-1.1 mg/dL)
- Angiotensin-converting enzyme (ACE) level: Elevated
- 1,25-dihydroxyvitamin D level: Elevated

IMAGING:
A chest x-ray shows bilateral hilar lymphadenopathy and reticulonodular infiltrates.

PATHOLOGY:
A biopsy of a mediastinal lymph node is performed, which shows noncaseating granulomas.

## Encounter 5: source 2022-03-15, practice 2023-12-13, outpatient, Internal Medicine

Chief complaint: Persistent fatigue, vague musculoskeletal discomfort, blurry vision, and generalized weakness over six months

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. David Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue, vague musculoskeletal discomfort, blurry vision, and generalized weakness over six months

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of her sarcoidosis and primary hyperparathyroidism, previously managed for pulmonary and constitutional symptoms. She reports generalized weakness, occasional mild headaches, and intermittent blurry vision, particularly when reading at work. She also endorses mild shortness of breath when carrying groceries upstairs but denies cough, wheezing, fevers, or night sweats. She has noticed a recent decline in her concentration and productivity.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary hyperparathyroidism (diagnosed 2020-01-15)
- Sarcoidosis (diagnosed 2020-03-15)
- Ulcerative colitis

MEDICATIONS:
She does not take any medications

Home Medications:
- Mesalamine 1.2 g orally twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is unremarkable

SOCIAL HISTORY:
She works as a legal assistant

REVIEW OF SYSTEMS:
On review of systems, she mentions experiencing dryness and irritation in her eyes over the past few weeks

VITAL SIGNS:
Temperature is 36.8°C (98.2°F), blood pressure is 122/80 mmHg, pulse is 78/min, and respiratory rate is 14/min

PHYSICAL EXAMINATION:
Cardiopulmonary examination is unremarkable. Neurologic examination shows normal strength and reflexes. Dermatologic examination is notable for scattered firm, mildly tender, erythematous nodules on the anterior aspects of both lower legs.

LABORATORY DATA:
Laboratory evaluation reveals normal renal function. Serum calcium is elevated with suppressed parathyroid hormone levels.

IMAGING:
Chest x-ray demonstrates prominent bilateral hilar fullness without lung infiltrates or pleural effusion.

## Encounter 6: source 2023-03-15, practice 2024-12-16, outpatient, Neurology

Chief complaint: Three-week history of progressive right-sided facial weakness with persistent dry cough

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. Emily Thompson, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-week history of progressive right-sided facial weakness with persistent dry cough

HISTORY OF PRESENT ILLNESS:
The patient is a 37-year-old female with established sarcoidosis and primary hyperparathyroidism returning for ongoing evaluation, most recently assessed twelve months prior for constitutional and ocular symptoms. She also reports a persistent dry cough and occasional shortness of breath over the past two months. She denies any fever, weight loss, or night sweats.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary hyperparathyroidism (diagnosed 2020-01-15)
- Sarcoidosis (diagnosed 2020-03-15)
- Ulcerative colitis

MEDICATIONS:
Home Medications:
- Mesalamine 1.2 g orally twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal aunt with sarcoidosis
- Mother with systemic lupus erythematosus
- Paternal cousin with ulcerative colitis
- Father with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, you note right facial droop with inability to close her right eye or raise her right eyebrow. Her lung examination reveals bilateral hilar fullness, but no wheezes, rales, or rhonchi.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum calcium: 11.2 mg/dL (Normal: 8.5-10.2 mg/dL)
- Serum angiotensin-converting enzyme (ACE): Elevated
- Complete blood count: Normal
- Antinuclear antibody (ANA): Negative
- Rheumatoid factor (RF): Negative

IMAGING:
A chest X-ray shows bilateral hilar lymphadenopathy.

## Encounter 7: source 2024-03-15, practice 2025-12-11, ed, Emergency Medicine

Chief complaint: Two-week history of progressively worsening pain and swelling in right knee and left ankle with low-grade fevers

ENCOUNTER NOTE
Date: 2024-03-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-week history of progressively worsening pain and swelling in right knee and left ankle with low-grade fevers

HISTORY OF PRESENT ILLNESS:
The patient with sarcoidosis, ulcerative colitis, hyperparathyroidism, and prior facial weakness presents. She denies recent trauma to the affected joints but reports low-grade fevers and a rash on her lower legs.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary hyperparathyroidism (diagnosed 2020-01-15)
- Sarcoidosis (diagnosed 2020-03-15)
- Ulcerative colitis

Her ulcerative colitis has been well-controlled on mesalamine.

MEDICATIONS:
Mesalamine

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal aunt with sarcoidosis
- Mother with systemic lupus erythematosus
- Paternal cousin with ulcerative colitis
- Father with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
Physical examination reveals a warm, swollen right knee with limited range of motion due to pain, and a similarly affected left ankle. There is no evidence of joint erosion or deformity. A pustular rash is noted on her lower extremities.

LABORATORY DATA:
Laboratory tests reveal:
- Hemoglobin: 12.1 g/dL
- White blood cell count: 9,800 /μL
- Erythrocyte sedimentation rate (ESR): 45 mm/hr (normal: 0-20 mm/hr)
- C-reactive protein (CRP): 3.2 mg/dL (normal: <0.8 mg/dL)
- Rheumatoid factor (RF): Negative
- Antinuclear antibody (ANA): Negative

IMAGING:
Radiographs of the affected joints show soft tissue swelling without joint space narrowing, erosions, or osteopenia.
