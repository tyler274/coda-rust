# synthetic_hospital patient 2002

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 46 at first encounter, sex M, Non-Hispanic White, Medicare (disability).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2019-10-21 (Mon) | outpatient | Psychiatry |
| 1 | 2020-07-15 | 2020-04-06 (Mon) | outpatient | Internal Medicine |
| 2 | 2021-03-15 | 2020-12-10 (Thu) | outpatient | Pulmonary & Critical Care |
| 3 | 2022-01-15 | 2021-10-18 (Mon) | outpatient | Endocrinology |
| 4 | 2023-01-15 | 2022-10-24 (Mon) | outpatient | Neurology |
| 5 | 2024-01-15 | 2023-10-16 (Mon) | ed | Emergency Medicine |
| 6 | 2025-01-15 | 2024-10-23 (Wed) | inpatient | Internal Medicine |
| 7 | 2026-01-15 | 2025-10-15 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare (disability)",
 "occupation": "Retired construction worker (disability)",
 "alcohol_use": "Denies alcohol use",
 "family_history": [
  "Father: Type 2 diabetes mellitus, hypertension",
  "Mother: Schizophrenia",
  "Brother: Hepatitis B carrier"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 2 years ago, 10 pack-year history",
 "home_medications": [
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  },
  {
   "dose": "2 mg daily (currently noncompliant)",
   "name": "Risperidone"
  },
  {
   "dose": "25 mg at bedtime as needed for sleep",
   "name": "Diphenhydramine"
  },
  {
   "dose": "500 mg as needed for dyspepsia",
   "name": "Calcium carbonate"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Hypertension",
  "Schizophrenia",
  "Type 2 Diabetes Mellitus",
  "Chronic Hepatitis B Infection",
  "Peptic Ulcer Disease",
  "Inguinal hernia, reducible"
 ],
 "age_at_first_encounter": 46
}
```

Primary diagnoses: ["Primary polydipsia", "Schizophrenia", "Cervical radiculopathy due to C6-C7 disc herniation", "Ectopic ACTH production", "Severe frostbite with tissue necrosis and microvascular thrombosis", "Polyarteritis nodosa", "Obstructive sleep apnea", "Iron deficiency anemia"]
Comorbidities: ["Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2019-10-21, outpatient, Psychiatry

Chief complaint: Scheduled psychiatric follow-up and preoperative consultation for elective inguinal hernia repair.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Scheduled psychiatric follow-up and preoperative consultation for elective inguinal hernia repair.

HISTORY OF PRESENT ILLNESS:
He has been noncompliant with his antipsychotic medication for the past month. His sister reports that he has been experiencing auditory hallucinations and believes that the television is sending him secret messages. Today, he is scheduled to discuss elective hernia repair surgery due to a reducible inguinal hernia that causes discomfort during physical activity.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Schizophrenia
- Type 2 Diabetes Mellitus
- Chronic Hepatitis B Infection
- Peptic Ulcer Disease
- Inguinal hernia, reducible

The patient has a history of type 2 diabetes mellitus and hypertension, which are well-controlled with metformin and lisinopril, respectively.

MEDICATIONS:
He takes metformin and lisinopril.

Home Medications:
- Risperidone 2 mg daily (currently noncompliant)
- Diphenhydramine 25 mg at bedtime as needed for sleep
- Calcium carbonate 500 mg as needed for dyspepsia

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Schizophrenia
- Brother: Hepatitis B carrier

SOCIAL HISTORY:
Smoking: Former smoker, quit 2 years ago, 10 pack-year history. Alcohol: Denies alcohol use. Occupation: Retired construction worker (disability).

VITAL SIGNS:
His vital signs are stable

PHYSICAL EXAMINATION:
The patient appears disheveled, with poor eye contact and intermittent mumbling to himself. On examination, the patient is alert but appears to be responding to internal stimuli. Physical examination is unremarkable except for the known inguinal hernia.

LABORATORY DATA:
Hemoglobin A1c: 6.8%
- Fasting blood glucose: 138 mg/dL
- Serum creatinine: 1.0 mg/dL
- Blood urea nitrogen: 14 mg/dL
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L

## Encounter 1: source 2020-07-15, practice 2020-04-06, outpatient, Internal Medicine

Chief complaint: Progressive fatigue and exertional dyspnea over the past 3 months.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue and exertional dyspnea over the past 3 months.

HISTORY OF PRESENT ILLNESS:
The patient, evaluated six months ago for psychiatric preoperative hernia consultation, presents to the clinic with a 3-month history of progressive fatigue and exertional dyspnea. He denies any chest pain, palpitations, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Hepatitis B Infection
- Peptic Ulcer Disease
- Inguinal hernia, reducible

He has a past medical history of hypertension and peptic ulcer disease.

MEDICATIONS:
His medications include lisinopril and occasional over-the-counter antacids.

Home Medications:
- Metformin 1000 mg twice daily
- Risperidone 2 mg daily (currently noncompliant)
- Diphenhydramine 25 mg at bedtime as needed for sleep
- Calcium carbonate 500 mg as needed for dyspepsia

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Schizophrenia
- Brother: Hepatitis B carrier

SOCIAL HISTORY:
He has a 10-year history of smoking one pack of cigarettes per day but quit 2 years ago.

PHYSICAL EXAMINATION:
On physical examination, his skin and conjunctivae are pale. Stool guaiac test is positive.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 10.2 g/dL (Normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 72 fL (Normal: 80-96 fL)
- Serum iron: 30 µg/dL (Normal: 60-170 µg/dL)
- Total iron-binding capacity (TIBC): 450 µg/dL (Normal: 250-450 µg/dL)
- Ferritin: 15 ng/mL (Normal: 30-300 ng/mL)
- Transferrin saturation: 7% (Normal: 20-50%)

## Encounter 2: source 2021-03-15, practice 2020-12-10, outpatient, Pulmonary & Critical Care

Chief complaint: Excessive daytime sleepiness, near-miss motor vehicle collision, and witnessed apneic episodes during sleep.

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Priya Patel, MD
Department: Pulmonary & Critical Care
Visit Type: outpatient

CHIEF COMPLAINT:
Excessive daytime sleepiness, near-miss motor vehicle collision, and witnessed apneic episodes during sleep.

HISTORY OF PRESENT ILLNESS:
The patient with schizophrenia and iron deficiency anemia evaluated in July 2020 presents with excessive daytime sleepiness. He reports that he has nearly fallen asleep at the wheel on multiple occasions, including a recent minor motor vehicle collision. His wife reports loud snoring and has witnessed him stop breathing at night.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia (diagnosed 2020-01-15)
- Iron deficiency anemia (diagnosed 2020-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Hepatitis B Infection
- Peptic Ulcer Disease
- Inguinal hernia, reducible

MEDICATIONS:
He admits to occasionally taking diphenhydramine to help with sleep.

Home Medications:
- Lisinopril 20 mg daily
- Metformin 1000 mg twice daily
- Risperidone 2 mg daily (currently noncompliant)
- Calcium carbonate 500 mg as needed for dyspepsia

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Schizophrenia
- Brother: Hepatitis B carrier

SOCIAL HISTORY:
He denies alcohol use

VITAL SIGNS:
His BMI is 32 kg/m^2. His blood pressure is 145/90 mmHg.

PHYSICAL EXAMINATION:
His neck circumference is 17 inches. Physical examination reveals crowded oropharynx.

OTHER STUDIES:
The patient's Epworth Sleepiness Scale score is 16 (out of 24), indicating severe daytime sleepiness.

## Encounter 3: source 2022-01-15, practice 2021-10-18, outpatient, Endocrinology

Chief complaint: Progressive weight gain, worsening hypertension, proximal muscle weakness, and facial plethora developing over 4 months.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. David Kim, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive weight gain, worsening hypertension, proximal muscle weakness, and facial plethora developing over 4 months.

HISTORY OF PRESENT ILLNESS:
The patient with hypertension, schizophrenia, and diabetes, previously evaluated for sleep apnea and anemia, presents with symptoms progressing over the past 4 months. He notes increased fat around his face and neck, easy bruising, poor wound healing, intermittent diplopia, and difficulty rising from chairs.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia (diagnosed 2020-01-15)
- Iron deficiency anemia (diagnosed 2020-07-15)
- Obstructive sleep apnea (diagnosed 2021-03-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Hepatitis B Infection
- Peptic Ulcer Disease
- Inguinal hernia, reducible

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Metformin 1000 mg twice daily
- Risperidone 2 mg daily (currently noncompliant)
- Diphenhydramine 25 mg at bedtime as needed for sleep
- Calcium carbonate 500 mg as needed for dyspepsia

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Schizophrenia
- Brother: Hepatitis B carrier

SOCIAL HISTORY:
Smoking: Former smoker, quit 2 years ago, 10 pack-year history. Alcohol: Denies alcohol use. Occupation: Retired construction worker (disability).

PHYSICAL EXAMINATION:
On neurologic exam, his deep tendon reflexes are slightly diminished but improve after brief exercise. He is able to maintain upward gaze for 60 seconds without ptosis.

LABORATORY DATA:
Labs show elevated serum cortisol and elevated ACTH levels.

OTHER STUDIES:
A low-dose dexamethasone test fails to suppress cortisol. High-dose dexamethasone is then administered.

## Encounter 4: source 2023-01-15, practice 2022-10-24, outpatient, Neurology

Chief complaint: Six-week history of neck pain with intermittent tingling in the left arm exacerbated by movement.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Jennifer Okafor, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-week history of neck pain with intermittent tingling in the left arm exacerbated by movement.

HISTORY OF PRESENT ILLNESS:
The patient with schizophrenia and hernia history presents with neck pain. He reports the pain is exacerbated by turning his head to the left and by lifting heavy objects at work.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia (diagnosed 2020-01-15)
- Iron deficiency anemia (diagnosed 2020-07-15)
- Obstructive sleep apnea (diagnosed 2021-03-15)
- Ectopic ACTH production (diagnosed 2022-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Hepatitis B Infection
- Peptic Ulcer Disease
- Inguinal hernia, reducible

His medical history is significant for hypertension, controlled with lisinopril.

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Metformin 1000 mg twice daily
- Risperidone 2 mg daily (currently noncompliant)
- Diphenhydramine 25 mg at bedtime as needed for sleep
- Calcium carbonate 500 mg as needed for dyspepsia

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Schizophrenia
- Brother: Hepatitis B carrier

SOCIAL HISTORY:
Smoking: Former smoker, quit 2 years ago, 10 pack-year history. Alcohol: Denies alcohol use. Occupation: Retired construction worker (disability).

PHYSICAL EXAMINATION:
On physical examination, there is tenderness to palpation over the left paracervical musculature. The patient exhibits weakness during left arm biceps and wrist extensor strength testing. The biceps reflex is diminished on the left side. Sensation is decreased over the left thumb and index finger.

LABORATORY DATA:
Laboratory values:
- Complete blood count: within normal limits
- Erythrocyte sedimentation rate: within normal limits
- C-reactive protein: within normal limits

IMAGING:
An MRI of the cervical spine reveals a herniated disc at the C6-C7 level with nerve root impingement.

## Encounter 5: source 2024-01-15, practice 2023-10-16, ed, Emergency Medicine

Chief complaint: Worsening fatigue, headaches, dizziness, and compulsive water consumption resulting in polydipsia.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Robert Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening fatigue, headaches, dizziness, and compulsive water consumption resulting in polydipsia.

HISTORY OF PRESENT ILLNESS:
This 50-year-old male with schizophrenia and ectopic ACTH production presents with increased water intake. She notes that he has been more withdrawn recently, spending most of his time at home, and has been drinking large amounts of water throughout the day. He carries a water bottle with him everywhere and wakes up multiple times at night to drink. He has not seen his psychiatrist in several months, as he discontinued follow-up stating he felt “controlled by the government.”

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia (diagnosed 2020-01-15)
- Iron deficiency anemia (diagnosed 2020-07-15)
- Obstructive sleep apnea (diagnosed 2021-03-15)
- Ectopic ACTH production (diagnosed 2022-01-15)
- Cervical radiculopathy due to C6-C7 disc herniation (diagnosed 2023-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Hepatitis B Infection
- Peptic Ulcer Disease
- Inguinal hernia, reducible

His medical history includes hypertension,.

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Metformin 1000 mg twice daily
- Risperidone 2 mg daily (currently noncompliant)
- Diphenhydramine 25 mg at bedtime as needed for sleep
- Calcium carbonate 500 mg as needed for dyspepsia

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Schizophrenia
- Brother: Hepatitis B carrier

SOCIAL HISTORY:
He lives alone after retiring early.

REVIEW OF SYSTEMS:
He denies nausea, vomiting, diarrhea, recent illness, or changes in appetite.

VITAL SIGNS:
Temperature is normal, blood pressure is 122/78 mmHg, and pulse is 80/min.

PHYSICAL EXAMINATION:
On examination, he appears thin, with dry skin and flat affect. Neurologic examination is nonfocal.

LABORATORY DATA:
Laboratory studies reveal:
Serum sodium 127 mEq/L
Serum osmolality 258 mOsm/kg
Urine osmolality 115 mOsm/kg
Fasting glucose 88 mg/dL

## Encounter 6: source 2025-01-15, practice 2024-10-23, inpatient, Internal Medicine

Chief complaint: Sudden onset of severe periumbilical abdominal pain with associated low-grade fever and recent unintentional weight loss.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Aisha Hassan, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Sudden onset of severe periumbilical abdominal pain with associated low-grade fever and recent unintentional weight loss.

HISTORY OF PRESENT ILLNESS:
The patient is a 51-year-old male with chronic hepatitis B, schizophrenia, type 2 diabetes, and reducible inguinal hernia who presents with severe abdominal pain that began suddenly 6 hours ago. The pain is localized to the periumbilical region and is described as sharp and constant. He also reports a low-grade fever and a recent unintentional weight loss of 10 pounds over the past month.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia (diagnosed 2020-01-15)
- Iron deficiency anemia (diagnosed 2020-07-15)
- Obstructive sleep apnea (diagnosed 2021-03-15)
- Ectopic ACTH production (diagnosed 2022-01-15)
- Cervical radiculopathy due to C6-C7 disc herniation (diagnosed 2023-01-15)
- Primary polydipsia (diagnosed 2024-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Hepatitis B Infection
- Peptic Ulcer Disease
- Inguinal hernia, reducible

His past medical history is significant for hypertension.

MEDICATIONS:
He takes lisinopril.

Home Medications:
- Metformin 1000 mg twice daily
- Risperidone 2 mg daily (currently noncompliant)
- Diphenhydramine 25 mg at bedtime as needed for sleep
- Calcium carbonate 500 mg as needed for dyspepsia

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Schizophrenia
- Brother: Hepatitis B carrier

SOCIAL HISTORY:
Smoking: Former smoker, quit 2 years ago, 10 pack-year history. Alcohol: Denies alcohol use. Occupation: Retired construction worker (disability).

VITAL SIGNS:
Blood pressure is 145/90 mmHg, heart rate is 98 beats per minute, and temperature is 38.1°C (100.6°F)

PHYSICAL EXAMINATION:
Abdominal examination reveals tenderness to palpation in the periumbilical area with no rebound or guarding. Bowel sounds are normal. There is no jaundice or rash.

LABORATORY DATA:
- Hemoglobin: 13.2 g/dL
- White blood cell count: 12,000/mm³
- Platelet count: 250,000/mm³
- Serum creatinine: 1.8 mg/dL (baseline 1.2 mg/dL)
- AST: 45 U/L
- ALT: 50 U/L
- Hepatitis B surface antigen: Positive

IMAGING:
An abdominal CT scan with contrast shows no evidence of appendicitis, cholecystitis, or pancreatitis but reveals multiple small aneurysms and irregular narrowing of the mesenteric arteries.

## Encounter 7: source 2026-01-15, practice 2025-10-15, ed, Emergency Medicine

Chief complaint: Severe pain, discoloration, and tissue necrosis of fingers after prolonged cold exposure during snowstorm.

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe pain, discoloration, and tissue necrosis of fingers after prolonged cold exposure during snowstorm.

HISTORY OF PRESENT ILLNESS:
The 52-year-old male with schizophrenia, diabetes, and prior polyarteritis nodosa presents after he was stranded overnight in his car during a snowstorm. He reports that he was unable to keep the car heater running and did not have gloves.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia (diagnosed 2020-01-15)
- Iron deficiency anemia (diagnosed 2020-07-15)
- Obstructive sleep apnea (diagnosed 2021-03-15)
- Ectopic ACTH production (diagnosed 2022-01-15)
- Cervical radiculopathy due to C6-C7 disc herniation (diagnosed 2023-01-15)
- Primary polydipsia (diagnosed 2024-01-15)
- Polyarteritis nodosa (diagnosed 2025-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Hepatitis B Infection
- Peptic Ulcer Disease
- Inguinal hernia, reducible

His past medical history is significant for hypertension, controlled with lisinopril.

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Metformin 1000 mg twice daily
- Risperidone 2 mg daily (currently noncompliant)
- Diphenhydramine 25 mg at bedtime as needed for sleep
- Calcium carbonate 500 mg as needed for dyspepsia

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Schizophrenia
- Brother: Hepatitis B carrier

SOCIAL HISTORY:
Smoking: Former smoker, quit 2 years ago, 10 pack-year history. Alcohol: Denies alcohol use. Occupation: Retired construction worker (disability).

VITAL SIGNS:
His temperature is 35.6°C (96.1°F), blood pressure is 135/85 mmHg, pulse is 88 beats per minute, and respiratory rate is 16 beats per minute.

PHYSICAL EXAMINATION:
On examination, his fingers are cold, mottled, and have a waxy appearance. There is no active bleeding or blister formation. Sensation is decreased, and capillary refill is absent in the affected fingers. The rest of the physical examination is unremarkable.

PLAN:
Initial management includes rapid rewarming of his fingers in a warm water bath at 40°C (104°F) for 30 minutes. Despite rewarming, the patient's fingers remain discolored with persistent sensory loss and absent capillary refill.
