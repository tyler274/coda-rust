# synthetic_hospital patient 1813

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-10-20 (Mon) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-06-15 | 2026-03-27 (Fri) | outpatient | Obstetrics & Gynecology |
| 2 | 2020-08-15 | 2026-05-28 (Thu) | outpatient | Obstetrics & Gynecology |
| 3 | 2020-09-15 | 2026-06-27 (Sat) | ed | Emergency Medicine |
| 4 | 2020-09-15 | 2026-06-27 (Sat) | ed | Emergency Medicine |
| 5 | 2020-10-15 | 2026-07-27 (Mon) | inpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "None",
 "family_history": [
  "Maternal history of third-trimester bleeding disorders",
  "Paternal history of VACTERL association features (congenital vertebral anomalies)",
  "No history of heritable thrombophilia"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 tablet daily",
   "name": "Prenatal vitamin with iron"
  },
  {
   "dose": "400 mcg daily",
   "name": "Folic acid"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Primigravid pregnancy",
  "Retained copper intrauterine device in situ",
  "Placenta previa",
  "Incomplete adult immunization (susceptible to diphtheria, tetanus, and pertussis)",
  "Pregnancy complicated by fetal tracheoesophageal fistula with esophageal atresia"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Decreased fetal movements", "Placental Abruption", "Copper-containing intrauterine device", "Tracheoesophageal fistula with esophageal atresia", "Diphtheria", "Vasa previa"]
Comorbidities: ["Pregnancy", "Pregnancy at 36 weeks gestation"]

## Encounter 0: source 2020-01-15, practice 2025-10-20, outpatient, Obstetrics & Gynecology

Chief complaint: Initial prenatal visit at 8 weeks gestation with incidentally discovered retained intrauterine device

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Initial prenatal visit at 8 weeks gestation with incidentally discovered retained intrauterine device

HISTORY OF PRESENT ILLNESS:
She is currently at 28 weeks gestation with her first pregnancy. Her prenatal course has been unremarkable. She denies any vaginal bleeding, abdominal pain, or trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Primigravid pregnancy
- Retained copper intrauterine device in situ
- Placenta previa
- Incomplete adult immunization (susceptible to diphtheria, tetanus, and pertussis)
- Pregnancy complicated by fetal tracheoesophageal fistula with esophageal atresia

MEDICATIONS:
Home Medications:
- Prenatal vitamin with iron 1 tablet daily
- Folic acid 400 mcg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal history of third-trimester bleeding disorders
- Paternal history of VACTERL association features (congenital vertebral anomalies)
- No history of heritable thrombophilia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

VITAL SIGNS:
Her blood pressure is 120/78 mmHg, heart rate is 72 beats per minute

PHYSICAL EXAMINATION:
Her physical examination is normal with a fundal height appropriate for gestational age. Fetal heart tones are normal.

LABORATORY DATA:
- Hemoglobin: 12.5 g/dL
- Hematocrit: 37%
- Blood type: A negative
- Indirect Coombs test: Negative
- Urinalysis: Normal

## Encounter 1: source 2020-06-15, practice 2026-03-27, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit with increasing maternal abdominal distension and suspected polyhydramnios

ENCOUNTER NOTE
Date: 2020-06-15
Provider: Dr. Maria Gonzalez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit with increasing maternal abdominal distension and suspected polyhydramnios

HISTORY OF PRESENT ILLNESS:
Returning after initial visit with retained IUD. She reports increasing abdominal distension but denies contractions, leakage of fluid, or vaginal bleeding.

PAST MEDICAL HISTORY:
Active Problem List:
- Copper-containing intrauterine device (diagnosed 2020-01-15)
- Primigravid pregnancy
- Retained copper intrauterine device in situ
- Placenta previa
- Incomplete adult immunization (susceptible to diphtheria, tetanus, and pertussis)
- Pregnancy complicated by fetal tracheoesophageal fistula with esophageal atresia

MEDICATIONS:
Home Medications:
- Prenatal vitamin with iron 1 tablet daily
- Folic acid 400 mcg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal history of third-trimester bleeding disorders
- Paternal history of VACTERL association features (congenital vertebral anomalies)
- No history of heritable thrombophilia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
Fundal height is greater than expected for gestational age,

IMAGING:
Ultrasound reveals excessive amniotic fluid. Doppler studies are normal. Amniotic fluid index is increased.

## Encounter 2: source 2020-08-15, practice 2026-05-28, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal care at 28 weeks gestation with acute onset pharyngitis and low-grade fever

ENCOUNTER NOTE
Date: 2020-08-15
Provider: Dr. Abdi Hassan, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal care at 28 weeks gestation with acute onset pharyngitis and low-grade fever

PAST MEDICAL HISTORY:
Active Problem List:
- Copper-containing intrauterine device (diagnosed 2020-01-15)
- Tracheoesophageal fistula with esophageal atresia (diagnosed 2020-06-15)
- Primigravid pregnancy
- Retained copper intrauterine device in situ
- Placenta previa
- Incomplete adult immunization (susceptible to diphtheria, tetanus, and pertussis)
- Pregnancy complicated by fetal tracheoesophageal fistula with esophageal atresia

She reports that she received all recommended childhood immunizations but is unsure about her adult vaccinations.

MEDICATIONS:
Home Medications:
- Prenatal vitamin with iron 1 tablet daily
- Folic acid 400 mcg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal history of third-trimester bleeding disorders
- Paternal history of VACTERL association features (congenital vertebral anomalies)
- No history of heritable thrombophilia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

ASSESSMENT AND PLAN:
This vaccine helps prevent neonatal infection with a bacterium that inhibits elongation factor-2 (EF-2). The immunologic component used to elicit protective immunity in this vaccine is best described as

Her physician recommends administration of the Tdap vaccine during this visit.

## Encounter 3: source 2020-09-15, practice 2026-06-27, ed, Emergency Medicine

Chief complaint: Decreased fetal movement over the past 24 hours noted by patient

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Emily Watson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Decreased fetal movement over the past 24 hours noted by patient

HISTORY OF PRESENT ILLNESS:
The 28-year-old primigravid patient with retained copper IUD, placenta previa, fetal tracheoesophageal fistula, and recent diphtheria presents for evaluation of decreased fetal movement. She reports that the baby used to be very active, especially after meals, but she has not felt the usual activity since yesterday evening. Her pregnancy has been uncomplicated. She denies any vaginal bleeding, leakage of fluid, or contractions. The patient is anxious about the well-being of her baby.

PAST MEDICAL HISTORY:
Active Problem List:
- Copper-containing intrauterine device (diagnosed 2020-01-15)
- Tracheoesophageal fistula with esophageal atresia (diagnosed 2020-06-15)
- Diphtheria (diagnosed 2020-08-15)
- Primigravid pregnancy
- Retained copper intrauterine device in situ
- Placenta previa
- Incomplete adult immunization (susceptible to diphtheria, tetanus, and pertussis)
- Pregnancy complicated by fetal tracheoesophageal fistula with esophageal atresia

MEDICATIONS:
Home Medications:
- Prenatal vitamin with iron 1 tablet daily
- Folic acid 400 mcg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal history of third-trimester bleeding disorders
- Paternal history of VACTERL association features (congenital vertebral anomalies)
- No history of heritable thrombophilia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

VITAL SIGNS:
Her blood pressure today is 120/78 mmHg.

PHYSICAL EXAMINATION:
On examination, the uterus is appropriately sized for gestational age, and fetal heart tones are heard at 140 beats per minute. There is no tenderness or irritability on palpation of the uterus.

LABORATORY DATA:
Her prenatal labs have been unremarkable.

## Encounter 4: source 2020-09-15, practice 2026-06-27, ed, Emergency Medicine

Chief complaint: Sudden onset of severe abdominal pain and vaginal bleeding

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Raj Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe abdominal pain and vaginal bleeding

PAST MEDICAL HISTORY:
Active Problem List:
- Copper-containing intrauterine device (diagnosed 2020-01-15)
- Tracheoesophageal fistula with esophageal atresia (diagnosed 2020-06-15)
- Diphtheria (diagnosed 2020-08-15)
- Decreased fetal movements (diagnosed 2020-09-15)
- Primigravid pregnancy
- Retained copper intrauterine device in situ
- Placenta previa
- Incomplete adult immunization (susceptible to diphtheria, tetanus, and pertussis)
- Pregnancy complicated by fetal tracheoesophageal fistula with esophageal atresia

She has a known history of placenta previa.

PAST SURGICAL HISTORY:
She has had two previous cesarean deliveries.

MEDICATIONS:
Home Medications:
- Prenatal vitamin with iron 1 tablet daily
- Folic acid 400 mcg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal history of third-trimester bleeding disorders
- Paternal history of VACTERL association features (congenital vertebral anomalies)
- No history of heritable thrombophilia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

VITAL SIGNS:
Her vital signs on admission are: blood pressure 110/70 mmHg, heart rate 98 bpm, respiratory rate 18 breaths/min, and temperature 37.0°C (98.6°F).

PHYSICAL EXAMINATION:
On examination, her abdomen is tender to palpation, particularly in the lower quadrants, without rebound or guarding. Fetal heart tones are 140 beats/min.

ASSESSMENT AND PLAN:
Suspected placental abruption

The obstetric team recommends an emergency cesarean delivery due to suspected placental abruption. However, the patient refuses the procedure, stating she has read about the risks of multiple cesareans and wishes to avoid any further surgeries. She appears alert and oriented, and understands the potential risks to herself and the fetus but remains firm in her decision.

## Encounter 5: source 2020-10-15, practice 2026-07-27, inpatient, Obstetrics & Gynecology

Chief complaint: Sudden gush of vaginal bleeding immediately after rupture of membranes during early labor

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Lisa Park, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Sudden gush of vaginal bleeding immediately after rupture of membranes during early labor

HISTORY OF PRESENT ILLNESS:
This primigravid female with retained IUD, fetal esophageal atresia, and recent placental abruption presents with vaginal bleeding. The blood was dark red, and she describes the onset as abrupt and not accompanied by abdominal pain. Her pregnancy has been otherwise uncomplicated, with regular prenatal care and normal ultrasounds until this point.

PAST MEDICAL HISTORY:
Active Problem List:
- Copper-containing intrauterine device (diagnosed 2020-01-15)
- Tracheoesophageal fistula with esophageal atresia (diagnosed 2020-06-15)
- Diphtheria (diagnosed 2020-08-15)
- Decreased fetal movements (diagnosed 2020-09-15)
- Placental Abruption (diagnosed 2020-09-15)
- Primigravid pregnancy
- Retained copper intrauterine device in situ
- Placenta previa
- Incomplete adult immunization (susceptible to diphtheria, tetanus, and pertussis)
- Pregnancy complicated by fetal tracheoesophageal fistula with esophageal atresia

MEDICATIONS:
Home Medications:
- Prenatal vitamin with iron 1 tablet daily
- Folic acid 400 mcg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal history of third-trimester bleeding disorders
- Paternal history of VACTERL association features (congenital vertebral anomalies)
- No history of heritable thrombophilia

SOCIAL HISTORY:
She does not smoke, drink alcohol, or use illicit drugs.

PHYSICAL EXAMINATION:
On arrival, the mother appears anxious but hemodynamically stable. Abdominal examination shows a gravid, soft uterus with no tenderness or rigidity. Contractions are mild and irregular. Speculum examination reveals moderate blood in the vaginal vault,

OTHER STUDIES:
Continuous monitoring demonstrates a baseline fetal heart rate of 140/min with sudden recurrent decelerations shortly after the bleeding began.

PLAN:
Preparations for urgent intervention are underway.
