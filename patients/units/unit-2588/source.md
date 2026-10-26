# synthetic_hospital patient 2588

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 25 at first encounter, sex M, Non-Hispanic White, Private PPO insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2019-02-11 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2019-08-02 (Fri) | outpatient | Urology |
| 2 | 2021-01-15 | 2020-02-11 (Tue) | ed | Emergency Medicine |
| 3 | 2023-01-15 | 2022-02-04 (Fri) | ed | Emergency Medicine |
| 4 | 2027-01-15 | 2026-01-26 (Mon) | outpatient | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private PPO insurance",
 "occupation": "Marketing associate",
 "alcohol_use": "Social alcohol use (3-4 drinks per week)",
 "family_history": [
  "Father with hypertension (diagnosed age 45)",
  "Maternal grandmother with type 2 diabetes"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Non-smoker",
 "home_medications": [],
 "surgical_history": [
  "Wisdom teeth extraction, age 18"
 ],
 "chronic_conditions": [],
 "age_at_first_encounter": 25
}
```

Primary diagnoses: ["Epididymitis (antibiotic therapy)", "Gonococcal urethritis", "Primary syphilis", "Acute HIV infection"]
Comorbidities: ["Exposure to sexually transmitted infection", "High risk sexual behavior"]

## Encounter 0: source 2020-01-15, practice 2019-02-11, outpatient, Internal Medicine

Chief complaint: Painless ulcer on the base of the penile shaft for 3 weeks

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Painless ulcer on the base of the penile shaft for 3 weeks

HISTORY OF PRESENT ILLNESS:
He reports having unprotected sexual intercourse with a new partner approximately 5 weeks ago. He denies any discharge from the penis, testicular pain, or systemic symptoms such as fever or weight loss.

PAST MEDICAL HISTORY:
His past medical history is unremarkable

PAST SURGICAL HISTORY:
- Wisdom teeth extraction, age 18

ALLERGIES:
He has no known drug allergies.

FAMILY HISTORY:
- Father with hypertension (diagnosed age 45)
- Maternal grandmother with type 2 diabetes

SOCIAL HISTORY:
Smoking: Non-smoker. Alcohol: Social alcohol use (3-4 drinks per week). Occupation: Marketing associate.

PHYSICAL EXAMINATION:
On physical examination, there is a single, well-demarcated, nonexudative ulcer measuring 1 cm in diameter at the base of the penile shaft. There is no inguinal lymphadenopathy. The remainder of the genital examination is normal.

LABORATORY DATA:
Rapid plasma reagin (RPR) test returns positive. HIV test is negative.

Laboratory Values:
- RPR: Positive
- HIV antibody/antigen: Negative

## Encounter 1: source 2020-07-15, practice 2019-08-02, outpatient, Urology

Chief complaint: Burning sensation during urination and yellowish penile discharge for 3 days

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Burning sensation during urination and yellowish penile discharge for 3 days

HISTORY OF PRESENT ILLNESS:
The patient, with history of prior penile ulcer, reports having unprotected sexual intercourse with a new partner two weeks prior.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary syphilis (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- Wisdom teeth extraction, age 18

MEDICATIONS:
He takes no medications.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension (diagnosed age 45)
- Maternal grandmother with type 2 diabetes

SOCIAL HISTORY:
Smoking: Non-smoker. Alcohol: Social alcohol use (3-4 drinks per week). Occupation: Marketing associate.

VITAL SIGNS:
His vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, you note purulent discharge at the urethral meatus. There is no rash, lymphadenopathy, or joint swelling.

LABORATORY DATA:
A urine sample is collected, and the following results are obtained:

- Urine nucleic acid amplification test (NAAT) for Neisseria gonorrhoeae: Positive
- Urine NAAT for Chlamydia trachomatis: Negative
- Urinalysis: No evidence of urinary tract infection

ASSESSMENT AND PLAN:
Based on the patient's presentation and test results,

## Encounter 2: source 2021-01-15, practice 2020-02-11, ed, Emergency Medicine

Chief complaint: Severe left testicular pain progressive over 2 days with fever and penile discharge

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe left testicular pain progressive over 2 days with fever and penile discharge

HISTORY OF PRESENT ILLNESS:
The patient, with a history of primary syphilis diagnosed in January 2020 and gonococcal urethritis in July 2020, presents to the emergency department with a 2-day history of severe left testicular pain that has progressively worsened. He reports a low-grade fever and a clear discharge from his penis. He denies dysuria or back pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary syphilis (diagnosed 2020-01-15)
- Gonococcal urethritis (diagnosed 2020-07-15)

PAST SURGICAL HISTORY:
- Wisdom teeth extraction, age 18

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension (diagnosed age 45)
- Maternal grandmother with type 2 diabetes

SOCIAL HISTORY:
He is sexually active with multiple partners. He does not use condoms regularly.

PHYSICAL EXAMINATION:
On physical examination, the left testicle is markedly tender and swollen. The overlying scrotal skin appears erythematous but without any lesions or crepitus.

LABORATORY DATA:
Urinalysis shows 10-20 white blood cells per high-power field, and urine nucleic acid amplification testing is positive for Chlamydia trachomatis.

## Encounter 3: source 2023-01-15, practice 2022-02-04, ed, Emergency Medicine

Chief complaint: Fever, headache, and diffuse maculopapular rash for 1 week

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, headache, and diffuse maculopapular rash for 1 week

HISTORY OF PRESENT ILLNESS:
Patient with history of primary syphilis, gonococcal urethritis, and epididymitis presents to emergency department with a 1-week history of fever, headache, and diffuse maculopapular rash.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary syphilis (diagnosed 2020-01-15)
- Gonococcal urethritis (diagnosed 2020-07-15)
- Epididymitis (antibiotic therapy) (diagnosed 2021-01-15)

PAST SURGICAL HISTORY:
- Wisdom teeth extraction, age 18

MEDICATIONS:
He takes no medications.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension (diagnosed age 45)
- Maternal grandmother with type 2 diabetes

SOCIAL HISTORY:
He reports recent unprotected sexual encounters with multiple partners.

VITAL SIGNS:
On examination, he is febrile with a temperature of 38.7°C (101.7°F),

PHYSICAL EXAMINATION:
The rash is noted on his trunk and extremities. Neurological examination reveals nuchal rigidity.

LABORATORY DATA:
Laboratory tests show:

- White blood cell count: 9,000/mm³
- Hemoglobin: 14 g/dL
- Platelet count: 250,000/mm³
- Cerebrospinal fluid (CSF) analysis:
- WBC: 50/mm³ (lymphocytic predominance)
- Protein: 50 mg/dL
- Glucose: 60 mg/dL (serum glucose 90 mg/dL)
- Rapid plasma reagin (RPR) test: negative
- Monospot test: negative

## Encounter 4: source 2027-01-15, practice 2026-01-26, outpatient, Internal Medicine

Chief complaint: Painless penile ulcer present for 3 weeks

ENCOUNTER NOTE
Date: 2027-01-15
Provider: Dr. David Kim, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Painless penile ulcer present for 3 weeks

HISTORY OF PRESENT ILLNESS:
The patient, with a history of syphilis and HIV, returns reporting that the ulcer appeared 3 weeks ago and has not changed in size. He denies any discharge from the lesion.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary syphilis (diagnosed 2020-01-15)
- Gonococcal urethritis (diagnosed 2020-07-15)
- Epididymitis (antibiotic therapy) (diagnosed 2021-01-15)
- Acute HIV infection (diagnosed 2023-01-15)

PAST SURGICAL HISTORY:
- Wisdom teeth extraction, age 18

MEDICATIONS:
He takes no medications.

ALLERGIES:
The patient reports a severe allergy to penicillin, with anaphylaxis noted in his medical record.

FAMILY HISTORY:
- Father with hypertension (diagnosed age 45)
- Maternal grandmother with type 2 diabetes

SOCIAL HISTORY:
He admits to unprotected sexual encounters with multiple partners in the past 6 months.

PHYSICAL EXAMINATION:
On physical examination, there is a single, indurated, non-tender ulcer on the shaft of the penis with non-fluctuant, rubbery inguinal lymphadenopathy.

LABORATORY DATA:
Rapid plasma reagin (RPR) test returns positive with a titer of 1:64.
