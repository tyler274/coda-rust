# synthetic_hospital patient 2597

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex M, Hispanic/Latino, Private PPO (employer-sponsored).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-08-01 (Fri) | outpatient | Gastroenterology |
| 1 | 2020-02-15 | 2025-09-12 (Fri) | ed | Emergency Medicine |
| 2 | 2020-07-15 | 2026-02-06 (Fri) | outpatient | Infectious Diseases |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private PPO (employer-sponsored)",
 "occupation": "Software Developer",
 "alcohol_use": "Social (special occasions only)",
 "family_history": [
  "Father: hypertension",
  "Mother: hypothyroidism",
  "Sister: asthma"
 ],
 "race_ethnicity": "Hispanic/Latino",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Acute Hepatitis A"]
Comorbidities: ["Occupational exposure to hepatitis A virus"]

## Encounter 0: source 2020-01-15, practice 2025-08-01, outpatient, Gastroenterology

Chief complaint: 6-day history of fatigue, loss of appetite, and mild abdominal discomfort with dark urine and yellowing of eyes

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
6-day history of fatigue, loss of appetite, and mild abdominal discomfort with dark urine and yellowing of eyes

HISTORY OF PRESENT ILLNESS:
He also reports dark urine and yellowing of his eyes noticed by his wife this morning. The patient recently returned from visiting family in a rural region of Mexico, where he attended several large gatherings and ate food from street vendors.

PAST MEDICAL HISTORY:
He has no significant past medical history. He does not recall any blood transfusions or previous hepatitis infection.

MEDICATIONS:
He does not take any medications.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: hypertension
- Mother: hypothyroidism
- Sister: asthma

SOCIAL HISTORY:
He drinks alcohol only on special occasions and denies use of illicit drugs.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
Physical examination shows scleral icterus and mild hepatomegaly.

LABORATORY DATA:
Laboratory tests show elevated aminotransferases and hyperbilirubinemia. Laboratory results:
Alanine aminotransferase     945 U/L
Aspartate aminotransferase    812 U/L
Total bilirubin           5.1 mg/dL
Alkaline phosphatase       189 U/L

## Encounter 1: source 2020-02-15, practice 2025-09-12, ed, Emergency Medicine

Chief complaint: 3-day history of nausea, vomiting, and abdominal pain with dark urine, yellowing skin, and low-grade fever

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. James Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
3-day history of nausea, vomiting, and abdominal pain with dark urine, yellowing skin, and low-grade fever

HISTORY OF PRESENT ILLNESS:
He returns one month after gastroenterology evaluation for prior symptoms. He reports the onset of dark urine and yellowing of his skin today. He has a low-grade fever of 37.8°C.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute Hepatitis A (diagnosed 2020-01-15)

He denies any significant past medical history.

MEDICATIONS:
He takes no medications.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: hypertension
- Mother: hypothyroidism
- Sister: asthma

SOCIAL HISTORY:
The patient works as a staff member in a local day care center and recently returned from a trip to a country with poor sanitation.
He has no history of intravenous drug use or blood transfusions.

PHYSICAL EXAMINATION:
On physical examination, he is icteric and has hepatomegaly without ascites or spider angiomas.

LABORATORY DATA:
His laboratory results are as follows:

- Total bilirubin: 4.2 mg/dL (normal 0.3-1.2 mg/dL)
- Direct bilirubin: 1.8 mg/dL (normal 0.0-0.3 mg/dL)
- AST: 1200 U/L (normal 10-40 U/L)
- ALT: 1100 U/L (normal 7-56 U/L)
- Alkaline phosphatase: 150 U/L (normal 40-130 U/L)

## Encounter 2: source 2020-07-15, practice 2026-02-06, outpatient, Infectious Diseases

Chief complaint: 2-day history of nausea, vomiting, and right upper quadrant abdominal pain

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Aisha Patel, MD
Department: Infectious Diseases
Visit Type: outpatient

CHIEF COMPLAINT:
2-day history of nausea, vomiting, and right upper quadrant abdominal pain

HISTORY OF PRESENT ILLNESS:
The patient returns with recurrent symptoms following prior presentations in January and February 2020. He presents to the clinic with a 2-day history of nausea, vomiting, and right upper quadrant abdominal pain. He reports that his partner was recently diagnosed with hepatitis A. He denies any history of intravenous drug use, recent travel, or consumption of undercooked seafood.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute Hepatitis A (diagnosed 2020-01-15)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: hypertension
- Mother: hypothyroidism
- Sister: asthma

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (special occasions only). Occupation: Software Developer.

PHYSICAL EXAMINATION:
On examination, he is afebrile, with scleral icterus and mild hepatomegaly.

LABORATORY DATA:
His laboratory tests show:

- Total bilirubin: 2.8 mg/dL (normal 0.3-1.2 mg/dL)
- Direct bilirubin: 1.1 mg/dL (normal 0.0-0.3 mg/dL)
- AST: 110 U/L (normal 10-40 U/L)
- ALT: 125 U/L (normal 7-56 U/L)
- Alkaline phosphatase: 90 U/L (normal 40-130 U/L)
- Anti-HAV IgM: Positive

ASSESSMENT AND PLAN:
Given the patient's exposure history and laboratory findings,
