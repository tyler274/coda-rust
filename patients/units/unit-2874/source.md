# synthetic_hospital patient 2874

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 63 at first encounter, sex F, Non-Hispanic White, Medicare (disability).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-10-11 (Wed) | outpatient | Obstetrics & Gynecology |
| 1 | 2022-01-15 | 2025-09-29 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Sulfa drugs (rash)",
  "Latex (contact dermatitis)"
 ],
 "insurance": "Medicare (disability)",
 "occupation": "Retired factory worker",
 "alcohol_use": "None",
 "family_history": [
  "Mother: cervical cancer (deceased age 67)",
  "Father: lung cancer (deceased age 71)",
  "Sister: breast cancer (age 55)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 42 pack-year history, contemplation stage of behavior change (acknowledges smoking-related harm but expresses ambivalence about quitting)",
 "home_medications": [
  {
   "dose": "250/50 mcg inhaled twice daily",
   "name": "Fluticasone/salmeterol"
  },
  {
   "dose": "18 mcg inhaled once daily",
   "name": "Tiotropium"
  },
  {
   "dose": "90 mcg inhaled as needed for shortness of breath",
   "name": "Albuterol"
  }
 ],
 "surgical_history": [
  "Loop electrosurgical excision procedure (LEEP) for cervical dysplasia (age 58)",
  "Tubal ligation (age 32)"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (COPD)",
  "High-risk human papillomavirus (HPV) infection"
 ],
 "age_at_first_encounter": 63
}
```

Primary diagnoses: ["Invasive vaginal squamous cell carcinoma"]
Comorbidities: ["Tobacco use disorder"]

## Encounter 0: source 2020-01-15, practice 2023-10-11, outpatient, Obstetrics & Gynecology

Chief complaint: 3-month history of intermittent vaginal bleeding and foul-smelling vaginal discharge

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Patel, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of intermittent vaginal bleeding and foul-smelling vaginal discharge

HISTORY OF PRESENT ILLNESS:
She reports no significant pain but feels a sense of discomfort in the pelvic area.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (COPD)
- High-risk human papillomavirus (HPV) infection

Her past medical history is significant for chronic tobacco use and a high-risk strain of human papillomavirus (HPV) infection diagnosed 5 years ago.

PAST SURGICAL HISTORY:
- Loop electrosurgical excision procedure (LEEP) for cervical dysplasia (age 58)
- Tubal ligation (age 32)

MEDICATIONS:
Home Medications:
- Fluticasone/salmeterol 250/50 mcg inhaled twice daily
- Tiotropium 18 mcg inhaled once daily
- Albuterol 90 mcg inhaled as needed for shortness of breath

ALLERGIES:
Sulfa drugs (rash), Latex (contact dermatitis)

FAMILY HISTORY:
She has no family history of gynecologic cancers.

SOCIAL HISTORY:
Smoking: Current smoker, 42 pack-year history, contemplation stage of behavior change (acknowledges smoking-related harm but expresses ambivalence about quitting). Alcohol: None. Occupation: Retired factory worker.

PHYSICAL EXAMINATION:
On physical examination, an irregular, ulcerated lesion is noted on the posterior wall of the vagina.

PATHOLOGY:
A biopsy of the lesion is performed, and the pathology report indicates atypical squamous cells with invasion into the subepithelial connective tissue.

## Encounter 1: source 2022-01-15, practice 2025-09-29, ed, Emergency Medicine

Chief complaint: Acute exacerbation of COPD with increased dyspnea

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute exacerbation of COPD with increased dyspnea

HISTORY OF PRESENT ILLNESS:
The patient is a 65-year-old female with chronic obstructive pulmonary disease and high-risk human papillomavirus infection who was previously diagnosed with invasive vaginal squamous cell carcinoma in January 2020. She presents to the emergency department with an exacerbation of COPD.

PAST MEDICAL HISTORY:
Active Problem List:
- Invasive vaginal squamous cell carcinoma (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (COPD)
- High-risk human papillomavirus (HPV) infection

PAST SURGICAL HISTORY:
- Loop electrosurgical excision procedure (LEEP) for cervical dysplasia (age 58)
- Tubal ligation (age 32)

MEDICATIONS:
Home Medications:
- Fluticasone/salmeterol 250/50 mcg inhaled twice daily
- Tiotropium 18 mcg inhaled once daily
- Albuterol 90 mcg inhaled as needed for shortness of breath

ALLERGIES:
Sulfa drugs (rash), Latex (contact dermatitis)

FAMILY HISTORY:
- Mother: cervical cancer (deceased age 67)
- Father: lung cancer (deceased age 71)
- Sister: breast cancer (age 55)

SOCIAL HISTORY:
The patient is a long-term smoker. After the patient is ready to be discharged, she acknowledges that her smoking has caused problems but says that it is probably too late now to quit anyway.
