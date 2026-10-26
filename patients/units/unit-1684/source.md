# synthetic_hospital patient 1684

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Hispanic/Latina, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-07-24 (Wed) | ed | Emergency Medicine |
| 1 | 2021-01-15 | 2025-08-11 (Mon) | ed | Emergency Medicine |
| 2 | 2021-09-15 | 2026-04-08 (Wed) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Sulfa drugs"
 ],
 "insurance": "Medicaid",
 "occupation": "Retail store manager",
 "alcohol_use": "None",
 "family_history": [
  "Mother with placenta previa requiring cesarean delivery",
  "Maternal aunt with uterine rupture during labor"
 ],
 "race_ethnicity": "Hispanic/Latina",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 tablet daily",
   "name": "Prenatal vitamin with iron"
  },
  {
   "dose": "325 mg three times daily",
   "name": "Ferrous sulfate"
  },
  {
   "dose": "600 mg twice daily",
   "name": "Calcium carbonate"
  }
 ],
 "surgical_history": [
  "Low transverse cesarean section x 2 (2019, 2021)"
 ],
 "chronic_conditions": [
  "Iron deficiency anemia",
  "Obesity (BMI 34)"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Uterine rupture"]
Comorbidities: ["History of previous cesarean delivery", "Term pregnancy", "Anemia in pregnancy", "Fetal distress complicating labor"]

## Encounter 0: source 2020-01-15, practice 2024-07-24, ed, Emergency Medicine

Chief complaint: Sudden onset of severe abdominal pain that started 2 hours ago, constant and worsening, without vaginal bleeding

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Vasquez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe abdominal pain that started 2 hours ago, constant and worsening, without vaginal bleeding

HISTORY OF PRESENT ILLNESS:
She reports that the pain is constant and seems to be worsening. She denies any vaginal bleeding or leakage of fluid.

PAST MEDICAL HISTORY:
Active Problem List:
- Iron deficiency anemia
- Obesity (BMI 34)

Her past medical history is significant for two prior cesarean deliveries.

PAST SURGICAL HISTORY:
- Low transverse cesarean section x 2 (2019, 2021)

MEDICATIONS:
Home Medications:
- Prenatal vitamin with iron 1 tablet daily
- Ferrous sulfate 325 mg three times daily
- Calcium carbonate 600 mg twice daily

ALLERGIES:
Sulfa drugs

FAMILY HISTORY:
- Mother with placenta previa requiring cesarean delivery
- Maternal aunt with uterine rupture during labor

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retail store manager.

VITAL SIGNS:
On examination, her vital signs are stable

PHYSICAL EXAMINATION:
She appears to be in distress. Abdominal examination reveals tenderness over the uterine fundus with guarding. On pelvic examination, the fetal head, which was previously engaged, is now palpable at a higher station than noted in her prenatal records.

IMAGING:
An ultrasound confirms that the fetus is in a vertex presentation.

OTHER STUDIES:
Fetal heart rate monitoring shows repetitive late decelerations and decreased variability. The patient's contractions are noted to be irregular with progressively decreasing amplitude.

## Encounter 1: source 2021-01-15, practice 2025-08-11, ed, Emergency Medicine

Chief complaint: Sudden onset of severe abdominal pain and vaginal bleeding with 'popping' sensation

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe abdominal pain and vaginal bleeding with 'popping' sensation

HISTORY OF PRESENT ILLNESS:
This 33-year-old obese female with iron deficiency anemia and uterine rupture in January 2020 returns with recurrent symptoms at 38 weeks gestation, reporting a 'popping' sensation followed by the pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Uterine rupture (diagnosed 2020-01-15)
- Iron deficiency anemia
- Obesity (BMI 34)

PAST SURGICAL HISTORY:
History of two prior cesarean deliveries

MEDICATIONS:
Home Medications:
- Prenatal vitamin with iron 1 tablet daily
- Ferrous sulfate 325 mg three times daily
- Calcium carbonate 600 mg twice daily

ALLERGIES:
Sulfa drugs

FAMILY HISTORY:
- Mother with placenta previa requiring cesarean delivery
- Maternal aunt with uterine rupture during labor

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retail store manager.

VITAL SIGNS:
On examination, her blood pressure is 110/70 mmHg, heart rate is 102 beats per minute, and temperature is 37.2°C (99.0°F).

PHYSICAL EXAMINATION:
Abdominal examination reveals a tender uterus with guarding. Fetal heart tones are irregular, with a baseline of 110 bpm and minimal variability. There is no palpable presenting part above the pelvis.

LABORATORY DATA:
Her laboratory values are as follows:

- Hemoglobin: 10.2 g/dL (normal: 12-16 g/dL)
- Hematocrit: 30.4% (normal: 37-47%)
- Platelets: 210,000/mm³ (normal: 150,000-400,000/mm³)
- White blood cell count: 12,000/mm³ (normal: 4,500-11,000/mm³)

## Encounter 2: source 2021-09-15, practice 2026-04-08, icu, Emergency Medicine

Chief complaint: Sudden onset of severe abdominal pain and vaginal bleeding with pain radiating to back

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Sudden onset of severe abdominal pain and vaginal bleeding with pain radiating to back

HISTORY OF PRESENT ILLNESS:
The patient, a 33-year-old female with iron deficiency anemia and obesity, returns for recurrent severe abdominal pain as seen in January 2020 and January 2021. She reports that the pain is constant and radiating to her back.

PAST MEDICAL HISTORY:
Active Problem List:
- Uterine rupture (diagnosed 2020-01-15)
- Iron deficiency anemia
- Obesity (BMI 34)

Her past medical history is significant for two prior cesarean deliveries.

PAST SURGICAL HISTORY:
- Low transverse cesarean section x 2 (2019, 2021)

MEDICATIONS:
Home Medications:
- Prenatal vitamin with iron 1 tablet daily
- Ferrous sulfate 325 mg three times daily
- Calcium carbonate 600 mg twice daily

ALLERGIES:
Sulfa drugs

FAMILY HISTORY:
- Mother with placenta previa requiring cesarean delivery
- Maternal aunt with uterine rupture during labor

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retail store manager.

VITAL SIGNS:
Blood pressure is 100/60 mmHg, heart rate is 110 bpm

PHYSICAL EXAMINATION:
On examination, she appears to be in distress. Abdominal examination reveals tenderness over the uterine fundus and an irregular abdominal contour.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin: 9.8 g/dL (normal: 12-16 g/dL)
- Hematocrit: 29% (normal: 37-47%)
- Platelets: 210,000/mm³ (normal: 150,000-400,000/mm³)
- White blood cell count: 12,000/mm³ (normal: 4,500-11,000/mm³)

OTHER STUDIES:
Fetal heart monitoring shows late decelerations.
