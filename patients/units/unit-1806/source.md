# synthetic_hospital patient 1806

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 26 at first encounter, sex F, Non-Hispanic White, Private health insurance (PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-03-07 (Fri) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-02-15 | 2025-04-14 (Mon) | outpatient | Obstetrics & Gynecology |
| 2 | 2020-05-15 | 2025-07-18 (Fri) | outpatient | Obstetrics & Gynecology |
| 3 | 2020-08-15 | 2025-10-05 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private health insurance (PPO)",
 "occupation": "Elementary school teacher",
 "alcohol_use": "None; abstinent during pregnancy",
 "family_history": [
  "Mother: Asthma",
  "Father: Hypertension",
  "Sister: Seasonal allergies"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "90 mcg (2 puffs) PRN for wheezing/shortness of breath",
   "name": "Albuterol HFA inhaler"
  },
  {
   "dose": "1 tablet daily",
   "name": "Prenatal multivitamin with folic acid"
  }
 ],
 "surgical_history": [
  "Wisdom teeth extraction",
  "Tonsillectomy"
 ],
 "chronic_conditions": [
  "Asthma, mild intermittent",
  "Underimmunization status for pertussis"
 ],
 "age_at_first_encounter": 26
}
```

Primary diagnoses: ["Underimmunization status for pertussis vaccine", "Pertussis", "Syphilis complicating pregnancy"]
Comorbidities: ["Pregnancy (20 weeks gestation)"]

## Encounter 0: source 2020-01-15, practice 2025-03-07, outpatient, Obstetrics & Gynecology

Chief complaint: 8 weeks pregnant and requesting guidance on vaccinations during pregnancy, specifically asking whether she should receive the MMR vaccine now to protect herself and her fetus.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
8 weeks pregnant and requesting guidance on vaccinations during pregnancy, specifically asking whether she should receive the MMR vaccine now to protect herself and her fetus.

HISTORY OF PRESENT ILLNESS:
She is 8 weeks pregnant and feels well. She asks whether she should receive the measles, mumps, and rubella (MMR) vaccine now to protect herself and her fetus.

PAST MEDICAL HISTORY:
Active Problem List:
- Asthma, mild intermittent
- Underimmunization status for pertussis

PAST SURGICAL HISTORY:
- Wisdom teeth extraction
- Tonsillectomy

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg (2 puffs) PRN for wheezing/shortness of breath
- Prenatal multivitamin with folic acid 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Asthma
- Father: Hypertension
- Sister: Seasonal allergies

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None; abstinent during pregnancy. Occupation: Elementary school teacher.

LABORATORY DATA:
Serologic testing shows she is non-immune to rubella

## Encounter 1: source 2020-02-15, practice 2025-04-14, outpatient, Obstetrics & Gynecology

Chief complaint: First prenatal visit at 10 weeks gestation with mild nausea controlled with ginger, here for comprehensive obstetric evaluation and ongoing care.

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
First prenatal visit at 10 weeks gestation with mild nausea controlled with ginger, here for comprehensive obstetric evaluation and ongoing care.

HISTORY OF PRESENT ILLNESS:
The patient returns at 10 weeks following vaccination counseling. She reports feeling well with mild nausea controlled with ginger and inquires about rubella vaccine.

PAST MEDICAL HISTORY:
Active Problem List:
- Underimmunization status for pertussis vaccine (diagnosed 2020-01-15)
- Asthma, mild intermittent
- Underimmunization status for pertussis

PAST SURGICAL HISTORY:
- Wisdom teeth extraction
- Tonsillectomy

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg (2 puffs) PRN for wheezing/shortness of breath
- Prenatal multivitamin with folic acid 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Asthma
- Father: Hypertension
- Sister: Seasonal allergies

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None; abstinent during pregnancy. Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
Her physical examination is unremarkable.

LABORATORY DATA:
Her obstetrician orders routine prenatal labs, which include:

- Blood type: A positive
- Hemoglobin: 12.8 g/dL
- Hematocrit: 38.5%
- WBC count: 6,500/mm³
- Platelets: 210,000/mm³
- Rubella IgG antibody: Negative

ASSESSMENT AND PLAN:
The patient's prenatal screening shows that she is not immune to rubella.

## Encounter 2: source 2020-05-15, practice 2025-07-18, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit at 24 weeks gestation reporting occasional mild headaches and a non-itchy rash that appeared on her palms and soles two weeks ago.

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Michael Torres, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit at 24 weeks gestation reporting occasional mild headaches and a non-itchy rash that appeared on her palms and soles two weeks ago.

HISTORY OF PRESENT ILLNESS:
Returning for prenatal care at 24 weeks with asthma and pertussis underimmunization, she reports feeling generally well but mentions occasional mild headaches and a non-itchy rash that appeared on her palms and soles two weeks ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Underimmunization status for pertussis vaccine (diagnosed 2020-01-15)
- Asthma, mild intermittent
- Underimmunization status for pertussis

PAST SURGICAL HISTORY:
- Wisdom teeth extraction
- Tonsillectomy

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg (2 puffs) PRN for wheezing/shortness of breath
- Prenatal multivitamin with folic acid 1 tablet daily

ALLERGIES:
She has no known drug allergies

FAMILY HISTORY:
- Mother: Asthma
- Father: Hypertension
- Sister: Seasonal allergies

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None; abstinent during pregnancy. Occupation: Elementary school teacher.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, you note a macular rash on her palms and soles, and generalized lymphadenopathy.

LABORATORY DATA:
Laboratory tests reveal the following:

- Rapid plasma reagin (RPR): Positive
- Treponema pallidum particle agglutination assay (TP-PA): Positive
- HIV test: Negative
- Complete blood count: Normal
- Liver function tests: Normal

## Encounter 3: source 2020-08-15, practice 2025-10-05, ed, Emergency Medicine

Chief complaint: 3-week history of severe coughing spells worse at night with loud high-pitched whooping sound and occasional cough-induced vomiting.

ENCOUNTER NOTE
Date: 2020-08-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
3-week history of severe coughing spells worse at night with loud high-pitched whooping sound and occasional cough-induced vomiting.

HISTORY OF PRESENT ILLNESS:
The patient is a 26-year-old female with mild intermittent asthma and underimmunization status for pertussis noted during prenatal visits in January and February 2020. She presents with severe coughing spells that are worse at night, stating that the coughing is so intense that it often ends with a loud, high-pitched 'whoop' sound before she can catch her breath. She also mentions occasional cough-induced vomiting. She has not had any fever, but the cough has been disrupting her sleep and is affecting her work performance.

PAST MEDICAL HISTORY:
Active Problem List:
- Underimmunization status for pertussis vaccine (diagnosed 2020-01-15)
- Syphilis complicating pregnancy (diagnosed 2020-05-15)
- Asthma, mild intermittent
- Underimmunization status for pertussis

She has a past medical history of asthma.

PAST SURGICAL HISTORY:
- Wisdom teeth extraction
- Tonsillectomy

MEDICATIONS:
She takes for which she uses an albuterol inhaler as needed.

Home Medications:
- Albuterol HFA inhaler 90 mcg (2 puffs) PRN for wheezing/shortness of breath
- Prenatal multivitamin with folic acid 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Asthma
- Father: Hypertension
- Sister: Seasonal allergies

SOCIAL HISTORY:
She has not traveled recently and is up to date with her vaccinations, including receiving the Tdap vaccine 10 years ago.

PHYSICAL EXAMINATION:
On physical examination, she appears fatigued. Lung auscultation reveals no wheezing or crackles, and the rest of the examination is unremarkable.

LABORATORY DATA:
Her white blood cell count is 15,000/µL with 80% lymphocytes.
