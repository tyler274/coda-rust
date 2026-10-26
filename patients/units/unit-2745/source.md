# synthetic_hospital patient 2745

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 44 at first encounter, sex F, Hispanic, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-11-27 (Fri) | outpatient | Endocrinology |
| 1 | 2020-07-15 | 2021-05-28 (Fri) | outpatient | Rheumatology |
| 2 | 2021-03-15 | 2022-01-26 (Wed) | outpatient | Internal Medicine |
| 3 | 2021-11-15 | 2022-09-26 (Mon) | outpatient | Rheumatology |
| 4 | 2023-03-15 | 2024-01-29 (Mon) | outpatient | Gastroenterology |
| 5 | 2024-03-15 | 2025-02-03 (Mon) | outpatient | Neurology |
| 6 | 2025-07-15 | 2026-05-27 (Wed) | ed | Emergency Medicine |
| 7 | 2025-08-15 | 2026-07-10 (Fri) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Anti-IgA antibodies (anaphylactic risk with blood products)",
  "Sulfasalazine (severe gastrointestinal intolerance)"
 ],
 "insurance": "Medicaid",
 "occupation": "International public health consultant",
 "alcohol_use": "Former heavy use; currently abstinent for 18 months",
 "family_history": [
  "Mother: Rheumatoid arthritis",
  "Father: Essential hypertension",
  "Sister: Hashimoto's thyroiditis",
  "Maternal aunt: Crohn's disease"
 ],
 "race_ethnicity": "Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "15 mg oral weekly",
   "name": "Methotrexate"
  },
  {
   "dose": "1 mg oral daily",
   "name": "Folic acid"
  },
  {
   "dose": "75 mcg oral daily",
   "name": "Levothyroxine"
  },
  {
   "dose": "2000 IU oral daily",
   "name": "Cholecalciferol"
  }
 ],
 "surgical_history": [
  "Cesarean section complicated by postpartum hemorrhage requiring packed red blood cell transfusion (age 41)",
  "Colonoscopy with biopsy confirming Crohn's disease (age 42)"
 ],
 "chronic_conditions": [
  "Rheumatoid arthritis, seropositive",
  "Crohn's disease, ileocolonic",
  "Central hypothyroidism secondary to Sheehan's syndrome",
  "Selective IgA deficiency",
  "Major depressive disorder, recurrent, currently untreated",
  "History of alcohol use disorder, in early remission"
 ],
 "age_at_first_encounter": 44
}
```

Primary diagnoses: ["Methotrexate-induced folate deficiency", "Central hypothyroidism", "Crohn's disease flare with inflammatory stricture (without complete obstruction)", "Metronidazole-induced polyneuropathy", "Pellagra (Niacin deficiency)", "Acute promyelocytic leukemia", "Acute hemolytic transfusion reaction"]
Comorbidities: ["Anemia", "Rheumatoid arthritis"]

## Encounter 0: source 2020-01-15, practice 2020-11-27, outpatient, Endocrinology

Chief complaint: Five months of worsening fatigue, increasing forgetfulness, and decreased exercise tolerance

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Five months of worsening fatigue, increasing forgetfulness, and decreased exercise tolerance

HISTORY OF PRESENT ILLNESS:
She reports feeling unmotivated, emotionally flat, and frequently chilly even during warmer days. She has also experienced mild weight gain and occasional constipation. Her husband notes she has become less interactive and more withdrawn over the past few months. She denies any recent illness, medications, or travel.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatoid arthritis, seropositive
- Crohn's disease, ileocolonic
- Central hypothyroidism secondary to Sheehan's syndrome
- Selective IgA deficiency
- Major depressive disorder, recurrent, currently untreated
- History of alcohol use disorder, in early remission

Her medical history is significant for a postpartum hemorrhage that required transfusion three years ago. Since that time, her menstrual cycles have been irregular, and for the past year she has had complete cessation of menses.

PAST SURGICAL HISTORY:
- Cesarean section complicated by postpartum hemorrhage requiring packed red blood cell transfusion (age 41)
- Colonoscopy with biopsy confirming Crohn's disease (age 42)

MEDICATIONS:
Home Medications:
- Methotrexate 15 mg oral weekly
- Folic acid 1 mg oral daily
- Levothyroxine 75 mcg oral daily
- Cholecalciferol 2000 IU oral daily

ALLERGIES:
Anti-IgA antibodies (anaphylactic risk with blood products), Sulfasalazine (severe gastrointestinal intolerance)

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Father: Essential hypertension
- Sister: Hashimoto's thyroiditis
- Maternal aunt: Crohn's disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former heavy use; currently abstinent for 18 months. Occupation: International public health consultant.

VITAL SIGNS:
On physical exam, her pulse is 55/min, and blood pressure is 94/60 mm Hg.

PHYSICAL EXAMINATION:
The thyroid gland is small and non-tender. Her skin is dry, and the outer eyebrows appear thinned. She has reduced axillary and pubic hair. Neurologic exam reveals mild bradykinesia and delayed relaxation of deep tendon reflexes.

LABORATORY DATA:
Laboratory studies reveal anemia and mild hyponatremia.

IMAGING:
MRI of the brain shows an empty sella.

## Encounter 1: source 2020-07-15, practice 2021-05-28, outpatient, Rheumatology

Chief complaint: Routine follow-up with worsening fatigue over two months and easy bruising

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Torres, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up with worsening fatigue over two months and easy bruising

HISTORY OF PRESENT ILLNESS:
The patient with seropositive rheumatoid arthritis returns for rheumatology follow-up following her January 2020 endocrinology evaluation for fatigue. She reports feeling more fatigued over the last two months and has noticed that she bruises easily.

PAST MEDICAL HISTORY:
Active Problem List:
- Central hypothyroidism (diagnosed 2020-01-15)
- Rheumatoid arthritis, seropositive
- Crohn's disease, ileocolonic
- Central hypothyroidism secondary to Sheehan's syndrome
- Selective IgA deficiency
- Major depressive disorder, recurrent, currently untreated
- History of alcohol use disorder, in early remission

PAST SURGICAL HISTORY:
- Cesarean section complicated by postpartum hemorrhage requiring packed red blood cell transfusion (age 41)
- Colonoscopy with biopsy confirming Crohn's disease (age 42)

MEDICATIONS:
She takes methotrexate 15 mg weekly for the past 3 years.

Home Medications:
- Folic acid 1 mg oral daily
- Levothyroxine 75 mcg oral daily
- Cholecalciferol 2000 IU oral daily

ALLERGIES:
Anti-IgA antibodies (anaphylactic risk with blood products), Sulfasalazine (severe gastrointestinal intolerance)

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Father: Essential hypertension
- Sister: Hashimoto's thyroiditis
- Maternal aunt: Crohn's disease

SOCIAL HISTORY:
She does not drink alcohol

REVIEW OF SYSTEMS:
She denies any changes in bowel habits, hematemesis, or melena

PHYSICAL EXAMINATION:
She appears pale and has several ecchymoses on her arms and legs

LABORATORY DATA:
Laboratory tests reveal the following:

- Hemoglobin: 9.8 g/dL (Normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 102 fL (Normal: 80-100 fL)
- Platelet count: 200,000/mm³ (Normal: 150,000-450,000/mm³)
- Aspartate aminotransferase (AST): 70 U/L (Normal: 10-40 U/L)
- Alanine aminotransferase (ALT): 85 U/L (Normal: 7-56 U/L)
- Alkaline phosphatase (ALP): 90 U/L (Normal: 40-130 U/L)
- Total bilirubin: 1.2 mg/dL (Normal: 0.1-1.2 mg/dL)

## Encounter 2: source 2021-03-15, practice 2022-01-26, outpatient, Internal Medicine

Chief complaint: One-month history of diarrhea, generalized weakness, and photosensitive rash on face and dorsal hands

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
One-month history of diarrhea, generalized weakness, and photosensitive rash on face and dorsal hands

HISTORY OF PRESENT ILLNESS:
The patient with rheumatoid arthritis and prior deficiencies presents with new complaints. She reports that the rash initially appeared as erythema but has progressed to hyperpigmented plaques with scaling. She also complains of difficulty concentrating and memory lapses.

PAST MEDICAL HISTORY:
Active Problem List:
- Central hypothyroidism (diagnosed 2020-01-15)
- Methotrexate-induced folate deficiency (diagnosed 2020-07-15)
- Rheumatoid arthritis, seropositive
- Crohn's disease, ileocolonic
- Central hypothyroidism secondary to Sheehan's syndrome
- Selective IgA deficiency
- Major depressive disorder, recurrent, currently untreated
- History of alcohol use disorder, in early remission

Her medical history is significant for alcohol use disorder and depression, for which she is not currently receiving treatment.

PAST SURGICAL HISTORY:
- Cesarean section complicated by postpartum hemorrhage requiring packed red blood cell transfusion (age 41)
- Colonoscopy with biopsy confirming Crohn's disease (age 42)

MEDICATIONS:
Home Medications:
- Methotrexate 15 mg oral weekly
- Folic acid 1 mg oral daily
- Levothyroxine 75 mcg oral daily
- Cholecalciferol 2000 IU oral daily

ALLERGIES:
Anti-IgA antibodies (anaphylactic risk with blood products), Sulfasalazine (severe gastrointestinal intolerance)

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Father: Essential hypertension
- Sister: Hashimoto's thyroiditis
- Maternal aunt: Crohn's disease

SOCIAL HISTORY:
She has a limited income and admits to a diet primarily consisting of corn-based products.

PHYSICAL EXAMINATION:
Physical examination reveals a malnourished appearance, glossitis, and bilateral symmetric hyperpigmented plaques on sun-exposed areas. Neurological examination is notable for mild confusion and ataxia.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin: 10.2 g/dL (Normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 95 fL (Normal: 80-100 fL)
- Serum albumin: 3.2 g/dL (Normal: 3.5-5.0 g/dL)
- Serum niacin: Pending
- Serum thiamine: Normal
- Serum folate: Normal
- Serum vitamin B12: Normal

ASSESSMENT AND PLAN:
Based on the patient's presentation and history,

## Encounter 3: source 2021-11-15, practice 2022-09-26, outpatient, Rheumatology

Chief complaint: Fatigue, painful mouth sores, and a recent episode of epistaxis

ENCOUNTER NOTE
Date: 2021-11-15
Provider: Dr. Michael Torres, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, painful mouth sores, and a recent episode of epistaxis

HISTORY OF PRESENT ILLNESS:
The patient with seropositive rheumatoid arthritis, central hypothyroidism, and history of methotrexate-induced folate deficiency returns for outpatient rheumatology follow-up. Her rheumatoid arthritis symptoms have improved significantly since starting the medication.

PAST MEDICAL HISTORY:
Active Problem List:
- Central hypothyroidism (diagnosed 2020-01-15)
- Methotrexate-induced folate deficiency (diagnosed 2020-07-15)
- Pellagra (Niacin deficiency) (diagnosed 2021-03-15)
- Rheumatoid arthritis, seropositive
- Crohn's disease, ileocolonic
- Central hypothyroidism secondary to Sheehan's syndrome
- Selective IgA deficiency
- Major depressive disorder, recurrent, currently untreated
- History of alcohol use disorder, in early remission

PAST SURGICAL HISTORY:
- Cesarean section complicated by postpartum hemorrhage requiring packed red blood cell transfusion (age 41)
- Colonoscopy with biopsy confirming Crohn's disease (age 42)

MEDICATIONS:
She has been on methotrexate 15 mg weekly for the past 6 months.

Home Medications:
- Folic acid 1 mg oral daily
- Levothyroxine 75 mcg oral daily
- Cholecalciferol 2000 IU oral daily

ALLERGIES:
Anti-IgA antibodies (anaphylactic risk with blood products), Sulfasalazine (severe gastrointestinal intolerance)

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Father: Essential hypertension
- Sister: Hashimoto's thyroiditis
- Maternal aunt: Crohn's disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former heavy use; currently abstinent for 18 months. Occupation: International public health consultant.

PHYSICAL EXAMINATION:
On examination, she has multiple shallow ulcers on her buccal mucosa and her conjunctivae appear pale.

LABORATORY DATA:
Her laboratory tests show:

- Hemoglobin: 10.2 g/dL (Normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 102 fL (Normal: 80-100 fL)
- Platelet count: 200,000/mm³ (Normal: 150,000-450,000/mm³)
- White blood cell count: 5,000/mm³ (Normal: 4,500-11,000/mm³)
- Liver function tests: within normal limits

## Encounter 4: source 2023-03-15, practice 2024-01-29, outpatient, Gastroenterology

Chief complaint: Several months of intermittent abdominal pain and non-bloody diarrhea with unintentional weight loss

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. David Kim, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Several months of intermittent abdominal pain and non-bloody diarrhea with unintentional weight loss

HISTORY OF PRESENT ILLNESS:
The patient with ileocolonic Crohn's disease and history of methotrexate-induced deficiencies presents with several months of intermittent abdominal pain and non-bloody diarrhea. She reports unintentional weight loss and increased stool frequency, particularly after meals.

PAST MEDICAL HISTORY:
Active Problem List:
- Central hypothyroidism (diagnosed 2020-01-15)
- Methotrexate-induced folate deficiency (diagnosed 2020-07-15)
- Pellagra (Niacin deficiency) (diagnosed 2021-03-15)
- Rheumatoid arthritis, seropositive
- Crohn's disease, ileocolonic
- Central hypothyroidism secondary to Sheehan's syndrome
- Selective IgA deficiency
- Major depressive disorder, recurrent, currently untreated
- History of alcohol use disorder, in early remission

PAST SURGICAL HISTORY:
- Cesarean section complicated by postpartum hemorrhage requiring packed red blood cell transfusion (age 41)
- Colonoscopy with biopsy confirming Crohn's disease (age 42)

MEDICATIONS:
Home Medications:
- Methotrexate 15 mg oral weekly
- Folic acid 1 mg oral daily
- Levothyroxine 75 mcg oral daily
- Cholecalciferol 2000 IU oral daily

ALLERGIES:
Anti-IgA antibodies (anaphylactic risk with blood products), Sulfasalazine (severe gastrointestinal intolerance)

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Father: Essential hypertension
- Sister: Hashimoto's thyroiditis
- Maternal aunt: Crohn's disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former heavy use; currently abstinent for 18 months. Occupation: International public health consultant.

LABORATORY DATA:
Laboratory studies show mild anemia and low serum vitamin D. Sudan Black B stain of the stool is positive for fat.

PATHOLOGY:
Colonoscopy with terminal ileum biopsy reveals patchy areas of inflammation and granuloma formation.

## Encounter 5: source 2024-03-15, practice 2025-02-03, outpatient, Neurology

Chief complaint: Three-month history of persistent nausea, decreased appetite, and burning sensation in hands and feet

ENCOUNTER NOTE
Date: 2024-03-15
Provider: Dr. Lisa Wong, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of persistent nausea, decreased appetite, and burning sensation in hands and feet

HISTORY OF PRESENT ILLNESS:
The patient is a 48-year-old female with seropositive rheumatoid arthritis, ileocolonic Crohn's disease, and history of methotrexate-induced folate deficiency who presents with a 3-month history of persistent nausea, decreased appetite, and a burning sensation in her hands and feet. She was recently started on treatment for a chronic infection.

PAST MEDICAL HISTORY:
Active Problem List:
- Central hypothyroidism (diagnosed 2020-01-15)
- Methotrexate-induced folate deficiency (diagnosed 2020-07-15)
- Pellagra (Niacin deficiency) (diagnosed 2021-03-15)
- Crohn's disease flare with inflammatory stricture (without complete obstruction) (diagnosed 2023-03-15)
- Rheumatoid arthritis, seropositive
- Crohn's disease, ileocolonic
- Central hypothyroidism secondary to Sheehan's syndrome
- Selective IgA deficiency
- Major depressive disorder, recurrent, currently untreated
- History of alcohol use disorder, in early remission

PAST SURGICAL HISTORY:
- Cesarean section complicated by postpartum hemorrhage requiring packed red blood cell transfusion (age 41)
- Colonoscopy with biopsy confirming Crohn's disease (age 42)

MEDICATIONS:
Review of her medications reveals a multi-drug regimen including rifampin, ethambutol, and two other agents.

Home Medications:
- Methotrexate 15 mg oral weekly
- Folic acid 1 mg oral daily
- Levothyroxine 75 mcg oral daily
- Cholecalciferol 2000 IU oral daily

ALLERGIES:
Anti-IgA antibodies (anaphylactic risk with blood products), Sulfasalazine (severe gastrointestinal intolerance)

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Father: Essential hypertension
- Sister: Hashimoto's thyroiditis
- Maternal aunt: Crohn's disease

SOCIAL HISTORY:
She denies alcohol use

PHYSICAL EXAMINATION:
On physical exam, she appears thin and pale, with reduced vibratory sensation and pinprick response in both lower extremities.

LABORATORY DATA:
Laboratory testing shows normocytic anemia and mildly elevated liver enzymes.

## Encounter 6: source 2025-07-15, practice 2026-05-27, ed, Emergency Medicine

Chief complaint: One-week history of fatigue, easy bruising, and persistent epistaxis

ENCOUNTER NOTE
Date: 2025-07-15
Provider: Dr. Robert Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
One-week history of fatigue, easy bruising, and persistent epistaxis

HISTORY OF PRESENT ILLNESS:
The patient has seropositive rheumatoid arthritis and prior methotrexate-induced folate deficiency. She presents to the emergency department with fatigue, easy bruising, and epistaxis that has persisted for the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Central hypothyroidism (diagnosed 2020-01-15)
- Methotrexate-induced folate deficiency (diagnosed 2020-07-15)
- Pellagra (Niacin deficiency) (diagnosed 2021-03-15)
- Crohn's disease flare with inflammatory stricture (without complete obstruction) (diagnosed 2023-03-15)
- Metronidazole-induced polyneuropathy (diagnosed 2024-03-15)
- Rheumatoid arthritis, seropositive
- Crohn's disease, ileocolonic
- Central hypothyroidism secondary to Sheehan's syndrome
- Selective IgA deficiency
- Major depressive disorder, recurrent, currently untreated
- History of alcohol use disorder, in early remission

PAST SURGICAL HISTORY:
- Cesarean section complicated by postpartum hemorrhage requiring packed red blood cell transfusion (age 41)
- Colonoscopy with biopsy confirming Crohn's disease (age 42)

MEDICATIONS:
Home Medications:
- Methotrexate 15 mg oral weekly
- Folic acid 1 mg oral daily
- Levothyroxine 75 mcg oral daily
- Cholecalciferol 2000 IU oral daily

ALLERGIES:
Anti-IgA antibodies (anaphylactic risk with blood products), Sulfasalazine (severe gastrointestinal intolerance)

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Father: Essential hypertension
- Sister: Hashimoto's thyroiditis
- Maternal aunt: Crohn's disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former heavy use; currently abstinent for 18 months. Occupation: International public health consultant.

VITAL SIGNS:
Her temperature is 37.2°C (99°F), blood pressure is 120/75 mmHg, pulse is 95/min, and respiratory rate is 16/min.

PHYSICAL EXAMINATION:
On physical examination, she is pale and has petechiae on her lower extremities.

LABORATORY DATA:
- Hemoglobin: 9.2 g/dL (Normal: 12-16 g/dL)
- White blood cell count: 3.0 x 10^9/L (Normal: 4.5-11 x 10^9/L)
- Platelet count: 20 x 10^9/L (Normal: 150-400 x 10^9/L)
- Prothrombin time (PT): 14 seconds (Normal: 12-15 seconds)
- Activated partial thromboplastin time (aPTT): 35 seconds (Normal: 30-40 seconds)
- Fibrinogen: 100 mg/dL (Normal: 200-400 mg/dL)
- D-dimer: Elevated

OTHER STUDIES:
A peripheral blood smear shows numerous promyelocytes with Auer rods.

## Encounter 7: source 2025-08-15, practice 2026-07-10, icu, Emergency Medicine

Chief complaint: Sudden onset of difficulty breathing and feeling of impending doom ten minutes after red blood cell transfusion

ENCOUNTER NOTE
Date: 2025-08-15
Provider: Dr. Angela Martinez, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Sudden onset of difficulty breathing and feeling of impending doom ten minutes after red blood cell transfusion

HISTORY OF PRESENT ILLNESS:
With selective IgA deficiency and recent APL, she presents to the emergency department 10 minutes after red blood cell transfusion for chronic anemia.

PAST MEDICAL HISTORY:
Active Problem List:
- Central hypothyroidism (diagnosed 2020-01-15)
- Methotrexate-induced folate deficiency (diagnosed 2020-07-15)
- Pellagra (Niacin deficiency) (diagnosed 2021-03-15)
- Crohn's disease flare with inflammatory stricture (without complete obstruction) (diagnosed 2023-03-15)
- Metronidazole-induced polyneuropathy (diagnosed 2024-03-15)
- Acute promyelocytic leukemia (diagnosed 2025-07-15)
- Rheumatoid arthritis, seropositive
- Crohn's disease, ileocolonic
- Central hypothyroidism secondary to Sheehan's syndrome
- Selective IgA deficiency
- Major depressive disorder, recurrent, currently untreated
- History of alcohol use disorder, in early remission

PAST SURGICAL HISTORY:
- Cesarean section complicated by postpartum hemorrhage requiring packed red blood cell transfusion (age 41)
- Colonoscopy with biopsy confirming Crohn's disease (age 42)

MEDICATIONS:
Home Medications:
- Methotrexate 15 mg oral weekly
- Folic acid 1 mg oral daily
- Levothyroxine 75 mcg oral daily
- Cholecalciferol 2000 IU oral daily

ALLERGIES:
Anti-IgA antibodies (anaphylactic risk with blood products), Sulfasalazine (severe gastrointestinal intolerance)

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Father: Essential hypertension
- Sister: Hashimoto's thyroiditis
- Maternal aunt: Crohn's disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former heavy use; currently abstinent for 18 months. Occupation: International public health consultant.

VITAL SIGNS:
Blood pressure is 80/50 mmHg, heart rate is 120 beats per minute, respiratory rate is 28 breaths per minute, and oxygen saturation is 88% on room air

PHYSICAL EXAMINATION:
Widespread urticaria, angioedema, and wheezing bilaterally

PLAN:
The transfusion is immediately stopped.
