# synthetic_hospital patient 1794

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 1 | 2020-05-15 | 2024-12-07 (Sat) | ed | Emergency Medicine |
| 2 | 2021-07-15 | 2026-02-06 (Fri) | ed | Emergency Medicine |
| 3 | 2021-09-15 | 2026-04-20 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Sulfa drugs - rash",
  "Latex - contact dermatitis"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Regional sales coordinator",
 "alcohol_use": "Social, 1-2 drinks per month",
 "family_history": [
  "Mother with chronic hypertension and preeclampsia",
  "Maternal aunt with history of ectopic pregnancy",
  "Paternal grandmother with deep vein thrombosis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 0.5 packs per day",
 "home_medications": [
  {
   "dose": "200 mg three times daily",
   "name": "Labetalol"
  },
  {
   "dose": "1 tablet daily",
   "name": "Prenatal vitamin with 1 mg folate"
  }
 ],
 "surgical_history": [
  "Laparoscopic left salpingectomy (ruptured ectopic pregnancy)",
  "Emergency low transverse cesarean section (placental abruption)"
 ],
 "chronic_conditions": [
  "Chronic hypertension",
  "History of ectopic pregnancy"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Ruptured ectopic pregnancy", "Placental Abruption", "Cholera"]
Comorbidities: ["Anemia in pregnancy", "Hypovolemic shock"]

## Encounter 1: source 2020-05-15, practice 2024-12-07, ed, Emergency Medicine

Chief complaint: Sudden onset of severe abdominal pain and vaginal bleeding at 18 weeks gestation

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Carlos Martinez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe abdominal pain and vaginal bleeding at 18 weeks gestation

HISTORY OF PRESENT ILLNESS:
This 32-year-old patient with chronic hypertension on labetalol and a history of ectopic pregnancy presents to the emergency department. She is at 18 weeks gestation in her first pregnancy, which had been uneventful until this point. The patient is tearful and expresses fear that she has done something wrong to cause this situation. She asks if the baby is okay.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic hypertension
- History of ectopic pregnancy

PAST SURGICAL HISTORY:
- Laparoscopic left salpingectomy (ruptured ectopic pregnancy)
- Emergency low transverse cesarean section (placental abruption)

MEDICATIONS:
Home Medications:
- Labetalol 200 mg three times daily
- Prenatal vitamin with 1 mg folate 1 tablet daily

ALLERGIES:
She has no history of drug allergies

FAMILY HISTORY:
- Mother with chronic hypertension and preeclampsia
- Maternal aunt with history of ectopic pregnancy
- Paternal grandmother with deep vein thrombosis

SOCIAL HISTORY:
Smoking: Current smoker, 0.5 packs per day. Alcohol: Social, 1-2 drinks per month. Occupation: Regional sales coordinator.

VITAL SIGNS:
On examination, her blood pressure is 100/60 mmHg, heart rate is 110 beats per minute, and temperature is 37.2°C (99.0°F)

PHYSICAL EXAMINATION:
Abdominal examination reveals generalized tenderness with guarding. Pelvic examination shows a closed cervical os with no products of conception visible.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin: 9.8 g/dL (normal: 12-16 g/dL)
- White blood cell count: 12,000/mm³ (normal: 4,500-11,000/mm³)
- Platelets: 250,000/mm³ (normal: 150,000-400,000/mm³)
- Beta-hCG: 25,000 mIU/mL (normal for gestational age: 1,080-56,500 mIU/mL)
- Prothrombin time (PT): 12 seconds (normal: 11-13.5 seconds)
- Partial thromboplastin time (PTT): 30 seconds (normal: 25-35 seconds)

IMAGING:
Bedside ultrasound reveals an empty uterus with free fluid in the pelvis.

## Encounter 2: source 2021-07-15, practice 2026-02-06, ed, Emergency Medicine

Chief complaint: Sudden onset of severe abdominal pain and vaginal bleeding

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Jennifer Liu, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe abdominal pain and vaginal bleeding

HISTORY OF PRESENT ILLNESS:
This 33-year-old female with chronic hypertension and a history of ruptured ectopic pregnancy in May 2020 presents with sudden onset of severe abdominal pain and vaginal bleeding. She reports no trauma or falls.

PAST MEDICAL HISTORY:
Active Problem List:
- Ruptured ectopic pregnancy (diagnosed 2020-05-15)
- Chronic hypertension
- History of ectopic pregnancy

PAST SURGICAL HISTORY:
- Laparoscopic left salpingectomy (ruptured ectopic pregnancy)
- Emergency low transverse cesarean section (placental abruption)

MEDICATIONS:
Home Medications:
- Labetalol 200 mg three times daily
- Prenatal vitamin with 1 mg folate 1 tablet daily

ALLERGIES:
Sulfa drugs - rash, Latex - contact dermatitis

FAMILY HISTORY:
- Mother with chronic hypertension and preeclampsia
- Maternal aunt with history of ectopic pregnancy
- Paternal grandmother with deep vein thrombosis

SOCIAL HISTORY:
She admits to smoking about half a pack of cigarettes per day throughout the pregnancy.

VITAL SIGNS:
Blood pressure is 155/100 mmHg, heart rate is 98 bpm, and temperature is 37.2°C (99.0°F).

PHYSICAL EXAMINATION:
The abdomen is tense and tender to palpation, especially over the uterus. Fetal heart tones are irregular and tachycardic. Speculum examination confirms active bleeding but no cervical dilation. The patient appears anxious and in distress.

LABORATORY DATA:
- Hemoglobin: 10.2 g/dL (normal: 12-16 g/dL)
- Platelets: 220,000/mm³ (normal: 150,000-400,000/mm³)
- Prothrombin time (PT): 12 seconds (normal: 11-13.5 seconds)
- Activated partial thromboplastin time (aPTT): 30 seconds (normal: 30-40 seconds)
- Fibrinogen: 400 mg/dL (normal: 200-400 mg/dL)

IMAGING:
Fetal ultrasound shows a retroplacental clot and decreased amniotic fluid volume.

## Encounter 3: source 2021-09-15, practice 2026-04-20, ed, Emergency Medicine

Chief complaint: Profuse watery diarrhea and vomiting for 1 day after recent travel

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Amara Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Profuse watery diarrhea and vomiting for 1 day after recent travel

HISTORY OF PRESENT ILLNESS:
This 33-year-old female with chronic hypertension, ectopic pregnancy, and placental abruption presents with watery diarrhea. She recently returned from a trip to a coastal area experiencing flooding and poor sanitation, reporting passing large volumes of watery, pale stools without blood or mucus.

PAST MEDICAL HISTORY:
Active Problem List:
- Ruptured ectopic pregnancy (diagnosed 2020-05-15)
- Placental Abruption (diagnosed 2021-07-15)
- Chronic hypertension
- History of ectopic pregnancy

PAST SURGICAL HISTORY:
- Laparoscopic left salpingectomy (ruptured ectopic pregnancy)
- Emergency low transverse cesarean section (placental abruption)

MEDICATIONS:
Home Medications:
- Labetalol 200 mg three times daily
- Prenatal vitamin with 1 mg folate 1 tablet daily

ALLERGIES:
Sulfa drugs - rash, Latex - contact dermatitis

FAMILY HISTORY:
- Mother with chronic hypertension and preeclampsia
- Maternal aunt with history of ectopic pregnancy
- Paternal grandmother with deep vein thrombosis

SOCIAL HISTORY:
Smoking: Current smoker, 0.5 packs per day. Alcohol: Social, 1-2 drinks per month. Occupation: Regional sales coordinator.

VITAL SIGNS:
Her temperature is 37°C (98.6°F), pulse is 110/min, blood pressure is 95/65 mm Hg, and respiratory rate is 16/min

PHYSICAL EXAMINATION:
She appears weak and mildly confused. Mucous membranes are dry, and skin turgor is decreased.

LABORATORY DATA:
Laboratory studies reveal elevated hematocrit and blood urea nitrogen, consistent with dehydration. Stool cultures later identify Vibrio cholerae.
