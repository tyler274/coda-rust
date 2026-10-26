# synthetic_hospital patient 2301

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-09-23 (Tue) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2026-03-20 (Fri) | icu | Pulmonary & Critical Care |
| 2 | 2020-08-15 | 2026-04-28 (Tue) | icu | General Surgery |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Sulfa drugs (rash)"
 ],
 "insurance": "Medicare",
 "occupation": "Retired construction foreman",
 "alcohol_use": "Social, 1-2 drinks per week",
 "family_history": [
  "Father: Myocardial infarction at age 68",
  "Mother: Chronic obstructive pulmonary disease",
  "Brother: Ischemic stroke at age 66"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 48 pack-year history, quit 2 years ago",
 "home_medications": [
  {
   "dose": "18 mcg inhaled daily",
   "name": "Tiotropium bromide"
  },
  {
   "dose": "250/50 mcg inhaled twice daily",
   "name": "Fluticasone propionate/salmeterol"
  },
  {
   "dose": "90 mcg inhaled every 4-6 hours as needed for dyspnea",
   "name": "Albuterol sulfate"
  },
  {
   "dose": "10 mg orally daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg orally daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg orally daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Appendectomy (age 14)",
  "Left inguinal hernia repair (age 61)"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (GOLD Group C)",
  "Hypertension",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Acute cholecystitis", "Refractory hypoxemia in ARDS"]
Comorbidities: ["Chronic obstructive pulmonary disease", "Dependence on mechanical ventilation", "Acute ischemic stroke"]

## Encounter 0: source 2020-01-15, practice 2025-09-23, ed, Emergency Medicine

Chief complaint: 2-day history of fever and right upper quadrant abdominal pain

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-day history of fever and right upper quadrant abdominal pain

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (GOLD Group C)
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Appendectomy (age 14)
- Left inguinal hernia repair (age 61)

MEDICATIONS:
Home Medications:
- Tiotropium bromide 18 mcg inhaled daily
- Fluticasone propionate/salmeterol 250/50 mcg inhaled twice daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed for dyspnea
- Lisinopril 10 mg orally daily
- Atorvastatin 40 mg orally daily
- Aspirin 81 mg orally daily

ALLERGIES:
Sulfa drugs (rash)

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Chronic obstructive pulmonary disease
- Brother: Ischemic stroke at age 66

SOCIAL HISTORY:
Smoking: Former smoker, 48 pack-year history, quit 2 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
The patient is sedated but grimaces when the RUQ is palpated.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 15,000/mm³
- Hemoglobin: 12.2 g/dL
- Platelets: 250,000/mm³
- Aspartate aminotransferase (AST): 80 U/L
- Alanine aminotransferase (ALT): 75 U/L
- Alkaline phosphatase: 200 U/L
- Total bilirubin: 1.8 mg/dL
- Direct bilirubin: 1.2 mg/dL

IMAGING:
An abdominal ultrasound shows gallbladder wall thickening, distension, and pericholecystic fluid without evidence of gallstones.

## Encounter 1: source 2020-07-15, practice 2026-03-20, icu, Pulmonary & Critical Care

Chief complaint: acute hypoxemic respiratory failure

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Aisha Johnson, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Acute hypoxemic respiratory failure

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with COPD (GOLD Group C), hypertension, hyperlipidemia, and prior emergency department evaluation for acute cholecystitis six months prior presents with acute respiratory symptoms. He is intubated and mechanically ventilated due to worsening dyspnea and hypoxia.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute cholecystitis (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (GOLD Group C)
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Appendectomy (age 14)
- Left inguinal hernia repair (age 61)

MEDICATIONS:
Home Medications:
- Tiotropium bromide 18 mcg inhaled daily
- Fluticasone propionate/salmeterol 250/50 mcg inhaled twice daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed for dyspnea
- Lisinopril 10 mg orally daily
- Atorvastatin 40 mg orally daily
- Aspirin 81 mg orally daily

ALLERGIES:
Sulfa drugs (rash)

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Chronic obstructive pulmonary disease
- Brother: Ischemic stroke at age 66

SOCIAL HISTORY:
Smoking: Former smoker, 48 pack-year history, quit 2 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired construction foreman.

VITAL SIGNS:
His current ventilator settings are a tidal volume of 450 mL, respiratory rate of 16 breaths per minute, FiO2 of 60%, and PEEP of 5 cm H2O. Despite these settings, his oxygen saturation remains at 88%.

PHYSICAL EXAMINATION:
His physical examination reveals bilateral crackles and decreased breath sounds at the lung bases.

LABORATORY DATA:
His initial arterial blood gas (ABG) on 60% FiO2 shows a pH of 7.32, PaCO2 of 50 mmHg, PaO2 of 58 mmHg, and HCO3- of 24 mEq/L.

IMAGING:
Chest X-ray shows bilateral infiltrates consistent with acute respiratory distress syndrome (ARDS).

## Encounter 2: source 2020-08-15, practice 2026-04-28, icu, General Surgery

Chief complaint: Fever and new-onset right upper quadrant pain

ENCOUNTER NOTE
Date: 2020-08-15
Provider: Dr. Robert Kim, MD
Department: General Surgery
Visit Type: icu

CHIEF COMPLAINT:
Fever and new-onset right upper quadrant pain

HISTORY OF PRESENT ILLNESS:
The patient is a 67-year-old male with COPD (GOLD C), hypertension, hyperlipidemia, and episodes in January and July admitted to the ICU. Over the past 24 hours, the patient has developed a fever of 38.9°C, leukocytosis, and new-onset right upper quadrant (RUQ) pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute cholecystitis (diagnosed 2020-01-15)
- Refractory hypoxemia in ARDS (diagnosed 2020-07-15)
- Chronic obstructive pulmonary disease (GOLD Group C)
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Appendectomy (age 14)
- Left inguinal hernia repair (age 61)

MEDICATIONS:
Home Medications:
- Tiotropium bromide 18 mcg inhaled daily
- Fluticasone propionate/salmeterol 250/50 mcg inhaled twice daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed for dyspnea
- Lisinopril 10 mg orally daily
- Atorvastatin 40 mg orally daily
- Aspirin 81 mg orally daily

ALLERGIES:
Sulfa drugs (rash)

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Chronic obstructive pulmonary disease
- Brother: Ischemic stroke at age 66

SOCIAL HISTORY:
Smoking: Former smoker, 48 pack-year history, quit 2 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
The patient is sedated but grimaces when the RUQ is palpated. Murphy's sign is difficult to assess due to sedation.

LABORATORY DATA:
- White blood cell count: 16,000/mm³
- Hemoglobin: 13.2 g/dL
- Platelets: 210,000/mm³
- Aspartate aminotransferase (AST): 70 U/L
- Alanine aminotransferase (ALT): 65 U/L
- Alkaline phosphatase: 120 U/L
- Total bilirubin: 1.2 mg/dL
- Lipase: 22 U/L

IMAGING:
An abdominal ultrasound shows no evidence of gallstones, but the gallbladder wall is thickened with pericholecystic fluid.
