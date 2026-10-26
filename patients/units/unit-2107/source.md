# synthetic_hospital patient 2107

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 38 at first encounter, sex M, Asian (Chinese), Private PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-12-28 (Mon) | outpatient | Gastroenterology |
| 1 | 2020-04-15 | 2021-04-05 (Mon) | outpatient | Endocrinology |
| 2 | 2023-01-15 | 2023-12-19 (Tue) | ed | Emergency Medicine |
| 3 | 2025-01-15 | 2025-12-19 (Fri) | outpatient | Rheumatology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Sulfamethoxazole (anaphylaxis)",
  "Latex (contact dermatitis)"
 ],
 "insurance": "Private PPO",
 "occupation": "Information technology specialist",
 "alcohol_use": "Occasional social use (1-2 drinks per month)",
 "family_history": [
  "Father: Multiple endocrine neoplasia type 2 (MEN2), medullary thyroid carcinoma",
  "Paternal aunt: Pheochromocytoma",
  "Mother: Type 2 diabetes mellitus, essential hypertension",
  "Sister: Chronic hepatitis B carrier"
 ],
 "race_ethnicity": "Asian (Chinese)",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "25 mg PO daily",
   "name": "Tenofovir alafenamide"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Prednisone"
  },
  {
   "dose": "75 mg PO daily",
   "name": "Azathioprine"
  },
  {
   "dose": "4 mg PO daily",
   "name": "Doxazosin"
  },
  {
   "dose": "300 mg PO three times daily",
   "name": "Gabapentin"
  },
  {
   "dose": "125 mcg PO daily",
   "name": "Levothyroxine"
  }
 ],
 "surgical_history": [
  "Left adrenalectomy (age 38, for pheochromocytoma)",
  "Total thyroidectomy with central neck dissection (age 39, for medullary thyroid carcinoma)"
 ],
 "chronic_conditions": [
  "Chronic hepatitis B infection",
  "Polyarteritis nodosa",
  "Multiple endocrine neoplasia type 2 (MEN2)",
  "Hypertension",
  "Peripheral sensory neuropathy"
 ],
 "age_at_first_encounter": 38
}
```

Primary diagnoses: ["Polyarteritis nodosa", "Chronic hepatitis B infection", "Multiple endocrine neoplasia type 2"]
Comorbidities: ["Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2020-12-28, outpatient, Gastroenterology

Chief complaint: Evaluation of chronic hepatitis B infection with fatigue and mild right upper quadrant discomfort

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Jennifer Liu, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of chronic hepatitis B infection with fatigue and mild right upper quadrant discomfort

HISTORY OF PRESENT ILLNESS:
He reports fatigue and mild right upper quadrant discomfort for several months.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic hepatitis B infection
- Polyarteritis nodosa
- Multiple endocrine neoplasia type 2 (MEN2)
- Hypertension
- Peripheral sensory neuropathy

PAST SURGICAL HISTORY:
- Left adrenalectomy (age 38, for pheochromocytoma)
- Total thyroidectomy with central neck dissection (age 39, for medullary thyroid carcinoma)

MEDICATIONS:
Home Medications:
- Tenofovir alafenamide 25 mg PO daily
- Prednisone 5 mg PO daily
- Azathioprine 75 mg PO daily
- Doxazosin 4 mg PO daily
- Gabapentin 300 mg PO three times daily
- Levothyroxine 125 mcg PO daily

ALLERGIES:
Sulfamethoxazole (anaphylaxis), Latex (contact dermatitis)

FAMILY HISTORY:
- Father: Multiple endocrine neoplasia type 2 (MEN2), medullary thyroid carcinoma
- Paternal aunt: Pheochromocytoma
- Mother: Type 2 diabetes mellitus, essential hypertension
- Sister: Chronic hepatitis B carrier

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per month). Occupation: Information technology specialist.

LABORATORY DATA:
Laboratory testing shows elevated transaminases and positive hepatitis B surface antigen (HBsAg) for the past 8 months.

PATHOLOGY:
A liver biopsy demonstrates chronic inflammation with areas of hepatocyte injury.

## Encounter 1: source 2020-04-15, practice 2021-04-05, outpatient, Endocrinology

Chief complaint: Episodic headaches, palpitations, diaphoresis, and fatigue concerning for catecholamine excess

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Robert Kim, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Episodic headaches, palpitations, diaphoresis, and fatigue concerning for catecholamine excess

HISTORY OF PRESENT ILLNESS:
The patient with MEN2 and chronic hepatitis B returns for endocrinology evaluation following his January gastroenterology visit. He notes that he occasionally experiences a racing heart and excessive sweating, especially when these headaches occur. His appetite is normal, but he feels more sluggish lately and has had a few episodes of vague abdominal discomfort.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic hepatitis B infection (diagnosed 2020-01-15)
- Polyarteritis nodosa
- Multiple endocrine neoplasia type 2 (MEN2)
- Hypertension
- Peripheral sensory neuropathy

PAST SURGICAL HISTORY:
- Left adrenalectomy (age 38, for pheochromocytoma)
- Total thyroidectomy with central neck dissection (age 39, for medullary thyroid carcinoma)

MEDICATIONS:
Home Medications:
- Tenofovir alafenamide 25 mg PO daily
- Prednisone 5 mg PO daily
- Azathioprine 75 mg PO daily
- Doxazosin 4 mg PO daily
- Gabapentin 300 mg PO three times daily
- Levothyroxine 125 mcg PO daily

ALLERGIES:
Sulfamethoxazole (anaphylaxis), Latex (contact dermatitis)

FAMILY HISTORY:
Concerning family history of endocrine surgeries

SOCIAL HISTORY:
He denies use of illicit substances

VITAL SIGNS:
On examination, his blood pressure is 166/94 mmHg and heart rate is 92/min.

PHYSICAL EXAMINATION:
Cardiac and abdominal exams are normal.

LABORATORY DATA:
Laboratory studies show borderline hypercalcemia and mildly elevated urinary catecholamine metabolites. additional lab work shows an increased serum calcitonin level.

IMAGING:
Further imaging is notable for a thyroid nodule and adrenal fullness

OTHER STUDIES:
Further analysis reveals a germline point mutation associated with disordered cell growth across multiple tissues.

PLAN:
The patient is referred for genetic counseling.

## Encounter 2: source 2023-01-15, practice 2023-12-19, ed, Emergency Medicine

Chief complaint: Persistent fatigue, generalized weakness, and dark-colored urine

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Angela Martinez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Persistent fatigue, generalized weakness, and dark-colored urine

HISTORY OF PRESENT ILLNESS:
The patient is a 41-year-old male with polyarteritis nodosa, chronic hepatitis B, and MEN2, evaluated in Gastroenterology and Endocrinology, who presents to the emergency department. He is brought in due to persistent fatigue, generalized weakness, and vague abdominal discomfort that have gradually worsened over past 2 weeks. He reports low-grade fevers, unintentional weight loss of 4 kg over last month, and dark-colored urine.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic hepatitis B infection (diagnosed 2020-01-15)
- Multiple endocrine neoplasia type 2 (diagnosed 2020-04-15)
- Polyarteritis nodosa
- Multiple endocrine neoplasia type 2 (MEN2)
- Hypertension
- Peripheral sensory neuropathy

His past medical history is notable for a hepatitis B infection diagnosed 8 months ago, for which he did not complete antiviral treatment.

PAST SURGICAL HISTORY:
- Left adrenalectomy (age 38, for pheochromocytoma)
- Total thyroidectomy with central neck dissection (age 39, for medullary thyroid carcinoma)

MEDICATIONS:
Home Medications:
- Tenofovir alafenamide 25 mg PO daily
- Prednisone 5 mg PO daily
- Azathioprine 75 mg PO daily
- Doxazosin 4 mg PO daily
- Gabapentin 300 mg PO three times daily
- Levothyroxine 125 mcg PO daily

ALLERGIES:
Sulfamethoxazole (anaphylaxis), Latex (contact dermatitis)

FAMILY HISTORY:
- Father: Multiple endocrine neoplasia type 2 (MEN2), medullary thyroid carcinoma
- Paternal aunt: Pheochromocytoma
- Mother: Type 2 diabetes mellitus, essential hypertension
- Sister: Chronic hepatitis B carrier

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per month). Occupation: Information technology specialist.

VITAL SIGNS:
Vital signs show blood pressure of 168/94 mm Hg and temperature of 37.9°C (100.2°F).

PHYSICAL EXAMINATION:
Physical examination reveals no rash or joint deformities. The abdomen is soft with mild tenderness to palpation in the right flank.

LABORATORY DATA:
Laboratory studies show elevated ESR and creatinine, mild normocytic anemia, and urinalysis with microscopic hematuria. ANCA testing is negative.

IMAGING:
A mesenteric angiogram is obtained and shown below.

## Encounter 3: source 2025-01-15, practice 2025-12-19, outpatient, Rheumatology

Chief complaint: Worsening fatigue with unintentional weight loss, lower extremity paresthesia, and painful thigh ulcer

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. David Park, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening fatigue with unintentional weight loss, lower extremity paresthesia, and painful thigh ulcer

HISTORY OF PRESENT ILLNESS:
This 43-year-old man from China with chronic hepatitis B, MEN2, and polyarteritis nodosa diagnosed in 2023 returns for follow-up. He presents with worsening fatigue, unintentional weight loss, and intermittent abdominal pain over the past month, along with numbness and tingling in his lower legs and a recent painful skin ulcer on his thigh.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic hepatitis B infection (diagnosed 2020-01-15)
- Multiple endocrine neoplasia type 2 (diagnosed 2020-04-15)
- Polyarteritis nodosa (diagnosed 2023-01-15)
- Multiple endocrine neoplasia type 2 (MEN2)
- Hypertension
- Peripheral sensory neuropathy

PAST SURGICAL HISTORY:
- Left adrenalectomy (age 38, for pheochromocytoma)
- Total thyroidectomy with central neck dissection (age 39, for medullary thyroid carcinoma)

MEDICATIONS:
Home Medications:
- Tenofovir alafenamide 25 mg PO daily
- Prednisone 5 mg PO daily
- Azathioprine 75 mg PO daily
- Doxazosin 4 mg PO daily
- Gabapentin 300 mg PO three times daily
- Levothyroxine 125 mcg PO daily

ALLERGIES:
Sulfamethoxazole (anaphylaxis), Latex (contact dermatitis)

FAMILY HISTORY:
- Father: Multiple endocrine neoplasia type 2 (MEN2), medullary thyroid carcinoma
- Paternal aunt: Pheochromocytoma
- Mother: Type 2 diabetes mellitus, essential hypertension
- Sister: Chronic hepatitis B carrier

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per month). Occupation: Information technology specialist.

PHYSICAL EXAMINATION:
On physical exam, he has hepatomegaly

LABORATORY DATA:
Laboratory testing reveals elevated AST and ALT. Urinalysis shows mild proteinuria but no red blood cell casts.

IMAGING:
Chest imaging is unremarkable, with no evidence of pulmonary infiltrates, nodules, or hemorrhage. A renal angiogram demonstrates multiple microaneurysms and areas of narrowing in medium-sized arteries.

PATHOLOGY:
A liver biopsy shows transmural inflammation of medium-sized arteries with fibrinoid necrosis and surrounding inflammatory infiltrates.
