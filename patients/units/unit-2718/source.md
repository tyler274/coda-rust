# synthetic_hospital patient 2718

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex F, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-04-26 (Tue) | inpatient | General Surgery |
| 1 | 2023-01-15 | 2025-04-21 (Mon) | outpatient | Internal Medicine |
| 2 | 2023-02-15 | 2025-05-19 (Mon) | outpatient | Endocrinology |
| 3 | 2023-07-15 | 2025-10-21 (Tue) | outpatient | Gastroenterology |
| 4 | 2023-09-15 | 2025-12-29 (Mon) | outpatient | Hematology |
| 5 | 2023-10-15 | 2026-01-19 (Mon) | outpatient | Hematology |
| 6 | 2023-11-15 | 2026-02-27 (Fri) | ed | Emergency Medicine |
| 7 | 2023-12-15 | 2026-03-30 (Mon) | icu | Neurology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Administrative Assistant",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Type 2 diabetes mellitus",
  "Father: Hypertension",
  "Maternal aunt: Morbid obesity"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "1000 mcg PO daily",
   "name": "Cyanocobalamin"
  },
  {
   "dose": "600 mg/400 units PO BID",
   "name": "Calcium carbonate with vitamin D"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Multivitamin"
  }
 ],
 "surgical_history": [
  "Laparoscopic Roux-en-Y gastric bypass (age 42)",
  "Emergency exploratory laparotomy with repair of anastomotic leak"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus (poorly controlled)",
  "Hypertension",
  "Morbid obesity, status post Roux-en-Y gastric bypass",
  "Vitamin B12 deficiency",
  "Cushing's syndrome",
  "Recurrent sinusitis"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Vitamin B12 deficiency", "Anastomotic leak (postoperative)", "Anastomotic stricture", "Obesity", "Cavernous sinus thrombosis", "Streptococcal infection or syphilis", "Cushing syndrome"]
Comorbidities: ["Vitamin B12 deficiency", "Status post Roux-en-Y gastric bypass surgery", "Morbid obesity", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2022-04-26, inpatient, General Surgery

Chief complaint: Increasing abdominal pain, fever, and shortness of breath 5 days after laparoscopic Roux-en-Y gastric bypass.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: General Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Increasing abdominal pain, fever, and shortness of breath 5 days after laparoscopic Roux-en-Y gastric bypass.

HISTORY OF PRESENT ILLNESS:
A 42-year-old woman presents to the emergency department with a 2-day history of increasing abdominal pain, fever, and shortness of breath. She underwent a laparoscopic Roux-en-Y gastric bypass for morbid obesity 5 days ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus (poorly controlled)
- Hypertension
- Morbid obesity, status post Roux-en-Y gastric bypass
- Vitamin B12 deficiency
- Cushing's syndrome
- Recurrent sinusitis

PAST SURGICAL HISTORY:
- Laparoscopic Roux-en-Y gastric bypass (age 42)
- Emergency exploratory laparotomy with repair of anastomotic leak

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Cyanocobalamin 1000 mcg PO daily
- Calcium carbonate with vitamin D 600 mg/400 units PO BID
- Multivitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus
- Father: Hypertension
- Maternal aunt: Morbid obesity

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Assistant.

VITAL SIGNS:
Temperature is 38.7°C (101.7°F), heart rate is 110 beats per minute, respiratory rate is 22 breaths per minute, and blood pressure is 135/85 mmHg

PHYSICAL EXAMINATION:
Her abdomen is distended with diffuse tenderness on palpation, but no rebound tenderness or guarding.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 14,000 /µL
- Hemoglobin: 12.5 g/dL
- Platelet count: 250,000 /µL
- Serum creatinine: 1.0 mg/dL
- Blood urea nitrogen: 18 mg/dL
- Serum glucose: 180 mg/dL

IMAGING:
An upright chest X-ray shows no free air under the diaphragm.

## Encounter 1: source 2023-01-15, practice 2025-04-21, outpatient, Internal Medicine

Chief complaint: Follow-up visit for weight management with frustration regarding minimal weight loss despite diet and exercise efforts over the past 6 months.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit for weight management with frustration regarding minimal weight loss despite diet and exercise efforts over the past 6 months.

HISTORY OF PRESENT ILLNESS:
The patient, status post Roux-en-Y gastric bypass complicated by anastomotic leak in 2020, returns for follow-up of weight management. She reports a 6-month history of attempting to lose weight with minimal success. She has been following a low-calorie diet and has joined a local gym, but she has only lost 2 kg over the last 6 months. She expresses frustration and is seeking advice on how to improve her weight loss efforts.

PAST MEDICAL HISTORY:
Active Problem List:
- Anastomotic leak (postoperative) (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus (poorly controlled)
- Hypertension
- Morbid obesity, status post Roux-en-Y gastric bypass
- Vitamin B12 deficiency
- Cushing's syndrome
- Recurrent sinusitis

PAST SURGICAL HISTORY:
- Laparoscopic Roux-en-Y gastric bypass (age 42)
- Emergency exploratory laparotomy with repair of anastomotic leak

MEDICATIONS:
Her current medications include metformin and lisinopril.

Home Medications:
- Cyanocobalamin 1000 mcg PO daily
- Calcium carbonate with vitamin D 600 mg/400 units PO BID
- Multivitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus
- Father: Hypertension
- Maternal aunt: Morbid obesity

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Assistant.

VITAL SIGNS:
On examination, her blood pressure is 132/86 mmHg

PHYSICAL EXAMINATION:
BMI is 34 kg/m^2, and waist circumference is 102 cm.

LABORATORY DATA:
Her recent laboratory tests show:

- Fasting plasma glucose: 146 mg/dL
- HbA1c: 7.2%
- Total cholesterol: 210 mg/dL
- HDL cholesterol: 38 mg/dL
- LDL cholesterol: 130 mg/dL
- Triglycerides: 220 mg/dL

## Encounter 2: source 2023-02-15, practice 2025-05-19, outpatient, Endocrinology

Chief complaint: Ongoing back pain after twisting to reach for a heavy object, with several months of progressive central weight gain and abdominal obesity.

ENCOUNTER NOTE
Date: 2023-02-15
Provider: Dr. Aisha Patel, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Ongoing back pain after twisting to reach for a heavy object, with several months of progressive central weight gain and abdominal obesity.

HISTORY OF PRESENT ILLNESS:
The patient returns for Endocrinology evaluation one month after her weight management follow-up, in the context of her history of Roux-en-Y gastric bypass complicated by anastomotic leak. She presents with back pain that began after she twisted to reach for a heavy object on a high shelf. She denies major trauma but describes a "popping" sensation in her back followed by severe discomfort. Over the last several months, she has noticed a significant increase in weight, particularly around her midsection, which she attributes to recent life stressors. Her energy levels have declined, and she finds herself easily fatigued.

PAST MEDICAL HISTORY:
Active Problem List:
- Anastomotic leak (postoperative) (diagnosed 2020-01-15)
- Obesity (diagnosed 2023-01-15)
- Type 2 Diabetes Mellitus (poorly controlled)
- Hypertension
- Morbid obesity, status post Roux-en-Y gastric bypass
- Vitamin B12 deficiency
- Cushing's syndrome
- Recurrent sinusitis

Past medical history is non-contributory.

PAST SURGICAL HISTORY:
- Laparoscopic Roux-en-Y gastric bypass (age 42)
- Emergency exploratory laparotomy with repair of anastomotic leak

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Cyanocobalamin 1000 mcg PO daily
- Calcium carbonate with vitamin D 600 mg/400 units PO BID
- Multivitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus
- Father: Hypertension
- Maternal aunt: Morbid obesity

SOCIAL HISTORY:
She does not consume alcohol, tobacco, or illicit drugs.

PHYSICAL EXAMINATION:
Examination reveals scattered purplish markings across her lower abdomen and upper thighs. On evaluation, she has tenderness to palpation over the thoracolumbar spine and limited range of motion due to pain.

LABORATORY DATA:
Laboratory findings are as follows:
Cortisol (8 AM)                 29 µg/dL
Fasting glucose               132 mg/dL
Calcium                       9.2 mg/dL
Alkaline phosphatase            138 U/L
24-hour urinary free cortisol   388 µg/day

IMAGING:
Imaging shows an anterior wedge compression fracture at L1 without evidence of malignancy or infection. A dual-energy X-ray absorptiometry (DEXA) scan reveals decreased bone mineral density.

## Encounter 3: source 2023-07-15, practice 2025-10-21, outpatient, Gastroenterology

Chief complaint: Six-month history of bloating, flatulence, and watery diarrhea with 10-pound unintentional weight loss.

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. James O'Brien, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of bloating, flatulence, and watery diarrhea with 10-pound unintentional weight loss.

HISTORY OF PRESENT ILLNESS:
This 48-year-old female with a history of Roux-en-Y gastric bypass complicated by anastomotic leak and recent Cushing's syndrome diagnosis presents with gastrointestinal symptoms. She reports a weight loss of 10 pounds during this period despite no changes in diet or exercise.

PAST MEDICAL HISTORY:
Active Problem List:
- Anastomotic leak (postoperative) (diagnosed 2020-01-15)
- Obesity (diagnosed 2023-01-15)
- Cushing syndrome (diagnosed 2023-02-15)
- Type 2 Diabetes Mellitus (poorly controlled)
- Hypertension
- Morbid obesity, status post Roux-en-Y gastric bypass
- Vitamin B12 deficiency
- Cushing's syndrome
- Recurrent sinusitis

Her medical history is significant for type 2 diabetes mellitus, which is well-controlled with metformin.

PAST SURGICAL HISTORY:
She underwent a Roux-en-Y gastric bypass procedure 2 years ago for morbid obesity.

MEDICATIONS:
She takes metformin.

Home Medications:
- Lisinopril 20 mg PO daily
- Cyanocobalamin 1000 mcg PO daily
- Calcium carbonate with vitamin D 600 mg/400 units PO BID
- Multivitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus
- Father: Hypertension
- Maternal aunt: Morbid obesity

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Assistant.

REVIEW OF SYSTEMS:
She denies any fever, vomiting, or blood in her stool.

PHYSICAL EXAMINATION:
Physical examination reveals a mildly distended abdomen with diffuse tenderness to palpation but no rebound or guarding. Bowel sounds are hyperactive.

LABORATORY DATA:
Laboratory tests reveal the following:

- Hemoglobin: 11.2 g/dL (Normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 74 fL (Normal: 80-100 fL)
- Serum vitamin B12: 150 pg/mL (Normal: 200-900 pg/mL)
- Fasting glucose: 98 mg/dL (Normal: 70-100 mg/dL)
- Stool studies: Negative for ova, parasites, and Clostridioides difficile toxin

## Encounter 4: source 2023-09-15, practice 2025-12-29, outpatient, Hematology

Chief complaint: Three-month history of progressive fatigue, generalized weakness, and sore tongue.

ENCOUNTER NOTE
Date: 2023-09-15
Provider: Dr. Priya Sharma, MD
Department: Hematology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of progressive fatigue, generalized weakness, and sore tongue.

HISTORY OF PRESENT ILLNESS:
This 48-year-old female with a history of Roux-en-Y gastric bypass complicated by anastomotic leak and recent anastomotic stricture presents to the Hematology clinic. She reports a 3-month history of fatigue, weakness, and a sore tongue.

PAST MEDICAL HISTORY:
Active Problem List:
- Anastomotic leak (postoperative) (diagnosed 2020-01-15)
- Obesity (diagnosed 2023-01-15)
- Cushing syndrome (diagnosed 2023-02-15)
- Anastomotic stricture (diagnosed 2023-07-15)
- Type 2 Diabetes Mellitus (poorly controlled)
- Hypertension
- Morbid obesity, status post Roux-en-Y gastric bypass
- Vitamin B12 deficiency
- Cushing's syndrome
- Recurrent sinusitis

She has a past medical history of type 2 diabetes mellitus and hypertension.

PAST SURGICAL HISTORY:
- Laparoscopic Roux-en-Y gastric bypass (age 42)
- Emergency exploratory laparotomy with repair of anastomotic leak

MEDICATIONS:
, for which she takes metformin and lisinopril, respectively.

Home Medications:
- Cyanocobalamin 1000 mcg PO daily
- Calcium carbonate with vitamin D 600 mg/400 units PO BID
- Multivitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus
- Father: Hypertension
- Maternal aunt: Morbid obesity

SOCIAL HISTORY:
She does not consume alcohol and has a diet low in animal products.

PHYSICAL EXAMINATION:
On physical examination, she appears pale, and her tongue is beefy red and smooth. Neurologic examination reveals decreased vibration sense in her lower extremities and a positive Romberg sign.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 9.8 g/dL (Normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 115 fL (Normal: 80-100 fL)
- Serum folate: 4 ng/mL (Normal: >5.4 ng/mL)
- Serum cobalamin (vitamin B12): 150 pg/mL (Normal: 200-900 pg/mL)
- Serum homocysteine: Elevated
- Serum methylmalonic acid: Elevated

## Encounter 5: source 2023-10-15, practice 2026-01-19, outpatient, Hematology

Chief complaint: Persistent three-month history of fatigue, weakness, and sore tongue with associated 5 kg weight loss despite supplementation.

ENCOUNTER NOTE
Date: 2023-10-15
Provider: Dr. Priya Sharma, MD
Department: Hematology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent three-month history of fatigue, weakness, and sore tongue with associated 5 kg weight loss despite supplementation.

HISTORY OF PRESENT ILLNESS:
The patient with Roux-en-Y gastric bypass and Cushing's syndrome returns for Hematology follow-up, presenting with a 3-month history of fatigue, weakness, and a sore tongue. She reports a weight loss of 5 kg during this period.

PAST MEDICAL HISTORY:
Active Problem List:
- Anastomotic leak (postoperative) (diagnosed 2020-01-15)
- Obesity (diagnosed 2023-01-15)
- Cushing syndrome (diagnosed 2023-02-15)
- Anastomotic stricture (diagnosed 2023-07-15)
- Vitamin B12 deficiency (diagnosed 2023-09-15)
- Type 2 Diabetes Mellitus (poorly controlled)
- Hypertension
- Morbid obesity, status post Roux-en-Y gastric bypass
- Cushing's syndrome
- Recurrent sinusitis

PAST SURGICAL HISTORY:
- Laparoscopic Roux-en-Y gastric bypass (age 42)
- Emergency exploratory laparotomy with repair of anastomotic leak

MEDICATIONS:
She takes metformin.

Home Medications:
- Lisinopril 20 mg PO daily
- Cyanocobalamin 1000 mcg PO daily
- Calcium carbonate with vitamin D 600 mg/400 units PO BID
- Multivitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus
- Father: Hypertension
- Maternal aunt: Morbid obesity

SOCIAL HISTORY:
She does not consume alcohol

PHYSICAL EXAMINATION:
On physical examination, she appears pale, and her tongue is beefy red and smooth. Her neurological examination is normal.

LABORATORY DATA:
- Hemoglobin: 9.8 g/dL (Normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 115 fL (Normal: 80-100 fL)
- Reticulocyte count: 1.5% (Normal: 0.5-1.5%)
- Serum B12: 180 pg/mL (Normal: 200-900 pg/mL)
- Serum folate: 12 ng/mL (Normal: 3-17 ng/mL)

## Encounter 6: source 2023-11-15, practice 2026-02-27, ed, Emergency Medicine

Chief complaint: Two-day history of high fever, chills, and painful erythematous rash on right lower leg that began as a small area and rapidly expanded.

ENCOUNTER NOTE
Date: 2023-11-15
Provider: Dr. Fatima Al-Farsi, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-day history of high fever, chills, and painful erythematous rash on right lower leg that began as a small area and rapidly expanded.

HISTORY OF PRESENT ILLNESS:
This 48-year-old female with a history of poorly controlled Type 2 Diabetes Mellitus, Cushing's syndrome, status post Roux-en-Y gastric bypass, and recent evaluation for persistent Vitamin B12 deficiency presents to the Emergency Department with a two-day history of high fever, chills, and painful erythematous rash on the right lower leg. She reports that the rash began as a small red area but has rapidly expanded.

PAST MEDICAL HISTORY:
Active Problem List:
- Anastomotic leak (postoperative) (diagnosed 2020-01-15)
- Obesity (diagnosed 2023-01-15)
- Cushing syndrome (diagnosed 2023-02-15)
- Anastomotic stricture (diagnosed 2023-07-15)
- Vitamin B12 deficiency (diagnosed 2023-09-15)
- Type 2 Diabetes Mellitus (poorly controlled)
- Hypertension
- Morbid obesity, status post Roux-en-Y gastric bypass
- Cushing's syndrome
- Recurrent sinusitis

Her past medical history is significant for type 2 diabetes mellitus, which is controlled with metformin.

PAST SURGICAL HISTORY:
- Laparoscopic Roux-en-Y gastric bypass (age 42)
- Emergency exploratory laparotomy with repair of anastomotic leak

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Cyanocobalamin 1000 mcg PO daily
- Calcium carbonate with vitamin D 600 mg/400 units PO BID
- Multivitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus
- Father: Hypertension
- Maternal aunt: Morbid obesity

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Assistant.

VITAL SIGNS:
On physical examination, her temperature is 38.7°C (101.7°F), heart rate is 102 beats per minute, and blood pressure is 135/85 mmHg.

PHYSICAL EXAMINATION:
Examination of her right lower leg reveals a bright red, warm, and tender area with raised, sharply demarcated borders. There is no purulent drainage or fluctuance. The remainder of the physical examination is unremarkable.

LABORATORY DATA:
Laboratory studies show:

- White blood cell count: 14,000/mm³ (normal 4,500-11,000/mm³)
- C-reactive protein: 25 mg/L (normal <10 mg/L)
- Blood cultures: No growth to date

## Encounter 7: source 2023-12-15, practice 2026-03-30, icu, Neurology

Chief complaint: Three-day history of progressive severe headache, fever, periorbital swelling, decreased vision, and diplopia following manipulation of a facial furuncle.

ENCOUNTER NOTE
Date: 2023-12-15
Provider: Dr. Robert Johnson, MD
Department: Neurology
Visit Type: icu

CHIEF COMPLAINT:
Three-day history of progressive severe headache, fever, periorbital swelling, decreased vision, and diplopia following manipulation of a facial furuncle.

HISTORY OF PRESENT ILLNESS:
This patient with poorly controlled diabetes, Cushing's syndrome, and recent emergency department evaluation reports severe, constant headache worsened with eye movement, decreased vision, and double vision. She recently had a pimple on her upper lip that she popped.

PAST MEDICAL HISTORY:
Active Problem List:
- Anastomotic leak (postoperative) (diagnosed 2020-01-15)
- Obesity (diagnosed 2023-01-15)
- Cushing syndrome (diagnosed 2023-02-15)
- Anastomotic stricture (diagnosed 2023-07-15)
- Vitamin B12 deficiency (diagnosed 2023-09-15)
- Streptococcal infection or syphilis (diagnosed 2023-11-15)
- Type 2 Diabetes Mellitus (poorly controlled)
- Hypertension
- Morbid obesity, status post Roux-en-Y gastric bypass
- Cushing's syndrome
- Recurrent sinusitis

She has a history of recurrent sinusitis.

PAST SURGICAL HISTORY:
- Laparoscopic Roux-en-Y gastric bypass (age 42)
- Emergency exploratory laparotomy with repair of anastomotic leak

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Cyanocobalamin 1000 mcg PO daily
- Calcium carbonate with vitamin D 600 mg/400 units PO BID
- Multivitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus
- Father: Hypertension
- Maternal aunt: Morbid obesity

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Assistant.

VITAL SIGNS:
Vital signs are: temperature 38.7°C, blood pressure 130/85 mmHg, heart rate 102 bpm, respiratory rate 18 breaths/min, and oxygen saturation 97% on room air

PHYSICAL EXAMINATION:
On examination, she has bilateral periorbital edema and erythema, chemosis, and proptosis. Her visual acuity is decreased bilaterally. Extraocular movements are painful and limited in all directions. Pupils are sluggishly reactive to light. There is decreased sensation over the forehead and nose. Fundoscopic examination reveals bilateral papilledema. No neck stiffness or photophobia is noted.

LABORATORY DATA:
Laboratory values:
- White blood cell count: 15,000 /μL
- Hemoglobin: 13.2 g/dL
- Platelet count: 250,000 /μL
- Blood glucose: 280 mg/dL
- C-reactive protein: 24 mg/L (normal <5 mg/L)
- Erythrocyte sedimentation rate: 55 mm/hr (normal 0-20 mm/hr)

IMAGING:
A contrast-enhanced MRI of the brain is ordered.
