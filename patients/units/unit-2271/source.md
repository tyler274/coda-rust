# synthetic_hospital patient 2271

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 65 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-05-13 (Wed) | outpatient | Nephrology |
| 1 | 2020-10-15 | 2021-02-12 (Fri) | outpatient | Cardiology |
| 2 | 2022-01-15 | 2022-05-06 (Fri) | ed | Emergency Medicine |
| 3 | 2024-01-15 | 2024-05-11 (Sat) | ed | Emergency Medicine |
| 4 | 2026-01-15 | 2026-05-15 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired financial analyst, active gardener",
 "alcohol_use": "None",
 "family_history": [
  "Father: Myocardial infarction at age 68",
  "Mother: Cerebrovascular accident",
  "Brother: End-stage renal disease on hemodialysis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Enalapril"
  },
  {
   "dose": "50 mg PO twice daily",
   "name": "Metoprolol tartrate"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "667 mg PO three times daily with meals",
   "name": "Calcium acetate"
  },
  {
   "dose": "4000 units subcutaneous weekly",
   "name": "Epoetin alfa"
  }
 ],
 "surgical_history": [
  "Cholecystectomy",
  "Left femoral-popliteal bypass graft (2 years prior)"
 ],
 "chronic_conditions": [
  "Hypertension (poorly controlled)",
  "Peripheral Vascular Disease",
  "Hyperlipidemia",
  "Chronic Kidney Disease Stage 3b"
 ],
 "age_at_first_encounter": 65
}
```

Primary diagnoses: ["Acute mesenteric ischemia", "Subclavian steal syndrome due to left subclavian artery stenosis", "Hyperzincemia", "Hyperkalemia", "Coagulopathy due to vitamin K deficiency"]
Comorbidities: ["Peripheral arterial disease", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2020-05-13, outpatient, Nephrology

Chief complaint: Routine follow-up for chronic kidney disease management.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for chronic kidney disease management.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension (poorly controlled)
- Peripheral Vascular Disease
- Hyperlipidemia
- Chronic Kidney Disease Stage 3b

PAST SURGICAL HISTORY:
- Cholecystectomy
- Left femoral-popliteal bypass graft (2 years prior)

MEDICATIONS:
He has been adherent to his medication regimen, which includes lisinopril and atorvastatin.

Home Medications:
- Enalapril 10 mg PO daily
- Metoprolol tartrate 50 mg PO twice daily
- Calcium acetate 667 mg PO three times daily with meals
- Epoetin alfa 4000 units subcutaneous weekly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Cerebrovascular accident
- Brother: End-stage renal disease on hemodialysis

SOCIAL HISTORY:
He does not smoke and maintains a healthy diet.

VITAL SIGNS:
His blood pressure today is 132/84 mmHg

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

LABORATORY DATA:
His lipid panel from today shows:

- Total cholesterol: 190 mg/dL
- HDL cholesterol: 45 mg/dL
- LDL cholesterol: 100 mg/dL
- Triglycerides: 225 mg/dL

ASSESSMENT AND PLAN:
The patient inquires about the potential benefits of adding fish oil supplements to further reduce his cardiovascular risk. Based on a recent large randomized controlled trial, the absolute risk reduction (ARR) of major cardiovascular events with fish oil supplementation was found to be 1.5%. Calculate the number needed to treat (NNT) to prevent one additional major cardiovascular event with fish oil supplementation.

## Encounter 1: source 2020-10-15, practice 2021-02-12, outpatient, Cardiology

Chief complaint: Transient episodes of dizziness, blurred vision, and lightheadedness when using left arm.

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Michael Okonkwo, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Transient episodes of dizziness, blurred vision, and lightheadedness when using left arm.

HISTORY OF PRESENT ILLNESS:
This 65-year-old male with peripheral vascular disease and poorly controlled hypertension, last evaluated for hyperzincemia, presents with transient episodes of dizziness, blurred vision, and lightheadedness, especially when using his left arm vigorously. He also reports occasional numbness and tingling in the left hand while gardening.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperzincemia (diagnosed 2020-01-15)
- Hypertension (poorly controlled)
- Peripheral Vascular Disease
- Hyperlipidemia
- Chronic Kidney Disease Stage 3b

His medical history includes hypertension and peripheral vascular disease.

PAST SURGICAL HISTORY:
- Cholecystectomy
- Left femoral-popliteal bypass graft (2 years prior)

MEDICATIONS:
Home Medications:
- Enalapril 10 mg PO daily
- Metoprolol tartrate 50 mg PO twice daily
- Atorvastatin 40 mg PO daily
- Calcium acetate 667 mg PO three times daily with meals
- Epoetin alfa 4000 units subcutaneous weekly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Cerebrovascular accident
- Brother: End-stage renal disease on hemodialysis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired financial analyst, active gardener.

VITAL SIGNS:
Blood pressure is 145/92 mmHg in the right arm and 118/74 mmHg in the left arm.

PHYSICAL EXAMINATION:
On exam, there is a bruit over the left supraclavicular fossa.

IMAGING:
MRI of the brain is unremarkable, but Doppler ultrasound shows reversed flow in the left vertebral artery.

## Encounter 2: source 2022-01-15, practice 2022-05-06, ed, Emergency Medicine

Chief complaint: Increased bruising and mild gum bleeding over the past few days following recent hospital discharge.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Jessica Lee, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increased bruising and mild gum bleeding over the past few days following recent hospital discharge.

HISTORY OF PRESENT ILLNESS:
The 67-year-old male with stage 3b chronic kidney disease, peripheral vascular disease, hyperzincemia, and subclavian steal syndrome returns to the emergency department for increased bruising and mild gum bleeding over the past few days. He was recently discharged after a prolonged hospital stay for a complicated biliary tract infection treated with multiple broad-spectrum antibiotics.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperzincemia (diagnosed 2020-01-15)
- Subclavian steal syndrome due to left subclavian artery stenosis (diagnosed 2020-10-15)
- Hypertension (poorly controlled)
- Peripheral Vascular Disease
- Hyperlipidemia
- Chronic Kidney Disease Stage 3b

PAST SURGICAL HISTORY:
- Cholecystectomy
- Left femoral-popliteal bypass graft (2 years prior)

MEDICATIONS:
Medications include metoprolol and a recent 6-week course of ceftriaxone and metronidazole.

Home Medications:
- Enalapril 10 mg PO daily
- Metoprolol tartrate 50 mg PO twice daily
- Atorvastatin 40 mg PO daily
- Calcium acetate 667 mg PO three times daily with meals
- Epoetin alfa 4000 units subcutaneous weekly

ALLERGIES:
NKDA

FAMILY HISTORY:
He has no family history of bleeding disorders

SOCIAL HISTORY:
He does not drink alcohol

VITAL SIGNS:
Vital signs are normal

PHYSICAL EXAMINATION:
On physical exam, He has scattered ecchymoses on his forearms and oozing from the gingival margin. Neurologic exam is normal.

LABORATORY DATA:
Laboratory testing shows a prolonged prothrombin time (PT), normal partial thromboplastin time (PTT), and normal bleeding time; platelet count is within normal limits.

## Encounter 3: source 2024-01-15, practice 2024-05-11, ed, Emergency Medicine

Chief complaint: Acute onset weakness and palpitations.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Robert Taylor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset weakness and palpitations.

HISTORY OF PRESENT ILLNESS:
This 69-year-old male with CKD and prior subclavian steal syndrome reports that his symptoms started a few hours ago and have progressively worsened.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperzincemia (diagnosed 2020-01-15)
- Subclavian steal syndrome due to left subclavian artery stenosis (diagnosed 2020-10-15)
- Coagulopathy due to vitamin K deficiency (diagnosed 2022-01-15)
- Hypertension (poorly controlled)
- Peripheral Vascular Disease
- Hyperlipidemia
- Chronic Kidney Disease Stage 3b

PAST SURGICAL HISTORY:
- Cholecystectomy
- Left femoral-popliteal bypass graft (2 years prior)

MEDICATIONS:
His medications include enalapril, calcium acetate, and erythropoietin.

Home Medications:
- Metoprolol tartrate 50 mg PO twice daily
- Atorvastatin 40 mg PO daily
- Epoetin alfa 4000 units subcutaneous weekly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Cerebrovascular accident
- Brother: End-stage renal disease on hemodialysis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired financial analyst, active gardener.

VITAL SIGNS:
Blood pressure is 145/90 mmHg, heart rate is 55 bpm

PHYSICAL EXAMINATION:
On examination, he appears lethargic

LABORATORY DATA:
Laboratory results are as follows:

- Serum potassium: 7.2 mEq/L (normal: 3.5-5.0 mEq/L)
- Serum creatinine: 4.5 mg/dL (normal: 0.6-1.2 mg/dL)
- Blood urea nitrogen (BUN): 55 mg/dL (normal: 7-20 mg/dL)
- Serum glucose: 110 mg/dL (normal: 70-99 mg/dL)

OTHER STUDIES:
An ECG shows peaked T waves and a widened QRS complex.

## Encounter 4: source 2026-01-15, practice 2026-05-15, ed, Emergency Medicine

Chief complaint: Intermittent lower abdominal cramping for two days, most notable during and after bowel movements, with loose bloody stools.

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. Emily Xu, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Intermittent lower abdominal cramping for two days, most notable during and after bowel movements, with loose bloody stools.

HISTORY OF PRESENT ILLNESS:
This 71-year-old male with peripheral vascular disease, chronic kidney disease, and prior subclavian steal syndrome presents to the emergency department with onset of loose stools that occasionally appear streaked with red. He also describes a sensation of incomplete evacuation. His appetite is decreased, and he has felt generally fatigued. He denies fever, recent travel, or use of antibiotics.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperzincemia (diagnosed 2020-01-15)
- Subclavian steal syndrome due to left subclavian artery stenosis (diagnosed 2020-10-15)
- Coagulopathy due to vitamin K deficiency (diagnosed 2022-01-15)
- Hyperkalemia (diagnosed 2024-01-15)
- Hypertension (poorly controlled)
- Peripheral Vascular Disease
- Hyperlipidemia
- Chronic Kidney Disease Stage 3b

His past medical history includes poorly controlled hypertension and peripheral vascular disease.

PAST SURGICAL HISTORY:
- Cholecystectomy
- Left femoral-popliteal bypass graft (2 years prior)

MEDICATIONS:
Home Medications:
- Enalapril 10 mg PO daily
- Metoprolol tartrate 50 mg PO twice daily
- Atorvastatin 40 mg PO daily
- Calcium acetate 667 mg PO three times daily with meals
- Epoetin alfa 4000 units subcutaneous weekly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Cerebrovascular accident
- Brother: End-stage renal disease on hemodialysis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired financial analyst, active gardener.

PHYSICAL EXAMINATION:
On examination, his abdomen is soft but mildly tender in the left lower quadrant without rebound or guarding. His femoral and dorsalis pedis pulses are weak and diminished bilaterally.

LABORATORY DATA:
Laboratory evaluation shows a mildly elevated white blood cell count.

IMAGING:
Imaging reveals nonspecific thickening of a colonic segment.

OTHER STUDIES:
Colonoscopy is subsequently performed to further evaluate his symptoms.
