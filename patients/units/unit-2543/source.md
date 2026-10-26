# synthetic_hospital patient 2543

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 25 at first encounter, sex M, Non-Hispanic White, Private employer-sponsored plan.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-10-01 (Tue) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2025-04-11 (Fri) | inpatient | Orthopedic Surgery |
| 2 | 2021-09-15 | 2026-06-16 (Tue) | icu | Surgery |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private employer-sponsored plan",
 "occupation": "Construction laborer",
 "alcohol_use": "Social alcohol use with occasional binge episodes",
 "family_history": [
  "Father with alcohol use disorder",
  "Maternal uncle with bipolar disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 18 months ago",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 25
}
```

Primary diagnoses: ["Hypermetabolic response to severe burn injury", "Stimulant intoxication", "Acute osteomyelitis of foot due to Pseudomonas aeruginosa"]
Comorbidities: ["Sinus tachycardia secondary to burns", "Essential hypertension", "Fever"]

## Encounter 0: source 2020-01-15, practice 2024-10-01, ed, Emergency Medicine

Chief complaint: Brought to the emergency department by friends after acting unusually at a party with agitation, rapid speech, and profuse sweating

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought to the emergency department by friends after acting unusually at a party with agitation, rapid speech, and profuse sweating

HISTORY OF PRESENT ILLNESS:
He is agitated, speaking rapidly, and appears to be sweating profusely. His friends mention that he may have taken some "party drugs" but are unsure of what he ingested.

PAST MEDICAL HISTORY:
His medical history is unremarkable,

ALLERGIES:
He has no known drug allergies.

FAMILY HISTORY:
- Father with alcohol use disorder
- Maternal uncle with bipolar disorder

SOCIAL HISTORY:
Smoking: Former smoker, quit 18 months ago. Alcohol: Social alcohol use with occasional binge episodes. Occupation: Construction laborer.

VITAL SIGNS:
On examination, his temperature is 38.9°C (102°F), blood pressure is 165/95 mmHg, pulse is 130/min, and respiratory rate is 24/min.

PHYSICAL EXAMINATION:
His pupils are dilated, and his skin is warm and dry. He is oriented to person and place but not to time. The patient's muscular tone is increased, and he exhibits occasional myoclonic jerks.

LABORATORY DATA:
Laboratory values are as follows:
- Sodium: 140 mEq/L
- Potassium: 4.1 mEq/L
- Chloride: 102 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen: 18 mg/dL
- Creatinine: 1.0 mg/dL
- Glucose: 150 mg/dL
- Complete blood count is within normal limits

Urine toxicology screen is pending.

OTHER STUDIES:
An electrocardiogram shows sinus tachycardia with no other abnormalities.

## Encounter 1: source 2020-07-15, practice 2025-04-11, inpatient, Orthopedic Surgery

Chief complaint: 2-day history of increasing pain, redness, and swelling on the sole of his right foot after stepping on a nail 5 days prior

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Orthopedic Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
2-day history of increasing pain, redness, and swelling on the sole of his right foot after stepping on a nail 5 days prior

HISTORY OF PRESENT ILLNESS:
The patient is a 25-year-old male with a prior ED visit for agitation who reports stepping on a nail 5 days ago while wearing sneakers. He cleaned the wound with water at home and did not seek medical attention.

PAST MEDICAL HISTORY:
Active Problem List:
- Stimulant intoxication (diagnosed 2020-01-15)

MEDICATIONS:
He takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder
- Maternal uncle with bipolar disorder

SOCIAL HISTORY:
Smoking: Former smoker, quit 18 months ago. Alcohol: Social alcohol use with occasional binge episodes. Occupation: Construction laborer.

VITAL SIGNS:
His temperature is 38.2°C (100.8°F), heart rate is 88/min, and blood pressure is 130/75 mmHg.

PHYSICAL EXAMINATION:
On physical examination, there is a 1 cm puncture wound with surrounding erythema, warmth, and tenderness on the plantar surface of the right foot. There is also mild pain with passive movement of the toes.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 12,000/uL
- Hemoglobin: 14.2 g/dL
- Platelet count: 250,000/uL
- Erythrocyte sedimentation rate (ESR): 45 mm/hr
- C-reactive protein (CRP): 25 mg/L

IMAGING:
Radiographs of the foot show no fractures but there is evidence of periosteal reaction along the proximal phalanx of the second toe.

## Encounter 2: source 2021-09-15, practice 2026-06-16, icu, Surgery

Chief complaint: Second and third-degree burns covering 40% of total body surface area after house fire with inhalational injury requiring intubation

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Elena Vasquez, MD
Department: Surgery
Visit Type: icu

CHIEF COMPLAINT:
Second and third-degree burns covering 40% of total body surface area after house fire with inhalational injury requiring intubation

HISTORY OF PRESENT ILLNESS:
The patient has history of agitation and foot infection. He is intubated due to inhalational injury. His urine output has been maintained at 0.5 mL/kg/hr with IV fluid resuscitation.

PAST MEDICAL HISTORY:
Active Problem List:
- Stimulant intoxication (diagnosed 2020-01-15)
- Acute osteomyelitis of foot due to Pseudomonas aeruginosa (diagnosed 2020-07-15)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder
- Maternal uncle with bipolar disorder

SOCIAL HISTORY:
Smoking: Former smoker, quit 18 months ago. Alcohol: Social alcohol use with occasional binge episodes. Occupation: Construction laborer.

VITAL SIGNS:
On day 3 post-admission, the patient's temperature is 38.9°C, heart rate is 120 bpm, and blood pressure is 155/90 mmHg

PHYSICAL EXAMINATION:
An agitated patient with warm, flushed skin. The burn wounds appear clean and are being managed with topical antimicrobial therapy.

LABORATORY DATA:
Serum glucose: 180 mg/dL
- Serum insulin: Elevated
- Serum cortisol: Elevated
- Serum albumin: 2.5 g/dL
- Urine urea nitrogen: Increased
