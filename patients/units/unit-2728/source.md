# synthetic_hospital patient 2728

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 44 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-03-21 (Mon) | outpatient | Endocrinology |
| 1 | 2020-07-15 | 2022-09-19 (Mon) | outpatient | Endocrinology |
| 2 | 2022-01-15 | 2024-03-19 (Tue) | outpatient | Gastroenterology |
| 3 | 2022-07-15 | 2024-09-30 (Mon) | outpatient | Gastroenterology |
| 4 | 2023-07-15 | 2025-09-15 (Mon) | ed | Emergency Medicine |
| 5 | 2024-01-15 | 2026-03-26 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Self-employed landscaper and horticulturist",
 "alcohol_use": "Social, 1-2 glasses of wine monthly",
 "family_history": [
  "Father: Hypertension, ruptured cerebral aneurysm at age 67",
  "Mother: Type 2 diabetes mellitus, hyperlipidemia",
  "Paternal aunt: Primary hyperaldosteronism"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
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
   "dose": "40 mEq PO BID",
   "name": "Potassium chloride"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Pantoprazole"
  }
 ],
 "surgical_history": [
  "Diagnostic laparoscopy (negative) age 32"
 ],
 "chronic_conditions": [
  "Primary hyperaldosteronism (Conn syndrome)",
  "Resistant hypertension",
  "Pancreatic neuroendocrine tumor (VIPoma)",
  "Peptic ulcer disease",
  "Chronic hypokalemia"
 ],
 "age_at_first_encounter": 44
}
```

Primary diagnoses: ["Primary Hyperaldosteronism (Conn's Syndrome)", "Pancreatic neuroendocrine tumor localization", "Primary hyperaldosteronism", "Hypokalemia managed with oral replacement (tolerating PO intake)", "Ruptured cerebral aneurysm"]
Comorbidities: ["Hypochloremic metabolic alkalosis secondary to vomiting with hypovolemia", "Dehydration", "Hypokalemia managed with oral replacement (tolerating PO intake)", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2022-03-21, outpatient, Endocrinology

Chief complaint: worsening headaches, fatigue, and muscle weakness

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening headaches, fatigue, and muscle weakness

HISTORY OF PRESENT ILLNESS:
6-month history of worsening headaches, fatigue, and muscle weakness. She also reports episodes of palpitations and diaphoresis.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary hyperaldosteronism (Conn syndrome)
- Resistant hypertension
- Pancreatic neuroendocrine tumor (VIPoma)
- Peptic ulcer disease
- Chronic hypokalemia

Her past medical history is significant for hypertension, which was diagnosed 2 years ago and has been poorly controlled.

PAST SURGICAL HISTORY:
- Diagnostic laparoscopy (negative) age 32

MEDICATIONS:
Three antihypertensive medications, including a calcium channel blocker, an ACE inhibitor, and a thiazide diuretic

Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Potassium chloride 40 mEq PO BID
- Pantoprazole 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, ruptured cerebral aneurysm at age 67
- Mother: Type 2 diabetes mellitus, hyperlipidemia
- Paternal aunt: Primary hyperaldosteronism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 glasses of wine monthly. Occupation: Self-employed landscaper and horticulturist.

VITAL SIGNS:
Blood pressure is 158/100 mmHg

PHYSICAL EXAMINATION:
There are no abdominal bruits

LABORATORY DATA:
Serum potassium: 3.1 mEq/L (normal: 3.5-5.0 mEq/L)
Serum sodium: 145 mEq/L (normal: 135-145 mEq/L)
Plasma aldosterone concentration: Elevated
Plasma renin activity: Low

## Encounter 1: source 2020-07-15, practice 2022-09-19, outpatient, Endocrinology

Chief complaint: persistent headaches and episodes of muscle weakness

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent headaches and episodes of muscle weakness

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of primary hyperaldosteronism with 6-month history of persistent headaches and episodes of muscle weakness. Despite medication, her blood pressure remains elevated.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary Hyperaldosteronism (Conn's Syndrome) (diagnosed 2020-01-15)
- Primary hyperaldosteronism (Conn syndrome)
- Resistant hypertension
- Pancreatic neuroendocrine tumor (VIPoma)
- Peptic ulcer disease
- Chronic hypokalemia

PAST SURGICAL HISTORY:
- Diagnostic laparoscopy (negative) age 32

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 10 mg PO daily
- Potassium chloride 40 mEq PO BID
- Pantoprazole 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
She denies any family history of endocrine disorders

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 glasses of wine monthly. Occupation: Self-employed landscaper and horticulturist.

VITAL SIGNS:
Blood pressure is 158/100 mmHg

PHYSICAL EXAMINATION:
No signs of edema

LABORATORY DATA:
Serum sodium: 146 mEq/L (Normal: 135-145 mEq/L)
- Serum potassium: 2.9 mEq/L (Normal: 3.5-5.1 mEq/L)
- Serum bicarbonate: 32 mEq/L (Normal: 22-29 mEq/L)
- Plasma renin activity: Low
- Serum aldosterone: Elevated

## Encounter 2: source 2022-01-15, practice 2024-03-19, outpatient, Gastroenterology

Chief complaint: Profuse, watery diarrhea

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Profuse, watery diarrhea

HISTORY OF PRESENT ILLNESS:
The patient returns following prior endocrinology encounters for hyperaldosteronism and reports several weeks of profuse, watery diarrhea that persists even when she is fasting. She also reports muscle cramps and episodes of lightheadedness.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary Hyperaldosteronism (Conn's Syndrome) (diagnosed 2020-01-15)
- Primary hyperaldosteronism (diagnosed 2020-07-15)
- Primary hyperaldosteronism (Conn syndrome)
- Resistant hypertension
- Pancreatic neuroendocrine tumor (VIPoma)
- Peptic ulcer disease
- Chronic hypokalemia

PAST SURGICAL HISTORY:
- Diagnostic laparoscopy (negative) age 32

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Potassium chloride 40 mEq PO BID
- Pantoprazole 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, ruptured cerebral aneurysm at age 67
- Mother: Type 2 diabetes mellitus, hyperlipidemia
- Paternal aunt: Primary hyperaldosteronism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 glasses of wine monthly. Occupation: Self-employed landscaper and horticulturist.

PHYSICAL EXAMINATION:
Signs of dehydration

LABORATORY DATA:
Potassium level of 2 mEq/L, a pH of 7.30, glucose of 150 mg/dl and low stool osmotic gap

## Encounter 3: source 2022-07-15, practice 2024-09-30, outpatient, Gastroenterology

Chief complaint: 6-month history of profuse watery diarrhea and a recent onset of muscle cramps

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of profuse watery diarrhea and a recent onset of muscle cramps

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of chronic diarrhea initially assessed six months prior, with history of hyperaldosteronism and neuroendocrine tumor. She reports having up to eight bowel movements per day, which are not bloody or associated with abdominal pain. Despite the diarrhea, she has not experienced any significant weight loss. She denies any recent travel or sick contacts.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary Hyperaldosteronism (Conn's Syndrome) (diagnosed 2020-01-15)
- Primary hyperaldosteronism (diagnosed 2020-07-15)
- Pancreatic neuroendocrine tumor localization (diagnosed 2022-01-15)
- Primary hyperaldosteronism (Conn syndrome)
- Resistant hypertension
- Pancreatic neuroendocrine tumor (VIPoma)
- Peptic ulcer disease
- Chronic hypokalemia

PAST SURGICAL HISTORY:
- Diagnostic laparoscopy (negative) age 32

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Potassium chloride 40 mEq PO BID
- Pantoprazole 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, ruptured cerebral aneurysm at age 67
- Mother: Type 2 diabetes mellitus, hyperlipidemia
- Paternal aunt: Primary hyperaldosteronism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 glasses of wine monthly. Occupation: Self-employed landscaper and horticulturist.

VITAL SIGNS:
On physical examination, her blood pressure is 110/70 mmHg, heart rate is 95 bpm

PHYSICAL EXAMINATION:
She appears dehydrated. Abdominal examination is unremarkable with no masses or organomegaly.

LABORATORY DATA:
Laboratory studies reveal:
- Sodium: 140 mEq/L
- Potassium: 2.9 mEq/L
- Chloride: 95 mEq/L
- Bicarbonate: 32 mEq/L
- Blood urea nitrogen (BUN): 18 mg/dL
- Creatinine: 0.9 mg/dL
- Fasting glucose: 90 mg/dL
- Stool osmotic gap: 50 mOsm/kg
- Gastrin level: Normal
- Vasoactive intestinal peptide (VIP) level: 200 pg/mL

## Encounter 4: source 2023-07-15, practice 2025-09-15, ed, Emergency Medicine

Chief complaint: 2-day history of severe nausea and repeated episodes of vomiting

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-day history of severe nausea and repeated episodes of vomiting

HISTORY OF PRESENT ILLNESS:
This patient with Conn syndrome and VIPoma presents to the ED. She reports that she has been unable to keep any food or liquids down. She has been experiencing intermittent epigastric pain for the past few weeks.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary Hyperaldosteronism (Conn's Syndrome) (diagnosed 2020-01-15)
- Primary hyperaldosteronism (diagnosed 2020-07-15)
- Pancreatic neuroendocrine tumor localization (diagnosed 2022-01-15)
- Primary hyperaldosteronism (Conn syndrome)
- Resistant hypertension
- Pancreatic neuroendocrine tumor (VIPoma)
- Peptic ulcer disease
- Chronic hypokalemia

She has a past medical history of peptic ulcer disease.

PAST SURGICAL HISTORY:
- Diagnostic laparoscopy (negative) age 32

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Potassium chloride 40 mEq PO BID
- Pantoprazole 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, ruptured cerebral aneurysm at age 67
- Mother: Type 2 diabetes mellitus, hyperlipidemia
- Paternal aunt: Primary hyperaldosteronism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 glasses of wine monthly. Occupation: Self-employed landscaper and horticulturist.

VITAL SIGNS:
Her blood pressure is 100/60 mmHg, heart rate is 110 beats per minute

PHYSICAL EXAMINATION:
She appears dehydrated

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum sodium: 140 mEq/L
- Serum potassium: 2.9 mEq/L
- Serum chloride: 90 mEq/L
- Serum bicarbonate: 32 mEq/L
- Blood urea nitrogen: 28 mg/dL
- Serum creatinine: 1.0 mg/dL
- Urine chloride: 10 mEq/L

## Encounter 5: source 2024-01-15, practice 2026-03-26, ed, Emergency Medicine

Chief complaint: sudden onset of severe headache described as "the worst headache of my life"

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe headache described as "the worst headache of my life"

HISTORY OF PRESENT ILLNESS:
The patient with primary hyperaldosteronism and pancreatic VIPoma, previously managed for chronic headaches and recurrent hypokalemia, presents with sudden onset of severe headache described as the worst headache of her life that started 1 hour ago while she was gardening. She also reports neck stiffness and photophobia.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary Hyperaldosteronism (Conn's Syndrome) (diagnosed 2020-01-15)
- Primary hyperaldosteronism (diagnosed 2020-07-15)
- Pancreatic neuroendocrine tumor localization (diagnosed 2022-01-15)
- Hypokalemia managed with oral replacement (tolerating PO intake) (diagnosed 2023-07-15)
- Primary hyperaldosteronism (Conn syndrome)
- Resistant hypertension
- Pancreatic neuroendocrine tumor (VIPoma)
- Peptic ulcer disease
- Chronic hypokalemia

PAST SURGICAL HISTORY:
- Diagnostic laparoscopy (negative) age 32

MEDICATIONS:
She takes lisinopril.

Home Medications:
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Potassium chloride 40 mEq PO BID
- Pantoprazole 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, ruptured cerebral aneurysm at age 67
- Mother: Type 2 diabetes mellitus, hyperlipidemia
- Paternal aunt: Primary hyperaldosteronism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 glasses of wine monthly. Occupation: Self-employed landscaper and horticulturist.

VITAL SIGNS:
Blood pressure of 158/94 mmHg, heart rate of 102 beats per minute, and temperature of 37.2°C (99°F)

PHYSICAL EXAMINATION:
On examination, she is alert but anxious. Neurological examination reveals nuchal rigidity. There is no focal neurological deficit.

LABORATORY DATA:
Lumbar puncture is performed, and the cerebrospinal fluid analysis reveals xanthochromia.

IMAGING:
A noncontrast CT scan of the head shows a hyperdense area around the brainstem and basal cisterns.
