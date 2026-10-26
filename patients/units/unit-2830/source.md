# synthetic_hospital patient 2830

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 65 at first encounter, sex F, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-06-10 (Thu) | outpatient | Ophthalmology |
| 1 | 2020-07-15 | 2021-11-29 (Mon) | outpatient | Rheumatology |
| 2 | 2021-01-15 | 2022-06-06 (Mon) | outpatient | Internal Medicine |
| 3 | 2021-07-15 | 2022-12-06 (Tue) | outpatient | Orthopedics & Sports Medicine |
| 4 | 2022-01-15 | 2023-06-12 (Mon) | outpatient | Orthopedics & Sports Medicine |
| 5 | 2022-07-15 | 2023-12-05 (Tue) | outpatient | Neurology |
| 6 | 2023-01-15 | 2024-06-10 (Mon) | outpatient | Rheumatology |
| 7 | 2024-07-15 | 2025-12-01 (Mon) | outpatient | Psychiatry |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired retail supervisor",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Late-onset Alzheimer disease (diagnosed age 78)",
  "Father: Type 2 diabetes mellitus, hypertension, hyperlipidemia; died of myocardial infarction age 72",
  "Maternal grandmother: Primary open-angle glaucoma",
  "Sister: Osteoarthritis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 1 pack per day for 35 years (35 pack-years)",
 "home_medications": [
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "0.005% ophthalmic solution 1 drop OU at bedtime",
   "name": "Latanoprost"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Donepezil"
  },
  {
   "dose": "300 mg PO three times daily",
   "name": "Gabapentin"
  }
 ],
 "surgical_history": [
  "Hysterectomy (age 48)",
  "Cholecystectomy (age 52)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Obesity (BMI 33 kg/m\u00b2)",
  "Hyperlipidemia",
  "Tobacco use disorder"
 ],
 "age_at_first_encounter": 65
}
```

Primary diagnoses: ["Osteoarthritis exacerbation/flare", "Osteoarthritis of knee", "Alzheimer disease", "Osteoarthritis", "Primary open-angle glaucoma", "Postherpetic neuralgia"]
Comorbidities: ["Essential hypertension", "Obesity", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2021-06-10, outpatient, Ophthalmology

Chief complaint: Gradual, painless, bilateral vision loss over the past 6 months.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Ophthalmology
Visit Type: outpatient

CHIEF COMPLAINT:
Gradual, painless, bilateral vision loss over the past 6 months.

HISTORY OF PRESENT ILLNESS:
6-month history of gradual, painless, bilateral vision loss. She denies any redness or discharge from the eyes.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Obesity (BMI 33 kg/m²)
- Hyperlipidemia
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Hysterectomy (age 48)
- Cholecystectomy (age 52)

MEDICATIONS:
She takes hydrochlorothiazide, metformin.

Home Medications:
- Atorvastatin 40 mg PO daily
- Latanoprost 0.005% ophthalmic solution 1 drop OU at bedtime
- Donepezil 10 mg PO daily
- Gabapentin 300 mg PO three times daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Late-onset Alzheimer disease (diagnosed age 78)
- Father: Type 2 diabetes mellitus, hypertension, hyperlipidemia; died of myocardial infarction age 72
- Maternal grandmother: Primary open-angle glaucoma
- Sister: Osteoarthritis

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 35 years (35 pack-years). Alcohol: None. Occupation: Retired retail supervisor.

PHYSICAL EXAMINATION:
Visual acuity is decreased bilaterally. Intraocular pressure (IOP) readings are 28 mmHg in the right eye and 27 mmHg in the left eye. Fundoscopic examination reveals increased cup-to-disc ratio in both eyes. Central visual fields are intact, but there is peripheral visual field loss. The anterior chambers are deep and angles are open on gonioscopy.

## Encounter 1: source 2020-07-15, practice 2021-11-29, outpatient, Rheumatology

Chief complaint: Progressive pain in both hands worsening over the past year, worse in the morning but improving with activity.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive pain in both hands worsening over the past year, worse in the morning but improving with activity.

HISTORY OF PRESENT ILLNESS:
The patient with hypertension, type 2 diabetes, and prior open-angle glaucoma evaluation presents with hand pain that has been progressively worsening over the past year. She reports that the pain is worse in the morning but improves with activity throughout the day. She denies any redness or warmth over the joints.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary open-angle glaucoma (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Obesity (BMI 33 kg/m²)
- Hyperlipidemia
- Tobacco use disorder

Her medical history is significant for hypertension, and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Hysterectomy (age 48)
- Cholecystectomy (age 52)

MEDICATIONS:
She takes hydrochlorothiazide and metformin.

Home Medications:
- Atorvastatin 40 mg PO daily
- Latanoprost 0.005% ophthalmic solution 1 drop OU at bedtime
- Donepezil 10 mg PO daily
- Gabapentin 300 mg PO three times daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Late-onset Alzheimer disease (diagnosed age 78)
- Father: Type 2 diabetes mellitus, hypertension, hyperlipidemia; died of myocardial infarction age 72
- Maternal grandmother: Primary open-angle glaucoma
- Sister: Osteoarthritis

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 35 years (35 pack-years). Alcohol: None. Occupation: Retired retail supervisor.

PHYSICAL EXAMINATION:
On physical examination, you note bony enlargements of the distal and proximal interphalangeal joints of both hands. There is mild tenderness on palpation of these joints, and crepitus is felt with passive movement. No erythema or warmth is noted.

IMAGING:
An X-ray of the hands is performed and shows decreased joint space, subchondral sclerosis, and periarticular osteophytes in the affected joints.

## Encounter 2: source 2021-01-15, practice 2022-06-06, outpatient, Internal Medicine

Chief complaint: Gradually worsening right knee pain over the past 6 months with morning stiffness lasting less than 20 minutes.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Gradually worsening right knee pain over the past 6 months with morning stiffness lasting less than 20 minutes.

HISTORY OF PRESENT ILLNESS:
The patient presents with progressive right knee symptoms in the context of prior hand osteoarthritis. The pain is worse with activity and improves with rest. She finds it difficult to get up after prolonged sitting and complains that her knees feel stiff in the morning but improve within 20 minutes.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary open-angle glaucoma (diagnosed 2020-01-15)
- Osteoarthritis (diagnosed 2020-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Obesity (BMI 33 kg/m²)
- Hyperlipidemia
- Tobacco use disorder

Her past medical history includes hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Hysterectomy (age 48)
- Cholecystectomy (age 52)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO daily
- Latanoprost 0.005% ophthalmic solution 1 drop OU at bedtime
- Donepezil 10 mg PO daily
- Gabapentin 300 mg PO three times daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Late-onset Alzheimer disease (diagnosed age 78)
- Father: Type 2 diabetes mellitus, hypertension, hyperlipidemia; died of myocardial infarction age 72
- Maternal grandmother: Primary open-angle glaucoma
- Sister: Osteoarthritis

SOCIAL HISTORY:
She has smoked one pack per day for the last 35 years and has a BMI of 33 kg/m².

PHYSICAL EXAMINATION:
On examination, there is mild crepitus and tenderness along the medial joint line of both knees but no erythema or effusion. Her gait is normal, and her neurological exam is unremarkable.

IMAGING:
Radiographs show joint space narrowing and osteophyte formation in the knees.

PLAN:
The patient is interested in reducing the long-term burden of her symptoms.

## Encounter 3: source 2021-07-15, practice 2022-12-06, outpatient, Orthopedics & Sports Medicine

Chief complaint: Acute worsening of chronic right knee pain with increased stiffness.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James Wilson, MD
Department: Orthopedics & Sports Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Acute worsening of chronic right knee pain with increased stiffness.

HISTORY OF PRESENT ILLNESS:
The patient with history of osteoarthritis returns for follow-up of right knee symptoms initially evaluated six months ago, noting a 6-month history of right knee pain that worsens with activity and improves with rest. She reports morning stiffness lasting about 20 minutes.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary open-angle glaucoma (diagnosed 2020-01-15)
- Osteoarthritis (diagnosed 2020-07-15)
- Osteoarthritis of knee (diagnosed 2021-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Obesity (BMI 33 kg/m²)
- Hyperlipidemia
- Tobacco use disorder

She has a history of hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Hysterectomy (age 48)
- Cholecystectomy (age 52)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO daily
- Latanoprost 0.005% ophthalmic solution 1 drop OU at bedtime
- Donepezil 10 mg PO daily
- Gabapentin 300 mg PO three times daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Late-onset Alzheimer disease (diagnosed age 78)
- Father: Type 2 diabetes mellitus, hypertension, hyperlipidemia; died of myocardial infarction age 72
- Maternal grandmother: Primary open-angle glaucoma
- Sister: Osteoarthritis

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 35 years (35 pack-years). Alcohol: None. Occupation: Retired retail supervisor.

VITAL SIGNS:
Her BMI is 32 kg/m^2.

PHYSICAL EXAMINATION:
On physical examination, there is bony enlargement of the right knee with tenderness to palpation over the medial joint line. The knee exhibits crepitus with flexion and extension, and there is a painful limitation of both passive and active range of motion. No warmth or erythema is noted.

IMAGING:
Radiographs of the right knee show joint space narrowing, subchondral sclerosis, and osteophyte formation.

## Encounter 4: source 2022-01-15, practice 2023-06-12, outpatient, Orthopedics & Sports Medicine

Chief complaint: Recurrent severe right knee pain exacerbation with activity intolerance.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James Wilson, MD
Department: Orthopedics & Sports Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Recurrent severe right knee pain exacerbation with activity intolerance.

HISTORY OF PRESENT ILLNESS:
The patient returns with recurrent right knee pain. She reports that the pain worsens with activity and improves with rest. She denies any recent trauma or injury.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary open-angle glaucoma (diagnosed 2020-01-15)
- Osteoarthritis (diagnosed 2020-07-15)
- Osteoarthritis of knee (diagnosed 2021-01-15)
- Osteoarthritis exacerbation/flare (diagnosed 2021-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Obesity (BMI 33 kg/m²)
- Hyperlipidemia
- Tobacco use disorder

Her medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Hysterectomy (age 48)
- Cholecystectomy (age 52)

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO daily
- Latanoprost 0.005% ophthalmic solution 1 drop OU at bedtime
- Donepezil 10 mg PO daily
- Gabapentin 300 mg PO three times daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Late-onset Alzheimer disease (diagnosed age 78)
- Father: Type 2 diabetes mellitus, hypertension, hyperlipidemia; died of myocardial infarction age 72
- Maternal grandmother: Primary open-angle glaucoma
- Sister: Osteoarthritis

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 35 years (35 pack-years). Alcohol: None. Occupation: Retired retail supervisor.

PHYSICAL EXAMINATION:
On physical examination, you note bony enlargement around the knee joint, and palpation elicits crepitus. The range of motion is painful and slightly decreased.

IMAGING:
An X-ray of the knee is performed, showing joint space narrowing, osteophytes, and subchondral sclerosis. There is no evidence of joint effusion.

## Encounter 5: source 2022-07-15, practice 2023-12-05, outpatient, Neurology

Chief complaint: Persistent burning pain on the right side of the chest for 5 months following an episode of shingles.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Elena Rodriguez, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent burning pain on the right side of the chest for 5 months following an episode of shingles.

HISTORY OF PRESENT ILLNESS:
The patient is a 67-year-old female with primary open-angle glaucoma and osteoarthritis who returns for evaluation of persistent pain. She reports that the pain began after an episode of shingles, which was treated with valacyclovir. The pain is described as sharp and shooting, worsening with light touch. She has tried over-the-counter acetaminophen and ibuprofen with no significant relief.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary open-angle glaucoma (diagnosed 2020-01-15)
- Osteoarthritis (diagnosed 2020-07-15)
- Osteoarthritis of knee (diagnosed 2021-01-15)
- Osteoarthritis exacerbation/flare (diagnosed 2021-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Obesity (BMI 33 kg/m²)
- Hyperlipidemia
- Tobacco use disorder

Her medical history is significant for hypertension, controlled with hydrochlorothiazide, and type 2 diabetes mellitus, managed with metformin.

PAST SURGICAL HISTORY:
- Hysterectomy (age 48)
- Cholecystectomy (age 52)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO daily
- Latanoprost 0.005% ophthalmic solution 1 drop OU at bedtime
- Donepezil 10 mg PO daily
- Gabapentin 300 mg PO three times daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Late-onset Alzheimer disease (diagnosed age 78)
- Father: Type 2 diabetes mellitus, hypertension, hyperlipidemia; died of myocardial infarction age 72
- Maternal grandmother: Primary open-angle glaucoma
- Sister: Osteoarthritis

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 35 years (35 pack-years). Alcohol: None. Occupation: Retired retail supervisor.

PHYSICAL EXAMINATION:
On physical examination, there is hyperpigmentation and scarring along the right T5 dermatome, and light touch with a cotton swab elicits severe pain in this area.

## Encounter 6: source 2023-01-15, practice 2024-06-10, outpatient, Rheumatology

Chief complaint: Worsening bilateral knee pain and new bony enlargements in the finger joints.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Marcus Johnson, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening bilateral knee pain and new bony enlargements in the finger joints.

HISTORY OF PRESENT ILLNESS:
She returns with progression of osteoarthritis following rheumatology evaluation in 2020 and orthopedic evaluations in 2021 and 2022. She describes the pain as aching and stiff, especially after prolonged standing or walking. The discomfort improves with rest but worsens throughout the day. She has also noticed bony enlargements around the joints of her fingers.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary open-angle glaucoma (diagnosed 2020-01-15)
- Osteoarthritis (diagnosed 2020-07-15)
- Osteoarthritis of knee (diagnosed 2021-01-15)
- Osteoarthritis exacerbation/flare (diagnosed 2021-07-15)
- Postherpetic neuralgia (diagnosed 2022-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Obesity (BMI 33 kg/m²)
- Hyperlipidemia
- Tobacco use disorder

Her medical history is notable for obesity and hypertension.

PAST SURGICAL HISTORY:
- Hysterectomy (age 48)
- Cholecystectomy (age 52)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO daily
- Latanoprost 0.005% ophthalmic solution 1 drop OU at bedtime
- Donepezil 10 mg PO daily
- Gabapentin 300 mg PO three times daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Late-onset Alzheimer disease (diagnosed age 78)
- Father: Type 2 diabetes mellitus, hypertension, hyperlipidemia; died of myocardial infarction age 72
- Maternal grandmother: Primary open-angle glaucoma
- Sister: Osteoarthritis

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 35 years (35 pack-years). Alcohol: None. Occupation: Retired retail supervisor.

PHYSICAL EXAMINATION:
On physical exam, there is tenderness over the medial joint line of the right knee and Heberden nodes on the distal interphalangeal (DIP) joints. No significant warmth or erythema is noted, and the joints are not swollen. Her symptoms are worse on the right than the left.

LABORATORY DATA:
Synovial fluid analysis shows low white blood cell count.

OTHER STUDIES:
A study is conducted to analyze the chondrocyte secretions in affected joints.

## Encounter 7: source 2024-07-15, practice 2025-12-01, outpatient, Psychiatry

Chief complaint: Progressively worsening forgetfulness, disorientation, and word-finding difficulties over the past 18 months.

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Aisha Washington, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Progressively worsening forgetfulness, disorientation, and word-finding difficulties over the past 18 months.

HISTORY OF PRESENT ILLNESS:
This 69-year-old female with hypertension, type 2 diabetes mellitus, hyperlipidemia, and history of primary open-angle glaucoma, osteoarthritis, and postherpetic neuralgia presents to the Psychiatry clinic. She is brought to the clinic by her daughter due to progressively worsening forgetfulness and disorientation over the past 18 months. The patient has misplaced common household items, gotten lost driving on familiar roads, and occasionally struggles to find the correct words when speaking. She also displays increased irritability and minor personality changes, which are new for her.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary open-angle glaucoma (diagnosed 2020-01-15)
- Osteoarthritis (diagnosed 2020-07-15)
- Osteoarthritis of knee (diagnosed 2021-01-15)
- Osteoarthritis exacerbation/flare (diagnosed 2021-07-15)
- Postherpetic neuralgia (diagnosed 2022-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Obesity (BMI 33 kg/m²)
- Hyperlipidemia
- Tobacco use disorder

Her medical history is significant for well-controlled hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Hysterectomy (age 48)
- Cholecystectomy (age 52)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO daily
- Latanoprost 0.005% ophthalmic solution 1 drop OU at bedtime
- Donepezil 10 mg PO daily
- Gabapentin 300 mg PO three times daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Late-onset Alzheimer disease (diagnosed age 78)
- Father: Type 2 diabetes mellitus, hypertension, hyperlipidemia; died of myocardial infarction age 72
- Maternal grandmother: Primary open-angle glaucoma
- Sister: Osteoarthritis

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 35 years (35 pack-years). Alcohol: None. Occupation: Retired retail supervisor.

PHYSICAL EXAMINATION:
She is oriented to person but not to place or time. Neurologic examination shows no focal deficits. Her Mini-Mental State Examination score is 22/30.

IMAGING:
MRI of the brain reveals diffuse cortical atrophy, most pronounced in the hippocampus and temporal lobes.
