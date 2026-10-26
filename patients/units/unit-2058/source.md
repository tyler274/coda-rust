# synthetic_hospital patient 2058

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 55 at first encounter, sex M, Non-Hispanic White, Commercial health insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-03-24 (Fri) | outpatient | Dermatology |
| 1 | 2020-04-15 | 2023-06-26 (Mon) | outpatient | Neurology |
| 2 | 2020-09-15 | 2023-11-13 (Mon) | outpatient | Endocrinology |
| 3 | 2021-03-15 | 2024-05-13 (Mon) | outpatient | Nephrology |
| 4 | 2021-09-15 | 2024-11-26 (Tue) | ed | Emergency Medicine |
| 5 | 2022-03-15 | 2025-05-13 (Tue) | inpatient | Internal Medicine |
| 6 | 2022-09-15 | 2025-11-15 (Sat) | ed | Emergency Medicine |
| 7 | 2023-03-15 | 2026-05-12 (Tue) | inpatient | Infectious Disease |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial health insurance",
 "occupation": "Construction site supervisor",
 "alcohol_use": "No alcohol use",
 "family_history": [
  "Father: Type 2 diabetes mellitus, hypertension, premature coronary artery disease",
  "Mother: Chronic obstructive pulmonary disease (never smoker), osteoporosis",
  "Brother: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "18 mcg inhaled daily",
   "name": "Tiotropium bromide"
  },
  {
   "dose": "300 mg PO three times daily",
   "name": "Gabapentin"
  },
  {
   "dose": "Applied to affected nails daily",
   "name": "Ciclopirox topical solution"
  }
 ],
 "surgical_history": [
  "Dental extraction with implant placement (recent)",
  "Debridement of right foot ulcer (recent)",
  "Primary suture repair of left lower extremity laceration (recent)"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus (15-year duration, variable glycemic control)",
  "Hypertension",
  "Chronic Kidney Disease Stage 3",
  "Chronic Obstructive Pulmonary Disease (occupational dust exposure-related)",
  "Diabetic Peripheral Neuropathy (sensorimotor)",
  "Onychomycosis",
  "Aspiration risk due to gastroesophageal reflux disease"
 ],
 "age_at_first_encounter": 55
}
```

Primary diagnoses: ["Type 2 diabetes mellitus", "Chronic Kidney Disease with Volume Overload", "Diabetic motor neuropathy", "Recurrent aspiration pneumonia", "Necrotizing fasciitis", "Pneumococcal pneumonia", "Staphylococcus aureus bacteremia with vertebral osteomyelitis", "Onychomycosis"]
Comorbidities: ["Chronic obstructive pulmonary disease", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2023-03-24, outpatient, Dermatology

Chief complaint: Thickened, discolored toenails for 6 months causing embarrassment

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Thickened, discolored toenails for 6 months causing embarrassment

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 6-month history of changes in his toenails. He reports that his toenails have become thickened, discolored, and difficult to trim. He denies any pain but feels embarrassed to wear sandals in public.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus (15-year duration, variable glycemic control)
- Hypertension
- Chronic Kidney Disease Stage 3
- Chronic Obstructive Pulmonary Disease (occupational dust exposure-related)
- Diabetic Peripheral Neuropathy (sensorimotor)
- Onychomycosis
- Aspiration risk due to gastroesophageal reflux disease

His medical history is significant for type 2 diabetes mellitus, well-controlled on metformin, and a previous episode of athlete's foot treated with topical antifungals.

PAST SURGICAL HISTORY:
- Dental extraction with implant placement (recent)
- Debridement of right foot ulcer (recent)
- Primary suture repair of left lower extremity laceration (recent)

MEDICATIONS:
He takes metformin.

Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Tiotropium bromide 18 mcg inhaled daily
- Gabapentin 300 mg PO three times daily
- Ciclopirox topical solution Applied to affected nails daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension, premature coronary artery disease
- Mother: Chronic obstructive pulmonary disease (never smoker), osteoporosis
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Construction site supervisor.

PHYSICAL EXAMINATION:
On physical examination, the toenails of both feet are dystrophic with yellow-brown discoloration, subungual hyperkeratosis, and onycholysis. There is no evidence of acute infection or cellulitis.

OTHER STUDIES:
A potassium hydroxide (KOH) microscopy of nail scrapings is performed, revealing branching hyphae.

## Encounter 1: source 2020-04-15, practice 2023-06-26, outpatient, Neurology

Chief complaint: Progressive numbness and tingling in feet with stocking distribution and nocturnal stumbling

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. James Okafor, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive numbness and tingling in feet with stocking distribution and nocturnal stumbling

HISTORY OF PRESENT ILLNESS:
The diabetic patient returns for neurologic follow-up post-onychomycosis. He describes the sensation as a "stocking" distribution. He also reports occasional stumbling when walking, especially in the dark.

PAST MEDICAL HISTORY:
Active Problem List:
- Onychomycosis (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus (15-year duration, variable glycemic control)
- Hypertension
- Chronic Kidney Disease Stage 3
- Chronic Obstructive Pulmonary Disease (occupational dust exposure-related)
- Diabetic Peripheral Neuropathy (sensorimotor)
- Aspiration risk due to gastroesophageal reflux disease

His diabetes has been managed with metformin and dietary modifications.

PAST SURGICAL HISTORY:
- Dental extraction with implant placement (recent)
- Debridement of right foot ulcer (recent)
- Primary suture repair of left lower extremity laceration (recent)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Tiotropium bromide 18 mcg inhaled daily
- Gabapentin 300 mg PO three times daily
- Ciclopirox topical solution Applied to affected nails daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension, premature coronary artery disease
- Mother: Chronic obstructive pulmonary disease (never smoker), osteoporosis
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Construction site supervisor.

PHYSICAL EXAMINATION:
On physical examination, vibratory sense is diminished at the toes and ankles bilaterally. Proprioception is impaired at the toes. Ankle reflexes are diminished, but knee reflexes are normal. His gait is steady with eyes open but becomes unsteady with eyes closed. The remainder of the neurological examination, including strength testing, is normal.

## Encounter 2: source 2020-09-15, practice 2023-11-13, outpatient, Endocrinology

Chief complaint: Routine diabetes management and discussion of HbA1c optimization

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Emily Rodriguez, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine diabetes management and discussion of HbA1c optimization

PAST MEDICAL HISTORY:
Active Problem List:
- Onychomycosis (diagnosed 2020-01-15)
- Diabetic motor neuropathy (diagnosed 2020-04-15)
- Type 2 Diabetes Mellitus (15-year duration, variable glycemic control)
- Hypertension
- Chronic Kidney Disease Stage 3
- Chronic Obstructive Pulmonary Disease (occupational dust exposure-related)
- Diabetic Peripheral Neuropathy (sensorimotor)
- Aspiration risk due to gastroesophageal reflux disease

PAST SURGICAL HISTORY:
- Dental extraction with implant placement (recent)
- Debridement of right foot ulcer (recent)
- Primary suture repair of left lower extremity laceration (recent)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Tiotropium bromide 18 mcg inhaled daily
- Gabapentin 300 mg PO three times daily
- Ciclopirox topical solution Applied to affected nails daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension, premature coronary artery disease
- Mother: Chronic obstructive pulmonary disease (never smoker), osteoporosis
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Construction site supervisor.

PLAN:
The physician discusses using a pharmacotherapy that increases endogenous insulin release via direct agonism of the glucagon-like peptide-1 (GLP-1) receptors.

## Encounter 3: source 2021-03-15, practice 2024-05-13, outpatient, Nephrology

Chief complaint: Persistently elevated blood pressure despite current regimen

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Michael Thompson, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistently elevated blood pressure despite current regimen

HISTORY OF PRESENT ILLNESS:
He returns for nephrology follow-up with diabetes, neuropathy, and onychomycosis. Over the past three clinic visits, his blood pressure has ranged between 142–150 mm Hg systolic and 85–90 mm Hg diastolic.

PAST MEDICAL HISTORY:
Active Problem List:
- Onychomycosis (diagnosed 2020-01-15)
- Diabetic motor neuropathy (diagnosed 2020-04-15)
- Type 2 diabetes mellitus (diagnosed 2020-09-15)
- Type 2 Diabetes Mellitus (15-year duration, variable glycemic control)
- Hypertension
- Chronic Kidney Disease Stage 3
- Chronic Obstructive Pulmonary Disease (occupational dust exposure-related)
- Diabetic Peripheral Neuropathy (sensorimotor)
- Aspiration risk due to gastroesophageal reflux disease

He has a 6-year history of type 2 diabetes mellitus that is well controlled with metformin.

PAST SURGICAL HISTORY:
- Dental extraction with implant placement (recent)
- Debridement of right foot ulcer (recent)
- Primary suture repair of left lower extremity laceration (recent)

MEDICATIONS:
He takes metformin.

Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Tiotropium bromide 18 mcg inhaled daily
- Gabapentin 300 mg PO three times daily
- Ciclopirox topical solution Applied to affected nails daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension, premature coronary artery disease
- Mother: Chronic obstructive pulmonary disease (never smoker), osteoporosis
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
He does not smoke, drink alcohol, or use recreational drugs.

PHYSICAL EXAMINATION:
Funduscopic examination shows mild arteriolar narrowing but no hemorrhages or exudates. His physical examination is otherwise normal.

LABORATORY DATA:
Urinalysis shows trace protein. Laboratory results show a serum creatinine of 1.1 mg/dL, a hemoglobin A1c of 7.2% and a potassium of 4.3 mEq/L.

PLAN:
He is started on pharmacologic treatment to manage his blood pressure and protect his kidneys.

## Encounter 4: source 2021-09-15, practice 2024-11-26, ed, Emergency Medicine

Chief complaint: Severe pain, redness, and swelling at left leg laceration site sutured 5 days ago

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe pain, redness, and swelling at left leg laceration site sutured 5 days ago

HISTORY OF PRESENT ILLNESS:
This 56-year-old male with Type 2 Diabetes Mellitus and diabetic peripheral neuropathy (Neurology 2020-04-15) presents to the emergency department with a 2-day history of increasing pain, redness, and swelling in his left lower leg where he had a laceration sutured 5 days ago. He reports the pain as severe and out of proportion to the wound appearance.

PAST MEDICAL HISTORY:
Active Problem List:
- Onychomycosis (diagnosed 2020-01-15)
- Diabetic motor neuropathy (diagnosed 2020-04-15)
- Type 2 diabetes mellitus (diagnosed 2020-09-15)
- Chronic Kidney Disease with Volume Overload (diagnosed 2021-03-15)
- Type 2 Diabetes Mellitus (15-year duration, variable glycemic control)
- Hypertension
- Chronic Kidney Disease Stage 3
- Chronic Obstructive Pulmonary Disease (occupational dust exposure-related)
- Diabetic Peripheral Neuropathy (sensorimotor)
- Aspiration risk due to gastroesophageal reflux disease

PAST SURGICAL HISTORY:
- Dental extraction with implant placement (recent)
- Debridement of right foot ulcer (recent)
- Primary suture repair of left lower extremity laceration (recent)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Tiotropium bromide 18 mcg inhaled daily
- Gabapentin 300 mg PO three times daily
- Ciclopirox topical solution Applied to affected nails daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension, premature coronary artery disease
- Mother: Chronic obstructive pulmonary disease (never smoker), osteoporosis
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Construction site supervisor.

VITAL SIGNS:
His temperature is 38.9°C (102°F), heart rate is 110 bpm, and blood pressure is 135/85 mmHg.

PHYSICAL EXAMINATION:
On examination, the wound margins are indurated with a surrounding erythematous halo. There is decreased sensitivity to light touch around the wound edges. A cloudy-gray discharge is noted, and palpation of the surrounding tissue elicits a crackling sensation.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 18,000 /μL
- Hemoglobin: 13.2 g/dL
- Sodium: 135 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 1.1 mg/dL
- C-reactive protein: 24 mg/L (normal <3 mg/L)
- Blood cultures: Pending

## Encounter 5: source 2022-03-15, practice 2025-05-13, inpatient, Internal Medicine

Chief complaint: Fever, chills, and productive cough with foul sputum following recent dental implant placement

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Fever, chills, and productive cough with foul sputum following recent dental implant placement

HISTORY OF PRESENT ILLNESS:
This 57-year-old with Type 2 Diabetes and aspiration risk presents, reporting a recent dental procedure involving tooth extraction and subsequent placement of a dental implant 1 week ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Onychomycosis (diagnosed 2020-01-15)
- Diabetic motor neuropathy (diagnosed 2020-04-15)
- Type 2 diabetes mellitus (diagnosed 2020-09-15)
- Chronic Kidney Disease with Volume Overload (diagnosed 2021-03-15)
- Necrotizing fasciitis (diagnosed 2021-09-15)
- Type 2 Diabetes Mellitus (15-year duration, variable glycemic control)
- Hypertension
- Chronic Kidney Disease Stage 3
- Chronic Obstructive Pulmonary Disease (occupational dust exposure-related)
- Diabetic Peripheral Neuropathy (sensorimotor)
- Aspiration risk due to gastroesophageal reflux disease

He has a history of type 2 diabetes mellitus and chronic obstructive pulmonary disease (COPD).

PAST SURGICAL HISTORY:
- Dental extraction with implant placement (recent)
- Debridement of right foot ulcer (recent)
- Primary suture repair of left lower extremity laceration (recent)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Tiotropium bromide 18 mcg inhaled daily
- Gabapentin 300 mg PO three times daily
- Ciclopirox topical solution Applied to affected nails daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension, premature coronary artery disease
- Mother: Chronic obstructive pulmonary disease (never smoker), osteoporosis
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Construction site supervisor.

VITAL SIGNS:
On physical examination, his temperature is 38.7°C (101.7°F), blood pressure is 135/85 mmHg, pulse is 98 beats per minute, and respiratory rate is 22 breaths per minute.

PHYSICAL EXAMINATION:
Lung auscultation reveals decreased breath sounds and coarse crackles in the right lower lobe.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 14,000 /µL (normal: 4,500-11,000 /µL)
- Neutrophils: 80% (normal: 40-60%)
- Hemoglobin: 13.2 g/dL (normal: 13.8-17.2 g/dL)
- Blood glucose: 190 mg/dL (normal: 70-99 mg/dL)

IMAGING:
Chest X-ray: Infiltrate in the right lower lobe

## Encounter 6: source 2022-09-15, practice 2025-11-15, ed, Emergency Medicine

Chief complaint: Acute productive cough, fever, pleuritic chest pain, and shortness of breath

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. Lisa Wong, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute productive cough, fever, pleuritic chest pain, and shortness of breath

HISTORY OF PRESENT ILLNESS:
This 57-year-old male with chronic obstructive pulmonary disease and prior aspiration pneumonia presents to the emergency department with a 3-day history of productive cough, fever, pleuritic chest pain, and shortness of breath.

PAST MEDICAL HISTORY:
Active Problem List:
- Onychomycosis (diagnosed 2020-01-15)
- Diabetic motor neuropathy (diagnosed 2020-04-15)
- Type 2 diabetes mellitus (diagnosed 2020-09-15)
- Chronic Kidney Disease with Volume Overload (diagnosed 2021-03-15)
- Necrotizing fasciitis (diagnosed 2021-09-15)
- Recurrent aspiration pneumonia (diagnosed 2022-03-15)
- Type 2 Diabetes Mellitus (15-year duration, variable glycemic control)
- Hypertension
- Chronic Kidney Disease Stage 3
- Chronic Obstructive Pulmonary Disease (occupational dust exposure-related)
- Diabetic Peripheral Neuropathy (sensorimotor)
- Aspiration risk due to gastroesophageal reflux disease

His past medical history includes type 2 diabetes mellitus and chronic obstructive pulmonary disease.

PAST SURGICAL HISTORY:
- Dental extraction with implant placement (recent)
- Debridement of right foot ulcer (recent)
- Primary suture repair of left lower extremity laceration (recent)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Tiotropium bromide 18 mcg inhaled daily
- Gabapentin 300 mg PO three times daily
- Ciclopirox topical solution Applied to affected nails daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension, premature coronary artery disease
- Mother: Chronic obstructive pulmonary disease (never smoker), osteoporosis
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Construction site supervisor.

VITAL SIGNS:
On examination, temperature is 38.9°C (102°F), pulse is 110/min, respiratory rate is 26/min, and oxygen saturation is 90% on room air.

PHYSICAL EXAMINATION:
Auscultation reveals bronchial breath sounds and inspiratory crackles over the right lower lung field.

LABORATORY DATA:
Blood cultures grow Streptococcus pneumoniae .

IMAGING:
Chest x-ray demonstrates dense consolidation of the right lower lobe without pleural effusion. Within 24 hours, follow-up imaging shows involvement of the entire right lower lobe.

## Encounter 7: source 2023-03-15, practice 2026-05-12, inpatient, Infectious Disease

Chief complaint: Worsening lower back pain with fever over 5 days

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. Robert Johnson, MD
Department: Infectious Disease
Visit Type: inpatient

CHIEF COMPLAINT:
Worsening lower back pain with fever over 5 days

HISTORY OF PRESENT ILLNESS:
The patient has a history of necrotizing fasciitis and recurrent aspiration pneumonia. He presents with worsening lower back pain and fever over the past 5 days.

PAST MEDICAL HISTORY:
Active Problem List:
- Onychomycosis (diagnosed 2020-01-15)
- Diabetic motor neuropathy (diagnosed 2020-04-15)
- Type 2 diabetes mellitus (diagnosed 2020-09-15)
- Chronic Kidney Disease with Volume Overload (diagnosed 2021-03-15)
- Necrotizing fasciitis (diagnosed 2021-09-15)
- Recurrent aspiration pneumonia (diagnosed 2022-03-15)
- Pneumococcal pneumonia (diagnosed 2022-09-15)
- Type 2 Diabetes Mellitus (15-year duration, variable glycemic control)
- Hypertension
- Chronic Kidney Disease Stage 3
- Chronic Obstructive Pulmonary Disease (occupational dust exposure-related)
- Diabetic Peripheral Neuropathy (sensorimotor)
- Aspiration risk due to gastroesophageal reflux disease

He has a history of poorly controlled type 2 diabetes and was recently discharged after receiving treatment for a right foot ulcer.

PAST SURGICAL HISTORY:
- Dental extraction with implant placement (recent)
- Debridement of right foot ulcer (recent)
- Primary suture repair of left lower extremity laceration (recent)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Tiotropium bromide 18 mcg inhaled daily
- Gabapentin 300 mg PO three times daily
- Ciclopirox topical solution Applied to affected nails daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension, premature coronary artery disease
- Mother: Chronic obstructive pulmonary disease (never smoker), osteoporosis
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Construction site supervisor.

VITAL SIGNS:
Temperature is 38.9°C (102°F), blood pressure is 134/76 mm Hg, and pulse is 102/min

PHYSICAL EXAMINATION:
The lower back is tender to palpation over the lumbar spine, with limited range of motion due to pain. Neurologic exam is intact.

LABORATORY DATA:
Blood cultures drawn on admission grow Staph aureus in both aerobic and anaerobic bottles within 12 hours.

IMAGING:
MRI of the spine reveals vertebral osteomyelitis and adjacent epidural phlegmon.
