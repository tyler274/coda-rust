# synthetic_hospital patient 2822

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex F, Non-Hispanic White, Medicare Part B.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-05-26 (Fri) | outpatient | Neurology |
| 1 | 2020-04-15 | 2023-09-04 (Mon) | outpatient | Psychiatry |
| 2 | 2021-01-15 | 2024-06-10 (Mon) | outpatient | Geriatrics |
| 3 | 2021-07-15 | 2024-12-05 (Thu) | ed | Emergency Medicine |
| 4 | 2022-01-15 | 2025-05-26 (Mon) | outpatient | Palliative Care |
| 5 | 2022-07-15 | 2025-12-04 (Thu) | ed | Emergency Medicine |
| 6 | 2022-09-15 | 2026-01-24 (Sat) | inpatient | Internal Medicine |
| 7 | 2023-01-15 | 2026-06-06 (Sat) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies",
  "Contrast dye (iodinated): unknown"
 ],
 "insurance": "Medicare Part B",
 "occupation": "Retired elementary school teacher",
 "alcohol_use": "2-3 glasses of wine daily",
 "family_history": [
  "Mother: Breast cancer (age 72)",
  "Father: Type 2 diabetes mellitus and hypertension",
  "Maternal aunt: Alzheimer's disease",
  "Brother: Coronary artery disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 30 years ago (1 pack-year)",
 "home_medications": [
  {
   "dose": "24 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "sliding scale with meals",
   "name": "Insulin lispro"
  },
  {
   "dose": "1000 mg oral twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg oral once daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg oral once daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg oral once daily",
   "name": "Aspirin"
  },
  {
   "dose": "650 mg oral every 6 hours as needed for arthritis pain",
   "name": "Acetaminophen"
  },
  {
   "dose": "600 mg/400 IU oral twice daily",
   "name": "Calcium carbonate with vitamin D"
  }
 ],
 "surgical_history": [
  "Modified radical mastectomy with axillary lymph node dissection (5 years ago)",
  "Open reduction and internal fixation of right femoral neck fracture (2 years ago)",
  "Percutaneous coronary intervention with drug-eluting stent placement (10 years ago)"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus (25-year history, poorly controlled)",
  "Hypertension",
  "Metastatic breast cancer (ER/PR positive, HER2 negative)",
  "Coronary artery disease (status post myocardial infarction)",
  "Alzheimer's disease",
  "Osteoarthritis",
  "Osteoporosis with history of right femoral neck fracture"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Delayed circumcision pending patient assent", "Malignancy-associated hypercalcemia", "Septic shock", "Adult physical abuse", "Transfusion-associated circulatory overload", "Alzheimer's disease", "Major depressive disorder", "Posterior vitreous detachment"]
Comorbidities: ["Anemia", "Dehydration", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2023-05-26, outpatient, Neurology

Chief complaint: Brought to clinic by her daughter due to concerns about her mother's memory and increasing forgetfulness over the past year.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Brought to clinic by her daughter due to concerns about her mother's memory and increasing forgetfulness over the past year.

HISTORY OF PRESENT ILLNESS:
Over the past year, she has noticed her mother becoming increasingly forgetful, misplacing items, and struggling to recall recent events.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus (25-year history, poorly controlled)
- Hypertension
- Metastatic breast cancer (ER/PR positive, HER2 negative)
- Coronary artery disease (status post myocardial infarction)
- Alzheimer's disease
- Osteoarthritis
- Osteoporosis with history of right femoral neck fracture

The patient has a past medical history of hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Modified radical mastectomy with axillary lymph node dissection (5 years ago)
- Open reduction and internal fixation of right femoral neck fracture (2 years ago)
- Percutaneous coronary intervention with drug-eluting stent placement (10 years ago)

MEDICATIONS:
Her medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale with meals
- Aspirin 81 mg oral once daily
- Acetaminophen 650 mg oral every 6 hours as needed for arthritis pain
- Calcium carbonate with vitamin D 600 mg/400 IU oral twice daily

ALLERGIES:
No known drug allergies, Contrast dye (iodinated): unknown

FAMILY HISTORY:
- Mother: Breast cancer (age 72)
- Father: Type 2 diabetes mellitus and hypertension
- Maternal aunt: Alzheimer's disease
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 30 years ago (1 pack-year). Alcohol: 2-3 glasses of wine daily. Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
On examination, the patient is alert but appears mildly disoriented to time. She is unable to recall three objects after five minutes and has difficulty with performing serial 7s. Her Mini-Mental State Examination (MMSE) score is 20 out of 30, indicating mild cognitive impairment.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin A1c: 7.2%
- Serum sodium: 140 mEq/L
- Serum potassium: 4.1 mEq/L
- Serum creatinine: 1.0 mg/dL
- Fasting glucose: 154 mg/dL
- Thyroid-stimulating hormone (TSH): 2.5 μIU/mL

IMAGING:
A CT scan of the head shows generalized cerebral atrophy without focal lesions.

## Encounter 1: source 2020-04-15, practice 2023-09-04, outpatient, Psychiatry

Chief complaint: Three-month history of difficulty falling asleep, frequent nighttime awakenings, loss of interest in hobbies, and feelings of hopelessness.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of difficulty falling asleep, frequent nighttime awakenings, loss of interest in hobbies, and feelings of hopelessness.

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old female with Alzheimer's disease diagnosed at her January 2020 neurology visit, along with Type 2 Diabetes Mellitus, metastatic breast cancer, and coronary artery disease, who presents to the clinic with a three-month history of difficulty falling asleep and frequent nighttime awakenings. She reports feeling tired during the day but cannot take naps. She has lost interest in her hobbies and has been feeling hopeless about the future. She denies any recent stressors or changes in her life.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer's disease (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus (25-year history, poorly controlled)
- Hypertension
- Metastatic breast cancer (ER/PR positive, HER2 negative)
- Coronary artery disease (status post myocardial infarction)
- Osteoarthritis
- Osteoporosis with history of right femoral neck fracture

Her past medical history includes hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Modified radical mastectomy with axillary lymph node dissection (5 years ago)
- Open reduction and internal fixation of right femoral neck fracture (2 years ago)
- Percutaneous coronary intervention with drug-eluting stent placement (10 years ago)

MEDICATIONS:
Her medications include metformin and lisinopril.

Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale with meals
- Atorvastatin 40 mg oral once daily
- Aspirin 81 mg oral once daily
- Acetaminophen 650 mg oral every 6 hours as needed for arthritis pain
- Calcium carbonate with vitamin D 600 mg/400 IU oral twice daily

ALLERGIES:
No known drug allergies, Contrast dye (iodinated): unknown

FAMILY HISTORY:
- Mother: Breast cancer (age 72)
- Father: Type 2 diabetes mellitus and hypertension
- Maternal aunt: Alzheimer's disease
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 30 years ago (1 pack-year). Alcohol: 2-3 glasses of wine daily. Occupation: Retired elementary school teacher.

VITAL SIGNS:
On physical examination, her blood pressure is 130/80 mmHg, heart rate is 78 beats per minute, and BMI is 28 kg/m^2.

PHYSICAL EXAMINATION:
She appears fatigued but is otherwise well-appearing. Neurological examination is unremarkable.

LABORATORY DATA:
Laboratory tests reveal:
- Hemoglobin A1c: 6.8%
- Thyroid-stimulating hormone (TSH): 2.5 mIU/L
- Serum electrolytes: Within normal limits
- Complete blood count (CBC): Within normal limits

## Encounter 2: source 2021-01-15, practice 2024-06-10, outpatient, Geriatrics

Chief complaint: Brought to the clinic by her niece for evaluation of fatigue, generalized body aches, and multiple unexplained bruises.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Jennifer Park, MD
Department: Geriatrics
Visit Type: outpatient

CHIEF COMPLAINT:
Brought to the clinic by her niece for evaluation of fatigue, generalized body aches, and multiple unexplained bruises.

HISTORY OF PRESENT ILLNESS:
The patient with Alzheimer's disease and depression is brought by her niece for evaluation of fatigue, body aches, and bruising. She reports generalized body aches, and her niece says she has been 'a little more forgetful' and 'clumsier' lately. When asked directly about bruising, she looks away and mumbles, 'I bump into things a lot.'

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer's disease (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-04-15)
- Type 2 Diabetes Mellitus (25-year history, poorly controlled)
- Hypertension
- Metastatic breast cancer (ER/PR positive, HER2 negative)
- Coronary artery disease (status post myocardial infarction)
- Osteoarthritis
- Osteoporosis with history of right femoral neck fracture

PAST SURGICAL HISTORY:
- Modified radical mastectomy with axillary lymph node dissection (5 years ago)
- Open reduction and internal fixation of right femoral neck fracture (2 years ago)
- Percutaneous coronary intervention with drug-eluting stent placement (10 years ago)

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale with meals
- Metformin 1000 mg oral twice daily
- Lisinopril 20 mg oral once daily
- Atorvastatin 40 mg oral once daily
- Aspirin 81 mg oral once daily
- Acetaminophen 650 mg oral every 6 hours as needed for arthritis pain
- Calcium carbonate with vitamin D 600 mg/400 IU oral twice daily

ALLERGIES:
No known drug allergies, Contrast dye (iodinated): unknown

FAMILY HISTORY:
- Mother: Breast cancer (age 72)
- Father: Type 2 diabetes mellitus and hypertension
- Maternal aunt: Alzheimer's disease
- Brother: Coronary artery disease

SOCIAL HISTORY:
The patient lives with her extended family. The patient admits to drinking 2–3 glasses of wine daily to help her sleep. She has missed a few follow-up appointments due to “transportation issues.”

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, the patient appears frail and underweight. There are scattered ecchymoses of varying sizes and colors across the abdomen, upper arms, and thighs, without clear underlying trauma. There is no hepatosplenomegaly, and neurologic exam is non-focal.

LABORATORY DATA:
Laboratory tests show:
Hemoglobin                       11.2 g/dL
Platelet count                       192,000/mm³
Prothrombin time                       12.4 seconds
Partial thromboplastin time                28 seconds

## Encounter 3: source 2021-07-15, practice 2024-12-05, ed, Emergency Medicine

Chief complaint: Acute onset of blurry vision with intermittent flashes of light and new floaters in the right eye for the past 10 hours.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of blurry vision with intermittent flashes of light and new floaters in the right eye for the past 10 hours.

HISTORY OF PRESENT ILLNESS:
This 69-year-old female with long-standing Type 2 Diabetes Mellitus, Alzheimer's disease diagnosed in 2020, and recent evaluation for adult physical abuse presents to the Emergency Department with acute visual symptoms. She describes the vision loss as blurry with intermittent flashes of light and new-onset floaters, and states that she first noticed the symptoms after walking outside on a windy day. She denies trauma, eye rubbing, or recent ocular procedures. She reports no eye redness or discharge, but notes mild aching in the affected eye.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer's disease (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-04-15)
- Adult physical abuse (diagnosed 2021-01-15)
- Type 2 Diabetes Mellitus (25-year history, poorly controlled)
- Hypertension
- Metastatic breast cancer (ER/PR positive, HER2 negative)
- Coronary artery disease (status post myocardial infarction)
- Osteoarthritis
- Osteoporosis with history of right femoral neck fracture

She has a 25-year history of poorly controlled type 2 diabetes mellitus and hypertension.

PAST SURGICAL HISTORY:
- Modified radical mastectomy with axillary lymph node dissection (5 years ago)
- Open reduction and internal fixation of right femoral neck fracture (2 years ago)
- Percutaneous coronary intervention with drug-eluting stent placement (10 years ago)

MEDICATIONS:
She takes treated with insulin and lisinopril, respectively.

Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale with meals
- Metformin 1000 mg oral twice daily
- Atorvastatin 40 mg oral once daily
- Aspirin 81 mg oral once daily
- Acetaminophen 650 mg oral every 6 hours as needed for arthritis pain
- Calcium carbonate with vitamin D 600 mg/400 IU oral twice daily

ALLERGIES:
No known drug allergies, Contrast dye (iodinated): unknown

FAMILY HISTORY:
- Mother: Breast cancer (age 72)
- Father: Type 2 diabetes mellitus and hypertension
- Maternal aunt: Alzheimer's disease
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 30 years ago (1 pack-year). Alcohol: 2-3 glasses of wine daily. Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
On examination, her visual acuity in the right eye is significantly reduced compared to the left, but the globe appears intact with no evidence of conjunctival injection. Intraocular pressure is normal bilaterally. Funduscopic exam is shown below.

## Encounter 4: source 2022-01-15, practice 2025-05-26, outpatient, Palliative Care

Chief complaint: Decline in functional status with desire to discontinue all medications including chemotherapy, antihypertensives, and diabetes drugs.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Emily Thompson, MD
Department: Palliative Care
Visit Type: outpatient

CHIEF COMPLAINT:
Decline in functional status with desire to discontinue all medications including chemotherapy, antihypertensives, and diabetes drugs.

HISTORY OF PRESENT ILLNESS:
The patient with Alzheimer's disease and metastatic breast cancer returns to the Palliative Care clinic with functional decline, having previously presented with memory loss and unexplained bruising. She has been on multiple lines of chemotherapy, with the last regimen discontinued 2 months ago due to disease progression and significant side effects. The patient expresses a desire to stop all her medications, including her oral anti-hypertensives and diabetes management drugs. She also states that she no longer wishes to eat or drink, explaining that she accepts her impending death and prefers to die naturally without further medical intervention.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer's disease (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-04-15)
- Adult physical abuse (diagnosed 2021-01-15)
- Posterior vitreous detachment (diagnosed 2021-07-15)
- Type 2 Diabetes Mellitus (25-year history, poorly controlled)
- Hypertension
- Metastatic breast cancer (ER/PR positive, HER2 negative)
- Coronary artery disease (status post myocardial infarction)
- Osteoarthritis
- Osteoporosis with history of right femoral neck fracture

PAST SURGICAL HISTORY:
- Modified radical mastectomy with axillary lymph node dissection (5 years ago)
- Open reduction and internal fixation of right femoral neck fracture (2 years ago)
- Percutaneous coronary intervention with drug-eluting stent placement (10 years ago)

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale with meals
- Metformin 1000 mg oral twice daily
- Lisinopril 20 mg oral once daily
- Atorvastatin 40 mg oral once daily
- Aspirin 81 mg oral once daily
- Acetaminophen 650 mg oral every 6 hours as needed for arthritis pain
- Calcium carbonate with vitamin D 600 mg/400 IU oral twice daily

ALLERGIES:
No known drug allergies, Contrast dye (iodinated): unknown

FAMILY HISTORY:
- Mother: Breast cancer (age 72)
- Father: Type 2 diabetes mellitus and hypertension
- Maternal aunt: Alzheimer's disease
- Brother: Coronary artery disease

SOCIAL HISTORY:
Her daughter, who is her healthcare proxy, supports her decision.

VITAL SIGNS:
Her blood pressure is 130/85 mmHg, heart rate is 88 beats per minute, and her respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
She is alert and oriented to person, place, and time. On examination, the patient appears cachectic with dry mucous membranes.

LABORATORY DATA:
Sodium: 140 mEq/L
Potassium: 4.2 mEq/L
Chloride: 102 mEq/L
Bicarbonate: 24 mEq/L
Blood urea nitrogen: 21 mg/dL
Creatinine: 1.0 mg/dL
Glucose: 146 mg/dL
Hemoglobin: 9.8 g/dL

## Encounter 5: source 2022-07-15, practice 2025-12-04, ed, Emergency Medicine

Chief complaint: One-week history of progressive weakness, constipation, and worsening confusion in the setting of known metastatic breast cancer.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
One-week history of progressive weakness, constipation, and worsening confusion in the setting of known metastatic breast cancer.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer's disease (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-04-15)
- Adult physical abuse (diagnosed 2021-01-15)
- Posterior vitreous detachment (diagnosed 2021-07-15)
- Delayed circumcision pending patient assent (diagnosed 2022-01-15)
- Type 2 Diabetes Mellitus (25-year history, poorly controlled)
- Hypertension
- Metastatic breast cancer (ER/PR positive, HER2 negative)
- Coronary artery disease (status post myocardial infarction)
- Osteoarthritis
- Osteoporosis with history of right femoral neck fracture

She has a past medical history of hypertension, type 2 diabetes, and breast cancer treated with mastectomy and adjuvant chemotherapy five years ago.

PAST SURGICAL HISTORY:
- Modified radical mastectomy with axillary lymph node dissection (5 years ago)
- Open reduction and internal fixation of right femoral neck fracture (2 years ago)
- Percutaneous coronary intervention with drug-eluting stent placement (10 years ago)

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale with meals
- Metformin 1000 mg oral twice daily
- Lisinopril 20 mg oral once daily
- Atorvastatin 40 mg oral once daily
- Aspirin 81 mg oral once daily
- Acetaminophen 650 mg oral every 6 hours as needed for arthritis pain
- Calcium carbonate with vitamin D 600 mg/400 IU oral twice daily

ALLERGIES:
No known drug allergies, Contrast dye (iodinated): unknown

FAMILY HISTORY:
- Mother: Breast cancer (age 72)
- Father: Type 2 diabetes mellitus and hypertension
- Maternal aunt: Alzheimer's disease
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 30 years ago (1 pack-year). Alcohol: 2-3 glasses of wine daily. Occupation: Retired elementary school teacher.

REVIEW OF SYSTEMS:
She denies any fever, cough, or urinary symptoms.

VITAL SIGNS:
Her blood pressure is 130/85 mmHg, heart rate is 88 beats per minute, and she is afebrile.

PHYSICAL EXAMINATION:
On examination, she is lethargic but oriented to person and place, not to time. Abdominal examination reveals mild diffuse tenderness without rebound or guarding. Neurological examination shows generalized muscle weakness without focal deficits.

LABORATORY DATA:
Serum calcium: 14.2 mg/dL (normal: 8.5-10.2 mg/dL)
Serum creatinine: 1.8 mg/dL (normal: 0.6-1.1 mg/dL)
Blood urea nitrogen (BUN): 30 mg/dL (normal: 7-20 mg/dL)
Serum phosphorus: 2.4 mg/dL (normal: 2.5-4.5 mg/dL)
Serum albumin: 4.0 g/dL (normal: 3.5-5.0 g/dL)
Parathyroid hormone (PTH): Low

## Encounter 6: source 2022-09-15, practice 2026-01-24, inpatient, Internal Medicine

Chief complaint: Progressive fatigue requiring admission for anemia workup; on hospital day 2, developed acute shortness of breath and chest heaviness one hour after completing transfusion of two units packed red blood cells.

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. Aisha Patel, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Progressive fatigue requiring admission for anemia workup; on hospital day 2, developed acute shortness of breath and chest heaviness one hour after completing transfusion of two units packed red blood cells.

HISTORY OF PRESENT ILLNESS:
The patient is a 70-year-old with metastatic breast cancer admitted for progressive anemia, following recent palliative care evaluation and emergency department treatment for hypercalcemia. On hospital day 2, she receives two units of packed red blood cells for persistent anemia. One hour after the transfusion is completed, she develops shortness of breath and a sensation of chest heaviness. There is no rash, no fever, no hypotension, and no hemoglobinuria.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer's disease (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-04-15)
- Adult physical abuse (diagnosed 2021-01-15)
- Posterior vitreous detachment (diagnosed 2021-07-15)
- Delayed circumcision pending patient assent (diagnosed 2022-01-15)
- Malignancy-associated hypercalcemia (diagnosed 2022-07-15)
- Type 2 Diabetes Mellitus (25-year history, poorly controlled)
- Hypertension
- Metastatic breast cancer (ER/PR positive, HER2 negative)
- Coronary artery disease (status post myocardial infarction)
- Osteoarthritis
- Osteoporosis with history of right femoral neck fracture

PAST SURGICAL HISTORY:
- Modified radical mastectomy with axillary lymph node dissection (5 years ago)
- Open reduction and internal fixation of right femoral neck fracture (2 years ago)
- Percutaneous coronary intervention with drug-eluting stent placement (10 years ago)

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale with meals
- Metformin 1000 mg oral twice daily
- Lisinopril 20 mg oral once daily
- Atorvastatin 40 mg oral once daily
- Aspirin 81 mg oral once daily
- Acetaminophen 650 mg oral every 6 hours as needed for arthritis pain
- Calcium carbonate with vitamin D 600 mg/400 IU oral twice daily

ALLERGIES:
No known drug allergies, Contrast dye (iodinated): unknown

FAMILY HISTORY:
- Mother: Breast cancer (age 72)
- Father: Type 2 diabetes mellitus and hypertension
- Maternal aunt: Alzheimer's disease
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 30 years ago (1 pack-year). Alcohol: 2-3 glasses of wine daily. Occupation: Retired elementary school teacher.

VITAL SIGNS:
Temperature 37.0°C, blood pressure 168/90 mmHg, pulse 98/min, and respiratory rate 24/min. Oxygen saturation is 87% on room air.

PHYSICAL EXAMINATION:
New bilateral crackles at the lung bases and jugular venous distension

LABORATORY DATA:
Elevated brain natriuretic peptide level

IMAGING:
A chest radiograph shows new bilateral pulmonary opacities

## Encounter 7: source 2023-01-15, practice 2026-06-06, icu, Emergency Medicine

Chief complaint: Two-day history of progressive shortness of breath, confusion, and decreased urine output with high fever and chills for the past 24 hours.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Robert Liu, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Two-day history of progressive shortness of breath, confusion, and decreased urine output with high fever and chills for the past 24 hours.

HISTORY OF PRESENT ILLNESS:
This 71-year-old female with metastatic breast cancer, Alzheimer's disease, and prior transfusion-associated circulatory overload presents to the emergency department with a 2-day history of progressive shortness of breath, confusion, and decreased urine output. She reports a high fever and chills for the past 24 hours.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer's disease (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-04-15)
- Adult physical abuse (diagnosed 2021-01-15)
- Posterior vitreous detachment (diagnosed 2021-07-15)
- Delayed circumcision pending patient assent (diagnosed 2022-01-15)
- Malignancy-associated hypercalcemia (diagnosed 2022-07-15)
- Transfusion-associated circulatory overload (diagnosed 2022-09-15)
- Type 2 Diabetes Mellitus (25-year history, poorly controlled)
- Hypertension
- Metastatic breast cancer (ER/PR positive, HER2 negative)
- Coronary artery disease (status post myocardial infarction)
- Osteoarthritis
- Osteoporosis with history of right femoral neck fracture

PAST SURGICAL HISTORY:
- Modified radical mastectomy with axillary lymph node dissection (5 years ago)
- Open reduction and internal fixation of right femoral neck fracture (2 years ago)
- Percutaneous coronary intervention with drug-eluting stent placement (10 years ago)

MEDICATIONS:
She takes metformin, lisinopril, and atorvastatin.

Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale with meals
- Aspirin 81 mg oral once daily
- Acetaminophen 650 mg oral every 6 hours as needed for arthritis pain
- Calcium carbonate with vitamin D 600 mg/400 IU oral twice daily

ALLERGIES:
No known drug allergies, Contrast dye (iodinated): unknown

FAMILY HISTORY:
- Mother: Breast cancer (age 72)
- Father: Type 2 diabetes mellitus and hypertension
- Maternal aunt: Alzheimer's disease
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 30 years ago (1 pack-year). Alcohol: 2-3 glasses of wine daily. Occupation: Retired elementary school teacher.

VITAL SIGNS:
Temperature is 38.9°C (102°F), blood pressure is 90/60 mmHg, heart rate is 110 beats/min, and respiratory rate is 28 breaths/min

PHYSICAL EXAMINATION:
She appears lethargic and is oriented to person only. Lung auscultation reveals bilateral crackles, and her skin is warm with petechiae on her lower extremities.

LABORATORY DATA:
White blood cell count: 18,000/mm³
Hemoglobin: 10 g/dL
Platelets: 50,000/mm³
Blood urea nitrogen: 40 mg/dL
Serum creatinine: 3.5 mg/dL (baseline 1.0 mg/dL)
Lactic acid: 5 mmol/L
Arterial blood gas: pH 7.25, PaCO2 30 mmHg, HCO3- 15 mEq/L
Blood cultures are pending.
Urinalysis shows muddy brown casts.

PLAN:
The patient is started on broad-spectrum antibiotics and intravenous fluids.
