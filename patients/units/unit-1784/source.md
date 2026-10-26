# synthetic_hospital patient 1784

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, Non-Hispanic White, Private PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-07-15 (Fri) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-03-15 | 2022-09-09 (Fri) | outpatient | Obstetrics & Gynecology |
| 2 | 2020-06-15 | 2022-12-20 (Tue) | outpatient | Obstetrics & Gynecology |
| 3 | 2023-01-15 | 2025-07-14 (Mon) | outpatient | Obstetrics & Gynecology |
| 4 | 2023-03-15 | 2025-09-17 (Wed) | outpatient | Obstetrics & Gynecology |
| 5 | 2023-05-15 | 2025-11-14 (Fri) | outpatient | Obstetrics & Gynecology |
| 6 | 2023-07-15 | 2026-01-18 (Sun) | inpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Latex (Type I hypersensitivity)",
  "Sulfa drugs (rash)"
 ],
 "insurance": "Private PPO",
 "occupation": "Retail store manager",
 "alcohol_use": "None",
 "family_history": [
  "Maternal first cousin with myelomeningocele",
  "Paternal aunt with systemic lupus erythematosus",
  "Mother with type 2 diabetes mellitus",
  "Father with essential hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 tablet PO daily (containing folate 4mg)",
   "name": "Prenatal vitamin (high-dose folic acid)"
  },
  {
   "dose": "325mg (65mg elemental iron) PO daily",
   "name": "Ferrous sulfate"
  },
  {
   "dose": "500mg PO twice daily",
   "name": "Metformin"
  }
 ],
 "surgical_history": [
  "Tonsillectomy (age 12)",
  "Laparoscopic right ovarian cystectomy (corpus luteum hemorrhage, age 24)"
 ],
 "chronic_conditions": [
  "Polycystic ovary syndrome",
  "Iron deficiency anemia",
  "Generalized anxiety disorder"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Physiologic dyspnea of pregnancy", "Pregnancy (second trimester)", "Acute fatty liver of pregnancy", "Gastroschisis", "Myelomeningocele with Chiari II malformation and hydrocephalus", "Physiologic pruritus gravidarum"]
Comorbidities: ["Pregnancy (second trimester)"]

## Encounter 0: source 2020-01-15, practice 2022-07-15, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit at 16 weeks of gestation

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit at 16 weeks of gestation

HISTORY OF PRESENT ILLNESS:
She reports feeling well with no complaints.

PAST MEDICAL HISTORY:
Active Problem List:
- Polycystic ovary syndrome
- Iron deficiency anemia
- Generalized anxiety disorder

Her previous pregnancy was uncomplicated, and she delivered a healthy full-term infant.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Laparoscopic right ovarian cystectomy (corpus luteum hemorrhage, age 24)

MEDICATIONS:
Home Medications:
- Prenatal vitamin (high-dose folic acid) 1 tablet PO daily (containing folate 4mg)
- Ferrous sulfate 325mg (65mg elemental iron) PO daily
- Metformin 500mg PO twice daily

ALLERGIES:
Latex (Type I hypersensitivity), Sulfa drugs (rash)

FAMILY HISTORY:
Her family history is non-contributory.

SOCIAL HISTORY:
She does not smoke, drink alcohol, or use illicit drugs.

VITAL SIGNS:
On examination, her blood pressure is 120/78 mmHg, heart rate is 72 beats per minute

PHYSICAL EXAMINATION:
Her abdomen is consistent with a 16-week gestation.

LABORATORY DATA:
Her routine second-trimester maternal serum screening returns with the following results:

- Human chorionic gonadotropin (hCG): within normal limits
- Unconjugated estriol (uE3): within normal limits
- Inhibin A: within normal limits
- Alpha-fetoprotein (AFP): significantly elevated

IMAGING:
An ultrasound is performed to evaluate the fetal anatomy. The ultrasound reveals a single fetus with a normal amount of amniotic fluid and no evidence of neural tube defects. However, the fetus has an anterior abdominal wall defect with herniation of the bowel.

ASSESSMENT AND PLAN:
Based on the clinical scenario and the ultrasound findings,

## Encounter 1: source 2020-03-15, practice 2022-09-09, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit, feeling well with no complaints

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit, feeling well with no complaints

PAST MEDICAL HISTORY:
Active Problem List:
- Gastroschisis (diagnosed 2020-01-15)
- Polycystic ovary syndrome
- Iron deficiency anemia
- Generalized anxiety disorder

She has had an uncomplicated pregnancy thus far.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Laparoscopic right ovarian cystectomy (corpus luteum hemorrhage, age 24)

MEDICATIONS:
Home Medications:
- Prenatal vitamin (high-dose folic acid) 1 tablet PO daily (containing folate 4mg)
- Ferrous sulfate 325mg (65mg elemental iron) PO daily
- Metformin 500mg PO twice daily

ALLERGIES:
Latex (Type I hypersensitivity), Sulfa drugs (rash)

FAMILY HISTORY:
- Maternal first cousin with myelomeningocele
- Paternal aunt with systemic lupus erythematosus
- Mother with type 2 diabetes mellitus
- Father with essential hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retail store manager.

VITAL SIGNS:
Her blood pressure is 120/75 mmHg, and her pulse is 78 beats per minute.

PHYSICAL EXAMINATION:
Physical examination is normal with no signs of edema.

LABORATORY DATA:
Laboratory tests reveal the following:

- Hemoglobin: 11.2 g/dL
- Hematocrit: 33%
- Blood urea nitrogen (BUN): 8 mg/dL
- Serum creatinine: 0.5 mg/dL
- Urine protein: 150 mg/24 hours

## Encounter 2: source 2020-06-15, practice 2022-12-20, outpatient, Obstetrics & Gynecology

Chief complaint: Sharp, intermittent right-sided lower abdominal and pelvic pain

ENCOUNTER NOTE
Date: 2020-06-15
Provider: Dr. Maria Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Sharp, intermittent right-sided lower abdominal and pelvic pain

HISTORY OF PRESENT ILLNESS:
The 28-year-old with PCOS and iron deficiency anemia returns for prenatal care of pregnancy with fetal gastroschisis. She reports a 5-day history of sharp, intermittent right-sided lower abdominal and pelvic pain that worsens with sudden movements and when changing positions, and that the pain sometimes radiates to her right groin and vaginal area. She denies vaginal bleeding, contractions, or urinary symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Gastroschisis (diagnosed 2020-01-15)
- Pregnancy (second trimester) (diagnosed 2020-03-15)
- Polycystic ovary syndrome
- Iron deficiency anemia
- Generalized anxiety disorder

Her prenatal course has been uncomplicated so far, and she is otherwise healthy.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Laparoscopic right ovarian cystectomy (corpus luteum hemorrhage, age 24)

MEDICATIONS:
Home Medications:
- Prenatal vitamin (high-dose folic acid) 1 tablet PO daily (containing folate 4mg)
- Ferrous sulfate 325mg (65mg elemental iron) PO daily
- Metformin 500mg PO twice daily

ALLERGIES:
Latex (Type I hypersensitivity), Sulfa drugs (rash)

FAMILY HISTORY:
- Maternal first cousin with myelomeningocele
- Paternal aunt with systemic lupus erythematosus
- Mother with type 2 diabetes mellitus
- Father with essential hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retail store manager.

PHYSICAL EXAMINATION:
On examination, her abdomen is soft and non-tender except for mild discomfort on deep palpation over the right lower quadrant. The uterus is palpable at the level of the umbilicus, consistent with gestational age, and there is no cervical dilation.

ASSESSMENT AND PLAN:
The clinician suspects that her symptoms are due to stretching of a specific pelvic structure associated with the expanding uterus.

## Encounter 3: source 2023-01-15, practice 2025-07-14, outpatient, Obstetrics & Gynecology

Chief complaint: First prenatal visit at 18 weeks gestation

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Aisha Patel, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
First prenatal visit at 18 weeks gestation

HISTORY OF PRESENT ILLNESS:
This 31-year-old with PCOS and prior 2020 pregnancy complicated by gastroschisis presents at 18 weeks for initial prenatal care. She reports that she did not know she was pregnant until 2 months ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Gastroschisis (diagnosed 2020-01-15)
- Pregnancy (second trimester) (diagnosed 2020-03-15)
- Physiologic pruritus gravidarum (diagnosed 2020-06-15)
- Polycystic ovary syndrome
- Iron deficiency anemia
- Generalized anxiety disorder

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Laparoscopic right ovarian cystectomy (corpus luteum hemorrhage, age 24)

MEDICATIONS:
Home Medications:
- Prenatal vitamin (high-dose folic acid) 1 tablet PO daily (containing folate 4mg)
- Ferrous sulfate 325mg (65mg elemental iron) PO daily
- Metformin 500mg PO twice daily

ALLERGIES:
Latex (Type I hypersensitivity), Sulfa drugs (rash)

FAMILY HISTORY:
- Maternal first cousin with myelomeningocele
- Paternal aunt with systemic lupus erythematosus
- Mother with type 2 diabetes mellitus
- Father with essential hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retail store manager.

IMAGING:
Fetal ultrasound reveals a posterior spinal defect with a protruding sac.

ASSESSMENT AND PLAN:
A defect of

## Encounter 4: source 2023-03-15, practice 2025-09-17, outpatient, Obstetrics & Gynecology

Chief complaint: Shortness of breath for the past two weeks

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. Aisha Patel, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Shortness of breath for the past two weeks

HISTORY OF PRESENT ILLNESS:
The patient returns for prenatal follow-up of her current pregnancy complicated by fetal myelomeningocele, following a prior pregnancy with gastroschisis. She reports dyspnea more pronounced lying flat and improving sitting up, denying chest pain, cough, or wheezing.

PAST MEDICAL HISTORY:
Active Problem List:
- Gastroschisis (diagnosed 2020-01-15)
- Pregnancy (second trimester) (diagnosed 2020-03-15)
- Physiologic pruritus gravidarum (diagnosed 2020-06-15)
- Myelomeningocele with Chiari II malformation and hydrocephalus (diagnosed 2023-01-15)
- Polycystic ovary syndrome
- Iron deficiency anemia
- Generalized anxiety disorder

She has had an uncomplicated pregnancy thus far.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Laparoscopic right ovarian cystectomy (corpus luteum hemorrhage, age 24)

MEDICATIONS:
Home Medications:
- Prenatal vitamin (high-dose folic acid) 1 tablet PO daily (containing folate 4mg)
- Ferrous sulfate 325mg (65mg elemental iron) PO daily
- Metformin 500mg PO twice daily

ALLERGIES:
Latex (Type I hypersensitivity), Sulfa drugs (rash)

FAMILY HISTORY:
- Maternal first cousin with myelomeningocele
- Paternal aunt with systemic lupus erythematosus
- Mother with type 2 diabetes mellitus
- Father with essential hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retail store manager.

VITAL SIGNS:
On examination, her respiratory rate is 18 breaths per minute, heart rate is 88 beats per minute, blood pressure is 120/75 mmHg, and oxygen saturation is 98% on room air.

PHYSICAL EXAMINATION:
Lung auscultation reveals clear breath sounds bilaterally. Cardiac examination is normal with no murmurs, rubs, or gallops. Lower extremity examination shows no edema.

LABORATORY DATA:
Laboratory tests reveal the following:

- Hemoglobin: 11.5 g/dL
- Hematocrit: 34%
- Platelet count: 220,000/mm³
- Arterial blood gas (ABG):
- pH: 7.44
- PaCO2: 30 mmHg
- PaO2: 105 mmHg
- HCO3-: 18 mEq/L

## Encounter 5: source 2023-05-15, practice 2025-11-14, outpatient, Obstetrics & Gynecology

Chief complaint: Recent changes in her appearance including facial hair growth and acne

ENCOUNTER NOTE
Date: 2023-05-15
Provider: Dr. Priya Sharma, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Recent changes in her appearance including facial hair growth and acne

HISTORY OF PRESENT ILLNESS:
The patient is a 31-year-old with polycystic ovary syndrome who returns for continued prenatal care following her recent evaluation for dyspnea. She presents with her husband, concerned about recent changes in her appearance. She reports a significant increase in facial hair growth, deepening of her voice, and acne over the past two months. She denies any visual changes, headaches, or abdominal pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Gastroschisis (diagnosed 2020-01-15)
- Pregnancy (second trimester) (diagnosed 2020-03-15)
- Physiologic pruritus gravidarum (diagnosed 2020-06-15)
- Myelomeningocele with Chiari II malformation and hydrocephalus (diagnosed 2023-01-15)
- Physiologic dyspnea of pregnancy (diagnosed 2023-03-15)
- Polycystic ovary syndrome
- Iron deficiency anemia
- Generalized anxiety disorder

Her pregnancy has been uncomplicated thus far.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Laparoscopic right ovarian cystectomy (corpus luteum hemorrhage, age 24)

MEDICATIONS:
Home Medications:
- Prenatal vitamin (high-dose folic acid) 1 tablet PO daily (containing folate 4mg)
- Ferrous sulfate 325mg (65mg elemental iron) PO daily
- Metformin 500mg PO twice daily

ALLERGIES:
Latex (Type I hypersensitivity), Sulfa drugs (rash)

FAMILY HISTORY:
- Maternal first cousin with myelomeningocele
- Paternal aunt with systemic lupus erythematosus
- Mother with type 2 diabetes mellitus
- Father with essential hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retail store manager.

VITAL SIGNS:
Her blood pressure is 120/78 mmHg, heart rate is 72 beats per minute

PHYSICAL EXAMINATION:
On physical examination, she appears well. She has noticeable facial hirsutism and temporal balding.

LABORATORY DATA:
Her laboratory tests are as follows:

- Hemoglobin: 12.5 g/dL
- Platelets: 250,000/mm^3
- Fasting glucose: 88 mg/dL
- Total testosterone: 150 ng/dL (normal non-pregnant female range: 15-70 ng/dL)
- Dehydroepiandrosterone sulfate (DHEAS): 180 µg/dL (normal non-pregnant female range: 35-430 µg/dL)

IMAGING:
Pelvic ultrasound reveals bilateral, multicystic ovarian masses measuring 3 cm on the right and 4 cm on the left. There are no signs of fetal distress or abnormalities.

## Encounter 6: source 2023-07-15, practice 2026-01-18, inpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit with malaise and palpitations

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. James O'Brien, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Routine prenatal visit with malaise and palpitations

HISTORY OF PRESENT ILLNESS:
The patient with prior gastroschisis returns for prenatal care and reports feeling generally well but mentions occasional palpitations and increased sweating, which she attributes to her pregnancy.

PAST MEDICAL HISTORY:
Active Problem List:
- Gastroschisis (diagnosed 2020-01-15)
- Pregnancy (second trimester) (diagnosed 2020-03-15)
- Physiologic pruritus gravidarum (diagnosed 2020-06-15)
- Myelomeningocele with Chiari II malformation and hydrocephalus (diagnosed 2023-01-15)
- Physiologic dyspnea of pregnancy (diagnosed 2023-03-15)
- Polycystic ovary syndrome
- Iron deficiency anemia
- Generalized anxiety disorder

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Laparoscopic right ovarian cystectomy (corpus luteum hemorrhage, age 24)

MEDICATIONS:
Home Medications:
- Prenatal vitamin (high-dose folic acid) 1 tablet PO daily (containing folate 4mg)
- Ferrous sulfate 325mg (65mg elemental iron) PO daily
- Metformin 500mg PO twice daily

ALLERGIES:
Latex (Type I hypersensitivity), Sulfa drugs (rash)

FAMILY HISTORY:
Her family history is noncontributory.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retail store manager.

VITAL SIGNS:
Her blood pressure is 118/76 mmHg, heart rate is 88 beats per minute

PHYSICAL EXAMINATION:
She appears euthyroid. Her thyroid is not enlarged or tender on palpation, and no nodules are palpable. Reflexes are normal, and no tremor is noted.

LABORATORY DATA:
Routine laboratory tests were performed, and the results are as follows:

- Human chorionic gonadotropin (hCG): Elevated for gestational age
- Thyroid-stimulating hormone (TSH): Slightly decreased
- Free thyroxine (FT4): Upper limit of normal
- Total thyroxine (TT4): Elevated
