# synthetic_hospital patient 2841

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 72 at first encounter, sex F, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-06-05 (Fri) | ed | Emergency Medicine |
| 1 | 2020-01-15 | 2020-06-07 (Sun) | inpatient | Internal Medicine |
| 2 | 2020-02-15 | 2020-07-12 (Sun) | ed | Emergency Medicine |
| 3 | 2020-03-15 | 2020-07-28 (Tue) | ed | Emergency Medicine |
| 4 | 2020-05-15 | 2020-10-12 (Mon) | outpatient | Internal Medicine |
| 5 | 2024-01-15 | 2024-06-11 (Tue) | ed | Emergency Medicine |
| 6 | 2026-01-15 | 2026-06-08 (Mon) | ed | Emergency Medicine |
| 7 | 2026-02-15 | 2026-07-01 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Heparin (unfractionated) - heparin-induced thrombocytopenia",
  "Enoxaparin - heparin-induced thrombocytopenia",
  "Dalteparin - heparin-induced thrombocytopenia",
  "Contrast dye - anaphylaxis"
 ],
 "insurance": "Medicare",
 "occupation": "Retired elementary school teacher",
 "alcohol_use": "Occasional (1-2 glasses of wine on weekends)",
 "family_history": [
  "Father: Coronary artery disease, myocardial infarction at age 65",
  "Mother: Ischemic stroke at age 78",
  "Brother: Atrial fibrillation",
  "Sister: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "150 mg orally twice daily",
   "name": "Dabigatran"
  },
  {
   "dose": "10 mg orally daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg orally daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "25 mg orally daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "50 mg orally daily",
   "name": "Metoprolol succinate"
  }
 ],
 "surgical_history": [
  "Open repair of strangulated femoral hernia with small bowel resection (age 72)",
  "Left hip replacement (age 68)"
 ],
 "chronic_conditions": [
  "Chronic atrial fibrillation (persistent)",
  "Hypertension",
  "Hyperlipidemia",
  "Stage 2 chronic kidney disease",
  "Congestive heart failure",
  "History of heparin-induced thrombocytopenia",
  "History of ischemic stroke"
 ],
 "age_at_first_encounter": 72
}
```

Primary diagnoses: ["Heparin-induced thrombocytopenia", "Long-term anticoagulant therapy", "Acute ischemic stroke", "Digoxin toxicity", "Strangulated femoral hernia with small bowel obstruction"]
Comorbidities: ["Chronic atrial fibrillation", "Status post hip replacement surgery", "Hyperlipidemia", "Essential hypertension", "Acute ischemic stroke"]

## Encounter 0: source 2020-01-15, practice 2020-06-05, ed, Emergency Medicine

Chief complaint: Progressive lower abdominal pain and vomiting for 2 days with bloating and no bowel movement.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive lower abdominal pain and vomiting for 2 days with bloating and no bowel movement.

HISTORY OF PRESENT ILLNESS:
A 78-year-old woman presents to the emergency department with a 2-day history of progressive lower abdominal pain and vomiting. She reports that her abdomen feels bloated and she has not had a bowel movement since the symptoms started.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic atrial fibrillation (persistent)
- Hypertension
- Hyperlipidemia
- Stage 2 chronic kidney disease
- Congestive heart failure
- History of heparin-induced thrombocytopenia
- History of ischemic stroke

PAST SURGICAL HISTORY:
Hysterectomy performed 20 years ago

MEDICATIONS:
Home Medications:
- Dabigatran 150 mg orally twice daily
- Lisinopril 10 mg orally daily
- Atorvastatin 40 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Metoprolol succinate 50 mg orally daily

ALLERGIES:
Heparin (unfractionated) - heparin-induced thrombocytopenia, Enoxaparin - heparin-induced thrombocytopenia, Dalteparin - heparin-induced thrombocytopenia, Contrast dye - anaphylaxis

FAMILY HISTORY:
- Father: Coronary artery disease, myocardial infarction at age 65
- Mother: Ischemic stroke at age 78
- Brother: Atrial fibrillation
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 glasses of wine on weekends). Occupation: Retired elementary school teacher.

VITAL SIGNS:
On physical examination, her blood pressure is 150/90 mmHg, heart rate is 98 beats per minute, and temperature is 37.2°C (99.0°F).

PHYSICAL EXAMINATION:
Abdominal examination reveals a tender mass in the right groin region, with overlying skin erythema. Bowel sounds are high-pitched and tinkling. No other masses are palpable, and there is no rebound tenderness or guarding.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 12,000 /μL
- Hemoglobin: 13.2 g/dL
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 1.0 mg/dL

IMAGING:
A plain abdominal x-ray shows distended loops of small bowel with multiple air-fluid levels.

## Encounter 1: source 2020-01-15, practice 2020-06-07, inpatient, Internal Medicine

Chief complaint: Progressive pain and erythema at abdominal subcutaneous injection sites receiving post-operative heparin prophylaxis.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Michael Patel, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Progressive pain and erythema at abdominal subcutaneous injection sites receiving post-operative heparin prophylaxis.

HISTORY OF PRESENT ILLNESS:
The patient returns following emergent surgical repair of a strangulated femoral hernia performed earlier today, with a known history of heparin-induced thrombocytopenia. She reports a 2-day history of progressive pain and erythema at her abdominal subcutaneous injection sites.

PAST MEDICAL HISTORY:
Active Problem List:
- Strangulated femoral hernia with small bowel obstruction (diagnosed 2020-01-15)
- Chronic atrial fibrillation (persistent)
- Hypertension
- Hyperlipidemia
- Stage 2 chronic kidney disease
- Congestive heart failure
- History of heparin-induced thrombocytopenia
- History of ischemic stroke

PAST SURGICAL HISTORY:
Recent total knee replacement surgery

MEDICATIONS:
She has been on enoxaparin for thromboprophylaxis since her surgery 10 days ago.

Home Medications:
- Dabigatran 150 mg orally twice daily
- Lisinopril 10 mg orally daily
- Atorvastatin 40 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Metoprolol succinate 50 mg orally daily

ALLERGIES:
Heparin (unfractionated) - heparin-induced thrombocytopenia, Enoxaparin - heparin-induced thrombocytopenia, Dalteparin - heparin-induced thrombocytopenia, Contrast dye - anaphylaxis

FAMILY HISTORY:
- Father: Coronary artery disease, myocardial infarction at age 65
- Mother: Ischemic stroke at age 78
- Brother: Atrial fibrillation
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 glasses of wine on weekends). Occupation: Retired elementary school teacher.

VITAL SIGNS:
Her temperature is 37.8°C (100°F), blood pressure is 145/85 mmHg, pulse is 88/min, and respirations are 16/min.

PHYSICAL EXAMINATION:
Physical examination reveals a 5 cm area of induration and necrosis surrounded by erythema on her abdomen at the sites of enoxaparin injections.

LABORATORY DATA:
Laboratory studies show:

- Platelet count: 55,000/µL (decreased from 250,000/µL at the time of surgery)
- Hemoglobin: 13.2 g/dL
- White blood cell count: 7,800/µL
- Prothrombin time (PT): 12 seconds
- Activated partial thromboplastin time (aPTT): 28 seconds
- Fibrinogen: Normal
- D-dimer: Elevated

## Encounter 2: source 2020-02-15, practice 2020-07-12, ed, Emergency Medicine

Chief complaint: Acute onset shortness of breath and right-sided chest pain while at rest, without recent travel or immobilization.

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset shortness of breath and right-sided chest pain while at rest, without recent travel or immobilization.

HISTORY OF PRESENT ILLNESS:
This patient returns one month after heparin-induced thrombocytopenia with acute symptoms. She reports that her symptoms started suddenly while she was watching television. She denies any recent travel, prolonged immobilization, or previous thromboembolic events.

PAST MEDICAL HISTORY:
Active Problem List:
- Strangulated femoral hernia with small bowel obstruction (diagnosed 2020-01-15)
- Heparin-induced thrombocytopenia (diagnosed 2020-01-15)
- Chronic atrial fibrillation (persistent)
- Hypertension
- Hyperlipidemia
- Stage 2 chronic kidney disease
- Congestive heart failure
- History of ischemic stroke

PAST SURGICAL HISTORY:
Recent total knee replacement surgery

MEDICATIONS:
Her current medications include warfarin and metoprolol.

Home Medications:
- Dabigatran 150 mg orally twice daily
- Lisinopril 10 mg orally daily
- Atorvastatin 40 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Metoprolol succinate 50 mg orally daily

ALLERGIES:
Heparin (unfractionated) - heparin-induced thrombocytopenia, Enoxaparin - heparin-induced thrombocytopenia, Dalteparin - heparin-induced thrombocytopenia, Contrast dye - anaphylaxis

FAMILY HISTORY:
- Father: Coronary artery disease, myocardial infarction at age 65
- Mother: Ischemic stroke at age 78
- Brother: Atrial fibrillation
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 glasses of wine on weekends). Occupation: Retired elementary school teacher.

VITAL SIGNS:
With a respiratory rate of 28 breaths per minute, her blood pressure is 135/85 mmHg, heart rate is 110 beats per minute, and oxygen saturation is 89% on room air

PHYSICAL EXAMINATION:
On physical examination, she is tachypneic.
Her right lower extremity is swollen and tender to palpation.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 13.2 g/dL
- Platelet count: 55 x 10^9/L (decreased from 250 x 10^9/L measured 7 days ago)
- INR: 2.1
- aPTT: 68 seconds (elevated)
- D-dimer: Elevated

IMAGING:
A chest CT angiogram reveals a large pulmonary embolus in the right pulmonary artery.

## Encounter 3: source 2020-03-15, practice 2020-07-28, ed, Emergency Medicine

Chief complaint: Left-sided weakness and slurred speech developing over 2 days.

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Left-sided weakness and slurred speech developing over 2 days.

HISTORY OF PRESENT ILLNESS:
This patient with chronic atrial fibrillation and recent heparin-induced thrombocytopenia presents to the emergency department with a 2-day history of left-sided weakness and slurred speech.

PAST MEDICAL HISTORY:
Active Problem List:
- Strangulated femoral hernia with small bowel obstruction (diagnosed 2020-01-15)
- Heparin-induced thrombocytopenia (diagnosed 2020-01-15)
- Chronic atrial fibrillation (persistent)
- Hypertension
- Hyperlipidemia
- Stage 2 chronic kidney disease
- Congestive heart failure
- History of ischemic stroke

PAST SURGICAL HISTORY:
Recent total knee replacement surgery

MEDICATIONS:
She has been on enoxaparin for thromboprophylaxis since her surgery 10 days ago.

Home Medications:
- Dabigatran 150 mg orally twice daily
- Lisinopril 10 mg orally daily
- Atorvastatin 40 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Metoprolol succinate 50 mg orally daily

ALLERGIES:
Heparin (unfractionated) - heparin-induced thrombocytopenia, Enoxaparin - heparin-induced thrombocytopenia, Dalteparin - heparin-induced thrombocytopenia, Contrast dye - anaphylaxis

FAMILY HISTORY:
- Father: Coronary artery disease, myocardial infarction at age 65
- Mother: Ischemic stroke at age 78
- Brother: Atrial fibrillation
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 glasses of wine on weekends). Occupation: Retired elementary school teacher.

VITAL SIGNS:
Her temperature is 37.2°C (99.0°F), blood pressure is 155/90 mmHg, pulse is 78/min, and respirations are 16/min.

PHYSICAL EXAMINATION:
Physical examination reveals mild left facial droop and left upper extremity weakness.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 13.2 g/dL
- White blood cell count: 7.2 x 10^3/µL
- Platelet count: 55 x 10^3/µL (decreased from 250 x 10^3/µL measured preoperatively)
- Activated partial thromboplastin time (aPTT): 45 seconds (within the therapeutic range)
- Serum creatinine: 1.0 mg/dL

IMAGING:
A CT scan of the head shows no evidence of hemorrhage.

## Encounter 4: source 2020-05-15, practice 2020-10-12, outpatient, Internal Medicine

Chief complaint: Follow-up visit after hospitalization for ischemic stroke 2 months ago to review anticoagulation plan.

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Jennifer Walsh, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit after hospitalization for ischemic stroke 2 months ago to review anticoagulation plan.

HISTORY OF PRESENT ILLNESS:
The patient is a 72-year-old female with persistent atrial fibrillation and a history of heparin-induced thrombocytopenia complicated by ischemic stroke who returns for follow-up two months after her recent hospitalization to review her anticoagulation plan. During that admission, she was evaluated by neurology and cardiology, and after discussion about long-term anticoagulation, she was transitioned from warfarin to dabigatran. The change was made because she had difficulty keeping her INR in the therapeutic range despite consistent dosing and diet. Since starting the new medication, she has noticed no bruising, bleeding gums, melena, or hematuria. She takes her medication twice daily and has not missed any doses.

PAST MEDICAL HISTORY:
Active Problem List:
- Strangulated femoral hernia with small bowel obstruction (diagnosed 2020-01-15)
- Heparin-induced thrombocytopenia (diagnosed 2020-01-15)
- Chronic atrial fibrillation (persistent)
- Hypertension
- Hyperlipidemia
- Stage 2 chronic kidney disease
- Congestive heart failure
- History of ischemic stroke

She has a history of persistent atrial fibrillation, hypertension, hyperlipidemia, and stage 2 chronic kidney disease.

PAST SURGICAL HISTORY:
- Open repair of strangulated femoral hernia with small bowel resection (age 72)
- Left hip replacement (age 68)

MEDICATIONS:
Her medications include lisinopril, atorvastatin, hydrochlorothiazide, and the new anticoagulant.

Home Medications:
- Dabigatran 150 mg orally twice daily
- Metoprolol succinate 50 mg orally daily

ALLERGIES:
Heparin (unfractionated) - heparin-induced thrombocytopenia, Enoxaparin - heparin-induced thrombocytopenia, Dalteparin - heparin-induced thrombocytopenia, Contrast dye - anaphylaxis

FAMILY HISTORY:
- Father: Coronary artery disease, myocardial infarction at age 65
- Mother: Ischemic stroke at age 78
- Brother: Atrial fibrillation
- Sister: Hypertension

SOCIAL HISTORY:
She reports she enjoys gardening, baking for her grandchildren, and occasionally helping with her church choir. She reports she drinks an occasional glass of wine on weekends but does not use tobacco or illicit drugs.

REVIEW OF SYSTEMS:
She denies chest pain, palpitations, shortness of breath, or leg swelling.

VITAL SIGNS:
Blood pressure is 128/76 mm Hg, pulse is 74/min and irregularly irregular, respirations are 16/min, and temperature is 36.9°C (98.4°F).

PHYSICAL EXAMINATION:
Physical examination shows no pallor, petechiae, jaundice, or edema. Cardiac examination reveals an irregularly irregular rhythm. The lungs are clear to auscultation. Neurologic examination is normal.

## Encounter 5: source 2024-01-15, practice 2024-06-11, ed, Emergency Medicine

Chief complaint: Sudden onset of right-sided weakness and difficulty speaking.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Maria Garcia, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of right-sided weakness and difficulty speaking.

PAST MEDICAL HISTORY:
Active Problem List:
- Strangulated femoral hernia with small bowel obstruction (diagnosed 2020-01-15)
- Heparin-induced thrombocytopenia (diagnosed 2020-01-15)
- Long-term anticoagulant therapy (diagnosed 2020-05-15)
- Chronic atrial fibrillation (persistent)
- Hypertension
- Hyperlipidemia
- Stage 2 chronic kidney disease
- Congestive heart failure
- History of ischemic stroke

Her medical history includes hypertension and atrial fibrillation.

PAST SURGICAL HISTORY:
- Open repair of strangulated femoral hernia with small bowel resection (age 72)
- Left hip replacement (age 68)

MEDICATIONS:
Home Medications:
- Dabigatran 150 mg orally twice daily
- Lisinopril 10 mg orally daily
- Atorvastatin 40 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Metoprolol succinate 50 mg orally daily

ALLERGIES:
Heparin (unfractionated) - heparin-induced thrombocytopenia, Enoxaparin - heparin-induced thrombocytopenia, Dalteparin - heparin-induced thrombocytopenia, Contrast dye - anaphylaxis

FAMILY HISTORY:
- Father: Coronary artery disease, myocardial infarction at age 65
- Mother: Ischemic stroke at age 78
- Brother: Atrial fibrillation
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 glasses of wine on weekends). Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
Neurological examination shows right facial droop, decreased strength in the right arm and leg, and expressive aphasia.

IMAGING:
Head CT scan reveals an evolving infarct in the left middle cerebral artery territory.

PATHOLOGY:
Autopsy performed weeks later shows a cystic cavity in the affected brain region filled with debris and surrounded by microglial cells.

## Encounter 6: source 2026-01-15, practice 2026-06-08, ed, Emergency Medicine

Chief complaint: Nausea, vomiting, and confusion for 2 days without recent illness or medication changes.

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. James Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Nausea, vomiting, and confusion for 2 days without recent illness or medication changes.

HISTORY OF PRESENT ILLNESS:
The patient is a 78-year-old woman with persistent atrial fibrillation, congestive heart failure, and history of heparin-induced thrombocytopenia and ischemic stroke. She presents to the emergency department with a 2-day history of nausea, vomiting, and confusion and reports no recent illnesses, changes in medications, or dietary changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Strangulated femoral hernia with small bowel obstruction (diagnosed 2020-01-15)
- Heparin-induced thrombocytopenia (diagnosed 2020-01-15)
- Long-term anticoagulant therapy (diagnosed 2020-05-15)
- Acute ischemic stroke (diagnosed 2024-01-15)
- Chronic atrial fibrillation (persistent)
- Hypertension
- Hyperlipidemia
- Stage 2 chronic kidney disease
- Congestive heart failure

PAST SURGICAL HISTORY:
- Open repair of strangulated femoral hernia with small bowel resection (age 72)
- Left hip replacement (age 68)

MEDICATIONS:
She takes digoxin and hydrochlorothiazide.

Home Medications:
- Dabigatran 150 mg orally twice daily
- Lisinopril 10 mg orally daily
- Atorvastatin 40 mg orally daily
- Metoprolol succinate 50 mg orally daily

ALLERGIES:
Heparin (unfractionated) - heparin-induced thrombocytopenia, Enoxaparin - heparin-induced thrombocytopenia, Dalteparin - heparin-induced thrombocytopenia, Contrast dye - anaphylaxis

FAMILY HISTORY:
- Father: Coronary artery disease, myocardial infarction at age 65
- Mother: Ischemic stroke at age 78
- Brother: Atrial fibrillation
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 glasses of wine on weekends). Occupation: Retired elementary school teacher.

VITAL SIGNS:
Blood pressure is 130/80 mmHg, heart rate is 52 beats per minute

PHYSICAL EXAMINATION:
She is alert but appears disoriented to time

LABORATORY DATA:
Her serum electrolytes and renal function from two weeks ago were within normal limits, but today's laboratory results are as follows: - Serum creatinine: 2.1 mg/dL (increased from 1.0 mg/dL) - Blood urea nitrogen (BUN): 44 mg/dL - Serum potassium: 5.5 mEq/L - Serum digoxin level: 3.0 ng/mL (therapeutic range: 0.5-2.0 ng/mL)

OTHER STUDIES:
An ECG shows occasional premature ventricular contractions

## Encounter 7: source 2026-02-15, practice 2026-07-01, ed, Emergency Medicine

Chief complaint: Recurrent nausea, vomiting, decreased appetite, dizziness and blurred vision for 2 days.

ENCOUNTER NOTE
Date: 2026-02-15
Provider: Dr. James Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Recurrent nausea, vomiting, decreased appetite, dizziness and blurred vision for 2 days.

HISTORY OF PRESENT ILLNESS:
This patient with congestive heart failure and atrial fibrillation returns to the emergency department one month following a similar visit, presenting with nausea, vomiting, and decreased appetite for the past two days. She also reports episodes of dizziness and blurred vision.

PAST MEDICAL HISTORY:
Active Problem List:
- Strangulated femoral hernia with small bowel obstruction (diagnosed 2020-01-15)
- Heparin-induced thrombocytopenia (diagnosed 2020-01-15)
- Long-term anticoagulant therapy (diagnosed 2020-05-15)
- Acute ischemic stroke (diagnosed 2024-01-15)
- Digoxin toxicity (diagnosed 2026-01-15)
- Chronic atrial fibrillation (persistent)
- Hypertension
- Hyperlipidemia
- Stage 2 chronic kidney disease
- Congestive heart failure

PAST SURGICAL HISTORY:
- Open repair of strangulated femoral hernia with small bowel resection (age 72)
- Left hip replacement (age 68)

MEDICATIONS:
Her medications include digoxin, warfarin, furosemide, and lisinopril.

Home Medications:
- Dabigatran 150 mg orally twice daily
- Atorvastatin 40 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Metoprolol succinate 50 mg orally daily

ALLERGIES:
Heparin (unfractionated) - heparin-induced thrombocytopenia, Enoxaparin - heparin-induced thrombocytopenia, Dalteparin - heparin-induced thrombocytopenia, Contrast dye - anaphylaxis

FAMILY HISTORY:
- Father: Coronary artery disease, myocardial infarction at age 65
- Mother: Ischemic stroke at age 78
- Brother: Atrial fibrillation
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 glasses of wine on weekends). Occupation: Retired elementary school teacher.

VITAL SIGNS:
On examination, her blood pressure is 130/80 mmHg, heart rate is 50 beats per minute, and respiratory rate is 18 breaths per minute.

LABORATORY DATA:
Serum electrolytes are as follows:

- Sodium: 140 mEq/L
- Potassium: 3.5 mEq/L
- Chloride: 100 mEq/L
- Bicarbonate: 24 mEq/L
- Calcium: 9.2 mg/dL
- Magnesium: 1.8 mg/dL

Serum digoxin level is pending.

OTHER STUDIES:
An ECG shows atrial tachycardia with 2:1 AV block.
