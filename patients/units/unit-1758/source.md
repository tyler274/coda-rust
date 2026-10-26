# synthetic_hospital patient 1758

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 27 at first encounter, sex F, Non-Hispanic White, Private PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-07-14 (Tue) | outpatient | Endocrinology |
| 1 | 2020-07-15 | 2021-01-12 (Tue) | outpatient | Endocrinology |
| 2 | 2021-07-15 | 2022-01-21 (Fri) | outpatient | Endocrinology |
| 3 | 2022-07-15 | 2023-01-09 (Mon) | outpatient | Obstetrics & Gynecology |
| 4 | 2023-04-15 | 2023-10-14 (Sat) | inpatient | Obstetrics & Gynecology |
| 5 | 2023-05-15 | 2023-11-16 (Thu) | inpatient | Hematology & Oncology |
| 6 | 2024-07-15 | 2025-01-06 (Mon) | ed | Emergency Medicine |
| 7 | 2025-07-15 | 2026-01-15 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Pollen (seasonal)",
  "Phenytoin (Stevens-Johnson syndrome)"
 ],
 "insurance": "Private PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "No alcohol use",
 "family_history": [
  "Mother: Hypothyroidism",
  "Father: Asthma",
  "Sister: Systemic lupus erythematosus",
  "Maternal aunt: Thyroid disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "300 mg PO daily",
   "name": "Phenytoin"
  },
  {
   "dose": "90 mcg (2 puffs) PRN every 4-6 hours",
   "name": "Albuterol HFA inhaler"
  },
  {
   "dose": "0.5 mg PO twice weekly",
   "name": "Cabergoline"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Methimazole"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Prenatal vitamins"
  }
 ],
 "surgical_history": [
  "Cesarean section"
 ],
 "chronic_conditions": [
  "Asthma",
  "Epilepsy (well-controlled)",
  "Plurihormonal pituitary macroadenoma (prolactin- and TSH-secreting)",
  "Graves disease",
  "History of acute promyelocytic leukemia (APL) with t(15;17)"
 ],
 "age_at_first_encounter": 27
}
```

Primary diagnoses: ["Graves disease", "Status asthmaticus with impending respiratory failure", "Stevens-Johnson syndrome", "Disseminated intravascular coagulation", "Prolactinoma", "Secondary hyperthyroidism due to TSH-secreting pituitary adenoma", "Pulmonary embolism"]
Comorbidities: ["Sinus tachycardia secondary to burns", "Cancer-related constitutional symptoms", "Oligomenorrhea"]

## Encounter 0: source 2020-01-15, practice 2020-07-14, outpatient, Endocrinology

Chief complaint: Persistent milky nipple discharge and irregular menstrual periods over the past 8 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent milky nipple discharge and irregular menstrual periods over the past 8 months

HISTORY OF PRESENT ILLNESS:
The discharge has been present for about 8 months and is milky in appearance, non-bloody, and occurs spontaneously. Her menses occur every 6 to 8 weeks and are lighter than they used to be.

PAST MEDICAL HISTORY:
Active Problem List:
- Asthma
- Epilepsy (well-controlled)
- Plurihormonal pituitary macroadenoma (prolactin- and TSH-secreting)
- Graves disease
- History of acute promyelocytic leukemia (APL) with t(15;17)

PAST SURGICAL HISTORY:
- Cesarean section

MEDICATIONS:
She is.

Home Medications:
- Phenytoin 300 mg PO daily
- Albuterol HFA inhaler 90 mcg (2 puffs) PRN every 4-6 hours
- Cabergoline 0.5 mg PO twice weekly
- Methimazole 10 mg PO daily
- Prenatal vitamins 1 tablet PO daily

ALLERGIES:
Pollen (seasonal), Phenytoin (Stevens-Johnson syndrome)

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Asthma
- Sister: Systemic lupus erythematosus
- Maternal aunt: Thyroid disease

SOCIAL HISTORY:
She has never been pregnant and does not smoke, drink alcohol, or use recreational drugs.

REVIEW OF SYSTEMS:
She denies recent weight changes, headaches, or visual symptoms.

PHYSICAL EXAMINATION:
On physical exam, gentle pressure on both breasts elicits milky discharge from the nipples. There are no masses, skin changes, or signs of inflammation. Visual field testing is normal.

LABORATORY DATA:
A serum pregnancy test is negative. Laboratory testing reveals elevated serum prolactin.

IMAGING:
MRI of the brain shows a 6-mm lesion in the anterior pituitary.

## Encounter 1: source 2020-07-15, practice 2021-01-12, outpatient, Endocrinology

Chief complaint: Unintentional weight loss, heat intolerance, and frequent palpitations over the past 3 months

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Unintentional weight loss, heat intolerance, and frequent palpitations over the past 3 months

HISTORY OF PRESENT ILLNESS:
She returns six months after her evaluation for galactorrhea, with unintentional weight loss, heat intolerance, and frequent palpitations over the past 3 months and menses irregular for 6 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Prolactinoma (diagnosed 2020-01-15)
- Asthma
- Epilepsy (well-controlled)
- Plurihormonal pituitary macroadenoma (prolactin- and TSH-secreting)
- Graves disease
- History of acute promyelocytic leukemia (APL) with t(15;17)

PAST SURGICAL HISTORY:
- Cesarean section

MEDICATIONS:
Home Medications:
- Phenytoin 300 mg PO daily
- Albuterol HFA inhaler 90 mcg (2 puffs) PRN every 4-6 hours
- Cabergoline 0.5 mg PO twice weekly
- Methimazole 10 mg PO daily
- Prenatal vitamins 1 tablet PO daily

ALLERGIES:
Pollen (seasonal), Phenytoin (Stevens-Johnson syndrome)

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Asthma
- Sister: Systemic lupus erythematosus
- Maternal aunt: Thyroid disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Elementary school teacher.

VITAL SIGNS:
HR is 108

PHYSICAL EXAMINATION:
Physical examination shows a diffusely enlarged, non-tender thyroid gland and a fine resting tremor in her hands. On further examination, she has mild bilateral lid lag and periorbital swelling.

LABORATORY DATA:
Thyroid-stimulating hormone (TSH) is low.

## Encounter 2: source 2021-07-15, practice 2022-01-21, outpatient, Endocrinology

Chief complaint: Several months of restlessness, irritability, and insomnia with self-directed thyroid hormone use

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Several months of restlessness, irritability, and insomnia with self-directed thyroid hormone use

HISTORY OF PRESENT ILLNESS:
The patient with a plurihormonal pituitary macroadenoma and history of Graves disease returns for follow-up. She has unintentionally lost 10 pounds over the past 3 months despite having a good appetite. Her sister reports that the patient has become obsessed with her appearance and takes triiodothyronine she purchased online. The patient denies chest pain, tremors, or recent illness.

PAST MEDICAL HISTORY:
Active Problem List:
- Prolactinoma (diagnosed 2020-01-15)
- Graves disease (diagnosed 2020-07-15)
- Asthma
- Epilepsy (well-controlled)
- Plurihormonal pituitary macroadenoma (prolactin- and TSH-secreting)
- History of acute promyelocytic leukemia (APL) with t(15;17)

PAST SURGICAL HISTORY:
- Cesarean section

MEDICATIONS:
Home Medications:
- Phenytoin 300 mg PO daily
- Albuterol HFA inhaler 90 mcg (2 puffs) PRN every 4-6 hours
- Cabergoline 0.5 mg PO twice weekly
- Methimazole 10 mg PO daily
- Prenatal vitamins 1 tablet PO daily

ALLERGIES:
Pollen (seasonal), Phenytoin (Stevens-Johnson syndrome)

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Asthma
- Sister: Systemic lupus erythematosus
- Maternal aunt: Thyroid disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Elementary school teacher.

VITAL SIGNS:
Physical exam reveals a heart rate of 106/min and blood pressure of 132/76 mm Hg.

PHYSICAL EXAMINATION:
She appears anxious but is alert and cooperative. The thyroid gland is non-palpable with no tenderness or enlargement. There is no lid lag, proptosis, or pretibial edema.

PLAN:
Laboratory studies and a radioactive iodine uptake (RAIU) scan are ordered.

## Encounter 3: source 2022-07-15, practice 2023-01-09, outpatient, Obstetrics & Gynecology

Chief complaint: Persistent fatigue, anxiety, and difficulty sleeping during pregnancy with weight loss despite increased appetite

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Aisha Patel, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue, anxiety, and difficulty sleeping during pregnancy with weight loss despite increased appetite

HISTORY OF PRESENT ILLNESS:
The patient returns for obstetric care with a history of plurihormonal pituitary macroadenoma and Graves disease, previously managed by Endocrinology. She reports feeling her heart racing frequently, experiencing excessive sweating, and having frequent loose stools. Despite eating more than usual, she has lost 5 kg over this period. Her pregnancy has otherwise been uncomplicated. She denies chest pain, shortness of breath, or visual changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Prolactinoma (diagnosed 2020-01-15)
- Graves disease (diagnosed 2020-07-15)
- Secondary hyperthyroidism due to TSH-secreting pituitary adenoma (diagnosed 2021-07-15)
- Asthma
- Epilepsy (well-controlled)
- Plurihormonal pituitary macroadenoma (prolactin- and TSH-secreting)
- History of acute promyelocytic leukemia (APL) with t(15;17)

PAST SURGICAL HISTORY:
- Cesarean section

MEDICATIONS:
She is taking prenatal vitamins regularly.

Home Medications:
- Phenytoin 300 mg PO daily
- Albuterol HFA inhaler 90 mcg (2 puffs) PRN every 4-6 hours
- Cabergoline 0.5 mg PO twice weekly
- Methimazole 10 mg PO daily

ALLERGIES:
Pollen (seasonal), Phenytoin (Stevens-Johnson syndrome)

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Asthma
- Sister: Systemic lupus erythematosus
- Maternal aunt: Thyroid disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Elementary school teacher.

VITAL SIGNS:
On examination, her pulse is 116/min, blood pressure is 122/76 mmHg, and BMI is 18 kg/m².

PHYSICAL EXAMINATION:
She appears anxious and restless. Her skin is warm and moist, and a fine tremor is noted in both hands. Thyroid examination reveals a diffusely enlarged, non-tender gland without nodules. Fetal heart tones are normal, and there is no lower extremity edema.

LABORATORY DATA:
Laboratory studies show:
TSH <0.01 μU/mL
Free T4 4.5 ng/dL
Total T3 270 ng/dL

OTHER STUDIES:
ECG shows sinus tachycardia.

PLAN:
She is started on appropriate pharmacologic therapy for her condition.

## Encounter 4: source 2023-04-15, practice 2023-10-14, inpatient, Obstetrics & Gynecology

Chief complaint: Sudden-onset shortness of breath and pleuritic chest pain on postoperative day 2 following Cesarean section

ENCOUNTER NOTE
Date: 2023-04-15
Provider: Dr. Aisha Patel, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Sudden-onset shortness of breath and pleuritic chest pain on postoperative day 2 following Cesarean section

HISTORY OF PRESENT ILLNESS:
The patient is a 30-year-old female with a plurihormonal pituitary macroadenoma and Graves disease, with hyperthyroidism in pregnancy, and presents on postoperative day 2 following uncomplicated Cesarean section. She develops sudden-onset shortness of breath and pleuritic chest pain upon standing to void.

PAST MEDICAL HISTORY:
Active Problem List:
- Prolactinoma (diagnosed 2020-01-15)
- Graves disease (diagnosed 2020-07-15)
- Secondary hyperthyroidism due to TSH-secreting pituitary adenoma (diagnosed 2021-07-15)
- Asthma
- Epilepsy (well-controlled)
- Plurihormonal pituitary macroadenoma (prolactin- and TSH-secreting)
- History of acute promyelocytic leukemia (APL) with t(15;17)

PAST SURGICAL HISTORY:
- Cesarean section

MEDICATIONS:
Home Medications:
- Phenytoin 300 mg PO daily
- Albuterol HFA inhaler 90 mcg (2 puffs) PRN every 4-6 hours
- Cabergoline 0.5 mg PO twice weekly
- Methimazole 10 mg PO daily
- Prenatal vitamins 1 tablet PO daily

ALLERGIES:
Pollen (seasonal), Phenytoin (Stevens-Johnson syndrome)

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Asthma
- Sister: Systemic lupus erythematosus
- Maternal aunt: Thyroid disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Elementary school teacher.

VITAL SIGNS:
Temp 98.6 F, BP 120/78, HR 115, RR 26

LABORATORY DATA:
Arterial blood gas analysis shows pH 7.48, pCO 2 30 mm Hg, and pO 2 70 mm Hg on room air

IMAGING:
A ventilation-perfusion scan and spiral CT of the chest confirm the diagnosis

## Encounter 5: source 2023-05-15, practice 2023-11-16, inpatient, Hematology & Oncology

Chief complaint: Persistent oozing of blood from IV line and central venous catheter insertion sites during admission

ENCOUNTER NOTE
Date: 2023-05-15
Provider: Dr. James Wilson, MD
Department: Hematology & Oncology
Visit Type: inpatient

CHIEF COMPLAINT:
Persistent oozing of blood from IV line and central venous catheter insertion sites during admission

HISTORY OF PRESENT ILLNESS:
This patient with acute promyelocytic leukemia and recent postpartum PE presents with bleeding. During her current admission, the nurses note repeated blood-staining of her dressings and slow, continuous bleeding from venipuncture sites. She denies any trauma or recent surgeries.

PAST MEDICAL HISTORY:
Active Problem List:
- Prolactinoma (diagnosed 2020-01-15)
- Graves disease (diagnosed 2020-07-15)
- Secondary hyperthyroidism due to TSH-secreting pituitary adenoma (diagnosed 2021-07-15)
- Pulmonary embolism (diagnosed 2023-04-15)
- Asthma
- Epilepsy (well-controlled)
- Plurihormonal pituitary macroadenoma (prolactin- and TSH-secreting)
- History of acute promyelocytic leukemia (APL) with t(15;17)

She was recently hospitalized and underwent induction chemotherapy for acute leukemia.

PAST SURGICAL HISTORY:
- Cesarean section

MEDICATIONS:
Home Medications:
- Phenytoin 300 mg PO daily
- Albuterol HFA inhaler 90 mcg (2 puffs) PRN every 4-6 hours
- Cabergoline 0.5 mg PO twice weekly
- Methimazole 10 mg PO daily
- Prenatal vitamins 1 tablet PO daily

ALLERGIES:
Pollen (seasonal), Phenytoin (Stevens-Johnson syndrome)

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Asthma
- Sister: Systemic lupus erythematosus
- Maternal aunt: Thyroid disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Elementary school teacher.

VITAL SIGNS:
Mildly tachycardic

PHYSICAL EXAMINATION:
On examination, she is alert but appears pale. Petechiae are noted on her lower extremities, and there is mild bruising on her arms. She has no active external hemorrhage, but blood continues to ooze from her central line. There is no hepatosplenomegaly.

LABORATORY DATA:
A peripheral blood smear is sent and is pending. Basic coagulation studies are ordered.

## Encounter 6: source 2024-07-15, practice 2025-01-06, ed, Emergency Medicine

Chief complaint: Painful oral ulcers and worsening rash with eye redness and pain over the past 3 days

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Lisa Wong, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Painful oral ulcers and worsening rash with eye redness and pain over the past 3 days

HISTORY OF PRESENT ILLNESS:
This 31-year-old with epilepsy, plurihormonal pituitary macroadenoma, Graves disease, and APL with recent DIC presents for evaluation. She reports that the rash began on her trunk and has spread to her face and extremities, with some areas developing blisters that have ruptured. She also complains of eye redness and pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Prolactinoma (diagnosed 2020-01-15)
- Graves disease (diagnosed 2020-07-15)
- Secondary hyperthyroidism due to TSH-secreting pituitary adenoma (diagnosed 2021-07-15)
- Pulmonary embolism (diagnosed 2023-04-15)
- Disseminated intravascular coagulation (diagnosed 2023-05-15)
- Asthma
- Epilepsy (well-controlled)
- Plurihormonal pituitary macroadenoma (prolactin- and TSH-secreting)
- History of acute promyelocytic leukemia (APL) with t(15;17)

She has a history of epilepsy, well-controlled.

PAST SURGICAL HISTORY:
- Cesarean section

MEDICATIONS:
She takes on phenytoin for the past two years.

Home Medications:
- Albuterol HFA inhaler 90 mcg (2 puffs) PRN every 4-6 hours
- Cabergoline 0.5 mg PO twice weekly
- Methimazole 10 mg PO daily
- Prenatal vitamins 1 tablet PO daily

ALLERGIES:
Pollen (seasonal), Phenytoin (Stevens-Johnson syndrome)

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Asthma
- Sister: Systemic lupus erythematosus
- Maternal aunt: Thyroid disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Elementary school teacher.

VITAL SIGNS:
Her temperature is 38.9°C (102°F), blood pressure is 100/60 mmHg, pulse is 110/min, and respiratory rate is 22/min.

PHYSICAL EXAMINATION:
Physical examination reveals erythematous macules and atypical targetoid lesions with central dusky purpura and blistering on her trunk, face, and extremities. Nikolsky sign is positive. Mucosal involvement is noted with hemorrhagic crusting of the lips, oral mucosal erosions, and conjunctival injection.

LABORATORY DATA:
Laboratory studies show:

- White blood cell count: 15,000/mm³ (normal 4,500-11,000/mm³)
- Hemoglobin: 13.2 g/dL (normal 12-16 g/dL)
- Platelet count: 250,000/mm³ (normal 150,000-450,000/mm³)
- Alanine aminotransferase (ALT): 45 U/L (normal 7-56 U/L)
- Aspartate aminotransferase (AST): 50 U/L (normal 8-48 U/L)
- Blood urea nitrogen (BUN): 18 mg/dL (normal 7-20 mg/dL)
- Creatinine: 1.0 mg/dL (normal 0.6-1.1 mg/dL)

## Encounter 7: source 2025-07-15, practice 2026-01-15, ed, Emergency Medicine

Chief complaint: Shortness of breath and wheezing for 2 hours after pollen exposure with poor response to rescue inhaler

ENCOUNTER NOTE
Date: 2025-07-15
Provider: Dr. Robert Martinez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Shortness of breath and wheezing for 2 hours after pollen exposure with poor response to rescue inhaler

HISTORY OF PRESENT ILLNESS:
This 32-year-old female with asthma, epilepsy, plurihormonal pituitary macroadenoma, Graves disease, and acute promyelocytic leukemia in remission presents to the emergency department with a 2-hour history of shortness of breath and wheezing. She reports that her symptoms began after exposure to pollen during a morning walk. She used her albuterol inhaler twice without significant relief.

PAST MEDICAL HISTORY:
Active Problem List:
- Prolactinoma (diagnosed 2020-01-15)
- Graves disease (diagnosed 2020-07-15)
- Secondary hyperthyroidism due to TSH-secreting pituitary adenoma (diagnosed 2021-07-15)
- Pulmonary embolism (diagnosed 2023-04-15)
- Disseminated intravascular coagulation (diagnosed 2023-05-15)
- Stevens-Johnson syndrome (diagnosed 2024-07-15)
- Asthma
- Epilepsy (well-controlled)
- Plurihormonal pituitary macroadenoma (prolactin- and TSH-secreting)
- History of acute promyelocytic leukemia (APL) with t(15;17)

She has a past medical history of asthma.

PAST SURGICAL HISTORY:
- Cesarean section

MEDICATIONS:
Home Medications:
- Phenytoin 300 mg PO daily
- Albuterol HFA inhaler 90 mcg (2 puffs) PRN every 4-6 hours
- Cabergoline 0.5 mg PO twice weekly
- Methimazole 10 mg PO daily
- Prenatal vitamins 1 tablet PO daily

ALLERGIES:
Pollen (seasonal), Phenytoin (Stevens-Johnson syndrome)

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Asthma
- Sister: Systemic lupus erythematosus
- Maternal aunt: Thyroid disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Elementary school teacher.

REVIEW OF SYSTEMS:
She denies fever, chills, or recent illness

VITAL SIGNS:
Her oxygen saturation is 92% on room air. Her temperature is 37.1°C, blood pressure is 130/75 mmHg, and pulse is 110 beats per minute.

PHYSICAL EXAMINATION:
On examination, she is anxious, tachypneic with a respiratory rate of 28 breaths per minute, and using accessory muscles to breathe. Auscultation of the chest reveals diffuse expiratory wheezes.

LABORATORY DATA:
Arterial blood gas shows a pH of 7.45, PaCO2 of 32 mmHg, PaO2 of 75 mmHg, and HCO3- of 24 mEq/L.

Laboratory Values:
- pH: 7.45
- PaCO2: 32 mmHg
- PaO2: 75 mmHg
- HCO3-: 24 mEq/L
