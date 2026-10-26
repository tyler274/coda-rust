# synthetic_hospital patient 2599

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex M, African American, Private (employer-sponsored PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-06-19 (Wed) | outpatient | Rheumatology |
| 1 | 2022-01-15 | 2026-06-06 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private (employer-sponsored PPO)",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Mother: Systemic lupus erythematosus (diagnosed age 34)",
  "Maternal aunt: Rheumatoid arthritis",
  "Father: Hypertension"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "200 mg PO daily",
   "name": "Hydroxychloroquine"
  },
  {
   "dose": "400 mg PO every 8 hours PRN joint pain",
   "name": "Ibuprofen"
  },
  {
   "dose": "2000 IU PO daily",
   "name": "Vitamin D3"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Systemic lupus erythematosus",
  "History of frequent upper respiratory infections since childhood"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Systemic lupus erythematosus with arthropathy", "Poststreptococcal glomerulonephritis"]
Comorbidities: ["Upper respiratory tract infection"]

## Encounter 0: source 2020-01-15, practice 2024-06-19, outpatient, Rheumatology

Chief complaint: 3-month history of recurrent, painful skin lesions and mouth ulcers

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Amara Okafor, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of recurrent, painful skin lesions and mouth ulcers

HISTORY OF PRESENT ILLNESS:
He also reports episodes of joint pain and swelling in his fingers and knees, which improve with rest.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus
- History of frequent upper respiratory infections since childhood

His medical history is significant for frequent respiratory infections since childhood.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO daily
- Ibuprofen 400 mg PO every 8 hours PRN joint pain
- Vitamin D3 2000 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Systemic lupus erythematosus (diagnosed age 34)
- Maternal aunt: Rheumatoid arthritis
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
Physical examination reveals several erythematous maculopapular lesions on his elbows, as well as oral aphthous ulcers. There are no genital ulcers.

LABORATORY DATA:
Laboratory studies show an elevated ESR, and his serum complement C3 is low.

OTHER STUDIES:
Genetic testing reveals a mutation in the C1q gene.

## Encounter 1: source 2022-01-15, practice 2026-06-06, ed, Emergency Medicine

Chief complaint: Cola-colored urine and periorbital edema

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Cola-colored urine and periorbital edema

HISTORY OF PRESENT ILLNESS:
This 30-year-old male with systemic lupus erythematosus and frequent childhood upper respiratory infections presents with a 2-day history of cola-colored urine and periorbital edema, reports a recent upper respiratory tract infection resolved 1 week ago, and denies pain on urination or recent trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus with arthropathy (diagnosed 2020-01-15)
- Systemic lupus erythematosus
- History of frequent upper respiratory infections since childhood

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO daily
- Ibuprofen 400 mg PO every 8 hours PRN joint pain
- Vitamin D3 2000 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Systemic lupus erythematosus (diagnosed age 34)
- Maternal aunt: Rheumatoid arthritis
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
His blood pressure is 145/90 mmHg

PHYSICAL EXAMINATION:
He has mild bilateral periorbital edema

LABORATORY DATA:
Urinalysis reveals the following:

- Color: Dark brown
- Specific gravity: 1.025
- pH: 6.0
- Protein: 2+
- Blood: 3+
- RBCs: Dysmorphic
- RBC casts: Present
- WBCs: 2-5/hpf
- No bacteria or crystals
