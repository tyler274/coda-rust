# synthetic_hospital patient 1845

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, Non-Hispanic White, Blue Cross PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2026-01-27 (Tue) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2026-05-01 (Fri) | outpatient | Internal Medicine |
| 2 | 2020-06-15 | 2026-06-16 (Tue) | follow_up | Urology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Social (1-2 drinks per weekend)",
 "family_history": [
  "Mother with recurrent UTIs",
  "Father with hypertension",
  "Maternal grandmother with type 2 diabetes"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "200 mg PO PRN dysmenorrhea (1-2 days per month)",
   "name": "Ibuprofen"
  }
 ],
 "surgical_history": [
  "Tonsillectomy, age 7"
 ],
 "chronic_conditions": [],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Uncomplicated cystitis", "Hematuria"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2026-01-27, outpatient, Internal Medicine

Chief complaint: Dysuria, increased urinary frequency, and urgency for 2 days

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Dysuria, increased urinary frequency, and urgency for 2 days

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with a 2-day history of . She denies vaginal discharge or bleeding, flank pain, fever, or chills. Her last menstrual period was 3 weeks ago.

PAST MEDICAL HISTORY:
She has no significant past medical history

PAST SURGICAL HISTORY:
- Tonsillectomy, age 7

MEDICATIONS:
Home Medications:
- Ibuprofen 200 mg PO PRN dysmenorrhea (1-2 days per month)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with recurrent UTIs
- Father with hypertension
- Maternal grandmother with type 2 diabetes

SOCIAL HISTORY:
She is sexually active with one partner and uses condoms inconsistently.

VITAL SIGNS:
On physical examination, her vital signs are within normal limits.

PHYSICAL EXAMINATION:
Abdominal examination reveals suprapubic tenderness without rebound or guarding.

LABORATORY DATA:
A urine dipstick is negative for leukocyte esterase and nitrites. A urine pregnancy test is negative.

ASSESSMENT AND PLAN:
Given the negative urine dipstick,

## Encounter 1: source 2020-04-15, practice 2026-05-01, outpatient, Internal Medicine

Chief complaint: Dysuria, increased urinary frequency, and suprapubic pain for 2 days

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Dysuria, increased urinary frequency, and suprapubic pain for 2 days

HISTORY OF PRESENT ILLNESS:
The patient returns with symptoms similar to January 2020. She reports 2-day history of dysuria, increased urinary frequency, and suprapubic pain without fever, back pain, or vaginal discharge.

PAST MEDICAL HISTORY:
Active Problem List:
- Uncomplicated cystitis (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- Tonsillectomy, age 7

MEDICATIONS:
She takes no medications other than an occasional ibuprofen for menstrual cramps

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with recurrent UTIs
- Father with hypertension
- Maternal grandmother with type 2 diabetes

SOCIAL HISTORY:
She is sexually active with one partner and uses a diaphragm with spermicidal gel for contraception. Her last menstrual period was 2 weeks ago.

VITAL SIGNS:
Her vital signs are within normal limits

PHYSICAL EXAMINATION:
Abdominal examination reveals mild suprapubic tenderness. There is no costovertebral angle tenderness.

LABORATORY DATA:
Urinalysis shows:
- Leukocyte esterase: Positive
- Nitrites: Positive
- WBC: 15-20/high-power field
- RBC: 0-2/high-power field
- Bacteria: Many Gram-negative rods

## Encounter 2: source 2020-06-15, practice 2026-06-16, follow_up, Urology

Chief complaint: Follow-up visit for persistent pink urine 6 weeks after UTI treatment despite resolution of dysuria and urgency

ENCOUNTER NOTE
Date: 2020-06-15
Provider: Dr. Robert Okonkwo, MD
Department: Urology
Visit Type: follow_up

CHIEF COMPLAINT:
Follow-up visit for persistent pink urine 6 weeks after UTI treatment despite resolution of dysuria and urgency

HISTORY OF PRESENT ILLNESS:
The patient is a 28-year-old female with a history of recurrent cystitis treated in January and April 2020 who returns for follow-up six weeks after her most recent UTI treatment. She reports noticing pink urine since yesterday. She denies recent sexual activity, vaginal discharge, or use of new personal hygiene products. She reports that her UTI symptoms resolved with treatment, but she still occasionally notices pink urine.

PAST MEDICAL HISTORY:
Active Problem List:
- Uncomplicated cystitis (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- Tonsillectomy, age 7

MEDICATIONS:
Home Medications:
- Ibuprofen 200 mg PO PRN dysmenorrhea (1-2 days per month)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with recurrent UTIs
- Father with hypertension
- Maternal grandmother with type 2 diabetes

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per weekend). Occupation: Elementary school teacher.

VITAL SIGNS:
Her vital signs are within normal limits

PHYSICAL EXAMINATION:
Her abdominal exam reveals suprapubic tenderness without costovertebral angle tenderness

LABORATORY DATA:
A urinalysis shows: - pH: 6.0 - Specific gravity: 1.025 - Leukocyte esterase: Positive - Nitrites: Positive - Red blood cells: 25/hpf - White blood cells: 10/hpf - No casts or crystals. A urine culture is pending. Repeat urinalysis shows persistent hematuria with 20 red blood cells/hpf. There are no white blood cells, nitrites, or leukocyte esterase present.

ASSESSMENT AND PLAN:
The patient is diagnosed with a urinary tract infection (UTI)

Prescribed a course of antibiotics
