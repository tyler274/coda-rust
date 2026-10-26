# synthetic_hospital patient 2017

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 55 at first encounter, sex M, Non-Hispanic White, Medicare (disability) and private supplemental.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-10-31 (Mon) | outpatient | Pulmonary & Critical Care |
| 1 | 2020-07-15 | 2023-05-11 (Thu) | outpatient | Cardiology |
| 2 | 2021-01-15 | 2023-11-17 (Fri) | outpatient | Internal Medicine |
| 3 | 2021-07-15 | 2024-05-10 (Fri) | outpatient | Pulmonary & Critical Care |
| 4 | 2022-01-15 | 2024-11-11 (Mon) | outpatient | Hematology & Oncology |
| 5 | 2022-07-15 | 2025-05-05 (Mon) | outpatient | Hematology & Oncology |
| 6 | 2023-01-15 | 2025-11-12 (Wed) | ed | Emergency Medicine |
| 7 | 2023-07-15 | 2026-05-02 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare (disability) and private supplemental",
 "occupation": "Former construction foreman, currently on medical leave",
 "alcohol_use": "Occasional social use, 1-2 drinks per month",
 "family_history": [
  "Father: Lung cancer, deceased age 60",
  "Mother: Hypertension, age 80",
  "Paternal uncle: Colon cancer"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 80 pack-year history, quit within the last month",
 "home_medications": [
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
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
  "Inguinal hernia repair (age 45)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia",
  "Peripheral artery disease",
  "Severe upper lobe predominant emphysema (COPD)"
 ],
 "age_at_first_encounter": 55
}
```

Primary diagnoses: ["Superior vena cava syndrome secondary to lung cancer", "Stage IIIB Non-Small Cell Lung Cancer", "Tracheal neoplasm", "Colorectal neoplasia", "Renal artery stenosis", "Severe upper lobe predominant emphysema", "Metastatic carcinoma to brain"]
Comorbidities: ["Tobacco use disorder", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2022-10-31, outpatient, Pulmonary & Critical Care

Chief complaint: Follow-up appointment after recent pulmonary function testing

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonary & Critical Care
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up appointment after recent pulmonary function testing

HISTORY OF PRESENT ILLNESS:
He reports gradually worsening shortness of breath over the past year, which he initially attributed to aging. He now finds it difficult to walk several blocks without stopping to catch his breath. He denies cough or sputum production but admits to occasionally using his wife’s albuterol inhaler for relief.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- Peripheral artery disease
- Severe upper lobe predominant emphysema (COPD)

PAST SURGICAL HISTORY:
- Inguinal hernia repair (age 45)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, deceased age 60
- Mother: Hypertension, age 80
- Paternal uncle: Colon cancer

SOCIAL HISTORY:
He has an 80-pack-year history of cigarette smoking and quit only recently.

PHYSICAL EXAMINATION:
Physical examination reveals decreased breath sounds bilaterally and hyperresonance to percussion. The chest wall has increased anteroposterior diameter, and accessory muscle use is noted during quiet breathing. Cardiopulmonary examination reveals a loud P2. There is mild peripheral edema.

OTHER STUDIES:
Pulmonary function tests show a reduced FEV1/FVC ratio of 55%. Total lung capacity and residual volume are both elevated.

## Encounter 1: source 2020-07-15, practice 2023-05-11, outpatient, Cardiology

Chief complaint: Progressively worsening hypertension

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Miller, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressively worsening hypertension

HISTORY OF PRESENT ILLNESS:
The patient is a 55-year-old male with hypertension, hyperlipidemia, peripheral artery disease, and severe upper lobe predominant emphysema who presents with progressively worsening hypertension following his January 2020 pulmonary evaluation. He has a 12-year history of well-controlled high blood pressure managed with hydrochlorothiazide. However, over the past 6 months, his systolic blood pressure has increased to 180–190 mm Hg, requiring the recent addition of a second antihypertensive agent.

PAST MEDICAL HISTORY:
Active Problem List:
- Severe upper lobe predominant emphysema (diagnosed 2020-01-15)
- Hypertension
- Hyperlipidemia
- Peripheral artery disease
- Severe upper lobe predominant emphysema (COPD)

His medical history includes smoking and peripheral artery disease.

PAST SURGICAL HISTORY:
He underwent a femoral artery stent placement 2 years ago

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, deceased age 60
- Mother: Hypertension, age 80
- Paternal uncle: Colon cancer

SOCIAL HISTORY:
Smoking: Former smoker, 80 pack-year history, quit within the last month. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Former construction foreman, currently on medical leave.

REVIEW OF SYSTEMS:
He denies headaches, palpitations, or muscle weakness.

LABORATORY DATA:
Laboratory testing shows a serum creatinine of 1.7 mg/dL (baseline 1.0) and markedly elevated plasma renin activity.

## Encounter 2: source 2021-01-15, practice 2023-11-17, outpatient, Internal Medicine

Chief complaint: Routine check-up

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Maria Garcia, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine check-up

HISTORY OF PRESENT ILLNESS:
The patient returns for routine outpatient follow-up approximately six months after cardiology evaluation for renal artery stenosis and ongoing management of severe upper lobe predominant emphysema, hypertension, hyperlipidemia, and peripheral artery disease. He reports no complaints. He states he has been participating in a local health program that encourages a plant-based diet and regular exercise. He proudly reports a 10-pound weight loss over the past 6 months. He expresses interest in the results of a recent observational study that suggests a significant reduction in the incidence of stroke among participants who follow a similar lifestyle program, though he voices concern regarding the validity of the study's findings.

PAST MEDICAL HISTORY:
Active Problem List:
- Severe upper lobe predominant emphysema (diagnosed 2020-01-15)
- Renal artery stenosis (diagnosed 2020-07-15)
- Hypertension
- Hyperlipidemia
- Peripheral artery disease
- Severe upper lobe predominant emphysema (COPD)

PAST SURGICAL HISTORY:
- Inguinal hernia repair (age 45)

MEDICATIONS:
He takes currently taking hydrochlorothiazide.

Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, deceased age 60
- Mother: Hypertension, age 80
- Paternal uncle: Colon cancer

SOCIAL HISTORY:
Smoking: Former smoker, 80 pack-year history, quit within the last month. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Former construction foreman, currently on medical leave.

VITAL SIGNS:
His blood pressure is 138/86 mmHg. His current BMI is 27 kg/m^2.

LABORATORY DATA:
Laboratory Values:
- Total cholesterol: 190 mg/dL
- HDL cholesterol: 40 mg/dL
- LDL cholesterol: 120 mg/dL
- Triglycerides: 150 mg/dL
- Fasting blood glucose: 98 mg/dL

## Encounter 3: source 2021-07-15, practice 2024-05-10, outpatient, Pulmonary & Critical Care

Chief complaint: Progressive dyspnea, hoarseness, and a cough

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonary & Critical Care
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive dyspnea, hoarseness, and a cough

HISTORY OF PRESENT ILLNESS:
The patient returns to Pulmonary and Critical Care with severe upper lobe predominant emphysema and presents with a 3-month history of progressive dyspnea, hoarseness, and a cough. He denies any fever, weight loss, or night sweats.

PAST MEDICAL HISTORY:
Active Problem List:
- Severe upper lobe predominant emphysema (diagnosed 2020-01-15)
- Renal artery stenosis (diagnosed 2020-07-15)
- Hypertension
- Hyperlipidemia
- Peripheral artery disease
- Severe upper lobe predominant emphysema (COPD)

His medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Inguinal hernia repair (age 45)

MEDICATIONS:
He takes controlled with lisinopril.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, deceased age 60
- Mother: Hypertension, age 80
- Paternal uncle: Colon cancer

SOCIAL HISTORY:
With a history of smoking
30-pack-year smoking history

PHYSICAL EXAMINATION:
On physical examination, you note stridor that is present both during inspiration and expiration. His voice is raspy, and there is no palpable lymphadenopathy.

OTHER STUDIES:
Pulmonary function tests reveal a flow-volume loop with flattening of both the inspiratory and expiratory limbs.

## Encounter 4: source 2022-01-15, practice 2024-11-11, outpatient, Hematology & Oncology

Chief complaint: Progressive fatigue and shortness of breath on exertion

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Aisha Patel, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue and shortness of breath on exertion

HISTORY OF PRESENT ILLNESS:
This 57-year-old male with severe upper lobe predominant emphysema and recent evaluation for tracheal pathology presents to the Hematology and Oncology clinic with a 3-month history of progressive fatigue and shortness of breath on exertion. He reports a 10-pound unintentional weight loss over the same period but denies any fever, night sweats, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Severe upper lobe predominant emphysema (diagnosed 2020-01-15)
- Renal artery stenosis (diagnosed 2020-07-15)
- Tracheal neoplasm (diagnosed 2021-07-15)
- Hypertension
- Hyperlipidemia
- Peripheral artery disease
- Severe upper lobe predominant emphysema (COPD)

His past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Inguinal hernia repair (age 45)

MEDICATIONS:
He takes controlled with lisinopril.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, deceased age 60
- Mother: Hypertension, age 80
- Paternal uncle: Colon cancer

SOCIAL HISTORY:
He has a 30-pack-year smoking history

PHYSICAL EXAMINATION:
On physical examination, he appears pale and has conjunctival pallor. His stool is guaiac positive.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 9.2 g/dL (normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 72 fL (normal: 80-96 fL)
- Serum ferritin: 15 ng/mL (normal: 24-336 ng/mL)
- Serum iron: 30 µg/dL (normal: 65-176 µg/dL)
- Total iron-binding capacity (TIBC): 450 µg/dL (normal: 250-450 µg/dL)

## Encounter 5: source 2022-07-15, practice 2025-05-05, outpatient, Hematology & Oncology

Chief complaint: Chronic cough and recent weight loss

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Aisha Patel, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Chronic cough and recent weight loss

HISTORY OF PRESENT ILLNESS:
The patient with severe emphysema returns for chronic cough and weight loss, reporting a cough productive of yellowish sputum for the past 3 months and denying fever, night sweats, or hemoptysis.

PAST MEDICAL HISTORY:
Active Problem List:
- Severe upper lobe predominant emphysema (diagnosed 2020-01-15)
- Renal artery stenosis (diagnosed 2020-07-15)
- Tracheal neoplasm (diagnosed 2021-07-15)
- Colorectal neoplasia (diagnosed 2022-01-15)
- Hypertension
- Hyperlipidemia
- Peripheral artery disease
- Severe upper lobe predominant emphysema (COPD)

PAST SURGICAL HISTORY:
- Inguinal hernia repair (age 45)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, deceased age 60
- Mother: Hypertension, age 80
- Paternal uncle: Colon cancer

SOCIAL HISTORY:
History of smoking

VITAL SIGNS:
His blood pressure is 145/90 mmHg, heart rate is 88 beats per minute, and respiratory rate is 18 breaths per minute.

PHYSICAL EXAMINATION:
Physical examination reveals decreased breath sounds and dullness to percussion at the right lung base.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin: 13.5 g/dL
- White blood cell count: 7,500/mm^3
- Platelets: 250,000/mm^3
- Serum sodium: 140 mEq/L
- Serum potassium: 4.2 mEq/L
- Serum creatinine: 1.0 mg/dL
- Fasting glucose: 100 mg/dL

IMAGING:
Chest X-ray shows a mass in the right lower lobe.

PATHOLOGY:
Histopathology confirms non-small cell lung cancer (NSCLC).

OTHER STUDIES:
A case-control study is initiated to identify potential risk factors for NSCLC. Researchers select 100 patients with newly diagnosed NSCLC as cases and 100 age-matched controls without lung cancer from the same geographic area. They collect data on smoking history, occupational exposures, and family history of cancer.

PLAN:
The patient is referred to a pulmonologist who performs a bronchoscopy and biopsies the mass.

## Encounter 6: source 2023-01-15, practice 2025-11-12, ed, Emergency Medicine

Chief complaint: Intermittent, dull abdominal pain over the past 3 months

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Michael Roberts, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Intermittent, dull abdominal pain over the past 3 months

HISTORY OF PRESENT ILLNESS:
This patient with Stage IIIB lung cancer presents with abdominal pain. The pain is poorly localized, occurs more often after meals, and occasionally radiates to the back.

PAST MEDICAL HISTORY:
Active Problem List:
- Severe upper lobe predominant emphysema (diagnosed 2020-01-15)
- Renal artery stenosis (diagnosed 2020-07-15)
- Tracheal neoplasm (diagnosed 2021-07-15)
- Colorectal neoplasia (diagnosed 2022-01-15)
- Stage IIIB Non-Small Cell Lung Cancer (diagnosed 2022-07-15)
- Hypertension
- Hyperlipidemia
- Peripheral artery disease
- Severe upper lobe predominant emphysema (COPD)

His medical history includes hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Inguinal hernia repair (age 45)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, deceased age 60
- Mother: Hypertension, age 80
- Paternal uncle: Colon cancer

SOCIAL HISTORY:
He has smoked one pack of cigarettes daily for 35 years.

REVIEW OF SYSTEMS:
He has no nausea, vomiting, diarrhea, or hematochezia.

PHYSICAL EXAMINATION:
Physical examination reveals a pulsatile mass in the central abdomen without tenderness. Bowel sounds are normal.

IMAGING:
Abdominal CT angiography demonstrates a saccular aneurysm arising from the proximal superior mesenteric artery just distal to its origin from the aorta. The aneurysm measures 3.8 cm in diameter and projects posteriorly.

ASSESSMENT AND PLAN:
Based on its location, this aneurysm is most likely to compress and impair venous drainage from

## Encounter 7: source 2023-07-15, practice 2026-05-02, ed, Emergency Medicine

Chief complaint: New-onset seizure

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Jennifer Wong, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
New-onset seizure

HISTORY OF PRESENT ILLNESS:
The patient with Stage IIIB non-small cell lung cancer and prior SVC syndrome presents with new-onset seizure. His wife reports that he has had a persistent headache for the past two weeks, which has not improved with over-the-counter analgesics. She also mentions that he has been increasingly forgetful and had difficulty finding words during conversations.

PAST MEDICAL HISTORY:
Active Problem List:
- Severe upper lobe predominant emphysema (diagnosed 2020-01-15)
- Renal artery stenosis (diagnosed 2020-07-15)
- Tracheal neoplasm (diagnosed 2021-07-15)
- Colorectal neoplasia (diagnosed 2022-01-15)
- Stage IIIB Non-Small Cell Lung Cancer (diagnosed 2022-07-15)
- Superior vena cava syndrome secondary to lung cancer (diagnosed 2023-01-15)
- Hypertension
- Hyperlipidemia
- Peripheral artery disease
- Severe upper lobe predominant emphysema (COPD)

PAST SURGICAL HISTORY:
- Inguinal hernia repair (age 45)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, deceased age 60
- Mother: Hypertension, age 80
- Paternal uncle: Colon cancer

SOCIAL HISTORY:
40-pack-year smoking

VITAL SIGNS:
His vital signs are stable.

PHYSICAL EXAMINATION:
On examination, the patient is postictal but has no evident focal neurologic deficits.

IMAGING:
An MRI of the brain is performed and reveals multiple, well-circumscribed lesions at the gray and white matter junction with surrounding vasogenic edema.
