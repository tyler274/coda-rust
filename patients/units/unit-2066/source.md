# synthetic_hospital patient 2066

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 47 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-04-02 (Fri) | outpatient | Gastroenterology |
| 1 | 2020-05-15 | 2021-08-09 (Mon) | outpatient | Neurology |
| 2 | 2021-01-15 | 2022-04-04 (Mon) | outpatient | Hematology & Oncology |
| 3 | 2022-01-15 | 2023-04-04 (Tue) | outpatient | Gastroenterology |
| 4 | 2023-01-15 | 2024-03-28 (Thu) | ed | Emergency Medicine |
| 5 | 2023-07-15 | 2024-10-01 (Tue) | ed | Emergency Medicine |
| 6 | 2024-01-15 | 2025-04-10 (Thu) | ed | Emergency Medicine |
| 7 | 2024-07-15 | 2025-10-05 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Unemployed; former construction laborer (currently on disability)",
 "alcohol_use": "Severe daily use: approximately 1 pint (16 oz) of vodka daily for >10 years; binge pattern with poor nutritional intake during drinking episodes; last consumption was 2 days prior to presentation",
 "family_history": [
  "Father: Alcohol use disorder with cirrhosis (deceased age 54)",
  "Mother: Type 2 diabetes mellitus, peripheral neuropathy",
  "One brother: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 1 pack per day for 20 years",
 "home_medications": [
  {
   "dose": "100 mg PO TID",
   "name": "Thiamine"
  },
  {
   "dose": "1 mg PO daily",
   "name": "Folic acid"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Pyridoxine"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Multivitamin"
  },
  {
   "dose": "36,000 units PO with meals and snacks",
   "name": "Pancrelipase"
  },
  {
   "dose": "400 mg PO TID",
   "name": "Magnesium oxide"
  },
  {
   "dose": "300 mg PO TID",
   "name": "Gabapentin"
  }
 ],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "Chronic alcohol use disorder, severe",
  "Chronic calcific pancreatitis with exocrine insufficiency",
  "Severe protein-calorie malnutrition with multiple micronutrient deficiencies (thiamine, pyridoxine, B12, folate, magnesium, fat-soluble vitamins)",
  "Alcoholic peripheral sensory-motor polyneuropathy",
  "Acquired sideroblastic anemia (alcohol-related)"
 ],
 "age_at_first_encounter": 47
}
```

Primary diagnoses: ["Hypomagnesemia", "Chronic pancreatitis (dietary management)", "Peripheral neuropathy", "Wernicke encephalopathy", "Sideroblastic anemia (acquired, alcohol-related)"]
Comorbidities: ["Protein-energy malnutrition", "Alcohol use disorder", "Alcoholic liver disease", "Chronic pancreatitis with exocrine insufficiency"]

## Encounter 0: source 2020-01-15, practice 2021-04-02, outpatient, Gastroenterology

Chief complaint: Intermittent epigastric pain radiating to the back, worse after fatty meals, with loose foul-smelling stools and weight loss.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent epigastric pain radiating to the back, worse after fatty meals, with loose foul-smelling stools and weight loss.

HISTORY OF PRESENT ILLNESS:
2-year history of intermittent epigastric pain that radiates to his back. He reports that the pain worsens after meals, especially those high in fat. He also complains of loose, foul-smelling stools and weight loss despite no change in diet.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic alcohol use disorder, severe
- Chronic calcific pancreatitis with exocrine insufficiency
- Severe protein-calorie malnutrition with multiple micronutrient deficiencies (thiamine, pyridoxine, B12, folate, magnesium, fat-soluble vitamins)
- Alcoholic peripheral sensory-motor polyneuropathy
- Acquired sideroblastic anemia (alcohol-related)

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Thiamine 100 mg PO TID
- Folic acid 1 mg PO daily
- Pyridoxine 50 mg PO daily
- Multivitamin 1 tablet PO daily
- Pancrelipase 36,000 units PO with meals and snacks
- Magnesium oxide 400 mg PO TID
- Gabapentin 300 mg PO TID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 54)
- Mother: Type 2 diabetes mellitus, peripheral neuropathy
- One brother: Hypertension

SOCIAL HISTORY:
History of chronic alcohol use

PHYSICAL EXAMINATION:
On physical examination, the patient appears cachectic and has mild epigastric tenderness to palpation without rebound or guarding.

LABORATORY DATA:
Serum amylase: 160 U/L (Normal: 30-110 U/L)
- Serum lipase: 300 U/L (Normal: 0-160 U/L)
- Fecal elastase: 100 µg/g (Normal: >200 µg/g)
- Fasting blood glucose: 126 mg/dL (Normal: 70-99 mg/dL)

IMAGING:
Abdominal CT scan reveals calcifications in the pancreas.

## Encounter 1: source 2020-05-15, practice 2021-08-09, outpatient, Neurology

Chief complaint: Progressive numbness and burning sensation in feet and hands described as 'wearing socks and gloves,' with difficulty walking in the dark and recent falls.

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Marcus Johnson, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive numbness and burning sensation in feet and hands described as 'wearing socks and gloves,' with difficulty walking in the dark and recent falls.

HISTORY OF PRESENT ILLNESS:
The patient returns for neurology evaluation after January gastroenterology treatment for pancreatitis. He describes the sensation as "wearing socks and gloves all the time." He also reports difficulty walking in the dark and has had several falls.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis (dietary management) (diagnosed 2020-01-15)
- Chronic alcohol use disorder, severe
- Chronic calcific pancreatitis with exocrine insufficiency
- Severe protein-calorie malnutrition with multiple micronutrient deficiencies (thiamine, pyridoxine, B12, folate, magnesium, fat-soluble vitamins)
- Alcoholic peripheral sensory-motor polyneuropathy
- Acquired sideroblastic anemia (alcohol-related)

His past medical history is significant for chronic alcoholism and poor nutritional intake.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Thiamine 100 mg PO TID
- Folic acid 1 mg PO daily
- Pyridoxine 50 mg PO daily
- Multivitamin 1 tablet PO daily
- Pancrelipase 36,000 units PO with meals and snacks
- Magnesium oxide 400 mg PO TID
- Gabapentin 300 mg PO TID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 54)
- Mother: Type 2 diabetes mellitus, peripheral neuropathy
- One brother: Hypertension

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 20 years. Alcohol: Severe daily use: approximately 1 pint (16 oz) of vodka daily for >10 years; binge pattern with poor nutritional intake during drinking episodes; last consumption was 2 days prior to presentation. Occupation: Unemployed; former construction laborer (currently on disability).

PHYSICAL EXAMINATION:
On physical examination, the patient has decreased sensation to light touch and vibration in a stocking-glove distribution. Ankle reflexes are absent bilaterally.

LABORATORY DATA:
Laboratory studies are as follows:

- Complete Blood Count (CBC): Normal
- Serum Vitamin B12 Level: Normal
- Fasting Blood Glucose: Normal
- Liver Function Tests (LFTs): Elevated AST and ALT
- Serum Thiamine Level: Low

## Encounter 2: source 2021-01-15, practice 2022-04-04, outpatient, Hematology & Oncology

Chief complaint: Fatigue and weakness over the past two months, with poor dietary intake lacking fruits and vegetables.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Patel, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue and weakness over the past two months, with poor dietary intake lacking fruits and vegetables.

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up, having previously presented with epigastric pain with steatorrhea and extremity numbness. He reports fatigue and weakness for the past two months with poor dietary intake lacking fruits and vegetables.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis (dietary management) (diagnosed 2020-01-15)
- Peripheral neuropathy (diagnosed 2020-05-15)
- Chronic alcohol use disorder, severe
- Chronic calcific pancreatitis with exocrine insufficiency
- Severe protein-calorie malnutrition with multiple micronutrient deficiencies (thiamine, pyridoxine, B12, folate, magnesium, fat-soluble vitamins)
- Alcoholic peripheral sensory-motor polyneuropathy
- Acquired sideroblastic anemia (alcohol-related)

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Thiamine 100 mg PO TID
- Folic acid 1 mg PO daily
- Pyridoxine 50 mg PO daily
- Multivitamin 1 tablet PO daily
- Pancrelipase 36,000 units PO with meals and snacks
- Magnesium oxide 400 mg PO TID
- Gabapentin 300 mg PO TID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 54)
- Mother: Type 2 diabetes mellitus, peripheral neuropathy
- One brother: Hypertension

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 20 years. Alcohol: Severe daily use: approximately 1 pint (16 oz) of vodka daily for >10 years; binge pattern with poor nutritional intake during drinking episodes; last consumption was 2 days prior to presentation. Occupation: Unemployed; former construction laborer (currently on disability).

PHYSICAL EXAMINATION:
On physical examination, he appears pale with conjunctival pallor. No hepatosplenomegaly is noted.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 9.8 g/dL (Normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 74 fL (Normal: 80-96 fL)
- Serum iron: 190 µg/dL (Normal: 65-176 µg/dL)
- Total iron-binding capacity (TIBC): 210 µg/dL (Normal: 250-450 µg/dL)
- Ferritin: 300 ng/mL (Normal: 30-400 ng/mL)

A peripheral blood smear shows ringed sideroblasts.

## Encounter 3: source 2022-01-15, practice 2023-04-04, outpatient, Gastroenterology

Chief complaint: Persistent, dull epigastric pain radiating to the back with progressive weight loss and bulky, foul-smelling stools over the past year.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent, dull epigastric pain radiating to the back with progressive weight loss and bulky, foul-smelling stools over the past year.

HISTORY OF PRESENT ILLNESS:
The patient with chronic calcific pancreatitis and alcohol use disorder returns for gastroenterology follow-up, having been previously evaluated in January 2020. He mentions that his symptoms have been progressively worsening over the past year. He denies any recent changes in diet or medication.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis (dietary management) (diagnosed 2020-01-15)
- Peripheral neuropathy (diagnosed 2020-05-15)
- Sideroblastic anemia (acquired, alcohol-related) (diagnosed 2021-01-15)
- Chronic alcohol use disorder, severe
- Chronic calcific pancreatitis with exocrine insufficiency
- Severe protein-calorie malnutrition with multiple micronutrient deficiencies (thiamine, pyridoxine, B12, folate, magnesium, fat-soluble vitamins)
- Alcoholic peripheral sensory-motor polyneuropathy

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Thiamine 100 mg PO TID
- Folic acid 1 mg PO daily
- Pyridoxine 50 mg PO daily
- Multivitamin 1 tablet PO daily
- Pancrelipase 36,000 units PO with meals and snacks
- Magnesium oxide 400 mg PO TID
- Gabapentin 300 mg PO TID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 54)
- Mother: Type 2 diabetes mellitus, peripheral neuropathy
- One brother: Hypertension

SOCIAL HISTORY:
History of chronic alcohol abuse

PHYSICAL EXAMINATION:
On physical examination, the patient appears cachectic and mildly jaundiced. His abdomen is tender to palpation in the epigastric region without rebound or guarding.

LABORATORY DATA:
Serum amylase: Normal
Serum lipase: Normal
Fecal elastase: 200 µg/g)
Fasting blood glucose: 160 mg/dL
Hemoglobin A1c: 7.2%

## Encounter 4: source 2023-01-15, practice 2024-03-28, ed, Emergency Medicine

Chief complaint: Acute onset confusion, difficulty walking, and double vision for the past two days following weeks of heavy drinking with poor nutritional intake.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset confusion, difficulty walking, and double vision for the past two days following weeks of heavy drinking with poor nutritional intake.

HISTORY OF PRESENT ILLNESS:
A 50-year-old male with chronic alcoholism, chronic calcific pancreatitis with exocrine insufficiency, and severe malnutrition previously evaluated in gastroenterology and neurology clinics presents to the emergency department with confusion, difficulty walking, and double vision for the past two days. His friends report that he has been drinking heavily for several weeks and has not been eating well.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis (dietary management) (diagnosed 2020-01-15)
- Peripheral neuropathy (diagnosed 2020-05-15)
- Sideroblastic anemia (acquired, alcohol-related) (diagnosed 2021-01-15)
- Chronic alcohol use disorder, severe
- Chronic calcific pancreatitis with exocrine insufficiency
- Severe protein-calorie malnutrition with multiple micronutrient deficiencies (thiamine, pyridoxine, B12, folate, magnesium, fat-soluble vitamins)
- Alcoholic peripheral sensory-motor polyneuropathy

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Thiamine 100 mg PO TID
- Folic acid 1 mg PO daily
- Pyridoxine 50 mg PO daily
- Multivitamin 1 tablet PO daily
- Pancrelipase 36,000 units PO with meals and snacks
- Magnesium oxide 400 mg PO TID
- Gabapentin 300 mg PO TID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 54)
- Mother: Type 2 diabetes mellitus, peripheral neuropathy
- One brother: Hypertension

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 20 years. Alcohol: Severe daily use: approximately 1 pint (16 oz) of vodka daily for >10 years; binge pattern with poor nutritional intake during drinking episodes; last consumption was 2 days prior to presentation. Occupation: Unemployed; former construction laborer (currently on disability).

PHYSICAL EXAMINATION:
On examination, the patient is disoriented to time and place. He has horizontal nystagmus and bilateral abducens nerve palsies. His gait is wide-based and unsteady.

LABORATORY DATA:
Laboratory studies are as follows:

- Hemoglobin: 13.2 g/dL
- Mean corpuscular volume (MCV): 96 fL
- Serum glucose: 98 mg/dL
- Serum thiamine: Not measured
- Serum electrolytes: Within normal limits
- Liver function tests: Mildly elevated AST and ALT

## Encounter 5: source 2023-07-15, practice 2024-10-01, ed, Emergency Medicine

Chief complaint: Generalized weakness and fatigue with suspected electrolyte imbalance despite prescribed oral magnesium supplementation.

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Emily Watson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Generalized weakness and fatigue with suspected electrolyte imbalance despite prescribed oral magnesium supplementation.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis (dietary management) (diagnosed 2020-01-15)
- Peripheral neuropathy (diagnosed 2020-05-15)
- Sideroblastic anemia (acquired, alcohol-related) (diagnosed 2021-01-15)
- Wernicke encephalopathy (diagnosed 2023-01-15)
- Chronic alcohol use disorder, severe
- Chronic calcific pancreatitis with exocrine insufficiency
- Severe protein-calorie malnutrition with multiple micronutrient deficiencies (thiamine, pyridoxine, B12, folate, magnesium, fat-soluble vitamins)
- Alcoholic peripheral sensory-motor polyneuropathy

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Thiamine 100 mg PO TID
- Folic acid 1 mg PO daily
- Pyridoxine 50 mg PO daily
- Multivitamin 1 tablet PO daily
- Pancrelipase 36,000 units PO with meals and snacks
- Magnesium oxide 400 mg PO TID
- Gabapentin 300 mg PO TID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 54)
- Mother: Type 2 diabetes mellitus, peripheral neuropathy
- One brother: Hypertension

SOCIAL HISTORY:
He reports poor dietary intake and admits to consuming a pint of vodka daily

VITAL SIGNS:
His vital signs are stable

PHYSICAL EXAMINATION:
On examination, the patient appears lethargic but is oriented to time, place, and person. Physical examination reveals decreased muscle strength, no edema, and normal heart and lung examinations.

LABORATORY DATA:
Initial laboratory tests reveal the following:

- Serum potassium: 2.9 mEq/L (Normal: 3.5-5.0 mEq/L)
- Serum magnesium: 1.2 mg/dL (Normal: 1.7-2.2 mg/dL)
- Serum calcium: 9.0 mg/dL (Normal: 8.5-10.2 mg/dL)
- Serum phosphorus: 2.5 mg/dL (Normal: 2.5-4.5 mg/dL)
- Serum creatinine: 0.9 mg/dL (Normal: 0.6-1.2 mg/dL)

Despite oral potassium supplementation, the patient's serum potassium level remains persistently low on subsequent testing.

## Encounter 6: source 2024-01-15, practice 2025-04-10, ed, Emergency Medicine

Chief complaint: Recurrent confusion, difficulty walking, and double vision that started two days ago after several weeks of heavy drinking and minimal food intake.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Michael Chang, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Recurrent confusion, difficulty walking, and double vision that started two days ago after several weeks of heavy drinking and minimal food intake.

HISTORY OF PRESENT ILLNESS:
This 51-year-old male with chronic alcohol use disorder, severe protein-calorie malnutrition, and prior Wernicke encephalopathy returns to the emergency department with confusion, difficulty walking, and double vision that started two days ago. His friends report that he has been drinking heavily for the past several weeks and has not eaten much.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis (dietary management) (diagnosed 2020-01-15)
- Peripheral neuropathy (diagnosed 2020-05-15)
- Sideroblastic anemia (acquired, alcohol-related) (diagnosed 2021-01-15)
- Wernicke encephalopathy (diagnosed 2023-01-15)
- Hypomagnesemia (diagnosed 2023-07-15)
- Chronic alcohol use disorder, severe
- Chronic calcific pancreatitis with exocrine insufficiency
- Severe protein-calorie malnutrition with multiple micronutrient deficiencies (thiamine, pyridoxine, B12, folate, magnesium, fat-soluble vitamins)
- Alcoholic peripheral sensory-motor polyneuropathy

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Thiamine 100 mg PO TID
- Folic acid 1 mg PO daily
- Pyridoxine 50 mg PO daily
- Multivitamin 1 tablet PO daily
- Pancrelipase 36,000 units PO with meals and snacks
- Magnesium oxide 400 mg PO TID
- Gabapentin 300 mg PO TID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 54)
- Mother: Type 2 diabetes mellitus, peripheral neuropathy
- One brother: Hypertension

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 20 years. Alcohol: Severe daily use: approximately 1 pint (16 oz) of vodka daily for >10 years; binge pattern with poor nutritional intake during drinking episodes; last consumption was 2 days prior to presentation. Occupation: Unemployed; former construction laborer (currently on disability).

PHYSICAL EXAMINATION:
On examination, the patient is disoriented to time and place, has nystagmus, and demonstrates a wide-based gait. His vital signs are stable, and there is no evidence of trauma.

LABORATORY DATA:
Serum glucose: 90 mg/dL
Serum sodium: 140 mEq/L
Serum potassium: 4.0 mEq/L
Serum creatinine: 1.1 mg/dL
Serum aspartate aminotransferase (AST): 120 U/L
Serum alanine aminotransferase (ALT): 50 U/L
Serum thiamine: Not measured

## Encounter 7: source 2024-07-15, practice 2025-10-05, ed, Emergency Medicine

Chief complaint: Muscle cramps, tingling in hands and feet, and witnessed generalized tonic-clonic seizure with last alcohol consumption two days prior to presentation.

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Lisa Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Muscle cramps, tingling in hands and feet, and witnessed generalized tonic-clonic seizure with last alcohol consumption two days prior to presentation.

HISTORY OF PRESENT ILLNESS:
The patient with chronic calcific pancreatitis and prior hypomagnesemia presents to the emergency department with complaints of muscle cramps, tingling in his hands and feet, and a recent seizure. He admits to poor dietary intake and has not consumed alcohol for the past two days.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis (dietary management) (diagnosed 2020-01-15)
- Peripheral neuropathy (diagnosed 2020-05-15)
- Sideroblastic anemia (acquired, alcohol-related) (diagnosed 2021-01-15)
- Wernicke encephalopathy (diagnosed 2023-01-15)
- Hypomagnesemia (diagnosed 2023-07-15)
- Chronic alcohol use disorder, severe
- Chronic calcific pancreatitis with exocrine insufficiency
- Severe protein-calorie malnutrition with multiple micronutrient deficiencies (thiamine, pyridoxine, B12, folate, magnesium, fat-soluble vitamins)
- Alcoholic peripheral sensory-motor polyneuropathy

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Thiamine 100 mg PO TID
- Folic acid 1 mg PO daily
- Pyridoxine 50 mg PO daily
- Multivitamin 1 tablet PO daily
- Pancrelipase 36,000 units PO with meals and snacks
- Magnesium oxide 400 mg PO TID
- Gabapentin 300 mg PO TID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 54)
- Mother: Type 2 diabetes mellitus, peripheral neuropathy
- One brother: Hypertension

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 20 years. Alcohol: Severe daily use: approximately 1 pint (16 oz) of vodka daily for >10 years; binge pattern with poor nutritional intake during drinking episodes; last consumption was 2 days prior to presentation. Occupation: Unemployed; former construction laborer (currently on disability).

PHYSICAL EXAMINATION:
On examination, the patient appears malnourished. Chvostek's and Trousseau's signs are positive.

LABORATORY DATA:
Laboratory results are as follows:

- Serum calcium: 7.2 mg/dL (Normal: 8.5-10.2 mg/dL)
- Serum magnesium: 1.0 mg/dL (Normal: 1.7-2.2 mg/dL)
- Serum phosphorus: 3.0 mg/dL (Normal: 2.5-4.5 mg/dL)
- Serum PTH: low

OTHER STUDIES:
An electrocardiogram shows a prolonged QT interval.
