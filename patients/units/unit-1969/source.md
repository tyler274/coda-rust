# synthetic_hospital patient 1969

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 50 at first encounter, sex M, Non-Hispanic White, Commercial insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-03-25 (Fri) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2022-06-20 (Mon) | outpatient | Internal Medicine |
| 2 | 2021-03-15 | 2023-06-05 (Mon) | outpatient | Gastroenterology |
| 3 | 2021-09-15 | 2023-11-30 (Thu) | outpatient | Psychiatry |
| 4 | 2022-03-15 | 2024-06-03 (Mon) | outpatient | Internal Medicine |
| 5 | 2023-01-15 | 2025-03-18 (Tue) | outpatient | Internal Medicine |
| 6 | 2023-07-15 | 2025-10-02 (Thu) | ed | Emergency Medicine |
| 7 | 2024-01-15 | 2026-03-26 (Thu) | outpatient | Rheumatology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial insurance",
 "occupation": "Warehouse supervisor",
 "alcohol_use": "Alcohol use disorder with heavy daily consumption (6-8 beers daily for past 10 years); periods of reduced intake or social-only use reported during some encounters",
 "family_history": [
  "Father with type 2 diabetes mellitus and gout",
  "Mother with essential hypertension",
  "Paternal uncle with alcohol use disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "20 mg daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Type 2 diabetes mellitus",
  "Essential hypertension",
  "Gout",
  "Obesity",
  "Alcohol use disorder",
  "Achalasia",
  "History of nephrolithiasis",
  "Folate deficiency anemia"
 ],
 "age_at_first_encounter": 50
}
```

Primary diagnoses: ["Crystal-induced arthropathy (Gout or Pseudogout)", "Alcohol use disorder", "Essential hypertension", "Folate deficiency anemia", "Ureterolithiasis", "Achalasia", "Obesity"]
Comorbidities: ["Alcohol use disorder", "Metabolic syndrome", "Obesity", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2022-03-25, outpatient, Internal Medicine

Chief complaint: Routine health examination, no complaints

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health examination, no complaints

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus
- Essential hypertension
- Gout
- Obesity
- Alcohol use disorder
- Achalasia
- History of nephrolithiasis
- Folate deficiency anemia

His medical history is significant for type 2 diabetes mellitus, which is well-controlled.

MEDICATIONS:
He takes metformin.

Home Medications:
- Lisinopril 20 mg daily
- Atorvastatin 20 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is notable for hypertension and stroke

SOCIAL HISTORY:
He reports a sedentary lifestyle. He does not smoke and drinks alcohol socially

VITAL SIGNS:
Blood pressure readings are 142/92 mmHg and 138/90 mmHg on two separate occasions

PHYSICAL EXAMINATION:
His BMI is 32 kg/m^2. Physical examination is unremarkable except for central obesity

LABORATORY DATA:
Laboratory values are as follows:

- Fasting glucose: 98 mg/dL
- HbA1c: 6.2%
- Total cholesterol: 190 mg/dL
- HDL cholesterol: 40 mg/dL
- LDL cholesterol: 120 mg/dL
- Triglycerides: 150 mg/dL

## Encounter 1: source 2020-04-15, practice 2022-06-20, outpatient, Internal Medicine

Chief complaint: Routine follow-up for obesity management

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for obesity management

HISTORY OF PRESENT ILLNESS:
The patient is a 50-year-old male with Type 2 diabetes mellitus, essential hypertension, and obesity returning for follow-up three months after his last visit. He reports difficulty adhering to the dietary and exercise recommendations provided during his last visit 3 months ago. He states he finds the goals overwhelming and is discouraged by his lack of progress. He has been trying to "eat healthier" but has not lost any weight.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Type 2 diabetes mellitus
- Gout
- Obesity
- Alcohol use disorder
- Achalasia
- History of nephrolithiasis
- Folate deficiency anemia

MEDICATIONS:
His current medications include metformin and lisinopril.

Home Medications:
- Atorvastatin 20 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with type 2 diabetes mellitus and gout
- Mother with essential hypertension
- Paternal uncle with alcohol use disorder

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Alcohol use disorder with heavy daily consumption (6-8 beers daily for past 10 years); periods of reduced intake or social-only use reported during some encounters. Occupation: Warehouse supervisor.

VITAL SIGNS:
His blood pressure is 142/88 mmHg

PHYSICAL EXAMINATION:
On examination, BMI is 32 kg/m^2, and the rest of the physical examination is unremarkable.

LABORATORY DATA:
Recent laboratory values are as follows:

- Fasting glucose: 158 mg/dL
- HbA1c: 7.8%
- Total cholesterol: 210 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 140 mg/dL
- Triglycerides: 250 mg/dL

## Encounter 2: source 2021-03-15, practice 2023-06-05, outpatient, Gastroenterology

Chief complaint: Progressive difficulty swallowing solids and liquids for 6 months

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Emily Johnson, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty swallowing solids and liquids for 6 months

HISTORY OF PRESENT ILLNESS:
The patient, previously managed for hypertension and obesity, presents with difficulty swallowing. He reports occasional regurgitation of undigested food and a sensation of food sticking in his mid-chest after eating. He denies any significant weight loss, vomiting, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Obesity (diagnosed 2020-04-15)
- Type 2 diabetes mellitus
- Gout
- Alcohol use disorder
- Achalasia
- History of nephrolithiasis
- Folate deficiency anemia

MEDICATIONS:
Metformin, lisinopril, and atorvastatin

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with type 2 diabetes mellitus and gout
- Mother with essential hypertension
- Paternal uncle with alcohol use disorder

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Alcohol use disorder with heavy daily consumption (6-8 beers daily for past 10 years); periods of reduced intake or social-only use reported during some encounters. Occupation: Warehouse supervisor.

VITAL SIGNS:
Blood pressure is 138/86 mmHg, heart rate is 78 bpm, and BMI is 32 kg/m^2

PHYSICAL EXAMINATION:
Cardiopulmonary examination is unremarkable. Abdominal examination reveals no hepatosplenomegaly or masses.

LABORATORY DATA:
Hemoglobin: 14.2 g/dL
- Fasting glucose: 152 mg/dL
- Serum creatinine: 1.0 mg/dL
- Thyroid-stimulating hormone (TSH): 2.5 μIU/mL
- Serum electrolytes: Within normal limits

OTHER STUDIES:
An upper endoscopy shows a dilated esophagus with retained food, but no masses or strictures. Esophageal manometry is performed and reveals aperistalsis and incomplete lower esophageal sphincter relaxation.

## Encounter 3: source 2021-09-15, practice 2023-11-30, outpatient, Psychiatry

Chief complaint: Desire to address alcohol consumption

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. David Park, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Desire to address alcohol consumption

HISTORY OF PRESENT ILLNESS:
The patient presents to Psychiatry regarding alcohol use, returning for care following recent achalasia evaluation and chronic disease management. He reports drinking six beers daily for the past 10 years and has unsuccessfully attempted to quit multiple times. He expresses a strong desire to stop drinking due to its impact on his family life and job performance.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Obesity (diagnosed 2020-04-15)
- Achalasia (diagnosed 2021-03-15)
- Type 2 diabetes mellitus
- Gout
- Alcohol use disorder
- History of nephrolithiasis
- Folate deficiency anemia

His medical history is significant for hypertension, controlled with lisinopril, and type 2 diabetes mellitus, managed with metformin.

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Atorvastatin 20 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with type 2 diabetes mellitus and gout
- Mother with essential hypertension
- Paternal uncle with alcohol use disorder

SOCIAL HISTORY:
He denies any illicit drug use or previous opioid prescriptions.

VITAL SIGNS:
On examination, his blood pressure is 138/85 mmHg, heart rate is 78 bpm, and BMI is 30 kg/m^2.

PHYSICAL EXAMINATION:
His liver is palpable 2 cm below the right costal margin, but there is no jaundice, ascites, or spider angiomas.

LABORATORY DATA:
Laboratory tests reveal the following:

 - Complete blood count: within normal limits
 - Liver function tests:
 - AST: 45 U/L (normal: 10-40 U/L)
 - ALT: 52 U/L (normal: 7-56 U/L)
 - Alkaline phosphatase: 85 U/L (normal: 40-130 U/L)
 - Total bilirubin: 1.2 mg/dL (normal: 0.3-1.2 mg/dL)
 - Serum creatinine: 0.9 mg/dL (normal: 0.6-1.2 mg/dL)

## Encounter 4: source 2022-03-15, practice 2024-06-03, outpatient, Internal Medicine

Chief complaint: Fatigue, shortness of breath on exertion, and sore red tongue

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, shortness of breath on exertion, and sore red tongue

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of his achalasia and alcohol use disorder, presenting with a 2-month history of fatigue, shortness of breath on exertion, and a sore, red tongue.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Obesity (diagnosed 2020-04-15)
- Achalasia (diagnosed 2021-03-15)
- Alcohol use disorder (diagnosed 2021-09-15)
- Type 2 diabetes mellitus
- Gout
- History of nephrolithiasis
- Folate deficiency anemia

He has a past medical history of hypertension and type 2 diabetes mellitus.

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Atorvastatin 20 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with type 2 diabetes mellitus and gout
- Mother with essential hypertension
- Paternal uncle with alcohol use disorder

SOCIAL HISTORY:
He reports a dietary preference for fast food and admits to consuming six to eight cans of beer daily for several years.

PHYSICAL EXAMINATION:
On physical examination, he is pale, and his tongue is smooth and erythematous.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 9.2 g/dL (Normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 115 fL (Normal: 80-96 fL)
- Reticulocyte count: 1% (Normal: 0.5-1.5%)
- Serum folate: 2 ng/mL (Normal: 3-17 ng/mL)
- Serum cobalamin (vitamin B12): 400 pg/mL (Normal: 200-900 pg/mL)
- Methylmalonic acid: Normal

## Encounter 5: source 2023-01-15, practice 2025-03-18, outpatient, Internal Medicine

Chief complaint: Routine follow-up for hypertension management

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for hypertension management

HISTORY OF PRESENT ILLNESS:
The patient is a 53-year-old male with a history of essential hypertension initially documented in 2020, returning for routine follow-up of his blood pressure management. He presents today with a medical history significant for obesity, achalasia, alcohol use disorder, and folate deficiency anemia addressed in prior encounters. He reports adherence to his medications and a recent increase in physical activity. Despite these efforts, he expresses frustration about his persistent high blood pressure. Over the past six months, the clinic has implemented a new intervention program aimed at improving blood pressure control among its patients. The program includes patient education, medication titration protocols, and monthly follow-up visits. The patient's blood pressure readings from the last six visits are as follows:

- Visit 1: 150/92 mmHg
- Visit 2: 148/90 mmHg
- Visit 3: 145/88 mmHg
- Visit 4: 142/86 mmHg
- Visit 5: 140/84 mmHg
- Visit 6: 138/82 mmHg

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Obesity (diagnosed 2020-04-15)
- Achalasia (diagnosed 2021-03-15)
- Alcohol use disorder (diagnosed 2021-09-15)
- Folate deficiency anemia (diagnosed 2022-03-15)
- Type 2 diabetes mellitus
- Gout
- History of nephrolithiasis

MEDICATIONS:
His current medications include metformin 1000 mg twice daily, lisinopril 20 mg daily, and atorvastatin 20 mg daily.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with type 2 diabetes mellitus and gout
- Mother with essential hypertension
- Paternal uncle with alcohol use disorder

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Alcohol use disorder with heavy daily consumption (6-8 beers daily for past 10 years); periods of reduced intake or social-only use reported during some encounters. Occupation: Warehouse supervisor.

VITAL SIGNS:
Blood pressure of 136/80 mmHg. His BMI is 30 kg/m^2

PHYSICAL EXAMINATION:
Physical examination today shows a blood pressure of 136/80 mmHg. His BMI is 30 kg/m^2, and the rest of the examination is unremarkable.

## Encounter 6: source 2023-07-15, practice 2025-10-02, ed, Emergency Medicine

Chief complaint: Left flank pain that started 2 days ago

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Maria Garcia, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Left flank pain that started 2 days ago

HISTORY OF PRESENT ILLNESS:
This 53-year-old male with a history of nephrolithiasis presents with left flank pain. The pain is sharp, radiates to the groin, and is associated with nausea but no vomiting. He denies any fever, dysuria, or change in urine color.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Obesity (diagnosed 2020-04-15)
- Achalasia (diagnosed 2021-03-15)
- Alcohol use disorder (diagnosed 2021-09-15)
- Folate deficiency anemia (diagnosed 2022-03-15)
- Type 2 diabetes mellitus
- Gout
- History of nephrolithiasis

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Atorvastatin 20 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with type 2 diabetes mellitus and gout
- Mother with essential hypertension
- Paternal uncle with alcohol use disorder

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Alcohol use disorder with heavy daily consumption (6-8 beers daily for past 10 years); periods of reduced intake or social-only use reported during some encounters. Occupation: Warehouse supervisor.

VITAL SIGNS:
BMI is 32 kg/m^2, and his blood pressure is 135/85 mmHg

PHYSICAL EXAMINATION:
Costovertebral angle tenderness on the left side

LABORATORY DATA:
Urinalysis shows a pH of 5.0 and the presence of uric acid crystals. Serum uric acid level is elevated.

IMAGING:
Renal ultrasound reveals a 6 mm radiolucent stone in the left ureter.

## Encounter 7: source 2024-01-15, practice 2026-03-26, outpatient, Rheumatology

Chief complaint: Severe pain, redness, and swelling in right great toe for 2 days

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. James Thompson, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Severe pain, redness, and swelling in right great toe for 2 days

HISTORY OF PRESENT ILLNESS:
This patient with history of gout, alcohol use disorder, and nephrolithiasis presents with recurrent symptoms. He describes the pain as throbbing and states that it began suddenly overnight. He has a history of similar, less severe episodes over the past two years, which resolved spontaneously.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Obesity (diagnosed 2020-04-15)
- Achalasia (diagnosed 2021-03-15)
- Alcohol use disorder (diagnosed 2021-09-15)
- Folate deficiency anemia (diagnosed 2022-03-15)
- Ureterolithiasis (diagnosed 2023-07-15)
- Type 2 diabetes mellitus
- Gout
- History of nephrolithiasis

He also reports a history of hypertension and type 2 diabetes mellitus.

MEDICATIONS:
His medications include lisinopril and metformin.

Home Medications:
- Atorvastatin 20 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with type 2 diabetes mellitus and gout
- Mother with essential hypertension
- Paternal uncle with alcohol use disorder

SOCIAL HISTORY:
He admits to consuming beer daily.

VITAL SIGNS:
He has a body mass index (BMI) of 32 kg/m^2.

PHYSICAL EXAMINATION:
On physical examination, the right first metatarsophalangeal (MTP) joint is erythematous, warm, and exquisitely tender to touch.

LABORATORY DATA:
Laboratory findings are as follows:

- Serum uric acid: 9.2 mg/dL (normal: 3.4-7.0 mg/dL)
- C-reactive protein (CRP): 24 mg/L (normal: <5 mg/L)
- Complete blood count (CBC), renal function, and serum glucose are within normal limits.
