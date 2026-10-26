# synthetic_hospital patient 2573

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 22 at first encounter, sex M, White, non-Hispanic, University student health insurance plan.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-01-21 (Tue) | outpatient | Family Medicine |
| 1 | 2020-07-15 | 2025-07-28 (Mon) | outpatient | Family Medicine |
| 2 | 2021-07-15 | 2026-07-25 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "University student health insurance plan",
 "occupation": "University undergraduate student",
 "alcohol_use": "Social alcohol use (2-3 drinks on weekends)",
 "family_history": [
  "Father: Hypertension (diagnosed age 45)",
  "Mother: Major depressive disorder"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [],
 "age_at_first_encounter": 22
}
```

Primary diagnoses: ["Gonococcal proctitis", "Gonococcal urethritis", "Primary syphilis"]
Comorbidities: ["High risk sexual behavior"]

## Encounter 0: source 2020-01-15, practice 2025-01-21, outpatient, Family Medicine

Chief complaint: Burning sensation during urination and white discharge from penis for 3 days

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Family Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Burning sensation during urination and white discharge from penis for 3 days

HISTORY OF PRESENT ILLNESS:
That started 3 days ago. He reports having unprotected sexual intercourse with a new partner two weeks prior.

PAST MEDICAL HISTORY:
His past medical history is unremarkable,

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes no medications.

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension (diagnosed age 45)
- Mother: Major depressive disorder

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social alcohol use (2-3 drinks on weekends). Occupation: University undergraduate student.

PHYSICAL EXAMINATION:
On physical examination, there is no rash, lymphadenopathy, or joint swelling. Genital examination reveals clear discharge from the urethra. No penile lesions are noted, and the testes are non-tender without swelling.

LABORATORY DATA:
Urinalysis shows:
- Leukocyte esterase: Positive
- Nitrites: Negative
- WBC: 10-15/hpf
- RBC: 0-2/hpf
Nucleic acid amplification testing (NAAT) results are pending.

## Encounter 1: source 2020-07-15, practice 2025-07-28, outpatient, Family Medicine

Chief complaint: Painless ulcer on penis that appeared 3 weeks after unprotected sexual intercourse with a new partner

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Family Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Painless ulcer on penis that appeared 3 weeks after unprotected sexual intercourse with a new partner

HISTORY OF PRESENT ILLNESS:
The patient is a 22-year-old male who returns six months after prior evaluation for burning sensation during urination and white discharge from penis. He reports that the ulcer appeared 3 weeks after he had unprotected sexual intercourse with a new partner. He denies any other symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Gonococcal urethritis (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- None

ALLERGIES:
He has no known drug allergies

FAMILY HISTORY:
- Father: Hypertension (diagnosed age 45)
- Mother: Major depressive disorder

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social alcohol use (2-3 drinks on weekends). Occupation: University undergraduate student.

PHYSICAL EXAMINATION:
On physical examination, there is a single, well-demarcated, indurated, painless ulcer on the shaft of his penis. There is no inguinal lymphadenopathy. The remainder of the examination is unremarkable.

LABORATORY DATA:
Laboratory tests are ordered:

- Rapid plasma reagin (RPR): Positive
- HIV antigen/antibody combination assay: Negative
- Hepatitis B surface antigen: Negative
- Hepatitis C antibody: Negative
- Fluorescent treponemal antibody absorption (FTA-ABS): Pending

## Encounter 2: source 2021-07-15, practice 2026-07-25, ed, Emergency Medicine

Chief complaint: Rectal pain, mucopurulent discharge, and intermittent rectal bleeding with tenesmus for 1 week

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Priya Sharma, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Rectal pain, mucopurulent discharge, and intermittent rectal bleeding with tenesmus for 1 week

HISTORY OF PRESENT ILLNESS:
The patient, with history of gonococcal urethritis and primary syphilis, reports mucopurulent anal discharge and intermittent rectal bleeding. He also complains of anal pruritus and has experienced pain during defecation.

PAST MEDICAL HISTORY:
Active Problem List:
- Gonococcal urethritis (diagnosed 2020-01-15)
- Primary syphilis (diagnosed 2020-07-15)

PAST SURGICAL HISTORY:
- None

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension (diagnosed age 45)
- Mother: Major depressive disorder

SOCIAL HISTORY:
The patient has a history of unprotected sexual intercourse with multiple partners, including receptive anal intercourse. His last sexual encounter was approximately 2 weeks ago.

VITAL SIGNS:
The patient's vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, the patient appears uncomfortable. Anorectal examination reveals erythema and tenderness of the anal canal, with a small amount of purulent discharge. No masses are palpable, and the prostate is non-tender on digital rectal examination.

LABORATORY DATA:
Laboratory tests are ordered, and the results are as follows:

- White blood cell count: 12,000 /µL
- Neisseria gonorrhoeae nucleic acid amplification test (NAAT) from rectal swab: Positive
- Rapid plasma reagin (RPR): Non-reactive
- HIV 1/2 antigen/antibody combination immunoassay: Non-reactive
