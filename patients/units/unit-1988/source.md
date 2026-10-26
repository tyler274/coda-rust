# synthetic_hospital patient 1988

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 58 at first encounter, sex M, Non-Hispanic White, Commercial health insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-10-21 (Thu) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2022-01-24 (Mon) | outpatient | Internal Medicine |
| 2 | 2021-07-15 | 2023-04-24 (Mon) | outpatient | Cardiology |
| 3 | 2023-01-15 | 2024-10-25 (Fri) | ed | Emergency Medicine |
| 4 | 2024-01-15 | 2025-10-15 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Commercial health insurance",
 "occupation": "Construction worker",
 "alcohol_use": "Social",
 "family_history": [
  "Father: Myocardial infarction at age 62",
  "Mother: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 1 pack per day for 30 years (30 pack-years); multiple previous unaided quit attempts; currently motivated for pharmacotherapy-assisted cessation",
 "home_medications": [
  {
   "dose": "10 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Percutaneous coronary intervention"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia",
  "Coronary artery disease",
  "Tobacco use disorder"
 ],
 "age_at_first_encounter": 58
}
```

Primary diagnoses: ["Tobacco use disorder", "Acute ST-segment elevation myocardial infarction (STEMI) of inferior wall", "ST-segment elevation myocardial infarction, inferior wall", "Residual myocardial ischemia post-NSTEMI"]
Comorbidities: ["Tobacco use disorder", "Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2021-10-21, outpatient, Internal Medicine

Chief complaint: presents to the clinic for a routine check-up

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the clinic for a routine check-up

HISTORY OF PRESENT ILLNESS:
He has no complaints and is interested in participating in a study that is recruiting individuals at his workplace to evaluate the impact of a new intervention on cardiovascular health. The study will compare the incidence of myocardial infarction over the next 10 years between construction workers who are provided with a smoking cessation program and nicotine replacement therapy versus those who continue their usual habits without intervention. Based on the information provided,

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- Coronary artery disease
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention

MEDICATIONS:
He is adherent to his medications, which include lisinopril and atorvastatin.

Home Medications:
- Aspirin 81 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension

SOCIAL HISTORY:
He is a construction worker and has been smoking a pack of cigarettes daily for the past 30 years.

VITAL SIGNS:
Blood pressure is 132/84 mmHg

PHYSICAL EXAMINATION:
Physical examination is unremarkable

LABORATORY DATA:
Total cholesterol: 190 mg/dL
- HDL cholesterol: 40 mg/dL
- LDL cholesterol: 110 mg/dL
- Triglycerides: 200 mg/dL
- Fasting plasma glucose: 88 mg/dL

## Encounter 1: source 2020-04-15, practice 2022-01-24, outpatient, Internal Medicine

Chief complaint: expressing a desire to quit smoking

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. James Okafor, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Expressing a desire to quit smoking

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up care, presenting with an expressed desire to quit smoking in the context of his cardiovascular history. He has smoked a pack of cigarettes daily for the past 30 years and has attempted to quit multiple times in the past, without success. He reports that his previous attempts were unaided by medications or counseling. He is motivated to quit and is interested in pharmacotherapy to assist with smoking cessation.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- Coronary artery disease
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention

MEDICATIONS:
His current medications include lisinopril and atorvastatin.

Home Medications:
- Aspirin 81 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 30 years (30 pack-years); multiple previous unaided quit attempts; currently motivated for pharmacotherapy-assisted cessation. Alcohol: Social. Occupation: Construction worker.

REVIEW OF SYSTEMS:
He denies any respiratory symptoms, chest pain, or palpitations.

VITAL SIGNS:
His blood pressure is 138/86 mmHg, heart rate is 78 beats per minute, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
His lungs are clear to auscultation, and the cardiovascular examination is unremarkable.

## Encounter 2: source 2021-07-15, practice 2023-04-24, outpatient, Cardiology

Chief complaint: comes to the clinic for routine follow-up

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Maria Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
He comes to the clinic for routine follow-up

HISTORY OF PRESENT ILLNESS:
He returns to the cardiology clinic for routine follow-up with continued management of hypertension, hyperlipidemia, coronary artery disease, and tobacco use disorder, following prior smoking cessation counseling. He was diagnosed 12 years ago and has been on combination antiretroviral therapy (ART) with excellent adherence. He reports no current symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Tobacco use disorder (diagnosed 2020-04-15)
- Hypertension
- Hyperlipidemia
- Coronary artery disease

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Atorvastatin 40 mg daily
- Aspirin 81 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension

SOCIAL HISTORY:
He continues to smoke one pack of cigarettes daily.

VITAL SIGNS:
His BMI is 28 kg/m².

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
His most recent labs show an undetectable viral load and a CD4+ T-cell count of 620/mm³.

OTHER STUDIES:
He is up to date with age-appropriate cancer screenings, and his last colonoscopy was normal.

## Encounter 3: source 2023-01-15, practice 2024-10-25, ed, Emergency Medicine

Chief complaint: severe substernal chest pain

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe substernal chest pain

HISTORY OF PRESENT ILLNESS:
This 61-year-old male with coronary artery disease, prior NSTEMI, hypertension, hyperlipidemia, and tobacco use disorder is brought to the emergency department 45 minutes after developing severe substernal chest pain while climbing stairs. His pain radiates to the left arm and jaw, and he appears diaphoretic.

PAST MEDICAL HISTORY:
Active Problem List:
- Tobacco use disorder (diagnosed 2020-04-15)
- Residual myocardial ischemia post-NSTEMI (diagnosed 2021-07-15)
- Hypertension
- Hyperlipidemia
- Coronary artery disease

He has a history of hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention

MEDICATIONS:
Medications include aspirin, atorvastatin, and lisinopril.

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension

SOCIAL HISTORY:
Smokes one pack of cigarettes daily

VITAL SIGNS:
Blood pressure is 165/92 mm Hg, pulse is 104/min, and respirations are 20/min.

PHYSICAL EXAMINATION:
Cardiac examination reveals normal heart sounds without murmurs or rubs.

OTHER STUDIES:
ECG shows ST-segment elevations in leads II, III, and aVF.

ASSESSMENT AND PLAN:
This drug most likely exerts its pharmacologic effect by binding to an enzyme located in

He is started on oxygen therapy and a medication that rapidly improves his chest discomfort by increasing intracellular concentrations of cyclic guanosine monophosphate (cGMP), resulting in vascular smooth muscle relaxation and systemic vasodilation.

## Encounter 4: source 2024-01-15, practice 2025-10-15, ed, Emergency Medicine

Chief complaint: severe substernal chest pain that began suddenly 45 minutes ago while walking to the store

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe substernal chest pain that began suddenly 45 minutes ago while walking to the store

HISTORY OF PRESENT ILLNESS:
This 62-year-old with CAD returns one year after MI with chest pain he describes as heavy and squeezing, radiating to his left arm, and associated with nausea and shortness of breath.

PAST MEDICAL HISTORY:
Active Problem List:
- Tobacco use disorder (diagnosed 2020-04-15)
- Residual myocardial ischemia post-NSTEMI (diagnosed 2021-07-15)
- Acute ST-segment elevation myocardial infarction (STEMI) of inferior wall (diagnosed 2023-01-15)
- Hypertension
- Hyperlipidemia
- Coronary artery disease

He has a history of hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention

MEDICATIONS:
He has not been taking his prescribed medications regularly.

Home Medications:
- Lisinopril 10 mg daily
- Atorvastatin 40 mg daily
- Aspirin 81 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His father died of a "heart problem" at age 62.

SOCIAL HISTORY:
He has smoked one pack of cigarettes daily for the past 30 years and drinks alcohol socially.

VITAL SIGNS:
On arrival, his blood pressure is 154/92 mm Hg, pulse 102/min, and respiratory rate 22/min.

PHYSICAL EXAMINATION:
He appears anxious and diaphoretic. Physical examination shows cool extremities but no murmurs or gallops.

OTHER STUDIES:
An ECG shows ST-segment elevations in leads II, III, and aVF.

PLAN:
As part of the initial emergency management, he is immediately given a medication by mouth before reperfusion is arranged.
