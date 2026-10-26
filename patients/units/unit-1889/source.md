# synthetic_hospital patient 1889

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, White, non-Hispanic, Employer-sponsored PPO plan.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-03-09 (Mon) | outpatient | Neurology |
| 1 | 2020-07-15 | 2020-09-07 (Mon) | outpatient | Neurology |
| 2 | 2021-07-15 | 2021-09-07 (Tue) | outpatient | Internal Medicine |
| 3 | 2023-01-15 | 2023-02-23 (Thu) | telehealth | Neurology |
| 4 | 2026-01-15 | 2026-03-11 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Employer-sponsored PPO plan",
 "occupation": "Software engineer",
 "alcohol_use": "Occasional social use (1-2 drinks per month)",
 "family_history": [
  "Maternal aunt with rheumatoid arthritis",
  "Mother with Hashimoto thyroiditis"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "60 mg by mouth three times daily",
   "name": "Pyridostigmine"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Myasthenia gravis"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Myasthenia gravis", "Myasthenic crisis"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2020-03-09, outpatient, Neurology

Chief complaint: Intermittent double vision and difficulty keeping eyelids open, worse in the evening

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent double vision and difficulty keeping eyelids open, worse in the evening

HISTORY OF PRESENT ILLNESS:
A 28-year-old woman presents to the clinic with a 3-month history of intermittent double vision and difficulty keeping her eyelids open, which worsens towards the evening. She also reports occasional difficulty swallowing and a change in her voice quality after prolonged conversations.

PAST MEDICAL HISTORY:
Active Problem List:
- Myasthenia gravis

MEDICATIONS:
Home Medications:
- Pyridostigmine 60 mg by mouth three times daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal aunt with rheumatoid arthritis
- Mother with Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per month). Occupation: Software engineer.

PHYSICAL EXAMINATION:
On physical examination, you note bilateral ptosis and a nasal speech pattern. Her ptosis improves after the administration of ice for 2 minutes.

OTHER STUDIES:
Repetitive nerve stimulation shows a decremental response in the amplitude of the compound muscle action potential.

## Encounter 1: source 2020-07-15, practice 2020-09-07, outpatient, Neurology

Chief complaint: Fluctuating muscle weakness and computer-related eye fatigue

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Fluctuating muscle weakness and computer-related eye fatigue

HISTORY OF PRESENT ILLNESS:
The patient returns to the neurology clinic for follow-up of myasthenia gravis initially evaluated in January 2020 for intermittent double vision and difficulty keeping eyelids open. She presents with a 6-month history of fluctuating muscle weakness that worsens towards the evening. She reports difficulty in keeping her eyelids open after working on her computer for several hours and has occasionally noticed double vision. She also mentions that her voice tends to become hoarse after prolonged conversations.

PAST MEDICAL HISTORY:
Active Problem List:
- Myasthenia gravis (diagnosed 2020-01-15)

MEDICATIONS:
Home Medications:
- Pyridostigmine 60 mg by mouth three times daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal aunt with rheumatoid arthritis
- Mother with Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per month). Occupation: Software engineer.

PHYSICAL EXAMINATION:
On physical examination, you observe bilateral ptosis and diplopia that worsens with sustained upward gaze. The rest of the neurological examination is normal.

LABORATORY DATA:
Routine blood tests are within normal limits.

OTHER STUDIES:
You decide to perform a bedside ice pack test. After applying an ice pack over her eyelids for 2 minutes, there is a noticeable improvement in her ptosis.

## Encounter 2: source 2021-07-15, practice 2021-09-07, outpatient, Internal Medicine

Chief complaint: Intermittent double vision, slurred speech, and difficulty swallowing

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent double vision, slurred speech, and difficulty swallowing

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up evaluation of her known myasthenia gravis, previously assessed in Neurology in January and July 2020. She presents with a 6-month history of intermittent double vision and difficulty keeping her eyes open, especially towards the end of the day. She also reports episodes of slurred speech and difficulty swallowing that improve with rest.

PAST MEDICAL HISTORY:
Active Problem List:
- Myasthenia gravis (diagnosed 2020-01-15)

MEDICATIONS:
Home Medications:
- Pyridostigmine 60 mg by mouth three times daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal aunt with rheumatoid arthritis
- Mother with Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per month). Occupation: Software engineer.

PHYSICAL EXAMINATION:
On examination, you note bilateral ptosis that worsens after sustained upward gaze and improvement in muscle strength after a short period of rest.

LABORATORY DATA:
Blood tests reveal the presence of acetylcholine receptor antibodies.

OTHER STUDIES:
Repetitive nerve stimulation shows a decremental response,

## Encounter 3: source 2023-01-15, practice 2023-02-23, telehealth, Neurology

Chief complaint: Intermittent double vision and voice changes after prolonged conversations

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Aisha Patel, MD
Department: Neurology
Visit Type: telehealth

CHIEF COMPLAINT:
Intermittent double vision and voice changes after prolonged conversations

HISTORY OF PRESENT ILLNESS:
This 31-year-old female with Myasthenia gravis since 2020 returns for Neurology follow-up regarding recurrent symptoms. She reports a 6-month history of intermittent double vision and difficulty keeping her eyelids open towards the end of the day. She also reports occasional difficulty swallowing and slurred speech after prolonged conversations. Her symptoms seem to improve with rest.

PAST MEDICAL HISTORY:
Active Problem List:
- Myasthenia gravis (diagnosed 2020-01-15)

MEDICATIONS:
Home Medications:
- Pyridostigmine 60 mg by mouth three times daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal aunt with rheumatoid arthritis
- Mother with Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per month). Occupation: Software engineer.

PHYSICAL EXAMINATION:
Bilateral ptosis that worsens after sustained upward gaze and improvement in muscle strength following administration of ice over the eyelids

IMAGING:
A chest CT scan reveals an anterior mediastinal mass.

## Encounter 4: source 2026-01-15, practice 2026-03-11, ed, Emergency Medicine

Chief complaint: Acute onset increasing shortness of breath and difficulty swallowing

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. Emily Zhao, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset increasing shortness of breath and difficulty swallowing

HISTORY OF PRESENT ILLNESS:
The patient with myasthenia gravis followed since 2020 for fluctuating ocular and bulbar symptoms presents to the emergency department with a 2-day history of increasing shortness of breath and difficulty swallowing. She reports that her symptoms are worse at the end of the day and improve with rest.

PAST MEDICAL HISTORY:
Active Problem List:
- Myasthenia gravis (diagnosed 2020-01-15)

Her past medical history is significant for myasthenia gravis,.

MEDICATIONS:
For which she takes pyridostigmine.

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal aunt with rheumatoid arthritis
- Mother with Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per month). Occupation: Software engineer.

VITAL SIGNS:
Vital signs reveal a respiratory rate of 28 breaths per minute, heart rate of 110 beats per minute, blood pressure of 135/85 mmHg, and oxygen saturation of 89% on room air.

PHYSICAL EXAMINATION:
On examination, she has bilateral ptosis and diplopia. Her speech is nasal, and she is unable to lift her head off the bed.

LABORATORY DATA:
Arterial blood gas shows a pH of 7.32, PaCO2 of 50 mmHg, PaO2 of 60 mmHg, and bicarbonate of 24 mEq/L.

OTHER STUDIES:
Forced vital capacity is measured at 15 mL/kg.

PLAN:
The patient is started on supplemental oxygen via nasal cannula, but her respiratory effort continues to worsen.
