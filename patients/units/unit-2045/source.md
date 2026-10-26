# synthetic_hospital patient 2045

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-01-17 (Fri) | outpatient | Internal Medicine |
| 1 | 2020-05-15 | 2020-05-04 (Mon) | outpatient | Dermatology |
| 2 | 2021-04-15 | 2021-04-13 (Tue) | outpatient | Internal Medicine |
| 3 | 2021-10-15 | 2021-10-01 (Fri) | outpatient | Internal Medicine |
| 4 | 2022-10-15 | 2022-10-07 (Fri) | outpatient | Endocrinology |
| 5 | 2023-10-15 | 2023-10-18 (Wed) | outpatient | Internal Medicine |
| 6 | 2024-10-15 | 2024-10-07 (Mon) | outpatient | Endocrinology |
| 7 | 2025-10-15 | 2025-10-17 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Administrative office manager",
 "alcohol_use": "Social (1-2 drinks on weekends)",
 "family_history": [
  "Father: Lung cancer, deceased at age 60",
  "Mother: Hypertension, hyperlipidemia"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 30 pack-year history (approximately 1 pack per day since age 15)",
 "home_medications": [
  {
   "dose": "10 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "20 mg daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "10 mg nightly",
   "name": "Olanzapine"
  },
  {
   "dose": "200-400 mg PRN for knee pain",
   "name": "Ibuprofen"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia",
  "Metabolic syndrome",
  "Schizophrenia (stable)",
  "Chronic osteoarthritis of the knee",
  "Tobacco use disorder"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Bleeding duodenal ulcer", "Plaque psoriasis", "Essential hypertension", "Tobacco use disorder", "Antipsychotic-induced type 2 diabetes mellitus", "ACTH-producing pituitary adenoma"]
Comorbidities: ["Family history of malignant neoplasm of lung", "Obesity", "Hyperlipidemia", "Essential hypertension", "Metabolic syndrome"]

## Encounter 0: source 2020-01-15, practice 2020-01-17, outpatient, Internal Medicine

Chief complaint: presents to the clinic for a routine follow-up

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the clinic for a routine follow-up

HISTORY OF PRESENT ILLNESS:
He reports no new symptoms and is adherent to his current medication regimen
He mentions that a pharmaceutical representative recently visited his office and left several samples of a new brand-name antihypertensive drug, along with some promotional materials. The representative suggested that this new drug has superior efficacy compared to other antihypertensives. The patient inquires whether he should switch to the new medication.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- Metabolic syndrome
- Schizophrenia (stable)
- Chronic osteoarthritis of the knee
- Tobacco use disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes lisinopril and atorvastatin.

Home Medications:
- Olanzapine 10 mg nightly
- Ibuprofen 200-400 mg PRN for knee pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, deceased at age 60
- Mother: Hypertension, hyperlipidemia

SOCIAL HISTORY:
Smoking: Current smoker, 30 pack-year history (approximately 1 pack per day since age 15). Alcohol: Social (1-2 drinks on weekends). Occupation: Administrative office manager.

PHYSICAL EXAMINATION:
On physical examination, his blood pressure is 132/84 mmHg, and his heart rate is 78 bpm.

LABORATORY DATA:
Laboratory values are as follows:

- Serum sodium: 140 mEq/L
- Serum potassium: 4.2 mEq/L
- Serum creatinine: 1.0 mg/dL
- Total cholesterol: 190 mg/dL
- LDL cholesterol: 100 mg/dL
- HDL cholesterol: 45 mg/dL
- Triglycerides: 150 mg/dL

## Encounter 1: source 2020-05-15, practice 2020-05-04, outpatient, Dermatology

Chief complaint: rash that has gradually worsened over the past 4 months

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Michael Rodriguez, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Rash that has gradually worsened over the past 4 months

HISTORY OF PRESENT ILLNESS:
The patient, a 45-year-old male with hypertension, presents to Dermatology four months after his last Internal Medicine visit with a worsening rash. It initially appeared on his elbows and knees and later spread to his scalp and lower back. The lesions are mildly itchy but not painful. He denies joint pain, fever, weight loss, or new medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Hypertension
- Hyperlipidemia
- Metabolic syndrome
- Schizophrenia (stable)
- Chronic osteoarthritis of the knee
- Tobacco use disorder

He has a history of metabolic syndrome.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes lisinopril and atorvastatin.

Home Medications:
- Olanzapine 10 mg nightly
- Ibuprofen 200-400 mg PRN for knee pain

ALLERGIES:
NKDA

FAMILY HISTORY:
His father had a similar rash in middle age.

SOCIAL HISTORY:
Smoking: Current smoker, 30 pack-year history (approximately 1 pack per day since age 15). Alcohol: Social (1-2 drinks on weekends). Occupation: Administrative office manager.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
Physical examination reveals well-circumscribed erythematous plaques with overlying silvery-white scale on the extensor surfaces, scalp, and lower back. Nail pitting is observed. No mucosal or systemic findings are present.

## Encounter 2: source 2021-04-15, practice 2021-04-13, outpatient, Internal Medicine

Chief complaint: presents to the clinic for a routine check-up

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the clinic for a routine check-up

HISTORY OF PRESENT ILLNESS:
The patient returns to the internal medicine clinic for routine follow-up of hypertension and hyperlipidemia, with prior dermatologic evaluation for plaque psoriasis. He has no complaints but is concerned about his risk of developing lung cancer. The patient inquires about a recent study he read that showed a strong association between cigarette smoking and lung cancer. He is particularly interested in understanding the concept of dose-response relationship mentioned in the study.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Plaque psoriasis (diagnosed 2020-05-15)
- Hypertension
- Hyperlipidemia
- Metabolic syndrome
- Schizophrenia (stable)
- Chronic osteoarthritis of the knee
- Tobacco use disorder

His medical history is significant for hypertension, and hyperlipidemia.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes controlled with lisinopril, managed with atorvastatin.

Home Medications:
- Olanzapine 10 mg nightly
- Ibuprofen 200-400 mg PRN for knee pain

ALLERGIES:
NKDA

FAMILY HISTORY:
As his father was diagnosed with lung cancer at the age of 55.

SOCIAL HISTORY:
With a 20-year history of smoking one pack of cigarettes per day

VITAL SIGNS:
On physical examination, his blood pressure is 130/85 mmHg, heart rate is 78 beats per minute, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
His lung examination is unremarkable. He has no lymphadenopathy or clubbing.

LABORATORY DATA:
Laboratory values are as follows:

- Complete Blood Count (CBC): Within normal limits
- Fasting Lipid Panel:
- Total Cholesterol: 190 mg/dL
- HDL Cholesterol: 45 mg/dL
- LDL Cholesterol: 120 mg/dL
- Triglycerides: 250 mg/dL

## Encounter 3: source 2021-10-15, practice 2021-10-01, outpatient, Internal Medicine

Chief complaint: seeking help to quit smoking

ENCOUNTER NOTE
Date: 2021-10-15
Provider: Dr. James Park, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Seeking help to quit smoking

HISTORY OF PRESENT ILLNESS:
The patient is a 46-year-old male with hypertension, hyperlipidemia, metabolic syndrome, and stable schizophrenia who returns to the internal medicine clinic for continued assistance with smoking cessation, previously evaluated in April 2021. He reports smoking one pack of cigarettes daily for the past 20 years and has attempted to quit multiple times, including using nicotine patches and gum, but has always relapsed due to cravings and irritability. He states he is now "serious about quitting" because he recently became a grandfather and wants to improve his health. He expresses interest in a medication that can help reduce cravings and withdrawal symptoms without causing weight gain. He also does not want any problem with sleeping as he heard there is a smoking cessation drug that causes bad dreams.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Plaque psoriasis (diagnosed 2020-05-15)
- Tobacco use disorder (diagnosed 2021-04-15)
- Hypertension
- Hyperlipidemia
- Metabolic syndrome
- Schizophrenia (stable)
- Chronic osteoarthritis of the knee

He has a history of hypertension and hyperlipidemia, both controlled with medication. He denies current depression, past psychiatric illness, seizures, or eating disorders.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Atorvastatin 20 mg daily
- Olanzapine 10 mg nightly
- Ibuprofen 200-400 mg PRN for knee pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, deceased at age 60
- Mother: Hypertension, hyperlipidemia

SOCIAL HISTORY:
He drinks alcohol socially.

VITAL SIGNS:
His BMI is 28 kg/m²

## Encounter 4: source 2022-10-15, practice 2022-10-07, outpatient, Endocrinology

Chief complaint: presents to the clinic for a routine follow-up

ENCOUNTER NOTE
Date: 2022-10-15
Provider: Dr. Emily Thompson, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the clinic for a routine follow-up

HISTORY OF PRESENT ILLNESS:
The patient presents to endocrinology for follow-up with metabolic syndrome and olanzapine therapy. He reports increased thirst, frequent urination, and a 10-pound weight gain over the last three months. He denies any changes in diet or activity level.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Plaque psoriasis (diagnosed 2020-05-15)
- Tobacco use disorder (diagnosed 2021-04-15)
- Hypertension
- Hyperlipidemia
- Metabolic syndrome
- Schizophrenia (stable)
- Chronic osteoarthritis of the knee

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He has been stable on olanzapine for the past two years

Home Medications:
- Lisinopril 10 mg daily
- Atorvastatin 20 mg daily
- Ibuprofen 200-400 mg PRN for knee pain

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is significant for type 2 diabetes mellitus in his mother.

SOCIAL HISTORY:
Smoking: Current smoker, 30 pack-year history (approximately 1 pack per day since age 15). Alcohol: Social (1-2 drinks on weekends). Occupation: Administrative office manager.

VITAL SIGNS:
His blood pressure is 135/85 mmHg.

PHYSICAL EXAMINATION:
On physical examination, his BMI is 32 kg/m^2, and he appears cushingoid.

LABORATORY DATA:
Laboratory tests reveal the following:

- Fasting plasma glucose: 126 mg/dL
- Hemoglobin A1c: 7.2%
- Total cholesterol: 220 mg/dL
- HDL cholesterol: 35 mg/dL
- Triglycerides: 250 mg/dL
- LDL cholesterol: 145 mg/dL

## Encounter 5: source 2023-10-15, practice 2023-10-18, outpatient, Internal Medicine

Chief complaint: presents to the clinic for a routine check-up

ENCOUNTER NOTE
Date: 2023-10-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the clinic for a routine check-up

HISTORY OF PRESENT ILLNESS:
The patient returns for routine follow-up of his hypertension, hyperlipidemia, and tobacco use disorder, previously counseled for smoking cessation and managed for antipsychotic-induced diabetes. He has no complaints and is currently asymptomatic. The patient inquires about his risk for developing certain diseases given his smoking history. He is particularly concerned about lung cancer, as his father died from it at the age of 60. He asks about the best way to study the relationship between smoking and lung cancer in the population.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Plaque psoriasis (diagnosed 2020-05-15)
- Tobacco use disorder (diagnosed 2021-04-15)
- Antipsychotic-induced type 2 diabetes mellitus (diagnosed 2022-10-15)
- Hypertension
- Hyperlipidemia
- Metabolic syndrome
- Schizophrenia (stable)
- Chronic osteoarthritis of the knee

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
His medications include lisinopril and hydrochlorothiazide.

Home Medications:
- Atorvastatin 20 mg daily
- Olanzapine 10 mg nightly
- Ibuprofen 200-400 mg PRN for knee pain

ALLERGIES:
NKDA

FAMILY HISTORY:
His father died from it at the age of 60

SOCIAL HISTORY:
30-pack-year smoking history

VITAL SIGNS:
His blood pressure is 135/85 mmHg, heart rate is 78 bpm, and respiratory rate is 16 breaths per minute

PHYSICAL EXAMINATION:
His cardiovascular and respiratory examinations are unremarkable.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin: 14.5 g/dL
- White blood cell count: 7,000/mm^3
- Platelets: 250,000/mm^3
- Fasting glucose: 98 mg/dL
- Serum creatinine: 1.0 mg/dL
- Total cholesterol: 210 mg/dL
- LDL cholesterol: 140 mg/dL
- HDL cholesterol: 45 mg/dL
- Triglycerides: 150 mg/dL

## Encounter 6: source 2024-10-15, practice 2024-10-07, outpatient, Endocrinology

Chief complaint: progressive weight gain and fatigue over the past several months

ENCOUNTER NOTE
Date: 2024-10-15
Provider: Dr. Aisha Patel, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive weight gain and fatigue over the past several months

HISTORY OF PRESENT ILLNESS:
The patient returns to Endocrinology with progressive weight gain and fatigue, having been managed here for antipsychotic-induced diabetes. He notes increasing abdominal girth and facial fullness despite no changes in his diet or exercise routine. His wife reports that he has been more irritable lately and has developed purplish stretch marks on his abdomen.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Plaque psoriasis (diagnosed 2020-05-15)
- Tobacco use disorder (diagnosed 2021-04-15)
- Antipsychotic-induced type 2 diabetes mellitus (diagnosed 2022-10-15)
- Hypertension
- Hyperlipidemia
- Metabolic syndrome
- Schizophrenia (stable)
- Chronic osteoarthritis of the knee

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Atorvastatin 20 mg daily
- Olanzapine 10 mg nightly
- Ibuprofen 200-400 mg PRN for knee pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, deceased at age 60
- Mother: Hypertension, hyperlipidemia

SOCIAL HISTORY:
Smoking: Current smoker, 30 pack-year history (approximately 1 pack per day since age 15). Alcohol: Social (1-2 drinks on weekends). Occupation: Administrative office manager.

REVIEW OF SYSTEMS:
He denies headaches, visual changes, or nipple discharge.

VITAL SIGNS:
His blood pressure is 156/92 mm Hg

PHYSICAL EXAMINATION:
There is proximal muscle weakness in the upper and lower extremities. His face is round and flushed, and his abdomen is obese with wide violaceous striae. On palpation, there is point tenderness over the lumbar spine.

LABORATORY DATA:
Serum laboratory testing is ordered.

IMAGING:
MRI of the brain reveals a pituitary mass, and spinal x-ray shows an L1 vertebral compression fracture.

## Encounter 7: source 2025-10-15, practice 2025-10-17, ed, Emergency Medicine

Chief complaint: black, tarry stools and a dull, gnawing epigastric pain

ENCOUNTER NOTE
Date: 2025-10-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Black, tarry stools and a dull, gnawing epigastric pain

HISTORY OF PRESENT ILLNESS:
This 50-year-old male with hypertension, hyperlipidemia, metabolic syndrome, schizophrenia, and ACTH-producing pituitary adenoma presents with a 2-day history of black, tarry stools and a dull, gnawing epigastric pain that wakes him up at night. He reports that the pain temporarily improves after eating but returns with greater intensity.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Plaque psoriasis (diagnosed 2020-05-15)
- Tobacco use disorder (diagnosed 2021-04-15)
- Antipsychotic-induced type 2 diabetes mellitus (diagnosed 2022-10-15)
- ACTH-producing pituitary adenoma (diagnosed 2024-10-15)
- Hypertension
- Hyperlipidemia
- Metabolic syndrome
- Schizophrenia (stable)
- Chronic osteoarthritis of the knee

His past medical history includes hypertension and dyslipidemia.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes for which he takes lisinopril and atorvastatin, respectively. He also mentions using over-the-counter ibuprofen occasionally for chronic knee pain.

Home Medications:
- Olanzapine 10 mg nightly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, deceased at age 60
- Mother: Hypertension, hyperlipidemia

SOCIAL HISTORY:
Smoking: Current smoker, 30 pack-year history (approximately 1 pack per day since age 15). Alcohol: Social (1-2 drinks on weekends). Occupation: Administrative office manager.

VITAL SIGNS:
Blood pressure is 130/85 mmHg, heart rate is 98 bpm

PHYSICAL EXAMINATION:
He appears pale. Abdominal examination reveals mild epigastric tenderness to palpation without rebound or guarding.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin: 10.2 g/dL (Normal: 13.8-17.2 g/dL)
- Hematocrit: 30.5% (Normal: 40.7-50.3%)
- Mean corpuscular volume (MCV): 86 fL (Normal: 80-96 fL)
- Blood urea nitrogen (BUN): 29 mg/dL (Normal: 7-20 mg/dL)
- Serum creatinine: 1.0 mg/dL (Normal: 0.6-1.2 mg/dL)
- Liver enzymes: Within normal limits

Stool guaiac test is positive.
