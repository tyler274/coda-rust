# synthetic_hospital patient 1832

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 20 at first encounter, sex F, Non-Hispanic White, University student health insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2019-01-28 (Mon) | outpatient | Allergy & Immunology |
| 1 | 2020-04-15 | 2019-04-26 (Fri) | outpatient | Cardiology |
| 2 | 2020-10-15 | 2019-10-23 (Wed) | outpatient | Psychiatry |
| 3 | 2022-01-15 | 2021-01-25 (Mon) | outpatient | Gastroenterology |
| 4 | 2025-01-15 | 2024-01-30 (Tue) | outpatient | Pulmonary & Critical Care |
| 5 | 2027-01-15 | 2026-01-26 (Mon) | outpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "University student health insurance",
 "occupation": "Medical student",
 "alcohol_use": "None",
 "family_history": [
  "Father: Hypertrophic cardiomyopathy (diagnosed age 45)",
  "Mother: Phenylketonuria carrier, autoimmune thyroiditis",
  "Older brother: Common variable immunodeficiency",
  "Paternal grandfather: Sudden cardiac death (age 50)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "20g weekly",
   "name": "Immune globulin (subcutaneous)"
  },
  {
   "dose": "50 mg daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "1000 mg daily",
   "name": "Sapropterin dihydrochloride"
  },
  {
   "dose": "20 mg daily",
   "name": "Fluoxetine"
  },
  {
   "dose": "1 tablet daily",
   "name": "Prenatal vitamin with low phenylalanine"
  }
 ],
 "surgical_history": [
  "Tonsillectomy (age 12)",
  "Sinus surgery (age 19)"
 ],
 "chronic_conditions": [
  "Common variable immunodeficiency",
  "Anorexia nervosa, binge-eating/purging type",
  "Hypertrophic obstructive cardiomyopathy",
  "Phenylketonuria",
  "Gilbert syndrome"
 ],
 "age_at_first_encounter": 20
}
```

Primary diagnoses: ["Common variable immunodeficiency", "Bronchial carcinoid tumor", "Anorexia nervosa, binge-eating/purging type", "Hypertrophic obstructive cardiomyopathy", "Phenylketonuria", "Gilbert syndrome"]
Comorbidities: ["Opportunistic pulmonary infection requiring diagnostic sampling", "Family History of Hereditary Hemorrhagic Telangiectasia", "Underweight"]

## Encounter 0: source 2020-01-15, practice 2019-01-28, outpatient, Allergy & Immunology

Chief complaint: Two-year history of recurrent sinus infections, two recent episodes of pneumonia, chronic diarrhea, and weight loss over the past year.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Allergy & Immunology
Visit Type: outpatient

CHIEF COMPLAINT:
Two-year history of recurrent sinus infections, two recent episodes of pneumonia, chronic diarrhea, and weight loss over the past year.

HISTORY OF PRESENT ILLNESS:
She reports chronic diarrhea and weight loss over the past year.

PAST MEDICAL HISTORY:
Active Problem List:
- Common variable immunodeficiency
- Anorexia nervosa, binge-eating/purging type
- Hypertrophic obstructive cardiomyopathy
- Phenylketonuria
- Gilbert syndrome

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Sinus surgery (age 19)

MEDICATIONS:
Home Medications:
- Immune globulin (subcutaneous) 20g weekly
- Metoprolol succinate 50 mg daily
- Sapropterin dihydrochloride 1000 mg daily
- Fluoxetine 20 mg daily
- Prenatal vitamin with low phenylalanine 1 tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Mentions that her father had similar issues and required frequent antibiotics.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Medical student.

PHYSICAL EXAMINATION:
On examination, she appears underweight and has crackles at the lung bases bilaterally.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 13.2 g/dL
- White blood cell count: 6,800 cells/mm³
- Platelets: 250,000 cells/mm³
- Serum immunoglobulin G (IgG): 200 mg/dL (Low)
- Serum immunoglobulin A (IgA): <7 mg/dL (Low)
- Serum immunoglobulin M (IgM): 25 mg/dL (Low)

IMAGING:
- Chest X-ray: Scattered infiltrates in both lower lobes

ASSESSMENT AND PLAN:
Given the patient's clinical presentation and laboratory findings,

## Encounter 1: source 2020-04-15, practice 2019-04-26, outpatient, Cardiology

Chief complaint: Episodes of dizziness and shortness of breath when walking to class or climbing stairs, with chest tightness and palpitations after physical activity.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Johnson, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Episodes of dizziness and shortness of breath when walking to class or climbing stairs, with chest tightness and palpitations after physical activity.

HISTORY OF PRESENT ILLNESS:
Patient returns to cardiology after Allergy and Immunology visit. She also notes several episodes of chest tightness and palpitations, particularly after physical activity.

PAST MEDICAL HISTORY:
Active Problem List:
- Common variable immunodeficiency (diagnosed 2020-01-15)
- Anorexia nervosa, binge-eating/purging type
- Hypertrophic obstructive cardiomyopathy
- Phenylketonuria
- Gilbert syndrome

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Sinus surgery (age 19)

MEDICATIONS:
Home Medications:
- Immune globulin (subcutaneous) 20g weekly
- Metoprolol succinate 50 mg daily
- Sapropterin dihydrochloride 1000 mg daily
- Fluoxetine 20 mg daily
- Prenatal vitamin with low phenylalanine 1 tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Her father died suddenly at age 42 while jogging, and an uncle was diagnosed with an "enlarged heart."

SOCIAL HISTORY:
There is no history of smoking, drug use

VITAL SIGNS:
Vital signs are normal, and oxygen saturation is 99% on room air.

PHYSICAL EXAMINATION:
Cardiac examination reveals a brisk carotid upstroke and a systolic murmur at the left lower sternal border that becomes louder with Valsalva.

IMAGING:
Echocardiography reveals asymmetric septal hypertrophy and systolic anterior motion of the mitral valve.

OTHER STUDIES:
An ECG shows signs of left ventricular hypertrophy.

ASSESSMENT AND PLAN:
She is diagnosed with hypertrophic obstructive cardiomyopathy (HOCM).

Initial therapy is most likely to focus on increasing

## Encounter 2: source 2020-10-15, practice 2019-10-23, outpatient, Psychiatry

Chief complaint: Fatigue and sore throat with significant weight loss over the past year attributed to 'healthier diet and exercise.'

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Priya Patel, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue and sore throat with significant weight loss over the past year attributed to 'healthier diet and exercise.'

HISTORY OF PRESENT ILLNESS:
The patient is a 20-year-old female with common variable immunodeficiency, hypertrophic obstructive cardiomyopathy, phenylketonuria, and Gilbert syndrome presenting to Psychiatry following Allergy/Immunology and Cardiology encounters. She reports a history of significant weight loss over the past year, which she attributes to a "healthier diet and exercise." Despite being underweight for her height, she expresses a fear of gaining weight and dissatisfaction with her body image. She admits to episodes of binge eating followed by self-induced vomiting.

PAST MEDICAL HISTORY:
Active Problem List:
- Common variable immunodeficiency (diagnosed 2020-01-15)
- Hypertrophic obstructive cardiomyopathy (diagnosed 2020-04-15)
- Anorexia nervosa, binge-eating/purging type
- Phenylketonuria
- Gilbert syndrome

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Sinus surgery (age 19)

MEDICATIONS:
Home Medications:
- Immune globulin (subcutaneous) 20g weekly
- Metoprolol succinate 50 mg daily
- Sapropterin dihydrochloride 1000 mg daily
- Fluoxetine 20 mg daily
- Prenatal vitamin with low phenylalanine 1 tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertrophic cardiomyopathy (diagnosed age 45)
- Mother: Phenylketonuria carrier, autoimmune thyroiditis
- Older brother: Common variable immunodeficiency
- Paternal grandfather: Sudden cardiac death (age 50)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Medical student.

VITAL SIGNS:
Her BMI is 16.5 kg/m^2

PHYSICAL EXAMINATION:
On examination, she has parotid gland hypertrophy, pharyngeal erythema, and dental enamel erosion. Her hands show scars on the dorsum.

LABORATORY DATA:
Laboratory findings are as follows:

- Serum potassium: 3.2 mEq/L (Normal: 3.5-5.0 mEq/L)
- Serum sodium: 140 mEq/L (Normal: 135-145 mEq/L)
- Serum chloride: 88 mEq/L (Normal: 98-107 mEq/L)
- Serum bicarbonate: 32 mEq/L (Normal: 22-29 mEq/L)
- Serum magnesium: 1.6 mg/dL (Normal: 1.7-2.2 mg/dL)

## Encounter 3: source 2022-01-15, practice 2021-01-25, outpatient, Gastroenterology

Chief complaint: Two-day history of yellowing of the eyes while studying for final exams; reports feeling 'worn out' but denies pain, fever, or dark urine.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. David Kim, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Two-day history of yellowing of the eyes while studying for final exams; reports feeling 'worn out' but denies pain, fever, or dark urine.

HISTORY OF PRESENT ILLNESS:
The patient with established common variable immunodeficiency, hypertrophic obstructive cardiomyopathy, anorexia nervosa, phenylketonuria, and Gilbert syndrome returns for evaluation. She is currently studying for her final exams in medical school and reports feeling "a little worn out," but otherwise feels well. She denies abdominal pain, nausea, vomiting, changes in bowel movements, or dark urine. She has no fever, weight loss, or recent travel.

PAST MEDICAL HISTORY:
Active Problem List:
- Common variable immunodeficiency (diagnosed 2020-01-15)
- Hypertrophic obstructive cardiomyopathy (diagnosed 2020-04-15)
- Anorexia nervosa, binge-eating/purging type (diagnosed 2020-10-15)
- Phenylketonuria
- Gilbert syndrome

She has never required hospitalization and has never had abnormal liver tests.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Sinus surgery (age 19)

MEDICATIONS:
She does not take any medications or supplements.

Home Medications:
- Immune globulin (subcutaneous) 20g weekly
- Metoprolol succinate 50 mg daily
- Sapropterin dihydrochloride 1000 mg daily
- Fluoxetine 20 mg daily
- Prenatal vitamin with low phenylalanine 1 tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Her family history is notable for her brother having been told he had "a harmless liver issue" in his teens.

SOCIAL HISTORY:
She denies any alcohol use.

VITAL SIGNS:
Her BMI is 20, and vital signs are within normal limits.

PHYSICAL EXAMINATION:
Physical exam is normal except for mild scleral icterus. There is no hepatosplenomegaly or abdominal tenderness. The rest of her physical examination, including neurologic, cardiovascular, and skin examination, is unremarkable.

PLAN:
A comprehensive metabolic panel and complete blood count are ordered.

## Encounter 4: source 2025-01-15, practice 2024-01-30, outpatient, Pulmonary & Critical Care

Chief complaint: Six-month history of intermittent wheezing and shortness of breath with recurrent episodes of pneumonia always affecting the right lower lobe.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Pulmonary & Critical Care
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of intermittent wheezing and shortness of breath with recurrent episodes of pneumonia always affecting the right lower lobe.

HISTORY OF PRESENT ILLNESS:
The patient, with history of recurrent sinus infections and prior weight loss, returns with ongoing respiratory symptoms. She reports recurrent episodes of pneumonia, always affecting the right lower lobe, over the past year. She denies smoking, weight loss, or night sweats.

PAST MEDICAL HISTORY:
Active Problem List:
- Common variable immunodeficiency (diagnosed 2020-01-15)
- Hypertrophic obstructive cardiomyopathy (diagnosed 2020-04-15)
- Anorexia nervosa, binge-eating/purging type (diagnosed 2020-10-15)
- Gilbert syndrome (diagnosed 2022-01-15)
- Phenylketonuria

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Sinus surgery (age 19)

MEDICATIONS:
She is.

Home Medications:
- Immune globulin (subcutaneous) 20g weekly
- Metoprolol succinate 50 mg daily
- Sapropterin dihydrochloride 1000 mg daily
- Fluoxetine 20 mg daily
- Prenatal vitamin with low phenylalanine 1 tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertrophic cardiomyopathy (diagnosed age 45)
- Mother: Phenylketonuria carrier, autoimmune thyroiditis
- Older brother: Common variable immunodeficiency
- Paternal grandfather: Sudden cardiac death (age 50)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Medical student.

PHYSICAL EXAMINATION:
On physical examination, she has decreased breath sounds and dullness to percussion over the right lower lobe.

LABORATORY DATA:
Laboratory tests reveal:
- Hemoglobin: 13.2 g/dL
- White blood cell count: 9,000/uL
- Platelets: 250,000/uL
- Serum sodium: 140 mEq/L
- Serum potassium: 4.2 mEq/L
- Serum calcium: 9.8 mg/dL
- Fasting glucose: 90 mg/dL

IMAGING:
Chest X-ray reveals a right lower lobe consolidation. A CT scan of the chest shows a well-circumscribed, enhancing mass in the proximal right lower lobe bronchus.

## Encounter 5: source 2027-01-15, practice 2026-01-26, outpatient, Obstetrics & Gynecology

Chief complaint: Preconception counseling visit with her husband to discuss pregnancy planning.

ENCOUNTER NOTE
Date: 2027-01-15
Provider: Dr. Emily Watson, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Preconception counseling visit with her husband to discuss pregnancy planning.

PAST MEDICAL HISTORY:
Active Problem List:
- Common variable immunodeficiency (diagnosed 2020-01-15)
- Hypertrophic obstructive cardiomyopathy (diagnosed 2020-04-15)
- Anorexia nervosa, binge-eating/purging type (diagnosed 2020-10-15)
- Gilbert syndrome (diagnosed 2022-01-15)
- Bronchial carcinoid tumor (diagnosed 2025-01-15)
- Phenylketonuria

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Sinus surgery (age 19)

MEDICATIONS:
Home Medications:
- Immune globulin (subcutaneous) 20g weekly
- Metoprolol succinate 50 mg daily
- Sapropterin dihydrochloride 1000 mg daily
- Fluoxetine 20 mg daily
- Prenatal vitamin with low phenylalanine 1 tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Her family history is remarkable for a sister who died at the age of 6 from phenylketonuria. Her husband's family history is unremarkable.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Medical student.
