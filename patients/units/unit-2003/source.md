# synthetic_hospital patient 2003

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 47 at first encounter, sex M, Non-Hispanic White, Commercial PPO (Employer-Sponsored).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-07-11 (Mon) | outpatient | Nephrology |
| 1 | 2020-03-15 | 2022-08-30 (Tue) | outpatient | Internal Medicine |
| 2 | 2020-07-15 | 2023-01-02 (Mon) | outpatient | Urology |
| 3 | 2021-03-15 | 2023-08-25 (Fri) | outpatient | Endocrinology |
| 4 | 2021-09-15 | 2024-03-04 (Mon) | outpatient | Cardiology |
| 5 | 2021-11-15 | 2024-04-30 (Tue) | ed | Emergency Medicine |
| 6 | 2022-07-15 | 2024-12-30 (Mon) | outpatient | Cardiology |
| 7 | 2023-07-15 | 2026-01-07 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO (Employer-Sponsored)",
 "occupation": "Financial Analyst",
 "alcohol_use": "Former heavy drinker, 3-4 drinks nightly, quit 8 months ago",
 "family_history": [
  "Mother: Medullary thyroid carcinoma diagnosed age 42, status post thyroidectomy",
  "Sister: Primary hyperparathyroidism",
  "Father: Coronary artery disease with MI at age 55",
  "Maternal cousin: Pheochromocytoma"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 1 pack per day for 22 years, quit 8 months ago",
 "home_medications": [
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Metoprolol Succinate"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Sertraline"
  },
  {
   "dose": "2 mg PO daily",
   "name": "Sirolimus"
  },
  {
   "dose": "20 mg PO BID",
   "name": "Propranolol"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Renal transplantation (living unrelated donor, 6 months prior)",
  "Left parathyroidectomy (partial, 2 years prior)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Major Depressive Disorder",
  "Generalized Anxiety Disorder",
  "Coronary Artery Disease",
  "Aortic Valve Stenosis",
  "Status post renal transplantation",
  "Nephrolithiasis",
  "Multiple Endocrine Neoplasia Type 2",
  "Essential Tremor",
  "Peyronie Disease"
 ],
 "age_at_first_encounter": 47
}
```

Primary diagnoses: ["Beta-blocker overdose", "Long term (current) use of immunosuppressants", "Premature atrial contractions", "Aortic valve stenosis", "History of kidney transplantation", "Multiple endocrine neoplasia type 2", "Peyronie disease"]
Comorbidities: ["Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2022-07-11, outpatient, Nephrology

Chief complaint: Routine follow-up 6 months status post renal transplantation

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up 6 months status post renal transplantation

HISTORY OF PRESENT ILLNESS:
He reports feeling well and has been compliant with his medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Major Depressive Disorder
- Generalized Anxiety Disorder
- Coronary Artery Disease
- Aortic Valve Stenosis
- Status post renal transplantation
- Nephrolithiasis
- Multiple Endocrine Neoplasia Type 2
- Essential Tremor
- Peyronie Disease

Medical history is significant only for hypertension, which is well controlled.

PAST SURGICAL HISTORY:
Renal transplantation 6 months ago

MEDICATIONS:
The patient is maintained on sirolimus as part of his immunosuppressive regimen.

Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Metoprolol Succinate 25 mg PO daily
- Sertraline 50 mg PO daily
- Propranolol 20 mg PO BID
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Family history is notable for coronary artery disease in his father.

SOCIAL HISTORY:
He does not smoke, drink alcohol, or use illicit drugs.

REVIEW OF SYSTEMS:
He denies fever, dysuria, hematuria, or graft tenderness. His appetite is good, and weight has been stable.

VITAL SIGNS:
Vital signs are within normal limits

PHYSICAL EXAMINATION:
Physical examination shows a healthy-appearing man with a well-healed abdominal surgical scar

LABORATORY DATA:
Laboratory studies demonstrate normal creatinine and blood counts.

## Encounter 1: source 2020-03-15, practice 2022-08-30, outpatient, Internal Medicine

Chief complaint: Evaluation two weeks after starting new medication for essential tremor with mild fatigue

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation two weeks after starting new medication for essential tremor with mild fatigue

HISTORY OF PRESENT ILLNESS:
He returns two weeks after starting new tremor medication following his January transplant follow-up. He reports mild fatigue during exercise but denies shortness of breath, chest pain, or lightheadedness.

PAST MEDICAL HISTORY:
Active Problem List:
- History of kidney transplantation (diagnosed 2020-01-15)
- Hypertension
- Major Depressive Disorder
- Generalized Anxiety Disorder
- Coronary Artery Disease
- Aortic Valve Stenosis
- Status post renal transplantation
- Nephrolithiasis
- Multiple Endocrine Neoplasia Type 2
- Essential Tremor
- Peyronie Disease

His past medical history is significant for hypertension and anxiety.

PAST SURGICAL HISTORY:
- Renal transplantation (living unrelated donor, 6 months prior)
- Left parathyroidectomy (partial, 2 years prior)

MEDICATIONS:
His medication list has recently updated to include propanolol.

Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Metoprolol Succinate 25 mg PO daily
- Sertraline 50 mg PO daily
- Sirolimus 2 mg PO daily
- Propranolol 20 mg PO BID
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Medullary thyroid carcinoma diagnosed age 42, status post thyroidectomy
- Sister: Primary hyperparathyroidism
- Father: Coronary artery disease with MI at age 55
- Maternal cousin: Pheochromocytoma

SOCIAL HISTORY:
He does not smoke or drink alcohol.

VITAL SIGNS:
On examination, his blood pressure is 126/78 mmHg and heart rate is 56/min.

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a normal S1 and S2 without murmurs, gallops, or rubs. The patient’s extremities are warm, and his capillary refill is slightly delayed. The physician notes that his hands are steady today with no observable tremor.

## Encounter 2: source 2020-07-15, practice 2023-01-02, outpatient, Urology

Chief complaint: Penile pain during erections and new penile curvature for 3 months

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Thompson, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Penile pain during erections and new penile curvature for 3 months

HISTORY OF PRESENT ILLNESS:
The patient is a 47-year-old male status post renal transplantation on chronic immunosuppression who was evaluated in internal medicine four months prior for fatigue following propranolol initiation, and now presents to urology with three months of penile pain during erections and new penile curvature. He denies any history of trauma to the genital area.

PAST MEDICAL HISTORY:
Active Problem List:
- History of kidney transplantation (diagnosed 2020-01-15)
- Long term (current) use of immunosuppressants (diagnosed 2020-03-15)
- Hypertension
- Major Depressive Disorder
- Generalized Anxiety Disorder
- Coronary Artery Disease
- Aortic Valve Stenosis
- Status post renal transplantation
- Nephrolithiasis
- Multiple Endocrine Neoplasia Type 2
- Essential Tremor
- Peyronie Disease

His past medical history is significant for hypertension,.

PAST SURGICAL HISTORY:
- Renal transplantation (living unrelated donor, 6 months prior)
- Left parathyroidectomy (partial, 2 years prior)

MEDICATIONS:
He takes for which he takes lisinopril.

Home Medications:
- Amlodipine 5 mg PO daily
- Metoprolol Succinate 25 mg PO daily
- Sertraline 50 mg PO daily
- Sirolimus 2 mg PO daily
- Propranolol 20 mg PO BID
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Medullary thyroid carcinoma diagnosed age 42, status post thyroidectomy
- Sister: Primary hyperparathyroidism
- Father: Coronary artery disease with MI at age 55
- Maternal cousin: Pheochromocytoma

SOCIAL HISTORY:
Smoking: Former smoker, 1 pack per day for 22 years, quit 8 months ago. Alcohol: Former heavy drinker, 3-4 drinks nightly, quit 8 months ago. Occupation: Financial Analyst.

VITAL SIGNS:
His blood pressure is 135/85 mmHg,

PHYSICAL EXAMINATION:
On physical examination, a palpable, non-tender dorsal penile plaque is noted. There is no urethral discharge or lymphadenopathy.
and the rest of the examination is unremarkable.

LABORATORY DATA:
No abnormalities are found on complete blood count and basic metabolic panel.

## Encounter 3: source 2021-03-15, practice 2023-08-25, outpatient, Endocrinology

Chief complaint: Recurrent epigastric pain, diarrhea, heartburn, and 10-pound weight loss over 6 months

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Aisha Patel, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Recurrent epigastric pain, diarrhea, heartburn, and 10-pound weight loss over 6 months

HISTORY OF PRESENT ILLNESS:
The patient is a 48-year-old male with multiple endocrine neoplasia type 2 who was last seen in Urology eight months ago and presents with a 6-month history of recurrent epigastric pain, diarrhea, and heartburn that is partially relieved by over-the-counter antacids. He reports a 10-pound weight loss during this period despite no changes in diet or exercise.

PAST MEDICAL HISTORY:
Active Problem List:
- History of kidney transplantation (diagnosed 2020-01-15)
- Long term (current) use of immunosuppressants (diagnosed 2020-03-15)
- Peyronie disease (diagnosed 2020-07-15)
- Hypertension
- Major Depressive Disorder
- Generalized Anxiety Disorder
- Coronary Artery Disease
- Aortic Valve Stenosis
- Status post renal transplantation
- Nephrolithiasis
- Multiple Endocrine Neoplasia Type 2
- Essential Tremor

His medical history is significant for nephrolithiasis and a recent diagnosis of hypertension.

PAST SURGICAL HISTORY:
- Renal transplantation (living unrelated donor, 6 months prior)
- Left parathyroidectomy (partial, 2 years prior)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Metoprolol Succinate 25 mg PO daily
- Sertraline 50 mg PO daily
- Sirolimus 2 mg PO daily
- Propranolol 20 mg PO BID
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His mother had surgery for a pituitary adenoma and his sister was recently diagnosed with hyperparathyroidism.

SOCIAL HISTORY:
Smoking: Former smoker, 1 pack per day for 22 years, quit 8 months ago. Alcohol: Former heavy drinker, 3-4 drinks nightly, quit 8 months ago. Occupation: Financial Analyst.

PHYSICAL EXAMINATION:
Physical examination reveals mild epigastric tenderness.

LABORATORY DATA:
Laboratory results are as follows:

- Serum calcium: 11.2 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum gastrin: 1000 pg/mL (normal: <100 pg/mL)
- Serum PTH: 88 pg/mL (normal: 15-65 pg/mL)
- Fasting glucose: 96 mg/dL (normal: 70-99 mg/dL)

OTHER STUDIES:
Upper endoscopy shows multiple peptic ulcers in the duodenum.

## Encounter 4: source 2021-09-15, practice 2024-03-04, outpatient, Cardiology

Chief complaint: Progressive dyspnea on exertion and chest tightness over 6 months

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Robert Kim, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive dyspnea on exertion and chest tightness over 6 months

HISTORY OF PRESENT ILLNESS:
The patient is a 48-year-old male with known aortic valve stenosis, coronary artery disease, and status post renal transplantation who presents to Cardiology following recent Endocrinology evaluation. He reports a decrease in his exercise tolerance, now becoming short of breath after walking two blocks, whereas six months ago he could jog for several miles without issue. He denies any history of rheumatic fever.

PAST MEDICAL HISTORY:
Active Problem List:
- History of kidney transplantation (diagnosed 2020-01-15)
- Long term (current) use of immunosuppressants (diagnosed 2020-03-15)
- Peyronie disease (diagnosed 2020-07-15)
- Multiple endocrine neoplasia type 2 (diagnosed 2021-03-15)
- Hypertension
- Major Depressive Disorder
- Generalized Anxiety Disorder
- Coronary Artery Disease
- Aortic Valve Stenosis
- Status post renal transplantation
- Nephrolithiasis
- Essential Tremor

His past medical history is significant for hypertension, controlled with lisinopril.

PAST SURGICAL HISTORY:
- Renal transplantation (living unrelated donor, 6 months prior)
- Left parathyroidectomy (partial, 2 years prior)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Metoprolol Succinate 25 mg PO daily
- Sertraline 50 mg PO daily
- Sirolimus 2 mg PO daily
- Propranolol 20 mg PO BID
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Medullary thyroid carcinoma diagnosed age 42, status post thyroidectomy
- Sister: Primary hyperparathyroidism
- Father: Coronary artery disease with MI at age 55
- Maternal cousin: Pheochromocytoma

SOCIAL HISTORY:
Smoking: Former smoker, 1 pack per day for 22 years, quit 8 months ago. Alcohol: Former heavy drinker, 3-4 drinks nightly, quit 8 months ago. Occupation: Financial Analyst.

VITAL SIGNS:
The patient's blood pressure is 130/80 mmHg, heart rate is 78 bpm, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
On physical examination, you note a systolic ejection murmur that is loudest at the right second intercostal space and radiates to the carotids.

IMAGING:
An echocardiogram reveals a calcified aortic valve with reduced leaflet mobility and a peak velocity of 4.5 m/s across the aortic valve.

## Encounter 5: source 2021-11-15, practice 2024-04-30, ed, Emergency Medicine

Chief complaint: Found unresponsive at home after recently increasing cardiac medication dose for chest discomfort

ENCOUNTER NOTE
Date: 2021-11-15
Provider: Dr. David Okonkwo, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Found unresponsive at home after recently increasing cardiac medication dose for chest discomfort

HISTORY OF PRESENT ILLNESS:
This 48-year-old male with aortic valve stenosis and coronary artery disease, recently evaluated for chest tightness, presents via emergency services. According to family, he had recently increased the dose of one of his cardiac medications due to ongoing episodes of chest discomfort. In the field, he was noted to be poorly responsive and had minimal improvement after receiving fluids.

PAST MEDICAL HISTORY:
Active Problem List:
- History of kidney transplantation (diagnosed 2020-01-15)
- Long term (current) use of immunosuppressants (diagnosed 2020-03-15)
- Peyronie disease (diagnosed 2020-07-15)
- Multiple endocrine neoplasia type 2 (diagnosed 2021-03-15)
- Aortic valve stenosis (diagnosed 2021-09-15)
- Hypertension
- Major Depressive Disorder
- Generalized Anxiety Disorder
- Coronary Artery Disease
- Status post renal transplantation
- Nephrolithiasis
- Essential Tremor

He has a history of hypertension and coronary artery disease and has been treated with multiple agents over the years.

PAST SURGICAL HISTORY:
- Renal transplantation (living unrelated donor, 6 months prior)
- Left parathyroidectomy (partial, 2 years prior)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Metoprolol Succinate 25 mg PO daily
- Sertraline 50 mg PO daily
- Sirolimus 2 mg PO daily
- Propranolol 20 mg PO BID
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Medullary thyroid carcinoma diagnosed age 42, status post thyroidectomy
- Sister: Primary hyperparathyroidism
- Father: Coronary artery disease with MI at age 55
- Maternal cousin: Pheochromocytoma

SOCIAL HISTORY:
Smoking: Former smoker, 1 pack per day for 22 years, quit 8 months ago. Alcohol: Former heavy drinker, 3-4 drinks nightly, quit 8 months ago. Occupation: Financial Analyst.

VITAL SIGNS:
Temperature 36.5°C, blood pressure 72/48 mmHg, heart rate 38/min, and respiratory rate 10/min

PHYSICAL EXAMINATION:
Cool extremities and delayed capillary refill

OTHER STUDIES:
ECG reveals a slow rhythm with QRS prolongation

PLAN:
Despite multiple doses of atropine and fluid boluses, his perfusion remains poor. He is given a medication that leads to gradual improvement in blood pressure and mental status.

## Encounter 6: source 2022-07-15, practice 2024-12-30, outpatient, Cardiology

Chief complaint: Intermittent palpitations described as sudden fluttering sensations over past 2 months

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Emily Watson, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent palpitations described as sudden fluttering sensations over past 2 months

HISTORY OF PRESENT ILLNESS:
The 49-year-old male with coronary artery disease and prior beta-blocker overdose returns reporting palpitations that have been occurring intermittently for the past two months. He describes the palpitations as sudden fluttering sensations in his chest that last for a few minutes and resolve spontaneously.

PAST MEDICAL HISTORY:
Active Problem List:
- History of kidney transplantation (diagnosed 2020-01-15)
- Long term (current) use of immunosuppressants (diagnosed 2020-03-15)
- Peyronie disease (diagnosed 2020-07-15)
- Multiple endocrine neoplasia type 2 (diagnosed 2021-03-15)
- Aortic valve stenosis (diagnosed 2021-09-15)
- Beta-blocker overdose (diagnosed 2021-11-15)
- Hypertension
- Major Depressive Disorder
- Generalized Anxiety Disorder
- Coronary Artery Disease
- Status post renal transplantation
- Nephrolithiasis
- Essential Tremor

PAST SURGICAL HISTORY:
- Renal transplantation (living unrelated donor, 6 months prior)
- Left parathyroidectomy (partial, 2 years prior)

MEDICATIONS:
He takes hydrochlorothiazide and amlodipine.

Home Medications:
- Lisinopril 20 mg PO daily
- Metoprolol Succinate 25 mg PO daily
- Sertraline 50 mg PO daily
- Sirolimus 2 mg PO daily
- Propranolol 20 mg PO BID
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Medullary thyroid carcinoma diagnosed age 42, status post thyroidectomy
- Sister: Primary hyperparathyroidism
- Father: Coronary artery disease with MI at age 55
- Maternal cousin: Pheochromocytoma

SOCIAL HISTORY:
Daily tobacco use. He also admits to consuming 3-4 alcoholic drinks on most evenings

VITAL SIGNS:
Blood pressure is 138/86 mmHg, heart rate is 78 bpm and irregular, and respiratory rate is 16 breaths per minute

PHYSICAL EXAMINATION:
His cardiovascular examination reveals an irregularly irregular rhythm without murmurs, rubs, or gallops

LABORATORY DATA:
His complete blood count, electrolytes, thyroid function tests, and cardiac enzymes are within normal limits.

Laboratory values:
- Hemoglobin: 14.5 g/dL
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Thyroid-Stimulating Hormone (TSH): 2.1 mIU/L
- Troponin I: <0.01 ng/mL

OTHER STUDIES:
An electrocardiogram (ECG) shows premature atrial contractions

## Encounter 7: source 2023-07-15, practice 2026-01-07, ed, Emergency Medicine

Chief complaint: Acute onset lethargy and confusion over past several hours

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Lisa Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset lethargy and confusion over past several hours

HISTORY OF PRESENT ILLNESS:
The patient returns to the emergency department, brought by his family, following a prior episode of unresponsiveness, and they report that he has been increasingly lethargic and confused over the past few hours.

PAST MEDICAL HISTORY:
Active Problem List:
- History of kidney transplantation (diagnosed 2020-01-15)
- Long term (current) use of immunosuppressants (diagnosed 2020-03-15)
- Peyronie disease (diagnosed 2020-07-15)
- Multiple endocrine neoplasia type 2 (diagnosed 2021-03-15)
- Aortic valve stenosis (diagnosed 2021-09-15)
- Beta-blocker overdose (diagnosed 2021-11-15)
- Premature atrial contractions (diagnosed 2022-07-15)
- Hypertension
- Major Depressive Disorder
- Generalized Anxiety Disorder
- Coronary Artery Disease
- Status post renal transplantation
- Nephrolithiasis
- Essential Tremor

PAST SURGICAL HISTORY:
- Renal transplantation (living unrelated donor, 6 months prior)
- Left parathyroidectomy (partial, 2 years prior)

MEDICATIONS:
He takes metoprolol and sertraline.

Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Metoprolol Succinate 25 mg PO daily
- Sirolimus 2 mg PO daily
- Propranolol 20 mg PO BID
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Medullary thyroid carcinoma diagnosed age 42, status post thyroidectomy
- Sister: Primary hyperparathyroidism
- Father: Coronary artery disease with MI at age 55
- Maternal cousin: Pheochromocytoma

SOCIAL HISTORY:
Smoking: Former smoker, 1 pack per day for 22 years, quit 8 months ago. Alcohol: Former heavy drinker, 3-4 drinks nightly, quit 8 months ago. Occupation: Financial Analyst.

VITAL SIGNS:
Heart rate of 45 bpm, blood pressure is 85/50 mmHg, and respiratory rate is 22 breaths per minute

PHYSICAL EXAMINATION:
On examination, the patient is bradycardic. He is drowsy but arousable. His lungs are clear to auscultation, and his skin is cool and clammy.

LABORATORY DATA:
Glucose: 55 mg/dL (Normal: 70-99 mg/dL)
- Sodium: 140 mEq/L (Normal: 135-145 mEq/L)
- Potassium: 4.2 mEq/L (Normal: 3.5-5.1 mEq/L)
- Creatinine: 1.0 mg/dL (Normal: 0.6-1.2 mg/dL)
- Troponin: <0.01 ng/mL (Normal: <0.04 ng/mL)

OTHER STUDIES:
An ECG shows sinus bradycardia with no other abnormalities.
