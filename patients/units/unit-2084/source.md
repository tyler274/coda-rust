# synthetic_hospital patient 2084

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Medicaid, limited rural access.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-11-14 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2023-01-29 (Sun) | ed | Emergency Medicine |
| 2 | 2020-07-15 | 2023-05-15 (Mon) | outpatient | Internal Medicine |
| 3 | 2021-01-15 | 2023-11-15 (Wed) | outpatient | Pulmonology |
| 4 | 2021-07-15 | 2024-05-06 (Mon) | outpatient | Oncology |
| 5 | 2022-01-15 | 2024-11-12 (Tue) | inpatient | General Surgery |
| 6 | 2022-07-15 | 2025-05-16 (Fri) | inpatient | Oncology |
| 7 | 2023-01-15 | 2025-11-16 (Sun) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid, limited rural access",
 "occupation": "Construction worker (heavy equipment operator)",
 "alcohol_use": "Regular use, 3-4 drinks daily, primarily beer",
 "family_history": [
  "Father: Lung cancer (diagnosed age 52)",
  "Mother: Hypertension, Type 2 diabetes",
  "Older brother: Aortic aneurysm"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Active smoker, 60 pack-year history (2 packs per day since age 15)",
 "home_medications": [
  {
   "dose": "20 mg PO BID",
   "name": "Omeprazole"
  },
  {
   "dose": "10 mg PO daily (sporadely taken)",
   "name": "Lisinopril"
  },
  {
   "dose": "600 mg PO PRN back pain",
   "name": "Ibuprofen"
  },
  {
   "dose": "1 tablet PO daily (when remembered)",
   "name": "Multivitamin"
  }
 ],
 "surgical_history": [
  "Ureteroscopic stone removal (right kidney), 2 months prior"
 ],
 "chronic_conditions": [
  "Gastroesophageal reflux disease",
  "Hypertension (poorly controlled)",
  "Primary hyperparathyroidism",
  "History of calcium nephrolithiasis",
  "Nicotine dependence",
  "Protein-energy malnutrition with niacin deficiency",
  "Chronic obstructive pulmonary disease (GOLD Stage II)"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Non-small cell lung cancer, right upper lobe", "Primary hyperparathyroidism", "Perforated peptic ulcer disease", "Malignant neoplasm of lymph nodes", "Pellagra (Niacin deficiency)", "Aortic dissection"]
Comorbidities: ["Urolithiasis (Renal Colic)", "Dehydration", "Tobacco use disorder"]

## Encounter 0: source 2020-01-15, practice 2022-11-14, outpatient, Internal Medicine

Chief complaint: Worsening confusion, skin changes, diarrhea, and weight loss brought in by sister

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening confusion, skin changes, diarrhea, and weight loss brought in by sister

HISTORY OF PRESENT ILLNESS:
She reports that he has become increasingly irritable and forgetful over the past several weeks and has difficulty forming coherent sentences. He has also had persistent diarrhea for about a month, which he has attributed to “bad food.”

PAST MEDICAL HISTORY:
Active Problem List:
- Gastroesophageal reflux disease
- Hypertension (poorly controlled)
- Primary hyperparathyroidism
- History of calcium nephrolithiasis
- Nicotine dependence
- Protein-energy malnutrition with niacin deficiency
- Chronic obstructive pulmonary disease (GOLD Stage II)

His past medical history is unknown.

PAST SURGICAL HISTORY:
- Ureteroscopic stone removal (right kidney), 2 months prior

MEDICATIONS:
Home Medications:
- Omeprazole 20 mg PO BID
- Lisinopril 10 mg PO daily (sporadely taken)
- Ibuprofen 600 mg PO PRN back pain
- Multivitamin 1 tablet PO daily (when remembered)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 52)
- Mother: Hypertension, Type 2 diabetes
- Older brother: Aortic aneurysm

SOCIAL HISTORY:
His diet consists mostly of cornmeal, and he lives alone in a rural area with minimal access to medical care.

PHYSICAL EXAMINATION:
On physical examination, he appears disoriented and has dry mucous membranes. His skin shows well-demarcated, hyperpigmented, scaly lesions in a bilateral distribution on the dorsal surfaces of his hands, forearms, and around his neck, where the pattern resembles a collar. The affected areas are rough and cracked but not vesicular or blistering. His neurologic exam reveals a broad-based gait and slowed responses.

LABORATORY DATA:
Laboratory testing shows mild normocytic anemia but no leukocytosis. Liver and renal function tests are normal.

IMAGING:
Head CT is unremarkable.

## Encounter 1: source 2020-04-15, practice 2023-01-29, ed, Emergency Medicine

Chief complaint: Sudden onset excruciating left groin pain radiating from back

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset excruciating left groin pain radiating from back

HISTORY OF PRESENT ILLNESS:
The patient is a 45-year-old male with primary hyperparathyroidism and history of calcium nephrolithiasis who was previously evaluated in January for pellagra. He presents with pain that he said started at his back and has migrated downward.

PAST MEDICAL HISTORY:
Active Problem List:
- Pellagra (Niacin deficiency) (diagnosed 2020-01-15)
- Gastroesophageal reflux disease
- Hypertension (poorly controlled)
- Primary hyperparathyroidism
- History of calcium nephrolithiasis
- Nicotine dependence
- Protein-energy malnutrition with niacin deficiency
- Chronic obstructive pulmonary disease (GOLD Stage II)

PAST SURGICAL HISTORY:
- Ureteroscopic stone removal (right kidney), 2 months prior

MEDICATIONS:
Home Medications:
- Omeprazole 20 mg PO BID
- Lisinopril 10 mg PO daily (sporadely taken)
- Ibuprofen 600 mg PO PRN back pain
- Multivitamin 1 tablet PO daily (when remembered)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 52)
- Mother: Hypertension, Type 2 diabetes
- Older brother: Aortic aneurysm

SOCIAL HISTORY:
Smoking: Active smoker, 60 pack-year history (2 packs per day since age 15). Alcohol: Regular use, 3-4 drinks daily, primarily beer. Occupation: Construction worker (heavy equipment operator).

VITAL SIGNS:
Vitals are normal.

LABORATORY DATA:
Urinalysis shows trace RBCs and no protein. Laboratory studies show a serum calcium of 10.5 mg/dL and phosphate of 2.1 mg/dL. Antinuclear antibody is negative.

## Encounter 2: source 2020-07-15, practice 2023-05-15, outpatient, Internal Medicine

Chief complaint: Painless lump on neck discovered 2 weeks ago, requesting cancer screening

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Emily Johnson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Painless lump on neck discovered 2 weeks ago, requesting cancer screening

HISTORY OF PRESENT ILLNESS:
He returns with a neck lump following treatment for pellagra and hyperparathyroidism. He has no history of fever, night sweats, or weight loss. The patient is concerned about the possibility of cancer and requests a screening test.

PAST MEDICAL HISTORY:
Active Problem List:
- Pellagra (Niacin deficiency) (diagnosed 2020-01-15)
- Primary hyperparathyroidism (diagnosed 2020-04-15)
- Gastroesophageal reflux disease
- Hypertension (poorly controlled)
- History of calcium nephrolithiasis
- Nicotine dependence
- Protein-energy malnutrition with niacin deficiency
- Chronic obstructive pulmonary disease (GOLD Stage II)

PAST SURGICAL HISTORY:
- Ureteroscopic stone removal (right kidney), 2 months prior

MEDICATIONS:
Home Medications:
- Omeprazole 20 mg PO BID
- Lisinopril 10 mg PO daily (sporadely taken)
- Ibuprofen 600 mg PO PRN back pain
- Multivitamin 1 tablet PO daily (when remembered)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 52)
- Mother: Hypertension, Type 2 diabetes
- Older brother: Aortic aneurysm

SOCIAL HISTORY:
He has a 30-pack-year smoking history and consumes alcohol regularly.

PHYSICAL EXAMINATION:
On physical examination, there is a 3 cm firm, non-tender lymph node on the right side of his neck. The rest of the examination is unremarkable.

IMAGING:
A chest X-ray shows no abnormalities.

PLAN:
The physician explains that a certain test has a sensitivity of 90% and a specificity of 95% for detecting the type of cancer that is a concern based on his risk factors. The prevalence of this cancer in the general population is 0.5%, but in individuals with his risk profile, it is 5%.

## Encounter 3: source 2021-01-15, practice 2023-11-15, outpatient, Pulmonology

Chief complaint: Persistent cough and unexplained weight loss over 3 months

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. David Park, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent cough and unexplained weight loss over 3 months

HISTORY OF PRESENT ILLNESS:
The patient is a 46-year-old male with a history of nicotine dependence, primary hyperparathyroidism, and malignant neoplasm of lymph nodes who presents to the Pulmonology clinic for evaluation of new symptoms. He reports a persistent cough and unexplained weight loss over the past 3 months. He denies fever, night sweats, or hemoptysis.

PAST MEDICAL HISTORY:
Active Problem List:
- Pellagra (Niacin deficiency) (diagnosed 2020-01-15)
- Primary hyperparathyroidism (diagnosed 2020-04-15)
- Malignant neoplasm of lymph nodes (diagnosed 2020-07-15)
- Gastroesophageal reflux disease
- Hypertension (poorly controlled)
- History of calcium nephrolithiasis
- Nicotine dependence
- Protein-energy malnutrition with niacin deficiency
- Chronic obstructive pulmonary disease (GOLD Stage II)

PAST SURGICAL HISTORY:
- Ureteroscopic stone removal (right kidney), 2 months prior

MEDICATIONS:
Home Medications:
- Omeprazole 20 mg PO BID
- Lisinopril 10 mg PO daily (sporadely taken)
- Ibuprofen 600 mg PO PRN back pain
- Multivitamin 1 tablet PO daily (when remembered)

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is significant for lung cancer in his father.

SOCIAL HISTORY:
History of smoking

PHYSICAL EXAMINATION:
On physical examination, the patient appears cachectic. Lung auscultation reveals decreased breath sounds in the right upper lobe.

IMAGING:
A chest X-ray shows a mass in the right upper lobe, and a subsequent CT scan confirms the presence of a 4 cm mass with mediastinal lymphadenopathy.

PATHOLOGY:
A biopsy of the mass is performed, and the pathology report indicates non-small cell lung cancer (NSCLC).

OTHER STUDIES:
The patient is enrolled in a case-control study to investigate the association between a specific genetic mutation and the development of NSCLC. The study includes 100 cases with NSCLC and 100 controls without NSCLC. The presence of the genetic mutation is 60% in cases and 20% in controls.

## Encounter 4: source 2021-07-15, practice 2024-05-06, outpatient, Oncology

Chief complaint: Persistent cough and continued unintentional weight loss

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Maria Gonzalez, MD
Department: Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent cough and continued unintentional weight loss

HISTORY OF PRESENT ILLNESS:
He returns for oncology follow-up with history of pellagra, complaining of a persistent cough and unintentional weight loss over the past 3 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Pellagra (Niacin deficiency) (diagnosed 2020-01-15)
- Primary hyperparathyroidism (diagnosed 2020-04-15)
- Malignant neoplasm of lymph nodes (diagnosed 2020-07-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2021-01-15)
- Gastroesophageal reflux disease
- Hypertension (poorly controlled)
- History of calcium nephrolithiasis
- Nicotine dependence
- Protein-energy malnutrition with niacin deficiency
- Chronic obstructive pulmonary disease (GOLD Stage II)

PAST SURGICAL HISTORY:
- Ureteroscopic stone removal (right kidney), 2 months prior

MEDICATIONS:
Home Medications:
- Omeprazole 20 mg PO BID
- Lisinopril 10 mg PO daily (sporadely taken)
- Ibuprofen 600 mg PO PRN back pain
- Multivitamin 1 tablet PO daily (when remembered)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 52)
- Mother: Hypertension, Type 2 diabetes
- Older brother: Aortic aneurysm

SOCIAL HISTORY:
He has a 30-pack-year smoking history and works in construction

VITAL SIGNS:
On physical examination, he is afebrile, blood pressure is 130/85 mmHg, heart rate is 78 bpm, and respiratory rate is 16 breaths per minute

PHYSICAL EXAMINATION:
Lung auscultation reveals decreased breath sounds in the right upper lobe

IMAGING:
A chest X-ray shows a right upper lobe mass

PATHOLOGY:
A biopsy of the mass is performed, and the pathology report confirms small cell lung cancer

OTHER STUDIES:
The patient is enrolled in a case-control study to identify potential risk factors for small cell lung cancer. The study includes 100 patients with small cell lung cancer and 300 controls without lung cancer. Smoking is found to be significantly associated with small cell lung cancer, with an odds ratio of 20.

ASSESSMENT AND PLAN:
Given the rarity of small cell lung cancer in the general population,

## Encounter 5: source 2022-01-15, practice 2024-11-12, inpatient, General Surgery

Chief complaint: Sudden severe epigastric pain following months of intermittent nighttime discomfort

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James Washington, MD
Department: General Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Sudden severe epigastric pain following months of intermittent nighttime discomfort

HISTORY OF PRESENT ILLNESS:
This 47-year-old male with a history of protein-energy malnutrition and pellagra diagnosed in 2020 and non-small cell lung cancer diagnosed in 2021 presents to the emergency department. He reports a 6-hour history of sudden-onset, severe abdominal pain. The pain is localized to the epigastric region and described as sharp and constant. He reports a 3-month history of intermittent epigastric discomfort, often occurring at night and improving with eating. He has been taking over-the-counter antacids with partial relief.

PAST MEDICAL HISTORY:
Active Problem List:
- Pellagra (Niacin deficiency) (diagnosed 2020-01-15)
- Primary hyperparathyroidism (diagnosed 2020-04-15)
- Malignant neoplasm of lymph nodes (diagnosed 2020-07-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2021-01-15)
- Gastroesophageal reflux disease
- Hypertension (poorly controlled)
- History of calcium nephrolithiasis
- Nicotine dependence
- Protein-energy malnutrition with niacin deficiency
- Chronic obstructive pulmonary disease (GOLD Stage II)

His past medical history is significant for gastroesophageal reflux disease.

PAST SURGICAL HISTORY:
- Ureteroscopic stone removal (right kidney), 2 months prior

MEDICATIONS:
Home Medications:
- Omeprazole 20 mg PO BID
- Lisinopril 10 mg PO daily (sporadely taken)
- Ibuprofen 600 mg PO PRN back pain
- Multivitamin 1 tablet PO daily (when remembered)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 52)
- Mother: Hypertension, Type 2 diabetes
- Older brother: Aortic aneurysm

SOCIAL HISTORY:
A 20-pack-year smoking history.

VITAL SIGNS:
His vital signs are: blood pressure 100/60 mmHg, heart rate 120 beats per minute, respiratory rate 22 breaths per minute, and temperature 38.3°C.

PHYSICAL EXAMINATION:
On examination, the patient appears diaphoretic and in distress.
Abdominal examination reveals diffuse tenderness with rebound tenderness and guarding. Bowel sounds are hypoactive.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 16,000 /µL
- Hemoglobin: 13.2 g/dL
- Blood urea nitrogen: 29 mg/dL
- Serum creatinine: 1.1 mg/dL
- Serum amylase: 90 U/L

IMAGING:
An upright chest X-ray shows free air under the diaphragm.

## Encounter 6: source 2022-07-15, practice 2025-05-16, inpatient, Oncology

Chief complaint: Worsening fatigue, hemoptysis, constipation, and muscle weakness with 10kg weight loss

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Maria Gonzalez, MD
Department: Oncology
Visit Type: inpatient

CHIEF COMPLAINT:
Worsening fatigue, hemoptysis, constipation, and muscle weakness with 10kg weight loss

HISTORY OF PRESENT ILLNESS:
This 47-year-old male with non-small cell lung cancer and recent perforated peptic ulcer disease returns for evaluation. He presents with a 2-month history of worsening fatigue, poor appetite, and a 10-kg unintentional weight loss. He has been coughing up blood-tinged sputum intermittently for the past month. He also reports recent episodes of constipation and generalized muscle weakness.

PAST MEDICAL HISTORY:
Active Problem List:
- Pellagra (Niacin deficiency) (diagnosed 2020-01-15)
- Primary hyperparathyroidism (diagnosed 2020-04-15)
- Malignant neoplasm of lymph nodes (diagnosed 2020-07-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2021-01-15)
- Perforated peptic ulcer disease (diagnosed 2022-01-15)
- Gastroesophageal reflux disease
- Hypertension (poorly controlled)
- History of calcium nephrolithiasis
- Nicotine dependence
- Protein-energy malnutrition with niacin deficiency
- Chronic obstructive pulmonary disease (GOLD Stage II)

One month ago, he was treated for an episode of sharp flank pain.

PAST SURGICAL HISTORY:
- Ureteroscopic stone removal (right kidney), 2 months prior

MEDICATIONS:
Home Medications:
- Omeprazole 20 mg PO BID
- Lisinopril 10 mg PO daily (sporadely taken)
- Ibuprofen 600 mg PO PRN back pain
- Multivitamin 1 tablet PO daily (when remembered)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 52)
- Mother: Hypertension, Type 2 diabetes
- Older brother: Aortic aneurysm

SOCIAL HISTORY:
60-pack-year smoking history

PHYSICAL EXAMINATION:
On physical exam, he appears thin and mildly dehydrated. Pulmonary auscultation reveals decreased breath sounds over the right upper lobe.

IMAGING:
Chest x-ray demonstrates a 2.5-cm cavitary lesion in the right upper lung field.

PATHOLOGY:
Bronchoscopic biopsy of the lesion shows malignant cells with keratin pearls and prominent intercellular bridges.

PLAN:
Laboratory studies are ordered to evaluate his metabolic status.

## Encounter 7: source 2023-01-15, practice 2025-11-16, icu, Emergency Medicine

Chief complaint: Sudden onset tearing chest pain radiating to back, 9/10 intensity

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Lisa Patel, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Sudden onset tearing chest pain radiating to back, 9/10 intensity

HISTORY OF PRESENT ILLNESS:
This 48-year-old male with poorly controlled hypertension, active non-small cell lung cancer, and history of perforated peptic ulcer disease presents with sudden onset tearing chest pain. He describes the pain as tearing and rates it 9 out of 10 in intensity.

PAST MEDICAL HISTORY:
Active Problem List:
- Pellagra (Niacin deficiency) (diagnosed 2020-01-15)
- Primary hyperparathyroidism (diagnosed 2020-04-15)
- Malignant neoplasm of lymph nodes (diagnosed 2020-07-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2021-01-15)
- Perforated peptic ulcer disease (diagnosed 2022-01-15)
- Gastroesophageal reflux disease
- Hypertension (poorly controlled)
- History of calcium nephrolithiasis
- Nicotine dependence
- Protein-energy malnutrition with niacin deficiency
- Chronic obstructive pulmonary disease (GOLD Stage II)

He has a history of poorly controlled hypertension and.

PAST SURGICAL HISTORY:
- Ureteroscopic stone removal (right kidney), 2 months prior

MEDICATIONS:
Home Medications:
- Omeprazole 20 mg PO BID
- Lisinopril 10 mg PO daily (sporadely taken)
- Ibuprofen 600 mg PO PRN back pain
- Multivitamin 1 tablet PO daily (when remembered)

ALLERGIES:
NKDA

FAMILY HISTORY:
His brother recently had a myocardial infarction.

SOCIAL HISTORY:
Smokes a pack of cigarettes daily

VITAL SIGNS:
On examination, his blood pressure is 180/95 mmHg in the right arm and 170/90 mmHg in the left arm. His heart rate is 98 beats per minute, and his respiratory rate is 22 breaths per minute.

PHYSICAL EXAMINATION:
He appears diaphoretic and anxious. Auscultation of the heart reveals no murmurs, rubs, or gallops. His lungs are clear to auscultation. There is no peripheral edema.

LABORATORY DATA:
His laboratory values are as follows:

- Troponin I: <0.04 ng/mL (normal <0.04 ng/mL)
- Creatine kinase-MB: 5 U/L (normal 0-25 U/L)
- D-dimer: 500 ng/mL FEU (normal <500 ng/mL FEU)
- Complete blood count and metabolic panel: within normal limits

IMAGING:
A chest X-ray is unremarkable.

OTHER STUDIES:
An electrocardiogram (ECG) shows no evidence of ST-segment changes, T-wave inversions, or Q waves.

ASSESSMENT AND PLAN:
Given the patient's presentation and test results,
