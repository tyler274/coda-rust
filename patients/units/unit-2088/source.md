# synthetic_hospital patient 2088

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Commercial insurance (Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-02-12 (Wed) | outpatient | Pulmonology |
| 1 | 2020-07-15 | 2025-08-04 (Mon) | outpatient | Internal Medicine |
| 2 | 2021-01-15 | 2026-02-03 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial insurance (Blue Cross Blue Shield)",
 "occupation": "Building restoration contractor",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Father: Type 2 Diabetes Mellitus (diagnosed age 50)",
  "Mother: Hypertension",
  "Paternal uncle: Lung cancer"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker with 30 pack-year history (approximately 1 pack per day since age 15)",
 "home_medications": [
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Glipizide"
  }
 ],
 "surgical_history": [
  "Tonsillectomy (age 12)"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus (poorly controlled, diagnosed within past year)",
  "Tobacco use disorder"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Opportunistic pulmonary infection requiring diagnostic sampling", "Cellulitis of left lower limb", "Histoplasmosis"]
Comorbidities: ["Poorly controlled diabetes mellitus", "Tobacco use disorder"]

## Encounter 0: source 2020-01-15, practice 2025-02-12, outpatient, Pulmonology

Chief complaint: 3-month history of nonproductive cough, progressive fatigue, and unintentional weight loss with night sweats

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of nonproductive cough, progressive fatigue, and unintentional weight loss with night sweats

HISTORY OF PRESENT ILLNESS:
He also reports intermittent low-grade fevers and mild night sweats. The patient lives in Missouri and works as a building restoration contractor, frequently entering old, abandoned structures. He recalls cleaning several attics heavily contaminated with bat droppings 4 months ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus (poorly controlled, diagnosed within past year)
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Glipizide 10 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus (diagnosed age 50)
- Mother: Hypertension
- Paternal uncle: Lung cancer

SOCIAL HISTORY:
He has smoked one pack of cigarettes daily for 20 years

PHYSICAL EXAMINATION:
Physical examination shows mild pallor and scattered crackles over both lung fields

LABORATORY DATA:
HIV testing is negative

IMAGING:
Chest x-ray reveals multiple small, nodular opacities throughout both lungs with hilar lymphadenopathy

PATHOLOGY:
Staining highlights small, ovoid organisms within macrophages

OTHER STUDIES:
Bronchoscopy with transbronchial biopsy is performed, and histopathologic examination is shown

## Encounter 1: source 2020-07-15, practice 2025-08-04, outpatient, Internal Medicine

Chief complaint: left lower leg pain and redness with low-grade fever

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Left lower leg pain and redness with low-grade fever

HISTORY OF PRESENT ILLNESS:
Patient is a 45-year-old with poorly controlled T2DM, tobacco use disorder, and prior pulmonary symptoms who presents to the clinic with a 3-day history of left lower leg pain and redness. He denies any recent trauma or insect bites. He reports a low-grade fever and chills at home.

PAST MEDICAL HISTORY:
Active Problem List:
- Histoplasmosis (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus (poorly controlled, diagnosed within past year)
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Glipizide 10 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus (diagnosed age 50)
- Mother: Hypertension
- Paternal uncle: Lung cancer

SOCIAL HISTORY:
Smoking: Current smoker with 30 pack-year history (approximately 1 pack per day since age 15). Alcohol: Social (1-2 drinks per week). Occupation: Building restoration contractor.

VITAL SIGNS:
The patient has a temperature of 38.2°C (100.8°F), a heart rate of 88 beats per minute, and a blood pressure of 135/85 mmHg.

PHYSICAL EXAMINATION:
On examination, his left lower leg is erythematous, warm, and tender to touch with poorly demarcated borders extending from the ankle to the mid-calf. There is no purulent drainage or fluctuance, and the leg is not significantly swollen.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 12,000 /μL
- Hemoglobin: 14 g/dL
- Platelet count: 250,000 /μL
- C-reactive protein: 24 mg/L (normal <10 mg/L)
- Blood cultures: No growth (collected 48 hours ago)

ASSESSMENT AND PLAN:
Based on the clinical presentation and findings,

## Encounter 2: source 2021-01-15, practice 2026-02-03, ed, Emergency Medicine

Chief complaint: acute onset increasing shortness of breath, productive cough with yellow sputum, and fever

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Emily Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset increasing shortness of breath, productive cough with yellow sputum, and fever

HISTORY OF PRESENT ILLNESS:
The patient has poorly controlled Type 2 Diabetes and Histoplasmosis. He returns to the emergency department with a 2-day history of increasing shortness of breath, productive cough with yellow sputum, and fever.

PAST MEDICAL HISTORY:
Active Problem List:
- Histoplasmosis (diagnosed 2020-01-15)
- Cellulitis of left lower limb (diagnosed 2020-07-15)
- Type 2 Diabetes Mellitus (poorly controlled, diagnosed within past year)
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Glipizide 10 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus (diagnosed age 50)
- Mother: Hypertension
- Paternal uncle: Lung cancer

SOCIAL HISTORY:
He has a 30-pack-year smoking history and reports no recent travel or sick contacts.

VITAL SIGNS:
On examination, he is tachypneic with a respiratory rate of 28 breaths per minute, febrile with a temperature of 38.7°C (101.7°F), and has a pulse of 110 beats per minute. His oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
Lung auscultation reveals crackles in the right lower lobe.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 18,000/mm³ (normal 4,500-11,000)
- Hemoglobin: 14 g/dL (normal 13.8-17.2)
- Platelets: 250,000/mm³ (normal 150,000-450,000)
- Blood urea nitrogen: 28 mg/dL (normal 7-20)
- Creatinine: 1.2 mg/dL (normal 0.6-1.3)
- Glucose: 300 mg/dL (normal 70-99)

IMAGING:
A chest X-ray shows a right lower lobe consolidation.

ASSESSMENT AND PLAN:
The patient is diagnosed with community-acquired pneumonia and diabetic ketoacidosis is ruled out.

He is started on appropriate intravenous antibiotics and insulin therapy. After 24 hours of treatment, the patient insists on leaving the hospital against medical advice because he is concerned about the cost of hospitalization. The provider explains the risks of leaving the hospital without completing treatment, including the potential for worsening infection and respiratory failure. The patient remains adamant about leaving.
