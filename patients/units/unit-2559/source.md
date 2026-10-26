# synthetic_hospital patient 2559

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-10-28 (Mon) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2025-04-19 (Sat) | icu | Neurology |
| 2 | 2021-07-15 | 2026-05-05 (Tue) | icu | Infectious Disease |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Construction laborer",
 "alcohol_use": "Social, 1-2 drinks per month",
 "family_history": [
  "Father with coronary artery disease",
  "Mother with hypertension",
  "Brother with major depressive disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 1 pack per day for 12 years",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [
  "Human Immunodeficiency Virus infection",
  "Opioid use disorder"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Tuberculous meningitis", "Wound botulism", "Septic arthritis"]
Comorbidities: ["Intravenous Drug Use (Substance Use Disorder)"]

## Encounter 0: source 2020-01-15, practice 2024-10-28, ed, Emergency Medicine

Chief complaint: 2-week history of fever, malaise, and multiple joint pains beginning in the knees and spreading to wrists and elbows.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-week history of fever, malaise, and multiple joint pains beginning in the knees and spreading to wrists and elbows.

HISTORY OF PRESENT ILLNESS:
He reports the pain began in his knees and has since spread to his wrists and elbows. He denies any recent trauma or travel.

PAST MEDICAL HISTORY:
Active Problem List:
- Human Immunodeficiency Virus infection
- Opioid use disorder

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with coronary artery disease
- Mother with hypertension
- Brother with major depressive disorder

SOCIAL HISTORY:
With a history of intravenous drug use

VITAL SIGNS:
On examination, his temperature is 38.9°C (102°F), heart rate is 110 bpm, and blood pressure is 130/85 mmHg.

PHYSICAL EXAMINATION:
Musculoskeletal examination reveals warmth, erythema, and marked tenderness with limited range of motion in the affected joints. Cardiac auscultation identifies a new regurgitant murmur.

LABORATORY DATA:
Laboratory studies are as follows:

- White blood cell count: 15,000/mm3
- Erythrocyte sedimentation rate (ESR): 85 mm/hr
- C-reactive protein (CRP): 120 mg/L
- Blood cultures: Pending
- Synovial fluid analysis from the knee:
- Leukocyte count: 60,000/mm3
- Neutrophil percentage: 90%
- Gram stain: No organisms seen
- Culture: Pending

## Encounter 1: source 2020-07-15, practice 2025-04-19, icu, Neurology

Chief complaint: 2-day history of double vision, drooping eyelids, difficulty swallowing, and slurred speech.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Neurology
Visit Type: icu

CHIEF COMPLAINT:
2-day history of double vision, drooping eyelids, difficulty swallowing, and slurred speech.

HISTORY OF PRESENT ILLNESS:
Patient has HIV, OUD, and prior septic arthritis. He reports that the symptoms started with drooping eyelids and have progressed to include slurred speech.

PAST MEDICAL HISTORY:
Active Problem List:
- Septic arthritis (diagnosed 2020-01-15)
- Human Immunodeficiency Virus infection
- Opioid use disorder

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with coronary artery disease
- Mother with hypertension
- Brother with major depressive disorder

SOCIAL HISTORY:
He has a history of intravenous drug use and admits to injecting heroin using a non-sterile needle 3 days ago.

VITAL SIGNS:
On examination, the patient is afebrile, blood pressure is 130/85 mmHg, pulse is 70/min, and respiratory rate is 16/min.

PHYSICAL EXAMINATION:
Neurological examination reveals bilateral ptosis, fixed dilated pupils, and symmetric descending weakness. Sensation is intact, and deep tendon reflexes are normal. The patient has no rash or signs of injection site infection, but a small puncture wound is noted on the right forearm.

LABORATORY DATA:
Laboratory studies show:

- White blood cell count: 12,000/mm³
- Hemoglobin: 14.2 g/dL
- Platelet count: 250,000/mm³
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 1.0 mg/dL
- Blood glucose: 98 mg/dL

## Encounter 2: source 2021-07-15, practice 2026-05-05, icu, Infectious Disease

Chief complaint: 2-week history of progressive headache, fever, vomiting, photophobia, and neck stiffness.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Priya Patel, MD
Department: Infectious Disease
Visit Type: icu

CHIEF COMPLAINT:
2-week history of progressive headache, fever, vomiting, photophobia, and neck stiffness.

HISTORY OF PRESENT ILLNESS:
The patient is a 33-year-old male with HIV and opioid use disorder previously hospitalized for joint involvement and neurological deficits. He reports associated photophobia and neck stiffness.

PAST MEDICAL HISTORY:
Active Problem List:
- Septic arthritis (diagnosed 2020-01-15)
- Wound botulism (diagnosed 2020-07-15)
- Human Immunodeficiency Virus infection
- Opioid use disorder

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with coronary artery disease
- Mother with hypertension
- Brother with major depressive disorder

SOCIAL HISTORY:
History of IV drug use

VITAL SIGNS:
Temperature is 38.6°C, heart rate is 102 bpm

PHYSICAL EXAMINATION:
Lethargic but oriented to time, place, and person. neck stiffness with positive Brudzinski's sign is noted. Fundoscopic examination reveals no papilledema. Cranial nerves are intact, and there are no focal neurological deficits.

LABORATORY DATA:
Lumbar puncture is performed, and the cerebrospinal fluid (CSF) analysis reveals:

- WBC: 150 cells/mm^3 (80% lymphocytes)
- Protein: 100 mg/dL
- Glucose: 20 mg/dL (serum glucose 90 mg/dL)
- Acid-fast bacilli (AFB) smear: Pending
- CSF Gram stain: No organisms seen

IMAGING:
A CT scan of the head shows mild hydrocephalus and basilar meningeal enhancement. There is no evidence of stroke.
