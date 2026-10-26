# synthetic_hospital patient 1836

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 33 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-09-20 (Tue) | outpatient | Endocrinology |
| 1 | 2020-05-15 | 2023-01-16 (Mon) | outpatient | Hematology |
| 2 | 2020-11-15 | 2023-07-17 (Mon) | outpatient | Obstetrics & Gynecology |
| 3 | 2022-03-15 | 2024-11-11 (Mon) | ed | Emergency Medicine |
| 4 | 2022-11-15 | 2025-07-26 (Sat) | ed | Emergency Medicine |
| 5 | 2023-07-15 | 2026-03-17 (Tue) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Ceftriaxone (anaphylaxis)",
  "Sulfonamides (rash)",
  "Penicillin (hives)"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Software developer (remote/work-from-home)",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Mother: Rheumatoid arthritis",
  "Father: Asthma, seasonal allergies",
  "Sister: Crohn's disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "90 mcg, 2 inhalations every 4-6 hours PRN for wheezing/shortness of breath",
   "name": "Albuterol HFA inhaler"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Prenatal multivitamin"
  },
  {
   "dose": "325 mg (65 mg elemental iron) PO daily with food",
   "name": "Ferrous sulfate"
  },
  {
   "dose": "2000 international units PO daily",
   "name": "Cholecalciferol (Vitamin D3)"
  },
  {
   "dose": "1000 mcg PO daily",
   "name": "Cyanocobalamin (Vitamin B12)"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Crohn's disease (ileocolonic, currently mild)",
  "Asthma",
  "Urethral diverticulum",
  "History of recurrent urinary tract infections",
  "Vitamin D deficiency",
  "Vitamin B12 deficiency"
 ],
 "age_at_first_encounter": 33
}
```

Primary diagnoses: ["Acute pyelonephritis", "Anesthetic medication reaction", "Secondary hyperparathyroidism due to vitamin D deficiency", "Vitamin B12 deficiency", "Urethral diverticulum"]
Comorbidities: ["History of septic shock", "Crohn Disease", "Status post right hemicolectomy"]

## Encounter 0: source 2020-01-15, practice 2022-09-20, outpatient, Endocrinology

Chief complaint: Six-month history of progressive fatigue, muscle weakness, and generalized bone pain

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of progressive fatigue, muscle weakness, and generalized bone pain

PAST MEDICAL HISTORY:
Active Problem List:
- Crohn's disease (ileocolonic, currently mild)
- Asthma
- Urethral diverticulum
- History of recurrent urinary tract infections
- Vitamin D deficiency
- Vitamin B12 deficiency

She has a history of Crohn's disease.

PAST SURGICAL HISTORY:
She underwent an ileal resection two years ago.

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 inhalations every 4-6 hours PRN for wheezing/shortness of breath
- Prenatal multivitamin 1 tablet PO daily
- Ferrous sulfate 325 mg (65 mg elemental iron) PO daily with food
- Cholecalciferol (Vitamin D3) 2000 international units PO daily
- Cyanocobalamin (Vitamin B12) 1000 mcg PO daily

ALLERGIES:
Ceftriaxone (anaphylaxis), Sulfonamides (rash), Penicillin (hives)

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Father: Asthma, seasonal allergies
- Sister: Crohn's disease

SOCIAL HISTORY:
She works from home as a software developer and admits to spending most of her time indoors.

VITAL SIGNS:
Her blood pressure is 130/85 mmHg, heart rate is 78 bpm, and BMI is 22 kg/m^2.

PHYSICAL EXAMINATION:
On physical examination, she appears pale and reports tenderness on palpation of her lower limbs.

LABORATORY DATA:
Laboratory results are as follows:

- Serum calcium: 8.1 mg/dL (Normal: 8.5-10.2 mg/dL)
- Serum phosphate: 3.0 mg/dL (Normal: 2.5-4.5 mg/dL)
- Alkaline phosphatase: 120 U/L (Normal: 30-120 U/L)
- Serum 25-hydroxyvitamin D: 12 ng/mL (Normal: 20-50 ng/mL)
- Parathyroid hormone (PTH): 88 pg/mL (Normal: 10-65 pg/mL)

## Encounter 1: source 2020-05-15, practice 2023-01-16, outpatient, Hematology

Chief complaint: Six-month history of fatigue, intermittent palpitations, and recent painful glossitis

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Michael Rodriguez, MD
Department: Hematology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of fatigue, intermittent palpitations, and recent painful glossitis

PAST MEDICAL HISTORY:
Active Problem List:
- Secondary hyperparathyroidism due to vitamin D deficiency (diagnosed 2020-01-15)
- Crohn's disease (ileocolonic, currently mild)
- Asthma
- Urethral diverticulum
- History of recurrent urinary tract infections
- Vitamin D deficiency
- Vitamin B12 deficiency

She has a past medical history of Crohn's disease.

PAST SURGICAL HISTORY:
She underwent an ileal resection two years ago

MEDICATIONS:
She does not take any medications

Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 inhalations every 4-6 hours PRN for wheezing/shortness of breath
- Prenatal multivitamin 1 tablet PO daily
- Ferrous sulfate 325 mg (65 mg elemental iron) PO daily with food
- Cholecalciferol (Vitamin D3) 2000 international units PO daily
- Cyanocobalamin (Vitamin B12) 1000 mcg PO daily

ALLERGIES:
Ceftriaxone (anaphylaxis), Sulfonamides (rash), Penicillin (hives)

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Father: Asthma, seasonal allergies
- Sister: Crohn's disease

SOCIAL HISTORY:
Follows a strict vegetarian diet

VITAL SIGNS:
On physical examination, her vital signs are within normal limits.

PHYSICAL EXAMINATION:
Her skin appears pale

LABORATORY DATA:
A complete blood count reveals macrocytic anemia. Serum laboratory values are as follows:

- Hemoglobin: 9.8 g/dL (Normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 105 fL (Normal: 80-100 fL)
- Homocysteine: Elevated
- Methylmalonic acid (MMA): Elevated

## Encounter 2: source 2020-11-15, practice 2023-07-17, outpatient, Obstetrics & Gynecology

Chief complaint: Six-month history of dyspareunia and intermittent dysuria with sensation of fullness in anterior vaginal wall

ENCOUNTER NOTE
Date: 2020-11-15
Provider: Dr. Emily Watson, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of dyspareunia and intermittent dysuria with sensation of fullness in anterior vaginal wall

HISTORY OF PRESENT ILLNESS:
The patient is a 33-year-old female with Crohn's disease and recent evaluations for vitamin deficiencies who presents with a 6-month history of painful intercourse and intermittent episodes of dysuria. She also reports a sensation of fullness in the anterior vaginal wall.

PAST MEDICAL HISTORY:
Active Problem List:
- Secondary hyperparathyroidism due to vitamin D deficiency (diagnosed 2020-01-15)
- Vitamin B12 deficiency (diagnosed 2020-05-15)
- Crohn's disease (ileocolonic, currently mild)
- Asthma
- Urethral diverticulum
- History of recurrent urinary tract infections
- Vitamin D deficiency

Her past medical history is significant for recurrent urinary tract infections.

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 inhalations every 4-6 hours PRN for wheezing/shortness of breath
- Prenatal multivitamin 1 tablet PO daily
- Ferrous sulfate 325 mg (65 mg elemental iron) PO daily with food
- Cholecalciferol (Vitamin D3) 2000 international units PO daily
- Cyanocobalamin (Vitamin B12) 1000 mcg PO daily

ALLERGIES:
Ceftriaxone (anaphylaxis), Sulfonamides (rash), Penicillin (hives)

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Father: Asthma, seasonal allergies
- Sister: Crohn's disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Software developer (remote/work-from-home).

PHYSICAL EXAMINATION:
On physical examination, a tender mass is palpable on the anterior vaginal wall, and expression of the mass yields a small amount of purulent discharge.

LABORATORY DATA:
Urinalysis is unremarkable.

## Encounter 3: source 2022-03-15, practice 2024-11-11, ed, Emergency Medicine

Chief complaint: Three days of fever, chills, and left-sided flank pain with one week of dysuria and frequency

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. James Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Three days of fever, chills, and left-sided flank pain with one week of dysuria and frequency

HISTORY OF PRESENT ILLNESS:
The patient with Crohn's disease and urethral diverticulum presents with fever, chills, and left-sided flank pain for three days. She also reports dysuria and urinary frequency over the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Secondary hyperparathyroidism due to vitamin D deficiency (diagnosed 2020-01-15)
- Vitamin B12 deficiency (diagnosed 2020-05-15)
- Urethral diverticulum (diagnosed 2020-11-15)
- Crohn's disease (ileocolonic, currently mild)
- Asthma
- History of recurrent urinary tract infections
- Vitamin D deficiency

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 inhalations every 4-6 hours PRN for wheezing/shortness of breath
- Prenatal multivitamin 1 tablet PO daily
- Ferrous sulfate 325 mg (65 mg elemental iron) PO daily with food
- Cholecalciferol (Vitamin D3) 2000 international units PO daily
- Cyanocobalamin (Vitamin B12) 1000 mcg PO daily

ALLERGIES:
Ceftriaxone (anaphylaxis), Sulfonamides (rash), Penicillin (hives)

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Father: Asthma, seasonal allergies
- Sister: Crohn's disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Software developer (remote/work-from-home).

VITAL SIGNS:
Her temperature is 38.7°C, blood pressure is 110/68 mm Hg, heart rate is 102/min, and respiratory rate is 18/min.

PHYSICAL EXAMINATION:
She has mild left costovertebral angle tenderness on percussion.

LABORATORY DATA:
Urinalysis shows pyuria and bacteriuria, and urine culture grows Escherichia coli.

PATHOLOGY:
Imaging findings from a renal biopsy are shown in the picture below.

## Encounter 4: source 2022-11-15, practice 2025-07-26, ed, Emergency Medicine

Chief complaint: Two-day history of fever, dysuria, and left-sided flank pain in patient with history of pregnancy-related UTIs

ENCOUNTER NOTE
Date: 2022-11-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-day history of fever, dysuria, and left-sided flank pain in patient with history of pregnancy-related UTIs

HISTORY OF PRESENT ILLNESS:
This 35-year-old female with a known urethral diverticulum and history of acute pyelonephritis in March 2022 returns to the emergency department with a 2-day history of fever, dysuria, and left-sided flank pain. She reports a history of recurrent urinary tract infections (UTIs) during her first trimester, for which she was treated with nitrofurantoin. On further questioning, she mentions experiencing increased urinary frequency and urgency over the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Secondary hyperparathyroidism due to vitamin D deficiency (diagnosed 2020-01-15)
- Vitamin B12 deficiency (diagnosed 2020-05-15)
- Urethral diverticulum (diagnosed 2020-11-15)
- Acute pyelonephritis (diagnosed 2022-03-15)
- Crohn's disease (ileocolonic, currently mild)
- Asthma
- History of recurrent urinary tract infections
- Vitamin D deficiency

PAST SURGICAL HISTORY:
Her past surgical history is notable for a laparoscopic appendectomy 5 years ago, with an uncomplicated recovery.

MEDICATIONS:
She has been compliant with her prenatal vitamins and iron supplementation, but denies any recent medication changes.

Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 inhalations every 4-6 hours PRN for wheezing/shortness of breath
- Prenatal multivitamin 1 tablet PO daily
- Ferrous sulfate 325 mg (65 mg elemental iron) PO daily with food
- Cholecalciferol (Vitamin D3) 2000 international units PO daily
- Cyanocobalamin (Vitamin B12) 1000 mcg PO daily

ALLERGIES:
Ceftriaxone (anaphylaxis), Sulfonamides (rash), Penicillin (hives)

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Father: Asthma, seasonal allergies
- Sister: Crohn's disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Software developer (remote/work-from-home).

VITAL SIGNS:
Physical examination reveals a temperature of 38.5°C (101.3°F), blood pressure of 110/70 mm Hg, and pulse of 98 beats/min.

PHYSICAL EXAMINATION:
Tenderness is noted on palpation of the left costovertebral angle.

LABORATORY DATA:
Laboratory studies reveal a white blood cell count of 14,000/mm³, and a urinalysis positive for leukocyte esterase and nitrites. Urine culture grows Escherichia coli .

IMAGING:
Renal ultrasound shows no evidence of hydronephrosis or renal stones.

## Encounter 5: source 2023-07-15, practice 2026-03-17, icu, Emergency Medicine

Chief complaint: Sudden throat tightness, dizziness, and impending doom during intravenous antibiotic infusion for right-sided pyelonephritis

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Sudden throat tightness, dizziness, and impending doom during intravenous antibiotic infusion for right-sided pyelonephritis

HISTORY OF PRESENT ILLNESS:
The patient is a 36-year-old female with a history of urethral diverticulum and recurrent urinary tract infections, including prior episodes of pyelonephritis in 2022, who presents with symptoms for the past 2 days. She is diagnosed with pyelonephritis and is started on intravenous ceftriaxone after confirming no prior history of reaction to this drug. Ten minutes into the infusion, she suddenly complains of throat tightness, dizziness, and a sensation of impending doom. Her voice becomes hoarse, and she begins to cough forcefully. She then develops generalized flushing, and her face and lips appear swollen.

PAST MEDICAL HISTORY:
Active Problem List:
- Secondary hyperparathyroidism due to vitamin D deficiency (diagnosed 2020-01-15)
- Vitamin B12 deficiency (diagnosed 2020-05-15)
- Urethral diverticulum (diagnosed 2020-11-15)
- Acute pyelonephritis (diagnosed 2022-03-15)
- Crohn's disease (ileocolonic, currently mild)
- Asthma
- History of recurrent urinary tract infections
- Vitamin D deficiency

She has had multiple urinary tract infections in the past.

MEDICATIONS:
She takes intravenous ceftriaxone.

Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 inhalations every 4-6 hours PRN for wheezing/shortness of breath
- Prenatal multivitamin 1 tablet PO daily
- Ferrous sulfate 325 mg (65 mg elemental iron) PO daily with food
- Cholecalciferol (Vitamin D3) 2000 international units PO daily
- Cyanocobalamin (Vitamin B12) 1000 mcg PO daily

ALLERGIES:
Trimethoprim-sulfamethoxazole and ciprofloxacin

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Father: Asthma, seasonal allergies
- Sister: Crohn's disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Software developer (remote/work-from-home).

VITAL SIGNS:
On presentation, she is febrile to 38.9°C (102.0°F). Vital signs show a temperature of 37.2°C (99°F), pulse of 132/min, respirations of 28/min, and blood pressure of 68/42 mm Hg. Her oxygen saturation drops to 91% on room air.

PHYSICAL EXAMINATION:
Physical examination reveals periorbital and lip edema, diffuse erythematous urticarial plaques over her chest and arms, and bilateral expiratory wheezes and inspiratory stridor. She appears anxious and is using accessory muscles to breathe. Jugular venous pressure is not elevated, and cardiac exam reveals tachycardia without murmurs. There is no peripheral edema or signs of fluid overload.

LABORATORY DATA:
Urinalysis shows pyuria, positive leukocyte esterase, and nitrites

ASSESSMENT AND PLAN:
Pyelonephritis

Started on intravenous ceftriaxone after confirming no prior history of reaction to this drug. She is admitted to the intensive care unit for further monitoring.
