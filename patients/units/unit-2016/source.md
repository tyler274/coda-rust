# synthetic_hospital patient 2016

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 57 at first encounter, sex M, Non-Hispanic White, Commercial preferred provider organization (PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2019-12-02 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2020-05-18 (Mon) | outpatient | Internal Medicine |
| 2 | 2021-01-15 | 2020-11-16 (Mon) | outpatient | Pulmonology |
| 3 | 2021-07-15 | 2021-05-31 (Mon) | outpatient | Pulmonology |
| 4 | 2022-07-15 | 2022-05-17 (Tue) | outpatient | Oncology |
| 5 | 2024-01-15 | 2023-11-27 (Mon) | outpatient | Gastroenterology |
| 6 | 2025-01-15 | 2024-11-20 (Wed) | ed | Emergency Medicine |
| 7 | 2026-01-15 | 2025-11-17 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial preferred provider organization (PPO)",
 "occupation": "Retired construction foreman",
 "alcohol_use": "Occasional social use (1-2 drinks per week)",
 "family_history": [
  "Father: Coronary artery disease, myocardial infarction at age 65",
  "Mother: Hypertension, type 2 diabetes",
  "Paternal uncle: Lung cancer diagnosed age 60",
  "No known family history of gastrointestinal malignancy"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker (40 pack-years), quit approximately 5 years ago at age 52",
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
   "dose": "40 mg PO nightly",
   "name": "Atorvastatin"
  },
  {
   "dose": "500 mg PO PRN for heartburn",
   "name": "Calcium carbonate (Tums)"
  }
 ],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia",
  "Gastroesophageal reflux disease (GERD)",
  "Tobacco use disorder, in sustained remission"
 ],
 "age_at_first_encounter": 57
}
```

Primary diagnoses: ["Solitary pulmonary nodule with high suspicion for malignancy", "Non-small cell lung cancer, right upper lobe", "Perforated peptic ulcer", "Suspected malignant neoplasm of lung", "Esophageal carcinoma", "Coagulative necrosis (secondary to acute myocardial infarction)"]
Comorbidities: ["Tobacco use disorder", "Tobacco use disorder", "Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2019-12-02, outpatient, Internal Medicine

Chief complaint: Routine check-up with concern regarding personal lung cancer risk due to 40 pack-year smoking history.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine check-up with concern regarding personal lung cancer risk due to 40 pack-year smoking history.

HISTORY OF PRESENT ILLNESS:
He has no complaints but is concerned about his risk of developing lung cancer due to his smoking habit. He denies any respiratory symptoms,

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- Gastroesophageal reflux disease (GERD)
- Tobacco use disorder, in sustained remission

His past medical history is significant for hypertension, controlled with lisinopril, and hyperlipidemia, managed with atorvastatin.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO nightly
- Calcium carbonate (Tums) 500 mg PO PRN for heartburn

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Coronary artery disease, myocardial infarction at age 65
- Mother: Hypertension, type 2 diabetes
- Paternal uncle: Lung cancer diagnosed age 60
- No known family history of gastrointestinal malignancy

SOCIAL HISTORY:
With a 30-pack-year smoking history

VITAL SIGNS:
His vital signs are within normal limits.

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

OTHER STUDIES:
A recent study in the clinic's population showed that the risk of lung cancer in smokers is 15% while in non-smokers it is 2%. Based on this information, calculate the attributable risk percentage (ARP) for lung cancer in smokers.

## Encounter 1: source 2020-07-15, practice 2020-05-18, outpatient, Internal Medicine

Chief complaint: Routine follow-up visit with heightened anxiety about lung cancer risk after multiple friends received diagnoses.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up visit with heightened anxiety about lung cancer risk after multiple friends received diagnoses.

HISTORY OF PRESENT ILLNESS:
The patient returns for routine follow-up six months after his prior visit, during which he expressed concern regarding personal lung cancer risk due to his 40 pack-year smoking history. He has no complaints. He mentions that several of his friends have recently been diagnosed with lung cancer, and he is concerned about his own risk. The patient inquires about the factors that may have contributed to his friends' diagnoses and whether he should be concerned about his own health.

PAST MEDICAL HISTORY:
Active Problem List:
- Suspected malignant neoplasm of lung (diagnosed 2020-01-15)
- Hypertension
- Hyperlipidemia
- Gastroesophageal reflux disease (GERD)
- Tobacco use disorder, in sustained remission

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes currently on hydrochlorothiazide for blood pressure control.

Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO nightly
- Calcium carbonate (Tums) 500 mg PO PRN for heartburn

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Coronary artery disease, myocardial infarction at age 65
- Mother: Hypertension, type 2 diabetes
- Paternal uncle: Lung cancer diagnosed age 60
- No known family history of gastrointestinal malignancy

SOCIAL HISTORY:
He has a 40-pack-year smoking history but quit smoking 5 years ago

VITAL SIGNS:
His blood pressure today is 138/86 mmHg

PHYSICAL EXAMINATION:
His physical examination is unremarkable

LABORATORY DATA:
Laboratory values:
- Hemoglobin: 14.2 g/dL
- WBC count: 7.2 x 10^9/L
- Platelets: 250 x 10^9/L
- Fasting glucose: 98 mg/dL
- Total cholesterol: 190 mg/dL
- HDL cholesterol: 40 mg/dL
- LDL cholesterol: 120 mg/dL
- Triglycerides: 150 mg/dL

IMAGING:
His recent chest X-ray was normal

## Encounter 2: source 2021-01-15, practice 2020-11-16, outpatient, Pulmonology

Chief complaint: Annual routine check-up with no current respiratory symptoms; remains adherent to antihypertensive regimen.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Jennifer Park, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Annual routine check-up with no current respiratory symptoms; remains adherent to antihypertensive regimen.

HISTORY OF PRESENT ILLNESS:
He returns for follow-up regarding suspected malignancy. He reports no current symptoms and is adherent to his medication regimen.

PAST MEDICAL HISTORY:
Active Problem List:
- Suspected malignant neoplasm of lung (diagnosed 2020-01-15)
- Hypertension
- Hyperlipidemia
- Gastroesophageal reflux disease (GERD)
- Tobacco use disorder, in sustained remission

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes lisinopril and atorvastatin.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Calcium carbonate (Tums) 500 mg PO PRN for heartburn

ALLERGIES:
NKDA

FAMILY HISTORY:
No family history of cancer

SOCIAL HISTORY:
History of smoking

VITAL SIGNS:
Blood pressure is 132/84 mmHg, heart rate is 78 bpm, and respiratory rate is 16 breaths per minute

PHYSICAL EXAMINATION:
His lung examination is unremarkable.

LABORATORY DATA:
Hemoglobin: 14.2 g/dL
- White blood cell count: 7.2 x 10^3/µL
- Platelets: 250 x 10^3/µL
- Fasting glucose: 98 mg/dL
- Total cholesterol: 180 mg/dL
- LDL cholesterol: 95 mg/dL
- HDL cholesterol: 45 mg/dL
- Triglycerides: 150 mg/dL

IMAGING:
A low-dose computed tomography (CT) scan of the chest is performed as part of a lung cancer screening due to his smoking history, which reveals a 4 mm non-calcified nodule in the right lower lobe. The radiologist who interpreted the scan was aware of the patient's smoking history.

## Encounter 3: source 2021-07-15, practice 2021-05-31, outpatient, Pulmonology

Chief complaint: Follow-up evaluation for pulmonary nodule with new episode of coughing up small amount of blood-tinged sputum.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Jennifer Park, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up evaluation for pulmonary nodule with new episode of coughing up small amount of blood-tinged sputum.

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of his solitary pulmonary nodule. He has no complaints but mentions a recent episode of coughing up a small amount of blood-tinged sputum.

PAST MEDICAL HISTORY:
Active Problem List:
- Suspected malignant neoplasm of lung (diagnosed 2020-01-15)
- Solitary pulmonary nodule with high suspicion for malignancy (diagnosed 2021-01-15)
- Hypertension
- Hyperlipidemia
- Gastroesophageal reflux disease (GERD)
- Tobacco use disorder, in sustained remission

His past medical history is significant for hypertension, controlled with lisinopril, and hypercholesterolemia, managed with atorvastatin.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO nightly
- Calcium carbonate (Tums) 500 mg PO PRN for heartburn

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Coronary artery disease, myocardial infarction at age 65
- Mother: Hypertension, type 2 diabetes
- Paternal uncle: Lung cancer diagnosed age 60
- No known family history of gastrointestinal malignancy

SOCIAL HISTORY:
History of smoking

REVIEW OF SYSTEMS:
He denies any weight loss, night sweats, or fever.

VITAL SIGNS:
On physical examination, his blood pressure is 132/84 mmHg, heart rate is 78 beats per minute, respiratory rate is 16 breaths per minute, and oxygen saturation is 97% on room air

PHYSICAL EXAMINATION:
Lung auscultation reveals no abnormalities.

IMAGING:
A chest X-ray is performed, which shows a 2 cm solitary pulmonary nodule in the right upper lobe. There are no previous chest X-rays available for comparison.

## Encounter 4: source 2022-07-15, practice 2022-05-17, outpatient, Oncology

Chief complaint: Progressive swelling and pain in fingers and knees over two months with family noticing new digital clubbing.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Amara Okafor, MD
Department: Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive swelling and pain in fingers and knees over two months with family noticing new digital clubbing.

HISTORY OF PRESENT ILLNESS:
The patient returns for oncology consultation in the context of his prior pulmonary nodule surveillance and reports progressive swelling and pain in his fingers and knees over the past two months. He also mentions that his family has noticed that the tips of his fingers appear to be more rounded than usual.

PAST MEDICAL HISTORY:
Active Problem List:
- Suspected malignant neoplasm of lung (diagnosed 2020-01-15)
- Solitary pulmonary nodule with high suspicion for malignancy (diagnosed 2021-01-15)
- Hypertension
- Hyperlipidemia
- Gastroesophageal reflux disease (GERD)
- Tobacco use disorder, in sustained remission

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO nightly
- Calcium carbonate (Tums) 500 mg PO PRN for heartburn

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Coronary artery disease, myocardial infarction at age 65
- Mother: Hypertension, type 2 diabetes
- Paternal uncle: Lung cancer diagnosed age 60
- No known family history of gastrointestinal malignancy

SOCIAL HISTORY:
40-pack-year smoking history

REVIEW OF SYSTEMS:
He denies any fever, weight loss, or respiratory symptoms.

PHYSICAL EXAMINATION:
Clubbing of the fingers and bilateral knee effusions without overlying erythema or warmth. His lungs are clear to auscultation, and the rest of the examination is unremarkable.

IMAGING:
Chest x-ray, which reveals a 3 cm mass in the right upper lobe of the lung

## Encounter 5: source 2024-01-15, practice 2023-11-27, outpatient, Gastroenterology

Chief complaint: Six-month history of progressive dysphagia, unintentional weight loss of 10 kg, and persistent cough worsening after meals.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. James Thompson, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of progressive dysphagia, unintentional weight loss of 10 kg, and persistent cough worsening after meals.

HISTORY OF PRESENT ILLNESS:
The patient with lung cancer and tobacco use disorder in remission, with prior pulmonary evaluation, presents to Gastroenterology. He denies any history of heartburn but mentions occasional regurgitation of undigested food, particularly when lying down, and night sweats and mild hoarseness over the past 2 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Suspected malignant neoplasm of lung (diagnosed 2020-01-15)
- Solitary pulmonary nodule with high suspicion for malignancy (diagnosed 2021-01-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2022-07-15)
- Hypertension
- Hyperlipidemia
- Gastroesophageal reflux disease (GERD)
- Tobacco use disorder, in sustained remission

His medical history includes hypertension.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO nightly
- Calcium carbonate (Tums) 500 mg PO PRN for heartburn

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Coronary artery disease, myocardial infarction at age 65
- Mother: Hypertension, type 2 diabetes
- Paternal uncle: Lung cancer diagnosed age 60
- No known family history of gastrointestinal malignancy

SOCIAL HISTORY:
He is a former smoker with a 40-pack-year history

PHYSICAL EXAMINATION:
On physical examination, there is mild cervical lymphadenopathy and tenderness in the upper abdomen.

IMAGING:
Barium swallow shows a smooth narrowing of the lower esophagus,

PATHOLOGY:
Biopsy results shows neoplasia with eosinophilic pearls and intercellular bridges.

OTHER STUDIES:
Endoscopy reveals an unusual smooth mass located in the distal esophagus.

## Encounter 6: source 2025-01-15, practice 2024-11-20, ed, Emergency Medicine

Chief complaint: Sudden-onset severe epigastric abdominal pain radiating to the back with associated nausea for two hours.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Lisa Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset severe epigastric abdominal pain radiating to the back with associated nausea for two hours.

HISTORY OF PRESENT ILLNESS:
The patient has a history of esophageal carcinoma and non-small cell lung cancer. He describes the pain as sharp and constant, located in the epigastric region, and radiating to the back. He reports nausea but no vomiting.

PAST MEDICAL HISTORY:
Active Problem List:
- Suspected malignant neoplasm of lung (diagnosed 2020-01-15)
- Solitary pulmonary nodule with high suspicion for malignancy (diagnosed 2021-01-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2022-07-15)
- Esophageal carcinoma (diagnosed 2024-01-15)
- Hypertension
- Hyperlipidemia
- Gastroesophageal reflux disease (GERD)
- Tobacco use disorder, in sustained remission

His past medical history includes hypertension, for which he takes lisinopril, and a 10-year history of intermittent burning epigastric pain, usually relieved by antacids.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO nightly
- Calcium carbonate (Tums) 500 mg PO PRN for heartburn

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Coronary artery disease, myocardial infarction at age 65
- Mother: Hypertension, type 2 diabetes
- Paternal uncle: Lung cancer diagnosed age 60
- No known family history of gastrointestinal malignancy

SOCIAL HISTORY:
He smokes half a pack of cigarettes per day and consumes alcohol occasionally.

VITAL SIGNS:
On examination, his vital signs are: blood pressure 130/85 mmHg, heart rate 110 bpm, respiratory rate 22 breaths/min, and temperature 38.1°C (100.6°F).

PHYSICAL EXAMINATION:
Abdominal examination reveals diffuse tenderness, rebound tenderness, and guarding. Bowel sounds are hypoactive.

LABORATORY DATA:
Laboratory values are as follows:

- White blood cell count: 16,000/mm³
- Hemoglobin: 13.5 g/dL
- Platelet count: 250,000/mm³
- Serum amylase: 90 U/L (normal range: 30-110 U/L)
- Serum lipase: 45 U/L (normal range: 0-160 U/L)

IMAGING:
An upright chest x-ray shows subdiaphragmatic free air.

## Encounter 7: source 2026-01-15, practice 2025-11-17, ed, Emergency Medicine

Chief complaint: Sudden onset of severe chest pain radiating to left arm beginning one hour ago while at rest.

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. Robert Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe chest pain radiating to left arm beginning one hour ago while at rest.

HISTORY OF PRESENT ILLNESS:
The patient is a 63-year-old male with a history of non-small cell lung cancer, esophageal carcinoma, and a recent perforated peptic ulcer who presents with sudden onset of severe chest pain radiating to the left arm. The pain began one hour ago while he was at rest.

PAST MEDICAL HISTORY:
Active Problem List:
- Suspected malignant neoplasm of lung (diagnosed 2020-01-15)
- Solitary pulmonary nodule with high suspicion for malignancy (diagnosed 2021-01-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2022-07-15)
- Esophageal carcinoma (diagnosed 2024-01-15)
- Perforated peptic ulcer (diagnosed 2025-01-15)
- Hypertension
- Hyperlipidemia
- Gastroesophageal reflux disease (GERD)
- Tobacco use disorder, in sustained remission

His medical history includes hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO nightly
- Calcium carbonate (Tums) 500 mg PO PRN for heartburn

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Coronary artery disease, myocardial infarction at age 65
- Mother: Hypertension, type 2 diabetes
- Paternal uncle: Lung cancer diagnosed age 60
- No known family history of gastrointestinal malignancy

SOCIAL HISTORY:
He is a smoker with a 40-pack-year history.

VITAL SIGNS:
On examination, blood pressure is 150/92 mmHg and pulse is 98/min.

PHYSICAL EXAMINATION:
He appears diaphoretic and anxious.

LABORATORY DATA:
Laboratory evaluation is performed.
Troponin I Elevated
CK-MB                                     Elevated

PATHOLOGY:
Autopsy of a patient who dies 2 days after such an event would most likely show

OTHER STUDIES:
ECG shows ST-segment elevations in leads II, III, and aVF.
