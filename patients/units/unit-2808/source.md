# synthetic_hospital patient 2808

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 44 at first encounter, sex F, White, non-Hispanic, Employer-sponsored private insurance (Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-07-22 (Tue) | outpatient | Internal Medicine |
| 1 | 2020-03-15 | 2025-09-19 (Fri) | outpatient | Ophthalmology |
| 2 | 2020-06-15 | 2025-12-31 (Wed) | outpatient | Neurology |
| 3 | 2020-10-15 | 2026-04-20 (Mon) | outpatient | Hematology and Oncology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Employer-sponsored private insurance (Blue Cross Blue Shield)",
 "occupation": "Administrative coordinator",
 "alcohol_use": "None",
 "family_history": [
  "Mother with Hashimoto thyroiditis",
  "Maternal aunt with rheumatoid arthritis",
  "Father with type 2 diabetes"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [],
 "age_at_first_encounter": 44
}
```

Primary diagnoses: ["Myasthenia gravis", "Thymoma"]
Comorbidities: ["Thymoma"]

## Encounter 0: source 2020-01-15, practice 2025-07-22, outpatient, Internal Medicine

Chief complaint: 3-month history of progressive muscle fatigue with difficulty chewing and eyelid drooping by evening

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of progressive muscle fatigue with difficulty chewing and eyelid drooping by evening

HISTORY OF PRESENT ILLNESS:
She reports difficulty chewing and keeping her eyelids open by evening, along with intermittent double vision. Her symptoms are worse at the end of the day and improve with rest.

PAST SURGICAL HISTORY:
- No prior surgeries

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with Hashimoto thyroiditis
- Maternal aunt with rheumatoid arthritis
- Father with type 2 diabetes

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative coordinator.

PHYSICAL EXAMINATION:
Physical examination shows mild ptosis and weakness of neck flexion, but intact reflexes and sensation.

IMAGING:
Chest imaging performed for unrelated screening reveals an anterior mediastinal mass.

OTHER STUDIES:
Electrophysiologic studies show a decremental response to repetitive nerve stimulation.

PLAN:
She is started on a medication.

## Encounter 1: source 2020-03-15, practice 2025-09-19, outpatient, Ophthalmology

Chief complaint: Intermittent double vision and drooping of eyelids worsening in the evening

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Emily Park, MD
Department: Ophthalmology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent double vision and drooping of eyelids worsening in the evening

HISTORY OF PRESENT ILLNESS:
The patient returns two months after initial presentation with progressive muscle fatigue, difficulty chewing, and eyelid drooping. She presents with intermittent double vision and drooping of her eyelids that become more pronounced in the evening. She notes increasing difficulty chewing and swallowing solid foods as the day progresses, as well as mild shortness of breath with exertion. She denies any recent illnesses.

PAST MEDICAL HISTORY:
Active Problem List:
- Myasthenia gravis (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- No prior surgeries

ALLERGIES:
NKDA

FAMILY HISTORY:
Family history of neuromuscular disease.

SOCIAL HISTORY:
Toxin exposures,

PHYSICAL EXAMINATION:
On exam, she demonstrates fatigable ptosis and struggles to maintain upward gaze for longer than 30 seconds. When asked to stand from a squatting position, she requires the use of her upper limbs for assistance. Muscle strength is otherwise preserved early in the day, and deep tendon reflexes are normal.

LABORATORY DATA:
Laboratory analysis reveals normal creatine kinase and negative anti-nuclear antibody.

IMAGING:
A chest CT scan identifies an anterior mediastinal mass.

## Encounter 2: source 2020-06-15, practice 2025-12-31, outpatient, Neurology

Chief complaint: Progressive muscle weakness with jaw fatigue and difficulty climbing stairs

ENCOUNTER NOTE
Date: 2020-06-15
Provider: Dr. Marcus Rodriguez, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive muscle weakness with jaw fatigue and difficulty climbing stairs

HISTORY OF PRESENT ILLNESS:
The patient returns for ongoing neurology evaluation of previously diagnosed myasthenia gravis, following prior assessments in Internal Medicine and Ophthalmology over the past five months. She reports that for the past several months she has experienced increasing fatigue throughout the day, especially when chewing, talking, or climbing stairs. She often has to rest after eating because her jaw becomes too tired to chew, and she has noticed intermittent double vision in the evenings. She denies recent illness, chest pain, or sensory changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Myasthenia gravis (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
She takes no daily medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with Hashimoto thyroiditis
- Maternal aunt with rheumatoid arthritis
- Father with type 2 diabetes

SOCIAL HISTORY:
She does not smoke, drink alcohol, or use illicit drugs.

VITAL SIGNS:
Vital signs are stable.

PHYSICAL EXAMINATION:
On physical examination, she has mild bilateral ptosis that worsens after sustaining upward gaze for 30 seconds. Extraocular movements are intact but weak with prolonged testing. Muscle strength is 5/5 in the morning but decreases to 3/5 in the proximal upper extremities after repetitive use. Reflexes are intact, and sensation is preserved.

LABORATORY DATA:
Laboratory studies are within normal limits.

IMAGING:
Chest imaging is ordered to evaluate for thymoma

PLAN:
She is admitted for further workup. she is started on neostigmine as part of initial therapy. After her first dose, her symptoms improve noticeably, with resolution of diplopia and improved chewing ability at dinner.

## Encounter 3: source 2020-10-15, practice 2026-04-20, outpatient, Hematology and Oncology

Chief complaint: Progressive fatigue over the past year with intermittent eyelid drooping and difficulty chewing

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. James O'Brien, MD
Department: Hematology and Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue over the past year with intermittent eyelid drooping and difficulty chewing

HISTORY OF PRESENT ILLNESS:
The patient is a 44-year-old female returning for evaluation following prior assessments in Internal Medicine, Ophthalmology, and Neurology. She reports that her symptoms often worsen in the evening and improve after resting. She has noticed intermittent drooping of her eyelids and occasional difficulty chewing food during prolonged meals. There is no history of recent trauma, fever, or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Myasthenia gravis (diagnosed 2020-01-15)

She has no prior neurologic illnesses.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
She takes no regular medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with Hashimoto thyroiditis
- Maternal aunt with rheumatoid arthritis
- Father with type 2 diabetes

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative coordinator.

PHYSICAL EXAMINATION:
Physical examination today shows fluctuating weakness of the extraocular muscles and mild dysarthria. Sensation and reflexes are intact, and limb strength is normal at rest but diminishes with repeated use.

LABORATORY DATA:
Laboratory testing reveals no metabolic abnormalities.

IMAGING:
A chest radiograph demonstrates widening of the mediastinum.
