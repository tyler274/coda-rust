# synthetic_hospital patient 2632

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 24 at first encounter, sex M, White, Hispanic, University student health insurance plan.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-06-16 (Mon) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2025-12-16 (Tue) | ed | Neurology |
| 2 | 2021-01-15 | 2026-06-25 (Thu) | icu | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "University student health insurance plan",
 "occupation": "Graduate student (Environmental Science - Coastal Ecology)",
 "alcohol_use": "Social, 2-3 drinks on weekends",
 "family_history": [
  "Father: Hypertension, diagnosed age 45",
  "Mother: Hypothyroidism",
  "No family history of autoimmune neurological disorders"
 ],
 "race_ethnicity": "White, Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 24
}
```

Primary diagnoses: ["Guillain-Barr\u00e9 syndrome", "Campylobacter jejuni enteritis (recent infection)"]
Comorbidities: ["Campylobacter jejuni enteritis (recent infection)"]

## Encounter 0: source 2020-01-15, practice 2025-06-16, ed, Emergency Medicine

Chief complaint: progressive weakness and difficulty walking

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive weakness and difficulty walking

HISTORY OF PRESENT ILLNESS:
He is brought to the emergency department by his partner due to progressive weakness and difficulty walking. One week ago, he had an episode of profuse watery and bloody diarrhea after eating undercooked chicken at a street market while traveling. The diarrhea resolved spontaneously after three days. However, over the past four days, he has developed tingling in his feet, which has progressed to symmetric weakness in both legs. This morning, he was unable to get out of bed due to bilateral leg paralysis. He also notes difficulty climbing stairs and mild shortness of breath

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, diagnosed age 45
- Mother: Hypothyroidism
- No family history of autoimmune neurological disorders

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 2-3 drinks on weekends. Occupation: Graduate student (Environmental Science - Coastal Ecology).

REVIEW OF SYSTEMS:
He denies vision or bowel/bladder changes.

VITAL SIGNS:
On physical examination, his temperature is 37.1°C, and vital signs are stable.

PHYSICAL EXAMINATION:
Neurologic exam reveals flaccid paralysis of both lower limbs with absent deep tendon reflexes. There is decreased sensation to vibration and proprioception in the feet. Strength in the upper extremities is preserved, and cranial nerves are intact.

LABORATORY DATA:
A lumbar puncture is performed and shows albuminocytologic dissociation.

## Encounter 1: source 2020-07-15, practice 2025-12-16, ed, Neurology

Chief complaint: progressive weakness in his legs that has ascended to involve his arms

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Neurology
Visit Type: ed

CHIEF COMPLAINT:
Progressive weakness in his legs that has ascended to involve his arms

HISTORY OF PRESENT ILLNESS:
The patient returns to the emergency department with neurological symptoms following a recent diarrheal illness, having previously been diagnosed with Campylobacter jejuni enteritis. He reports a 2-day history of progressive weakness in his legs that has ascended to involve his arms. He reports difficulty walking and has fallen twice. He also complains of tingling sensations in his feet and hands. Two weeks prior, he experienced a diarrheal illness after eating undercooked poultry, which resolved within a few days.

PAST MEDICAL HISTORY:
Active Problem List:
- Campylobacter jejuni enteritis (recent infection) (diagnosed 2020-01-15)

MEDICATIONS:
He takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, diagnosed age 45
- Mother: Hypothyroidism
- No family history of autoimmune neurological disorders

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 2-3 drinks on weekends. Occupation: Graduate student (Environmental Science - Coastal Ecology).

VITAL SIGNS:
His vital signs are stable

PHYSICAL EXAMINATION:
He exhibits bilateral lower extremity weakness with a strength of 3/5 and upper extremity strength of 4/5. Deep tendon reflexes are diminished throughout. Sensation to light touch is decreased in a glove-and-stocking distribution. No cranial nerve deficits are noted.

LABORATORY DATA:
Laboratory Values:
Cerebrospinal Fluid (CSF):
- Protein: 85 mg/dL (Normal: 15-45 mg/dL)
- White Blood Cell Count: 5 cells/µL (Normal: 0-5 cells/µL)
- Glucose: 60 mg/dL (Normal: 40-70 mg/dL)

OTHER STUDIES:
A lumbar puncture is performed, and the cerebrospinal fluid analysis reveals an elevated protein level with a normal white blood cell count.

## Encounter 2: source 2021-01-15, practice 2026-06-25, icu, Internal Medicine

Chief complaint: progressive shortness of breath and difficulty taking deep breaths

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: icu

CHIEF COMPLAINT:
Progressive shortness of breath and difficulty taking deep breaths

HISTORY OF PRESENT ILLNESS:
The patient presents to the ICU one year following an initial diarrheal illness and subsequent neurological symptoms. One week prior to his initial presentation, he had a self-limited episode of diarrhea after returning from a beach trip. Since then, he has experienced increasing weakness in his legs that has now spread to his arms. He also reports difficulty climbing stairs and feels "winded" even when lying down. He denies cough, chest pain, or fever.

PAST MEDICAL HISTORY:
Active Problem List:
- Campylobacter jejuni enteritis (recent infection) (diagnosed 2020-01-15)
- Guillain-Barré syndrome (diagnosed 2020-07-15)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, diagnosed age 45
- Mother: Hypothyroidism
- No family history of autoimmune neurological disorders

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 2-3 drinks on weekends. Occupation: Graduate student (Environmental Science - Coastal Ecology).

VITAL SIGNS:
Pulse oximetry is 89% on room air, which improves with supplemental oxygen.

PHYSICAL EXAMINATION:
On examination, he is alert but appears anxious and speaks in short phrases. His respiratory rate is 24/min, and he is using accessory muscles. Breath sounds are diminished at the bases, but there are no crackles or wheezes. Neurologic exam reveals symmetric weakness in the upper and lower extremities, areflexia, and intact sensation.

LABORATORY DATA:
Arterial blood gas reveals hypoxemia and elevated CO₂.

IMAGING:
Chest x-ray is normal.

OTHER STUDIES:
Vital capacity is reduced on bedside spirometry.
Nerve conduction studies are pending.
