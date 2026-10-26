# synthetic_hospital patient 2052

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-06-13 (Mon) | outpatient | Rheumatology |
| 1 | 2020-04-15 | 2022-09-02 (Fri) | outpatient | Internal Medicine |
| 2 | 2021-01-15 | 2023-05-29 (Mon) | outpatient | Neurology |
| 3 | 2022-01-15 | 2024-05-31 (Fri) | outpatient | Internal Medicine |
| 4 | 2022-07-15 | 2024-12-09 (Mon) | outpatient | Internal Medicine |
| 5 | 2023-07-15 | 2025-11-28 (Fri) | procedure | Gastroenterology |
| 6 | 2024-01-15 | 2026-06-09 (Tue) | ed | Emergency Medicine |
| 7 | 2024-03-15 | 2026-07-29 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Financial consultant (recently retired due to cognitive decline)",
 "alcohol_use": "No current alcohol use (history of occasional social drinking)",
 "family_history": [
  "Hypertension in first-degree relative",
  "Huntington disease in parent",
  "Coronary artery disease in first-degree relative"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25 mg daily",
   "name": "Hydrochlorothiazide"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Hypertension",
  "Mixed hyperlipidemia",
  "Huntington disease",
  "Psoriatic arthritis",
  "Chronic epigastric pain"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Surveillance colonoscopy at 3-year interval (inappropriate timing)", "Mixed hyperlipidemia", "Polyarteritis nodosa", "Perforated peptic ulcer", "Huntington disease", "Psoriatic Arthritis"]
Comorbidities: ["Cardiovascular disease risk factors", "Obesity", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2022-06-13, outpatient, Rheumatology

Chief complaint: Joint pain and morning stiffness in fingers and toes for the past 6 months with sausage-like swelling

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Joint pain and morning stiffness in fingers and toes for the past 6 months with sausage-like swelling

HISTORY OF PRESENT ILLNESS:
He reports that the pain is worse in the morning and improves with activity throughout the day. He also notes swelling in his fingers and toes, which he describes as looking like "sausages."

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Mixed hyperlipidemia
- Huntington disease
- Psoriatic arthritis
- Chronic epigastric pain

MEDICATIONS:
He takes for which he takes hydrochlorothiazide.

Home Medications:
- Lisinopril 20 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Hypertension in first-degree relative
- Huntington disease in parent
- Coronary artery disease in first-degree relative

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No current alcohol use (history of occasional social drinking). Occupation: Financial consultant (recently retired due to cognitive decline).

REVIEW OF SYSTEMS:
He denies any recent infections or gastrointestinal symptoms.

PHYSICAL EXAMINATION:
On physical examination, you note the presence of silvery scales on his elbows and knees, pitting of the fingernails, and swelling of the distal interphalangeal joints.

LABORATORY DATA:
Laboratory findings are as follows:

- Erythrocyte sedimentation rate (ESR): 40 mm/hr (normal: <20 mm/hr)
- C-reactive protein (CRP): 3.0 mg/dL (normal: <1.0 mg/dL)
- Rheumatoid factor (RF): Negative
- Anti-cyclic citrullinated peptide (anti-CCP) antibodies: Negative
- Complete blood count (CBC): Within normal limits

## Encounter 1: source 2020-04-15, practice 2022-09-02, outpatient, Internal Medicine

Chief complaint: Routine follow-up visit to discuss cardiovascular risk and significance of lipid levels regarding coronary artery disease

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Thompson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up visit to discuss cardiovascular risk and significance of lipid levels regarding coronary artery disease

HISTORY OF PRESENT ILLNESS:
The patient with hypertension, mixed hyperlipidemia, Huntington disease, and chronic epigastric pain returns for routine follow-up following his January Rheumatology evaluation for psoriatic arthritis. He has no complaints and is currently asymptomatic. He is interested in understanding his risk for developing coronary artery disease and asks about the significance of a recent study he read, which reported a correlation coefficient (r) of +0.8 between serum LDL cholesterol levels and the incidence of coronary artery disease in a large population study.

PAST MEDICAL HISTORY:
Active Problem List:
- Psoriatic Arthritis (diagnosed 2020-01-15)
- Hypertension
- Mixed hyperlipidemia
- Huntington disease
- Chronic epigastric pain

MEDICATIONS:
He takes on lisinopril.

Home Medications:
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Notable for coronary artery disease in his father, who had a myocardial infarction at age 50

SOCIAL HISTORY:
He does not smoke and drinks alcohol socially

VITAL SIGNS:
Blood pressure is 128/82 mmHg, heart rate is 72 beats per minute, and BMI is 27 kg/m^2

PHYSICAL EXAMINATION:
Unremarkable

LABORATORY DATA:
Laboratory results are as follows:

- Total cholesterol: 210 mg/dL
- HDL cholesterol: 40 mg/dL
- LDL cholesterol: 130 mg/dL
- Triglycerides: 200 mg/dL
- Fasting glucose: 98 mg/dL

## Encounter 2: source 2021-01-15, practice 2023-05-29, outpatient, Neurology

Chief complaint: Progressive changes in mood, thinking, and movement with cognitive decline necessitating early retirement

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. David Kim, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive changes in mood, thinking, and movement with cognitive decline necessitating early retirement

HISTORY OF PRESENT ILLNESS:
The patient, with established histories of hypertension, mixed hyperlipidemia, and psoriatic arthritis previously managed in internal medicine and rheumatology, presents for neurologic evaluation. Per his wife, over the past several years she has noticed that he has become more forgetful, often misplacing items or repeating the same questions. Recently, he has struggled to keep track of bills and could no longer manage his work as a financial consultant, leading to his early retirement. She also reports that he has become irritable and at times displays inappropriate laughter or anger. In addition, he has developed frequent, unpredictable jerking movements of his face, arms, and legs that worsen with stress and interfere with daily activities.

PAST MEDICAL HISTORY:
Active Problem List:
- Psoriatic Arthritis (diagnosed 2020-01-15)
- Mixed hyperlipidemia (diagnosed 2020-04-15)
- Hypertension
- Huntington disease
- Chronic epigastric pain

MEDICATIONS:
The patient takes medication for mild hypertension but no psychiatric drugs.

Home Medications:
- Lisinopril 20 mg daily
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His father reportedly had similar symptoms in his 50s, though no formal diagnosis was made.

SOCIAL HISTORY:
He does not use alcohol, tobacco, or illicit drugs.

REVIEW OF SYSTEMS:
The patient denies headaches, vision changes, or episodes of loss of consciousness.

PHYSICAL EXAMINATION:
On physical exam, he is oriented to person but not to time or place. His speech is fluent but tangential, and he has difficulty recalling words and following complex instructions. Neurologic exam shows irregular, nonrhythmic, involuntary movements involving the upper and lower extremities as well as facial grimacing. Reflexes and strength are otherwise intact. On mental status testing, he is unable to recall any of three objects after five minutes and struggles to interpret common proverbs. Cranial nerve exam is normal. Gait is wide-based and irregular due to sudden, unpredictable jerks.

IMAGING:
MRI of the brain is obtained.

## Encounter 3: source 2022-01-15, practice 2024-05-31, outpatient, Internal Medicine

Chief complaint: Routine annual health maintenance examination with no acute complaints

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Aisha Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine annual health maintenance examination with no acute complaints

PAST MEDICAL HISTORY:
Active Problem List:
- Psoriatic Arthritis (diagnosed 2020-01-15)
- Mixed hyperlipidemia (diagnosed 2020-04-15)
- Huntington disease (diagnosed 2021-01-15)
- Hypertension
- Chronic epigastric pain

His medical history is significant for hypertension, which is well-controlled on lisinopril.

MEDICATIONS:
He takes lisinopril.

Home Medications:
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His father had a myocardial infarction at the age of 50.

SOCIAL HISTORY:
He does not smoke and drinks alcohol socially.

VITAL SIGNS:
His blood pressure is 130/85 mmHg, heart rate is 78 beats per minute, and BMI is 29 kg/m^2

PHYSICAL EXAMINATION:
His cardiovascular and respiratory examinations are unremarkable.

LABORATORY DATA:
Total cholesterol: 220 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 160 mg/dL
- Triglycerides: 150 mg/dL
- Fasting plasma glucose: 98 mg/dL

PLAN:
The patient is interested in participating in a study that is comparing the incidence of coronary artery disease (CAD) in individuals with and without a history of hypertension. Based on the cohort study design,

## Encounter 4: source 2022-07-15, practice 2024-12-09, outpatient, Internal Medicine

Chief complaint: Routine health examination with interest in participating in antihypertensive clinical trial

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Robert Johnson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health examination with interest in participating in antihypertensive clinical trial

HISTORY OF PRESENT ILLNESS:
The patient returns for routine health maintenance with established hypertension, mixed hyperlipidemia, Huntington disease, psoriatic arthritis, and chronic epigastric pain. He is interested in participating in a clinical trial he saw advertised in a local newspaper for a new antihypertensive medication. The trial is seeking volunteers with a family history of hypertension.

PAST MEDICAL HISTORY:
Active Problem List:
- Psoriatic Arthritis (diagnosed 2020-01-15)
- Mixed hyperlipidemia (diagnosed 2020-04-15)
- Huntington disease (diagnosed 2021-01-15)
- Hypertension
- Chronic epigastric pain

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is notable for hypertension in his father.

SOCIAL HISTORY:
He is an office worker and leads a sedentary lifestyle.
He does not smoke and drinks alcohol socially.

VITAL SIGNS:
On examination, his blood pressure is 135/85 mmHg, heart rate is 78 bpm, and BMI is 29 kg/m^2.

LABORATORY DATA:
Laboratory tests reveal the following:

- Fasting glucose: 100 mg/dL
- Total cholesterol: 210 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 145 mg/dL
- Triglycerides: 150 mg/dL

## Encounter 5: source 2023-07-15, practice 2025-11-28, procedure, Gastroenterology

Chief complaint: Scheduled surveillance colonoscopy for routine health maintenance

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Rajesh Gupta, MD
Department: Gastroenterology
Visit Type: procedure

CHIEF COMPLAINT:
Scheduled surveillance colonoscopy for routine health maintenance

HISTORY OF PRESENT ILLNESS:
This 48-year-old male with Huntington disease, psoriatic arthritis, and hypertension returns for scheduled surveillance colonoscopy. He has no complaints and is currently asymptomatic.

PAST MEDICAL HISTORY:
Active Problem List:
- Psoriatic Arthritis (diagnosed 2020-01-15)
- Mixed hyperlipidemia (diagnosed 2020-04-15)
- Huntington disease (diagnosed 2021-01-15)
- Hypertension
- Chronic epigastric pain

His medical history is significant for hypertension, controlled with lisinopril.

PAST SURGICAL HISTORY:
He has no history of gastrointestinal symptoms or surgeries.

MEDICATIONS:
He takes lisinopril.

Home Medications:
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His father was diagnosed with colon cancer at the age of 56.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No current alcohol use (history of occasional social drinking). Occupation: Financial consultant (recently retired due to cognitive decline).

VITAL SIGNS:
His blood pressure is 132/84 mmHg, and his body mass index (BMI) is 27 kg/m^2.

PHYSICAL EXAMINATION:
The patient's physical examination is unremarkable.

LABORATORY DATA:
His most recent blood work from six months ago showed normal metabolic panel and complete blood count.

## Encounter 6: source 2024-01-15, practice 2026-06-09, ed, Emergency Medicine

Chief complaint: Two-month history of fevers, weight loss, and malaise with new severe left thigh pain and foot drop

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Lisa Martinez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-month history of fevers, weight loss, and malaise with new severe left thigh pain and foot drop

HISTORY OF PRESENT ILLNESS:
This 49-year-old male with longitudinal history significant for Huntington disease diagnosed in 2021 and psoriatic arthritis diagnosed in 2020 presents to the emergency department. He reports new onset of severe, sharp pain in his left thigh and foot drop that started a week ago. He also mentions a painful, raised rash on his lower legs.

PAST MEDICAL HISTORY:
Active Problem List:
- Psoriatic Arthritis (diagnosed 2020-01-15)
- Mixed hyperlipidemia (diagnosed 2020-04-15)
- Huntington disease (diagnosed 2021-01-15)
- Surveillance colonoscopy at 3-year interval (inappropriate timing) (diagnosed 2023-07-15)
- Hypertension
- Chronic epigastric pain

MEDICATIONS:
Lisinopril and hydrochlorothiazide

ALLERGIES:
NKDA

FAMILY HISTORY:
- Hypertension in first-degree relative
- Huntington disease in parent
- Coronary artery disease in first-degree relative

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No current alcohol use (history of occasional social drinking). Occupation: Financial consultant (recently retired due to cognitive decline).

VITAL SIGNS:
Blood pressure is 150/90 mmHg

PHYSICAL EXAMINATION:
Palpable purpuric rash on both lower extremities. Neurological examination reveals weakness in dorsiflexion and eversion of the left foot.

LABORATORY DATA:
- Hemoglobin: 13.2 g/dL
- White blood cell count: 11,000 /µL
- Erythrocyte sedimentation rate (ESR): 80 mm/hr
- Creatinine: 1.8 mg/dL (baseline 1.0 mg/dL)
- Urinalysis: Proteinuria and red blood cell casts

## Encounter 7: source 2024-03-15, practice 2026-07-29, ed, Emergency Medicine

Chief complaint: Sudden-onset severe epigastric abdominal pain radiating to back for 2 hours after large meal

ENCOUNTER NOTE
Date: 2024-03-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset severe epigastric abdominal pain radiating to back for 2 hours after large meal

HISTORY OF PRESENT ILLNESS:
The patient is a 49-year-old male with chronic epigastric pain who presents two months after Polyarteritis nodosa. He describes the pain as sharp and constant, located in the epigastric region, and radiating to the back. He reports that the pain began shortly after eating a large meal and has not improved with over-the-counter antacids.

PAST MEDICAL HISTORY:
Active Problem List:
- Psoriatic Arthritis (diagnosed 2020-01-15)
- Mixed hyperlipidemia (diagnosed 2020-04-15)
- Huntington disease (diagnosed 2021-01-15)
- Surveillance colonoscopy at 3-year interval (inappropriate timing) (diagnosed 2023-07-15)
- Polyarteritis nodosa (diagnosed 2024-01-15)
- Hypertension
- Chronic epigastric pain

His past medical history is significant for chronic epigastric pain.

MEDICATIONS:
For which.

Home Medications:
- Lisinopril 20 mg daily
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Hypertension in first-degree relative
- Huntington disease in parent
- Coronary artery disease in first-degree relative

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No current alcohol use (history of occasional social drinking). Occupation: Financial consultant (recently retired due to cognitive decline).

VITAL SIGNS:
On examination, his temperature is 38.2°C (100.8°F), heart rate is 110 beats per minute, blood pressure is 130/85 mmHg, and respiratory rate is 22 breaths per minute.

PHYSICAL EXAMINATION:
Abdominal examination reveals diffuse tenderness, rebound tenderness, and guarding. Bowel sounds are hypoactive.

LABORATORY DATA:
Laboratory tests are as follows:

- White blood cell count: 14,000/mm³
- Hemoglobin: 13.5 g/dL
- Platelet count: 250,000/mm³
- Serum amylase: 90 U/L (normal range: 30-110 U/L)
- Serum lipase: 45 U/L (normal range: 0-160 U/L)

IMAGING:
An upright chest x-ray shows subdiaphragmatic free air.
