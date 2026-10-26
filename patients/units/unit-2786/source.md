# synthetic_hospital patient 2786

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 37 at first encounter, sex F, Caucasian, Private employer-sponsored.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-05-31 (Mon) | outpatient | Endocrinology |
| 1 | 2020-07-15 | 2021-11-29 (Mon) | outpatient | Endocrinology |
| 2 | 2021-01-15 | 2022-05-23 (Mon) | ed | Emergency Medicine |
| 3 | 2021-05-15 | 2022-09-30 (Fri) | icu | Emergency Medicine |
| 4 | 2021-07-15 | 2022-11-29 (Tue) | outpatient | Otolaryngology |
| 5 | 2023-01-15 | 2024-05-24 (Fri) | ed | Emergency Medicine |
| 6 | 2025-01-15 | 2026-06-09 (Tue) | ed | Cardiology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Methimazole (severe adverse reaction: agranulocytosis)",
  "Contrast dye (anaphylaxis)"
 ],
 "insurance": "Private employer-sponsored",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Mother: Autoimmune hypothyroidism (Hashimoto's thyroiditis)",
  "Sister: Graves' disease",
  "Paternal grandmother: Rheumatoid arthritis"
 ],
 "race_ethnicity": "Caucasian",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "20 mg oral twice daily",
   "name": "Propranolol"
  },
  {
   "dose": "40 mg oral daily (tapering regimen for ophthalmopathy prophylaxis)",
   "name": "Prednisone"
  },
  {
   "dose": "500 mg oral every 6 hours as needed for fever or pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Radioiodine thyroid ablation (performed 8 weeks prior to index encounter)"
 ],
 "chronic_conditions": [
  "Graves' disease",
  "Thyroid-associated orbitopathy",
  "History of antithyroid drug-induced agranulocytosis",
  "Status post radioiodine ablation"
 ],
 "age_at_first_encounter": 37
}
```

Primary diagnoses: ["Orbital compartment syndrome", "Graves disease", "Thyroid Storm", "Subacute thyroiditis", "Hyperthyroidism", "Agranulocytosis"]
Comorbidities: ["Atrial fibrillation", "Hyperthyroidism", "Graves disease"]

## Encounter 0: source 2020-01-15, practice 2021-05-31, outpatient, Endocrinology

Chief complaint: 2-month history of weight loss despite increased appetite, palpitations, heat intolerance, and tremor

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
2-month history of weight loss despite increased appetite, palpitations, heat intolerance, and tremor

HISTORY OF PRESENT ILLNESS:
She also reports recent difficulty sleeping and increased frequency of bowel movements. Her menses have also become unpredictable.

PAST MEDICAL HISTORY:
Active Problem List:
- Graves' disease
- Thyroid-associated orbitopathy
- History of antithyroid drug-induced agranulocytosis
- Status post radioiodine ablation

PAST SURGICAL HISTORY:
- Radioiodine thyroid ablation (performed 8 weeks prior to index encounter)

MEDICATIONS:
Home Medications:
- Propranolol 20 mg oral twice daily
- Prednisone 40 mg oral daily (tapering regimen for ophthalmopathy prophylaxis)
- Acetaminophen 500 mg oral every 6 hours as needed for fever or pain

ALLERGIES:
Methimazole (severe adverse reaction: agranulocytosis), Contrast dye (anaphylaxis)

FAMILY HISTORY:
- Mother: Autoimmune hypothyroidism (Hashimoto's thyroiditis)
- Sister: Graves' disease
- Paternal grandmother: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
HR is 90. BP is 130/80.

PHYSICAL EXAMINATION:
There is a fine tremor of the hands on physical exam. On further exam, she has mild lid lag with proptosis and pretibial edema.

LABORATORY DATA:
Laboratory testing shows a nearly undectable thyroid-stimulating hormone (TSH) level.

IMAGING:
Radioactive iodine uptake shows diffusely increased uptake throughout the gland.

## Encounter 1: source 2020-07-15, practice 2021-11-29, outpatient, Endocrinology

Chief complaint: 6-month history of increasing fatigue, heat intolerance, palpitations, and proximal muscle weakness

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of increasing fatigue, heat intolerance, palpitations, and proximal muscle weakness

HISTORY OF PRESENT ILLNESS:
The patient with Graves disease, thyroid-associated orbitopathy, history of antithyroid drug-induced agranulocytosis, and status post radioiodine ablation returns for follow-up six months after initial diagnosis. She presents with a 6-month history of increasing fatigue, heat intolerance, palpitations, 10-pound weight loss despite increased appetite, and difficulty climbing stairs and rising from a seated position.

PAST MEDICAL HISTORY:
Active Problem List:
- Graves disease (diagnosed 2020-01-15)
- Thyroid-associated orbitopathy
- History of antithyroid drug-induced agranulocytosis
- Status post radioiodine ablation

Her medical history is significant for Graves' disease, for which she has been noncompliant with her medications.

PAST SURGICAL HISTORY:
- Radioiodine thyroid ablation (performed 8 weeks prior to index encounter)

MEDICATIONS:
Home Medications:
- Propranolol 20 mg oral twice daily
- Prednisone 40 mg oral daily (tapering regimen for ophthalmopathy prophylaxis)
- Acetaminophen 500 mg oral every 6 hours as needed for fever or pain

ALLERGIES:
Methimazole (severe adverse reaction: agranulocytosis), Contrast dye (anaphylaxis)

FAMILY HISTORY:
- Mother: Autoimmune hypothyroidism (Hashimoto's thyroiditis)
- Sister: Graves' disease
- Paternal grandmother: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, she has a fine tremor, an enlarged thyroid gland without nodules, and a resting heart rate of 102 bpm. Muscle strength is 3/5 in the proximal muscles of the lower extremities and 4/5 in the upper extremities. Reflexes are brisk, and there is mild muscle atrophy in the thighs.

LABORATORY DATA:
Laboratory tests reveal the following:

- Thyroid-stimulating hormone (TSH): <0.01 mIU/L (normal range: 0.4-4.0 mIU/L)
- Free thyroxine (FT4): 3.2 ng/dL (normal range: 0.9-1.7 ng/dL)
- Creatine kinase (CK): 55 U/L (normal range: 38-174 U/L)

## Encounter 2: source 2021-01-15, practice 2022-05-23, ed, Emergency Medicine

Chief complaint: 2-day history of sore throat and fever with progressive worsening

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-day history of sore throat and fever with progressive worsening

HISTORY OF PRESENT ILLNESS:
The patient with Graves disease, thyroid-associated orbitopathy, and history of antithyroid drug-induced agranulocytosis presents to the emergency department. She reports that her symptoms have progressively worsened despite symptomatic treatment with acetaminophen.

PAST MEDICAL HISTORY:
Active Problem List:
- Graves disease (diagnosed 2020-01-15)
- Thyroid-associated orbitopathy
- History of antithyroid drug-induced agranulocytosis
- Status post radioiodine ablation

PAST SURGICAL HISTORY:
- Radioiodine thyroid ablation (performed 8 weeks prior to index encounter)

MEDICATIONS:
She has been on methimazole for the past 6 months.

Home Medications:
- Propranolol 20 mg oral twice daily
- Prednisone 40 mg oral daily (tapering regimen for ophthalmopathy prophylaxis)
- Acetaminophen 500 mg oral every 6 hours as needed for fever or pain

ALLERGIES:
Methimazole (severe adverse reaction: agranulocytosis), Contrast dye (anaphylaxis)

FAMILY HISTORY:
- Mother: Autoimmune hypothyroidism (Hashimoto's thyroiditis)
- Sister: Graves' disease
- Paternal grandmother: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Her temperature is 38.9°C (102°F), blood pressure is 130/85 mmHg, pulse is 88/min, and respiratory rate is 16/min.

PHYSICAL EXAMINATION:
Physical examination reveals pharyngeal erythema without exudates, and no cervical lymphadenopathy is palpable. Her thyroid is diffusely enlarged with a bruit. No rashes or joint swelling are noted.

LABORATORY DATA:
Laboratory values are as follows:

- White Blood Cell Count (WBC): 1,500/mm³ (Normal: 4,500-11,000/mm³)
- Neutrophils: 20% (Normal: 40-60%)
- Hemoglobin: 13.2 g/dL (Normal: 12-16 g/dL)
- Platelets: 250,000/mm³ (Normal: 150,000-450,000/mm³)
- Thyroid-Stimulating Hormone (TSH): <0.01 IU/mL (Normal: 0.4-4.0 IU/mL)
- Free T4: 3.0 ng/dL (Normal: 0.9-1.7 ng/dL)

## Encounter 3: source 2021-05-15, practice 2022-09-30, icu, Emergency Medicine

Chief complaint: Worsening confusion, agitation, and high fever over the past day

ENCOUNTER NOTE
Date: 2021-05-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Worsening confusion, agitation, and high fever over the past day

HISTORY OF PRESENT ILLNESS:
This 38-year-old female with Graves disease, thyroid-associated orbitopathy, and prior antithyroid drug-induced agranulocytosis, status post radioiodine ablation, returns to the emergency department and is brought by her partner due to worsening confusion and agitation over the past day. Over the past three days, she has experienced fever, chills, and a sore throat but did not seek medical attention.

PAST MEDICAL HISTORY:
Active Problem List:
- Graves disease (diagnosed 2020-01-15)
- Agranulocytosis (diagnosed 2021-01-15)
- Thyroid-associated orbitopathy
- History of antithyroid drug-induced agranulocytosis
- Status post radioiodine ablation

She has a history of untreated hyperthyroidism diagnosed eight months ago but discontinued follow-up because she felt "her symptoms were manageable.".

PAST SURGICAL HISTORY:
- Radioiodine thyroid ablation (performed 8 weeks prior to index encounter)

MEDICATIONS:
Home Medications:
- Propranolol 20 mg oral twice daily
- Prednisone 40 mg oral daily (tapering regimen for ophthalmopathy prophylaxis)
- Acetaminophen 500 mg oral every 6 hours as needed for fever or pain

ALLERGIES:
Methimazole (severe adverse reaction: agranulocytosis), Contrast dye (anaphylaxis)

FAMILY HISTORY:
- Mother: Autoimmune hypothyroidism (Hashimoto's thyroiditis)
- Sister: Graves' disease
- Paternal grandmother: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Temperature is 39.7°C (103.5°F), pulse is 140/min and irregular, blood pressure is 150/80 mmHg, and respiratory rate is 22/min.

PHYSICAL EXAMINATION:
On arrival, she is restless, disoriented, and diaphoretic. Physical examination reveals warm, moist skin, a fine tremor of both hands, and brisk reflexes.

LABORATORY DATA:
Laboratory studies reveal: TSH <0.01 μU/mL Free T4 6.4 ng/dL Total T3 350 ng/dL

OTHER STUDIES:
ECG shows atrial fibrillation with rapid ventricular response.

## Encounter 4: source 2021-07-15, practice 2022-11-29, outpatient, Otolaryngology

Chief complaint: Anterior neck discomfort that began a week ago without palpitations or weight changes

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James O'Brien, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Anterior neck discomfort that began a week ago without palpitations or weight changes

HISTORY OF PRESENT ILLNESS:
Graves' disease patient post-thyroid storm presents with anterior neck discomfort that began a week ago. She denies palpitations or significant weight changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Graves disease (diagnosed 2020-01-15)
- Agranulocytosis (diagnosed 2021-01-15)
- Thyroid Storm (diagnosed 2021-05-15)
- Thyroid-associated orbitopathy
- History of antithyroid drug-induced agranulocytosis
- Status post radioiodine ablation

PAST SURGICAL HISTORY:
- Radioiodine thyroid ablation (performed 8 weeks prior to index encounter)

MEDICATIONS:
Home Medications:
- Propranolol 20 mg oral twice daily
- Prednisone 40 mg oral daily (tapering regimen for ophthalmopathy prophylaxis)
- Acetaminophen 500 mg oral every 6 hours as needed for fever or pain

ALLERGIES:
Methimazole (severe adverse reaction: agranulocytosis), Contrast dye (anaphylaxis)

FAMILY HISTORY:
- Mother: Autoimmune hypothyroidism (Hashimoto's thyroiditis)
- Sister: Graves' disease
- Paternal grandmother: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Temperature is 38.1°C (100.6°F), blood pressure is 122/74 mmHg, pulse is 102/min, and respiratory rate is 16/min

PHYSICAL EXAMINATION:
Palpation of the anterior neck reveals a mildly enlarged, tender thyroid gland without discrete nodules. There is no cervical lymphadenopathy. Cardiopulmonary and neurologic exams are unremarkable.

LABORATORY DATA:
TSH 0.03 µU/mL
Free T4 2.9 ng/dL
ESR 66 mm/hr

IMAGING:
Radioiodine uptake Decreased

PLAN:
The patient is treated with NSAIDs for symptomatic relief.

## Encounter 5: source 2023-01-15, practice 2024-05-24, ed, Emergency Medicine

Chief complaint: Acute visual changes, severe orbital pain, and eyelid swelling

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Priya Sharma, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute visual changes, severe orbital pain, and eyelid swelling

HISTORY OF PRESENT ILLNESS:
The patient with Graves disease, thyroid-associated orbitopathy, and status post radioiodine ablation presents with acute visual changes, severe orbital pain, and eyelid swelling. She also reports increased anxiety and difficulty sleeping.

PAST MEDICAL HISTORY:
Active Problem List:
- Graves disease (diagnosed 2020-01-15)
- Agranulocytosis (diagnosed 2021-01-15)
- Thyroid Storm (diagnosed 2021-05-15)
- Subacute thyroiditis (diagnosed 2021-07-15)
- Thyroid-associated orbitopathy
- History of antithyroid drug-induced agranulocytosis
- Status post radioiodine ablation

Her past medical history is significant for Graves' disease diagnosed 6 months ago, for which she received radioiodine therapy 8 weeks prior.

PAST SURGICAL HISTORY:
- Radioiodine thyroid ablation (performed 8 weeks prior to index encounter)

MEDICATIONS:
Home Medications:
- Propranolol 20 mg oral twice daily
- Prednisone 40 mg oral daily (tapering regimen for ophthalmopathy prophylaxis)
- Acetaminophen 500 mg oral every 6 hours as needed for fever or pain

ALLERGIES:
Methimazole (severe adverse reaction: agranulocytosis), Contrast dye (anaphylaxis)

FAMILY HISTORY:
- Mother: Autoimmune hypothyroidism (Hashimoto's thyroiditis)
- Sister: Graves' disease
- Paternal grandmother: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, her heart rate is 102 beats per minute, and she has a fine tremor of her outstretched hands. Her thyroid gland is non-tender and has decreased in size compared to previous examinations. She also has mild proptosis and periorbital edema.

LABORATORY DATA:
Laboratory tests reveal the following:

- Thyroid-stimulating hormone (TSH): <0.01 mIU/L (normal range: 0.4-4.0 mIU/L)
- Free thyroxine (FT4): 3.2 ng/dL (normal range: 0.9-1.7 ng/dL)
- Total triiodothyronine (T3): 220 ng/dL (normal range: 80-180 ng/dL)
- Thyroid-stimulating immunoglobulins (TSI): Elevated

## Encounter 6: source 2025-01-15, practice 2026-06-09, ed, Cardiology

Chief complaint: Acute onset palpitations and fatigue lasting 3 hours

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. David Kim, MD
Department: Cardiology
Visit Type: ed

CHIEF COMPLAINT:
Acute onset palpitations and fatigue lasting 3 hours

HISTORY OF PRESENT ILLNESS:
A 42-year-old woman with Graves disease, orbitopathy, and prior radioiodine ablation presents to the emergency department with palpitations and fatigue that started 3 hours ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Graves disease (diagnosed 2020-01-15)
- Agranulocytosis (diagnosed 2021-01-15)
- Thyroid Storm (diagnosed 2021-05-15)
- Subacute thyroiditis (diagnosed 2021-07-15)
- Orbital compartment syndrome (diagnosed 2023-01-15)
- Thyroid-associated orbitopathy
- History of antithyroid drug-induced agranulocytosis
- Status post radioiodine ablation

PAST SURGICAL HISTORY:
- Radioiodine thyroid ablation (performed 8 weeks prior to index encounter)

MEDICATIONS:
Home Medications:
- Propranolol 20 mg oral twice daily
- Prednisone 40 mg oral daily (tapering regimen for ophthalmopathy prophylaxis)
- Acetaminophen 500 mg oral every 6 hours as needed for fever or pain

ALLERGIES:
Methimazole (severe adverse reaction: agranulocytosis), Contrast dye (anaphylaxis)

FAMILY HISTORY:
Her family history is non-contributory.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

REVIEW OF SYSTEMS:
She denies chest pain, shortness of breath, or syncope.

VITAL SIGNS:
On examination, her blood pressure is 132/86 mmHg, heart rate is 160 beats per minute and irregular, respiratory rate is 18 breaths per minute, and oxygen saturation is 98% on room air.

PHYSICAL EXAMINATION:
Her thyroid gland is not enlarged, and there is no tremor.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin: 14.2 g/dL
- White blood cell count: 7.2 x 10^9/L
- Platelet count: 250 x 10^9/L
- Sodium: 140 mmol/L
- Potassium: 4.2 mmol/L
- Creatinine: 0.9 mg/dL
- Thyroid-stimulating hormone (TSH): 0.01 µIU/mL (normal range: 0.4-4.0 µIU/mL)
- Free T4: 3.2 ng/dL (normal range: 0.8-1.8 ng/dL)

OTHER STUDIES:
An electrocardiogram (ECG) shows an irregularly irregular rhythm, absent P waves, and varying R-R intervals.
