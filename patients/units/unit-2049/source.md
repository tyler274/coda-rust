# synthetic_hospital patient 2049

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Commercial.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-06-16 (Wed) | outpatient | Endocrinology |
| 1 | 2020-04-15 | 2021-09-13 (Mon) | outpatient | Nephrology |
| 2 | 2021-01-15 | 2022-06-10 (Fri) | outpatient | Pulmonology |
| 3 | 2021-09-15 | 2023-02-22 (Wed) | outpatient | Cardiology |
| 4 | 2022-05-15 | 2023-10-20 (Fri) | outpatient | Neurology |
| 5 | 2023-01-15 | 2024-06-11 (Tue) | ed | Emergency Medicine |
| 6 | 2023-07-15 | 2024-12-06 (Fri) | icu | Critical Care |
| 7 | 2025-01-15 | 2026-06-13 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Seasonal pollen (hay fever)",
  "Hymenoptera venom (suspected)"
 ],
 "insurance": "Commercial",
 "occupation": "Construction worker",
 "alcohol_use": "Social, 1-2 drinks per week",
 "family_history": [
  "Father: Autosomal dominant polycystic kidney disease (ADPKD), hypertension, bicuspid aortic valve",
  "Mother: Hypertension",
  "Paternal grandfather: Chronic kidney disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 1 pack per day (30 pack-years)",
 "home_medications": [
  {
   "dose": "20 mg oral daily",
   "name": "Lisinopril"
  },
  {
   "dose": "90 mcg inhalation every 4-6 hours as needed for wheezing",
   "name": "Albuterol HFA"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Chronic obstructive pulmonary disease (COPD)",
  "Idiopathic pulmonary fibrosis",
  "Autosomal dominant polycystic kidney disease (ADPKD)",
  "Acromegaly",
  "Congenital bicuspid aortic valve with chronic aortic regurgitation",
  "Cervical radiculopathy",
  "Seasonal allergic rhinitis"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Aortic dissection", "Aortic Regurgitation", "Autosomal dominant polycystic kidney disease", "Respiratory arrest", "Acromegaly", "Idiopathic pulmonary fibrosis", "Anaphylaxis", "Cervical radiculopathy due to C6-C7 disc herniation"]
Comorbidities: ["Tobacco use disorder", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2021-06-16, outpatient, Endocrinology

Chief complaint: Progressive snoring, sleep apnea episodes, and increased ring and shoe size over the past 6 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive snoring, sleep apnea episodes, and increased ring and shoe size over the past 6 months

HISTORY OF PRESENT ILLNESS:
His wife also reports that he has become more irritable and seems to have difficulty with concentration. The patient mentions that he has noticed an increase in his ring and shoe size.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Chronic obstructive pulmonary disease (COPD)
- Idiopathic pulmonary fibrosis
- Autosomal dominant polycystic kidney disease (ADPKD)
- Acromegaly
- Congenital bicuspid aortic valve with chronic aortic regurgitation
- Cervical radiculopathy
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg oral daily
- Albuterol HFA 90 mcg inhalation every 4-6 hours as needed for wheezing

ALLERGIES:
Seasonal pollen (hay fever), Hymenoptera venom (suspected)

FAMILY HISTORY:
- Father: Autosomal dominant polycystic kidney disease (ADPKD), hypertension, bicuspid aortic valve
- Mother: Hypertension
- Paternal grandfather: Chronic kidney disease

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day (30 pack-years). Alcohol: Social, 1-2 drinks per week. Occupation: Construction worker.

VITAL SIGNS:
His blood pressure is 145/90 mmHg.

PHYSICAL EXAMINATION:
On examination, the patient has an enlarged tongue with teeth indentations, coarse facial features, and thickened skinfolds.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum insulin-like growth factor-1 (IGF-1): Elevated
- Fasting glucose: 110 mg/dL
- Thyroid-stimulating hormone (TSH): Normal
- Free T4: Normal

## Encounter 1: source 2020-04-15, practice 2021-09-13, outpatient, Nephrology

Chief complaint: Recurrent flank pain, occasional headaches, and blood in urine over the past month

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. James Okafor, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Recurrent flank pain, occasional headaches, and blood in urine over the past month

HISTORY OF PRESENT ILLNESS:
He returns for ADPKD follow-up after endocrinology. He reports noticing blood in his urine several times over the past month.

PAST MEDICAL HISTORY:
Active Problem List:
- Acromegaly (diagnosed 2020-01-15)
- Hypertension
- Chronic obstructive pulmonary disease (COPD)
- Idiopathic pulmonary fibrosis
- Autosomal dominant polycystic kidney disease (ADPKD)
- Congenital bicuspid aortic valve with chronic aortic regurgitation
- Cervical radiculopathy
- Seasonal allergic rhinitis

His medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes lisinopril.

Home Medications:
- Albuterol HFA 90 mcg inhalation every 4-6 hours as needed for wheezing

ALLERGIES:
Seasonal pollen (hay fever), Hymenoptera venom (suspected)

FAMILY HISTORY:
His mother had chronic kidney disease and died in her early 50s, but he is unsure of the exact cause.

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day (30 pack-years). Alcohol: Social, 1-2 drinks per week. Occupation: Construction worker.

VITAL SIGNS:
His blood pressure is 155/95 mmHg

PHYSICAL EXAMINATION:
Palpation of the abdomen reveals bilateral flank masses

LABORATORY DATA:
Urinalysis shows red blood cells but no casts

IMAGING:
Renal ultrasound demonstrates numerous bilateral renal cysts of varying sizes

## Encounter 2: source 2021-01-15, practice 2022-06-10, outpatient, Pulmonology

Chief complaint: Progressive shortness of breath and dry cough over 6 months

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Maria Rodriguez, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath and dry cough over 6 months

HISTORY OF PRESENT ILLNESS:
Patient with acromegaly and ADPKD returns for evaluation. He denies any recent travel or sick contacts.

PAST MEDICAL HISTORY:
Active Problem List:
- Acromegaly (diagnosed 2020-01-15)
- Autosomal dominant polycystic kidney disease (diagnosed 2020-04-15)
- Hypertension
- Chronic obstructive pulmonary disease (COPD)
- Idiopathic pulmonary fibrosis
- Autosomal dominant polycystic kidney disease (ADPKD)
- Congenital bicuspid aortic valve with chronic aortic regurgitation
- Cervical radiculopathy
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg oral daily
- Albuterol HFA 90 mcg inhalation every 4-6 hours as needed for wheezing

ALLERGIES:
Seasonal pollen (hay fever), Hymenoptera venom (suspected)

FAMILY HISTORY:
- Father: Autosomal dominant polycystic kidney disease (ADPKD), hypertension, bicuspid aortic valve
- Mother: Hypertension
- Paternal grandfather: Chronic kidney disease

SOCIAL HISTORY:
His past medical history is significant for smoking one pack of cigarettes per day for the past 20 years. The patient reports that several of his coworkers have similar symptoms.

PHYSICAL EXAMINATION:
On physical examination, you note digital clubbing and inspiratory crackles at the lung bases.

LABORATORY DATA:
Laboratory Values:
- Complete Blood Count (CBC): Normal
- Erythrocyte Sedimentation Rate (ESR): Elevated
- Antinuclear Antibodies (ANA): Negative
- Rheumatoid Factor (RF): Negative

IMAGING:
High-resolution computed tomography (HRCT) of the chest demonstrates bilateral subpleural reticular opacities and honeycombing.

OTHER STUDIES:
Pulmonary function tests show a restrictive pattern with reduced forced vital capacity (FVC) and a normal forced expiratory volume in 1 second (FEV1)/FVC ratio.

ASSESSMENT AND PLAN:
Given the patient's occupational history and clinical findings,

## Encounter 3: source 2021-09-15, practice 2023-02-22, outpatient, Cardiology

Chief complaint: Progressive dyspnea on exertion and decreased exercise tolerance over 6 months

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Robert Kim, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive dyspnea on exertion and decreased exercise tolerance over 6 months

HISTORY OF PRESENT ILLNESS:
Patient returns to cardiology with symptoms in context of bicuspid valve disease, acromegaly, and pulmonology evaluation. He presents with a 6-month history of progressive dyspnea on exertion and a decreased exercise tolerance. He also reports occasional palpitations but denies chest pain, syncope, or leg swelling.

PAST MEDICAL HISTORY:
Active Problem List:
- Acromegaly (diagnosed 2020-01-15)
- Autosomal dominant polycystic kidney disease (diagnosed 2020-04-15)
- Idiopathic pulmonary fibrosis (diagnosed 2021-01-15)
- Hypertension
- Chronic obstructive pulmonary disease (COPD)
- Autosomal dominant polycystic kidney disease (ADPKD)
- Congenital bicuspid aortic valve with chronic aortic regurgitation
- Cervical radiculopathy
- Seasonal allergic rhinitis

His past medical history is significant for a heart murmur identified in childhood, though he was lost to follow-up and never underwent echocardiography.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg oral daily
- Albuterol HFA 90 mcg inhalation every 4-6 hours as needed for wheezing

ALLERGIES:
Seasonal pollen (hay fever), Hymenoptera venom (suspected)

FAMILY HISTORY:
- Father: Autosomal dominant polycystic kidney disease (ADPKD), hypertension, bicuspid aortic valve
- Mother: Hypertension
- Paternal grandfather: Chronic kidney disease

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day (30 pack-years). Alcohol: Social, 1-2 drinks per week. Occupation: Construction worker.

VITAL SIGNS:
His blood pressure is 135/50 mmHg, and the pulse is 80 bpm and bounding.

PHYSICAL EXAMINATION:
On physical examination, you note a high-pitched, blowing diastolic murmur best heard at the left sternal border.

IMAGING:
A transthoracic echocardiogram reveals a bicuspid aortic valve with significant regurgitation and mild aortic root dilation.

OTHER STUDIES:
An electrocardiogram shows left ventricular hypertrophy.

## Encounter 4: source 2022-05-15, practice 2023-10-20, outpatient, Neurology

Chief complaint: 6-week history of sharp shooting pains radiating down right arm to ring and little fingers with fine motor difficulty

ENCOUNTER NOTE
Date: 2022-05-15
Provider: Dr. Aisha Patel, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
6-week history of sharp shooting pains radiating down right arm to ring and little fingers with fine motor difficulty

HISTORY OF PRESENT ILLNESS:
The patient with history of acromegaly, autosomal dominant polycystic kidney disease, idiopathic pulmonary fibrosis, and congenital bicuspid aortic valve presents with a 6-week history of upper extremity symptoms. He reports that the pain is exacerbated by extending his neck and turning his head to the right. He also complains of difficulty with fine motor tasks, such as buttoning his shirt and gripping his hammer.

PAST MEDICAL HISTORY:
Active Problem List:
- Acromegaly (diagnosed 2020-01-15)
- Autosomal dominant polycystic kidney disease (diagnosed 2020-04-15)
- Idiopathic pulmonary fibrosis (diagnosed 2021-01-15)
- Aortic Regurgitation (diagnosed 2021-09-15)
- Hypertension
- Chronic obstructive pulmonary disease (COPD)
- Autosomal dominant polycystic kidney disease (ADPKD)
- Congenital bicuspid aortic valve with chronic aortic regurgitation
- Cervical radiculopathy
- Seasonal allergic rhinitis

His past medical history is significant for hypertension, controlled with lisinopril.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes lisinopril.

Home Medications:
- Albuterol HFA 90 mcg inhalation every 4-6 hours as needed for wheezing

ALLERGIES:
Seasonal pollen (hay fever), Hymenoptera venom (suspected)

FAMILY HISTORY:
- Father: Autosomal dominant polycystic kidney disease (ADPKD), hypertension, bicuspid aortic valve
- Mother: Hypertension
- Paternal grandfather: Chronic kidney disease

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day (30 pack-years). Alcohol: Social, 1-2 drinks per week. Occupation: Construction worker.

PHYSICAL EXAMINATION:
On physical examination, there is decreased sensation over the medial aspect of his right forearm and the ulnar side of his hand. Motor examination reveals weakness in thumb adduction and finger abduction. Reflexes are normal, and there is no atrophy noted.

IMAGING:
An MRI of the cervical spine shows a right-sided paracentral disc herniation at the C7-T1 level.

## Encounter 5: source 2023-01-15, practice 2024-06-11, ed, Emergency Medicine

Chief complaint: Sudden onset difficulty breathing, facial swelling, and diffuse itchy rash after gardening injury

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Michael Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset difficulty breathing, facial swelling, and diffuse itchy rash after gardening injury

HISTORY OF PRESENT ILLNESS:
This patient with acromegaly, idiopathic pulmonary fibrosis, and autosomal dominant polycystic kidney disease presents with acute symptoms. He reports that he was gardening 30 minutes prior to the onset of symptoms when he felt a sharp pain on his arm. He initially dismissed it as a thorn prick but developed symptoms shortly after.

PAST MEDICAL HISTORY:
Active Problem List:
- Acromegaly (diagnosed 2020-01-15)
- Autosomal dominant polycystic kidney disease (diagnosed 2020-04-15)
- Idiopathic pulmonary fibrosis (diagnosed 2021-01-15)
- Aortic Regurgitation (diagnosed 2021-09-15)
- Cervical radiculopathy due to C6-C7 disc herniation (diagnosed 2022-05-15)
- Hypertension
- Chronic obstructive pulmonary disease (COPD)
- Autosomal dominant polycystic kidney disease (ADPKD)
- Congenital bicuspid aortic valve with chronic aortic regurgitation
- Cervical radiculopathy
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg oral daily
- Albuterol HFA 90 mcg inhalation every 4-6 hours as needed for wheezing

ALLERGIES:
Seasonal pollen (hay fever), Hymenoptera venom (suspected)

FAMILY HISTORY:
- Father: Autosomal dominant polycystic kidney disease (ADPKD), hypertension, bicuspid aortic valve
- Mother: Hypertension
- Paternal grandfather: Chronic kidney disease

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day (30 pack-years). Alcohol: Social, 1-2 drinks per week. Occupation: Construction worker.

VITAL SIGNS:
His blood pressure is 90/60 mmHg, heart rate is 120 beats per minute, respiratory rate is 28 breaths per minute, and oxygen saturation is 88% on room air.

PHYSICAL EXAMINATION:
Physical examination reveals bilateral wheezes, generalized urticaria, and angioedema of the lips and tongue. He is anxious and struggling to speak in full sentences.

LABORATORY DATA:
Serum tryptase: Elevated
Complete blood count: Within normal limits
Basic metabolic panel: Within normal limits

## Encounter 6: source 2023-07-15, practice 2024-12-06, icu, Critical Care

Chief complaint: Acute respiratory arrest following 2-day history of increasing shortness of breath and productive cough

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. David Park, MD
Department: Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Acute respiratory arrest following 2-day history of increasing shortness of breath and productive cough

HISTORY OF PRESENT ILLNESS:
The patient with COPD and IPF presents with respiratory symptoms. He reports that his symptoms have progressively worsened despite using his albuterol inhaler more frequently.

PAST MEDICAL HISTORY:
Active Problem List:
- Acromegaly (diagnosed 2020-01-15)
- Autosomal dominant polycystic kidney disease (diagnosed 2020-04-15)
- Idiopathic pulmonary fibrosis (diagnosed 2021-01-15)
- Aortic Regurgitation (diagnosed 2021-09-15)
- Cervical radiculopathy due to C6-C7 disc herniation (diagnosed 2022-05-15)
- Anaphylaxis (diagnosed 2023-01-15)
- Hypertension
- Chronic obstructive pulmonary disease (COPD)
- Autosomal dominant polycystic kidney disease (ADPKD)
- Congenital bicuspid aortic valve with chronic aortic regurgitation
- Cervical radiculopathy
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg oral daily
- Albuterol HFA 90 mcg inhalation every 4-6 hours as needed for wheezing

ALLERGIES:
Seasonal pollen (hay fever), Hymenoptera venom (suspected)

FAMILY HISTORY:
- Father: Autosomal dominant polycystic kidney disease (ADPKD), hypertension, bicuspid aortic valve
- Mother: Hypertension
- Paternal grandfather: Chronic kidney disease

SOCIAL HISTORY:
His medical history is significant for a 30-pack-year smoking history.

REVIEW OF SYSTEMS:
He denies chest pain, fever, or chills.

PHYSICAL EXAMINATION:
On physical examination, he is tachypneic with a respiratory rate of 28 breaths per minute, uses accessory muscles to breathe, and has diffuse wheezing on auscultation.

LABORATORY DATA:
Arterial blood gas (ABG) analysis reveals the following:

- pH: 7.28
- PaCO2: 60 mmHg
- PaO2: 55 mmHg
- HCO3- (calculated): 24 mEq/L

## Encounter 7: source 2025-01-15, practice 2026-06-13, ed, Emergency Medicine

Chief complaint: Sudden onset of severe tearing chest pain radiating to back beginning 2 hours ago

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Lisa Wong, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe tearing chest pain radiating to back beginning 2 hours ago

HISTORY OF PRESENT ILLNESS:
A 50-year-old male with a history of congenital bicuspid aortic valve with chronic aortic regurgitation and acromegaly presents to the emergency department with a sudden onset of severe chest pain radiating to his back, described as tearing in nature, and beginning 2 hours ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Acromegaly (diagnosed 2020-01-15)
- Autosomal dominant polycystic kidney disease (diagnosed 2020-04-15)
- Idiopathic pulmonary fibrosis (diagnosed 2021-01-15)
- Aortic Regurgitation (diagnosed 2021-09-15)
- Cervical radiculopathy due to C6-C7 disc herniation (diagnosed 2022-05-15)
- Anaphylaxis (diagnosed 2023-01-15)
- Respiratory arrest (diagnosed 2023-07-15)
- Hypertension
- Chronic obstructive pulmonary disease (COPD)
- Autosomal dominant polycystic kidney disease (ADPKD)
- Congenital bicuspid aortic valve with chronic aortic regurgitation
- Cervical radiculopathy
- Seasonal allergic rhinitis

He has a history of poorly controlled hypertension.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg oral daily
- Albuterol HFA 90 mcg inhalation every 4-6 hours as needed for wheezing

ALLERGIES:
Seasonal pollen (hay fever), Hymenoptera venom (suspected)

FAMILY HISTORY:
- Father: Autosomal dominant polycystic kidney disease (ADPKD), hypertension, bicuspid aortic valve
- Mother: Hypertension
- Paternal grandfather: Chronic kidney disease

SOCIAL HISTORY:
Smokes a pack of cigarettes daily

VITAL SIGNS:
On examination, his blood pressure is 210/120 mmHg in the right arm and 180/100 mmHg in the left arm. His heart rate is 110 beats per minute, and his respiratory rate is 22 breaths per minute.

PHYSICAL EXAMINATION:
He appears diaphoretic and anxious.

LABORATORY DATA:
Laboratory values are as follows:
- Hemoglobin: 14 g/dL
- White blood cell count: 12,000/mm^3
- Platelets: 250,000/mm^3
- Serum creatinine: 1.2 mg/dL
- Troponin I: Normal

IMAGING:
A chest X-ray shows a widened mediastinum.

OTHER STUDIES:
An ECG shows no ST-segment elevations or depressions.
