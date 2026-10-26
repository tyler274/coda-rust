# synthetic_hospital patient 2844

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex F, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-10-04 (Tue) | outpatient | Cardiology |
| 1 | 2020-04-15 | 2022-12-23 (Fri) | outpatient | Ophthalmology |
| 2 | 2020-09-15 | 2023-06-05 (Mon) | outpatient | Internal Medicine |
| 3 | 2021-03-15 | 2023-11-27 (Mon) | outpatient | Neurosurgery |
| 4 | 2021-09-15 | 2024-06-03 (Mon) | outpatient | Vascular Surgery |
| 5 | 2022-03-15 | 2024-11-21 (Thu) | outpatient | Obstetrics & Gynecology |
| 6 | 2022-09-15 | 2025-05-21 (Wed) | ed | Emergency Medicine |
| 7 | 2023-03-15 | 2025-11-30 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired elementary school principal",
 "alcohol_use": "Occasional (1-2 glasses of wine per week)",
 "family_history": [
  "Mother: Breast cancer (age 62)",
  "Father: Ischemic stroke (age 71)",
  "Sister: Age-related macular degeneration"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "75 mg PO nightly",
   "name": "Amitriptyline"
  },
  {
   "dose": "60 mg PO daily (tapering regimen for giant cell arteritis)",
   "name": "Prednisone"
  },
  {
   "dose": "200 mg PO twice daily",
   "name": "Dipyridamole"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Total abdominal hysterectomy with bilateral salpingo-oophorectomy (3 months prior)",
  "Laminectomy and foraminotomy C5-C6 (planned)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia",
  "Major depressive disorder",
  "Age-related macular degeneration, dry type",
  "Cervical spinal stenosis with cervical spondylotic myelopathy",
  "Giant cell arteritis",
  "Subclavian steal syndrome",
  "Ovarian serous adenocarcinoma",
  "Coronary artery disease"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Major depressive disorder", "Cervical spinal stenosis", "Malignant neoplasm of ovary", "Tricyclic antidepressant toxicity", "Giant cell arteritis or autoimmune inflammatory condition", "Subclavian steal syndrome", "Age-related macular degeneration, dry type"]
Comorbidities: ["Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2022-10-04, outpatient, Cardiology

Chief complaint: Exertional fatigue and intermittent chest discomfort over the past three months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Exertional fatigue and intermittent chest discomfort over the past three months

HISTORY OF PRESENT ILLNESS:
A 64-year-old woman comes to the cardiology clinic for evaluation of exertional fatigue and intermittent chest discomfort over the past three months. The discomfort is described as a vague pressure in the central chest that occurs when she walks long distances or climbs stairs and resolves with rest.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- Major depressive disorder
- Age-related macular degeneration, dry type
- Cervical spinal stenosis with cervical spondylotic myelopathy
- Giant cell arteritis
- Subclavian steal syndrome
- Ovarian serous adenocarcinoma
- Coronary artery disease

She has a history of hypertension and hyperlipidemia, both well-controlled.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (3 months prior)
- Laminectomy and foraminotomy C5-C6 (planned)

MEDICATIONS:
She takes with lisinopril and atorvastatin.

Home Medications:
- Amlodipine 5 mg PO daily
- Amitriptyline 75 mg PO nightly
- Prednisone 60 mg PO daily (tapering regimen for giant cell arteritis)
- Dipyridamole 200 mg PO twice daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (age 62)
- Father: Ischemic stroke (age 71)
- Sister: Age-related macular degeneration

SOCIAL HISTORY:
She does not smoke and drinks alcohol occasionally.

VITAL SIGNS:
Blood pressure today is 132/78 mmHg, and pulse is 76/min and regular.

PHYSICAL EXAMINATION:
Cardiovascular examination reveals normal heart sounds without murmurs, rubs, or gallops.

OTHER STUDIES:
An ECG performed in the clinic is normal.

## Encounter 1: source 2020-04-15, practice 2022-12-23, outpatient, Ophthalmology

Chief complaint: Progressive difficulty reading and recognizing faces over the past several months

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Ophthalmology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty reading and recognizing faces over the past several months

HISTORY OF PRESENT ILLNESS:
This is a 68-year-old female with established dry age-related macular degeneration who returns for reassessment of progressive visual concerns, following recent Cardiology evaluation for chest discomfort. She states that her peripheral vision remains intact, but she finds it increasingly hard to focus on words in the center of a page. Her condition has been slowly worsening, and she has stopped knitting, a hobby she previously enjoyed, because "the center of the pattern looks like it's missing." She denies any eye pain, redness, or flashes of light.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- Major depressive disorder
- Age-related macular degeneration, dry type
- Cervical spinal stenosis with cervical spondylotic myelopathy
- Giant cell arteritis
- Subclavian steal syndrome
- Ovarian serous adenocarcinoma
- Coronary artery disease

Her past medical history includes hypertension and hyperlipidemia, both well-controlled with medications.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (3 months prior)
- Laminectomy and foraminotomy C5-C6 (planned)

MEDICATIONS:
Home Medications:
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Amitriptyline 75 mg PO nightly
- Prednisone 60 mg PO daily (tapering regimen for giant cell arteritis)
- Dipyridamole 200 mg PO twice daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (age 62)
- Father: Ischemic stroke (age 71)
- Sister: Age-related macular degeneration

SOCIAL HISTORY:
She is a non-smoker and lives independently.

PHYSICAL EXAMINATION:
During the visit, she is asked to look at an Amsler grid, and she reports that the straight lines appear wavy and distorted in the central area. Fundoscopy reveals multiple yellowish-white deposits clustered around the macula. The retina surrounding the macula appears thin and mildly pigmented but without evidence of hemorrhage or neovascularization.

IMAGING:
The image below shows the fundoscopic finding (left) and her perception of the Amsler grid test (right).

## Encounter 2: source 2020-09-15, practice 2023-06-05, outpatient, Internal Medicine

Chief complaint: Six-month history of memory problems, social withdrawal, and loss of interest in hobbies

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Aisha Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of memory problems, social withdrawal, and loss of interest in hobbies

HISTORY OF PRESENT ILLNESS:
The patient returns following ophthalmologic assessment five months prior, now reporting a six-month history of becoming increasingly forgetful, misplacing items, and struggling to manage her finances. She also appears to have lost interest in her hobbies and has been socially withdrawing.

PAST MEDICAL HISTORY:
Active Problem List:
- Age-related macular degeneration, dry type (diagnosed 2020-04-15)
- Hypertension
- Hyperlipidemia
- Major depressive disorder
- Cervical spinal stenosis with cervical spondylotic myelopathy
- Giant cell arteritis
- Subclavian steal syndrome
- Ovarian serous adenocarcinoma
- Coronary artery disease

The patient has a past medical history of hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (3 months prior)
- Laminectomy and foraminotomy C5-C6 (planned)

MEDICATIONS:
Her medications include lisinopril and atorvastatin.

Home Medications:
- Amlodipine 5 mg PO daily
- Amitriptyline 75 mg PO nightly
- Prednisone 60 mg PO daily (tapering regimen for giant cell arteritis)
- Dipyridamole 200 mg PO twice daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (age 62)
- Father: Ischemic stroke (age 71)
- Sister: Age-related macular degeneration

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 glasses of wine per week). Occupation: Retired elementary school principal.

REVIEW OF SYSTEMS:
She denies any headaches, dizziness, or focal neurological deficits.

PHYSICAL EXAMINATION:
On examination, the patient has a flat affect and is slow to answer questions but is oriented to person, place, and time. Her Mini-Mental State Examination (MMSE) score is 24/30, with deficits noted in attention and calculation.

LABORATORY DATA:
Her laboratory tests are as follows:

- Complete blood count: within normal limits
- Thyroid-stimulating hormone: 2.5 mIU/L (normal range: 0.4-4.0 mIU/L)
- Vitamin B12: 350 pg/mL (normal range: 200-900 pg/mL)
- Fasting glucose: 98 mg/dL (normal range: 70-100 mg/dL)

## Encounter 3: source 2021-03-15, practice 2023-11-27, outpatient, Neurosurgery

Chief complaint: Several months of progressive difficulty using hands to grasp small objects and perform fine motor tasks

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. James Wilson, MD
Department: Neurosurgery
Visit Type: outpatient

CHIEF COMPLAINT:
Several months of progressive difficulty using hands to grasp small objects and perform fine motor tasks

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old female with established cervical spinal stenosis and cervical spondylotic myelopathy presenting with progression of upper extremity symptoms. Her recent medical history includes management of coronary artery disease, age-related macular degeneration, and major depressive disorder. She reports struggling to grasp small objects like keys and has recently needed assistance with tasks like buttoning her blouse and opening jars. She denies recent trauma, falls, or fever. Her symptoms have been slowly progressive and worse on the right. She also describes intermittent tingling and dull aching pain that radiates down her right arm, sometimes reaching the hand. She does not report neck stiffness, gait instability, or bowel/bladder incontinence.

PAST MEDICAL HISTORY:
Active Problem List:
- Age-related macular degeneration, dry type (diagnosed 2020-04-15)
- Major depressive disorder (diagnosed 2020-09-15)
- Hypertension
- Hyperlipidemia
- Cervical spinal stenosis with cervical spondylotic myelopathy
- Giant cell arteritis
- Subclavian steal syndrome
- Ovarian serous adenocarcinoma
- Coronary artery disease

Her medical history includes hypertension and hyperlipidemia, both controlled with medication.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (3 months prior)
- Laminectomy and foraminotomy C5-C6 (planned)

MEDICATIONS:
Home Medications:
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Amitriptyline 75 mg PO nightly
- Prednisone 60 mg PO daily (tapering regimen for giant cell arteritis)
- Dipyridamole 200 mg PO twice daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (age 62)
- Father: Ischemic stroke (age 71)
- Sister: Age-related macular degeneration

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 glasses of wine per week). Occupation: Retired elementary school principal.

PHYSICAL EXAMINATION:
Neurologic examination shows decreased grip strength and reduced fine motor coordination on the right. Reflexes are symmetric, and no spasticity or hyperreflexia is observed. There is patchy sensory loss in the right hand.

## Encounter 4: source 2021-09-15, practice 2024-06-03, outpatient, Vascular Surgery

Chief complaint: Six-month history of intermittent dizziness and left arm fatigue with exertion

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Fatima Al-Hassan, MD
Department: Vascular Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of intermittent dizziness and left arm fatigue with exertion

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old female with coronary artery disease and giant cell arteritis on corticosteroids who returns for vascular evaluation after cardiology care. She presents with a 6-month history of intermittent dizziness and left arm fatigue. She reports these symptoms often occur after carrying groceries or walking her dog. She also describes experiencing transient episodes of blurred vision and difficulty with balance.

PAST MEDICAL HISTORY:
Active Problem List:
- Age-related macular degeneration, dry type (diagnosed 2020-04-15)
- Major depressive disorder (diagnosed 2020-09-15)
- Cervical spinal stenosis (diagnosed 2021-03-15)
- Hypertension
- Hyperlipidemia
- Cervical spinal stenosis with cervical spondylotic myelopathy
- Giant cell arteritis
- Subclavian steal syndrome
- Ovarian serous adenocarcinoma
- Coronary artery disease

Her past medical history includes hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (3 months prior)
- Laminectomy and foraminotomy C5-C6 (planned)

MEDICATIONS:
Home Medications:
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Amitriptyline 75 mg PO nightly
- Prednisone 60 mg PO daily (tapering regimen for giant cell arteritis)
- Dipyridamole 200 mg PO twice daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (age 62)
- Father: Ischemic stroke (age 71)
- Sister: Age-related macular degeneration

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 glasses of wine per week). Occupation: Retired elementary school principal.

VITAL SIGNS:
On physical examination, her blood pressure is 150/90 mmHg in the right arm and 130/80 mmHg in the left arm.

PHYSICAL EXAMINATION:
Auscultation of the chest is unremarkable, and carotid bruits are absent. Neurological examination is normal at rest. Ankle-brachial index is 1.0 bilaterally.

IMAGING:
Doppler ultrasound of the upper extremities reveals a significant stenosis of the proximal left subclavian artery with reversed flow in the left vertebral artery.

## Encounter 5: source 2022-03-15, practice 2024-11-21, outpatient, Obstetrics & Gynecology

Chief complaint: Three-month history of progressive abdominal bloating and early satiety with unintentional weight loss

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Emily Thompson, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of progressive abdominal bloating and early satiety with unintentional weight loss

HISTORY OF PRESENT ILLNESS:
This 70-year-old female with hypertension, hyperlipidemia, coronary artery disease, subclavian steal syndrome, and giant cell arteritis on prednisone taper presents with three-month progressive abdominal bloating and early satiety. She has also unintentionally lost 4 kg (9 lbs) over the past 2 months. She denies nausea, vomiting, constipation, or vaginal bleeding.

PAST MEDICAL HISTORY:
Active Problem List:
- Age-related macular degeneration, dry type (diagnosed 2020-04-15)
- Major depressive disorder (diagnosed 2020-09-15)
- Cervical spinal stenosis (diagnosed 2021-03-15)
- Subclavian steal syndrome (diagnosed 2021-09-15)
- Hypertension
- Hyperlipidemia
- Cervical spinal stenosis with cervical spondylotic myelopathy
- Giant cell arteritis
- Ovarian serous adenocarcinoma
- Coronary artery disease

Her medical history includes hypertension and hyperlipidemia,.

PAST SURGICAL HISTORY:
Surgical history includes a cholecystectomy and appendectomy in her 40s.

MEDICATIONS:
She takes managed with amlodipine and atorvastatin.

Home Medications:
- Amitriptyline 75 mg PO nightly
- Prednisone 60 mg PO daily (tapering regimen for giant cell arteritis)
- Dipyridamole 200 mg PO twice daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Family history is notable for her mother dying of “stomach problems” in her 60s and a sister recently diagnosed with breast cancer.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 glasses of wine per week). Occupation: Retired elementary school principal.

PHYSICAL EXAMINATION:
On examination, her abdomen is mildly distended with shifting dullness and a fluid wave. There is no palpable adnexal mass on bimanual exam,

LABORATORY DATA:
Laboratory results are pending.

IMAGING:
Pelvic ultrasound reveals bilateral complex adnexal masses with solid and cystic components and moderate free fluid in the abdomen.

PATHOLOGY:
Biopsy reveals papillary structures with concentric calcifications.

PLAN:
She is referred for diagnostic laparoscopy,

## Encounter 6: source 2022-09-15, practice 2025-05-21, ed, Emergency Medicine

Chief complaint: Two-week history of persistent temporal headache with recent transient vision loss in the right eye

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-week history of persistent temporal headache with recent transient vision loss in the right eye

HISTORY OF PRESENT ILLNESS:
This 70-year-old female with giant cell arteritis on prednisone taper presents with two-week temporal headache. She describes throbbing pain rated 7 out of 10 in intensity. She reports tenderness when combing her hair and expresses concern about a recent episode of transient vision loss in the right eye that resolved within a few minutes.

PAST MEDICAL HISTORY:
Active Problem List:
- Age-related macular degeneration, dry type (diagnosed 2020-04-15)
- Major depressive disorder (diagnosed 2020-09-15)
- Cervical spinal stenosis (diagnosed 2021-03-15)
- Subclavian steal syndrome (diagnosed 2021-09-15)
- Malignant neoplasm of ovary (diagnosed 2022-03-15)
- Hypertension
- Hyperlipidemia
- Cervical spinal stenosis with cervical spondylotic myelopathy
- Giant cell arteritis
- Ovarian serous adenocarcinoma
- Coronary artery disease

She has a past medical history of hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (3 months prior)
- Laminectomy and foraminotomy C5-C6 (planned)

MEDICATIONS:
Home Medications:
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Amitriptyline 75 mg PO nightly
- Prednisone 60 mg PO daily (tapering regimen for giant cell arteritis)
- Dipyridamole 200 mg PO twice daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (age 62)
- Father: Ischemic stroke (age 71)
- Sister: Age-related macular degeneration

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 glasses of wine per week). Occupation: Retired elementary school principal.

PHYSICAL EXAMINATION:
On physical examination, her temporal arteries appear prominent and are tender to palpation. Her jaw becomes fatigued while chewing. Visual acuity is normal,

LABORATORY DATA:
Laboratory findings are as follows:

- Erythrocyte sedimentation rate (ESR): 85 mm/hr (Normal: 0-20 mm/hr)
- C-reactive protein (CRP): 4.5 mg/dL (Normal: <0.9 mg/dL)
- Complete blood count (CBC): Normal
- Comprehensive metabolic panel (CMP): Normal

## Encounter 7: source 2023-03-15, practice 2025-11-30, ed, Emergency Medicine

Chief complaint: Two-day history of altered mental status with confusion, agitation, and visual hallucinations

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. Maria Gonzalez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-day history of altered mental status with confusion, agitation, and visual hallucinations

HISTORY OF PRESENT ILLNESS:
This patient with major depressive disorder returns to the emergency department. Her daughter reports that the patient has been increasingly confused over the past two days, with episodes of agitation and visual hallucinations.

PAST MEDICAL HISTORY:
Active Problem List:
- Age-related macular degeneration, dry type (diagnosed 2020-04-15)
- Major depressive disorder (diagnosed 2020-09-15)
- Cervical spinal stenosis (diagnosed 2021-03-15)
- Subclavian steal syndrome (diagnosed 2021-09-15)
- Malignant neoplasm of ovary (diagnosed 2022-03-15)
- Giant cell arteritis or autoimmune inflammatory condition (diagnosed 2022-09-15)
- Hypertension
- Hyperlipidemia
- Cervical spinal stenosis with cervical spondylotic myelopathy
- Giant cell arteritis
- Ovarian serous adenocarcinoma
- Coronary artery disease

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (3 months prior)
- Laminectomy and foraminotomy C5-C6 (planned)

MEDICATIONS:
The patient's medications include amitriptyline and hydrochlorothiazide.

Home Medications:
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Prednisone 60 mg PO daily (tapering regimen for giant cell arteritis)
- Dipyridamole 200 mg PO twice daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (age 62)
- Father: Ischemic stroke (age 71)
- Sister: Age-related macular degeneration

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 glasses of wine per week). Occupation: Retired elementary school principal.

VITAL SIGNS:
Heart rate of 110 bpm, blood pressure of 145/90 mmHg, and a temperature of 37.2°C (99°F)

PHYSICAL EXAMINATION:
On examination, the patient is disoriented to time and place. She has dry mucous membranes, dilated pupils that are slow to react to light, and flushed skin. Bowel sounds are hypoactive, and she is unable to void on command.

OTHER STUDIES:
An electrocardiogram (ECG) is performed, showing a QRS duration of 120 ms and a QTc interval of 500 ms.
