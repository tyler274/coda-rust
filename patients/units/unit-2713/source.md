# synthetic_hospital patient 2713

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-04-26 (Mon) | outpatient | Neurology |
| 1 | 2020-02-15 | 2021-05-31 (Mon) | follow_up | General Surgery |
| 2 | 2020-07-15 | 2021-10-22 (Fri) | ed | Emergency Medicine |
| 3 | 2021-07-15 | 2022-10-14 (Fri) | outpatient | Otolaryngology |
| 4 | 2022-07-15 | 2023-10-30 (Mon) | outpatient | Endocrinology |
| 5 | 2024-01-15 | 2025-04-23 (Wed) | ed | Emergency Medicine |
| 6 | 2024-07-15 | 2025-10-19 (Sun) | ed | Emergency Medicine |
| 7 | 2025-01-15 | 2026-04-20 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies",
  "No known food allergies"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Office administrator",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Mother: Hereditary angioedema (confirmed C1 inhibitor deficiency)",
  "Father: Hypertension, cerebrovascular accident at age 60"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "25 mg daily",
   "name": "Hydrochlorothiazide"
  }
 ],
 "surgical_history": [
  "Open cholecystectomy (age 45)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hereditary angioedema (C1 inhibitor deficiency)",
  "Cervical degenerative disc disease with C6-C7 herniation"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Atrial flutter", "Pheochromocytoma", "Acoustic neuroma (Vestibular schwannoma)", "Hereditary Angioedema (C1 Inhibitor Deficiency)", "Cervical radiculopathy due to C6-C7 intervertebral disc herniation", "Weber syndrome (Midbrain infarction)", "Drug-induced acute pancreatitis", "Postoperative incisional pain"]
Comorbidities: ["Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2021-04-26, outpatient, Neurology

Chief complaint: Neck pain radiating down her left arm with associated numbness in the thumb and index finger

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Neck pain radiating down her left arm with associated numbness in the thumb and index finger

HISTORY OF PRESENT ILLNESS:
A 45-year-old female presents to the clinic with a 6-week history of neck pain radiating down her left arm. She describes the pain as a sharp, burning sensation that worsens with neck movement. She also reports numbness in her left thumb and index finger.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hereditary angioedema (C1 inhibitor deficiency)
- Cervical degenerative disc disease with C6-C7 herniation

Her medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Open cholecystectomy (age 45)

MEDICATIONS:
Hydrochlorothiazide

ALLERGIES:
No known drug allergies, No known food allergies

FAMILY HISTORY:
- Mother: Hereditary angioedema (confirmed C1 inhibitor deficiency)
- Father: Hypertension, cerebrovascular accident at age 60

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Office administrator.

PHYSICAL EXAMINATION:
On physical examination, she has weakness in her left biceps and a diminished brachioradialis reflex.

IMAGING:
Cervical spine MRI shows a C6-C7 disk herniation.

## Encounter 1: source 2020-02-15, practice 2021-05-31, follow_up, General Surgery

Chief complaint: Constant dull incisional pain 5 days after open cholecystectomy unrelieved by acetaminophen

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. Michael Rodriguez, MD
Department: General Surgery
Visit Type: follow_up

CHIEF COMPLAINT:
Constant dull incisional pain 5 days after open cholecystectomy unrelieved by acetaminophen

HISTORY OF PRESENT ILLNESS:
The patient with hereditary angioedema and cervical degenerative disc disease returns 5 days postoperative from an open cholecystectomy following recent radiculopathy evaluation. She describes the pain as a constant, dull ache that has not improved with over-the-counter acetaminophen. She denies fever, nausea, vomiting, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Cervical radiculopathy due to C6-C7 intervertebral disc herniation (diagnosed 2020-01-15)
- Hypertension
- Hereditary angioedema (C1 inhibitor deficiency)
- Cervical degenerative disc disease with C6-C7 herniation

Her past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Open cholecystectomy (age 45)

MEDICATIONS:
She takes controlled with lisinopril.

Home Medications:
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Hereditary angioedema (confirmed C1 inhibitor deficiency)
- Father: Hypertension, cerebrovascular accident at age 60

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Office administrator.

VITAL SIGNS:
Her vital signs are stable

PHYSICAL EXAMINATION:
The rest of the physical examination is unremarkable. On examination, the incision site is clean, well-approximated, and without erythema, warmth, or discharge. There is mild tenderness to palpation, but no fluctuance or crepitus.

LABORATORY DATA:
Laboratory tests from the day of surgery are as follows:

- White blood cell count: 7,000/uL
- Hemoglobin: 13.2 g/dL
- Platelet count: 250,000/uL
- Serum creatinine: 0.9 mg/dL
- Total bilirubin: 0.8 mg/dL
- Alanine aminotransferase (ALT): 22 U/L
- Aspartate aminotransferase (AST): 25 U/L

## Encounter 2: source 2020-07-15, practice 2021-10-22, ed, Emergency Medicine

Chief complaint: Sudden-onset abdominal pain and lip swelling consistent with known hereditary angioedema

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Emily Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset abdominal pain and lip swelling consistent with known hereditary angioedema

HISTORY OF PRESENT ILLNESS:
The patient, known to have hereditary angioedema with C1 inhibitor deficiency and status post open cholecystectomy, presents to the Emergency Department with acute abdominal pain. The abdominal pain began 4 hours ago and is described as deep, crampy, and non-radiating. Her parents report that she has had multiple similar episodes since early childhood, occasionally accompanied by swelling of her face, extremities, or genitalia. These episodes typically last 2–3 days and resolve spontaneously. She was previously treated with antihistamines, but they have never been effective. She denies rash, fever, or any recent illness.

PAST MEDICAL HISTORY:
Active Problem List:
- Cervical radiculopathy due to C6-C7 intervertebral disc herniation (diagnosed 2020-01-15)
- Postoperative incisional pain (diagnosed 2020-02-15)
- Hypertension
- Hereditary angioedema (C1 inhibitor deficiency)
- Cervical degenerative disc disease with C6-C7 herniation

PAST SURGICAL HISTORY:
- Open cholecystectomy (age 45)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
No known drug allergies, No known food allergies

FAMILY HISTORY:
Her maternal uncle reportedly died from unexplained throat swelling in his 20s.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Office administrator.

VITAL SIGNS:
Her blood pressure is 140/90 mm Hg, pulse is 105/min, and respirations are 28/min.

PHYSICAL EXAMINATION:
On physical exam, she is afebrile but appears uncomfortable.
She has perioral swelling without urticaria and diffuse abdominal tenderness to palpation, but no rebound or guarding. Bowel sounds are hyperactive, and no organomegaly is present.

LABORATORY DATA:
A decrease in

IMAGING:
A CT scan of the abdomen shows bowel wall edema and small-volume ascites.

## Encounter 3: source 2021-07-15, practice 2022-10-14, outpatient, Otolaryngology

Chief complaint: Six-month history of progressive right-sided hearing loss, tinnitus, and facial weakness

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. David Park, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of progressive right-sided hearing loss, tinnitus, and facial weakness

HISTORY OF PRESENT ILLNESS:
This 46-year-old female with hereditary angioedema and cervical degenerative disc disease presents with progressive loss of hearing in her right ear associated with occasional tinnitus. She denies vertigo, headaches, or changes in taste.

PAST MEDICAL HISTORY:
Active Problem List:
- Cervical radiculopathy due to C6-C7 intervertebral disc herniation (diagnosed 2020-01-15)
- Postoperative incisional pain (diagnosed 2020-02-15)
- Hereditary Angioedema (C1 Inhibitor Deficiency) (diagnosed 2020-07-15)
- Hypertension
- Cervical degenerative disc disease with C6-C7 herniation

PAST SURGICAL HISTORY:
- Open cholecystectomy (age 45)

MEDICATIONS:
Hydrochlorothiazide

ALLERGIES:
No known drug allergies, No known food allergies

FAMILY HISTORY:
- Mother: Hereditary angioedema (confirmed C1 inhibitor deficiency)
- Father: Hypertension, cerebrovascular accident at age 60

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Office administrator.

PHYSICAL EXAMINATION:
On physical examination, you note decreased corneal reflex and a mild right-sided facial droop.

IMAGING:
MRI of the brain with gadolinium contrast reveals a 2 cm mass at the right cerebellopontine angle.

OTHER STUDIES:
Audiometry shows sensorineural hearing loss on the right side.

## Encounter 4: source 2022-07-15, practice 2023-10-30, outpatient, Endocrinology

Chief complaint: Intermittent severe headaches, palpitations, and diaphoresis occurring over the past few months

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Aisha Patel, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent severe headaches, palpitations, and diaphoresis occurring over the past few months

HISTORY OF PRESENT ILLNESS:
This 47-year-old female with hypertension, hereditary angioedema, cervical disc disease, and prior evaluation for cervical radiculopathy and acoustic neuroma reports severe headaches, palpitations, and diaphoresis. These symptoms have been occurring intermittently over the past few months but have become more frequent and intense. She denies any chest pain, dyspnea, or recent illness.

PAST MEDICAL HISTORY:
Active Problem List:
- Cervical radiculopathy due to C6-C7 intervertebral disc herniation (diagnosed 2020-01-15)
- Postoperative incisional pain (diagnosed 2020-02-15)
- Hereditary Angioedema (C1 Inhibitor Deficiency) (diagnosed 2020-07-15)
- Acoustic neuroma (Vestibular schwannoma) (diagnosed 2021-07-15)
- Hypertension
- Cervical degenerative disc disease with C6-C7 herniation

PAST SURGICAL HISTORY:
- Open cholecystectomy (age 45)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
No known drug allergies, No known food allergies

FAMILY HISTORY:
Her family history is significant for her father who had thyroid cancer.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Office administrator.

VITAL SIGNS:
Her blood pressure is 220/110 mmHg, heart rate is 118 bpm, and she is afebrile.

PHYSICAL EXAMINATION:
Physical examination reveals a well-nourished woman in distress due to a headache. Her skin is notably moist, and she has a fine tremor. An abdominal examination is unremarkable.

LABORATORY DATA:
Laboratory tests reveal:

- Serum glucose: 110 mg/dL
- Serum potassium: 3.8 mEq/L
- Serum creatinine: 0.9 mg/dL
- Plasma metanephrines: Pending
- Urinary catecholamines: Pending

OTHER STUDIES:
An ECG shows sinus tachycardia with no ST or T wave changes.

PLAN:
The patient is given intravenous phentolamine, after which her blood pressure decreases to 150/90 mmHg and her symptoms improve.

## Encounter 5: source 2024-01-15, practice 2025-04-23, ed, Emergency Medicine

Chief complaint: Acute onset palpitations and lightheadedness while gardening 30 minutes ago

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset palpitations and lightheadedness while gardening 30 minutes ago

HISTORY OF PRESENT ILLNESS:
This 49-year-old female has a history of hypertension, hereditary angioedema (C1 inhibitor deficiency), cervical degenerative disc disease, and prior pheochromocytoma. She presents with palpitations and lightheadedness that started while she was gardening 30 minutes ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Cervical radiculopathy due to C6-C7 intervertebral disc herniation (diagnosed 2020-01-15)
- Postoperative incisional pain (diagnosed 2020-02-15)
- Hereditary Angioedema (C1 Inhibitor Deficiency) (diagnosed 2020-07-15)
- Acoustic neuroma (Vestibular schwannoma) (diagnosed 2021-07-15)
- Pheochromocytoma (diagnosed 2022-07-15)
- Hypertension
- Cervical degenerative disc disease with C6-C7 herniation

She has a history of hypertension, well-controlled on hydrochlorothiazide, and no known cardiac disease.

PAST SURGICAL HISTORY:
- Open cholecystectomy (age 45)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
No known drug allergies, No known food allergies

FAMILY HISTORY:
- Mother: Hereditary angioedema (confirmed C1 inhibitor deficiency)
- Father: Hypertension, cerebrovascular accident at age 60

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Office administrator.

VITAL SIGNS:
On examination, her blood pressure is 130/85 mmHg, heart rate is 170 beats per minute, respiratory rate is 18 breaths per minute, and oxygen saturation is 98% on room air.

PHYSICAL EXAMINATION:
Her lungs are clear to auscultation, and there is no leg swelling.

OTHER STUDIES:
The cardiac monitor shows a narrow-QRS-complex tachycardia without visible P waves.

PLAN:
An intravenous line is established, and adenosine is administered. Shortly after administration, the heart rate slows, and distinct P waves with a sawtooth pattern are observed before the tachycardia resumes.

## Encounter 6: source 2024-07-15, practice 2025-10-19, ed, Emergency Medicine

Chief complaint: Sudden dizziness, vomiting, and difficulty walking after neck extension while painting ceiling

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Maria Garcia, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden dizziness, vomiting, and difficulty walking after neck extension while painting ceiling

HISTORY OF PRESENT ILLNESS:
The patient is a 49-year-old female with known cervical degenerative disc disease and C6-C7 herniation previously evaluated for radiculopathy who presents with symptoms that started 2 hours ago after she extended her neck to paint her ceiling. She reports that the room seems to be spinning around her, and she has vomited twice. She also notes numbness on the left side of her face and weakness in her right arm and leg.

PAST MEDICAL HISTORY:
Active Problem List:
- Cervical radiculopathy due to C6-C7 intervertebral disc herniation (diagnosed 2020-01-15)
- Postoperative incisional pain (diagnosed 2020-02-15)
- Hereditary Angioedema (C1 Inhibitor Deficiency) (diagnosed 2020-07-15)
- Acoustic neuroma (Vestibular schwannoma) (diagnosed 2021-07-15)
- Pheochromocytoma (diagnosed 2022-07-15)
- Atrial flutter (diagnosed 2024-01-15)
- Hypertension
- Cervical degenerative disc disease with C6-C7 herniation

Her past medical history is significant for hypertension,.

PAST SURGICAL HISTORY:
- Open cholecystectomy (age 45)

MEDICATIONS:
For which she takes hydrochlorothiazide.

ALLERGIES:
No known drug allergies, No known food allergies

FAMILY HISTORY:
- Mother: Hereditary angioedema (confirmed C1 inhibitor deficiency)
- Father: Hypertension, cerebrovascular accident at age 60

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Office administrator.

VITAL SIGNS:
On examination, her blood pressure is 150/90 mmHg, heart rate is 85 beats per minute, and she is afebrile.

PHYSICAL EXAMINATION:
Neurological examination reveals decreased sensation to light touch and pinprick on the left side of her face and decreased strength in the right upper and lower extremities. The remainder of the cranial nerve examination is normal.

IMAGING:
Head CT without contrast shows no acute abnormalities.

## Encounter 7: source 2025-01-15, practice 2026-04-20, ed, Emergency Medicine

Chief complaint: Severe epigastric pain radiating to the back for 24 hours, worsened by lying flat

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe epigastric pain radiating to the back for 24 hours, worsened by lying flat

HISTORY OF PRESENT ILLNESS:
This 50-year-old female with hypertension, hereditary angioedema, and prior open cholecystectomy presents with abdominal pain. She describes the pain as constant and sharp, and it worsens when she lies flat. She denies any history of alcohol abuse or gallstones.

PAST MEDICAL HISTORY:
Active Problem List:
- Cervical radiculopathy due to C6-C7 intervertebral disc herniation (diagnosed 2020-01-15)
- Postoperative incisional pain (diagnosed 2020-02-15)
- Hereditary Angioedema (C1 Inhibitor Deficiency) (diagnosed 2020-07-15)
- Acoustic neuroma (Vestibular schwannoma) (diagnosed 2021-07-15)
- Pheochromocytoma (diagnosed 2022-07-15)
- Atrial flutter (diagnosed 2024-01-15)
- Weber syndrome (Midbrain infarction) (diagnosed 2024-07-15)
- Hypertension
- Cervical degenerative disc disease with C6-C7 herniation

Her past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Open cholecystectomy (age 45)

MEDICATIONS:
She does not take any other medications.

ALLERGIES:
No known drug allergies, No known food allergies

FAMILY HISTORY:
- Mother: Hereditary angioedema (confirmed C1 inhibitor deficiency)
- Father: Hypertension, cerebrovascular accident at age 60

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Office administrator.

VITAL SIGNS:
On physical examination, her blood pressure is 145/90 mmHg, heart rate is 102 beats per minute, and temperature is 37.8°C (100°F).

PHYSICAL EXAMINATION:
Her abdomen is tender to palpation in the epigastric region with no rebound or guarding. Bowel sounds are normal.

LABORATORY DATA:
Laboratory tests reveal:

- White blood cell count: 12,000 /µL
- Hemoglobin: 13.5 g/dL
- Platelets: 250,000 /µL
- Serum amylase: 900 U/L (Normal: 30-110 U/L)
- Serum lipase: 850 U/L (Normal: 0-160 U/L)
- Triglycerides: 180 mg/dL
- Calcium: 9.2 mg/dL
- Liver function tests: Normal
