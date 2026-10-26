# synthetic_hospital patient 2723

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 56 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-02-15 (Wed) | outpatient | Internal Medicine |
| 1 | 2020-03-15 | 2023-04-11 (Tue) | outpatient | Internal Medicine |
| 2 | 2020-09-15 | 2023-10-11 (Wed) | outpatient | Nephrology |
| 3 | 2021-03-15 | 2024-04-12 (Fri) | outpatient | Cardiology |
| 4 | 2021-09-15 | 2024-10-18 (Fri) | outpatient | Internal Medicine |
| 5 | 2022-03-15 | 2025-04-24 (Thu) | ed | Emergency Medicine |
| 6 | 2022-09-15 | 2025-10-10 (Fri) | inpatient | Internal Medicine |
| 7 | 2023-05-15 | 2026-06-12 (Fri) | icu | Critical Care |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "ACE inhibitors (angioedema/cough)",
  "Shellfish"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Office administrator",
 "alcohol_use": "Occasional social use (1-2 drinks monthly)",
 "family_history": [
  "Father: Hypertension, Type 2 diabetes mellitus",
  "Mother: Hyperlipidemia, Stroke at age 68",
  "Maternal aunt: Bipolar disorder",
  "Brother: Premature coronary artery disease (MI at age 52)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "300 mg PO BID",
   "name": "Lithium carbonate"
  },
  {
   "dose": "100 mg PO daily",
   "name": "Sertraline"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Hyperlipidemia",
  "Bipolar I Disorder",
  "Major Depressive Disorder"
 ],
 "age_at_first_encounter": 56
}
```

Primary diagnoses: ["Renal artery stenosis", "Lisinopril adverse effect", "Lithium toxicity", "Severe lithium toxicity", "Secondary hypertension due to renal artery stenosis"]
Comorbidities: ["Hyperlipidemia", "Essential hypertension", "Bipolar Disorder"]

## Encounter 0: source 2020-01-15, practice 2023-02-15, outpatient, Internal Medicine

Chief complaint: Interested in enrolling in a clinical research study for cardiovascular risk reduction.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Interested in enrolling in a clinical research study for cardiovascular risk reduction.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Bipolar I Disorder
- Major Depressive Disorder

MEDICATIONS:
Home Medications:
- Lithium carbonate 300 mg PO BID
- Sertraline 100 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
ACE inhibitors (angioedema/cough), Shellfish

FAMILY HISTORY:
- Father: Hypertension, Type 2 diabetes mellitus
- Mother: Hyperlipidemia, Stroke at age 68
- Maternal aunt: Bipolar disorder
- Brother: Premature coronary artery disease (MI at age 52)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks monthly). Occupation: Office administrator.

OTHER STUDIES:
She is enrolled in a randomized controlled trial comparing a new lipid-lowering agent to standard statin therapy. The primary outcome is a reduction in LDL cholesterol by at least 30 mg/dL after 12 weeks. The investigators expect a moderate effect size and set the significance level (alpha) at 0.05. Due to constraints in recruitment, only 60 patients are enrolled in each arm of the study. At the conclusion, the mean LDL reduction in the experimental group is 32 mg/dL, and in the control group, it is 27 mg/dL, with a calculated p-value of 0.08. The investigators note that, given the sample size and expected effect, the study had a calculated power of 65%.

## Encounter 1: source 2020-03-15, practice 2023-04-11, outpatient, Internal Medicine

Chief complaint: Persistent dry cough for the past 2 months after starting new blood pressure medication.

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. James Wilson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent dry cough for the past 2 months after starting new blood pressure medication.

HISTORY OF PRESENT ILLNESS:
The patient with hypertension and diabetes returns with persistent dry cough that is non-productive and not associated with dyspnea, wheezing, or chest pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Bipolar I Disorder
- Major Depressive Disorder

She has a history of hypertension.

MEDICATIONS:
She was started on a new medication during her last visit 3 months ago. Her other medications include atorvastatin and a daily aspirin.

Home Medications:
- Lithium carbonate 300 mg PO BID
- Sertraline 100 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
No history of allergies

FAMILY HISTORY:
- Father: Hypertension, Type 2 diabetes mellitus
- Mother: Hyperlipidemia, Stroke at age 68
- Maternal aunt: Bipolar disorder
- Brother: Premature coronary artery disease (MI at age 52)

SOCIAL HISTORY:
She has never smoked

VITAL SIGNS:
Blood pressure is 132/84 mmHg, heart rate is 78 beats per minute, and respiratory rate is 16 breaths per minute

PHYSICAL EXAMINATION:
Lung auscultation reveals no wheezes, rales, or rhonchi. The rest of the examination is unremarkable.

LABORATORY DATA:
Complete blood count: within normal limits
Serum electrolytes: within normal limits
Serum creatinine: 1.0 mg/dL

IMAGING:
Chest X-ray: no acute cardiopulmonary process

## Encounter 2: source 2020-09-15, practice 2023-10-11, outpatient, Nephrology

Chief complaint: Six-month history of difficult-to-control hypertension despite adherence to triple therapy.

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Priya Patel, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of difficult-to-control hypertension despite adherence to triple therapy.

HISTORY OF PRESENT ILLNESS:
The patient with hypertension and mood disorders, following lisinopril cessation due to cough, presents with 6-month history of difficult-to-control hypertension. Despite adherence to metoprolol, amlodipine, and hydrochlorothiazide, blood pressure remains elevated at 162/98 mmHg.

PAST MEDICAL HISTORY:
Active Problem List:
- Lisinopril adverse effect (diagnosed 2020-03-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Bipolar I Disorder
- Major Depressive Disorder

MEDICATIONS:
She takes metoprolol, amlodipine, and hydrochlorothiazide.

Home Medications:
- Lithium carbonate 300 mg PO BID
- Sertraline 100 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
ACE inhibitors (angioedema/cough), Shellfish

FAMILY HISTORY:
- Father: Hypertension, Type 2 diabetes mellitus
- Mother: Hyperlipidemia, Stroke at age 68
- Maternal aunt: Bipolar disorder
- Brother: Premature coronary artery disease (MI at age 52)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks monthly). Occupation: Office administrator.

REVIEW OF SYSTEMS:
She reports no symptoms of chest pain, palpitations, or headaches.

VITAL SIGNS:
162/98 mmHg

PHYSICAL EXAMINATION:
Prominent bruit lateral to the umbilicus on the right side

LABORATORY DATA:
- Serum creatinine: 1.2 mg/dL (elevated)
- Blood urea nitrogen (BUN): 22 mg/dL
- Serum potassium: 3.4 mEq/L (low)
- Plasma renin activity: High on the right, low on the left
- Serum aldosterone: Elevated

IMAGING:
An abdominal ultrasound with Doppler shows right kidney size reduction compared to the left.

## Encounter 3: source 2021-03-15, practice 2024-04-12, outpatient, Cardiology

Chief complaint: Enrolling in randomized controlled trial comparing new lipid-lowering agent with standard statin therapy.

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Michael Chang, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Enrolling in randomized controlled trial comparing new lipid-lowering agent with standard statin therapy.

PAST MEDICAL HISTORY:
Active Problem List:
- Lisinopril adverse effect (diagnosed 2020-03-15)
- Renal artery stenosis (diagnosed 2020-09-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Bipolar I Disorder
- Major Depressive Disorder

MEDICATIONS:
Home Medications:
- Lithium carbonate 300 mg PO BID
- Sertraline 100 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
ACE inhibitors (angioedema/cough), Shellfish

FAMILY HISTORY:
- Father: Hypertension, Type 2 diabetes mellitus
- Mother: Hyperlipidemia, Stroke at age 68
- Maternal aunt: Bipolar disorder
- Brother: Premature coronary artery disease (MI at age 52)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks monthly). Occupation: Office administrator.

OTHER STUDIES:
The investigators wish to detect a true difference in mean LDL reduction of at least 18 mg/dL, with a standard deviation of 20 mg/dL in both groups. They set the significance level (α) at 0.05 and calculate that a sample size of 50 per group provides a study power of 70%. After analyzing the results, the p-value is found to be 0.11, and the null hypothesis is not rejected. The researchers are concerned about the possibility of a type II (β) error.

## Encounter 4: source 2021-09-15, practice 2024-10-18, outpatient, Internal Medicine

Chief complaint: Routine research follow-up visit for clinical trial participation and study methodology review.

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Emily Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine research follow-up visit for clinical trial participation and study methodology review.

PAST MEDICAL HISTORY:
Active Problem List:
- Lisinopril adverse effect (diagnosed 2020-03-15)
- Renal artery stenosis (diagnosed 2020-09-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Bipolar I Disorder
- Major Depressive Disorder

MEDICATIONS:
Home Medications:
- Lithium carbonate 300 mg PO BID
- Sertraline 100 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
ACE inhibitors (angioedema/cough), Shellfish

FAMILY HISTORY:
- Father: Hypertension, Type 2 diabetes mellitus
- Mother: Hyperlipidemia, Stroke at age 68
- Maternal aunt: Bipolar disorder
- Brother: Premature coronary artery disease (MI at age 52)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks monthly). Occupation: Office administrator.

OTHER STUDIES:
Participates in a randomized controlled trial assessing the effect of a new antihypertensive drug versus placebo. The investigators predefine the significance level (alpha) at 0.01 instead of the more common 0.05 due to concerns about multiple comparisons and the risk of type I errors. The trial enrolls 150 patients in each arm and observes a mean systolic blood pressure reduction of 7 mmHg (p = 0.03) in the treatment group compared to placebo.

## Encounter 5: source 2022-03-15, practice 2025-04-24, ed, Emergency Medicine

Chief complaint: Two-day history of nausea, vomiting, diarrhea, and hand tremors with confusion and difficulty walking.

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-day history of nausea, vomiting, diarrhea, and hand tremors with confusion and difficulty walking.

HISTORY OF PRESENT ILLNESS:
This 58-year-old female with bipolar I disorder, hypertension, and renal artery stenosis presents to the emergency department with a 2-day history of nausea, vomiting, diarrhea, and hand tremors. She reports that she has been feeling increasingly confused and has had difficulty walking due to a sense of imbalance.

PAST MEDICAL HISTORY:
Active Problem List:
- Lisinopril adverse effect (diagnosed 2020-03-15)
- Renal artery stenosis (diagnosed 2020-09-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Bipolar I Disorder
- Major Depressive Disorder

Her medical history is significant for hypertension.

MEDICATIONS:
She takes stabilized on lithium. she recently started taking a new medication, but she cannot recall its name.

Home Medications:
- Lithium carbonate 300 mg PO BID
- Sertraline 100 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
ACE inhibitors (angioedema/cough), Shellfish

FAMILY HISTORY:
- Father: Hypertension, Type 2 diabetes mellitus
- Mother: Hyperlipidemia, Stroke at age 68
- Maternal aunt: Bipolar disorder
- Brother: Premature coronary artery disease (MI at age 52)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks monthly). Occupation: Office administrator.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
Physical examination reveals mild ataxia and a fine tremor in both hands.

LABORATORY DATA:
Laboratory values are as follows:

- Serum lithium: 2.5 mEq/L (therapeutic range 0.6-1.2 mEq/L)
- Serum creatinine: 1.1 mg/dL
- Serum sodium: 140 mEq/L
- Serum potassium: 4.2 mEq/L
- Serum calcium: 9.6 mg/dL

## Encounter 6: source 2022-09-15, practice 2025-10-10, inpatient, Internal Medicine

Chief complaint: Recurrent episodes of acute shortness of breath at night relieved by sitting upright, following recent hospitalization for flash pulmonary edema.

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Recurrent episodes of acute shortness of breath at night relieved by sitting upright, following recent hospitalization for flash pulmonary edema.

HISTORY OF PRESENT ILLNESS:
The patient with a history of renal artery stenosis returns with recurrent episodes of acute shortness of breath at night that awakens her from sleep. She reports that these episodes are relieved by sitting upright. She also mentions a recent episode of flash pulmonary edema that required hospitalization.

PAST MEDICAL HISTORY:
Active Problem List:
- Lisinopril adverse effect (diagnosed 2020-03-15)
- Renal artery stenosis (diagnosed 2020-09-15)
- Lithium toxicity (diagnosed 2022-03-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Bipolar I Disorder
- Major Depressive Disorder

MEDICATIONS:
She takes hydrochlorothiazide and amlodipine.

Home Medications:
- Lithium carbonate 300 mg PO BID
- Sertraline 100 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
ACE inhibitors (angioedema/cough), Shellfish

FAMILY HISTORY:
- Father: Hypertension, Type 2 diabetes mellitus
- Mother: Hyperlipidemia, Stroke at age 68
- Maternal aunt: Bipolar disorder
- Brother: Premature coronary artery disease (MI at age 52)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks monthly). Occupation: Office administrator.

VITAL SIGNS:
Blood pressure is 162/98 mmHg

PHYSICAL EXAMINATION:
A bruit is audible over the right upper quadrant of the abdomen

LABORATORY DATA:
Serum creatinine: 1.5 mg/dL (elevated)
- Blood urea nitrogen (BUN): 28 mg/dL (elevated)
- Serum potassium: 4.2 mEq/L (normal)
- Urinalysis: No proteinuria or hematuria

IMAGING:
A Doppler ultrasound of the renal arteries shows a significant stenosis of the right renal artery.

## Encounter 7: source 2023-05-15, practice 2026-06-12, icu, Critical Care

Chief complaint: Severe worsening nausea, vomiting, diarrhea, hand tremors, confusion, and unsteady gait progressing over 2 days.

ENCOUNTER NOTE
Date: 2023-05-15
Provider: Dr. Lisa Thompson, MD
Department: Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Severe worsening nausea, vomiting, diarrhea, hand tremors, confusion, and unsteady gait progressing over 2 days.

HISTORY OF PRESENT ILLNESS:
The patient is a 59-year-old female with Bipolar I Disorder on chronic lithium therapy and a history of prior lithium toxicity in March 2022 who presents with recurrence of similar symptoms. She reports a 2-day history of nausea, vomiting, diarrhea, and hand tremors. She reports feeling "confused and unsteady" when walking. She denies any recent illness, travel, or changes in her diet.

PAST MEDICAL HISTORY:
Active Problem List:
- Lisinopril adverse effect (diagnosed 2020-03-15)
- Renal artery stenosis (diagnosed 2020-09-15)
- Lithium toxicity (diagnosed 2022-03-15)
- Secondary hypertension due to renal artery stenosis (diagnosed 2022-09-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Bipolar I Disorder
- Major Depressive Disorder

MEDICATIONS:
She takes lithium, hydrochlorothiazide for hypertension, and sertraline for depression.

Home Medications:
- Lithium carbonate 300 mg PO BID
- Metoprolol succinate 50 mg PO daily
- Amlodipine 10 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
ACE inhibitors (angioedema/cough), Shellfish

FAMILY HISTORY:
- Father: Hypertension, Type 2 diabetes mellitus
- Mother: Hyperlipidemia, Stroke at age 68
- Maternal aunt: Bipolar disorder
- Brother: Premature coronary artery disease (MI at age 52)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks monthly). Occupation: Office administrator.

VITAL SIGNS:
Blood pressure 130/85 mmHg, heart rate 92 beats/min, respiratory rate 18 breaths/min, and temperature 37.1°C (98.8°F)

PHYSICAL EXAMINATION:
On physical examination, she is alert but appears distressed. Neurological examination reveals mild ataxia and a fine tremor in both hands.

LABORATORY DATA:
Serum sodium: 140 mEq/L
- Serum potassium: 4.2 mEq/L
- Serum creatinine: 1.1 mg/dL
- Serum lithium: 2.5 mEq/L (therapeutic range: 0.6-1.2 mEq/L)
- Serum TSH: 2.5 µIU/mL
