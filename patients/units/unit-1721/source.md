# synthetic_hospital patient 1721

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 33 at first encounter, sex F, Asian American, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-10-18 (Fri) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-01-15 | 2024-10-18 (Fri) | inpatient | Obstetrics & Gynecology |
| 2 | 2021-01-15 | 2025-11-01 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Commercial PPO",
 "occupation": "High school mathematics teacher",
 "alcohol_use": "None (abstinent)",
 "family_history": [
  "Mother with gestational hypertension",
  "Maternal aunt with asthma",
  "No family history of thromboembolic disease"
 ],
 "race_ethnicity": "Asian American",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "90 mcg/actuation inhaler, 2 inhalations every 4-6 hours as needed for wheezing",
   "name": "Albuterol sulfate"
  }
 ],
 "surgical_history": [
  "Dilation and curettage for first-trimester spontaneous abortion (age 31)",
  "Suction curettage for complete hydatidiform mole (age 33)",
  "Uncomplicated spontaneous vaginal delivery (age 34)"
 ],
 "chronic_conditions": [
  "Mild intermittent asthma"
 ],
 "age_at_first_encounter": 33
}
```

Primary diagnoses: ["Complete hydatidiform mole", "Postpartum hemorrhage"]
Comorbidities: ["Bilateral theca lutein cysts", "Gestational hypertension", "Hyperemesis gravidarum"]

## Encounter 0: source 2020-01-15, practice 2024-10-18, outpatient, Obstetrics & Gynecology

Chief complaint: Nausea, vomiting, and light vaginal spotting at 8 weeks gestation

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Sharma, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Nausea, vomiting, and light vaginal spotting at 8 weeks gestation

HISTORY OF PRESENT ILLNESS:
After experiencing persistent nausea and multiple episodes of vomiting over the past 10 days. She also reports light vaginal spotting but denies abdominal cramping or passage of tissue.

PAST MEDICAL HISTORY:
Active Problem List:
- Mild intermittent asthma

Her prenatal history is notable for a previous first-trimester miscarriage.

PAST SURGICAL HISTORY:
- Dilation and curettage for first-trimester spontaneous abortion (age 31)
- Suction curettage for complete hydatidiform mole (age 33)
- Uncomplicated spontaneous vaginal delivery (age 34)

MEDICATIONS:
Home Medications:
- Albuterol sulfate 90 mcg/actuation inhaler, 2 inhalations every 4-6 hours as needed for wheezing

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with gestational hypertension
- Maternal aunt with asthma
- No family history of thromboembolic disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (abstinent). Occupation: High school mathematics teacher.

VITAL SIGNS:
On examination, her blood pressure is 142/88 mm Hg and pulse is 98/min.

PHYSICAL EXAMINATION:
The uterus is palpated at the level of the umbilicus, and there is fullness noted in both adnexal regions, more prominent on the right.

LABORATORY DATA:
Serum β-hCG is 290,000 mIU/mL.

IMAGING:
Transvaginal ultrasound reveals a heterogeneous intrauterine mass with diffuse cystic spaces and no fetal pole.

## Encounter 1: source 2020-01-15, practice 2024-10-18, inpatient, Obstetrics & Gynecology

Chief complaint: Severe persistent nausea and vomiting with confirmed complete hydatidiform mole requiring surgical management

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Sharma, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Severe persistent nausea and vomiting with confirmed complete hydatidiform mole requiring surgical management

HISTORY OF PRESENT ILLNESS:
The patient is a 33-year-old female with mild intermittent asthma who was evaluated earlier today in the outpatient setting. She returns for inpatient management, presenting at 8 weeks of gestation with persistent nausea and multiple episodes of vomiting over the past 10 days. She also reports light vaginal spotting but denies abdominal cramping or passage of tissue.

PAST MEDICAL HISTORY:
Active Problem List:
- Complete hydatidiform mole (diagnosed 2020-01-15)
- Mild intermittent asthma

Her prenatal history is notable for a previous first-trimester miscarriage.

PAST SURGICAL HISTORY:
- Dilation and curettage for first-trimester spontaneous abortion (age 31)
- Suction curettage for complete hydatidiform mole (age 33)
- Uncomplicated spontaneous vaginal delivery (age 34)

MEDICATIONS:
Home Medications:
- Albuterol sulfate 90 mcg/actuation inhaler, 2 inhalations every 4-6 hours as needed for wheezing

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with gestational hypertension
- Maternal aunt with asthma
- No family history of thromboembolic disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (abstinent). Occupation: High school mathematics teacher.

VITAL SIGNS:
On examination, her blood pressure is 144/88 mm Hg and pulse is 98/min.

PHYSICAL EXAMINATION:
The uterus is palpated at the level of the umbilicus, and there is fullness noted in both adnexal regions.

LABORATORY DATA:
Serum β-hCG is 290,000 mIU/mL.

IMAGING:
Transvaginal ultrasound reveals a heterogeneous intrauterine mass with diffuse cystic spaces and no fetal pole.

## Encounter 2: source 2021-01-15, practice 2025-11-01, ed, Emergency Medicine

Chief complaint: Heavy vaginal bleeding 2 hours after spontaneous vaginal delivery

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Heavy vaginal bleeding 2 hours after spontaneous vaginal delivery

HISTORY OF PRESENT ILLNESS:
The 34-year-old female with mild intermittent asthma and history of complete hydatidiform mole managed surgically one year ago presents to the emergency department 2 hours after a spontaneous vaginal delivery with heavy vaginal bleeding. She delivered a healthy full-term infant after an uncomplicated pregnancy. Her prenatal course was notable for well-controlled gestational hypertension, without the need for medication postpartum. Bimanual massage and administration of oxytocin have not decreased the bleeding.

PAST MEDICAL HISTORY:
Active Problem List:
- Complete hydatidiform mole (diagnosed 2020-01-15)
- Mild intermittent asthma

Her past medical history is significant for mild intermittent asthma, for which she uses an albuterol inhaler as needed.

PAST SURGICAL HISTORY:
- Dilation and curettage for first-trimester spontaneous abortion (age 31)
- Suction curettage for complete hydatidiform mole (age 33)
- Uncomplicated spontaneous vaginal delivery (age 34)

MEDICATIONS:
She takes albuterol inhaler as needed.

Home Medications:
- Albuterol sulfate 90 mcg/actuation inhaler, 2 inhalations every 4-6 hours as needed for wheezing

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with gestational hypertension
- Maternal aunt with asthma
- No family history of thromboembolic disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (abstinent). Occupation: High school mathematics teacher.

VITAL SIGNS:
On examination, her blood pressure is 150/95 mmHg, heart rate is 110 beats per minute

PHYSICAL EXAMINATION:
The uterine fundus is boggy
