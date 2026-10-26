# synthetic_hospital patient 2308

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White, Medicare with supplemental private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-11-21 (Thu) | inpatient | General Surgery |
| 1 | 2020-07-15 | 2025-05-22 (Thu) | icu | Emergency Medicine |
| 2 | 2021-01-15 | 2025-11-21 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Sulfamethoxazole-trimethoprim (rash)",
  "Iodinated contrast media (anaphylaxis)"
 ],
 "insurance": "Medicare with supplemental private insurance",
 "occupation": "Retired postal service supervisor",
 "alcohol_use": "Occasional social use, 1-2 glasses of wine per month",
 "family_history": [
  "Father: colon cancer diagnosed age 74, deceased",
  "Mother: type 2 diabetes mellitus with diabetic nephropathy",
  "One older brother: benign prostatic hyperplasia"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 30 pack-year history, quit 8 years ago",
 "home_medications": [
  {
   "dose": "15 mg by mouth every 12 hours",
   "name": "Morphine sulfate extended-release"
  },
  {
   "dose": "500 mg by mouth every 6 hours as needed for pain or fever",
   "name": "Acetaminophen"
  },
  {
   "dose": "0.4 mg by mouth daily",
   "name": "Tamsulosin"
  },
  {
   "dose": "800 mg by mouth three times daily with meals",
   "name": "Sevelamer carbonate"
  },
  {
   "dose": "1 tablet by mouth daily",
   "name": "Nephrocaps (renal multivitamin)"
  },
  {
   "dose": "81 mg by mouth daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Open right hemicolectomy with primary anastomosis for stage IIA colon cancer",
  "Cystoscopy with bladder biopsy",
  "Left radiocephalic arteriovenous fistula creation for hemodialysis access",
  "Bilateral inguinal hernia repair, age 54"
 ],
 "chronic_conditions": [
  "Benign prostatic hyperplasia",
  "Chronic kidney disease, stage 5 (end-stage renal disease) on maintenance hemodialysis",
  "Chronic lower back pain, secondary to degenerative disc disease",
  "History of colon adenocarcinoma, status post right hemicolectomy"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Septic shock", "Paralytic ileus", "Iliofemoral Deep Vein Thrombosis"]
Comorbidities: ["Status post laparoscopic cholecystectomy"]

## Encounter 0: source 2020-01-15, practice 2024-11-21, inpatient, General Surgery

Chief complaint: Persistent nausea, abdominal distension, and has not passed flatus or stool since surgery.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: General Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Persistent nausea, abdominal distension, and has not passed flatus or stool since surgery.

HISTORY OF PRESENT ILLNESS:
He presents to the surgical ward on postoperative day 5 following an open right hemicolectomy for colon cancer.

PAST MEDICAL HISTORY:
Active Problem List:
- Benign prostatic hyperplasia
- Chronic kidney disease, stage 5 (end-stage renal disease) on maintenance hemodialysis
- Chronic lower back pain, secondary to degenerative disc disease
- History of colon adenocarcinoma, status post right hemicolectomy

His past medical history is significant for chronic back pain for which he takes extended-release morphine.

PAST SURGICAL HISTORY:
- Open right hemicolectomy with primary anastomosis for stage IIA colon cancer
- Cystoscopy with bladder biopsy
- Left radiocephalic arteriovenous fistula creation for hemodialysis access
- Bilateral inguinal hernia repair, age 54

MEDICATIONS:
Home Medications:
- Morphine sulfate extended-release 15 mg by mouth every 12 hours
- Acetaminophen 500 mg by mouth every 6 hours as needed for pain or fever
- Tamsulosin 0.4 mg by mouth daily
- Sevelamer carbonate 800 mg by mouth three times daily with meals
- Nephrocaps (renal multivitamin) 1 tablet by mouth daily
- Aspirin 81 mg by mouth daily

ALLERGIES:
Sulfamethoxazole-trimethoprim (rash), Iodinated contrast media (anaphylaxis)

FAMILY HISTORY:
- Father: colon cancer diagnosed age 74, deceased
- Mother: type 2 diabetes mellitus with diabetic nephropathy
- One older brother: benign prostatic hyperplasia

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-year history, quit 8 years ago. Alcohol: Occasional social use, 1-2 glasses of wine per month. Occupation: Retired postal service supervisor.

VITAL SIGNS:
His temperature is 37.2°C (99°F), blood pressure is 135/85 mmHg, and heart rate is 88 beats per minute.

PHYSICAL EXAMINATION:
On examination, his abdomen is distended and tympanic to percussion, with hypoactive bowel sounds.

LABORATORY DATA:
Laboratory tests reveal:
- Hemoglobin: 12.8 g/dL
- White blood cell count: 9,000/mm³
- Potassium: 3.9 mEq/L
- Sodium: 140 mEq/L
- Creatinine: 1.0 mg/dL
- Blood urea nitrogen: 18 mg/dL

## Encounter 1: source 2020-07-15, practice 2025-05-22, icu, Emergency Medicine

Chief complaint: Confusion and chills 2 days after discharge following cystoscopic procedure, with dysuria and malaise.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Confusion and chills 2 days after discharge following cystoscopic procedure, with dysuria and malaise.

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with end-stage renal disease on maintenance hemodialysis and a history of paralytic ileus following right hemicolectomy earlier this year presents to the hospital two days after discharge following a cystoscopic procedure with confusion and chills. He reports dysuria and malaise that began the previous evening.

PAST MEDICAL HISTORY:
Active Problem List:
- Paralytic ileus (diagnosed 2020-01-15)
- Benign prostatic hyperplasia
- Chronic kidney disease, stage 5 (end-stage renal disease) on maintenance hemodialysis
- Chronic lower back pain, secondary to degenerative disc disease
- History of colon adenocarcinoma, status post right hemicolectomy

PAST SURGICAL HISTORY:
- Open right hemicolectomy with primary anastomosis for stage IIA colon cancer
- Cystoscopy with bladder biopsy
- Left radiocephalic arteriovenous fistula creation for hemodialysis access
- Bilateral inguinal hernia repair, age 54

MEDICATIONS:
Home Medications:
- Morphine sulfate extended-release 15 mg by mouth every 12 hours
- Acetaminophen 500 mg by mouth every 6 hours as needed for pain or fever
- Tamsulosin 0.4 mg by mouth daily
- Sevelamer carbonate 800 mg by mouth three times daily with meals
- Nephrocaps (renal multivitamin) 1 tablet by mouth daily
- Aspirin 81 mg by mouth daily

ALLERGIES:
Sulfamethoxazole-trimethoprim (rash), Iodinated contrast media (anaphylaxis)

FAMILY HISTORY:
- Father: colon cancer diagnosed age 74, deceased
- Mother: type 2 diabetes mellitus with diabetic nephropathy
- One older brother: benign prostatic hyperplasia

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-year history, quit 8 years ago. Alcohol: Occasional social use, 1-2 glasses of wine per month. Occupation: Retired postal service supervisor.

VITAL SIGNS:
His temperature is 38.9°C (102°F), heart rate is 110/min, and blood pressure is 80/50 mm Hg.

PHYSICAL EXAMINATION:
Physical examination reveals diaphoresis but no focal signs of infection.

LABORATORY DATA:
Laboratory studies demonstrate leukocytosis and elevated serum lactate. Urinalysis reveals numerous white blood cells and bacteria. Blood cultures are pending, but Gram stain of urine shows pink, rod-shaped organisms that ferment lactose.

ASSESSMENT AND PLAN:
Over the next few hours, his blood pressure remains low despite fluid resuscitation.

The patient is started on empiric broad-spectrum antibiotics and intravenous fluids.

## Encounter 2: source 2021-01-15, practice 2025-11-21, ed, Emergency Medicine

Chief complaint: Left calf pain and swelling for 2 days.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Left calf pain and swelling for 2 days.

HISTORY OF PRESENT ILLNESS:
This 68-year-old with ESRD on hemodialysis and history of paralytic ileus and septic shock presents to the emergency department with a 2-day history of left calf pain and swelling. His postoperative course was uneventful, and he was discharged on postoperative day 3.

PAST MEDICAL HISTORY:
Active Problem List:
- Paralytic ileus (diagnosed 2020-01-15)
- Septic shock (diagnosed 2020-07-15)
- Benign prostatic hyperplasia
- Chronic kidney disease, stage 5 (end-stage renal disease) on maintenance hemodialysis
- Chronic lower back pain, secondary to degenerative disc disease
- History of colon adenocarcinoma, status post right hemicolectomy

PAST SURGICAL HISTORY:
He underwent a right hemicolectomy for colon cancer 5 days ago.

MEDICATIONS:
He takes acetaminophen and a multivitamin.

Home Medications:
- Morphine sulfate extended-release 15 mg by mouth every 12 hours
- Tamsulosin 0.4 mg by mouth daily
- Sevelamer carbonate 800 mg by mouth three times daily with meals
- Nephrocaps (renal multivitamin) 1 tablet by mouth daily
- Aspirin 81 mg by mouth daily

ALLERGIES:
Sulfamethoxazole-trimethoprim (rash), Iodinated contrast media (anaphylaxis)

FAMILY HISTORY:
- Father: colon cancer diagnosed age 74, deceased
- Mother: type 2 diabetes mellitus with diabetic nephropathy
- One older brother: benign prostatic hyperplasia

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-year history, quit 8 years ago. Alcohol: Occasional social use, 1-2 glasses of wine per month. Occupation: Retired postal service supervisor.

PHYSICAL EXAMINATION:
On examination, his left calf is erythematous, warm, and tender to palpation. The circumference of the left calf is 3 cm greater than the right.

LABORATORY DATA:
Hemoglobin: 10.2 g/dL
- Platelet count: 250,000/mm^3
- Creatinine: 6.5 mg/dL (baseline)
- INR: 1.0

IMAGING:
Doppler ultrasound of the lower extremities reveals a noncompressible segment in the left popliteal vein.
