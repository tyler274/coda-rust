# synthetic_hospital patient 2613

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex M, Turkish, Aetna Better Health.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-04-15 (Mon) | outpatient | Rheumatology |
| 1 | 2022-01-15 | 2026-04-11 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Aetna Better Health",
 "occupation": "Software developer",
 "alcohol_use": "Social (1-2 drinks on weekends)",
 "family_history": [
  "Maternal uncle with Beh\u00e7et syndrome",
  "Paternal grandmother with rheumatoid arthritis"
 ],
 "race_ethnicity": "Turkish",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "0.6 mg orally daily",
   "name": "Colchicine"
  },
  {
   "dose": "5 mg orally daily",
   "name": "Prednisone"
  }
 ],
 "surgical_history": [
  "Tonsillectomy (age 7)"
 ],
 "chronic_conditions": [
  "Beh\u00e7et syndrome"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Beh\u00e7et syndrome", "Heat exhaustion"]
Comorbidities: ["Delayed management of life-threatening respiratory failure"]

## Encounter 0: source 2020-01-15, practice 2024-04-15, outpatient, Rheumatology

Chief complaint: Painful recurrent oral ulcers, genital ulcers, and blurred vision

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Ayşe Demir, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Painful recurrent oral ulcers, genital ulcers, and blurred vision

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 6-month history of painful oral ulcers, which have recurred three times. He also reports two episodes of painful genital ulcers and a recent onset of blurred vision. He denies any history of sexually transmitted infections and is in a monogamous relationship.

PAST MEDICAL HISTORY:
Active Problem List:
- Behçet syndrome

PAST SURGICAL HISTORY:
- Tonsillectomy (age 7)

MEDICATIONS:
Home Medications:
- Colchicine 0.6 mg orally daily
- Prednisone 5 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal uncle with Behçet syndrome
- Paternal grandmother with rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks on weekends). Occupation: Software developer.

PHYSICAL EXAMINATION:
On examination, you note several small, round ulcers with erythematous borders on the buccal mucosa and similar lesions on the scrotum. His visual acuity is decreased, and fundoscopic examination reveals uveitis. His joint examination is normal.

LABORATORY DATA:
Laboratory studies show:

- Complete blood count: Normal
- Erythrocyte sedimentation rate (ESR): Elevated
- Antinuclear antibody (ANA): Negative
- Rheumatoid factor (RF): Negative
- Pathergy test: Positive

## Encounter 1: source 2022-01-15, practice 2026-04-11, ed, Emergency Medicine

Chief complaint: Headache, fatigue, and lightheadedness after arrival at high altitude

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Headache, fatigue, and lightheadedness after arrival at high altitude

HISTORY OF PRESENT ILLNESS:
This 30-year-old male with Behçet syndrome, diagnosed by Rheumatology in January 2020 for recurrent oral and genital ulcers with blurred vision and maintained on colchicine and prednisone, presents to the Emergency Department. He arrived from sea level the previous evening to begin a weeklong hiking trip in the mountains, is staying at an elevation of 10,500 feet, and reports no prior experience at high altitude. Since arrival, he has felt increasingly short of breath, particularly at rest and during sleep, denying chest pain, cough, or fever.

PAST MEDICAL HISTORY:
Active Problem List:
- Behçet syndrome (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- Tonsillectomy (age 7)

MEDICATIONS:
Home Medications:
- Colchicine 0.6 mg orally daily
- Prednisone 5 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal uncle with Behçet syndrome
- Paternal grandmother with rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks on weekends). Occupation: Software developer.

VITAL SIGNS:
Vital signs show oxygen saturation of 92% on room air, respiratory rate of 24/min, and blood pressure of 124/76 mm Hg.

PHYSICAL EXAMINATION:
He is alert but appears mildly anxious. On exam, he is breathing deeply but without wheezing or rales. Neurologic examination is nonfocal.
