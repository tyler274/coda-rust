# synthetic_hospital patient 2038

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-06-28 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2021-12-23 (Thu) | outpatient | Internal Medicine |
| 2 | 2021-07-15 | 2023-01-03 (Tue) | outpatient | Internal Medicine |
| 3 | 2022-01-15 | 2023-06-26 (Mon) | ed | Emergency Medicine |
| 4 | 2023-01-15 | 2024-07-06 (Sat) | ed | Emergency Medicine |
| 5 | 2023-07-15 | 2025-01-03 (Fri) | ed | Emergency Medicine |
| 6 | 2024-01-15 | 2025-07-11 (Fri) | icu | Critical Care Medicine |
| 7 | 2025-01-15 | 2026-07-06 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Construction laborer",
 "alcohol_use": "Severe alcohol use disorder (DSM-5: consumes approximately 1 pint of vodka daily for past 10-15 years; history of underreporting intake as 'social' use; tolerance; withdrawal symptoms; multiple unsuccessful quit attempts; recent increased motivation to quit following family pressure and legal altercation)",
 "family_history": [
  "Father: Myocardial infarction at age 50",
  "Mother: Type 2 diabetes mellitus, hypertension",
  "Paternal uncle: Alcohol use disorder with cirrhosis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  }
 ],
 "surgical_history": [
  "Tonsillectomy, age 8"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Alcohol Use Disorder, Severe"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Alcohol use disorder", "Acute mechanical low back pain", "Diabetic ketoacidosis", "Complicated parapneumonic effusion", "Delirium tremens", "ST-segment elevation myocardial infarction, inferior wall", "Type 2 diabetes mellitus"]
Comorbidities: ["Uncomplicated alcohol withdrawal", "Alcohol use disorder", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2021-06-28, outpatient, Internal Medicine

Chief complaint: Routine health check-up with no acute complaints

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health check-up with no acute complaints

HISTORY OF PRESENT ILLNESS:
He has no complaints and feels generally well.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Alcohol Use Disorder, Severe

He has a history of hypertension.

PAST SURGICAL HISTORY:
- Tonsillectomy, age 8

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg PO daily
- Metformin 1000 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is significant for type 2 diabetes in his father.

SOCIAL HISTORY:
He does not smoke and drinks alcohol socially.

VITAL SIGNS:
On examination, his blood pressure is 132/84 mmHg, heart rate is 78 bpm, and BMI is 30 kg/m^2.

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

LABORATORY DATA:
Fasting plasma glucose: 126 mg/dL
- Hemoglobin A1c: 6.5%
- Total cholesterol: 210 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 145 mg/dL
- Triglycerides: 150 mg/dL

## Encounter 1: source 2020-07-15, practice 2021-12-23, outpatient, Internal Medicine

Chief complaint: History of chronic alcohol use with expressed desire to quit

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
History of chronic alcohol use with expressed desire to quit

HISTORY OF PRESENT ILLNESS:
The patient is a 45-year-old male with hypertension and type 2 diabetes mellitus who returns for follow-up six months after his initial diabetes diagnosis, now presenting with concerns regarding chronic alcohol use. He reports consuming approximately six beers daily for the past 10 years. Recently, he has expressed a desire to quit drinking due to increasing pressure from his family and concerns about his health. He has tried to stop drinking multiple times but has been unsuccessful. He reports experiencing tremors, anxiety, and sweating when he attempts to abstain.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Hypertension
- Alcohol Use Disorder, Severe

His past medical history is significant for hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Tonsillectomy, age 8

MEDICATIONS:
His current medications include lisinopril and metformin.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 50
- Mother: Type 2 diabetes mellitus, hypertension
- Paternal uncle: Alcohol use disorder with cirrhosis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Severe alcohol use disorder (DSM-5: consumes approximately 1 pint of vodka daily for past 10-15 years; history of underreporting intake as 'social' use; tolerance; withdrawal symptoms; multiple unsuccessful quit attempts; recent increased motivation to quit following family pressure and legal altercation). Occupation: Construction laborer.

VITAL SIGNS:
His blood pressure is 140/90 mmHg, heart rate is 96 bpm

PHYSICAL EXAMINATION:
On physical examination, he appears mildly anxious.

LABORATORY DATA:
- Serum gamma-glutamyl transferase (GGT): 95 U/L (normal 0-50 U/L)
- Mean corpuscular volume (MCV): 96 fL (normal 80-96 fL)
- Aspartate aminotransferase (AST): 50 U/L (normal 0-40 U/L)
- Alanine aminotransferase (ALT): 45 U/L (normal 0-41 U/L)

ASSESSMENT AND PLAN:
Given the patient's moderate to severe alcohol use disorder and his desire to achieve abstinence,

## Encounter 2: source 2021-07-15, practice 2023-01-03, outpatient, Internal Medicine

Chief complaint: Routine follow-up with recent desire to cut down drinking after legal altercation

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up with recent desire to cut down drinking after legal altercation

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of his alcohol use disorder in the context of hypertension and diabetes, previously evaluated in July 2020 with expressed desire to quit. He reports a recent desire to cut down on his drinking after a recent altercation with law enforcement. He admits to consuming about a pint of vodka daily for the past 10 years. His last drink was 12 hours ago. The patient rates his motivation to quit drinking at a 6 out of 10.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Alcohol use disorder (diagnosed 2020-07-15)
- Hypertension

PAST SURGICAL HISTORY:
- Tonsillectomy, age 8

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Metformin 1000 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 50
- Mother: Type 2 diabetes mellitus, hypertension
- Paternal uncle: Alcohol use disorder with cirrhosis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Severe alcohol use disorder (DSM-5: consumes approximately 1 pint of vodka daily for past 10-15 years; history of underreporting intake as 'social' use; tolerance; withdrawal symptoms; multiple unsuccessful quit attempts; recent increased motivation to quit following family pressure and legal altercation). Occupation: Construction laborer.

VITAL SIGNS:
BP 150/90 mmHg, HR 100 bpm, Temp 37.2°C, RR 16/min

PHYSICAL EXAMINATION:
He appears anxious and has a mild tremor of his hands. Physical examination reveals an enlarged liver with a smooth edge palpable 3 cm below the right costal margin.

LABORATORY DATA:
Laboratory tests show an AST of 120 U/L, ALT of 50 U/L, and a GGT of 300 U/L.

## Encounter 3: source 2022-01-15, practice 2023-06-26, ed, Emergency Medicine

Chief complaint: Two-week history of mechanical low back pain with request for oxycodone

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-week history of mechanical low back pain with request for oxycodone

HISTORY OF PRESENT ILLNESS:
The patient, with hypertension, type 2 diabetes mellitus, and severe alcohol use disorder with prior reduction attempts, presents with back pain. He describes the pain as a constant dull ache, which worsens with movement and is relieved by rest. He denies any recent trauma, fever, or weight loss. The patient requests a prescription for oxycodone for pain relief, stating that over-the-counter medications have not been effective.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Alcohol use disorder (diagnosed 2020-07-15)
- Hypertension

PAST SURGICAL HISTORY:
- Tonsillectomy, age 8

MEDICATIONS:
Metformin and lisinopril

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 50
- Mother: Type 2 diabetes mellitus, hypertension
- Paternal uncle: Alcohol use disorder with cirrhosis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Severe alcohol use disorder (DSM-5: consumes approximately 1 pint of vodka daily for past 10-15 years; history of underreporting intake as 'social' use; tolerance; withdrawal symptoms; multiple unsuccessful quit attempts; recent increased motivation to quit following family pressure and legal altercation). Occupation: Construction laborer.

PHYSICAL EXAMINATION:
On physical examination, there is tenderness to palpation over the lumbar paraspinal muscles without overlying skin changes. Straight leg raise test is negative bilaterally. Neurological examination is unremarkable.

LABORATORY DATA:
Hemoglobin A1c: 6.8%
- Fasting Blood Glucose: 132 mg/dL
- Serum Creatinine: 1.0 mg/dL
- Urinalysis: Normal

## Encounter 4: source 2023-01-15, practice 2024-07-06, ed, Emergency Medicine

Chief complaint: Ten-day productive cough, fever, and worsening right-sided pleuritic chest pain

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Ten-day productive cough, fever, and worsening right-sided pleuritic chest pain

HISTORY OF PRESENT ILLNESS:
This 48-year-old male with hypertension, type 2 diabetes, and alcohol use disorder was diagnosed with community-acquired pneumonia and started on oral antibiotics 7 days ago. Despite initial improvement, his symptoms have worsened over the past 2 days.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Alcohol use disorder (diagnosed 2020-07-15)
- Acute mechanical low back pain (diagnosed 2022-01-15)
- Hypertension

PAST SURGICAL HISTORY:
- Tonsillectomy, age 8

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Metformin 1000 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 50
- Mother: Type 2 diabetes mellitus, hypertension
- Paternal uncle: Alcohol use disorder with cirrhosis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Severe alcohol use disorder (DSM-5: consumes approximately 1 pint of vodka daily for past 10-15 years; history of underreporting intake as 'social' use; tolerance; withdrawal symptoms; multiple unsuccessful quit attempts; recent increased motivation to quit following family pressure and legal altercation). Occupation: Construction laborer.

VITAL SIGNS:
On examination, he is febrile (38.7°C), tachypneic

PHYSICAL EXAMINATION:
He has decreased breath sounds with dullness to percussion at the right lung base.

LABORATORY DATA:
Laboratory tests reveal leukocytosis.

IMAGING:
A chest x-ray shows a right-sided pleural effusion with loculated areas.

OTHER STUDIES:
Thoracentesis is performed, and the pleural fluid analysis shows:

- Appearance: Turbid
- WBC: 18,000 cells/µL with 80% neutrophils
- Protein (pleural): 4.5 g/dL
- LDH (pleural): 600 U/L
- Glucose (pleural): 30 mg/dL
- pH (pleural): 7.1

## Encounter 5: source 2023-07-15, practice 2025-01-03, ed, Emergency Medicine

Chief complaint: Two-day history of nausea, vomiting, and abdominal pain

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-day history of nausea, vomiting, and abdominal pain

HISTORY OF PRESENT ILLNESS:
This 48-year-old male with Type 2 diabetes and severe alcohol use disorder returns. He admits to increased alcohol consumption over the past week and irregular metformin adherence.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Alcohol use disorder (diagnosed 2020-07-15)
- Acute mechanical low back pain (diagnosed 2022-01-15)
- Complicated parapneumonic effusion (diagnosed 2023-01-15)
- Hypertension

PAST SURGICAL HISTORY:
- Tonsillectomy, age 8

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Metformin 1000 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 50
- Mother: Type 2 diabetes mellitus, hypertension
- Paternal uncle: Alcohol use disorder with cirrhosis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Severe alcohol use disorder (DSM-5: consumes approximately 1 pint of vodka daily for past 10-15 years; history of underreporting intake as 'social' use; tolerance; withdrawal symptoms; multiple unsuccessful quit attempts; recent increased motivation to quit following family pressure and legal altercation). Occupation: Construction laborer.

VITAL SIGNS:
His temperature is 37.6°C (99.7°F), blood pressure is 110/70 mmHg, pulse is 105/min, and respiratory rate is 28/min

PHYSICAL EXAMINATION:
He appears lethargic and has deep, rapid respirations
Physical examination reveals dry mucous membranes and epigastric tenderness

LABORATORY DATA:
Laboratory studies are as follows:

- Serum sodium: 140 mEq/L
- Serum potassium: 4.2 mEq/L
- Serum chloride: 96 mEq/L
- Serum bicarbonate: 12 mEq/L
- Serum glucose: 380 mg/dL
- Serum creatinine: 1.2 mg/dL
- Arterial blood gas: pH 7.25, pCO2 28 mmHg, pO2 98 mmHg

## Encounter 6: source 2024-01-15, practice 2025-07-11, icu, Critical Care Medicine

Chief complaint: Severe confusion, agitation, and visual hallucinations after abrupt alcohol cessation

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Aisha Johnson, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Severe confusion, agitation, and visual hallucinations after abrupt alcohol cessation

HISTORY OF PRESENT ILLNESS:
This 49-year-old male with severe alcohol use disorder, previously evaluated for cessation attempts in 2020 and 2021 and recently hospitalized for diabetic ketoacidosis, presents with acute behavioral changes following abrupt alcohol cessation. She reports he has been drinking heavily for over 15 years, usually consuming a pint of vodka daily, but stopped abruptly 3 days ago. Yesterday, he became restless, sweaty, and unable to sleep. Overnight, he began shouting about "giant insects crawling on the walls" and tried to leave the house barefoot. This morning, he became combative and struck a piece of furniture, injuring his hand.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Alcohol use disorder (diagnosed 2020-07-15)
- Acute mechanical low back pain (diagnosed 2022-01-15)
- Complicated parapneumonic effusion (diagnosed 2023-01-15)
- Diabetic ketoacidosis (diagnosed 2023-07-15)
- Hypertension

PAST SURGICAL HISTORY:
- Tonsillectomy, age 8

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Metformin 1000 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 50
- Mother: Type 2 diabetes mellitus, hypertension
- Paternal uncle: Alcohol use disorder with cirrhosis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Severe alcohol use disorder (DSM-5: consumes approximately 1 pint of vodka daily for past 10-15 years; history of underreporting intake as 'social' use; tolerance; withdrawal symptoms; multiple unsuccessful quit attempts; recent increased motivation to quit following family pressure and legal altercation). Occupation: Construction laborer.

VITAL SIGNS:
Temperature is 38.2°C (100.8°F), blood pressure is 172/98 mm Hg, pulse is 136/min, and respirations are 24/min. Oxygen saturation is 97% on room air.

PHYSICAL EXAMINATION:
On arrival, he is flushed, diaphoretic, and tremulous. He is disoriented to time and place, cannot follow two-step commands, and is responding to visual hallucinations. Cardiac monitoring shows sinus tachycardia. Neurologic exam reveals coarse tremors without focal deficits.

LABORATORY DATA:
Laboratory studies are pending.

## Encounter 7: source 2025-01-15, practice 2026-07-06, ed, Emergency Medicine

Chief complaint: Sudden-onset severe chest pressure radiating to left arm

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset severe chest pressure radiating to left arm

HISTORY OF PRESENT ILLNESS:
This 50-year-old male with severe alcohol use disorder (prior delirium tremens) and Type 2 diabetes mellitus complicated by prior diabetic ketoacidosis presents with sudden-onset severe chest pressure radiating to the left arm. He describes the pain as "pressure-like" and rates it 9 out of 10. He also reports shortness of breath and diaphoresis. He recalls a similar episode two months ago diagnosed as gastroesophageal reflux disease after a normal ECG and stress test.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Alcohol use disorder (diagnosed 2020-07-15)
- Acute mechanical low back pain (diagnosed 2022-01-15)
- Complicated parapneumonic effusion (diagnosed 2023-01-15)
- Diabetic ketoacidosis (diagnosed 2023-07-15)
- Delirium tremens (diagnosed 2024-01-15)
- Hypertension

PAST SURGICAL HISTORY:
- Tonsillectomy, age 8

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Metformin 1000 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 50
- Mother: Type 2 diabetes mellitus, hypertension
- Paternal uncle: Alcohol use disorder with cirrhosis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Severe alcohol use disorder (DSM-5: consumes approximately 1 pint of vodka daily for past 10-15 years; history of underreporting intake as 'social' use; tolerance; withdrawal symptoms; multiple unsuccessful quit attempts; recent increased motivation to quit following family pressure and legal altercation). Occupation: Construction laborer.

VITAL SIGNS:
His blood pressure is 160/90 mmHg, heart rate is 110 bpm, and respiratory rate is 22 breaths per minute.

PHYSICAL EXAMINATION:
Physical examination reveals a diaphoretic male in distress. Cardiac auscultation is notable for a regular rhythm without murmurs, rubs, or gallops. Lung auscultation is clear bilaterally.

LABORATORY DATA:
His initial troponin I level is 0.5 ng/mL (normal <0.04 ng/mL).
Laboratory Values:
- Troponin I: 0.5 ng/mL (normal <0.04 ng/mL)
- Creatinine: 1.2 mg/dL (normal 0.6-1.3 mg/dL)
- Hemoglobin A1c: 7.8% (normal <5.7% for non-diabetics)

OTHER STUDIES:
An ECG shows ST-segment elevations in leads II, III, and aVF.

ASSESSMENT AND PLAN:
The emergency physician is considering thrombolytic therapy

The patient is given aspirin, nitroglycerin, and morphine, and the pain improves slightly.
