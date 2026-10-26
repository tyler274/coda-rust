# synthetic_hospital patient 2823

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 66 at first encounter, sex F, Non-Hispanic White, Medicare with supplemental private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-02-17 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2020-05-15 (Fri) | outpatient | Internal Medicine |
| 2 | 2021-01-15 | 2021-02-18 (Thu) | outpatient | Endocrinology |
| 3 | 2022-01-15 | 2022-02-14 (Mon) | outpatient | Endocrinology |
| 4 | 2023-01-15 | 2023-02-15 (Wed) | outpatient | Obstetrics & Gynecology |
| 5 | 2024-01-15 | 2024-02-26 (Mon) | follow_up | Internal Medicine |
| 6 | 2025-01-15 | 2025-02-24 (Mon) | ed | Emergency Medicine |
| 7 | 2026-01-15 | 2026-02-25 (Wed) | icu | Critical Care Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare with supplemental private insurance",
 "occupation": "Retired administrative assistant",
 "alcohol_use": "Occasional (wine, 1-2 glasses per week)",
 "family_history": [
  "Mother: Type 2 diabetes mellitus, osteoporosis",
  "Father: Hypertension, chronic obstructive pulmonary disease",
  "Maternal aunt: Hip fracture (osteoporosis)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Glyburide"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "12.5 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Omeprazole"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Prednisone"
  },
  {
   "dose": "15 mg PO weekly",
   "name": "Methotrexate"
  },
  {
   "dose": "250/50 mcg inhaled BID",
   "name": "Fluticasone-Salmeterol"
  },
  {
   "dose": "90 mcg inhaled PRN",
   "name": "Albuterol"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "70 mg PO weekly",
   "name": "Alendronate"
  },
  {
   "dose": "600 mg PO TID",
   "name": "Calcium Carbonate"
  },
  {
   "dose": "2000 units PO daily",
   "name": "Cholecalciferol"
  }
 ],
 "surgical_history": [
  "Total abdominal hysterectomy (age 52, for uterine fibroids)",
  "Open reduction and internal fixation of left distal radius fracture (low-trauma)"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus (poorly controlled)",
  "Hypertension",
  "Osteoporosis",
  "Rheumatoid Arthritis",
  "Chronic Obstructive Pulmonary Disease (severe, GOLD Stage D)",
  "Chronic Kidney Disease (Stage 3b, progressive)",
  "Gastroesophageal Reflux Disease"
 ],
 "age_at_first_encounter": 66
}
```

Primary diagnoses: ["Vulvar Squamous Cell Carcinoma", "Postmenopausal osteoporosis with fragility fracture", "Diffuse cortical necrosis", "Herpes Zoster (Shingles), T8 Dermatome", "Type 2 diabetes mellitus", "Osteoporosis", "Exogenous glucocorticoid use", "Hyperparathyroidism"]
Comorbidities: ["Postmenopausal status", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2020-02-17, outpatient, Internal Medicine

Chief complaint: Routine health maintenance examination for management of poorly controlled Type 2 diabetes mellitus and medication reconciliation.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health maintenance examination for management of poorly controlled Type 2 diabetes mellitus and medication reconciliation.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus (poorly controlled)
- Hypertension
- Osteoporosis
- Rheumatoid Arthritis
- Chronic Obstructive Pulmonary Disease (severe, GOLD Stage D)
- Chronic Kidney Disease (Stage 3b, progressive)
- Gastroesophageal Reflux Disease

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy (age 52, for uterine fibroids)
- Open reduction and internal fixation of left distal radius fracture (low-trauma)

MEDICATIONS:
She takes lisinopril, metformin, and glyburide.

Home Medications:
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Omeprazole 20 mg PO daily
- Prednisone 10 mg PO daily
- Methotrexate 15 mg PO weekly
- Fluticasone-Salmeterol 250/50 mcg inhaled BID
- Albuterol 90 mcg inhaled PRN
- Aspirin 81 mg PO daily
- Alendronate 70 mg PO weekly
- Calcium Carbonate 600 mg PO TID
- Cholecalciferol 2000 units PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus, osteoporosis
- Father: Hypertension, chronic obstructive pulmonary disease
- Maternal aunt: Hip fracture (osteoporosis)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (wine, 1-2 glasses per week). Occupation: Retired administrative assistant.

PLAN:
The physician considers an additional agent that promotes endogenous insulin release via enzyme inhibition.

## Encounter 1: source 2020-04-15, practice 2020-05-15, outpatient, Internal Medicine

Chief complaint: Follow-up visit to discuss recent bone density testing results and evaluation of progressive height loss over several years.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit to discuss recent bone density testing results and evaluation of progressive height loss over several years.

HISTORY OF PRESENT ILLNESS:
She returns for bone density follow-up after her January diabetes visit. She denies fracture history but reports losing 5 cm (2 in) of height over the past several years.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus (poorly controlled)
- Hypertension
- Osteoporosis
- Rheumatoid Arthritis
- Chronic Obstructive Pulmonary Disease (severe, GOLD Stage D)
- Chronic Kidney Disease (Stage 3b, progressive)
- Gastroesophageal Reflux Disease

Medical history is significant for hypertension and gastroesophageal reflux disease,.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy (age 52, for uterine fibroids)
- Open reduction and internal fixation of left distal radius fracture (low-trauma)

MEDICATIONS:
She takes for which she takes lisinopril and omeprazole.

Home Medications:
- Metformin 1000 mg PO BID
- Glyburide 5 mg PO daily
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Prednisone 10 mg PO daily
- Methotrexate 15 mg PO weekly
- Fluticasone-Salmeterol 250/50 mcg inhaled BID
- Albuterol 90 mcg inhaled PRN
- Aspirin 81 mg PO daily
- Alendronate 70 mg PO weekly
- Calcium Carbonate 600 mg PO TID
- Cholecalciferol 2000 units PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus, osteoporosis
- Father: Hypertension, chronic obstructive pulmonary disease
- Maternal aunt: Hip fracture (osteoporosis)

SOCIAL HISTORY:
She is a nonsmoker and drinks wine occasionally.

LABORATORY DATA:
Laboratory studies show serum calcium, phosphate, parathyroid hormone, and alkaline phosphatase within normal limits.

IMAGING:
Dual-energy x-ray absorptiometry (DEXA) of the lumbar spine and hip demonstrates a T-score of –2.8.

PLAN:
The patient is started on a medication that is taken orally with water on an empty stomach in the morning and requires the patient to remain upright for at least 30 minutes after ingestion.

## Encounter 2: source 2021-01-15, practice 2021-02-18, outpatient, Endocrinology

Chief complaint: Evaluation of ongoing fatigue, generalized weakness, and persistent bone pain refractory to current therapy.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Emily Johnson, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of ongoing fatigue, generalized weakness, and persistent bone pain refractory to current therapy.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Osteoporosis (diagnosed 2020-04-15)
- Type 2 Diabetes Mellitus (poorly controlled)
- Hypertension
- Rheumatoid Arthritis
- Chronic Obstructive Pulmonary Disease (severe, GOLD Stage D)
- Chronic Kidney Disease (Stage 3b, progressive)
- Gastroesophageal Reflux Disease

She has a history of long-standing poorly controlled type 2 diabetes mellitus and hypertension. Laboratory data over the past decade have shown a progressive decline in renal function, and she was recently initiated on hemodialysis.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy (age 52, for uterine fibroids)
- Open reduction and internal fixation of left distal radius fracture (low-trauma)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Glyburide 5 mg PO daily
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Omeprazole 20 mg PO daily
- Prednisone 10 mg PO daily
- Methotrexate 15 mg PO weekly
- Fluticasone-Salmeterol 250/50 mcg inhaled BID
- Albuterol 90 mcg inhaled PRN
- Aspirin 81 mg PO daily
- Alendronate 70 mg PO weekly
- Calcium Carbonate 600 mg PO TID
- Cholecalciferol 2000 units PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus, osteoporosis
- Father: Hypertension, chronic obstructive pulmonary disease
- Maternal aunt: Hip fracture (osteoporosis)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (wine, 1-2 glasses per week). Occupation: Retired administrative assistant.

PHYSICAL EXAMINATION:
On exam, she has normocytic anemia and mild lower-extremity edema.

LABORATORY DATA:
Her current serum creatinine is 5.8 mg/dL.

IMAGING:
Reduced bone mineral density on DEXA

ASSESSMENT AND PLAN:
Her nephrologist notes that her laboratory studies reflect tertiary hyperparathyroidism.

## Encounter 3: source 2022-01-15, practice 2022-02-14, outpatient, Endocrinology

Chief complaint: Progressive fatigue, worsening proximal muscle weakness with difficulty climbing stairs, weight gain, and abdominal striae over the past year.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. David Kim, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue, worsening proximal muscle weakness with difficulty climbing stairs, weight gain, and abdominal striae over the past year.

HISTORY OF PRESENT ILLNESS:
She returns to Endocrinology one year after prior evaluation, with ongoing osteoporosis and rheumatoid arthritis. She reports difficulty climbing stairs, rising from chairs, and combing her hair due to generalized weakness. Her skin has become fragile with easy bruising, and she has noticed reddish-purple stretch marks on her abdomen.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Osteoporosis (diagnosed 2020-04-15)
- Hyperparathyroidism (diagnosed 2021-01-15)
- Type 2 Diabetes Mellitus (poorly controlled)
- Hypertension
- Rheumatoid Arthritis
- Chronic Obstructive Pulmonary Disease (severe, GOLD Stage D)
- Chronic Kidney Disease (Stage 3b, progressive)
- Gastroesophageal Reflux Disease

Her medical history includes rheumatoid arthritis, hypertension, and chronic obstructive pulmonary disease (COPD). She has been hospitalized twice this year for COPD exacerbations requiring intubation.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy (age 52, for uterine fibroids)
- Open reduction and internal fixation of left distal radius fracture (low-trauma)

MEDICATIONS:
Her medications include metformin, amlodipine, inhaled fluticasone-salmeterol, albuterol, methotrexate, and daily oral prednisone for her rheumatoid arthritis and severe COPD.

Home Medications:
- Glyburide 5 mg PO daily
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Omeprazole 20 mg PO daily
- Aspirin 81 mg PO daily
- Alendronate 70 mg PO weekly
- Calcium Carbonate 600 mg PO TID
- Cholecalciferol 2000 units PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus, osteoporosis
- Father: Hypertension, chronic obstructive pulmonary disease
- Maternal aunt: Hip fracture (osteoporosis)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (wine, 1-2 glasses per week). Occupation: Retired administrative assistant.

REVIEW OF SYSTEMS:
She denies changes in appetite or sleep and has not experienced fevers, chest pain, or palpitations.

VITAL SIGNS:
Examination shows blood pressure 164/92 mmHg and pulse 86/min. BMI is 31 kg/m².

PHYSICAL EXAMINATION:
She has rounded facies with supraclavicular fullness, dorsocervical fat pad prominence, thin extremities with proximal muscle wasting, and wide violaceous striae on her abdomen.

## Encounter 4: source 2023-01-15, practice 2023-02-15, outpatient, Obstetrics & Gynecology

Chief complaint: Persistent vulvar irritation, pruritus, and discomfort when sitting or walking for several months, with progressive enlargement of a discolored lesion.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Maria Gonzalez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent vulvar irritation, pruritus, and discomfort when sitting or walking for several months, with progressive enlargement of a discolored lesion.

HISTORY OF PRESENT ILLNESS:
The patient presents with progressive vulvar symptoms amid ongoing management of rheumatoid arthritis on chronic immunosuppression and recent evaluation for glucocorticoid-related complications. She reports progressive discomfort, especially while sitting or walking, and describes the area as "itchy and sensitive." She has also noticed a small area of discoloration that has gradually enlarged, though she delayed seeking care due to embarrassment. The patient denies vaginal discharge, bleeding, or recent sexual activity.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Osteoporosis (diagnosed 2020-04-15)
- Hyperparathyroidism (diagnosed 2021-01-15)
- Exogenous glucocorticoid use (diagnosed 2022-01-15)
- Type 2 Diabetes Mellitus (poorly controlled)
- Hypertension
- Rheumatoid Arthritis
- Chronic Obstructive Pulmonary Disease (severe, GOLD Stage D)
- Chronic Kidney Disease (Stage 3b, progressive)
- Gastroesophageal Reflux Disease

Her last Pap smear was over a decade ago. Medical history includes type 2 diabetes and well-controlled hypertension.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy (age 52, for uterine fibroids)
- Open reduction and internal fixation of left distal radius fracture (low-trauma)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Glyburide 5 mg PO daily
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Omeprazole 20 mg PO daily
- Prednisone 10 mg PO daily
- Methotrexate 15 mg PO weekly
- Fluticasone-Salmeterol 250/50 mcg inhaled BID
- Albuterol 90 mcg inhaled PRN
- Aspirin 81 mg PO daily
- Alendronate 70 mg PO weekly
- Calcium Carbonate 600 mg PO TID
- Cholecalciferol 2000 units PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus, osteoporosis
- Father: Hypertension, chronic obstructive pulmonary disease
- Maternal aunt: Hip fracture (osteoporosis)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (wine, 1-2 glasses per week). Occupation: Retired administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, there is a 1.5 cm pale, thickened plaque with slightly irregular borders located on the right labia majora. No inguinal lymphadenopathy is palpated.

## Encounter 5: source 2024-01-15, practice 2024-02-26, follow_up, Internal Medicine

Chief complaint: Routine follow-up appointment for management of osteoporosis after sustaining a recent fragility fracture.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: follow_up

CHIEF COMPLAINT:
Routine follow-up appointment for management of osteoporosis after sustaining a recent fragility fracture.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Osteoporosis (diagnosed 2020-04-15)
- Hyperparathyroidism (diagnosed 2021-01-15)
- Exogenous glucocorticoid use (diagnosed 2022-01-15)
- Vulvar Squamous Cell Carcinoma (diagnosed 2023-01-15)
- Type 2 Diabetes Mellitus (poorly controlled)
- Hypertension
- Rheumatoid Arthritis
- Chronic Obstructive Pulmonary Disease (severe, GOLD Stage D)
- Chronic Kidney Disease (Stage 3b, progressive)
- Gastroesophageal Reflux Disease

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy (age 52, for uterine fibroids)
- Open reduction and internal fixation of left distal radius fracture (low-trauma)

MEDICATIONS:
She takes metformin, lisinopril, and aspirin.

Home Medications:
- Glyburide 5 mg PO daily
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Omeprazole 20 mg PO daily
- Prednisone 10 mg PO daily
- Methotrexate 15 mg PO weekly
- Fluticasone-Salmeterol 250/50 mcg inhaled BID
- Albuterol 90 mcg inhaled PRN
- Alendronate 70 mg PO weekly
- Calcium Carbonate 600 mg PO TID
- Cholecalciferol 2000 units PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
No family history of fractures or malignancy

SOCIAL HISTORY:
She does not smoke or consume alcohol

VITAL SIGNS:
Blood pressure is 138/84 mm Hg, and BMI is 24 kg/m²

PHYSICAL EXAMINATION:
Physical examination is unremarkable

IMAGING:
A DEXA scan shows a T-score of −2.8 in the lumbar spine

PLAN:
In addition to calcium and vitamin D supplementation,

## Encounter 6: source 2025-01-15, practice 2025-02-24, ed, Emergency Medicine

Chief complaint: Acute onset of painful, fluid-filled vesicular rash localized to the left flank in a dermatomal distribution, present for three days.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of painful, fluid-filled vesicular rash localized to the left flank in a dermatomal distribution, present for three days.

HISTORY OF PRESENT ILLNESS:
This 71-year-old female with rheumatoid arthritis on chronic immunosuppressive therapy and history of vulvar squamous cell carcinoma presents with acute dermatologic symptoms. She reports that the rash began as a cluster of red bumps and has progressed to fluid-filled blisters that are localized to a single stripe on her skin. She denies any recent illnesses but mentions that she has been under significant stress caring for her ailing husband.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Osteoporosis (diagnosed 2020-04-15)
- Hyperparathyroidism (diagnosed 2021-01-15)
- Exogenous glucocorticoid use (diagnosed 2022-01-15)
- Vulvar Squamous Cell Carcinoma (diagnosed 2023-01-15)
- Postmenopausal osteoporosis with fragility fracture (diagnosed 2024-01-15)
- Type 2 Diabetes Mellitus (poorly controlled)
- Hypertension
- Rheumatoid Arthritis
- Chronic Obstructive Pulmonary Disease (severe, GOLD Stage D)
- Chronic Kidney Disease (Stage 3b, progressive)
- Gastroesophageal Reflux Disease

Her past medical history includes hypertension, for which she takes hydrochlorothiazide, and type 2 diabetes mellitus controlled with metformin.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy (age 52, for uterine fibroids)
- Open reduction and internal fixation of left distal radius fracture (low-trauma)

MEDICATIONS:
She takes hydrochlorothiazide, metformin.

Home Medications:
- Glyburide 5 mg PO daily
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Omeprazole 20 mg PO daily
- Prednisone 10 mg PO daily
- Methotrexate 15 mg PO weekly
- Fluticasone-Salmeterol 250/50 mcg inhaled BID
- Albuterol 90 mcg inhaled PRN
- Aspirin 81 mg PO daily
- Alendronate 70 mg PO weekly
- Calcium Carbonate 600 mg PO TID
- Cholecalciferol 2000 units PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus, osteoporosis
- Father: Hypertension, chronic obstructive pulmonary disease
- Maternal aunt: Hip fracture (osteoporosis)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (wine, 1-2 glasses per week). Occupation: Retired administrative assistant.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, you note a band of erythematous papules and vesicles on an erythematous base along the T8 dermatome on her left side. There is no evidence of rash crossing the midline. She has no other skin abnormalities.

LABORATORY DATA:
Laboratory values:
- HIV antibody test: Negative
- Fasting blood glucose: 148 mg/dL
- Complete blood count: Normal
- ESR: 20 mm/hr (normal <20)

## Encounter 7: source 2026-01-15, practice 2026-02-25, icu, Critical Care Medicine

Chief complaint: Altered mental status and severe hypotension following two days of high fever and chills.

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. Robert Thompson, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Altered mental status and severe hypotension following two days of high fever and chills.

HISTORY OF PRESENT ILLNESS:
This 72-year-old female with severe COPD, progressive CKD, and chronic immunosuppression presents with altered mental status and hypotension. Her symptoms began shortly after an episode of fever and chills that persisted for two days.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Osteoporosis (diagnosed 2020-04-15)
- Hyperparathyroidism (diagnosed 2021-01-15)
- Exogenous glucocorticoid use (diagnosed 2022-01-15)
- Vulvar Squamous Cell Carcinoma (diagnosed 2023-01-15)
- Postmenopausal osteoporosis with fragility fracture (diagnosed 2024-01-15)
- Herpes Zoster (Shingles), T8 Dermatome (diagnosed 2025-01-15)
- Type 2 Diabetes Mellitus (poorly controlled)
- Hypertension
- Rheumatoid Arthritis
- Chronic Obstructive Pulmonary Disease (severe, GOLD Stage D)
- Chronic Kidney Disease (Stage 3b, progressive)
- Gastroesophageal Reflux Disease

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy (age 52, for uterine fibroids)
- Open reduction and internal fixation of left distal radius fracture (low-trauma)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Glyburide 5 mg PO daily
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Omeprazole 20 mg PO daily
- Prednisone 10 mg PO daily
- Methotrexate 15 mg PO weekly
- Fluticasone-Salmeterol 250/50 mcg inhaled BID
- Albuterol 90 mcg inhaled PRN
- Aspirin 81 mg PO daily
- Alendronate 70 mg PO weekly
- Calcium Carbonate 600 mg PO TID
- Cholecalciferol 2000 units PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus, osteoporosis
- Father: Hypertension, chronic obstructive pulmonary disease
- Maternal aunt: Hip fracture (osteoporosis)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (wine, 1-2 glasses per week). Occupation: Retired administrative assistant.

VITAL SIGNS:
On arrival, she is febrile, hypotensive, and tachycardic.

PHYSICAL EXAMINATION:
Physical examination reveals cool extremities, delayed capillary refill, and dry mucous membranes.

LABORATORY DATA:
Laboratory tests show elevated serum creatinine and blood urea nitrogen, along with high lactate levels and leukocytosis.

PATHOLOGY:
Histologic examination reveals widespread necrosis involving parts of the renal cortex.

OTHER STUDIES:
A renal biopsy is performed to evaluate the cause of renal failure

ASSESSMENT AND PLAN:
Despite aggressive management, her urine output remains low, and repeat labs show worsening renal function. Over the following days, she becomes anuric,

She is admitted to the intensive care unit and started on broad-spectrum antibiotics and intravenous fluids.
