# synthetic_hospital patient 1816

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, African American, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-12-26 (Mon) | outpatient | Endocrinology |
| 1 | 2023-01-15 | 2025-12-26 (Fri) | outpatient | Obstetrics & Gynecology |
| 2 | 2023-03-15 | 2026-03-02 (Mon) | outpatient | Obstetrics & Gynecology |
| 3 | 2023-07-15 | 2026-06-22 (Mon) | outpatient | Obstetrics & Gynecology |
| 4 | 2023-08-15 | 2026-08-06 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Sulfonamides (hives)",
  "Penicillin (rash)"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Type 2 Diabetes Mellitus, hypertension",
  "Father: Sickle cell trait",
  "Older sister: Sickle cell disease",
  "Maternal aunt: Hashimoto thyroiditis"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "Basal rate 1.1 units/hr; bolus per carbohydrate ratio",
   "name": "Insulin Aspart (via insulin pump)"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Methimazole"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Prenatal Vitamin with Folic Acid"
  }
 ],
 "surgical_history": [
  "Laparoscopic cholecystectomy (age 26)"
 ],
 "chronic_conditions": [
  "Type 1 Diabetes Mellitus (since age 7)",
  "Sickle Cell Disease (HbSS)",
  "Graves Disease",
  "Mild non-proliferative diabetic retinopathy"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Fetal Growth Restriction with Oligohydramnios", "Bilateral adrenal hemorrhage (Waterhouse-Friderichsen syndrome)", "Pre-existing type 1 diabetes mellitus in pregnancy, first trimester", "Diabetic nephropathy", "Uncontrolled Graves disease hyperthyroidism"]
Comorbidities: ["Type 1 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2022-12-26, outpatient, Endocrinology

Chief complaint: Persistent anxiety, weight loss despite increased appetite, difficulty sleeping, and hand tremor over the past month

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent anxiety, weight loss despite increased appetite, difficulty sleeping, and hand tremor over the past month

HISTORY OF PRESENT ILLNESS:
She reports feeling her heart race frequently and has recently noticed a tremor in her hands. She denies palpitations, recent illness, or medication changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 1 Diabetes Mellitus (since age 7)
- Sickle Cell Disease (HbSS)
- Graves Disease
- Mild non-proliferative diabetic retinopathy

She has had type I diabetes mellitus since childhood, for which she uses an insulin pump.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 26)

MEDICATIONS:
Home Medications:
- Insulin Aspart (via insulin pump) Basal rate 1.1 units/hr; bolus per carbohydrate ratio
- Methimazole 10 mg PO daily
- Prenatal Vitamin with Folic Acid 1 tablet PO daily

ALLERGIES:
Sulfonamides (hives), Penicillin (rash)

FAMILY HISTORY:
- Mother: Type 2 Diabetes Mellitus, hypertension
- Father: Sickle cell trait
- Older sister: Sickle cell disease
- Maternal aunt: Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

VITAL SIGNS:
Resting heart rate of 108/min

PHYSICAL EXAMINATION:
A fine tremor of the outstretched hands. The thyroid is not visibly enlarged, but palpation reveals a symmetrically enlarged and non-tender gland. Deep tendon reflexes are brisk. Her eyes are wide, and exophthalmos is noted bilaterally.

LABORATORY DATA:
A thyroid panel

IMAGING:
Radioactive iodine uptake (RAIU) scan

## Encounter 1: source 2023-01-15, practice 2025-12-26, outpatient, Obstetrics & Gynecology

Chief complaint: First prenatal visit at 10 weeks gestation for ongoing pregnancy management

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Maria Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
First prenatal visit at 10 weeks gestation for ongoing pregnancy management

HISTORY OF PRESENT ILLNESS:
The patient with Type 1 Diabetes Mellitus since age 7, Sickle Cell Disease, Graves disease, and prior 2020 Endocrinology evaluation for hyperthyroidism presents at 10 weeks gestation. She has been compliant with her insulin regimen and reports good glycemic control.

PAST MEDICAL HISTORY:
Active Problem List:
- Uncontrolled Graves disease hyperthyroidism (diagnosed 2020-01-15)
- Type 1 Diabetes Mellitus (since age 7)
- Sickle Cell Disease (HbSS)
- Graves Disease
- Mild non-proliferative diabetic retinopathy

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 26)

MEDICATIONS:
Home Medications:
- Insulin Aspart (via insulin pump) Basal rate 1.1 units/hr; bolus per carbohydrate ratio
- Methimazole 10 mg PO daily
- Prenatal Vitamin with Folic Acid 1 tablet PO daily

ALLERGIES:
Sulfonamides (hives), Penicillin (rash)

FAMILY HISTORY:
She has no significant family history of renal disease.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

VITAL SIGNS:
Her blood pressure is 128/82 mmHg,

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
Urinalysis shows no proteinuria or glucosuria.

PLAN:
Her obstetrician decides to order additional tests to establish baseline renal function.

## Encounter 2: source 2023-03-15, practice 2026-03-02, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit at 18 weeks gestation with bilateral lower extremity edema and occasional headaches

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. Maria Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit at 18 weeks gestation with bilateral lower extremity edema and occasional headaches

HISTORY OF PRESENT ILLNESS:
The patient with Type 1 Diabetes Mellitus since age 7, Graves disease, and Sickle Cell Disease returns for prenatal care at 18 weeks gestation following her initial visit at 10 weeks. She has been compliant with her insulin regimen and reports checking her blood glucose levels four times a day. She mentions that over the past two weeks, she has noticed increased swelling in her legs and occasional headaches.

PAST MEDICAL HISTORY:
Active Problem List:
- Uncontrolled Graves disease hyperthyroidism (diagnosed 2020-01-15)
- Pre-existing type 1 diabetes mellitus in pregnancy, first trimester (diagnosed 2023-01-15)
- Type 1 Diabetes Mellitus (since age 7)
- Sickle Cell Disease (HbSS)
- Graves Disease
- Mild non-proliferative diabetic retinopathy

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 26)

MEDICATIONS:
She takes insulin regimen.

Home Medications:
- Insulin Aspart (via insulin pump) Basal rate 1.1 units/hr; bolus per carbohydrate ratio
- Methimazole 10 mg PO daily
- Prenatal Vitamin with Folic Acid 1 tablet PO daily

ALLERGIES:
Sulfonamides (hives), Penicillin (rash)

FAMILY HISTORY:
- Mother: Type 2 Diabetes Mellitus, hypertension
- Father: Sickle cell trait
- Older sister: Sickle cell disease
- Maternal aunt: Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

REVIEW OF SYSTEMS:
She denies any visual changes or abdominal pain.

VITAL SIGNS:
Her blood pressure is 145/95 mmHg.

PHYSICAL EXAMINATION:
Her urine dipstick shows 3+ proteinuria.

LABORATORY DATA:
Laboratory results are as follows:

- Serum creatinine: 1.8 mg/dL (baseline 6 months ago was 1.0 mg/dL)
- Urine protein/creatinine ratio: 3.5 g/g
- Hemoglobin A1c: 7.2%
- Platelet count: 210,000/µL
- Liver function tests: Within normal limits

## Encounter 3: source 2023-07-15, practice 2026-06-22, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit at 32 weeks gestation with increasingly difficult blood glucose control requiring frequent insulin adjustments

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. James Okafor, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit at 32 weeks gestation with increasingly difficult blood glucose control requiring frequent insulin adjustments

HISTORY OF PRESENT ILLNESS:
The patient with Type 1 Diabetes returns for routine prenatal care at 32 weeks gestation, continuing management following prior nephropathy. She has been compliant with her insulin regimen and reports no episodes of hypoglycemia or diabetic ketoacidosis. However, she mentions that her blood glucose levels have been more difficult to control during the pregnancy, with frequent adjustments to her insulin dosage.

PAST MEDICAL HISTORY:
Active Problem List:
- Uncontrolled Graves disease hyperthyroidism (diagnosed 2020-01-15)
- Pre-existing type 1 diabetes mellitus in pregnancy, first trimester (diagnosed 2023-01-15)
- Diabetic nephropathy (diagnosed 2023-03-15)
- Type 1 Diabetes Mellitus (since age 7)
- Sickle Cell Disease (HbSS)
- Graves Disease
- Mild non-proliferative diabetic retinopathy

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 26)

MEDICATIONS:
Home Medications:
- Insulin Aspart (via insulin pump) Basal rate 1.1 units/hr; bolus per carbohydrate ratio
- Methimazole 10 mg PO daily
- Prenatal Vitamin with Folic Acid 1 tablet PO daily

ALLERGIES:
Sulfonamides (hives), Penicillin (rash)

FAMILY HISTORY:
- Mother: Type 2 Diabetes Mellitus, hypertension
- Father: Sickle cell trait
- Older sister: Sickle cell disease
- Maternal aunt: Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

VITAL SIGNS:
On examination, her blood pressure is 128/82 mmHg

PHYSICAL EXAMINATION:
Her fundal height measures 28 cm, which is less than expected for her gestational age. Fetal heart tones are normal.

IMAGING:
An ultrasound is performed, revealing an estimated fetal weight below the 10th percentile for gestational age and amniotic fluid index (AFI) of 4 cm. An umbilical artery Doppler ultrasound is subsequently performed.

## Encounter 4: source 2023-08-15, practice 2026-08-06, ed, Emergency Medicine

Chief complaint: Decreased fetal movements for 2 days in the third trimester

ENCOUNTER NOTE
Date: 2023-08-15
Provider: Dr. Michael Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Decreased fetal movements for 2 days in the third trimester

HISTORY OF PRESENT ILLNESS:
This 35-year-old female with Type 1 Diabetes Mellitus, Sickle Cell Disease, and Graves Disease presents to the emergency department with a 2-day history of decreased fetal movements following recent care for fetal growth restriction with oligohydramnios. She reports no recent painful crises. Her pregnancy has been complicated by two episodes of pyelonephritis requiring hospitalization and antibiotics.

PAST MEDICAL HISTORY:
Active Problem List:
- Uncontrolled Graves disease hyperthyroidism (diagnosed 2020-01-15)
- Pre-existing type 1 diabetes mellitus in pregnancy, first trimester (diagnosed 2023-01-15)
- Diabetic nephropathy (diagnosed 2023-03-15)
- Fetal Growth Restriction with Oligohydramnios (diagnosed 2023-07-15)
- Type 1 Diabetes Mellitus (since age 7)
- Sickle Cell Disease (HbSS)
- Graves Disease
- Mild non-proliferative diabetic retinopathy

She has a known history of sickle cell disease.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 26)

MEDICATIONS:
Home Medications:
- Insulin Aspart (via insulin pump) Basal rate 1.1 units/hr; bolus per carbohydrate ratio
- Methimazole 10 mg PO daily
- Prenatal Vitamin with Folic Acid 1 tablet PO daily

ALLERGIES:
Sulfonamides (hives), Penicillin (rash)

FAMILY HISTORY:
- Mother: Type 2 Diabetes Mellitus, hypertension
- Father: Sickle cell trait
- Older sister: Sickle cell disease
- Maternal aunt: Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

VITAL SIGNS:
On examination, her blood pressure is 120/78 mmHg, heart rate is 88 beats per minute, and temperature is 37.2°C (99.0°F).

PHYSICAL EXAMINATION:
Abdominal examination reveals a gravid uterus consistent with gestational age, but the fetal heart tones are 110 beats per minute.

IMAGING:
An ultrasound is performed, which shows an amniotic fluid index of 3 cm.
