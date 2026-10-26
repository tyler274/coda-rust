# synthetic_hospital patient 1745

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, Non-Hispanic Black, Private commercial insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-06-27 (Mon) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2022-12-28 (Wed) | outpatient | Obstetrics & Gynecology |
| 2 | 2020-09-15 | 2023-02-25 (Sat) | ed | Emergency Medicine |
| 3 | 2021-07-15 | 2023-12-23 (Sat) | ed | Emergency Medicine |
| 4 | 2022-07-15 | 2024-12-31 (Tue) | ed | Emergency Medicine |
| 5 | 2022-07-15 | 2024-12-31 (Tue) | inpatient | Obstetrics & Gynecology |
| 6 | 2023-07-15 | 2025-12-28 (Sun) | ed | Emergency Medicine |
| 7 | 2023-07-15 | 2025-12-28 (Sun) | icu | Pulmonary & Critical Care |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Private commercial insurance",
 "occupation": "Office administrative assistant",
 "alcohol_use": "None",
 "family_history": [
  "Mother with chronic hypertension",
  "Maternal aunt with cervical insufficiency and preterm delivery",
  "Paternal grandmother with asthma"
 ],
 "race_ethnicity": "Non-Hispanic Black",
 "smoking_status": "Current smoker, 1 pack per day for 10 years",
 "home_medications": [
  {
   "dose": "200 mg PO twice daily",
   "name": "Labetalol"
  },
  {
   "dose": "90 mcg (2 puffs) inhaled every 4 to 6 hours as needed for wheezing or shortness of breath",
   "name": "Albuterol HFA inhaler"
  },
  {
   "dose": "1 tablet daily",
   "name": "Prenatal multivitamins"
  }
 ],
 "surgical_history": [
  "Cesarean delivery (age 26)",
  "Dilation and evacuation for second-trimester pregnancy loss (age 27)",
  "Laparoscopic right salpingectomy for ruptured ectopic pregnancy"
 ],
 "chronic_conditions": [
  "Asthma",
  "Chronic hypertension",
  "Tobacco use disorder",
  "History of cervical insufficiency"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Late spontaneous abortion (history)", "Postpartum hemorrhage", "Refractory hypoxemia in ARDS", "Cervical insufficiency", "Ruptured ectopic pregnancy with hemodynamic instability", "Traumatic hemorrhagic shock with acute coagulopathy", "Placental abruption (Abruptio placentae)", "Acute cystitis"]
Comorbidities: ["Sinus tachycardia secondary to burns", "Acute posthemorrhagic anemia", "Hypotension"]

## Encounter 0: source 2020-01-15, practice 2022-06-27, ed, Emergency Medicine

Chief complaint: severe lower abdominal pain and vaginal bleeding

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe lower abdominal pain and vaginal bleeding

HISTORY OF PRESENT ILLNESS:
She presents to the emergency department with a 6-hour history of severe lower abdominal pain and vaginal bleeding. She reports that the bleeding started suddenly and has soaked through several sanitary pads. Her last menstrual period was 7 weeks ago, and she had a positive home pregnancy test 3 weeks prior.

PAST MEDICAL HISTORY:
Active Problem List:
- Asthma
- Chronic hypertension
- Tobacco use disorder
- History of cervical insufficiency

PAST SURGICAL HISTORY:
- Cesarean delivery (age 26)
- Dilation and evacuation for second-trimester pregnancy loss (age 27)
- Laparoscopic right salpingectomy for ruptured ectopic pregnancy

MEDICATIONS:
Home Medications:
- Labetalol 200 mg PO twice daily
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4 to 6 hours as needed for wheezing or shortness of breath
- Prenatal multivitamins 1 tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with chronic hypertension
- Maternal aunt with cervical insufficiency and preterm delivery
- Paternal grandmother with asthma

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 10 years. Alcohol: None. Occupation: Office administrative assistant.

VITAL SIGNS:
On examination, her blood pressure is 100/60 mmHg, pulse is 110 beats per minute, and temperature is 37.2°C (99.0°F)

PHYSICAL EXAMINATION:
Abdominal examination reveals suprapubic tenderness without rebound or guarding. Pelvic examination shows an open cervical os with moderate bleeding and no tissue in the cervical canal or vagina.

LABORATORY DATA:
Laboratory tests reveal the following:

- Hemoglobin: 10.2 g/dL (normal: 12-16 g/dL)
- White blood cell count: 9,000/mm³ (normal: 4,500-11,000/mm³)
- Platelets: 250,000/mm³ (normal: 150,000-400,000/mm³)
- Serum beta-hCG: 50 mIU/mL (decreased from 1500 mIU/mL one week ago)
- Blood type: A positive

IMAGING:
Transvaginal ultrasonography reveals an empty uterus with no evidence of intrauterine or ectopic pregnancy.

## Encounter 1: source 2020-07-15, practice 2022-12-28, outpatient, Obstetrics & Gynecology

Chief complaint: presents to the obstetrics clinic for a routine prenatal visit

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Maria Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
She presents to the obstetrics clinic for a routine prenatal visit

HISTORY OF PRESENT ILLNESS:
The patient returns to the obstetrics clinic for routine prenatal care with a history of cervical insufficiency and late spontaneous abortion in January 2020. She reports no vaginal bleeding or discharge and no abdominal pain. Her current pregnancy has been uneventful, and she has been compliant with prenatal vitamins and follow-up. The patient expresses concern about the risk of losing this pregnancy as she did the previous one.

PAST MEDICAL HISTORY:
Active Problem List:
- Late spontaneous abortion (history) (diagnosed 2020-01-15)
- Asthma
- Chronic hypertension
- Tobacco use disorder
- History of cervical insufficiency

She has a history of a second-trimester pregnancy loss at 18 weeks gestation one year ago, which was attributed to painless cervical dilation and membrane bulging.

PAST SURGICAL HISTORY:
- Cesarean delivery (age 26)
- Dilation and evacuation for second-trimester pregnancy loss (age 27)
- Laparoscopic right salpingectomy for ruptured ectopic pregnancy

MEDICATIONS:
She takes prenatal vitamins.

Home Medications:
- Labetalol 200 mg PO twice daily
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4 to 6 hours as needed for wheezing or shortness of breath
- Prenatal multivitamins 1 tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with chronic hypertension
- Maternal aunt with cervical insufficiency and preterm delivery
- Paternal grandmother with asthma

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 10 years. Alcohol: None. Occupation: Office administrative assistant.

PHYSICAL EXAMINATION:
Her obstetric examination reveals a closed cervical os with no effacement.

IMAGING:
Transvaginal ultrasound shows a cervical length of 25 mm with no funneling.

## Encounter 2: source 2020-09-15, practice 2023-02-25, ed, Emergency Medicine

Chief complaint: fever, chills, right flank pain, and nausea for the past 24 hours

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, chills, right flank pain, and nausea for the past 24 hours

HISTORY OF PRESENT ILLNESS:
The patient is a 28-year-old female with a history of cervical insufficiency and late spontaneous abortion who presents with acute urinary symptoms. She was last seen in the obstetrics clinic in July for prenatal care. She notes that the pain began in her lower back and gradually migrated upward. She has also experienced urinary urgency and dysuria for several days, but thought it was "just part of pregnancy." Her prenatal course has been uncomplicated so far, denies prior UTIs during this pregnancy. She was screened for asymptomatic bacteriuria during her first trimester but did not return for follow-up.

PAST MEDICAL HISTORY:
Active Problem List:
- Late spontaneous abortion (history) (diagnosed 2020-01-15)
- Cervical insufficiency (diagnosed 2020-07-15)
- Asthma
- Chronic hypertension
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Cesarean delivery (age 26)
- Dilation and evacuation for second-trimester pregnancy loss (age 27)
- Laparoscopic right salpingectomy for ruptured ectopic pregnancy

MEDICATIONS:
She takes her only medication is prenatal vitamins.

Home Medications:
- Labetalol 200 mg PO twice daily
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4 to 6 hours as needed for wheezing or shortness of breath
- Prenatal multivitamins 1 tablet daily

ALLERGIES:
She has no known allergies

FAMILY HISTORY:
- Mother with chronic hypertension
- Maternal aunt with cervical insufficiency and preterm delivery
- Paternal grandmother with asthma

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 10 years. Alcohol: None. Occupation: Office administrative assistant.

VITAL SIGNS:
On examination, temperature is 38.7°C (101.7°F), heart rate 112/min, and blood pressure 106/66 mm Hg

PHYSICAL EXAMINATION:
She appears ill and mildly uncomfortable. Abdominal exam reveals right costovertebral angle tenderness without rebound or guarding.

LABORATORY DATA:
Urinalysis reveals pyuria, bacteriuria, and positive nitrites.

IMAGING:
A renal ultrasound shows mild right-sided hydronephrosis.

OTHER STUDIES:
Fetal monitoring shows reactive tracings.

PLAN:
She is admitted for IV antibiotics and observation.

## Encounter 3: source 2021-07-15, practice 2023-12-23, ed, Emergency Medicine

Chief complaint: sudden onset of severe lower abdominal pain

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe lower abdominal pain

HISTORY OF PRESENT ILLNESS:
The patient returns to the emergency department with a history of cervical insufficiency and prior late spontaneous abortion, presenting with sudden onset of severe lower abdominal pain that started 6 hours ago. She reports that her last menstrual period was 6 weeks ago, but she has had irregular cycles in the past.

PAST MEDICAL HISTORY:
Active Problem List:
- Late spontaneous abortion (history) (diagnosed 2020-01-15)
- Cervical insufficiency (diagnosed 2020-07-15)
- Acute cystitis (diagnosed 2020-09-15)
- Asthma
- Chronic hypertension
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Cesarean delivery (age 26)
- Dilation and evacuation for second-trimester pregnancy loss (age 27)
- Laparoscopic right salpingectomy for ruptured ectopic pregnancy

MEDICATIONS:
Home Medications:
- Labetalol 200 mg PO twice daily
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4 to 6 hours as needed for wheezing or shortness of breath
- Prenatal multivitamins 1 tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with chronic hypertension
- Maternal aunt with cervical insufficiency and preterm delivery
- Paternal grandmother with asthma

SOCIAL HISTORY:
She is sexually active with one partner and does not use contraception.

VITAL SIGNS:
Her blood pressure is 90/60 mmHg, heart rate is 120 beats per minute, and temperature is 37.2°C (99.0°F).

PHYSICAL EXAMINATION:
On examination, she appears pale and anxious.
Abdominal examination reveals diffuse tenderness with rebound tenderness and guarding. A pelvic examination shows cervical motion tenderness.

LABORATORY DATA:
Her hemoglobin level is 9.2 g/dL, and her serum beta-hCG is 3500 mIU/mL.

IMAGING:
A transvaginal ultrasound reveals an empty uterus with no evidence of intrauterine pregnancy and a complex adnexal mass on the left side. There is also free fluid in the pelvis.

## Encounter 4: source 2022-07-15, practice 2024-12-31, ed, Emergency Medicine

Chief complaint: sudden onset of severe abdominal pain and vaginal bleeding for the past hour

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe abdominal pain and vaginal bleeding for the past hour

HISTORY OF PRESENT ILLNESS:
The patient is a 30-year-old female with a history of cervical insufficiency, late spontaneous abortion in 2020, and ruptured ectopic pregnancy in 2021 who presents to the emergency department with sudden onset of severe abdominal pain and vaginal bleeding beginning one hour ago. She reports no recent trauma or falls.

PAST MEDICAL HISTORY:
Active Problem List:
- Late spontaneous abortion (history) (diagnosed 2020-01-15)
- Cervical insufficiency (diagnosed 2020-07-15)
- Acute cystitis (diagnosed 2020-09-15)
- Ruptured ectopic pregnancy with hemodynamic instability (diagnosed 2021-07-15)
- Asthma
- Chronic hypertension
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Cesarean delivery (age 26)
- Dilation and evacuation for second-trimester pregnancy loss (age 27)
- Laparoscopic right salpingectomy for ruptured ectopic pregnancy

MEDICATIONS:
She takes labetalol.

Home Medications:
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4 to 6 hours as needed for wheezing or shortness of breath
- Prenatal multivitamins 1 tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with chronic hypertension
- Maternal aunt with cervical insufficiency and preterm delivery
- Paternal grandmother with asthma

SOCIAL HISTORY:
10-year history of smoking a pack of cigarettes per day

VITAL SIGNS:
Blood pressure is 155/100 mmHg, heart rate is 98 beats per minute, and temperature is 37.2°C (99.0°F)

PHYSICAL EXAMINATION:
The abdomen is tense and tender to palpation, especially in the uterine area, which is firm and irritable. Fetal heart tones are 140 beats per minute. Speculum examination reveals moderate dark red vaginal bleeding. The cervix is closed and there is no evidence of placental tissue in the cervical canal or vagina.

LABORATORY DATA:
Laboratory Values:
- Hemoglobin: 9.8 g/dL (Normal: 12-16 g/dL)
- Hematocrit: 29% (Normal: 36-46%)
- Platelets: 210,000/mm³ (Normal: 150,000-400,000/mm³)
- Prothrombin time (PT): 12 seconds (Normal: 11-13.5 seconds)
- Partial thromboplastin time (PTT): 30 seconds (Normal: 25-35 seconds)

## Encounter 5: source 2022-07-15, practice 2024-12-31, inpatient, Obstetrics & Gynecology

Chief complaint: presents to the labor and delivery unit in active labor

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Maria Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
She presents to the labor and delivery unit in active labor

HISTORY OF PRESENT ILLNESS:
The patient is a 30-year-old with cervical insufficiency, late spontaneous abortion, and ruptured ectopic pregnancy presenting with abdominal pain and vaginal bleeding. Her prenatal course is unremarkable. She delivers a healthy male infant vaginally with the assistance of a vacuum extractor due to prolonged second stage. Placenta is delivered intact, but shortly after, she begins to experience profuse vaginal bleeding.

PAST MEDICAL HISTORY:
Active Problem List:
- Late spontaneous abortion (history) (diagnosed 2020-01-15)
- Cervical insufficiency (diagnosed 2020-07-15)
- Acute cystitis (diagnosed 2020-09-15)
- Ruptured ectopic pregnancy with hemodynamic instability (diagnosed 2021-07-15)
- Placental abruption (Abruptio placentae) (diagnosed 2022-07-15)
- Asthma
- Chronic hypertension
- Tobacco use disorder

She has a history of one previous cesarean delivery.

PAST SURGICAL HISTORY:
- Cesarean delivery (age 26)
- Dilation and evacuation for second-trimester pregnancy loss (age 27)
- Laparoscopic right salpingectomy for ruptured ectopic pregnancy

MEDICATIONS:
Home Medications:
- Labetalol 200 mg PO twice daily
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4 to 6 hours as needed for wheezing or shortness of breath
- Prenatal multivitamins 1 tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with chronic hypertension
- Maternal aunt with cervical insufficiency and preterm delivery
- Paternal grandmother with asthma

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 10 years. Alcohol: None. Occupation: Office administrative assistant.

VITAL SIGNS:
Her blood pressure is 90/60 mmHg, heart rate is 110 beats per minute,

PHYSICAL EXAMINATION:
On examination, her uterus is boggy and enlarged.
and she appears pale and anxious.

PLAN:
Bimanual uterine massage is initiated, and intravenous access is established.

## Encounter 6: source 2023-07-15, practice 2025-12-28, ed, Emergency Medicine

Chief complaint: severe abdominal pain and dizziness after high-speed motor vehicle collision

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. David Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe abdominal pain and dizziness after high-speed motor vehicle collision

HISTORY OF PRESENT ILLNESS:
The patient with cervical insufficiency and prior obstetric hemorrhage is brought to the emergency department after being involved in a high-speed motor vehicle collision. She was the restrained driver, and the airbags deployed.

PAST MEDICAL HISTORY:
Active Problem List:
- Late spontaneous abortion (history) (diagnosed 2020-01-15)
- Cervical insufficiency (diagnosed 2020-07-15)
- Acute cystitis (diagnosed 2020-09-15)
- Ruptured ectopic pregnancy with hemodynamic instability (diagnosed 2021-07-15)
- Placental abruption (Abruptio placentae) (diagnosed 2022-07-15)
- Postpartum hemorrhage (diagnosed 2022-07-15)
- Asthma
- Chronic hypertension
- Tobacco use disorder

She has had an uncomplicated pregnancy thus far.

PAST SURGICAL HISTORY:
- Cesarean delivery (age 26)
- Dilation and evacuation for second-trimester pregnancy loss (age 27)
- Laparoscopic right salpingectomy for ruptured ectopic pregnancy

MEDICATIONS:
Home Medications:
- Labetalol 200 mg PO twice daily
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4 to 6 hours as needed for wheezing or shortness of breath
- Prenatal multivitamins 1 tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with chronic hypertension
- Maternal aunt with cervical insufficiency and preterm delivery
- Paternal grandmother with asthma

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 10 years. Alcohol: None. Occupation: Office administrative assistant.

VITAL SIGNS:
On examination, her blood pressure is 85/50 mmHg, heart rate is 120 beats per minute, and respiratory rate is 22 beats per minute. The patient's blood pressure drops to 75/45 mmHg despite initial crystalloid fluid resuscitation.

PHYSICAL EXAMINATION:
Her skin is cool and clammy. Abdominal examination reveals diffuse tenderness, uterine contractions, and a small amount of vaginal bleeding. Fetal heart tones are 140 beats per minute.

LABORATORY DATA:
The initial laboratory results are as follows:

- Hemoglobin: 9 g/dL (Normal: 12-16 g/dL)
- Hematocrit: 27% (Normal: 37-47%)
- Platelets: 250,000/mm³ (Normal: 150,000-400,000/mm³)
- Prothrombin time (PT): 14 seconds (Normal: 11-13.5 seconds)
- Partial thromboplastin time (PTT): 35 seconds (Normal: 25-35 seconds)

IMAGING:
A focused assessment with sonography for trauma (FAST) exam shows free fluid in the abdomen.

## Encounter 7: source 2023-07-15, practice 2025-12-28, icu, Pulmonary & Critical Care

Chief complaint: severe respiratory distress following motor vehicle collision

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Emily Zhang, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Severe respiratory distress following motor vehicle collision

HISTORY OF PRESENT ILLNESS:
This 31-year-old with asthma, chronic hypertension, and cervical insufficiency with prior adverse obstetric outcomes is transferred for critical care. She was brought to the emergency department with severe respiratory distress following a motor vehicle collision 4 hours ago. She was intubated at the scene due to hypoxemia and altered mental status.

PAST MEDICAL HISTORY:
Active Problem List:
- Late spontaneous abortion (history) (diagnosed 2020-01-15)
- Cervical insufficiency (diagnosed 2020-07-15)
- Acute cystitis (diagnosed 2020-09-15)
- Ruptured ectopic pregnancy with hemodynamic instability (diagnosed 2021-07-15)
- Placental abruption (Abruptio placentae) (diagnosed 2022-07-15)
- Postpartum hemorrhage (diagnosed 2022-07-15)
- Traumatic hemorrhagic shock with acute coagulopathy (diagnosed 2023-07-15)
- Asthma
- Chronic hypertension
- Tobacco use disorder

Her medical history is significant for asthma.

PAST SURGICAL HISTORY:
- Cesarean delivery (age 26)
- Dilation and evacuation for second-trimester pregnancy loss (age 27)
- Laparoscopic right salpingectomy for ruptured ectopic pregnancy

MEDICATIONS:
She takes for which she uses an albuterol inhaler as needed.

Home Medications:
- Labetalol 200 mg PO twice daily
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4 to 6 hours as needed for wheezing or shortness of breath
- Prenatal multivitamins 1 tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with chronic hypertension
- Maternal aunt with cervical insufficiency and preterm delivery
- Paternal grandmother with asthma

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 10 years. Alcohol: None. Occupation: Office administrative assistant.

VITAL SIGNS:
On examination, her blood pressure is 95/60 mmHg, heart rate is 122 beats/min, respiratory rate is 28 breaths/min on mechanical ventilation, and oxygen saturation is 88% on FiO2 of 100%

PHYSICAL EXAMINATION:
Auscultation of the lungs reveals diffuse bilateral crackles.

LABORATORY DATA:
Arterial blood gas on 100% oxygen shows:

- pH: 7.32
- PaCO2: 48 mmHg
- PaO2: 58 mmHg
- HCO3-: 24 mEq/L
- Lactate: 3.2 mmol/L

IMAGING:
Chest X-ray shows bilateral alveolar infiltrates.
