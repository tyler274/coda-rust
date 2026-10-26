# synthetic_hospital patient 2258

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2019-10-03 (Thu) | outpatient | Neurology |
| 1 | 2020-03-15 | 2019-12-09 (Mon) | inpatient | Orthopedic Surgery |
| 2 | 2021-03-15 | 2020-12-10 (Thu) | ed | Emergency Medicine |
| 3 | 2022-03-15 | 2021-11-25 (Thu) | ed | Emergency Medicine |
| 4 | 2023-03-15 | 2022-11-28 (Mon) | ed | Emergency Medicine |
| 5 | 2024-03-15 | 2023-12-09 (Sat) | ed | Emergency Medicine |
| 6 | 2025-03-15 | 2024-12-03 (Tue) | icu | Cardiology |
| 7 | 2026-03-15 | 2025-11-25 (Tue) | outpatient | Neurology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired high school mathematics teacher",
 "alcohol_use": "Social, 2-3 drinks per week",
 "family_history": [
  "Father with hypertension and coronary artery disease",
  "Mother with osteoporosis and hip fracture",
  "Brother with chronic gout",
  "Maternal grandmother with dementia"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 15 years ago, 20 pack-year history",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "12.5 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Donepezil"
  },
  {
   "dose": "300 mg PO daily",
   "name": "Allopurinol"
  },
  {
   "dose": "17 g PO daily as needed",
   "name": "Polyethylene glycol 3350"
  },
  {
   "dose": "500 mg/200 IU PO twice daily",
   "name": "Calcium carbonate with Vitamin D3"
  }
 ],
 "surgical_history": [
  "Open reduction and internal fixation of right femoral neck fracture"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Osteopenia",
  "Mild cognitive impairment",
  "Chronic tophaceous gout",
  "Chronic idiopathic constipation"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Flaccid paralysis", "Parkinson disease dementia", "Papillary muscle rupture", "Septic arthritis of the knee", "Ventricular septal defect", "Creutzfeldt-Jakob disease", "Intestinal obstruction requiring cross-sectional imaging", "Sigmoid volvulus"]
Comorbidities: ["Acute myocardial infarction", "Essential hypertension", "Mild cognitive impairment"]

## Encounter 0: source 2020-01-15, practice 2019-10-03, outpatient, Neurology

Chief complaint: Progressive memory loss, visual hallucinations, and gait instability with falls

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive memory loss, visual hallucinations, and gait instability with falls

HISTORY OF PRESENT ILLNESS:
He presents to the clinic accompanied by his wife. She reports that over the past year, he has been experiencing progressive memory loss, visual hallucinations, and episodes of staring into space. Recently, he has also developed a shuffling gait and has had two falls without loss of consciousness. His wife denies any fluctuation in his level of consciousness or lucid intervals.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Osteopenia
- Mild cognitive impairment
- Chronic tophaceous gout
- Chronic idiopathic constipation

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of right femoral neck fracture

MEDICATIONS:
His current medications include lisinopril and donepezil.

Home Medications:
- Hydrochlorothiazide 12.5 mg PO daily
- Allopurinol 300 mg PO daily
- Polyethylene glycol 3350 17 g PO daily as needed
- Calcium carbonate with Vitamin D3 500 mg/200 IU PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension and coronary artery disease
- Mother with osteoporosis and hip fracture
- Brother with chronic gout
- Maternal grandmother with dementia

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago, 20 pack-year history. Alcohol: Social, 2-3 drinks per week. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, the patient has a resting tremor, bradykinesia, and rigidity. His Mini-Mental State Examination score is 20/30.

IMAGING:
An MRI of the brain shows generalized atrophy without acute changes.

## Encounter 1: source 2020-03-15, practice 2019-12-09, inpatient, Orthopedic Surgery

Chief complaint: Displaced femoral neck fracture with postoperative flaccid paralysis

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Michael Rodriguez, MD
Department: Orthopedic Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Displaced femoral neck fracture with postoperative flaccid paralysis

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with Parkinson disease dementia and a history of gait instability with falls previously assessed in Neurology two months prior, returns for inpatient management following orthopedic surgery. He is admitted to the hospital following surgical repair of a displaced femoral neck fracture sustained during a fall at home. Postoperatively, he is kept non–weight-bearing on the affected side for 6 weeks to allow for osseous healing. Due to ongoing pain, limited mobility, and fear of falling again, the patient spends most of his time in bed and has been reluctant to engage in physical therapy sessions. Over the past few weeks, the nurses have noticed that the patient has become weaker, especially in his lower extremities, and now requires assistance with transfers and repositioning.

PAST MEDICAL HISTORY:
Active Problem List:
- Parkinson disease dementia (diagnosed 2020-01-15)
- Hypertension
- Osteopenia
- Mild cognitive impairment
- Chronic tophaceous gout
- Chronic idiopathic constipation

He has a history of hypertension, osteopenia, and mild cognitive impairment.

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of right femoral neck fracture

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Donepezil 5 mg PO daily
- Allopurinol 300 mg PO daily
- Polyethylene glycol 3350 17 g PO daily as needed
- Calcium carbonate with Vitamin D3 500 mg/200 IU PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension and coronary artery disease
- Mother with osteoporosis and hip fracture
- Brother with chronic gout
- Maternal grandmother with dementia

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago, 20 pack-year history. Alcohol: Social, 2-3 drinks per week. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
Repeat physical examination reveals significant bilateral thigh muscle wasting compared to admission, despite stable nutritional intake and no new neurologic deficits.

LABORATORY DATA:
Lab results are within normal limits, including creatine kinase and electrolytes.

ASSESSMENT AND PLAN:
The clinical team attributes his decline to prolonged disuse of skeletal muscle.

## Encounter 2: source 2021-03-15, practice 2020-12-10, ed, Emergency Medicine

Chief complaint: Worsening abdominal pain, distension, and obsipation

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening abdominal pain, distension, and obsipation

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old male with Parkinson disease dementia, prior femoral neck fracture with flaccid paralysis, and chronic idiopathic constipation who presents to the emergency department with a 48-hour history of worsening abdominal pain and distension. He reports multiple episodes of non-bilious vomiting and the inability to pass flatus or have a bowel movement.

PAST MEDICAL HISTORY:
Active Problem List:
- Parkinson disease dementia (diagnosed 2020-01-15)
- Flaccid paralysis (diagnosed 2020-03-15)
- Hypertension
- Osteopenia
- Mild cognitive impairment
- Chronic tophaceous gout
- Chronic idiopathic constipation

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of right femoral neck fracture

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg PO daily
- Donepezil 5 mg PO daily
- Allopurinol 300 mg PO daily
- Polyethylene glycol 3350 17 g PO daily as needed
- Calcium carbonate with Vitamin D3 500 mg/200 IU PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension and coronary artery disease
- Mother with osteoporosis and hip fracture
- Brother with chronic gout
- Maternal grandmother with dementia

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago, 20 pack-year history. Alcohol: Social, 2-3 drinks per week. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On examination, his abdomen is distended and tympanic to percussion with tenderness in the lower quadrants. There is no rebound tenderness or guarding. Rectal examination reveals an empty rectal vault.

LABORATORY DATA:
White blood cell count: 12,000 /µL
Hemoglobin: 13.5 g/dL
Potassium: 3.5 mEq/L
Sodium: 140 mEq/L
Chloride: 100 mEq/L
Bicarbonate: 24 mEq/L
Blood urea nitrogen: 18 mg/dL
Creatinine: 1.0 mg/dL

IMAGING:
An abdominal x-ray shows a dilated, inverted, U-shaped loop of bowel in the left lower quadrant with the apex pointing towards the right upper quadrant.

## Encounter 3: source 2022-03-15, practice 2021-11-25, ed, Emergency Medicine

Chief complaint: Severe pain, redness, and swelling in right knee

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. David Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe pain, redness, and swelling in right knee

HISTORY OF PRESENT ILLNESS:
This 69-year-old male with chronic tophaceous gout and history of femoral neck fracture presents to the emergency department with a 2-day history of severe pain, redness, and swelling in his right knee. He denies any recent trauma or injury to the knee.

PAST MEDICAL HISTORY:
Active Problem List:
- Parkinson disease dementia (diagnosed 2020-01-15)
- Flaccid paralysis (diagnosed 2020-03-15)
- Sigmoid volvulus (diagnosed 2021-03-15)
- Hypertension
- Osteopenia
- Mild cognitive impairment
- Chronic tophaceous gout
- Chronic idiopathic constipation

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of right femoral neck fracture

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Donepezil 5 mg PO daily
- Allopurinol 300 mg PO daily
- Polyethylene glycol 3350 17 g PO daily as needed
- Calcium carbonate with Vitamin D3 500 mg/200 IU PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension and coronary artery disease
- Mother with osteoporosis and hip fracture
- Brother with chronic gout
- Maternal grandmother with dementia

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago, 20 pack-year history. Alcohol: Social, 2-3 drinks per week. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
He has a low-grade fever of 100.4°F (38°C).

PHYSICAL EXAMINATION:
On physical examination, the knee is warm, erythematous, and tender to palpation with a decreased range of motion.

LABORATORY DATA:
Aspiration of the knee joint yields synovial fluid with the following characteristics:

 - Appearance: Cloudy
 - WBC count: 50,000/µL (75% neutrophils)
 - Negative birefringent crystals present
 - Gram stain: No organisms seen

## Encounter 4: source 2023-03-15, practice 2022-11-28, ed, Emergency Medicine

Chief complaint: Crampy abdominal pain, nausea, vomiting, and obstipation

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. Maria Gonzalez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Crampy abdominal pain, nausea, vomiting, and obstipation

HISTORY OF PRESENT ILLNESS:
This patient with chronic constipation and prior sigmoid volvulus presents with abdominal pain. He describes the pain as crampy, located around the umbilicus, and worsening with time. He denies any bowel movements or passage of gas since the onset of pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Parkinson disease dementia (diagnosed 2020-01-15)
- Flaccid paralysis (diagnosed 2020-03-15)
- Sigmoid volvulus (diagnosed 2021-03-15)
- Septic arthritis of the knee (diagnosed 2022-03-15)
- Hypertension
- Osteopenia
- Mild cognitive impairment
- Chronic tophaceous gout
- Chronic idiopathic constipation

PAST SURGICAL HISTORY:
A remote history of an open cholecystectomy

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Donepezil 5 mg PO daily
- Allopurinol 300 mg PO daily
- Polyethylene glycol 3350 17 g PO daily as needed
- Calcium carbonate with Vitamin D3 500 mg/200 IU PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension and coronary artery disease
- Mother with osteoporosis and hip fracture
- Brother with chronic gout
- Maternal grandmother with dementia

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago, 20 pack-year history. Alcohol: Social, 2-3 drinks per week. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
His vital signs are as follows: BP 150/90 mmHg, HR 102 bpm, RR 22 breaths/min, and T 37.8°C

PHYSICAL EXAMINATION:
On examination, his abdomen is distended, with high-pitched bowel sounds and tenderness on palpation. There is no rebound tenderness.

LABORATORY DATA:
Laboratory tests reveal a leukocytosis of 15,000 cells/µL.

IMAGING:
An upright abdominal x-ray shows multiple air-fluid levels and dilated loops of small bowel. A subsequent CT scan of the abdomen with contrast demonstrates a transition point in the distal ileum with no passage of contrast beyond, and there is evidence of free air in the peritoneal cavity.

## Encounter 5: source 2024-03-15, practice 2023-12-09, ed, Emergency Medicine

Chief complaint: Progressive fatigue, dyspnea on exertion, and lightheadedness

ENCOUNTER NOTE
Date: 2024-03-15
Provider: Dr. James Williams, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive fatigue, dyspnea on exertion, and lightheadedness

HISTORY OF PRESENT ILLNESS:
This 71-year-old with hypertension, cognitive impairment, and neurodegenerative and orthopedic history states that over the past 3 days, he has had increasing difficulty walking across the room without becoming winded. He also reports some lightheadedness and a vague sense of chest pressure.

PAST MEDICAL HISTORY:
Active Problem List:
- Parkinson disease dementia (diagnosed 2020-01-15)
- Flaccid paralysis (diagnosed 2020-03-15)
- Sigmoid volvulus (diagnosed 2021-03-15)
- Septic arthritis of the knee (diagnosed 2022-03-15)
- Intestinal obstruction requiring cross-sectional imaging (diagnosed 2023-03-15)
- Hypertension
- Osteopenia
- Mild cognitive impairment
- Chronic tophaceous gout
- Chronic idiopathic constipation

He has a history of hypertension and underwent a hospitalization for a “heart attack” approximately 6 days ago, during which he was treated medically and discharged.

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of right femoral neck fracture

MEDICATIONS:
He is currently taking aspirin, metoprolol, and lisinopril.

Home Medications:
- Hydrochlorothiazide 12.5 mg PO daily
- Donepezil 5 mg PO daily
- Allopurinol 300 mg PO daily
- Polyethylene glycol 3350 17 g PO daily as needed
- Calcium carbonate with Vitamin D3 500 mg/200 IU PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension and coronary artery disease
- Mother with osteoporosis and hip fracture
- Brother with chronic gout
- Maternal grandmother with dementia

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago, 20 pack-year history. Alcohol: Social, 2-3 drinks per week. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
On examination, his blood pressure is 106/64 mm Hg, heart rate is 102/min, and oxygen saturation is 93% on room air.

PHYSICAL EXAMINATION:
Jugular venous distension is noted, and bibasilar crackles are heard on lung auscultation. Cardiac examination reveals a systolic murmur. There is no peripheral edema.

OTHER STUDIES:
ECG shows sinus tachycardia without ST changes.

## Encounter 6: source 2025-03-15, practice 2024-12-03, icu, Cardiology

Chief complaint: Sudden worsening shortness of breath and acute confusion

ENCOUNTER NOTE
Date: 2025-03-15
Provider: Dr. Robert Kim, MD
Department: Cardiology
Visit Type: icu

CHIEF COMPLAINT:
Sudden worsening shortness of breath and acute confusion

HISTORY OF PRESENT ILLNESS:
This patient with Parkinson disease dementia and prior ventricular septal defect presents to the emergency department with worsening shortness of breath and confusion.

PAST MEDICAL HISTORY:
Active Problem List:
- Parkinson disease dementia (diagnosed 2020-01-15)
- Flaccid paralysis (diagnosed 2020-03-15)
- Sigmoid volvulus (diagnosed 2021-03-15)
- Septic arthritis of the knee (diagnosed 2022-03-15)
- Intestinal obstruction requiring cross-sectional imaging (diagnosed 2023-03-15)
- Ventricular septal defect (diagnosed 2024-03-15)
- Hypertension
- Osteopenia
- Mild cognitive impairment
- Chronic tophaceous gout
- Chronic idiopathic constipation

He has a history of hypertension and a myocardial infarction 5 days ago.

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of right femoral neck fracture

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Donepezil 5 mg PO daily
- Allopurinol 300 mg PO daily
- Polyethylene glycol 3350 17 g PO daily as needed
- Calcium carbonate with Vitamin D3 500 mg/200 IU PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension and coronary artery disease
- Mother with osteoporosis and hip fracture
- Brother with chronic gout
- Maternal grandmother with dementia

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago, 20 pack-year history. Alcohol: Social, 2-3 drinks per week. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
Oxygen saturation on room air is 87%. Blood pressure is 85/60 mm Hg, heart rate is 112/min, and respiratory rate is 28/min.

PHYSICAL EXAMINATION:
On examination, he is dyspneic, and auscultation reveals bibasilar crackles. A new, loud, systolic murmur is heard best at the apex and radiates toward the axilla. The jugular venous pressure is elevated. His extremities are cool, and he appears to be in distress.

IMAGING:
Chest x-ray reveals bilateral pulmonary infiltrates consistent with pulmonary edema.

OTHER STUDIES:
ECG shows sinus tachycardia without ST changes.

## Encounter 7: source 2026-03-15, practice 2025-11-25, outpatient, Neurology

Chief complaint: Rapidly progressive memory loss, confusion, and myoclonic jerks

ENCOUNTER NOTE
Date: 2026-03-15
Provider: Dr. Lisa Thompson, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Rapidly progressive memory loss, confusion, and myoclonic jerks

HISTORY OF PRESENT ILLNESS:
The patient returns to Neurology following prior neurologic encounters and recent cardiac events. He presents to the clinic with a 3-month history of rapidly progressive memory loss and confusion. His wife reports that he has been experiencing jerky movements of his limbs, particularly when startled.

PAST MEDICAL HISTORY:
Active Problem List:
- Parkinson disease dementia (diagnosed 2020-01-15)
- Flaccid paralysis (diagnosed 2020-03-15)
- Sigmoid volvulus (diagnosed 2021-03-15)
- Septic arthritis of the knee (diagnosed 2022-03-15)
- Intestinal obstruction requiring cross-sectional imaging (diagnosed 2023-03-15)
- Ventricular septal defect (diagnosed 2024-03-15)
- Papillary muscle rupture (diagnosed 2025-03-15)
- Hypertension
- Osteopenia
- Mild cognitive impairment
- Chronic tophaceous gout
- Chronic idiopathic constipation

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of right femoral neck fracture

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Donepezil 5 mg PO daily
- Allopurinol 300 mg PO daily
- Polyethylene glycol 3350 17 g PO daily as needed
- Calcium carbonate with Vitamin D3 500 mg/200 IU PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension and coronary artery disease
- Mother with osteoporosis and hip fracture
- Brother with chronic gout
- Maternal grandmother with dementia

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago, 20 pack-year history. Alcohol: Social, 2-3 drinks per week. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On examination, the patient appears disoriented and has difficulty following simple commands. Neurological examination reveals generalized myoclonus.

LABORATORY DATA:
Routine blood tests are normal. A lumbar puncture is performed, and cerebrospinal fluid analysis reveals elevated 14-3-3 proteins.

IMAGING:
An MRI of the brain shows no significant abnormalities.

OTHER STUDIES:
An electroencephalogram (EEG) demonstrates periodic sharp-wave complexes.
