# synthetic_hospital patient 2051

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Medicare primary (ESRD eligibility), Blue Cross Blue Shield secondary.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-01-07 (Thu) | inpatient | Nephrology |
| 1 | 2020-03-15 | 2021-03-01 (Mon) | ed | Emergency Medicine |
| 2 | 2020-04-15 | 2021-04-12 (Mon) | inpatient | Gastroenterology |
| 3 | 2020-05-15 | 2021-04-28 (Wed) | outpatient | Nephrology |
| 4 | 2020-06-15 | 2021-06-07 (Mon) | ed | Nephrology |
| 5 | 2021-01-15 | 2022-01-12 (Wed) | inpatient | Pulmonology |
| 6 | 2021-07-15 | 2022-07-05 (Tue) | icu | Critical Care Medicine |
| 7 | 2025-01-15 | 2026-01-14 (Wed) | outpatient | Hematology & Oncology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare primary (ESRD eligibility), Blue Cross Blue Shield secondary",
 "occupation": "High school mathematics teacher",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Father: Autosomal dominant polycystic kidney disease, deceased age 58 from ruptured cerebral aneurysm",
  "Mother: Hypertension, type 2 diabetes mellitus",
  "Sister: Autosomal dominant polycystic kidney disease, currently on hemodialysis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 12 months prior to transplantation",
 "home_medications": [
  {
   "dose": "5 mg PO BID (target trough 8-10 ng/mL)",
   "name": "Tacrolimus"
  },
  {
   "dose": "750 mg PO BID",
   "name": "Mycophenolate mofetil"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Prednisone"
  },
  {
   "dose": "24 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "Sliding scale 1:12 ratio with meals",
   "name": "Insulin lispro"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "Single-strength tablet PO daily (PCP prophylaxis)",
   "name": "Trimethoprim-sulfamethoxazole"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Deceased donor renal transplantation (6 months prior to initial encounter)",
  "Left native nephrectomy (10 years prior for infected cyst)",
  "Right native kidney in situ"
 ],
 "chronic_conditions": [
  "Autosomal dominant polycystic kidney disease",
  "Status post deceased donor renal transplantation",
  "Type 2 diabetes mellitus",
  "Hypertension",
  "Chronic kidney disease stage 5T"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Acute rejection of lung transplant", "Chronic kidney allograft rejection", "Autosomal Dominant Polycystic Kidney Disease", "History of Pneumocystis jirovecii pneumonia", "Diabetic ketoacidosis", "Cytomegalovirus colitis", "Post-transplant lymphoproliferative disorder, polymorphic type, EBV-associated"]
Comorbidities: ["History of kidney transplantation", "Iatrogenic immunosuppression", "End-stage renal disease on hemodialysis", "Essential hypertension", "Hyperkalemia"]

## Encounter 0: source 2020-01-15, practice 2021-01-07, inpatient, Nephrology

Chief complaint: Decreased urine output, fatigue, and lower extremity swelling two weeks after kidney transplantation

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Nephrology
Visit Type: inpatient

CHIEF COMPLAINT:
Decreased urine output, fatigue, and lower extremity swelling two weeks after kidney transplantation

HISTORY OF PRESENT ILLNESS:
Two weeks after receiving a renal transplant

PAST MEDICAL HISTORY:
Active Problem List:
- Autosomal dominant polycystic kidney disease
- Status post deceased donor renal transplantation
- Type 2 diabetes mellitus
- Hypertension
- Chronic kidney disease stage 5T

PAST SURGICAL HISTORY:
- Deceased donor renal transplantation (6 months prior to initial encounter)
- Left native nephrectomy (10 years prior for infected cyst)
- Right native kidney in situ

MEDICATIONS:
Home Medications:
- Tacrolimus 5 mg PO BID (target trough 8-10 ng/mL)
- Mycophenolate mofetil 750 mg PO BID
- Prednisone 5 mg PO daily
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro Sliding scale 1:12 ratio with meals
- Amlodipine 10 mg PO daily
- Trimethoprim-sulfamethoxazole Single-strength tablet PO daily (PCP prophylaxis)
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Autosomal dominant polycystic kidney disease, deceased age 58 from ruptured cerebral aneurysm
- Mother: Hypertension, type 2 diabetes mellitus
- Sister: Autosomal dominant polycystic kidney disease, currently on hemodialysis

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 months prior to transplantation. Alcohol: Social (1-2 drinks per month). Occupation: High school mathematics teacher.

VITAL SIGNS:
Blood pressure of 160/95 mm Hg and a temperature of 37.2°C (99°F)

PHYSICAL EXAMINATION:
Mild bilateral lower extremity edema and left lower quadrant tenderness near the graft site

LABORATORY DATA:
Serum creatinine of 3.2 mg/dL (baseline 1.1 mg/dL), BUN of 46 mg/dL, and elevated serum potassium. Urinalysis shows mild proteinuria and white blood cell casts.

PATHOLOGY:
Biopsy of the renal graft demonstrates mononuclear cell infiltration and tubular injury

## Encounter 1: source 2020-03-15, practice 2021-03-01, ed, Emergency Medicine

Chief complaint: Worsening right flank pain and gross hematuria

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening right flank pain and gross hematuria

PAST MEDICAL HISTORY:
Active Problem List:
- Acute rejection of lung transplant (diagnosed 2020-01-15)
- Autosomal dominant polycystic kidney disease
- Status post deceased donor renal transplantation
- Type 2 diabetes mellitus
- Hypertension
- Chronic kidney disease stage 5T

PAST SURGICAL HISTORY:
- Deceased donor renal transplantation (6 months prior to initial encounter)
- Left native nephrectomy (10 years prior for infected cyst)
- Right native kidney in situ

MEDICATIONS:
Home Medications:
- Tacrolimus 5 mg PO BID (target trough 8-10 ng/mL)
- Mycophenolate mofetil 750 mg PO BID
- Prednisone 5 mg PO daily
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro Sliding scale 1:12 ratio with meals
- Amlodipine 10 mg PO daily
- Trimethoprim-sulfamethoxazole Single-strength tablet PO daily (PCP prophylaxis)
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
He has a family history of kidney disease; his father required dialysis in his 50s.

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 months prior to transplantation. Alcohol: Social (1-2 drinks per month). Occupation: High school mathematics teacher.

VITAL SIGNS:
Blood pressure is 160/100 mmHg

PHYSICAL EXAMINATION:
Physical examination shows bilateral flank masses

LABORATORY DATA:
Urinalysis shows microscopic hematuria and mild proteinuria

IMAGING:
A renal ultrasound demonstrates multiple large cysts in both kidneys. Brain MRI performed for headache reveals a saccular aneurysm at the anterior communicating artery.

ASSESSMENT AND PLAN:
A mutation involving

Genetic testing is ordered.

## Encounter 2: source 2020-04-15, practice 2021-04-12, inpatient, Gastroenterology

Chief complaint: Abdominal pain, vomiting, and bloody diarrhea

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: inpatient

CHIEF COMPLAINT:
Abdominal pain, vomiting, and bloody diarrhea

HISTORY OF PRESENT ILLNESS:
The patient with ADPKD s/p renal transplant, prior rejection, and hematuria presents to the clinic with a 1-week history of abdominal pain, vomiting, and bloody diarrhea. He denies any recent travel or sick contacts.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute rejection of lung transplant (diagnosed 2020-01-15)
- Autosomal Dominant Polycystic Kidney Disease (diagnosed 2020-03-15)
- Status post deceased donor renal transplantation
- Type 2 diabetes mellitus
- Hypertension
- Chronic kidney disease stage 5T

PAST SURGICAL HISTORY:
He underwent a renal transplantation 3 months ago

MEDICATIONS:
He has been on immunosuppressive therapy with tacrolimus, mycophenolate mofetil, and prednisone.

Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro Sliding scale 1:12 ratio with meals
- Amlodipine 10 mg PO daily
- Trimethoprim-sulfamethoxazole Single-strength tablet PO daily (PCP prophylaxis)
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Autosomal dominant polycystic kidney disease, deceased age 58 from ruptured cerebral aneurysm
- Mother: Hypertension, type 2 diabetes mellitus
- Sister: Autosomal dominant polycystic kidney disease, currently on hemodialysis

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 months prior to transplantation. Alcohol: Social (1-2 drinks per month). Occupation: High school mathematics teacher.

VITAL SIGNS:
On examination, he is afebrile, blood pressure is 130/85 mmHg, heart rate is 88 beats per minute

PHYSICAL EXAMINATION:
Abdominal examination reveals diffuse tenderness without rebound or guarding.

LABORATORY DATA:
Laboratory tests reveal the following:

- Hemoglobin: 12.1 g/dL
- White blood cell count: 9,000/uL with a normal differential
- Platelets: 250,000/uL
- Serum creatinine: 1.2 mg/dL (baseline post-transplant)
- Liver enzymes: within normal limits
- CMV PCR: Positive with high viral load

OTHER STUDIES:
An endoscopy is performed, showing multiple, large, shallow ulcers in the colon. Biopsies of the ulcers are taken.

## Encounter 3: source 2020-05-15, practice 2021-04-28, outpatient, Nephrology

Chief complaint: Routine follow-up for rising serum creatinine

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. James Wilson, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for rising serum creatinine

HISTORY OF PRESENT ILLNESS:
This 45-year-old male with autosomal dominant polycystic kidney disease returns for routine nephrology follow-up four months after deceased donor kidney transplantation, following cytomegalovirus colitis one month ago. Four months after receiving a kidney transplant due to end-stage renal disease secondary to hypertension, he has no complaints and has been adherent to his immunosuppressive regimen, which includes tacrolimus, mycophenolate mofetil, and prednisone. The patient's serum creatinine level has risen from a baseline of 1.2 mg/dL to 2.8 mg/dL since his last visit one month ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute rejection of lung transplant (diagnosed 2020-01-15)
- Autosomal Dominant Polycystic Kidney Disease (diagnosed 2020-03-15)
- Cytomegalovirus colitis (diagnosed 2020-04-15)
- Status post deceased donor renal transplantation
- Type 2 diabetes mellitus
- Hypertension
- Chronic kidney disease stage 5T

PAST SURGICAL HISTORY:
- Deceased donor renal transplantation (6 months prior to initial encounter)
- Left native nephrectomy (10 years prior for infected cyst)
- Right native kidney in situ

MEDICATIONS:
He takes tacrolimus, mycophenolate mofetil, and prednisone.

Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro Sliding scale 1:12 ratio with meals
- Amlodipine 10 mg PO daily
- Trimethoprim-sulfamethoxazole Single-strength tablet PO daily (PCP prophylaxis)
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Autosomal dominant polycystic kidney disease, deceased age 58 from ruptured cerebral aneurysm
- Mother: Hypertension, type 2 diabetes mellitus
- Sister: Autosomal dominant polycystic kidney disease, currently on hemodialysis

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 months prior to transplantation. Alcohol: Social (1-2 drinks per month). Occupation: High school mathematics teacher.

VITAL SIGNS:
Blood pressure is 132/86 mmHg

PHYSICAL EXAMINATION:
Physical examination is unremarkable

LABORATORY DATA:
Serum creatinine: 2.8 mg/dL (baseline: 1.2 mg/dL)
Blood urea nitrogen (BUN): 44 mg/dL
Urinalysis: No proteinuria or hematuria
Serum tacrolimus level: Within therapeutic range

## Encounter 4: source 2020-06-15, practice 2021-06-07, ed, Nephrology

Chief complaint: Malaise and decreased urine output two weeks after last clinic visit

ENCOUNTER NOTE
Date: 2020-06-15
Provider: Dr. Michael Chang, MD
Department: Nephrology
Visit Type: ed

CHIEF COMPLAINT:
Malaise and decreased urine output two weeks after last clinic visit

HISTORY OF PRESENT ILLNESS:
The patient with autosomal dominant polycystic kidney disease status post deceased donor renal transplantation returns two weeks after his last clinic visit with malaise and decreased urine output. He reports mild tenderness over the graft site but no fever or chills.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute rejection of lung transplant (diagnosed 2020-01-15)
- Autosomal Dominant Polycystic Kidney Disease (diagnosed 2020-03-15)
- Cytomegalovirus colitis (diagnosed 2020-04-15)
- Status post deceased donor renal transplantation
- Type 2 diabetes mellitus
- Hypertension
- Chronic kidney disease stage 5T

PAST SURGICAL HISTORY:
Undergoes kidney transplantation

MEDICATIONS:
Home Medications:
- Tacrolimus 5 mg PO BID (target trough 8-10 ng/mL)
- Mycophenolate mofetil 750 mg PO BID
- Prednisone 5 mg PO daily
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro Sliding scale 1:12 ratio with meals
- Amlodipine 10 mg PO daily
- Trimethoprim-sulfamethoxazole Single-strength tablet PO daily (PCP prophylaxis)
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Autosomal dominant polycystic kidney disease, deceased age 58 from ruptured cerebral aneurysm
- Mother: Hypertension, type 2 diabetes mellitus
- Sister: Autosomal dominant polycystic kidney disease, currently on hemodialysis

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 months prior to transplantation. Alcohol: Social (1-2 drinks per month). Occupation: High school mathematics teacher.

VITAL SIGNS:
His blood pressure is elevated compared with baseline.

PHYSICAL EXAMINATION:
Physical examination shows a firm, slightly enlarged graft on deep palpation.

LABORATORY DATA:
Serum creatinine has risen sharply from postoperative baseline values.

IMAGING:
Doppler ultrasonography of the graft shows patent vessels with normal perfusion and no evidence of obstruction.

OTHER STUDIES:
A renal biopsy is performed for diagnostic evaluation.

## Encounter 5: source 2021-01-15, practice 2022-01-12, inpatient, Pulmonology

Chief complaint: Progressive dyspnea, nonproductive cough, and low-grade fevers

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. David Kim, MD
Department: Pulmonology
Visit Type: inpatient

CHIEF COMPLAINT:
Progressive dyspnea, nonproductive cough, and low-grade fevers

HISTORY OF PRESENT ILLNESS:
The patient with ADPKD s/p renal transplant and prior acute rejection presents to the clinic with a 2-week history of progressive dyspnea, nonproductive cough, and low-grade fevers.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute rejection of lung transplant (diagnosed 2020-01-15)
- Autosomal Dominant Polycystic Kidney Disease (diagnosed 2020-03-15)
- Cytomegalovirus colitis (diagnosed 2020-04-15)
- Chronic kidney allograft rejection (diagnosed 2020-06-15)
- Status post deceased donor renal transplantation
- Type 2 diabetes mellitus
- Hypertension
- Chronic kidney disease stage 5T

PAST SURGICAL HISTORY:
He underwent a renal transplant 6 months ago

MEDICATIONS:
He has been on a regimen of tacrolimus, mycophenolate mofetil, and prednisone.

Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro Sliding scale 1:12 ratio with meals
- Amlodipine 10 mg PO daily
- Trimethoprim-sulfamethoxazole Single-strength tablet PO daily (PCP prophylaxis)
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Autosomal dominant polycystic kidney disease, deceased age 58 from ruptured cerebral aneurysm
- Mother: Hypertension, type 2 diabetes mellitus
- Sister: Autosomal dominant polycystic kidney disease, currently on hemodialysis

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 months prior to transplantation. Alcohol: Social (1-2 drinks per month). Occupation: High school mathematics teacher.

VITAL SIGNS:
On examination, he is afebrile, tachypneic with a respiratory rate of 28 breaths per minute, and his oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
Lung auscultation reveals bilateral diffuse crackles.

LABORATORY DATA:
Laboratory tests reveal:

- White blood cell count: 6,000 cells/µL
- Hemoglobin: 13.2 g/dL
- Platelet count: 250,000 cells/µL
- Serum creatinine: 1.5 mg/dL (baseline 1.3 mg/dL)
- Lactate dehydrogenase (LDH): 450 U/L (normal 140-280 U/L)

IMAGING:
Chest X-ray shows bilateral interstitial infiltrates.

OTHER STUDIES:
An induced sputum sample is negative for bacterial, viral, and fungal pathogens on routine stains and cultures.

## Encounter 6: source 2021-07-15, practice 2022-07-05, icu, Critical Care Medicine

Chief complaint: Shortness of breath, polyuria, and increased thirst

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Lisa Thompson, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Shortness of breath, polyuria, and increased thirst

HISTORY OF PRESENT ILLNESS:
The patient is a 46-year-old male with autosomal dominant polycystic kidney disease status post deceased donor renal transplantation and history of allograft rejection. He presents to the emergency department with a 2-day history of shortness of breath, polyuria, and an increased thirst. He reports that he has been non-compliant with his insulin regimen.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute rejection of lung transplant (diagnosed 2020-01-15)
- Autosomal Dominant Polycystic Kidney Disease (diagnosed 2020-03-15)
- Cytomegalovirus colitis (diagnosed 2020-04-15)
- Chronic kidney allograft rejection (diagnosed 2020-06-15)
- History of Pneumocystis jirovecii pneumonia (diagnosed 2021-01-15)
- Status post deceased donor renal transplantation
- Type 2 diabetes mellitus
- Hypertension
- Chronic kidney disease stage 5T

PAST SURGICAL HISTORY:
- Deceased donor renal transplantation (6 months prior to initial encounter)
- Left native nephrectomy (10 years prior for infected cyst)
- Right native kidney in situ

MEDICATIONS:
Home Medications:
- Tacrolimus 5 mg PO BID (target trough 8-10 ng/mL)
- Mycophenolate mofetil 750 mg PO BID
- Prednisone 5 mg PO daily
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro Sliding scale 1:12 ratio with meals
- Amlodipine 10 mg PO daily
- Trimethoprim-sulfamethoxazole Single-strength tablet PO daily (PCP prophylaxis)
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Autosomal dominant polycystic kidney disease, deceased age 58 from ruptured cerebral aneurysm
- Mother: Hypertension, type 2 diabetes mellitus
- Sister: Autosomal dominant polycystic kidney disease, currently on hemodialysis

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 months prior to transplantation. Alcohol: Social (1-2 drinks per month). Occupation: High school mathematics teacher.

VITAL SIGNS:
His blood pressure is 130/85 mmHg, heart rate is 110 bpm, and temperature is 37.2°C (99°F).

PHYSICAL EXAMINATION:
His breathing is deep and rapid, and his skin is warm and dry

LABORATORY DATA:
- Serum glucose: 480 mg/dL
- Serum bicarbonate (HCO3): 12 mEq/L
- Arterial pH: 7.20
- Serum potassium: 5.2 mEq/L
- Serum sodium: 140 mEq/L
- Serum chloride (Cl): 95 mEq/L
- Blood urea nitrogen (BUN): 28 mg/dL
- Serum creatinine: 1.2 mg/dL
- Urine ketones: Positive

## Encounter 7: source 2025-01-15, practice 2026-01-14, outpatient, Hematology & Oncology

Chief complaint: Fatigue, intermittent fever, and 5-pound weight loss over 3 weeks

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Robert Garcia, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, intermittent fever, and 5-pound weight loss over 3 weeks

HISTORY OF PRESENT ILLNESS:
The patient is a 50-year-old male with a deceased donor renal transplant on immunosuppression complicated by prior CMV colitis and Pneumocystis pneumonia. He presents with a 3-week history of fatigue, intermittent fever, and a 5-pound weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute rejection of lung transplant (diagnosed 2020-01-15)
- Autosomal Dominant Polycystic Kidney Disease (diagnosed 2020-03-15)
- Cytomegalovirus colitis (diagnosed 2020-04-15)
- Chronic kidney allograft rejection (diagnosed 2020-06-15)
- History of Pneumocystis jirovecii pneumonia (diagnosed 2021-01-15)
- Diabetic ketoacidosis (diagnosed 2021-07-15)
- Status post deceased donor renal transplantation
- Type 2 diabetes mellitus
- Hypertension
- Chronic kidney disease stage 5T

He underwent a renal transplant 6 months ago.

PAST SURGICAL HISTORY:
- Deceased donor renal transplantation (6 months prior to initial encounter)
- Left native nephrectomy (10 years prior for infected cyst)
- Right native kidney in situ

MEDICATIONS:
He has been on tacrolimus, mycophenolate, and prednisone for immunosuppression.

Home Medications:
- Mycophenolate mofetil 750 mg PO BID
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro Sliding scale 1:12 ratio with meals
- Amlodipine 10 mg PO daily
- Trimethoprim-sulfamethoxazole Single-strength tablet PO daily (PCP prophylaxis)
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Autosomal dominant polycystic kidney disease, deceased age 58 from ruptured cerebral aneurysm
- Mother: Hypertension, type 2 diabetes mellitus
- Sister: Autosomal dominant polycystic kidney disease, currently on hemodialysis

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 months prior to transplantation. Alcohol: Social (1-2 drinks per month). Occupation: High school mathematics teacher.

VITAL SIGNS:
Temperature is 38.3°C (100.9°F)

PHYSICAL EXAMINATION:
Bilateral cervical lymphadenopathy and mild hepatosplenomegaly

LABORATORY DATA:
White blood cell count: 12,000/µL
- Hemoglobin: 10.5 g/dL
- Platelet count: 150,000/µL
- Serum creatinine: 1.2 mg/dL (baseline post-transplant)
- Lactate dehydrogenase (LDH): 450 U/L (normal 140-280 U/L)
- EBV PCR: Positive with high viral load

PATHOLOGY:
An excisional biopsy of a cervical lymph node is performed, and histopathology reveals a polymorphic lymphoid proliferation with numerous plasma cells. Immunohistochemistry is positive for CD20 and EBV-encoded RNA (EBER).
