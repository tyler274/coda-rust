# synthetic_hospital patient 1993

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 50 at first encounter, sex M, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-09-05 (Mon) | outpatient | Endocrinology |
| 1 | 2020-07-15 | 2023-03-08 (Wed) | outpatient | Pulmonology |
| 2 | 2021-03-15 | 2023-10-27 (Fri) | ed | Emergency Medicine |
| 3 | 2021-11-15 | 2024-07-02 (Tue) | inpatient | Gastroenterology |
| 4 | 2022-05-15 | 2025-01-09 (Thu) | icu | Emergency Medicine |
| 5 | 2022-11-15 | 2025-07-03 (Thu) | ed | Emergency Medicine |
| 6 | 2022-12-15 | 2025-08-09 (Sat) | icu | Hematology/Oncology |
| 7 | 2023-07-15 | 2026-03-14 (Sat) | ed | Orthopedics |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Construction supervisor",
 "alcohol_use": "Chronic heavy daily use, 6-12 beers per day with recent binge episodes",
 "family_history": [
  "Father: Hypertension, stroke at age 68",
  "Mother: Autoimmune thyroid disease",
  "Brother: Acromegaly (diagnosed age 45)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "20 mg oral daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25 mg oral daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "90 mcg, 2 puffs inhaled every 4-6 hours PRN for shortness of breath",
   "name": "Albuterol HFA inhaler"
  }
 ],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Asthma",
  "Acromegaly secondary to pituitary macroadenoma",
  "Alcohol use disorder",
  "Granulomatosis with polyangiitis",
  "Microscopic polyangiitis",
  "Diffuse large B-cell lymphoma"
 ],
 "age_at_first_encounter": 50
}
```

Primary diagnoses: ["Tumor lysis syndrome", "Microscopic polyangiitis", "Granulomatosis with polyangiitis", "Acromegaly", "Rotator cuff tear", "Severe acute pancreatitis (alcohol-induced)", "Pituitary apoplexy with secondary adrenal insufficiency (adrenal crisis)"]
Comorbidities: ["Acute kidney injury", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2022-09-05, outpatient, Endocrinology

Chief complaint: Six-month history of progressive joint pain and stiffness in knees and hands, with tightening of wedding ring and increase in shoe size.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of progressive joint pain and stiffness in knees and hands, with tightening of wedding ring and increase in shoe size.

HISTORY OF PRESENT ILLNESS:
He reports that his symptoms are most pronounced in the morning and involve his knees and hands. He also mentions that his wedding ring no longer fits, and he has noticed an increase in shoe size.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Asthma
- Acromegaly secondary to pituitary macroadenoma
- Alcohol use disorder
- Granulomatosis with polyangiitis
- Microscopic polyangiitis
- Diffuse large B-cell lymphoma

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg oral daily
- Hydrochlorothiazide 25 mg oral daily
- Albuterol HFA inhaler 90 mcg, 2 puffs inhaled every 4-6 hours PRN for shortness of breath

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, stroke at age 68
- Mother: Autoimmune thyroid disease
- Brother: Acromegaly (diagnosed age 45)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Chronic heavy daily use, 6-12 beers per day with recent binge episodes. Occupation: Construction supervisor.

VITAL SIGNS:
On examination, his blood pressure is 140/85 mmHg, heart rate is 78 bpm

PHYSICAL EXAMINATION:
He appears to have an enlarged jaw and tongue. His hands are noted to have soft tissue swelling, and his knee examination reveals bony enlargement with preserved range of motion.

LABORATORY DATA:
Laboratory tests reveal the following:
- Serum insulin-like growth factor 1 (IGF-1): Elevated
- Serum growth hormone (GH): Elevated
- Oral glucose tolerance test: No suppression of GH

IMAGING:
Radiographs of the hands and knees show widened joint spaces without significant osteophyte formation.

## Encounter 1: source 2020-07-15, practice 2023-03-08, outpatient, Pulmonology

Chief complaint: Two-month history of persistent sinusitis unresponsive to antibiotics, productive cough with occasional hemoptysis, and 10-pound unintentional weight loss.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Two-month history of persistent sinusitis unresponsive to antibiotics, productive cough with occasional hemoptysis, and 10-pound unintentional weight loss.

HISTORY OF PRESENT ILLNESS:
This 50-year-old male with a history of acromegaly diagnosed six months prior presents to the Pulmonology clinic with a 2-month history of persistent sinusitis unresponsive to antibiotics, a productive cough with occasional hemoptysis, and a 10-pound unintentional weight loss. He also reports generalized malaise and a new onset of joint pains.

PAST MEDICAL HISTORY:
Active Problem List:
- Acromegaly (diagnosed 2020-01-15)
- Hypertension
- Asthma
- Acromegaly secondary to pituitary macroadenoma
- Alcohol use disorder
- Granulomatosis with polyangiitis
- Microscopic polyangiitis
- Diffuse large B-cell lymphoma

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes albuterol inhaler as needed.

Home Medications:
- Lisinopril 20 mg oral daily
- Hydrochlorothiazide 25 mg oral daily
- Albuterol HFA inhaler 90 mcg, 2 puffs inhaled every 4-6 hours PRN for shortness of breath

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, stroke at age 68
- Mother: Autoimmune thyroid disease
- Brother: Acromegaly (diagnosed age 45)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Chronic heavy daily use, 6-12 beers per day with recent binge episodes. Occupation: Construction supervisor.

VITAL SIGNS:
Afebrile, blood pressure is 145/90 mmHg

PHYSICAL EXAMINATION:
Diffuse wheezes on lung auscultation. There is no skin rash noted.

LABORATORY DATA:
Urinalysis shows red blood cell casts, and serum creatinine is elevated at 2.8 mg/dL from a baseline of 1.0 mg/dL a year ago. His complete blood count reveals normocytic anemia, and his erythrocyte sedimentation rate is elevated.

IMAGING:
Chest X-ray demonstrates bilateral nodular infiltrates.

## Encounter 2: source 2021-03-15, practice 2023-10-27, ed, Emergency Medicine

Chief complaint: Two-week history of fatigue, decreased urine output, and new-onset bilateral lower extremity edema with dark-colored urine.

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Emily Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-week history of fatigue, decreased urine output, and new-onset bilateral lower extremity edema with dark-colored urine.

HISTORY OF PRESENT ILLNESS:
The patient is a 51-year-old male with a history of granulomatosis with polyangiitis who presents to the emergency department with a 2-week history of fatigue, decreased urine output, and new-onset swelling in his lower extremities. He also reports dark-colored urine over the past 3 days.

PAST MEDICAL HISTORY:
Active Problem List:
- Acromegaly (diagnosed 2020-01-15)
- Granulomatosis with polyangiitis (diagnosed 2020-07-15)
- Hypertension
- Asthma
- Acromegaly secondary to pituitary macroadenoma
- Alcohol use disorder
- Microscopic polyangiitis
- Diffuse large B-cell lymphoma

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg oral daily
- Hydrochlorothiazide 25 mg oral daily
- Albuterol HFA inhaler 90 mcg, 2 puffs inhaled every 4-6 hours PRN for shortness of breath

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, stroke at age 68
- Mother: Autoimmune thyroid disease
- Brother: Acromegaly (diagnosed age 45)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Chronic heavy daily use, 6-12 beers per day with recent binge episodes. Occupation: Construction supervisor.

REVIEW OF SYSTEMS:
He denies sinus congestion, ear pain, or rashes.

VITAL SIGNS:
Vital signs show blood pressure of 155/95 mm Hg.

PHYSICAL EXAMINATION:
Physical exam reveals bilateral crackles in the lungs and 2+ pitting edema in both legs.

LABORATORY DATA:
Urinalysis shows proteinuria and numerous red blood cells. Serum creatinine is 4.6 mg/dL, and C-reactive protein is elevated. A p-ANCA titer is positive.

OTHER STUDIES:
A biopsy is performed.

## Encounter 3: source 2021-11-15, practice 2024-07-02, inpatient, Gastroenterology

Chief complaint: Severe epigastric abdominal pain radiating to the back with nausea and vomiting following heavy alcohol consumption.

ENCOUNTER NOTE
Date: 2021-11-15
Provider: Dr. David Kim, MD
Department: Gastroenterology
Visit Type: inpatient

CHIEF COMPLAINT:
Severe epigastric abdominal pain radiating to the back with nausea and vomiting following heavy alcohol consumption.

HISTORY OF PRESENT ILLNESS:
The patient with acromegaly, granulomatosis with polyangiitis, microscopic polyangiitis, and alcohol use disorder presents to the emergency department with severe epigastric abdominal pain radiating to the back that started 24 hours ago, associated with nausea and vomiting. He admits to heavy alcohol consumption over the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Acromegaly (diagnosed 2020-01-15)
- Granulomatosis with polyangiitis (diagnosed 2020-07-15)
- Microscopic polyangiitis (diagnosed 2021-03-15)
- Hypertension
- Asthma
- Acromegaly secondary to pituitary macroadenoma
- Alcohol use disorder
- Diffuse large B-cell lymphoma

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg oral daily
- Hydrochlorothiazide 25 mg oral daily
- Albuterol HFA inhaler 90 mcg, 2 puffs inhaled every 4-6 hours PRN for shortness of breath

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, stroke at age 68
- Mother: Autoimmune thyroid disease
- Brother: Acromegaly (diagnosed age 45)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Chronic heavy daily use, 6-12 beers per day with recent binge episodes. Occupation: Construction supervisor.

VITAL SIGNS:
On examination, his temperature is 38.3°C (100.9°F), heart rate is 110 beats per minute, respiratory rate is 22 breaths per minute, and blood pressure is 90/60 mmHg.

PHYSICAL EXAMINATION:
His abdomen is distended, with diffuse tenderness and decreased bowel sounds.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 16,000/mm³
- Hematocrit: 48% (normal: 38.3-48.6% for males)
- Blood urea nitrogen (BUN): 36 mg/dL (normal: 7-20 mg/dL)
- Serum amylase: 1200 U/L (normal: 30-110 U/L)
- Serum lipase: 2500 U/L (normal: 0-160 U/L)
- Serum calcium: 7.8 mg/dL (normal: 8.5-10.2 mg/dL)
- C-reactive protein (CRP): 150 mg/L (normal: <10 mg/L)

IMAGING:
Abdominal ultrasound shows no gallstones or biliary ductal dilatation.

ASSESSMENT AND PLAN:
The patient&#x27;s condition deteriorates over the next 48 hours with persistent hypotension despite aggressive fluid resuscitation, oliguria, and increasing oxygen requirements.

## Encounter 4: source 2022-05-15, practice 2025-01-09, icu, Emergency Medicine

Chief complaint: Abrupt-onset severe headache described as 'worst headache of life' with associated nausea, blurred vision, and confusion.

ENCOUNTER NOTE
Date: 2022-05-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Abrupt-onset severe headache described as 'worst headache of life' with associated nausea, blurred vision, and confusion.

HISTORY OF PRESENT ILLNESS:
The patient with known pituitary macroadenoma and systemic vasculitides presents with acute neurological symptoms. He describes it as the "worst headache" he has ever experienced, with associated nausea and blurred vision. His wife reports that he seemed confused and had difficulty recognizing objects around him.

PAST MEDICAL HISTORY:
Active Problem List:
- Acromegaly (diagnosed 2020-01-15)
- Granulomatosis with polyangiitis (diagnosed 2020-07-15)
- Microscopic polyangiitis (diagnosed 2021-03-15)
- Severe acute pancreatitis (alcohol-induced) (diagnosed 2021-11-15)
- Hypertension
- Asthma
- Acromegaly secondary to pituitary macroadenoma
- Alcohol use disorder
- Diffuse large B-cell lymphoma

He has a history of hypertension and was recently evaluated for progressive fatigue, decreased libido, and mild weight gain, but no treatment was initiated.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes no anticoagulants

Home Medications:
- Lisinopril 20 mg oral daily
- Hydrochlorothiazide 25 mg oral daily
- Albuterol HFA inhaler 90 mcg, 2 puffs inhaled every 4-6 hours PRN for shortness of breath

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, stroke at age 68
- Mother: Autoimmune thyroid disease
- Brother: Acromegaly (diagnosed age 45)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Chronic heavy daily use, 6-12 beers per day with recent binge episodes. Occupation: Construction supervisor.

VITAL SIGNS:
Temperature is 36.8 °C (98.2 °F), blood pressure is 88/54 mmHg, pulse is 118/min, and respiratory rate is 22/min.

PHYSICAL EXAMINATION:
On examination, the patient is diaphoretic and restless. He has bilateral ptosis with impaired extraocular movements and reduced peripheral vision. The heart and lungs are clear.

LABORATORY DATA:
Laboratory studies show mild hyponatremia and hypoglycemia.

IMAGING:
MRI of the brain reveals an enlarged sellar mass with evidence of recent hemorrhage.

## Encounter 5: source 2022-11-15, practice 2025-07-03, ed, Emergency Medicine

Chief complaint: Nausea, vomiting, and lethargy diagnosed one week after high-grade B-cell lymphoma diagnosis and two days prior to scheduled chemotherapy.

ENCOUNTER NOTE
Date: 2022-11-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Nausea, vomiting, and lethargy diagnosed one week after high-grade B-cell lymphoma diagnosis and two days prior to scheduled chemotherapy.

HISTORY OF PRESENT ILLNESS:
The patient is a 52-year-old male with a history of acromegaly secondary to pituitary macroadenoma, granulomatosis with polyangiitis, microscopic polyangiitis, alcohol use disorder, and recent pituitary apoplexy who presents with nausea, vomiting, and lethargy. He was diagnosed with high-grade B-cell lymphoma one week ago and is scheduled to start chemotherapy in two days. He reports no fever, cough, or diarrhea.

PAST MEDICAL HISTORY:
Active Problem List:
- Acromegaly (diagnosed 2020-01-15)
- Granulomatosis with polyangiitis (diagnosed 2020-07-15)
- Microscopic polyangiitis (diagnosed 2021-03-15)
- Severe acute pancreatitis (alcohol-induced) (diagnosed 2021-11-15)
- Pituitary apoplexy with secondary adrenal insufficiency (adrenal crisis) (diagnosed 2022-05-15)
- Hypertension
- Asthma
- Acromegaly secondary to pituitary macroadenoma
- Alcohol use disorder
- Diffuse large B-cell lymphoma

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Lisinopril 20 mg oral daily
- Albuterol HFA inhaler 90 mcg, 2 puffs inhaled every 4-6 hours PRN for shortness of breath

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, stroke at age 68
- Mother: Autoimmune thyroid disease
- Brother: Acromegaly (diagnosed age 45)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Chronic heavy daily use, 6-12 beers per day with recent binge episodes. Occupation: Construction supervisor.

VITAL SIGNS:
Blood pressure is 145/90 mmHg, heart rate is 98 bpm

PHYSICAL EXAMINATION:
He is alert but appears fatigued. He has no palpable lymphadenopathy or hepatosplenomegaly.

LABORATORY DATA:
- Sodium: 138 mEq/L
- Potassium: 6.5 mEq/L
- Chloride: 100 mEq/L
- Bicarbonate: 22 mEq/L
- Blood urea nitrogen: 28 mg/dL
- Creatinine: 2.1 mg/dL (increased from a baseline of 1.0 mg/dL)
- Uric acid: 12 mg/dL
- Calcium: 7.8 mg/dL
- Phosphorus: 6.5 mg/dL

OTHER STUDIES:
An electrocardiogram shows peaked T waves.

## Encounter 6: source 2022-12-15, practice 2025-08-09, icu, Hematology/Oncology

Chief complaint: Nausea, vomiting, lethargy, and muscle cramps occurring 48 hours after receiving first cycle of R-CHOP chemotherapy.

ENCOUNTER NOTE
Date: 2022-12-15
Provider: Dr. Fatima Al-Hassan, MD
Department: Hematology/Oncology
Visit Type: icu

CHIEF COMPLAINT:
Nausea, vomiting, lethargy, and muscle cramps occurring 48 hours after receiving first cycle of R-CHOP chemotherapy.

HISTORY OF PRESENT ILLNESS:
The 52-year-old with diffuse large B-cell lymphoma and prior tumor lysis syndrome presents 48 hours after his first R-CHOP cycle. He appears fatigued and complains of muscle cramps.

PAST MEDICAL HISTORY:
Active Problem List:
- Acromegaly (diagnosed 2020-01-15)
- Granulomatosis with polyangiitis (diagnosed 2020-07-15)
- Microscopic polyangiitis (diagnosed 2021-03-15)
- Severe acute pancreatitis (alcohol-induced) (diagnosed 2021-11-15)
- Pituitary apoplexy with secondary adrenal insufficiency (adrenal crisis) (diagnosed 2022-05-15)
- Tumor lysis syndrome (diagnosed 2022-11-15)
- Hypertension
- Asthma
- Acromegaly secondary to pituitary macroadenoma
- Alcohol use disorder
- Diffuse large B-cell lymphoma

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg oral daily
- Hydrochlorothiazide 25 mg oral daily
- Albuterol HFA inhaler 90 mcg, 2 puffs inhaled every 4-6 hours PRN for shortness of breath

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, stroke at age 68
- Mother: Autoimmune thyroid disease
- Brother: Acromegaly (diagnosed age 45)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Chronic heavy daily use, 6-12 beers per day with recent binge episodes. Occupation: Construction supervisor.

VITAL SIGNS:
His temperature is 37.2°C (99°F), blood pressure is 130/85 mmHg, pulse is 98/min, and respirations are 18/min.

PHYSICAL EXAMINATION:
Physical examination reveals mild flank tenderness.

LABORATORY DATA:
Laboratory studies are as follows:

- Sodium: 138 mEq/L
- Potassium: 6.2 mEq/L
- Chloride: 100 mEq/L
- Bicarbonate: 22 mEq/L
- Blood urea nitrogen: 28 mg/dL
- Creatinine: 2.1 mg/dL (baseline 0.9 mg/dL)
- Calcium: 7.2 mg/dL
- Phosphorus: 6.5 mg/dL
- Uric acid: 12 mg/dL

OTHER STUDIES:
An electrocardiogram shows peaked T waves.

## Encounter 7: source 2023-07-15, practice 2026-03-14, ed, Orthopedics

Chief complaint: Severe pain and limited range of motion in right shoulder after falling from a ladder onto the shoulder.

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Robert Thompson, MD
Department: Orthopedics
Visit Type: ed

CHIEF COMPLAINT:
Severe pain and limited range of motion in right shoulder after falling from a ladder onto the shoulder.

HISTORY OF PRESENT ILLNESS:
The patient with acromegaly, granulomatosis with polyangiitis, and diffuse large B-cell lymphoma presents after a fall. He reports that he landed directly on his shoulder and felt immediate pain. After reduction of the dislocation, the patient still has difficulty with shoulder abduction and weakness on resisted external rotation.

PAST MEDICAL HISTORY:
Active Problem List:
- Acromegaly (diagnosed 2020-01-15)
- Granulomatosis with polyangiitis (diagnosed 2020-07-15)
- Microscopic polyangiitis (diagnosed 2021-03-15)
- Severe acute pancreatitis (alcohol-induced) (diagnosed 2021-11-15)
- Pituitary apoplexy with secondary adrenal insufficiency (adrenal crisis) (diagnosed 2022-05-15)
- Tumor lysis syndrome (diagnosed 2022-11-15)
- Hypertension
- Asthma
- Acromegaly secondary to pituitary macroadenoma
- Alcohol use disorder
- Diffuse large B-cell lymphoma

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes lisinopril.

Home Medications:
- Hydrochlorothiazide 25 mg oral daily
- Albuterol HFA inhaler 90 mcg, 2 puffs inhaled every 4-6 hours PRN for shortness of breath

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, stroke at age 68
- Mother: Autoimmune thyroid disease
- Brother: Acromegaly (diagnosed age 45)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Chronic heavy daily use, 6-12 beers per day with recent binge episodes. Occupation: Construction supervisor.

PHYSICAL EXAMINATION:
The right shoulder appears squared off and there is a prominent acromion. There is tenderness over the anterior aspect of the shoulder. The patient is unable to abduct his right arm beyond 30 degrees and has weakness on resisted external rotation. There is no sensory deficit noted in the affected arm.

IMAGING:
An X-ray of the shoulder shows an anterior dislocation without fracture.
