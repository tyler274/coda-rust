# synthetic_hospital patient 2278

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 65 at first encounter, sex M, Non-Hispanic White, Medicare Part A and B.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-11-15 (Fri) | ed | Emergency Medicine |
| 1 | 2020-01-15 | 2024-11-27 (Wed) | icu | Intensive Care Unit |
| 2 | 2021-07-15 | 2026-05-19 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Sulfa antibiotics - hives",
  "Iodinated contrast - anaphylaxis (premedicate)"
 ],
 "insurance": "Medicare Part A and B",
 "occupation": "Retired construction foreman",
 "alcohol_use": "Occasional social use, 1-2 beers per week",
 "family_history": [
  "Father: Lung cancer, deceased age 67 (heavy smoker)",
  "Mother: Type 2 diabetes mellitus and chronic kidney disease, alive age 88",
  "Older brother: Myocardial infarction at age 62"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 50 pack-years, quit 18 months ago upon diagnosis of lung cancer",
 "home_medications": [
  {
   "dose": "2 puffs inhaled every 6 hours as needed for shortness of breath",
   "name": "Albuterol-ipratropium inhaler (Combivent)"
  },
  {
   "dose": "500 mg PO twice daily with meals (held if NPO or eGFR <30)",
   "name": "Metformin"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "5 mg PO every 6 hours as needed for cancer-related pain",
   "name": "Oxycodone immediate release"
  },
  {
   "dose": "2 L/min continuous via nasal cannula",
   "name": "Home oxygen therapy"
  }
 ],
 "surgical_history": [
  "CT-guided core needle biopsy of left upper lobe mass (18 months prior)",
  "Mediastinoscopy with lymph node sampling (18 months prior)",
  "Left femoral-popliteal bypass with synthetic graft (5 years prior)",
  "Diagnostic cardiac catheterization (6 years prior)"
 ],
 "chronic_conditions": [
  "Stage IV non-small cell lung cancer (adenocarcinoma)",
  "Severe chronic obstructive pulmonary disease (COPD) with chronic bronchitis phenotype",
  "Type 2 diabetes mellitus with diabetic nephropathy",
  "Chronic kidney disease, stage 3b",
  "Peripheral arterial disease",
  "Systemic hypertension"
 ],
 "age_at_first_encounter": 65
}
```

Primary diagnoses: ["Septic shock", "Simple pneumothorax"]
Comorbidities: ["Lactic acidosis", "Acute kidney injury", "Type 2 diabetes mellitus", "Leukocytosis"]

## Encounter 0: source 2020-01-15, practice 2024-11-15, ed, Emergency Medicine

Chief complaint: Altered mental status and decreased oral intake for the past day

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Altered mental status and decreased oral intake for the past day

HISTORY OF PRESENT ILLNESS:
She reports he has had a worsening productive cough for several days and had difficulty walking this morning. He has not had any recent travel or exposure to sick contacts.

PAST MEDICAL HISTORY:
Active Problem List:
- Stage IV non-small cell lung cancer (adenocarcinoma)
- Severe chronic obstructive pulmonary disease (COPD) with chronic bronchitis phenotype
- Type 2 diabetes mellitus with diabetic nephropathy
- Chronic kidney disease, stage 3b
- Peripheral arterial disease
- Systemic hypertension

PAST SURGICAL HISTORY:
- CT-guided core needle biopsy of left upper lobe mass (18 months prior)
- Mediastinoscopy with lymph node sampling (18 months prior)
- Left femoral-popliteal bypass with synthetic graft (5 years prior)
- Diagnostic cardiac catheterization (6 years prior)

MEDICATIONS:
He takes albuterol-ipratropium inhalers and metformin.

Home Medications:
- Albuterol-ipratropium inhaler (Combivent) 2 puffs inhaled every 6 hours as needed for shortness of breath
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily
- Amlodipine 5 mg PO daily
- Oxycodone immediate release 5 mg PO every 6 hours as needed for cancer-related pain
- Home oxygen therapy 2 L/min continuous via nasal cannula

ALLERGIES:
Sulfa antibiotics - hives, Iodinated contrast - anaphylaxis (premedicate)

FAMILY HISTORY:
- Father: Lung cancer, deceased age 67 (heavy smoker)
- Mother: Type 2 diabetes mellitus and chronic kidney disease, alive age 88
- Older brother: Myocardial infarction at age 62

SOCIAL HISTORY:
Smoking: Former smoker, 50 pack-years, quit 18 months ago upon diagnosis of lung cancer. Alcohol: Occasional social use, 1-2 beers per week. Occupation: Retired construction foreman.

VITAL SIGNS:
On arrival, his temperature is 35.5°C (95.9°F), pulse is 122/min, respiratory rate is 26/min, and blood pressure is 84/52 mm Hg. Oxygen saturation is 90% on room air.

PHYSICAL EXAMINATION:
He appears confused and is disoriented to place and time. Physical examination reveals dry mucous membranes, decreased breath sounds in the right lower lobe, and crackles in both bases. No rash or trauma is observed. There is no jugular venous distention or peripheral edema. Heart sounds are normal.

LABORATORY DATA:
Leukocytes 23,000/mm³ (2% neutrophils, 14% bands)
Lactate 4.5 mmol/L
BUN 28 mg/dL
Creatinine 2.1 mg/dL
Glucose 148 mg/dL
ABG on room air PaCO₂ 32 mm Hg, HCO₃⁻ 15 mEq/L, pH 7.30

IMAGING:
Chest x-ray reveals a large right lower lobe consolidation with air bronchograms.

## Encounter 1: source 2020-01-15, practice 2024-11-27, icu, Intensive Care Unit

Chief complaint: Admission to intensive care unit for sepsis secondary to pneumonia complicated by pneumothorax

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. James Okafor, MD
Department: Intensive Care Unit
Visit Type: icu

CHIEF COMPLAINT:
Admission to intensive care unit for sepsis secondary to pneumonia complicated by pneumothorax

HISTORY OF PRESENT ILLNESS:
The patient is a 65-year-old male with Stage IV non-small cell lung cancer, severe COPD on home oxygen therapy, and chronic kidney disease stage 3b who was evaluated in the emergency department earlier today for altered mental status and septic shock, and is now admitted to the intensive care unit. He is started on broad-spectrum antibiotics and requires central venous access for administration of vasopressors. A subclavian central venous catheter is placed without immediate complications. The patient's blood pressure improves with fluid resuscitation and vasopressors. However, 30 minutes after catheter placement, the patient develops sudden onset of shortness of breath, chest pain, and a drop in oxygen saturation to 88% on room air.

PAST MEDICAL HISTORY:
Active Problem List:
- Septic shock (diagnosed 2020-01-15)
- Stage IV non-small cell lung cancer (adenocarcinoma)
- Severe chronic obstructive pulmonary disease (COPD) with chronic bronchitis phenotype
- Type 2 diabetes mellitus with diabetic nephropathy
- Chronic kidney disease, stage 3b
- Peripheral arterial disease
- Systemic hypertension

PAST SURGICAL HISTORY:
- CT-guided core needle biopsy of left upper lobe mass (18 months prior)
- Mediastinoscopy with lymph node sampling (18 months prior)
- Left femoral-popliteal bypass with synthetic graft (5 years prior)
- Diagnostic cardiac catheterization (6 years prior)

MEDICATIONS:
Home Medications:
- Albuterol-ipratropium inhaler (Combivent) 2 puffs inhaled every 6 hours as needed for shortness of breath
- Metformin 500 mg PO twice daily with meals (held if NPO or eGFR <30)
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily
- Amlodipine 5 mg PO daily
- Oxycodone immediate release 5 mg PO every 6 hours as needed for cancer-related pain
- Home oxygen therapy 2 L/min continuous via nasal cannula

ALLERGIES:
Sulfa antibiotics - hives, Iodinated contrast - anaphylaxis (premedicate)

FAMILY HISTORY:
- Father: Lung cancer, deceased age 67 (heavy smoker)
- Mother: Type 2 diabetes mellitus and chronic kidney disease, alive age 88
- Older brother: Myocardial infarction at age 62

SOCIAL HISTORY:
Smoking: Former smoker, 50 pack-years, quit 18 months ago upon diagnosis of lung cancer. Alcohol: Occasional social use, 1-2 beers per week. Occupation: Retired construction foreman.

VITAL SIGNS:
He is hypotensive with a blood pressure of 85/50 mmHg, tachycardic with a heart rate of 120 bpm, and febrile with a temperature of 38.9°C (102°F)

PHYSICAL EXAMINATION:
On examination, breath sounds are diminished on the right side, and there is tracheal deviation to the left.

LABORATORY DATA:
Laboratory values:
- Hemoglobin: 12.2 g/dL
- White blood cell count: 18,000/mm^3
- Platelets: 250,000/mm^3
- Sodium: 138 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 1.1 mg/dL
- Blood urea nitrogen: 28 mg/dL

IMAGING:
A stat chest x-ray is performed. Chest x-ray shows the catheter tip projecting over the right apex outside the silhouette of the superior vena cava.

## Encounter 2: source 2021-07-15, practice 2026-05-19, ed, Emergency Medicine

Chief complaint: Fever, chills, and confusion for 2 days

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Maria Gonzalez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, chills, and confusion for 2 days

HISTORY OF PRESENT ILLNESS:
The patient, with Stage IV non-small cell lung cancer and severe COPD on continuous home oxygen, returns to the emergency department eighteen months following his prior admission for septic shock complicated by pneumothorax. His wife reports that he has not been himself, appearing lethargic and disoriented.

PAST MEDICAL HISTORY:
Active Problem List:
- Septic shock (diagnosed 2020-01-15)
- Simple pneumothorax (diagnosed 2020-01-15)
- Stage IV non-small cell lung cancer (adenocarcinoma)
- Severe chronic obstructive pulmonary disease (COPD) with chronic bronchitis phenotype
- Type 2 diabetes mellitus with diabetic nephropathy
- Chronic kidney disease, stage 3b
- Peripheral arterial disease
- Systemic hypertension

He has a history of peripheral vascular disease.

PAST SURGICAL HISTORY:
He underwent a femoral-popliteal bypass 1 year ago

MEDICATIONS:
Home Medications:
- Albuterol-ipratropium inhaler (Combivent) 2 puffs inhaled every 6 hours as needed for shortness of breath
- Metformin 500 mg PO twice daily with meals (held if NPO or eGFR <30)
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily
- Amlodipine 5 mg PO daily
- Oxycodone immediate release 5 mg PO every 6 hours as needed for cancer-related pain
- Home oxygen therapy 2 L/min continuous via nasal cannula

ALLERGIES:
Sulfa antibiotics - hives, Iodinated contrast - anaphylaxis (premedicate)

FAMILY HISTORY:
- Father: Lung cancer, deceased age 67 (heavy smoker)
- Mother: Type 2 diabetes mellitus and chronic kidney disease, alive age 88
- Older brother: Myocardial infarction at age 62

SOCIAL HISTORY:
Smoking: Former smoker, 50 pack-years, quit 18 months ago upon diagnosis of lung cancer. Alcohol: Occasional social use, 1-2 beers per week. Occupation: Retired construction foreman.

VITAL SIGNS:
Temperature is 39.2°C (102.5°F), blood pressure is 85/50 mmHg, heart rate is 120 beats per minute, and respiratory rate is 22 breaths per minute

PHYSICAL EXAMINATION:
His skin is warm and flushed. Capillary refill time is less than 2 seconds.

LABORATORY DATA:
- White blood cell count: 18,000/mm³
- Hemoglobin: 12 g/dL
- Platelets: 150,000/mm³
- Serum creatinine: 2.5 mg/dL (baseline 1.9 mg/dL)
- Blood urea nitrogen: 40 mg/dL
- Lactate: 4.5 mmol/L
- Arterial blood gas: pH 7.30, PaCO2 32 mmHg, HCO3- 18 mmol/L

OTHER STUDIES:
A central venous catheter is placed, and hemodynamic monitoring reveals a cardiac index of 4.2 L/min/m² (normal 2.6-4.2 L/min/m²), systemic vascular resistance of 700 dyn·s/cm⁵ (normal 900-1400 dyn·s/cm⁵), and mixed venous oxygen saturation of 75% (normal 60-75%).

PLAN:
Broad-spectrum antibiotics are initiated.
