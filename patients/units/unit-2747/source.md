# synthetic_hospital patient 2747

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 52 at first encounter, sex F, Non-Hispanic White, Employer-sponsored private health insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-01-31 (Mon) | outpatient | Gastroenterology |
| 1 | 2021-01-15 | 2023-02-01 (Wed) | outpatient | Rheumatology |
| 2 | 2022-01-15 | 2024-01-29 (Mon) | outpatient | Rheumatology |
| 3 | 2023-01-15 | 2025-01-27 (Mon) | outpatient | Pulmonology |
| 4 | 2024-07-15 | 2026-07-16 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Employer-sponsored private health insurance",
 "occupation": "Librarian",
 "alcohol_use": "No alcohol use",
 "family_history": [
  "Mother: Autoimmune hypothyroidism",
  "Sister: Raynaud's phenomenon",
  "Maternal aunt: Rheumatoid arthritis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "20 mg daily",
   "name": "Omeprazole"
  },
  {
   "dose": "75 mcg daily",
   "name": "Levothyroxine"
  },
  {
   "dose": "30 mg daily",
   "name": "Nifedipine extended-release"
  },
  {
   "dose": "1000 mg twice daily",
   "name": "Mycophenolate mofetil"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Systemic sclerosis (scleroderma)",
  "Interstitial lung disease associated with connective tissue disease",
  "Polymyositis",
  "Gastroesophageal reflux disease",
  "Hypothyroidism",
  "Raynaud's phenomenon",
  "History of Helicobacter pylori infection (eradicated)",
  "History of gastric MALT lymphoma"
 ],
 "age_at_first_encounter": 52
}
```

Primary diagnoses: ["Interstitial lung disease", "Polymyositis", "Extranodal marginal zone B-cell lymphoma of mucosa-associated lymphoid tissue (MALT lymphoma) of the stomach", "Scleroderma renal crisis"]
Comorbidities: ["Undifferentiated Connective Tissue Disease", "Gastroesophageal reflux disease"]

## Encounter 0: source 2020-01-15, practice 2022-01-31, outpatient, Gastroenterology

Chief complaint: Six-month history of epigastric pain worsening with meals and 10-pound weight loss

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of epigastric pain worsening with meals and 10-pound weight loss

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with a 6-month history of epigastric pain that worsens with meals. She also reports a 10-pound weight loss over the same period but denies nausea, vomiting, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic sclerosis (scleroderma)
- Interstitial lung disease associated with connective tissue disease
- Polymyositis
- Gastroesophageal reflux disease
- Hypothyroidism
- Raynaud's phenomenon
- History of Helicobacter pylori infection (eradicated)
- History of gastric MALT lymphoma

Her past medical history is significant for gastroesophageal reflux disease (GERD) and a remote Helicobacter pylori infection treated 5 years ago.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Omeprazole 20 mg daily
- Levothyroxine 75 mcg daily
- Nifedipine extended-release 30 mg daily
- Mycophenolate mofetil 1000 mg twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Autoimmune hypothyroidism
- Sister: Raynaud's phenomenon
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
She does not smoke or consume alcohol.

PHYSICAL EXAMINATION:
Physical examination reveals mild tenderness to palpation in the epigastric region without rebound or guarding.

LABORATORY DATA:
Laboratory studies show:

- Hemoglobin: 12.8 g/dL (Normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 89 fL (Normal: 80-100 fL)
- Platelets: 250,000/uL (Normal: 150,000-450,000/uL)
- Lactate dehydrogenase (LDH): 190 U/L (Normal: 140-280 U/L)
- Serum albumin: 4.2 g/dL (Normal: 3.5-5.0 g/dL)

PATHOLOGY:
Biopsy of the mass is performed, and histopathology is consistent with extranodal marginal zone B cell lymphoma of mucosa-associated lymphoid tissue (MALT lymphoma).

OTHER STUDIES:
Esophagogastroduodenoscopy (EGD) reveals a 2 cm ulcerated mass in the gastric antrum.

## Encounter 1: source 2021-01-15, practice 2023-02-01, outpatient, Rheumatology

Chief complaint: Six-month history of progressive skin tightening, difficulty swallowing, and dry cough

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Rodriguez, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of progressive skin tightening, difficulty swallowing, and dry cough

HISTORY OF PRESENT ILLNESS:
A 53-year-old woman with systemic sclerosis, interstitial lung disease, and a history of gastric MALT lymphoma diagnosed in January 2020 presents to the rheumatology clinic with a 6-month history of increasing shortness of breath and a dry cough. She also reports difficulty swallowing and a sensation of tightness in her skin, which has progressively thickened over her face, neck, and the upper part of her chest.

PAST MEDICAL HISTORY:
Active Problem List:
- Extranodal marginal zone B-cell lymphoma of mucosa-associated lymphoid tissue (MALT lymphoma) of the stomach (diagnosed 2020-01-15)
- Systemic sclerosis (scleroderma)
- Interstitial lung disease associated with connective tissue disease
- Polymyositis
- Gastroesophageal reflux disease
- Hypothyroidism
- Raynaud's phenomenon
- History of Helicobacter pylori infection (eradicated)
- History of gastric MALT lymphoma

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Omeprazole 20 mg daily
- Levothyroxine 75 mcg daily
- Nifedipine extended-release 30 mg daily
- Mycophenolate mofetil 1000 mg twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Autoimmune hypothyroidism
- Sister: Raynaud's phenomenon
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Librarian.

PHYSICAL EXAMINATION:
Taut, shiny skin with loss of normal facial creases and telangiectasias over her face and chest. Her lung auscultation reveals bibasilar velcro-like crackles. Her fingers show sclerodactyly and there are no digital ulcers.

LABORATORY DATA:
- Antinuclear antibodies (ANA): Positive
- Anti-topoisomerase I (anti-Scl-70) antibodies: Positive
- Forced vital capacity (FVC): 65% of predicted
- Diffusion capacity for carbon monoxide (DLCO): 45% of predicted

## Encounter 2: source 2022-01-15, practice 2024-01-29, outpatient, Rheumatology

Chief complaint: Six-month history of increasing difficulty climbing stairs and rising from seated position

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James Rodriguez, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of increasing difficulty climbing stairs and rising from seated position

HISTORY OF PRESENT ILLNESS:
The patient is a 54-year-old female with systemic sclerosis, scleroderma renal crisis, and gastric MALT lymphoma returning for follow-up. She presents to the clinic with a 6-month history of increasing difficulty climbing stairs and getting up from a seated position. She reports a mild, diffuse joint pain but denies skin rash or involvement of the distal muscles.

PAST MEDICAL HISTORY:
Active Problem List:
- Extranodal marginal zone B-cell lymphoma of mucosa-associated lymphoid tissue (MALT lymphoma) of the stomach (diagnosed 2020-01-15)
- Scleroderma renal crisis (diagnosed 2021-01-15)
- Systemic sclerosis (scleroderma)
- Interstitial lung disease associated with connective tissue disease
- Polymyositis
- Gastroesophageal reflux disease
- Hypothyroidism
- Raynaud's phenomenon
- History of Helicobacter pylori infection (eradicated)
- History of gastric MALT lymphoma

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Omeprazole 20 mg daily
- Levothyroxine 75 mcg daily
- Nifedipine extended-release 30 mg daily
- Mycophenolate mofetil 1000 mg twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Autoimmune hypothyroidism
- Sister: Raynaud's phenomenon
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Librarian.

PHYSICAL EXAMINATION:
On physical examination, she has symmetrical proximal muscle weakness in her upper and lower extremities. There is no muscle tenderness, and the rest of the examination is unremarkable.

LABORATORY DATA:
Laboratory studies are as follows:

- Creatine kinase (CK): 980 U/L (Normal: 38-174 U/L)
- Alanine aminotransferase (ALT): 88 U/L (Normal: 7-56 U/L)
- Aspartate aminotransferase (AST): 92 U/L (Normal: 8-48 U/L)
- Antinuclear antibodies (ANA): Positive
- Anti-Jo-1 antibodies: Positive

PATHOLOGY:
A muscle biopsy is performed, which shows a mononuclear infiltrate surrounding necrotic and regenerating muscle fibers.

## Encounter 3: source 2023-01-15, practice 2025-01-27, outpatient, Pulmonology

Chief complaint: Six-month history of progressive dyspnea on exertion and nonproductive cough

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Priya Patel, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of progressive dyspnea on exertion and nonproductive cough

HISTORY OF PRESENT ILLNESS:
With systemic sclerosis and polymyositis, the patient returns for follow-up. She presents to the clinic with a 6-month history of progressive dyspnea on exertion and a nonproductive cough. She denies any fever, chest pain, or wheezing.

PAST MEDICAL HISTORY:
Active Problem List:
- Extranodal marginal zone B-cell lymphoma of mucosa-associated lymphoid tissue (MALT lymphoma) of the stomach (diagnosed 2020-01-15)
- Scleroderma renal crisis (diagnosed 2021-01-15)
- Polymyositis (diagnosed 2022-01-15)
- Systemic sclerosis (scleroderma)
- Interstitial lung disease associated with connective tissue disease
- Gastroesophageal reflux disease
- Hypothyroidism
- Raynaud's phenomenon
- History of Helicobacter pylori infection (eradicated)
- History of gastric MALT lymphoma

She has a past medical history of gastroesophageal reflux disease (GERD) and hypothyroidism.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She takes for which she takes omeprazole and levothyroxine, respectively.

Home Medications:
- Nifedipine extended-release 30 mg daily
- Mycophenolate mofetil 1000 mg twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Autoimmune hypothyroidism
- Sister: Raynaud's phenomenon
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
She has never smoked and works as a librarian.

VITAL SIGNS:
Oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
On physical examination, fine inspiratory crackles are heard at the lung bases bilaterally.

LABORATORY DATA:
Laboratory Values:
- Hemoglobin: 13.2 g/dL
- White Blood Cell Count: 7,000/uL
- Platelets: 250,000/uL
- Erythrocyte Sedimentation Rate: 20 mm/hr
- Antinuclear Antibodies (ANA): Positive
- Rheumatoid Factor (RF): Negative

OTHER STUDIES:
Pulmonary function tests reveal a forced vital capacity (FVC) of 78% predicted, a total lung capacity (TLC) of 72% predicted, and a diffusing capacity for carbon monoxide (DLCO) of 65% predicted.

## Encounter 4: source 2024-07-15, practice 2026-07-16, ed, Emergency Medicine

Chief complaint: Two-day history of severe headache, shortness of breath, and generalized malaise

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Michael Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-day history of severe headache, shortness of breath, and generalized malaise

HISTORY OF PRESENT ILLNESS:
This 56-year-old female with systemic sclerosis, interstitial lung disease, polymyositis, and history of scleroderma renal crisis in 2021 presents to the emergency department with a 2-day history of severe headache, shortness of breath, and generalized malaise. She reports that her blood pressure has always been within the normal range during her routine visits.

PAST MEDICAL HISTORY:
Active Problem List:
- Extranodal marginal zone B-cell lymphoma of mucosa-associated lymphoid tissue (MALT lymphoma) of the stomach (diagnosed 2020-01-15)
- Scleroderma renal crisis (diagnosed 2021-01-15)
- Polymyositis (diagnosed 2022-01-15)
- Interstitial lung disease (diagnosed 2023-01-15)
- Systemic sclerosis (scleroderma)
- Interstitial lung disease associated with connective tissue disease
- Gastroesophageal reflux disease
- Hypothyroidism
- Raynaud's phenomenon
- History of Helicobacter pylori infection (eradicated)
- History of gastric MALT lymphoma

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Omeprazole 20 mg daily
- Levothyroxine 75 mcg daily
- Nifedipine extended-release 30 mg daily
- Mycophenolate mofetil 1000 mg twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Autoimmune hypothyroidism
- Sister: Raynaud's phenomenon
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Librarian.

VITAL SIGNS:
On physical examination, her blood pressure is 190/110 mmHg, heart rate is 88 beats per minute, and respiratory rate is 20 breaths per minute.

PHYSICAL EXAMINATION:
She has diffuse skin thickening and telangiectasias on her face. Auscultation of the lungs reveals fine crackles at the bases.

LABORATORY DATA:
Laboratory findings are as follows:

- Serum creatinine: 2.8 mg/dL (baseline 0.9 mg/dL)
- Blood urea nitrogen (BUN): 40 mg/dL
- Urinalysis: Proteinuria, hematuria, and red blood cell casts
- Serum potassium: 5.2 mEq/L
- Hemoglobin: 10.2 g/dL
- Platelet count: 210,000/uL
- Antinuclear antibodies (ANA): Positive
- Anti-Scl-70 antibodies: Positive
