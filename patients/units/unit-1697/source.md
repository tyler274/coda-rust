# synthetic_hospital patient 1697

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 27 at first encounter, sex F, Non-Hispanic White, Private PPO through employer.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-09-10 (Sun) | inpatient | Obstetrics & Gynecology |
| 1 | 2020-01-15 | 2023-09-10 (Sun) | inpatient | Neonatology |
| 2 | 2020-02-15 | 2023-10-03 (Tue) | icu | Neonatology |
| 3 | 2022-09-15 | 2026-05-11 (Mon) | ed | Emergency Medicine |
| 4 | 2022-09-15 | 2026-05-11 (Mon) | ed | Emergency Medicine |
| 5 | 2022-10-15 | 2026-05-30 (Sat) | ed | Emergency Medicine |
| 6 | 2022-10-15 | 2026-06-09 (Tue) | procedure | Obstetrics & Gynecology |
| 7 | 2022-12-15 | 2026-08-03 (Mon) | outpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Private PPO through employer",
 "occupation": "Human resources coordinator",
 "alcohol_use": "Occasional social use (1-2 drinks per month)",
 "family_history": [
  "Maternal history of preeclampsia",
  "Maternal aunt with history of gestational trophoblastic disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 tablet PO daily",
   "name": "Ethinyl estradiol/norgestimate"
  },
  {
   "dose": "325 mg PO daily",
   "name": "Ferrous sulfate"
  }
 ],
 "surgical_history": [
  "Low transverse cesarean section (age 25)",
  "Suction dilation and curettage (age 27)"
 ],
 "chronic_conditions": [
  "History of preterm birth (32 weeks gestation)"
 ],
 "age_at_first_encounter": 27
}
```

Primary diagnoses: ["Complete hydatidiform mole", "Anemia of prematurity", "Intraventricular hemorrhage", "Acute fetal distress requiring emergent delivery"]
Comorbidities: ["Severe preeclampsia", "Multiparity"]

## Encounter 0: source 2020-01-15, practice 2023-09-10, inpatient, Obstetrics & Gynecology

Chief complaint: Sudden gush of clear fluid from vagina with irregular contractions at 32 weeks gestation

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Johnson, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Sudden gush of clear fluid from vagina with irregular contractions at 32 weeks gestation

HISTORY OF PRESENT ILLNESS:
She denies any vaginal bleeding or significant pain, but notes irregular contractions about 10 minutes apart. Her pregnancy has been uncomplicated, and prenatal care has been adequate with no evidence of maternal or fetal complications.

PAST MEDICAL HISTORY:
Active Problem List:
- History of preterm birth (32 weeks gestation)

PAST SURGICAL HISTORY:
- Low transverse cesarean section (age 25)
- Suction dilation and curettage (age 27)

MEDICATIONS:
Home Medications:
- Ethinyl estradiol/norgestimate 1 tablet PO daily
- Ferrous sulfate 325 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal history of preeclampsia
- Maternal aunt with history of gestational trophoblastic disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per month). Occupation: Human resources coordinator.

VITAL SIGNS:
On examination, her vital signs are stable

PHYSICAL EXAMINATION:
Speculum examination confirms the rupture of membranes. The patient's cervix is dilated to 3 cm, and the fetus is in a cephalic presentation.

OTHER STUDIES:
Continuous fetal heart rate monitoring is initiated, and the tracing shows abrupt decreases in the fetal heart rate to 90 bpm from a baseline of 140 bpm with a rapid return to baseline. These decelerations vary in timing with respect to contractions.

## Encounter 1: source 2020-01-15, practice 2023-09-10, inpatient, Neonatology

Chief complaint: Infant episodes of cyanosis lasting 20 seconds during sleep

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Michael Chen, MD
Department: Neonatology
Visit Type: inpatient

CHIEF COMPLAINT:
Infant episodes of cyanosis lasting 20 seconds during sleep

HISTORY OF PRESENT ILLNESS:
The patient is status post emergent delivery at 32 weeks gestation for acute fetal distress. The infant presents on day 2 of life with episodes observed by nursing staff of cyanosis lasting 20 seconds, during which the infant appears limp, followed by spontaneous recovery. The episodes occur predominantly during sleep and are not associated with feeding.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute fetal distress requiring emergent delivery (diagnosed 2020-01-15)
- History of preterm birth (32 weeks gestation)

PAST SURGICAL HISTORY:
- Low transverse cesarean section (age 25)
- Suction dilation and curettage (age 27)

MEDICATIONS:
Home Medications:
- Ethinyl estradiol/norgestimate 1 tablet PO daily
- Ferrous sulfate 325 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal history of preeclampsia
- Maternal aunt with history of gestational trophoblastic disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per month). Occupation: Human resources coordinator.

VITAL SIGNS:
The infant's temperature is 36.8°C, heart rate is 145 beats per minute, respiratory rate is 40 breaths per minute, and oxygen saturation is 98% on room air between episodes.

PHYSICAL EXAMINATION:
Physical examination reveals a well-appearing preterm infant with clear lungs and a normal heart examination. No other abnormalities are noted.

## Encounter 2: source 2020-02-15, practice 2023-10-03, icu, Neonatology

Chief complaint: Neonate with apnea and bradycardia episodes while on CPAP

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. Michael Chen, MD
Department: Neonatology
Visit Type: icu

CHIEF COMPLAINT:
Neonate with apnea and bradycardia episodes while on CPAP

HISTORY OF PRESENT ILLNESS:
The neonate presents following emergent 32-week delivery and prior evaluation for cyanotic episodes. The neonate had an Apgar score of 8 and 9 at 1 and 5 minutes, respectively. He is currently on continuous positive airway pressure (CPAP) for respiratory distress syndrome.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute fetal distress requiring emergent delivery (diagnosed 2020-01-15)
- Anemia of prematurity (diagnosed 2020-01-15)
- History of preterm birth (32 weeks gestation)

PAST SURGICAL HISTORY:
- Low transverse cesarean section (age 25)
- Suction dilation and curettage (age 27)

MEDICATIONS:
Home Medications:
- Ethinyl estradiol/norgestimate 1 tablet PO daily
- Ferrous sulfate 325 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal history of preeclampsia
- Maternal aunt with history of gestational trophoblastic disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per month). Occupation: Human resources coordinator.

PHYSICAL EXAMINATION:
The physical examination is significant for a soft, sunken anterior fontanelle, and the rest of the examination is unremarkable. The neonate has not had any seizures, and the neurological examination is non-focal.

## Encounter 3: source 2022-09-15, practice 2026-05-11, ed, Emergency Medicine

Chief complaint: Intermittent vaginal bleeding and pelvic pressure for two weeks in current pregnancy

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. Emily Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Intermittent vaginal bleeding and pelvic pressure for two weeks in current pregnancy

HISTORY OF PRESENT ILLNESS:
This 29-year-old female with a history of preterm delivery at 32 weeks gestation complicated by neonatal anemia of prematurity and intraventricular hemorrhage presents to the emergency department with complaints of intermittent vaginal bleeding and pelvic pressure for the past two weeks. She mentions that her current pregnancy has been different from her first, with more pronounced nausea and vomiting. She denies any fever, chills, or pain, and she has had no prenatal care for this pregnancy.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute fetal distress requiring emergent delivery (diagnosed 2020-01-15)
- Anemia of prematurity (diagnosed 2020-01-15)
- Intraventricular hemorrhage (diagnosed 2020-02-15)
- History of preterm birth (32 weeks gestation)

PAST SURGICAL HISTORY:
- Low transverse cesarean section (age 25)
- Suction dilation and curettage (age 27)

MEDICATIONS:
Home Medications:
- Ethinyl estradiol/norgestimate 1 tablet PO daily
- Ferrous sulfate 325 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal history of preeclampsia
- Maternal aunt with history of gestational trophoblastic disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per month). Occupation: Human resources coordinator.

VITAL SIGNS:
Blood pressure is 130/85 mmHg, heart rate is 88 beats per minute

PHYSICAL EXAMINATION:
On physical examination, her blood pressure is 130/85 mmHg, heart rate is 88 beats per minute, and her abdomen is soft, non-tender, with a fundal height that appears larger than expected for her gestational age.

LABORATORY DATA:
Her laboratory results are as follows:

- Human chorionic gonadotropin (hCG): 200,000 mIU/mL (expected range for gestational age: 8,000-58,000 mIU/mL)
- Hemoglobin: 12.2 g/dL
- White blood cell count: 7,000/uL
- Platelets: 250,000/uL

IMAGING:
A pelvic ultrasound is performed, revealing an enlarged uterus with a heterogeneous mass composed of numerous cystic spaces without evidence of a developing fetus. Bilateral enlarged ovaries with multiple cysts are also noted.

## Encounter 4: source 2022-09-15, practice 2026-05-11, ed, Emergency Medicine

Chief complaint: Increasing vaginal bleeding and lower abdominal pain for two days

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. James Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increasing vaginal bleeding and lower abdominal pain for two days

HISTORY OF PRESENT ILLNESS:
With 32-week preterm delivery history, she returns today reporting bleeding that is painless and has been increasing in volume.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute fetal distress requiring emergent delivery (diagnosed 2020-01-15)
- Anemia of prematurity (diagnosed 2020-01-15)
- Intraventricular hemorrhage (diagnosed 2020-02-15)
- Complete hydatidiform mole (diagnosed 2022-09-15)
- History of preterm birth (32 weeks gestation)

Her previous pregnancy was uneventful, and she delivered a healthy baby at term.

PAST SURGICAL HISTORY:
- Low transverse cesarean section (age 25)
- Suction dilation and curettage (age 27)

MEDICATIONS:
Home Medications:
- Ethinyl estradiol/norgestimate 1 tablet PO daily
- Ferrous sulfate 325 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal history of preeclampsia
- Maternal aunt with history of gestational trophoblastic disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per month). Occupation: Human resources coordinator.

VITAL SIGNS:
On examination, her blood pressure is 130/85 mmHg, heart rate is 88 beats per minute

PHYSICAL EXAMINATION:
Abdominal examination reveals a uterus larger than expected for gestational age.

LABORATORY DATA:
Laboratory results are as follows:

- β-hCG: 250,000 mIU/mL (normal for gestational age: 1,080 – 56,500 mIU/mL)
- Hemoglobin: 11.2 g/dL
- White blood cell count: 9,000 /μL
- Platelets: 250,000 /μL

IMAGING:
Pelvic ultrasound shows a snowstorm pattern without evidence of a fetus and bilateral enlarged multicystic ovaries.

ASSESSMENT AND PLAN:
Based on the clinical presentation and test results,

## Encounter 5: source 2022-10-15, practice 2026-05-30, ed, Emergency Medicine

Chief complaint: Severe headaches, visual disturbances, and generalized edema

ENCOUNTER NOTE
Date: 2022-10-15
Provider: Dr. James Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe headaches, visual disturbances, and generalized edema

HISTORY OF PRESENT ILLNESS:
This 29-year-old female with history of preterm birth returns to the emergency department one month after prior visits for vaginal bleeding. She presents with a 2-day history of severe headaches, visual disturbances, and generalized edema.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute fetal distress requiring emergent delivery (diagnosed 2020-01-15)
- Anemia of prematurity (diagnosed 2020-01-15)
- Intraventricular hemorrhage (diagnosed 2020-02-15)
- Complete hydatidiform mole (diagnosed 2022-09-15)
- History of preterm birth (32 weeks gestation)

Her previous pregnancy was uncomplicated.

PAST SURGICAL HISTORY:
- Low transverse cesarean section (age 25)
- Suction dilation and curettage (age 27)

MEDICATIONS:
Home Medications:
- Ethinyl estradiol/norgestimate 1 tablet PO daily
- Ferrous sulfate 325 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal history of preeclampsia
- Maternal aunt with history of gestational trophoblastic disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per month). Occupation: Human resources coordinator.

VITAL SIGNS:
Her blood pressure is 165/105 mmHg.

PHYSICAL EXAMINATION:
Physical examination reveals bilateral pitting edema in the lower extremities and brisk deep tendon reflexes.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 13.2 g/dL
- Platelet count: 210,000/mm³
- Serum creatinine: 0.9 mg/dL
- Aspartate aminotransferase (AST): 30 U/L
- Alanine aminotransferase (ALT): 28 U/L
- Urine protein/creatinine ratio: 0.3 (normal <0.2)
- Beta-hCG: 200,000 mIU/mL (expected range for gestational age: 1,080 – 56,500 mIU/mL)

IMAGING:
An ultrasound of the pelvis shows a "snowstorm" pattern with no evidence of a developing fetus.

## Encounter 6: source 2022-10-15, practice 2026-06-09, procedure, Obstetrics & Gynecology

Chief complaint: Pelvic ultrasound and surgical management of suspected molar pregnancy

ENCOUNTER NOTE
Date: 2022-10-15
Provider: Dr. Sarah Johnson, MD
Department: Obstetrics & Gynecology
Visit Type: procedure

CHIEF COMPLAINT:
Pelvic ultrasound and surgical management of suspected molar pregnancy

PAST MEDICAL HISTORY:
Active Problem List:
- Acute fetal distress requiring emergent delivery (diagnosed 2020-01-15)
- Anemia of prematurity (diagnosed 2020-01-15)
- Intraventricular hemorrhage (diagnosed 2020-02-15)
- Complete hydatidiform mole (diagnosed 2022-09-15)
- History of preterm birth (32 weeks gestation)

PAST SURGICAL HISTORY:
- Low transverse cesarean section (age 25)
- Suction dilation and curettage (age 27)

MEDICATIONS:
Home Medications:
- Ethinyl estradiol/norgestimate 1 tablet PO daily
- Ferrous sulfate 325 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal history of preeclampsia
- Maternal aunt with history of gestational trophoblastic disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per month). Occupation: Human resources coordinator.

## Encounter 7: source 2022-12-15, practice 2026-08-03, outpatient, Obstetrics & Gynecology

Chief complaint: Follow-up visit after suction curettage for complete hydatidiform mole

ENCOUNTER NOTE
Date: 2022-12-15
Provider: Dr. Lisa Wong, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit after suction curettage for complete hydatidiform mole

HISTORY OF PRESENT ILLNESS:
This 29-year-old female with a history of preterm birth at 32 weeks gestation returns for follow-up. She underwent suction curettage and has been followed with serial β-hCG levels. Her initial β-hCG level was 200,000 mIU/mL. Today, she reports adherence to the recommended contraception and denies any symptoms of vaginal bleeding or pelvic pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute fetal distress requiring emergent delivery (diagnosed 2020-01-15)
- Anemia of prematurity (diagnosed 2020-01-15)
- Intraventricular hemorrhage (diagnosed 2020-02-15)
- Complete hydatidiform mole (diagnosed 2022-09-15)
- History of preterm birth (32 weeks gestation)

PAST SURGICAL HISTORY:
- Low transverse cesarean section (age 25)
- Suction dilation and curettage (age 27)

MEDICATIONS:
Home Medications:
- Ethinyl estradiol/norgestimate 1 tablet PO daily
- Ferrous sulfate 325 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal history of preeclampsia
- Maternal aunt with history of gestational trophoblastic disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per month). Occupation: Human resources coordinator.

PHYSICAL EXAMINATION:
Her physical examination is unremarkable.

LABORATORY DATA:
Her serial β-hCG levels are as follows:

- 1 month post-curettage: 5,000 mIU/mL
- 6 weeks post-curettage: 1,200 mIU/mL
- 8 weeks post-curettage: 800 mIU/mL
