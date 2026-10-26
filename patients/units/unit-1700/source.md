# synthetic_hospital patient 1700

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, Thai, Private health insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-04-01 (Sat) | ed | Emergency Medicine |
| 1 | 2022-07-15 | 2025-10-10 (Fri) | outpatient | Obstetrics & Gynecology |
| 2 | 2022-09-15 | 2025-12-03 (Wed) | outpatient | Obstetrics & Gynecology |
| 3 | 2022-11-15 | 2026-02-02 (Mon) | outpatient | Obstetrics & Gynecology |
| 4 | 2023-01-15 | 2026-03-26 (Thu) | ed | Emergency Medicine |
| 5 | 2023-02-15 | 2026-04-30 (Thu) | ed | Emergency Medicine |
| 6 | 2023-03-15 | 2026-05-28 (Thu) | ed | Obstetrics & Gynecology |
| 7 | 2023-04-15 | 2026-06-23 (Tue) | inpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Private health insurance",
 "occupation": "Kindergarten teacher",
 "alcohol_use": "None",
 "family_history": [
  "Alpha-thalassemia trait in first-degree relatives",
  "Rheumatic heart disease in mother",
  "Uncomplicated pregnancies in mother"
 ],
 "race_ethnicity": "Thai",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 tablet daily",
   "name": "Prenatal vitamin"
  },
  {
   "dose": "1 mg daily",
   "name": "Folic acid"
  }
 ],
 "surgical_history": [
  "Vaginal delivery"
 ],
 "chronic_conditions": [
  "Alpha-thalassemia trait",
  "Rheumatic mitral stenosis"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Decreased fetal movements in third trimester", "Meralgia paresthetica", "Fetal distress (Category III fetal heart rate tracing)", "Rheumatic mitral stenosis", "Labor arrest disorder", "Deep vein thrombosis of left lower extremity in pregnancy", "Uncomplicated pregnancy without urinary tract infection", "Alpha-thalassemia"]
Comorbidities: ["Pregnancy", "Weight gain in pregnancy", "Multiparity"]

## Encounter 0: source 2020-01-15, practice 2023-04-01, ed, Emergency Medicine

Chief complaint: Progressive dyspnea, orthopnea, and palpitations over 2 days

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive dyspnea, orthopnea, and palpitations over 2 days

PAST MEDICAL HISTORY:
Active Problem List:
- Alpha-thalassemia trait
- Rheumatic mitral stenosis

PAST SURGICAL HISTORY:
- Vaginal delivery

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily
- Folic acid 1 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Alpha-thalassemia trait in first-degree relatives
- Rheumatic heart disease in mother
- Uncomplicated pregnancies in mother

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Kindergarten teacher.

VITAL SIGNS:
On examination, her blood pressure is 110/70 mmHg, heart rate is irregularly irregular at 120 bpm, respiratory rate is 22 breaths/min, and oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
Auscultation of the heart reveals a diastolic rumbling murmur best heard at the apex with an opening snap. Lung examination shows bibasilar crackles.

IMAGING:
Chest X-ray demonstrates pulmonary venous congestion and an enlarged left atrium.

OTHER STUDIES:
An electrocardiogram shows atrial fibrillation with no ST changes or T wave abnormalities.

## Encounter 1: source 2022-07-15, practice 2025-10-10, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit at 20 weeks gestation with questions about influenza vaccination

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Maria Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit at 20 weeks gestation with questions about influenza vaccination

HISTORY OF PRESENT ILLNESS:
The patient is a 30-year-old female with alpha-thalassemia trait and rheumatic mitral stenosis initially diagnosed in January 2020 following an emergency department presentation with progressive dyspnea, orthopnea, and palpitations, who returns for routine prenatal care. She is currently at 20 weeks' gestation in her first pregnancy. Her prenatal course has been uncomplicated, and she reports no symptoms today. She recalls receiving the flu shot a couple of years ago but has not yet been vaccinated this year. She expresses concern about receiving vaccines during pregnancy and asks if she should wait until after delivery.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatic mitral stenosis (diagnosed 2020-01-15)
- Alpha-thalassemia trait

Her vaccinations are up to date, including Tdap and MMR prior to pregnancy.

PAST SURGICAL HISTORY:
- Vaginal delivery

MEDICATIONS:
She takes only a prenatal vitamin.

Home Medications:
- Folic acid 1 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Alpha-thalassemia trait in first-degree relatives
- Rheumatic heart disease in mother
- Uncomplicated pregnancies in mother

SOCIAL HISTORY:
She works as a kindergarten teacher and is concerned about being exposed to illnesses from her students during the winter season.

## Encounter 2: source 2022-09-15, practice 2025-12-03, outpatient, Obstetrics & Gynecology

Chief complaint: Routine obstetric evaluation at 28 weeks gestation

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. Maria Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine obstetric evaluation at 28 weeks gestation

HISTORY OF PRESENT ILLNESS:
Patient with alpha-thalassemia trait and rheumatic mitral stenosis returns for prenatal visit at 28 weeks with no significant complaints during the pregnancy and denies recent illnesses or trauma. There is no evidence of maternal diabetes or alloimmunization.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatic mitral stenosis (diagnosed 2020-01-15)
- Uncomplicated pregnancy without urinary tract infection (diagnosed 2022-07-15)
- Alpha-thalassemia trait

Previous pregnancies were uncomplicated.

PAST SURGICAL HISTORY:
- Vaginal delivery

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily
- Folic acid 1 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
There is no family history of genetic conditions

SOCIAL HISTORY:
The patient and her partner are both originally from Thailand.

LABORATORY DATA:
Routine serologies for infections including Toxoplasma gondii, cytomegalovirus, and parvovirus B19 are negative. Noninvasive prenatal testing did not show any chromosomal abnormalities.

IMAGING:
Ultrasound reveals a fetus with significant abdominal fluid accumulation, bilateral fluid collections in the chest cavity, and a small amount of fluid around the heart. Amniotic fluid volume appears normal, and fetal growth parameters are within the expected range.

PLAN:
The neonatology team is consulted, and maternal peripheral smear is sent for evaluation.

## Encounter 3: source 2022-11-15, practice 2026-02-02, outpatient, Obstetrics & Gynecology

Chief complaint: Burning pain and tingling over the lateral right thigh for 2 months

ENCOUNTER NOTE
Date: 2022-11-15
Provider: Dr. Priya Patel, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Burning pain and tingling over the lateral right thigh for 2 months

HISTORY OF PRESENT ILLNESS:
The patient is a 30-year-old female with alpha-thalassemia trait and rheumatic mitral stenosis who returns to the obstetrics clinic for continued prenatal care following her routine evaluation at 28 weeks gestation, presenting with a 2-month history of burning pain and tingling over the lateral aspect of her right thigh. She denies any recent trauma or injury to the area. She mentions that the symptoms are exacerbated by standing for long periods and are relieved when lying down. She has gained 25 pounds during her pregnancy.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatic mitral stenosis (diagnosed 2020-01-15)
- Uncomplicated pregnancy without urinary tract infection (diagnosed 2022-07-15)
- Alpha-thalassemia (diagnosed 2022-09-15)
- Alpha-thalassemia trait

PAST SURGICAL HISTORY:
- Vaginal delivery

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily
- Folic acid 1 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Alpha-thalassemia trait in first-degree relatives
- Rheumatic heart disease in mother
- Uncomplicated pregnancies in mother

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Kindergarten teacher.

PHYSICAL EXAMINATION:
On physical examination, there is no erythema or edema, but there is a distinct area of decreased sensation over the anterolateral aspect of the thigh. Hip and spine examinations are normal. Deep tendon reflexes are intact, and straight leg raise test is negative bilaterally.

## Encounter 4: source 2023-01-15, practice 2026-03-26, ed, Emergency Medicine

Chief complaint: Progressive swelling and discomfort in left leg over 3 days

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive swelling and discomfort in left leg over 3 days

HISTORY OF PRESENT ILLNESS:
This 31-year-old with alpha-thalassemia trait and rheumatic mitral stenosis presents during pregnancy with left leg symptoms. She reports difficulty walking due to tightness and aching pain that worsens with prolonged standing. She has no history of trauma or recent travel.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatic mitral stenosis (diagnosed 2020-01-15)
- Uncomplicated pregnancy without urinary tract infection (diagnosed 2022-07-15)
- Alpha-thalassemia (diagnosed 2022-09-15)
- Meralgia paresthetica (diagnosed 2022-11-15)
- Alpha-thalassemia trait

Her pregnancy has been uncomplicated thus far, and she has attended all scheduled prenatal visits.

PAST SURGICAL HISTORY:
- Vaginal delivery

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily
- Folic acid 1 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Alpha-thalassemia trait in first-degree relatives
- Rheumatic heart disease in mother
- Uncomplicated pregnancies in mother

SOCIAL HISTORY:
She does not use tobacco, alcohol, or drugs.

VITAL SIGNS:
On examination, she is afebrile with stable vital signs.

PHYSICAL EXAMINATION:
Her left calf is visibly swollen and tender to palpation compared to the right. There is mild erythema but no skin breakdown or ulceration. Peripheral pulses are intact bilaterally.

IMAGING:
Compression ultrasonography reveals a noncompressible left popliteal vein consistent with deep vein thrombosis.

## Encounter 5: source 2023-02-15, practice 2026-04-30, ed, Emergency Medicine

Chief complaint: Decreased fetal movements over the past 24 hours

ENCOUNTER NOTE
Date: 2023-02-15
Provider: Dr. Aisha Washington, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Decreased fetal movements over the past 24 hours

HISTORY OF PRESENT ILLNESS:
The 31-year-old with rheumatic mitral stenosis and alpha-thalassemia trait presents in her third trimester with decreased fetal movements, one month after DVT. She reports that the baby used to be very active, especially after meals, but she has felt only a few movements since yesterday morning. She denies any vaginal bleeding, leakage of fluid, or contractions. The patient is visibly concerned and asks about the health of her baby.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatic mitral stenosis (diagnosed 2020-01-15)
- Uncomplicated pregnancy without urinary tract infection (diagnosed 2022-07-15)
- Alpha-thalassemia (diagnosed 2022-09-15)
- Meralgia paresthetica (diagnosed 2022-11-15)
- Deep vein thrombosis of left lower extremity in pregnancy (diagnosed 2023-01-15)
- Alpha-thalassemia trait

Her pregnancy has been uncomplicated.

PAST SURGICAL HISTORY:
- Vaginal delivery

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily
- Folic acid 1 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Alpha-thalassemia trait in first-degree relatives
- Rheumatic heart disease in mother
- Uncomplicated pregnancies in mother

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Kindergarten teacher.

VITAL SIGNS:
Blood pressure is 120/78 mmHg, heart rate is 88 bpm

PHYSICAL EXAMINATION:
On examination, fetal heart rate auscultated by Doppler is 140 bpm. There is no abdominal tenderness, and the uterus is appropriately sized for gestational age.

LABORATORY DATA:
Her prenatal labs have been unremarkable, and her glucose challenge test was normal.

## Encounter 6: source 2023-03-15, practice 2026-05-28, ed, Obstetrics & Gynecology

Chief complaint: Decreased fetal movements over 6 hours

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. David Kim, MD
Department: Obstetrics & Gynecology
Visit Type: ed

CHIEF COMPLAINT:
Decreased fetal movements over 6 hours

HISTORY OF PRESENT ILLNESS:
31-year-old female with alpha-thalassemia trait and rheumatic mitral stenosis returns to the emergency department with a 6-hour history of recurrent decreased fetal movements. She has had an uneventful pregnancy until now, with regular prenatal care.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatic mitral stenosis (diagnosed 2020-01-15)
- Uncomplicated pregnancy without urinary tract infection (diagnosed 2022-07-15)
- Alpha-thalassemia (diagnosed 2022-09-15)
- Meralgia paresthetica (diagnosed 2022-11-15)
- Deep vein thrombosis of left lower extremity in pregnancy (diagnosed 2023-01-15)
- Decreased fetal movements in third trimester (diagnosed 2023-02-15)
- Alpha-thalassemia trait

Her first child was delivered vaginally at term without complications.

PAST SURGICAL HISTORY:
- Vaginal delivery

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily
- Folic acid 1 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Alpha-thalassemia trait in first-degree relatives
- Rheumatic heart disease in mother
- Uncomplicated pregnancies in mother

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Kindergarten teacher.

VITAL SIGNS:
Blood pressure is 120/75 mmHg, pulse is 80 beats per minute, and temperature is 36.8°C (98.2°F)

PHYSICAL EXAMINATION:
Fetal heart rate monitoring shows minimal variability with no accelerations and occasional late decelerations.

IMAGING:
An ultrasound is performed, which reveals oligohydramnios.

OTHER STUDIES:
A biophysical profile is conducted, yielding a score of 4/10 (non-stress test: 0/2, fetal breathing movements: 0/2, fetal movements: 2/2, fetal tone: 2/2, amniotic fluid volume: 0/2).

## Encounter 7: source 2023-04-15, practice 2026-06-23, inpatient, Obstetrics & Gynecology

Chief complaint: Regular contractions every 3 minutes and labor arrest after water broke

ENCOUNTER NOTE
Date: 2023-04-15
Provider: Dr. Robert Johnson, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Regular contractions every 3 minutes and labor arrest after water broke

HISTORY OF PRESENT ILLNESS:
Patient with rheumatic mitral stenosis and alpha-thalassemia trait returns in labor following emergency department visits. She reports that her water broke 2 hours ago. Her prenatal course was unremarkable. After 5 hours of adequate contractions, the patient is now experiencing intense pain and requests pain relief.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatic mitral stenosis (diagnosed 2020-01-15)
- Uncomplicated pregnancy without urinary tract infection (diagnosed 2022-07-15)
- Alpha-thalassemia (diagnosed 2022-09-15)
- Meralgia paresthetica (diagnosed 2022-11-15)
- Deep vein thrombosis of left lower extremity in pregnancy (diagnosed 2023-01-15)
- Decreased fetal movements in third trimester (diagnosed 2023-02-15)
- Fetal distress (Category III fetal heart rate tracing) (diagnosed 2023-03-15)
- Alpha-thalassemia trait

PAST SURGICAL HISTORY:
- Vaginal delivery

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily
- Folic acid 1 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Alpha-thalassemia trait in first-degree relatives
- Rheumatic heart disease in mother
- Uncomplicated pregnancies in mother

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Kindergarten teacher.

VITAL SIGNS:
Her vital signs are stable

PHYSICAL EXAMINATION:
On admission, her cervix is dilated to 6 cm, 90% effaced, and the fetus is at a -2 station. The fetal heart tracing is reassuring. a repeat cervical exam shows no change in dilation, effacement, or station. she has no signs of infection

OTHER STUDIES:
Intrauterine pressure catheter showing an average Montevideo unit of 220 over 10 minutes
