# synthetic_hospital patient 2725

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 37 at first encounter, sex F, African American, Blue Cross Blue Shield.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-07-24 (Mon) | outpatient | Endocrinology |
| 1 | 2020-04-15 | 2023-10-23 (Mon) | outpatient | Endocrinology |
| 2 | 2020-07-15 | 2024-01-16 (Tue) | outpatient | Dermatology |
| 3 | 2021-01-15 | 2024-07-17 (Wed) | inpatient | Obstetrics & Gynecology |
| 4 | 2021-07-15 | 2025-01-20 (Mon) | outpatient | Cardiology |
| 5 | 2022-01-15 | 2025-07-28 (Mon) | telehealth | Dermatology |
| 6 | 2022-07-15 | 2026-01-09 (Fri) | ed | Emergency Medicine |
| 7 | 2023-01-15 | 2026-07-18 (Sat) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Nickel (contact dermatitis)",
  "Fragrances (contact dermatitis)"
 ],
 "insurance": "Blue Cross Blue Shield",
 "occupation": "Administrative Assistant",
 "alcohol_use": "None",
 "family_history": [
  "Mother with systemic lupus erythematosus",
  "Father with essential hypertension",
  "No family history of pheochromocytoma, neurofibromatosis, or multiple endocrine neoplasia syndromes"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg twice daily",
   "name": "Phenoxybenzamine"
  },
  {
   "dose": "50 mg daily",
   "name": "Atenolol"
  },
  {
   "dose": "10 mg daily",
   "name": "Amlodipine"
  },
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25 mg daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "20 mg three times daily",
   "name": "Sildenafil"
  },
  {
   "dose": "20 mg daily",
   "name": "Omeprazole"
  }
 ],
 "surgical_history": [
  "Total abdominal hysterectomy"
 ],
 "chronic_conditions": [
  "Systemic Sclerosis (Diffuse Cutaneous)",
  "Pulmonary Arterial Hypertension",
  "Pheochromocytoma",
  "Severe Secondary Hypertension",
  "Raynaud Phenomenon",
  "Nickel Contact Sensitivity"
 ],
 "age_at_first_encounter": 37
}
```

Primary diagnoses: ["Scleroderma renal crisis", "Pulmonary Hypertension", "Pheochromocytoma", "Allergic contact dermatitis", "Septic shock", "Ureteral laceration (iatrogenic)"]
Comorbidities: ["Acute kidney injury", "Essential hypertension", "Systemic sclerosis"]

## Encounter 0: source 2020-01-15, practice 2023-07-24, outpatient, Endocrinology

Chief complaint: Six-month history of difficult-to-control hypertension with episodic palpitations, diaphoresis, and severe headaches

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of difficult-to-control hypertension with episodic palpitations, diaphoresis, and severe headaches

HISTORY OF PRESENT ILLNESS:
She reports episodic palpitations, diaphoresis, and severe headaches. These episodes occur randomly and are sometimes accompanied by a sense of impending doom.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic Sclerosis (Diffuse Cutaneous)
- Pulmonary Arterial Hypertension
- Pheochromocytoma
- Severe Secondary Hypertension
- Raynaud Phenomenon
- Nickel Contact Sensitivity

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy

MEDICATIONS:
She takes despite adherence to a three-drug regimen including a calcium channel blocker, an ACE inhibitor, and a thiazide diuretic.

Home Medications:
- Phenoxybenzamine 10 mg twice daily
- Atenolol 50 mg daily
- Amlodipine 10 mg daily
- Lisinopril 20 mg daily
- Hydrochlorothiazide 25 mg daily
- Sildenafil 20 mg three times daily
- Omeprazole 20 mg daily

ALLERGIES:
Nickel (contact dermatitis), Fragrances (contact dermatitis)

FAMILY HISTORY:
Her family history is notable for her father who had thyroid surgery for an unspecified condition.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Assistant.

VITAL SIGNS:
On physical examination, her blood pressure is 162/98 mmHg, heart rate is 102 beats per minute,

PHYSICAL EXAMINATION:
She appears anxious. An abdominal exam reveals no masses or bruits.

LABORATORY DATA:
A random plasma metanephrine test shows:

- Metanephrine: 1.2 nmol/L (Normal < 0.5 nmol/L)
- Normetanephrine: 2.4 nmol/L (Normal < 0.9 nmol/L)

## Encounter 1: source 2020-04-15, practice 2023-10-23, outpatient, Endocrinology

Chief complaint: Six-month history of increasing facial fullness, progressive muscle weakness, easy bruising, and irregular menstrual cycles

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Okafor, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of increasing facial fullness, progressive muscle weakness, easy bruising, and irregular menstrual cycles

HISTORY OF PRESENT ILLNESS:
The patient is a 37-year-old woman with diffuse cutaneous systemic sclerosis and pulmonary arterial hypertension who returns to endocrinology for follow-up. She presents with a 6-month history of increasing facial fullness, progressive muscle weakness, and easy bruising. She also reports worsening fatigue and irregular menstrual cycles.

PAST MEDICAL HISTORY:
Active Problem List:
- Pheochromocytoma (diagnosed 2020-01-15)
- Systemic Sclerosis (Diffuse Cutaneous)
- Pulmonary Arterial Hypertension
- Severe Secondary Hypertension
- Raynaud Phenomenon
- Nickel Contact Sensitivity

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy

MEDICATIONS:
Home Medications:
- Phenoxybenzamine 10 mg twice daily
- Atenolol 50 mg daily
- Amlodipine 10 mg daily
- Lisinopril 20 mg daily
- Hydrochlorothiazide 25 mg daily
- Sildenafil 20 mg three times daily
- Omeprazole 20 mg daily

ALLERGIES:
Nickel (contact dermatitis), Fragrances (contact dermatitis)

FAMILY HISTORY:
- Mother with systemic lupus erythematosus
- Father with essential hypertension
- No family history of pheochromocytoma, neurofibromatosis, or multiple endocrine neoplasia syndromes

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Assistant.

VITAL SIGNS:
Elevated blood pressure

PHYSICAL EXAMINATION:
On exam, she has scattered ecchymoses on the upper limbs, and reduced proximal muscle strength.

LABORATORY DATA:
Elevated fasting glucose and undectable ACTH levels

OTHER STUDIES:
A high-dose dexamethasone suppression test fails to reduce serum cortisol levels.

## Encounter 2: source 2020-07-15, practice 2024-01-16, outpatient, Dermatology

Chief complaint: Three-week history of pruritic rash localized to left wrist after wearing new wristwatch

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Patel, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-week history of pruritic rash localized to left wrist after wearing new wristwatch

HISTORY OF PRESENT ILLNESS:
The patient with systemic sclerosis, pheochromocytoma, and nickel sensitivity presents for dermatologic evaluation. The symptoms began shortly after she started wearing a new wristwatch. Although she stopped using the watch 3 days ago, her symptoms have persisted. She denies recent changes in detergents or soaps, but she frequently applies perfume to her wrists.

PAST MEDICAL HISTORY:
Active Problem List:
- Pheochromocytoma (diagnosed 2020-01-15)
- Systemic Sclerosis (Diffuse Cutaneous)
- Pulmonary Arterial Hypertension
- Severe Secondary Hypertension
- Raynaud Phenomenon
- Nickel Contact Sensitivity

She has a history of hypertension treated with atenolol for the past 2 years.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy

MEDICATIONS:
She takes atenolol for the past 2 years.

Home Medications:
- Phenoxybenzamine 10 mg twice daily
- Amlodipine 10 mg daily
- Lisinopril 20 mg daily
- Hydrochlorothiazide 25 mg daily
- Sildenafil 20 mg three times daily
- Omeprazole 20 mg daily

ALLERGIES:
Nickel (contact dermatitis), Fragrances (contact dermatitis)

FAMILY HISTORY:
- Mother with systemic lupus erythematosus
- Father with essential hypertension
- No family history of pheochromocytoma, neurofibromatosis, or multiple endocrine neoplasia syndromes

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Assistant.

PHYSICAL EXAMINATION:
On examination, her wrist is pruritic and she reports a burning sensation in the affected area. There is no systemic involvement. The skin findings are shown in the picture below.

## Encounter 3: source 2021-01-15, practice 2024-07-17, inpatient, Obstetrics & Gynecology

Chief complaint: Acute onset of constant lower abdominal pain 12 hours after total abdominal hysterectomy for menorrhagia

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Jennifer Walsh, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Acute onset of constant lower abdominal pain 12 hours after total abdominal hysterectomy for menorrhagia

HISTORY OF PRESENT ILLNESS:
The patient with known systemic sclerosis, pulmonary arterial hypertension, and recently diagnosed pheochromocytoma presents with acute lower abdominal pain. Pain began 12 hours after undergoing a total abdominal hysterectomy for menorrhagia. She reports that the pain is constant and does not radiate. She denies any nausea, vomiting, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Pheochromocytoma (diagnosed 2020-01-15)
- Allergic contact dermatitis (diagnosed 2020-07-15)
- Systemic Sclerosis (Diffuse Cutaneous)
- Pulmonary Arterial Hypertension
- Severe Secondary Hypertension
- Raynaud Phenomenon
- Nickel Contact Sensitivity

Hypertension, which is well-controlled.

PAST SURGICAL HISTORY:
Total abdominal hysterectomy for menorrhagia

MEDICATIONS:
Hydrochlorothiazide

Home Medications:
- Phenoxybenzamine 10 mg twice daily
- Atenolol 50 mg daily
- Amlodipine 10 mg daily
- Lisinopril 20 mg daily
- Sildenafil 20 mg three times daily
- Omeprazole 20 mg daily

ALLERGIES:
Nickel (contact dermatitis), Fragrances (contact dermatitis)

FAMILY HISTORY:
- Mother with systemic lupus erythematosus
- Father with essential hypertension
- No family history of pheochromocytoma, neurofibromatosis, or multiple endocrine neoplasia syndromes

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Assistant.

VITAL SIGNS:
Blood pressure is 130/85 mmHg, heart rate is 90 beats per minute, and temperature is 37.2°C (99°F)

PHYSICAL EXAMINATION:
Abdominal examination reveals mild distension with diffuse tenderness, but no rebound or guarding. She has no costovertebral angle tenderness. Her urine output has been normal since surgery.

LABORATORY DATA:
Hemoglobin: 12.5 g/dL
White blood cell count: 11,000/mm³
Serum creatinine: 0.9 mg/dL
Blood urea nitrogen (BUN): 14 mg/dL
Urinalysis: Normal, no hematuria or pyuria

IMAGING:
CT scan of the abdomen and pelvis with intravenous contrast shows a large volume of fluid in the peritoneal cavity with no evidence of bowel injury or intra-abdominal abscess

## Encounter 4: source 2021-07-15, practice 2025-01-20, outpatient, Cardiology

Chief complaint: Gradually worsening exertional shortness of breath and mild ankle swelling over several months

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. David Kim, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Gradually worsening exertional shortness of breath and mild ankle swelling over several months

HISTORY OF PRESENT ILLNESS:
The patient returns for cardiology follow-up for diffuse cutaneous Systemic Sclerosis and Pulmonary Arterial Hypertension, with gradually worsening exertional symptoms over several months. She reports feeling winded when climbing stairs or walking up inclines and occasionally experiences a dull discomfort in her chest during exertion. She has also noticed mild swelling in her ankles at the end of the day. She denies orthopnea, paroxysmal nocturnal dyspnea, or recent infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Pheochromocytoma (diagnosed 2020-01-15)
- Allergic contact dermatitis (diagnosed 2020-07-15)
- Ureteral laceration (iatrogenic) (diagnosed 2021-01-15)
- Systemic Sclerosis (Diffuse Cutaneous)
- Pulmonary Arterial Hypertension
- Severe Secondary Hypertension
- Raynaud Phenomenon
- Nickel Contact Sensitivity

Her medical history is notable for occasional episodes of cold-induced discoloration of her fingers, which she has never evaluated.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy

MEDICATIONS:
Home Medications:
- Phenoxybenzamine 10 mg twice daily
- Atenolol 50 mg daily
- Amlodipine 10 mg daily
- Lisinopril 20 mg daily
- Hydrochlorothiazide 25 mg daily
- Sildenafil 20 mg three times daily
- Omeprazole 20 mg daily

ALLERGIES:
Nickel (contact dermatitis), Fragrances (contact dermatitis)

FAMILY HISTORY:
- Mother with systemic lupus erythematosus
- Father with essential hypertension
- No family history of pheochromocytoma, neurofibromatosis, or multiple endocrine neoplasia syndromes

SOCIAL HISTORY:
She does not smoke or use recreational drugs.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
Cardiopulmonary exam reveals a prominent second heart sound, and trace edema is noted in both ankles. There are no rales, wheezes, or murmurs.

IMAGING:
Chest imaging shows no infiltrates or structural abnormalities.
Transthoracic echocardiography reveals right-sided cardiac chamber enlargement with preserved left ventricular size and function.

OTHER STUDIES:
Pulmonary function testing is unremarkable, and
An ECG shows right axis deviation and nonspecific ST-T changes.

## Encounter 5: source 2022-01-15, practice 2025-07-28, telehealth, Dermatology

Chief complaint: Recurrent three-week history of left wrist rash after new jewelry exposure

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James Rodriguez, MD
Department: Dermatology
Visit Type: telehealth

CHIEF COMPLAINT:
Recurrent three-week history of left wrist rash after new jewelry exposure

HISTORY OF PRESENT ILLNESS:
The patient is a 39-year-old female with diffuse cutaneous systemic sclerosis and nickel contact sensitivity who presents via telehealth for recurrent left wrist rash similar to her July 2020 presentation. The symptoms began shortly after she started wearing a new wristwatch. Although she stopped using the watch 3 days ago, her symptoms have persisted. She denies recent changes in detergents or soaps, but she frequently applies perfume to her wrists.

PAST MEDICAL HISTORY:
Active Problem List:
- Pheochromocytoma (diagnosed 2020-01-15)
- Allergic contact dermatitis (diagnosed 2020-07-15)
- Ureteral laceration (iatrogenic) (diagnosed 2021-01-15)
- Pulmonary Hypertension (diagnosed 2021-07-15)
- Systemic Sclerosis (Diffuse Cutaneous)
- Pulmonary Arterial Hypertension
- Severe Secondary Hypertension
- Raynaud Phenomenon
- Nickel Contact Sensitivity

She has a history of hypertension treated with atenolol for the past 2 years.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy

MEDICATIONS:
Home Medications:
- Phenoxybenzamine 10 mg twice daily
- Atenolol 50 mg daily
- Amlodipine 10 mg daily
- Lisinopril 20 mg daily
- Hydrochlorothiazide 25 mg daily
- Sildenafil 20 mg three times daily
- Omeprazole 20 mg daily

ALLERGIES:
Nickel (contact dermatitis), Fragrances (contact dermatitis)

FAMILY HISTORY:
- Mother with systemic lupus erythematosus
- Father with essential hypertension
- No family history of pheochromocytoma, neurofibromatosis, or multiple endocrine neoplasia syndromes

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Assistant.

PHYSICAL EXAMINATION:
On examination, her wrist is pruritic and she reports a burning sensation in the affected area. There is no systemic involvement. The skin findings are shown in the picture below.

## Encounter 6: source 2022-07-15, practice 2026-01-09, ed, Emergency Medicine

Chief complaint: Two-day history of severe headache, blurred vision, and nausea with inability to tolerate oral antihypertensives

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-day history of severe headache, blurred vision, and nausea with inability to tolerate oral antihypertensives

HISTORY OF PRESENT ILLNESS:
This 39-year-old female with diffuse cutaneous systemic sclerosis, pulmonary arterial hypertension, and history of pheochromocytoma presents with inability to tolerate oral antihypertensives. She reports that she has not been able to keep her antihypertensive medications down due to vomiting.

PAST MEDICAL HISTORY:
Active Problem List:
- Pheochromocytoma (diagnosed 2020-01-15)
- Allergic contact dermatitis (diagnosed 2020-07-15)
- Ureteral laceration (iatrogenic) (diagnosed 2021-01-15)
- Pulmonary Hypertension (diagnosed 2021-07-15)
- Systemic Sclerosis (Diffuse Cutaneous)
- Pulmonary Arterial Hypertension
- Severe Secondary Hypertension
- Raynaud Phenomenon
- Nickel Contact Sensitivity

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy

MEDICATIONS:
Home Medications:
- Phenoxybenzamine 10 mg twice daily
- Atenolol 50 mg daily
- Amlodipine 10 mg daily
- Lisinopril 20 mg daily
- Hydrochlorothiazide 25 mg daily
- Sildenafil 20 mg three times daily
- Omeprazole 20 mg daily

ALLERGIES:
Nickel (contact dermatitis), Fragrances (contact dermatitis)

FAMILY HISTORY:
- Mother with systemic lupus erythematosus
- Father with essential hypertension
- No family history of pheochromocytoma, neurofibromatosis, or multiple endocrine neoplasia syndromes

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Assistant.

VITAL SIGNS:
Her blood pressure is 220/120 mmHg, heart rate is 88 beats per minute, and respiratory rate is 18 breaths per minute.

PHYSICAL EXAMINATION:
Physical examination reveals diffuse skin thickening and tenderness over the extensor surfaces of her joints. Fundoscopic examination shows bilateral flame-shaped hemorrhages and cotton wool spots.

LABORATORY DATA:
Laboratory studies are as follows:

- Serum creatinine: 3.2 mg/dL (baseline 0.9 mg/dL)
- Blood urea nitrogen (BUN): 45 mg/dL
- Hemoglobin: 9.2 g/dL
- Platelet count: 90,000/mm^3
- Urinalysis: Specific gravity 1.010, trace protein, no cells or casts
- Peripheral blood smear: Schistocytes present

## Encounter 7: source 2023-01-15, practice 2026-07-18, icu, Emergency Medicine

Chief complaint: Dysuria, left flank pain progressing to lethargy and anuria over 24 hours

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Robert Chen, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Dysuria, left flank pain progressing to lethargy and anuria over 24 hours

HISTORY OF PRESENT ILLNESS:
Patient with diffuse cutaneous scleroderma and prior renal crisis presents with dysuria and left flank pain. Over the past 24 hours, she has become increasingly lethargic and has not urinated.

PAST MEDICAL HISTORY:
Active Problem List:
- Pheochromocytoma (diagnosed 2020-01-15)
- Allergic contact dermatitis (diagnosed 2020-07-15)
- Ureteral laceration (iatrogenic) (diagnosed 2021-01-15)
- Pulmonary Hypertension (diagnosed 2021-07-15)
- Scleroderma renal crisis (diagnosed 2022-07-15)
- Systemic Sclerosis (Diffuse Cutaneous)
- Pulmonary Arterial Hypertension
- Severe Secondary Hypertension
- Raynaud Phenomenon
- Nickel Contact Sensitivity

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy

MEDICATIONS:
Home Medications:
- Phenoxybenzamine 10 mg twice daily
- Atenolol 50 mg daily
- Amlodipine 10 mg daily
- Lisinopril 20 mg daily
- Hydrochlorothiazide 25 mg daily
- Sildenafil 20 mg three times daily
- Omeprazole 20 mg daily

ALLERGIES:
Nickel (contact dermatitis), Fragrances (contact dermatitis)

FAMILY HISTORY:
- Mother with systemic lupus erythematosus
- Father with essential hypertension
- No family history of pheochromocytoma, neurofibromatosis, or multiple endocrine neoplasia syndromes

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Assistant.

VITAL SIGNS:
Vitals are: temperature 39 C, BP 80/40, HR 100, RR 22.

PHYSICAL EXAMINATION:
She appears acutely ill with cool extremities and delayed capillary refill. Skin examination is normal.

LABORATORY DATA:
Laboratory studies show elevated creatinine and blood urea nitrogen, elevated lactate, and neutrophilic leukocytosis. Blood cultures are pending.
