# synthetic_hospital patient 2144

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 52 at first encounter, sex M, Asian (East Asian), Private insurance (employer-sponsored).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-12-08 (Fri) | outpatient | Gastroenterology |
| 1 | 2022-01-15 | 2025-12-15 (Mon) | inpatient | Gastroenterology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Private insurance (employer-sponsored)",
 "occupation": "Logistics coordinator",
 "alcohol_use": "None",
 "family_history": [
  "Father: Hepatocellular carcinoma (age 71, deceased)",
  "Mother: Type 2 diabetes mellitus",
  "Older brother: Chronic hepatitis B infection"
 ],
 "race_ethnicity": "Asian (East Asian)",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "300 mg by mouth once daily",
   "name": "Tenofovir disoproxil fumarate"
  },
  {
   "dose": "5 mg by mouth once daily",
   "name": "Amlodipine"
  },
  {
   "dose": "2000 international units by mouth once daily",
   "name": "Cholecalciferol"
  }
 ],
 "surgical_history": [
  "Inguinal hernia repair (age 45)"
 ],
 "chronic_conditions": [
  "Chronic hepatitis B infection",
  "Hypertension",
  "Vitamin D deficiency"
 ],
 "age_at_first_encounter": 52
}
```

Primary diagnoses: ["Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate)", "Chronic hepatitis B infection"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2023-12-08, outpatient, Gastroenterology

Chief complaint: Routine check-up for chronic hepatitis B management

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine check-up for chronic hepatitis B management

HISTORY OF PRESENT ILLNESS:
He reports feeling well and has no symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic hepatitis B infection
- Hypertension
- Vitamin D deficiency

PAST SURGICAL HISTORY:
- Inguinal hernia repair (age 45)

MEDICATIONS:
Home Medications:
- Tenofovir disoproxil fumarate 300 mg by mouth once daily
- Amlodipine 5 mg by mouth once daily
- Cholecalciferol 2000 international units by mouth once daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hepatocellular carcinoma (age 71, deceased)
- Mother: Type 2 diabetes mellitus
- Older brother: Chronic hepatitis B infection

SOCIAL HISTORY:
He immigrated to the United States from East Asia as a child

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
Laboratory tests reveal mildly elevated alanine aminotransferase (ALT). Hepatitis B serologic testing shows:
HBsAg                                                               Positive
Anti-HBs (HBsAb)                                              Negative
Anti-HBc (HBcAb)                                              Positive (IgG)
HBeAg                                                               Negative
Anti-HBe                                                            Positive

## Encounter 1: source 2022-01-15, practice 2025-12-15, inpatient, Gastroenterology

Chief complaint: Progressive fatigue, abdominal distension, and unintentional weight loss over 2 months

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Marcus Johnson, MD
Department: Gastroenterology
Visit Type: inpatient

CHIEF COMPLAINT:
Progressive fatigue, abdominal distension, and unintentional weight loss over 2 months

HISTORY OF PRESENT ILLNESS:
The patient with chronic hepatitis B returns two years after routine outpatient management with a 2-month history of progressive fatigue, abdominal distension, and a 10-pound unintentional weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic hepatitis B infection (diagnosed 2020-01-15)
- Hypertension
- Vitamin D deficiency

He has a past medical history of chronic hepatitis B infection for which he has been intermittently compliant with antiviral therapy.

PAST SURGICAL HISTORY:
- Inguinal hernia repair (age 45)

MEDICATIONS:
Home Medications:
- Tenofovir disoproxil fumarate 300 mg by mouth once daily
- Amlodipine 5 mg by mouth once daily
- Cholecalciferol 2000 international units by mouth once daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hepatocellular carcinoma (age 71, deceased)
- Mother: Type 2 diabetes mellitus
- Older brother: Chronic hepatitis B infection

SOCIAL HISTORY:
He denies alcohol use

PHYSICAL EXAMINATION:
On physical examination, he is jaundiced with palpable hepatomegaly and a fluid wave is noted on abdominal examination

LABORATORY DATA:
- Serum α-fetoprotein (AFP): 800 ng/mL (Normal: < 10 ng/mL)
- Alanine aminotransferase (ALT): 90 U/L (Normal: 7-56 U/L)
- Aspartate aminotransferase (AST): 110 U/L (Normal: 10-40 U/L)
- Alkaline phosphatase (ALP): 300 U/L (Normal: 40-130 U/L)
- Total bilirubin: 3.2 mg/dL (Normal: 0.3-1.2 mg/dL)

IMAGING:
Abdominal ultrasound reveals a 4 cm mass in the right lobe of the liver with irregular borders. There is no evidence of gallstones or biliary ductal dilatation.
