# synthetic_hospital patient 1677

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2018-12-20 (Thu) | ed | Emergency Medicine |
| 1 | 2020-12-15 | 2019-11-26 (Tue) | ed | Emergency Medicine |
| 2 | 2021-02-15 | 2020-01-17 (Fri) | outpatient | Obstetrics & Gynecology |
| 3 | 2021-02-15 | 2020-01-17 (Fri) | outpatient | Endocrinology |
| 4 | 2021-03-15 | 2020-02-24 (Mon) | outpatient | Endocrinology |
| 5 | 2021-06-15 | 2020-06-02 (Tue) | outpatient | Obstetrics & Gynecology |
| 6 | 2021-09-15 | 2020-08-18 (Tue) | ed | Emergency Medicine |
| 7 | 2027-01-15 | 2025-12-19 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Administrative assistant at a local community college",
 "alcohol_use": "None; sober from alcohol since initiating substance use treatment",
 "family_history": [
  "Mother: Hypothyroidism, postpartum depression",
  "Father: Migraine headaches",
  "Maternal aunt: Bipolar I disorder",
  "Sister: Hashimoto thyroiditis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 4 years ago prior to conception of first child",
 "home_medications": [
  {
   "dose": "60 mg oral daily",
   "name": "Methadone"
  },
  {
   "dose": "50 mg oral PRN migraine headache (max 200 mg/day)",
   "name": "Sumatriptan"
  },
  {
   "dose": "1 tablet daily",
   "name": "Prenatal vitamin with folate"
  }
 ],
 "surgical_history": [
  "Laparoscopic appendectomy (age 32)",
  "Laparoscopic appendectomy with abscess drainage (age 33, postpartum)",
  "Left salpingectomy for ruptured tubal ectopic pregnancy",
  "Normal spontaneous vaginal delivery x2"
 ],
 "chronic_conditions": [
  "Migraine without aura",
  "Opioid use disorder, sustained remission on medication-assisted therapy",
  "History of postpartum psychosis",
  "History of postpartum thyroiditis",
  "History of ruptured ectopic pregnancy"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Acute appendicitis", "Subdermal progestin implant contraception", "Postpartum psychosis", "Postpartum thyroiditis (thyrotoxic phase)", "Postpartum thyroiditis, hyperthyroid phase", "Galactocele", "Ruptured left tubal pregnancy"]
Comorbidities: ["Lactation status", "Exclusive breastfeeding", "Primiparity"]

## Encounter 0: source 2020-01-15, practice 2018-12-20, ed, Emergency Medicine

Chief complaint: Severe right lower quadrant abdominal pain for 8 hours

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Michael Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe right lower quadrant abdominal pain for 8 hours

HISTORY OF PRESENT ILLNESS:
She presents to the emergency department with severe right lower quadrant abdominal pain that started 8 hours ago. The pain is sharp, constant, and worsens with movement. She denies any nausea, vomiting, or changes in bowel habits. Her last menstrual period was 3 weeks ago and was normal.

PAST MEDICAL HISTORY:
Active Problem List:
- Migraine without aura
- Opioid use disorder, sustained remission on medication-assisted therapy
- History of postpartum psychosis
- History of postpartum thyroiditis
- History of ruptured ectopic pregnancy

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 32)
- Laparoscopic appendectomy with abscess drainage (age 33, postpartum)
- Left salpingectomy for ruptured tubal ectopic pregnancy
- Normal spontaneous vaginal delivery x2

MEDICATIONS:
Home Medications:
- Methadone 60 mg oral daily
- Sumatriptan 50 mg oral PRN migraine headache (max 200 mg/day)
- Prenatal vitamin with folate 1 tablet daily

ALLERGIES:
She has no known drug allergies.

FAMILY HISTORY:
- Mother: Hypothyroidism, postpartum depression
- Father: Migraine headaches
- Maternal aunt: Bipolar I disorder
- Sister: Hashimoto thyroiditis

SOCIAL HISTORY:
With a history of intravenous heroin use, currently in a methadone maintenance program,

VITAL SIGNS:
On examination, her temperature is 38.2°C (100.8°F), blood pressure is 130/85 mmHg, pulse is 95/min, and respiratory rate is 18/min.

PHYSICAL EXAMINATION:
Abdominal examination reveals tenderness and rebound tenderness in the right lower quadrant. There is no guarding or rigidity. Pelvic examination is unremarkable.

LABORATORY DATA:
Urinalysis is normal. Pregnancy test is negative. White blood cell count is 12,000/µL with 80% neutrophils.

IMAGING:
Abdominal ultrasound shows a normal gallbladder, liver, and kidneys, but there is a 3 cm fluid-filled structure in the right adnexa with surrounding free fluid.

## Encounter 1: source 2020-12-15, practice 2019-11-26, ed, Emergency Medicine

Chief complaint: Acting increasingly strange and confused 10 days after delivering first child

ENCOUNTER NOTE
Date: 2020-12-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acting increasingly strange and confused 10 days after delivering first child

HISTORY OF PRESENT ILLNESS:
This 32-year-old female with a history of opioid use disorder in sustained remission on methadone maintenance, treated previously in this emergency department for acute appendicitis, presents 10 days following the delivery of her first child. Her husband reports that she has not been sleeping and has been acting increasingly strange since the delivery. She has been heard talking to herself and seems to be responding to voices that no one else can hear. She has expressed beliefs that the baby is possessed and needs to be protected from evil spirits.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute appendicitis (diagnosed 2020-01-15)
- Migraine without aura
- Opioid use disorder, sustained remission on medication-assisted therapy
- History of postpartum psychosis
- History of postpartum thyroiditis
- History of ruptured ectopic pregnancy

She had an uncomplicated pregnancy and vaginal delivery.

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 32)
- Laparoscopic appendectomy with abscess drainage (age 33, postpartum)
- Left salpingectomy for ruptured tubal ectopic pregnancy
- Normal spontaneous vaginal delivery x2

MEDICATIONS:
Home Medications:
- Methadone 60 mg oral daily
- Sumatriptan 50 mg oral PRN migraine headache (max 200 mg/day)
- Prenatal vitamin with folate 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypothyroidism, postpartum depression
- Father: Migraine headaches
- Maternal aunt: Bipolar I disorder
- Sister: Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Former smoker, quit 4 years ago prior to conception of first child. Alcohol: None; sober from alcohol since initiating substance use treatment. Occupation: Administrative assistant at a local community college.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, she is agitated, appears to be preoccupied with internal stimuli, and is not oriented to time or place.

LABORATORY DATA:
Laboratory studies show:
- Hemoglobin: 12.5 g/dL
- White blood cell count: 8,000/mm^3
- Platelets: 250,000/mm^3
- Thyroid-stimulating hormone (TSH): 2.5 mIU/L
- Urinalysis: normal

## Encounter 2: source 2021-02-15, practice 2020-01-17, outpatient, Obstetrics & Gynecology

Chief complaint: Postpartum checkup and contraceptive counseling 6 weeks after delivery

ENCOUNTER NOTE
Date: 2021-02-15
Provider: Dr. Jennifer Okafor, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Postpartum checkup and contraceptive counseling 6 weeks after delivery

HISTORY OF PRESENT ILLNESS:
The patient is a 33-year-old with a history of postpartum psychosis, ruptured ectopic pregnancy, and opioid use disorder in sustained remission on methadone maintenance who returns for scheduled six-week postpartum care. She gave birth to a healthy baby girl 6 weeks ago and has been exclusively breastfeeding. She reports no complications during delivery and her postpartum course has been uneventful. She is interested in discussing contraceptive options as she is not planning to have another child in the near future.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute appendicitis (diagnosed 2020-01-15)
- Postpartum psychosis (diagnosed 2020-12-15)
- Migraine without aura
- Opioid use disorder, sustained remission on medication-assisted therapy
- History of postpartum thyroiditis
- History of ruptured ectopic pregnancy

Her past medical history is significant for migraine headaches without aura.

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 32)
- Laparoscopic appendectomy with abscess drainage (age 33, postpartum)
- Left salpingectomy for ruptured tubal ectopic pregnancy
- Normal spontaneous vaginal delivery x2

MEDICATIONS:
Home Medications:
- Methadone 60 mg oral daily
- Sumatriptan 50 mg oral PRN migraine headache (max 200 mg/day)
- Prenatal vitamin with folate 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypothyroidism, postpartum depression
- Father: Migraine headaches
- Maternal aunt: Bipolar I disorder
- Sister: Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Former smoker, quit 4 years ago prior to conception of first child. Alcohol: None; sober from alcohol since initiating substance use treatment. Occupation: Administrative assistant at a local community college.

VITAL SIGNS:
Her blood pressure is 118/76 mmHg,

PHYSICAL EXAMINATION:
Her physical examination is unremarkable.

LABORATORY DATA:
Her complete blood count and basic metabolic panel are within normal limits.

## Encounter 3: source 2021-02-15, practice 2020-01-17, outpatient, Endocrinology

Chief complaint: Difficulty sleeping, shakiness, and increased anxiety 6 weeks postpartum

ENCOUNTER NOTE
Date: 2021-02-15
Provider: Dr. David Kim, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Difficulty sleeping, shakiness, and increased anxiety 6 weeks postpartum

HISTORY OF PRESENT ILLNESS:
The patient presents to the Endocrinology clinic for evaluation six weeks postpartum. She has a known history of postpartum thyroiditis and prior postpartum psychosis following her first delivery, and she was seen earlier today in Obstetrics & Gynecology for her routine postpartum examination. She delivered a full-term healthy infant 6 weeks ago and has been breastfeeding without complications. Over the last two weeks, she has noticed feeling unusually restless, with frequent palpitations and difficulty concentrating. She denies chest pain, shortness of breath, or recent illness. Her appetite is normal, but she has noticed her clothes fitting more loosely despite no efforts to lose weight. She has no neck pain or tenderness, and no visual complaints.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute appendicitis (diagnosed 2020-01-15)
- Postpartum psychosis (diagnosed 2020-12-15)
- Subdermal progestin implant contraception (diagnosed 2021-02-15)
- Migraine without aura
- Opioid use disorder, sustained remission on medication-assisted therapy
- History of postpartum thyroiditis
- History of ruptured ectopic pregnancy

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 32)
- Laparoscopic appendectomy with abscess drainage (age 33, postpartum)
- Left salpingectomy for ruptured tubal ectopic pregnancy
- Normal spontaneous vaginal delivery x2

MEDICATIONS:
Home Medications:
- Methadone 60 mg oral daily
- Sumatriptan 50 mg oral PRN migraine headache (max 200 mg/day)
- Prenatal vitamin with folate 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypothyroidism, postpartum depression
- Father: Migraine headaches
- Maternal aunt: Bipolar I disorder
- Sister: Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Former smoker, quit 4 years ago prior to conception of first child. Alcohol: None; sober from alcohol since initiating substance use treatment. Occupation: Administrative assistant at a local community college.

PHYSICAL EXAMINATION:
On physical exam, she has a fine tremor in both hands and a heart rate of 102/min. The thyroid gland is slightly enlarged and firm but non-tender. There is no evidence of lid lag or exophthalmos.

LABORATORY DATA:
Thyroid function testing shows elevated free T4 and suppressed TSH

IMAGING:
A radioactive iodine uptake scan reveals low uptake.

## Encounter 4: source 2021-03-15, practice 2020-02-24, outpatient, Endocrinology

Chief complaint: Persistent fatigue and difficulty concentrating 3 months after delivery

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. David Kim, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue and difficulty concentrating 3 months after delivery

HISTORY OF PRESENT ILLNESS:
The patient returns to Endocrinology following last month's evaluation. She has history of postpartum psychosis and presents with symptoms that began approximately six weeks ago. She delivered a healthy full-term infant via vaginal delivery three months ago and is currently breastfeeding. Her appetite is unchanged, though she has gained some weight and feels more tired than usual, despite adequate sleep.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute appendicitis (diagnosed 2020-01-15)
- Postpartum psychosis (diagnosed 2020-12-15)
- Subdermal progestin implant contraception (diagnosed 2021-02-15)
- Postpartum thyroiditis (thyrotoxic phase) (diagnosed 2021-02-15)
- Migraine without aura
- Opioid use disorder, sustained remission on medication-assisted therapy
- History of postpartum thyroiditis
- History of ruptured ectopic pregnancy

She has no personal history of thyroid disease.

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 32)
- Laparoscopic appendectomy with abscess drainage (age 33, postpartum)
- Left salpingectomy for ruptured tubal ectopic pregnancy
- Normal spontaneous vaginal delivery x2

MEDICATIONS:
There is no history of medication use.

Home Medications:
- Methadone 60 mg oral daily
- Sumatriptan 50 mg oral PRN migraine headache (max 200 mg/day)
- Prenatal vitamin with folate 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
She has no family history of thyroid disease.

SOCIAL HISTORY:
Smoking: Former smoker, quit 4 years ago prior to conception of first child. Alcohol: None; sober from alcohol since initiating substance use treatment. Occupation: Administrative assistant at a local community college.

REVIEW OF SYSTEMS:
She denies depressive thoughts, although she feels more emotionally sensitive.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
Physical examination reveals no goiter, nodules, or tenderness over the thyroid gland. Cardiopulmonary and neurologic examinations are unremarkable.

LABORATORY DATA:
Laboratory results show elevated TSH and low free T4; serum autoantibodies are not detected.

## Encounter 5: source 2021-06-15, practice 2020-06-02, outpatient, Obstetrics & Gynecology

Chief complaint: Gradually enlarging lump in right breast while breastfeeding

ENCOUNTER NOTE
Date: 2021-06-15
Provider: Dr. Jennifer Okafor, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Gradually enlarging lump in right breast while breastfeeding

HISTORY OF PRESENT ILLNESS:
This 33-year-old with opioid use disorder in remission on methadone, history of postpartum psychosis and thyroiditis, presents six months postpartum. She delivered vaginally without complications and has been exclusively breastfeeding. She reports a gradually enlarging, mildly uncomfortable swelling in the outer quadrant of her right breast that does not fluctuate in size with feeding. She denies fever, chills, or nipple discharge.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute appendicitis (diagnosed 2020-01-15)
- Postpartum psychosis (diagnosed 2020-12-15)
- Subdermal progestin implant contraception (diagnosed 2021-02-15)
- Postpartum thyroiditis (thyrotoxic phase) (diagnosed 2021-02-15)
- Postpartum thyroiditis, hyperthyroid phase (diagnosed 2021-03-15)
- Migraine without aura
- Opioid use disorder, sustained remission on medication-assisted therapy
- History of postpartum thyroiditis
- History of ruptured ectopic pregnancy

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 32)
- Laparoscopic appendectomy with abscess drainage (age 33, postpartum)
- Left salpingectomy for ruptured tubal ectopic pregnancy
- Normal spontaneous vaginal delivery x2

MEDICATIONS:
Home Medications:
- Methadone 60 mg oral daily
- Sumatriptan 50 mg oral PRN migraine headache (max 200 mg/day)
- Prenatal vitamin with folate 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypothyroidism, postpartum depression
- Father: Migraine headaches
- Maternal aunt: Bipolar I disorder
- Sister: Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Former smoker, quit 4 years ago prior to conception of first child. Alcohol: None; sober from alcohol since initiating substance use treatment. Occupation: Administrative assistant at a local community college.

VITAL SIGNS:
Her temperature is 36.8°C (98.2°F), and vital signs are otherwise normal.

PHYSICAL EXAMINATION:
On examination, there is a 4-cm smooth, fluctuant, mobile mass just lateral to the areola without erythema or warmth. The mass is nontender, and no axillary lymphadenopathy is present.

## Encounter 6: source 2021-09-15, practice 2020-08-18, ed, Emergency Medicine

Chief complaint: Acute onset of left lower abdominal pain with vaginal spotting

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Maria Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of left lower abdominal pain with vaginal spotting

HISTORY OF PRESENT ILLNESS:
This 33-year-old female with history of migraine, postpartum psychosis, thyroiditis, and opioid use disorder in remission on methadone presents with acute abdominal pain. Nine months postpartum following contraceptive counseling, she reports that the pain is sharp, located on the left side, and began suddenly 4 hours ago. She also mentions that she has had light vaginal spotting for the past two days. Her last menstrual period was 6 weeks ago, and she has a regular 28-day cycle. She has been breastfeeding her 9-month-old child and has not yet resumed contraception.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute appendicitis (diagnosed 2020-01-15)
- Postpartum psychosis (diagnosed 2020-12-15)
- Subdermal progestin implant contraception (diagnosed 2021-02-15)
- Postpartum thyroiditis (thyrotoxic phase) (diagnosed 2021-02-15)
- Postpartum thyroiditis, hyperthyroid phase (diagnosed 2021-03-15)
- Galactocele (diagnosed 2021-06-15)
- Migraine without aura
- Opioid use disorder, sustained remission on medication-assisted therapy
- History of postpartum thyroiditis
- History of ruptured ectopic pregnancy

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 32)
- Laparoscopic appendectomy with abscess drainage (age 33, postpartum)
- Left salpingectomy for ruptured tubal ectopic pregnancy
- Normal spontaneous vaginal delivery x2

MEDICATIONS:
Home Medications:
- Methadone 60 mg oral daily
- Sumatriptan 50 mg oral PRN migraine headache (max 200 mg/day)
- Prenatal vitamin with folate 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypothyroidism, postpartum depression
- Father: Migraine headaches
- Maternal aunt: Bipolar I disorder
- Sister: Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Former smoker, quit 4 years ago prior to conception of first child. Alcohol: None; sober from alcohol since initiating substance use treatment. Occupation: Administrative assistant at a local community college.

VITAL SIGNS:
Her vital signs are as follows: blood pressure 90/60 mmHg, heart rate 120 beats per minute, respiratory rate 22 breaths per minute, and temperature 37.0°C (98.6°F).

PHYSICAL EXAMINATION:
On examination, she appears anxious and is in distress. Abdominal examination reveals left lower quadrant tenderness with rebound tenderness. A pelvic examination shows a closed cervical os with no active bleeding, but there is tenderness during movement of the cervix.

LABORATORY DATA:
Laboratory Values:
- Human chorionic gonadotropin (hCG): 2500 mIU/mL
- Hemoglobin: 9.8 g/dL
- Hematocrit: 29%
- White blood cell count: 12,000/mm³
- Platelets: 250,000/mm³

IMAGING:
A bedside ultrasound reveals an empty uterus and a complex adnexal mass on the left side with moderate free fluid in the pelvis.

## Encounter 7: source 2027-01-15, practice 2025-12-19, ed, Emergency Medicine

Chief complaint: Abdominal pain localized to right lower quadrant 3 weeks after second delivery

ENCOUNTER NOTE
Date: 2027-01-15
Provider: Dr. James Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Abdominal pain localized to right lower quadrant 3 weeks after second delivery

HISTORY OF PRESENT ILLNESS:
The patient is a 39-year-old female, now 3 weeks postpartum after her second vaginal delivery, with a history of acute appendicitis in 2020, postpartum psychosis, and ruptured ectopic pregnancy. She gave birth to her second child 3 weeks ago via normal vaginal delivery. The pain initially started around her umbilicus but has now localized to the right lower quadrant. She also reports a low-grade fever, nausea, and a single episode of vomiting. Her appetite has decreased since the onset of pain. She denies any urinary symptoms, vaginal discharge, or bleeding.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute appendicitis (diagnosed 2020-01-15)
- Postpartum psychosis (diagnosed 2020-12-15)
- Subdermal progestin implant contraception (diagnosed 2021-02-15)
- Postpartum thyroiditis (thyrotoxic phase) (diagnosed 2021-02-15)
- Postpartum thyroiditis, hyperthyroid phase (diagnosed 2021-03-15)
- Galactocele (diagnosed 2021-06-15)
- Ruptured left tubal pregnancy (diagnosed 2021-09-15)
- Migraine without aura
- Opioid use disorder, sustained remission on medication-assisted therapy
- History of postpartum thyroiditis
- History of ruptured ectopic pregnancy

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 32)
- Laparoscopic appendectomy with abscess drainage (age 33, postpartum)
- Left salpingectomy for ruptured tubal ectopic pregnancy
- Normal spontaneous vaginal delivery x2

MEDICATIONS:
Home Medications:
- Methadone 60 mg oral daily
- Sumatriptan 50 mg oral PRN migraine headache (max 200 mg/day)
- Prenatal vitamin with folate 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypothyroidism, postpartum depression
- Father: Migraine headaches
- Maternal aunt: Bipolar I disorder
- Sister: Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Former smoker, quit 4 years ago prior to conception of first child. Alcohol: None; sober from alcohol since initiating substance use treatment. Occupation: Administrative assistant at a local community college.

VITAL SIGNS:
Her temperature is 38.1°C (100.6°F), heart rate is 102 beats per minute, and blood pressure is 125/78 mmHg

PHYSICAL EXAMINATION:
Abdominal examination reveals tenderness in the right lower quadrant with rebound tenderness. Bowel sounds are normal.

LABORATORY DATA:
Laboratory tests reveal:
- White blood cell count: 14,000/mm³ (Normal: 4,500-11,000/mm³)
- Hemoglobin: 12.5 g/dL (Normal: 12-16 g/dL)
- C-reactive protein: 25 mg/L (Normal: <5 mg/L)
- Urinalysis: Normal
