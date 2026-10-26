# synthetic_hospital patient 2756

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 47 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-09-11 (Mon) | outpatient | Bariatric Surgery |
| 1 | 2020-07-15 | 2024-03-11 (Mon) | outpatient | Gastroenterology |
| 2 | 2021-07-15 | 2025-03-10 (Mon) | outpatient | Rheumatology |
| 3 | 2022-07-15 | 2026-03-09 (Mon) | outpatient | Neurology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Remote administrative assistant",
 "alcohol_use": "Rare (1-2 drinks per year)",
 "family_history": [
  "Sister with celiac disease",
  "Mother with osteoporosis",
  "Father with obesity and type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 tablet daily (patient noncompliant)",
   "name": "Multivitamin with minerals (bariatric formulation)"
  },
  {
   "dose": "600 mg/400 IU three times daily (patient noncompliant)",
   "name": "Calcium citrate with vitamin D"
  },
  {
   "dose": "1000 mcg daily (patient noncompliant)",
   "name": "Vitamin B12 (sublingual)"
  },
  {
   "dose": "325 mg daily (patient noncompliant)",
   "name": "Iron sulfate"
  }
 ],
 "surgical_history": [
  "Roux-en-Y gastric bypass 18 months prior to initial presentation",
  "Laparoscopic cholecystectomy 5 years prior"
 ],
 "chronic_conditions": [
  "Morbid obesity (status post surgical weight loss)",
  "Celiac disease",
  "Status post Roux-en-Y gastric bypass",
  "Postmenopausal status (2 years)",
  "Malabsorption syndrome"
 ],
 "age_at_first_encounter": 47
}
```

Primary diagnoses: ["Osteomalacia", "Copper deficiency", "Celiac disease", "Morbid obesity"]
Comorbidities: ["Bile Acid Diarrhea", "Status post Roux-en-Y gastric bypass surgery", "Morbid obesity"]

## Encounter 0: source 2020-01-15, practice 2023-09-11, outpatient, Bariatric Surgery

Chief complaint: Routine examination following bariatric surgery with occasional knee stiffness after prolonged sitting.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Sharma, MD
Department: Bariatric Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Routine examination following bariatric surgery with occasional knee stiffness after prolonged sitting.

HISTORY OF PRESENT ILLNESS:
She has no current joint pain but reports occasional stiffness in her knees after sitting for extended periods.

PAST MEDICAL HISTORY:
Active Problem List:
- Morbid obesity (status post surgical weight loss)
- Celiac disease
- Status post Roux-en-Y gastric bypass
- Postmenopausal status (2 years)
- Malabsorption syndrome

She has been postmenopausal for 2 years.

PAST SURGICAL HISTORY:
- Roux-en-Y gastric bypass 18 months prior to initial presentation
- Laparoscopic cholecystectomy 5 years prior

MEDICATIONS:
Home Medications:
- Multivitamin with minerals (bariatric formulation) 1 tablet daily (patient noncompliant)
- Calcium citrate with vitamin D 600 mg/400 IU three times daily (patient noncompliant)
- Vitamin B12 (sublingual) 1000 mcg daily (patient noncompliant)
- Iron sulfate 325 mg daily (patient noncompliant)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Sister with celiac disease
- Mother with osteoporosis
- Father with obesity and type 2 diabetes mellitus

SOCIAL HISTORY:
She works from home and does not engage in regular physical activity.

VITAL SIGNS:
Her BMI is 41 kg/m².

PHYSICAL EXAMINATION:
Vital signs and physical exam are unremarkable.

## Encounter 1: source 2020-07-15, practice 2024-03-11, outpatient, Gastroenterology

Chief complaint: Six-month history of abdominal bloating, foul-smelling greasy stools, and unintentional weight loss with lower extremity swelling.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Kwame Asante, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of abdominal bloating, foul-smelling greasy stools, and unintentional weight loss with lower extremity swelling.

HISTORY OF PRESENT ILLNESS:
The patient, status post Roux-en-Y gastric bypass with celiac disease and malabsorption syndrome, returns six months after her prior bariatric surgery evaluation. She reports frequent episodes of fatigue and mild lower extremity swelling.

PAST MEDICAL HISTORY:
Active Problem List:
- Morbid obesity (diagnosed 2020-01-15)
- Morbid obesity (status post surgical weight loss)
- Celiac disease
- Status post Roux-en-Y gastric bypass
- Postmenopausal status (2 years)
- Malabsorption syndrome

PAST SURGICAL HISTORY:
- Roux-en-Y gastric bypass 18 months prior to initial presentation
- Laparoscopic cholecystectomy 5 years prior

MEDICATIONS:
Home Medications:
- Multivitamin with minerals (bariatric formulation) 1 tablet daily (patient noncompliant)
- Calcium citrate with vitamin D 600 mg/400 IU three times daily (patient noncompliant)
- Vitamin B12 (sublingual) 1000 mcg daily (patient noncompliant)
- Iron sulfate 325 mg daily (patient noncompliant)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Sister with celiac disease
- Mother with osteoporosis
- Father with obesity and type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per year). Occupation: Remote administrative assistant.

PHYSICAL EXAMINATION:
Pale conjunctivae and trace bilateral ankle edema

LABORATORY DATA:
Hemoglobin of 10.2 g/dL and MCV 73 fL

OTHER STUDIES:
A D-xylose test shows low urinary excretion of the sugar following oral administration.

## Encounter 2: source 2021-07-15, practice 2025-03-10, outpatient, Rheumatology

Chief complaint: Six-month history of diffuse musculoskeletal pain and generalized weakness with difficulty climbing stairs or rising from seated position.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Elena Volkov, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of diffuse musculoskeletal pain and generalized weakness with difficulty climbing stairs or rising from seated position.

HISTORY OF PRESENT ILLNESS:
The patient returns for evaluation with a history of Roux-en-Y gastric bypass and celiac disease. She reports feeling fatigued and has noticed difficulty climbing stairs and getting up from a seated position. Since the surgery, she has lost 150 pounds and has not been compliant with her multivitamin supplementation.

PAST MEDICAL HISTORY:
Active Problem List:
- Morbid obesity (diagnosed 2020-01-15)
- Celiac disease (diagnosed 2020-07-15)
- Morbid obesity (status post surgical weight loss)
- Status post Roux-en-Y gastric bypass
- Postmenopausal status (2 years)
- Malabsorption syndrome

She has a history of morbid obesity.

PAST SURGICAL HISTORY:
She underwent Roux-en-Y gastric bypass surgery 18 months ago.

MEDICATIONS:
Home Medications:
- Multivitamin with minerals (bariatric formulation) 1 tablet daily (patient noncompliant)
- Calcium citrate with vitamin D 600 mg/400 IU three times daily (patient noncompliant)
- Vitamin B12 (sublingual) 1000 mcg daily (patient noncompliant)
- Iron sulfate 325 mg daily (patient noncompliant)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Sister with celiac disease
- Mother with osteoporosis
- Father with obesity and type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per year). Occupation: Remote administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, she has tenderness to palpation over her long bones and pelvis.

LABORATORY DATA:
Laboratory results are as follows:

- Serum calcium: 9.2 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum phosphorus: 2.9 mg/dL (normal: 2.5-4.5 mg/dL)
- Alkaline phosphatase: 140 U/L (normal: 30-120 U/L)
- Serum 25-hydroxyvitamin D: Pending

## Encounter 3: source 2022-07-15, practice 2026-03-09, outpatient, Neurology

Chief complaint: Six-month history of difficulty walking, balance problems in the dark, and pins and needles sensation in hands and feet.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of difficulty walking, balance problems in the dark, and pins and needles sensation in hands and feet.

HISTORY OF PRESENT ILLNESS:
This 49-year-old female with Roux-en-Y gastric bypass and malabsorptive conditions was previously evaluated for musculoskeletal weakness. She returns with a 6-month history of difficulty walking and a sensation of pins and needles in her hands and feet. She reports a gradual onset of balance problems, especially in the dark.

PAST MEDICAL HISTORY:
Active Problem List:
- Morbid obesity (diagnosed 2020-01-15)
- Celiac disease (diagnosed 2020-07-15)
- Osteomalacia (diagnosed 2021-07-15)
- Morbid obesity (status post surgical weight loss)
- Status post Roux-en-Y gastric bypass
- Postmenopausal status (2 years)
- Malabsorption syndrome

Her past medical history is significant for morbid obesity.

PAST SURGICAL HISTORY:
Roux-en-Y gastric bypass surgery 2 years ago

MEDICATIONS:
She admits to noncompliance with her multivitamin supplements.

Home Medications:
- Multivitamin with minerals (bariatric formulation) 1 tablet daily (patient noncompliant)
- Calcium citrate with vitamin D 600 mg/400 IU three times daily (patient noncompliant)
- Vitamin B12 (sublingual) 1000 mcg daily (patient noncompliant)
- Iron sulfate 325 mg daily (patient noncompliant)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Sister with celiac disease
- Mother with osteoporosis
- Father with obesity and type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per year). Occupation: Remote administrative assistant.

PHYSICAL EXAMINATION:
Physical examination reveals decreased vibratory sensation in the lower extremities, a positive Romberg sign, and pallor.

LABORATORY DATA:
Laboratory studies show:

- Hemoglobin: 10.2 g/dL (Normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 105 fL (Normal: 80-100 fL)
- White blood cell count: 4.8 x 10^9/L (Normal: 4-11 x 10^9/L)
- Platelet count: 200 x 10^9/L (Normal: 150-450 x 10^9/L)
- Serum copper: 40 mcg/dL (Normal: 70-140 mcg/dL)
- Serum zinc: 150 mcg/dL (Normal: 60-120 mcg/dL)
