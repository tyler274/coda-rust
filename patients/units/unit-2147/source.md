# synthetic_hospital patient 2147

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 41 at first encounter, sex M, Non-Hispanic White, Private employer-sponsored PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 2 | 2020-10-15 | 2026-06-20 (Sat) | ed | Emergency Medicine |
| 3 | 2020-10-15 | 2026-06-20 (Sat) | inpatient | Orthopedic Surgery |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private employer-sponsored PPO",
 "occupation": "Construction site supervisor",
 "alcohol_use": "Social (2-3 drinks per week)",
 "family_history": [
  "Father: Myocardial infarction at age 58",
  "Mother: Type 2 Diabetes Mellitus",
  "Brother: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 8 pack-years, quit 2 years ago",
 "home_medications": [
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg PO nightly",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Tonsillectomy (age 7)",
  "Open reduction internal fixation left ankle (age 32)"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus",
  "Hypertension",
  "Obesity (BMI 34)",
  "Obstructive sleep apnea"
 ],
 "age_at_first_encounter": 41
}
```

Primary diagnoses: ["Flaccid paralysis", "Venous air embolism"]
Comorbidities: ["Multiple traumatic injuries"]

## Encounter 2: source 2020-10-15, practice 2026-06-20, ed, Emergency Medicine

Chief complaint: Brought to emergency department following high-speed motor vehicle collision. Acutely dyspneic with chest tightness and dizziness after urgent central venous catheter placement in upright position.

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Maria Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought to emergency department following high-speed motor vehicle collision. Acutely dyspneic with chest tightness and dizziness after urgent central venous catheter placement in upright position.

HISTORY OF PRESENT ILLNESS:
This 41-year-old male with a history of Type 2 Diabetes Mellitus, Hypertension, Obesity (BMI 34), and Obstructive Sleep Apnea presents to the Emergency Department following a high-speed motor vehicle collision. Due to difficult peripheral access, a central venous catheter is urgently placed in the right internal jugular vein while the patient is in the upright position. Shortly after the procedure, the patient becomes acutely dyspneic and complains of a vague sense of chest tightness and dizziness. His blood pressure drops further, and he begins to gasp for air.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus
- Hypertension
- Obesity (BMI 34)
- Obstructive sleep apnea

PAST SURGICAL HISTORY:
- Tonsillectomy (age 7)
- Open reduction internal fixation left ankle (age 32)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO nightly
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 58
- Mother: Type 2 Diabetes Mellitus
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 8 pack-years, quit 2 years ago. Alcohol: Social (2-3 drinks per week). Occupation: Construction site supervisor.

VITAL SIGNS:
He is hypotensive and tachycardic on arrival.

PHYSICAL EXAMINATION:
Physical exam shows distended neck veins, mild confusion, and a new systolic murmur heard best along the left sternal border. Lungs are clear to auscultation.

IMAGING:
Chest x-ray is unremarkable except for proper catheter placement. A repeat bedside ultrasound of the heart is inconclusive due to poor acoustic windows.

OTHER STUDIES:
ECG reveals sinus tachycardia with non-specific ST-T changes.

## Encounter 3: source 2020-10-15, practice 2026-06-20, inpatient, Orthopedic Surgery

Chief complaint: Hospital day 10 status post open reduction internal fixation of multiple orthopedic injuries. New onset flaccid paralysis and significant decrease in lean body mass despite adequate nutritional support.

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. David Park, MD
Department: Orthopedic Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Hospital day 10 status post open reduction internal fixation of multiple orthopedic injuries. New onset flaccid paralysis and significant decrease in lean body mass despite adequate nutritional support.

HISTORY OF PRESENT ILLNESS:
The patient, with Diabetes, Hypertension, and Obesity, was admitted following a motor vehicle collision with dyspnea after central line placement. He sustains multiple orthopedic injuries requiring surgical fixation and is expected to be immobilized for several weeks. On hospital day 10, laboratory evaluation reveals a significant decrease in lean body mass, particularly in the lower limbs, despite adequate nutritional support.

PAST MEDICAL HISTORY:
Active Problem List:
- Venous air embolism (diagnosed 2020-10-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Obesity (BMI 34)
- Obstructive sleep apnea

PAST SURGICAL HISTORY:
- Tonsillectomy (age 7)
- Open reduction internal fixation left ankle (age 32)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO nightly
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 58
- Mother: Type 2 Diabetes Mellitus
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 8 pack-years, quit 2 years ago. Alcohol: Social (2-3 drinks per week). Occupation: Construction site supervisor.

PHYSICAL EXAMINATION:
Physical examination shows muscle wasting and reduced strength in the lower extremities. The patient is alert and oriented, and there are no signs of systemic infection.
