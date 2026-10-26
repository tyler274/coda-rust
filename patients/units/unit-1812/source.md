# synthetic_hospital patient 1812

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, African American, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-01-26 (Thu) | outpatient | Obstetrics & Gynecology |
| 1 | 2021-07-15 | 2024-08-08 (Thu) | ed | Emergency Medicine |
| 2 | 2022-07-15 | 2025-07-31 (Thu) | ed | Emergency Medicine |
| 3 | 2023-07-15 | 2026-07-31 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Rare (1-2 drinks per month)",
 "family_history": [
  "Maternal history of peripartum cardiomyopathy",
  "Sister with severe preeclampsia"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "1 tablet daily",
   "name": "Prenatal vitamin with DHA"
  },
  {
   "dose": "325 mg (65 mg elemental iron) daily",
   "name": "Ferrous sulfate"
  },
  {
   "dose": "5 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25 mg daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "100 mg twice daily",
   "name": "Docusate sodium"
  }
 ],
 "surgical_history": [
  "Cesarean section (low transverse)",
  "Vesicovaginal fistula repair",
  "Secondary repair of second-degree perineal tear"
 ],
 "chronic_conditions": [
  "Iron deficiency anemia",
  "History of peripartum cardiomyopathy",
  "History of vesicovaginal fistula"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Pudendal nerve injury", "Vesicovaginal fistula", "Postpartum endometritis", "Peripartum cardiomyopathy"]
Comorbidities: ["Prolonged labor", "Primiparity"]

## Encounter 0: source 2020-01-15, practice 2023-01-26, outpatient, Obstetrics & Gynecology

Chief complaint: Six-week history of continuous watery vaginal discharge following home delivery

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Aisha Johnson, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-week history of continuous watery vaginal discharge following home delivery

HISTORY OF PRESENT ILLNESS:
She reports that the symptoms began following a prolonged labor at home, which ended in a stillbirth. She has not had urinary incontinence.

PAST MEDICAL HISTORY:
Active Problem List:
- Iron deficiency anemia
- History of peripartum cardiomyopathy
- History of vesicovaginal fistula

PAST SURGICAL HISTORY:
- Cesarean section (low transverse)
- Vesicovaginal fistula repair
- Secondary repair of second-degree perineal tear

MEDICATIONS:
Home Medications:
- Prenatal vitamin with DHA 1 tablet daily
- Ferrous sulfate 325 mg (65 mg elemental iron) daily
- Lisinopril 5 mg daily
- Metoprolol succinate 25 mg daily
- Docusate sodium 100 mg twice daily

ALLERGIES:
She has no known drug allergies.

FAMILY HISTORY:
- Maternal history of peripartum cardiomyopathy
- Sister with severe preeclampsia

SOCIAL HISTORY:
Smoking: Never. Alcohol: Rare (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
On physical examination, the patient is afebrile, and vital signs are within normal limits.

PHYSICAL EXAMINATION:
Pelvic examination reveals an area of raised, red granulation tissue on the anterior vaginal wall.

OTHER STUDIES:
A bladder dye test is performed, and the dye is observed leaking into the vagina.

## Encounter 1: source 2021-07-15, practice 2024-08-08, ed, Emergency Medicine

Chief complaint: Fever of 38.9°C with foul-smelling lochia on postpartum day 3 after cesarean delivery

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Michael Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever of 38.9°C with foul-smelling lochia on postpartum day 3 after cesarean delivery

HISTORY OF PRESENT ILLNESS:
The patient is a 33-year-old female with iron deficiency anemia, peripartum cardiomyopathy, and history of vesicovaginal fistula following home delivery who presents on postpartum day 3 after cesarean delivery. She reports experiencing chills and malaise since the previous day. Her obstetric history is significant for a prolonged labor that resulted in a cesarean delivery due to fetal distress. She has been having foul-smelling lochia and notes that her uterus feels tender when she touches her abdomen.

PAST MEDICAL HISTORY:
Active Problem List:
- Vesicovaginal fistula (diagnosed 2020-01-15)
- Iron deficiency anemia
- History of peripartum cardiomyopathy

PAST SURGICAL HISTORY:
- Cesarean section (low transverse)
- Vesicovaginal fistula repair
- Secondary repair of second-degree perineal tear

MEDICATIONS:
Home Medications:
- Prenatal vitamin with DHA 1 tablet daily
- Ferrous sulfate 325 mg (65 mg elemental iron) daily
- Lisinopril 5 mg daily
- Metoprolol succinate 25 mg daily
- Docusate sodium 100 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal history of peripartum cardiomyopathy
- Sister with severe preeclampsia

SOCIAL HISTORY:
Smoking: Never. Alcohol: Rare (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Heart rate is 110 beats per minute, blood pressure is 120/75 mmHg

PHYSICAL EXAMINATION:
Abdominal examination reveals a tender uterus with no rebound or guarding

LABORATORY DATA:
White blood cell count: 18,000/mm³
- Hemoglobin: 11.2 g/dL
- Platelet count: 250,000/mm³
- C-reactive protein: 15 mg/L (normal <5 mg/L)

## Encounter 2: source 2022-07-15, practice 2025-07-31, ed, Emergency Medicine

Chief complaint: Inability to void and lower abdominal discomfort 24 hours after vaginal delivery

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Sarah Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Inability to void and lower abdominal discomfort 24 hours after vaginal delivery

HISTORY OF PRESENT ILLNESS:
This 34-year-old with vesicovaginal fistula, peripartum cardiomyopathy, and postpartum endometritis presents 24 hours postpartum. She had a vaginal delivery with a second-degree perineal tear that was repaired. Her labor was prolonged, lasting 18 hours, and she received an epidural for pain control.

PAST MEDICAL HISTORY:
Active Problem List:
- Vesicovaginal fistula (diagnosed 2020-01-15)
- Postpartum endometritis (diagnosed 2021-07-15)
- Iron deficiency anemia
- History of peripartum cardiomyopathy

PAST SURGICAL HISTORY:
- Cesarean section (low transverse)
- Vesicovaginal fistula repair
- Secondary repair of second-degree perineal tear

MEDICATIONS:
Home Medications:
- Prenatal vitamin with DHA 1 tablet daily
- Ferrous sulfate 325 mg (65 mg elemental iron) daily
- Lisinopril 5 mg daily
- Metoprolol succinate 25 mg daily
- Docusate sodium 100 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal history of peripartum cardiomyopathy
- Sister with severe preeclampsia

SOCIAL HISTORY:
Smoking: Never. Alcohol: Rare (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Her vital signs are stable, and she has no fever.

PHYSICAL EXAMINATION:
On examination, her lower abdomen is distended and dull to percussion.

IMAGING:
A bladder scan shows a post-void residual volume of 600 mL.

## Encounter 3: source 2023-07-15, practice 2026-07-31, ed, Emergency Medicine

Chief complaint: Worsening shortness of breath, orthopnea, and lower extremity edema 3 weeks postpartum

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. James Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening shortness of breath, orthopnea, and lower extremity edema 3 weeks postpartum

HISTORY OF PRESENT ILLNESS:
This 35-year-old female with a history of peripartum cardiomyopathy and prior postpartum complications presents approximately 3 weeks after delivering a full-term infant. She reports difficulty breathing while lying flat, and her lower legs have become swollen.

PAST MEDICAL HISTORY:
Active Problem List:
- Vesicovaginal fistula (diagnosed 2020-01-15)
- Postpartum endometritis (diagnosed 2021-07-15)
- Pudendal nerve injury (diagnosed 2022-07-15)
- Iron deficiency anemia
- History of peripartum cardiomyopathy

PAST SURGICAL HISTORY:
- Cesarean section (low transverse)
- Vesicovaginal fistula repair
- Secondary repair of second-degree perineal tear

MEDICATIONS:
Home Medications:
- Prenatal vitamin with DHA 1 tablet daily
- Ferrous sulfate 325 mg (65 mg elemental iron) daily
- Lisinopril 5 mg daily
- Metoprolol succinate 25 mg daily
- Docusate sodium 100 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal history of peripartum cardiomyopathy
- Sister with severe preeclampsia

SOCIAL HISTORY:
Smoking: Never. Alcohol: Rare (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Vital signs show blood pressure of 100/60 mmHg, pulse of 110/min, and respiratory rate of 24/min.

PHYSICAL EXAMINATION:
Physical exam reveals bilateral crackles at the lung bases and 2+ pitting edema of the lower extremities. There is an audible S3 gallop on cardiac auscultation.

IMAGING:
Echocardiogram shows a dilated left ventricle with an ejection fraction of 25%. Chest x-ray reveals pulmonary vascular congestion and cardiomegaly.

PLAN:
She is admitted to the intensive care unit for further management.
