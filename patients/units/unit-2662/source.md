# synthetic_hospital patient 2662

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 23 at first encounter, sex M, Non-Hispanic White, University student health insurance plan.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-01-07 (Tue) | outpatient | Physical Medicine & Rehabilitation |
| 1 | 2021-01-15 | 2026-01-22 (Thu) | inpatient | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "University student health insurance plan",
 "occupation": "Undergraduate student, competitive strongman athlete",
 "alcohol_use": "Rare/social (1-2 drinks per month)",
 "family_history": [
  "Hypertension (father)",
  "Type 2 diabetes mellitus (maternal grandmother)",
  "Irritable bowel syndrome (mother)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Pelvic floor dysfunction"
 ],
 "age_at_first_encounter": 23
}
```

Primary diagnoses: ["Protein-energy malnutrition", "Pelvic floor dysfunction"]
Comorbidities: ["Obesity"]

## Encounter 0: source 2020-01-15, practice 2025-01-07, outpatient, Physical Medicine & Rehabilitation

Chief complaint: Routine pre-travel check-up before semester abroad. Patient reports no active complaints but requires medical clearance for international travel.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Physical Medicine & Rehabilitation
Visit Type: outpatient

CHIEF COMPLAINT:
Routine pre-travel check-up before semester abroad. Patient reports no active complaints but requires medical clearance for international travel.

PAST MEDICAL HISTORY:
Active Problem List:
- Pelvic floor dysfunction

PAST SURGICAL HISTORY:
- None

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Hypertension (father)
- Type 2 diabetes mellitus (maternal grandmother)
- Irritable bowel syndrome (mother)

SOCIAL HISTORY:
He is a healthy college student who recently began competing in local strongman events. For the past several months, he has followed a structured training program focused on short, high-intensity sets involving maximal effort lifting, with exercises such as weighted sled pushes, deadlifts, and farmer’s carries. He performs very little aerobic activity and does not participate in endurance sports.

VITAL SIGNS:
His BMI is 26.1 kg/m².

PHYSICAL EXAMINATION:
On physical exam, he is normotensive, with a muscular build and no abnormalities noted.

PATHOLOGY:
He enrolls in a study examining muscle adaptations in athletes. A quadriceps biopsy is taken and examined. Compared to sedentary controls, his sample shows adaptation consistent with resistance training.

## Encounter 1: source 2021-01-15, practice 2026-01-22, inpatient, Internal Medicine

Chief complaint: Rescued after being lost in wilderness for 3 weeks with minimal caloric intake. Presents with severe cachexia, muscle wasting, and electrolyte disturbances requiring immediate metabolic stabilization.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Marcus Johnson, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Rescued after being lost in wilderness for 3 weeks with minimal caloric intake. Presents with severe cachexia, muscle wasting, and electrolyte disturbances requiring immediate metabolic stabilization.

HISTORY OF PRESENT ILLNESS:
The patient with pelvic floor dysfunction presents one year after pre-travel clearance following three weeks lost in the wilderness. During that time, he survived by drinking water and eating a few berries, but otherwise consumed no significant food. His family provides him with a large meal while awaiting medical attention. Within hours of eating, he becomes tachycardic and mildly confused.

PAST MEDICAL HISTORY:
Active Problem List:
- Pelvic floor dysfunction (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- None

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Hypertension (father)
- Type 2 diabetes mellitus (maternal grandmother)
- Irritable bowel syndrome (mother)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare/social (1-2 drinks per month). Occupation: Undergraduate student, competitive strongman athlete.

VITAL SIGNS:
His BMI is 27 kg/m². Vital signs are stable.

PHYSICAL EXAMINATION:
On arrival to the hospital, he is alert and oriented.

PLAN:
Blood tests are ordered to assess for acute metabolic derangements.
