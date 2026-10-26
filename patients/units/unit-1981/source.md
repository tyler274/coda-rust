# synthetic_hospital patient 1981

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 57 at first encounter, sex M, Non-Hispanic White, Commercial insurance through employer.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-10-21 (Wed) | outpatient | Gastroenterology |
| 1 | 2020-04-15 | 2021-02-01 (Mon) | outpatient | Internal Medicine |
| 2 | 2020-07-15 | 2021-05-05 (Wed) | outpatient | Cardiology |
| 3 | 2021-01-15 | 2021-10-28 (Thu) | outpatient | Pulmonology |
| 4 | 2021-07-15 | 2022-05-02 (Mon) | outpatient | Cardiology |
| 5 | 2022-07-15 | 2023-04-24 (Mon) | outpatient | Gastroenterology |
| 6 | 2025-01-15 | 2025-11-02 (Sun) | ed | Emergency Medicine |
| 7 | 2025-03-15 | 2025-12-25 (Thu) | inpatient | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial insurance through employer",
 "occupation": "Office Administrator",
 "alcohol_use": "None",
 "family_history": [
  "Father: premature coronary artery disease (myocardial infarction at age 45)",
  "Mother: type 2 diabetes mellitus",
  "Paternal uncle: hyperlipidemia",
  "Family history of gout"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "20 mg oral daily",
   "name": "Lisinopril"
  },
  {
   "dose": "1000 mg oral twice daily",
   "name": "Metformin"
  },
  {
   "dose": "80 mg oral daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "50 mg oral daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "81 mg oral daily",
   "name": "Aspirin"
  },
  {
   "dose": "75 mg oral daily",
   "name": "Clopidogrel"
  },
  {
   "dose": "0.6 mg oral as needed for gout flares",
   "name": "Colchicine"
  }
 ],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Hyperlipidemia",
  "Obesity",
  "Chronic Kidney Disease",
  "Gout"
 ],
 "age_at_first_encounter": 57
}
```

Primary diagnoses: ["Obesity hypoventilation syndrome", "Familial Hypercholesterolemia", "Hyperplastic polyps of colon (small, without dysplasia)", "Colorectal neoplasia", "Crystal-induced arthropathy (Gout or Pseudogout)", "Poorly controlled diabetes mellitus", "Renal vein thrombosis", "Uremic pericarditis"]
Comorbidities: ["Obesity", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2020-10-21, outpatient, Gastroenterology

Chief complaint: Routine health maintenance examination

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health maintenance examination

HISTORY OF PRESENT ILLNESS:
He reports no significant complaints and is adherent to his medications, which include lisinopril and atorvastatin.
but has never undergone colon cancer screening.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Obesity
- Chronic Kidney Disease
- Gout

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes lisinopril and atorvastatin.

Home Medications:
- Metformin 1000 mg oral twice daily
- Metoprolol succinate 50 mg oral daily
- Aspirin 81 mg oral daily
- Clopidogrel 75 mg oral daily
- Colchicine 0.6 mg oral as needed for gout flares

ALLERGIES:
NKDA

FAMILY HISTORY:
He has no family history of colorectal cancer

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Office Administrator.

VITAL SIGNS:
His blood pressure is 132/84 mmHg, and his BMI is 27 kg/m^2.

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

LABORATORY DATA:
Laboratory tests reveal normal complete blood count and metabolic panel. Fecal occult blood test is negative.

PATHOLOGY:
The pathology report confirms hyperplastic polyps without dysplasia.

OTHER STUDIES:
He undergoes a screening colonoscopy, which reveals three small (less than 5 mm) hyperplastic polyps in the sigmoid colon, which are removed.

## Encounter 1: source 2020-04-15, practice 2021-02-01, outpatient, Internal Medicine

Chief complaint: Follow-up of routine blood work

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Okafor, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up of routine blood work

HISTORY OF PRESENT ILLNESS:
This 57-year-old male with hypertension, type 2 diabetes mellitus, hyperlipidemia, obesity, chronic kidney disease, and gout, who recently underwent colonoscopy revealing hyperplastic polyps without dysplasia, returns for follow-up of routine blood work. He states he has been taking his medications as directed and recently started walking around his neighborhood to stay active. He reports feeling mostly well, though he mentions increased thirst and getting up at night to use the bathroom more than he used to. He denies weight changes, vision problems, or recent infections. He was started on a single agent several months ago and has not had any medication changes since. He expresses interest in a medication that can help improve insulin resistance and possibly benefit his liver.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperplastic polyps of colon (small, without dysplasia) (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Obesity
- Chronic Kidney Disease
- Gout

His past medical history includes hypertension and elevated blood sugar levels for the past few years.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He is currently taking lisinopril and metformin.

Home Medications:
- Atorvastatin 80 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Aspirin 81 mg oral daily
- Clopidogrel 75 mg oral daily
- Colchicine 0.6 mg oral as needed for gout flares

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: premature coronary artery disease (myocardial infarction at age 45)
- Mother: type 2 diabetes mellitus
- Paternal uncle: hyperlipidemia
- Family history of gout

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Office Administrator.

VITAL SIGNS:
On exam, his blood pressure is 138/82 mmHg and his BMI is 26 kg/m².

LABORATORY DATA:
His fasting labs show an elevated serum glucose and a glycated hemoglobin above target. His creatinine is within normal limits and liver enzymes are not elevated.

IMAGING:
Ultrasound of the abdomen performed last month for unrelated reasons showed increased hepatic echogenicity.

## Encounter 2: source 2020-07-15, practice 2021-05-05, outpatient, Cardiology

Chief complaint: Follow-up evaluation after recent adjustments to cardiovascular risk management

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Emily Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up evaluation after recent adjustments to cardiovascular risk management

HISTORY OF PRESENT ILLNESS:
He returns for cardiovascular follow-up with recent poorly controlled diabetes and colon polyps. Despite adherence to a low-saturated-fat diet and several months of maximally tolerated statin therapy, his most recent fasting lipid panel reveals persistently elevated low-density lipoprotein (LDL) cholesterol.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperplastic polyps of colon (small, without dysplasia) (diagnosed 2020-01-15)
- Poorly controlled diabetes mellitus (diagnosed 2020-04-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Obesity
- Chronic Kidney Disease
- Gout

He has a history of hypertension, type 2 diabetes mellitus, and long-standing hyperlipidemia.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg oral daily
- Metformin 1000 mg oral twice daily
- Atorvastatin 80 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Aspirin 81 mg oral daily
- Clopidogrel 75 mg oral daily
- Colchicine 0.6 mg oral as needed for gout flares

ALLERGIES:
NKDA

FAMILY HISTORY:
He has no family history of premature myocardial infarction.

SOCIAL HISTORY:
He does not drink alcohol regularly, denies smoking,

REVIEW OF SYSTEMS:
He reports occasional bloating but denies steatorrhea, diarrhea, or weight loss.

VITAL SIGNS:
On examination, his blood pressure is 128/74 mmHg, pulse 78/min, and BMI is 29 kg/m².

PHYSICAL EXAMINATION:
Physical examination shows no xanthomas, xanthelasmas, or corneal arcus. Cardiac and abdominal exams are normal.

PLAN:
His physician prescribes ezetimibe.

## Encounter 3: source 2021-01-15, practice 2021-10-28, outpatient, Pulmonology

Chief complaint: Daytime sleepiness, morning headaches, and difficulty concentrating at work

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Patel, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Daytime sleepiness, morning headaches, and difficulty concentrating at work

HISTORY OF PRESENT ILLNESS:
The patient is a 58-year-old male with hypertension, diabetes, hyperlipidemia, obesity, chronic kidney disease, and gout recently evaluated for familial hypercholesterolemia. He presents with daytime sleepiness, morning headaches, and difficulty concentrating at work. He mentions that his wife has observed him snoring loudly and occasionally gasping for air during sleep.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperplastic polyps of colon (small, without dysplasia) (diagnosed 2020-01-15)
- Poorly controlled diabetes mellitus (diagnosed 2020-04-15)
- Familial Hypercholesterolemia (diagnosed 2020-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Obesity
- Chronic Kidney Disease
- Gout

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg oral daily
- Metformin 1000 mg oral twice daily
- Atorvastatin 80 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Aspirin 81 mg oral daily
- Clopidogrel 75 mg oral daily
- Colchicine 0.6 mg oral as needed for gout flares

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: premature coronary artery disease (myocardial infarction at age 45)
- Mother: type 2 diabetes mellitus
- Paternal uncle: hyperlipidemia
- Family history of gout

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Office Administrator.

VITAL SIGNS:
His blood pressure is 145/92 mm Hg.

PHYSICAL EXAMINATION:
On examination, his BMI is calculated to be 34 kg/m^2. He appears somnolent and has pitting edema in his lower extremities. Lung auscultation reveals decreased breath sounds bilaterally without wheezes or crackles.

LABORATORY DATA:
An arterial blood gas (ABG) is obtained and reveals the following:

- pH: 7.32
- PaCO2: 52 mm Hg
- PaO2: 68 mm Hg
- HCO3^-: 28 mEq/L

## Encounter 4: source 2021-07-15, practice 2022-05-02, outpatient, Cardiology

Chief complaint: Lipid management and joint pain

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Robert Kim, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Lipid management and joint pain

HISTORY OF PRESENT ILLNESS:
The patient is a 58-year-old male with a history of familial hypercholesterolemia, poorly controlled diabetes mellitus, and obesity hypoventilation syndrome who returns for follow-up of cardiovascular risk management and evaluation of joint pain. He recently sustained a non–ST elevation myocardial infarction and was discharged on aspirin, a beta blocker, a statin, and a P2Y12 inhibitor. At his follow-up visit, his LDL cholesterol is at goal, but his HDL remains low and triglycerides are mildly elevated. He states he is motivated to make any changes needed to reduce his risk of another heart event and agrees. He subsequently returns with a painful, swollen right toe.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperplastic polyps of colon (small, without dysplasia) (diagnosed 2020-01-15)
- Poorly controlled diabetes mellitus (diagnosed 2020-04-15)
- Familial Hypercholesterolemia (diagnosed 2020-07-15)
- Obesity hypoventilation syndrome (diagnosed 2021-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Obesity
- Chronic Kidney Disease
- Gout

He has a history of hypertension, type 2 diabetes mellitus, and recurrent gout flares, the most recent of which occurred two months ago.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
His current medications include atorvastatin, metoprolol, lisinopril, metformin, and colchicine as needed.

Home Medications:
- Metoprolol succinate 50 mg oral daily
- Aspirin 81 mg oral daily
- Clopidogrel 75 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: premature coronary artery disease (myocardial infarction at age 45)
- Mother: type 2 diabetes mellitus
- Paternal uncle: hyperlipidemia
- Family history of gout

SOCIAL HISTORY:
He does not drink alcohol.

VITAL SIGNS:
His vital signs are unremarkable

PHYSICAL EXAMINATION:
Physical exam are unremarkable

PATHOLOGY:
Joint aspiration confirms monosodium urate crystals.

ASSESSMENT AND PLAN:
The physician considers adding a second lipid-lowering agent to target residual risk.

## Encounter 5: source 2022-07-15, practice 2023-04-24, outpatient, Gastroenterology

Chief complaint: Routine check-up with change in bowel habits

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Maria Santos, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine check-up with change in bowel habits

HISTORY OF PRESENT ILLNESS:
The patient is a 59-year-old male with hypertension, type 2 diabetes mellitus, hyperlipidemia, obesity, chronic kidney disease, and gout who returns to the Gastroenterology clinic for routine surveillance following prior colonoscopy in 2020 demonstrating hyperplastic polyps. He reports no acute complaints but mentions that he has noticed a change in his bowel habits over the past 6 months, with intermittent constipation and the occasional presence of blood on the toilet paper. He denies any abdominal pain, weight loss, or fever, and his last HbA1c was 7.2%.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperplastic polyps of colon (small, without dysplasia) (diagnosed 2020-01-15)
- Poorly controlled diabetes mellitus (diagnosed 2020-04-15)
- Familial Hypercholesterolemia (diagnosed 2020-07-15)
- Obesity hypoventilation syndrome (diagnosed 2021-01-15)
- Crystal-induced arthropathy (Gout or Pseudogout) (diagnosed 2021-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Obesity
- Chronic Kidney Disease
- Gout

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
His diabetes is managed with metformin

Home Medications:
- Lisinopril 20 mg oral daily
- Atorvastatin 80 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Aspirin 81 mg oral daily
- Clopidogrel 75 mg oral daily
- Colchicine 0.6 mg oral as needed for gout flares

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: premature coronary artery disease (myocardial infarction at age 45)
- Mother: type 2 diabetes mellitus
- Paternal uncle: hyperlipidemia
- Family history of gout

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Office Administrator.

VITAL SIGNS:
His BMI is 32 kg/m^2.

PHYSICAL EXAMINATION:
Physical examination is unremarkable except for mild abdominal distension. Digital rectal examination reveals no masses, but there is a small amount of bright red blood on the examining glove.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin: 12.8 g/dL (Normal: 13.8-17.2 g/dL)
- Fasting glucose: 158 mg/dL (Normal: 70-99 mg/dL)
- Carcinoembryonic antigen (CEA): 5.2 ng/mL (Normal: <3 ng/mL)

## Encounter 6: source 2025-01-15, practice 2025-11-02, ed, Emergency Medicine

Chief complaint: Left flank pain and swelling of left leg

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. David Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Left flank pain and swelling of left leg

HISTORY OF PRESENT ILLNESS:
This 62-year-old male with chronic kidney disease, diabetes mellitus, and obesity presents. He reports a 3-month history of frothy urine and recent weight gain.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperplastic polyps of colon (small, without dysplasia) (diagnosed 2020-01-15)
- Poorly controlled diabetes mellitus (diagnosed 2020-04-15)
- Familial Hypercholesterolemia (diagnosed 2020-07-15)
- Obesity hypoventilation syndrome (diagnosed 2021-01-15)
- Crystal-induced arthropathy (Gout or Pseudogout) (diagnosed 2021-07-15)
- Colorectal neoplasia (diagnosed 2022-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Obesity
- Chronic Kidney Disease
- Gout

His past medical history is significant for hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg oral daily
- Metformin 1000 mg oral twice daily
- Atorvastatin 80 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Aspirin 81 mg oral daily
- Clopidogrel 75 mg oral daily
- Colchicine 0.6 mg oral as needed for gout flares

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: premature coronary artery disease (myocardial infarction at age 45)
- Mother: type 2 diabetes mellitus
- Paternal uncle: hyperlipidemia
- Family history of gout

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Office Administrator.

VITAL SIGNS:
His blood pressure is 150/90 mmHg

PHYSICAL EXAMINATION:
He has pitting edema of the lower extremities, more pronounced on the left

LABORATORY DATA:
Laboratory findings are as follows:

- Serum albumin: 2.1 g/dL (Normal: 3.5-5.0 g/dL)
- Urine protein: 4+ on dipstick
- Serum creatinine: 1.2 mg/dL (Normal: 0.6-1.2 mg/dL)
- Urine RBC: 0-2 per high power field
- LDL cholesterol: 210 mg/dL (Normal: <100 mg/dL)
- Platelet count: 250,000/µL (Normal: 150,000-450,000/µL)

IMAGING:
A renal ultrasound shows loss of venous waveform in the left renal vein.

## Encounter 7: source 2025-03-15, practice 2025-12-25, inpatient, Emergency Medicine

Chief complaint: Sharp, substernal chest pain

ENCOUNTER NOTE
Date: 2025-03-15
Provider: Dr. Jennifer Lee, MD
Department: Emergency Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Sharp, substernal chest pain

HISTORY OF PRESENT ILLNESS:
The patient with chronic kidney disease and recent renal vein thrombosis reports a 2-day history of sharp, substernal chest pain worsening with deep breaths and lying flat. He denies recent illness or trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperplastic polyps of colon (small, without dysplasia) (diagnosed 2020-01-15)
- Poorly controlled diabetes mellitus (diagnosed 2020-04-15)
- Familial Hypercholesterolemia (diagnosed 2020-07-15)
- Obesity hypoventilation syndrome (diagnosed 2021-01-15)
- Crystal-induced arthropathy (Gout or Pseudogout) (diagnosed 2021-07-15)
- Colorectal neoplasia (diagnosed 2022-07-15)
- Renal vein thrombosis (diagnosed 2025-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Obesity
- Chronic Kidney Disease
- Gout

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg oral daily
- Metformin 1000 mg oral twice daily
- Atorvastatin 80 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Aspirin 81 mg oral daily
- Clopidogrel 75 mg oral daily
- Colchicine 0.6 mg oral as needed for gout flares

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: premature coronary artery disease (myocardial infarction at age 45)
- Mother: type 2 diabetes mellitus
- Paternal uncle: hyperlipidemia
- Family history of gout

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Office Administrator.

VITAL SIGNS:
Blood pressure is 150/90 mmHg, heart rate is 98 bpm, and temperature is 36.8°C (98.2°F).

PHYSICAL EXAMINATION:
A pericardial friction rub is heard on auscultation.

LABORATORY DATA:
Blood urea nitrogen (BUN): 72 mg/dL
Creatinine: 6.5 mg/dL
Potassium: 5.2 mEq/L
Hemoglobin: 10.2 g/dL
White blood cell count: 9,000/mm³
Platelet count: 250,000/mm³

OTHER STUDIES:
An electrocardiogram (ECG) shows no ST elevations or T wave inversions.
