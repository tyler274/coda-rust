# synthetic_hospital patient 2581

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 25 at first encounter, sex M, White, non-Hispanic (Lebanese descent), Private PPO (parent's employer-sponsored plan).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-01-11 (Sat) | ed | Emergency Medicine |
| 1 | 2021-01-15 | 2026-01-14 (Wed) | icu | Trauma Surgery |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private PPO (parent's employer-sponsored plan)",
 "occupation": "Graduate student (Mechanical Engineering)",
 "alcohol_use": "Occasional social use (1-2 drinks per month)",
 "family_history": [
  "Noncontributory"
 ],
 "race_ethnicity": "White, non-Hispanic (Lebanese descent)",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 25
}
```

Primary diagnoses: ["Traumatic liver laceration with hemoperitoneum", "Traumatic duodenal rupture"]
Comorbidities: ["Abdominal trauma requiring diagnostic peritoneal lavage"]

## Encounter 0: source 2020-01-15, practice 2025-01-11, ed, Emergency Medicine

Chief complaint: Mild to moderate upper abdominal pain following high-speed motor vehicle collision

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Mild to moderate upper abdominal pain following high-speed motor vehicle collision

HISTORY OF PRESENT ILLNESS:
He presents to the emergency department following a motor vehicle collision. He was the restrained driver in a car that hit a tree at high speed. He complains of mild to moderate abdominal pain, which he describes as a dull ache in the upper abdomen. He denies any nausea, vomiting, or changes in bowel habits.

PAST MEDICAL HISTORY:
His past medical history is unremarkable.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Noncontributory

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per month). Occupation: Graduate student (Mechanical Engineering).

PHYSICAL EXAMINATION:
On examination, the patient is alert but appears uncomfortable. On physical examination, his vital signs are stable, and there is mild tenderness to palpation in the epigastric region without rebound or guarding. Bowel sounds are normal. There is no ecchymosis or external sign of injury.

LABORATORY DATA:
Laboratory Values:
- Hemoglobin: 13.8 g/dL
- White blood cell count: 12,000 /μL
- Amylase: 90 U/L (normal range: 30-110 U/L)
- Lipase: 45 U/L (normal range: 0-160 U/L)

IMAGING:
An upright chest X-ray shows no rib fractures but reveals a small amount of free air under the diaphragm. An abdominal CT scan confirms the presence of retroperitoneal free air adjacent to the pancreas.

## Encounter 1: source 2021-01-15, practice 2026-01-14, icu, Trauma Surgery

Chief complaint: Hypotension and pallor following high-speed motor vehicle collision with airbag deployment

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Torres, MD
Department: Trauma Surgery
Visit Type: icu

CHIEF COMPLAINT:
Hypotension and pallor following high-speed motor vehicle collision with airbag deployment

HISTORY OF PRESENT ILLNESS:
The patient is a 26-year-old male with a history of abdominal trauma from a January 2020 motor vehicle collision, who presents following a separate high-speed impact. He was the restrained driver, and airbags were deployed. On arrival, he is alert but appears pale and anxious. The patient's blood pressure drops to 85/55 mmHg despite aggressive fluid resuscitation.

PAST MEDICAL HISTORY:
Active Problem List:
- Traumatic duodenal rupture (diagnosed 2020-01-15)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Noncontributory

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per month). Occupation: Graduate student (Mechanical Engineering).

VITAL SIGNS:
His vital signs are as follows: blood pressure 90/60 mmHg, heart rate 120 beats per minute, respiratory rate 22 breaths per minute, and oxygen saturation 97% on room air.

PHYSICAL EXAMINATION:
Physical examination reveals abdominal distension, tenderness to palpation in the right upper quadrant, and guarding. There is no rebound tenderness. Bowel sounds are normal.

LABORATORY DATA:
Hemoglobin is 9.8 g/dL (decreased from a baseline of 14.2 g/dL one month ago), hematocrit is 29.4%, and platelets are 250,000/µL. Liver function tests are within normal limits.

IMAGING:
The Focused Assessment with Sonography for Trauma (FAST) exam shows free fluid in the hepatorenal recess (Morison's pouch).
