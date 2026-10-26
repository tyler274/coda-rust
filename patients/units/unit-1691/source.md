# synthetic_hospital patient 1691

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Hispanic (Mexican-American), Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-06-25 (Wed) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-03-15 | 2025-08-12 (Tue) | outpatient | Obstetrics & Gynecology |
| 2 | 2020-04-15 | 2025-09-13 (Sat) | ed | Emergency Medicine |
| 3 | 2020-05-15 | 2025-10-24 (Fri) | ed | Emergency Medicine |
| 4 | 2020-06-15 | 2025-11-26 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Administrative coordinator (currently on modified bed rest)",
 "alcohol_use": "No current use; abstinent since pregnancy confirmation",
 "family_history": [
  "Type 2 diabetes mellitus (mother, diagnosed age 45)",
  "Twin pregnancy (maternal grandmother)"
 ],
 "race_ethnicity": "Hispanic (Mexican-American)",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 tablet PO daily",
   "name": "Prenatal vitamin with iron"
  },
  {
   "dose": "14 units subcutaneous at bedtime (titrated to fasting glucose)",
   "name": "Insulin NPH (intermediate-acting)"
  },
  {
   "dose": "sliding scale with meals per carbohydrate counting",
   "name": "Insulin lispro (rapid-acting)"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Obesity (BMI 32 kg/m^2)",
  "Gestational diabetes mellitus",
  "Placenta previa",
  "Monochorionic monoamniotic twin pregnancy"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Gestational diabetes mellitus", "Complete placenta previa with hemorrhage", "Monochorionic monoamniotic twin pregnancy", "Intrauterine fetal demise", "Conservative management of placenta previa with bed rest"]
Comorbidities: ["Personal History of Cholelithiasis", "Pregnancy (28 weeks gestation)"]

## Encounter 0: source 2020-01-15, practice 2025-06-25, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit for ongoing care of monochorionic monoamniotic twin pregnancy.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit for ongoing care of monochorionic monoamniotic twin pregnancy.

HISTORY OF PRESENT ILLNESS:
She has had an uncomplicated pregnancy thus far, but her obstetrician is concerned because her ultrasound shows a monochorionic monoamniotic twin pregnancy. The patient reports feeling frequent fetal movements and has no complaints of vaginal bleeding, leakage of fluid, or contractions.

PAST MEDICAL HISTORY:
Active Problem List:
- Obesity (BMI 32 kg/m^2)
- Gestational diabetes mellitus
- Placenta previa
- Monochorionic monoamniotic twin pregnancy

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Prenatal vitamin with iron 1 tablet PO daily
- Insulin NPH (intermediate-acting) 14 units subcutaneous at bedtime (titrated to fasting glucose)
- Insulin lispro (rapid-acting) sliding scale with meals per carbohydrate counting

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus (mother, diagnosed age 45)
- Twin pregnancy (maternal grandmother)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No current use; abstinent since pregnancy confirmation. Occupation: Administrative coordinator (currently on modified bed rest).

VITAL SIGNS:
Her blood pressure is 120/75 mmHg, heart rate is 88 beats per minute,

PHYSICAL EXAMINATION:
Her physical examination is unremarkable except for a fundal height that corresponds to 32 weeks gestation for a singleton pregnancy. Fetal heart tones are 140 and 142 beats per minute.

PLAN:
The patient is counseled about the risks associated with her type of twin pregnancy.

## Encounter 1: source 2020-03-15, practice 2025-08-12, outpatient, Obstetrics & Gynecology

Chief complaint: Follow-up for gestational diabetes mellitus with persistent hyperglycemic symptoms despite dietary modifications.

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up for gestational diabetes mellitus with persistent hyperglycemic symptoms despite dietary modifications.

HISTORY OF PRESENT ILLNESS:
The patient is a 32-year-old female with obesity, placenta previa, and monochorionic monoamniotic twin pregnancy who returns for follow-up of gestational diabetes mellitus initially diagnosed 4 weeks ago. She has a BMI of 32 kg/m^2 and reports adherence to the dietary modifications recommended by her dietitian and walks for 30 minutes daily. Despite these efforts, she notes frequent episodes of blurred vision and dizziness. Her fasting blood glucose levels for the past week are as follows: Monday 98 mg/dL, Tuesday 105 mg/dL, Wednesday 110 mg/dL, Thursday 115 mg/dL, Friday 108 mg/dL, Saturday 112 mg/dL, and Sunday 107 mg/dL. Her postprandial blood glucose levels have consistently been above 140 mg/dL.

PAST MEDICAL HISTORY:
Active Problem List:
- Monochorionic monoamniotic twin pregnancy (diagnosed 2020-01-15)
- Obesity (BMI 32 kg/m^2)
- Gestational diabetes mellitus
- Placenta previa

Her obstetric history is significant for one previous pregnancy, which was uncomplicated.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Prenatal vitamin with iron 1 tablet PO daily
- Insulin NPH (intermediate-acting) 14 units subcutaneous at bedtime (titrated to fasting glucose)
- Insulin lispro (rapid-acting) sliding scale with meals per carbohydrate counting

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus (mother, diagnosed age 45)
- Twin pregnancy (maternal grandmother)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No current use; abstinent since pregnancy confirmation. Occupation: Administrative coordinator (currently on modified bed rest).

PHYSICAL EXAMINATION:
The fetal heart rate is regular, and the growth parameters are within normal limits.

LABORATORY DATA:
Urinalysis shows no ketones or protein.

ASSESSMENT AND PLAN:
Based on the patient's persistent hyperglycemia despite dietary modifications,

## Encounter 2: source 2020-04-15, practice 2025-09-13, ed, Emergency Medicine

Chief complaint: Sudden onset of painless vaginal bleeding at 28 weeks gestation.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of painless vaginal bleeding at 28 weeks gestation.

HISTORY OF PRESENT ILLNESS:
This 32-year-old with monochorionic monoamniotic twins, placenta previa, and GDM is 28 weeks pregnant and reports no pain associated with the bleeding.

PAST MEDICAL HISTORY:
Active Problem List:
- Monochorionic monoamniotic twin pregnancy (diagnosed 2020-01-15)
- Gestational diabetes mellitus (diagnosed 2020-03-15)
- Obesity (BMI 32 kg/m^2)
- Placenta previa

Her first pregnancy was uncomplicated, and she delivered vaginally at term.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Prenatal vitamin with iron 1 tablet PO daily
- Insulin NPH (intermediate-acting) 14 units subcutaneous at bedtime (titrated to fasting glucose)
- Insulin lispro (rapid-acting) sliding scale with meals per carbohydrate counting

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus (mother, diagnosed age 45)
- Twin pregnancy (maternal grandmother)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No current use; abstinent since pregnancy confirmation. Occupation: Administrative coordinator (currently on modified bed rest).

VITAL SIGNS:
Her vital signs are stable

PHYSICAL EXAMINATION:
The abdomen is soft and non-tender. Speculum examination is deferred to avoid potential trauma to the cervix.

IMAGING:
A bedside ultrasound shows a placenta that is covering the internal cervical os.

OTHER STUDIES:
Fetal heart rate tracings are reactive with no decelerations.

## Encounter 3: source 2020-05-15, practice 2025-10-24, ed, Emergency Medicine

Chief complaint: Recurrent episode of sudden, painless, bright red vaginal bleeding with increased severity.

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Recurrent episode of sudden, painless, bright red vaginal bleeding with increased severity.

HISTORY OF PRESENT ILLNESS:
She returns for recurrent vaginal bleeding with known placenta previa and twin pregnancy. She is currently 28 weeks pregnant and reports no contractions, abdominal pain, or trauma. Her prenatal course has been unremarkable until now, with normal blood pressure and no proteinuria.

PAST MEDICAL HISTORY:
Active Problem List:
- Monochorionic monoamniotic twin pregnancy (diagnosed 2020-01-15)
- Gestational diabetes mellitus (diagnosed 2020-03-15)
- Conservative management of placenta previa with bed rest (diagnosed 2020-04-15)
- Obesity (BMI 32 kg/m^2)
- Placenta previa

Her first pregnancy was uncomplicated, and she delivered vaginally at term.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Prenatal vitamin with iron 1 tablet PO daily
- Insulin NPH (intermediate-acting) 14 units subcutaneous at bedtime (titrated to fasting glucose)
- Insulin lispro (rapid-acting) sliding scale with meals per carbohydrate counting

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus (mother, diagnosed age 45)
- Twin pregnancy (maternal grandmother)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No current use; abstinent since pregnancy confirmation. Occupation: Administrative coordinator (currently on modified bed rest).

VITAL SIGNS:
On examination, her vital signs are stable

PHYSICAL EXAMINATION:
The abdomen is soft and non-tender, and the fetal heart rate is 140 beats per minute. Speculum examination is deferred to avoid potential disruption of the placenta.

LABORATORY DATA:
Laboratory values:
- Hemoglobin: 11.2 g/dL
- Platelet count: 210,000/mm³
- Blood type: A positive
- Coagulation profile: Within normal limits

IMAGING:
Transabdominal ultrasound reveals a placenta that is covering the internal cervical os.

## Encounter 4: source 2020-06-15, practice 2025-11-26, ed, Emergency Medicine

Chief complaint: Absence of fetal movements for the past 24 hours in high-risk twin pregnancy.

ENCOUNTER NOTE
Date: 2020-06-15
Provider: Dr. James Whitmore, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Absence of fetal movements for the past 24 hours in high-risk twin pregnancy.

HISTORY OF PRESENT ILLNESS:
The patient is a 32-year-old female with monochorionic monoamniotic twin pregnancy, gestational diabetes mellitus, and complete placenta previa who presents to the emergency department with a chief complaint of not feeling fetal movements for the past 24 hours. She has had regular prenatal care with recent emergency visits for vaginal bleeding. The patient is understandably distressed and inquires about the next steps.

PAST MEDICAL HISTORY:
Active Problem List:
- Monochorionic monoamniotic twin pregnancy (diagnosed 2020-01-15)
- Gestational diabetes mellitus (diagnosed 2020-03-15)
- Conservative management of placenta previa with bed rest (diagnosed 2020-04-15)
- Complete placenta previa with hemorrhage (diagnosed 2020-05-15)
- Obesity (BMI 32 kg/m^2)
- Placenta previa

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Prenatal vitamin with iron 1 tablet PO daily
- Insulin NPH (intermediate-acting) 14 units subcutaneous at bedtime (titrated to fasting glucose)
- Insulin lispro (rapid-acting) sliding scale with meals per carbohydrate counting

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus (mother, diagnosed age 45)
- Twin pregnancy (maternal grandmother)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No current use; abstinent since pregnancy confirmation. Occupation: Administrative coordinator (currently on modified bed rest).

VITAL SIGNS:
On examination, her vital signs are stable,

PHYSICAL EXAMINATION:
Her abdomen is appropriately sized for gestational age, but fetal heart tones are not audible with a Doppler ultrasound device.

IMAGING:
A bedside ultrasound is performed, which shows absence of fetal cardiac activity.
