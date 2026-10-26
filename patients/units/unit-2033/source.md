# synthetic_hospital patient 2033

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 57 at first encounter, sex M, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-11-02 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2021-02-01 (Mon) | outpatient | Orthopedics |
| 2 | 2020-10-15 | 2021-08-09 (Mon) | outpatient | Hematology & Oncology |
| 3 | 2021-04-15 | 2022-02-10 (Thu) | outpatient | Hematology & Oncology |
| 4 | 2022-04-15 | 2023-02-07 (Tue) | outpatient | Neurology |
| 5 | 2023-04-15 | 2024-02-13 (Tue) | outpatient | Cardiology |
| 6 | 2024-04-15 | 2025-02-17 (Mon) | outpatient | Pulmonology |
| 7 | 2025-04-15 | 2026-02-09 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Office Administrator",
 "alcohol_use": "Social use (weekends only)",
 "family_history": [
  "Father: Hypertension, coronary artery disease",
  "Mother: Type 2 Diabetes Mellitus",
  "Paternal uncle: Myeloproliferative neoplasm"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "12.5 mg daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  }
 ],
 "surgical_history": [
  "Esophagogastroduodenoscopy with variceal band ligation (prior admission)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Obstructive Sleep Apnea (poor CPAP adherence)",
  "History of upper gastrointestinal bleeding"
 ],
 "age_at_first_encounter": 57
}
```

Primary diagnoses: ["Aortic valve stenosis", "Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease)", "Polycythemia vera", "Bleeding esophageal varices", "Amyotrophic lateral sclerosis", "Masked Hypertension", "Achilles tendinitis"]
Comorbidities: ["Left ventricular hypertrophy", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2020-11-02, outpatient, Internal Medicine

Chief complaint: Routine health check-up with no active complaints

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health check-up with no active complaints

HISTORY OF PRESENT ILLNESS:
He reports no complaints and is particularly proud of his commitment to a healthy lifestyle, which includes regular exercise and a low-sodium diet.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Obstructive Sleep Apnea (poor CPAP adherence)
- History of upper gastrointestinal bleeding

His medical history is significant for type 2 diabetes mellitus, which is well-controlled with metformin.

PAST SURGICAL HISTORY:
- Esophagogastroduodenoscopy with variceal band ligation (prior admission)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Hydrochlorothiazide 12.5 mg daily
- Metformin 1000 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is notable for hypertension and stroke in his parents.

SOCIAL HISTORY:
He denies any history of smoking or alcohol use.

VITAL SIGNS:
On physical examination, his blood pressure is 128/82 mmHg, heart rate is 72 beats per minute, and body mass index is 29 kg/m^2.

PHYSICAL EXAMINATION:
Cardiac examination reveals a sustained apical impulse and a soft S4 heart sound. Fundoscopic examination is normal.

LABORATORY DATA:
Laboratory findings are as follows:

- Fasting blood glucose: 98 mg/dL
- Hemoglobin A1c: 6.2%
- Serum creatinine: 1.0 mg/dL
- Total cholesterol: 190 mg/dL
- LDL cholesterol: 110 mg/dL
- HDL cholesterol: 45 mg/dL
- Triglycerides: 150 mg/dL
- Urinalysis: No proteinuria or hematuria

OTHER STUDIES:
An electrocardiogram shows left ventricular hypertrophy.

ASSESSMENT AND PLAN:
Given the patient's clinical presentation and findings,

## Encounter 1: source 2020-04-15, practice 2021-02-01, outpatient, Orthopedics

Chief complaint: Three-day history of right heel pain worsening with walking

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Johnson, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Three-day history of right heel pain worsening with walking

HISTORY OF PRESENT ILLNESS:
Patient, evaluated for masked hypertension in January, presents with right heel pain. He describes the pain as a constant, sharp sensation that worsens with walking. He denies any recent trauma or engaging in new physical activities.

PAST MEDICAL HISTORY:
Active Problem List:
- Masked Hypertension (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Obstructive Sleep Apnea (poor CPAP adherence)
- History of upper gastrointestinal bleeding

His medical history is significant for hypertension and a recent urinary tract infection for which he started taking ciprofloxacin 5 days ago.

PAST SURGICAL HISTORY:
- Esophagogastroduodenoscopy with variceal band ligation (prior admission)

MEDICATIONS:
The patient's current medications include ciprofloxacin and hydrochlorothiazide.

Home Medications:
- Lisinopril 20 mg daily
- Metformin 1000 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, coronary artery disease
- Mother: Type 2 Diabetes Mellitus
- Paternal uncle: Myeloproliferative neoplasm

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social use (weekends only). Occupation: Office Administrator.

VITAL SIGNS:
His vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, there is tenderness to palpation over the right Achilles tendon, and the patient reports pain with passive dorsiflexion of the right foot. There is no erythema, warmth, or swelling.

LABORATORY DATA:
Laboratory values:
- White blood cell count: 7,000/uL
- C-reactive protein: 2 mg/L
- Erythrocyte sedimentation rate: 15 mm/hr

## Encounter 2: source 2020-10-15, practice 2021-08-09, outpatient, Hematology & Oncology

Chief complaint: Persistent fatigue, intermittent headaches, and burning pain in hands and feet

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Priya Patel, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue, intermittent headaches, and burning pain in hands and feet

HISTORY OF PRESENT ILLNESS:
The patient is a 57-year-old male with hypertension, type 2 diabetes mellitus, obstructive sleep apnea with poor CPAP adherence, and prior upper gastrointestinal bleeding who presents for Hematology evaluation. He was previously seen for masked hypertension and Achilles tendinitis. He reports a recent sensation of fullness in the upper abdomen after eating and notes that his face often appears red, especially after taking hot showers. He has also experienced occasional burning pain and redness in his hands and feet that improves when he immerses them in cool water. He denies fever, weight loss, night sweats, or recent illness.

PAST MEDICAL HISTORY:
Active Problem List:
- Masked Hypertension (diagnosed 2020-01-15)
- Achilles tendinitis (diagnosed 2020-04-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Obstructive Sleep Apnea (poor CPAP adherence)
- History of upper gastrointestinal bleeding

His past medical history is notable only for well-controlled hypertension.

PAST SURGICAL HISTORY:
- Esophagogastroduodenoscopy with variceal band ligation (prior admission)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Hydrochlorothiazide 12.5 mg daily
- Metformin 1000 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, coronary artery disease
- Mother: Type 2 Diabetes Mellitus
- Paternal uncle: Myeloproliferative neoplasm

SOCIAL HISTORY:
He does not smoke or use illicit drugs and drinks alcohol only on weekends.

VITAL SIGNS:
Vital signs show a blood pressure of 148/90 mm Hg, pulse of 86/min, and respiratory rate of 14/min.

PHYSICAL EXAMINATION:
On examination, he appears flushed. The spleen is palpable just below the left costal margin. Cardiopulmonary and neurologic examinations are unremarkable.

LABORATORY DATA:
Laboratory studies reveal the following:
Hemoglobin                                20.3 g/dL
Hematocrit                                61%
WBC count                                 12,400 /mm³
Platelet count                            570,000 /mm³
Erythropoietin level                      Decreased

## Encounter 3: source 2021-04-15, practice 2022-02-10, outpatient, Hematology & Oncology

Chief complaint: Intermittent headaches, blurred vision, and facial redness

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. Priya Patel, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent headaches, blurred vision, and facial redness

HISTORY OF PRESENT ILLNESS:
Patient with HTN, DM, OSA returns for PV follow-up with headaches, blurred vision, and facial redness, reporting mild dizziness, facial redness, and occasional itching after warm showers. Denies recent infections, travel, or trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Masked Hypertension (diagnosed 2020-01-15)
- Achilles tendinitis (diagnosed 2020-04-15)
- Polycythemia vera (diagnosed 2020-10-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Obstructive Sleep Apnea (poor CPAP adherence)
- History of upper gastrointestinal bleeding

PAST SURGICAL HISTORY:
- Esophagogastroduodenoscopy with variceal band ligation (prior admission)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Hydrochlorothiazide 12.5 mg daily
- Metformin 1000 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, coronary artery disease
- Mother: Type 2 Diabetes Mellitus
- Paternal uncle: Myeloproliferative neoplasm

SOCIAL HISTORY:
He does not smoke and drinks alcohol socially.

VITAL SIGNS:
Blood pressure is 144/86 mmHg, and heart rate is 84/min.

PHYSICAL EXAMINATION:
Physical exam shows ruddy skin tone and no palpable organomegaly. Neurologic exam is normal. Fundoscopy reveals engorged retinal veins.

## Encounter 4: source 2022-04-15, practice 2023-02-07, outpatient, Neurology

Chief complaint: Progressive hand weakness, difficulty walking, and occasional slurred speech

ENCOUNTER NOTE
Date: 2022-04-15
Provider: Dr. Elena Rodriguez, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive hand weakness, difficulty walking, and occasional slurred speech

HISTORY OF PRESENT ILLNESS:
The patient is a 59-year-old male with hypertension, type 2 diabetes mellitus, obstructive sleep apnea with poor CPAP adherence, prior upper gastrointestinal bleeding, and polycythemia vera previously managed in Hematology, who presents to the Neurology clinic with a 6-month history of progressive weakness in his hands and difficulty walking. He reports that he frequently trips over his own feet and has difficulty holding onto objects, which has led to him dropping cups and utensils. He also mentions occasional slurring of his speech, especially when tired.

PAST MEDICAL HISTORY:
Active Problem List:
- Masked Hypertension (diagnosed 2020-01-15)
- Achilles tendinitis (diagnosed 2020-04-15)
- Polycythemia vera (diagnosed 2020-10-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Obstructive Sleep Apnea (poor CPAP adherence)
- History of upper gastrointestinal bleeding

His past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Esophagogastroduodenoscopy with variceal band ligation (prior admission)

MEDICATIONS:
He takes lisinopril.

Home Medications:
- Hydrochlorothiazide 12.5 mg daily
- Metformin 1000 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, coronary artery disease
- Mother: Type 2 Diabetes Mellitus
- Paternal uncle: Myeloproliferative neoplasm

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social use (weekends only). Occupation: Office Administrator.

PHYSICAL EXAMINATION:
On examination, you note atrophy of the intrinsic hand muscles and fasciculations in the upper extremities. His gait is spastic, and he has hyperreflexia in the lower extremities with an upgoing Babinski sign.

LABORATORY DATA:
Routine blood work, including complete blood count, electrolytes, renal function, and liver enzymes, is within normal limits.

OTHER STUDIES:
An EMG shows widespread denervation with both acute and chronic changes in multiple myotomes.

## Encounter 5: source 2023-04-15, practice 2024-02-13, outpatient, Cardiology

Chief complaint: Increasing exertional dyspnea, orthopnea, and palpitations over six months

ENCOUNTER NOTE
Date: 2023-04-15
Provider: Dr. James Wilson, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing exertional dyspnea, orthopnea, and palpitations over six months

HISTORY OF PRESENT ILLNESS:
The patient with masked hypertension, polycythemia vera, and amyotrophic lateral sclerosis presents with increasing exertional dyspnea and orthopnea over the past six months. He also reports a recent onset of palpitations.

PAST MEDICAL HISTORY:
Active Problem List:
- Masked Hypertension (diagnosed 2020-01-15)
- Achilles tendinitis (diagnosed 2020-04-15)
- Polycythemia vera (diagnosed 2020-10-15)
- Amyotrophic lateral sclerosis (diagnosed 2022-04-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Obstructive Sleep Apnea (poor CPAP adherence)
- History of upper gastrointestinal bleeding

PAST SURGICAL HISTORY:
- Esophagogastroduodenoscopy with variceal band ligation (prior admission)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Hydrochlorothiazide 12.5 mg daily
- Metformin 1000 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, coronary artery disease
- Mother: Type 2 Diabetes Mellitus
- Paternal uncle: Myeloproliferative neoplasm

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social use (weekends only). Occupation: Office Administrator.

VITAL SIGNS:
His blood pressure is 150/50 mmHg, and his pulse is 90 bpm and bounding.

PHYSICAL EXAMINATION:
On physical examination, you note a high-pitched, blowing diastolic murmur best heard at the left sternal border.

IMAGING:
Chest X-ray shows cardiomegaly with evidence of pulmonary congestion.

OTHER STUDIES:
An echocardiogram reveals left ventricular dilation with normal systolic function and severe aortic regurgitation.

## Encounter 6: source 2024-04-15, practice 2025-02-17, outpatient, Pulmonology

Chief complaint: Progressive exertional dyspnea, leg swelling, and mild abdominal discomfort

ENCOUNTER NOTE
Date: 2024-04-15
Provider: Dr. David Kim, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive exertional dyspnea, leg swelling, and mild abdominal discomfort

HISTORY OF PRESENT ILLNESS:
The patient with ALS and OSA with poor CPAP adherence returns reporting difficulty walking even short distances without becoming short of breath. He denies chest pain, orthopnea, or paroxysmal nocturnal dyspnea.

PAST MEDICAL HISTORY:
Active Problem List:
- Masked Hypertension (diagnosed 2020-01-15)
- Achilles tendinitis (diagnosed 2020-04-15)
- Polycythemia vera (diagnosed 2020-10-15)
- Amyotrophic lateral sclerosis (diagnosed 2022-04-15)
- Aortic valve stenosis (diagnosed 2023-04-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Obstructive Sleep Apnea (poor CPAP adherence)
- History of upper gastrointestinal bleeding

He has a history of untreated obstructive sleep apnea, and he was previously advised to use a CPAP device, but admits he rarely wears it.

PAST SURGICAL HISTORY:
- Esophagogastroduodenoscopy with variceal band ligation (prior admission)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Hydrochlorothiazide 12.5 mg daily
- Metformin 1000 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, coronary artery disease
- Mother: Type 2 Diabetes Mellitus
- Paternal uncle: Myeloproliferative neoplasm

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social use (weekends only). Occupation: Office Administrator.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
Physical examination reveals jugular venous distension, a prominent P2 heart sound, and a holosystolic murmur at the left lower sternal border that increases with inspiration. His lungs are clear to auscultation. The abdomen is soft but mildly distended with a fluid wave, and 2+ pitting edema is noted in both lower extremities.

IMAGING:
Chest x-ray shows a vertically elongated cardiac silhouette and hyperlucent lung fields. Echocardiography reveals right ventricular hypertrophy and tricuspid regurgitation, with normal left ventricular function.

OTHER STUDIES:
Pulmonary capillary wedge pressure is within normal limits.

## Encounter 7: source 2025-04-15, practice 2026-02-09, ed, Emergency Medicine

Chief complaint: Sudden-onset hematemesis with fatigue and abdominal swelling

ENCOUNTER NOTE
Date: 2025-04-15
Provider: Dr. Robert Lee, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset hematemesis with fatigue and abdominal swelling

HISTORY OF PRESENT ILLNESS:
With history of upper GI bleeding and cor pulmonale, he reports fatigue, abdominal swelling, and poor appetite over the past several months.

PAST MEDICAL HISTORY:
Active Problem List:
- Masked Hypertension (diagnosed 2020-01-15)
- Achilles tendinitis (diagnosed 2020-04-15)
- Polycythemia vera (diagnosed 2020-10-15)
- Amyotrophic lateral sclerosis (diagnosed 2022-04-15)
- Aortic valve stenosis (diagnosed 2023-04-15)
- Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease) (diagnosed 2024-04-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Obstructive Sleep Apnea (poor CPAP adherence)
- History of upper gastrointestinal bleeding

He has a history of hypertension and has been hospitalized in the past for gastrointestinal bleeding but has not followed up regularly.

PAST SURGICAL HISTORY:
- Esophagogastroduodenoscopy with variceal band ligation (prior admission)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Hydrochlorothiazide 12.5 mg daily
- Metformin 1000 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, coronary artery disease
- Mother: Type 2 Diabetes Mellitus
- Paternal uncle: Myeloproliferative neoplasm

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social use (weekends only). Occupation: Office Administrator.

VITAL SIGNS:
His temperature is 36.7°C (98°F), pulse is 120/min, blood pressure is 85/50 mm Hg, and oxygen saturation is 93% on room air.

PHYSICAL EXAMINATION:
On arrival, he is pale and diaphoretic, with blood noted in the oropharynx. Physical examination reveals scleral icterus, spider angiomas, palmar erythema, and a distended abdomen with shifting dullness.

PLAN:
He is promptly intubated for airway protection, and two large-bore IVs are placed for crystalloid and packed red blood cell transfusion.
