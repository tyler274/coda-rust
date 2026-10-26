# synthetic_hospital patient 2875

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 70 at first encounter, sex F, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-10-23 (Wed) | outpatient | Pulmonary and Critical Care Medicine |
| 1 | 2021-01-15 | 2025-10-20 (Mon) | outpatient | Pulmonary and Critical Care Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare",
 "occupation": "Retired elementary school teacher",
 "alcohol_use": "None",
 "family_history": [
  "Mother: osteoporosis",
  "Father: hypertension",
  "Maternal aunt: hip fracture at age 78"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [
  "Mild osteopenia"
 ],
 "age_at_first_encounter": 70
}
```

Primary diagnoses: ["Age-related cognitive decline", "FEV1 as predictor of postoperative outcomes"]
Comorbidities: ["Osteopenia requiring pharmacologic therapy"]

## Encounter 0: source 2020-01-15, practice 2024-10-23, outpatient, Pulmonary and Critical Care Medicine

Chief complaint: Participation in voluntary aging research study examining pulmonary function across the lifespan

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonary and Critical Care Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Participation in voluntary aging research study examining pulmonary function across the lifespan

HISTORY OF PRESENT ILLNESS:
She is in excellent health. She has not noticed any recent changes in her exercise tolerance or stamina. The patient is surprised by the results and expresses concern that she might have early lung disease. She asks what the results mean and whether she needs further testing or treatment.

PAST MEDICAL HISTORY:
Active Problem List:
- Mild osteopenia

She has no underlying pulmonary conditions. She has never been hospitalized for respiratory problems.

MEDICATIONS:
She is not on any prescription medications

ALLERGIES:
She reports no allergies.

FAMILY HISTORY:
She has no known family history of lung disease.

SOCIAL HISTORY:
She has never smoked, never worked in an industrial or hazardous environment, and has no history of secondhand smoke exposure. She lives independently, prepares her own meals, and exercises daily by walking 2–3 miles and tending her garden.

REVIEW OF SYSTEMS:
She denies shortness of breath, chest tightness, wheezing, cough, sputum production, or recent respiratory infections. She has never experienced seasonal allergies or asthma. She sleeps comfortably through the night and does not use pillows to elevate her head. She has no history of snoring, witnessed apneas, or early morning headaches.

VITAL SIGNS:
Vital signs are normal with oxygen saturation of 98% on room air. BMI is 22 kg/m².

PHYSICAL EXAMINATION:
On examination, she is alert and cooperative. Her lungs are clear to auscultation bilaterally with normal vesicular breath sounds and no wheezing, rales, or rhonchi. Cardiac examination reveals a regular rate and rhythm without murmurs or gallops. There is no peripheral edema or clubbing. Neurologic and musculoskeletal exams are unremarkable.

OTHER STUDIES:
As part of the research protocol, she undergoes spirometry. The results reveal a decrease in her forced expiratory volume in 1 second (FEV₁), a decrease in forced vital capacity (FVC), and a reduced FEV₁/FVC ratio, compared to those of a healthy 20-year-old female. Repeat spirometry a week later confirms the findings.

## Encounter 1: source 2021-01-15, practice 2025-10-20, outpatient, Pulmonary and Critical Care Medicine

Chief complaint: Follow-up participation in pulmonary research study involving comprehensive lung function testing

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Okafor, MD
Department: Pulmonary and Critical Care Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up participation in pulmonary research study involving comprehensive lung function testing

HISTORY OF PRESENT ILLNESS:
She returns for research follow-up with mild osteopenia. She reports feeling well and leads an active lifestyle, walking several miles daily and gardening regularly.

PAST MEDICAL HISTORY:
Active Problem List:
- FEV1 as predictor of postoperative outcomes (diagnosed 2020-01-15)
- Mild osteopenia

MEDICATIONS:
She takes no medications

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: osteoporosis
- Father: hypertension
- Maternal aunt: hip fracture at age 78

SOCIAL HISTORY:
Never smoked

REVIEW OF SYSTEMS:
She does not report any shortness of breath, chest pain, wheezing, or chronic cough.

VITAL SIGNS:
BMI is 23 kg/m²

PHYSICAL EXAMINATION:
On physical examination, her lungs are clear to auscultation without rales, rhonchi, or wheezes. Heart sounds are normal, and there is no peripheral edema.

OTHER STUDIES:
The study compares her pulmonary function to that of a 20-year-old healthy female control. Although both women are healthy and asymptomatic, pulmonary function testing reveals notable differences between them.
