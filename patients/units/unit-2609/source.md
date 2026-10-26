# synthetic_hospital patient 2609

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 34 at first encounter, sex M, White, Eastern European descent, Private PPO (Physician group plan).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-06-22 (Thu) | inpatient | Neurology |
| 1 | 2023-01-15 | 2026-07-06 (Mon) | outpatient | Psychiatry |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Private PPO (Physician group plan)",
 "occupation": "Family Medicine Physician",
 "alcohol_use": "Social, 1-2 drinks monthly",
 "family_history": [
  "Hypertension, father",
  "Hypothyroidism, mother",
  "No known hereditary neurological disorders"
 ],
 "race_ethnicity": "White, Eastern European descent",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "300 mg PO TID",
   "name": "Gabapentin"
  },
  {
   "dose": "500 mg PO PRN for pain",
   "name": "Acetaminophen"
  },
  {
   "dose": "100 mg PO daily",
   "name": "Docusate sodium"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Neurosyphilis with CNS involvement (Tabes dorsalis)",
  "History of tertiary syphilis infection"
 ],
 "age_at_first_encounter": 34
}
```

Primary diagnoses: ["Argyll-Robertson pupils", "Assessment of decision-making capacity"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2023-06-22, inpatient, Neurology

Chief complaint: Difficulty walking and intermittent blurred vision

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: inpatient

CHIEF COMPLAINT:
Difficulty walking and intermittent blurred vision

HISTORY OF PRESENT ILLNESS:
He presents in late summer with a 10-day history of difficulty walking and intermittent blurred vision. He reports traveling extensively across Eastern Europe and engaging in unprotected sexual activity.

PAST MEDICAL HISTORY:
Active Problem List:
- Neurosyphilis with CNS involvement (Tabes dorsalis)
- History of tertiary syphilis infection

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Gabapentin 300 mg PO TID
- Acetaminophen 500 mg PO PRN for pain
- Docusate sodium 100 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Hypertension, father
- Hypothyroidism, mother
- No known hereditary neurological disorders

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks monthly. Occupation: Family Medicine Physician.

PHYSICAL EXAMINATION:
Neurological examination reveals a positive Romberg sign and impaired joint position sense in the lower limbs. He has no muscle weakness or spasticity, and deep tendon reflexes are diminished. Ophthalmological assessment shows pupils that constrict when focusing on a near object but do not constrict in response to direct light. Extraocular movements are intact, and there is no ptosis or nystagmus. Cerebellar testing is normal, and there are no facial asymmetries.

LABORATORY DATA:
Serologies for Borrelia burgdorferi are negative.

IMAGING:
MRI of the brain is unremarkable.

## Encounter 1: source 2023-01-15, practice 2026-07-06, outpatient, Psychiatry

Chief complaint: Wonders if it would now be appropriate to ask her out on a date

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Marcus Johnson, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Wonders if it would now be appropriate to ask her out on a date

HISTORY OF PRESENT ILLNESS:
The patient is a 37-year-old male with neurosyphilis with CNS involvement (Tabes dorsalis) and tertiary syphilis, previously hospitalized in 2020 for difficulty walking, intermittent blurred vision, and Argyll-Robertson pupils, returning for follow-up, is having lunch with a colleague and explains that he recently ran into a woman he used to treat as a patient. He mentions he treated her for two episodes of acute sinusitis and one episode of a non-specific rash, all over a 3-month period about two years ago. They have not had any clinical contact since. He tells his colleague that they had a pleasant conversation.

PAST MEDICAL HISTORY:
Active Problem List:
- Argyll-Robertson pupils (diagnosed 2020-01-15)
- Neurosyphilis with CNS involvement (Tabes dorsalis)
- History of tertiary syphilis infection

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Gabapentin 300 mg PO TID
- Acetaminophen 500 mg PO PRN for pain
- Docusate sodium 100 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Hypertension, father
- Hypothyroidism, mother
- No known hereditary neurological disorders

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks monthly. Occupation: Family Medicine Physician.
