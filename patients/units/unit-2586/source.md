# synthetic_hospital patient 2586

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 30 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2026-01-25 (Sun) | inpatient | Internal Medicine |
| 1 | 2020-07-15 | 2026-08-04 (Tue) | outpatient | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Unemployed; applicant for military enlistment",
 "alcohol_use": "Social, 2-4 drinks on weekends",
 "family_history": [
  "Father: Alcohol use disorder, cirrhosis",
  "Mother: Major depressive disorder, generalized anxiety disorder",
  "Brother: Opioid use disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 3 pack-years, quit 18 months ago",
 "home_medications": [
  {
   "dose": "8mg/2mg sublingual daily",
   "name": "Buprenorphine/naloxone"
  },
  {
   "dose": "1mg oral daily",
   "name": "Folic acid"
  },
  {
   "dose": "100mg oral daily",
   "name": "Thiamine"
  },
  {
   "dose": "1 tablet daily",
   "name": "Multivitamin"
  }
 ],
 "surgical_history": [
  "Incision and drainage of right forearm abscess, 2022",
  "Incision and drainage of left gluteal abscess, 2020"
 ],
 "chronic_conditions": [
  "Opioid use disorder, active IV heroin use",
  "Chronic hepatitis C infection (genotype 1a)",
  "History of recurrent methicillin-sensitive Staphylococcus aureus (MSSA) skin abscesses",
  "Protein-calorie malnutrition"
 ],
 "age_at_first_encounter": 30
}
```

Primary diagnoses: ["Miliary tuberculosis", "Ethambutol-induced optic neuropathy"]
Comorbidities: ["Intravenous drug use (Substance use disorder)"]

## Encounter 0: source 2020-01-15, practice 2026-01-25, inpatient, Internal Medicine

Chief complaint: Intermittent fevers, night sweats, 10-pound weight loss, persistent cough with scant hemoptysis

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Intermittent fevers, night sweats, 10-pound weight loss, persistent cough with scant hemoptysis

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 3-week history of intermittent fevers, night sweats, and a 10-pound weight loss. He reports a persistent cough with scant hemoptysis. He denies recent travel or sick contacts.

PAST MEDICAL HISTORY:
Active Problem List:
- Opioid use disorder, active IV heroin use
- Chronic hepatitis C infection (genotype 1a)
- History of recurrent methicillin-sensitive Staphylococcus aureus (MSSA) skin abscesses
- Protein-calorie malnutrition

PAST SURGICAL HISTORY:
- Incision and drainage of right forearm abscess, 2022
- Incision and drainage of left gluteal abscess, 2020

MEDICATIONS:
Home Medications:
- Buprenorphine/naloxone 8mg/2mg sublingual daily
- Folic acid 1mg oral daily
- Thiamine 100mg oral daily
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder, cirrhosis
- Mother: Major depressive disorder, generalized anxiety disorder
- Brother: Opioid use disorder

SOCIAL HISTORY:
History of intravenous drug use

PHYSICAL EXAMINATION:
On examination, he is cachectic and febrile. Lung auscultation reveals diffuse rales.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 10.2 g/dL
- White blood cell count: 9,000/mm^3 with a normal differential
- Platelets: 350,000/mm^3
- HIV test: Negative
- Sputum acid-fast bacilli (AFB) stain: Positive

IMAGING:
A chest x-ray shows a diffuse reticulonodular pattern.

## Encounter 1: source 2020-07-15, practice 2026-08-04, outpatient, Internal Medicine

Chief complaint: Pre-enlistment physical examination required for military service

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Pre-enlistment physical examination required for military service

HISTORY OF PRESENT ILLNESS:
Patient with opioid use disorder and history of miliary tuberculosis returns for follow-up. He reports chronic cough for several months and night sweats.

PAST MEDICAL HISTORY:
Active Problem List:
- Miliary tuberculosis (diagnosed 2020-01-15)
- Opioid use disorder, active IV heroin use
- Chronic hepatitis C infection (genotype 1a)
- History of recurrent methicillin-sensitive Staphylococcus aureus (MSSA) skin abscesses
- Protein-calorie malnutrition

PAST SURGICAL HISTORY:
- Incision and drainage of right forearm abscess, 2022
- Incision and drainage of left gluteal abscess, 2020

MEDICATIONS:
Home Medications:
- Buprenorphine/naloxone 8mg/2mg sublingual daily
- Folic acid 1mg oral daily
- Thiamine 100mg oral daily
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder, cirrhosis
- Mother: Major depressive disorder, generalized anxiety disorder
- Brother: Opioid use disorder

SOCIAL HISTORY:
During the interview, he admits to frequent recreational drug use at parties, including stimulants and opioids.

REVIEW OF SYSTEMS:
He does not report any hallucinations, recent head injuries, or changes in pupil size.

PHYSICAL EXAMINATION:
On vision screening, he is unable to distinguish the number within the yellow circle below:
His pupils are equal and reactive to light, and extraocular movements are intact. No nystagmus or conjunctival injection is observed.
