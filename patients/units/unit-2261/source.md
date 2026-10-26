# synthetic_hospital patient 2261

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2018-12-17 (Mon) | outpatient | Hematology & Oncology |
| 1 | 2021-07-15 | 2020-06-15 (Mon) | ed | Emergency Medicine |
| 2 | 2022-07-15 | 2021-06-20 (Sun) | ed | Emergency Medicine |
| 3 | 2023-07-15 | 2022-06-19 (Sun) | ed | Emergency Medicine |
| 4 | 2024-07-15 | 2023-06-15 (Thu) | ed | Emergency Medicine |
| 5 | 2025-07-15 | 2024-06-15 (Sat) | ed | Emergency Medicine |
| 6 | 2026-07-15 | 2025-06-30 (Mon) | ed | Emergency Medicine |
| 7 | 2027-07-15 | 2026-06-24 (Wed) | icu | Intensive Care Unit |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare",
 "occupation": "Retired postal worker",
 "alcohol_use": "Social, 1-2 drinks per week",
 "family_history": [
  "Father: Myocardial infarction at age 72",
  "Mother: Ischemic stroke at age 75",
  "Brother: Deep vein thrombosis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 30 pack-year history",
 "home_medications": [
  {
   "dose": "5 mg daily, titrated to INR goal 2.0-3.0",
   "name": "Warfarin"
  },
  {
   "dose": "10 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "50 mg daily",
   "name": "Metoprolol succinate"
  }
 ],
 "surgical_history": [
  "Right inguinal hernia repair",
  "Thoracic epidural steroid injection"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Chronic atrial fibrillation",
  "History of ischemic stroke",
  "Polycythemia vera"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Acute ischemic stroke", "Atrial fibrillation with rapid ventricular response", "Central retinal artery occlusion", "Chorea", "Intracerebral hemorrhage with airway compromise", "Polycythemia vera", "Postoperative hemorrhage following inguinal hernia repair", "Spinal epidural hematoma"]
Comorbidities: ["Chronic atrial fibrillation", "Medication noncompliance", "History of stroke", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2018-12-17, outpatient, Hematology & Oncology

Chief complaint: Increasing fatigue, headaches, and uncomfortable itching sensation after showers over the past 6 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing fatigue, headaches, and uncomfortable itching sensation after showers over the past 6 months

HISTORY OF PRESENT ILLNESS:
6-month history of increasing fatigue, headaches, and an uncomfortable itching sensation after showers. He also reports experiencing two episodes of transient visual disturbances in the past month.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Chronic atrial fibrillation
- History of ischemic stroke
- Polycythemia vera

PAST SURGICAL HISTORY:
- Right inguinal hernia repair
- Thoracic epidural steroid injection

MEDICATIONS:
He takes lisinopril.

Home Medications:
- Warfarin 5 mg daily, titrated to INR goal 2.0-3.0
- Metoprolol succinate 50 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 72
- Mother: Ischemic stroke at age 75
- Brother: Deep vein thrombosis

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-year history. Alcohol: Social, 1-2 drinks per week. Occupation: Retired postal worker.

PHYSICAL EXAMINATION:
On physical examination, the patient appears plethoric with a ruddy complexion. His spleen is palpable 3 cm below the left costal margin.

LABORATORY DATA:
Hemoglobin: 18.5 g/dL (Normal: 13.8-17.2 g/dL)
- Hematocrit: 55% (Normal: 40.7-50.3%)
- White blood cell count: 12,000 /µL (Normal: 4,500-11,000 /µL)
- Platelet count: 450,000 /µL (Normal: 150,000-450,000 /µL)
- Erythropoietin level: Low

## Encounter 1: source 2021-07-15, practice 2020-06-15, ed, Emergency Medicine

Chief complaint: Rapidly expanding, painful mass in the right groin area 6 hours after elective hernia repair

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Rapidly expanding, painful mass in the right groin area 6 hours after elective hernia repair

HISTORY OF PRESENT ILLNESS:
This patient with polycythemia vera presents. He underwent an elective right inguinal hernia repair 6 hours ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Polycythemia vera (diagnosed 2020-01-15)
- Hypertension
- Chronic atrial fibrillation
- History of ischemic stroke

His past medical history is significant for hypertension, which is well-controlled with lisinopril.

PAST SURGICAL HISTORY:
- Right inguinal hernia repair
- Thoracic epidural steroid injection

MEDICATIONS:
He takes lisinopril.

Home Medications:
- Warfarin 5 mg daily, titrated to INR goal 2.0-3.0
- Metoprolol succinate 50 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
He denies any personal or family history of easy bleeding or bruising.

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-year history. Alcohol: Social, 1-2 drinks per week. Occupation: Retired postal worker.

VITAL SIGNS:
On examination, his blood pressure is 135/85 mmHg, heart rate is 88 beats per minute, and temperature is 37.2°C (99.0°F).

PHYSICAL EXAMINATION:
The right groin is swollen, tense, and tender to palpation. There is no overlying skin erythema or warmth.

LABORATORY DATA:
His complete blood count, coagulation profile, and renal function tests are within normal limits.

Laboratory values:
- Hemoglobin: 14.2 g/dL
- Hematocrit: 42.5%
- Platelet count: 250,000/mm^3
- INR: 1.0
- aPTT: 30 seconds
- Creatinine: 0.9 mg/dL

## Encounter 2: source 2022-07-15, practice 2021-06-20, ed, Emergency Medicine

Chief complaint: Severe, localized thoracic back pain and progressive weakness in both legs for 6 hours

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Jennifer Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe, localized thoracic back pain and progressive weakness in both legs for 6 hours

HISTORY OF PRESENT ILLNESS:
The 70-year-old male on warfarin with polycythemia vera returns one year after hemorrhage with 6-hour history of severe, localized thoracic back pain and progressive weakness in both legs. He reports no recent trauma but had an epidural steroid injection for chronic back pain 2 days ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Polycythemia vera (diagnosed 2020-01-15)
- Postoperative hemorrhage following inguinal hernia repair (diagnosed 2021-07-15)
- Hypertension
- Chronic atrial fibrillation
- History of ischemic stroke

PAST SURGICAL HISTORY:
- Right inguinal hernia repair
- Thoracic epidural steroid injection

MEDICATIONS:
He has been on warfarin therapy.

Home Medications:
- Lisinopril 10 mg daily
- Metoprolol succinate 50 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 72
- Mother: Ischemic stroke at age 75
- Brother: Deep vein thrombosis

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-year history. Alcohol: Social, 1-2 drinks per week. Occupation: Retired postal worker.

VITAL SIGNS:
Vital signs are stable

PHYSICAL EXAMINATION:
Decreased sensation to pinprick below the level of the umbilicus and 3/5 strength in the bilateral lower extremities. Anal tone is decreased.

LABORATORY DATA:
INR is 3.5

IMAGING:
Urgent MRI of the thoracic spine is obtained and shows an epidural mass compressing the spinal cord

## Encounter 3: source 2023-07-15, practice 2022-06-19, ed, Emergency Medicine

Chief complaint: Sudden onset of left-sided weakness and slurred speech that began two hours ago

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of left-sided weakness and slurred speech that began two hours ago

PAST MEDICAL HISTORY:
Active Problem List:
- Polycythemia vera (diagnosed 2020-01-15)
- Postoperative hemorrhage following inguinal hernia repair (diagnosed 2021-07-15)
- Spinal epidural hematoma (diagnosed 2022-07-15)
- Hypertension
- Chronic atrial fibrillation
- History of ischemic stroke

PAST SURGICAL HISTORY:
- Right inguinal hernia repair
- Thoracic epidural steroid injection

MEDICATIONS:
He has been noncompliant with his medications, which include metoprolol and warfarin.

Home Medications:
- Lisinopril 10 mg daily
- Metoprolol succinate 50 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 72
- Mother: Ischemic stroke at age 75
- Brother: Deep vein thrombosis

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-year history. Alcohol: Social, 1-2 drinks per week. Occupation: Retired postal worker.

VITAL SIGNS:
On examination, his blood pressure is 160/90 mmHg, heart rate is irregularly irregular at 110 bpm,

PHYSICAL EXAMINATION:
He has a left facial droop with left upper and lower extremity weakness.

LABORATORY DATA:
His laboratory values are as follows:
- INR: 1.2
- PT: 11 seconds (control 12 seconds)
- aPTT: 30 seconds (control 30 seconds)
- CBC: within normal limits
- Serum creatinine: 1.1 mg/dL

IMAGING:
A non-contrast head CT scan shows no hemorrhage or mass effect.

## Encounter 4: source 2024-07-15, practice 2023-06-15, ed, Emergency Medicine

Chief complaint: Palpitations and mild dyspnea that started 2 hours ago

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Palpitations and mild dyspnea that started 2 hours ago

HISTORY OF PRESENT ILLNESS:
This 72-year-old male with chronic atrial fibrillation and prior ischemic stroke returns to the emergency department with complaints of palpitations and mild dyspnea that started 2 hours ago. He has been on warfarin for stroke prevention but admits to missing doses over the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Polycythemia vera (diagnosed 2020-01-15)
- Postoperative hemorrhage following inguinal hernia repair (diagnosed 2021-07-15)
- Spinal epidural hematoma (diagnosed 2022-07-15)
- Acute ischemic stroke (diagnosed 2023-07-15)
- Hypertension
- Chronic atrial fibrillation

PAST SURGICAL HISTORY:
- Right inguinal hernia repair
- Thoracic epidural steroid injection

MEDICATIONS:
He takes warfarin for stroke prevention.

Home Medications:
- Lisinopril 10 mg daily
- Metoprolol succinate 50 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 72
- Mother: Ischemic stroke at age 75
- Brother: Deep vein thrombosis

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-year history. Alcohol: Social, 1-2 drinks per week. Occupation: Retired postal worker.

VITAL SIGNS:
Blood pressure 150/90 mmHg, heart rate 130 bpm, respiratory rate 18 breaths/min, and oxygen saturation 97% on room air

PHYSICAL EXAMINATION:
Irregularly irregular pulse and no signs of heart failure

LABORATORY DATA:
His current INR is 1.5, below his therapeutic range of 2-3

OTHER STUDIES:
An electrocardiogram confirms atrial fibrillation with a rapid ventricular response

PLAN:
The decision is made to start intravenous heparin to achieve rapid anticoagulation

## Encounter 5: source 2025-07-15, practice 2024-06-15, ed, Emergency Medicine

Chief complaint: Acute vision loss in right eye described as a 'shade being pulled down' that began one hour ago

ENCOUNTER NOTE
Date: 2025-07-15
Provider: Dr. James Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute vision loss in right eye described as a 'shade being pulled down' that began one hour ago

HISTORY OF PRESENT ILLNESS:
This 73-year-old male has a history of polycythemia vera, chronic atrial fibrillation, and ischemic stroke. He presents to the emergency department with acute vision loss in his right eye that began one hour ago. He describes it as a "shade being pulled down" over his visual field, with no associated pain, floaters, or photophobia. He denies any history of similar episodes.

PAST MEDICAL HISTORY:
Active Problem List:
- Polycythemia vera (diagnosed 2020-01-15)
- Postoperative hemorrhage following inguinal hernia repair (diagnosed 2021-07-15)
- Spinal epidural hematoma (diagnosed 2022-07-15)
- Acute ischemic stroke (diagnosed 2023-07-15)
- Atrial fibrillation with rapid ventricular response (diagnosed 2024-07-15)
- Hypertension
- Chronic atrial fibrillation

His past medical history includes poorly controlled hypertension and a prior stroke five years ago, for which he was briefly on antiplatelet therapy but stopped taking it. He also reports intermittent episodes of palpitations but has not sought evaluation. He has no known history of diabetes.

PAST SURGICAL HISTORY:
- Right inguinal hernia repair
- Thoracic epidural steroid injection

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily, titrated to INR goal 2.0-3.0
- Lisinopril 10 mg daily
- Metoprolol succinate 50 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 72
- Mother: Ischemic stroke at age 75
- Brother: Deep vein thrombosis

SOCIAL HISTORY:
He has a 30 pack-year smoking history

PHYSICAL EXAMINATION:
Visual acuity in the right eye is reduced to light perception only. Intraocular pressure is within normal limits. A dilated fundoscopic examination is performed, and the image below is obtained.

## Encounter 6: source 2026-07-15, practice 2025-06-30, ed, Emergency Medicine

Chief complaint: Involuntary, violent, flinging movements of left arm and leg that started suddenly while watching television

ENCOUNTER NOTE
Date: 2026-07-15
Provider: Dr. Elena Vasquez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Involuntary, violent, flinging movements of left arm and leg that started suddenly while watching television

HISTORY OF PRESENT ILLNESS:
This male with chronic atrial fibrillation, polycythemia vera on warfarin, and prior ischemic stroke and central retinal artery occlusion is brought to the emergency department after suddenly developing involuntary, violent, flinging movements of his left arm and leg while watching television at home. His wife reports that he has no history of head trauma or similar prior episodes.

PAST MEDICAL HISTORY:
Active Problem List:
- Polycythemia vera (diagnosed 2020-01-15)
- Postoperative hemorrhage following inguinal hernia repair (diagnosed 2021-07-15)
- Spinal epidural hematoma (diagnosed 2022-07-15)
- Acute ischemic stroke (diagnosed 2023-07-15)
- Atrial fibrillation with rapid ventricular response (diagnosed 2024-07-15)
- Central retinal artery occlusion (diagnosed 2025-07-15)
- Hypertension
- Chronic atrial fibrillation

PAST SURGICAL HISTORY:
- Right inguinal hernia repair
- Thoracic epidural steroid injection

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily, titrated to INR goal 2.0-3.0
- Lisinopril 10 mg daily
- Metoprolol succinate 50 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 72
- Mother: Ischemic stroke at age 75
- Brother: Deep vein thrombosis

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-year history. Alcohol: Social, 1-2 drinks per week. Occupation: Retired postal worker.

REVIEW OF SYSTEMS:
He denies weakness, numbness, or visual changes.

PHYSICAL EXAMINATION:
Neurologic examination shows hyperkinetic, irregular, large-amplitude movements affecting the proximal musculature of the left upper and lower extremities. His strength is preserved. There is no sensory deficit, gait abnormality, or tremor.

## Encounter 7: source 2027-07-15, practice 2026-06-24, icu, Intensive Care Unit

Chief complaint: Sudden onset of right-sided weakness, difficulty speaking, and collapse while walking

ENCOUNTER NOTE
Date: 2027-07-15
Provider: Dr. Robert Singh, MD
Department: Intensive Care Unit
Visit Type: icu

CHIEF COMPLAINT:
Sudden onset of right-sided weakness, difficulty speaking, and collapse while walking

HISTORY OF PRESENT ILLNESS:
Patient with polycythemia vera, atrial fibrillation, and prior stroke. His wife reports that he was in his usual state of health until he collapsed while walking to the bathroom.

PAST MEDICAL HISTORY:
Active Problem List:
- Polycythemia vera (diagnosed 2020-01-15)
- Postoperative hemorrhage following inguinal hernia repair (diagnosed 2021-07-15)
- Spinal epidural hematoma (diagnosed 2022-07-15)
- Acute ischemic stroke (diagnosed 2023-07-15)
- Atrial fibrillation with rapid ventricular response (diagnosed 2024-07-15)
- Central retinal artery occlusion (diagnosed 2025-07-15)
- Chorea (diagnosed 2026-07-15)
- Hypertension
- Chronic atrial fibrillation

PAST SURGICAL HISTORY:
- Right inguinal hernia repair
- Thoracic epidural steroid injection

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily, titrated to INR goal 2.0-3.0
- Lisinopril 10 mg daily
- Metoprolol succinate 50 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 72
- Mother: Ischemic stroke at age 75
- Brother: Deep vein thrombosis

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-year history. Alcohol: Social, 1-2 drinks per week. Occupation: Retired postal worker.

VITAL SIGNS:
His blood pressure is 220/120 mmHg, heart rate is 88 beats per minute, and respiratory rate is 18 breaths per minute.

PHYSICAL EXAMINATION:
On examination, the patient is drowsy but arousable. Neurological examination reveals right-sided hemiparesis, expressive aphasia, and a leftward gaze preference.

IMAGING:
A non-contrast head CT scan shows a 3 cm hyperdense area in the left basal ganglia with surrounding edema but no midline shift.
