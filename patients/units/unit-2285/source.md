# synthetic_hospital patient 2285

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 64 at first encounter, sex M, White (Northern European ancestry), Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-01-11 (Mon) | outpatient | Internal Medicine |
| 1 | 2021-01-15 | 2022-01-21 (Fri) | outpatient | Nephrology |
| 2 | 2023-01-15 | 2024-01-19 (Fri) | outpatient | Urology |
| 3 | 2025-01-15 | 2026-01-09 (Fri) | inpatient | General Surgery |
| 4 | 2025-07-15 | 2026-07-12 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired postal worker",
 "alcohol_use": "None",
 "family_history": [
  "Father with Type 2 Diabetes Mellitus",
  "Mother with gastric adenocarcinoma",
  "Maternal grandfather with colon cancer"
 ],
 "race_ethnicity": "White (Northern European ancestry)",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "30 units subcutaneously at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus (25-year duration, poorly controlled)",
  "Hypertension",
  "Diabetic peripheral neuropathy",
  "Diabetic retinopathy",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 64
}
```

Primary diagnoses: ["Chronic kidney disease", "Neurogenic bladder due to sacral nerve root injury", "Gastric adenocarcinoma", "Vitamin B12 deficiency"]
Comorbidities: ["Type 2 diabetes mellitus", "Diabetic autonomic neuropathy", "Iron Deficiency Anemia"]

## Encounter 0: source 2020-01-15, practice 2021-01-11, outpatient, Internal Medicine

Chief complaint: progressive fatigue, numbness in feet, and trouble walking over the past several months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Sharma, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue, numbness in feet, and trouble walking over the past several months

HISTORY OF PRESENT ILLNESS:
Progressive fatigue, numbness in his feet, and trouble walking over the past several months. He reports that he has also been experiencing occasional memory lapses and a sensation of "pins and needles" in his hands and legs.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus (25-year duration, poorly controlled)
- Hypertension
- Diabetic peripheral neuropathy
- Diabetic retinopathy
- Hyperlipidemia

He was diagnosed with type 2 diabetes 8 years ago but has had poor follow-up and limited access to care.

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Insulin glargine 30 units subcutaneously at bedtime
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Type 2 Diabetes Mellitus
- Mother with gastric adenocarcinoma
- Maternal grandfather with colon cancer

SOCIAL HISTORY:
He is originally from northern Europe and follows a vegan diet for ethical reasons and denies alcohol or tobacco use.

PHYSICAL EXAMINATION:
On examination, he is pale and appears mildly disoriented to date. Neurologic exam shows decreased proprioception and vibration sense in both lower extremities, with a wide-based gait.

LABORATORY DATA:
Hematologic labs reveal macrocytic anemia with a hemoglobin of 9.2 g/dL and mean corpuscular volume (MCV) of 112 fL. Peripheral smear shows hypersegmented neutrophils. Additional testing confirms a deficiency in a cofactor required for homocysteine remethylation.

## Encounter 1: source 2021-01-15, practice 2022-01-21, outpatient, Nephrology

Chief complaint: progressive fatigue and exertional dyspnea over 2 months

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Robert Johnson, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue and exertional dyspnea over 2 months

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up approximately one year after his prior encounter for vitamin B12 deficiency. He presents with a 2-month history of progressive fatigue and exertional dyspnea. He denies chest pain, palpitations, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitamin B12 deficiency (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus (25-year duration, poorly controlled)
- Hypertension
- Diabetic peripheral neuropathy
- Diabetic retinopathy
- Hyperlipidemia

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Insulin glargine 30 units subcutaneously at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Type 2 Diabetes Mellitus
- Mother with gastric adenocarcinoma
- Maternal grandfather with colon cancer

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired postal worker.

VITAL SIGNS:
On physical examination, his blood pressure is 140/85 mmHg, heart rate is 88 beats per minute, and respiratory rate is 18 breaths per minute.

PHYSICAL EXAMINATION:
Cardiac examination reveals a regular rhythm without murmurs, rubs, or gallops. Lung fields are clear to auscultation bilaterally.

LABORATORY DATA:
- Hemoglobin: 9.8 g/dL (Normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 72 fL (Normal: 80-96 fL)
- Serum creatinine: 1.9 mg/dL (Normal: 0.6-1.3 mg/dL)
- Blood urea nitrogen (BUN): 36 mg/dL (Normal: 7-20 mg/dL)
- Serum potassium: 5.5 mEq/L (Normal: 3.5-5.0 mEq/L)

## Encounter 2: source 2023-01-15, practice 2024-01-19, outpatient, Urology

Chief complaint: progressively worsening difficulty initiating urination with slow stream and post-void dribbling over the past year

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Aisha Washington, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressively worsening difficulty initiating urination with slow stream and post-void dribbling over the past year

HISTORY OF PRESENT ILLNESS:
The patient is a 67-year-old male with a 25-year history of poorly controlled Type 2 Diabetes Mellitus complicated by peripheral neuropathy, who was previously evaluated for vitamin B12 deficiency and chronic kidney disease, and now presents with urinary complaints. Over the past year, he has noticed progressively worsening difficulty initiating urination and reports that his urine stream is slow and occasionally stops and starts. He also describes frequent dribbling of urine, especially after he thinks he has finished voiding. Sometimes he feels the urge to urinate but cannot empty his bladder completely. Over the last few weeks, he has begun waking up multiple times at night with wet underwear but no awareness that he had urinated.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitamin B12 deficiency (diagnosed 2020-01-15)
- Chronic kidney disease (diagnosed 2021-01-15)
- Type 2 Diabetes Mellitus (25-year duration, poorly controlled)
- Hypertension
- Diabetic peripheral neuropathy
- Diabetic retinopathy
- Hyperlipidemia

His past medical history is significant for poorly controlled type 2 diabetes mellitus for over 15 years with known peripheral neuropathy and retinopathy.

MEDICATIONS:
He takes insulin and multiple oral antihyperglycemics but has missed several follow-up appointments.

Home Medications:
- Metformin 1000 mg PO BID
- Insulin glargine 30 units subcutaneously at bedtime
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Type 2 Diabetes Mellitus
- Mother with gastric adenocarcinoma
- Maternal grandfather with colon cancer

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired postal worker.

REVIEW OF SYSTEMS:
He denies dysuria, suprapubic pain, fever, or hematuria.

PHYSICAL EXAMINATION:
On physical examination, the bladder is palpable above the pubic symphysis and mildly tender to percussion. A focused neurologic exam shows decreased sensation in the lower extremities with diminished ankle reflexes bilaterally.

OTHER STUDIES:
Post-void residual urine volume is measured at 360 mL.

## Encounter 3: source 2025-01-15, practice 2026-01-09, inpatient, General Surgery

Chief complaint: ongoing upper abdominal discomfort with early fullness and nausea

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Carlos Martinez, MD
Department: General Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Ongoing upper abdominal discomfort with early fullness and nausea

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old male with poorly controlled diabetes mellitus of 25 years duration complicated by peripheral neuropathy, retinopathy, chronic kidney disease, prior B12 deficiency, and neurogenic bladder, presenting with abdominal discomfort that began several months ago and has gradually worsened. He describes the pain as dull and constant, located just below the breastbone, and notes that it seems to get worse after eating, leading him to reduce his food intake. He reports early fullness and occasional episodes of nausea, although he has not vomited recently. Over the last few months, he has unintentionally lost about 15 pounds.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitamin B12 deficiency (diagnosed 2020-01-15)
- Chronic kidney disease (diagnosed 2021-01-15)
- Neurogenic bladder due to sacral nerve root injury (diagnosed 2023-01-15)
- Type 2 Diabetes Mellitus (25-year duration, poorly controlled)
- Hypertension
- Diabetic peripheral neuropathy
- Diabetic retinopathy
- Hyperlipidemia

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Insulin glargine 30 units subcutaneously at bedtime
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Type 2 Diabetes Mellitus
- Mother with gastric adenocarcinoma
- Maternal grandfather with colon cancer

SOCIAL HISTORY:
He denies alcohol use

PHYSICAL EXAMINATION:
Physical examination reveals mild epigastric tenderness but no palpable abdominal mass. Cardiopulmonary and neurological exams are unremarkable.

LABORATORY DATA:
Laboratory investigations are notable for mild iron-deficiency anemia, and liver enzymes are within normal limits.

IMAGING:
An upper abdominal imaging study shows signs suggestive of delayed gastric emptying and focal narrowing near the distal stomach.

PATHOLOGY:
A biopsy reveals malignant transformation involving the pyloric region.

## Encounter 4: source 2025-07-15, practice 2026-07-12, ed, Emergency Medicine

Chief complaint: 4-month history of continuous urinary leakage and incomplete bladder emptying

ENCOUNTER NOTE
Date: 2025-07-15
Provider: Dr. Jennifer Liu, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
4-month history of continuous urinary leakage and incomplete bladder emptying

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old male with a history of neurogenic bladder secondary to sacral nerve root injury initially evaluated in 2023, complicated by 25 years of poorly controlled Type 2 Diabetes Mellitus with peripheral neuropathy and recent diagnosis of gastric adenocarcinoma, who presents to the Emergency Department with progression of lower urinary tract symptoms. He describes his leakage as continuous dribbling of urine even after voiding, accompanied by a persistent feeling of incomplete bladder emptying. He reports that he frequently finds himself needing to change his underwear multiple times per day and that these symptoms have gradually worsened over time. He denies any burning during urination, hematuria, or suprapubic pain. He also notes that despite making frequent trips to the bathroom, he never feels fully relieved. His wife mentions that he often has difficulty initiating urination and sometimes seems to have a prolonged delay before a strong stream begins. On further questioning, he admits that he previously had episodes suggestive of urinary retention, for which he was intermittently catheterized in the past.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitamin B12 deficiency (diagnosed 2020-01-15)
- Chronic kidney disease (diagnosed 2021-01-15)
- Neurogenic bladder due to sacral nerve root injury (diagnosed 2023-01-15)
- Gastric adenocarcinoma (diagnosed 2025-01-15)
- Type 2 Diabetes Mellitus (25-year duration, poorly controlled)
- Hypertension
- Diabetic peripheral neuropathy
- Diabetic retinopathy
- Hyperlipidemia

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Insulin glargine 30 units subcutaneously at bedtime
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Type 2 Diabetes Mellitus
- Mother with gastric adenocarcinoma
- Maternal grandfather with colon cancer

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired postal worker.

PHYSICAL EXAMINATION:
His physical exam reveals a soft, non-tender abdomen with a palpable lower abdominal mass that is consistent with bladder distension. Digital rectal examination shows a normal-sized, smooth prostate. Neurologic examination reveals decreased sensation in the feet and legs, consistent with diabetic peripheral neuropathy.

LABORATORY DATA:
Laboratory studies reveal an HbA1c of 9.2% and normal renal function.

IMAGING:
A bladder ultrasound confirms a post-void residual volume of 450 mL, indicating significant urinary retention.
