# synthetic_hospital patient 1995

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 52 at first encounter, sex M, Non-Hispanic Black, Commercial PPO through employer.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-02-11 (Fri) | outpatient | Internal Medicine |
| 1 | 2021-01-15 | 2023-02-15 (Wed) | outpatient | Internal Medicine |
| 2 | 2022-01-15 | 2024-02-05 (Mon) | outpatient | Internal Medicine |
| 3 | 2023-01-15 | 2025-02-07 (Fri) | outpatient | Cardiology |
| 4 | 2024-01-15 | 2026-02-10 (Tue) | outpatient | Cardiology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO through employer",
 "occupation": "Regional sales manager",
 "alcohol_use": "Social, 2-3 drinks per month",
 "family_history": [
  "Father: Myocardial infarction at age 68",
  "Mother: Hypertension, type 2 diabetes",
  "Paternal uncle: Sudden cardiac death at age 55"
 ],
 "race_ethnicity": "Non-Hispanic Black",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 52
}
```

Primary diagnoses: ["Essential hypertension", "Aortic valve stenosis", "Hypertrophic obstructive cardiomyopathy", "Stable Angina Pectoris (Medical Management)"]
Comorbidities: ["Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2022-02-11, outpatient, Internal Medicine

Chief complaint: presents to the clinic for a follow-up after participating in a randomized controlled trial comparing the efficacy of two antihypertensive medications

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the clinic for a follow-up after participating in a randomized controlled trial comparing the efficacy of two antihypertensive medications

HISTORY OF PRESENT ILLNESS:
He was randomized to receive either drug A or drug B, both of which are known to be effective in lowering blood pressure. His blood pressure at the start of the trial was 158/98 mmHg. After 12 weeks of treatment, his blood pressure is now 138/88 mmHg.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Hypertension, type 2 diabetes
- Paternal uncle: Sudden cardiac death at age 55

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 2-3 drinks per month. Occupation: Regional sales manager.

OTHER STUDIES:
The primary outcome of the study is the mean change in systolic blood pressure from baseline. The 95% confidence interval for the difference in mean systolic blood pressure reduction between drug A and drug B is reported as -3 to +2 mmHg.

## Encounter 1: source 2021-01-15, practice 2023-02-15, outpatient, Internal Medicine

Chief complaint: presents to the clinic for a follow-up visit

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Marcus Johnson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the clinic for a follow-up visit

HISTORY OF PRESENT ILLNESS:
The patient is a 53-year-old male with hypertension and hyperlipidemia returning for longitudinal follow-up after his prior encounter involving antihypertensive drug trial participation. He has been participating in a clinical trial for a new antihypertensive medication. At the start of the trial, his blood pressure was 158/98 mmHg. After 6 months of treatment, his blood pressure averages 138/88 mmHg. The patient is curious about the significance of these results.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Hypertension, type 2 diabetes
- Paternal uncle: Sudden cardiac death at age 55

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 2-3 drinks per month. Occupation: Regional sales manager.

OTHER STUDIES:
The trial's preliminary results show a 95% confidence interval (CI) for the mean reduction in systolic blood pressure of 15 to 25 mmHg.

## Encounter 2: source 2022-01-15, practice 2024-02-05, outpatient, Internal Medicine

Chief complaint: presents to the clinic complaining of intermittent chest pain that occurs during his morning walks

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the clinic complaining of intermittent chest pain that occurs during his morning walks

HISTORY OF PRESENT ILLNESS:
The 54-year-old male returns for follow-up of hypertension and hyperlipidemia. The pain is described as a pressure-like sensation over the left chest that radiates to his left arm, lasting for about 10 minutes and resolving with rest. He denies any shortness of breath, palpitations, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2021-01-15)
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
His current medications include lisinopril and atorvastatin.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Hypertension, type 2 diabetes
- Paternal uncle: Sudden cardiac death at age 55

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 2-3 drinks per month. Occupation: Regional sales manager.

VITAL SIGNS:
On physical examination, his blood pressure is 138/86 mmHg, heart rate is 78 bpm, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
Cardiac examination reveals a regular rhythm without murmurs, rubs, or gallops.

LABORATORY DATA:
His fasting lipid panel is as follows:
- Total cholesterol: 190 mg/dL
- HDL cholesterol: 40 mg/dL
- LDL cholesterol: 120 mg/dL
- Triglycerides: 150 mg/dL

IMAGING:
He is referred for a coronary angiography, which reveals a 70% stenosis of the left anterior descending artery.

OTHER STUDIES:
An electrocardiogram shows no acute changes.
The patient undergoes a stress test, which demonstrates ST-segment depression in the anterior leads at high levels of exertion.

PLAN:
The patient is counseled on management options, including medical therapy versus percutaneous coronary intervention (PCI). A recent randomized controlled trial comparing medical therapy alone to PCI plus medical therapy in patients with stable angina found a p-value of 0.04 for the primary outcome of reducing cardiac death or nonfatal myocardial infarction.

## Encounter 3: source 2023-01-15, practice 2025-02-07, outpatient, Cardiology

Chief complaint: presents to the clinic with a 6-month history of progressive exertional dyspnea and chest pain

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Priya Patel, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the clinic with a 6-month history of progressive exertional dyspnea and chest pain

HISTORY OF PRESENT ILLNESS:
Patient returns with progressive symptoms following prior evaluation for exercise-induced chest pain. He reports symptoms particularly pronounced when climbing stairs or carrying heavy objects.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2021-01-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2022-01-15)
- Hypertension
- Hyperlipidemia

His past medical history is significant for hypertension, which is currently managed with lisinopril.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
His current medications include lisinopril and atorvastatin.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Hypertension, type 2 diabetes
- Paternal uncle: Sudden cardiac death at age 55

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 2-3 drinks per month. Occupation: Regional sales manager.

PHYSICAL EXAMINATION:
On physical examination, you note a harsh systolic murmur that increases in intensity with the Valsalva maneuver and decreases with squatting.

IMAGING:
An echocardiogram reveals asymmetric septal hypertrophy and systolic anterior motion of the mitral valve leaflet, with a left ventricular outflow tract gradient of 55 mmHg at rest.

OTHER STUDIES:
His EKG shows left ventricular hypertrophy.

## Encounter 4: source 2024-01-15, practice 2026-02-10, outpatient, Cardiology

Chief complaint: chest pain on exertion for the past 6 months

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. James O'Brien, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Chest pain on exertion for the past 6 months

HISTORY OF PRESENT ILLNESS:
The patient returns with chest pain on exertion. The pain is described as a pressure-like sensation over the precordium, radiating to the left arm, and is relieved by rest.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2021-01-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2022-01-15)
- Hypertrophic obstructive cardiomyopathy (diagnosed 2023-01-15)
- Hypertension
- Hyperlipidemia

He has a history of hyperlipidemia and hypertension, both controlled with medication.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Hypertension, type 2 diabetes
- Paternal uncle: Sudden cardiac death at age 55

SOCIAL HISTORY:
He denies any history of smoking.

VITAL SIGNS:
On physical examination, his blood pressure is 140/90 mmHg, heart rate is 78 bpm, and BMI is 27 kg/m^2.

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a harsh, systolic ejection murmur best heard at the right upper sternal border that radiates to the neck. There is no carotid bruit, and pulses are equal bilaterally.

IMAGING:
An echocardiogram demonstrates a thickened aortic valve with a peak velocity of 4.5 m/s across the valve and a mean gradient of 50 mmHg.

OTHER STUDIES:
An ECG shows left ventricular hypertrophy.
