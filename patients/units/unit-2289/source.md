# synthetic_hospital patient 2289

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 65 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-07-17 (Mon) | outpatient | Pulmonology |
| 1 | 2020-03-15 | 2023-09-08 (Fri) | outpatient | Internal Medicine |
| 2 | 2020-04-15 | 2023-10-10 (Tue) | outpatient | Pulmonology |
| 3 | 2020-05-15 | 2023-11-18 (Sat) | ed | Emergency Medicine |
| 4 | 2021-03-15 | 2024-09-14 (Sat) | ed | Emergency Medicine |
| 5 | 2021-09-15 | 2025-03-08 (Sat) | inpatient | Oncology |
| 6 | 2022-05-15 | 2025-11-16 (Sun) | ed | Emergency Medicine |
| 7 | 2023-01-15 | 2026-07-04 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired naval shipyard worker (30 years service, retired age 55)",
 "alcohol_use": "Social (occasional beer, 1-2 per month)",
 "family_history": [
  "Father: Lung cancer (deceased age 68, heavy smoker)",
  "Mother: Emphysema",
  "Sibling: Coronary artery disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 90 pack-year history, quit 2 years ago after multiple previous quit attempts",
 "home_medications": [
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
  "Right-sided thoracentesis with chemical pleurodesis (for recurrent effusion)",
  "Central venous port placement",
  "CT-guided transthoracic needle biopsy of lung mass",
  "No colonoscopy or other cancer screening procedures performed to date"
 ],
 "chronic_conditions": [
  "Stage IV lung cancer with extensive-stage disease",
  "Bronchiectasis",
  "Recurrent malignant pleural effusion",
  "Hyperlipidemia",
  "Asbestos exposure",
  "Paraneoplastic syndrome history (SIADH)"
 ],
 "age_at_first_encounter": 65
}
```

Primary diagnoses: ["Suspected malignant neoplasm of lung", "Malignant pleural effusion", "Syndrome of inappropriate antidiuretic hormone secretion (SIADH)", "Catheter-related bloodstream infection", "Obstructive Atelectasis", "Bronchiectasis"]
Comorbidities: ["History of recent pneumonia", "Suspected malignant neoplasm of lung"]

## Encounter 0: source 2020-01-15, practice 2023-07-17, outpatient, Pulmonology

Chief complaint: Chronic productive cough with foul-smelling, yellow-green sputum every morning.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Chronic productive cough with foul-smelling, yellow-green sputum every morning.

HISTORY OF PRESENT ILLNESS:
He reports coughing up large amounts of foul-smelling, yellow-green sputum every morning, especially after waking. He has had frequent respiratory infections over the past 5 years, each requiring antibiotics. He denies weight loss, fever, or night sweats.

PAST MEDICAL HISTORY:
Active Problem List:
- Stage IV lung cancer with extensive-stage disease
- Bronchiectasis
- Recurrent malignant pleural effusion
- Hyperlipidemia
- Asbestos exposure
- Paraneoplastic syndrome history (SIADH)

PAST SURGICAL HISTORY:
- Right-sided thoracentesis with chemical pleurodesis (for recurrent effusion)
- Central venous port placement
- CT-guided transthoracic needle biopsy of lung mass
- No colonoscopy or other cancer screening procedures performed to date

MEDICATIONS:
Home Medications:
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 68, heavy smoker)
- Mother: Emphysema
- Sibling: Coronary artery disease

SOCIAL HISTORY:
He smoked for 35 years but quit a decade ago.

PHYSICAL EXAMINATION:
On examination, he has digital clubbing and scattered rhonchi over both lung bases.

IMAGING:
Chest CT reveals dilated, thick-walled bronchi with mucus plugging and cystic changes in the lower lobes.

## Encounter 1: source 2020-03-15, practice 2023-09-08, outpatient, Internal Medicine

Chief complaint: Routine health maintenance visit with no major complaints.

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. James Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health maintenance visit with no major complaints.

HISTORY OF PRESENT ILLNESS:
The patient is a 65-year-old male with Stage IV lung cancer and bronchiectasis returning for routine health maintenance following his January Pulmonology evaluation for chronic productive cough. He has no major complaints.

PAST MEDICAL HISTORY:
Active Problem List:
- Bronchiectasis (diagnosed 2020-01-15)
- Stage IV lung cancer with extensive-stage disease
- Recurrent malignant pleural effusion
- Hyperlipidemia
- Asbestos exposure
- Paraneoplastic syndrome history (SIADH)

PAST SURGICAL HISTORY:
- Right-sided thoracentesis with chemical pleurodesis (for recurrent effusion)
- Central venous port placement
- CT-guided transthoracic needle biopsy of lung mass
- No colonoscopy or other cancer screening procedures performed to date

MEDICATIONS:
He takes a statin for hyperlipidemia and a low-dose aspirin

Home Medications:
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
He has no family history of cancer

SOCIAL HISTORY:
He has a 40-pack-year smoking history but quit 5 years ago
exercises occasionally

VITAL SIGNS:
His vital signs today are within normal limits

PHYSICAL EXAMINATION:
Physical examination is unremarkable

ASSESSMENT AND PLAN:
This patient is at greatest risk for death from

## Encounter 2: source 2020-04-15, practice 2023-10-10, outpatient, Pulmonology

Chief complaint: Chronic cough and progressive dyspnea.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Chronic cough and progressive dyspnea.

HISTORY OF PRESENT ILLNESS:
The patient is a 65-year-old male with Stage IV lung cancer and bronchiectasis who returns to pulmonology following prior evaluations in January and March. He presents with chronic cough and progressive dyspnea. He denies chest pain, hemoptysis, or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Bronchiectasis (diagnosed 2020-01-15)
- Suspected malignant neoplasm of lung (diagnosed 2020-03-15)
- Stage IV lung cancer with extensive-stage disease
- Recurrent malignant pleural effusion
- Hyperlipidemia
- Asbestos exposure
- Paraneoplastic syndrome history (SIADH)

PAST SURGICAL HISTORY:
- Right-sided thoracentesis with chemical pleurodesis (for recurrent effusion)
- Central venous port placement
- CT-guided transthoracic needle biopsy of lung mass
- No colonoscopy or other cancer screening procedures performed to date

MEDICATIONS:
Home Medications:
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 68, heavy smoker)
- Mother: Emphysema
- Sibling: Coronary artery disease

SOCIAL HISTORY:
He worked in naval shipyards for over 30 years and retired 10 years ago. He smoked a pack of cigarettes daily for 40 years but quit 5 years ago.

PHYSICAL EXAMINATION:
Physical examination reveals bilateral inspiratory crackles at the lung bases.

IMAGING:
A chest x-ray shows interstitial fibrosis and pleural plaques. High-resolution CT confirms diffuse subpleural reticulation and honeycombing.

OTHER STUDIES:
Pulmonary function testing shows a restrictive pattern.

PLAN:
His physician discusses long-term complications of his condition.

## Encounter 3: source 2020-05-15, practice 2023-11-18, ed, Emergency Medicine

Chief complaint: Progressive confusion, fatigue, and constipation over several days.

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive confusion, fatigue, and constipation over several days.

HISTORY OF PRESENT ILLNESS:
The patient with Stage IV lung cancer, seen one month ago, is brought by his wife to the emergency department with progressive confusion, fatigue, and constipation over the past several days.

PAST MEDICAL HISTORY:
Active Problem List:
- Bronchiectasis (diagnosed 2020-01-15)
- Suspected malignant neoplasm of lung (diagnosed 2020-03-15)
- Stage IV lung cancer with extensive-stage disease
- Recurrent malignant pleural effusion
- Hyperlipidemia
- Asbestos exposure
- Paraneoplastic syndrome history (SIADH)

PAST SURGICAL HISTORY:
- Right-sided thoracentesis with chemical pleurodesis (for recurrent effusion)
- Central venous port placement
- CT-guided transthoracic needle biopsy of lung mass
- No colonoscopy or other cancer screening procedures performed to date

MEDICATIONS:
Home Medications:
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 68, heavy smoker)
- Mother: Emphysema
- Sibling: Coronary artery disease

SOCIAL HISTORY:
He has a 40-pack-year smoking history but no regular medical care.

PHYSICAL EXAMINATION:
On exam, he appears lethargic and mildly dehydrated. Neurologic exam reveals slowed reflexes and difficulty concentrating.

LABORATORY DATA:
Laboratory studies show:
Calcium												   13.9 mg/dL
Phosphate 1.9 mg/dL
PTH Low

IMAGING:
A chest x-ray reveals a 4.5-cm mass in the right hilar region.

## Encounter 4: source 2021-03-15, practice 2024-09-14, ed, Emergency Medicine

Chief complaint: Dyspnea, cough, and chest pain worse when lying down.

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Michael Chang, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Dyspnea, cough, and chest pain worse when lying down.

HISTORY OF PRESENT ILLNESS:
Patient with Stage IV lung cancer and recurrent effusion returns with dyspnea, cough, and chest pain. He denies recent trauma or injury. Despite initial improvement, symptoms recur within a week, suggesting reaccumulation of the effusion.

PAST MEDICAL HISTORY:
Active Problem List:
- Bronchiectasis (diagnosed 2020-01-15)
- Suspected malignant neoplasm of lung (diagnosed 2020-03-15)
- Stage IV lung cancer with extensive-stage disease
- Recurrent malignant pleural effusion
- Hyperlipidemia
- Asbestos exposure
- Paraneoplastic syndrome history (SIADH)

PAST SURGICAL HISTORY:
- Right-sided thoracentesis with chemical pleurodesis (for recurrent effusion)
- Central venous port placement
- CT-guided transthoracic needle biopsy of lung mass
- No colonoscopy or other cancer screening procedures performed to date

MEDICATIONS:
Home Medications:
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 68, heavy smoker)
- Mother: Emphysema
- Sibling: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, 90 pack-year history, quit 2 years ago after multiple previous quit attempts. Alcohol: Social (occasional beer, 1-2 per month). Occupation: Retired naval shipyard worker (30 years service, retired age 55).

VITAL SIGNS:
On physical examination, his blood pressure is 100/60 mmHg, heart rate is 110 bpm, respiratory rate is 22 breaths per minute, and oxygen saturation is 92% on room air

PHYSICAL EXAMINATION:
Jugular venous pressure is elevated, and heart sounds are muffled.

IMAGING:
An echocardiogram reveals a large pericardial effusion with evidence of cardiac tamponade.

PATHOLOGY:
Pericardiocentesis is performed, and cytological analysis confirms the presence of malignant cells.

## Encounter 5: source 2021-09-15, practice 2025-03-08, inpatient, Oncology

Chief complaint: Fever, chills, and shortness of breath during chemotherapy admission.

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Emily Watson, MD
Department: Oncology
Visit Type: inpatient

CHIEF COMPLAINT:
Fever, chills, and shortness of breath during chemotherapy admission.

HISTORY OF PRESENT ILLNESS:
The patient is a 66-year-old male with Stage IV lung cancer with extensive-stage disease, recurrent malignant pleural effusion, and bronchiectasis who presents for scheduled chemotherapy. He is admitted to the hospital for chemotherapy via a recently placed central venous catheter. On the third day of admission, he develops a fever of 38.9°C (102°F), chills, and shortness of breath. He denies any cough, sputum production, or urinary symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Bronchiectasis (diagnosed 2020-01-15)
- Suspected malignant neoplasm of lung (diagnosed 2020-03-15)
- Malignant pleural effusion (diagnosed 2021-03-15)
- Stage IV lung cancer with extensive-stage disease
- Recurrent malignant pleural effusion
- Hyperlipidemia
- Asbestos exposure
- Paraneoplastic syndrome history (SIADH)

PAST SURGICAL HISTORY:
- Right-sided thoracentesis with chemical pleurodesis (for recurrent effusion)
- Central venous port placement
- CT-guided transthoracic needle biopsy of lung mass
- No colonoscopy or other cancer screening procedures performed to date

MEDICATIONS:
Home Medications:
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 68, heavy smoker)
- Mother: Emphysema
- Sibling: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, 90 pack-year history, quit 2 years ago after multiple previous quit attempts. Alcohol: Social (occasional beer, 1-2 per month). Occupation: Retired naval shipyard worker (30 years service, retired age 55).

VITAL SIGNS:
Blood pressure 110/70 mmHg, heart rate 110 beats/min, respiratory rate 22 breaths/min, and oxygen saturation 94% on room air

PHYSICAL EXAMINATION:
Physical examination reveals no obvious source of infection, but there is mild tenderness on palpation of the skin overlying the central venous catheter insertion site. There is no erythema or purulent discharge noted. Auscultation of the lungs reveals no crackles or wheezes.

LABORATORY DATA:
White blood cell count: 12,000/µL (normal 4,500-11,000/µL)
- Hemoglobin: 10.2 g/dL (normal 13.8-17.2 g/dL)
- Platelets: 250,000/µL (normal 150,000-450,000/µL)
- Blood cultures: Pending
- Urinalysis: Normal

IMAGING:
A chest X-ray shows no infiltrates or consolidations.

## Encounter 6: source 2022-05-15, practice 2025-11-16, ed, Emergency Medicine

Chief complaint: Increasing confusion and lethargy over the past 3 days.

ENCOUNTER NOTE
Date: 2022-05-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increasing confusion and lethargy over the past 3 days.

HISTORY OF PRESENT ILLNESS:
The patient is a 67-year-old male with Stage IV lung cancer, recurrent malignant pleural effusion, and history of paraneoplastic syndrome who returns to the emergency department with a presentation similar to his prior confusion episode. He is brought by his wife due to increasing confusion and lethargy over the past 3 days. She notes that he has been drinking normal amounts of fluid but has not had much appetite lately.

PAST MEDICAL HISTORY:
Active Problem List:
- Bronchiectasis (diagnosed 2020-01-15)
- Suspected malignant neoplasm of lung (diagnosed 2020-03-15)
- Malignant pleural effusion (diagnosed 2021-03-15)
- Catheter-related bloodstream infection (diagnosed 2021-09-15)
- Stage IV lung cancer with extensive-stage disease
- Recurrent malignant pleural effusion
- Hyperlipidemia
- Asbestos exposure
- Paraneoplastic syndrome history (SIADH)

He has a history of small cell lung cancer and underwent chemotherapy one month ago.

PAST SURGICAL HISTORY:
- Right-sided thoracentesis with chemical pleurodesis (for recurrent effusion)
- Central venous port placement
- CT-guided transthoracic needle biopsy of lung mass
- No colonoscopy or other cancer screening procedures performed to date

MEDICATIONS:
Home Medications:
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 68, heavy smoker)
- Mother: Emphysema
- Sibling: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, 90 pack-year history, quit 2 years ago after multiple previous quit attempts. Alcohol: Social (occasional beer, 1-2 per month). Occupation: Retired naval shipyard worker (30 years service, retired age 55).

VITAL SIGNS:
Vitals are normal.

PHYSICAL EXAMINATION:
On exam, he is oriented only to person and appears euvolemic with normal skin turgor and no signs of edema. A Foley catheter is placed.

## Encounter 7: source 2023-01-15, practice 2026-07-04, ed, Emergency Medicine

Chief complaint: Progressive shortness of breath and productive cough with occasional hemoptysis.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Lisa Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive shortness of breath and productive cough with occasional hemoptysis.

HISTORY OF PRESENT ILLNESS:
The patient returns with progressive shortness of breath in the setting of extensive-stage lung cancer and bronchiectasis. He reports occasional streaks of blood in his sputum but denies chest pain or fever. His appetite has decreased recently, and he has unintentionally lost 5 kg (11 lb) over the last 3 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Bronchiectasis (diagnosed 2020-01-15)
- Suspected malignant neoplasm of lung (diagnosed 2020-03-15)
- Malignant pleural effusion (diagnosed 2021-03-15)
- Catheter-related bloodstream infection (diagnosed 2021-09-15)
- Syndrome of inappropriate antidiuretic hormone secretion (SIADH) (diagnosed 2022-05-15)
- Stage IV lung cancer with extensive-stage disease
- Recurrent malignant pleural effusion
- Hyperlipidemia
- Asbestos exposure
- Paraneoplastic syndrome history (SIADH)

He has had multiple episodes of bronchitis in the past but has never been hospitalized for respiratory illness.

PAST SURGICAL HISTORY:
- Right-sided thoracentesis with chemical pleurodesis (for recurrent effusion)
- Central venous port placement
- CT-guided transthoracic needle biopsy of lung mass
- No colonoscopy or other cancer screening procedures performed to date

MEDICATIONS:
Home Medications:
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 68, heavy smoker)
- Mother: Emphysema
- Sibling: Coronary artery disease

SOCIAL HISTORY:
He has a 90-pack-year smoking history and quit two years ago.

VITAL SIGNS:
On physical examination, his temperature is 36.7°C (98.1°F), blood pressure is 132/84 mmHg, pulse is 94/min, and oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
Breath sounds are decreased over the right middle and lower lung fields with dullness to percussion. There are no crackles or wheezes. Trachea is palpably deviated toward the right side.

IMAGING:
Chest x-ray shows a dense opacity in the right mid- and lower lung zones with mediastinal shift toward the right. CT of the chest reveals a central right bronchial mass with associated volume loss in the right lung.
