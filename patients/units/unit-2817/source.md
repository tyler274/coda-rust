# synthetic_hospital patient 2817

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex F, Southeast Asian (Vietnamese), Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-04-03 (Wed) | outpatient | Gastroenterology |
| 1 | 2021-07-15 | 2025-10-10 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicaid",
 "occupation": "International aid worker",
 "alcohol_use": "None",
 "family_history": [
  "Paternal cousin with active pulmonary tuberculosis",
  "Father with hepatocellular carcinoma secondary to hepatitis B",
  "Mother with chronic hepatitis B infection"
 ],
 "race_ethnicity": "Southeast Asian (Vietnamese)",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "300 mg PO daily",
   "name": "Tenofovir disoproxil fumarate"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Chronic hepatitis B infection"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Chronic hepatitis B infection", "Tuberculosis"]
Comorbidities: ["Recent immigrant status with acculturation difficulty"]

## Encounter 0: source 2020-01-15, practice 2024-04-03, outpatient, Gastroenterology

Chief complaint: Abnormal liver function tests discovered on routine laboratory screening

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Abnormal liver function tests discovered on routine laboratory screening

HISTORY OF PRESENT ILLNESS:
She comes to the clinic for evaluation after routine laboratory tests revealed abnormal liver function. She recently immigrated from Southeast Asia and has never been vaccinated against hepatitis B. She reports feeling well without fatigue, jaundice, or abdominal pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic hepatitis B infection

MEDICATIONS:
Home Medications:
- Tenofovir disoproxil fumarate 300 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Paternal cousin with active pulmonary tuberculosis
- Father with hepatocellular carcinoma secondary to hepatitis B
- Mother with chronic hepatitis B infection

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: International aid worker.

LABORATORY DATA:
Laboratory studies show elevated alanine aminotransferase and aspartate aminotransferase. Hepatitis B serology reveals:
HBsAg Positive
Anti-HBs (HBsAb) Negative
Anti-HBc (HBcAb) Positive (IgG)
HBeAg Positive
Anti-HBe Negative

## Encounter 1: source 2021-07-15, practice 2025-10-10, ed, Emergency Medicine

Chief complaint: 2-month history of intermittent fevers, night sweats, and 5 kg unintentional weight loss

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-month history of intermittent fevers, night sweats, and 5 kg unintentional weight loss

HISTORY OF PRESENT ILLNESS:
This 46-year-old female with chronic hepatitis B presents for emergency care. She emigrated from Nigeria 6 months ago and has a history of close contact with a relative diagnosed with active pulmonary tuberculosis.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic hepatitis B infection (diagnosed 2020-01-15)

MEDICATIONS:
Home Medications:
- Tenofovir disoproxil fumarate 300 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Paternal cousin with active pulmonary tuberculosis
- Father with hepatocellular carcinoma secondary to hepatitis B
- Mother with chronic hepatitis B infection

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: International aid worker.

LABORATORY DATA:
A nucleic acid amplification test (NAAT) for Mycobacterium tuberculosis is performed, which has a sensitivity of 92% and specificity of 98%. The test returns positive. The laboratory provides a positive likelihood ratio (LR+) of 46 and reports the disease prevalence in the clinic population as 10%.

ASSESSMENT AND PLAN:
Her physician estimates her pre-test probability of tuberculosis is 30%.

The physician wants to determine how likely it is that this patient actually has active tuberculosis given her positive NAAT result.
