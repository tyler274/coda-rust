# synthetic_hospital patient 1997

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 55 at first encounter, sex M, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-12-02 (Fri) | outpatient | Cardiology |
| 1 | 2021-01-15 | 2023-11-27 (Mon) | outpatient | Internal Medicine |
| 2 | 2023-01-15 | 2025-12-11 (Thu) | ed | Emergency Medicine |
| 3 | 2023-07-15 | 2026-06-10 (Wed) | inpatient | Cardiology |
| 4 | 2023-08-15 | 2026-06-30 (Tue) | inpatient | General Surgery |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Accounting manager",
 "alcohol_use": "Occasional social use (1-2 drinks per week)",
 "family_history": [
  "Father: Myocardial infarction at age 62",
  "Mother: Hypertension",
  "Brother: Hyperlipidemia"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg oral daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg oral nightly",
   "name": "Atorvastatin"
  },
  {
   "dose": "12.5 mg oral daily",
   "name": "Hydrochlorothiazide"
  }
 ],
 "surgical_history": [
  "Cardiac catheterization with stent placement"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia",
  "Coronary artery disease"
 ],
 "age_at_first_encounter": 55
}
```

Primary diagnoses: ["Hyperzincemia", "Loss of consciousness (syncope)", "Acute myocardial infarction", "Total parenteral nutrition administration"]
Comorbidities: ["Cardiovascular disease risk factors", "Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2022-12-02, outpatient, Cardiology

Chief complaint: Routine cardiovascular follow-up and inquiry about omega-3 fatty acid supplementation

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine cardiovascular follow-up and inquiry about omega-3 fatty acid supplementation

HISTORY OF PRESENT ILLNESS:
The patient inquires about the benefits of adding omega-3 fatty acids to his regimen for further cardiovascular risk reduction.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- Coronary artery disease

PAST SURGICAL HISTORY:
- Cardiac catheterization with stent placement

MEDICATIONS:
He has been adherent to his medication regimen, which includes lisinopril and atorvastatin.

Home Medications:
- Hydrochlorothiazide 12.5 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is significant for coronary artery disease in his father.

SOCIAL HISTORY:
He does not smoke and drinks alcohol occasionally.

VITAL SIGNS:
On examination, his blood pressure is 132/84 mmHg, heart rate is 78 bpm, and BMI is 29 kg/m^2.

PHYSICAL EXAMINATION:
His cardiovascular and respiratory examinations are unremarkable.

LABORATORY DATA:
Recent laboratory tests show:

- Total cholesterol: 190 mg/dL
- HDL cholesterol: 45 mg/dL
- LDL cholesterol: 100 mg/dL
- Triglycerides: 225 mg/dL
- Fasting glucose: 96 mg/dL

ASSESSMENT AND PLAN:
A recent randomized controlled trial assessing the impact of omega-3 fatty acid supplementation on cardiovascular outcomes reported an adjusted relative risk (RR) of 0.95 with a 95% confidence interval (CI) of 0.85 to 1.05. Based on this information,

## Encounter 1: source 2021-01-15, practice 2023-11-27, outpatient, Internal Medicine

Chief complaint: Routine annual check-up for chronic conditions

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine annual check-up for chronic conditions

HISTORY OF PRESENT ILLNESS:
The patient, a 56-year-old male with hypertension, hyperlipidemia, and coronary artery disease, returns for routine annual follow-up following his prior cardiology evaluation in January 2020. He reports no new symptoms and is adherent to his medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperzincemia (diagnosed 2020-01-15)
- Hypertension
- Hyperlipidemia
- Coronary artery disease

PAST SURGICAL HISTORY:
- Cardiac catheterization with stent placement

MEDICATIONS:
He is currently taking lisinopril and atorvastatin.

Home Medications:
- Hydrochlorothiazide 12.5 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension
- Brother: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Accounting manager.

VITAL SIGNS:
His blood pressure is 132/84 mmHg

PHYSICAL EXAMINATION:
His physical examination is unremarkable

LABORATORY DATA:
His recent laboratory tests show:

- Total cholesterol: 190 mg/dL
- HDL cholesterol: 45 mg/dL
- LDL cholesterol: 100 mg/dL
- Triglycerides: 225 mg/dL

ASSESSMENT AND PLAN:
The patient is interested in the results of his cholesterol management and asks about the reliability of his LDL cholesterol level. You explain that the laboratory test has a 95% confidence interval (CI) of 95-105 mg/dL for LDL cholesterol. He inquires about what would happen to the CI if he wanted to be 99% confident about his LDL level.

## Encounter 2: source 2023-01-15, practice 2025-12-11, ed, Emergency Medicine

Chief complaint: Sudden loss of consciousness while watching television

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden loss of consciousness while watching television

HISTORY OF PRESENT ILLNESS:
This 58-year-old male with hypertension, hyperlipidemia, coronary artery disease, and hyperzincemia previously documented in 2020 and 2021 presents to the emergency department after experiencing a sudden loss of consciousness while watching television. He reports that the episode occurred without any preceding symptoms such as dizziness, palpitations, or chest pain. His wife, who witnessed the event, states that he was unresponsive for about 30 seconds and did not exhibit any convulsive movements.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperzincemia (diagnosed 2020-01-15)
- Hypertension
- Hyperlipidemia
- Coronary artery disease

PAST SURGICAL HISTORY:
- Cardiac catheterization with stent placement

MEDICATIONS:
His medications include lisinopril and atorvastatin.

Home Medications:
- Hydrochlorothiazide 12.5 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension
- Brother: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Accounting manager.

VITAL SIGNS:
On examination, his blood pressure is 135/85 mmHg, heart rate is 72 beats per minute, and he is afebrile.

PHYSICAL EXAMINATION:
Cardiac examination reveals a regular rhythm without murmurs, rubs, or gallops. Neurological examination is unremarkable.

LABORATORY DATA:
His complete blood count, basic metabolic panel, and serum glucose level are within normal limits.

OTHER STUDIES:
An ECG performed in the emergency department shows no acute changes.

## Encounter 3: source 2023-07-15, practice 2026-06-10, inpatient, Cardiology

Chief complaint: Acute myocardial infarction

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Priya Patel, MD
Department: Cardiology
Visit Type: inpatient

CHIEF COMPLAINT:
Acute myocardial infarction

HISTORY OF PRESENT ILLNESS:
This 58-year-old male with hypertension, hyperlipidemia, and coronary artery disease returns following prior evaluations for hyperzincemia and syncope. He reports no complaints and is adherent to his medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperzincemia (diagnosed 2020-01-15)
- Loss of consciousness (syncope) (diagnosed 2023-01-15)
- Hypertension
- Hyperlipidemia
- Coronary artery disease

PAST SURGICAL HISTORY:
- Cardiac catheterization with stent placement

MEDICATIONS:
He is currently taking lisinopril and atorvastatin.

Home Medications:
- Hydrochlorothiazide 12.5 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is significant for coronary artery disease in his father.

SOCIAL HISTORY:
He does not smoke and drinks alcohol socially.

VITAL SIGNS:
His blood pressure is 132/84 mmHg, and his heart rate is 78 bpm.

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

LABORATORY DATA:
Laboratory results are as follows:

- Total cholesterol: 190 mg/dL
- HDL cholesterol: 40 mg/dL
- LDL cholesterol: 120 mg/dL
- Triglycerides: 150 mg/dL
- Fasting glucose: 98 mg/dL

ASSESSMENT AND PLAN:
The patient is interested in participating in a clinical trial comparing the incidence of myocardial infarction (MI) in patients with similar profiles taking different lipid-lowering therapies. Cohort 1 is taking atorvastatin, and cohort 2 is taking a new lipid-lowering agent. After 5 years, the rate of MI in cohort 1 is 2%, and in cohort 2 is 4%. Calculate the relative risk (RR) of MI for patients in cohort 1 compared to cohort 2.

## Encounter 4: source 2023-08-15, practice 2026-06-30, inpatient, General Surgery

Chief complaint: Initiation of total parenteral nutrition

ENCOUNTER NOTE
Date: 2023-08-15
Provider: Dr. David Kim, MD
Department: General Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Initiation of total parenteral nutrition

HISTORY OF PRESENT ILLNESS:
This 58-year-old male with coronary artery disease, hypertension, and hyperlipidemia presents for initiation of total parenteral nutrition approximately one month following his acute myocardial infarction. He reports no complaints. He reports participation in a local health study examining the effects of a new dietary supplement on cardiovascular health. He states that since starting the supplement, his cholesterol levels have improved.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperzincemia (diagnosed 2020-01-15)
- Loss of consciousness (syncope) (diagnosed 2023-01-15)
- Acute myocardial infarction (diagnosed 2023-07-15)
- Hypertension
- Hyperlipidemia
- Coronary artery disease

PAST SURGICAL HISTORY:
- Cardiac catheterization with stent placement

MEDICATIONS:
He is currently taking hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg oral daily
- Atorvastatin 40 mg oral nightly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension
- Brother: Hyperlipidemia

SOCIAL HISTORY:
Smoking
However, upon reviewing the study details, you notice that participants were also advised to follow a strict exercise regimen and a Mediterranean diet.

VITAL SIGNS:
His blood pressure is 138/86 mmHg.

LABORATORY DATA:
His recent blood tests from the study show:

- Total cholesterol: 190 mg/dL
- HDL cholesterol: 40 mg/dL
- LDL cholesterol: 120 mg/dL
- Triglycerides: 150 mg/dL
- Fasting glucose: 96 mg/dL
