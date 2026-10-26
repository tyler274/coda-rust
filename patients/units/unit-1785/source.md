# synthetic_hospital patient 1785

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 26 at first encounter, sex F, White, University student health insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-10-27 (Wed) | outpatient | Family Medicine |
| 1 | 2020-07-15 | 2022-05-02 (Mon) | outpatient | Internal Medicine |
| 2 | 2021-01-15 | 2022-10-28 (Fri) | outpatient | Dermatology |
| 3 | 2021-07-15 | 2023-05-05 (Fri) | outpatient | Dermatology |
| 4 | 2022-01-15 | 2023-10-31 (Tue) | ed | Emergency Medicine |
| 5 | 2022-07-15 | 2024-04-29 (Mon) | inpatient | Neurology |
| 6 | 2023-01-15 | 2024-10-25 (Fri) | ed | Obstetrics & Gynecology |
| 7 | 2024-01-15 | 2025-11-04 (Tue) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "University student health insurance",
 "occupation": "Graduate student",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Paternal uncle with cerebral arteriovenous malformation",
  "Mother with hypertension"
 ],
 "race_ethnicity": "White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 tablet PO daily",
   "name": "Ethinyl estradiol/norgestimate"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Recurrent oral herpes simplex virus infection",
  "Genital herpes simplex virus infection",
  "History of chlamydia infection (treated 2 years prior)",
  "Intermittent tension-type headaches"
 ],
 "age_at_first_encounter": 26
}
```

Primary diagnoses: ["Herpes Simplex Virus Infection", "Cerebral arteriovenous malformation", "Herpes Zoster", "Acute urticaria", "Guillain-Barr\u00e9 syndrome", "Acute bacterial cystitis", "Amphetamine intoxication", "Ectopic pregnancy (left tubal)"]
Comorbidities: ["Fever", "History of intravenous drug use", "Psychological stress", "Acute pharyngitis"]

## Encounter 0: source 2020-01-15, practice 2021-10-27, outpatient, Family Medicine

Chief complaint: Painful vesicular lesions on her lower lip

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Family Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Painful vesicular lesions on her lower lip

HISTORY OF PRESENT ILLNESS:
That appeared after a week of fever and sore throat. She reports similar outbreaks in the past, usually triggered by stress or illness.

PAST MEDICAL HISTORY:
Active Problem List:
- Recurrent oral herpes simplex virus infection
- Genital herpes simplex virus infection
- History of chlamydia infection (treated 2 years prior)
- Intermittent tension-type headaches

MEDICATIONS:
Home Medications:
- Ethinyl estradiol/norgestimate 1 tablet PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Paternal uncle with cerebral arteriovenous malformation
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Graduate student.

PHYSICAL EXAMINATION:
Physical examination reveals clusters of small vesicles on an erythematous base along the vermillion border.

LABORATORY DATA:
A swab from the lesion confirms herpes simplex virus (HSV) infection.

PLAN:
She is prescribed valacyclovir.

## Encounter 1: source 2020-07-15, practice 2022-05-02, outpatient, Internal Medicine

Chief complaint: Painful urination and blood in urine

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Painful urination and blood in urine

HISTORY OF PRESENT ILLNESS:
This 26-year-old female with recurrent oral and genital herpes simplex virus infection and prior chlamydia, last evaluated in January for labial vesicular lesions, presents with urinary complaints. She reports a fever of 38.2°C and lower abdominal pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Herpes Simplex Virus Infection (diagnosed 2020-01-15)
- Recurrent oral herpes simplex virus infection
- Genital herpes simplex virus infection
- History of chlamydia infection (treated 2 years prior)
- Intermittent tension-type headaches

MEDICATIONS:
She takes uses oral contraceptives for birth control.

Home Medications:
- Ethinyl estradiol/norgestimate 1 tablet PO daily

ALLERGIES:
She has no known drug allergies.

FAMILY HISTORY:
- Paternal uncle with cerebral arteriovenous malformation
- Mother with hypertension

SOCIAL HISTORY:
She is sexually active with one partner

VITAL SIGNS:
Her vital signs are stable except for the fever.

PHYSICAL EXAMINATION:
On physical examination, she has suprapubic tenderness but no costovertebral angle tenderness.
No vaginal discharge or irritation is noted on pelvic examination.

LABORATORY DATA:
Urinalysis reveals:

- Color: Red
- Appearance: Cloudy
- Specific gravity: 1.025
- pH: 6.0
- Leukocyte esterase: Positive
- Nitrites: Positive
- Red blood cells: >50 per high power field
- White blood cells: >10 per high power field
- Bacteria: Many

## Encounter 2: source 2021-01-15, practice 2022-10-28, outpatient, Dermatology

Chief complaint: Itchy, red, raised skin lesions on trunk and arms

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Emily Johnson, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Itchy, red, raised skin lesions on trunk and arms

HISTORY OF PRESENT ILLNESS:
The 27-year-old female with recurrent oral and genital HSV infection, evaluated in 2020 for oral lesions and cystitis, presents with itchy, red, raised skin lesions on her trunk and arms. She reports that the lesions come and go, with individual spots resolving within a day. She denies recent travel, new medications, or dietary changes. She mentions that she had a sore throat and low-grade fever one week ago that resolved spontaneously.

PAST MEDICAL HISTORY:
Active Problem List:
- Herpes Simplex Virus Infection (diagnosed 2020-01-15)
- Acute bacterial cystitis (diagnosed 2020-07-15)
- Recurrent oral herpes simplex virus infection
- Genital herpes simplex virus infection
- History of chlamydia infection (treated 2 years prior)
- Intermittent tension-type headaches

MEDICATIONS:
Home Medications:
- Ethinyl estradiol/norgestimate 1 tablet PO daily

ALLERGIES:
She has no known drug allergies.

FAMILY HISTORY:
- Paternal uncle with cerebral arteriovenous malformation
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Graduate student.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, you note multiple well-demarcated erythematous plaques with central pallor on her trunk and arms. There is no angioedema or respiratory distress.

## Encounter 3: source 2021-07-15, practice 2023-05-05, outpatient, Dermatology

Chief complaint: Painful rash on right thigh

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Emily Johnson, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Painful rash on right thigh

HISTORY OF PRESENT ILLNESS:
Returning to Dermatology, the patient with recurrent HSV presents with a new rash. She describes the rash as a group of small blisters on a red base. She also reports a burning sensation in the area before the rash appeared.

PAST MEDICAL HISTORY:
Active Problem List:
- Herpes Simplex Virus Infection (diagnosed 2020-01-15)
- Acute bacterial cystitis (diagnosed 2020-07-15)
- Acute urticaria (diagnosed 2021-01-15)
- Recurrent oral herpes simplex virus infection
- Genital herpes simplex virus infection
- History of chlamydia infection (treated 2 years prior)
- Intermittent tension-type headaches

Her medical history is significant for genital herpes.

MEDICATIONS:
Home Medications:
- Ethinyl estradiol/norgestimate 1 tablet PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Paternal uncle with cerebral arteriovenous malformation
- Mother with hypertension

SOCIAL HISTORY:
She is currently sexually active with one partner

VITAL SIGNS:
The patient's temperature is 37.2°C (99°F), blood pressure is 120/78 mmHg, and heart rate is 78 beats per minute.

PHYSICAL EXAMINATION:
On physical examination, the rash is localized to a single dermatome and does not cross the midline. No other skin lesions are noted.

LABORATORY DATA:
Laboratory Values:
- Complete Blood Count (CBC): Within normal limits
- Rapid Plasma Reagin (RPR): Non-reactive

ASSESSMENT AND PLAN:
Given the patient's history and presentation, you consider herpes zoster and herpes simplex as potential diagnoses.

## Encounter 4: source 2022-01-15, practice 2023-10-31, ed, Emergency Medicine

Chief complaint: Generalized tonic-clonic seizure at music festival

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Generalized tonic-clonic seizure at music festival

HISTORY OF PRESENT ILLNESS:
The patient, with recurrent HSV, prior chlamydia, and previous evaluation for cystitis and dermatologic conditions, is brought to the emergency department by her friends after experiencing a generalized tonic-clonic seizure at a music festival. Her friends report that she has been dancing for hours without rest.

PAST MEDICAL HISTORY:
Active Problem List:
- Herpes Simplex Virus Infection (diagnosed 2020-01-15)
- Acute bacterial cystitis (diagnosed 2020-07-15)
- Acute urticaria (diagnosed 2021-01-15)
- Herpes Zoster (diagnosed 2021-07-15)
- Recurrent oral herpes simplex virus infection
- Genital herpes simplex virus infection
- History of chlamydia infection (treated 2 years prior)
- Intermittent tension-type headaches

MEDICATIONS:
Home Medications:
- Ethinyl estradiol/norgestimate 1 tablet PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Paternal uncle with cerebral arteriovenous malformation
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Graduate student.

VITAL SIGNS:
Her blood pressure is 160/100 mmHg, heart rate is 120 beats per minute, and temperature is 38.2°C (100.8°F).

PHYSICAL EXAMINATION:
On examination, the patient is agitated, her skin is warm and moist, and her pupils are dilated and reactive.

LABORATORY DATA:
Laboratory studies are as follows:

- Sodium: 128 mEq/L (Normal: 135-145 mEq/L)
- Potassium: 4.2 mEq/L (Normal: 3.5-5.1 mEq/L)
- Chloride: 100 mEq/L (Normal: 96-106 mEq/L)
- Bicarbonate: 24 mEq/L (Normal: 22-29 mEq/L)
- Blood urea nitrogen: 14 mg/dL (Normal: 7-20 mg/dL)
- Creatinine: 0.9 mg/dL (Normal: 0.6-1.1 mg/dL)
- Glucose: 98 mg/dL (Normal: 70-99 mg/dL)
- Serum osmolality: 260 mOsm/kg (Normal: 275-295 mOsm/kg)
- Urine osmolality: 100 mOsm/kg (Normal: 50-1200 mOsm/kg)

## Encounter 5: source 2022-07-15, practice 2024-04-29, inpatient, Neurology

Chief complaint: Progressive weakness in legs with tingling

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Aisha Patel, MD
Department: Neurology
Visit Type: inpatient

CHIEF COMPLAINT:
Progressive weakness in legs with tingling

HISTORY OF PRESENT ILLNESS:
The patient is a 28-year-old with recurrent oral and genital HSV and prior amphetamine intoxication who presents with progressive neurological symptoms. She reports that the weakness began in her feet and has ascended to her thighs. She also notes tingling sensations in her toes. She denies any recent illness, but she recalls a diarrheal episode about 3 weeks ago that resolved without treatment.

PAST MEDICAL HISTORY:
Active Problem List:
- Herpes Simplex Virus Infection (diagnosed 2020-01-15)
- Acute bacterial cystitis (diagnosed 2020-07-15)
- Acute urticaria (diagnosed 2021-01-15)
- Herpes Zoster (diagnosed 2021-07-15)
- Amphetamine intoxication (diagnosed 2022-01-15)
- Recurrent oral herpes simplex virus infection
- Genital herpes simplex virus infection
- History of chlamydia infection (treated 2 years prior)
- Intermittent tension-type headaches

MEDICATIONS:
Home Medications:
- Ethinyl estradiol/norgestimate 1 tablet PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Paternal uncle with cerebral arteriovenous malformation
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Graduate student.

VITAL SIGNS:
On physical examination, her vital signs are stable.

PHYSICAL EXAMINATION:
Neurological examination reveals bilateral lower extremity weakness, more pronounced proximally than distally, with areflexia. Sensory examination is normal. Cranial nerves are intact, and there is no facial or bulbar weakness.

LABORATORY DATA:
A lumbar puncture is performed, and the cerebrospinal fluid (CSF) analysis shows:

- Protein: 85 mg/dL (normal 15-45 mg/dL)
- White blood cell count: 5 cells/µL (normal 0-5 cells/µL)
- Glucose: 60 mg/dL (normal 40-70 mg/dL)

## Encounter 6: source 2023-01-15, practice 2024-10-25, ed, Obstetrics & Gynecology

Chief complaint: Right lower quadrant abdominal pain and light vaginal bleeding

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Maria Gonzalez, MD
Department: Obstetrics & Gynecology
Visit Type: ed

CHIEF COMPLAINT:
Right lower quadrant abdominal pain and light vaginal bleeding

HISTORY OF PRESENT ILLNESS:
This 29-year-old female with recurrent oral and genital herpes simplex virus infections and treated chlamydia infection presents with a 6-hour history of sudden onset, sharp, right lower quadrant abdominal pain. She also reports light vaginal bleeding that started around the same time as the pain. Her last menstrual period was 6 weeks ago, and she has a regular 28-day cycle.

PAST MEDICAL HISTORY:
Active Problem List:
- Herpes Simplex Virus Infection (diagnosed 2020-01-15)
- Acute bacterial cystitis (diagnosed 2020-07-15)
- Acute urticaria (diagnosed 2021-01-15)
- Herpes Zoster (diagnosed 2021-07-15)
- Amphetamine intoxication (diagnosed 2022-01-15)
- Guillain-Barré syndrome (diagnosed 2022-07-15)
- Recurrent oral herpes simplex virus infection
- Genital herpes simplex virus infection
- History of chlamydia infection (treated 2 years prior)
- Intermittent tension-type headaches

MEDICATIONS:
Home Medications:
- Ethinyl estradiol/norgestimate 1 tablet PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Paternal uncle with cerebral arteriovenous malformation
- Mother with hypertension

SOCIAL HISTORY:
She is sexually active with one partner and uses condoms inconsistently.

VITAL SIGNS:
Her vital signs are stable

PHYSICAL EXAMINATION:
She appears uncomfortable. Abdominal examination reveals right lower quadrant tenderness with rebound tenderness. Pelvic examination shows a small amount of blood in the vaginal vault and right adnexal tenderness.

LABORATORY DATA:
A urine pregnancy test is positive.

IMAGING:
Transvaginal ultrasound reveals an empty uterine cavity and a gestational sac with a yolk sac in the right adnexal region. There is no evidence of intrauterine pregnancy.

## Encounter 7: source 2024-01-15, practice 2025-11-04, icu, Emergency Medicine

Chief complaint: Sudden onset severe headache, nausea, and vomiting

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Robert Liu, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Sudden onset severe headache, nausea, and vomiting

HISTORY OF PRESENT ILLNESS:
The patient is a 30-year-old female with recurrent HSV infection, intermittent tension-type headaches, and prior neurologic complications who presents with symptoms. She describes the worst headache she has experienced, occipital in location, and it started while she was playing basketball. She reports intermittent, milder headaches over the past few months, which she attributes to stress from college exams.

PAST MEDICAL HISTORY:
Active Problem List:
- Herpes Simplex Virus Infection (diagnosed 2020-01-15)
- Acute bacterial cystitis (diagnosed 2020-07-15)
- Acute urticaria (diagnosed 2021-01-15)
- Herpes Zoster (diagnosed 2021-07-15)
- Amphetamine intoxication (diagnosed 2022-01-15)
- Guillain-Barré syndrome (diagnosed 2022-07-15)
- Ectopic pregnancy (left tubal) (diagnosed 2023-01-15)
- Recurrent oral herpes simplex virus infection
- Genital herpes simplex virus infection
- History of chlamydia infection (treated 2 years prior)
- Intermittent tension-type headaches

MEDICATIONS:
Home Medications:
- Ethinyl estradiol/norgestimate 1 tablet PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Paternal uncle with cerebral arteriovenous malformation
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Graduate student.

VITAL SIGNS:
Blood pressure of 130/85 mmHg, heart rate of 98 beats per minute, and afebrile

PHYSICAL EXAMINATION:
On examination, she is alert but anxious. Neurological examination reveals a right homonymous hemianopia. Fundoscopic examination is normal.

IMAGING:
A non-contrast head CT scan shows a left occipital lobe hemorrhage without evidence of calcification or mass effect.
