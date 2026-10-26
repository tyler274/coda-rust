# synthetic_hospital patient 2109

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 55 at first encounter, sex M, Non-Hispanic White, Medicare (primary) with Blue Cross Blue Shield supplemental.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-10-21 (Fri) | follow_up | Vascular Surgery |
| 1 | 2020-07-15 | 2023-04-27 (Thu) | outpatient | Neurology |
| 2 | 2021-01-15 | 2023-11-06 (Mon) | outpatient | Nephrology |
| 3 | 2022-01-15 | 2024-10-21 (Mon) | outpatient | Cardiology |
| 4 | 2023-01-15 | 2025-10-25 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare (primary) with Blue Cross Blue Shield supplemental",
 "occupation": "Rancher",
 "alcohol_use": "Rare (holidays only, 1-2 drinks)",
 "family_history": [
  "Father: Hypertension, myocardial infarction at age 68",
  "Mother: Type 2 diabetes mellitus, chronic kidney disease",
  "Brother: Hyperlipidemia",
  "Paternal grandfather: Congestive heart failure"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "667 mg by mouth three times daily with meals",
   "name": "Calcium acetate"
  },
  {
   "dose": "800 mg by mouth three times daily with meals",
   "name": "Sevelamer carbonate"
  },
  {
   "dose": "10,000 units subcutaneously monthly",
   "name": "Epoetin alfa"
  },
  {
   "dose": "10 mg by mouth daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg by mouth daily",
   "name": "Furosemide"
  },
  {
   "dose": "81 mg by mouth daily",
   "name": "Aspirin"
  },
  {
   "dose": "1 tablet by mouth daily",
   "name": "Multivitamin"
  },
  {
   "dose": "500 mg by mouth as needed for pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Left forearm arteriovenous fistula creation",
  "Wide local excision of soft-tissue sarcoma, right thigh"
 ],
 "chronic_conditions": [
  "End-stage renal disease on maintenance hemodialysis",
  "Chemotherapy-induced cardiomyopathy with left ventricular dysfunction (s/p doxorubicin for soft-tissue sarcoma)",
  "Carpal tunnel syndrome, right hand",
  "Anemia of chronic kidney disease",
  "History of soft-tissue sarcoma of the right thigh (status post resection, in remission)"
 ],
 "age_at_first_encounter": 55
}
```

Primary diagnoses: ["Acute decompensated heart failure", "Arteriovenous fistula", "Carpal tunnel syndrome", "Trastuzumab-induced cardiotoxicity with asymptomatic left ventricular dysfunction", "Anemia of chronic kidney disease"]
Comorbidities: ["End-stage renal disease on hemodialysis", "Hyperphosphatemia"]

## Encounter 0: source 2020-01-15, practice 2022-10-21, follow_up, Vascular Surgery

Chief complaint: Routine post-operative evaluation two weeks after arteriovenous fistula creation in the left forearm for hemodialysis access, with new-onset fatigue and mild exertional dyspnea.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. James Chen, MD
Department: Vascular Surgery
Visit Type: follow_up

CHIEF COMPLAINT:
Routine post-operative evaluation two weeks after arteriovenous fistula creation in the left forearm for hemodialysis access, with new-onset fatigue and mild exertional dyspnea.

HISTORY OF PRESENT ILLNESS:
Two weeks after surgical creation of an arteriovenous (AV) fistula in his left forearm for hemodialysis access. He reports new-onset fatigue and mild shortness of breath while walking up stairs, which he attributes to anemia. He denies chest pain, edema, or orthopnea. Compared to his presurgical state,

PAST MEDICAL HISTORY:
Active Problem List:
- End-stage renal disease on maintenance hemodialysis
- Chemotherapy-induced cardiomyopathy with left ventricular dysfunction (s/p doxorubicin for soft-tissue sarcoma)
- Carpal tunnel syndrome, right hand
- Anemia of chronic kidney disease
- History of soft-tissue sarcoma of the right thigh (status post resection, in remission)

PAST SURGICAL HISTORY:
Surgical creation of an arteriovenous (AV) fistula in his left forearm for hemodialysis access

MEDICATIONS:
Home Medications:
- Calcium acetate 667 mg by mouth three times daily with meals
- Sevelamer carbonate 800 mg by mouth three times daily with meals
- Epoetin alfa 10,000 units subcutaneously monthly
- Lisinopril 10 mg by mouth daily
- Furosemide 40 mg by mouth daily
- Aspirin 81 mg by mouth daily
- Multivitamin 1 tablet by mouth daily
- Acetaminophen 500 mg by mouth as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension, myocardial infarction at age 68
- Mother: Type 2 diabetes mellitus, chronic kidney disease
- Brother: Hyperlipidemia
- Paternal grandfather: Congestive heart failure

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (holidays only, 1-2 drinks). Occupation: Rancher.

VITAL SIGNS:
His temperature is 98.7°F (37.1°C), heart rate is 104/min, blood pressure is 132/64 mmHg, respiratory rate is 16/min, and oxygen saturation is 98% on room air.

PHYSICAL EXAMINATION:
On exam, his extremities are warm with bounding peripheral pulses, and there is a prominent bruit over the AV fistula site. His apex beat is displaced laterally.

## Encounter 1: source 2020-07-15, practice 2023-04-27, outpatient, Neurology

Chief complaint: Six-month history of intermittent numbness, tingling, and burning pain in the right hand, worse at night and during dialysis sessions, affecting the thumb, index, and middle fingers.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Maria Rodriguez, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of intermittent numbness, tingling, and burning pain in the right hand, worse at night and during dialysis sessions, affecting the thumb, index, and middle fingers.

HISTORY OF PRESENT ILLNESS:
The patient is a 55-year-old male with end-stage renal disease on maintenance hemodialysis, chemotherapy-induced cardiomyopathy, and a history of soft-tissue sarcoma status post resection, who presents to the Neurology clinic six months after undergoing arteriovenous fistula creation. He presents with a 6-month history of intermittent numbness, tingling, and burning pain in his right hand. He reports that these symptoms are worse at night and during his dialysis sessions. He also notes that the symptoms seem to be more pronounced in the thumb, index, and middle fingers. The patient has been on dialysis for 3 years and has an arteriovenous fistula in his right arm.

PAST MEDICAL HISTORY:
Active Problem List:
- Arteriovenous fistula (diagnosed 2020-01-15)
- End-stage renal disease on maintenance hemodialysis
- Chemotherapy-induced cardiomyopathy with left ventricular dysfunction (s/p doxorubicin for soft-tissue sarcoma)
- Carpal tunnel syndrome, right hand
- Anemia of chronic kidney disease
- History of soft-tissue sarcoma of the right thigh (status post resection, in remission)

PAST SURGICAL HISTORY:
- Left forearm arteriovenous fistula creation
- Wide local excision of soft-tissue sarcoma, right thigh

MEDICATIONS:
Home Medications:
- Calcium acetate 667 mg by mouth three times daily with meals
- Sevelamer carbonate 800 mg by mouth three times daily with meals
- Epoetin alfa 10,000 units subcutaneously monthly
- Lisinopril 10 mg by mouth daily
- Furosemide 40 mg by mouth daily
- Aspirin 81 mg by mouth daily
- Multivitamin 1 tablet by mouth daily
- Acetaminophen 500 mg by mouth as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension, myocardial infarction at age 68
- Mother: Type 2 diabetes mellitus, chronic kidney disease
- Brother: Hyperlipidemia
- Paternal grandfather: Congestive heart failure

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (holidays only, 1-2 drinks). Occupation: Rancher.

PHYSICAL EXAMINATION:
On physical examination, Tinel&#x27;s sign is positive over the right carpal tunnel, and Phalen&#x27;s test reproduces his symptoms after 30 seconds. Sensation is decreased in the median nerve distribution, and there is weakness of thumb opposition. No edema or erythema is noted.

LABORATORY DATA:
The patient&#x27;s recent laboratory values are as follows:

- Serum creatinine: 6.5 mg/dL
- Blood urea nitrogen (BUN): 55 mg/dL
- Hemoglobin: 10.2 g/dL
- Calcium: 9.2 mg/dL
- Phosphorus: 5.4 mg/dL
- Parathyroid hormone (PTH): 300 pg/mL (elevated)

## Encounter 2: source 2021-01-15, practice 2023-11-06, outpatient, Nephrology

Chief complaint: Routine follow-up for ESRD with progressive fatigue over several weeks and exertional dyspnea limiting daily walks, without chest pain or palpitations.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. David Kim, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for ESRD with progressive fatigue over several weeks and exertional dyspnea limiting daily walks, without chest pain or palpitations.

HISTORY OF PRESENT ILLNESS:
The patient returns for routine nephrology follow-up of ESRD on maintenance hemodialysis and chemotherapy-induced cardiomyopathy, with prior arteriovenous fistula creation and neurology evaluation. He reports worsening fatigue over several weeks with exertional dyspnea limiting daily walks, denying chest pain, palpitations, or dizziness.

PAST MEDICAL HISTORY:
Active Problem List:
- Arteriovenous fistula (diagnosed 2020-01-15)
- Carpal tunnel syndrome (diagnosed 2020-07-15)
- End-stage renal disease on maintenance hemodialysis
- Chemotherapy-induced cardiomyopathy with left ventricular dysfunction (s/p doxorubicin for soft-tissue sarcoma)
- Carpal tunnel syndrome, right hand
- Anemia of chronic kidney disease
- History of soft-tissue sarcoma of the right thigh (status post resection, in remission)

PAST SURGICAL HISTORY:
- Left forearm arteriovenous fistula creation
- Wide local excision of soft-tissue sarcoma, right thigh

MEDICATIONS:
His medications include calcium acetate, sevelamer, and a monthly erythropoiesis-stimulating agent (ESA).

Home Medications:
- Sevelamer carbonate 800 mg by mouth three times daily with meals
- Epoetin alfa 10,000 units subcutaneously monthly
- Lisinopril 10 mg by mouth daily
- Furosemide 40 mg by mouth daily
- Aspirin 81 mg by mouth daily
- Multivitamin 1 tablet by mouth daily
- Acetaminophen 500 mg by mouth as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension, myocardial infarction at age 68
- Mother: Type 2 diabetes mellitus, chronic kidney disease
- Brother: Hyperlipidemia
- Paternal grandfather: Congestive heart failure

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (holidays only, 1-2 drinks). Occupation: Rancher.

VITAL SIGNS:
His blood pressure is 138/82 mmHg, and heart rate is 88 beats per minute.

PHYSICAL EXAMINATION:
Physical examination reveals pallor and no other abnormalities.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 8.2 g/dL (Normal: 13.8-17.2 g/dL for men)
- Mean corpuscular volume (MCV): 86 fL (Normal: 80-96 fL)
- Serum ferritin: 100 ng/mL (Normal: 30-400 ng/mL)
- Transferrin saturation: 15% (Normal: 20-50%)
- Serum creatinine: 6.5 mg/dL (Normal: 0.6-1.3 mg/dL)
- Blood urea nitrogen (BUN): 55 mg/dL (Normal: 7-20 mg/dL)

## Encounter 3: source 2022-01-15, practice 2024-10-21, outpatient, Cardiology

Chief complaint: Gradually progressive shortness of breath over several months with orthopnea requiring extra pillows and paroxysmal nocturnal dyspnea with nighttime coughing fits.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Sarah Johnson, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Gradually progressive shortness of breath over several months with orthopnea requiring extra pillows and paroxysmal nocturnal dyspnea with nighttime coughing fits.

HISTORY OF PRESENT ILLNESS:
The patient is a 57-year-old male with end-stage renal disease on maintenance hemodialysis and chemotherapy-induced cardiomyopathy following prior doxorubicin therapy for soft-tissue sarcoma, returning to the Cardiology clinic with progressive cardiopulmonary symptoms. Since his Nephrology evaluation one year prior for exertional dyspnea limiting daily walks, his symptoms have gradually worsened. He reports that he notices shortness of breath most prominently when walking the fence line after breakfast. He has begun propping himself up with extra pillows because lying flat makes his breathing feel heavy. He reports waking at night with sudden coughing fits that settle after he sits by the open window. He has also noticed swelling around his ankles by evening that makes his boots tighter than usual. His appetite is fair, but he feels more easily fatigued doing chores that never used to bother him. He denies fever or recent upper respiratory symptoms. He describes a vague, dull sense of chest pressure with exertion that is not sharp and does not change with breathing. He does not recall any palpitations or episodes of fainting. He denies recent viral illnesses, tick bites, or new rashes. He states his weight has been fairly steady, although his belt has needed a new notch lately. He denies orthostatic dizziness but admits to feeling slowed down. He reports that lately dawns feel heavier and fence lines feel longer. He now pauses twice to catch his breath before he reaches the old cottonwood. Climbing the windmill ladder, once a warmup, has become a task he plans his day around. Neighbors have commented that he looks puffy in the face. He denies pleuritic pain or relief when he leans forward. He has not had recent viral gastroenteritis or flu-like illness. He has not been back to the hospital recently except for routine checkups. He has had no leg pain when walking on level ground and no calf tenderness. He denies recent prolonged travel. After a brief rest in the clinic, he states he feels about the same and is eager to get back to the ranch before sundown. He inquires what this might mean for his work, his cattle, and his nights under the sky.

PAST MEDICAL HISTORY:
Active Problem List:
- Arteriovenous fistula (diagnosed 2020-01-15)
- Carpal tunnel syndrome (diagnosed 2020-07-15)
- Anemia of chronic kidney disease (diagnosed 2021-01-15)
- End-stage renal disease on maintenance hemodialysis
- Chemotherapy-induced cardiomyopathy with left ventricular dysfunction (s/p doxorubicin for soft-tissue sarcoma)
- Carpal tunnel syndrome, right hand
- History of soft-tissue sarcoma of the right thigh (status post resection, in remission)

He did not receive chest radiation. He has not had chest trauma and has never undergone cardiac surgery.

PAST SURGICAL HISTORY:
- Left forearm arteriovenous fistula creation
- Wide local excision of soft-tissue sarcoma, right thigh

MEDICATIONS:
He takes a daily multivitamin and occasionally an over-the-counter pain reliever for back soreness. He does not use anabolic steroids or herbal supplements.

Home Medications:
- Calcium acetate 667 mg by mouth three times daily with meals
- Sevelamer carbonate 800 mg by mouth three times daily with meals
- Epoetin alfa 10,000 units subcutaneously monthly
- Lisinopril 10 mg by mouth daily
- Furosemide 40 mg by mouth daily
- Aspirin 81 mg by mouth daily
- Acetaminophen 500 mg by mouth as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His father died of “old age” in his late 80s, and his mother is living with osteoarthritis. No one in his family has been told they have hypertrophic cardiomyopathy or sudden cardiac death.

SOCIAL HISTORY:
He says he has cut back on alcohol over the last decade and now drinks only on holidays. He has never used cocaine or other stimulants. He does not smoke and has not smoked in the past. He eats a simple diet of beans, eggs, and beef when available. He prefers his coffee black, without milk or sugar. But he says wife prefers her coffee with milk. His job keeps him outdoors in all seasons. He has lived most of his adult life on a ranch in Nevada. The silence of the ranch he says is broken only by the lowing of cows and the steady hum of the wind sweeping across open plains. At night, under a sky scattered with a thousand stars, he sometimes feels both small and infinite, as if the desert itself has claimed him as one of its own.

PHYSICAL EXAMINATION:
On exam today, he appears comfortable at rest but becomes winded while changing positions. His jugular veins are mildly distended when reclining and soften when he sits up. Lung exam reveals fine crackles at the bases that clear only partially with deep breaths. Cardiac exam reveals a laterally displaced apical impulse and a soft, low-pitched extra sound early in diastole. There is a holosystolic murmur best heard at the apex that radiates to the axilla. His abdomen is soft with mild right upper quadrant fullness and no tenderness. He has pitting edema to the mid-shins. Peripheral pulses are intact and symmetric.

IMAGING:
A chest image is performed and reviewed. Transthoracic echocardiography is arranged to assess structure and function.

OTHER STUDIES:
An electrocardiogram is obtained as part of the evaluation.

## Encounter 4: source 2023-01-15, practice 2025-10-25, ed, Emergency Medicine

Chief complaint: Two-month history of progressive difficulty breathing on exertion with bilateral lower extremity edema, nocturnal dyspnea, and increased fatigue noted by his wife.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Michael Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-month history of progressive difficulty breathing on exertion with bilateral lower extremity edema, nocturnal dyspnea, and increased fatigue noted by his wife.

HISTORY OF PRESENT ILLNESS:
The patient is a 58-year-old with end-stage renal disease on hemodialysis and chemotherapy-induced cardiomyopathy presenting with progression of symptoms. His wife says he has been more fatigued and occasionally wakes up at night feeling short of breath. He denies chest pain, cough, or recent fever. He has not had any recent infections or changes in his dialysis schedule.

PAST MEDICAL HISTORY:
Active Problem List:
- Arteriovenous fistula (diagnosed 2020-01-15)
- Carpal tunnel syndrome (diagnosed 2020-07-15)
- Anemia of chronic kidney disease (diagnosed 2021-01-15)
- Trastuzumab-induced cardiotoxicity with asymptomatic left ventricular dysfunction (diagnosed 2022-01-15)
- End-stage renal disease on maintenance hemodialysis
- Chemotherapy-induced cardiomyopathy with left ventricular dysfunction (s/p doxorubicin for soft-tissue sarcoma)
- Carpal tunnel syndrome, right hand
- History of soft-tissue sarcoma of the right thigh (status post resection, in remission)

He has chronic kidney disease and undergoes hemodialysis three times weekly through a surgically created arteriovenous fistula in his left forearm.

PAST SURGICAL HISTORY:
- Left forearm arteriovenous fistula creation
- Wide local excision of soft-tissue sarcoma, right thigh

MEDICATIONS:
Home Medications:
- Calcium acetate 667 mg by mouth three times daily with meals
- Sevelamer carbonate 800 mg by mouth three times daily with meals
- Epoetin alfa 10,000 units subcutaneously monthly
- Lisinopril 10 mg by mouth daily
- Furosemide 40 mg by mouth daily
- Aspirin 81 mg by mouth daily
- Multivitamin 1 tablet by mouth daily
- Acetaminophen 500 mg by mouth as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension, myocardial infarction at age 68
- Mother: Type 2 diabetes mellitus, chronic kidney disease
- Brother: Hyperlipidemia
- Paternal grandfather: Congestive heart failure

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (holidays only, 1-2 drinks). Occupation: Rancher.

VITAL SIGNS:
On examination, his blood pressure is 138/72 mm Hg, heart rate is 102/min, and respiratory rate is 20/min

PHYSICAL EXAMINATION:
Jugular venous pulsations are visible at 45°. Cardiac auscultation reveals a soft systolic murmur over the precordium, and there is bilateral pitting edema up to the knees. Lungs have bibasilar crackles. There is a palpable thrill over the left forearm fistula site.
