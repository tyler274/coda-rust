# synthetic_hospital patient 1835

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-06-02 (Fri) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2023-11-27 (Mon) | outpatient | Endocrinology |
| 2 | 2021-01-15 | 2024-05-20 (Mon) | outpatient | Endocrinology |
| 3 | 2022-07-15 | 2025-11-19 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Penicillin",
  "Shellfish"
 ],
 "insurance": "Medicaid",
 "occupation": "Part-time administrative assistant",
 "alcohol_use": "None",
 "family_history": [
  "Mother: major depressive disorder",
  "Father: substance use disorder (alcohol)",
  "Brother: schizophrenia"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 1 year ago",
 "home_medications": [
  {
   "dose": "4 mg PO daily",
   "name": "risperidone"
  },
  {
   "dose": "85 mg PO daily",
   "name": "methadone"
  }
 ],
 "surgical_history": [
  "Cesarean section (remote)",
  "Dental extraction"
 ],
 "chronic_conditions": [
  "Schizophrenia",
  "Opioid use disorder, in sustained remission on maintenance therapy"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Hyperprolactinemia", "Tricuspid Regurgitation"]
Comorbidities: ["Schizophrenia"]

## Encounter 0: source 2020-01-15, practice 2023-06-02, outpatient, Internal Medicine

Chief complaint: Irregular menstrual cycles and milky nipple discharge for 6 months.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Irregular menstrual cycles and milky nipple discharge for 6 months.

HISTORY OF PRESENT ILLNESS:
6-month history of irregular menstrual cycles and a recent onset of milky nipple discharge. She denies any recent pregnancies, breast pain, or changes in medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia
- Opioid use disorder, in sustained remission on maintenance therapy

Schizophrenia, which has been well-controlled.

PAST SURGICAL HISTORY:
- Cesarean section (remote)
- Dental extraction

MEDICATIONS:
She takes risperidone for the past two years.

Home Medications:
- methadone 85 mg PO daily

ALLERGIES:
Penicillin, Shellfish

FAMILY HISTORY:
- Mother: major depressive disorder
- Father: substance use disorder (alcohol)
- Brother: schizophrenia

SOCIAL HISTORY:
Smoking: Former smoker, quit 1 year ago. Alcohol: None. Occupation: Part-time administrative assistant.

VITAL SIGNS:
Vital signs are within normal limits

PHYSICAL EXAMINATION:
No palpable breast mass. Neurological examination is unremarkable.

LABORATORY DATA:
Serum prolactin: 85 ng/mL (Normal: 4-23 ng/mL)
TSH: 2.5 mIU/L (Normal: 0.4-4.0 mIU/L)
FSH: 7.2 mIU/mL (Normal: 3.5-12.5 mIU/mL)
LH: 6.1 mIU/mL (Normal: 2.4-12.6 mIU/mL)
Beta-hCG: <5 mIU/mL (Negative)

## Encounter 1: source 2020-07-15, practice 2023-11-27, outpatient, Endocrinology

Chief complaint: 3-month history of irregular menstrual cycles and a recent onset of milky nipple discharge.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Amara Okafor, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of irregular menstrual cycles and a recent onset of milky nipple discharge.

HISTORY OF PRESENT ILLNESS:
The patient is a 28-year-old female with schizophrenia and opioid use disorder in sustained remission on risperidone and methadone maintenance who returns for evaluation of recurrent galactorrhea and menstrual irregularity following a prior Internal Medicine visit in January 2020. She denies any headache, visual changes, or history of childbirth.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperprolactinemia (diagnosed 2020-01-15)
- Schizophrenia
- Opioid use disorder, in sustained remission on maintenance therapy

She has been treated for schizophrenia for the past 2 years and recently had her medication adjusted due to persistent auditory hallucinations.

PAST SURGICAL HISTORY:
- Cesarean section (remote)
- Dental extraction

MEDICATIONS:
Her current medication includes risperidone 4 mg daily.

Home Medications:
- methadone 85 mg PO daily

ALLERGIES:
Penicillin, Shellfish

FAMILY HISTORY:
- Mother: major depressive disorder
- Father: substance use disorder (alcohol)
- Brother: schizophrenia

SOCIAL HISTORY:
Smoking: Former smoker, quit 1 year ago. Alcohol: None. Occupation: Part-time administrative assistant.

VITAL SIGNS:
Her vital signs are within normal limits

PHYSICAL EXAMINATION:
On physical examination, no abnormalities are noted except for the presence of galactorrhea

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum prolactin: 85 ng/mL (Normal: 4-23 ng/mL)
- TSH: 2.5 mIU/L (Normal: 0.4-4.0 mIU/L)
- FSH: 7.2 mIU/mL (Normal: 3.5-12.5 mIU/mL)
- LH: 6.8 mIU/mL (Normal: 2.4-12.6 mIU/mL)

## Encounter 2: source 2021-01-15, practice 2024-05-20, outpatient, Endocrinology

Chief complaint: Persistent headaches and spontaneous milky discharge from both breasts for the past 5 months.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Amara Okafor, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent headaches and spontaneous milky discharge from both breasts for the past 5 months.

HISTORY OF PRESENT ILLNESS:
The patient returns to Endocrinology following assessments in January and July 2020, reporting persistent headaches and spontaneous milky discharge from both breasts for the past 5 months. Her last pregnancy was over 2 years ago, and she breastfed for 8 months. She denies visual disturbances, hot flashes, or signs of estrogen deficiency. Her menstrual cycles are regular, occurring every 28–30 days.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperprolactinemia (diagnosed 2020-01-15)
- Schizophrenia
- Opioid use disorder, in sustained remission on maintenance therapy

PAST SURGICAL HISTORY:
- Cesarean section (remote)
- Dental extraction

MEDICATIONS:
She denies any use of medications.

Home Medications:
- risperidone 4 mg PO daily
- methadone 85 mg PO daily

ALLERGIES:
Penicillin, Shellfish

FAMILY HISTORY:
- Mother: major depressive disorder
- Father: substance use disorder (alcohol)
- Brother: schizophrenia

SOCIAL HISTORY:
She denies any use of illicit substances

VITAL SIGNS:
Vital signs are normal.

PHYSICAL EXAMINATION:
Physical examination reveals bilateral galactorrhea without masses or skin changes. Visual field testing is full to confrontation.

## Encounter 3: source 2022-07-15, practice 2025-11-19, ed, Emergency Medicine

Chief complaint: 2-week history of puffiness of the legs, abdominal pain, and a feeling of 'wind in her chest.'

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-week history of puffiness of the legs, abdominal pain, and a feeling of 'wind in her chest.'

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperprolactinemia (diagnosed 2020-01-15)
- Schizophrenia
- Opioid use disorder, in sustained remission on maintenance therapy

She has a history of schizophrenia and is enrolled in the community methadone program.

PAST SURGICAL HISTORY:
- Cesarean section (remote)
- Dental extraction

MEDICATIONS:
Home Medications:
- risperidone 4 mg PO daily
- methadone 85 mg PO daily

ALLERGIES:
Penicillin, Shellfish

FAMILY HISTORY:
- Mother: major depressive disorder
- Father: substance use disorder (alcohol)
- Brother: schizophrenia

SOCIAL HISTORY:
Smoking: Former smoker, quit 1 year ago. Alcohol: None. Occupation: Part-time administrative assistant.

PHYSICAL EXAMINATION:
Physical examination shows a pulsatile liver, JVD, mild peripheral edema, and a systolic murmur that increases with inspiration. There is oscillation of the second heart sound with the respiratory cycle.
