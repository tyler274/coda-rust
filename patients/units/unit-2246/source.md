# synthetic_hospital patient 2246

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-04-07 (Fri) | outpatient | Pulmonology |
| 1 | 2020-04-15 | 2023-07-10 (Mon) | outpatient | Hematology |
| 2 | 2020-10-15 | 2024-01-08 (Mon) | outpatient | Cardiology |
| 3 | 2021-03-15 | 2024-05-31 (Fri) | ed | Emergency Medicine |
| 4 | 2021-09-15 | 2024-12-05 (Thu) | ed | Emergency Medicine |
| 5 | 2022-03-15 | 2025-05-28 (Wed) | ed | Emergency Medicine |
| 6 | 2022-09-15 | 2025-12-04 (Thu) | ed | Emergency Medicine |
| 7 | 2023-03-15 | 2026-05-30 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Semi-retired maintenance technician",
 "alcohol_use": "Heavy daily use (6-10 standard drinks/day, primarily beer, for 30+ years)",
 "family_history": [
  "Hypertension (father)",
  "Ischemic stroke (mother, age 78)",
  "Interstitial lung disease (maternal uncle)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Atenolol"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Omeprazole"
  },
  {
   "dose": "300 mg PO nightly",
   "name": "Phenytoin"
  },
  {
   "dose": "100 mg PO daily",
   "name": "Thiamine"
  }
 ],
 "surgical_history": [
  "Right knee arthroplasty",
  "Multiple dental extractions"
 ],
 "chronic_conditions": [
  "Alcohol Use Disorder, severe",
  "Gastroesophageal Reflux Disease (GERD)",
  "Hypertension, poorly controlled",
  "Epilepsy",
  "Idiopathic Pulmonary Fibrosis",
  "Sideroblastic anemia",
  "Tachycardia-mediated cardiomyopathy",
  "Recurrent aspiration",
  "Periodontal disease with poor dentition"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Recurrent aspiration pneumonia", "Idiopathic Pulmonary Fibrosis", "Lung abscess (aspiration)", "Wernicke encephalopathy", "Sideroblastic anemia", "Tachycardia-mediated cardiomyopathy"]
Comorbidities: ["Alcohol use disorder", "Gastroesophageal reflux disease", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2023-04-07, outpatient, Pulmonology

Chief complaint: Progressive dyspnea on exertion and nonproductive cough for 6 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive dyspnea on exertion and nonproductive cough for 6 months

PAST MEDICAL HISTORY:
Active Problem List:
- Alcohol Use Disorder, severe
- Gastroesophageal Reflux Disease (GERD)
- Hypertension, poorly controlled
- Epilepsy
- Idiopathic Pulmonary Fibrosis
- Sideroblastic anemia
- Tachycardia-mediated cardiomyopathy
- Recurrent aspiration
- Periodontal disease with poor dentition

PAST SURGICAL HISTORY:
- Right knee arthroplasty
- Multiple dental extractions

MEDICATIONS:
His medications include lisinopril and omeprazole.

Home Medications:
- Atenolol 50 mg PO daily
- Phenytoin 300 mg PO nightly
- Thiamine 100 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Hypertension (father)
- Ischemic stroke (mother, age 78)
- Interstitial lung disease (maternal uncle)

SOCIAL HISTORY:
He has never smoked and has no known occupational exposures.

REVIEW OF SYSTEMS:
He denies any recent infections, chest pain, or leg swelling.

VITAL SIGNS:
Oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
On physical examination, fine inspiratory crackles are heard at the lung bases bilaterally.

IMAGING:
High-resolution computed tomography (HRCT) of the chest shows subpleural reticular opacities with honeycombing but no evidence of emphysema or nodules.

OTHER STUDIES:
Pulmonary function tests reveal a restrictive pattern with reduced forced vital capacity (FVC) and a normal forced expiratory volume in 1 second (FEV1)/FVC ratio.

## Encounter 1: source 2020-04-15, practice 2023-07-10, outpatient, Hematology

Chief complaint: Progressive fatigue and decreased exercise tolerance

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Okonkwo, MD
Department: Hematology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue and decreased exercise tolerance

HISTORY OF PRESENT ILLNESS:
The patient with a history of idiopathic pulmonary fibrosis returns for hematology evaluation approximately three months following his prior pulmonology encounter. He presents with progressive fatigue and decreased exercise tolerance over the past several months. He denies recent blood loss, changes in bowel habits, or melena. He mentions occasional numbness and tingling in his feet and difficulty concentrating at work.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Alcohol Use Disorder, severe
- Gastroesophageal Reflux Disease (GERD)
- Hypertension, poorly controlled
- Epilepsy
- Sideroblastic anemia
- Tachycardia-mediated cardiomyopathy
- Recurrent aspiration
- Periodontal disease with poor dentition

He has a history of hypertension and epilepsy.

PAST SURGICAL HISTORY:
- Right knee arthroplasty
- Multiple dental extractions

MEDICATIONS:
He takes for which he takes lisinopril and phenytoin.

Home Medications:
- Atenolol 50 mg PO daily
- Omeprazole 40 mg PO daily
- Thiamine 100 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Hypertension (father)
- Ischemic stroke (mother, age 78)
- Interstitial lung disease (maternal uncle)

SOCIAL HISTORY:
He reports frequent alcohol use and says he drinks "about a six-pack" most evenings to help him unwind. He lives alone and admits that his diet consists mostly of boxed and canned foods.

REVIEW OF SYSTEMS:
On review of systems, he denies fever, weight loss, or night sweats.

PHYSICAL EXAMINATION:
Physical exam shows pallor and mild ataxia. Neurologic testing reveals decreased vibratory sense and ankle reflexes bilaterally. Cardiac and pulmonary exams are normal.

LABORATORY DATA:
Laboratory tests reveal a normocytic anemia. Iron studies show elevated serum iron, increased ferritin, and decreased total iron-binding capacity. Reticulocyte count is low. A peripheral smear is shown:

## Encounter 2: source 2020-10-15, practice 2024-01-08, outpatient, Cardiology

Chief complaint: Progressive exertional dyspnea and orthopnea

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Jennifer Walsh, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive exertional dyspnea and orthopnea

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Sideroblastic anemia (diagnosed 2020-04-15)
- Alcohol Use Disorder, severe
- Gastroesophageal Reflux Disease (GERD)
- Hypertension, poorly controlled
- Epilepsy
- Tachycardia-mediated cardiomyopathy
- Recurrent aspiration
- Periodontal disease with poor dentition

PAST SURGICAL HISTORY:
- Right knee arthroplasty
- Multiple dental extractions

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Atenolol 50 mg PO daily
- Omeprazole 40 mg PO daily
- Phenytoin 300 mg PO nightly
- Thiamine 100 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Hypertension (father)
- Ischemic stroke (mother, age 78)
- Interstitial lung disease (maternal uncle)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Heavy daily use (6-10 standard drinks/day, primarily beer, for 30+ years). Occupation: Semi-retired maintenance technician.

PHYSICAL EXAMINATION:
Jugular venous distension, bibasilar crackles, a third heart sound (S3), and a laterally displaced point of maximal impulse

IMAGING:
Chest x-ray is shown below. Echocardiography demonstrates global hypokinesis and left ventricular dilation.

## Encounter 3: source 2021-03-15, practice 2024-05-31, ed, Emergency Medicine

Chief complaint: New-onset confusion and difficulty walking

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. David Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
New-onset confusion and difficulty walking

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old male with severe alcohol use disorder, idiopathic pulmonary fibrosis, sideroblastic anemia, and tachycardia-mediated cardiomyopathy who presents with acute neurological symptoms. He is brought to the emergency department by his wife due to new-onset confusion and difficulty walking over the past 24 hours. He was discharged from hospital three days ago after elective right knee arthroplasty. His postoperative course was uneventful, and he had been recovering well until yesterday.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Sideroblastic anemia (diagnosed 2020-04-15)
- Tachycardia-mediated cardiomyopathy (diagnosed 2020-10-15)
- Alcohol Use Disorder, severe
- Gastroesophageal Reflux Disease (GERD)
- Hypertension, poorly controlled
- Epilepsy
- Recurrent aspiration
- Periodontal disease with poor dentition

PAST SURGICAL HISTORY:
- Right knee arthroplasty
- Multiple dental extractions

MEDICATIONS:
He takes atenolol for hypertension and has taken one dose of oxycodone in the last 24 hours.

Home Medications:
- Lisinopril 20 mg PO daily
- Omeprazole 40 mg PO daily
- Phenytoin 300 mg PO nightly
- Thiamine 100 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Hypertension (father)
- Ischemic stroke (mother, age 78)
- Interstitial lung disease (maternal uncle)

SOCIAL HISTORY:
His wife reports that he has a long history of daily alcohol consumption, drinking 6–10 cans of beer per day for many years, although he initially denied this to the surgical team.

VITAL SIGNS:
On examination, the patient is afebrile, with a blood pressure of 125/76 mm Hg, pulse of 88/min, and respiratory rate of 15/min.

PHYSICAL EXAMINATION:
He is alert but disoriented to time and place. He cannot recall three simple objects after a few minutes and needs repeated prompting to follow commands. He has a wide-based gait and horizontal nystagmus with limited abduction of the right eye. His surgical incision is clean.

LABORATORY DATA:
Laboratory studies, including complete blood count, serum electrolytes, glucose, and renal and hepatic function, are within normal limits. Urine drug screen is negative.

## Encounter 4: source 2021-09-15, practice 2024-12-05, ed, Emergency Medicine

Chief complaint: 2-week history of low-grade fever, malaise, and productive cough with foul-smelling sputum

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Lisa Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-week history of low-grade fever, malaise, and productive cough with foul-smelling sputum

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old male with severe alcohol use disorder, idiopathic pulmonary fibrosis, and recurrent aspiration who presents to the Emergency Department six months following an episode of Wernicke encephalopathy. He reports a recent episode of "choking on food" while drinking heavily.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Sideroblastic anemia (diagnosed 2020-04-15)
- Tachycardia-mediated cardiomyopathy (diagnosed 2020-10-15)
- Wernicke encephalopathy (diagnosed 2021-03-15)
- Alcohol Use Disorder, severe
- Gastroesophageal Reflux Disease (GERD)
- Hypertension, poorly controlled
- Epilepsy
- Recurrent aspiration
- Periodontal disease with poor dentition

PAST SURGICAL HISTORY:
- Right knee arthroplasty
- Multiple dental extractions

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Atenolol 50 mg PO daily
- Omeprazole 40 mg PO daily
- Phenytoin 300 mg PO nightly
- Thiamine 100 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Hypertension (father)
- Ischemic stroke (mother, age 78)
- Interstitial lung disease (maternal uncle)

SOCIAL HISTORY:
History of chronic alcoholism and poor dental hygiene

VITAL SIGNS:
On examination, he is afebrile, blood pressure is 130/80 mmHg, heart rate is 92 beats per minute, respiratory rate is 20 breaths per minute, and oxygen saturation is 94% on room air

PHYSICAL EXAMINATION:
Lung auscultation reveals crackles at the right lower lung base. Neurological examination shows a decreased gag reflex.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 14,000 /µL (normal: 4,500-11,000 /µL)
- Neutrophils: 80% (normal: 40-60%)
- Blood alcohol level: Elevated

IMAGING:
Chest X-ray: Infiltrate in the right lower lobe

## Encounter 5: source 2022-03-15, practice 2025-05-28, ed, Emergency Medicine

Chief complaint: Fever, malaise, and productive cough with foul-smelling sputum for 3 weeks

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, malaise, and productive cough with foul-smelling sputum for 3 weeks

HISTORY OF PRESENT ILLNESS:
The patient returns with recurrent symptoms similar to his prior aspiration event. He presents to the emergency department with a 3-week history of fever, malaise, and productive cough with foul-smelling sputum and reports a recent episode of loss of consciousness while drinking.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Sideroblastic anemia (diagnosed 2020-04-15)
- Tachycardia-mediated cardiomyopathy (diagnosed 2020-10-15)
- Wernicke encephalopathy (diagnosed 2021-03-15)
- Recurrent aspiration pneumonia (diagnosed 2021-09-15)
- Alcohol Use Disorder, severe
- Gastroesophageal Reflux Disease (GERD)
- Hypertension, poorly controlled
- Epilepsy
- Recurrent aspiration
- Periodontal disease with poor dentition

PAST SURGICAL HISTORY:
- Right knee arthroplasty
- Multiple dental extractions

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Atenolol 50 mg PO daily
- Omeprazole 40 mg PO daily
- Phenytoin 300 mg PO nightly
- Thiamine 100 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Hypertension (father)
- Ischemic stroke (mother, age 78)
- Interstitial lung disease (maternal uncle)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Heavy daily use (6-10 standard drinks/day, primarily beer, for 30+ years). Occupation: Semi-retired maintenance technician.

VITAL SIGNS:
On examination, his temperature is 38.5°C (101.3°F), heart rate is 102 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 94% on room air

PHYSICAL EXAMINATION:
Lung auscultation reveals decreased breath sounds and dullness to percussion at the right lung base

LABORATORY DATA:
White blood cell count: 15,000/µL (normal: 4,500-11,000/µL)
- Hemoglobin: 13.2 g/dL (normal: 13.8-17.2 g/dL)
- Platelets: 250,000/µL (normal: 150,000-450,000/µL)
- Sodium: 138 mEq/L (normal: 135-145 mEq/L)
- Potassium: 4.2 mEq/L (normal: 3.5-5.1 mEq/L)
- Blood urea nitrogen: 18 mg/dL (normal: 7-20 mg/dL)
- Creatinine: 1.0 mg/dL (normal: 0.6-1.2 mg/dL)

IMAGING:
Chest radiography shows a cavitary lesion in the right lower lobe with an air-fluid level

## Encounter 6: source 2022-09-15, practice 2025-12-04, ed, Emergency Medicine

Chief complaint: Acute onset of shortness of breath and cough

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. Maria Gonzalez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of shortness of breath and cough

HISTORY OF PRESENT ILLNESS:
The patient returns following prior aspiration pneumonia and lung abscess, presenting with acute onset of shortness of breath and cough after a witnessed episode of vomiting

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Sideroblastic anemia (diagnosed 2020-04-15)
- Tachycardia-mediated cardiomyopathy (diagnosed 2020-10-15)
- Wernicke encephalopathy (diagnosed 2021-03-15)
- Recurrent aspiration pneumonia (diagnosed 2021-09-15)
- Lung abscess (aspiration) (diagnosed 2022-03-15)
- Alcohol Use Disorder, severe
- Gastroesophageal Reflux Disease (GERD)
- Hypertension, poorly controlled
- Epilepsy
- Recurrent aspiration
- Periodontal disease with poor dentition

PAST SURGICAL HISTORY:
- Right knee arthroplasty
- Multiple dental extractions

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Atenolol 50 mg PO daily
- Omeprazole 40 mg PO daily
- Phenytoin 300 mg PO nightly
- Thiamine 100 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Hypertension (father)
- Ischemic stroke (mother, age 78)
- Interstitial lung disease (maternal uncle)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Heavy daily use (6-10 standard drinks/day, primarily beer, for 30+ years). Occupation: Semi-retired maintenance technician.

VITAL SIGNS:
His temperature is 37.8°C (100°F), blood pressure is 155/90 mmHg, heart rate is 110 beats/min, respiratory rate is 28 breaths/min, and oxygen saturation is 89% on room air

PHYSICAL EXAMINATION:
He appears anxious and is using accessory muscles to breathe. Physical examination reveals bilateral crackles in the lower lung fields.

LABORATORY DATA:
White blood cell count: 12,000/mm³
Hemoglobin: 14 g/dL
Platelet count: 250,000/mm³
Arterial blood gas:
pH: 7.45
PaCO2: 35 mmHg
PaO2: 60 mmHg

IMAGING:
Chest X-ray shows bilateral infiltrates.

## Encounter 7: source 2023-03-15, practice 2026-05-30, ed, Emergency Medicine

Chief complaint: Acute onset of shortness of breath and cough after large meal

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of shortness of breath and cough after large meal

HISTORY OF PRESENT ILLNESS:
This 70-year-old male with recurrent aspiration, prior aspiration pneumonias, and poor dentition presents with acute onset of shortness of breath and cough that began several hours after a large meal. He reports that he was lying down after eating when he suddenly experienced a bout of severe coughing and choking.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Sideroblastic anemia (diagnosed 2020-04-15)
- Tachycardia-mediated cardiomyopathy (diagnosed 2020-10-15)
- Wernicke encephalopathy (diagnosed 2021-03-15)
- Recurrent aspiration pneumonia (diagnosed 2021-09-15)
- Lung abscess (aspiration) (diagnosed 2022-03-15)
- Alcohol Use Disorder, severe
- Gastroesophageal Reflux Disease (GERD)
- Hypertension, poorly controlled
- Epilepsy
- Recurrent aspiration
- Periodontal disease with poor dentition

PAST SURGICAL HISTORY:
- Right knee arthroplasty
- Multiple dental extractions

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Atenolol 50 mg PO daily
- Omeprazole 40 mg PO daily
- Phenytoin 300 mg PO nightly
- Thiamine 100 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Hypertension (father)
- Ischemic stroke (mother, age 78)
- Interstitial lung disease (maternal uncle)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Heavy daily use (6-10 standard drinks/day, primarily beer, for 30+ years). Occupation: Semi-retired maintenance technician.

VITAL SIGNS:
Temperature is 37.8°C, blood pressure is 145/90 mmHg, heart rate is 102 beats/min, respiratory rate is 26 breaths/min, and oxygen saturation is 89% on room air

PHYSICAL EXAMINATION:
Lung auscultation reveals bilateral crackles, more prominent in the right lower lobe

LABORATORY DATA:
Arterial blood gas on room air shows:
- pH: 7.45
- PaCO2: 35 mmHg
- PaO2: 60 mmHg
- HCO3-: 24 mEq/L
- Lactate: 1.2 mmol/L

IMAGING:
Chest X-ray shows bilateral alveolar infiltrates with a predilection for the right lower lobe
