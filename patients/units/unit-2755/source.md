# synthetic_hospital patient 2755

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 36 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-03-21 (Mon) | outpatient | Allergy & Immunology |
| 1 | 2021-01-15 | 2023-03-21 (Tue) | outpatient | Rheumatology |
| 2 | 2022-01-15 | 2024-03-11 (Mon) | outpatient | Gastroenterology |
| 3 | 2023-01-15 | 2025-03-19 (Wed) | outpatient | Endocrinology |
| 4 | 2024-01-15 | 2026-03-16 (Mon) | outpatient | Neurology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Penicillin (rash)",
  "Sulfa drugs (hives)"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Administrative Assistant",
 "alcohol_use": "Social (1-2 drinks monthly), denies abuse",
 "family_history": [
  "Mother: Hashimoto's thyroiditis, rheumatoid arthritis",
  "Father: Type 1 diabetes mellitus",
  "Sister: Systemic lupus erythematosus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "112 mcg PO daily",
   "name": "Levothyroxine"
  },
  {
   "dose": "24 units SC at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "sliding scale SC before meals",
   "name": "Insulin lispro"
  },
  {
   "dose": "20 mg PO qAM, 10 mg PO qPM",
   "name": "Hydrocortisone"
  },
  {
   "dose": "0.1 mg PO daily",
   "name": "Fludrocortisone"
  },
  {
   "dose": "200 mg PO BID",
   "name": "Hydroxychloroquine"
  },
  {
   "dose": "75 mg PO daily",
   "name": "Azathioprine"
  },
  {
   "dose": "20g weekly",
   "name": "IVIG (subcutaneous)"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Omeprazole"
  }
 ],
 "surgical_history": [
  "Chest tube thoracostomy (prior pneumonitis with parapneumonic effusion)",
  "Functional endoscopic sinus surgery (prior)"
 ],
 "chronic_conditions": [
  "Hashimoto's thyroiditis",
  "Type 1 diabetes mellitus",
  "Common variable immunodeficiency",
  "Mixed connective tissue disease",
  "Autoimmune hepatitis",
  "Primary adrenal insufficiency",
  "Suprascapular neuropathy (right shoulder)"
 ],
 "age_at_first_encounter": 36
}
```

Primary diagnoses: ["Common variable immunodeficiency", "Mixed connective tissue disease", "Autoimmune Hepatitis", "Suprascapular nerve entrapment", "Primary adrenal insufficiency"]
Comorbidities: ["Opportunistic pulmonary infection requiring diagnostic sampling", "Hashimoto thyroiditis"]

## Encounter 0: source 2020-01-15, practice 2022-03-21, outpatient, Allergy & Immunology

Chief complaint: Three-year history of recurrent sinus infections with two episodes of pneumonia in the last year, persistent cough, fatigue, and 10-pound weight loss

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Allergy & Immunology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-year history of recurrent sinus infections with two episodes of pneumonia in the last year, persistent cough, fatigue, and 10-pound weight loss

HISTORY OF PRESENT ILLNESS:
3-year history of recurrent sinus infections and two episodes of pneumonia in the last year. She reports a persistent cough, fatigue, and a recent unexplained weight loss of 10 pounds.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto's thyroiditis
- Type 1 diabetes mellitus
- Common variable immunodeficiency
- Mixed connective tissue disease
- Autoimmune hepatitis
- Primary adrenal insufficiency
- Suprascapular neuropathy (right shoulder)

Her past medical history is significant for autoimmune thyroiditis.

PAST SURGICAL HISTORY:
- Chest tube thoracostomy (prior pneumonitis with parapneumonic effusion)
- Functional endoscopic sinus surgery (prior)

MEDICATIONS:
Home Medications:
- Levothyroxine 112 mcg PO daily
- Insulin glargine 24 units SC at bedtime
- Insulin lispro sliding scale SC before meals
- Hydrocortisone 20 mg PO qAM, 10 mg PO qPM
- Fludrocortisone 0.1 mg PO daily
- Hydroxychloroquine 200 mg PO BID
- Azathioprine 75 mg PO daily
- IVIG (subcutaneous) 20g weekly
- Omeprazole 20 mg PO daily

ALLERGIES:
Penicillin (rash), Sulfa drugs (hives)

FAMILY HISTORY:
- Mother: Hashimoto's thyroiditis, rheumatoid arthritis
- Father: Type 1 diabetes mellitus
- Sister: Systemic lupus erythematosus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks monthly), denies abuse. Occupation: Administrative Assistant.

PHYSICAL EXAMINATION:
On physical examination, she is afebrile, and her lung auscultation reveals decreased breath sounds at the bases bilaterally without wheezes or crackles.

LABORATORY DATA:
- Hemoglobin: 13.2 g/dL (normal: 12-16 g/dL)
- White blood cell count: 6,500 cells/µL (normal: 4,500-11,000 cells/µL)
- Platelets: 250,000 cells/µL (normal: 150,000-450,000 cells/µL)
- Serum immunoglobulin G (IgG): 200 mg/dL (normal: 700-1600 mg/dL)
- Serum immunoglobulin A (IgA): 20 mg/dL (normal: 70-400 mg/dL)
- Serum immunoglobulin M (IgM): 25 mg/dL (normal: 40-230 mg/dL)

## Encounter 1: source 2021-01-15, practice 2023-03-21, outpatient, Rheumatology

Chief complaint: Six-month history of fatigue, joint pain, and intermittent swelling of hands with episodes of fingers turning white and blue in cold temperatures

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of fatigue, joint pain, and intermittent swelling of hands with episodes of fingers turning white and blue in cold temperatures

HISTORY OF PRESENT ILLNESS:
The patient returns to Rheumatology clinic for follow-up of her mixed connective tissue disease in the context of common variable immunodeficiency established one year prior. She reports experiencing episodes of fingers turning white and blue when exposed to cold temperatures. She also mentions difficulty swallowing and a recent onset of muscle weakness.

PAST MEDICAL HISTORY:
Active Problem List:
- Common variable immunodeficiency (diagnosed 2020-01-15)
- Hashimoto's thyroiditis
- Type 1 diabetes mellitus
- Mixed connective tissue disease
- Autoimmune hepatitis
- Primary adrenal insufficiency
- Suprascapular neuropathy (right shoulder)

Her past medical history is significant for Hashimoto's thyroiditis.

PAST SURGICAL HISTORY:
- Chest tube thoracostomy (prior pneumonitis with parapneumonic effusion)
- Functional endoscopic sinus surgery (prior)

MEDICATIONS:
Home Medications:
- Levothyroxine 112 mcg PO daily
- Insulin glargine 24 units SC at bedtime
- Insulin lispro sliding scale SC before meals
- Hydrocortisone 20 mg PO qAM, 10 mg PO qPM
- Fludrocortisone 0.1 mg PO daily
- Hydroxychloroquine 200 mg PO BID
- Azathioprine 75 mg PO daily
- IVIG (subcutaneous) 20g weekly
- Omeprazole 20 mg PO daily

ALLERGIES:
Penicillin (rash), Sulfa drugs (hives)

FAMILY HISTORY:
- Mother: Hashimoto's thyroiditis, rheumatoid arthritis
- Father: Type 1 diabetes mellitus
- Sister: Systemic lupus erythematosus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks monthly), denies abuse. Occupation: Administrative Assistant.

PHYSICAL EXAMINATION:
On physical examination, you note symmetrical swelling of the fingers and hands, and palpation of the muscles elicits tenderness. Nailfold capillaroscopy shows dilated capillary loops.

LABORATORY DATA:
Laboratory tests reveal the following:

- ANA: Positive
- Anti-dsDNA: Negative
- Anti-Smith: Negative
- Anti-U1 RNP: Positive
- Anti-centromere: Negative
- Anti-Scl-70: Negative
- CK: Elevated
- ESR: Elevated

## Encounter 2: source 2022-01-15, practice 2024-03-11, outpatient, Gastroenterology

Chief complaint: Six-month history of progressive fatigue, joint pain, and occasional right upper quadrant abdominal discomfort

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of progressive fatigue, joint pain, and occasional right upper quadrant abdominal discomfort

HISTORY OF PRESENT ILLNESS:
This 37-year-old woman with common variable immunodeficiency and mixed connective tissue disease presents with a 6-month history of fatigue, joint pain, and occasional right upper quadrant discomfort.

PAST MEDICAL HISTORY:
Active Problem List:
- Common variable immunodeficiency (diagnosed 2020-01-15)
- Mixed connective tissue disease (diagnosed 2021-01-15)
- Hashimoto's thyroiditis
- Type 1 diabetes mellitus
- Autoimmune hepatitis
- Primary adrenal insufficiency
- Suprascapular neuropathy (right shoulder)

Her past medical history is significant for Hashimoto's thyroiditis.

PAST SURGICAL HISTORY:
- Chest tube thoracostomy (prior pneumonitis with parapneumonic effusion)
- Functional endoscopic sinus surgery (prior)

MEDICATIONS:
Home Medications:
- Levothyroxine 112 mcg PO daily
- Insulin glargine 24 units SC at bedtime
- Insulin lispro sliding scale SC before meals
- Hydrocortisone 20 mg PO qAM, 10 mg PO qPM
- Fludrocortisone 0.1 mg PO daily
- Hydroxychloroquine 200 mg PO BID
- Azathioprine 75 mg PO daily
- IVIG (subcutaneous) 20g weekly
- Omeprazole 20 mg PO daily

ALLERGIES:
Penicillin (rash), Sulfa drugs (hives)

FAMILY HISTORY:
- Mother: Hashimoto's thyroiditis, rheumatoid arthritis
- Father: Type 1 diabetes mellitus
- Sister: Systemic lupus erythematosus

SOCIAL HISTORY:
She denies any history of alcohol abuse, drug use, or high-risk sexual behaviors.

PHYSICAL EXAMINATION:
On physical examination, she has mild hepatomegaly without splenomegaly. Her skin and sclerae are anicteric.

LABORATORY DATA:
Laboratory studies reveal the following:

- Aspartate aminotransferase (AST): 220 U/L (Normal: 10-40 U/L)
- Alanine aminotransferase (ALT): 240 U/L (Normal: 7-56 U/L)
- Alkaline phosphatase (ALP): 90 U/L (Normal: 40-130 U/L)
- Total bilirubin: 0.8 mg/dL (Normal: 0.3-1.2 mg/dL)
- Serum globulins: 4.5 g/dL (Normal: 2.0-3.5 g/dL)
- Gamma-glutamyl transferase (GGT): 45 U/L (Normal: 9-48 U/L)

Serologic testing shows positive antinuclear antibodies (ANA) and anti-smooth muscle antibodies (ASMA).

## Encounter 3: source 2023-01-15, practice 2025-03-19, outpatient, Endocrinology

Chief complaint: Six-month history of progressive fatigue, generalized weakness, 10-pound weight loss, orthostatic dizziness, and syncopal episodes

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James O'Brien, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of progressive fatigue, generalized weakness, 10-pound weight loss, orthostatic dizziness, and syncopal episodes

HISTORY OF PRESENT ILLNESS:
The patient is a 39-year-old female with common variable immunodeficiency diagnosed in 2020, mixed connective tissue disease diagnosed in 2021, and autoimmune hepatitis diagnosed in 2022, who presents with a six-month history of progressive fatigue, generalized weakness, and a 10-pound weight loss. She reports occasional episodes of dizziness, especially upon standing, and has had several episodes of syncope, along with persistent nausea and decreased appetite.

PAST MEDICAL HISTORY:
Active Problem List:
- Common variable immunodeficiency (diagnosed 2020-01-15)
- Mixed connective tissue disease (diagnosed 2021-01-15)
- Autoimmune Hepatitis (diagnosed 2022-01-15)
- Hashimoto's thyroiditis
- Type 1 diabetes mellitus
- Primary adrenal insufficiency
- Suprascapular neuropathy (right shoulder)

Her past medical history is significant for type 1 diabetes mellitus and autoimmune thyroiditis.

PAST SURGICAL HISTORY:
- Chest tube thoracostomy (prior pneumonitis with parapneumonic effusion)
- Functional endoscopic sinus surgery (prior)

MEDICATIONS:
Home Medications:
- Levothyroxine 112 mcg PO daily
- Insulin glargine 24 units SC at bedtime
- Insulin lispro sliding scale SC before meals
- Hydrocortisone 20 mg PO qAM, 10 mg PO qPM
- Fludrocortisone 0.1 mg PO daily
- Hydroxychloroquine 200 mg PO BID
- Azathioprine 75 mg PO daily
- IVIG (subcutaneous) 20g weekly
- Omeprazole 20 mg PO daily

ALLERGIES:
Penicillin (rash), Sulfa drugs (hives)

FAMILY HISTORY:
- Mother: Hashimoto's thyroiditis, rheumatoid arthritis
- Father: Type 1 diabetes mellitus
- Sister: Systemic lupus erythematosus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks monthly), denies abuse. Occupation: Administrative Assistant.

VITAL SIGNS:
On physical examination, her blood pressure is 90/60 mmHg, and her heart rate is 55 bpm.

PHYSICAL EXAMINATION:
She appears thin and has hyperpigmented creases on her palms.

LABORATORY DATA:
Laboratory results are as follows:

- Sodium: 130 mEq/L (Normal: 135-145 mEq/L)
- Potassium: 5.5 mEq/L (Normal: 3.5-5.0 mEq/L)
- Chloride: 95 mEq/L (Normal: 98-106 mEq/L)
- Bicarbonate: 22 mEq/L (Normal: 22-29 mEq/L)
- Blood urea nitrogen (BUN): 18 mg/dL (Normal: 7-20 mg/dL)
- Creatinine: 0.9 mg/dL (Normal: 0.6-1.1 mg/dL)
- Glucose: 72 mg/dL (Normal: 70-99 mg/dL)
- Cortisol (8 AM): 3 µg/dL (Normal: 7-25 µg/dL)
- ACTH: 1200 pg/mL (Normal: 10-60 pg/mL)

## Encounter 4: source 2024-01-15, practice 2026-03-16, outpatient, Neurology

Chief complaint: Persistent difficulty lifting right arm overhead and reaching high shelves several weeks after hospitalization for severe lung infection

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Aisha Johnson, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent difficulty lifting right arm overhead and reaching high shelves several weeks after hospitalization for severe lung infection

HISTORY OF PRESENT ILLNESS:
The patient is a 40-year-old woman with common variable immunodeficiency and a history of mixed connective tissue disease and autoimmune hepatitis who presents for evaluation of persistent right upper extremity symptoms following recent hospitalization for a severe lung infection complicated by pleural fluid accumulation. She reports persistent difficulty lifting her right arm overhead several weeks after hospitalization. She describes trouble reaching high shelves and brushing her hair. Her prior admission involved chest drainage due to accumulation of infected fluid in the right pleural cavity. Recovery was complicated by discomfort near the upper chest wall, though the infection has resolved.

PAST MEDICAL HISTORY:
Active Problem List:
- Common variable immunodeficiency (diagnosed 2020-01-15)
- Mixed connective tissue disease (diagnosed 2021-01-15)
- Autoimmune Hepatitis (diagnosed 2022-01-15)
- Primary adrenal insufficiency (diagnosed 2023-01-15)
- Hashimoto's thyroiditis
- Type 1 diabetes mellitus
- Suprascapular neuropathy (right shoulder)

PAST SURGICAL HISTORY:
- Chest tube thoracostomy (prior pneumonitis with parapneumonic effusion)
- Functional endoscopic sinus surgery (prior)

MEDICATIONS:
Home Medications:
- Levothyroxine 112 mcg PO daily
- Insulin glargine 24 units SC at bedtime
- Insulin lispro sliding scale SC before meals
- Hydrocortisone 20 mg PO qAM, 10 mg PO qPM
- Fludrocortisone 0.1 mg PO daily
- Hydroxychloroquine 200 mg PO BID
- Azathioprine 75 mg PO daily
- IVIG (subcutaneous) 20g weekly
- Omeprazole 20 mg PO daily

ALLERGIES:
Penicillin (rash), Sulfa drugs (hives)

FAMILY HISTORY:
- Mother: Hashimoto's thyroiditis, rheumatoid arthritis
- Father: Type 1 diabetes mellitus
- Sister: Systemic lupus erythematosus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks monthly), denies abuse. Occupation: Administrative Assistant.

PHYSICAL EXAMINATION:
On examination, motor strength and reflexes in the arm and forearm are preserved. Passive and active range of motion at the glenohumeral joint is mostly intact. When asked to push against the wall with both palms forward and arms extended, there is noticeable protrusion along the inner border of her right scapula. No sensory deficits are detected over the shoulder or arm. There is no muscle wasting or deformity over the deltoid, supraspinatus, or trapezius regions.

IMAGING:
A plain radiograph of the shoulder shows no fractures or dislocations.
