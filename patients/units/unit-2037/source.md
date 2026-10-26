# synthetic_hospital patient 2037

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 58 at first encounter, sex M, Non-Hispanic White, Medicare (disability) and state Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-06-07 (Mon) | outpatient | Gastroenterology |
| 1 | 2020-07-15 | 2021-12-03 (Fri) | outpatient | Neurology |
| 2 | 2021-07-15 | 2022-12-06 (Tue) | outpatient | Nephrology |
| 3 | 2022-07-15 | 2023-12-18 (Mon) | outpatient | Nephrology |
| 4 | 2023-07-15 | 2024-12-16 (Mon) | outpatient | Internal Medicine |
| 5 | 2024-07-15 | 2025-12-15 (Mon) | outpatient | Cardiology |
| 6 | 2025-01-15 | 2026-06-14 (Sun) | icu | Neurology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare (disability) and state Medicaid",
 "occupation": "Former warehouse supervisor, currently on medical disability",
 "alcohol_use": "No alcohol use",
 "family_history": [
  "Father: hypertension, cerebrovascular accident",
  "Mother: Parkinson's disease",
  "Sibling: chronic kidney disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "667 mg three times daily with meals",
   "name": "Calcium acetate"
  },
  {
   "dose": "25-100 mg three times daily",
   "name": "Carbidopa-levodopa"
  },
  {
   "dose": "800 mg three times daily as needed for back pain",
   "name": "Ibuprofen"
  },
  {
   "dose": "160/800 mg daily",
   "name": "Trimethoprim-sulfamethoxazole"
  },
  {
   "dose": "1 tablet daily",
   "name": "Multivitamin"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Chronic kidney disease stage 4",
  "Parkinson's disease",
  "Whipple disease",
  "Anemia of chronic kidney disease"
 ],
 "age_at_first_encounter": 58
}
```

Primary diagnoses: ["Cerebellar hemorrhage", "Chronic pyelonephritis", "Anemia of chronic kidney disease", "Chronic Kidney Disease with Volume Overload", "Parkinson disease with psychosis", "Whipple disease"]
Comorbidities: ["Essential hypertension", "Chronic kidney disease stage 4"]

## Encounter 0: source 2020-01-15, practice 2021-06-07, outpatient, Gastroenterology

Chief complaint: Intermittent arthralgias, unexplained weight loss of 10 kg, low-grade fevers, and chronic diarrhea

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent arthralgias, unexplained weight loss of 10 kg, low-grade fevers, and chronic diarrhea

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 6-month history of intermittent arthralgias, unexplained weight loss of 10 kg, low-grade fevers, and chronic diarrhea. He reports abdominal pain that is crampy in nature and has noticed a significant decrease in his appetite.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Chronic kidney disease stage 4
- Parkinson's disease
- Whipple disease
- Anemia of chronic kidney disease

His past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes lisinopril.

Home Medications:
- Calcium acetate 667 mg three times daily with meals
- Carbidopa-levodopa 25-100 mg three times daily
- Ibuprofen 800 mg three times daily as needed for back pain
- Trimethoprim-sulfamethoxazole 160/800 mg daily
- Multivitamin 1 tablet daily

ALLERGIES:
He has no known drug allergies

FAMILY HISTORY:
- Father: hypertension, cerebrovascular accident
- Mother: Parkinson's disease
- Sibling: chronic kidney disease

SOCIAL HISTORY:
He does not smoke or consume alcohol

VITAL SIGNS:
On physical examination, his temperature is 37.8°C (100°F), blood pressure is 130/85 mmHg, and heart rate is 88 beats per minute

PHYSICAL EXAMINATION:
Abdominal examination reveals mild diffuse tenderness without rebound or guarding. Bowel sounds are normal.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 11.2 g/dL (Normal: 13.8-17.2 g/dL)
- White blood cell count: 7,000/mm³ (Normal: 4,500-11,000/mm³)
- Platelet count: 250,000/mm³ (Normal: 150,000-450,000/mm³)
- Erythrocyte sedimentation rate (ESR): 45 mm/hr (Normal: 0-22 mm/hr)
- C-reactive protein (CRP): 25 mg/L (Normal: <10 mg/L)
- Serum albumin: 3.2 g/dL (Normal: 3.5-5.0 g/dL)

PATHOLOGY:
Biopsy of the small intestine shows periodic acid-Schiff (PAS)-positive material in the lamina propria

OTHER STUDIES:
An upper endoscopy is performed

## Encounter 1: source 2020-07-15, practice 2021-12-03, outpatient, Neurology

Chief complaint: 6-month history of a shaking sensation in his right hand

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Okafor, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of a shaking sensation in his right hand

HISTORY OF PRESENT ILLNESS:
The patient returns six months following his Whipple disease diagnosis. He reports that the tremor is more noticeable at rest and improves with voluntary movement. He also mentions that his handwriting has become smaller and that he has difficulty with buttoning his shirt.

PAST MEDICAL HISTORY:
Active Problem List:
- Whipple disease (diagnosed 2020-01-15)
- Hypertension
- Chronic kidney disease stage 4
- Parkinson's disease
- Anemia of chronic kidney disease

His past medical history is significant for hypertension, controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Lisinopril 20 mg daily
- Calcium acetate 667 mg three times daily with meals
- Carbidopa-levodopa 25-100 mg three times daily
- Ibuprofen 800 mg three times daily as needed for back pain
- Trimethoprim-sulfamethoxazole 160/800 mg daily
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: hypertension, cerebrovascular accident
- Mother: Parkinson's disease
- Sibling: chronic kidney disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Former warehouse supervisor, currently on medical disability.

PHYSICAL EXAMINATION:
On examination, you observe a 5-7 Hz resting tremor in his right hand, which is asymmetric. There is also noticeable rigidity when the right wrist is passively moved. His gait is normal, and there are no signs of ataxia.

LABORATORY DATA:
Routine blood work, including complete blood count, electrolytes, and thyroid function tests, are within normal limits.

CBC:
- Hemoglobin: 14.2 g/dL
- WBC: 6.8 x 10^3/uL
- Platelets: 250 x 10^3/uL

Electrolytes:
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 102 mEq/L
- Bicarbonate: 24 mEq/L

Thyroid Function Tests:
- TSH: 2.1 mIU/L
- Free T4: 1.2 ng/dL

## Encounter 2: source 2021-07-15, practice 2022-12-06, outpatient, Nephrology

Chief complaint: 2-year history of progressively worsening fatigue and occasional headaches

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Maria Rodriguez, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
2-year history of progressively worsening fatigue and occasional headaches

PAST MEDICAL HISTORY:
Active Problem List:
- Whipple disease (diagnosed 2020-01-15)
- Parkinson disease with psychosis (diagnosed 2020-07-15)
- Hypertension
- Chronic kidney disease stage 4
- Parkinson's disease
- Anemia of chronic kidney disease

He has a past medical history of hypertension, which he admits to managing inconsistently due to financial constraints.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
His current medications include hydrochlorothiazide and occasional ibuprofen for headaches.

Home Medications:
- Lisinopril 20 mg daily
- Calcium acetate 667 mg three times daily with meals
- Carbidopa-levodopa 25-100 mg three times daily
- Trimethoprim-sulfamethoxazole 160/800 mg daily
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: hypertension, cerebrovascular accident
- Mother: Parkinson's disease
- Sibling: chronic kidney disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Former warehouse supervisor, currently on medical disability.

PHYSICAL EXAMINATION:
On physical examination, his blood pressure is 162/98 mmHg, and there is no evidence of edema. Fundoscopic examination reveals mild arteriolar narrowing without exudates or hemorrhages.

LABORATORY DATA:
Laboratory findings are as follows:

- Serum creatinine: 2.8 mg/dL (increased from 1.4 mg/dL six months ago)
- Blood urea nitrogen (BUN): 44 mg/dL
- Urinalysis: Trace protein, no casts or cells
- Serum potassium: 4.2 mEq/L
- Hemoglobin: 12.1 g/dL
- Estimated Glomerular Filtration Rate (eGFR): 30 mL/min/1.73m^2

IMAGING:
Abdominal ultrasound shows bilateral small kidneys with increased echogenicity.

## Encounter 3: source 2022-07-15, practice 2023-12-18, outpatient, Nephrology

Chief complaint: Routine follow-up visit with increased fatigue

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Maria Rodriguez, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up visit with increased fatigue

HISTORY OF PRESENT ILLNESS:
He returns for nephrology follow-up with Whipple disease, Parkinson disease, and chronic kidney disease. He reports increased fatigue for two months and denies chest pain, shortness of breath, or leg swelling.

PAST MEDICAL HISTORY:
Active Problem List:
- Whipple disease (diagnosed 2020-01-15)
- Parkinson disease with psychosis (diagnosed 2020-07-15)
- Chronic Kidney Disease with Volume Overload (diagnosed 2021-07-15)
- Hypertension
- Chronic kidney disease stage 4
- Parkinson's disease
- Anemia of chronic kidney disease

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
His medications include lisinopril, calcium acetate, and a multivitamin.

Home Medications:
- Carbidopa-levodopa 25-100 mg three times daily
- Ibuprofen 800 mg three times daily as needed for back pain
- Trimethoprim-sulfamethoxazole 160/800 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: hypertension, cerebrovascular accident
- Mother: Parkinson's disease
- Sibling: chronic kidney disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Former warehouse supervisor, currently on medical disability.

VITAL SIGNS:
His blood pressure is 142/88 mmHg.

PHYSICAL EXAMINATION:
Physical examination is unremarkable except for pallor.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 8.5 g/dL (Normal: 13.8-17.2 g/dL)
- Hematocrit: 25.5% (Normal: 40.7-50.3%)
- Mean corpuscular volume (MCV): 86 fL (Normal: 80-96 fL)
- Serum creatinine: 4.2 mg/dL (Normal: 0.6-1.2 mg/dL)
- Estimated glomerular filtration rate (eGFR): 18 mL/min/1.73m^2 (Normal: >90 mL/min/1.73m^2)
- Blood urea nitrogen (BUN): 54 mg/dL (Normal: 6-20 mg/dL)

PLAN:
Given the patient's CKD and anemia, you decide to initiate treatment with erythropoietin.

## Encounter 4: source 2023-07-15, practice 2024-12-16, outpatient, Internal Medicine

Chief complaint: Fatigue, generalized weakness, and bilateral flank pain

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Sarah Johnson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, generalized weakness, and bilateral flank pain

HISTORY OF PRESENT ILLNESS:
The patient is a 61-year-old male with established Whipple disease, Parkinson disease with psychosis, and chronic kidney disease stage 4 previously evaluated by Nephrology for volume overload and anemia, who presents to the clinic with a 2-month history of fatigue, generalized weakness, and a recent onset of dull, bilateral flank pain. He reports no changes in urine color or frequency but notes that his urine has been frothy. He has been taking over-the-counter ibuprofen 800 mg three times daily for the past 5 years to manage his back pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Whipple disease (diagnosed 2020-01-15)
- Parkinson disease with psychosis (diagnosed 2020-07-15)
- Chronic Kidney Disease with Volume Overload (diagnosed 2021-07-15)
- Anemia of chronic kidney disease (diagnosed 2022-07-15)
- Hypertension
- Chronic kidney disease stage 4
- Parkinson's disease

His medical history is significant for hypertension, controlled with lisinopril.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes over-the-counter ibuprofen 800 mg three times daily, lisinopril.

Home Medications:
- Calcium acetate 667 mg three times daily with meals
- Carbidopa-levodopa 25-100 mg three times daily
- Trimethoprim-sulfamethoxazole 160/800 mg daily
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: hypertension, cerebrovascular accident
- Mother: Parkinson's disease
- Sibling: chronic kidney disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Former warehouse supervisor, currently on medical disability.

VITAL SIGNS:
Blood pressure is 145/90 mmHg

PHYSICAL EXAMINATION:
On physical examination, his blood pressure is 145/90 mmHg, and there is mild bilateral costovertebral angle tenderness.

LABORATORY DATA:
- Serum creatinine: 2.8 mg/dL (increased from a baseline of 1.0 mg/dL one year ago)
- Blood urea nitrogen (BUN): 45 mg/dL
- Urinalysis: Proteinuria, no hematuria, no casts
- Urine protein/creatinine ratio: 2.1 mg/mg
- Serum C-reactive protein (CRP): Normal

## Encounter 5: source 2024-07-15, practice 2025-12-15, outpatient, Cardiology

Chief complaint: Enrollment in randomized controlled trial assessing new antihypertensive medication

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Wei Zhang, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Enrollment in randomized controlled trial assessing new antihypertensive medication

PAST MEDICAL HISTORY:
Active Problem List:
- Whipple disease (diagnosed 2020-01-15)
- Parkinson disease with psychosis (diagnosed 2020-07-15)
- Chronic Kidney Disease with Volume Overload (diagnosed 2021-07-15)
- Anemia of chronic kidney disease (diagnosed 2022-07-15)
- Chronic pyelonephritis (diagnosed 2023-07-15)
- Hypertension
- Chronic kidney disease stage 4
- Parkinson's disease

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Calcium acetate 667 mg three times daily with meals
- Carbidopa-levodopa 25-100 mg three times daily
- Ibuprofen 800 mg three times daily as needed for back pain
- Trimethoprim-sulfamethoxazole 160/800 mg daily
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: hypertension, cerebrovascular accident
- Mother: Parkinson's disease
- Sibling: chronic kidney disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Former warehouse supervisor, currently on medical disability.

OTHER STUDIES:
The primary outcome is the reduction in systolic blood pressure after 6 months. The initial analysis uses a two-tailed alpha of 0.05, yielding a 95% confidence interval for the mean difference in systolic blood pressure of -8 mmHg to -2 mmHg. The study team then decides to repeat the analysis with a two-tailed alpha of 0.01 to address concerns about type I error. After recalculating, the new confidence interval for the mean difference is wider, now ranging from -9 mmHg to -1 mmHg. Investigators discuss the implications of this change for statistical inference and clinical interpretation.

## Encounter 6: source 2025-01-15, practice 2026-06-14, icu, Neurology

Chief complaint: Sudden onset of severe occipital headache and repeated vomiting

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Robert Chen, MD
Department: Neurology
Visit Type: icu

CHIEF COMPLAINT:
Sudden onset of severe occipital headache and repeated vomiting

HISTORY OF PRESENT ILLNESS:
This 63-year-old male with Parkinson's disease and stage 4 chronic kidney disease presents with acute neurological complaints. He also complains of dizziness and difficulty walking. His wife notes that he has been unsteady and tends to veer to the left when ambulating.

PAST MEDICAL HISTORY:
Active Problem List:
- Whipple disease (diagnosed 2020-01-15)
- Parkinson disease with psychosis (diagnosed 2020-07-15)
- Chronic Kidney Disease with Volume Overload (diagnosed 2021-07-15)
- Anemia of chronic kidney disease (diagnosed 2022-07-15)
- Chronic pyelonephritis (diagnosed 2023-07-15)
- Hypertension
- Chronic kidney disease stage 4
- Parkinson's disease

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Calcium acetate 667 mg three times daily with meals
- Carbidopa-levodopa 25-100 mg three times daily
- Ibuprofen 800 mg three times daily as needed for back pain
- Trimethoprim-sulfamethoxazole 160/800 mg daily
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: hypertension, cerebrovascular accident
- Mother: Parkinson's disease
- Sibling: chronic kidney disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Former warehouse supervisor, currently on medical disability.

VITAL SIGNS:
His blood pressure is 220/120 mmHg.

PHYSICAL EXAMINATION:
On examination, the patient appears distressed and diaphoretic. Neurological examination reveals nystagmus on lateral gaze, neck stiffness without Kernig's or Brudzinski's signs, and a pronounced tendency to fall to the left with attempts at sitting up or walking. There is no facial asymmetry, hemiparesis, or sensory deficit.

LABORATORY DATA:
- Hemoglobin: 14.2 g/dL
- White blood cell count: 7.8 x 10^3/uL
- Platelets: 250 x 10^3/uL
- INR: 1.0
- Serum glucose: 110 mg/dL

IMAGING:
A noncontrast head CT scan is performed immediately.
