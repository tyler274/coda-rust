# synthetic_hospital patient 1996

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 56 at first encounter, sex M, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2019-10-14 (Mon) | outpatient | Internal Medicine |
| 1 | 2021-01-15 | 2020-10-06 (Tue) | outpatient | Internal Medicine |
| 2 | 2023-01-15 | 2022-10-13 (Thu) | outpatient | Neurology |
| 3 | 2025-01-15 | 2024-10-18 (Fri) | follow_up | Cardiology |
| 4 | 2026-01-15 | 2025-10-08 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Financial Analyst",
 "alcohol_use": "Social (2-3 drinks per week)",
 "family_history": [
  "Father: Premature coronary artery disease (MI at age 52)",
  "Mother: Dementia with behavioral changes (onset age 78)",
  "No family history of sudden cardiac death"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "75 mg PO daily",
   "name": "Clopidogrel"
  },
  {
   "dose": "0.4 mg SL PRN chest pain",
   "name": "Nitroglycerin"
  }
 ],
 "surgical_history": [
  "Percutaneous coronary intervention with drug-eluting stent placement (LAD)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia",
  "Coronary Artery Disease",
  "Frontotemporal Dementia"
 ],
 "age_at_first_encounter": 56
}
```

Primary diagnoses: ["Frontotemporal dementia", "Vasospastic angina", "Status post percutaneous coronary intervention", "Hyperlipidemia"]
Comorbidities: ["Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2019-10-14, outpatient, Internal Medicine

Chief complaint: Comes to the clinic for routine management of hyperlipidemia.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Comes to the clinic for routine management of hyperlipidemia.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- Coronary Artery Disease
- Frontotemporal Dementia

He has a history of hypertension and is otherwise healthy.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (LAD)

MEDICATIONS:
Home Medications:
- Atorvastatin 40 mg PO daily
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Aspirin 81 mg PO daily
- Clopidogrel 75 mg PO daily
- Nitroglycerin 0.4 mg SL PRN chest pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Premature coronary artery disease (MI at age 52)
- Mother: Dementia with behavioral changes (onset age 78)
- No family history of sudden cardiac death

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (2-3 drinks per week). Occupation: Financial Analyst.

LABORATORY DATA:
His recent lipid panel shows a total cholesterol of 280 mg/dL and an LDL cholesterol of 190 mg/dL

PLAN:
The physician prescribes cholestyramine to help lower his cholesterol levels. Cholestyramine works through

## Encounter 1: source 2021-01-15, practice 2020-10-06, outpatient, Internal Medicine

Chief complaint: Presents to the clinic for a follow-up visit for hypertension management.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Okafor, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Presents to the clinic for a follow-up visit for hypertension management.

HISTORY OF PRESENT ILLNESS:
The 57-year-old male with hypertension, hyperlipidemia, coronary artery disease, and frontotemporal dementia returns for hypertension management. He has been participating in a randomized controlled trial comparing the efficacy of a new antihypertensive drug to a placebo. His blood pressure has been well-controlled on his current regimen of lisinopril and atorvastatin.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Hypertension
- Coronary Artery Disease
- Frontotemporal Dementia

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (LAD)

MEDICATIONS:
He takes lisinopril and atorvastatin.

Home Medications:
- Amlodipine 5 mg PO daily
- Aspirin 81 mg PO daily
- Clopidogrel 75 mg PO daily
- Nitroglycerin 0.4 mg SL PRN chest pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Premature coronary artery disease (MI at age 52)
- Mother: Dementia with behavioral changes (onset age 78)
- No family history of sudden cardiac death

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (2-3 drinks per week). Occupation: Financial Analyst.

REVIEW OF SYSTEMS:
He denies any chest pain, shortness of breath, or headaches.

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

OTHER STUDIES:
The study results are now available, and the new drug has shown a statistically significant reduction in systolic blood pressure compared to the placebo with a p-value of 0.03. The 95% confidence interval for the mean difference in systolic blood pressure between the new drug and placebo is -14 mmHg to -6 mmHg. Based on this information,

## Encounter 2: source 2023-01-15, practice 2022-10-13, outpatient, Neurology

Chief complaint: Wife reports he has become increasingly irritable and has difficulty managing finances, with new compulsive habits such as collecting and sorting objects.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Maria Rodriguez, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Wife reports he has become increasingly irritable and has difficulty managing finances, with new compulsive habits such as collecting and sorting objects.

HISTORY OF PRESENT ILLNESS:
Patient returns for neurology follow-up with prior hyperlipidemia, hypertension care in 2020, 2021 who reports that over the past year, he has become increasingly irritable and has difficulty managing finances with new compulsive habits such as collecting and sorting objects.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Hypertension
- Coronary Artery Disease
- Frontotemporal Dementia

The patient has a history of hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (LAD)

MEDICATIONS:
Home Medications:
- Atorvastatin 40 mg PO daily
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Aspirin 81 mg PO daily
- Clopidogrel 75 mg PO daily
- Nitroglycerin 0.4 mg SL PRN chest pain

ALLERGIES:
NKDA

FAMILY HISTORY:
His father had a similar condition that began in his late 50s.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (2-3 drinks per week). Occupation: Financial Analyst.

PHYSICAL EXAMINATION:
On examination, the patient is neatly dressed but appears indifferent to the clinical setting. He is oriented to person, place, and time. His speech is fluent, but he seems to lack concern for the seriousness of his condition. Neurological examination reveals no focal deficits.

LABORATORY DATA:
Laboratory tests reveal normal complete blood count, comprehensive metabolic panel, and thyroid function tests.

IMAGING:
MRI of the brain shows atrophy of the frontal and anterior temporal lobes.

OTHER STUDIES:
Mini-Mental State Examination (MMSE) score is 28/30, with deductions for errors in serial 7s and recall of only 2 out of 3 objects at 5 minutes.

## Encounter 3: source 2025-01-15, practice 2024-10-18, follow_up, Cardiology

Chief complaint: Presents for follow-up after percutaneous coronary intervention with drug-eluting stent placement.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. David Kim, MD
Department: Cardiology
Visit Type: follow_up

CHIEF COMPLAINT:
Presents for follow-up after percutaneous coronary intervention with drug-eluting stent placement.

HISTORY OF PRESENT ILLNESS:
The patient, with hypertension and hyperlipidemia managed since 2020 and 2021 and frontotemporal dementia diagnosed in 2023, returns for follow-up after undergoing percutaneous coronary intervention with placement of a drug-eluting stent (DES) in the left anterior descending artery 3 months ago. The procedure was performed after exertional angina and significant luminal narrowing on coronary angiography. He is currently asymptomatic and adherent to dual antiplatelet therapy. The stent was coated with paclitaxel.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Frontotemporal dementia (diagnosed 2023-01-15)
- Hypertension
- Coronary Artery Disease

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (LAD)

MEDICATIONS:
Home Medications:
- Atorvastatin 40 mg PO daily
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Aspirin 81 mg PO daily
- Clopidogrel 75 mg PO daily
- Nitroglycerin 0.4 mg SL PRN chest pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Premature coronary artery disease (MI at age 52)
- Mother: Dementia with behavioral changes (onset age 78)
- No family history of sudden cardiac death

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (2-3 drinks per week). Occupation: Financial Analyst.

ASSESSMENT AND PLAN:
The cardiologist explains that the drug coating helps reduce the risk of in-stent restenosis, which was more common with older bare-metal stents.

## Encounter 4: source 2026-01-15, practice 2025-10-08, ed, Emergency Medicine

Chief complaint: Recurrent episodes of substernal chest discomfort occurring at rest and waking him from sleep.

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. Emily Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Recurrent episodes of substernal chest discomfort occurring at rest and waking him from sleep.

HISTORY OF PRESENT ILLNESS:
The patient is a 62-year-old male with coronary artery disease status post percutaneous coronary intervention in 2025, hypertension, hyperlipidemia, and frontotemporal dementia who presents with recurrent chest discomfort. He describes the pain as tight and pressure-like, occurring primarily at rest and sometimes waking him from sleep. He denies exertional angina and reports that exercise tolerance has not significantly changed.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Frontotemporal dementia (diagnosed 2023-01-15)
- Status post percutaneous coronary intervention (diagnosed 2025-01-15)
- Hypertension
- Coronary Artery Disease

His history is significant for long-standing hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (LAD)

MEDICATIONS:
Medications include a statin and a thiazide diuretic.

Home Medications:
- Atorvastatin 40 mg PO daily
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Aspirin 81 mg PO daily
- Clopidogrel 75 mg PO daily
- Nitroglycerin 0.4 mg SL PRN chest pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Premature coronary artery disease (MI at age 52)
- Mother: Dementia with behavioral changes (onset age 78)
- No family history of sudden cardiac death

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (2-3 drinks per week). Occupation: Financial Analyst.

PHYSICAL EXAMINATION:
Physical examination today is unremarkable

IMAGING:
To further evaluate his symptoms, a coronary angiography is performed with intracoronary acetylcholine infusion.

OTHER STUDIES:
His resting ECG is normal. Stress testing shows no ischemic changes during exercise.
