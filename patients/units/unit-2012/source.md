# synthetic_hospital patient 2012

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 57 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-06-27 (Mon) | outpatient | Hematology-Oncology |
| 1 | 2020-04-15 | 2022-09-12 (Mon) | outpatient | Hematology |
| 2 | 2020-07-15 | 2022-12-20 (Tue) | outpatient | Gastroenterology |
| 3 | 2021-01-15 | 2023-06-30 (Fri) | outpatient | Gastroenterology |
| 4 | 2023-01-15 | 2025-06-24 (Tue) | inpatient | Internal Medicine |
| 5 | 2023-03-15 | 2025-08-15 (Fri) | ed | Emergency Medicine |
| 6 | 2023-07-15 | 2025-12-12 (Fri) | ed | Emergency Medicine |
| 7 | 2024-01-15 | 2026-06-19 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Former warehouse forklift operator, currently unemployed on disability",
 "alcohol_use": "Heavy daily consumption, 4-6 beers daily, consistent with severe alcohol use disorder",
 "family_history": [
  "Father: Alcohol use disorder with cirrhosis, deceased age 64",
  "Mother: Hypertension, chronic kidney disease",
  "Brother: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Active smoker, 30 pack-year history",
 "home_medications": [
  {
   "dose": "300 mg PO daily",
   "name": "Phenytoin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Furosemide"
  },
  {
   "dose": "100 mg PO daily",
   "name": "Spironolactone"
  },
  {
   "dose": "100 mg PO daily",
   "name": "Pyridoxine"
  },
  {
   "dose": "100 mg PO daily",
   "name": "Thiamine"
  }
 ],
 "surgical_history": [
  "Excisional biopsy of right cervical lymph node",
  "Diagnostic paracentesis",
  "Liver biopsy"
 ],
 "chronic_conditions": [
  "Epilepsy",
  "Hypertension",
  "Chronic kidney disease stage 3",
  "Dyslipidemia",
  "Chronic hepatitis C infection",
  "Alcohol use disorder",
  "Cirrhosis",
  "Chronic pancreatitis",
  "Sideroblastic anemia",
  "Non-Hodgkin lymphoma"
 ],
 "age_at_first_encounter": 57
}
```

Primary diagnoses: ["Sideroblastic anemia", "Malignant neoplasm of lymph nodes", "Acute Gouty Arthritis (Podagra)", "Alcoholic cirrhosis", "Hemobilia", "Hepatic encephalopathy", "Ascites due to cirrhosis", "Sinus tachycardia secondary to burns"]
Comorbidities: ["Portal hypertension", "Essential hypertension", "Alcohol use disorder"]

## Encounter 0: source 2020-01-15, practice 2022-06-27, outpatient, Hematology-Oncology

Chief complaint: Painless lump on the right side of his neck for 2 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Hematology-Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Painless lump on the right side of his neck for 2 months

HISTORY OF PRESENT ILLNESS:
He denies any recent infections, fever, night sweats, or weight loss. The patient has no hoarseness, dysphagia, or otalgia.

PAST MEDICAL HISTORY:
Active Problem List:
- Epilepsy
- Hypertension
- Chronic kidney disease stage 3
- Dyslipidemia
- Chronic hepatitis C infection
- Alcohol use disorder
- Cirrhosis
- Chronic pancreatitis
- Sideroblastic anemia
- Non-Hodgkin lymphoma

His medical history is significant for hypertension, controlled with lisinopril.

PAST SURGICAL HISTORY:
- Excisional biopsy of right cervical lymph node
- Diagnostic paracentesis
- Liver biopsy

MEDICATIONS:
Home Medications:
- Phenytoin 300 mg PO daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Furosemide 40 mg PO daily
- Spironolactone 100 mg PO daily
- Pyridoxine 100 mg PO daily
- Thiamine 100 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis, deceased age 64
- Mother: Hypertension, chronic kidney disease
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
He has a 30-pack-year smoking history and admits to heavy alcohol use.

PHYSICAL EXAMINATION:
On examination, you note a solitary, firm, non-tender lymph node measuring approximately 2 cm in diameter located in the right cervical chain. There are no other palpable lymph nodes. The oropharyngeal examination is unremarkable.

LABORATORY DATA:
Laboratory tests reveal:
- Complete Blood Count: Within normal limits
- Erythrocyte Sedimentation Rate: 20 mm/hr (normal range: 0-22 mm/hr)
- HIV test: Negative

## Encounter 1: source 2020-04-15, practice 2022-09-12, outpatient, Hematology

Chief complaint: Progressive fatigue and reduced exercise tolerance for several months

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Hematology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue and reduced exercise tolerance for several months

HISTORY OF PRESENT ILLNESS:
The patient with chronic hepatitis C returns following neck mass evaluation. He reports needing to rest after walking one to two blocks and occasionally feels lightheaded. He denies chest pain, melena, or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Malignant neoplasm of lymph nodes (diagnosed 2020-01-15)
- Epilepsy
- Hypertension
- Chronic kidney disease stage 3
- Dyslipidemia
- Chronic hepatitis C infection
- Alcohol use disorder
- Cirrhosis
- Chronic pancreatitis
- Sideroblastic anemia
- Non-Hodgkin lymphoma

PAST SURGICAL HISTORY:
- Excisional biopsy of right cervical lymph node
- Diagnostic paracentesis
- Liver biopsy

MEDICATIONS:
He takes phenytoin and lisinopril.

Home Medications:
- Atorvastatin 40 mg PO daily
- Furosemide 40 mg PO daily
- Spironolactone 100 mg PO daily
- Pyridoxine 100 mg PO daily
- Thiamine 100 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis, deceased age 64
- Mother: Hypertension, chronic kidney disease
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
He lives alone and eats mostly processed and frozen foods. He drinks 4–6 beers daily, stating it helps him "relax and sleep."

PHYSICAL EXAMINATION:
Physical exam shows pale conjunctiva and mild weakness on strength testing in the lower extremities. His vibratory sense is reduced in the feet and ankle reflexes are diminished bilaterally.

LABORATORY DATA:
Labs reveal a normocytic anemia with elevated serum iron, elevated ferritin, decreased total iron-binding capacity, and a low reticulocyte count. A peripheral smear is shown.

## Encounter 2: source 2020-07-15, practice 2022-12-20, outpatient, Gastroenterology

Chief complaint: Increasing abdominal fullness and swelling in legs over past few months

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing abdominal fullness and swelling in legs over past few months

HISTORY OF PRESENT ILLNESS:
This 57-year-old male with a history of Non-Hodgkin lymphoma and sideroblastic anemia, evaluated by Hematology-Oncology in January and Hematology in April 2020, presents to the Gastroenterology clinic complaining of increasing abdominal fullness and swelling in his legs over the past few months. He says his shoes feel tight by the evening, and he has gained several pounds despite eating less. He also reports early satiety and occasional mild discomfort in the right upper abdomen. He denies chest pain or shortness of breath.

PAST MEDICAL HISTORY:
Active Problem List:
- Malignant neoplasm of lymph nodes (diagnosed 2020-01-15)
- Sideroblastic anemia (diagnosed 2020-04-15)
- Epilepsy
- Hypertension
- Chronic kidney disease stage 3
- Dyslipidemia
- Chronic hepatitis C infection
- Alcohol use disorder
- Cirrhosis
- Chronic pancreatitis
- Non-Hodgkin lymphoma

His past medical history is notable for long-standing alcohol use, but he has not seen a physician in several years.

PAST SURGICAL HISTORY:
- Excisional biopsy of right cervical lymph node
- Diagnostic paracentesis
- Liver biopsy

MEDICATIONS:
Home Medications:
- Phenytoin 300 mg PO daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Furosemide 40 mg PO daily
- Spironolactone 100 mg PO daily
- Pyridoxine 100 mg PO daily
- Thiamine 100 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis, deceased age 64
- Mother: Hypertension, chronic kidney disease
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Active smoker, 30 pack-year history. Alcohol: Heavy daily consumption, 4-6 beers daily, consistent with severe alcohol use disorder. Occupation: Former warehouse forklift operator, currently unemployed on disability.

VITAL SIGNS:
On examination, his blood pressure is 110/68 mm Hg, pulse is 84/min, and respiratory rate is 16/min.

PHYSICAL EXAMINATION:
Scleral icterus is noted. Cardiopulmonary exam is normal. The abdomen is distended with a fluid wave, and the liver edge is not palpable. There are scattered vascular markings on the anterior chest. Both lower extremities show pitting edema to the mid-shin.

## Encounter 3: source 2021-01-15, practice 2023-06-30, outpatient, Gastroenterology

Chief complaint: Progressive abdominal distension and discomfort over past two months

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive abdominal distension and discomfort over past two months

HISTORY OF PRESENT ILLNESS:
This 58-year-old male with cirrhosis, lymphoma, and anemia returns to the clinic with progressive abdominal distension and discomfort over the past two months. He reports significant weight gain despite a poor appetite.

PAST MEDICAL HISTORY:
Active Problem List:
- Malignant neoplasm of lymph nodes (diagnosed 2020-01-15)
- Sideroblastic anemia (diagnosed 2020-04-15)
- Alcoholic cirrhosis (diagnosed 2020-07-15)
- Epilepsy
- Hypertension
- Chronic kidney disease stage 3
- Dyslipidemia
- Chronic hepatitis C infection
- Alcohol use disorder
- Cirrhosis
- Chronic pancreatitis
- Non-Hodgkin lymphoma

His past medical history is notable for chronic hepatitis C infection.

PAST SURGICAL HISTORY:
- Excisional biopsy of right cervical lymph node
- Diagnostic paracentesis
- Liver biopsy

MEDICATIONS:
Home Medications:
- Phenytoin 300 mg PO daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Furosemide 40 mg PO daily
- Spironolactone 100 mg PO daily
- Pyridoxine 100 mg PO daily
- Thiamine 100 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis, deceased age 64
- Mother: Hypertension, chronic kidney disease
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Active smoker, 30 pack-year history. Alcohol: Heavy daily consumption, 4-6 beers daily, consistent with severe alcohol use disorder. Occupation: Former warehouse forklift operator, currently unemployed on disability.

PHYSICAL EXAMINATION:
On physical examination, the patient appears jaundiced with spider angiomas on the chest. Abdominal examination reveals a fluid wave and shifting dullness. Lower extremity pitting edema is present.

LABORATORY DATA:
Laboratory studies show:

- Serum albumin: 2.8 g/dL
- Serum total protein: 6.1 g/dL
- Serum creatinine: 1.2 mg/dL
- Serum sodium: 133 mEq/L
- INR: 1.6

IMAGING:
An abdominal ultrasound demonstrates cirrhotic liver morphology and moderate ascites.

OTHER STUDIES:
A diagnostic paracentesis is performed, and the analysis reveals:

- Ascitic fluid albumin: 1.1 g/dL
- Ascitic fluid total protein: 2.5 g/dL
- Ascitic fluid WBC count: 150 cells/mm^3 (80% neutrophils)

## Encounter 4: source 2023-01-15, practice 2025-06-24, inpatient, Internal Medicine

Chief complaint: Severe hypertension not responding to oral therapy

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Aisha Johnson, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Severe hypertension not responding to oral therapy

HISTORY OF PRESENT ILLNESS:
The 60-year-old man with a history of Non-Hodgkin lymphoma, sideroblastic anemia, and cirrhosis complicated by ascites returns for management of severe hypertension that has not responded to oral therapy.

PAST MEDICAL HISTORY:
Active Problem List:
- Malignant neoplasm of lymph nodes (diagnosed 2020-01-15)
- Sideroblastic anemia (diagnosed 2020-04-15)
- Alcoholic cirrhosis (diagnosed 2020-07-15)
- Ascites due to cirrhosis (diagnosed 2021-01-15)
- Epilepsy
- Hypertension
- Chronic kidney disease stage 3
- Dyslipidemia
- Chronic hepatitis C infection
- Alcohol use disorder
- Cirrhosis
- Chronic pancreatitis
- Non-Hodgkin lymphoma

PAST SURGICAL HISTORY:
- Excisional biopsy of right cervical lymph node
- Diagnostic paracentesis
- Liver biopsy

MEDICATIONS:
Home Medications:
- Phenytoin 300 mg PO daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Furosemide 40 mg PO daily
- Spironolactone 100 mg PO daily
- Pyridoxine 100 mg PO daily
- Thiamine 100 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis, deceased age 64
- Mother: Hypertension, chronic kidney disease
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Active smoker, 30 pack-year history. Alcohol: Heavy daily consumption, 4-6 beers daily, consistent with severe alcohol use disorder. Occupation: Former warehouse forklift operator, currently unemployed on disability.

VITAL SIGNS:
On admission, his blood pressure is 198/108 mm Hg. Thirty minutes later, his systolic blood pressure decreases to 138 mm Hg.

PHYSICAL EXAMINATION:
He remains alert and oriented but reports mild lightheadedness.

PLAN:
He is started on intravenous phentolamine for rapid blood pressure control.

## Encounter 5: source 2023-03-15, practice 2025-08-15, ed, Emergency Medicine

Chief complaint: Severe pain and swelling in right great toe for 2 days

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe pain and swelling in right great toe for 2 days

HISTORY OF PRESENT ILLNESS:
The patient is a 60-year-old man with alcoholic cirrhosis and non-Hodgkin lymphoma evaluated two months ago who returns with severe pain and swelling in his right great toe for the past 2 days.

PAST MEDICAL HISTORY:
Active Problem List:
- Malignant neoplasm of lymph nodes (diagnosed 2020-01-15)
- Sideroblastic anemia (diagnosed 2020-04-15)
- Alcoholic cirrhosis (diagnosed 2020-07-15)
- Ascites due to cirrhosis (diagnosed 2021-01-15)
- Sinus tachycardia secondary to burns (diagnosed 2023-01-15)
- Epilepsy
- Hypertension
- Chronic kidney disease stage 3
- Dyslipidemia
- Chronic hepatitis C infection
- Alcohol use disorder
- Cirrhosis
- Chronic pancreatitis
- Non-Hodgkin lymphoma

PAST SURGICAL HISTORY:
- Excisional biopsy of right cervical lymph node
- Diagnostic paracentesis
- Liver biopsy

MEDICATIONS:
His medications include lisinopril and atorvastatin.

Home Medications:
- Phenytoin 300 mg PO daily
- Furosemide 40 mg PO daily
- Spironolactone 100 mg PO daily
- Pyridoxine 100 mg PO daily
- Thiamine 100 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His father had hypertension and died from myocardial infarction at age 65.

SOCIAL HISTORY:
He drinks alcohol daily

VITAL SIGNS:
His temperature is 37.4°C (99.3°F), blood pressure is 142/86 mm Hg, pulse is 82/min, and respirations are 14/min.

PHYSICAL EXAMINATION:
On examination, his right first metatarsophalangeal joint is red, tender, warm, and swollen.

LABORATORY DATA:
Joint aspiration is performed and the synovial fluid is examined under polarized light microscopy, and findings are shown in the image below.

## Encounter 6: source 2023-07-15, practice 2025-12-12, ed, Emergency Medicine

Chief complaint: Increasing confusion over past two days

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Emily Washington, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increasing confusion over past two days

HISTORY OF PRESENT ILLNESS:
The patient has alcoholic cirrhosis with prior ascites. He is brought to the emergency department by his family for increasing confusion over the past two days.

PAST MEDICAL HISTORY:
Active Problem List:
- Malignant neoplasm of lymph nodes (diagnosed 2020-01-15)
- Sideroblastic anemia (diagnosed 2020-04-15)
- Alcoholic cirrhosis (diagnosed 2020-07-15)
- Ascites due to cirrhosis (diagnosed 2021-01-15)
- Sinus tachycardia secondary to burns (diagnosed 2023-01-15)
- Acute Gouty Arthritis (Podagra) (diagnosed 2023-03-15)
- Epilepsy
- Hypertension
- Chronic kidney disease stage 3
- Dyslipidemia
- Chronic hepatitis C infection
- Alcohol use disorder
- Cirrhosis
- Chronic pancreatitis
- Non-Hodgkin lymphoma

PAST SURGICAL HISTORY:
- Excisional biopsy of right cervical lymph node
- Diagnostic paracentesis
- Liver biopsy

MEDICATIONS:
He has been taking furosemide for ascites

Home Medications:
- Phenytoin 300 mg PO daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Spironolactone 100 mg PO daily
- Pyridoxine 100 mg PO daily
- Thiamine 100 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis, deceased age 64
- Mother: Hypertension, chronic kidney disease
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Active smoker, 30 pack-year history. Alcohol: Heavy daily consumption, 4-6 beers daily, consistent with severe alcohol use disorder. Occupation: Former warehouse forklift operator, currently unemployed on disability.

PHYSICAL EXAMINATION:
On examination, the patient is disoriented to time and place. His abdomen is distended with a fluid wave, and asterixis is present. His skin is warm and dry.

LABORATORY DATA:
- Sodium: 132 mEq/L
- Potassium: 3.0 mEq/L
- Chloride: 95 mEq/L
- Bicarbonate: 32 mEq/L
- Blood urea nitrogen: 18 mg/dL
- Serum creatinine: 1.2 mg/dL
- Serum ammonia: 110 µg/dL
- Arterial blood gas: pH 7.50, PaCO2 40 mmHg, PaO2 95 mmHg

## Encounter 7: source 2024-01-15, practice 2026-06-19, ed, Emergency Medicine

Chief complaint: Worsening right upper quadrant pain, jaundice, and melena for 2 days

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. David Singh, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening right upper quadrant pain, jaundice, and melena for 2 days

HISTORY OF PRESENT ILLNESS:
Presenting with RUQ pain and melena, the patient with cirrhosis reports significant weight loss over past few months but denies fever, chills, or recent travel.

PAST MEDICAL HISTORY:
Active Problem List:
- Malignant neoplasm of lymph nodes (diagnosed 2020-01-15)
- Sideroblastic anemia (diagnosed 2020-04-15)
- Alcoholic cirrhosis (diagnosed 2020-07-15)
- Ascites due to cirrhosis (diagnosed 2021-01-15)
- Sinus tachycardia secondary to burns (diagnosed 2023-01-15)
- Acute Gouty Arthritis (Podagra) (diagnosed 2023-03-15)
- Hepatic encephalopathy (diagnosed 2023-07-15)
- Epilepsy
- Hypertension
- Chronic kidney disease stage 3
- Dyslipidemia
- Chronic hepatitis C infection
- Alcohol use disorder
- Cirrhosis
- Chronic pancreatitis
- Non-Hodgkin lymphoma

His past medical history is notable for chronic pancreatitis.

PAST SURGICAL HISTORY:
He underwent an endoscopic retrograde cholangiopancreatography (ERCP) 1 week ago due to suspected choledocholithiasis.

MEDICATIONS:
Home Medications:
- Phenytoin 300 mg PO daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Furosemide 40 mg PO daily
- Spironolactone 100 mg PO daily
- Pyridoxine 100 mg PO daily
- Thiamine 100 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis, deceased age 64
- Mother: Hypertension, chronic kidney disease
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
With a history of chronic alcohol use

VITAL SIGNS:
On examination, his temperature is 37.2°C (99°F), blood pressure is 130/85 mmHg, pulse is 92/min, and respiratory rate is 18/min.

PHYSICAL EXAMINATION:
Abdominal examination reveals tenderness in the right upper quadrant without rebound or guarding.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 9.8 g/dL (normal: 13.8-17.2 g/dL)
- Total bilirubin: 3.2 mg/dL (normal: 0.3-1.2 mg/dL)
- Direct bilirubin: 2.1 mg/dL (normal: 0.0-0.3 mg/dL)
- Alkaline phosphatase: 210 U/L (normal: 40-130 U/L)
- Aspartate aminotransferase (AST): 68 U/L (normal: 10-40 U/L)
- Alanine aminotransferase (ALT): 55 U/L (normal: 7-56 U/L)

OTHER STUDIES:
Upper endoscopy reveals blood oozing from the ampulla of Vater.
