# synthetic_hospital patient 2304

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 72 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-05-03 (Tue) | ed | Emergency Medicine |
| 1 | 2020-10-15 | 2023-02-05 (Sun) | ed | Emergency Medicine |
| 2 | 2022-01-15 | 2024-05-05 (Sun) | inpatient | Internal Medicine |
| 3 | 2023-01-15 | 2025-04-30 (Wed) | icu | Pulmonary & Critical Care |
| 4 | 2024-01-15 | 2026-05-10 (Sun) | icu | Pulmonary & Critical Care |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired construction foreman",
 "alcohol_use": "None",
 "family_history": [
  "Father: Severe emphysema, deceased at 74",
  "Mother: Alzheimer disease, deceased at 81",
  "Sibling: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 60 pack-years, quit 4 years ago",
 "home_medications": [
  {
   "dose": "18 mcg inhaled daily",
   "name": "Tiotropium"
  },
  {
   "dose": "250/50 mcg inhaled twice daily",
   "name": "Fluticasone/salmeterol"
  },
  {
   "dose": "90 mcg inhaled every 4 hours as needed for wheezing or shortness of breath",
   "name": "Albuterol"
  },
  {
   "dose": "40 mg oral daily",
   "name": "Furosemide"
  },
  {
   "dose": "5 mg oral daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25 mg oral daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "5 mg oral twice daily",
   "name": "Apixaban"
  },
  {
   "dose": "40 mg oral daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "10 mg oral at bedtime",
   "name": "Donepezil"
  },
  {
   "dose": "2 L/min via nasal cannula at nighttime",
   "name": "Oxygen"
  }
 ],
 "surgical_history": [
  "Cataract extraction, right eye (2021)",
  "Open reduction and internal fixation of right femoral neck fracture (2019)"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (COPD), severe (GOLD Stage IV)",
  "Heart failure with reduced ejection fraction (HFrEF)",
  "Cor pulmonale (chronic right-sided heart failure secondary to pulmonary hypertension)",
  "Advanced dementia (vascular type)",
  "Cerebrovascular accident with residual left hemiparesis",
  "Home oxygen dependence (nocturnal)"
 ],
 "age_at_first_encounter": 72
}
```

Primary diagnoses: ["Acute-on-chronic respiratory failure secondary to COPD exacerbation with superimposed bacterial pneumonia", "Opportunistic pulmonary infection requiring diagnostic sampling", "Post-obstructive atelectasis", "Post-embolization syndrome", "Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease)"]
Comorbidities: ["Chronic obstructive pulmonary disease"]

## Encounter 0: source 2020-01-15, practice 2022-05-03, ed, Emergency Medicine

Chief complaint: Worsening leg swelling, increasing shortness of breath, and difficulty sleeping while lying flat.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening leg swelling, increasing shortness of breath, and difficulty sleeping while lying flat.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (COPD), severe (GOLD Stage IV)
- Heart failure with reduced ejection fraction (HFrEF)
- Cor pulmonale (chronic right-sided heart failure secondary to pulmonary hypertension)
- Advanced dementia (vascular type)
- Cerebrovascular accident with residual left hemiparesis
- Home oxygen dependence (nocturnal)

He has a long history of chronic obstructive pulmonary disease and has required multiple hospitalizations for respiratory exacerbations. He reports using home oxygen at night.

PAST SURGICAL HISTORY:
- Cataract extraction, right eye (2021)
- Open reduction and internal fixation of right femoral neck fracture (2019)

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled daily
- Fluticasone/salmeterol 250/50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4 hours as needed for wheezing or shortness of breath
- Furosemide 40 mg oral daily
- Lisinopril 5 mg oral daily
- Metoprolol succinate 25 mg oral daily
- Apixaban 5 mg oral twice daily
- Atorvastatin 40 mg oral daily
- Donepezil 10 mg oral at bedtime
- Oxygen 2 L/min via nasal cannula at nighttime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Severe emphysema, deceased at 74
- Mother: Alzheimer disease, deceased at 81
- Sibling: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 60 pack-years, quit 4 years ago. Alcohol: None. Occupation: Retired construction foreman.

VITAL SIGNS:
His temperature is 36.6°C (97.9°F), blood pressure is 112/68 mm Hg, pulse is 92/min, and respirations are 20/min.

PHYSICAL EXAMINATION:
Jugular venous distension is noted, and there is bilateral lower extremity pitting edema. Pulmonary auscultation reveals wheezing and prolonged expiration.

OTHER STUDIES:
ECG findings are shown below.

## Encounter 1: source 2020-10-15, practice 2023-02-05, ed, Emergency Medicine

Chief complaint: Increased shortness of breath and productive cough with decreased exercise tolerance over the last 48 hours.

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Michael Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increased shortness of breath and productive cough with decreased exercise tolerance over the last 48 hours.

HISTORY OF PRESENT ILLNESS:
The patient with severe chronic obstructive pulmonary disease and cor pulmonale, evaluated in January for leg swelling and orthopnea, returns with a 2-day history of increased shortness of breath and productive cough. He reports a decrease in his exercise tolerance over the last 48 hours.

PAST MEDICAL HISTORY:
Active Problem List:
- Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease) (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (COPD), severe (GOLD Stage IV)
- Heart failure with reduced ejection fraction (HFrEF)
- Cor pulmonale (chronic right-sided heart failure secondary to pulmonary hypertension)
- Advanced dementia (vascular type)
- Cerebrovascular accident with residual left hemiparesis
- Home oxygen dependence (nocturnal)

PAST SURGICAL HISTORY:
- Cataract extraction, right eye (2021)
- Open reduction and internal fixation of right femoral neck fracture (2019)

MEDICATIONS:
He takes tiotropium, salbutamol as needed, and a fluticasone/salmeterol inhaler.

Home Medications:
- Albuterol 90 mcg inhaled every 4 hours as needed for wheezing or shortness of breath
- Furosemide 40 mg oral daily
- Lisinopril 5 mg oral daily
- Metoprolol succinate 25 mg oral daily
- Apixaban 5 mg oral twice daily
- Atorvastatin 40 mg oral daily
- Donepezil 10 mg oral at bedtime
- Oxygen 2 L/min via nasal cannula at nighttime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Severe emphysema, deceased at 74
- Mother: Alzheimer disease, deceased at 81
- Sibling: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 60 pack-years, quit 4 years ago. Alcohol: None. Occupation: Retired construction foreman.

VITAL SIGNS:
Temperature is 37.8°C, blood pressure is 145/85 mmHg, heart rate is 102 beats per minute, respiratory rate is 26 breaths per minute, and oxygen saturation is 89% on room air

PHYSICAL EXAMINATION:
Chest examination reveals decreased breath sounds and dullness to percussion at the right base. The remainder of the examination is unremarkable.

IMAGING:
Chest X-ray shows an opacity in the right lower lobe with a slight shift of the mediastinum towards the right.

## Encounter 2: source 2022-01-15, practice 2024-05-05, inpatient, Internal Medicine

Chief complaint: Pneumonia with hypoxemia and tachypnea despite broad-spectrum antibiotics.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Emily Rodriguez, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Pneumonia with hypoxemia and tachypnea despite broad-spectrum antibiotics.

HISTORY OF PRESENT ILLNESS:
The patient is a 74-year-old male with severe COPD (GOLD Stage IV) and cor pulmonale who has required prior emergency care for respiratory decompensation and right heart failure exacerbations. He is admitted to the hospital with pneumonia. Despite broad-spectrum antibiotics and supportive care, his condition deteriorates over the next week. He is now hypoxic with an oxygen saturation of 88% on 4 L/min of oxygen via nasal cannula, tachypneic with a respiratory rate of 28 breaths/min, and increasingly lethargic.

PAST MEDICAL HISTORY:
Active Problem List:
- Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease) (diagnosed 2020-01-15)
- Post-obstructive atelectasis (diagnosed 2020-10-15)
- Chronic obstructive pulmonary disease (COPD), severe (GOLD Stage IV)
- Heart failure with reduced ejection fraction (HFrEF)
- Cor pulmonale (chronic right-sided heart failure secondary to pulmonary hypertension)
- Advanced dementia (vascular type)
- Cerebrovascular accident with residual left hemiparesis
- Home oxygen dependence (nocturnal)

PAST SURGICAL HISTORY:
- Cataract extraction, right eye (2021)
- Open reduction and internal fixation of right femoral neck fracture (2019)

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled daily
- Fluticasone/salmeterol 250/50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4 hours as needed for wheezing or shortness of breath
- Furosemide 40 mg oral daily
- Lisinopril 5 mg oral daily
- Metoprolol succinate 25 mg oral daily
- Apixaban 5 mg oral twice daily
- Atorvastatin 40 mg oral daily
- Donepezil 10 mg oral at bedtime
- Oxygen 2 L/min via nasal cannula at nighttime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Severe emphysema, deceased at 74
- Mother: Alzheimer disease, deceased at 81
- Sibling: Hypertension

SOCIAL HISTORY:
His family is distressed and insists on "doing everything," but they are also concerned about causing him suffering. The patient has a documented advance directive stating he would not want invasive procedures or life-sustaining treatment if there is no reasonable expectation of recovery.

VITAL SIGNS:
Oxygen saturation of 88% on 4 L/min of oxygen via nasal cannula, tachypneic with a respiratory rate of 28 breaths/min, blood pressure is 90/60 mmHg, heart rate is 110 beats/min

PHYSICAL EXAMINATION:
Bilateral crackles on lung auscultation

PLAN:
The care team suggests a No Escalation of Treatment (NEOT) order to honor the patient's wishes and to focus on comfort measures.

## Encounter 3: source 2023-01-15, practice 2025-04-30, icu, Pulmonary & Critical Care

Chief complaint: Acute respiratory distress and worsening hypoxemia requiring transfer from medical floor to intensive care unit.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James Thompson, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Acute respiratory distress and worsening hypoxemia requiring transfer from medical floor to intensive care unit.

HISTORY OF PRESENT ILLNESS:
This 75-year-old male with severe COPD, cor pulmonale, and history of atelectasis and opportunistic infection presents with acute respiratory distress requiring ICU transfer. He was initially admitted for a COPD exacerbation and treated with nebulizers, systemic corticosteroids, and antibiotics.

PAST MEDICAL HISTORY:
Active Problem List:
- Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease) (diagnosed 2020-01-15)
- Post-obstructive atelectasis (diagnosed 2020-10-15)
- Opportunistic pulmonary infection requiring diagnostic sampling (diagnosed 2022-01-15)
- Chronic obstructive pulmonary disease (COPD), severe (GOLD Stage IV)
- Heart failure with reduced ejection fraction (HFrEF)
- Cor pulmonale (chronic right-sided heart failure secondary to pulmonary hypertension)
- Advanced dementia (vascular type)
- Cerebrovascular accident with residual left hemiparesis
- Home oxygen dependence (nocturnal)

PAST SURGICAL HISTORY:
- Cataract extraction, right eye (2021)
- Open reduction and internal fixation of right femoral neck fracture (2019)

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled daily
- Fluticasone/salmeterol 250/50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4 hours as needed for wheezing or shortness of breath
- Furosemide 40 mg oral daily
- Lisinopril 5 mg oral daily
- Metoprolol succinate 25 mg oral daily
- Apixaban 5 mg oral twice daily
- Atorvastatin 40 mg oral daily
- Donepezil 10 mg oral at bedtime
- Oxygen 2 L/min via nasal cannula at nighttime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Severe emphysema, deceased at 74
- Mother: Alzheimer disease, deceased at 81
- Sibling: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 60 pack-years, quit 4 years ago. Alcohol: None. Occupation: Retired construction foreman.

VITAL SIGNS:
His blood pressure is 150/90 mmHg, heart rate is 110 beats per minute, and oxygen saturation is 88% on 4 liters of oxygen via nasal cannula.

PHYSICAL EXAMINATION:
On examination, he is tachypneic with a respiratory rate of 28 breaths per minute, uses accessory muscles to breathe, and has bilateral wheezes on auscultation.

LABORATORY DATA:
Arterial blood gas shows pH 7.32, PaCO2 60 mmHg, PaO2 55 mmHg, and HCO3- 28 mEq/L.

## Encounter 4: source 2024-01-15, practice 2026-05-10, icu, Pulmonary & Critical Care

Chief complaint: Acute respiratory failure requiring endotracheal intubation.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Aisha Patel, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Acute respiratory failure requiring endotracheal intubation.

HISTORY OF PRESENT ILLNESS:
The patient with severe COPD, heart failure, and dementia returns with acute respiratory failure requiring admission to the intensive care unit and undergoes endotracheal intubation due to hypoxemia and altered mental status.

PAST MEDICAL HISTORY:
Active Problem List:
- Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease) (diagnosed 2020-01-15)
- Post-obstructive atelectasis (diagnosed 2020-10-15)
- Opportunistic pulmonary infection requiring diagnostic sampling (diagnosed 2022-01-15)
- Acute-on-chronic respiratory failure secondary to COPD exacerbation with superimposed bacterial pneumonia (diagnosed 2023-01-15)
- Chronic obstructive pulmonary disease (COPD), severe (GOLD Stage IV)
- Heart failure with reduced ejection fraction (HFrEF)
- Cor pulmonale (chronic right-sided heart failure secondary to pulmonary hypertension)
- Advanced dementia (vascular type)
- Cerebrovascular accident with residual left hemiparesis
- Home oxygen dependence (nocturnal)

PAST SURGICAL HISTORY:
- Cataract extraction, right eye (2021)
- Open reduction and internal fixation of right femoral neck fracture (2019)

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled daily
- Fluticasone/salmeterol 250/50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4 hours as needed for wheezing or shortness of breath
- Furosemide 40 mg oral daily
- Lisinopril 5 mg oral daily
- Metoprolol succinate 25 mg oral daily
- Apixaban 5 mg oral twice daily
- Atorvastatin 40 mg oral daily
- Donepezil 10 mg oral at bedtime
- Oxygen 2 L/min via nasal cannula at nighttime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Severe emphysema, deceased at 74
- Mother: Alzheimer disease, deceased at 81
- Sibling: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 60 pack-years, quit 4 years ago. Alcohol: None. Occupation: Retired construction foreman.

VITAL SIGNS:
The patient's oxygen saturation improves to 92% on mechanical ventilation with a FiO2 of 60%.

PHYSICAL EXAMINATION:
After intubation, the nurse reports that the patient has asymmetric chest expansion, with the right side appearing more prominent during inspiration. On auscultation, breath sounds are markedly decreased on the left side, and there is no adventitious sound noted.

OTHER STUDIES:
The capnography shows a normal waveform.
