# synthetic_hospital patient 2550

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex M, Non-Hispanic White, Private employer-sponsored.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-12-05 (Thu) | outpatient | Nephrology |
| 1 | 2020-09-15 | 2025-08-08 (Fri) | ed | Emergency Medicine |
| 2 | 2021-05-15 | 2026-04-01 (Wed) | icu | Nephrology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private employer-sponsored",
 "occupation": "Accountant (previously graduate student)",
 "alcohol_use": "Former social drinker (quit after medical complications)",
 "family_history": [
  "Father with autosomal dominant polycystic kidney disease",
  "Paternal uncle with ruptured cerebral aneurysm",
  "Mother with hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Chronic Kidney Disease",
  "Autosomal Dominant Polycystic Kidney Disease"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Pit viper (Crotaline) envenomation", "Goodpasture syndrome", "Ruptured cerebral aneurysm"]
Comorbidities: ["Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2024-12-05, outpatient, Nephrology

Chief complaint: Elevated blood pressure readings noted at a pharmacy visit; mild headaches over the past few months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Patel, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Elevated blood pressure readings noted at a pharmacy visit; mild headaches over the past few months

HISTORY OF PRESENT ILLNESS:
He says he generally feels well but has been experiencing mild headaches over the past few months. He denies chest pain, vision changes, or shortness of breath. His appetite is normal and he has not noticed changes in urination. He does report that his abdomen sometimes feels “full” on both sides.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Chronic Kidney Disease
- Autosomal Dominant Polycystic Kidney Disease

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with autosomal dominant polycystic kidney disease
- Paternal uncle with ruptured cerebral aneurysm
- Mother with hypertension

SOCIAL HISTORY:
He works as an accountant and does not smoke or drink alcohol.

VITAL SIGNS:
On physical examination, his blood pressure is 165/102 mmHg and pulse is 78/min.

PHYSICAL EXAMINATION:
Cardiovascular and lung examinations are normal. Abdominal examination reveals bilateral, non-tender masses in the flank regions.

LABORATORY DATA:
Laboratory evaluation shows normal serum creatinine,

IMAGING:
A renal ultrasound is performed demonstrating enlarged kidneys with multiple cystic structures bilaterally.

## Encounter 1: source 2020-09-15, practice 2025-08-08, ed, Emergency Medicine

Chief complaint: Bitten on left lower leg by snake while hiking in wooded area of North Carolina

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Bitten on left lower leg by snake while hiking in wooded area of North Carolina

HISTORY OF PRESENT ILLNESS:
This 32-year-old male with hypertension, chronic kidney disease, and autosomal dominant polycystic kidney disease, evaluated in January 2020 for elevated blood pressure and headaches, presents to the emergency department 2 hours after being bitten on his left lower leg by a snake while hiking in a wooded area of North Carolina. He did not see the snake clearly but reports it was not brightly colored. Eight hours later, he complains of increasing pain and swelling at the bite site.

PAST MEDICAL HISTORY:
Active Problem List:
- Ruptured cerebral aneurysm (diagnosed 2020-01-15)
- Hypertension
- Chronic Kidney Disease
- Autosomal Dominant Polycystic Kidney Disease

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes lisinopril

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with autosomal dominant polycystic kidney disease
- Paternal uncle with ruptured cerebral aneurysm
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Never. Alcohol: Former social drinker (quit after medical complications). Occupation: Accountant (previously graduate student).

REVIEW OF SYSTEMS:
He denies any difficulty breathing, changes in vision, or gastrointestinal symptoms.

VITAL SIGNS:
His vital signs are stable, and he is afebrile.

PHYSICAL EXAMINATION:
On examination, the patient has two small puncture wounds on his medial malleolus with minimal surrounding erythema and no evidence of necrosis.

LABORATORY DATA:
Initial laboratory studies show:

- White blood cell count: 7,500/mm³
- Hemoglobin: 14.2 g/dL
- Platelet count: 250,000/mm³
- Prothrombin time (PT): 12 seconds (control 11-13.5 seconds)
- Partial thromboplastin time (PTT): 30 seconds (control 25-35 seconds)
- Fibrinogen: 400 mg/dL (normal range: 200-400 mg/dL)

Repeat laboratory studies now show:

- White blood cell count: 8,200/mm³
- Hemoglobin: 13.8 g/dL
- Platelet count: 195,000/mm³
- Prothrombin time (PT): 14 seconds
- Partial thromboplastin time (PTT): 32 seconds
- Fibrinogen: 300 mg/dL

PLAN:
The patient is observed closely with repeat wound examinations and laboratory evaluations.

## Encounter 2: source 2021-05-15, practice 2026-04-01, icu, Nephrology

Chief complaint: Coughing up blood for the past two days and dark-colored urine noticed over the past week

ENCOUNTER NOTE
Date: 2021-05-15
Provider: Dr. Kenji Tanaka, MD
Department: Nephrology
Visit Type: icu

CHIEF COMPLAINT:
Coughing up blood for the past two days and dark-colored urine noticed over the past week

HISTORY OF PRESENT ILLNESS:
The patient is a 33-year-old male with autosomal dominant polycystic kidney disease, chronic kidney disease, and hypertension who has previously presented with mild headaches and elevated blood pressure, and later with a lower extremity snakebite. He also reports dark-colored urine noticed over the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Ruptured cerebral aneurysm (diagnosed 2020-01-15)
- Pit viper (Crotaline) envenomation (diagnosed 2020-09-15)
- Hypertension
- Chronic Kidney Disease
- Autosomal Dominant Polycystic Kidney Disease

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with autosomal dominant polycystic kidney disease
- Paternal uncle with ruptured cerebral aneurysm
- Mother with hypertension

SOCIAL HISTORY:
He is a graduate student. He does not smoke and drinks alcohol socially.

REVIEW OF SYSTEMS:
He denies fever, chest pain, or recent infections.

VITAL SIGNS:
On examination, temperature is 36.8°C (98.2°F), blood pressure is 142/88 mmHg, pulse is 80/min, and respirations are 16/min.

PHYSICAL EXAMINATION:
Lung examination reveals scattered crackles bilaterally.

LABORATORY DATA:
Urinalysis shows hematuria with dysmorphic red blood cells and mild proteinuria.

IMAGING:
Chest x-ray demonstrates patchy bilateral infiltrates.
