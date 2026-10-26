# synthetic_hospital patient 2111

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 44 at first encounter, sex M, Non-Hispanic White, Aetna Better Health (Commercial PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-10-22 (Tue) | outpatient | Internal Medicine |
| 1 | 2021-05-15 | 2026-02-16 (Mon) | outpatient | Dermatology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Aetna Better Health (Commercial PPO)",
 "occupation": "Insurance Claims Adjuster",
 "alcohol_use": "Social, 2-3 beers on weekends",
 "family_history": [
  "Father: Type 2 diabetes mellitus, diagnosed age 52",
  "Mother: Hypertension and hyperlipidemia",
  "Maternal aunt: Plaque psoriasis",
  "Paternal grandfather: Myocardial infarction at age 68"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 3 years ago (15 pack-year history)",
 "home_medications": [
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg PO daily at bedtime",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "Cholecystectomy (laparoscopic), 2015",
  "Left knee arthroscopy (meniscal repair), 2010"
 ],
 "chronic_conditions": [
  "Metabolic syndrome",
  "Hypertension",
  "Dyslipidemia",
  "Obesity (Class I)"
 ],
 "age_at_first_encounter": 44
}
```

Primary diagnoses: ["Plaque psoriasis"]
Comorbidities: ["Metabolic syndrome"]

## Encounter 0: source 2020-01-15, practice 2024-10-22, outpatient, Internal Medicine

Chief complaint: Presents to the clinic as part of a metabolic syndrome research study.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Aisha Johnson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Presents to the clinic as part of a metabolic syndrome research study.

PAST MEDICAL HISTORY:
Active Problem List:
- Metabolic syndrome
- Hypertension
- Dyslipidemia
- Obesity (Class I)

PAST SURGICAL HISTORY:
- Cholecystectomy (laparoscopic), 2015
- Left knee arthroscopy (meniscal repair), 2010

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, diagnosed age 52
- Mother: Hypertension and hyperlipidemia
- Maternal aunt: Plaque psoriasis
- Paternal grandfather: Myocardial infarction at age 68

SOCIAL HISTORY:
Smoking: Former smoker, quit 3 years ago (15 pack-year history). Alcohol: Social, 2-3 beers on weekends. Occupation: Insurance Claims Adjuster.

OTHER STUDIES:
The study collects the following data on each participant: 1) fasting plasma glucose in mg/dL, 2) self-reported pain on a scale of 0 (no pain) to 10 (worst pain), 3) gender (male or female), 4) 2CK exam score (scaled 120 to 300, minimum possible score 120), and 5) blood pressure in mmHg. The principal investigator asks a medical student to classify each variable according to the most appropriate scale: nominal, ordinal, interval, or ratio.

## Encounter 1: source 2021-05-15, practice 2026-02-16, outpatient, Dermatology

Chief complaint: Presents to the clinic with a rash that has gradually worsened over the past 4 months.

ENCOUNTER NOTE
Date: 2021-05-15
Provider: Dr. Rajesh Patel, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Presents to the clinic with a rash that has gradually worsened over the past 4 months.

HISTORY OF PRESENT ILLNESS:
This 45-year-old male with metabolic syndrome, hypertension, dyslipidemia, and obesity class I, previously seen in January 2020 for metabolic syndrome assessment, presents to the clinic with a rash that has gradually worsened over the past 4 months. It initially appeared on his elbows and knees and later spread to his scalp and lower back. The lesions are mildly itchy but not painful. He denies joint pain, fever, weight loss, or new medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Metabolic syndrome
- Hypertension
- Dyslipidemia
- Obesity (Class I)

He has a history of metabolic syndrome.

PAST SURGICAL HISTORY:
- Cholecystectomy (laparoscopic), 2015
- Left knee arthroscopy (meniscal repair), 2010

MEDICATIONS:
He takes lisinopril and atorvastatin.

ALLERGIES:
NKDA

FAMILY HISTORY:
His father had a similar rash in middle age.

SOCIAL HISTORY:
Smoking: Former smoker, quit 3 years ago (15 pack-year history). Alcohol: Social, 2-3 beers on weekends. Occupation: Insurance Claims Adjuster.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
Physical examination reveals well-circumscribed erythematous plaques with overlying silvery-white scale on the extensor surfaces, scalp, and lower back. Nail pitting is observed. No mucosal or systemic findings are present.
