# synthetic_hospital patient 2067

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 56 at first encounter, sex M, Non-Hispanic White, Commercial insurance through employer.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-05-31 (Tue) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2022-08-26 (Fri) | outpatient | Nephrology |
| 2 | 2020-07-15 | 2022-11-25 (Fri) | outpatient | Pulmonology |
| 3 | 2021-01-15 | 2023-05-22 (Mon) | outpatient | Internal Medicine |
| 4 | 2021-07-15 | 2023-12-04 (Mon) | procedure | Dermatology |
| 5 | 2022-01-15 | 2024-05-31 (Fri) | outpatient | Cardiology |
| 6 | 2022-07-15 | 2024-11-28 (Thu) | ed | Emergency Medicine |
| 7 | 2023-07-15 | 2025-11-25 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial insurance through employer",
 "occupation": "Long-haul truck driver",
 "alcohol_use": "Social, 1-2 drinks per month",
 "family_history": [
  "Coronary artery disease in father (MI at age 62)",
  "Hypertension in mother",
  "End-stage renal disease of unclear etiology in maternal uncle"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 25 pack-years, quit 18 months ago",
 "home_medications": [
  {
   "dose": "5 mg twice daily",
   "name": "Tacrolimus"
  },
  {
   "dose": "5 mg daily",
   "name": "Prednisone"
  },
  {
   "dose": "10 mg daily",
   "name": "Amlodipine"
  },
  {
   "dose": "25 mg daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "40 mg daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "220 mg twice daily as needed for knee pain",
   "name": "Naproxen sodium"
  }
 ],
 "surgical_history": [
  "Renal transplantation (age 52)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia",
  "Chronic kidney disease, status post renal transplantation",
  "Peptic ulcer disease",
  "Chronic obstructive pulmonary disease",
  "Osteoarthritis of the knees"
 ],
 "age_at_first_encounter": 56
}
```

Primary diagnoses: ["Long-distance running", "Secondary hypertension due to renal artery stenosis", "Cutaneous squamous cell carcinoma", "Acute upper gastrointestinal hemorrhage secondary to peptic ulcer disease", "Upper Gastrointestinal Hemorrhage"]
Comorbidities: ["Upper gastrointestinal bleeding (peptic ulcer disease or erosive gastritis)", "Hyperlipidemia", "Long term (current) use of immunosuppressants", "Gradual tobacco reduction strategy (harm reduction)"]

## Encounter 0: source 2020-01-15, practice 2022-05-31, outpatient, Internal Medicine

Chief complaint: Routine check-up and dietary study participation review.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine check-up and dietary study participation review.

HISTORY OF PRESENT ILLNESS:
He reports no current symptoms. The patient mentions that he has been participating in a health study that investigates the relationship between dietary habits and heart disease. He is concerned about the preliminary findings suggesting that individuals who consume a high amount of dietary fiber have a lower incidence of heart disease. He has been eating more oatmeal and vegetables to increase his fiber intake.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- Chronic kidney disease, status post renal transplantation
- Peptic ulcer disease
- Chronic obstructive pulmonary disease
- Osteoarthritis of the knees

PAST SURGICAL HISTORY:
- Renal transplantation (age 52)

MEDICATIONS:
He is adherent to his medication regimen, which includes lisinopril and atorvastatin

Home Medications:
- Tacrolimus 5 mg twice daily
- Prednisone 5 mg daily
- Amlodipine 10 mg daily
- Hydrochlorothiazide 25 mg daily
- Naproxen sodium 220 mg twice daily as needed for knee pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Coronary artery disease in father (MI at age 62)
- Hypertension in mother
- End-stage renal disease of unclear etiology in maternal uncle

SOCIAL HISTORY:
He is a non-smoker and works as a truck driver

VITAL SIGNS:
His blood pressure is 132/84 mmHg

PHYSICAL EXAMINATION:
His physical examination is unremarkable

LABORATORY DATA:
His laboratory results are as follows:

- Total cholesterol: 190 mg/dL
- HDL cholesterol: 40 mg/dL
- LDL cholesterol: 120 mg/dL
- Triglycerides: 150 mg/dL
- Fasting glucose: 96 mg/dL
- HbA1c: 5.6%

ASSESSMENT AND PLAN:
Considering the potential confounding factors in this study

## Encounter 1: source 2020-04-15, practice 2022-08-26, outpatient, Nephrology

Chief complaint: Persistent headaches and episodes of dizziness despite antihypertensive therapy.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent headaches and episodes of dizziness despite antihypertensive therapy.

HISTORY OF PRESENT ILLNESS:
The patient with chronic kidney disease status post renal transplantation returns following Internal Medicine visit three months prior. He reports persistent headaches and episodes of dizziness for the past 6 months. Despite hydrochlorothiazide and amlodipine, blood pressure remains elevated at 162/98 mmHg. He reports compliance with medications and dietary sodium restriction.

PAST MEDICAL HISTORY:
Active Problem List:
- Long-distance running (diagnosed 2020-01-15)
- Hypertension
- Hyperlipidemia
- Chronic kidney disease, status post renal transplantation
- Peptic ulcer disease
- Chronic obstructive pulmonary disease
- Osteoarthritis of the knees

PAST SURGICAL HISTORY:
- Renal transplantation (age 52)

MEDICATIONS:
He takes hydrochlorothiazide and amlodipine.

Home Medications:
- Tacrolimus 5 mg twice daily
- Prednisone 5 mg daily
- Atorvastatin 40 mg daily
- Naproxen sodium 220 mg twice daily as needed for knee pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Coronary artery disease in father (MI at age 62)
- Hypertension in mother
- End-stage renal disease of unclear etiology in maternal uncle

SOCIAL HISTORY:
Smoking

VITAL SIGNS:
Blood pressure remains elevated at 162/98 mmHg

PHYSICAL EXAMINATION:
On physical examination, you note a systolic-diastolic bruit in the epigastric region. His pulses are symmetric, and there is no evidence of heart failure.

LABORATORY DATA:
Normal renal function

## Encounter 2: source 2020-07-15, practice 2022-11-25, outpatient, Pulmonology

Chief complaint: New cough and a 5-pound weight loss over the past month.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Aisha Patel, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
New cough and a 5-pound weight loss over the past month.

HISTORY OF PRESENT ILLNESS:
The patient with CKD status post renal transplant, last seen in Nephrology 3 months ago, presents to the clinic with a history of smoking and chronic obstructive pulmonary disease (COPD) with a new cough and 5-pound weight loss over the past month.

PAST MEDICAL HISTORY:
Active Problem List:
- Long-distance running (diagnosed 2020-01-15)
- Secondary hypertension due to renal artery stenosis (diagnosed 2020-04-15)
- Hypertension
- Hyperlipidemia
- Chronic kidney disease, status post renal transplantation
- Peptic ulcer disease
- Chronic obstructive pulmonary disease
- Osteoarthritis of the knees

PAST SURGICAL HISTORY:
- Renal transplantation (age 52)

MEDICATIONS:
Home Medications:
- Tacrolimus 5 mg twice daily
- Prednisone 5 mg daily
- Amlodipine 10 mg daily
- Hydrochlorothiazide 25 mg daily
- Atorvastatin 40 mg daily
- Naproxen sodium 220 mg twice daily as needed for knee pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Coronary artery disease in father (MI at age 62)
- Hypertension in mother
- End-stage renal disease of unclear etiology in maternal uncle

SOCIAL HISTORY:
Smoking: Former smoker, 25 pack-years, quit 18 months ago. Alcohol: Social, 1-2 drinks per month. Occupation: Long-haul truck driver.

REVIEW OF SYSTEMS:
He denies fever, night sweats, or hemoptysis.

VITAL SIGNS:
His vital signs are within normal limits.

PHYSICAL EXAMINATION:
Physical examination reveals decreased breath sounds in the right upper lobe.

IMAGING:
A chest X-ray shows a 2 cm mass in the right upper lobe. A computed tomography (CT) scan of the chest confirms the presence of the mass with no lymphadenopathy.

PATHOLOGY:
The pathology report indicates a non-small cell lung carcinoma (NSCLC).

OTHER STUDIES:
The patient undergoes a CT-guided biopsy of the lung mass.

ASSESSMENT AND PLAN:
The oncologist discusses potential biomarkers that could predict the response to targeted therapy. One such biomarker has recently been evaluated, and the receiver operating characteristic (ROC) curve for the biomarker test has shifted upward and to the right for the given cutoff point.

## Encounter 3: source 2021-01-15, practice 2023-05-22, outpatient, Internal Medicine

Chief complaint: Routine monitoring visit for Phase II clinical trial participation.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James O'Brien, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine monitoring visit for Phase II clinical trial participation.

PAST MEDICAL HISTORY:
Active Problem List:
- Long-distance running (diagnosed 2020-01-15)
- Secondary hypertension due to renal artery stenosis (diagnosed 2020-04-15)
- Hypertension
- Hyperlipidemia
- Chronic kidney disease, status post renal transplantation
- Peptic ulcer disease
- Chronic obstructive pulmonary disease
- Osteoarthritis of the knees

PAST SURGICAL HISTORY:
- Renal transplantation (age 52)

MEDICATIONS:
Home Medications:
- Tacrolimus 5 mg twice daily
- Prednisone 5 mg daily
- Amlodipine 10 mg daily
- Hydrochlorothiazide 25 mg daily
- Atorvastatin 40 mg daily
- Naproxen sodium 220 mg twice daily as needed for knee pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Coronary artery disease in father (MI at age 62)
- Hypertension in mother
- End-stage renal disease of unclear etiology in maternal uncle

SOCIAL HISTORY:
Smoking: Former smoker, 25 pack-years, quit 18 months ago. Alcohol: Social, 1-2 drinks per month. Occupation: Long-haul truck driver.

OTHER STUDIES:
Enrolled in a clinical trial for a novel cholesterol-lowering agent, Drug X. The trial consists of 300 patients with elevated LDL cholesterol who are randomized to receive either Drug X or placebo. Over a 6-month period, researchers collect data on LDL reduction and monitor for adverse events. The primary outcome is a statistically significant reduction in LDL cholesterol compared to placebo. No long-term cardiovascular outcomes are measured, and the sample size is insufficient to detect rare adverse effects. The investigators report a significant mean LDL reduction in the Drug X group compared to placebo, with a p-value of 0.01. The study is not blinded, but all participants are actual patients with the condition of interest.

## Encounter 4: source 2021-07-15, practice 2023-12-04, procedure, Dermatology

Chief complaint: Non-healing ulcer on the right forearm gradually increasing in size over 3 months.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Fatima Al-Rashid, MD
Department: Dermatology
Visit Type: procedure

CHIEF COMPLAINT:
Non-healing ulcer on the right forearm gradually increasing in size over 3 months.

HISTORY OF PRESENT ILLNESS:
This 57-year-old male with a history of renal transplantation on chronic immunosuppression, last evaluated in January 2021 for clinical trial monitoring, presents to the Dermatology clinic with a non-healing ulcer on his right forearm that has been gradually increasing in size over the past 3 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Long-distance running (diagnosed 2020-01-15)
- Secondary hypertension due to renal artery stenosis (diagnosed 2020-04-15)
- Hypertension
- Hyperlipidemia
- Chronic kidney disease, status post renal transplantation
- Peptic ulcer disease
- Chronic obstructive pulmonary disease
- Osteoarthritis of the knees

He has a history of a renal transplant 5 years ago.

PAST SURGICAL HISTORY:
- Renal transplantation (age 52)

MEDICATIONS:
He has been on chronic immunosuppressive therapy with tacrolimus and prednisone.

Home Medications:
- Amlodipine 10 mg daily
- Hydrochlorothiazide 25 mg daily
- Atorvastatin 40 mg daily
- Naproxen sodium 220 mg twice daily as needed for knee pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Coronary artery disease in father (MI at age 62)
- Hypertension in mother
- End-stage renal disease of unclear etiology in maternal uncle

SOCIAL HISTORY:
Smoking: Former smoker, 25 pack-years, quit 18 months ago. Alcohol: Social, 1-2 drinks per month. Occupation: Long-haul truck driver.

REVIEW OF SYSTEMS:
He denies any fever, weight loss, or night sweats.

PHYSICAL EXAMINATION:
On examination, the ulcer is 2 cm in diameter, with raised, everted edges and a central keratinous core. There is no lymphadenopathy. The rest of the skin examination is unremarkable.

LABORATORY DATA:
His recent laboratory tests are as follows:

- Complete Blood Count (CBC):
- Hemoglobin: 13.2 g/dL
- White Blood Cell Count (WBC): 7,500 cells/mm^3
- Platelets: 250,000 cells/mm^3
- Comprehensive Metabolic Panel (CMP):
- Creatinine: 1.4 mg/dL
- Blood Urea Nitrogen (BUN): 18 mg/dL
- Alanine Aminotransferase (ALT): 25 U/L
- Aspartate Aminotransferase (AST): 30 U/L

IMAGING:
Chest X-ray: No evidence of metastatic disease

## Encounter 5: source 2022-01-15, practice 2024-05-31, outpatient, Cardiology

Chief complaint: Enrollment evaluation for multicenter Phase III randomized controlled trial of new lipid-lowering agent.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Robert Kim, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Enrollment evaluation for multicenter Phase III randomized controlled trial of new lipid-lowering agent.

PAST MEDICAL HISTORY:
Active Problem List:
- Long-distance running (diagnosed 2020-01-15)
- Secondary hypertension due to renal artery stenosis (diagnosed 2020-04-15)
- Cutaneous squamous cell carcinoma (diagnosed 2021-07-15)
- Hypertension
- Hyperlipidemia
- Chronic kidney disease, status post renal transplantation
- Peptic ulcer disease
- Chronic obstructive pulmonary disease
- Osteoarthritis of the knees

PAST SURGICAL HISTORY:
- Renal transplantation (age 52)

MEDICATIONS:
Home Medications:
- Tacrolimus 5 mg twice daily
- Prednisone 5 mg daily
- Amlodipine 10 mg daily
- Hydrochlorothiazide 25 mg daily
- Atorvastatin 40 mg daily
- Naproxen sodium 220 mg twice daily as needed for knee pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Coronary artery disease in father (MI at age 62)
- Hypertension in mother
- End-stage renal disease of unclear etiology in maternal uncle

SOCIAL HISTORY:
Smoking: Former smoker, 25 pack-years, quit 18 months ago. Alcohol: Social, 1-2 drinks per month. Occupation: Long-haul truck driver.

OTHER STUDIES:
The study enrolls 6,000 patients who are randomized to receive either Drug X or a statin and are followed for five years. The primary endpoint is the occurrence of major adverse cardiovascular events (MACE), including myocardial infarction, stroke, or cardiovascular death. The trial is double-blinded and powered to detect a 20% reduction in MACE with Drug X relative to the statin group. After data analysis, the investigators report that the relative risk reduction for MACE with Drug X is statistically significant compared to the statin, with a p-value < 0.01. The study protocol was submitted to the FDA as part of a new drug application.

## Encounter 6: source 2022-07-15, practice 2024-11-28, ed, Emergency Medicine

Chief complaint: Hematemesis and melena for the past 2 hours with sharp epigastric pain.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Emily Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Hematemesis and melena for the past 2 hours with sharp epigastric pain.

HISTORY OF PRESENT ILLNESS:
This 58-year-old male with peptic ulcer disease and chronic kidney disease status post renal transplantation, recently evaluated in Cardiology, presents to the emergency department with a 2-hour history of hematemesis and melena. He reports a sharp, gnawing epigastric pain that has worsened over the past week. He admits to taking over-the-counter NSAIDs frequently for chronic knee pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Long-distance running (diagnosed 2020-01-15)
- Secondary hypertension due to renal artery stenosis (diagnosed 2020-04-15)
- Cutaneous squamous cell carcinoma (diagnosed 2021-07-15)
- Hypertension
- Hyperlipidemia
- Chronic kidney disease, status post renal transplantation
- Peptic ulcer disease
- Chronic obstructive pulmonary disease
- Osteoarthritis of the knees

PAST SURGICAL HISTORY:
- Renal transplantation (age 52)

MEDICATIONS:
He takes over-the-counter NSAIDs frequently.

Home Medications:
- Tacrolimus 5 mg twice daily
- Prednisone 5 mg daily
- Amlodipine 10 mg daily
- Hydrochlorothiazide 25 mg daily
- Atorvastatin 40 mg daily
- Naproxen sodium 220 mg twice daily as needed for knee pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Coronary artery disease in father (MI at age 62)
- Hypertension in mother
- End-stage renal disease of unclear etiology in maternal uncle

SOCIAL HISTORY:
Smoking: Former smoker, 25 pack-years, quit 18 months ago. Alcohol: Social, 1-2 drinks per month. Occupation: Long-haul truck driver.

VITAL SIGNS:
Blood pressure is 90/60 mmHg, heart rate is 120 beats per minute

PHYSICAL EXAMINATION:
Appears pale and diaphoretic. His abdomen is soft with mild epigastric tenderness. No masses or hepatosplenomegaly are noted.

## Encounter 7: source 2023-07-15, practice 2025-11-25, ed, Emergency Medicine

Chief complaint: Recurrent hematemesis and melena over 6 hours with burning epigastric pain.

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. David Smith, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Recurrent hematemesis and melena over 6 hours with burning epigastric pain.

HISTORY OF PRESENT ILLNESS:
The patient, with a history of peptic ulcer disease and prior upper gastrointestinal hemorrhage, presents to the emergency department with a 6-hour history of hematemesis and melena. He reports a burning epigastric pain that has worsened over the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Long-distance running (diagnosed 2020-01-15)
- Secondary hypertension due to renal artery stenosis (diagnosed 2020-04-15)
- Cutaneous squamous cell carcinoma (diagnosed 2021-07-15)
- Acute upper gastrointestinal hemorrhage secondary to peptic ulcer disease (diagnosed 2022-07-15)
- Hypertension
- Hyperlipidemia
- Chronic kidney disease, status post renal transplantation
- Peptic ulcer disease
- Chronic obstructive pulmonary disease
- Osteoarthritis of the knees

PAST SURGICAL HISTORY:
- Renal transplantation (age 52)

MEDICATIONS:
He has been taking over-the-counter naproxen for chronic knee pain.

Home Medications:
- Tacrolimus 5 mg twice daily
- Prednisone 5 mg daily
- Amlodipine 10 mg daily
- Hydrochlorothiazide 25 mg daily
- Atorvastatin 40 mg daily
- Naproxen sodium 220 mg twice daily as needed for knee pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Coronary artery disease in father (MI at age 62)
- Hypertension in mother
- End-stage renal disease of unclear etiology in maternal uncle

SOCIAL HISTORY:
Smoking: Former smoker, 25 pack-years, quit 18 months ago. Alcohol: Social, 1-2 drinks per month. Occupation: Long-haul truck driver.

VITAL SIGNS:
Blood pressure is 95/60 mmHg, heart rate is 110 beats per minute

PHYSICAL EXAMINATION:
On examination, he appears pale and diaphoretic. His abdominal examination reveals mild epigastric tenderness without rebound or guarding. Rectal examination confirms melena.

LABORATORY DATA:
Hemoglobin: 9.2 g/dL (normal: 13.8-17.2 g/dL)
- Hematocrit: 27.8% (normal: 40.7-50.3%)
- Blood urea nitrogen (BUN): 36 mg/dL (normal: 7-20 mg/dL)
- Creatinine: 0.9 mg/dL (normal: 0.6-1.3 mg/dL)
- BUN/Creatinine ratio: 40 (normal: 10-20)
