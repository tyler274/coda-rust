# synthetic_hospital patient 2080

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 56 at first encounter, sex M, Non-Hispanic White, Medicare and supplemental private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-02-24 (Wed) | outpatient | Gastroenterology |
| 1 | 2020-04-15 | 2021-05-26 (Wed) | outpatient | Internal Medicine |
| 2 | 2020-07-15 | 2021-08-23 (Mon) | ed | Emergency Medicine |
| 3 | 2021-01-15 | 2022-03-07 (Mon) | outpatient | Hematology & Oncology |
| 4 | 2022-01-15 | 2023-02-24 (Fri) | outpatient | Hematology & Oncology |
| 5 | 2023-01-15 | 2024-02-21 (Wed) | outpatient | Gastroenterology |
| 6 | 2024-01-15 | 2025-02-27 (Thu) | inpatient | Gastroenterology |
| 7 | 2025-01-15 | 2026-02-19 (Thu) | icu | Cardiology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare and supplemental private insurance",
 "occupation": "Former construction supervisor, currently on medical disability leave",
 "alcohol_use": "History of alcohol use disorder, in sustained remission (sober 1 year)",
 "family_history": [
  "Father: Alcohol use disorder with cirrhosis (deceased age 65)",
  "Mother: Type 2 diabetes mellitus",
  "Brother: Chronic pancreatitis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "30-pack-year smoking history",
 "home_medications": [
  {
   "dose": "36,000 units orally with meals and snacks",
   "name": "Pancrelipase (Creon)"
  },
  {
   "dose": "24 units subcutaneous at bedtime",
   "name": "Insulin glargine (Lantus)"
  }
 ],
 "surgical_history": [
  "Cholecystectomy (age 45)",
  "Appendectomy (age 12)"
 ],
 "chronic_conditions": [
  "Chronic pancreatitis with exocrine insufficiency",
  "Chronic hepatitis C infection with progressive hepatic dysfunction",
  "Diabetes mellitus secondary to chronic pancreatitis (Type 3c)"
 ],
 "age_at_first_encounter": 56
}
```

Primary diagnoses: ["Chronic pancreatitis", "Insulin-induced hypoglycemia", "Pancreatic duct leak", "Hairy cell leukemia", "Esophageal carcinoma", "Infective Endocarditis (Aortic Valve) with Complications", "Hypoalbuminemia", "Follicular lymphoma"]
Comorbidities: ["Chronic pancreatitis", "Anemia", "Splenomegaly", "Chronic pancreatitis with exocrine insufficiency", "Alcohol use"]

## Encounter 0: source 2020-01-15, practice 2021-02-24, outpatient, Gastroenterology

Chief complaint: Loose, bulky, foul-smelling stools with oily residue for 3 weeks and unintentional weight loss

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Loose, bulky, foul-smelling stools with oily residue for 3 weeks and unintentional weight loss

HISTORY OF PRESENT ILLNESS:
He says the stools have been unusually foul-smelling and leave an oily residue in the toilet bowl. He reports unintentional weight loss over the past month despite a stable appetite. He denies recent travel, fever, or blood in the stool.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis with exocrine insufficiency
- Chronic hepatitis C infection with progressive hepatic dysfunction
- Diabetes mellitus secondary to chronic pancreatitis (Type 3c)

His medical history includes recurrent episodes of upper abdominal pain over the last several years.

PAST SURGICAL HISTORY:
- Cholecystectomy (age 45)
- Appendectomy (age 12)

MEDICATIONS:
Home Medications:
- Pancrelipase (Creon) 36,000 units orally with meals and snacks
- Insulin glargine (Lantus) 24 units subcutaneous at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 65)
- Mother: Type 2 diabetes mellitus
- Brother: Chronic pancreatitis

SOCIAL HISTORY:
He occasionally drinks alcohol, but has significantly reduced his intake in the past year.

PHYSICAL EXAMINATION:
Physical examination shows mild abdominal tenderness without guarding or rebound. No jaundice or peripheral stigmata of chronic liver disease are noted.

LABORATORY DATA:
Laboratory evaluation shows mild anemia and low serum albumin.

IMAGING:
CT scan of the abdomen shows pancreatic atrophy with coarse calcifications and irregularly dilated pancreatic ducts.

## Encounter 1: source 2020-04-15, practice 2021-05-26, outpatient, Internal Medicine

Chief complaint: Routine follow-up for chronic conditions with mild fatigue

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for chronic conditions with mild fatigue

HISTORY OF PRESENT ILLNESS:
The patient returns for routine follow-up of his chronic pancreatitis with exocrine insufficiency, hepatitis C infection, and type 3c diabetes mellitus following his January gastroenterology evaluation. He reports mild fatigue but denies recent weight loss, heat intolerance, or palpitations.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis (diagnosed 2020-01-15)
- Chronic pancreatitis with exocrine insufficiency
- Chronic hepatitis C infection with progressive hepatic dysfunction
- Diabetes mellitus secondary to chronic pancreatitis (Type 3c)

He has a history of chronic hepatitis C infection diagnosed 15 years ago and developed signs of progressive hepatic dysfunction over the past decade.

PAST SURGICAL HISTORY:
- Cholecystectomy (age 45)
- Appendectomy (age 12)

MEDICATIONS:
He does not take thyroid medications, corticosteroids, or androgens.

Home Medications:
- Pancrelipase (Creon) 36,000 units orally with meals and snacks
- Insulin glargine (Lantus) 24 units subcutaneous at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 65)
- Mother: Type 2 diabetes mellitus
- Brother: Chronic pancreatitis

SOCIAL HISTORY:
Smoking: 30-pack-year smoking history. Alcohol: History of alcohol use disorder, in sustained remission (sober 1 year). Occupation: Former construction supervisor, currently on medical disability leave.

PHYSICAL EXAMINATION:
Physical examination shows temporal muscle wasting, spider angiomas on the upper chest, and mild bilateral ankle edema. There is no palpable thyroid enlargement or tenderness. Abdominal examination reveals a distended abdomen with a fluid wave.

LABORATORY DATA:
Laboratory evaluation reveals mild hypoalbuminemia and elevated serum transaminases. Thyroid function tests show reduced total thyroxine (T4) but normal thyroid-stimulating hormone (TSH) and free thyroxine (FT4) levels.

## Encounter 2: source 2020-07-15, practice 2021-08-23, ed, Emergency Medicine

Chief complaint: Dizziness, palpitations, and diaphoresis occurring before meals

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Emily Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Dizziness, palpitations, and diaphoresis occurring before meals

HISTORY OF PRESENT ILLNESS:
The patient is a 56-year-old male with chronic pancreatitis with exocrine insufficiency and Type 3c diabetes mellitus, previously evaluated for malabsorptive symptoms and hypoalbuminemia, who presents with dizziness, palpitations, and diaphoresis. These symptoms often occur before meals and resolve after eating.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis (diagnosed 2020-01-15)
- Hypoalbuminemia (diagnosed 2020-04-15)
- Chronic pancreatitis with exocrine insufficiency
- Chronic hepatitis C infection with progressive hepatic dysfunction
- Diabetes mellitus secondary to chronic pancreatitis (Type 3c)

PAST SURGICAL HISTORY:
Partial pancreatic resection two years ago due to a benign neoplasm

MEDICATIONS:
He takes pancreatic enzyme replacement and long-acting insulin.

Home Medications:
- Pancrelipase (Creon) 36,000 units orally with meals and snacks
- Insulin glargine (Lantus) 24 units subcutaneous at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 65)
- Mother: Type 2 diabetes mellitus
- Brother: Chronic pancreatitis

SOCIAL HISTORY:
Smoking: 30-pack-year smoking history. Alcohol: History of alcohol use disorder, in sustained remission (sober 1 year). Occupation: Former construction supervisor, currently on medical disability leave.

PHYSICAL EXAMINATION:
On physical examination, his vital signs are stable, and there are no remarkable findings.

LABORATORY DATA:
- Fasting blood glucose: 60 mg/dL (Normal: 70-99 mg/dL)
- Hemoglobin A1c: 6.2% (Normal: 4.8-5.6%)
- C-peptide: 0.5 ng/mL (Normal: 1.1-4.4 ng/mL)
- Serum insulin: Low

## Encounter 3: source 2021-01-15, practice 2022-03-07, outpatient, Hematology & Oncology

Chief complaint: Increasing fatigue, easy bruising, and 10-pound weight loss over 6 months

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Aisha Patel, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing fatigue, easy bruising, and 10-pound weight loss over 6 months

HISTORY OF PRESENT ILLNESS:
57-year-old male with chronic pancreatitis, hepatitis C, and diabetes mellitus returns after hypoalbuminemia and hypoglycemia. He presents to clinic with a 6-month history of increasing fatigue, easy bruising, and unintentional weight loss of 10 pounds. He denies fever, night sweats, or significant lymph node enlargement.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis (diagnosed 2020-01-15)
- Hypoalbuminemia (diagnosed 2020-04-15)
- Insulin-induced hypoglycemia (diagnosed 2020-07-15)
- Chronic pancreatitis with exocrine insufficiency
- Chronic hepatitis C infection with progressive hepatic dysfunction
- Diabetes mellitus secondary to chronic pancreatitis (Type 3c)

PAST SURGICAL HISTORY:
- Cholecystectomy (age 45)
- Appendectomy (age 12)

MEDICATIONS:
Home Medications:
- Pancrelipase (Creon) 36,000 units orally with meals and snacks
- Insulin glargine (Lantus) 24 units subcutaneous at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 65)
- Mother: Type 2 diabetes mellitus
- Brother: Chronic pancreatitis

SOCIAL HISTORY:
Smoking: 30-pack-year smoking history. Alcohol: History of alcohol use disorder, in sustained remission (sober 1 year). Occupation: Former construction supervisor, currently on medical disability leave.

PHYSICAL EXAMINATION:
On physical examination, he has marked splenomegaly without hepatomegaly or lymphadenopathy.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 10 g/dL (Normal: 13.8-17.2 g/dL)
- Platelet count: 80,000/mm³ (Normal: 150,000-450,000/mm³)
- White blood cell count: 12,000/mm³ (Normal: 4,500-11,000/mm³)
- Differential: 60% lymphocytes, many with cytoplasmic projections
- Neutrophils: 30%
- Monocytes: 5%
- Eosinophils: 3%
- Basophils: 2%
- Reticulocyte count: 1% (Normal: 0.5-1.5%)
- Peripheral smear: Presence of atypical lymphocytes with 'hairy' cytoplasmic projections

PATHOLOGY:
A bone marrow biopsy is performed and shows infiltration with atypical lymphocytes that stain positive for tartrate-resistant acid phosphatase (TRAP).

## Encounter 4: source 2022-01-15, practice 2023-02-24, outpatient, Hematology & Oncology

Chief complaint: Persistent fatigue, early satiety, and painless swelling in neck and groin

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Aisha Patel, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue, early satiety, and painless swelling in neck and groin

HISTORY OF PRESENT ILLNESS:
The patient with chronic pancreatitis, hepatitis C, and hairy cell leukemia returns with fatigue that has gradually progressed over the past several months. He has also experienced occasional night sweats and a 4-kg (9-lb) unintentional weight loss during this time. He denies fever, cough, chest pain, or recent infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis (diagnosed 2020-01-15)
- Hypoalbuminemia (diagnosed 2020-04-15)
- Insulin-induced hypoglycemia (diagnosed 2020-07-15)
- Hairy cell leukemia (diagnosed 2021-01-15)
- Chronic pancreatitis with exocrine insufficiency
- Chronic hepatitis C infection with progressive hepatic dysfunction
- Diabetes mellitus secondary to chronic pancreatitis (Type 3c)

PAST SURGICAL HISTORY:
- Cholecystectomy (age 45)
- Appendectomy (age 12)

MEDICATIONS:
Home Medications:
- Pancrelipase (Creon) 36,000 units orally with meals and snacks
- Insulin glargine (Lantus) 24 units subcutaneous at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 65)
- Mother: Type 2 diabetes mellitus
- Brother: Chronic pancreatitis

SOCIAL HISTORY:
Smoking: 30-pack-year smoking history. Alcohol: History of alcohol use disorder, in sustained remission (sober 1 year). Occupation: Former construction supervisor, currently on medical disability leave.

PHYSICAL EXAMINATION:
On examination, multiple non-tender, firm lymph nodes are palpable in the cervical and inguinal chains. Mild splenomegaly is noted.

LABORATORY DATA:
Laboratory studies reveal mild normocytic anemia and elevated lactate dehydrogenase.

PATHOLOGY:
A lymph node biopsy demonstrates closely packed follicles with a monotonous population of small cleaved cells. Immunophenotyping reveals CD10 and BCL2 positivity. Cytogenetic analysis identifies a chromosomal translocation involving chromosomes 14 and 18.

## Encounter 5: source 2023-01-15, practice 2024-02-21, outpatient, Gastroenterology

Chief complaint: Progressive difficulty swallowing solids and liquids over 3 months with 10-pound weight loss

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty swallowing solids and liquids over 3 months with 10-pound weight loss

HISTORY OF PRESENT ILLNESS:
The patient returns to Gastroenterology with progressive dysphagia in the setting of chronic pancreatitis and treated hematologic malignancies. He reports progressive difficulty swallowing initially to solids and now to liquids. He reports a 10-pound weight loss during this period without any changes in diet or exercise. He also complains of occasional regurgitation of undigested food and a sensation of food sticking in his chest after eating.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis (diagnosed 2020-01-15)
- Hypoalbuminemia (diagnosed 2020-04-15)
- Insulin-induced hypoglycemia (diagnosed 2020-07-15)
- Hairy cell leukemia (diagnosed 2021-01-15)
- Follicular lymphoma (diagnosed 2022-01-15)
- Chronic pancreatitis with exocrine insufficiency
- Chronic hepatitis C infection with progressive hepatic dysfunction
- Diabetes mellitus secondary to chronic pancreatitis (Type 3c)

PAST SURGICAL HISTORY:
- Cholecystectomy (age 45)
- Appendectomy (age 12)

MEDICATIONS:
Home Medications:
- Pancrelipase (Creon) 36,000 units orally with meals and snacks
- Insulin glargine (Lantus) 24 units subcutaneous at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 65)
- Mother: Type 2 diabetes mellitus
- Brother: Chronic pancreatitis

SOCIAL HISTORY:
He has a 30-pack-year smoking history and consumes alcohol socially.

PHYSICAL EXAMINATION:
On physical examination, he appears cachectic. There is no lymphadenopathy, and the abdominal examination is normal.

LABORATORY DATA:
Laboratory studies show mild anemia with a hemoglobin of 11.2 g/dL (normal: 13.8-17.2 g/dL).

IMAGING:
A barium esophagram shows an area of irregular narrowing in the distal esophagus.

## Encounter 6: source 2024-01-15, practice 2025-02-27, inpatient, Gastroenterology

Chief complaint: Constant abdominal pain radiating to the back with vomiting, worsening over 1 week

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. James Thompson, MD
Department: Gastroenterology
Visit Type: inpatient

CHIEF COMPLAINT:
Constant abdominal pain radiating to the back with vomiting, worsening over 1 week

HISTORY OF PRESENT ILLNESS:
The patient with chronic pancreatitis since 2020 returns. He reports that the pain is constant, radiating to the back, and has been worsening over the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis (diagnosed 2020-01-15)
- Hypoalbuminemia (diagnosed 2020-04-15)
- Insulin-induced hypoglycemia (diagnosed 2020-07-15)
- Hairy cell leukemia (diagnosed 2021-01-15)
- Follicular lymphoma (diagnosed 2022-01-15)
- Esophageal carcinoma (diagnosed 2023-01-15)
- Chronic pancreatitis with exocrine insufficiency
- Chronic hepatitis C infection with progressive hepatic dysfunction
- Diabetes mellitus secondary to chronic pancreatitis (Type 3c)

PAST SURGICAL HISTORY:
- Cholecystectomy (age 45)
- Appendectomy (age 12)

MEDICATIONS:
Home Medications:
- Pancrelipase (Creon) 36,000 units orally with meals and snacks
- Insulin glargine (Lantus) 24 units subcutaneous at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 65)
- Mother: Type 2 diabetes mellitus
- Brother: Chronic pancreatitis

SOCIAL HISTORY:
He has a history of alcohol abuse but has been sober for the past year.

VITAL SIGNS:
His vital signs are stable.

PHYSICAL EXAMINATION:
On examination, his abdomen is tender to palpation in the epigastric region without rebound or guarding.

LABORATORY DATA:
Laboratory results are as follows:

- Serum sodium: 140 mEq/L
- Serum potassium: 4.0 mEq/L
- Serum chloride: 108 mEq/L
- Serum bicarbonate (HCO3): 15 mEq/L
- Blood urea nitrogen (BUN): 18 mg/dL
- Serum creatinine: 1.0 mg/dL
- Serum glucose: 90 mg/dL
- Serum calcium: 9.2 mg/dL
- Anion gap: 17 mEq/L

IMAGING:
An abdominal CT scan shows no evidence of acute pancreatitis but reveals a small amount of free fluid near the tail of the pancreas.

## Encounter 7: source 2025-01-15, practice 2026-02-19, icu, Cardiology

Chief complaint: Fever, malaise, mild dyspnea, and fatigue for 1 week

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. David Park, MD
Department: Cardiology
Visit Type: icu

CHIEF COMPLAINT:
Fever, malaise, mild dyspnea, and fatigue for 1 week

HISTORY OF PRESENT ILLNESS:
The patient is a 61-year-old male with chronic pancreatitis, hepatitis C, diabetes mellitus, and history of lymphoma and esophageal carcinoma who presents to the emergency department with fever and malaise for the past week. He also reports mild dyspnea and fatigue.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis (diagnosed 2020-01-15)
- Hypoalbuminemia (diagnosed 2020-04-15)
- Insulin-induced hypoglycemia (diagnosed 2020-07-15)
- Hairy cell leukemia (diagnosed 2021-01-15)
- Follicular lymphoma (diagnosed 2022-01-15)
- Esophageal carcinoma (diagnosed 2023-01-15)
- Pancreatic duct leak (diagnosed 2024-01-15)
- Chronic pancreatitis with exocrine insufficiency
- Chronic hepatitis C infection with progressive hepatic dysfunction
- Diabetes mellitus secondary to chronic pancreatitis (Type 3c)

PAST SURGICAL HISTORY:
- Cholecystectomy (age 45)
- Appendectomy (age 12)

MEDICATIONS:
Home Medications:
- Pancrelipase (Creon) 36,000 units orally with meals and snacks
- Insulin glargine (Lantus) 24 units subcutaneous at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 65)
- Mother: Type 2 diabetes mellitus
- Brother: Chronic pancreatitis

SOCIAL HISTORY:
Smoking: 30-pack-year smoking history. Alcohol: History of alcohol use disorder, in sustained remission (sober 1 year). Occupation: Former construction supervisor, currently on medical disability leave.

VITAL SIGNS:
His vital signs are: temperature 38.8°C (101.8°F), blood pressure 120/72 mmHg, heart rate 96/min, respiratory rate 18/min, and oxygen saturation 97% on room air

PHYSICAL EXAMINATION:
On cardiac examination, an early diastolic murmur is heard at the left sternal border during full expiration.

LABORATORY DATA:
Laboratory studies reveal a hemoglobin of 10.2 g/dL and a leukocyte count of 12,200/mm³. Blood cultures are pending.

OTHER STUDIES:
A 12-lead ECG shows new-onset atrioventricular (AV) block.
