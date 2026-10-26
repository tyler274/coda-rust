# synthetic_hospital patient 1672

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Caucasian, Blue Cross PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-10-30 (Mon) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-02-15 | 2023-11-24 (Fri) | outpatient | Cardiology |
| 2 | 2020-04-15 | 2024-01-29 (Mon) | ed | Emergency Medicine |
| 3 | 2020-06-15 | 2024-03-22 (Fri) | outpatient | Endocrinology |
| 4 | 2020-12-15 | 2024-09-23 (Mon) | outpatient | Obstetrics & Gynecology |
| 5 | 2021-03-15 | 2024-12-18 (Wed) | ed | Emergency Medicine |
| 6 | 2021-05-15 | 2025-03-03 (Mon) | outpatient | Obstetrics & Gynecology |
| 7 | 2022-04-15 | 2026-01-26 (Mon) | outpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Sulfa drugs",
  "Latex"
 ],
 "insurance": "Blue Cross PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "None",
 "family_history": [
  "Mother with type 2 diabetes mellitus",
  "Maternal aunt with autoimmune thyroid disease",
  "Paternal grandmother with dilated cardiomyopathy"
 ],
 "race_ethnicity": "Caucasian",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg PO twice daily",
   "name": "Enalapril"
  },
  {
   "dose": "6.25 mg PO twice daily",
   "name": "Carvedilol"
  },
  {
   "dose": "500 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Sertraline"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Prenatal vitamin"
  }
 ],
 "surgical_history": [
  "Cesarean section (secondary to vasa previa with twin gestation)",
  "Dilation and curettage"
 ],
 "chronic_conditions": [
  "History of peripartum cardiomyopathy",
  "Impaired glucose tolerance",
  "History of gestational diabetes mellitus",
  "History of postpartum thyroiditis",
  "Generalized anxiety disorder"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Peripartum cardiomyopathy", "Dichorionic diamniotic twin pregnancy", "Postpartum thyroiditis, hyperthyroid phase", "Impaired glucose tolerance", "Exclusive breastfeeding", "Term Pregnancy Requiring Delivery", "Vasa previa"]
Comorbidities: ["Multiparity", "Personal History of Cholelithiasis", "Exclusive breastfeeding", "Primiparity"]

## Encounter 0: source 2020-01-15, practice 2023-10-30, outpatient, Obstetrics & Gynecology

Chief complaint: Persistent vaginal bleeding 4 weeks after spontaneous vaginal delivery

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent vaginal bleeding 4 weeks after spontaneous vaginal delivery

HISTORY OF PRESENT ILLNESS:
She reports persistent vaginal bleeding since delivery, which she describes as initially heavy and red, but now lighter and brownish. She denies any fever, abdominal pain, or foul-smelling discharge.

PAST MEDICAL HISTORY:
Active Problem List:
- History of peripartum cardiomyopathy
- Impaired glucose tolerance
- History of gestational diabetes mellitus
- History of postpartum thyroiditis
- Generalized anxiety disorder

She had an uncomplicated prenatal course.

PAST SURGICAL HISTORY:
- Cesarean section (secondary to vasa previa with twin gestation)
- Dilation and curettage

MEDICATIONS:
Home Medications:
- Enalapril 10 mg PO twice daily
- Carvedilol 6.25 mg PO twice daily
- Metformin 500 mg PO twice daily
- Sertraline 50 mg PO daily
- Prenatal vitamin 1 tablet PO daily

ALLERGIES:
Sulfa drugs, Latex

FAMILY HISTORY:
- Mother with type 2 diabetes mellitus
- Maternal aunt with autoimmune thyroid disease
- Paternal grandmother with dilated cardiomyopathy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, her vital signs are stable, and abdominal examination reveals a non-tender, soft abdomen with a well-healed episiotomy site. There is no evidence of genital tract trauma.

LABORATORY DATA:
Her complete blood count is within normal limits.

## Encounter 1: source 2020-02-15, practice 2023-11-24, outpatient, Cardiology

Chief complaint: Gradually worsening shortness of breath since final weeks of pregnancy

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. Michael Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Gradually worsening shortness of breath since final weeks of pregnancy

HISTORY OF PRESENT ILLNESS:
The patient presents to the cardiology clinic six weeks after delivering a healthy baby via spontaneous vaginal delivery at term, returning for evaluation after her obstetrics visit four weeks prior for postpartum bleeding. She has a history of peripartum cardiomyopathy, impaired glucose tolerance, and postpartum thyroiditis, and reports gradually worsening shortness of breath which started during the final weeks of her pregnancy and has continued to worsen since childbirth. She now feels winded after climbing a single flight of stairs and notes that lying flat causes discomfort. Her ankles appear swollen by the end of the day. She has also noticed increasing fatigue and mild difficulty sleeping.

PAST MEDICAL HISTORY:
Active Problem List:
- Exclusive breastfeeding (diagnosed 2020-01-15)
- History of peripartum cardiomyopathy
- Impaired glucose tolerance
- History of gestational diabetes mellitus
- History of postpartum thyroiditis
- Generalized anxiety disorder

This was her second pregnancy, and her previous pregnancy was complicated by similar but milder symptoms that resolved postpartum.

PAST SURGICAL HISTORY:
- Cesarean section (secondary to vasa previa with twin gestation)
- Dilation and curettage

MEDICATIONS:
Home Medications:
- Enalapril 10 mg PO twice daily
- Carvedilol 6.25 mg PO twice daily
- Metformin 500 mg PO twice daily
- Sertraline 50 mg PO daily
- Prenatal vitamin 1 tablet PO daily

ALLERGIES:
Sulfa drugs, Latex

FAMILY HISTORY:
- Mother with type 2 diabetes mellitus
- Maternal aunt with autoimmune thyroid disease
- Paternal grandmother with dilated cardiomyopathy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

REVIEW OF SYSTEMS:
There is no chest pain, palpitations, fever, or recent travel.

VITAL SIGNS:
On physical examination, her blood pressure is 112/76 mm Hg, pulse is 92/min, and respiratory rate is 18/min. Her BMI is 25 kg/m².

PHYSICAL EXAMINATION:
She appears tired but not in acute distress. Cardiopulmonary exam reveals bilateral basal crackles and mild jugular venous distension. No murmurs or gallops are heard. She has 2+ bilateral lower extremity pitting edema. Lungs are clear to percussion, and no wheezing is present.

LABORATORY DATA:
Pregnancy test is negative.

## Encounter 2: source 2020-04-15, practice 2024-01-29, ed, Emergency Medicine

Chief complaint: Progressive shortness of breath, orthopnea, and bilateral leg swelling

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Fatima Al-Hassan, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive shortness of breath, orthopnea, and bilateral leg swelling

HISTORY OF PRESENT ILLNESS:
The patient presents to the emergency department with worsening symptoms in the context of her known peripartum cardiomyopathy, previously evaluated by cardiology two months prior. She presents with a 2-week history of progressive shortness of breath, orthopnea, and bilateral leg swelling. She gave birth to her first child 3 months ago via normal vaginal delivery.

PAST MEDICAL HISTORY:
Active Problem List:
- Exclusive breastfeeding (diagnosed 2020-01-15)
- Peripartum cardiomyopathy (diagnosed 2020-02-15)
- Impaired glucose tolerance
- History of gestational diabetes mellitus
- History of postpartum thyroiditis
- Generalized anxiety disorder

PAST SURGICAL HISTORY:
- Cesarean section (secondary to vasa previa with twin gestation)
- Dilation and curettage

MEDICATIONS:
She took prenatal vitamins during her pregnancy.

Home Medications:
- Enalapril 10 mg PO twice daily
- Carvedilol 6.25 mg PO twice daily
- Metformin 500 mg PO twice daily
- Sertraline 50 mg PO daily

ALLERGIES:
Sulfa drugs, Latex

FAMILY HISTORY:
- Mother with type 2 diabetes mellitus
- Maternal aunt with autoimmune thyroid disease
- Paternal grandmother with dilated cardiomyopathy

SOCIAL HISTORY:
She denies alcohol, tobacco, or drug use.

VITAL SIGNS:
On examination, her blood pressure is 110/70 mmHg, heart rate is 102 bpm, respiratory rate is 22 breaths/min, and oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
Auscultation of the lungs reveals bilateral rales at the lung bases. Cardiac examination shows an S3 gallop, and there is 2+ pitting edema in both lower extremities up to the mid-shins.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin: 12.5 g/dL
- White blood cell count: 7,500/mm^3
- Platelets: 250,000/mm^3
- Serum sodium: 138 mEq/L
- Serum potassium: 4.2 mEq/L
- Serum creatinine: 0.9 mg/dL
- B-type natriuretic peptide (BNP): 900 pg/mL
- Troponin I: <0.04 ng/mL

IMAGING:
A chest X-ray shows cardiomegaly and pulmonary congestion. An echocardiogram reveals a left ventricular ejection fraction of 35% with global hypokinesis and no valvular abnormalities.

OTHER STUDIES:
An electrocardiogram shows sinus tachycardia with no ST changes or Q waves.

## Encounter 3: source 2020-06-15, practice 2024-03-22, outpatient, Endocrinology

Chief complaint: Palpitations, irritability, and 10-pound weight loss despite increased appetite

ENCOUNTER NOTE
Date: 2020-06-15
Provider: Dr. Robert Taylor, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Palpitations, irritability, and 10-pound weight loss despite increased appetite

HISTORY OF PRESENT ILLNESS:
Patient with peripartum cardiomyopathy, seen in April, presents to the clinic 5 months postpartum. She denies any pain or discomfort in her neck.

PAST MEDICAL HISTORY:
Active Problem List:
- Exclusive breastfeeding (diagnosed 2020-01-15)
- Peripartum cardiomyopathy (diagnosed 2020-02-15)
- Impaired glucose tolerance
- History of gestational diabetes mellitus
- History of postpartum thyroiditis
- Generalized anxiety disorder

She had an uncomplicated pregnancy and vaginal delivery.

PAST SURGICAL HISTORY:
- Cesarean section (secondary to vasa previa with twin gestation)
- Dilation and curettage

MEDICATIONS:
Home Medications:
- Enalapril 10 mg PO twice daily
- Carvedilol 6.25 mg PO twice daily
- Metformin 500 mg PO twice daily
- Sertraline 50 mg PO daily
- Prenatal vitamin 1 tablet PO daily

ALLERGIES:
Sulfa drugs, Latex

FAMILY HISTORY:
- Mother with type 2 diabetes mellitus
- Maternal aunt with autoimmune thyroid disease
- Paternal grandmother with dilated cardiomyopathy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

VITAL SIGNS:
On examination, her heart rate is 102 bpm

PHYSICAL EXAMINATION:
She has a fine tremor in her outstretched hands. There is no goiter or thyroid nodules palpable. Her skin is warm and moist.

LABORATORY DATA:
Laboratory tests reveal the following:

- Thyroid-stimulating hormone (TSH): <0.01 mIU/L (normal range: 0.4-4.0 mIU/L)
- Free thyroxine (T4): 2.5 ng/dL (normal range: 0.9-1.7 ng/dL)
- Total triiodothyronine (T3): 200 ng/dL (normal range: 80-180 ng/dL)

IMAGING:
A radioactive iodine uptake scan shows low uptake.

## Encounter 4: source 2020-12-15, practice 2024-09-23, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit at 18 weeks of gestation with twin pregnancy

ENCOUNTER NOTE
Date: 2020-12-15
Provider: Dr. Elena Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit at 18 weeks of gestation with twin pregnancy

HISTORY OF PRESENT ILLNESS:
The patient is a 32-year-old female with a history of peripartum cardiomyopathy and postpartum thyroiditis following her delivery earlier this year who returns to the obstetrics clinic for routine prenatal care at 18 weeks gestation with a twin pregnancy. She presents to the obstetrics clinic. She reports feeling well with no complaints.

PAST MEDICAL HISTORY:
Active Problem List:
- Exclusive breastfeeding (diagnosed 2020-01-15)
- Peripartum cardiomyopathy (diagnosed 2020-02-15)
- Postpartum thyroiditis, hyperthyroid phase (diagnosed 2020-06-15)
- Impaired glucose tolerance
- History of gestational diabetes mellitus
- History of postpartum thyroiditis
- Generalized anxiety disorder

Her previous pregnancy was uncomplicated, and she delivered a healthy full-term infant.

PAST SURGICAL HISTORY:
- Cesarean section (secondary to vasa previa with twin gestation)
- Dilation and curettage

MEDICATIONS:
Home Medications:
- Enalapril 10 mg PO twice daily
- Carvedilol 6.25 mg PO twice daily
- Metformin 500 mg PO twice daily
- Sertraline 50 mg PO daily
- Prenatal vitamin 1 tablet PO daily

ALLERGIES:
Sulfa drugs, Latex

FAMILY HISTORY:
- Mother with type 2 diabetes mellitus
- Maternal aunt with autoimmune thyroid disease
- Paternal grandmother with dilated cardiomyopathy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

VITAL SIGNS:
On examination, her blood pressure is 120/78 mmHg, heart rate is 72 beats per minute,

PHYSICAL EXAMINATION:
Her abdomen is consistent with her gestational age.

IMAGING:
A fetal ultrasound is performed, which reveals two intrauterine gestations with two separate placentas and two amniotic sacs. The fetal heart rates are 150 and 148 beats per minute, respectively, and both fetuses have normal anatomy.

## Encounter 5: source 2021-03-15, practice 2024-12-18, ed, Emergency Medicine

Chief complaint: Sudden onset of painless vaginal bleeding after feeling a pop in her abdomen

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of painless vaginal bleeding after feeling a pop in her abdomen

HISTORY OF PRESENT ILLNESS:
The patient, with a history of peripartum cardiomyopathy and postpartum thyroiditis after her first pregnancy and current twin gestation, presents with sudden onset of painless vaginal bleeding. She denies any trauma. Her first pregnancy was uncomplicated, and she had a spontaneous vaginal delivery at term. Her prenatal course was unremarkable until now, and her prenatal ultrasounds have not shown any abnormalities. However, the bleeding is continuous and seems to be increasing in volume. The patient reports feeling the baby move during the examination.

PAST MEDICAL HISTORY:
Active Problem List:
- Exclusive breastfeeding (diagnosed 2020-01-15)
- Peripartum cardiomyopathy (diagnosed 2020-02-15)
- Postpartum thyroiditis, hyperthyroid phase (diagnosed 2020-06-15)
- Dichorionic diamniotic twin pregnancy (diagnosed 2020-12-15)
- Impaired glucose tolerance
- History of gestational diabetes mellitus
- History of postpartum thyroiditis
- Generalized anxiety disorder

PAST SURGICAL HISTORY:
- Cesarean section (secondary to vasa previa with twin gestation)
- Dilation and curettage

MEDICATIONS:
Home Medications:
- Enalapril 10 mg PO twice daily
- Carvedilol 6.25 mg PO twice daily
- Metformin 500 mg PO twice daily
- Sertraline 50 mg PO daily
- Prenatal vitamin 1 tablet PO daily

ALLERGIES:
Sulfa drugs, Latex

FAMILY HISTORY:
- Mother with type 2 diabetes mellitus
- Maternal aunt with autoimmune thyroid disease
- Paternal grandmother with dilated cardiomyopathy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

VITAL SIGNS:
On examination, her vital signs are stable, and the fetal heart rate tracing is reassuring.

PHYSICAL EXAMINATION:
Speculum examination reveals blood coming from the cervical os but no active bleeding from the vaginal walls or cervix.

LABORATORY DATA:
Laboratory values:
- Hemoglobin: 12.2 g/dL
- Platelets: 250 x 10^3/uL
- Prothrombin time (PT): 12 seconds
- Activated partial thromboplastin time (aPTT): 30 seconds

## Encounter 6: source 2021-05-15, practice 2025-03-03, outpatient, Obstetrics & Gynecology

Chief complaint: Postpartum check-up 8 weeks after delivering second child with increased thirst and frequent urination

ENCOUNTER NOTE
Date: 2021-05-15
Provider: Dr. Elena Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Postpartum check-up 8 weeks after delivering second child with increased thirst and frequent urination

HISTORY OF PRESENT ILLNESS:
Patient with history of peripartum cardiomyopathy and gestational diabetes returns for postpartum follow-up, reporting feeling generally well but mentions increased thirst and frequent urination. She notes uncomplicated pregnancy and delivery and currently breastfeeds.

PAST MEDICAL HISTORY:
Active Problem List:
- Exclusive breastfeeding (diagnosed 2020-01-15)
- Peripartum cardiomyopathy (diagnosed 2020-02-15)
- Postpartum thyroiditis, hyperthyroid phase (diagnosed 2020-06-15)
- Dichorionic diamniotic twin pregnancy (diagnosed 2020-12-15)
- Vasa previa (diagnosed 2021-03-15)
- Impaired glucose tolerance
- History of gestational diabetes mellitus
- History of postpartum thyroiditis
- Generalized anxiety disorder

She has a past medical history of gestational diabetes mellitus, which was diet-controlled during her recent pregnancy.

PAST SURGICAL HISTORY:
- Cesarean section (secondary to vasa previa with twin gestation)
- Dilation and curettage

MEDICATIONS:
Home Medications:
- Enalapril 10 mg PO twice daily
- Carvedilol 6.25 mg PO twice daily
- Metformin 500 mg PO twice daily
- Sertraline 50 mg PO daily
- Prenatal vitamin 1 tablet PO daily

ALLERGIES:
Sulfa drugs, Latex

FAMILY HISTORY:
- Mother with type 2 diabetes mellitus
- Maternal aunt with autoimmune thyroid disease
- Paternal grandmother with dilated cardiomyopathy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

REVIEW OF SYSTEMS:
She denies any headaches, vision changes, or abdominal pain.

VITAL SIGNS:
Her vital signs are within normal limits

PHYSICAL EXAMINATION:
Physical examination is unremarkable

LABORATORY DATA:
Her postpartum screening includes a 2-hour (75-g) oral glucose tolerance test (OGTT) with the following results:

- Fasting plasma glucose: 88 mg/dL (Normal: <92 mg/dL)
- 1-hour glucose: 184 mg/dL (Normal: <180 mg/dL)
- 2-hour glucose: 158 mg/dL (Normal: <153 mg/dL)

## Encounter 7: source 2022-04-15, practice 2026-01-26, outpatient, Obstetrics & Gynecology

Chief complaint: Request for elective cesarean delivery due to significant anxiety about labor and vaginal delivery

ENCOUNTER NOTE
Date: 2022-04-15
Provider: Dr. Priya Natarajan, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Request for elective cesarean delivery due to significant anxiety about labor and vaginal delivery

HISTORY OF PRESENT ILLNESS:
The patient is a 34-year-old female with a history of peripartum cardiomyopathy following her first delivery and vasa previa in her prior twin gestation who presents requesting elective cesarean delivery for her current pregnancy. She carries diagnoses of generalized anxiety disorder, impaired glucose tolerance, and prior postpartum thyroiditis. She reports a previous uncomplicated vaginal delivery but has developed significant anxiety about the process of labor and vaginal delivery for her current pregnancy. She denies any current vaginal bleeding, rupture of membranes, or contractions. Her prenatal course has been unremarkable, with normal glucose tolerance testing and no evidence of gestational hypertension. Her first child was born at term with a birth weight of 3500 grams without complications. The patient is adamant about her decision despite counseling on the risks and benefits of cesarean versus vaginal delivery. She requests that you perform the cesarean delivery.

PAST MEDICAL HISTORY:
Active Problem List:
- Exclusive breastfeeding (diagnosed 2020-01-15)
- Peripartum cardiomyopathy (diagnosed 2020-02-15)
- Postpartum thyroiditis, hyperthyroid phase (diagnosed 2020-06-15)
- Dichorionic diamniotic twin pregnancy (diagnosed 2020-12-15)
- Vasa previa (diagnosed 2021-03-15)
- Impaired glucose tolerance (diagnosed 2021-05-15)
- History of gestational diabetes mellitus
- History of postpartum thyroiditis
- Generalized anxiety disorder

PAST SURGICAL HISTORY:
- Cesarean section (secondary to vasa previa with twin gestation)
- Dilation and curettage

MEDICATIONS:
She takes her only medication is a prenatal vitamin.

Home Medications:
- Enalapril 10 mg PO twice daily
- Carvedilol 6.25 mg PO twice daily
- Metformin 500 mg PO twice daily
- Sertraline 50 mg PO daily

ALLERGIES:
She has no known allergies

FAMILY HISTORY:
- Mother with type 2 diabetes mellitus
- Maternal aunt with autoimmune thyroid disease
- Paternal grandmother with dilated cardiomyopathy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

VITAL SIGNS:
On examination, her blood pressure is 120/78 mmHg, heart rate is 82 beats per minute

PHYSICAL EXAMINATION:
Fetal heart tracing is reassuring with no decelerations.

ASSESSMENT AND PLAN:
You are uncomfortable performing a cesarean delivery without a clear medical or obstetric indication.
