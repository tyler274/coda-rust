# synthetic_hospital patient 1688

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Non-Hispanic White, Private PPO through employer.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-10-13 (Fri) | ed | Emergency Medicine |
| 1 | 2021-04-15 | 2025-01-07 (Tue) | icu | Obstetrics & Gynecology |
| 2 | 2022-01-15 | 2025-10-05 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Sulfa drugs - rash",
  "Latex - contact dermatitis"
 ],
 "insurance": "Private PPO through employer",
 "occupation": "Human resources coordinator",
 "alcohol_use": "Social, 1-2 drinks per month when not pregnant",
 "family_history": [
  "Mother: Preeclampsia and chronic hypertension",
  "Father: Essential hypertension",
  "Maternal aunt: Placental abruption"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 4 years ago (prior to conception)",
 "home_medications": [
  {
   "dose": "200 mg PO BID",
   "name": "Labetalol"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Prenatal vitamin with DHA"
  },
  {
   "dose": "1 mg PO daily",
   "name": "Folic acid"
  }
 ],
 "surgical_history": [
  "Emergency cesarean section (age 32)",
  "Laparoscopic appendectomy (age 24)"
 ],
 "chronic_conditions": [
  "Chronic hypertension",
  "Anxiety disorder"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Amniotic fluid embolism syndrome", "Placental Abruption", "Amniotic fluid embolism"]
Comorbidities: ["Disseminated intravascular coagulation", "Hypovolemic shock"]

## Encounter 0: source 2020-01-15, practice 2023-10-13, ed, Emergency Medicine

Chief complaint: presents to the emergency department with sudden onset of severe abdominal pain and vaginal bleeding for the past hour

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
She presents to the emergency department with sudden onset of severe abdominal pain and vaginal bleeding for the past hour

HISTORY OF PRESENT ILLNESS:
She describes the pain as constant and sharp, localized to the abdomen without radiation. Her prenatal course was unremarkable except for mild hypertension, which has been managed with labetalol.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic hypertension
- Anxiety disorder

PAST SURGICAL HISTORY:
- Emergency cesarean section (age 32)
- Laparoscopic appendectomy (age 24)

MEDICATIONS:
Home Medications:
- Labetalol 200 mg PO BID
- Prenatal vitamin with DHA 1 tablet PO daily
- Folic acid 1 mg PO daily

ALLERGIES:
Sulfa drugs - rash, Latex - contact dermatitis

FAMILY HISTORY:
- Mother: Preeclampsia and chronic hypertension
- Father: Essential hypertension
- Maternal aunt: Placental abruption

SOCIAL HISTORY:
Smoking: Former smoker, quit 4 years ago (prior to conception). Alcohol: Social, 1-2 drinks per month when not pregnant. Occupation: Human resources coordinator.

VITAL SIGNS:
Blood pressure is 90/60 mmHg, heart rate is 120 beats per minute

PHYSICAL EXAMINATION:
On examination, she appears pale and diaphoretic. The abdomen is diffusely tender to palpation, and the uterus is firm and contracted. Fetal heart tones are 140 beats per minute. Speculum examination reveals moderate dark red vaginal bleeding.

LABORATORY DATA:
- Hemoglobin: 9 g/dL (baseline 12 g/dL)
- Platelet count: 150,000/mm³
- Prothrombin time (PT): 14 seconds (control 12 seconds)
- Activated partial thromboplastin time (aPTT): 35 seconds (control 30 seconds)
- Fibrinogen: 200 mg/dL (normal 400-600 mg/dL)

## Encounter 1: source 2021-04-15, practice 2025-01-07, icu, Obstetrics & Gynecology

Chief complaint: sudden difficulty breathing immediately following delivery of the placenta after uncomplicated vaginal delivery at term

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. Marcus Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: icu

CHIEF COMPLAINT:
Sudden difficulty breathing immediately following delivery of the placenta after uncomplicated vaginal delivery at term

HISTORY OF PRESENT ILLNESS:
The patient is a 33-year-old female with chronic hypertension and anxiety disorder who was previously evaluated in 2020 for placental abruption. She presents following an uncomplicated vaginal delivery at term. Immediately following delivery of the placenta, the patient reports sudden difficulty breathing and appears visibly anxious. She is given supplemental oxygen and IV fluids, but she continues to deteriorate. She is intubated for respiratory distress and transferred to the intensive care unit.

PAST MEDICAL HISTORY:
Active Problem List:
- Placental Abruption (diagnosed 2020-01-15)
- Chronic hypertension
- Anxiety disorder

Her pregnancy was uneventful, and prenatal labs were all within normal limits.

PAST SURGICAL HISTORY:
- Emergency cesarean section (age 32)
- Laparoscopic appendectomy (age 24)

MEDICATIONS:
Home Medications:
- Labetalol 200 mg PO BID
- Prenatal vitamin with DHA 1 tablet PO daily
- Folic acid 1 mg PO daily

ALLERGIES:
Sulfa drugs - rash, Latex - contact dermatitis

FAMILY HISTORY:
- Mother: Preeclampsia and chronic hypertension
- Father: Essential hypertension
- Maternal aunt: Placental abruption

SOCIAL HISTORY:
Smoking: Former smoker, quit 4 years ago (prior to conception). Alcohol: Social, 1-2 drinks per month when not pregnant. Occupation: Human resources coordinator.

VITAL SIGNS:
Her heart rate rapidly increases, and she becomes hypotensive.

PHYSICAL EXAMINATION:
Within minutes, the patient begins bleeding from her IV sites and her gums. The labor and delivery team notices that the uterus is firm and well-contracted, but there is unexpected blood loss saturating her perineal pads. Chest auscultation reveals bilateral rales, and her skin is cool and clammy. No uterine atony is noted on pelvic exam.

LABORATORY DATA:
Her peripheral smear shows fragmented red cells, and she is noted to have decreased urine output with rising creatinine levels.

IMAGING:
A stat echocardiogram reveals acute right ventricular dysfunction.

## Encounter 2: source 2022-01-15, practice 2025-10-05, ed, Emergency Medicine

Chief complaint: brought to the emergency department 30 minutes after a spontaneous vaginal delivery with sudden dyspnea and seizures

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought to the emergency department 30 minutes after a spontaneous vaginal delivery with sudden dyspnea and seizures

HISTORY OF PRESENT ILLNESS:
With chronic hypertension, abruption and AFE she returns. She is intubated for airway protection. Despite aggressive fluid resuscitation and blood product administration, her condition deteriorates.

PAST MEDICAL HISTORY:
Active Problem List:
- Placental Abruption (diagnosed 2020-01-15)
- Amniotic fluid embolism (diagnosed 2021-04-15)
- Chronic hypertension
- Anxiety disorder

PAST SURGICAL HISTORY:
- Emergency cesarean section (age 32)
- Laparoscopic appendectomy (age 24)

MEDICATIONS:
Home Medications:
- Labetalol 200 mg PO BID
- Prenatal vitamin with DHA 1 tablet PO daily
- Folic acid 1 mg PO daily

ALLERGIES:
Sulfa drugs - rash, Latex - contact dermatitis

FAMILY HISTORY:
- Mother: Preeclampsia and chronic hypertension
- Father: Essential hypertension
- Maternal aunt: Placental abruption

SOCIAL HISTORY:
Smoking: Former smoker, quit 4 years ago (prior to conception). Alcohol: Social, 1-2 drinks per month when not pregnant. Occupation: Human resources coordinator.

VITAL SIGNS:
Blood pressure is 70/40 mmHg, heart rate is 120 beats/min, respiratory rate is 30 breaths/min, and oxygen saturation is 85% on room air

PHYSICAL EXAMINATION:
Physical examination reveals diffuse rales in both lung fields and petechiae over her chest and arms.

LABORATORY DATA:
Hemoglobin: 9 g/dL (Normal: 12-16 g/dL)
- Platelet count: 50,000/mm³ (Normal: 150,000-400,000/mm³)
- Prothrombin time (PT): 18 seconds (Normal: 12-15 seconds)
- Activated partial thromboplastin time (aPTT): 40 seconds (Normal: 30-40 seconds)
- Fibrinogen: 100 mg/dL (Normal: 200-400 mg/dL)
- D-dimer: Elevated

IMAGING:
A bedside echocardiogram shows no evidence of right heart strain.
