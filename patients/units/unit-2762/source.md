# synthetic_hospital patient 2762

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 43 at first encounter, sex F, African American, Commercial PPO through school district.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-06-30 (Mon) | outpatient | Pulmonology |
| 1 | 2020-04-15 | 2025-09-27 (Sat) | inpatient | Obstetrics & Gynecology |
| 2 | 2020-04-15 | 2025-10-10 (Fri) | inpatient | Internal Medicine |
| 3 | 2020-05-15 | 2025-11-10 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO through school district",
 "occupation": "High school history teacher",
 "alcohol_use": "Occasional social use (1-2 glasses of wine per month)",
 "family_history": [
  "Mother with symptomatic uterine fibroids (hysterectomy at age 45)",
  "Maternal aunt with systemic lupus erythematosus",
  "Father with type 2 diabetes mellitus",
  "No known family history of sarcoidosis or chronic lung disease"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg by mouth daily (maintained for approximately 3 years)",
   "name": "Prednisone"
  },
  {
   "dose": "600 mg/400 IU by mouth twice daily",
   "name": "Calcium carbonate with Vitamin D3"
  },
  {
   "dose": "70 mg by mouth weekly",
   "name": "Alendronate"
  }
 ],
 "surgical_history": [
  "Total abdominal hysterectomy (age 43)",
  "Cystoscopy with retrograde pyelography (post-hysterectomy complication)",
  "Slit-lamp examination and intraocular pressure monitoring (multiple prior)"
 ],
 "chronic_conditions": [
  "Pulmonary sarcoidosis (Stage II)",
  "History of ocular sarcoidosis (anterior uveitis)",
  "Symptomatic uterine fibroids",
  "Chronic glucocorticoid-induced adrenal suppression",
  "Osteopenia secondary to long-term corticosteroid use"
 ],
 "age_at_first_encounter": 43
}
```

Primary diagnoses: ["Drug-induced adrenocortical insufficiency", "Ureteral injury", "Prerenal acute kidney injury secondary to hypovolemia", "Sarcoidosis"]
Comorbidities: ["Status post total abdominal hysterectomy", "Uterine leiomyoma"]

## Encounter 0: source 2020-01-15, practice 2025-06-30, outpatient, Pulmonology

Chief complaint: Progressive dyspnea on exertion and dry cough over the past 6 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive dyspnea on exertion and dry cough over the past 6 months

HISTORY OF PRESENT ILLNESS:
6-month history of progressive dyspnea on exertion and a dry cough. She denies any chest pain, fever, or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Pulmonary sarcoidosis (Stage II)
- History of ocular sarcoidosis (anterior uveitis)
- Symptomatic uterine fibroids
- Chronic glucocorticoid-induced adrenal suppression
- Osteopenia secondary to long-term corticosteroid use

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy (age 43)
- Cystoscopy with retrograde pyelography (post-hysterectomy complication)
- Slit-lamp examination and intraocular pressure monitoring (multiple prior)

MEDICATIONS:
Home Medications:
- Prednisone 10 mg by mouth daily (maintained for approximately 3 years)
- Calcium carbonate with Vitamin D3 600 mg/400 IU by mouth twice daily
- Alendronate 70 mg by mouth weekly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with symptomatic uterine fibroids (hysterectomy at age 45)
- Maternal aunt with systemic lupus erythematosus
- Father with type 2 diabetes mellitus
- No known family history of sarcoidosis or chronic lung disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 glasses of wine per month). Occupation: High school history teacher.

PHYSICAL EXAMINATION:
Bilateral hilar lymphadenopathy. Her skin examination reveals several non-tender, erythematous nodules on her shins.

OTHER STUDIES:
Pulmonary function tests (PFTs) are performed, showing:

- FEV1: 65% of predicted
- FVC: 68% of predicted
- FEV1/FVC ratio: 80% (normal >70%)
- Total lung capacity (TLC): 60% of predicted
- Diffusion capacity for carbon monoxide (DLCO): 55% of predicted

## Encounter 1: source 2020-04-15, practice 2025-09-27, inpatient, Obstetrics & Gynecology

Chief complaint: Elective total abdominal hysterectomy for symptomatic uterine fibroids complicated by refractory hypotension

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Maria Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Elective total abdominal hysterectomy for symptomatic uterine fibroids complicated by refractory hypotension

HISTORY OF PRESENT ILLNESS:
Patient returns for hysterectomy with sarcoidosis history. Preoperative evaluation was unremarkable. Two liters of intravenous fluids are given with minimal improvement.

PAST MEDICAL HISTORY:
Active Problem List:
- Sarcoidosis (diagnosed 2020-01-15)
- Pulmonary sarcoidosis (Stage II)
- History of ocular sarcoidosis (anterior uveitis)
- Symptomatic uterine fibroids
- Chronic glucocorticoid-induced adrenal suppression
- Osteopenia secondary to long-term corticosteroid use

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy (age 43)
- Cystoscopy with retrograde pyelography (post-hysterectomy complication)
- Slit-lamp examination and intraocular pressure monitoring (multiple prior)

MEDICATIONS:
She takes she has been taking daily prednisone for the past 3 years.

Home Medications:
- Calcium carbonate with Vitamin D3 600 mg/400 IU by mouth twice daily
- Alendronate 70 mg by mouth weekly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with symptomatic uterine fibroids (hysterectomy at age 45)
- Maternal aunt with systemic lupus erythematosus
- Father with type 2 diabetes mellitus
- No known family history of sarcoidosis or chronic lung disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 glasses of wine per month). Occupation: High school history teacher.

VITAL SIGNS:
Shortly after the procedure begins, her blood pressure drops to 76/50 mm Hg and her pulse is 108/min

PHYSICAL EXAMINATION:
She appears pale and sweaty

LABORATORY DATA:
Baseline labs, including hemoglobin and electrolytes, were normal

## Encounter 2: source 2020-04-15, practice 2025-10-10, inpatient, Internal Medicine

Chief complaint: Oliguria and fatigue following intraoperative hemorrhage during hysterectomy

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. James Park, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Oliguria and fatigue following intraoperative hemorrhage during hysterectomy

HISTORY OF PRESENT ILLNESS:
The patient is a 43-year-old female with Stage II pulmonary sarcoidosis and chronic glucocorticoid-induced adrenal suppression who presents to Internal Medicine on postoperative day zero. She undergoes an elective hysterectomy for symptomatic uterine fibroids. During the procedure, she experiences significant blood loss, and her blood pressure briefly drops to 80/40 mm Hg for approximately 30 seconds before being stabilized with intravenous fluids and blood transfusion. Vital signs normalize within an hour, and she remains hemodynamically stable.

PAST MEDICAL HISTORY:
Active Problem List:
- Sarcoidosis (diagnosed 2020-01-15)
- Drug-induced adrenocortical insufficiency (diagnosed 2020-04-15)
- Pulmonary sarcoidosis (Stage II)
- History of ocular sarcoidosis (anterior uveitis)
- Symptomatic uterine fibroids
- Chronic glucocorticoid-induced adrenal suppression
- Osteopenia secondary to long-term corticosteroid use

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy (age 43)
- Cystoscopy with retrograde pyelography (post-hysterectomy complication)
- Slit-lamp examination and intraocular pressure monitoring (multiple prior)

MEDICATIONS:
The attending physician reviews the chart and notes that she received 5 units of packed red blood cells intraoperatively.

Home Medications:
- Prednisone 10 mg by mouth daily (maintained for approximately 3 years)
- Calcium carbonate with Vitamin D3 600 mg/400 IU by mouth twice daily
- Alendronate 70 mg by mouth weekly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with symptomatic uterine fibroids (hysterectomy at age 45)
- Maternal aunt with systemic lupus erythematosus
- Father with type 2 diabetes mellitus
- No known family history of sarcoidosis or chronic lung disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 glasses of wine per month). Occupation: High school history teacher.

PHYSICAL EXAMINATION:
On examination, her skin is warm and well-perfused, and lung auscultation is clear. Foley catheter output remains minimal.

LABORATORY DATA:
Laboratory testing reveals elevated serum creatinine and BUN.

## Encounter 3: source 2020-05-15, practice 2025-11-10, ed, Emergency Medicine

Chief complaint: Acute left-sided flank pain two days post-hysterectomy

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute left-sided flank pain two days post-hysterectomy

HISTORY OF PRESENT ILLNESS:
The patient is a 43-year-old female with Stage II pulmonary sarcoidosis and chronic glucocorticoid-induced adrenal suppression who presents two days following total abdominal hysterectomy for symptomatic uterine fibroids, having previously been hospitalized for this procedure with adrenocortical insufficiency and acute kidney injury. She reports the surgery was uncomplicated and that she was discharged home the following day. However, since returning home, she has developed progressive lower abdominal discomfort and sharp pain radiating to the left flank. She has had no bowel movements since surgery but is passing gas.

PAST MEDICAL HISTORY:
Active Problem List:
- Sarcoidosis (diagnosed 2020-01-15)
- Drug-induced adrenocortical insufficiency (diagnosed 2020-04-15)
- Prerenal acute kidney injury secondary to hypovolemia (diagnosed 2020-04-15)
- Pulmonary sarcoidosis (Stage II)
- History of ocular sarcoidosis (anterior uveitis)
- Symptomatic uterine fibroids
- Chronic glucocorticoid-induced adrenal suppression
- Osteopenia secondary to long-term corticosteroid use

PAST SURGICAL HISTORY:
Total abdominal hysterectomy

MEDICATIONS:
Home Medications:
- Prednisone 10 mg by mouth daily (maintained for approximately 3 years)
- Calcium carbonate with Vitamin D3 600 mg/400 IU by mouth twice daily
- Alendronate 70 mg by mouth weekly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with symptomatic uterine fibroids (hysterectomy at age 45)
- Maternal aunt with systemic lupus erythematosus
- Father with type 2 diabetes mellitus
- No known family history of sarcoidosis or chronic lung disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 glasses of wine per month). Occupation: High school history teacher.

VITAL SIGNS:
Her temperature is 37.4°C (99.3°F), blood pressure is 128/78 mm Hg, and pulse is 88/min

PHYSICAL EXAMINATION:
Physical examination reveals left costovertebral angle tenderness but no rebound or guarding.

LABORATORY DATA:
Laboratory studies show a mild leukocytosis. Urinalysis reveals trace hematuria but is otherwise unremarkable.
