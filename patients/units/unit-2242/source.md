# synthetic_hospital patient 2242

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-02-02 (Tue) | follow_up | Internal Medicine |
| 1 | 2020-04-15 | 2021-05-02 (Sun) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2022-02-08 (Tue) | ed | Emergency Medicine |
| 3 | 2022-01-15 | 2023-02-08 (Wed) | ed | Emergency Medicine |
| 4 | 2022-07-15 | 2023-08-05 (Sat) | ed | Emergency Medicine |
| 5 | 2024-01-15 | 2025-01-28 (Tue) | outpatient | Cardiology |
| 6 | 2024-07-15 | 2025-08-06 (Wed) | ed | Emergency Medicine |
| 7 | 2025-07-15 | 2026-07-30 (Thu) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired construction foreman",
 "alcohol_use": "Social, 1-2 drinks per week",
 "family_history": [
  "Father: Myocardial infarction at age 62",
  "Mother: Hypertension and chronic kidney disease",
  "Brother: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 30 pack-years, quit 5 years ago",
 "home_medications": [
  {
   "dose": "40 mg PO daily",
   "name": "Furosemide"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "12.5 mg PO twice daily",
   "name": "Carvedilol"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "5 mg PO daily (titrate to INR 2.0-3.0)",
   "name": "Warfarin"
  },
  {
   "dose": "18 mcg inhaled daily",
   "name": "Tiotropium"
  },
  {
   "dose": "250/50 mcg inhaled twice daily",
   "name": "Fluticasone/salmeterol"
  }
 ],
 "surgical_history": [
  "Percutaneous coronary intervention with drug-eluting stent placement (anterior wall)",
  "Implantable cardioverter-defibrillator (ICD) placement",
  "Diagnostic cardiac catheterization"
 ],
 "chronic_conditions": [
  "Ischemic cardiomyopathy with reduced ejection fraction (HFrEF, EF 25-30%)",
  "Hypertension",
  "Hyperlipidemia",
  "Chronic atrial fibrillation",
  "Chronic obstructive pulmonary disease (COPD)",
  "Chronic kidney disease stage III",
  "History of anterior wall myocardial infarction",
  "History of lateral wall myocardial infarction"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Heart failure with reduced ejection fraction", "Left Lateral Wall Myocardial Infarction", "Aortic dissection"]
Comorbidities: ["Acute pulmonary edema", "Hyponatremia", "Acute kidney injury", "Ischemic cardiomyopathy", "Heart failure with reduced ejection fraction", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2021-02-02, follow_up, Internal Medicine

Chief complaint: Follow-up appointment after recent hospitalization for COPD exacerbation with increasing shortness of breath and weight gain

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: follow_up

CHIEF COMPLAINT:
Follow-up appointment after recent hospitalization for COPD exacerbation with increasing shortness of breath and weight gain

HISTORY OF PRESENT ILLNESS:
He was discharged from the hospital 5 days ago following an exacerbation of COPD. He reports increasing shortness of breath over the past two days, difficulty sleeping at night due to coughing, and a weight gain of 4 pounds since discharge.

PAST MEDICAL HISTORY:
Active Problem List:
- Ischemic cardiomyopathy with reduced ejection fraction (HFrEF, EF 25-30%)
- Hypertension
- Hyperlipidemia
- Chronic atrial fibrillation
- Chronic obstructive pulmonary disease (COPD)
- Chronic kidney disease stage III
- History of anterior wall myocardial infarction
- History of lateral wall myocardial infarction

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (anterior wall)
- Implantable cardioverter-defibrillator (ICD) placement
- Diagnostic cardiac catheterization

MEDICATIONS:
His medications include tiotropium, fluticasone/salmeterol, furosemide, lisinopril, and carvedilol.

Home Medications:
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Warfarin 5 mg PO daily (titrate to INR 2.0-3.0)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension and chronic kidney disease
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 5 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired construction foreman.

VITAL SIGNS:
On examination, his blood pressure is 145/90 mmHg, heart rate is 88 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 92% on room air

PHYSICAL EXAMINATION:
Lung auscultation reveals bilateral wheezes and rales at the lung bases. Lower extremity examination shows 2+ pitting edema.

LABORATORY DATA:
Laboratory values are as follows:

- Serum sodium: 138 mEq/L
- Serum potassium: 4.5 mEq/L
- Serum creatinine: 1.2 mg/dL
- B-type natriuretic peptide (BNP): 900 pg/mL (elevated)
- Complete blood count: within normal limits

## Encounter 1: source 2020-04-15, practice 2021-05-02, ed, Emergency Medicine

Chief complaint: Progressive shortness of breath, orthopnea, and leg swelling after running out of furosemide one week ago

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive shortness of breath, orthopnea, and leg swelling after running out of furosemide one week ago

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old male with known ischemic cardiomyopathy with reduced ejection fraction and prior evaluation in January 2020 for dyspnea and weight gain, now presenting with progressive symptoms. Over the past 10 days, he has become increasingly dyspneic while climbing stairs and now wakes up at night gasping for air. He also reports feeling bloated, with reduced appetite and weight gain despite minimal food intake. He ran out of furosemide one week ago and has not refilled it.

PAST MEDICAL HISTORY:
Active Problem List:
- Heart failure with reduced ejection fraction (diagnosed 2020-01-15)
- Ischemic cardiomyopathy with reduced ejection fraction (HFrEF, EF 25-30%)
- Hypertension
- Hyperlipidemia
- Chronic atrial fibrillation
- Chronic obstructive pulmonary disease (COPD)
- Chronic kidney disease stage III
- History of anterior wall myocardial infarction
- History of lateral wall myocardial infarction

His past medical history includes chronic systolic heart failure with reduced ejection fraction (EF 30%), hypertension, and a remote history of anterior wall myocardial infarction.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (anterior wall)
- Implantable cardioverter-defibrillator (ICD) placement
- Diagnostic cardiac catheterization

MEDICATIONS:
His current medications include carvedilol, lisinopril, atorvastatin, aspirin, and furosemide.

Home Medications:
- Warfarin 5 mg PO daily (titrate to INR 2.0-3.0)
- Tiotropium 18 mcg inhaled daily
- Fluticasone/salmeterol 250/50 mcg inhaled twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension and chronic kidney disease
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 5 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired construction foreman.

VITAL SIGNS:
His temperature is 36.8°C (98.2°F), blood pressure is 106/68 mm Hg, pulse is 96/min, and respiratory rate is 22/min.

PHYSICAL EXAMINATION:
On examination, the patient appears fatigued and mildly tachypneic. Jugular venous distension is noted 8 cm above the sternal angle at 45°. Cardiac auscultation reveals a displaced apical impulse and an S3 gallop. Lung examination shows bilateral basilar crackles. There is 3+ pitting edema in both lower extremities to the mid-shin and ascites is suspected based on shifting dullness.

LABORATORY DATA:
His serum sodium remains stable, but his potassium level drops to 3.2 mEq/L.

IMAGING:
Bedside ultrasound shows a dilated inferior vena cava with minimal collapse during inspiration. A chest X-ray reveals pulmonary vascular congestion and small bilateral pleural effusions.

PLAN:
The patient is admitted and receives intravenous furosemide. Within 6 hours, his urine output increases substantially, his dyspnea improves, and his oxygen requirements decrease. Repeat physical exam shows improved JVP and clearer lungs. The medical team discusses the pharmacologic mechanism of loop diuretics and their effect on renal electrolyte handling.

## Encounter 2: source 2021-01-15, practice 2022-02-08, ed, Emergency Medicine

Chief complaint: Worsening shortness of breath, orthopnea, and bilateral lower extremity edema over the past week

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Jennifer Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening shortness of breath, orthopnea, and bilateral lower extremity edema over the past week

HISTORY OF PRESENT ILLNESS:
Patient with ischemic cardiomyopathy (EF 25-30%) and prior ED visits presents to the emergency department with worsening shortness of breath, orthopnea, and bilateral lower extremity edema over the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Heart failure with reduced ejection fraction (diagnosed 2020-01-15)
- Ischemic cardiomyopathy with reduced ejection fraction (HFrEF, EF 25-30%)
- Hypertension
- Hyperlipidemia
- Chronic atrial fibrillation
- Chronic obstructive pulmonary disease (COPD)
- Chronic kidney disease stage III
- History of anterior wall myocardial infarction
- History of lateral wall myocardial infarction

He has a known ejection fraction of 25%.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (anterior wall)
- Implantable cardioverter-defibrillator (ICD) placement
- Diagnostic cardiac catheterization

MEDICATIONS:
His medications include lisinopril, carvedilol, and furosemide.

Home Medications:
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Warfarin 5 mg PO daily (titrate to INR 2.0-3.0)
- Tiotropium 18 mcg inhaled daily
- Fluticasone/salmeterol 250/50 mcg inhaled twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension and chronic kidney disease
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 5 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired construction foreman.

VITAL SIGNS:
On examination, his blood pressure is 90/60 mmHg, heart rate is 110 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
Lung auscultation reveals bilateral rales halfway up both lung fields. Cardiac examination shows an S3 gallop. Extremities are notable for 2+ pitting edema up to the mid-shins.

LABORATORY DATA:
Laboratory tests reveal:

- Serum sodium: 134 mEq/L
- Serum potassium: 4.8 mEq/L
- Serum creatinine: 1.9 mg/dL (elevated from baseline of 1.2 mg/dL)
- B-type natriuretic peptide (BNP): 900 pg/mL

IMAGING:
Chest X-ray shows cardiomegaly and pulmonary edema.

ASSESSMENT AND PLAN:
The patient is diagnosed with acute decompensated heart failure.

## Encounter 3: source 2022-01-15, practice 2023-02-08, ed, Emergency Medicine

Chief complaint: Severe chest pressure in the center of the chest radiating to the left arm and jaw with shortness of breath and diaphoresis

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe chest pressure in the center of the chest radiating to the left arm and jaw with shortness of breath and diaphoresis

HISTORY OF PRESENT ILLNESS:
This patient with HFrEF and prior MI presents with chest pain after HF exacerbations. The pain is described as a heavy pressure in the center of his chest, radiating to his left arm and jaw. He also reports shortness of breath and diaphoresis.

PAST MEDICAL HISTORY:
Active Problem List:
- Heart failure with reduced ejection fraction (diagnosed 2020-01-15)
- Ischemic cardiomyopathy with reduced ejection fraction (HFrEF, EF 25-30%)
- Hypertension
- Hyperlipidemia
- Chronic atrial fibrillation
- Chronic obstructive pulmonary disease (COPD)
- Chronic kidney disease stage III
- History of anterior wall myocardial infarction
- History of lateral wall myocardial infarction

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (anterior wall)
- Implantable cardioverter-defibrillator (ICD) placement
- Diagnostic cardiac catheterization

MEDICATIONS:
His medications include atorvastatin and lisinopril.

Home Medications:
- Furosemide 40 mg PO daily
- Carvedilol 12.5 mg PO twice daily
- Aspirin 81 mg PO daily
- Warfarin 5 mg PO daily (titrate to INR 2.0-3.0)
- Tiotropium 18 mcg inhaled daily
- Fluticasone/salmeterol 250/50 mcg inhaled twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension and chronic kidney disease
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 5 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired construction foreman.

VITAL SIGNS:
On examination, his blood pressure is 85/50 mmHg, heart rate is 110 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 90% on room air.

PHYSICAL EXAMINATION:
Physical examination reveals jugular venous distension, bilateral crackles halfway up both lung fields, and a third heart sound (S3).

LABORATORY DATA:
Laboratory results are as follows:

- Troponin I: 12 ng/mL (normal <0.04 ng/mL)
- Creatinine: 1.2 mg/dL (normal 0.6-1.2 mg/dL)
- B-type natriuretic peptide (BNP): 900 pg/mL (normal <100 pg/mL)

IMAGING:
An echocardiogram reveals severe hypokinesis of the left ventricular inferior wall.

OTHER STUDIES:
An electrocardiogram shows ST-segment elevation in leads II, III, and aVF.

PLAN:
The patient is started on oxygen, aspirin, and nitroglycerin. Despite these measures, his blood pressure remains low.

## Encounter 4: source 2022-07-15, practice 2023-08-05, ed, Emergency Medicine

Chief complaint: Worsening dyspnea, orthopnea, and bilateral lower extremity edema with 4 kg weight gain over the past week

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Angela Foster, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening dyspnea, orthopnea, and bilateral lower extremity edema with 4 kg weight gain over the past week

HISTORY OF PRESENT ILLNESS:
The patient is a 70-year-old male with known ischemic cardiomyopathy and recent myocardial infarction who presents to the emergency department with a 2-day history of worsening dyspnea, orthopnea, and bilateral lower extremity edema. He reports a weight gain of 4 kg over the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Heart failure with reduced ejection fraction (diagnosed 2020-01-15)
- Left Lateral Wall Myocardial Infarction (diagnosed 2022-01-15)
- Ischemic cardiomyopathy with reduced ejection fraction (HFrEF, EF 25-30%)
- Hypertension
- Hyperlipidemia
- Chronic atrial fibrillation
- Chronic obstructive pulmonary disease (COPD)
- Chronic kidney disease stage III
- History of anterior wall myocardial infarction
- History of lateral wall myocardial infarction

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (anterior wall)
- Implantable cardioverter-defibrillator (ICD) placement
- Diagnostic cardiac catheterization

MEDICATIONS:
He takes furosemide, lisinopril, and carvedilol.

Home Medications:
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Warfarin 5 mg PO daily (titrate to INR 2.0-3.0)
- Tiotropium 18 mcg inhaled daily
- Fluticasone/salmeterol 250/50 mcg inhaled twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension and chronic kidney disease
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 5 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired construction foreman.

VITAL SIGNS:
Blood pressure is 150/90 mmHg, heart rate is 96 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 88% on room air

PHYSICAL EXAMINATION:
Jugular venous pressure is elevated. Lung auscultation reveals bilateral rales halfway up both lung fields. Cardiac examination shows an S3 gallop. Extremities are notable for 2+ pitting edema up to the knees.

LABORATORY DATA:
Serum creatinine: 2.1 mg/dL (baseline 1.2 mg/dL), Blood urea nitrogen (BUN): 44 mg/dL, Serum sodium: 134 mEq/L, Serum potassium: 4.8 mEq/L, B-type natriuretic peptide (BNP): 900 pg/mL, Urinalysis: Specific gravity 1.010, no protein, no cells

IMAGING:
Chest X-ray shows pulmonary vascular congestion and cardiomegaly. An echocardiogram reveals a left ventricular ejection fraction of 30% with no valvular abnormalities.

## Encounter 5: source 2024-01-15, practice 2025-01-28, outpatient, Cardiology

Chief complaint: Gradually worsening shortness of breath over the past two months

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. James Patterson, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Gradually worsening shortness of breath over the past two months

HISTORY OF PRESENT ILLNESS:
A 72-year-old man with ischemic cardiomyopathy and recurrent heart failure exacerbations returns to the cardiology clinic for follow-up of worsening shortness of breath over the past 2 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Heart failure with reduced ejection fraction (diagnosed 2020-01-15)
- Left Lateral Wall Myocardial Infarction (diagnosed 2022-01-15)
- Ischemic cardiomyopathy with reduced ejection fraction (HFrEF, EF 25-30%)
- Hypertension
- Hyperlipidemia
- Chronic atrial fibrillation
- Chronic obstructive pulmonary disease (COPD)
- Chronic kidney disease stage III
- History of anterior wall myocardial infarction
- History of lateral wall myocardial infarction

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (anterior wall)
- Implantable cardioverter-defibrillator (ICD) placement
- Diagnostic cardiac catheterization

MEDICATIONS:
He takes lisinopril, furosemide, metoprolol, and warfarin.

Home Medications:
- Carvedilol 12.5 mg PO twice daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Tiotropium 18 mcg inhaled daily
- Fluticasone/salmeterol 250/50 mcg inhaled twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension and chronic kidney disease
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 5 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired construction foreman.

VITAL SIGNS:
Blood pressure is 108/66 mm Hg, pulse is irregularly irregular at 78/min, and oxygen saturation is 94% on room air

PHYSICAL EXAMINATION:
Jugular venous pressure is elevated, lung auscultation reveals bibasilar crackles, and there is bilateral pitting edema of the lower extremities

OTHER STUDIES:
ECG shows atrial fibrillation with a controlled ventricular response

PLAN:
The physician adds a medication that improves his symptoms by increasing myocardial contractility and providing ventricular rate control.

## Encounter 6: source 2024-07-15, practice 2025-08-06, ed, Emergency Medicine

Chief complaint: Increasing shortness of breath, orthopnea, and bilateral lower extremity edema with 4 kg weight gain

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increasing shortness of breath, orthopnea, and bilateral lower extremity edema with 4 kg weight gain

HISTORY OF PRESENT ILLNESS:
The patient with ischemic cardiomyopathy and recurrent heart failure exacerbations presents with increasing shortness of breath, orthopnea, and bilateral lower extremity edema over the past week. He reports weight gain of 4 kg during this period.

PAST MEDICAL HISTORY:
Active Problem List:
- Heart failure with reduced ejection fraction (diagnosed 2020-01-15)
- Left Lateral Wall Myocardial Infarction (diagnosed 2022-01-15)
- Ischemic cardiomyopathy with reduced ejection fraction (HFrEF, EF 25-30%)
- Hypertension
- Hyperlipidemia
- Chronic atrial fibrillation
- Chronic obstructive pulmonary disease (COPD)
- Chronic kidney disease stage III
- History of anterior wall myocardial infarction
- History of lateral wall myocardial infarction

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (anterior wall)
- Implantable cardioverter-defibrillator (ICD) placement
- Diagnostic cardiac catheterization

MEDICATIONS:
He takes lisinopril, carvedilol, and furosemide.

Home Medications:
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Warfarin 5 mg PO daily (titrate to INR 2.0-3.0)
- Tiotropium 18 mcg inhaled daily
- Fluticasone/salmeterol 250/50 mcg inhaled twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension and chronic kidney disease
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 5 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired construction foreman.

VITAL SIGNS:
Blood pressure is 130/85 mmHg, heart rate is 88 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 92% on room air

PHYSICAL EXAMINATION:
Jugular venous pressure is elevated, and auscultation of the lungs reveals bilateral crackles halfway up both lung fields. Cardiac examination shows an S3 gallop.

LABORATORY DATA:
- Sodium: 128 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 95 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen (BUN): 28 mg/dL
- Creatinine: 1.4 mg/dL
- Glucose: 98 mg/dL

## Encounter 7: source 2025-07-15, practice 2026-07-30, icu, Emergency Medicine

Chief complaint: Sudden onset of severe tearing chest pain radiating to the back

ENCOUNTER NOTE
Date: 2025-07-15
Provider: Dr. Robert Hassan, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Sudden onset of severe tearing chest pain radiating to the back

HISTORY OF PRESENT ILLNESS:
This 73-year-old male with ischemic cardiomyopathy with reduced ejection fraction and multiple prior admissions for heart failure exacerbations presents with sudden onset of severe chest pain. The pain is described as tearing in nature and has been persistent for the past 3 hours.

PAST MEDICAL HISTORY:
Active Problem List:
- Heart failure with reduced ejection fraction (diagnosed 2020-01-15)
- Left Lateral Wall Myocardial Infarction (diagnosed 2022-01-15)
- Ischemic cardiomyopathy with reduced ejection fraction (HFrEF, EF 25-30%)
- Hypertension
- Hyperlipidemia
- Chronic atrial fibrillation
- Chronic obstructive pulmonary disease (COPD)
- Chronic kidney disease stage III
- History of anterior wall myocardial infarction
- History of lateral wall myocardial infarction

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (anterior wall)
- Implantable cardioverter-defibrillator (ICD) placement
- Diagnostic cardiac catheterization

MEDICATIONS:
Home Medications:
- Furosemide 40 mg PO daily
- Lisinopril 20 mg PO daily
- Carvedilol 12.5 mg PO twice daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Warfarin 5 mg PO daily (titrate to INR 2.0-3.0)
- Tiotropium 18 mcg inhaled daily
- Fluticasone/salmeterol 250/50 mcg inhaled twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension and chronic kidney disease
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 5 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired construction foreman.

VITAL SIGNS:
His blood pressure is 170/90 mmHg, heart rate is 98 bpm

PHYSICAL EXAMINATION:
He is diaphoretic. Physical examination reveals a diastolic murmur best heard over the right sternal border. His extremities are cool to the touch with diminished pulses in the lower extremities.

LABORATORY DATA:
Serum creatinine: 2.8 mg/dL (baseline 1.9 mg/dL)
- Blood urea nitrogen (BUN): 44 mg/dL
- Hemoglobin: 13.2 g/dL
- White blood cell count: 9,000 /µL
- Platelets: 250,000 /µL
