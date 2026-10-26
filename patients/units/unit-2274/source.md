# synthetic_hospital patient 2274

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White (Northern European ancestry), Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-09-30 (Thu) | outpatient | Orthopedics |
| 1 | 2020-04-15 | 2022-01-11 (Tue) | outpatient | Hematology |
| 2 | 2021-07-15 | 2023-04-04 (Tue) | outpatient | Orthopedics |
| 3 | 2023-07-15 | 2025-04-12 (Sat) | ed | Emergency Medicine |
| 4 | 2024-01-15 | 2025-10-08 (Wed) | icu | Neurology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare",
 "occupation": "Retired postal worker",
 "alcohol_use": "Social (2-3 drinks on weekends)",
 "family_history": [
  "Father: Cerebrovascular accident at age 72",
  "Paternal uncle: Paget disease of bone",
  "Mother: Essential hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White (Northern European ancestry)",
 "smoking_status": "Current smoker (1 pack per day), relapsed 6 months ago after 20-year abstinence",
 "home_medications": [
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Essential hypertension",
  "Polycythemia vera",
  "Paget disease of bone (left femur and hip)",
  "Benign prostatic hyperplasia"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Essential hypertension", "Polycythemia vera", "Intracerebral hemorrhage with airway compromise", "Paget disease of bone"]
Comorbidities: ["Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2021-09-30, outpatient, Orthopedics

Chief complaint: Six-month history of dull, aching pain in left thigh and hip that is worse at night and unresponsive to over-the-counter analgesics.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of dull, aching pain in left thigh and hip that is worse at night and unresponsive to over-the-counter analgesics.

HISTORY OF PRESENT ILLNESS:
A 67-year-old male presents to the clinic with a 6-month history of increasing pain in his left thigh and hip. He describes the pain as a dull ache that is worse at night and does not improve with over-the-counter analgesics. He denies any recent trauma or injury.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension
- Polycythemia vera
- Paget disease of bone (left femur and hip)
- Benign prostatic hyperplasia

His medical history is significant for hypertension, controlled with hydrochlorothiazide, and benign prostatic hyperplasia.

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Cerebrovascular accident at age 72
- Paternal uncle: Paget disease of bone
- Mother: Essential hypertension

SOCIAL HISTORY:
Smoking: Current smoker (1 pack per day), relapsed 6 months ago after 20-year abstinence. Alcohol: Social (2-3 drinks on weekends). Occupation: Retired postal worker.

PHYSICAL EXAMINATION:
On physical examination, there is mild warmth and tenderness to palpation over the proximal left femur. There is no erythema or swelling. Neurovascular examination of the lower extremities is intact.

LABORATORY DATA:
Laboratory tests reveal:

- Serum calcium: 9.8 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum alkaline phosphatase: 450 U/L (normal: 40-130 U/L)
- Serum phosphate: 3.2 mg/dL (normal: 2.5-4.5 mg/dL)
- Serum creatinine: 1.0 mg/dL (normal: 0.6-1.2 mg/dL)

IMAGING:
A radiograph of the left femur shows thickened cortices and a mixed lytic and sclerotic appearance.

## Encounter 1: source 2020-04-15, practice 2022-01-11, outpatient, Hematology

Chief complaint: Persistent fatigue, headache, upper abdominal fullness, burning erythema of hands and feet, and severe pruritus after hot showers.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. David Kim, MD
Department: Hematology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue, headache, upper abdominal fullness, burning erythema of hands and feet, and severe pruritus after hot showers.

HISTORY OF PRESENT ILLNESS:
The patient with Polycythemia vera returns to Hematology following his January Orthopedics visit for Paget disease of the left femur and hip. He has noted intermittent redness and a burning sensation in his hands and feet that improves with exposure to cool water. Recently, he has begun experiencing mild pruritus, especially after hot showers, and reports occasional episodes of blurred vision. He denies fever, weight loss, night sweats, or recent infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Paget disease of bone (diagnosed 2020-01-15)
- Essential hypertension
- Polycythemia vera
- Paget disease of bone (left femur and hip)
- Benign prostatic hyperplasia

His past medical history is notable only for well-controlled hypertension.

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Cerebrovascular accident at age 72
- Paternal uncle: Paget disease of bone
- Mother: Essential hypertension

SOCIAL HISTORY:
He does not smoke and drinks alcohol socially on weekends.

VITAL SIGNS:
His vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, the patient appears flushed. The spleen is palpable just below the left costal margin. Fundoscopic examination reveals engorged retinal veins. Cardiopulmonary and neurologic examinations are unremarkable.

LABORATORY DATA:
Labs show:
Hemoglobin                                                20.8 g/dL
Hematocrit                                                 62%
MCV                                                          90 fL
White blood cell count                               15,500/mm³
Platelet count                                           710,000/mm³
Leukocyte alkaline phosphatase                 Elevated
Serum erythropoietin level                       Decreased

## Encounter 2: source 2021-07-15, practice 2023-04-04, outpatient, Orthopedics

Chief complaint: Progressive left hip pain with new onset left leg warmth and difficulty fitting left shoe due to bone enlargement.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Sarah Chen, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive left hip pain with new onset left leg warmth and difficulty fitting left shoe due to bone enlargement.

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of known Paget disease of the left femur and hip. He presents with a 6-month history of increasing left hip pain, especially at night. He also reports that his left leg feels warmer than the right and that he has noticed his shoes do not fit as well as they used to, with the left shoe feeling particularly tight.

PAST MEDICAL HISTORY:
Active Problem List:
- Paget disease of bone (diagnosed 2020-01-15)
- Polycythemia vera (diagnosed 2020-04-15)
- Essential hypertension
- Paget disease of bone (left femur and hip)
- Benign prostatic hyperplasia

MEDICATIONS:
He takes hydrochlorothiazide

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Cerebrovascular accident at age 72
- Paternal uncle: Paget disease of bone
- Mother: Essential hypertension

SOCIAL HISTORY:
Smoking: Current smoker (1 pack per day), relapsed 6 months ago after 20-year abstinence. Alcohol: Social (2-3 drinks on weekends). Occupation: Retired postal worker.

PHYSICAL EXAMINATION:
On physical examination, the left leg appears slightly larger than the right, and there is tenderness to palpation over the left hip. The skin over the left lower extremity is warm to the touch.

LABORATORY DATA:
Serum calcium: 9.8 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum alkaline phosphatase: 220 U/L (normal: 40-130 U/L)
- Serum phosphate: 3.5 mg/dL (normal: 2.5-4.5 mg/dL)
- Serum creatinine: 1.0 mg/dL (normal: 0.6-1.2 mg/dL)
- Urine hydroxyproline: Elevated

## Encounter 3: source 2023-07-15, practice 2025-04-12, ed, Emergency Medicine

Chief complaint: Sudden onset of left-sided weakness and difficulty speaking that began two hours ago, preceded by dropping a coffee cup and falling.

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Maria Garcia, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of left-sided weakness and difficulty speaking that began two hours ago, preceded by dropping a coffee cup and falling.

HISTORY OF PRESENT ILLNESS:
This 70-year-old male with polycythemia vera and progressive Paget disease of the left femur and hip presents acutely. His wife reports that he was in his usual state of health until this morning when he suddenly dropped a cup of coffee from his left hand and then fell to the ground.

PAST MEDICAL HISTORY:
Active Problem List:
- Paget disease of bone (diagnosed 2020-01-15)
- Polycythemia vera (diagnosed 2020-04-15)
- Essential hypertension
- Paget disease of bone (left femur and hip)
- Benign prostatic hyperplasia

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Cerebrovascular accident at age 72
- Paternal uncle: Paget disease of bone
- Mother: Essential hypertension

SOCIAL HISTORY:
Smoking: Current smoker (1 pack per day), relapsed 6 months ago after 20-year abstinence. Alcohol: Social (2-3 drinks on weekends). Occupation: Retired postal worker.

VITAL SIGNS:
His blood pressure is 220/120 mmHg.

PHYSICAL EXAMINATION:
On examination, the patient is alert but aphasic. Neurological examination reveals left-sided hemiparesis, left-sided sensory loss, and right conjugate gaze deviation.

IMAGING:
A non-contrast head CT scan shows a hyperdense region in the right basal ganglia with surrounding edema.

## Encounter 4: source 2024-01-15, practice 2025-10-08, icu, Neurology

Chief complaint: Sudden right facial and right arm weakness with slurred speech, rapidly progressing to loss of consciousness and neurological deterioration.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. James Patel, MD
Department: Neurology
Visit Type: icu

CHIEF COMPLAINT:
Sudden right facial and right arm weakness with slurred speech, rapidly progressing to loss of consciousness and neurological deterioration.

HISTORY OF PRESENT ILLNESS:
This 71-year-old male with essential hypertension, polycythemia vera, Paget disease of bone, and history of prior intracerebral hemorrhage is brought to the emergency department after his wife reports sudden weakness of the right face and right arm, accompanied by slurred speech, that began earlier that morning. Despite supportive care, his neurologic status deteriorates rapidly, and he dies within 24 hours.

PAST MEDICAL HISTORY:
Active Problem List:
- Paget disease of bone (diagnosed 2020-01-15)
- Polycythemia vera (diagnosed 2020-04-15)
- Intracerebral hemorrhage with airway compromise (diagnosed 2023-07-15)
- Essential hypertension
- Paget disease of bone (left femur and hip)
- Benign prostatic hyperplasia

He has a history of chronic headaches and has not seen a physician in several years.

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Cerebrovascular accident at age 72
- Paternal uncle: Paget disease of bone
- Mother: Essential hypertension

SOCIAL HISTORY:
He smokes one pack of cigarettes daily

PHYSICAL EXAMINATION:
On arrival, he is confused with right-sided facial droop, hemiparesis predominantly affecting the arm, and expressive aphasia.

PATHOLOGY:
Autopsy reveals an occlusive thrombus in the left middle cerebral artery (MCA) and histologic examination of the kidneys demonstrates the findings shown below.
