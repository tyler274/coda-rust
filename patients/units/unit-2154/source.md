# synthetic_hospital patient 2154

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 37 at first encounter, sex M, Non-Hispanic White, Employer-sponsored health insurance (PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-04-26 (Fri) | outpatient | Internal Medicine |
| 1 | 2022-01-15 | 2026-04-29 (Wed) | outpatient | Gastroenterology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Employer-sponsored health insurance (PPO)",
 "occupation": "Software developer",
 "alcohol_use": "None",
 "family_history": [
  "Father: End-stage renal disease secondary to polycystic kidney disease, hypertension (deceased age 65)",
  "Paternal uncle: Cerebral aneurysm",
  "Mother: Type 2 diabetes mellitus",
  "Older brother: Hypertension, polycystic kidney disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg by mouth daily",
   "name": "Lisinopril"
  },
  {
   "dose": "500 mg by mouth PRN headache",
   "name": "Acetaminophen"
  },
  {
   "dose": "10 mg by mouth PRN allergies",
   "name": "Cetirizine"
  }
 ],
 "surgical_history": [
  "Tonsillectomy and adenoidectomy, age 8"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Seasonal allergic rhinitis"
 ],
 "age_at_first_encounter": 37
}
```

Primary diagnoses: ["Autosomal Dominant Polycystic Kidney Disease", "Gilbert syndrome"]
Comorbidities: ["Family History of Stroke"]

## Encounter 0: source 2020-01-15, practice 2024-04-26, outpatient, Internal Medicine

Chief complaint: Persistent headaches over the past several months with dull, non-pulsatile pain worse in mornings; occasional abdominal fullness and increased urinary frequency

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Amara Okafor, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent headaches over the past several months with dull, non-pulsatile pain worse in mornings; occasional abdominal fullness and increased urinary frequency

HISTORY OF PRESENT ILLNESS:
He reports a dull, non-pulsatile pain that is worse in the mornings but improves as the day progresses. He has no nausea, vomiting, visual changes, or neurologic deficits. He also notes occasional vague abdominal fullness and increased urinary frequency, which he attributes to “drinking more water.”

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy, age 8

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg by mouth daily
- Acetaminophen 500 mg by mouth PRN headache
- Cetirizine 10 mg by mouth PRN allergies

ALLERGIES:
NKDA

FAMILY HISTORY:
His maternal uncle died suddenly in his 40s from a “brain hemorrhage.”

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Software developer.

VITAL SIGNS:
His blood pressure is 158/96 mm Hg.

PHYSICAL EXAMINATION:
On physical examination, the abdomen is mildly distended but non-tender. Fundoscopy is unremarkable.

LABORATORY DATA:
Laboratory studies show a serum creatinine of 1.5 mg/dL and normal electrolytes. A urinalysis shows trace proteinuria without hematuria.

## Encounter 1: source 2022-01-15, practice 2026-04-29, outpatient, Gastroenterology

Chief complaint: Intermittent scleral icterus beginning 6 days ago with mild fatigue

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent scleral icterus beginning 6 days ago with mild fatigue

HISTORY OF PRESENT ILLNESS:
The patient is a 39-year-old male with a history of hypertension, seasonal allergic rhinitis, and autosomal dominant polycystic kidney disease previously evaluated in 2020 who presents to the Gastroenterology clinic for evaluation of intermittent scleral icterus that began approximately 6 days ago. He reports mild fatigue but denies abdominal pain, pruritus, dark urine, or pale stools. He has never noticed similar symptoms before.

PAST MEDICAL HISTORY:
Active Problem List:
- Autosomal Dominant Polycystic Kidney Disease (diagnosed 2020-01-15)
- Hypertension
- Seasonal allergic rhinitis

He has no prior history of liver disease.

PAST SURGICAL HISTORY:
He underwent an uncomplicated laparoscopic cholecystectomy three weeks prior.

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg by mouth daily
- Acetaminophen 500 mg by mouth PRN headache
- Cetirizine 10 mg by mouth PRN allergies

ALLERGIES:
NKDA

FAMILY HISTORY:
His brother has had mild jaundice during periods of illness

SOCIAL HISTORY:
No significant alcohol use

PHYSICAL EXAMINATION:
Physical examination reveals mild jaundice but is otherwise unremarkable. Eye examination is shown below:

LABORATORY DATA:
Laboratory studies show: total bilirubin 2.6 mg/dL (reference: 0.2–1.2), direct bilirubin 0.2 mg/dL (reference: 0.0–0.3), AST 24 U/L (reference: 10–40), ALT 28 U/L (reference: 7–56), ALP 66 U/L (reference: 44–147), LDH 189 U/L (reference: 120–250). Hemoglobin and reticulocyte counts are normal.
