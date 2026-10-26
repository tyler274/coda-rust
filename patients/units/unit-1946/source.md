# synthetic_hospital patient 1946

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, Non-Hispanic White, Employer-sponsored PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-02-17 (Mon) | outpatient | Rheumatology |
| 1 | 2021-01-15 | 2026-02-12 (Thu) | outpatient | Cardiology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Employer-sponsored PPO",
 "occupation": "Marketing Coordinator",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Mother with Raynaud's phenomenon",
  "Maternal aunt with systemic lupus erythematosus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Chronic vasospastic disorder (Raynaud's phenomenon/Buerger's disease)"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2025-02-17, outpatient, Rheumatology

Chief complaint: Episodic numbness and color changes in her fingers occurring several times weekly, triggered by cold weather and work-related stress.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Episodic numbness and color changes in her fingers occurring several times weekly, triggered by cold weather and work-related stress.

HISTORY OF PRESENT ILLNESS:
She notes that these episodes occur several times a week and are often triggered by exposure to cold weather or during stressful situations at work. She describes the color changes as a sequence of white to blue, then red upon rewarming or relief of stress. She denies any ulcers or gangrene on her fingers.

PAST MEDICAL HISTORY:
Her past medical history is unremarkable

MEDICATIONS:
She is not currently taking any medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with Raynaud's phenomenon
- Maternal aunt with systemic lupus erythematosus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Marketing Coordinator.

VITAL SIGNS:
On physical examination, her vital signs are within normal limits

PHYSICAL EXAMINATION:
Examination of her hands shows no skin breakdown or ulceration, and her nailfold capillary examination is normal. There is no evidence of sclerodactyly or telangiectasias.

## Encounter 1: source 2021-01-15, practice 2026-02-12, outpatient, Cardiology

Chief complaint: Intermittent finger discomfort with cold exposure characterized by pallor, cyanosis, and subsequent erythema upon rewarming over the past year.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. David Okonkwo, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent finger discomfort with cold exposure characterized by pallor, cyanosis, and subsequent erythema upon rewarming over the past year.

HISTORY OF PRESENT ILLNESS:
The patient returns to the clinic one year after her initial Rheumatology evaluation for episodic digital color changes and numbness. She presents to the Cardiology clinic during the winter complaining of intermittent finger discomfort over the past year. She reports that when exposed to cold temperatures, her fingers first turn pale and cold, then bluish, and finally red and tingling upon rewarming. The episodes resolve spontaneously within minutes.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic vasospastic disorder (Raynaud's phenomenon/Buerger's disease) (diagnosed 2020-01-15)

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is unremarkable

SOCIAL HISTORY:
She does not smoke

REVIEW OF SYSTEMS:
She denies joint pain, rash, or difficulty swallowing.

VITAL SIGNS:
Her temperature is 36.9°C (98.4°F), blood pressure is 114/72 mm Hg, pulse is 76/min, and respiratory rate is 14/min.

PHYSICAL EXAMINATION:
Physical examination is normal

LABORATORY DATA:
Laboratory testing including antinuclear antibody (ANA) and rheumatoid factor is negative
