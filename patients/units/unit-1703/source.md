# synthetic_hospital patient 1703

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Southeast Asian (Vietnamese), Medicaid Managed Care.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-07-20 (Wed) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-04-15 | 2022-10-28 (Fri) | outpatient | Internal Medicine |
| 2 | 2020-10-15 | 2023-04-24 (Mon) | outpatient | Infectious Diseases |
| 3 | 2021-07-15 | 2024-01-16 (Tue) | outpatient | Hematology & Oncology |
| 4 | 2021-10-15 | 2024-04-29 (Mon) | outpatient | Psychiatry |
| 5 | 2022-07-15 | 2025-01-18 (Sat) | ed | Emergency Medicine |
| 6 | 2023-01-15 | 2025-07-29 (Tue) | ed | Emergency Medicine |
| 7 | 2024-01-15 | 2026-07-30 (Thu) | outpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid Managed Care",
 "occupation": "Production assembler",
 "alcohol_use": "None",
 "family_history": [
  "Type 2 diabetes mellitus in mother",
  "Systemic lupus erythematosus in maternal aunt",
  "Schizophrenia in paternal uncle",
  "Recurrent early pregnancy loss in sister"
 ],
 "race_ethnicity": "Southeast Asian (Vietnamese)",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "200 mg orally daily",
   "name": "Hydroxychloroquine"
  },
  {
   "dose": "2 mg orally daily",
   "name": "Risperidone"
  },
  {
   "dose": "1000 mg orally twice daily",
   "name": "Metformin"
  },
  {
   "dose": "1 tablet orally daily",
   "name": "Prenatal vitamin"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Systemic lupus erythematosus (SLE)",
  "Type 2 diabetes mellitus",
  "Schizophrenia",
  "Bulimia nervosa",
  "Antiphospholipid syndrome"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Lupus nephritis flare", "Glucocorticoid-induced neuropsychiatric symptoms", "Hypochloremic metabolic alkalosis secondary to vomiting with hypovolemia", "Glomerulonephritis", "Iatrogenic immunosuppression", "Antiphospholipid syndrome", "Independent double-check verification procedure", "Systemic Lupus Erythematosus with Central Nervous System Involvement (Neuropsychiatric SLE)"]
Comorbidities: ["Long-term glucocorticoid therapy", "Acute kidney injury", "Dehydration", "Systemic lupus erythematosus"]

## Encounter 0: source 2020-01-15, practice 2022-07-20, outpatient, Obstetrics & Gynecology

Chief complaint: History of two spontaneous miscarriages in the past year

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
History of two spontaneous miscarriages in the past year

HISTORY OF PRESENT ILLNESS:
She reports that both pregnancies were lost before the 10th week of gestation.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (SLE)
- Type 2 diabetes mellitus
- Schizophrenia
- Bulimia nervosa
- Antiphospholipid syndrome

Her past medical history is significant for systemic lupus erythematosus (SLE), which is currently well-controlled on hydroxychloroquine.

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg orally daily
- Risperidone 2 mg orally daily
- Metformin 1000 mg orally twice daily
- Prenatal vitamin 1 tablet orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus in mother
- Systemic lupus erythematosus in maternal aunt
- Schizophrenia in paternal uncle
- Recurrent early pregnancy loss in sister

SOCIAL HISTORY:
She denies any history of drug use, smoking, or alcohol consumption.

VITAL SIGNS:
Her blood pressure is 120/78 mmHg, heart rate is 72 beats per minute

PHYSICAL EXAMINATION:
On examination, she appears well, with no acute distress. The rest of the physical examination is unremarkable.

LABORATORY DATA:
Complete blood count: Normal
Prothrombin time (PT): 12 seconds (control: 11-13.5 seconds)
Activated partial thromboplastin time (aPTT): 55 seconds (control: 25-35 seconds)
aPTT after mixing with normal plasma: 50 seconds
Antinuclear antibodies (ANA): Positive
Anti-double-stranded DNA (anti-dsDNA): Negative
Anti-cardiolipin antibodies: Positive
Lupus anticoagulant: Positive
Beta-2 glycoprotein I antibodies: Not performed

## Encounter 1: source 2020-04-15, practice 2022-10-28, outpatient, Internal Medicine

Chief complaint: 2-day history of dysuria, increased urinary frequency, and lower abdominal pain with new lower leg rash

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
2-day history of dysuria, increased urinary frequency, and lower abdominal pain with new lower leg rash

HISTORY OF PRESENT ILLNESS:
This 32-year-old female with systemic lupus erythematosus and antiphospholipid syndrome, recently diagnosed after two miscarriages, presents for care. She reports using cranberry supplements to self-treat what she believes is a "bladder infection," based on similar symptoms she had in the past. She also reports a new rash on her lower legs.

PAST MEDICAL HISTORY:
Active Problem List:
- Antiphospholipid syndrome (diagnosed 2020-01-15)
- Systemic lupus erythematosus (SLE)
- Type 2 diabetes mellitus
- Schizophrenia
- Bulimia nervosa

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg orally daily
- Risperidone 2 mg orally daily
- Metformin 1000 mg orally twice daily
- Prenatal vitamin 1 tablet orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus in mother
- Systemic lupus erythematosus in maternal aunt
- Schizophrenia in paternal uncle
- Recurrent early pregnancy loss in sister

SOCIAL HISTORY:
She is a recent immigrant from Southeast Asia and expresses concern about taking medications that are not natural or familiar to her.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
Physical examination reveals suprapubic tenderness and a palpable, non-tender mass in the right lower quadrant. A non-blanching, palpable purpuric rash is noted on both lower extremities.

LABORATORY DATA:
Urinalysis shows pyuria and hematuria. Blood tests reveal:
- Serum creatinine: 2.1 mg/dL (elevated)
- Blood urea nitrogen (BUN): 36 mg/dL (elevated)
- C-reactive protein (CRP): 8 mg/L (elevated)
- Hemoglobin A1c: 9.2% (elevated)

ASSESSMENT AND PLAN:
Given the patient's cultural background and beliefs,

## Encounter 2: source 2020-10-15, practice 2023-04-24, outpatient, Infectious Diseases

Chief complaint: 3-week history of cough, night sweats, and low-grade fever

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Priya Patel, MD
Department: Infectious Diseases
Visit Type: outpatient

CHIEF COMPLAINT:
3-week history of cough, night sweats, and low-grade fever

HISTORY OF PRESENT ILLNESS:
This 32-year-old female with systemic lupus erythematosus, antiphospholipid syndrome, a history of spontaneous miscarriages, and previous glomerulonephritis presents for evaluation. She denies any recent travel or sick contacts.

PAST MEDICAL HISTORY:
Active Problem List:
- Antiphospholipid syndrome (diagnosed 2020-01-15)
- Glomerulonephritis (diagnosed 2020-04-15)
- Systemic lupus erythematosus (SLE)
- Type 2 diabetes mellitus
- Schizophrenia
- Bulimia nervosa

MEDICATIONS:
She has been on prednisone 20 mg daily for the past 6 months for SLE flare-ups.

Home Medications:
- Hydroxychloroquine 200 mg orally daily
- Risperidone 2 mg orally daily
- Metformin 1000 mg orally twice daily
- Prenatal vitamin 1 tablet orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus in mother
- Systemic lupus erythematosus in maternal aunt
- Schizophrenia in paternal uncle
- Recurrent early pregnancy loss in sister

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Production assembler.

VITAL SIGNS:
On physical examination, her temperature is 37.8°C, heart rate is 88/min, respiratory rate is 18/min, and oxygen saturation is 97% on room air.

PHYSICAL EXAMINATION:
Lung auscultation reveals bilateral crackles at the lung bases.

LABORATORY DATA:
Laboratory tests are ordered, including a sputum acid-fast bacillus (AFB) smear and culture, and an interferon-γ release assay (IGRA) for tuberculosis (TB). The AFB smear is negative, and the IGRA returns with an indeterminate result.

IMAGING:
A chest X-ray shows bilateral lower lobe infiltrates.

## Encounter 3: source 2021-07-15, practice 2024-01-16, outpatient, Hematology & Oncology

Chief complaint: 2-week history of increasing fatigue, easy bruising, and sore throat

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James O'Brien, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
2-week history of increasing fatigue, easy bruising, and sore throat

HISTORY OF PRESENT ILLNESS:
The patient with systemic lupus erythematosus, antiphospholipid syndrome, and history of glomerulonephritis and iatrogenic immunosuppression returns for evaluation. She reports no recent infections or changes in medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Antiphospholipid syndrome (diagnosed 2020-01-15)
- Glomerulonephritis (diagnosed 2020-04-15)
- Iatrogenic immunosuppression (diagnosed 2020-10-15)
- Systemic lupus erythematosus (SLE)
- Type 2 diabetes mellitus
- Schizophrenia
- Bulimia nervosa

MEDICATIONS:
She takes hydroxychloroquine and low-dose prednisone.

Home Medications:
- Risperidone 2 mg orally daily
- Metformin 1000 mg orally twice daily
- Prenatal vitamin 1 tablet orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus in mother
- Systemic lupus erythematosus in maternal aunt
- Schizophrenia in paternal uncle
- Recurrent early pregnancy loss in sister

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Production assembler.

PHYSICAL EXAMINATION:
On examination, she is pale and has scattered petechiae on her lower extremities. There is no lymphadenopathy or hepatosplenomegaly. Her joint examination is unremarkable.

LABORATORY DATA:
Hemoglobin: 8.2 g/dL (Normal: 12-16 g/dL)
White blood cell count: 2.1 x 10^9/L (Normal: 4-11 x 10^9/L)
Platelets: 50 x 10^9/L (Normal: 150-450 x 10^9/L)
Reticulocyte count: 1% (Normal: 0.5-2.5%)
Direct Coombs test: Positive
Antinuclear antibodies (ANA): Positive
Anti-double-stranded DNA antibodies: Positive
Complement levels (C3, C4): Low

## Encounter 4: source 2021-10-15, practice 2024-04-29, outpatient, Psychiatry

Chief complaint: 2-week history of mood swings, decreased need for sleep, and increased talkativeness

ENCOUNTER NOTE
Date: 2021-10-15
Provider: Dr. Aisha Johnson, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
2-week history of mood swings, decreased need for sleep, and increased talkativeness

HISTORY OF PRESENT ILLNESS:
The patient returns for outpatient psychiatric care three months following her prior evaluation for systemic lupus erythematosus with central nervous system involvement, presenting with a 2-week history of mood swings, decreased need for sleep, and increased talkativeness. She reports feeling "on top of the world" and has been more socially outgoing than usual.

PAST MEDICAL HISTORY:
Active Problem List:
- Antiphospholipid syndrome (diagnosed 2020-01-15)
- Glomerulonephritis (diagnosed 2020-04-15)
- Iatrogenic immunosuppression (diagnosed 2020-10-15)
- Systemic Lupus Erythematosus with Central Nervous System Involvement (Neuropsychiatric SLE) (diagnosed 2021-07-15)
- Systemic lupus erythematosus (SLE)
- Type 2 diabetes mellitus
- Schizophrenia
- Bulimia nervosa

She has no past psychiatric history but was diagnosed with systemic lupus erythematosus (SLE) 6 months ago.

MEDICATIONS:
She was started on prednisone 40 mg daily 3 weeks ago after a lupus flare involving her kidneys.

Home Medications:
- Hydroxychloroquine 200 mg orally daily
- Risperidone 2 mg orally daily
- Metformin 1000 mg orally twice daily
- Prenatal vitamin 1 tablet orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus in mother
- Systemic lupus erythematosus in maternal aunt
- Schizophrenia in paternal uncle
- Recurrent early pregnancy loss in sister

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Production assembler.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
Her physical examination is unremarkable,

LABORATORY DATA:
Laboratory studies show normal thyroid-stimulating hormone (TSH) and electrolytes. Urine toxicology screen is negative.

## Encounter 5: source 2022-07-15, practice 2025-01-18, ed, Emergency Medicine

Chief complaint: Altered mental status with refusal to eat or drink due to paranoid delusions

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Altered mental status with refusal to eat or drink due to paranoid delusions

HISTORY OF PRESENT ILLNESS:
Patient is a 34-year-old female with schizophrenia, neuropsychiatric systemic lupus erythematosus, and antiphospholipid syndrome presenting to the emergency department. She is found to be minimally responsive and dehydrated. Her family reports that she has been refusing to eat or drink for the past 3 days due to a belief that her food and water are being poisoned.

PAST MEDICAL HISTORY:
Active Problem List:
- Antiphospholipid syndrome (diagnosed 2020-01-15)
- Glomerulonephritis (diagnosed 2020-04-15)
- Iatrogenic immunosuppression (diagnosed 2020-10-15)
- Systemic Lupus Erythematosus with Central Nervous System Involvement (Neuropsychiatric SLE) (diagnosed 2021-07-15)
- Glucocorticoid-induced neuropsychiatric symptoms (diagnosed 2021-10-15)
- Systemic lupus erythematosus (SLE)
- Type 2 diabetes mellitus
- Schizophrenia
- Bulimia nervosa

Her medical history is significant for multiple hospitalizations for psychotic episodes.

MEDICATIONS:
She takes she has been stable on risperidone for the past year.

Home Medications:
- Hydroxychloroquine 200 mg orally daily
- Metformin 1000 mg orally twice daily
- Prenatal vitamin 1 tablet orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus in mother
- Systemic lupus erythematosus in maternal aunt
- Schizophrenia in paternal uncle
- Recurrent early pregnancy loss in sister

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Production assembler.

VITAL SIGNS:
Tachycardic with a heart rate of 110 bpm, blood pressure is 90/60 mmHg, and she is febrile with a temperature of 38.5°C (101.3°F)

PHYSICAL EXAMINATION:
She is oriented to person but not to place or time.

LABORATORY DATA:
- Sodium: 150 mEq/L (Normal: 135-145 mEq/L)
- Potassium: 3.5 mEq/L (Normal: 3.5-5.1 mEq/L)
- Chloride: 110 mEq/L (Normal: 98-106 mEq/L)
- Bicarbonate: 24 mEq/L (Normal: 22-29 mEq/L)
- Blood urea nitrogen (BUN): 40 mg/dL (Normal: 7-20 mg/dL)
- Creatinine: 1.8 mg/dL (Normal: 0.6-1.1 mg/dL)
- Glucose: 90 mg/dL (Normal: 70-99 mg/dL)

ASSESSMENT AND PLAN:
Suspected dehydration and acute kidney injury

The patient is started on intravenous fluids. Once she is medically stabilized, she adamantly refuses further treatment, insisting that the hospital staff is part of the conspiracy to poison her. She is able to articulate her understanding of her medical condition and the potential consequences of refusing treatment.

## Encounter 6: source 2023-01-15, practice 2025-07-29, ed, Emergency Medicine

Chief complaint: 3-day history of severe vomiting unable to tolerate food or liquids

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Maria Garcia, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
3-day history of severe vomiting unable to tolerate food or liquids

HISTORY OF PRESENT ILLNESS:
The patient with SLE and bulimia nervosa returns with vomiting. She reports that she has been unable to keep any food or liquids down. She denies diarrhea, fever, or recent travel.

PAST MEDICAL HISTORY:
Active Problem List:
- Antiphospholipid syndrome (diagnosed 2020-01-15)
- Glomerulonephritis (diagnosed 2020-04-15)
- Iatrogenic immunosuppression (diagnosed 2020-10-15)
- Systemic Lupus Erythematosus with Central Nervous System Involvement (Neuropsychiatric SLE) (diagnosed 2021-07-15)
- Glucocorticoid-induced neuropsychiatric symptoms (diagnosed 2021-10-15)
- Independent double-check verification procedure (diagnosed 2022-07-15)
- Systemic lupus erythematosus (SLE)
- Type 2 diabetes mellitus
- Schizophrenia
- Bulimia nervosa

Her past medical history is significant for bulimia nervosa, which she states has been more difficult to control in recent months.

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg orally daily
- Risperidone 2 mg orally daily
- Metformin 1000 mg orally twice daily
- Prenatal vitamin 1 tablet orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus in mother
- Systemic lupus erythematosus in maternal aunt
- Schizophrenia in paternal uncle
- Recurrent early pregnancy loss in sister

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Production assembler.

VITAL SIGNS:
Her blood pressure is 95/60 mmHg, heart rate is 110 beats per minute

PHYSICAL EXAMINATION:
On examination, she appears dehydrated.

LABORATORY DATA:
Laboratory studies are obtained and reveal the following:

- Sodium (Na+): 130 mEq/L
- Potassium (K+): 2.9 mEq/L
- Chloride (Cl-): 88 mEq/L
- Bicarbonate (HCO3-): 32 mEq/L
- Blood urea nitrogen (BUN): 28 mg/dL
- Creatinine: 0.9 mg/dL
- Urine sodium (uNa+): 10 mEq/L
- Urine chloride (uCl-): 5 mEq/L

## Encounter 7: source 2024-01-15, practice 2026-07-30, outpatient, Obstetrics & Gynecology

Chief complaint: 2-week history of increasing facial puffiness, joint pain, and malar rash during pregnancy

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Emily Nguyen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
2-week history of increasing facial puffiness, joint pain, and malar rash during pregnancy

HISTORY OF PRESENT ILLNESS:
The patient with systemic lupus erythematosus and antiphospholipid syndrome returns to the obstetrics clinic. She presents to the clinic with a 2-week history of increasing facial puffiness, joint pain, and rash over her cheeks. She denies any recent infections, travel, or new medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Antiphospholipid syndrome (diagnosed 2020-01-15)
- Glomerulonephritis (diagnosed 2020-04-15)
- Iatrogenic immunosuppression (diagnosed 2020-10-15)
- Systemic Lupus Erythematosus with Central Nervous System Involvement (Neuropsychiatric SLE) (diagnosed 2021-07-15)
- Glucocorticoid-induced neuropsychiatric symptoms (diagnosed 2021-10-15)
- Independent double-check verification procedure (diagnosed 2022-07-15)
- Hypochloremic metabolic alkalosis secondary to vomiting with hypovolemia (diagnosed 2023-01-15)
- Systemic lupus erythematosus (SLE)
- Type 2 diabetes mellitus
- Schizophrenia
- Bulimia nervosa

She has a past medical history of systemic lupus erythematosus (SLE) but has been in remission for the past year.

MEDICATIONS:
Her medications include hydroxychloroquine and prenatal vitamins.

Home Medications:
- Risperidone 2 mg orally daily
- Metformin 1000 mg orally twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus in mother
- Systemic lupus erythematosus in maternal aunt
- Schizophrenia in paternal uncle
- Recurrent early pregnancy loss in sister

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Production assembler.

VITAL SIGNS:
Her blood pressure is 150/95 mmHg

PHYSICAL EXAMINATION:
On physical examination, her blood pressure is 150/95 mmHg, and she has 2+ pitting edema in her lower extremities. A malar rash is noted across her cheeks and the bridge of her nose. Joint examination reveals tenderness and swelling in her wrists and knees.

LABORATORY DATA:
Urinalysis shows proteinuria (>3+ on dipstick) and red blood cell casts. Laboratory values are as follows:

- Hemoglobin: 11.2 g/dL
- Platelet count: 190,000/uL
- Serum creatinine: 1.2 mg/dL (baseline 0.8 mg/dL)
- Complement C3: Low
- Complement C4: Low
- Anti-double-stranded DNA antibodies: Positive
- Antiphospholipid antibodies: Negative
