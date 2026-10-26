# synthetic_hospital patient 1819

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 35 at first encounter, sex F, Hispanic, Blue Cross PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-04-11 (Tue) | outpatient | Rheumatology |
| 1 | 2020-10-15 | 2024-01-10 (Wed) | outpatient | Obstetrics & Gynecology |
| 2 | 2022-07-15 | 2025-10-11 (Sat) | inpatient | Hematology/Oncology |
| 3 | 2023-01-15 | 2026-04-27 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "None",
 "family_history": [
  "Mother: rheumatoid arthritis",
  "Maternal aunt: systemic lupus erythematosus",
  "Father: hypertension"
 ],
 "race_ethnicity": "Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "200 mg oral daily",
   "name": "Hydroxychloroquine"
  },
  {
   "dose": "1 mg oral daily",
   "name": "Folic acid"
  },
  {
   "dose": "2000 units oral daily",
   "name": "Vitamin D3"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Systemic lupus erythematosus",
  "Hypergonadotropic hypogonadism",
  "History of acute promyelocytic leukemia in complete remission"
 ],
 "age_at_first_encounter": 35
}
```

Primary diagnoses: ["Systemic lupus erythematosus", "Medication-induced psychosis (dopaminergic therapy)", "Acute Promyelocytic Leukemia (ATRA therapy mechanism)", "Hypergonadotropic hypogonadism"]
Comorbidities: ["Thrombocytopenia", "Family History of Hereditary Hemorrhagic Telangiectasia"]

## Encounter 0: source 2020-01-15, practice 2023-04-11, outpatient, Rheumatology

Chief complaint: Six-month history of fatigue, joint pain, and occasional facial rashes after sun exposure

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of fatigue, joint pain, and occasional facial rashes after sun exposure

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with a 6-month history of fatigue, joint pain, and occasional rashes that appear on her face after sun exposure.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus
- Hypergonadotropic hypogonadism
- History of acute promyelocytic leukemia in complete remission

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg oral daily
- Folic acid 1 mg oral daily
- Vitamin D3 2000 units oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Mentions that her mother had an autoimmune condition.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, she has a malar rash and mild synovitis in her wrists and knees.

LABORATORY DATA:
Laboratory tests reveal the following:

- Hemoglobin: 10.5 g/dL (Normal: 12-16 g/dL)
- White blood cell count: 3,500 /µL (Normal: 4,500-11,000 /µL)
- Platelet count: 100,000 /µL (Normal: 150,000-450,000 /µL)
- Antinuclear antibody (ANA) titer: Positive at 1:640
- Anti-double-stranded DNA (anti-dsDNA): Positive
- Complement levels (C3, C4): Low

OTHER STUDIES:
The patient is part of a clinical trial comparing the efficacy of three new drugs for the treatment of systemic lupus erythematosus (SLE). The primary outcome is the reduction in the SLE Disease Activity Index (SLEDAI) score after 6 months of treatment. The results show nonoverlapping 95% confidence intervals for the mean reduction in SLEDAI scores between the three drugs. Based on the educational objective,

## Encounter 1: source 2020-10-15, practice 2024-01-10, outpatient, Obstetrics & Gynecology

Chief complaint: Eighteen months of unsuccessful attempts to conceive despite regular 28-day menstrual cycles

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Eighteen months of unsuccessful attempts to conceive despite regular 28-day menstrual cycles

HISTORY OF PRESENT ILLNESS:
The patient with systemic lupus erythematosus and a history of acute promyelocytic leukemia in remission presents to obstetrics and gynecology. Her menstrual cycles are regular every 28 days.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Hypergonadotropic hypogonadism
- History of acute promyelocytic leukemia in complete remission

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg oral daily
- Folic acid 1 mg oral daily
- Vitamin D3 2000 units oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Her mother experienced menopause at age 38

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
Laboratory studies reveal normal TSH and prolactin levels, low-normal estradiol, and low anti-Müllerian hormone.

ASSESSMENT AND PLAN:
Further evaluation demonstrates diminished ovarian reserve.

## Encounter 2: source 2022-07-15, practice 2025-10-11, inpatient, Hematology/Oncology

Chief complaint: One-month history of persistent fatigue, intermittent headaches, easy bruising, and mild gum bleeding

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Michael Thompson, MD
Department: Hematology/Oncology
Visit Type: inpatient

CHIEF COMPLAINT:
One-month history of persistent fatigue, intermittent headaches, easy bruising, and mild gum bleeding

HISTORY OF PRESENT ILLNESS:
The patient with history of APL in remission and SLE presents with persistent fatigue and intermittent headaches over the past month. She reports occasional easy bruising and mild gum bleeding, which she attributes to stress and poor sleep.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Hypergonadotropic hypogonadism (diagnosed 2020-10-15)
- History of acute promyelocytic leukemia in complete remission

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg oral daily
- Folic acid 1 mg oral daily
- Vitamin D3 2000 units oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: rheumatoid arthritis
- Maternal aunt: systemic lupus erythematosus
- Father: hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

VITAL SIGNS:
Vital signs are normal

PHYSICAL EXAMINATION:
Physical exam reveals mild pallor and scattered petechiae over her lower extremities

LABORATORY DATA:
Laboratory studies show hemoglobin 8.4 g/dL, platelets 34,000/mm³, and WBC count 2,600/mm³

PATHOLOGY:
Bone marrow biopsy confirms a diagnosis consistent with a t(15;17) translocation

OTHER STUDIES:
A peripheral smear is shown

PLAN:
She is started on a drug that induces differentiation of the malignant cells through interaction with a nuclear receptor

## Encounter 3: source 2023-01-15, practice 2026-04-27, ed, Emergency Medicine

Chief complaint: Two-day history of auditory hallucinations and paranoid delusions

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-day history of auditory hallucinations and paranoid delusions

HISTORY OF PRESENT ILLNESS:
This 38-year-old female with systemic lupus erythematosus, hypergonadotropic hypogonadism, and acute promyelocytic leukemia in remission presents with new psychiatric symptoms. She reports hearing voices telling her that she is being followed and that her family is in danger.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Hypergonadotropic hypogonadism (diagnosed 2020-10-15)
- Acute Promyelocytic Leukemia (ATRA therapy mechanism) (diagnosed 2022-07-15)
- History of acute promyelocytic leukemia in complete remission

She has no past psychiatric history. Her medical history is significant for systemic lupus erythematosus (SLE), for which she was recently started on high-dose prednisone therapy due to a severe flare.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg oral daily
- Folic acid 1 mg oral daily
- Vitamin D3 2000 units oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: rheumatoid arthritis
- Maternal aunt: systemic lupus erythematosus
- Father: hypertension

SOCIAL HISTORY:
She denies any recent stressors, illicit drug use, or alcohol abuse

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, she is agitated, appears to be responding to internal stimuli, and has no focal neurological deficits.

LABORATORY DATA:
Laboratory studies show:

- Sodium: 140 mEq/L
- Potassium: 4.1 mEq/L
- Chloride: 102 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen (BUN): 18 mg/dL
- Creatinine: 0.9 mg/dL
- Glucose: 98 mg/dL
- Calcium: 9.2 mg/dL
- Complete blood count (CBC): within normal limits
- Urine toxicology screen: Negative
