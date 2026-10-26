# synthetic_hospital patient 1674

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 30 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-06-13 (Fri) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-03-15 | 2025-07-28 (Mon) | outpatient | Obstetrics & Gynecology |
| 2 | 2020-05-15 | 2025-09-28 (Sun) | ed | Emergency Medicine |
| 3 | 2020-07-15 | 2025-12-08 (Mon) | outpatient | Obstetrics & Gynecology |
| 4 | 2020-08-15 | 2025-12-26 (Fri) | ed | Emergency Medicine |
| 5 | 2020-08-15 | 2025-12-26 (Fri) | ed | Emergency Medicine |
| 6 | 2020-08-15 | 2025-12-30 (Tue) | inpatient | Obstetrics & Gynecology |
| 7 | 2020-09-15 | 2026-02-04 (Wed) | inpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "None (pregnant)",
 "family_history": [
  "Maternal history of preeclampsia",
  "Paternal history of hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 tablet daily",
   "name": "Prenatal vitamin"
  },
  {
   "dose": "325 mg daily",
   "name": "Ferrous sulfate"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Iron deficiency anemia"
 ],
 "age_at_first_encounter": 30
}
```

Primary diagnoses: ["Gestational hypertension", "Conservative management of placenta previa with bed rest", "Maternal chorioamnionitis", "Preterm premature rupture of membranes", "Hypotension due to neuraxial anesthesia", "Intra-amniotic infection (Chorioamnionitis) complicating Preterm Premature Rupture of Membranes", "Congenital cytomegalovirus infection", "Pelvic Girdle Pain (Pregnancy-related)"]
Comorbidities: ["Pregnancy at 32 weeks gestation", "Multiparity", "Personal History of Cholelithiasis"]

## Encounter 0: source 2020-01-15, practice 2025-06-13, outpatient, Obstetrics & Gynecology

Chief complaint: Persistent low back pain for 2 weeks, dull ache worsening by end of day and radiating to buttocks

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent low back pain for 2 weeks, dull ache worsening by end of day and radiating to buttocks

HISTORY OF PRESENT ILLNESS:
She presents to her obstetrician with a 2-week history of persistent low back pain. She describes the pain as a dull ache that worsens by the end of the day and radiates to her buttocks. She denies any trauma, vaginal bleeding, or contractions. She is concerned about the impact of the pain on her ability to work and care for her other child.

PAST MEDICAL HISTORY:
Active Problem List:
- Iron deficiency anemia

Her pregnancy has been uncomplicated.

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily
- Ferrous sulfate 325 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal history of preeclampsia
- Paternal history of hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (pregnant). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, there is no spinal tenderness, neurological deficits, or signs of infection. Fetal heart tones are normal.

PLAN:
She inquires about safe treatment options during pregnancy.

## Encounter 1: source 2020-03-15, practice 2025-07-28, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit, reports decreased fetal movement compared to first pregnancy

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Michael Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit, reports decreased fetal movement compared to first pregnancy

HISTORY OF PRESENT ILLNESS:
Patient returns for prenatal care with iron deficiency anemia and prior back pain. She reports feeling well but mentions her baby seems less active than she remembers from her first pregnancy.

PAST MEDICAL HISTORY:
Active Problem List:
- Pelvic Girdle Pain (Pregnancy-related) (diagnosed 2020-01-15)
- Iron deficiency anemia

She has had an uncomplicated prenatal course thus far. Her first child is healthy with normal development.

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily
- Ferrous sulfate 325 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal history of preeclampsia
- Paternal history of hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (pregnant). Occupation: Elementary school teacher.

REVIEW OF SYSTEMS:
She denies any fever, rash, or exposure to sick contacts.

VITAL SIGNS:
Her vital signs are within normal limits

PHYSICAL EXAMINATION:
Fetal heart tones are normal

LABORATORY DATA:
The patient's serology returns with the following results:

- IgG for cytomegalovirus (CMV): Positive
- IgM for CMV: Positive
- Rubella IgG: Positive
- Rubella IgM: Negative
- Toxoplasma gondii IgG: Negative
- Toxoplasma gondii IgM: Negative

IMAGING:
However, an ultrasound is performed due to her concern about decreased fetal movements, which reveals bilateral periventricular intracranial calcifications, intrahepatic calcifications, and evidence of fetal growth restriction. There are no signs of placental abnormalities.

## Encounter 2: source 2020-05-15, practice 2025-09-28, ed, Emergency Medicine

Chief complaint: Sudden onset of painless vaginal bleeding, history of low-lying placenta noted at 20 weeks

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Jennifer Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of painless vaginal bleeding, history of low-lying placenta noted at 20 weeks

HISTORY OF PRESENT ILLNESS:
This 30-year-old with iron deficiency anemia and back pain returns. She reports feeling the baby move less frequently over the past hour. Her pregnancy has been complicated by a low-lying placenta noted on an ultrasound at 20 weeks gestation.

PAST MEDICAL HISTORY:
Active Problem List:
- Pelvic Girdle Pain (Pregnancy-related) (diagnosed 2020-01-15)
- Congenital cytomegalovirus infection (diagnosed 2020-03-15)
- Iron deficiency anemia

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily
- Ferrous sulfate 325 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal history of preeclampsia
- Paternal history of hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (pregnant). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On examination, her vital signs are stable, and the abdomen is soft and non-tender. Fetal heart tones are 110 beats per minute. A speculum examination is performed, and shows a small amount of blood in the vaginal canal, but no active bleeding. The cervix is closed and there is no evidence of placental tissue in the cervical os.

LABORATORY DATA:
Hemoglobin: 12.2 g/dL
Platelets: 250,000 /μL
Prothrombin time (PT): 12 seconds
Partial thromboplastin time (PTT): 30 seconds

IMAGING:
An emergency bedside ultrasound reveals a placenta that is now appropriately located away from the cervix, but there are fetal vessels crossing the internal cervical os.

## Encounter 3: source 2020-07-15, practice 2025-12-08, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit with occasional mild headaches over past week, baseline BP 118/74

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit with occasional mild headaches over past week, baseline BP 118/74

HISTORY OF PRESENT ILLNESS:
Patient with iron deficiency returns for prenatal care following prior back pain, decreased fetal movement, and vaginal bleeding. She reports feeling generally well but mentions occasional mild headaches over the past week. Her baseline blood pressure at the beginning of pregnancy was 118/74 mmHg. The patient denies any visual disturbances or right upper quadrant pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Pelvic Girdle Pain (Pregnancy-related) (diagnosed 2020-01-15)
- Congenital cytomegalovirus infection (diagnosed 2020-03-15)
- Conservative management of placenta previa with bed rest (diagnosed 2020-05-15)
- Iron deficiency anemia

Her pregnancy has been uncomplicated thus far.

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily
- Ferrous sulfate 325 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal history of preeclampsia
- Paternal history of hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (pregnant). Occupation: Elementary school teacher.

VITAL SIGNS:
Her blood pressure is measured at 152/96 mmHg. Repeat measurement confirms persistent hypertension.

PHYSICAL EXAMINATION:
Physical examination is unremarkable with no signs of edema, and reflexes are normal. Fundoscopic examination does not reveal any retinal changes.

LABORATORY DATA:
Urinalysis shows a trace amount of protein.

Laboratory Values:
- Hemoglobin: 12.8 g/dL
- Platelet count: 210,000/mm³
- Serum creatinine: 0.8 mg/dL
- AST: 28 U/L
- ALT: 26 U/L
- Urine protein: Trace

## Encounter 4: source 2020-08-15, practice 2025-12-26, ed, Emergency Medicine

Chief complaint: Sudden gush of clear fluid from vagina while shopping 1 hour ago, denies bleeding or contractions

ENCOUNTER NOTE
Date: 2020-08-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden gush of clear fluid from vagina while shopping 1 hour ago, denies bleeding or contractions

HISTORY OF PRESENT ILLNESS:
The patient is a 30-year-old female with iron deficiency anemia. She has a history of gestational hypertension, placenta previa, and congenital cytomegalovirus infection during this pregnancy and presents to the emergency department with a complaint of sudden gush of clear fluid from the vagina while shopping 1 hour ago. She denies any vaginal bleeding, contractions, or abdominal pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Pelvic Girdle Pain (Pregnancy-related) (diagnosed 2020-01-15)
- Congenital cytomegalovirus infection (diagnosed 2020-03-15)
- Conservative management of placenta previa with bed rest (diagnosed 2020-05-15)
- Gestational hypertension (diagnosed 2020-07-15)
- Iron deficiency anemia

Her previous pregnancy was uncomplicated, and she had a vaginal delivery at term.

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily
- Ferrous sulfate 325 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal history of preeclampsia
- Paternal history of hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (pregnant). Occupation: Elementary school teacher.

VITAL SIGNS:
Her temperature is 36.8°C (98.2°F), blood pressure is 120/75 mmHg, pulse is 88/min, and respiratory rate is 16/min.

PHYSICAL EXAMINATION:
Speculum examination confirms pooling of clear fluid in the posterior fornix. Nitrazine test is positive, and ferning is observed under the microscope.

LABORATORY DATA:
Complete blood count and C-reactive protein are within normal limits.

OTHER STUDIES:
Fetal heart tracing is reassuring with a baseline of 140/min, moderate variability, no decelerations, and no contractions.

PLAN:
The patient is admitted for inpatient monitoring.

## Encounter 5: source 2020-08-15, practice 2025-12-26, ed, Emergency Medicine

Chief complaint: Clear vaginal fluid leakage and lower abdominal pain for 6 hours, denies trauma or sexual intercourse

ENCOUNTER NOTE
Date: 2020-08-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Clear vaginal fluid leakage and lower abdominal pain for 6 hours, denies trauma or sexual intercourse

HISTORY OF PRESENT ILLNESS:
Patient with iron deficiency anemia returns to the emergency department after presenting earlier today with fluid leakage, now with a 6-hour history of clear vaginal fluid leakage and lower abdominal pain. She denies any trauma or sexual intercourse prior to symptom onset.

PAST MEDICAL HISTORY:
Active Problem List:
- Pelvic Girdle Pain (Pregnancy-related) (diagnosed 2020-01-15)
- Congenital cytomegalovirus infection (diagnosed 2020-03-15)
- Conservative management of placenta previa with bed rest (diagnosed 2020-05-15)
- Gestational hypertension (diagnosed 2020-07-15)
- Preterm premature rupture of membranes (diagnosed 2020-08-15)
- Iron deficiency anemia

Her prenatal course was unremarkable except for mild anemia.

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily
- Ferrous sulfate 325 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal history of preeclampsia
- Paternal history of hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (pregnant). Occupation: Elementary school teacher.

VITAL SIGNS:
Temperature is 38.6°C (101.5°F), blood pressure is 110/70 mmHg, pulse is 110/min, and respiratory rate is 20/min

PHYSICAL EXAMINATION:
Fetal heart rate tracing shows a baseline of 160/min with minimal variability and no accelerations or decelerations. Speculum examination confirms pooling of fluid in the posterior vaginal fornix. The fluid is positive for ferning and nitrazine test.

LABORATORY DATA:
White blood cell count: 15,000/mm³
- Neutrophils: 80%
- Hemoglobin: 10.5 g/dL
- C-reactive protein: 25 mg/L (normal <10 mg/L)

## Encounter 6: source 2020-08-15, practice 2025-12-30, inpatient, Obstetrics & Gynecology

Chief complaint: Clear vaginal fluid leakage with mild lower abdominal discomfort for 4 hours, admitted for management

ENCOUNTER NOTE
Date: 2020-08-15
Provider: Dr. James Thompson, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Clear vaginal fluid leakage with mild lower abdominal discomfort for 4 hours, admitted for management

HISTORY OF PRESENT ILLNESS:
This patient returns for admission following ED presentation today, denying recent sexual intercourse or vaginal manipulation. Her prenatal course was unremarkable until this episode.

PAST MEDICAL HISTORY:
Active Problem List:
- Pelvic Girdle Pain (Pregnancy-related) (diagnosed 2020-01-15)
- Congenital cytomegalovirus infection (diagnosed 2020-03-15)
- Conservative management of placenta previa with bed rest (diagnosed 2020-05-15)
- Gestational hypertension (diagnosed 2020-07-15)
- Preterm premature rupture of membranes (diagnosed 2020-08-15)
- Maternal chorioamnionitis (diagnosed 2020-08-15)
- Iron deficiency anemia

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily
- Ferrous sulfate 325 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal history of preeclampsia
- Paternal history of hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (pregnant). Occupation: Elementary school teacher.

VITAL SIGNS:
On examination, her temperature is 38.6°C (101.5°F), heart rate is 110 bpm, blood pressure is 120/75 mmHg, and respiratory rate is 18 breaths per minute.

PHYSICAL EXAMINATION:
Fetal heart tracing shows a baseline rate of 170 bpm with minimal variability and no decelerations. Sterile speculum examination confirms pooling of fluid in the posterior vaginal fornix. Nitrazine test is positive, and ferning is observed under microscopy.

LABORATORY DATA:
Complete blood count reveals:

- White blood cell count: 15,000/µL
- Hemoglobin: 12.2 g/dL
- Platelet count: 210,000/µL

C-reactive protein (CRP) is elevated. Based on the clinical findings,

## Encounter 7: source 2020-09-15, practice 2026-02-04, inpatient, Obstetrics & Gynecology

Chief complaint: Active labor requesting pain relief, epidural placed 2 hours ago with subsequent hypotension and late decelerations

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Lisa Wong, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Active labor requesting pain relief, epidural placed 2 hours ago with subsequent hypotension and late decelerations

HISTORY OF PRESENT ILLNESS:
The 30-year-old with iron deficiency anemia, gestational hypertension, and recent intra-amniotic infection is in active labor. An epidural analgesia is placed without immediate complications. Two hours later, the fetal heart rate monitor shows recurrent late decelerations.

PAST MEDICAL HISTORY:
Active Problem List:
- Pelvic Girdle Pain (Pregnancy-related) (diagnosed 2020-01-15)
- Congenital cytomegalovirus infection (diagnosed 2020-03-15)
- Conservative management of placenta previa with bed rest (diagnosed 2020-05-15)
- Gestational hypertension (diagnosed 2020-07-15)
- Preterm premature rupture of membranes (diagnosed 2020-08-15)
- Maternal chorioamnionitis (diagnosed 2020-08-15)
- Intra-amniotic infection (Chorioamnionitis) complicating Preterm Premature Rupture of Membranes (diagnosed 2020-08-15)
- Iron deficiency anemia

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily
- Ferrous sulfate 325 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal history of preeclampsia
- Paternal history of hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (pregnant). Occupation: Elementary school teacher.

VITAL SIGNS:
The patient's blood pressure is noted to have dropped from 120/80 mmHg to 90/60 mmHg. Her pulse is 88 beats per minute, and she is breathing comfortably at 16 breaths per minute.

PHYSICAL EXAMINATION:
On examination, her skin is warm and well-perfused, and there is no evidence of bleeding.

PLAN:
The obstetrician is called to assess the situation.
