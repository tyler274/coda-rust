# synthetic_hospital patient 1910

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 24 at first encounter, sex F, White, University Student Health Plan.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-06-11 (Wed) | outpatient | Orthopedics & Sports Medicine |
| 1 | 2020-06-15 | 2025-11-12 (Wed) | outpatient | Orthopedics & Sports Medicine |
| 2 | 2020-07-15 | 2025-12-10 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "University Student Health Plan",
 "occupation": "Graduate Student",
 "alcohol_use": "None",
 "family_history": [
  "No significant family history"
 ],
 "race_ethnicity": "White",
 "smoking_status": "Never",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 24
}
```

Primary diagnoses: ["Tibial stress fracture", "Metatarsal stress fracture", "Accidental hypothermia"]
Comorbidities: ["Endurance athlete status", "Long-distance running"]

## Encounter 0: source 2020-01-15, practice 2025-06-11, outpatient, Orthopedics & Sports Medicine

Chief complaint: Progressive right foot pain for 2 weeks

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Orthopedics & Sports Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive right foot pain for 2 weeks

HISTORY OF PRESENT ILLNESS:
She reports that the pain began shortly after she started an intensive training program, which includes running several miles daily. She denies any history of trauma but notes that the pain is exacerbated by physical activity and relieved by rest. She has tried over-the-counter ibuprofen with minimal relief.

PAST MEDICAL HISTORY:
Her medical history is unremarkable,

MEDICATIONS:
She takes no other medications.

ALLERGIES:
NKDA

FAMILY HISTORY:
- No significant family history

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Graduate Student.

PHYSICAL EXAMINATION:
On physical examination, there is tenderness to palpation over the midfoot, with no swelling, erythema, or warmth. Point tenderness is most notable at the second metatarsal.

IMAGING:
An X-ray of the foot is unremarkable.

## Encounter 1: source 2020-06-15, practice 2025-11-12, outpatient, Orthopedics & Sports Medicine

Chief complaint: Right shin pain for 3 weeks

ENCOUNTER NOTE
Date: 2020-06-15
Provider: Dr. Michael Park, MD
Department: Orthopedics & Sports Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Right shin pain for 3 weeks

HISTORY OF PRESENT ILLNESS:
The patient returns five months after her prior evaluation for right foot pain, now presenting with right shin pain. She reports that the pain began shortly after she increased her running distance in preparation for an upcoming marathon. The pain is exacerbated by activity and relieved with rest. She denies any recent trauma or falls.

PAST MEDICAL HISTORY:
Active Problem List:
- Metatarsal stress fracture (diagnosed 2020-01-15)

MEDICATIONS:
She takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- No significant family history

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Graduate Student.

PHYSICAL EXAMINATION:
On physical examination, there is mild swelling over the distal third of the right tibia. Palpation reveals a focal area of tenderness. There is no erythema or warmth, and the range of motion of the ankle is normal.

IMAGING:
An X-ray of the right tibia is obtained and shows no acute fracture or dislocation.

ASSESSMENT AND PLAN:
Given the patient's history and physical examination findings,

## Encounter 2: source 2020-07-15, practice 2025-12-10, ed, Emergency Medicine

Chief complaint: Collapse and altered mental status at marathon finish line

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Collapse and altered mental status at marathon finish line

HISTORY OF PRESENT ILLNESS:
The patient, with prior right metatarsal and tibial stress fractures, presents to the Emergency Department. She is shivering, disoriented, and slurring her speech. Her friends report that she was adequately dressed for the race but seemed to struggle in the last few miles.

PAST MEDICAL HISTORY:
Active Problem List:
- Metatarsal stress fracture (diagnosed 2020-01-15)
- Tibial stress fracture (diagnosed 2020-06-15)

MEDICATIONS:
She takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- No significant family history

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Graduate Student.

VITAL SIGNS:
On examination, her temperature is 34.5°C (94.1°F), heart rate is 58 bpm, blood pressure is 100/60 mmHg, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
Her skin is cold to the touch, and her extremities are pale. Capillary refill time is 3 seconds. The remainder of the physical examination is noncontributory.

LABORATORY DATA:
Laboratory Values:
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 0.9 mg/dL
- Glucose: 90 mg/dL
- Complete blood count: Within normal limits
