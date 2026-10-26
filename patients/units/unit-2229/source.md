# synthetic_hospital patient 2229

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 62 at first encounter, sex M, Non-Hispanic White, Private commercial insurance (Medicare eligible at 65).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-05-16 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2022-08-24 (Wed) | outpatient | Internal Medicine |
| 2 | 2020-09-15 | 2023-01-23 (Mon) | outpatient | Podiatry |
| 3 | 2021-03-15 | 2023-07-17 (Mon) | procedure | Orthopedic Surgery |
| 4 | 2021-09-15 | 2024-01-19 (Fri) | ed | Emergency Medicine |
| 5 | 2022-05-15 | 2024-09-20 (Fri) | ed | Emergency Medicine |
| 6 | 2022-11-15 | 2025-03-24 (Mon) | ed | Emergency Medicine |
| 7 | 2023-07-15 | 2025-11-21 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private commercial insurance (Medicare eligible at 65)",
 "occupation": "Retired warehouse supervisor",
 "alcohol_use": "Social, 1-2 drinks per month",
 "family_history": [
  "Father: Type 2 diabetes mellitus, myocardial infarction at age 65 (deceased)",
  "Mother: Hypertension, ischemic stroke at age 78",
  "Older brother: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 20 pack-years, quit 5 years ago",
 "home_medications": [
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg PO daily at bedtime",
   "name": "Atorvastatin"
  },
  {
   "dose": "12.5 mg PO daily",
   "name": "Hydrochlorothiazide"
  }
 ],
 "surgical_history": [
  "Phacoemulsification with intraocular lens placement, right eye (2021)",
  "Scheduled: Right total knee arthroplasty"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus (20-year duration)",
  "Hypertension",
  "Chronic kidney disease stage 3",
  "Diabetic retinopathy",
  "Diabetic peripheral neuropathy",
  "Osteoarthritis (bilateral knees)",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 62
}
```

Primary diagnoses: ["Acute Decompensated Heart Failure with Pulmonary Edema", "Diabetic foot ulcer in proliferative phase of healing", "ST-segment elevation myocardial infarction, inferior wall", "Helicobacter pylori infection", "Type 2 diabetes mellitus", "Osteoarthritis exacerbation/flare"]
Comorbidities: ["Essential hypertension", "Type 2 diabetes mellitus", "Mixed hyperlipidemia"]

## Encounter 0: source 2020-01-15, practice 2022-05-16, outpatient, Internal Medicine

Chief complaint: Routine follow-up for type 2 diabetes mellitus management

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for type 2 diabetes mellitus management

HISTORY OF PRESENT ILLNESS:
He has been compliant with his medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (20-year duration)
- Hypertension
- Chronic kidney disease stage 3
- Diabetic retinopathy
- Diabetic peripheral neuropathy
- Osteoarthritis (bilateral knees)
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Phacoemulsification with intraocular lens placement, right eye (2021)
- Scheduled: Right total knee arthroplasty

MEDICATIONS:
He takes metformin, lisinopril, and atorvastatin.

Home Medications:
- Hydrochlorothiazide 12.5 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 65 (deceased)
- Mother: Hypertension, ischemic stroke at age 78
- Older brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Social, 1-2 drinks per month. Occupation: Retired warehouse supervisor.

REVIEW OF SYSTEMS:
He denies any chest pain, shortness of breath, or changes in vision.

VITAL SIGNS:
His blood pressure is 138/86 mmHg,

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

LABORATORY DATA:
Recent laboratory results are as follows:

- Hemoglobin A1c: 7.2%
- Fasting plasma glucose: 154 mg/dL
- Total cholesterol: 190 mg/dL
- LDL cholesterol: 100 mg/dL
- HDL cholesterol: 45 mg/dL
- Triglycerides: 250 mg/dL

ASSESSMENT AND PLAN:
The patient is interested in understanding what factors may be contributing to his difficulty in achieving better glycemic and lipid control.

You decide to perform a regression analysis to identify the variables that are most predictive of his hemoglobin A1c levels.

## Encounter 1: source 2020-04-15, practice 2022-08-24, outpatient, Internal Medicine

Chief complaint: Routine follow-up for chronic disease monitoring and medication adherence assessment

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for chronic disease monitoring and medication adherence assessment

HISTORY OF PRESENT ILLNESS:
He returns for chronic disease monitoring. He reports no new symptoms and is adherent to his medication regimen.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Type 2 diabetes mellitus (20-year duration)
- Hypertension
- Chronic kidney disease stage 3
- Diabetic retinopathy
- Diabetic peripheral neuropathy
- Osteoarthritis (bilateral knees)
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Phacoemulsification with intraocular lens placement, right eye (2021)
- Scheduled: Right total knee arthroplasty

MEDICATIONS:
His current medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Hydrochlorothiazide 12.5 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 65 (deceased)
- Mother: Hypertension, ischemic stroke at age 78
- Older brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Social, 1-2 drinks per month. Occupation: Retired warehouse supervisor.

VITAL SIGNS:
His blood pressure is 132/84 mmHg,

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

LABORATORY DATA:
His recent laboratory results are as follows:

- Fasting plasma glucose: 146 mg/dL
- HbA1c: 7.2%
- Total cholesterol: 190 mg/dL
- LDL cholesterol: 100 mg/dL
- HDL cholesterol: 45 mg/dL
- Triglycerides: 250 mg/dL

OTHER STUDIES:
The patient is part of a clinical trial comparing the efficacy of different dietary interventions on lipid profiles in patients with type 2 diabetes. The interventions include a low-fat diet, a Mediterranean diet, and a low-carbohydrate diet. After 6 months, an analysis of variance (ANOVA) test is performed to compare the mean LDL cholesterol levels among the three diet groups.

## Encounter 2: source 2020-09-15, practice 2023-01-23, outpatient, Podiatry

Chief complaint: Routine follow-up for diabetic foot ulcer with foul odor and occasional drainage

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Priya Patel, MD
Department: Podiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for diabetic foot ulcer with foul odor and occasional drainage

HISTORY OF PRESENT ILLNESS:
The patient returns for diabetic foot ulcer follow-up after internal medicine evaluation five months ago. He reports that over the past few weeks, he has noticed a foul smell from his right foot and that it occasionally leaks fluid. He denies any fever or pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Type 2 diabetes mellitus (20-year duration)
- Hypertension
- Chronic kidney disease stage 3
- Diabetic retinopathy
- Diabetic peripheral neuropathy
- Osteoarthritis (bilateral knees)
- Hyperlipidemia

His medical history is significant for hypertension and diabetic retinopathy.

PAST SURGICAL HISTORY:
- Phacoemulsification with intraocular lens placement, right eye (2021)
- Scheduled: Right total knee arthroplasty

MEDICATIONS:
He takes metformin and lisinopril.

Home Medications:
- Atorvastatin 40 mg PO daily at bedtime
- Hydrochlorothiazide 12.5 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 65 (deceased)
- Mother: Hypertension, ischemic stroke at age 78
- Older brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
He wears sandals at home and occasionally walks barefoot indoors.

VITAL SIGNS:
Temperature is 37.1°C, Blood pressure is 142/88 mmHg, Pulse is 84/min, and Respiratory rate 14/min.

PHYSICAL EXAMINATION:
Physical examination reveals a shallow ulcer on the plantar surface of the right foot surrounded by callused skin and minimal erythema. Sensation is diminished in both feet.

LABORATORY DATA:
Labs: HbA1c is 8.9%, WBC is 8,400/mm³.

## Encounter 3: source 2021-03-15, practice 2023-07-17, procedure, Orthopedic Surgery

Chief complaint: Scheduled right total knee arthroplasty

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. James Anderson, MD
Department: Orthopedic Surgery
Visit Type: procedure

CHIEF COMPLAINT:
Scheduled right total knee arthroplasty

HISTORY OF PRESENT ILLNESS:
The patient returns for scheduled arthroplasty with longstanding type 2 diabetes mellitus and prior podiatric evaluation six months ago for diabetic foot ulcer. The patient's name and procedure are confirmed with the patient's wristband and the surgical consent form. Upon reviewing the operating room schedule, the team notes a discrepancy: the schedule lists a left total knee arthroplasty for this patient. The patient insists that his right knee is the correct surgical site.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Diabetic foot ulcer in proliferative phase of healing (diagnosed 2020-09-15)
- Type 2 diabetes mellitus (20-year duration)
- Hypertension
- Chronic kidney disease stage 3
- Diabetic retinopathy
- Diabetic peripheral neuropathy
- Osteoarthritis (bilateral knees)
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Phacoemulsification with intraocular lens placement, right eye (2021)
- Scheduled: Right total knee arthroplasty

MEDICATIONS:
He reports compliance with his medications, including metoprolol and metformin.

Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily at bedtime
- Hydrochlorothiazide 12.5 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 65 (deceased)
- Mother: Hypertension, ischemic stroke at age 78
- Older brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Social, 1-2 drinks per month. Occupation: Retired warehouse supervisor.

VITAL SIGNS:
The patient's blood pressure is 145/90 mmHg, and his blood glucose is 180 mg/dL.

## Encounter 4: source 2021-09-15, practice 2024-01-19, ed, Emergency Medicine

Chief complaint: Fever, chills, and generalized weakness following lower leg injury at beach

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Lisa Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, chills, and generalized weakness following lower leg injury at beach

HISTORY OF PRESENT ILLNESS:
The patient is a 63-year-old male with type 2 diabetes mellitus, peripheral neuropathy, and chronic kidney disease stage 3, status post right total knee arthroplasty in March 2021 and prior diabetic foot ulcer, who presents with acute lower extremity complaints. He had been feeling well until yesterday, when they visited a beach along the Gulf Coast. He spent most of the afternoon walking barefoot along the shoreline and recalls accidentally scraping his lower leg on a sharp object buried in the sand. He has since developed increasing redness and swelling at the site. He also reports having eaten raw oysters at a local restaurant earlier that day.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Diabetic foot ulcer in proliferative phase of healing (diagnosed 2020-09-15)
- Osteoarthritis exacerbation/flare (diagnosed 2021-03-15)
- Type 2 diabetes mellitus (20-year duration)
- Hypertension
- Chronic kidney disease stage 3
- Diabetic retinopathy
- Diabetic peripheral neuropathy
- Osteoarthritis (bilateral knees)
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Phacoemulsification with intraocular lens placement, right eye (2021)
- Scheduled: Right total knee arthroplasty

MEDICATIONS:
He takes metformin and amlodipine.

Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily at bedtime
- Hydrochlorothiazide 12.5 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 65 (deceased)
- Mother: Hypertension, ischemic stroke at age 78
- Older brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Social, 1-2 drinks per month. Occupation: Retired warehouse supervisor.

VITAL SIGNS:
Temperature is 39.0°C (102.2°F), blood pressure is 90/52 mm Hg, and pulse is 118/min

PHYSICAL EXAMINATION:
The patient is ill-appearing and diaphoretic. His right lower leg shows erythema with rapidly spreading edema, dusky discoloration, and several hemorrhagic bullae.

LABORATORY DATA:
Leukocytosis and elevated serum creatinine. Blood cultures are pending.

## Encounter 5: source 2022-05-15, practice 2024-09-20, ed, Emergency Medicine

Chief complaint: Acute shortness of breath following severe smoke inhalation from warehouse fire

ENCOUNTER NOTE
Date: 2022-05-15
Provider: Dr. Robert Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute shortness of breath following severe smoke inhalation from warehouse fire

HISTORY OF PRESENT ILLNESS:
The patient with T2DM, CKD-3, s/p right TKA presents to the emergency department with acute shortness of breath following a severe inhalational injury from a warehouse fire.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Diabetic foot ulcer in proliferative phase of healing (diagnosed 2020-09-15)
- Osteoarthritis exacerbation/flare (diagnosed 2021-03-15)
- Helicobacter pylori infection (diagnosed 2021-09-15)
- Type 2 diabetes mellitus (20-year duration)
- Hypertension
- Chronic kidney disease stage 3
- Diabetic retinopathy
- Diabetic peripheral neuropathy
- Osteoarthritis (bilateral knees)
- Hyperlipidemia

He has a history of hypertension and type 2 diabetes.

PAST SURGICAL HISTORY:
- Phacoemulsification with intraocular lens placement, right eye (2021)
- Scheduled: Right total knee arthroplasty

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily at bedtime
- Hydrochlorothiazide 12.5 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 65 (deceased)
- Mother: Hypertension, ischemic stroke at age 78
- Older brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Social, 1-2 drinks per month. Occupation: Retired warehouse supervisor.

PHYSICAL EXAMINATION:
On arrival, he is hypoxemic and requires high-flow oxygen.

IMAGING:
A chest X-ray shows bilateral diffuse infiltrates.

PATHOLOGY:
The histologic section shown demonstrates alveolar structures, where there is disruption of the cells labeled Z.

OTHER STUDIES:
A lung biopsy is performed for research purposes.

PLAN:
He is intubated and started on mechanical ventilation.

## Encounter 6: source 2022-11-15, practice 2025-03-24, ed, Emergency Medicine

Chief complaint: Sudden-onset severe chest pain radiating to left arm with diaphoresis

ENCOUNTER NOTE
Date: 2022-11-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset severe chest pain radiating to left arm with diaphoresis

HISTORY OF PRESENT ILLNESS:
This patient with 20-year diabetes and prior foot ulcer presents with chest pain. He describes the pain as crushing and rates it 9 of 10 with shortness of breath and diaphoresis.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Diabetic foot ulcer in proliferative phase of healing (diagnosed 2020-09-15)
- Osteoarthritis exacerbation/flare (diagnosed 2021-03-15)
- Helicobacter pylori infection (diagnosed 2021-09-15)
- Type 2 diabetes mellitus (20-year duration)
- Hypertension
- Chronic kidney disease stage 3
- Diabetic retinopathy
- Diabetic peripheral neuropathy
- Osteoarthritis (bilateral knees)
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Phacoemulsification with intraocular lens placement, right eye (2021)
- Scheduled: Right total knee arthroplasty

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Hydrochlorothiazide 12.5 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 65 (deceased)
- Mother: Hypertension, ischemic stroke at age 78
- Older brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Social, 1-2 drinks per month. Occupation: Retired warehouse supervisor.

VITAL SIGNS:
Blood pressure is 160/90 mmHg, heart rate is 110 bpm, and respiratory rate is 22 breaths per minute

PHYSICAL EXAMINATION:
On examination, his He appears anxious and diaphoretic. Cardiac examination reveals a regular rhythm without murmurs, rubs, or gallops. Lung examination is clear to auscultation bilaterally.

LABORATORY DATA:
His initial laboratory results are as follows:

- Troponin I: 0.5 ng/mL (normal <0.04 ng/mL)
- Creatinine: 2.2 mg/dL (normal 0.6-1.2 mg/dL)
- Potassium: 5.5 mEq/L (normal 3.5-5.0 mEq/L)
- Hemoglobin A1c: 7.8% (normal <5.7%)

OTHER STUDIES:
An electrocardiogram (ECG) shows ST-segment elevation in leads II, III, and aVF.

PLAN:
The patient is given aspirin, nitroglycerin, and supplemental oxygen. Despite initial treatment, his chest pain persists.

## Encounter 7: source 2023-07-15, practice 2025-11-21, ed, Emergency Medicine

Chief complaint: Acute shortness of breath with pink frothy sputum and worsening lower extremity edema

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Jennifer Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute shortness of breath with pink frothy sputum and worsening lower extremity edema

HISTORY OF PRESENT ILLNESS:
This diabetic with prior MI presents with shortness of breath that began earlier in the day. He reports a 2-week history of increasing lower extremity edema and paroxysmal nocturnal dyspnea.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Diabetic foot ulcer in proliferative phase of healing (diagnosed 2020-09-15)
- Osteoarthritis exacerbation/flare (diagnosed 2021-03-15)
- Helicobacter pylori infection (diagnosed 2021-09-15)
- ST-segment elevation myocardial infarction, inferior wall (diagnosed 2022-11-15)
- Type 2 diabetes mellitus (20-year duration)
- Hypertension
- Chronic kidney disease stage 3
- Diabetic retinopathy
- Diabetic peripheral neuropathy
- Osteoarthritis (bilateral knees)
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Phacoemulsification with intraocular lens placement, right eye (2021)
- Scheduled: Right total knee arthroplasty

MEDICATIONS:
He takes metformin, lisinopril, and hydrochlorothiazide.

Home Medications:
- Atorvastatin 40 mg PO daily at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 65 (deceased)
- Mother: Hypertension, ischemic stroke at age 78
- Older brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Social, 1-2 drinks per month. Occupation: Retired warehouse supervisor.

VITAL SIGNS:
Tachypneic with a respiratory rate of 30 breaths per minute, blood pressure is 170/90 mmHg, heart rate is 110 beats per minute, and oxygen saturation is 88% on room air

PHYSICAL EXAMINATION:
Lung auscultation reveals bilateral rales halfway up both lung fields. Jugular venous pressure is elevated. The cardiac examination is notable for an S3 gallop.

LABORATORY DATA:
Serum creatinine: 1.5 mg/dL, BNP: 900 pg/mL, Serum potassium: 4.2 mEq/L, BUN: 28 mg/dL

IMAGING:
Chest X-ray shows cardiomegaly and pulmonary edema. ECG shows no acute ST changes.
