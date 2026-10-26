# synthetic_hospital patient 2288

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White, Medicare with supplemental private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-05-25 (Tue) | outpatient | Gastroenterology |
| 1 | 2020-07-15 | 2021-11-24 (Wed) | outpatient | Cardiology |
| 2 | 2021-07-15 | 2022-11-29 (Tue) | outpatient | Neurology |
| 3 | 2023-01-15 | 2024-06-01 (Sat) | icu | Cardiac Surgery |
| 4 | 2023-01-15 | 2024-06-04 (Tue) | ed | Emergency Medicine |
| 5 | 2023-01-15 | 2024-06-08 (Sat) | ed | Emergency Medicine |
| 6 | 2023-07-15 | 2024-12-02 (Mon) | outpatient | Pulmonology |
| 7 | 2025-01-15 | 2026-06-08 (Mon) | outpatient | Neurology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare with supplemental private insurance",
 "occupation": "Retired high school mathematics teacher",
 "alcohol_use": "No alcohol use",
 "family_history": [
  "Father: Myocardial infarction at age 63",
  "Mother: Alzheimer disease",
  "One older brother with hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "25/100 mg three times daily",
   "name": "Carbidopa/Levodopa"
  },
  {
   "dose": "1.5 mg daily",
   "name": "Pramipexole"
  },
  {
   "dose": "20 mg daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Coronary artery bypass grafting (CABG) with left internal mammary artery harvest",
  "Bilateral carpal tunnel release (age 64)"
 ],
 "chronic_conditions": [
  "Parkinson disease (diagnosed age 62)",
  "Coronary artery disease (severe multivessel)",
  "Cardiac amyloidosis (transthyretin wild-type)",
  "Achalasia",
  "Carpal tunnel syndrome (bilateral)",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Organic mood disorder due to structural intracranial lesion", "Phrenic nerve injury", "Achalasia", "Cardiac Amyloidosis", "Anticoagulation-related hemorrhage", "Postoperative mediastinitis", "Superficial surgical wound dehiscence", "Parkinson disease with psychosis"]
Comorbidities: ["Status post coronary artery bypass grafting", "Hyperlipidemia", "Coronary Artery Disease (Antiplatelet Therapy)"]

## Encounter 0: source 2020-01-15, practice 2021-05-25, outpatient, Gastroenterology

Chief complaint: Progressive difficulty swallowing with sensation of food slowing behind lower sternum

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Maria Gonzalez, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty swallowing with sensation of food slowing behind lower sternum

HISTORY OF PRESENT ILLNESS:
3-month history of progressive difficulty swallowing. He describes the sensation that both solid foods and liquids seem to slow down behind the lower part of his breast bone shortly after eating. He sometimes regurgitates undigested food but denies weight loss, chest pain, or heartburn.

PAST MEDICAL HISTORY:
Active Problem List:
- Parkinson disease (diagnosed age 62)
- Coronary artery disease (severe multivessel)
- Cardiac amyloidosis (transthyretin wild-type)
- Achalasia
- Carpal tunnel syndrome (bilateral)
- Hyperlipidemia

PAST SURGICAL HISTORY:
He has no prior upper gastrointestinal procedures

MEDICATIONS:
He takes only a statin for hyperlipidemia.

Home Medications:
- Carbidopa/Levodopa 25/100 mg three times daily
- Pramipexole 1.5 mg daily
- Atorvastatin 20 mg daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 63
- Mother: Alzheimer disease
- One older brother with hypertension

SOCIAL HISTORY:
He does not smoke or drink alcohol.

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

IMAGING:
A barium swallow is shown below.

## Encounter 1: source 2020-07-15, practice 2021-11-24, outpatient, Cardiology

Chief complaint: Progressive dyspnea on exertion and bilateral lower extremity edema over 6 months

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive dyspnea on exertion and bilateral lower extremity edema over 6 months

HISTORY OF PRESENT ILLNESS:
The patient with Parkinson disease, evaluated six months prior for dysphagia, presents to clinic with a 6-month history of progressive dyspnea on exertion and bilateral lower extremity edema.

PAST MEDICAL HISTORY:
Active Problem List:
- Achalasia (diagnosed 2020-01-15)
- Parkinson disease (diagnosed age 62)
- Coronary artery disease (severe multivessel)
- Cardiac amyloidosis (transthyretin wild-type)
- Carpal tunnel syndrome (bilateral)
- Hyperlipidemia

His past medical history is significant for carpal tunnel syndrome,.

PAST SURGICAL HISTORY:
Surgery 2 years ago.

MEDICATIONS:
Home Medications:
- Carbidopa/Levodopa 25/100 mg three times daily
- Pramipexole 1.5 mg daily
- Atorvastatin 20 mg daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 63
- Mother: Alzheimer disease
- One older brother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
On physical examination, his blood pressure is 130/85 mmHg, heart rate is 92 beats per minute,

PHYSICAL EXAMINATION:
Jugular venous pressure is elevated. Cardiac auscultation reveals a third heart sound (S3) but no murmurs. Lung fields are clear to auscultation.

LABORATORY DATA:
Laboratory findings are as follows:

- Serum creatinine: 1.2 mg/dL
- B-type natriuretic peptide (BNP): 900 pg/mL (normal <100 pg/mL)
- Urinalysis: 3+ protein, no blood or casts
- Serum albumin: 3.2 g/dL
- Total cholesterol: 210 mg/dL

IMAGING:
Echocardiogram: Increased left ventricular wall thickness with normal left ventricular cavity size, preserved ejection fraction, and speckled appearance of the myocardium

OTHER STUDIES:
Electrocardiogram (ECG): Low voltage QRS complexes and diffuse nonspecific ST-T wave abnormalities

## Encounter 2: source 2021-07-15, practice 2022-11-29, outpatient, Neurology

Chief complaint: Continued difficulty with fine motor movements and facial expressiveness despite medication compliance

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James Wilson, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Continued difficulty with fine motor movements and facial expressiveness despite medication compliance

HISTORY OF PRESENT ILLNESS:
This 68-year-old male with Parkinson disease, achalasia, and cardiac amyloidosis returns for follow-up of his movement disorder. His caregiver reports that he often hesitates when initiating motion and frequently appears stiff, especially in the morning and late in the day. Although he had some initial benefit from therapy started months ago, she feels his condition has plateaued and remains functionally limiting.

PAST MEDICAL HISTORY:
Active Problem List:
- Achalasia (diagnosed 2020-01-15)
- Cardiac Amyloidosis (diagnosed 2020-07-15)
- Parkinson disease (diagnosed age 62)
- Coronary artery disease (severe multivessel)
- Cardiac amyloidosis (transthyretin wild-type)
- Carpal tunnel syndrome (bilateral)
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Coronary artery bypass grafting (CABG) with left internal mammary artery harvest
- Bilateral carpal tunnel release (age 64)

MEDICATIONS:
He has been consistently taking a combination medication containing levodopa and carbidopa.

Home Medications:
- Carbidopa/Levodopa 25/100 mg three times daily
- Pramipexole 1.5 mg daily
- Atorvastatin 20 mg daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 63
- Mother: Alzheimer disease
- One older brother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Retired high school mathematics teacher.

REVIEW OF SYSTEMS:
His appetite is preserved and he sleeps through most nights. There is no report of muscle weakness or memory impairment, and he remains independent in basic daily needs.

PHYSICAL EXAMINATION:
Physical exam reveals slowed finger tapping, a low-volume voice, and reduced swing in the right arm while walking. His mood is subdued but cooperative, and his vitals are within normal limits.

LABORATORY DATA:
Basic labs including thyroid panel and metabolic profile are unremarkable.

PLAN:
A plan is made to adjust his regimen by adding a second medication targeting a different enzymatic mechanism.

## Encounter 3: source 2023-01-15, practice 2024-06-01, icu, Cardiac Surgery

Chief complaint: Persistent bleeding from chest tube and oozing around central line site following CABG

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Cardiac Surgery
Visit Type: icu

CHIEF COMPLAINT:
Persistent bleeding from chest tube and oozing around central line site following CABG

HISTORY OF PRESENT ILLNESS:
The patient is a 70-year-old male with a history of Parkinson disease, transthyretin wild-type cardiac amyloidosis diagnosed in 2020, and achalasia who undergoes coronary artery bypass grafting (CABG) for severe multivessel coronary artery disease. The surgery is uneventful, and he is transferred to the cardiac ICU on mechanical ventilation. Two hours later, the surgical team is called to the bedside because of persistent bleeding from the chest tube and oozing around the central line site. The patient has received several liters of IV fluids and blood products, but the bleeding continues. He had been anticoagulated intraoperatively with intravenous unfractionated heparin to maintain cardiopulmonary bypass.

PAST MEDICAL HISTORY:
Active Problem List:
- Achalasia (diagnosed 2020-01-15)
- Cardiac Amyloidosis (diagnosed 2020-07-15)
- Parkinson disease with psychosis (diagnosed 2021-07-15)
- Parkinson disease (diagnosed age 62)
- Coronary artery disease (severe multivessel)
- Cardiac amyloidosis (transthyretin wild-type)
- Carpal tunnel syndrome (bilateral)
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Coronary artery bypass grafting (CABG) with left internal mammary artery harvest
- Bilateral carpal tunnel release (age 64)

MEDICATIONS:
Home Medications:
- Carbidopa/Levodopa 25/100 mg three times daily
- Pramipexole 1.5 mg daily
- Atorvastatin 20 mg daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 63
- Mother: Alzheimer disease
- One older brother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Retired high school mathematics teacher.

LABORATORY DATA:
Postoperative labs reveal prolonged activated partial thromboplastin time (aPTT), normal prothrombin time (PT), normal platelet count, and normal fibrinogen. There is no evidence of thrombocytopenia or disseminated intravascular coagulation.

ASSESSMENT AND PLAN:
The surgical team suspects continued anticoagulation as the cause of bleeding and requests urgent reversal.

## Encounter 4: source 2023-01-15, practice 2024-06-04, ed, Emergency Medicine

Chief complaint: Mild chest pain and sensation of chest wall 'popping' with movement for 2 days

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Mild chest pain and sensation of chest wall 'popping' with movement for 2 days

PAST MEDICAL HISTORY:
Active Problem List:
- Achalasia (diagnosed 2020-01-15)
- Cardiac Amyloidosis (diagnosed 2020-07-15)
- Parkinson disease with psychosis (diagnosed 2021-07-15)
- Anticoagulation-related hemorrhage (diagnosed 2023-01-15)
- Parkinson disease (diagnosed age 62)
- Coronary artery disease (severe multivessel)
- Cardiac amyloidosis (transthyretin wild-type)
- Carpal tunnel syndrome (bilateral)
- Hyperlipidemia

PAST SURGICAL HISTORY:
He underwent coronary artery bypass grafting (CABG) 3 weeks ago.

MEDICATIONS:
Home Medications:
- Carbidopa/Levodopa 25/100 mg three times daily
- Pramipexole 1.5 mg daily
- Atorvastatin 20 mg daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 63
- Mother: Alzheimer disease
- One older brother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Retired high school mathematics teacher.

REVIEW OF SYSTEMS:
He denies fever, cough, or shortness of breath.

VITAL SIGNS:
On examination, his temperature is 37.2°C (99°F), blood pressure is 130/85 mmHg, pulse is 88/min, and respiratory rate is 16/min.

PHYSICAL EXAMINATION:
Cardiovascular examination reveals a well-healed median sternotomy scar. Palpation of the chest wall elicits a 'clicking' sensation and the patient reports discomfort. There is no erythema or purulent discharge from the incision site.

LABORATORY DATA:
Laboratory values are as follows:

- White blood cell count: 7,500/mm³
- Hemoglobin: 13.2 g/dL
- Platelet count: 250,000/mm³
- C-reactive protein: 8 mg/L (normal <5 mg/L)

IMAGING:
A chest X-ray shows a displaced sternal wire.

## Encounter 5: source 2023-01-15, practice 2024-06-08, ed, Emergency Medicine

Chief complaint: Fever, malaise, and increasing redness and pain around sternal wound 10 days post-CABG

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Amara Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, malaise, and increasing redness and pain around sternal wound 10 days post-CABG

HISTORY OF PRESENT ILLNESS:
This patient with known cardiac amyloidosis and Parkinson disease presents to the emergency department 10 days post-coronary artery bypass grafting (CABG), having recently been evaluated for wound dehiscence. He reports a low-grade fever started 3 days ago, which has progressively worsened.

PAST MEDICAL HISTORY:
Active Problem List:
- Achalasia (diagnosed 2020-01-15)
- Cardiac Amyloidosis (diagnosed 2020-07-15)
- Parkinson disease with psychosis (diagnosed 2021-07-15)
- Anticoagulation-related hemorrhage (diagnosed 2023-01-15)
- Superficial surgical wound dehiscence (diagnosed 2023-01-15)
- Parkinson disease (diagnosed age 62)
- Coronary artery disease (severe multivessel)
- Cardiac amyloidosis (transthyretin wild-type)
- Carpal tunnel syndrome (bilateral)
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Coronary artery bypass grafting (CABG) with left internal mammary artery harvest
- Bilateral carpal tunnel release (age 64)

MEDICATIONS:
Home Medications:
- Carbidopa/Levodopa 25/100 mg three times daily
- Pramipexole 1.5 mg daily
- Atorvastatin 20 mg daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 63
- Mother: Alzheimer disease
- One older brother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
On examination, his temperature is 38.9°C (102°F), heart rate is 110 bpm, and blood pressure is 130/85 mmHg.

PHYSICAL EXAMINATION:
The sternal wound is erythematous with some purulent discharge. There is notable swelling and tenderness to palpation over the sternum.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 15,000/uL (normal: 4,500-11,000/uL)
- Hemoglobin: 12.5 g/dL (normal: 13.8-17.2 g/dL)
- C-reactive protein: 24 mg/L (normal: <10 mg/L)
- Blood cultures: Pending

IMAGING:
A chest X-ray reveals a small fluid collection anterior to the sternum.

## Encounter 6: source 2023-07-15, practice 2024-12-02, outpatient, Pulmonology

Chief complaint: Difficulty breathing when lying flat and fatigue on exertion with paradoxical abdominal motion

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. David Kim, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Difficulty breathing when lying flat and fatigue on exertion with paradoxical abdominal motion

HISTORY OF PRESENT ILLNESS:
This 70-year-old male with Parkinson disease, transthyretin wild-type cardiac amyloidosis, and recent coronary artery bypass grafting complicated by mediastinitis presents to the Pulmonology clinic with difficulty breathing, especially when lying flat, and fatigue on exertion that has progressively worsened over the past two months. He reports a sensation of his abdomen moving inward when he takes a deep breath.

PAST MEDICAL HISTORY:
Active Problem List:
- Achalasia (diagnosed 2020-01-15)
- Cardiac Amyloidosis (diagnosed 2020-07-15)
- Parkinson disease with psychosis (diagnosed 2021-07-15)
- Anticoagulation-related hemorrhage (diagnosed 2023-01-15)
- Superficial surgical wound dehiscence (diagnosed 2023-01-15)
- Postoperative mediastinitis (diagnosed 2023-01-15)
- Parkinson disease (diagnosed age 62)
- Coronary artery disease (severe multivessel)
- Cardiac amyloidosis (transthyretin wild-type)
- Carpal tunnel syndrome (bilateral)
- Hyperlipidemia

His past medical history is significant for coronary artery disease,.

PAST SURGICAL HISTORY:
For which he underwent coronary artery bypass grafting (CABG) surgery three months ago.

MEDICATIONS:
Home Medications:
- Carbidopa/Levodopa 25/100 mg three times daily
- Pramipexole 1.5 mg daily
- Atorvastatin 20 mg daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 63
- Mother: Alzheimer disease
- One older brother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, you observe that his abdomen indeed moves inward during inspiration, and auscultation of the lungs reveals decreased breath sounds at the left lung base.

LABORATORY DATA:
Laboratory values:
- Hemoglobin: 13.8 g/dL
- White blood cell count: 7,200 /μL
- Platelet count: 250,000 /μL
- Serum electrolytes: Within normal limits
- Arterial blood gas on room air:
- pH: 7.42
- PaCO2: 44 mmHg
- PaO2: 72 mmHg
- HCO3-: 24 mEq/L

IMAGING:
Chest x-ray shows an elevated left hemidiaphragm.

## Encounter 7: source 2025-01-15, practice 2026-06-08, outpatient, Neurology

Chief complaint: Acute onset visual hallucinations and paranoid ideation accusing neighbors of stealing

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. James Wilson, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Acute onset visual hallucinations and paranoid ideation accusing neighbors of stealing

HISTORY OF PRESENT ILLNESS:
The patient is a 72-year-old male with Parkinson disease, transthyretin wild-type cardiac amyloidosis status post complicated coronary artery bypass grafting, and achalasia, returning for evaluation and presents to the clinic with his wife, who reports that over the past month, he has been experiencing visual hallucinations and has become increasingly paranoid, accusing neighbors of stealing from him. The patient has been on a stable regimen of carbidopa/levodopa and pramipexole. His symptoms are well controlled, and he denies any recent changes in his motor function.

PAST MEDICAL HISTORY:
Active Problem List:
- Achalasia (diagnosed 2020-01-15)
- Cardiac Amyloidosis (diagnosed 2020-07-15)
- Parkinson disease with psychosis (diagnosed 2021-07-15)
- Anticoagulation-related hemorrhage (diagnosed 2023-01-15)
- Superficial surgical wound dehiscence (diagnosed 2023-01-15)
- Postoperative mediastinitis (diagnosed 2023-01-15)
- Phrenic nerve injury (diagnosed 2023-07-15)
- Parkinson disease (diagnosed age 62)
- Coronary artery disease (severe multivessel)
- Cardiac amyloidosis (transthyretin wild-type)
- Carpal tunnel syndrome (bilateral)
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Coronary artery bypass grafting (CABG) with left internal mammary artery harvest
- Bilateral carpal tunnel release (age 64)

MEDICATIONS:
His current medications include:
- Carbidopa/Levodopa 25/100 mg three times daily
- Pramipexole 1.5 mg daily
- Atorvastatin 20 mg daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 63
- Mother: Alzheimer disease
- One older brother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
His vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, the patient is alert and oriented to person, place, and time. Neurological examination reveals a resting tremor, bradykinesia, and rigidity consistent with his known Parkinson disease. There are no signs of infection or delirium.
