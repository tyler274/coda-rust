# synthetic_hospital patient 1734

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 29 at first encounter, sex F, Caucasian, Blue Cross Blue Shield (PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-05-08 (Sun) | icu | Obstetrics & Gynecology |
| 1 | 2022-01-15 | 2024-04-29 (Mon) | outpatient | Obstetrics & Gynecology |
| 2 | 2022-04-15 | 2024-07-19 (Fri) | outpatient | Obstetrics & Gynecology |
| 3 | 2022-07-15 | 2024-10-25 (Fri) | outpatient | Obstetrics & Gynecology |
| 4 | 2022-09-15 | 2024-12-27 (Fri) | ed | Emergency Medicine |
| 5 | 2022-11-15 | 2025-02-18 (Tue) | inpatient | Obstetrics & Gynecology |
| 6 | 2022-12-15 | 2025-03-29 (Sat) | icu | Obstetrics & Gynecology |
| 7 | 2023-07-15 | 2025-10-27 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield (PPO)",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Rare social use, none during pregnancies",
 "family_history": [
  "Mother with chronic hypertension",
  "Maternal aunt with severe preeclampsia",
  "Previous child with transient hypoglycemia (resolved)",
  "No known bleeding disorders"
 ],
 "race_ethnicity": "Caucasian",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "200 mg PO twice daily (adherence reported)",
   "name": "Labetalol"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Prenatal vitamin with iron"
  }
 ],
 "surgical_history": [
  "Manual removal of retained placenta (age 27)",
  "Cesarean section (implied by possible repeat surgeries, but not explicitly stated - will leave as manual removal only)"
 ],
 "chronic_conditions": [
  "Chronic hypertension",
  "History of postpartum hemorrhage"
 ],
 "age_at_first_encounter": 29
}
```

Primary diagnoses: ["Disseminated intravascular coagulation", "Physiologic dyspnea of pregnancy", "Traumatic hemorrhagic shock with acute coagulopathy", "Bilateral adrenal hemorrhage (Waterhouse-Friderichsen syndrome)", "Pelvic Girdle Pain (Pregnancy-related)", "Acute fatty liver of pregnancy", "Umbilical cord compression causing recurrent variable decelerations"]
Comorbidities: ["Pregnancy", "Acute posthemorrhagic anemia"]

## Encounter 0: source 2020-01-15, practice 2022-05-08, icu, Obstetrics & Gynecology

Chief complaint: Generalized weakness and excessive vaginal bleeding 12 hours after delivering first child

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: icu

CHIEF COMPLAINT:
Generalized weakness and excessive vaginal bleeding 12 hours after delivering first child

HISTORY OF PRESENT ILLNESS:
12 hours after delivering her first child. The delivery was complicated by a retained placenta, which required manual removal. Her prenatal course was unremarkable

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic hypertension
- History of postpartum hemorrhage

PAST SURGICAL HISTORY:
- Manual removal of retained placenta (age 27)
- Cesarean section (implied by possible repeat surgeries, but not explicitly stated - will leave as manual removal only)

MEDICATIONS:
Home Medications:
- Labetalol 200 mg PO twice daily (adherence reported)
- Prenatal vitamin with iron 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with chronic hypertension
- Maternal aunt with severe preeclampsia
- Previous child with transient hypoglycemia (resolved)
- No known bleeding disorders

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare social use, none during pregnancies. Occupation: Elementary school teacher.

VITAL SIGNS:
Blood pressure is 90/60 mmHg, heart rate is 120 beats per minute, and temperature is 37.2°C (99.0°F)

PHYSICAL EXAMINATION:
She appears pale and diaphoretic. Pelvic examination reveals moderate vaginal bleeding.

LABORATORY DATA:
- Hemoglobin: 8 g/dL (Normal: 12-16 g/dL)
- Platelet count: 50,000/mm³ (Normal: 150,000-400,000/mm³)
- Prothrombin time (PT): 18 seconds (Normal: 12-14 seconds)
- Partial thromboplastin time (PTT): 45 seconds (Normal: 25-35 seconds)
- Fibrinogen: 150 mg/dL (Normal: 200-400 mg/dL)
- D-dimer: Elevated

## Encounter 1: source 2022-01-15, practice 2024-04-29, outpatient, Obstetrics & Gynecology

Chief complaint: Shortness of breath, especially when lying down, during second pregnancy

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Maria Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Shortness of breath, especially when lying down, during second pregnancy

HISTORY OF PRESENT ILLNESS:
This 31-year-old G2P1 with chronic hypertension and prior postpartum hemorrhage presents for second pregnancy with complaints of shortness of breath, especially when lying down. She denies any cough, fever, or wheezing.

PAST MEDICAL HISTORY:
Active Problem List:
- Disseminated intravascular coagulation (diagnosed 2020-01-15)
- Chronic hypertension
- History of postpartum hemorrhage

She has had an uncomplicated prenatal course thus far.

PAST SURGICAL HISTORY:
- Manual removal of retained placenta (age 27)
- Cesarean section (implied by possible repeat surgeries, but not explicitly stated - will leave as manual removal only)

MEDICATIONS:
Home Medications:
- Labetalol 200 mg PO twice daily (adherence reported)
- Prenatal vitamin with iron 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with chronic hypertension
- Maternal aunt with severe preeclampsia
- Previous child with transient hypoglycemia (resolved)
- No known bleeding disorders

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare social use, none during pregnancies. Occupation: Elementary school teacher.

VITAL SIGNS:
On examination, her respiratory rate is 18 breaths per minute, and oxygen saturation is 97% on room air.

PHYSICAL EXAMINATION:
Lung auscultation reveals clear breath sounds bilaterally. Her abdomen is gravid, and fundal height is appropriate for gestational age.

OTHER STUDIES:
A spirometry test yields the following results:

- Forced vital capacity (FVC): 4.0 L (normal)
- Forced expiratory volume in 1 second (FEV1): 3.2 L (normal)
- FEV1/FVC ratio: 80% (normal)
- Residual volume (RV): Decreased
- Functional residual capacity (FRC): Decreased

## Encounter 2: source 2022-04-15, practice 2024-07-19, outpatient, Obstetrics & Gynecology

Chief complaint: Progressive pain over pubic bone and bilateral groin areas worsened by walking or turning in bed

ENCOUNTER NOTE
Date: 2022-04-15
Provider: Dr. Maria Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive pain over pubic bone and bilateral groin areas worsened by walking or turning in bed

HISTORY OF PRESENT ILLNESS:
This 31-year-old with chronic hypertension and postpartum hemorrhage returns for prenatal care following her January visit. She reports that the pain is worse when walking, climbing stairs, or turning in bed. She denies leakage of fluid, vaginal bleeding, or contractions.

PAST MEDICAL HISTORY:
Active Problem List:
- Disseminated intravascular coagulation (diagnosed 2020-01-15)
- Physiologic dyspnea of pregnancy (diagnosed 2022-01-15)
- Chronic hypertension
- History of postpartum hemorrhage

Her pregnancy has been uncomplicated.

PAST SURGICAL HISTORY:
- Manual removal of retained placenta (age 27)
- Cesarean section (implied by possible repeat surgeries, but not explicitly stated - will leave as manual removal only)

MEDICATIONS:
Home Medications:
- Labetalol 200 mg PO twice daily (adherence reported)
- Prenatal vitamin with iron 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with chronic hypertension
- Maternal aunt with severe preeclampsia
- Previous child with transient hypoglycemia (resolved)
- No known bleeding disorders

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare social use, none during pregnancies. Occupation: Elementary school teacher.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, she has tenderness over the pubic symphysis and pain with passive abduction of her hips. There is no erythema or edema over the joints, and her skin is intact.

LABORATORY DATA:
Urinalysis is negative for protein, glucose, and infection.

## Encounter 3: source 2022-07-15, practice 2024-10-25, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit with reports of decreased fetal movements over past two days

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. James Park, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit with reports of decreased fetal movements over past two days

HISTORY OF PRESENT ILLNESS:
The patient is a 31-year-old G2P1 female with chronic hypertension and a history of postpartum hemorrhage, now returning for prenatal care following prior visits for physiologic dyspnea and pelvic girdle pain. She has a history of chronic hypertension, which has been poorly controlled during pregnancy despite adherence to labetalol therapy. She reports decreased fetal movements over the past two days. Her prenatal course has been otherwise unremarkable.

PAST MEDICAL HISTORY:
Active Problem List:
- Disseminated intravascular coagulation (diagnosed 2020-01-15)
- Physiologic dyspnea of pregnancy (diagnosed 2022-01-15)
- Pelvic Girdle Pain (Pregnancy-related) (diagnosed 2022-04-15)
- Chronic hypertension
- History of postpartum hemorrhage

PAST SURGICAL HISTORY:
- Manual removal of retained placenta (age 27)
- Cesarean section (implied by possible repeat surgeries, but not explicitly stated - will leave as manual removal only)

MEDICATIONS:
She takes labetalol therapy.

Home Medications:
- Prenatal vitamin with iron 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with chronic hypertension
- Maternal aunt with severe preeclampsia
- Previous child with transient hypoglycemia (resolved)
- No known bleeding disorders

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare social use, none during pregnancies. Occupation: Elementary school teacher.

VITAL SIGNS:
Her blood pressure is 152/96 mmHg

PHYSICAL EXAMINATION:
Fundal height is measuring 30 cm, which is less than expected for gestational age

IMAGING:
An ultrasound is performed, which shows an estimated fetal weight in the 8th percentile for gestational age, but with a normal head circumference. Doppler flow studies of the umbilical artery reveal increased resistance to blood flow.

## Encounter 4: source 2022-09-15, practice 2024-12-27, ed, Emergency Medicine

Chief complaint: Persistent nausea, vomiting, and severe epigastric pain radiating to right upper quadrant

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Persistent nausea, vomiting, and severe epigastric pain radiating to right upper quadrant

HISTORY OF PRESENT ILLNESS:
This 31-year-old female with chronic hypertension, postpartum hemorrhage, and recent bilateral adrenal hemorrhage presents to the emergency department with a 24-hour history of persistent nausea, vomiting, and severe epigastric pain. She reports that the pain radiates to her right upper quadrant.

PAST MEDICAL HISTORY:
Active Problem List:
- Disseminated intravascular coagulation (diagnosed 2020-01-15)
- Physiologic dyspnea of pregnancy (diagnosed 2022-01-15)
- Pelvic Girdle Pain (Pregnancy-related) (diagnosed 2022-04-15)
- Bilateral adrenal hemorrhage (Waterhouse-Friderichsen syndrome) (diagnosed 2022-07-15)
- Chronic hypertension
- History of postpartum hemorrhage

Her pregnancy has been uncomplicated until now.

PAST SURGICAL HISTORY:
- Manual removal of retained placenta (age 27)
- Cesarean section (implied by possible repeat surgeries, but not explicitly stated - will leave as manual removal only)

MEDICATIONS:
Home Medications:
- Labetalol 200 mg PO twice daily (adherence reported)
- Prenatal vitamin with iron 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with chronic hypertension
- Maternal aunt with severe preeclampsia
- Previous child with transient hypoglycemia (resolved)
- No known bleeding disorders

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare social use, none during pregnancies. Occupation: Elementary school teacher.

VITAL SIGNS:
On examination, her blood pressure is 145/95 mmHg, heart rate is 102 beats per minute, and she is afebrile.

PHYSICAL EXAMINATION:
Her skin and sclera are anicteric. Abdominal examination reveals right upper quadrant tenderness without rebound or guarding.

LABORATORY DATA:
Laboratory studies are as follows:

- Hemoglobin: 12.1 g/dL
- Platelet count: 155,000/mm^3
- Aspartate aminotransferase (AST): 250 U/L
- Alanine aminotransferase (ALT): 230 U/L
- Total bilirubin: 1.2 mg/dL
- Alkaline phosphatase: 120 U/L
- Serum glucose: 45 mg/dL
- Prothrombin time (PT): 16 seconds (control 12 seconds)
- Partial thromboplastin time (PTT): 40 seconds (control 30 seconds)
- Fibrinogen: 150 mg/dL

OTHER STUDIES:
Fetal monitoring shows a reactive fetal heart tracing.

PLAN:
The obstetric team is consulted.

## Encounter 5: source 2022-11-15, practice 2025-02-18, inpatient, Obstetrics & Gynecology

Chief complaint: Sudden onset irregular abdominal pain at term gestation

ENCOUNTER NOTE
Date: 2022-11-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Sudden onset irregular abdominal pain at term gestation

HISTORY OF PRESENT ILLNESS:
The patient is a 31-year-old G2P1 with chronic hypertension and prior postpartum hemorrhage who presents to the labor and delivery unit at term following recent acute fatty liver of pregnancy and adrenal hemorrhage. She reports a 1-hour history of sudden onset, irregular abdominal pain. She denies any vaginal bleeding or leakage of fluid. Her pregnancy has been uncomplicated, and she has received regular prenatal care.

PAST MEDICAL HISTORY:
Active Problem List:
- Disseminated intravascular coagulation (diagnosed 2020-01-15)
- Physiologic dyspnea of pregnancy (diagnosed 2022-01-15)
- Pelvic Girdle Pain (Pregnancy-related) (diagnosed 2022-04-15)
- Bilateral adrenal hemorrhage (Waterhouse-Friderichsen syndrome) (diagnosed 2022-07-15)
- Acute fatty liver of pregnancy (diagnosed 2022-09-15)
- Chronic hypertension
- History of postpartum hemorrhage

PAST SURGICAL HISTORY:
- Manual removal of retained placenta (age 27)
- Cesarean section (implied by possible repeat surgeries, but not explicitly stated - will leave as manual removal only)

MEDICATIONS:
Home Medications:
- Labetalol 200 mg PO twice daily (adherence reported)
- Prenatal vitamin with iron 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with chronic hypertension
- Maternal aunt with severe preeclampsia
- Previous child with transient hypoglycemia (resolved)
- No known bleeding disorders

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare social use, none during pregnancies. Occupation: Elementary school teacher.

VITAL SIGNS:
On examination, her blood pressure is 120/75 mmHg, heart rate is 88 beats per minute, and temperature is 37.0°C (98.6°F)

PHYSICAL EXAMINATION:
Fetal heart monitoring shows a baseline fetal heart rate of 140 beats per minute with recurrent variable decelerations that decrease to 90 beats per minute with >50% of uterine contractions. The decelerations are noted to have a rapid descent and return to baseline.
There is no evidence of maternal fever, tachycardia, or uterine tenderness.

PLAN:
The patient is placed in the left lateral decubitus position, and oxygen is administered via face mask. Despite these measures, the fetal heart tracing does not improve.

## Encounter 6: source 2022-12-15, practice 2025-03-29, icu, Obstetrics & Gynecology

Chief complaint: Sudden-onset severe abdominal pain and vaginal bleeding with significantly decreased fetal movements

ENCOUNTER NOTE
Date: 2022-12-15
Provider: Dr. James Park, MD
Department: Obstetrics & Gynecology
Visit Type: icu

CHIEF COMPLAINT:
Sudden-onset severe abdominal pain and vaginal bleeding with significantly decreased fetal movements

HISTORY OF PRESENT ILLNESS:
This 31-year-old G2P1 female with chronic hypertension has a history of postpartum hemorrhage complicated by disseminated intravascular coagulation and antepartum complications including adrenal hemorrhage. She presents to the emergency department with sudden-onset severe abdominal pain and vaginal bleeding that began 3 hours ago. She reports that the pain is constant and not associated with uterine contractions. Fetal movements have significantly decreased since the onset of symptoms. Shortly after arrival, the patient develops oozing of blood from her intravenous catheter site and persistent vaginal bleeding.

PAST MEDICAL HISTORY:
Active Problem List:
- Disseminated intravascular coagulation (diagnosed 2020-01-15)
- Physiologic dyspnea of pregnancy (diagnosed 2022-01-15)
- Pelvic Girdle Pain (Pregnancy-related) (diagnosed 2022-04-15)
- Bilateral adrenal hemorrhage (Waterhouse-Friderichsen syndrome) (diagnosed 2022-07-15)
- Acute fatty liver of pregnancy (diagnosed 2022-09-15)
- Umbilical cord compression causing recurrent variable decelerations (diagnosed 2022-11-15)
- Chronic hypertension
- History of postpartum hemorrhage

Her past pregnancies were uneventful.

PAST SURGICAL HISTORY:
- Manual removal of retained placenta (age 27)
- Cesarean section (implied by possible repeat surgeries, but not explicitly stated - will leave as manual removal only)

MEDICATIONS:
Home Medications:
- Labetalol 200 mg PO twice daily (adherence reported)
- Prenatal vitamin with iron 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with chronic hypertension
- Maternal aunt with severe preeclampsia
- Previous child with transient hypoglycemia (resolved)
- No known bleeding disorders

SOCIAL HISTORY:
She does not smoke or use drugs.

VITAL SIGNS:
Blood pressure is 90/58 mm Hg, pulse is 118/min

PHYSICAL EXAMINATION:
On examination, her uterus is firm and tender to palpation, and there is moderate vaginal bleeding. She appears pale and diaphoretic. There is no external trauma.

LABORATORY DATA:
Hemoglobin 8.3 g/dL
Platelet count 44,000/mm³
Prothrombin time (PT) 24 seconds
Activated PTT 60 seconds
Fibrinogen 90 mg/dL
Fibrin degradation products Elevated

## Encounter 7: source 2023-07-15, practice 2025-10-27, ed, Emergency Medicine

Chief complaint: Dizziness and fatigue following motor vehicle collision

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Dizziness and fatigue following motor vehicle collision

HISTORY OF PRESENT ILLNESS:
The patient is a 32-year-old female with chronic hypertension on labetalol and a history of postpartum hemorrhage complicated by disseminated intravascular coagulation, who presents after a motor vehicle collision. She reports that she was the restrained driver in a head-on collision at moderate speed. Airbags were deployed, and she did not lose consciousness. She complains of mild abdominal pain but no other specific symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Disseminated intravascular coagulation (diagnosed 2020-01-15)
- Physiologic dyspnea of pregnancy (diagnosed 2022-01-15)
- Pelvic Girdle Pain (Pregnancy-related) (diagnosed 2022-04-15)
- Bilateral adrenal hemorrhage (Waterhouse-Friderichsen syndrome) (diagnosed 2022-07-15)
- Acute fatty liver of pregnancy (diagnosed 2022-09-15)
- Umbilical cord compression causing recurrent variable decelerations (diagnosed 2022-11-15)
- Chronic hypertension
- History of postpartum hemorrhage

PAST SURGICAL HISTORY:
- Manual removal of retained placenta (age 27)
- Cesarean section (implied by possible repeat surgeries, but not explicitly stated - will leave as manual removal only)

MEDICATIONS:
Home Medications:
- Labetalol 200 mg PO twice daily (adherence reported)
- Prenatal vitamin with iron 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with chronic hypertension
- Maternal aunt with severe preeclampsia
- Previous child with transient hypoglycemia (resolved)
- No known bleeding disorders

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare social use, none during pregnancies. Occupation: Elementary school teacher.

VITAL SIGNS:
Her blood pressure is 90/60 mmHg, heart rate is 120 beats per minute, respiratory rate is 22 breaths per minute, and temperature is 37.0°C (98.6°F). The patient's blood pressure drops to 85/55 mmHg despite receiving 2 liters of normal saline.

PHYSICAL EXAMINATION:
On examination, she appears pale and anxious. Her skin is cool and clammy. Abdominal examination reveals mild tenderness in the left upper quadrant with no rebound or guarding. Bowel sounds are normal. Pelvic examination is unremarkable. No external injuries are noted.

LABORATORY DATA:
Laboratory studies show:

- Hemoglobin: 9.5 g/dL (normal: 12-16 g/dL)
- Hematocrit: 28% (normal: 36-46%)
- Platelets: 250,000/mm³ (normal: 150,000-400,000/mm³)
- White blood cells: 12,000/mm³ (normal: 4,500-11,000/mm³)
- Serum lactate: 4 mmol/L (normal: 0.5-2.2 mmol/L)

IMAGING:
Focused Assessment with Sonography for Trauma (FAST) exam is negative for free fluid. Chest X-ray is unremarkable.
