# synthetic_hospital patient 1885

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, Hispanic, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-07-01 (Mon) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-02-15 | 2024-07-19 (Fri) | outpatient | Obstetrics & Gynecology |
| 2 | 2020-03-15 | 2024-08-22 (Thu) | inpatient | Cardiology |
| 3 | 2021-07-15 | 2025-12-31 (Wed) | outpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Customer service representative",
 "alcohol_use": "None",
 "family_history": [
  "Maternal aunt with type 2 diabetes mellitus",
  "Paternal cousin with neural tube defect (anencephaly)",
  "Mother with history of cervical dysplasia"
 ],
 "race_ethnicity": "Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 tablet daily",
   "name": "Prenatal vitamin with folic acid"
  },
  {
   "dose": "325 mg daily",
   "name": "Ferrous sulfate"
  }
 ],
 "surgical_history": [
  "Uncomplicated vaginal delivery (primiparous, age 26)"
 ],
 "chronic_conditions": [
  "Rheumatic heart disease (mitral stenosis)",
  "Obesity (Class I)"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Anencephaly", "Severe symptomatic rheumatic mitral stenosis complicating pregnancy", "Cervical Cancer", "Gestational diabetes mellitus"]
Comorbidities: ["Polyhydramnios", "Pregnancy (second trimester, 20 weeks)"]

## Encounter 0: source 2020-01-15, practice 2024-07-01, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit at 24 weeks gestation for glucose screening

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit at 24 weeks gestation for glucose screening

HISTORY OF PRESENT ILLNESS:
She has a BMI of 31 kg/m² and has gained 14 kg (31 lb) since conception. Her pregnancy has been otherwise unremarkable.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatic heart disease (mitral stenosis)
- Obesity (Class I)

PAST SURGICAL HISTORY:
- Uncomplicated vaginal delivery (primiparous, age 26)

MEDICATIONS:
Home Medications:
- Prenatal vitamin with folic acid 1 tablet daily
- Ferrous sulfate 325 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal aunt with type 2 diabetes mellitus
- Paternal cousin with neural tube defect (anencephaly)
- Mother with history of cervical dysplasia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Customer service representative.

VITAL SIGNS:
Her vital signs are normal, and fetal heart rate is 145/min

PHYSICAL EXAMINATION:
Fundal height is 32 cm. No abnormalities are noted on physical exam.

IMAGING:
Ultrasound reveals a fetus at the 85th percentile for weight and shows moderate polyhydramnios. Fetal anatomy is grossly normal.

## Encounter 1: source 2020-02-15, practice 2024-07-19, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit at 28 weeks gestation

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. Elena Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit at 28 weeks gestation

HISTORY OF PRESENT ILLNESS:
The patient, a 28-year-old with rheumatic heart disease (mitral stenosis) and Class I obesity, returns for routine prenatal follow-up at 28 weeks gestation following her previous encounter at 24 weeks where gestational diabetes was identified. She feels well and has no complaints.

PAST MEDICAL HISTORY:
Active Problem List:
- Gestational diabetes mellitus (diagnosed 2020-01-15)
- Rheumatic heart disease (mitral stenosis)
- Obesity (Class I)

This is her second pregnancy and her prior child was healthy.

PAST SURGICAL HISTORY:
- Uncomplicated vaginal delivery (primiparous, age 26)

MEDICATIONS:
She is.

Home Medications:
- Prenatal vitamin with folic acid 1 tablet daily
- Ferrous sulfate 325 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal aunt with type 2 diabetes mellitus
- Paternal cousin with neural tube defect (anencephaly)
- Mother with history of cervical dysplasia

SOCIAL HISTORY:
She has not traveled recently

IMAGING:
Fetal ultrasound shows normal growth parameters and no abnormalities in the gastrointestinal tract; the esophagus appears patent and fluid is seen within the fetal stomach. However, the amniotic fluid index is measured at 27 cm. Fetal anatomy appears normal except for absent calvarial structures.

## Encounter 2: source 2020-03-15, practice 2024-08-22, inpatient, Cardiology

Chief complaint: Progressive dyspnea, orthopnea, and paroxysmal nocturnal dyspnea

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Michael Chen, MD
Department: Cardiology
Visit Type: inpatient

CHIEF COMPLAINT:
Progressive dyspnea, orthopnea, and paroxysmal nocturnal dyspnea

PAST MEDICAL HISTORY:
Active Problem List:
- Gestational diabetes mellitus (diagnosed 2020-01-15)
- Anencephaly (diagnosed 2020-02-15)
- Rheumatic heart disease (mitral stenosis)
- Obesity (Class I)

She has a known history of rheumatic heart disease but has been lost to follow-up for the past few years.

PAST SURGICAL HISTORY:
- Uncomplicated vaginal delivery (primiparous, age 26)

MEDICATIONS:
Home Medications:
- Prenatal vitamin with folic acid 1 tablet daily
- Ferrous sulfate 325 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal aunt with type 2 diabetes mellitus
- Paternal cousin with neural tube defect (anencephaly)
- Mother with history of cervical dysplasia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Customer service representative.

VITAL SIGNS:
On examination, her blood pressure is 110/70 mmHg, heart rate is 98 bpm, and oxygen saturation is 95% on room air.

PHYSICAL EXAMINATION:
Auscultation of the heart reveals a diastolic rumbling murmur best heard at the apex with an opening snap. Lung examination shows bibasilar crackles.

IMAGING:
An echocardiogram shows a mitral valve area of 1.0 cm² with a mean gradient of 10 mmHg and mild pulmonary hypertension.

## Encounter 3: source 2021-07-15, practice 2025-12-31, outpatient, Obstetrics & Gynecology

Chief complaint: Irregular vaginal bleeding, particularly after intercourse, for the past two months

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Priya Patel, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Irregular vaginal bleeding, particularly after intercourse, for the past two months

PAST MEDICAL HISTORY:
Active Problem List:
- Gestational diabetes mellitus (diagnosed 2020-01-15)
- Anencephaly (diagnosed 2020-02-15)
- Severe symptomatic rheumatic mitral stenosis complicating pregnancy (diagnosed 2020-03-15)
- Rheumatic heart disease (mitral stenosis)
- Obesity (Class I)

Her prenatal course has been unremarkable until now.

PAST SURGICAL HISTORY:
- Uncomplicated vaginal delivery (primiparous, age 26)

MEDICATIONS:
Home Medications:
- Prenatal vitamin with folic acid 1 tablet daily
- Ferrous sulfate 325 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal aunt with type 2 diabetes mellitus
- Paternal cousin with neural tube defect (anencephaly)
- Mother with history of cervical dysplasia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Customer service representative.

VITAL SIGNS:
Her vital signs are within normal limits and she denies any pain.

PHYSICAL EXAMINATION:
On physical examination, the cervix appears erythematous with a friable, exophytic mass noted at the external os. The remainder of the pelvic exam is unremarkable.

PLAN:
Given the findings, the obstetrician decides to perform a cervical biopsy.
