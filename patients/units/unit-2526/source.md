# synthetic_hospital patient 2526

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 35 at first encounter, sex M, African American, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-03-02 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-09-15 | 2020-11-06 (Fri) | outpatient | Nephrology |
| 2 | 2021-09-15 | 2021-11-03 (Wed) | outpatient | Endocrinology |
| 3 | 2022-09-15 | 2022-10-31 (Mon) | ed | Emergency Medicine |
| 4 | 2023-09-15 | 2023-11-05 (Sun) | ed | Emergency Medicine |
| 5 | 2024-09-15 | 2024-11-05 (Tue) | ed | Emergency Medicine |
| 6 | 2025-09-15 | 2025-10-30 (Thu) | ed | Emergency Medicine |
| 7 | 2025-11-15 | 2025-12-31 (Wed) | icu | Pulmonary & Critical Care |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Safety compliance officer",
 "alcohol_use": "None",
 "family_history": [
  "Father with substance use disorder",
  "Maternal uncle with multiple myeloma",
  "Mother with hypertension"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "1 pack per day",
 "home_medications": [
  {
   "dose": "300 mg orally daily (intermittently compliant)",
   "name": "Tenofovir disoproxil fumarate"
  },
  {
   "dose": "5 mg orally daily (intermittently compliant)",
   "name": "Amlodipine"
  }
 ],
 "surgical_history": [
  "Mitral valve repair"
 ],
 "chronic_conditions": [
  "Chronic hepatitis B infection",
  "Hypertension",
  "Opioid use disorder (heroin)",
  "Cocaine use disorder",
  "Chronic kidney disease secondary to heroin-associated nephropathy",
  "Multiple myeloma",
  "Adrenal Cushing's syndrome",
  "Status post mitral valve repair"
 ],
 "age_at_first_encounter": 35
}
```

Primary diagnoses: ["Focal Segmental Glomerulosclerosis secondary to heroin use (Heroin-Associated Nephropathy)", "Multiple myeloma cast nephropathy", "Acute ST-elevation myocardial infarction (STEMI), inferior wall", "Atrial fibrillation with rapid ventricular response", "Adrenal Cushing's syndrome", "Refractory hypoxemia in ARDS", "History of intravenous drug use", "Type A Aortic Dissection"]
Comorbidities: ["Essential hypertension", "History of intravenous drug use", "Nephrotic syndrome (edema management)"]

## Encounter 0: source 2020-01-15, practice 2020-03-02, outpatient, Internal Medicine

Chief complaint: Routine health check-up with no acute complaints

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health check-up with no acute complaints

HISTORY OF PRESENT ILLNESS:
He has no complaints and reports feeling well.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic hepatitis B infection
- Hypertension
- Opioid use disorder (heroin)
- Cocaine use disorder
- Chronic kidney disease secondary to heroin-associated nephropathy
- Multiple myeloma
- Adrenal Cushing's syndrome
- Status post mitral valve repair

His past medical history is significant for intravenous drug use during his twenties, but he has been clean for the past 8 years.

PAST SURGICAL HISTORY:
- Mitral valve repair

MEDICATIONS:
Home Medications:
- Tenofovir disoproxil fumarate 300 mg orally daily (intermittently compliant)
- Amlodipine 5 mg orally daily (intermittently compliant)

ALLERGIES:
He has no known drug allergies

FAMILY HISTORY:
His family history is non-contributory.

SOCIAL HISTORY:
He is an office worker and leads a sedentary lifestyle.

VITAL SIGNS:
On examination, his vital signs are within normal limits

PHYSICAL EXAMINATION:
His physical examination is unremarkable

LABORATORY DATA:
As part of the workup, you order a hepatitis panel, the results of which are as follows:

- HBsAg: Negative
- Anti-HBs: Positive
- Anti-HBc: Positive

## Encounter 1: source 2020-09-15, practice 2020-11-06, outpatient, Nephrology

Chief complaint: Progressive swelling in lower extremities and scrotum over 4 months with 9 kg weight gain

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. James Wilson, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive swelling in lower extremities and scrotum over 4 months with 9 kg weight gain

HISTORY OF PRESENT ILLNESS:
The patient, with chronic hepatitis B, opioid use disorder, and heroin-associated nephropathy, returns to nephrology following his January 2020 visit. He reports a 4-month history of progressive swelling in his lower extremities and scrotum with 9 kg (20 lb) weight gain during this period. His last HIV and hepatitis C screenings were negative 6 weeks ago.

PAST MEDICAL HISTORY:
Active Problem List:
- History of intravenous drug use (diagnosed 2020-01-15)
- Chronic hepatitis B infection
- Hypertension
- Opioid use disorder (heroin)
- Cocaine use disorder
- Chronic kidney disease secondary to heroin-associated nephropathy
- Multiple myeloma
- Adrenal Cushing's syndrome
- Status post mitral valve repair

PAST SURGICAL HISTORY:
- Mitral valve repair

MEDICATIONS:
Home Medications:
- Tenofovir disoproxil fumarate 300 mg orally daily (intermittently compliant)
- Amlodipine 5 mg orally daily (intermittently compliant)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with substance use disorder
- Maternal uncle with multiple myeloma
- Mother with hypertension

SOCIAL HISTORY:
Chronic opioid use, including daily heroin injection for the past 8 years. He also smokes one pack of cigarettes daily. He denies alcohol use.

VITAL SIGNS:
Temperature is 37.2°C (99°F), blood pressure is 148/102 mm Hg, pulse is 98/min.

PHYSICAL EXAMINATION:
Cardiopulmonary exam reveals distant heart sounds and decreased breath sounds at the lung bases. There is a holosystolic murmur at the apex radiating to the axilla. The abdomen is distended with shifting dullness, and there is bilateral 3+ pitting edema extending up to the thighs. Scrotum is notably edematous.

LABORATORY DATA:
Labs:
BUN 34 mg/dL
Creatinine 3.1 mg/dL
Albumin 2.1 g/dL
Total cholesterol                                                305 mg/dL
ALT 32 U/L
AST 98 U/L
C3 63 mg/dL
C4 22 mg/dL
Urinalysis:
Protein                                                                      4+
RBCs                                                                         40/hpf
WBCs                                                                         10/hpf
Granular casts                                                               Occasional
RBC casts Absent
Leukocyte esterase                                                           1+
Serologies: HIV antigen/antibody: negative, Hepatitis B surface antigen: negative, Hepatitis B surface antibody: positive, Hepatitis C antibody: negative.

## Encounter 2: source 2021-09-15, practice 2021-11-03, outpatient, Endocrinology

Chief complaint: Progressive weight gain, proximal muscle weakness, and easy bruising over 4 months

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Maria Rodriguez, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive weight gain, proximal muscle weakness, and easy bruising over 4 months

HISTORY OF PRESENT ILLNESS:
The patient is a 36-year-old male with chronic hepatitis B, hypertension, opioid and cocaine use disorders complicated by heroin-associated nephropathy (diagnosed September 2020), and multiple myeloma who presents to the Endocrinology clinic. He reports a 4-month history of progressive weight gain, muscle weakness, and easy bruising.

PAST MEDICAL HISTORY:
Active Problem List:
- History of intravenous drug use (diagnosed 2020-01-15)
- Focal Segmental Glomerulosclerosis secondary to heroin use (Heroin-Associated Nephropathy) (diagnosed 2020-09-15)
- Chronic hepatitis B infection
- Hypertension
- Opioid use disorder (heroin)
- Cocaine use disorder
- Chronic kidney disease secondary to heroin-associated nephropathy
- Multiple myeloma
- Adrenal Cushing's syndrome
- Status post mitral valve repair

PAST SURGICAL HISTORY:
- Mitral valve repair

MEDICATIONS:
Home Medications:
- Tenofovir disoproxil fumarate 300 mg orally daily (intermittently compliant)
- Amlodipine 5 mg orally daily (intermittently compliant)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with substance use disorder
- Maternal uncle with multiple myeloma
- Mother with hypertension

SOCIAL HISTORY:
Smoking: 1 pack per day. Alcohol: None. Occupation: Safety compliance officer.

VITAL SIGNS:
Blood pressure is 152/94 mmHg

PHYSICAL EXAMINATION:
He has developed supraclavicular fat pads and facial rounding

LABORATORY DATA:
Laboratory evaluation reveals a serum ACTH level that is undetectable and a serum cortisol concentration that remains elevated throughout the day. A low-dose dexamethasone suppression test fails to lower his cortisol level.

ASSESSMENT AND PLAN:
The physician explains that the goal is to distinguish between adrenal and ectopic causes of cortisol excess.

A high-dose dexamethasone test is now administered. Imaging studies are scheduled to evaluate further.

## Encounter 3: source 2022-09-15, practice 2022-10-31, ed, Emergency Medicine

Chief complaint: Acute onset of chest pain, shortness of breath, and palpitations during work

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. Michael Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of chest pain, shortness of breath, and palpitations during work

HISTORY OF PRESENT ILLNESS:
The patient with adrenal Cushing's syndrome and chronic kidney disease presents with symptoms that started abruptly while conducting a safety briefing an hour ago.

PAST MEDICAL HISTORY:
Active Problem List:
- History of intravenous drug use (diagnosed 2020-01-15)
- Focal Segmental Glomerulosclerosis secondary to heroin use (Heroin-Associated Nephropathy) (diagnosed 2020-09-15)
- Adrenal Cushing's syndrome (diagnosed 2021-09-15)
- Chronic hepatitis B infection
- Hypertension
- Opioid use disorder (heroin)
- Cocaine use disorder
- Chronic kidney disease secondary to heroin-associated nephropathy
- Multiple myeloma
- Status post mitral valve repair

He denies any previous cardiac history.

PAST SURGICAL HISTORY:
- Mitral valve repair

MEDICATIONS:
Home Medications:
- Tenofovir disoproxil fumarate 300 mg orally daily (intermittently compliant)
- Amlodipine 5 mg orally daily (intermittently compliant)

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is significant for coronary artery disease in his father at the age of 50.

SOCIAL HISTORY:
Mentions high workplace stress.

VITAL SIGNS:
On examination, his blood pressure is 150/95 mmHg, heart rate is 120 bpm and irregular, respiratory rate is 22 breaths/min, and oxygen saturation is 97% on room air.

PHYSICAL EXAMINATION:
His chest is clear to auscultation, and there is no leg swelling.

LABORATORY DATA:
Laboratory values:
- Troponin I: <0.04 ng/mL (normal)
- B-type natriuretic peptide (BNP): 80 pg/mL (normal range: 0-100 pg/mL)
- Thyroid-stimulating hormone (TSH): 2.5 mIU/L (normal range: 0.4-4.0 mIU/L)
- Complete blood count (CBC): within normal limits

OTHER STUDIES:
An ECG shows atrial fibrillation with a rapid ventricular response.

## Encounter 4: source 2023-09-15, practice 2023-11-05, ed, Emergency Medicine

Chief complaint: Swelling in legs and frothy urine for 3 weeks

ENCOUNTER NOTE
Date: 2023-09-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Swelling in legs and frothy urine for 3 weeks

HISTORY OF PRESENT ILLNESS:
The patient with CKD and mitral repair presents with a 3-week history of swelling in his legs and occasional frothy urine, reporting no recent travel, drug use, or new medications.

PAST MEDICAL HISTORY:
Active Problem List:
- History of intravenous drug use (diagnosed 2020-01-15)
- Focal Segmental Glomerulosclerosis secondary to heroin use (Heroin-Associated Nephropathy) (diagnosed 2020-09-15)
- Adrenal Cushing's syndrome (diagnosed 2021-09-15)
- Atrial fibrillation with rapid ventricular response (diagnosed 2022-09-15)
- Chronic hepatitis B infection
- Hypertension
- Opioid use disorder (heroin)
- Cocaine use disorder
- Chronic kidney disease secondary to heroin-associated nephropathy
- Multiple myeloma
- Status post mitral valve repair

His past medical history is significant for chronic hepatitis B infection, for which he has been intermittently compliant with antiviral therapy.

PAST SURGICAL HISTORY:
- Mitral valve repair

MEDICATIONS:
Home Medications:
- Tenofovir disoproxil fumarate 300 mg orally daily (intermittently compliant)
- Amlodipine 5 mg orally daily (intermittently compliant)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with substance use disorder
- Maternal uncle with multiple myeloma
- Mother with hypertension

SOCIAL HISTORY:
Smoking: 1 pack per day. Alcohol: None. Occupation: Safety compliance officer.

VITAL SIGNS:
Blood pressure is 140/90 mmHg

PHYSICAL EXAMINATION:
3+ pitting edema in both lower extremities up to the knees

LABORATORY DATA:
Urinalysis reveals 4+ proteinuria, and serum albumin is 2.5 g/dL. His liver function tests are mildly elevated. Hepatitis serologies confirm active hepatitis B infection.

PATHOLOGY:
Renal biopsy shows thickening of the glomerular basement membrane with subepithelial deposits.

## Encounter 5: source 2024-09-15, practice 2024-11-05, ed, Emergency Medicine

Chief complaint: Severe substernal chest pain with pressure-like quality and palpitations

ENCOUNTER NOTE
Date: 2024-09-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe substernal chest pain with pressure-like quality and palpitations

HISTORY OF PRESENT ILLNESS:
The patient with prior atrial fibrillation and mitral valve repair presents with severe substernal chest pain that started 30 minutes ago while he was at a party. He describes the pain as pressure-like with palpitations and a sense of impending doom.

PAST MEDICAL HISTORY:
Active Problem List:
- History of intravenous drug use (diagnosed 2020-01-15)
- Focal Segmental Glomerulosclerosis secondary to heroin use (Heroin-Associated Nephropathy) (diagnosed 2020-09-15)
- Adrenal Cushing's syndrome (diagnosed 2021-09-15)
- Atrial fibrillation with rapid ventricular response (diagnosed 2022-09-15)
- Multiple myeloma cast nephropathy (diagnosed 2023-09-15)
- Chronic hepatitis B infection
- Hypertension
- Opioid use disorder (heroin)
- Cocaine use disorder
- Chronic kidney disease secondary to heroin-associated nephropathy
- Multiple myeloma
- Status post mitral valve repair

He has a past medical history of hypertension.

PAST SURGICAL HISTORY:
- Mitral valve repair

MEDICATIONS:
Home Medications:
- Tenofovir disoproxil fumarate 300 mg orally daily (intermittently compliant)
- Amlodipine 5 mg orally daily (intermittently compliant)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with substance use disorder
- Maternal uncle with multiple myeloma
- Mother with hypertension

SOCIAL HISTORY:
He admits to occasional cocaine use, including earlier in the evening

VITAL SIGNS:
On examination, his blood pressure is 170/100 mmHg, heart rate is 110 beats per minute, respiratory rate is 20 breaths per minute, and temperature is 37.2°C (99.0°F).

PHYSICAL EXAMINATION:
His pupils are dilated, and he appears agitated.

LABORATORY DATA:
Cardiac enzymes are pending.

OTHER STUDIES:
An ECG shows ST-segment elevations in leads II, III, and aVF.

## Encounter 6: source 2025-09-15, practice 2025-10-30, ed, Emergency Medicine

Chief complaint: Sudden-onset severe tearing chest pain radiating to the back after cocaine use

ENCOUNTER NOTE
Date: 2025-09-15
Provider: Dr. Emily Washington, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset severe tearing chest pain radiating to the back after cocaine use

HISTORY OF PRESENT ILLNESS:
The patient with cocaine use disorder and prior STEMI presents with sudden-onset severe tearing chest pain radiating to the back, describing it as sharp and tearing and admitting to cocaine use earlier in the day.

PAST MEDICAL HISTORY:
Active Problem List:
- History of intravenous drug use (diagnosed 2020-01-15)
- Focal Segmental Glomerulosclerosis secondary to heroin use (Heroin-Associated Nephropathy) (diagnosed 2020-09-15)
- Adrenal Cushing's syndrome (diagnosed 2021-09-15)
- Atrial fibrillation with rapid ventricular response (diagnosed 2022-09-15)
- Multiple myeloma cast nephropathy (diagnosed 2023-09-15)
- Acute ST-elevation myocardial infarction (STEMI), inferior wall (diagnosed 2024-09-15)
- Chronic hepatitis B infection
- Hypertension
- Opioid use disorder (heroin)
- Cocaine use disorder
- Chronic kidney disease secondary to heroin-associated nephropathy
- Multiple myeloma
- Status post mitral valve repair

PAST SURGICAL HISTORY:
- Mitral valve repair

MEDICATIONS:
Home Medications:
- Tenofovir disoproxil fumarate 300 mg orally daily (intermittently compliant)
- Amlodipine 5 mg orally daily (intermittently compliant)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with substance use disorder
- Maternal uncle with multiple myeloma
- Mother with hypertension

SOCIAL HISTORY:
Smoking: 1 pack per day. Alcohol: None. Occupation: Safety compliance officer.

VITAL SIGNS:
His blood pressure is 180/110 mmHg, heart rate is 105 bpm, and respiratory rate is 22 breaths per minute.

PHYSICAL EXAMINATION:
Physical examination reveals a diaphoretic patient in distress with decreased breath sounds on the left side of the chest. His extremities are cool to the touch with delayed capillary refill.

IMAGING:
A chest X-ray shows a widened mediastinum and a small left-sided pleural effusion. A bedside echocardiogram reveals an intimal flap in the ascending aorta.

## Encounter 7: source 2025-11-15, practice 2025-12-31, icu, Pulmonary & Critical Care

Chief complaint: Fever, dyspnea, and productive cough with yellow sputum progressing to respiratory failure

ENCOUNTER NOTE
Date: 2025-11-15
Provider: Dr. David Okafor, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Fever, dyspnea, and productive cough with yellow sputum progressing to respiratory failure

HISTORY OF PRESENT ILLNESS:
The patient is a 40-year-old male with chronic hepatitis B, multiple myeloma, adrenal Cushing's syndrome, and status post mitral valve repair and Type A aortic dissection repair two months prior, who presents with fever and respiratory distress. He reports a subjective fever at home and increasing difficulty in breathing, especially when lying flat. Despite receiving intravenous fluids and broad-spectrum antibiotics, the patient's respiratory status worsens, requiring intubation and mechanical ventilation. The ventilator is set to deliver a tidal volume of 6 mL/kg of predicted body weight, but the peak inspiratory pressures are high, and the oxygenation remains poor even with a FiO2 of 0.8.

PAST MEDICAL HISTORY:
Active Problem List:
- History of intravenous drug use (diagnosed 2020-01-15)
- Focal Segmental Glomerulosclerosis secondary to heroin use (Heroin-Associated Nephropathy) (diagnosed 2020-09-15)
- Adrenal Cushing's syndrome (diagnosed 2021-09-15)
- Atrial fibrillation with rapid ventricular response (diagnosed 2022-09-15)
- Multiple myeloma cast nephropathy (diagnosed 2023-09-15)
- Acute ST-elevation myocardial infarction (STEMI), inferior wall (diagnosed 2024-09-15)
- Type A Aortic Dissection (diagnosed 2025-09-15)
- Chronic hepatitis B infection
- Hypertension
- Opioid use disorder (heroin)
- Cocaine use disorder
- Chronic kidney disease secondary to heroin-associated nephropathy
- Multiple myeloma
- Status post mitral valve repair

PAST SURGICAL HISTORY:
- Mitral valve repair

MEDICATIONS:
Home Medications:
- Tenofovir disoproxil fumarate 300 mg orally daily (intermittently compliant)
- Amlodipine 5 mg orally daily (intermittently compliant)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with substance use disorder
- Maternal uncle with multiple myeloma
- Mother with hypertension

SOCIAL HISTORY:
Smoking: 1 pack per day. Alcohol: None. Occupation: Safety compliance officer.

VITAL SIGNS:
On examination, he is tachypneic with a respiratory rate of 28 breaths per minute, febrile with a temperature of 38.9°C (102°F), and hypoxic with an oxygen saturation of 88% on room air.

PHYSICAL EXAMINATION:
Lung auscultation reveals bilateral crackles.

LABORATORY DATA:
Arterial blood gas on room air shows:

- pH: 7.48 (7.35-7.45)
- PaCO2: 30 mmHg (35-45 mmHg)
- PaO2: 58 mmHg (80-100 mmHg)
- HCO3-: 24 mEq/L (22-26 mEq/L)

The patient's white blood cell count is 18,000/uL with a left shift.

IMAGING:
Chest radiography shows bilateral alveolar infiltrates.
