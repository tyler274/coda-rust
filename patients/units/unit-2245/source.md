# synthetic_hospital patient 2245

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 65 at first encounter, sex M, Non-Hispanic White, Medicare with supplemental Plan F.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-11-19 (Thu) | outpatient | Cardiology |
| 1 | 2020-04-15 | 2021-02-22 (Mon) | outpatient | Internal Medicine |
| 2 | 2020-07-15 | 2021-05-31 (Mon) | ed | Emergency Medicine |
| 3 | 2021-01-15 | 2021-11-16 (Tue) | outpatient | Cardiology |
| 4 | 2022-01-15 | 2022-11-17 (Thu) | ed | Emergency Medicine |
| 5 | 2023-01-15 | 2023-11-27 (Mon) | ed | Emergency Medicine |
| 6 | 2024-01-15 | 2024-11-28 (Thu) | ed | Emergency Medicine |
| 7 | 2025-01-15 | 2025-11-19 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare with supplemental Plan F",
 "occupation": "Retired postal worker",
 "alcohol_use": "Social alcohol use (1-2 drinks per month)",
 "family_history": [
  "Father: Myocardial infarction at age 68 (deceased)",
  "Mother: Cerebrovascular accident at age 75",
  "Brother: Abdominal aortic aneurysm (status post endovascular repair)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "50-pack-year history (1 pack per day for 50 years, quit 3 months prior)",
 "home_medications": [
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Emergency ascending aortic replacement with valve-sparing root repair for Type A aortic dissection",
  "Endovascular repair of abdominal aortic aneurysm",
  "Percutaneous coronary intervention with drug-eluting stent to right coronary artery",
  "Exploratory laparotomy with primary repair of perforated sigmoid diverticulum"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia",
  "Severe calcific aortic stenosis",
  "Tobacco use disorder (former)",
  "Abdominal aortic aneurysm",
  "Coronary artery disease",
  "Solitary pulmonary nodule under active surveillance"
 ],
 "age_at_first_encounter": 65
}
```

Primary diagnoses: ["Aortic valve stenosis", "Solitary pulmonary nodule with high suspicion for malignancy", "Type A Aortic Dissection", "Symptomatic Abdominal Aortic Aneurysm", "Acute ST-segment elevation myocardial infarction (STEMI) of inferior wall", "Vagal bradycardia secondary to pneumoperitoneum"]
Comorbidities: ["Tobacco use disorder", "Hyperlipidemia", "Left ventricular hypertrophy", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2020-11-19, outpatient, Cardiology

Chief complaint: Progressive exertional dyspnea and decreased exercise tolerance over 6 months with recent chest discomfort on climbing stairs

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Vasquez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive exertional dyspnea and decreased exercise tolerance over 6 months with recent chest discomfort on climbing stairs

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 6-month history of progressive exertional dyspnea and decreased exercise tolerance. He reports a recent onset of chest discomfort on climbing stairs, which resolves with rest.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- Severe calcific aortic stenosis
- Tobacco use disorder (former)
- Abdominal aortic aneurysm
- Coronary artery disease
- Solitary pulmonary nodule under active surveillance

His past medical history is significant for hypertension, hyperlipidemia.

PAST SURGICAL HISTORY:
- Emergency ascending aortic replacement with valve-sparing root repair for Type A aortic dissection
- Endovascular repair of abdominal aortic aneurysm
- Percutaneous coronary intervention with drug-eluting stent to right coronary artery
- Exploratory laparotomy with primary repair of perforated sigmoid diverticulum

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68 (deceased)
- Mother: Cerebrovascular accident at age 75
- Brother: Abdominal aortic aneurysm (status post endovascular repair)

SOCIAL HISTORY:
50-pack-year smoking history

VITAL SIGNS:
On physical examination, his blood pressure is 130/70 mmHg, heart rate is 78 bpm, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a harsh, crescendo-decrescendo systolic murmur heard best at the second right intercostal space that radiates to the neck. The murmur peaks late in systole and is associated with a delayed and diminished carotid pulse. The second heart sound is soft.

OTHER STUDIES:
An electrocardiogram shows left ventricular hypertrophy.

## Encounter 1: source 2020-04-15, practice 2021-02-22, outpatient, Internal Medicine

Chief complaint: Routine check-up for surveillance of solitary pulmonary nodule with incidental mention of mild exertional chest pain

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Maria Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine check-up for surveillance of solitary pulmonary nodule with incidental mention of mild exertional chest pain

HISTORY OF PRESENT ILLNESS:
Patient returns for pulmonary nodule surveillance with severe aortic stenosis. He has no complaints but mentions a recent episode of mild chest pain while climbing stairs, which resolved with rest.

PAST MEDICAL HISTORY:
Active Problem List:
- Aortic valve stenosis (diagnosed 2020-01-15)
- Hypertension
- Hyperlipidemia
- Severe calcific aortic stenosis
- Tobacco use disorder (former)
- Abdominal aortic aneurysm
- Coronary artery disease
- Solitary pulmonary nodule under active surveillance

His past medical history is significant for hypertension, controlled with lisinopril, and hyperlipidemia, managed with atorvastatin.

PAST SURGICAL HISTORY:
- Emergency ascending aortic replacement with valve-sparing root repair for Type A aortic dissection
- Endovascular repair of abdominal aortic aneurysm
- Percutaneous coronary intervention with drug-eluting stent to right coronary artery
- Exploratory laparotomy with primary repair of perforated sigmoid diverticulum

MEDICATIONS:
He takes lisinopril, atorvastatin.

Home Medications:
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68 (deceased)
- Mother: Cerebrovascular accident at age 75
- Brother: Abdominal aortic aneurysm (status post endovascular repair)

SOCIAL HISTORY:
45-pack-year smoking history

VITAL SIGNS:
On physical examination, his blood pressure is 135/85 mmHg, heart rate is 78 beats per minute, and respiratory rate is 16 beats per minute

PHYSICAL EXAMINATION:
Lung auscultation reveals clear breath sounds bilaterally.

IMAGING:
A chest x-ray is performed, which shows a 2 cm, well-circumscribed, solitary pulmonary nodule in the right upper lobe. No previous imaging is available for comparison. A subsequent chest computed tomography (CT) scan confirms the presence of the nodule, which has a smooth border and a non-calcified appearance.

## Encounter 2: source 2020-07-15, practice 2021-05-31, ed, Emergency Medicine

Chief complaint: Syncopal episode while climbing stairs preceded by chest pressure and shortness of breath

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Syncopal episode while climbing stairs preceded by chest pressure and shortness of breath

HISTORY OF PRESENT ILLNESS:
The patient with known severe calcific aortic stenosis returns for evaluation of a syncopal episode while climbing stairs, following prior encounters in January and April for exertional chest discomfort and dyspnea. He reports that just before the episode, he felt chest pressure and shortness of breath. He has no history of syncope and denies any prodromal symptoms such as nausea or diaphoresis. He also denies any recent head trauma, seizures, or neurological deficits.

PAST MEDICAL HISTORY:
Active Problem List:
- Aortic valve stenosis (diagnosed 2020-01-15)
- Solitary pulmonary nodule with high suspicion for malignancy (diagnosed 2020-04-15)
- Hypertension
- Hyperlipidemia
- Severe calcific aortic stenosis
- Tobacco use disorder (former)
- Abdominal aortic aneurysm
- Coronary artery disease
- Solitary pulmonary nodule under active surveillance

PAST SURGICAL HISTORY:
- Emergency ascending aortic replacement with valve-sparing root repair for Type A aortic dissection
- Endovascular repair of abdominal aortic aneurysm
- Percutaneous coronary intervention with drug-eluting stent to right coronary artery
- Exploratory laparotomy with primary repair of perforated sigmoid diverticulum

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68 (deceased)
- Mother: Cerebrovascular accident at age 75
- Brother: Abdominal aortic aneurysm (status post endovascular repair)

SOCIAL HISTORY:
Smoking: 50-pack-year history (1 pack per day for 50 years, quit 3 months prior). Alcohol: Social alcohol use (1-2 drinks per month). Occupation: Retired postal worker.

VITAL SIGNS:
On examination, his blood pressure is 145/90 mmHg, heart rate is 78 bpm, and oxygen saturation is 97% on room air.

PHYSICAL EXAMINATION:
Cardiac examination reveals a harsh crescendo-decrescendo systolic murmur best heard at the right upper sternal border that radiates to the neck. There are no carotid bruits, and the neurological examination is normal.

OTHER STUDIES:
His EKG shows left ventricular hypertrophy with no acute ST or T wave changes.

## Encounter 3: source 2021-01-15, practice 2021-11-16, outpatient, Cardiology

Chief complaint: Worsening chest pain on exertion, episodes of dizziness, and recent syncope while getting out of bed

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Elena Vasquez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening chest pain on exertion, episodes of dizziness, and recent syncope while getting out of bed

HISTORY OF PRESENT ILLNESS:
The patient with severe calcific aortic stenosis returns following prior ED visit for exertional syncope. He presents to the clinic with complaints of chest pain on exertion and episodes of dizziness, especially when climbing stairs. He reports a recent episode of syncope while getting out of bed.

PAST MEDICAL HISTORY:
Active Problem List:
- Aortic valve stenosis (diagnosed 2020-01-15)
- Solitary pulmonary nodule with high suspicion for malignancy (diagnosed 2020-04-15)
- Hypertension
- Hyperlipidemia
- Severe calcific aortic stenosis
- Tobacco use disorder (former)
- Abdominal aortic aneurysm
- Coronary artery disease
- Solitary pulmonary nodule under active surveillance

He has a history of hypertension and hyperlipidemia, both controlled with medication.

PAST SURGICAL HISTORY:
- Emergency ascending aortic replacement with valve-sparing root repair for Type A aortic dissection
- Endovascular repair of abdominal aortic aneurysm
- Percutaneous coronary intervention with drug-eluting stent to right coronary artery
- Exploratory laparotomy with primary repair of perforated sigmoid diverticulum

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68 (deceased)
- Mother: Cerebrovascular accident at age 75
- Brother: Abdominal aortic aneurysm (status post endovascular repair)

SOCIAL HISTORY:
Smoking: 50-pack-year history (1 pack per day for 50 years, quit 3 months prior). Alcohol: Social alcohol use (1-2 drinks per month). Occupation: Retired postal worker.

VITAL SIGNS:
On examination, his blood pressure is 130/80 mmHg, heart rate is 78 bpm, and his BMI is 28.

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a harsh, late-peaking, crescendo-decrescendo systolic murmur best heard at the right upper sternal border. The murmur radiates to the carotids, and the carotid pulses are diminished and delayed. The S2 heart sound is soft and single.

OTHER STUDIES:
An EKG shows left ventricular hypertrophy.

## Encounter 4: source 2022-01-15, practice 2022-11-17, ed, Emergency Medicine

Chief complaint: Sudden onset of severe, sharp chest pain radiating to back described as the worst pain ever experienced with brief syncope

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe, sharp chest pain radiating to back described as the worst pain ever experienced with brief syncope

HISTORY OF PRESENT ILLNESS:
The patient is a 67-year-old male with severe calcific aortic stenosis and prior exertional chest pain and syncope. He presents to the emergency department with sudden onset of severe, sharp chest pain that radiates to his back. He describes the pain as the worst he has ever experienced. He also reports a brief episode of syncope earlier in the day.

PAST MEDICAL HISTORY:
Active Problem List:
- Aortic valve stenosis (diagnosed 2020-01-15)
- Solitary pulmonary nodule with high suspicion for malignancy (diagnosed 2020-04-15)
- Hypertension
- Hyperlipidemia
- Severe calcific aortic stenosis
- Tobacco use disorder (former)
- Abdominal aortic aneurysm
- Coronary artery disease
- Solitary pulmonary nodule under active surveillance

PAST SURGICAL HISTORY:
- Emergency ascending aortic replacement with valve-sparing root repair for Type A aortic dissection
- Endovascular repair of abdominal aortic aneurysm
- Percutaneous coronary intervention with drug-eluting stent to right coronary artery
- Exploratory laparotomy with primary repair of perforated sigmoid diverticulum

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68 (deceased)
- Mother: Cerebrovascular accident at age 75
- Brother: Abdominal aortic aneurysm (status post endovascular repair)

SOCIAL HISTORY:
Smoking: 50-pack-year history (1 pack per day for 50 years, quit 3 months prior). Alcohol: Social alcohol use (1-2 drinks per month). Occupation: Retired postal worker.

VITAL SIGNS:
His blood pressure is 180/95 mmHg in the right arm and 160/90 mmHg in the left arm.

PHYSICAL EXAMINATION:
Physical examination reveals a diastolic murmur over the right second intercostal space and a diminished left radial pulse.

LABORATORY DATA:
Laboratory Values:
- Hemoglobin: 14.2 g/dL
- White blood cell count: 12,000 /μL
- Platelets: 250,000 /μL
- Serum creatinine: 1.1 mg/dL
- Troponin I: <0.04 ng/mL (normal)

IMAGING:
Chest X-ray demonstrates a widened mediastinum. A bedside transthoracic echocardiogram reveals an intimal flap in the ascending aorta.

OTHER STUDIES:
Electrocardiogram (ECG) shows no evidence of ischemia or infarction.

## Encounter 5: source 2023-01-15, practice 2023-11-27, ed, Emergency Medicine

Chief complaint: Sudden onset of severe upper abdominal pain radiating to back with tearing sensation

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe upper abdominal pain radiating to back with tearing sensation

HISTORY OF PRESENT ILLNESS:
The patient with known abdominal aortic aneurysm and prior dissection describes the pain as a tearing sensation. He denies chest pain, shortness of breath, or lower extremity pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Aortic valve stenosis (diagnosed 2020-01-15)
- Solitary pulmonary nodule with high suspicion for malignancy (diagnosed 2020-04-15)
- Type A Aortic Dissection (diagnosed 2022-01-15)
- Hypertension
- Hyperlipidemia
- Severe calcific aortic stenosis
- Tobacco use disorder (former)
- Abdominal aortic aneurysm
- Coronary artery disease
- Solitary pulmonary nodule under active surveillance

PAST SURGICAL HISTORY:
- Emergency ascending aortic replacement with valve-sparing root repair for Type A aortic dissection
- Endovascular repair of abdominal aortic aneurysm
- Percutaneous coronary intervention with drug-eluting stent to right coronary artery
- Exploratory laparotomy with primary repair of perforated sigmoid diverticulum

MEDICATIONS:
His medications include lisinopril and atorvastatin.

Home Medications:
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68 (deceased)
- Mother: Cerebrovascular accident at age 75
- Brother: Abdominal aortic aneurysm (status post endovascular repair)

SOCIAL HISTORY:
Smoking: 50-pack-year history (1 pack per day for 50 years, quit 3 months prior). Alcohol: Social alcohol use (1-2 drinks per month). Occupation: Retired postal worker.

VITAL SIGNS:
On examination, his blood pressure is 150/90 mmHg, pulse is 80 beats per minute, and he is afebrile.

PHYSICAL EXAMINATION:
His abdomen is soft with mild tenderness in the upper quadrants without rebound or guarding. A pulsatile mass is palpable in the epigastric region.

LABORATORY DATA:
His labs are as follows:

- Hemoglobin: 14 g/dL
- White blood cell count: 7,000/mm^3
- Platelets: 250,000/mm^3
- Serum creatinine: 1.0 mg/dL
- Aspartate aminotransferase (AST): 30 U/L
- Alanine aminotransferase (ALT): 25 U/L

## Encounter 6: source 2024-01-15, practice 2024-11-28, ed, Emergency Medicine

Chief complaint: Severe chest pain beginning 2 hours ago at rest, constant heavy sensation in right subcostal area radiating to right shoulder with lightheadedness

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe chest pain beginning 2 hours ago at rest, constant heavy sensation in right subcostal area radiating to right shoulder with lightheadedness

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old male with severe calcific aortic stenosis, coronary artery disease, and prior emergency department presentations for Type A aortic dissection and symptomatic abdominal aortic aneurysm. He returns with severe chest pain that began 2 hours ago while he was resting. The pain is described as a constant, heavy sensation located in the right subcostal area, radiating to the right shoulder. He also reports feeling lightheaded and has noted shortness of breath.

PAST MEDICAL HISTORY:
Active Problem List:
- Aortic valve stenosis (diagnosed 2020-01-15)
- Solitary pulmonary nodule with high suspicion for malignancy (diagnosed 2020-04-15)
- Type A Aortic Dissection (diagnosed 2022-01-15)
- Symptomatic Abdominal Aortic Aneurysm (diagnosed 2023-01-15)
- Hypertension
- Hyperlipidemia
- Severe calcific aortic stenosis
- Tobacco use disorder (former)
- Abdominal aortic aneurysm
- Coronary artery disease
- Solitary pulmonary nodule under active surveillance

PAST SURGICAL HISTORY:
- Emergency ascending aortic replacement with valve-sparing root repair for Type A aortic dissection
- Endovascular repair of abdominal aortic aneurysm
- Percutaneous coronary intervention with drug-eluting stent to right coronary artery
- Exploratory laparotomy with primary repair of perforated sigmoid diverticulum

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68 (deceased)
- Mother: Cerebrovascular accident at age 75
- Brother: Abdominal aortic aneurysm (status post endovascular repair)

SOCIAL HISTORY:
Smoking: 50-pack-year history (1 pack per day for 50 years, quit 3 months prior). Alcohol: Social alcohol use (1-2 drinks per month). Occupation: Retired postal worker.

VITAL SIGNS:
On examination, his blood pressure is 85/50 mmHg, heart rate is 110 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
Jugular venous pressure is elevated. Heart sounds include a soft S1 and an S3 gallop. Lung fields are clear to auscultation. Extremities are cool, and there is no peripheral edema.

LABORATORY DATA:
Initial laboratory values are as follows:

- Troponin I: 8.0 ng/mL (normal <0.04 ng/mL)
- Creatinine: 1.1 mg/dL (normal 0.6-1.2 mg/dL)
- Hemoglobin: 14.5 g/dL (normal 13.8-17.2 g/dL)
- White blood cell count: 12,000 /µL (normal 4,500-11,000 /µL)

IMAGING:
The chest X-ray is unremarkable.

OTHER STUDIES:
An electrocardiogram (ECG) shows ST-segment elevations in leads II, III, and aVF with reciprocal changes in the anterior leads.

## Encounter 7: source 2025-01-15, practice 2025-11-19, ed, Emergency Medicine

Chief complaint: Dizziness and near-syncope without chest pain or shortness of breath

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Dizziness and near-syncope without chest pain or shortness of breath

HISTORY OF PRESENT ILLNESS:
This patient with severe aortic stenosis, dissection, AAA, and STEMI presents to the emergency department with complaints of dizziness and near-syncope. He denies chest pain, shortness of breath, or palpitations.

PAST MEDICAL HISTORY:
Active Problem List:
- Aortic valve stenosis (diagnosed 2020-01-15)
- Solitary pulmonary nodule with high suspicion for malignancy (diagnosed 2020-04-15)
- Type A Aortic Dissection (diagnosed 2022-01-15)
- Symptomatic Abdominal Aortic Aneurysm (diagnosed 2023-01-15)
- Acute ST-segment elevation myocardial infarction (STEMI) of inferior wall (diagnosed 2024-01-15)
- Hypertension
- Hyperlipidemia
- Severe calcific aortic stenosis
- Tobacco use disorder (former)
- Abdominal aortic aneurysm
- Coronary artery disease
- Solitary pulmonary nodule under active surveillance

PAST SURGICAL HISTORY:
- Emergency ascending aortic replacement with valve-sparing root repair for Type A aortic dissection
- Endovascular repair of abdominal aortic aneurysm
- Percutaneous coronary intervention with drug-eluting stent to right coronary artery
- Exploratory laparotomy with primary repair of perforated sigmoid diverticulum

MEDICATIONS:
He takes lisinopril and atorvastatin.

Home Medications:
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68 (deceased)
- Mother: Cerebrovascular accident at age 75
- Brother: Abdominal aortic aneurysm (status post endovascular repair)

SOCIAL HISTORY:
Smoking: 50-pack-year history (1 pack per day for 50 years, quit 3 months prior). Alcohol: Social alcohol use (1-2 drinks per month). Occupation: Retired postal worker.

VITAL SIGNS:
Blood pressure is 110/70 mmHg, heart rate is 38 beats per minute, respiratory rate is 16 breaths per minute, and oxygen saturation is 98% on room air

PHYSICAL EXAMINATION:
The patient is alert and oriented, but appears pale and diaphoretic. Cardiac examination reveals a slow, regular rhythm without murmurs, rubs, or gallops. Lung fields are clear to auscultation bilaterally.

LABORATORY DATA:
Initial laboratory results are as follows:

- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 102 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen (BUN): 18 mg/dL
- Creatinine: 1.0 mg/dL
- Glucose: 98 mg/dL
- Hemoglobin: 14 g/dL
- White blood cell count: 7,000/mm^3
- Platelet count: 250,000/mm^3

OTHER STUDIES:
An electrocardiogram (ECG) shows sinus bradycardia with no ST-segment changes or T-wave inversions.

PLAN:
Intravenous access is established, and the patient is placed on a cardiac monitor.
