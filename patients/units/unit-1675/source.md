# synthetic_hospital patient 1675

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 30 at first encounter, sex F, Non-Hispanic Black, Private PPO through employer.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2026-03-23 (Mon) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-02-15 | 2026-04-26 (Sun) | ed | Emergency Medicine |
| 2 | 2020-03-15 | 2026-05-13 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private PPO through employer",
 "occupation": "Human resources coordinator",
 "alcohol_use": "No alcohol use during pregnancy; occasional social drinker prior to conception",
 "family_history": [
  "Mother: Preeclampsia during pregnancies, hypertension",
  "Father: Essential hypertension diagnosed age 45",
  "Sister: Gestational diabetes"
 ],
 "race_ethnicity": "Non-Hispanic Black",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 tablet PO daily",
   "name": "Prenatal vitamin (with folic acid, iron, DHA)"
  },
  {
   "dose": "81 mg PO daily (initiated at 12 weeks for preeclampsia prophylaxis given prior gestational hypertension)",
   "name": "Low-dose aspirin"
  },
  {
   "dose": "500 mg PO twice daily",
   "name": "Calcium carbonate"
  }
 ],
 "surgical_history": [
  "Cesarean section (low transverse, 2 years prior)",
  "Laparoscopic appendectomy (age 18)"
 ],
 "chronic_conditions": [
  "History of gestational hypertension (prior pregnancy)",
  "Obesity (BMI 31 kg/m\u00b2)"
 ],
 "age_at_first_encounter": 30
}
```

Primary diagnoses: ["Mild preeclampsia without severe features", "Severe preeclampsia without HELLP features"]
Comorbidities: ["Pregnancy (34 weeks gestation)", "Multiparity"]

## Encounter 0: source 2020-01-15, practice 2026-03-23, outpatient, Obstetrics & Gynecology

Chief complaint: Follow-up evaluation of elevated blood pressure noted at prior prenatal visit with occasional headaches and mild swelling of hands and feet

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up evaluation of elevated blood pressure noted at prior prenatal visit with occasional headaches and mild swelling of hands and feet

HISTORY OF PRESENT ILLNESS:
She reports occasional headaches and some swelling in her hands and feet but denies visual changes, abdominal pain, or shortness of breath. Her pregnancy was previously uncomplicated.

PAST MEDICAL HISTORY:
Active Problem List:
- History of gestational hypertension (prior pregnancy)
- Obesity (BMI 31 kg/m²)

PAST SURGICAL HISTORY:
- Cesarean section (low transverse, 2 years prior)
- Laparoscopic appendectomy (age 18)

MEDICATIONS:
Home Medications:
- Prenatal vitamin (with folic acid, iron, DHA) 1 tablet PO daily
- Low-dose aspirin 81 mg PO daily (initiated at 12 weeks for preeclampsia prophylaxis given prior gestational hypertension)
- Calcium carbonate 500 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Preeclampsia during pregnancies, hypertension
- Father: Essential hypertension diagnosed age 45
- Sister: Gestational diabetes

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use during pregnancy; occasional social drinker prior to conception. Occupation: Human resources coordinator.

VITAL SIGNS:
Her current blood pressure is 152/98 mm Hg.

LABORATORY DATA:
Urinalysis reveals 2+ protein.

IMAGING:
Ultrasound shows a fetus at appropriate size for gestational age with normal amniotic fluid volume and movement. Doppler velocimetry of the uterine arteries demonstrates high-resistance blood flow with reduced diastolic flow velocity.

## Encounter 1: source 2020-02-15, practice 2026-04-26, ed, Emergency Medicine

Chief complaint: Persistent dull headache for three days unrelieved by rest or OTC medication and intermittent upper abdominal discomfort

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Persistent dull headache for three days unrelieved by rest or OTC medication and intermittent upper abdominal discomfort

HISTORY OF PRESENT ILLNESS:
This patient with a history of gestational hypertension in a prior pregnancy returns one month after her outpatient diagnosis of mild preeclampsia with worsening symptoms. She reports that the headache has lasted for three days and is not relieved with rest or over-the-counter medication. She has also noticed mild swelling in her legs and fingers. She denies visual changes, shortness of breath, fever, or contractions. Fetal movements remain normal. She has had regular prenatal care with no prior complications.

PAST MEDICAL HISTORY:
Active Problem List:
- Mild preeclampsia without severe features (diagnosed 2020-01-15)
- History of gestational hypertension (prior pregnancy)
- Obesity (BMI 31 kg/m²)

PAST SURGICAL HISTORY:
- Cesarean section (low transverse, 2 years prior)
- Laparoscopic appendectomy (age 18)

MEDICATIONS:
Home Medications:
- Prenatal vitamin (with folic acid, iron, DHA) 1 tablet PO daily
- Low-dose aspirin 81 mg PO daily (initiated at 12 weeks for preeclampsia prophylaxis given prior gestational hypertension)
- Calcium carbonate 500 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Preeclampsia during pregnancies, hypertension
- Father: Essential hypertension diagnosed age 45
- Sister: Gestational diabetes

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use during pregnancy; occasional social drinker prior to conception. Occupation: Human resources coordinator.

VITAL SIGNS:
Her blood pressure is 150/98 mm Hg, confirmed on repeat measurement

PHYSICAL EXAMINATION:
She has 1+ pedal edema bilaterally.

LABORATORY DATA:
Urinalysis reveals 2+ protein.

IMAGING:
An ultrasound shows normal fetal growth and amniotic fluid volume.

OTHER STUDIES:
Fetal heart rate monitoring is reactive.

## Encounter 2: source 2020-03-15, practice 2026-05-13, ed, Emergency Medicine

Chief complaint: Severe headache and blurred vision for the past 2 hours at 34 weeks gestation

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe headache and blurred vision for the past 2 hours at 34 weeks gestation

HISTORY OF PRESENT ILLNESS:
Following recent care for mild preeclampsia and prior gestational hypertension, she presents at 34 weeks of gestation to the emergency department with a severe headache and blurred vision for the past 2 hours.

PAST MEDICAL HISTORY:
Active Problem List:
- Mild preeclampsia without severe features (diagnosed 2020-01-15)
- History of gestational hypertension (prior pregnancy)
- Obesity (BMI 31 kg/m²)

She has a history of gestational hypertension in her previous pregnancy.

PAST SURGICAL HISTORY:
- Cesarean section (low transverse, 2 years prior)
- Laparoscopic appendectomy (age 18)

MEDICATIONS:
Home Medications:
- Prenatal vitamin (with folic acid, iron, DHA) 1 tablet PO daily
- Low-dose aspirin 81 mg PO daily (initiated at 12 weeks for preeclampsia prophylaxis given prior gestational hypertension)
- Calcium carbonate 500 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Preeclampsia during pregnancies, hypertension
- Father: Essential hypertension diagnosed age 45
- Sister: Gestational diabetes

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use during pregnancy; occasional social drinker prior to conception. Occupation: Human resources coordinator.

VITAL SIGNS:
Her blood pressure is 168/112 mm Hg, heart rate is 88 beats per minute, and respiratory rate is 18 breaths per minute.

PHYSICAL EXAMINATION:
Physical examination reveals 2+ lower extremity edema and deep tendon reflexes are 3+. Fundoscopic examination shows no papilledema.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 11.2 g/dL
- Platelet count: 95,000/uL
- Serum creatinine: 1.2 mg/dL
- Aspartate aminotransferase (AST): 70 U/L
- Alanine aminotransferase (ALT): 65 U/L
- Urinalysis: 2+ protein

OTHER STUDIES:
Fetal monitoring indicates a reactive fetal heart tracing.
