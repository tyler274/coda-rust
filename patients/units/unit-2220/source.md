# synthetic_hospital patient 2220

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 69 at first encounter, sex M, Non-Hispanic White, Medicare with supplemental private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-02-01 (Tue) | outpatient | Cardiology |
| 1 | 2020-04-15 | 2022-04-28 (Thu) | outpatient | Gastroenterology |
| 2 | 2020-07-15 | 2022-07-22 (Fri) | ed | Emergency Medicine |
| 3 | 2021-01-15 | 2023-01-30 (Mon) | ed | Emergency Medicine |
| 4 | 2021-07-15 | 2023-07-21 (Fri) | ed | Emergency Medicine |
| 5 | 2022-07-15 | 2024-08-01 (Thu) | ed | Emergency Medicine |
| 6 | 2023-07-15 | 2025-07-27 (Sun) | inpatient | Neurology |
| 7 | 2024-07-15 | 2026-08-03 (Mon) | icu | Neurocritical Care |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare with supplemental private insurance",
 "occupation": "Retired carpenter",
 "alcohol_use": "Occasional social use, 1-2 drinks per month",
 "family_history": [
  "Father: Myocardial infarction at age 72",
  "Mother: Ischemic stroke at age 78",
  "Paternal uncle: Colon cancer diagnosed age 75"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 10 years ago after 30 pack-year history",
 "home_medications": [
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "40 mg PO daily at bedtime",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "5 mg PO daily (dose adjusted per INR)",
   "name": "Warfarin"
  },
  {
   "dose": "30 mg PO twice daily",
   "name": "Morphine Sulfate SR"
  }
 ],
 "surgical_history": [
  "Open appendectomy",
  "Diagnostic cardiac catheterization"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Atrial Fibrillation",
  "Coronary Artery Disease",
  "Chronic lower back pain"
 ],
 "age_at_first_encounter": 69
}
```

Primary diagnoses: ["Acute ischemic stroke", "Atrial fibrillation", "Intracerebral hemorrhage with airway compromise", "Right-sided colon cancer", "Acute Appendicitis"]
Comorbidities: ["Atrial fibrillation", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2022-02-01, outpatient, Cardiology

Chief complaint: Intermittent palpitations and fatigue for 2 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent palpitations and fatigue for 2 months

HISTORY OF PRESENT ILLNESS:
He reports a 2-month history of intermittent palpitations and fatigue. He denies any chest pain, dyspnea, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Atrial Fibrillation
- Coronary Artery Disease
- Chronic lower back pain

PAST SURGICAL HISTORY:
- Open appendectomy
- Diagnostic cardiac catheterization

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Aspirin 81 mg PO daily
- Warfarin 5 mg PO daily (dose adjusted per INR)
- Morphine Sulfate SR 30 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 72
- Mother: Ischemic stroke at age 78
- Paternal uncle: Colon cancer diagnosed age 75

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago after 30 pack-year history. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retired carpenter.

PHYSICAL EXAMINATION:
On physical examination, his blood pressure is 142/88 mmHg, heart rate is irregularly irregular at 88 bpm, and there are no additional heart sounds or murmurs.

LABORATORY DATA:
His laboratory values are as follows:

- Hemoglobin A1c: 7.2%
- Serum creatinine: 1.1 mg/dL
- INR: 1.0

OTHER STUDIES:
An electrocardiogram (ECG) confirms atrial fibrillation (AF).

ASSESSMENT AND PLAN:
Based on the CHA2DS2-VASc score,

## Encounter 1: source 2020-04-15, practice 2022-04-28, outpatient, Gastroenterology

Chief complaint: Unintentional weight loss and intermittent abdominal discomfort over 3 months

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Aisha Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Unintentional weight loss and intermittent abdominal discomfort over 3 months

HISTORY OF PRESENT ILLNESS:
This 69-year-old male with hypertension, type 2 diabetes mellitus, atrial fibrillation, coronary artery disease, and chronic lower back pain presents to Gastroenterology three months following his January 2020 Cardiology evaluation. He presents for evaluation following unintentional weight loss and intermittent abdominal discomfort over the past 3 months. He reports changes in bowel habits, including occasional loose stools and a sensation of incomplete evacuation. He denies overt rectal bleeding but describes dark stools on a few occasions.

PAST MEDICAL HISTORY:
Active Problem List:
- Atrial fibrillation (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Coronary Artery Disease
- Chronic lower back pain

Past medical history includes type 2 diabetes and hypertension, both well controlled.

PAST SURGICAL HISTORY:
- Open appendectomy
- Diagnostic cardiac catheterization

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily at bedtime
- Aspirin 81 mg PO daily
- Warfarin 5 mg PO daily (dose adjusted per INR)
- Morphine Sulfate SR 30 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 72
- Mother: Ischemic stroke at age 78
- Paternal uncle: Colon cancer diagnosed age 75

SOCIAL HISTORY:
He worked as a carpenter for 40 years and quit smoking 10 years ago after a 30-pack-year history.

LABORATORY DATA:
Laboratory testing reveals iron deficiency anemia.

IMAGING:
CT scan shows multiple hypodense lesions in the right hepatic lobe and several small pulmonary nodules in both lungs.

PATHOLOGY:
Biopsy confirms moderately differentiated adenocarcinoma

OTHER STUDIES:
Colonoscopy reveals an ulcerated, obstructing mass in the ascending colon

PLAN:
The patient is referred to oncology for further management.

## Encounter 2: source 2020-07-15, practice 2022-07-22, ed, Emergency Medicine

Chief complaint: Sudden onset of palpitations and lightheadedness

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of palpitations and lightheadedness

HISTORY OF PRESENT ILLNESS:
The patient with atrial fibrillation and colon cancer returns to the emergency department due to a sudden onset of palpitations and lightheadedness that started 3 hours ago

PAST MEDICAL HISTORY:
Active Problem List:
- Atrial fibrillation (diagnosed 2020-01-15)
- Right-sided colon cancer (diagnosed 2020-04-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Coronary Artery Disease
- Chronic lower back pain

He has a history of long-standing hypertension and coronary artery disease.

PAST SURGICAL HISTORY:
- Open appendectomy
- Diagnostic cardiac catheterization

MEDICATIONS:
His medications include lisinopril and aspirin.

Home Medications:
- Metformin 1000 mg PO twice daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily at bedtime
- Warfarin 5 mg PO daily (dose adjusted per INR)
- Morphine Sulfate SR 30 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 72
- Mother: Ischemic stroke at age 78
- Paternal uncle: Colon cancer diagnosed age 75

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago after 30 pack-year history. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retired carpenter.

VITAL SIGNS:
On examination, his pulse is 120/min and irregularly irregular. Blood pressure is 130/84 mm Hg.

PHYSICAL EXAMINATION:
He has no jugular venous distention or peripheral edema.

OTHER STUDIES:
ECG findings are shown below.

## Encounter 3: source 2021-01-15, practice 2023-01-30, ed, Emergency Medicine

Chief complaint: Palpitations and mild dyspnea

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Maria Gonzalez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Palpitations and mild dyspnea

HISTORY OF PRESENT ILLNESS:
Patient with recurrent atrial fibrillation presents to the emergency department with palpitations and mild dyspnea that started 3 hours ago. He denies chest pain, dizziness, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Atrial fibrillation (diagnosed 2020-01-15)
- Right-sided colon cancer (diagnosed 2020-04-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Coronary Artery Disease
- Chronic lower back pain

PAST SURGICAL HISTORY:
- Open appendectomy
- Diagnostic cardiac catheterization

MEDICATIONS:
His medications include metformin, lisinopril, and hydrochlorothiazide

Home Medications:
- Atorvastatin 40 mg PO daily at bedtime
- Aspirin 81 mg PO daily
- Warfarin 5 mg PO daily (dose adjusted per INR)
- Morphine Sulfate SR 30 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 72
- Mother: Ischemic stroke at age 78
- Paternal uncle: Colon cancer diagnosed age 75

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago after 30 pack-year history. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retired carpenter.

VITAL SIGNS:
His blood pressure is 145/90 mmHg, heart rate is irregularly irregular at 110 bpm, and oxygen saturation is 97% on room air

PHYSICAL EXAMINATION:
Lung auscultation reveals no crackles, and there is no lower extremity edema

LABORATORY DATA:
- Hemoglobin: 14 g/dL
- Creatinine: 1.2 mg/dL
- Potassium: 4.2 mEq/L
- Thyroid-stimulating hormone (TSH): 2.5 mIU/L

OTHER STUDIES:
An electrocardiogram (ECG) shows no p waves, irregular R-R intervals, and a ventricular rate of approximately 110 bpm

## Encounter 4: source 2021-07-15, practice 2023-07-21, ed, Emergency Medicine

Chief complaint: Acute-onset right lower quadrant abdominal pain

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute-onset right lower quadrant abdominal pain

HISTORY OF PRESENT ILLNESS:
Patient with colon cancer and atrial fibrillation presents with acute-onset right lower quadrant abdominal pain since the morning. He describes the pain as sharp and constant, worsening with movement.

PAST MEDICAL HISTORY:
Active Problem List:
- Atrial fibrillation (diagnosed 2020-01-15)
- Right-sided colon cancer (diagnosed 2020-04-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Coronary Artery Disease
- Chronic lower back pain

PAST SURGICAL HISTORY:
- Open appendectomy
- Diagnostic cardiac catheterization

MEDICATIONS:
He takes sustained-release morphine, lisinopril, metformin.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily at bedtime
- Aspirin 81 mg PO daily
- Warfarin 5 mg PO daily (dose adjusted per INR)
- Morphine Sulfate SR 30 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 72
- Mother: Ischemic stroke at age 78
- Paternal uncle: Colon cancer diagnosed age 75

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago after 30 pack-year history. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retired carpenter.

VITAL SIGNS:
Temperature is 37.8°C (100°F), heart rate is 102 beats per minute, respiratory rate is 28 breaths per minute, blood pressure is 138/86 mmHg, and oxygen saturation is 98% on room air

PHYSICAL EXAMINATION:
Abdominal examination reveals rebound tenderness and guarding in the right lower quadrant. Bowel sounds are normal.

LABORATORY DATA:
White blood cell count: 14,000/uL
- Hemoglobin: 13.2 g/dL
- Platelet count: 250,000/uL
- Serum creatinine: 1.1 mg/dL
- Blood urea nitrogen: 18 mg/dL
- Serum glucose: 160 mg/dL
- Arterial blood gas:
- pH: 7.52
- PaCO2: 28 mmHg
- HCO3-: 24 mEq/L
- PaO2: 95 mmHg

## Encounter 5: source 2022-07-15, practice 2024-08-01, ed, Emergency Medicine

Chief complaint: Sudden onset of left-sided weakness and slurred speech

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of left-sided weakness and slurred speech

HISTORY OF PRESENT ILLNESS:
This 71-year-old male with hypertension, diabetes, and history of atrial fibrillation (previously evaluated in Cardiology and multiple ED visits) presents to the emergency department with sudden onset of left-sided weakness and slurred speech that began two hours ago while watching television. His wife reports that he was in his usual state of health earlier in the day.

PAST MEDICAL HISTORY:
Active Problem List:
- Atrial fibrillation (diagnosed 2020-01-15)
- Right-sided colon cancer (diagnosed 2020-04-15)
- Acute Appendicitis (diagnosed 2021-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Coronary Artery Disease
- Chronic lower back pain

PAST SURGICAL HISTORY:
- Open appendectomy
- Diagnostic cardiac catheterization

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily at bedtime
- Aspirin 81 mg PO daily
- Warfarin 5 mg PO daily (dose adjusted per INR)
- Morphine Sulfate SR 30 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 72
- Mother: Ischemic stroke at age 78
- Paternal uncle: Colon cancer diagnosed age 75

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago after 30 pack-year history. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retired carpenter.

VITAL SIGNS:
On examination, the patient's blood pressure is 165/95 mmHg, heart rate is 88 beats per minute, and he is afebrile.

PHYSICAL EXAMINATION:
Neurological examination reveals left facial droop, left arm and leg weakness, and dysarthria. His NIH Stroke Scale score is 12.

IMAGING:
A noncontrast CT scan of the head is performed immediately and shows no hemorrhage or other acute abnormalities.

## Encounter 6: source 2023-07-15, practice 2025-07-27, inpatient, Neurology

Chief complaint: Acute onset of left-sided weakness and slurred speech with subsequent deterioration

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Emily Watson, MD
Department: Neurology
Visit Type: inpatient

CHIEF COMPLAINT:
Acute onset of left-sided weakness and slurred speech with subsequent deterioration

HISTORY OF PRESENT ILLNESS:
This 72-year-old male with atrial fibrillation and prior similar symptoms in 2022 presents to the emergency department with acute onset of left-sided weakness and slurred speech that began 3 hours ago. Forty-eight hours later, the patient becomes increasingly drowsy and confused.

PAST MEDICAL HISTORY:
Active Problem List:
- Atrial fibrillation (diagnosed 2020-01-15)
- Right-sided colon cancer (diagnosed 2020-04-15)
- Acute Appendicitis (diagnosed 2021-07-15)
- Acute ischemic stroke (diagnosed 2022-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Coronary Artery Disease
- Chronic lower back pain

PAST SURGICAL HISTORY:
- Open appendectomy
- Diagnostic cardiac catheterization

MEDICATIONS:
He takes the patient is administered intravenous alteplase.

Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily at bedtime
- Aspirin 81 mg PO daily
- Warfarin 5 mg PO daily (dose adjusted per INR)
- Morphine Sulfate SR 30 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 72
- Mother: Ischemic stroke at age 78
- Paternal uncle: Colon cancer diagnosed age 75

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago after 30 pack-year history. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retired carpenter.

VITAL SIGNS:
On examination, his blood pressure is 170/95 mmHg, heart rate is 92 beats per minute with an irregular rhythm, and he is afebrile. His blood pressure is now 155/85 mmHg, heart rate 88 beats per minute, and temperature 37.1°C.

PHYSICAL EXAMINATION:
Neurological examination reveals left facial droop, left arm and leg drift, and dysarthria. Repeat neurological examination shows worsening left-sided weakness.

IMAGING:
A noncontrast CT scan of the head shows no hemorrhage or mass effect. An emergent noncontrast CT scan of the head is obtained.

## Encounter 7: source 2024-07-15, practice 2026-08-03, icu, Neurocritical Care

Chief complaint: Sudden onset of left-sided weakness and slurred speech with airway compromise

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Rajesh Singh, MD
Department: Neurocritical Care
Visit Type: icu

CHIEF COMPLAINT:
Sudden onset of left-sided weakness and slurred speech with airway compromise

HISTORY OF PRESENT ILLNESS:
Patient with AF and prior strokes presents with left-sided weakness and slurred speech that began two hours ago. Wife reports he was in usual state of health until this morning.

PAST MEDICAL HISTORY:
Active Problem List:
- Atrial fibrillation (diagnosed 2020-01-15)
- Right-sided colon cancer (diagnosed 2020-04-15)
- Acute Appendicitis (diagnosed 2021-07-15)
- Acute ischemic stroke (diagnosed 2022-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Coronary Artery Disease
- Chronic lower back pain

PAST SURGICAL HISTORY:
- Open appendectomy
- Diagnostic cardiac catheterization

MEDICATIONS:
He takes on warfarin therapy.

Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily at bedtime
- Aspirin 81 mg PO daily
- Morphine Sulfate SR 30 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 72
- Mother: Ischemic stroke at age 78
- Paternal uncle: Colon cancer diagnosed age 75

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago after 30 pack-year history. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retired carpenter.

VITAL SIGNS:
With a blood pressure of 190/100 mmHg, heart rate of 78 beats per minute, and irregularly irregular rhythm.

PHYSICAL EXAMINATION:
On examination, the patient is alert but anxious. Neurological examination reveals left facial droop, left hemiparesis, and dysarthria.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 13.2 g/dL
- Platelets: 250,000/uL
- INR: 3.2
- Creatinine: 1.0 mg/dL
- Blood Urea Nitrogen (BUN): 14 mg/dL

IMAGING:
A non-contrast head CT scan shows a right basal ganglia hemorrhage without evidence of intraventricular extension.
