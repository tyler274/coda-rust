# synthetic_hospital patient 2653

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 34 at first encounter, sex M, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-10-13 (Fri) | outpatient | Pulmonology |
| 1 | 2021-07-15 | 2025-03-27 (Thu) | ed | Emergency Medicine |
| 2 | 2022-01-15 | 2025-10-06 (Mon) | outpatient | Occupational Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Dust mites",
  "Seasonal pollen"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "High school shop teacher and part-time warehouse stocker",
 "alcohol_use": "Occasional social use (1-2 drinks per week)",
 "family_history": [
  "Maternal asthma",
  "Paternal allergic rhinitis",
  "Paternal hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "90 mcg per actuation, 2 puffs every 4-6 hours as needed for wheezing or shortness of breath",
   "name": "Albuterol HFA inhaler"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Asthma, mild persistent (childhood onset)"
 ],
 "age_at_first_encounter": 34
}
```

Primary diagnoses: ["Influenza", "Mild persistent asthma"]
Comorbidities: ["Occupational exposure to hazardous substances"]

## Encounter 0: source 2020-01-15, practice 2023-10-13, outpatient, Pulmonology

Chief complaint: Follow-up visit for worsening asthma control with increased shortness of breath and nocturnal symptoms

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit for worsening asthma control with increased shortness of breath and nocturnal symptoms

HISTORY OF PRESENT ILLNESS:
He reports having asthma since childhood, which he has managed solely with an albuterol inhaler that he uses “a few times a week,” mostly when he’s exercising or exposed to cold air. Over the past two months, however, he has noticed that he’s using his inhaler almost daily and is waking up 1–2 nights per week with coughing and wheezing. He denies recent infections or hospitalizations but admits that he hasn’t seen a doctor in over a year.

PAST MEDICAL HISTORY:
Active Problem List:
- Asthma, mild persistent (childhood onset)

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg per actuation, 2 puffs every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
Dust mites, Seasonal pollen

FAMILY HISTORY:
- Maternal asthma
- Paternal allergic rhinitis
- Paternal hypertension

SOCIAL HISTORY:
He works as a warehouse stocker and is often exposed to dust and fumes. He has no history of smoking,

VITAL SIGNS:
His BMI is 24.

PHYSICAL EXAMINATION:
Physical examination reveals mild expiratory wheezing but is otherwise normal.

OTHER STUDIES:
His peak expiratory flow is 80% of predicted. Pulmonary function testing done 6 months ago showed reversible obstruction.

ASSESSMENT AND PLAN:
The physician determines that the patient is not currently in an acute exacerbation and is well enough for outpatient management. However, his current symptoms suggest a change in asthma severity.

## Encounter 1: source 2021-07-15, practice 2025-03-27, ed, Emergency Medicine

Chief complaint: Sudden onset fever, myalgias, headache, and dry cough for 48 hours during peak influenza season

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset fever, myalgias, headache, and dry cough for 48 hours during peak influenza season

HISTORY OF PRESENT ILLNESS:
This 35-year-old male with childhood-onset mild persistent asthma, last evaluated in Pulmonology on 2020-01-15 for worsening control, presents with acute infectious symptoms. He reports feeling "run down" and has had a low-grade fever at home, measured at 38.2°C (100.8°F). He works as a school teacher and notes that several of his students have been absent due to similar symptoms. Influenza is circulating in the community, and it is currently peak influenza season.

PAST MEDICAL HISTORY:
Active Problem List:
- Mild persistent asthma (diagnosed 2020-01-15)
- Asthma, mild persistent (childhood onset)

He denies any significant past medical history.

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg per actuation, 2 puffs every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
Dust mites, Seasonal pollen

FAMILY HISTORY:
- Maternal asthma
- Paternal allergic rhinitis
- Paternal hypertension

SOCIAL HISTORY:
Smoking: Never. Alcohol: Occasional social use (1-2 drinks per week). Occupation: High school shop teacher and part-time warehouse stocker.

VITAL SIGNS:
Temperature is 38.3°C (100.9°F), blood pressure is 120/78 mmHg, pulse is 88/min, and respiratory rate is 16/min

PHYSICAL EXAMINATION:
His throat is mildly erythematous without exudates, and his lungs are clear to auscultation. No other abnormalities are noted.

## Encounter 2: source 2022-01-15, practice 2025-10-06, outpatient, Occupational Medicine

Chief complaint: Mandatory occupational health visit for infection prevention education following workplace influenza outbreak

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Priya Patel, MD
Department: Occupational Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Mandatory occupational health visit for infection prevention education following workplace influenza outbreak

HISTORY OF PRESENT ILLNESS:
The patient is a 36-year-old male with mild persistent asthma since childhood and prior influenza in July 2021 who presents for mandatory occupational health education after several staff members in the unit contract influenza within a 10-day period. During the session, the instructor emphasizes respiratory hygiene as a key element of infection control, especially during peak flu season. The technician asks what he should do if he suddenly feels the urge to sneeze while transporting sterile equipment and does not have a tissue immediately available.

PAST MEDICAL HISTORY:
Active Problem List:
- Mild persistent asthma (diagnosed 2020-01-15)
- Influenza (diagnosed 2021-07-15)
- Asthma, mild persistent (childhood onset)

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg per actuation, 2 puffs every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
Dust mites, Seasonal pollen

FAMILY HISTORY:
- Maternal asthma
- Paternal allergic rhinitis
- Paternal hypertension

SOCIAL HISTORY:
Smoking: Never. Alcohol: Occasional social use (1-2 drinks per week). Occupation: High school shop teacher and part-time warehouse stocker.
