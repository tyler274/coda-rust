# synthetic_hospital patient 1984

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 47 at first encounter, sex M, Non-Hispanic White, Commercial PPO (Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-03-22 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2021-06-21 (Mon) | outpatient | Endocrinology |
| 2 | 2021-01-15 | 2022-03-24 (Thu) | outpatient | Cardiology |
| 3 | 2021-07-15 | 2022-09-05 (Mon) | outpatient | Internal Medicine |
| 4 | 2022-07-15 | 2023-09-21 (Thu) | outpatient | Psychiatry |
| 5 | 2024-01-15 | 2025-03-13 (Thu) | ed | Emergency Medicine |
| 6 | 2024-07-15 | 2025-09-11 (Thu) | outpatient | Internal Medicine |
| 7 | 2025-01-15 | 2026-03-16 (Mon) | outpatient | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO (Blue Cross Blue Shield)",
 "occupation": "Construction Site Supervisor",
 "alcohol_use": "Heavy daily alcohol consumption (6+ beers on weekday evenings, increased quantity on weekends); patient frequently minimizes intake as 'social drinking' or 'a few beers to relax'",
 "family_history": [
  "Father: Type 2 Diabetes Mellitus, Essential Hypertension, premature coronary artery disease",
  "Mother: Obesity, Hypertension",
  "Brother: Alcohol Use Disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current cigarette smoker, 30-45 pack-year history (approximately 1-1.5 packs per day since age 17, with variable self-reporting between 0.5-1.5 packs/day depending on clinical context)",
 "home_medications": [
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "200 mg PO PRN headache",
   "name": "Ibuprofen"
  }
 ],
 "surgical_history": [
  "No significant surgical history"
 ],
 "chronic_conditions": [
  "Essential Hypertension",
  "Type 2 Diabetes Mellitus",
  "Metabolic Syndrome",
  "Tobacco Use Disorder",
  "Alcohol Use Disorder",
  "Obesity"
 ],
 "age_at_first_encounter": 47
}
```

Primary diagnoses: ["Assessment of decision-making capacity", "Essential hypertension", "Metabolic syndrome", "Tobacco use disorder", "Patient anger/acute situational irritability", "Alcohol use disorder", "Hypertensive retinopathy", "Prediabetes"]
Comorbidities: ["Poorly controlled diabetes mellitus", "Medication noncompliance", "Obesity", "Tobacco use disorder", "Metabolic syndrome", "Obesity", "Long-distance running", "Essential hypertension", "Mixed hyperlipidemia"]

## Encounter 0: source 2020-01-15, practice 2021-03-22, outpatient, Internal Medicine

Chief complaint: Routine health examination for metabolic syndrome screening.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health examination for metabolic syndrome screening.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential Hypertension
- Type 2 Diabetes Mellitus
- Metabolic Syndrome
- Tobacco Use Disorder
- Alcohol Use Disorder
- Obesity

PAST SURGICAL HISTORY:
- No significant surgical history

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Metformin 1000 mg PO BID
- Ibuprofen 200 mg PO PRN headache

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is notable for coronary artery disease in his father, who had a myocardial infarction at age 52

SOCIAL HISTORY:
He is a marketing executive with a high-stress job and admits to occasionally smoking cigars at social events. He denies any regular tobacco use and states that he drinks alcohol "only socially."

VITAL SIGNS:
On examination, his blood pressure is 142/88 mmHg, heart rate is 78 bpm, and BMI is 29 kg/m^2

PHYSICAL EXAMINATION:
His physical examination is unremarkable

LABORATORY DATA:
Laboratory values are as follows:

- Total cholesterol: 220 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 150 mg/dL
- Triglycerides: 175 mg/dL
- Fasting glucose: 96 mg/dL

## Encounter 1: source 2020-04-15, practice 2021-06-21, outpatient, Endocrinology

Chief complaint: Routine follow-up for abnormal glucose screening.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for abnormal glucose screening.

PAST MEDICAL HISTORY:
Active Problem List:
- Metabolic syndrome (diagnosed 2020-01-15)
- Essential Hypertension
- Type 2 Diabetes Mellitus
- Tobacco Use Disorder
- Alcohol Use Disorder
- Obesity

He has a history of hypertension diagnosed 5 years ago.

PAST SURGICAL HISTORY:
- No significant surgical history

MEDICATIONS:
He takes currently takes lisinopril.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Metformin 1000 mg PO BID
- Ibuprofen 200 mg PO PRN headache

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, Essential Hypertension, premature coronary artery disease
- Mother: Obesity, Hypertension
- Brother: Alcohol Use Disorder

SOCIAL HISTORY:
He is a heavy smoker, averaging 1.5 packs per day for the past 30 years. He does not exercise regularly. He eats fast food most days of the week.

VITAL SIGNS:
His BMI is 30 kg/m². His blood pressure is 138/88 mm Hg.

LABORATORY DATA:
Fasting glucose is 105 mg/dL, and HbA1c is 5.9%.

ASSESSMENT AND PLAN:
The physician discusses his risk of developing type 2 diabetes and the importance of preventive strategies.

## Encounter 2: source 2021-01-15, practice 2022-03-24, outpatient, Cardiology

Chief complaint: Routine follow-up for blood pressure management and recent weight gain.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Jennifer Okafor, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for blood pressure management and recent weight gain.

HISTORY OF PRESENT ILLNESS:
The patient with hypertension, diabetes, metabolic syndrome and obesity, previously evaluated for metabolic screening and prediabetes, returns for routine follow-up. He reports compliance with his current regimen of hydrochlorothiazide and enalapril but admits to continued difficulty in controlling his blood pressure. He is concerned about his weight, which has increased by 10 pounds since his last visit 6 months ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Metabolic syndrome (diagnosed 2020-01-15)
- Prediabetes (diagnosed 2020-04-15)
- Essential Hypertension
- Type 2 Diabetes Mellitus
- Tobacco Use Disorder
- Alcohol Use Disorder
- Obesity

PAST SURGICAL HISTORY:
- No significant surgical history

MEDICATIONS:
He takes hydrochlorothiazide and enalapril.

Home Medications:
- Lisinopril 20 mg PO daily
- Metformin 1000 mg PO BID
- Ibuprofen 200 mg PO PRN headache

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, Essential Hypertension, premature coronary artery disease
- Mother: Obesity, Hypertension
- Brother: Alcohol Use Disorder

SOCIAL HISTORY:
He has a 30-pack-year smoking history and drinks alcohol socially on weekends. His diet is high in processed foods and he exercises infrequently.

VITAL SIGNS:
Blood pressure is 146/92 mmHg, heart rate is 78 bpm

PHYSICAL EXAMINATION:
His current BMI is 32 kg/m^2. On physical examination, waist circumference is 102 cm.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum sodium: 140 mEq/L
- Serum potassium: 3.8 mEq/L
- Serum creatinine: 1.1 mg/dL
- Fasting glucose: 102 mg/dL
- Total cholesterol: 210 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 145 mg/dL
- Triglycerides: 150 mg/dL

## Encounter 3: source 2021-07-15, practice 2022-09-05, outpatient, Internal Medicine

Chief complaint: Routine follow-up and smoking cessation counseling.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James Wilson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up and smoking cessation counseling.

HISTORY OF PRESENT ILLNESS:
The patient is a 48-year-old male with a history of metabolic syndrome, prediabetes, and essential hypertension who returns for routine follow-up and smoking cessation counseling. He was last evaluated in Cardiology six months prior for blood pressure management and weight gain concerns. He reports that he has been thinking about quitting smoking for the past year but has not taken any steps towards cessation. He understands that smoking is detrimental to his health, especially given his medical history, but expresses concern about his ability to quit successfully. He has attempted to quit in the past but relapsed within a few weeks each time.

PAST MEDICAL HISTORY:
Active Problem List:
- Metabolic syndrome (diagnosed 2020-01-15)
- Prediabetes (diagnosed 2020-04-15)
- Essential hypertension (diagnosed 2021-01-15)
- Type 2 Diabetes Mellitus
- Tobacco Use Disorder
- Alcohol Use Disorder
- Obesity

PAST SURGICAL HISTORY:
- No significant surgical history

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Metformin 1000 mg PO BID
- Ibuprofen 200 mg PO PRN headache

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, Essential Hypertension, premature coronary artery disease
- Mother: Obesity, Hypertension
- Brother: Alcohol Use Disorder

SOCIAL HISTORY:
He reports smoking a pack of cigarettes daily for the past 30 years.

VITAL SIGNS:
His blood pressure today is 142/88 mmHg, and his BMI is 31 kg/m^2.

PHYSICAL EXAMINATION:
Physical examination is unremarkable except for mild obesity.

LABORATORY DATA:
His most recent laboratory results are as follows:

- Hemoglobin A1c: 7.8%
- Fasting plasma glucose: 172 mg/dL
- Total cholesterol: 210 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 145 mg/dL
- Triglycerides: 250 mg/dL

ASSESSMENT AND PLAN:
Based on the patient's stage of behavior change,

## Encounter 4: source 2022-07-15, practice 2023-09-21, outpatient, Psychiatry

Chief complaint: Routine health examination with behavioral health screening.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Priya Patel, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health examination with behavioral health screening.

HISTORY OF PRESENT ILLNESS:
The patient is a 49-year-old male with essential hypertension, type 2 diabetes mellitus, metabolic syndrome, tobacco use disorder, and obesity who presents for psychiatric evaluation following prior smoking cessation counseling. He returns today for routine health examination with behavioral health screening. On further questioning, he reveals that he consumes six beers every weekday evening and often more on weekends. He insists that this does not affect his work or personal life. His wife, who accompanied him, looks concerned and interjects that he has been more irritable lately and that they have had several arguments about his drinking.

PAST MEDICAL HISTORY:
Active Problem List:
- Metabolic syndrome (diagnosed 2020-01-15)
- Prediabetes (diagnosed 2020-04-15)
- Essential hypertension (diagnosed 2021-01-15)
- Tobacco use disorder (diagnosed 2021-07-15)
- Type 2 Diabetes Mellitus
- Alcohol Use Disorder
- Obesity

He denies any chronic illnesses.

PAST SURGICAL HISTORY:
- No significant surgical history

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Metformin 1000 mg PO BID
- Ibuprofen 200 mg PO PRN headache

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, Essential Hypertension, premature coronary artery disease
- Mother: Obesity, Hypertension
- Brother: Alcohol Use Disorder

SOCIAL HISTORY:
He is a construction worker and mentions occasional job-related stress. He smokes half a pack of cigarettes per day and admits to drinking "a few beers" after work to relax.

VITAL SIGNS:
His blood pressure is 140/90 mmHg, heart rate is 88 beats per minute, and BMI is 29 kg/m^2.

PHYSICAL EXAMINATION:
His liver is palpable 2 cm below the right costal margin, and his skin examination reveals spider angiomas on the upper torso.

LABORATORY DATA:
Serum gamma-glutamyl transferase (GGT): 95 U/L (Normal: 9-48 U/L), Mean corpuscular volume (MCV): 96 fL (Normal: 80-96 fL), Aspartate aminotransferase (AST): 70 U/L (Normal: 10-40 U/L), Alanine aminotransferase (ALT): 50 U/L (Normal: 7-56 U/L)

## Encounter 5: source 2024-01-15, practice 2025-03-13, ed, Emergency Medicine

Chief complaint: Persistent morning headaches, fatigue, and shortness of breath with exertion.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Persistent morning headaches, fatigue, and shortness of breath with exertion.

HISTORY OF PRESENT ILLNESS:
The patient is a 51-year-old male with metabolic syndrome, hypertension, type 2 diabetes, and substance use disorders who presents with persistent morning headaches and shortness of breath with exertion that has gradually worsened over the past few months. He describes the headaches as dull, bilateral, and more prominent in the early morning. He also notes that climbing stairs leaves him more winded than usual, though he has no chest pain, palpitations, or recent illness. He has not been exercising regularly and admits to a high-sodium diet and irregular medication use. He denies vision changes, dizziness, or weakness.

PAST MEDICAL HISTORY:
Active Problem List:
- Metabolic syndrome (diagnosed 2020-01-15)
- Prediabetes (diagnosed 2020-04-15)
- Essential hypertension (diagnosed 2021-01-15)
- Tobacco use disorder (diagnosed 2021-07-15)
- Alcohol use disorder (diagnosed 2022-07-15)
- Type 2 Diabetes Mellitus
- Obesity

Past medical history includes essential hypertension diagnosed 5 years ago, but he has been nonadherent with his follow-ups and medications.

PAST SURGICAL HISTORY:
- No significant surgical history

MEDICATIONS:
His only medication is hydrochlorothiazide, which he takes inconsistently.

Home Medications:
- Lisinopril 20 mg PO daily
- Metformin 1000 mg PO BID
- Ibuprofen 200 mg PO PRN headache

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, Essential Hypertension, premature coronary artery disease
- Mother: Obesity, Hypertension
- Brother: Alcohol Use Disorder

SOCIAL HISTORY:
He smokes one pack of cigarettes per day and drinks alcohol on weekends.

VITAL SIGNS:
On examination, his blood pressure is 188/106 mm Hg, pulse is 84/min, and respiratory rate is 18/min.

PHYSICAL EXAMINATION:
Cardiopulmonary exam reveals normal heart sounds and mild bibasilar crackles. Neurologic and abdominal exams are unremarkable. Visual acuity is normal.

## Encounter 6: source 2024-07-15, practice 2025-09-11, outpatient, Internal Medicine

Chief complaint: Urgent clinic visit for medication noncompliance and glucose monitoring issues.

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Maria Santos, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Urgent clinic visit for medication noncompliance and glucose monitoring issues.

HISTORY OF PRESENT ILLNESS:
The patient returns for urgent follow-up regarding metabolic syndrome, essential hypertension, and tobacco use disorder, following an emergency department presentation for hypertensive complications six months prior. He has been noncompliant with his antihypertensive medication and has not been monitoring his blood glucose regularly. He acknowledges the need to improve his health but expresses uncertainty about his ability to quit smoking and lose weight. He is interested in learning more about the benefits and challenges of these lifestyle changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Metabolic syndrome (diagnosed 2020-01-15)
- Prediabetes (diagnosed 2020-04-15)
- Essential hypertension (diagnosed 2021-01-15)
- Tobacco use disorder (diagnosed 2021-07-15)
- Alcohol use disorder (diagnosed 2022-07-15)
- Hypertensive retinopathy (diagnosed 2024-01-15)
- Type 2 Diabetes Mellitus
- Obesity

PAST SURGICAL HISTORY:
- No significant surgical history

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Metformin 1000 mg PO BID
- Ibuprofen 200 mg PO PRN headache

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, Essential Hypertension, premature coronary artery disease
- Mother: Obesity, Hypertension
- Brother: Alcohol Use Disorder

SOCIAL HISTORY:
He admits to smoking a pack of cigarettes daily for the past 30 years

VITAL SIGNS:
He has a body mass index (BMI) of 32 kg/m^2
His blood pressure is 152/96 mmHg

PHYSICAL EXAMINATION:
Physical examination is unremarkable except for acanthosis nigricans around the neck.

LABORATORY DATA:
, and his most recent laboratory results are as follows:

- Hemoglobin A1c: 8.2%
- Fasting plasma glucose: 176 mg/dL
- Total cholesterol: 220 mg/dL
- HDL cholesterol: 32 mg/dL
- LDL cholesterol: 145 mg/dL
- Triglycerides: 250 mg/dL

## Encounter 7: source 2025-01-15, practice 2026-03-16, outpatient, Internal Medicine

Chief complaint: Acute situational irritability and anger during scheduled appointment.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. David Thompson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Acute situational irritability and anger during scheduled appointment.

HISTORY OF PRESENT ILLNESS:
The patient with hypertension, diabetes, and metabolic syndrome returns for follow-up after prior medication noncompliance. He appears visibly upset and raises his voice, stating, "I've been waiting for over an hour, and no one seems to care! I have other things to do, you know!" He has no additional complaints

PAST MEDICAL HISTORY:
Active Problem List:
- Metabolic syndrome (diagnosed 2020-01-15)
- Prediabetes (diagnosed 2020-04-15)
- Essential hypertension (diagnosed 2021-01-15)
- Tobacco use disorder (diagnosed 2021-07-15)
- Alcohol use disorder (diagnosed 2022-07-15)
- Hypertensive retinopathy (diagnosed 2024-01-15)
- Assessment of decision-making capacity (diagnosed 2024-07-15)
- Type 2 Diabetes Mellitus
- Obesity

His medical history is significant for noncompliance with his antihypertensive medications.

PAST SURGICAL HISTORY:
- No significant surgical history

MEDICATIONS:
His current medications include metformin and occasional ibuprofen for headaches.

Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, Essential Hypertension, premature coronary artery disease
- Mother: Obesity, Hypertension
- Brother: Alcohol Use Disorder

SOCIAL HISTORY:
Smoking: Current cigarette smoker, 30-45 pack-year history (approximately 1-1.5 packs per day since age 17, with variable self-reporting between 0.5-1.5 packs/day depending on clinical context). Alcohol: Heavy daily alcohol consumption (6+ beers on weekday evenings, increased quantity on weekends); patient frequently minimizes intake as 'social drinking' or 'a few beers to relax'. Occupation: Construction Site Supervisor.

VITAL SIGNS:
On examination, his blood pressure is 160/100 mmHg, and his heart rate is 88 beats per minute.

PHYSICAL EXAMINATION:
His physical examination is otherwise unremarkable.

LABORATORY DATA:
Laboratory values are as follows:

- Fasting blood glucose: 158 mg/dL
- HbA1c: 7.8%
- Serum creatinine: 1.2 mg/dL
- Urinalysis: No proteinuria or hematuria

ASSESSMENT AND PLAN:
Given the patient's demeanor and the need to address his medical management,
