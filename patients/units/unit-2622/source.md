# synthetic_hospital patient 2622

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 34 at first encounter, sex M, Non-Hispanic White, Employer-sponsored PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-12-30 (Mon) | outpatient | Internal Medicine |
| 1 | 2021-07-15 | 2026-06-19 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Employer-sponsored PPO",
 "occupation": "International logistics consultant",
 "alcohol_use": "Social, 1-2 drinks per week",
 "family_history": [
  "Father: Alcohol use disorder",
  "Mother: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [
  "History of intravenous drug use (in remission)",
  "History of resolved acute viral hepatitis (age 24)"
 ],
 "age_at_first_encounter": 34
}
```

Primary diagnoses: ["History of intravenous drug use", "Acute viral hepatitis"]
Comorbidities: ["Occupational exposure to hepatitis A virus"]

## Encounter 0: source 2020-01-15, practice 2024-12-30, outpatient, Internal Medicine

Chief complaint: Routine health evaluation

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health evaluation

HISTORY OF PRESENT ILLNESS:
He comes to the clinic for a routine health evaluation. He has no current symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- History of intravenous drug use (in remission)
- History of resolved acute viral hepatitis (age 24)

He reports that about 10 years ago he was told he had "hepatitis" after traveling abroad, but he did not receive any antiviral treatment and eventually recovered fully.

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alcohol use disorder
- Mother: Hypertension

SOCIAL HISTORY:
He does not use intravenous drugs and has no history of blood transfusions.

LABORATORY DATA:
HBsAg                                            Negative
Anti-HBs (HBsAb)                        Positive
Anti-HBc (HBcAb)                        Positive
HBeAg                                           Negative
Anti-HBe                                        Positive

## Encounter 1: source 2021-07-15, practice 2026-06-19, ed, Emergency Medicine

Chief complaint: Malaise, fatigue, and dark-colored urine for the past week

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Malaise, fatigue, and dark-colored urine for the past week

HISTORY OF PRESENT ILLNESS:
This 35-year-old with resolved acute viral hepatitis and intravenous drug use presents with malaise, fatigue, and dark-colored urine for the past week. He reports mild nausea but no vomiting or abdominal pain. He recently returned from a trip to Southeast Asia.

PAST MEDICAL HISTORY:
Active Problem List:
- History of intravenous drug use (diagnosed 2020-01-15)
- History of intravenous drug use (in remission)
- History of resolved acute viral hepatitis (age 24)

MEDICATIONS:
He takes no medications

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alcohol use disorder
- Mother: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per week. Occupation: International logistics consultant.

PHYSICAL EXAMINATION:
Physical examination reveals mild scleral icterus and right upper quadrant tenderness without hepatomegaly.

LABORATORY DATA:
AST                                              980 U/L
ALT                                              1150 U/L
Total bilirubin                                  3.2 mg/dL
Alkaline phosphatase                             110 U/L
Albumin 4.2 g/dL
INR 1.0

PATHOLOGY:
A liver biopsy shows apoptotic hepatocytes surrounded by mononuclear infiltrates.
