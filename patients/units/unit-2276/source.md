# synthetic_hospital patient 2276

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 66 at first encounter, sex M, Non-Hispanic White, Medicare with supplemental private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-02-13 (Mon) | outpatient | Cardiology |
| 1 | 2020-05-15 | 2023-06-14 (Wed) | outpatient | Orthopedics |
| 2 | 2020-11-15 | 2023-12-08 (Fri) | outpatient | Vascular Medicine |
| 3 | 2022-03-15 | 2025-03-27 (Thu) | ed | Emergency Medicine |
| 4 | 2022-09-15 | 2025-10-13 (Mon) | outpatient | Pulmonology |
| 5 | 2023-02-15 | 2026-03-08 (Sun) | ed | Emergency Medicine |
| 6 | 2023-03-15 | 2026-03-27 (Fri) | icu | Critical Care |
| 7 | 2023-05-15 | 2026-06-12 (Fri) | outpatient | Neurology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare with supplemental private insurance",
 "occupation": "Retired construction worker",
 "alcohol_use": "Social use (1-2 drinks per month)",
 "family_history": [
  "Father: Myocardial infarction at age 68",
  "Mother: Type 2 diabetes mellitus",
  "Brother: Chronic obstructive pulmonary disease",
  "Paternal uncle: Lung cancer"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "45 pack-year history, currently smoking 1 pack per day",
 "home_medications": [
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "40 mg PO daily at bedtime",
   "name": "Atorvastatin"
  },
  {
   "dose": "20 units subcutaneous at bedtime",
   "name": "Insulin Glargine"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "90 mcg (2 puffs) inhaled every 4-6 hours as needed for shortness of breath",
   "name": "Albuterol HFA inhaler"
  }
 ],
 "surgical_history": [
  "Cardiac catheterization with coronary angiography (age 61)",
  "Exploratory laparotomy with small bowel resection and primary anastomosis for perforated viscus"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus",
  "Hypertension",
  "Hyperlipidemia",
  "Chronic Kidney Disease",
  "Chronic Obstructive Pulmonary Disease",
  "Peripheral Artery Disease",
  "Coronary Artery Disease (s/p cardiac catheterization)",
  "History of Rheumatic Fever",
  "Dupuytren Contracture"
 ],
 "age_at_first_encounter": 66
}
```

Primary diagnoses: ["Peripheral arterial disease with intermittent claudication", "Pneumococcal pneumonia", "History of rheumatic fever", "Dupuytren contracture", "Septic shock", "Suspected malignant neoplasm of lung", "Intestinal perforation", "Aminoglycoside ototoxicity"]
Comorbidities: ["Chronic obstructive pulmonary disease", "Tobacco use disorder", "Urinary tract infection", "Hyperlipidemia", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2023-02-13, outpatient, Cardiology

Chief complaint: Routine follow-up visit for cardiac monitoring

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up visit for cardiac monitoring

HISTORY OF PRESENT ILLNESS:
He reports no current chest pain but mentions that he gets mildly winded after climbing stairs.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- Chronic Kidney Disease
- Chronic Obstructive Pulmonary Disease
- Peripheral Artery Disease
- Coronary Artery Disease (s/p cardiac catheterization)
- History of Rheumatic Fever
- Dupuytren Contracture

He has a medical history of hypertension, type 2 diabetes mellitus, and hyperlipidemia. Five years ago, he was hospitalized for chest pain and underwent cardiac catheterization at that time.

PAST SURGICAL HISTORY:
- Cardiac catheterization with coronary angiography (age 61)
- Exploratory laparotomy with small bowel resection and primary anastomosis for perforated viscus

MEDICATIONS:
He currently takes lisinopril, metformin, and atorvastatin.

Home Medications:
- Insulin Glargine 20 units subcutaneous at bedtime
- Aspirin 81 mg PO daily
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4-6 hours as needed for shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Type 2 diabetes mellitus
- Brother: Chronic obstructive pulmonary disease
- Paternal uncle: Lung cancer

SOCIAL HISTORY:
He also has a 30-pack-year smoking history and continues to smoke one pack daily.

VITAL SIGNS:
His blood pressure is 132/82 mm Hg, pulse is 76/min, and BMI is 31 kg/m².

## Encounter 1: source 2020-05-15, practice 2023-06-14, outpatient, Orthopedics

Chief complaint: Difficulty straightening fingers and progressive hand contracture

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Marcus Rodriguez, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Difficulty straightening fingers and progressive hand contracture

HISTORY OF PRESENT ILLNESS:
The patient presents with hand contractures following his Cardiology visit. He reports that this problem has been gradually worsening over the past year. He denies any trauma or previous similar issues.

PAST MEDICAL HISTORY:
Active Problem List:
- History of rheumatic fever (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- Chronic Kidney Disease
- Chronic Obstructive Pulmonary Disease
- Peripheral Artery Disease
- Coronary Artery Disease (s/p cardiac catheterization)
- Dupuytren Contracture

His medical history is significant for type 2 diabetes mellitus and chronic obstructive pulmonary disease (COPD).

PAST SURGICAL HISTORY:
- Cardiac catheterization with coronary angiography (age 61)
- Exploratory laparotomy with small bowel resection and primary anastomosis for perforated viscus

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO daily at bedtime
- Insulin Glargine 20 units subcutaneous at bedtime
- Aspirin 81 mg PO daily
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4-6 hours as needed for shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Type 2 diabetes mellitus
- Brother: Chronic obstructive pulmonary disease
- Paternal uncle: Lung cancer

SOCIAL HISTORY:
Smoking: 45 pack-year history, currently smoking 1 pack per day. Alcohol: Social use (1-2 drinks per month). Occupation: Retired construction worker.

PHYSICAL EXAMINATION:
On physical examination, you notice puckering of the skin over the palmar surface of his hands and palpable fibrotic nodules along the course of the flexor tendons. There is a flexion deformity at the metacarpophalangeal and proximal interphalangeal joints of the fourth and fifth fingers of both hands. The patient is unable to fully extend these fingers. No erythema, warmth, or tenderness is noted over the affected areas. The rest of the musculoskeletal examination is unremarkable.

## Encounter 2: source 2020-11-15, practice 2023-12-08, outpatient, Vascular Medicine

Chief complaint: Pain and tightness in lower legs with walking that improves with rest

ENCOUNTER NOTE
Date: 2020-11-15
Provider: Dr. James Patterson, MD
Department: Vascular Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Pain and tightness in lower legs with walking that improves with rest

HISTORY OF PRESENT ILLNESS:
The patient with cardiovascular disease and Dupuytren evaluation six months prior reports the discomfort has gradually worsened over the past 6 months, limiting his ability to go on daily walks.

PAST MEDICAL HISTORY:
Active Problem List:
- History of rheumatic fever (diagnosed 2020-01-15)
- Dupuytren contracture (diagnosed 2020-05-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- Chronic Kidney Disease
- Chronic Obstructive Pulmonary Disease
- Peripheral Artery Disease
- Coronary Artery Disease (s/p cardiac catheterization)

PAST SURGICAL HISTORY:
- Cardiac catheterization with coronary angiography (age 61)
- Exploratory laparotomy with small bowel resection and primary anastomosis for perforated viscus

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO daily at bedtime
- Insulin Glargine 20 units subcutaneous at bedtime
- Aspirin 81 mg PO daily
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4-6 hours as needed for shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Type 2 diabetes mellitus
- Brother: Chronic obstructive pulmonary disease
- Paternal uncle: Lung cancer

SOCIAL HISTORY:
Smoking

REVIEW OF SYSTEMS:
He denies chest pain, dyspnea, or swelling of the legs.

PHYSICAL EXAMINATION:
On physical examination, his legs appear intact, but pulses are reduced at the feet. Capillary refill is slightly delayed, and the skin over his lower legs appears dry. Neurologic exam shows intact sensation and strength.

OTHER STUDIES:
Ankle-brachial index testing is performed and returns a value of 0.67.

PLAN:
The patient is started on a statin, antiplatelet therapy, and a supervised exercise program. Smoking cessation is also discussed. He now asks about additional pharmacologic options that could help him walk longer distances without discomfort.

## Encounter 3: source 2022-03-15, practice 2025-03-27, ed, Emergency Medicine

Chief complaint: Two days of fever, chills, and productive cough with rust-colored sputum

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Emily Watson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two days of fever, chills, and productive cough with rust-colored sputum

HISTORY OF PRESENT ILLNESS:
Patient has COPD and rheumatic fever. He reports rust-colored sputum and pleuritic chest pain localized to the right side.

PAST MEDICAL HISTORY:
Active Problem List:
- History of rheumatic fever (diagnosed 2020-01-15)
- Dupuytren contracture (diagnosed 2020-05-15)
- Peripheral arterial disease with intermittent claudication (diagnosed 2020-11-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- Chronic Kidney Disease
- Chronic Obstructive Pulmonary Disease
- Peripheral Artery Disease
- Coronary Artery Disease (s/p cardiac catheterization)

He has not received any vaccinations in several years.

PAST SURGICAL HISTORY:
- Cardiac catheterization with coronary angiography (age 61)
- Exploratory laparotomy with small bowel resection and primary anastomosis for perforated viscus

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO daily at bedtime
- Insulin Glargine 20 units subcutaneous at bedtime
- Aspirin 81 mg PO daily
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4-6 hours as needed for shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Type 2 diabetes mellitus
- Brother: Chronic obstructive pulmonary disease
- Paternal uncle: Lung cancer

SOCIAL HISTORY:
50-pack-year smoking history

VITAL SIGNS:
Temperature is 38.9°C (102°F), respiratory rate is 26/min, and oxygen saturation is 92% on room air

PHYSICAL EXAMINATION:
Auscultation reveals bronchial breath sounds and dullness to percussion over the right lower lobe

LABORATORY DATA:
Sputum Gram stain reveals Gram-positive, lancet-shaped diplococci, and culture confirms Strep pneumoniae as the causative organism.

IMAGING:
A chest x-ray shows consolidation of the right lower lobe

## Encounter 4: source 2022-09-15, practice 2025-10-13, outpatient, Pulmonology

Chief complaint: Three-month history of persistent cough, intermittent fevers, and 10-pound weight loss

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. Robert Kim, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of persistent cough, intermittent fevers, and 10-pound weight loss

HISTORY OF PRESENT ILLNESS:
The patient with COPD returns following March 2022 pneumonia. He reports three episodes of pneumonia in the past year, each treated with antibiotics with temporary resolution of symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- History of rheumatic fever (diagnosed 2020-01-15)
- Dupuytren contracture (diagnosed 2020-05-15)
- Peripheral arterial disease with intermittent claudication (diagnosed 2020-11-15)
- Pneumococcal pneumonia (diagnosed 2022-03-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- Chronic Kidney Disease
- Chronic Obstructive Pulmonary Disease
- Peripheral Artery Disease
- Coronary Artery Disease (s/p cardiac catheterization)

PAST SURGICAL HISTORY:
- Cardiac catheterization with coronary angiography (age 61)
- Exploratory laparotomy with small bowel resection and primary anastomosis for perforated viscus

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO daily at bedtime
- Insulin Glargine 20 units subcutaneous at bedtime
- Aspirin 81 mg PO daily
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4-6 hours as needed for shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Type 2 diabetes mellitus
- Brother: Chronic obstructive pulmonary disease
- Paternal uncle: Lung cancer

SOCIAL HISTORY:
45-pack-year smoking history

PHYSICAL EXAMINATION:
On examination, he appears cachectic. Auscultation of the lungs reveals decreased breath sounds and dullness to percussion at the right lower lobe. No lymphadenopathy is palpable.

LABORATORY DATA:
Laboratory tests reveal:

- WBC: 12,000 /µL (Normal: 4,500-11,000 /µL)
- Hemoglobin: 13.2 g/dL (Normal: 13.8-17.2 g/dL)
- Platelets: 250,000 /µL (Normal: 150,000-450,000 /µL)
- Sodium: 140 mEq/L (Normal: 135-145 mEq/L)
- Potassium: 4.2 mEq/L (Normal: 3.5-5.1 mEq/L)
- Creatinine: 1.0 mg/dL (Normal: 0.6-1.2 mg/dL)

IMAGING:
A chest X-ray shows a right lower lobe consolidation that has not changed since the last episode of pneumonia.

## Encounter 5: source 2023-02-15, practice 2026-03-08, ed, Emergency Medicine

Chief complaint: Fever, chills, and confusion with decreased responsiveness

ENCOUNTER NOTE
Date: 2023-02-15
Provider: Dr. Lisa Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, chills, and confusion with decreased responsiveness

HISTORY OF PRESENT ILLNESS:
This 69-year-old male with CAD, COPD, and history of pneumococcal pneumonia and suspected lung malignancy presents to emergency department with a 2-day history of fever, chills, and confusion. His wife reports that he has been less responsive and has not been eating well.

PAST MEDICAL HISTORY:
Active Problem List:
- History of rheumatic fever (diagnosed 2020-01-15)
- Dupuytren contracture (diagnosed 2020-05-15)
- Peripheral arterial disease with intermittent claudication (diagnosed 2020-11-15)
- Pneumococcal pneumonia (diagnosed 2022-03-15)
- Suspected malignant neoplasm of lung (diagnosed 2022-09-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- Chronic Kidney Disease
- Chronic Obstructive Pulmonary Disease
- Peripheral Artery Disease
- Coronary Artery Disease (s/p cardiac catheterization)

PAST SURGICAL HISTORY:
- Cardiac catheterization with coronary angiography (age 61)
- Exploratory laparotomy with small bowel resection and primary anastomosis for perforated viscus

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO daily at bedtime
- Insulin Glargine 20 units subcutaneous at bedtime
- Aspirin 81 mg PO daily
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4-6 hours as needed for shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Type 2 diabetes mellitus
- Brother: Chronic obstructive pulmonary disease
- Paternal uncle: Lung cancer

SOCIAL HISTORY:
Smoking: 45 pack-year history, currently smoking 1 pack per day. Alcohol: Social use (1-2 drinks per month). Occupation: Retired construction worker.

VITAL SIGNS:
His temperature is 39.2°C (102.5°F), blood pressure is 90/50 mmHg, heart rate is 120 beats per minute, and respiratory rate is 22 breaths per minute.

PHYSICAL EXAMINATION:
Physical examination reveals dry mucous membranes and poor skin turgor.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 18,000/mm³
- Hemoglobin: 12 g/dL
- Platelet count: 150,000/mm³
- Serum creatinine: 2.2 mg/dL (baseline 1.4 mg/dL)
- Blood urea nitrogen: 40 mg/dL
- Lactic acid: 4 mmol/L
- Urinalysis: Positive for leukocyte esterase and nitrites

## Encounter 6: source 2023-03-15, practice 2026-03-27, icu, Critical Care

Chief complaint: Post-operative management following laparotomy for perforated bowel

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. David Okafor, MD
Department: Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Post-operative management following laparotomy for perforated bowel

HISTORY OF PRESENT ILLNESS:
This 69-year-old male with Type 2 Diabetes Mellitus, Chronic Kidney Disease, Coronary Artery Disease, and suspected malignant neoplasm of the lung, who was hospitalized one month ago for septic shock, is admitted to the intensive care unit (ICU) following a laparotomy for a perforated bowel. Post-operatively, the patient is mechanically ventilated and sedated. On the second day of admission, the patient remains hemodynamically stable with a mean arterial pressure of 75 mmHg and is afebrile. The patient's daughter asks about his nutritional requirements and expresses concern about his prolonged fasting state.

PAST MEDICAL HISTORY:
Active Problem List:
- History of rheumatic fever (diagnosed 2020-01-15)
- Dupuytren contracture (diagnosed 2020-05-15)
- Peripheral arterial disease with intermittent claudication (diagnosed 2020-11-15)
- Pneumococcal pneumonia (diagnosed 2022-03-15)
- Suspected malignant neoplasm of lung (diagnosed 2022-09-15)
- Septic shock (diagnosed 2023-02-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- Chronic Kidney Disease
- Chronic Obstructive Pulmonary Disease
- Peripheral Artery Disease
- Coronary Artery Disease (s/p cardiac catheterization)

PAST SURGICAL HISTORY:
- Cardiac catheterization with coronary angiography (age 61)
- Exploratory laparotomy with small bowel resection and primary anastomosis for perforated viscus

MEDICATIONS:
He takes intravenous piperacillin-tazobactam, insulin glargine, and as-needed albuterol nebulizations.

Home Medications:
- Lisinopril 20 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO daily at bedtime
- Aspirin 81 mg PO daily
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4-6 hours as needed for shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Type 2 diabetes mellitus
- Brother: Chronic obstructive pulmonary disease
- Paternal uncle: Lung cancer

SOCIAL HISTORY:
Smoking: 45 pack-year history, currently smoking 1 pack per day. Alcohol: Social use (1-2 drinks per month). Occupation: Retired construction worker.

VITAL SIGNS:
Hemodynamically stable with a mean arterial pressure of 75 mmHg and is afebrile

## Encounter 7: source 2023-05-15, practice 2026-06-12, outpatient, Neurology

Chief complaint: Unsteadiness when walking and difficulty hearing, especially in noisy environments

ENCOUNTER NOTE
Date: 2023-05-15
Provider: Dr. Maria Santos, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Unsteadiness when walking and difficulty hearing, especially in noisy environments

HISTORY OF PRESENT ILLNESS:
The patient presents to neurology following hospitalization for septic shock and intestinal perforation. These symptoms have progressively worsened over the past two months. He denies recent head trauma or ear infections.

PAST MEDICAL HISTORY:
Active Problem List:
- History of rheumatic fever (diagnosed 2020-01-15)
- Dupuytren contracture (diagnosed 2020-05-15)
- Peripheral arterial disease with intermittent claudication (diagnosed 2020-11-15)
- Pneumococcal pneumonia (diagnosed 2022-03-15)
- Suspected malignant neoplasm of lung (diagnosed 2022-09-15)
- Septic shock (diagnosed 2023-02-15)
- Intestinal perforation (diagnosed 2023-03-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- Chronic Kidney Disease
- Chronic Obstructive Pulmonary Disease
- Peripheral Artery Disease
- Coronary Artery Disease (s/p cardiac catheterization)

PAST SURGICAL HISTORY:
- Cardiac catheterization with coronary angiography (age 61)
- Exploratory laparotomy with small bowel resection and primary anastomosis for perforated viscus

MEDICATIONS:
His current medications include metformin, lisinopril, and gentamicin for a recent complicated urinary tract infection.

Home Medications:
- Atorvastatin 40 mg PO daily at bedtime
- Insulin Glargine 20 units subcutaneous at bedtime
- Aspirin 81 mg PO daily
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4-6 hours as needed for shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Type 2 diabetes mellitus
- Brother: Chronic obstructive pulmonary disease
- Paternal uncle: Lung cancer

SOCIAL HISTORY:
Smoking: 45 pack-year history, currently smoking 1 pack per day. Alcohol: Social use (1-2 drinks per month). Occupation: Retired construction worker.

PHYSICAL EXAMINATION:
On physical examination, the patient has a normal otoscopic exam, but exhibits a positive Romberg sign and decreased bilateral hearing acuity.

LABORATORY DATA:
His most recent laboratory values are as follows:

- Serum creatinine: 2.8 mg/dL (elevated)
- Blood urea nitrogen (BUN): 44 mg/dL (elevated)
- Fasting blood glucose: 146 mg/dL
- Hemoglobin A1c: 7.2%
- Urinalysis: No abnormalities detected
