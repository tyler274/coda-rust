# synthetic_hospital patient 2102

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 48 at first encounter, sex M, White, non-Hispanic, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-10-23 (Mon) | outpatient | Gastroenterology |
| 1 | 2020-04-15 | 2024-01-26 (Fri) | outpatient | Dermatology |
| 2 | 2020-07-15 | 2024-04-19 (Fri) | telehealth | Dermatology |
| 3 | 2022-01-15 | 2025-10-20 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Office administrator",
 "alcohol_use": "None",
 "family_history": [
  "Father with coronary artery disease",
  "Mother with hypertension"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "5 mg orally twice daily",
   "name": "Tacrolimus"
  },
  {
   "dose": "10 mg orally daily",
   "name": "Prednisone"
  },
  {
   "dose": "800/160 mg orally three times weekly",
   "name": "Sulfamethoxazole-trimethoprim"
  }
 ],
 "surgical_history": [
  "Orthotopic liver transplantation (age 47)",
  "Packed red blood cell transfusion secondary to trauma (1990)"
 ],
 "chronic_conditions": [
  "Chronic hepatitis C infection",
  "Status post orthotopic liver transplantation",
  "Lichen planus",
  "History of Pneumocystis jirovecii pneumonia",
  "Opioid use disorder, in sustained remission"
 ],
 "age_at_first_encounter": 48
}
```

Primary diagnoses: ["History of Pneumocystis jirovecii pneumonia", "Chronic hepatitis C", "Lichen planus"]
Comorbidities: ["Chronic hepatitis C"]

## Encounter 0: source 2020-01-15, practice 2023-10-23, outpatient, Gastroenterology

Chief complaint: Persistent fatigue and muscle aches gradually worsening over the past several months with difficulty concentrating at work

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue and muscle aches gradually worsening over the past several months with difficulty concentrating at work

HISTORY OF PRESENT ILLNESS:
That have been gradually worsening over the past several months. He reports that he has not been able to concentrate at work and often feels "wiped out" even after a full night’s sleep. He denies fever, weight loss, or recent travel.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic hepatitis C infection
- Status post orthotopic liver transplantation
- Lichen planus
- History of Pneumocystis jirovecii pneumonia
- Opioid use disorder, in sustained remission

His past medical history is significant for intravenous heroin use in his 20s, but he has been sober for over a decade. He also received a blood transfusion following a car accident in 1990.

PAST SURGICAL HISTORY:
- Orthotopic liver transplantation (age 47)
- Packed red blood cell transfusion secondary to trauma (1990)

MEDICATIONS:
Is.

Home Medications:
- Tacrolimus 5 mg orally twice daily
- Prednisone 10 mg orally daily
- Sulfamethoxazole-trimethoprim 800/160 mg orally three times weekly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with coronary artery disease
- Mother with hypertension

SOCIAL HISTORY:
He does not drink alcohol

PHYSICAL EXAMINATION:
On exam, he has mild right upper quadrant tenderness without hepatomegaly or jaundice. The remainder of the physical exam is normal.

LABORATORY DATA:
Laboratory testing reveals mildly elevated aminotransferase levels, but bilirubin, alkaline phosphatase, and coagulation studies are within normal limits.

PLAN:
Serologic testing is ordered for further evaluation.

## Encounter 1: source 2020-04-15, practice 2024-01-26, outpatient, Dermatology

Chief complaint: Itchy rash on wrists and ankles that has developed gradually over the past several months

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Okonkwo, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Itchy rash on wrists and ankles that has developed gradually over the past several months

HISTORY OF PRESENT ILLNESS:
The transplant recipient returns after January evaluation with rash that has developed gradually over the past several months. He denies recent medication changes, fever, or systemic symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic hepatitis C (diagnosed 2020-01-15)
- Chronic hepatitis C infection
- Status post orthotopic liver transplantation
- Lichen planus
- History of Pneumocystis jirovecii pneumonia
- Opioid use disorder, in sustained remission

PAST SURGICAL HISTORY:
- Orthotopic liver transplantation (age 47)
- Packed red blood cell transfusion secondary to trauma (1990)

MEDICATIONS:
Home Medications:
- Tacrolimus 5 mg orally twice daily
- Prednisone 10 mg orally daily
- Sulfamethoxazole-trimethoprim 800/160 mg orally three times weekly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with coronary artery disease
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Office administrator.

PHYSICAL EXAMINATION:
On exam, he has violaceous, flat-topped papules with fine white lines on their surface, and similar lesions are noted in his oral mucosa.

LABORATORY DATA:
His liver function tests are mildly elevated.

PATHOLOGY:
A skin biopsy shows a band-like lymphocytic infiltrate at the dermal-epidermal junction with sawtoothing of the rete ridges.

PLAN:
The patient is reassured that his skin condition typically resolves over time without scarring.

## Encounter 2: source 2020-07-15, practice 2024-04-19, telehealth, Dermatology

Chief complaint: Follow-up evaluation of persistent itchy rash on wrists and ankles

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Okonkwo, MD
Department: Dermatology
Visit Type: telehealth

CHIEF COMPLAINT:
Follow-up evaluation of persistent itchy rash on wrists and ankles

HISTORY OF PRESENT ILLNESS:
The patient returns via telehealth for follow-up evaluation of wrist and ankle rash initially assessed in April 2020. Status post orthotopic liver transplantation on immunosuppression, he reports the rash has developed gradually over the past several months. He denies recent medication changes, fever, or systemic symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic hepatitis C (diagnosed 2020-01-15)
- Lichen planus (diagnosed 2020-04-15)
- Chronic hepatitis C infection
- Status post orthotopic liver transplantation
- History of Pneumocystis jirovecii pneumonia
- Opioid use disorder, in sustained remission

PAST SURGICAL HISTORY:
- Orthotopic liver transplantation (age 47)
- Packed red blood cell transfusion secondary to trauma (1990)

MEDICATIONS:
Home Medications:
- Tacrolimus 5 mg orally twice daily
- Prednisone 10 mg orally daily
- Sulfamethoxazole-trimethoprim 800/160 mg orally three times weekly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with coronary artery disease
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Office administrator.

PHYSICAL EXAMINATION:
On exam, he has violaceous, flat-topped papules with fine white lines on their surface, and similar lesions are noted in his oral mucosa.

LABORATORY DATA:
His liver function tests are mildly elevated.

PATHOLOGY:
A skin biopsy shows a band-like lymphocytic infiltrate at the dermal-epidermal junction with sawtoothing of the rete ridges.

PLAN:
The patient is reassured that his skin condition typically resolves over time without scarring.

## Encounter 3: source 2022-01-15, practice 2025-10-20, ed, Emergency Medicine

Chief complaint: Fever, cough, and shortness of breath for 3 days

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James Patterson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, cough, and shortness of breath for 3 days

HISTORY OF PRESENT ILLNESS:
The patient is a 50-year-old liver transplant recipient with a history of Pneumocystis jirovecii pneumonia. He presents to the emergency department with a 3-day history of fever, cough, and shortness of breath.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic hepatitis C (diagnosed 2020-01-15)
- Lichen planus (diagnosed 2020-04-15)
- Chronic hepatitis C infection
- Status post orthotopic liver transplantation
- History of Pneumocystis jirovecii pneumonia
- Opioid use disorder, in sustained remission

His post-transplant course has been complicated by episodes of acute rejection,.

PAST SURGICAL HISTORY:
He underwent a liver transplantation 4 months ago due to end-stage liver disease secondary to hepatitis C.

MEDICATIONS:
He takes for which he is on high-dose tacrolimus.

Home Medications:
- Prednisone 10 mg orally daily
- Sulfamethoxazole-trimethoprim 800/160 mg orally three times weekly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with coronary artery disease
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Office administrator.

VITAL SIGNS:
His temperature is 38.7°C (101.7°F), blood pressure is 130/85 mmHg, pulse is 110/min, and respiratory rate is 22/min.

PHYSICAL EXAMINATION:
On physical examination, he has diffuse crackles in both lung fields.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 12,000/µL (normal 4,500-11,000)
- Hemoglobin: 13.2 g/dL (normal 13.8-17.2)
- Platelets: 250,000/µL (normal 150,000-450,000)
- Serum creatinine: 1.1 mg/dL (normal 0.6-1.3)
- Blood cultures: No growth to date

IMAGING:
Chest X-ray shows bilateral interstitial infiltrates.
