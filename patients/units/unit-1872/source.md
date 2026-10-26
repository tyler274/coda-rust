# synthetic_hospital patient 1872

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 25 at first encounter, sex F, Black/African American, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2026-04-23 (Thu) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-04-15 | 2026-07-28 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Sulfamethoxazole (anaphylaxis risk)"
 ],
 "insurance": "Medicaid",
 "occupation": "Administrative assistant",
 "alcohol_use": "None",
 "family_history": [
  "Sickle cell disease in younger brother",
  "Sickle cell trait in mother",
  "Sickle cell trait in father",
  "Paternal aunt with systemic lupus erythematosus"
 ],
 "race_ethnicity": "Black/African American",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "1 mg PO daily",
   "name": "Folic acid"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Prenatal vitamin"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Sickle cell disease (HbSS genotype)",
  "HIV infection (diagnosed 12 months prior)"
 ],
 "age_at_first_encounter": 25
}
```

Primary diagnoses: ["HIV infection in pregnancy", "Sickle cell disease with vaso-occlusive crisis"]
Comorbidities: ["Pregnancy (28 weeks gestation)"]

## Encounter 0: source 2020-01-15, practice 2026-04-23, outpatient, Obstetrics & Gynecology

Chief complaint: presents to the prenatal clinic for a routine follow-up. She is concerned about the health of her unborn child and inquires about the risk of transmission of HIV to the baby.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Aisha Johnson, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
She presents to the prenatal clinic for a routine follow-up. She is concerned about the health of her unborn child and inquires about the risk of transmission of HIV to the baby.

HISTORY OF PRESENT ILLNESS:
She is concerned about the health of her unborn child and inquires about the risk of transmission of HIV to the baby.

PAST MEDICAL HISTORY:
Active Problem List:
- Sickle cell disease (HbSS genotype)
- HIV infection (diagnosed 12 months prior)

She was diagnosed with HIV 1 year ago and has not been on antiretroviral therapy (ART). Her medical history is otherwise unremarkable.

MEDICATIONS:
Home Medications:
- Folic acid 1 mg PO daily
- Prenatal vitamin 1 tablet PO daily

ALLERGIES:
Sulfamethoxazole (anaphylaxis risk)

FAMILY HISTORY:
- Sickle cell disease in younger brother
- Sickle cell trait in mother
- Sickle cell trait in father
- Paternal aunt with systemic lupus erythematosus

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Administrative assistant.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
Laboratory tests reveal the following:

- HIV RNA viral load: 15,000 copies/mL
- CD4 count: 450 cells/mm³
- Complete blood count: within normal limits
- Hepatitis B and C serologies: negative

## Encounter 1: source 2020-04-15, practice 2026-07-28, ed, Emergency Medicine

Chief complaint: presents to the emergency department with a 6-hour history of severe, sharp, and constant abdominal pain

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
She presents to the emergency department with a 6-hour history of severe, sharp, and constant abdominal pain

HISTORY OF PRESENT ILLNESS:
The patient with sickle cell disease (HbSS) and HIV infection diagnosed 12 months prior presents to the emergency department three months after her prenatal clinic visit. She denies vaginal bleeding or discharge. Her pregnancy has been complicated by two prior acute chest syndrome episodes requiring hospitalization and blood transfusions. Her pain does not improve with hydration and bed rest.

PAST MEDICAL HISTORY:
Active Problem List:
- HIV infection in pregnancy (diagnosed 2020-01-15)
- Sickle cell disease (HbSS genotype)
- HIV infection (diagnosed 12 months prior)

She has a known history of sickle cell disease and has been compliant with her prenatal care.

MEDICATIONS:
Home Medications:
- Folic acid 1 mg PO daily
- Prenatal vitamin 1 tablet PO daily

ALLERGIES:
Sulfamethoxazole (anaphylaxis risk)

FAMILY HISTORY:
- Sickle cell disease in younger brother
- Sickle cell trait in mother
- Sickle cell trait in father
- Paternal aunt with systemic lupus erythematosus

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Administrative assistant.

VITAL SIGNS:
On examination, her temperature is 37.8°C (100°F), blood pressure is 130/85 mmHg, heart rate is 105 beats per minute, and respiratory rate is 22 beats per minute.

PHYSICAL EXAMINATION:
Abdominal examination reveals diffuse tenderness, but no rebound or guarding. Fetal heart tones are normal.

LABORATORY DATA:
Laboratory studies show:

- Hemoglobin: 8.2 g/dL (baseline 9.5 g/dL)
- Leukocyte count: 15,000/mm³
- Platelet count: 350,000/mm³
- Lactate dehydrogenase (LDH): Elevated
- Indirect bilirubin: Elevated
- Haptoglobin: Decreased
- Urinalysis: No abnormalities
