# synthetic_hospital patient 2227

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 66 at first encounter, sex M, Non-Hispanic White, Medicare with supplemental commercial plan.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-06-25 (Fri) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2021-12-13 (Mon) | outpatient | Dermatology |
| 2 | 2021-01-15 | 2022-06-20 (Mon) | outpatient | Gastroenterology |
| 3 | 2021-08-15 | 2023-01-25 (Wed) | ed | Ophthalmology |
| 4 | 2022-07-15 | 2023-12-12 (Tue) | procedure | Cardiology |
| 5 | 2023-01-15 | 2024-06-28 (Fri) | ed | Internal Medicine |
| 6 | 2023-07-15 | 2024-12-25 (Wed) | ed | General Surgery |
| 7 | 2024-07-15 | 2025-12-29 (Mon) | outpatient | Gastroenterology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare with supplemental commercial plan",
 "occupation": "Retired construction foreman",
 "alcohol_use": "Social, approximately 1-2 drinks per week",
 "family_history": [
  "Father: Type 2 diabetes mellitus, myocardial infarction at age 72",
  "Mother: Hypertension, basal cell carcinoma",
  "Paternal uncle: Pancreatic adenocarcinoma"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 20 pack-years, quit 5 years ago",
 "home_medications": [
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "12.5 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "5 mg PO daily (dose adjusted per INR)",
   "name": "Warfarin"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "Cataract extraction, right eye (3 weeks prior)",
  "Coronary angiography (1 week prior)",
  "Tonsillectomy and adenoidectomy, age 7"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus (15-year duration)",
  "Hypertension",
  "Chronic atrial fibrillation",
  "Diabetic peripheral neuropathy"
 ],
 "age_at_first_encounter": 66
}
```

Primary diagnoses: ["Zenker diverticulum", "Venous Air Embolism", "Diabetic motor neuropathy", "Perianal abscess", "Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation", "Diabetic foot infection with osteomyelitis", "Acute postoperative endophthalmitis", "Basal Cell Carcinoma (Nodular type, of nose)"]
Comorbidities: ["Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2021-06-25, outpatient, Internal Medicine

Chief complaint: Follow-up regarding chronic foot pain

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Emily Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up regarding chronic foot pain

HISTORY OF PRESENT ILLNESS:
Over the past 6 months, he has developed burning discomfort and tingling in both feet that worsens at night and interferes with his sleep. He denies recent trauma, fever, or swelling.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension
- Chronic atrial fibrillation
- Diabetic peripheral neuropathy

He has a 15-year history of type 2 diabetes mellitus and hypertension.

PAST SURGICAL HISTORY:
- Cataract extraction, right eye (3 weeks prior)
- Coronary angiography (1 week prior)
- Tonsillectomy and adenoidectomy, age 7

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Warfarin 5 mg PO daily (dose adjusted per INR)
- Atorvastatin 40 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 72
- Mother: Hypertension, basal cell carcinoma
- Paternal uncle: Pancreatic adenocarcinoma

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Social, approximately 1-2 drinks per week. Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
Examination reveals decreased vibratory sensation and pinprick perception in a stocking distribution in both lower extremities. Motor strength is intact, and deep tendon reflexes are reduced at the ankles. The skin over the feet is intact without ulcers or discoloration.

LABORATORY DATA:
Laboratory testing shows HbA1c of 8.1%.

PLAN:
The physician recommends a topical cream for symptomatic relief, which the patient applies twice daily, leading to gradual improvement in pain over the next 4 weeks. The prescribed cream most likely depletes

## Encounter 1: source 2020-07-15, practice 2021-12-13, outpatient, Dermatology

Chief complaint: Slowly growing lesion on the nose for the past year

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Slowly growing lesion on the nose for the past year

HISTORY OF PRESENT ILLNESS:
This diabetic patient, seen in January for neuropathy, presents with a nose lesion that has been present for the past year and occasionally bleeds when he shaves but otherwise is asymptomatic.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetic motor neuropathy (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension
- Chronic atrial fibrillation
- Diabetic peripheral neuropathy

His medical history is significant for hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Cataract extraction, right eye (3 weeks prior)
- Coronary angiography (1 week prior)
- Tonsillectomy and adenoidectomy, age 7

MEDICATIONS:
He takes hydrochlorothiazide and metformin.

Home Medications:
- Lisinopril 10 mg PO daily
- Warfarin 5 mg PO daily (dose adjusted per INR)
- Atorvastatin 40 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 72
- Mother: Hypertension, basal cell carcinoma
- Paternal uncle: Pancreatic adenocarcinoma

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Social, approximately 1-2 drinks per week. Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
On physical examination, you note a 1.5 cm pearly, flesh-colored nodule with telangiectasias and a central depression on the lateral aspect of his nose. There is no regional lymphadenopathy.

PATHOLOGY:
A biopsy of the lesion is performed, and histopathology confirms the presence of basal cell carcinoma.

## Encounter 2: source 2021-01-15, practice 2022-06-20, outpatient, Gastroenterology

Chief complaint: Progressive difficulty swallowing both solids and liquids over 6 months

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty swallowing both solids and liquids over 6 months

HISTORY OF PRESENT ILLNESS:
This 67-year-old diabetic presents with dysphagia following care for foot pain and nasal lesion. He reports occasional regurgitation of undigested food, especially at night, which has led to episodes of nocturnal coughing. He also complains of bad breath despite maintaining good oral hygiene.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetic motor neuropathy (diagnosed 2020-01-15)
- Basal Cell Carcinoma (Nodular type, of nose) (diagnosed 2020-07-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension
- Chronic atrial fibrillation
- Diabetic peripheral neuropathy

His medical history is significant for hypertension, controlled with hydrochlorothiazide, and type 2 diabetes mellitus managed with metformin.

PAST SURGICAL HISTORY:
- Cataract extraction, right eye (3 weeks prior)
- Coronary angiography (1 week prior)
- Tonsillectomy and adenoidectomy, age 7

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Warfarin 5 mg PO daily (dose adjusted per INR)
- Atorvastatin 40 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 72
- Mother: Hypertension, basal cell carcinoma
- Paternal uncle: Pancreatic adenocarcinoma

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Social, approximately 1-2 drinks per week. Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
On physical examination, a neck exam reveals a small, non-tender mass in the left cervical region that gurgles on palpation.

IMAGING:
A barium swallow study shows a saccular outpouching arising from the posterior hypopharyngeal wall.

## Encounter 3: source 2021-08-15, practice 2023-01-25, ed, Ophthalmology

Chief complaint: Severe pain and decreased vision in right eye for 2 days following recent cataract surgery

ENCOUNTER NOTE
Date: 2021-08-15
Provider: Dr. Sarah Williams, MD
Department: Ophthalmology
Visit Type: ed

CHIEF COMPLAINT:
Severe pain and decreased vision in right eye for 2 days following recent cataract surgery

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with Type 2 Diabetes Mellitus and a history of foot neuropathy, nasal skin lesions, and swallowing difficulties presents with severe pain and decreased vision in the right eye following recent cataract surgery. He reports that the symptoms began suddenly and have progressively worsened.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetic motor neuropathy (diagnosed 2020-01-15)
- Basal Cell Carcinoma (Nodular type, of nose) (diagnosed 2020-07-15)
- Zenker diverticulum (diagnosed 2021-01-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension
- Chronic atrial fibrillation
- Diabetic peripheral neuropathy

PAST SURGICAL HISTORY:
The patient underwent cataract surgery on the same eye 4 weeks ago.

MEDICATIONS:
He takes metformin and lisinopril.

Home Medications:
- Hydrochlorothiazide 12.5 mg PO daily
- Warfarin 5 mg PO daily (dose adjusted per INR)
- Atorvastatin 40 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 72
- Mother: Hypertension, basal cell carcinoma
- Paternal uncle: Pancreatic adenocarcinoma

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Social, approximately 1-2 drinks per week. Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
His right eyelid is swollen, and the conjunctiva is injected. Slit-lamp examination reveals a hypopyon, corneal edema, and intense intraocular inflammation. Visual acuity is significantly reduced in the affected eye. Fundoscopic examination is limited due to the corneal edema.

## Encounter 4: source 2022-07-15, practice 2023-12-12, procedure, Cardiology

Chief complaint: Scheduled coronary angiography following positive stress test

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David Kim, MD
Department: Cardiology
Visit Type: procedure

CHIEF COMPLAINT:
Scheduled coronary angiography following positive stress test

HISTORY OF PRESENT ILLNESS:
This 68-year-old male with longstanding Type 2 Diabetes Mellitus, chronic atrial fibrillation, and history of diabetic motor neuropathy and Zenker diverticulum presents for scheduled coronary angiography. He denies any chest pain but reports occasional exertional fatigue.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetic motor neuropathy (diagnosed 2020-01-15)
- Basal Cell Carcinoma (Nodular type, of nose) (diagnosed 2020-07-15)
- Zenker diverticulum (diagnosed 2021-01-15)
- Acute postoperative endophthalmitis (diagnosed 2021-08-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension
- Chronic atrial fibrillation
- Diabetic peripheral neuropathy

He has a history of hypertension and type 2 diabetes mellitus
He does not have a history of chronic kidney disease,.

PAST SURGICAL HISTORY:
- Cataract extraction, right eye (3 weeks prior)
- Coronary angiography (1 week prior)
- Tonsillectomy and adenoidectomy, age 7

MEDICATIONS:
He takes managed with lisinopril and metformin, respectively.

Home Medications:
- Hydrochlorothiazide 12.5 mg PO daily
- Warfarin 5 mg PO daily (dose adjusted per INR)
- Atorvastatin 40 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 72
- Mother: Hypertension, basal cell carcinoma
- Paternal uncle: Pancreatic adenocarcinoma

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Social, approximately 1-2 drinks per week. Occupation: Retired construction foreman.

REVIEW OF SYSTEMS:
His urine output has been normal.

VITAL SIGNS:
His blood pressure is 128/76 mmHg, heart rate is 76/min,

LABORATORY DATA:
Creatinine is 1.5 mg/dL.

PLAN:
The procedure will require iodinated contrast.

## Encounter 5: source 2023-01-15, practice 2024-06-28, ed, Internal Medicine

Chief complaint: Non-healing ulcer on left foot for 3 weeks with redness and foul odor

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James Rodriguez, MD
Department: Internal Medicine
Visit Type: ed

CHIEF COMPLAINT:
Non-healing ulcer on left foot for 3 weeks with redness and foul odor

HISTORY OF PRESENT ILLNESS:
This patient with Type 2 Diabetes Mellitus of 15 years and prior diabetic motor neuropathy presents with a non-healing foot ulcer. He reports that the ulcer began as a small blister after wearing new shoes but has progressively enlarged and deepened. He denies fever or chills but notes increased redness and a foul odor from the wound.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetic motor neuropathy (diagnosed 2020-01-15)
- Basal Cell Carcinoma (Nodular type, of nose) (diagnosed 2020-07-15)
- Zenker diverticulum (diagnosed 2021-01-15)
- Acute postoperative endophthalmitis (diagnosed 2021-08-15)
- Venous Air Embolism (diagnosed 2022-07-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension
- Chronic atrial fibrillation
- Diabetic peripheral neuropathy

PAST SURGICAL HISTORY:
- Cataract extraction, right eye (3 weeks prior)
- Coronary angiography (1 week prior)
- Tonsillectomy and adenoidectomy, age 7

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Hydrochlorothiazide 12.5 mg PO daily
- Warfarin 5 mg PO daily (dose adjusted per INR)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 72
- Mother: Hypertension, basal cell carcinoma
- Paternal uncle: Pancreatic adenocarcinoma

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Social, approximately 1-2 drinks per week. Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
On physical examination, the ulcer is 3 cm in diameter with a purulent base and surrounding erythema. There is no crepitus, but the patient has diminished sensation to light touch in both feet. Pedal pulses are weak bilaterally.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 14,000 /µL
- Hemoglobin A1c: 9.2%
- Erythrocyte sedimentation rate: 70 mm/hr
- C-reactive protein: 15 mg/dL

IMAGING:
Plain radiographs of the foot: cortical bone erosion adjacent to the ulcer

## Encounter 6: source 2023-07-15, practice 2024-12-25, ed, General Surgery

Chief complaint: Sudden onset acute abdominal pain localized to the right lower quadrant

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Maria Gonzalez, MD
Department: General Surgery
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset acute abdominal pain localized to the right lower quadrant

HISTORY OF PRESENT ILLNESS:
This 69-year-old diabetic with prior foot infection presents with abdominal pain. He reports that the pain began suddenly 6 hours ago and is localized to the right lower quadrant. He denies any nausea, vomiting, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetic motor neuropathy (diagnosed 2020-01-15)
- Basal Cell Carcinoma (Nodular type, of nose) (diagnosed 2020-07-15)
- Zenker diverticulum (diagnosed 2021-01-15)
- Acute postoperative endophthalmitis (diagnosed 2021-08-15)
- Venous Air Embolism (diagnosed 2022-07-15)
- Diabetic foot infection with osteomyelitis (diagnosed 2023-01-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension
- Chronic atrial fibrillation
- Diabetic peripheral neuropathy

His past medical history is significant for hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Cataract extraction, right eye (3 weeks prior)
- Coronary angiography (1 week prior)
- Tonsillectomy and adenoidectomy, age 7

MEDICATIONS:
He takes warfarin therapy.

Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 72
- Mother: Hypertension, basal cell carcinoma
- Paternal uncle: Pancreatic adenocarcinoma

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Social, approximately 1-2 drinks per week. Occupation: Retired construction foreman.

VITAL SIGNS:
Temperature is 38.1°C (100.6°F), blood pressure is 145/90 mmHg, pulse is 85/min and irregularly irregular, and respiratory rate is 18/min

PHYSICAL EXAMINATION:
Abdominal examination reveals tenderness and rebound tenderness in the right lower quadrant.

LABORATORY DATA:
- Hemoglobin: 13.2 g/dL
- White blood cell count: 14,000/mm³
- INR: 3.8
- Creatinine: 1.1 mg/dL
- Blood urea nitrogen: 21 mg/dL

IMAGING:
An abdominal CT scan shows an appendiceal abscess.

PLAN:
The surgical team is consulted and recommends urgent appendectomy given the risk of rupture. Given the patient&#x27;s elevated INR and the need for urgent surgery,

## Encounter 7: source 2024-07-15, practice 2025-12-29, outpatient, Gastroenterology

Chief complaint: Constant gnawing epigastric pain for 3 months with weight loss and new jaundice

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Robert Chang, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Constant gnawing epigastric pain for 3 months with weight loss and new jaundice

HISTORY OF PRESENT ILLNESS:
70-year-old male with Type 2 Diabetes, hypertension, atrial fibrillation, neuropathy, and Zenker diverticulum presents with significant weight loss, decreased appetite, and recent onset of jaundice.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetic motor neuropathy (diagnosed 2020-01-15)
- Basal Cell Carcinoma (Nodular type, of nose) (diagnosed 2020-07-15)
- Zenker diverticulum (diagnosed 2021-01-15)
- Acute postoperative endophthalmitis (diagnosed 2021-08-15)
- Venous Air Embolism (diagnosed 2022-07-15)
- Diabetic foot infection with osteomyelitis (diagnosed 2023-01-15)
- Perianal abscess (diagnosed 2023-07-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension
- Chronic atrial fibrillation
- Diabetic peripheral neuropathy

His past medical history includes type 2 diabetes mellitus and hypertension.

PAST SURGICAL HISTORY:
- Cataract extraction, right eye (3 weeks prior)
- Coronary angiography (1 week prior)
- Tonsillectomy and adenoidectomy, age 7

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Warfarin 5 mg PO daily (dose adjusted per INR)
- Atorvastatin 40 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 72
- Mother: Hypertension, basal cell carcinoma
- Paternal uncle: Pancreatic adenocarcinoma

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Social, approximately 1-2 drinks per week. Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
On physical examination, he appears cachectic. His skin and sclerae are icteric. Abdominal examination reveals a nontender, distended gallbladder.

LABORATORY DATA:
Laboratory tests show:

- Total bilirubin: 5.2 mg/dL (normal: 0.3-1.2 mg/dL)
- Direct bilirubin: 3.8 mg/dL (normal: 0.0-0.3 mg/dL)
- Alkaline phosphatase: 420 U/L (normal: 40-120 U/L)
- Aspartate aminotransferase (AST): 85 U/L (normal: 10-40 U/L)
- Alanine aminotransferase (ALT): 90 U/L (normal: 7-56 U/L)
- CA 19-9: 150 U/mL (normal: <37 U/mL)

IMAGING:
Abdominal ultrasound shows dilated intrahepatic and extrahepatic bile ducts with no gallstones. A hypoechoic mass is noted in the head of the pancreas.
