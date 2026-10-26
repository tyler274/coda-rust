# synthetic_hospital patient 2265

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 69 at first encounter, sex M, Non-Hispanic White, Medicare with supplemental private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-10-14 (Thu) | outpatient | Rheumatology |
| 1 | 2020-03-15 | 2021-12-07 (Tue) | outpatient | Gastroenterology |
| 2 | 2020-05-15 | 2022-02-16 (Wed) | outpatient | Cardiology |
| 3 | 2021-03-15 | 2022-12-12 (Mon) | follow_up | Cardiology |
| 4 | 2022-03-15 | 2023-12-10 (Sun) | ed | Emergency Medicine |
| 5 | 2022-09-15 | 2024-06-19 (Wed) | ed | Emergency Medicine |
| 6 | 2023-09-15 | 2025-06-21 (Sat) | ed | Emergency Medicine |
| 7 | 2024-09-15 | 2026-06-16 (Tue) | ed | Neurology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare with supplemental private insurance",
 "occupation": "Retired commercial airline pilot",
 "alcohol_use": "Social, 1-2 drinks per week",
 "family_history": [
  "Father: atrial fibrillation, ischemic stroke",
  "Mother: Paget disease of bone",
  "Paternal grandfather: hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "50 mg daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "400 mg as needed for knee pain",
   "name": "Ibuprofen"
  },
  {
   "dose": "20 mg daily with food",
   "name": "Rivaroxaban"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Hypertension",
  "Atrial fibrillation (valvular, secondary to rheumatic heart disease)",
  "History of rheumatic fever",
  "Paget disease of bone",
  "Achalasia",
  "Osteoarthritis",
  "History of intracranial hemorrhage"
 ],
 "age_at_first_encounter": 69
}
```

Primary diagnoses: ["Left middle cerebral artery stroke", "Atrial fibrillation", "Acute mesenteric ischemia", "Achalasia", "Paget disease of bone", "Iliofemoral Deep Vein Thrombosis", "Acute arterial embolism of left lower extremity causing acute limb ischemia"]
Comorbidities: ["Atrial fibrillation", "Essential hypertension", "History of rheumatic fever"]

## Encounter 0: source 2020-01-15, practice 2021-10-14, outpatient, Rheumatology

Chief complaint: Left knee pain and difficulty walking with gradual bowing of the left leg

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Left knee pain and difficulty walking with gradual bowing of the left leg

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 6-month history of increasing left knee pain and difficulty walking. He reports a gradual bowing of his left leg. He also mentions occasional headaches and a recent difficulty in hearing, especially in noisy environments.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Atrial fibrillation (valvular, secondary to rheumatic heart disease)
- History of rheumatic fever
- Paget disease of bone
- Achalasia
- Osteoarthritis
- History of intracranial hemorrhage

His medical history is significant for hypertension, controlled with hydrochlorothiazide.

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg daily
- Metoprolol succinate 50 mg daily
- Ibuprofen 400 mg as needed for knee pain
- Rivaroxaban 20 mg daily with food

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: atrial fibrillation, ischemic stroke
- Mother: Paget disease of bone
- Paternal grandfather: hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per week. Occupation: Retired commercial airline pilot.

PHYSICAL EXAMINATION:
On physical examination, there is noticeable frontal bossing and warmth over the left tibia, which is tender to palpation. There is also a palpable enlargement of the tibia compared to the right. Knee examination reveals crepitus with movement and tenderness on the medial side. Neurological examination is unremarkable.

LABORATORY DATA:
Laboratory results are as follows:

- Serum calcium: 9.8 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum phosphorus: 3.5 mg/dL (normal: 2.5-4.5 mg/dL)
- Alkaline phosphatase: 280 U/L (normal: 40-130 U/L)
- Serum parathyroid hormone (PTH): 55 pg/mL (normal: 15-65 pg/mL)

## Encounter 1: source 2020-03-15, practice 2021-12-07, outpatient, Gastroenterology

Chief complaint: Progressive difficulty swallowing solids and liquids with 15-pound weight loss

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Michael Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty swallowing solids and liquids with 15-pound weight loss

HISTORY OF PRESENT ILLNESS:
He returns following rheumatology care for Paget disease. He reports a 15-pound weight loss during this period. He denies any history of heartburn or regurgitation.

PAST MEDICAL HISTORY:
Active Problem List:
- Paget disease of bone (diagnosed 2020-01-15)
- Hypertension
- Atrial fibrillation (valvular, secondary to rheumatic heart disease)
- History of rheumatic fever
- Achalasia
- Osteoarthritis
- History of intracranial hemorrhage

His past medical history is significant for hypertension, controlled with hydrochlorothiazide.

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg daily
- Metoprolol succinate 50 mg daily
- Ibuprofen 400 mg as needed for knee pain
- Rivaroxaban 20 mg daily with food

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: atrial fibrillation, ischemic stroke
- Mother: Paget disease of bone
- Paternal grandfather: hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per week. Occupation: Retired commercial airline pilot.

PHYSICAL EXAMINATION:
On physical examination, he appears cachectic. His abdomen is soft and non-tender, without any palpable masses.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 13.2 g/dL
- White blood cell count: 7,000/uL
- Platelets: 250,000/uL
- Serum electrolytes: Within normal limits
- Liver function tests: Within normal limits

IMAGING:
A barium swallow study shows a dilated esophagus with a "bird's beak" appearance at the distal end.

## Encounter 2: source 2020-05-15, practice 2022-02-16, outpatient, Cardiology

Chief complaint: Palpitations and intermittent shortness of breath for two weeks

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. James O'Brien, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Palpitations and intermittent shortness of breath for two weeks

HISTORY OF PRESENT ILLNESS:
The patient returns after rheumatology and gastroenterology visits with palpitations and intermittent shortness of breath for the past two weeks and denies chest pain, dizziness, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Paget disease of bone (diagnosed 2020-01-15)
- Achalasia (diagnosed 2020-03-15)
- Hypertension
- Atrial fibrillation (valvular, secondary to rheumatic heart disease)
- History of rheumatic fever
- Osteoarthritis
- History of intracranial hemorrhage

MEDICATIONS:
His current medications include hydrochlorothiazide and amlodipine.

Home Medications:
- Lisinopril 10 mg daily
- Metoprolol succinate 50 mg daily
- Ibuprofen 400 mg as needed for knee pain
- Rivaroxaban 20 mg daily with food

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: atrial fibrillation, ischemic stroke
- Mother: Paget disease of bone
- Paternal grandfather: hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per week. Occupation: Retired commercial airline pilot.

VITAL SIGNS:
His blood pressure is 158/92 mmHg, heart rate is irregularly irregular at 110 bpm, and respiratory rate is 18 breaths per minute.

PHYSICAL EXAMINATION:
On auscultation, there are no crackles or wheezes, and the remainder of the physical examination is unremarkable.

LABORATORY DATA:
Laboratory values are as follows:

- Serum sodium: 140 mEq/L
- Serum potassium: 4.2 mEq/L
- Serum creatinine: 1.1 mg/dL
- Thyroid-stimulating hormone (TSH): 2.5 µIU/mL
- Hemoglobin: 14 g/dL

OTHER STUDIES:
An electrocardiogram (ECG) shows no evidence of ischemia but confirms atrial fibrillation with a ventricular rate of approximately 110 bpm.

## Encounter 3: source 2021-03-15, practice 2022-12-12, follow_up, Cardiology

Chief complaint: Follow-up after left leg DVT diagnosed three weeks ago following transatlantic flight

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. James O'Brien, MD
Department: Cardiology
Visit Type: follow_up

CHIEF COMPLAINT:
Follow-up after left leg DVT diagnosed three weeks ago following transatlantic flight

HISTORY OF PRESENT ILLNESS:
The patient is a 70-year-old male with valvular atrial fibrillation secondary to rheumatic heart disease, Paget disease of bone, and achalasia who returns for follow-up of his left leg deep vein thrombosis. His symptoms began after a transatlantic flight, and he was started on oral rivaroxaban at the time of diagnosis. He reports that swelling and discomfort have improved significantly. He states he appreciates not having to undergo frequent blood tests or make dietary changes. He also inquires whether a recent course of antibiotics for sinusitis could interfere with his treatment.

PAST MEDICAL HISTORY:
Active Problem List:
- Paget disease of bone (diagnosed 2020-01-15)
- Achalasia (diagnosed 2020-03-15)
- Atrial fibrillation (diagnosed 2020-05-15)
- Hypertension
- Atrial fibrillation (valvular, secondary to rheumatic heart disease)
- History of rheumatic fever
- Osteoarthritis
- History of intracranial hemorrhage

His medical history includes hypertension and osteoarthritis.

MEDICATIONS:
He takes he takes lisinopril and ibuprofen.

Home Medications:
- Metoprolol succinate 50 mg daily
- Rivaroxaban 20 mg daily with food

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His brother who takes a different medication for atrial fibrillation and was told to avoid leafy greens and watch his antibiotic use

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per week. Occupation: Retired commercial airline pilot.

REVIEW OF SYSTEMS:
He denies new leg pain, shortness of breath, or bleeding

VITAL SIGNS:
Vital signs are normal

PHYSICAL EXAMINATION:
Physical exam shows mild residual edema in the left leg

## Encounter 4: source 2022-03-15, practice 2023-12-10, ed, Emergency Medicine

Chief complaint: Sudden-onset right arm weakness and slurred speech beginning two hours ago

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Robert Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset right arm weakness and slurred speech beginning two hours ago

HISTORY OF PRESENT ILLNESS:
This 71-year-old male with valvular atrial fibrillation and prior DVT is brought to the emergency department due to sudden-onset right arm weakness and slurred speech that began 2 hours ago. He has not seen a physician in many years.

PAST MEDICAL HISTORY:
Active Problem List:
- Paget disease of bone (diagnosed 2020-01-15)
- Achalasia (diagnosed 2020-03-15)
- Atrial fibrillation (diagnosed 2020-05-15)
- Iliofemoral Deep Vein Thrombosis (diagnosed 2021-03-15)
- Hypertension
- Atrial fibrillation (valvular, secondary to rheumatic heart disease)
- History of rheumatic fever
- Osteoarthritis
- History of intracranial hemorrhage

Recalls having rheumatic fever as a child.

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Metoprolol succinate 50 mg daily
- Ibuprofen 400 mg as needed for knee pain
- Rivaroxaban 20 mg daily with food

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: atrial fibrillation, ischemic stroke
- Mother: Paget disease of bone
- Paternal grandfather: hypertension

SOCIAL HISTORY:
No history of smoking

VITAL SIGNS:
His BP is 115/75 and HR is 90.

PHYSICAL EXAMINATION:
Neurologic exam shows right-sided facial droop and 3/5 motor strength in the right upper extremity. A low-pitched diastolic murmur is auscultated best at the apex.

IMAGING:
Carotid ultrasound shows no significant stenosis. A non-contrast head CT shows no hemorrhage.

## Encounter 5: source 2022-09-15, practice 2024-06-19, ed, Emergency Medicine

Chief complaint: Sudden-onset, severe diffuse abdominal pain with nausea for three hours

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. Emily Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset, severe diffuse abdominal pain with nausea for three hours

HISTORY OF PRESENT ILLNESS:
This 71-year-old with valvular atrial fibrillation presents with diffuse abdominal pain, 10/10 in intensity, and associated with nausea. He denies vomiting, diarrhea, or fever.

PAST MEDICAL HISTORY:
Active Problem List:
- Paget disease of bone (diagnosed 2020-01-15)
- Achalasia (diagnosed 2020-03-15)
- Atrial fibrillation (diagnosed 2020-05-15)
- Iliofemoral Deep Vein Thrombosis (diagnosed 2021-03-15)
- Hypertension
- Atrial fibrillation (valvular, secondary to rheumatic heart disease)
- History of rheumatic fever
- Osteoarthritis
- History of intracranial hemorrhage

MEDICATIONS:
He takes not on anticoagulation.

Home Medications:
- Lisinopril 10 mg daily
- Metoprolol succinate 50 mg daily
- Ibuprofen 400 mg as needed for knee pain
- Rivaroxaban 20 mg daily with food

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: atrial fibrillation, ischemic stroke
- Mother: Paget disease of bone
- Paternal grandfather: hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per week. Occupation: Retired commercial airline pilot.

VITAL SIGNS:
Blood pressure 145/90 mmHg, heart rate 110 bpm irregularly irregular, respiratory rate 22 breaths/min, and temperature 98.6°F (37°C)

PHYSICAL EXAMINATION:
Soft abdomen with severe diffuse tenderness to palpation, but no rebound or guarding. Bowel sounds are diminished.

LABORATORY DATA:
Hemoglobin: 13.2 g/dL
White blood cell count: 12,000/mm³
Platelets: 250,000/mm³
Sodium: 136 mEq/L
Potassium: 4.5 mEq/L
Chloride: 95 mEq/L
Bicarbonate: 15 mEq/L
Blood urea nitrogen: 22 mg/dL
Creatinine: 1.1 mg/dL
Glucose: 110 mg/dL
Lactate: 5.2 mmol/L
Arterial blood gas: pH 7.28, pCO2 28 mmHg, pO2 98 mmHg

OTHER STUDIES:
An electrocardiogram confirms atrial fibrillation with a rapid ventricular response.

## Encounter 6: source 2023-09-15, practice 2025-06-21, ed, Emergency Medicine

Chief complaint: Sudden onset of severe pain with pallor and coldness in left lower extremity

ENCOUNTER NOTE
Date: 2023-09-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe pain with pallor and coldness in left lower extremity

HISTORY OF PRESENT ILLNESS:
This patient with valvular atrial fibrillation and prior mesenteric ischemia presents reporting the leg became pale and cold approximately 2 hours ago. He denies any trauma or prior similar episodes.

PAST MEDICAL HISTORY:
Active Problem List:
- Paget disease of bone (diagnosed 2020-01-15)
- Achalasia (diagnosed 2020-03-15)
- Atrial fibrillation (diagnosed 2020-05-15)
- Iliofemoral Deep Vein Thrombosis (diagnosed 2021-03-15)
- Acute mesenteric ischemia (diagnosed 2022-09-15)
- Hypertension
- Atrial fibrillation (valvular, secondary to rheumatic heart disease)
- History of rheumatic fever
- Osteoarthritis
- History of intracranial hemorrhage

MEDICATIONS:
His medications include warfarin and metoprolol.

Home Medications:
- Lisinopril 10 mg daily
- Metoprolol succinate 50 mg daily
- Ibuprofen 400 mg as needed for knee pain
- Rivaroxaban 20 mg daily with food

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: atrial fibrillation, ischemic stroke
- Mother: Paget disease of bone
- Paternal grandfather: hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per week. Occupation: Retired commercial airline pilot.

VITAL SIGNS:
His vital signs are stable

PHYSICAL EXAMINATION:
On examination, the left lower extremity is cold to touch, with absent pulses distal to the femoral artery. There is no sensory loss or motor weakness, and the right lower extremity is normal. The rest of the physical examination is unremarkable.

LABORATORY DATA:
His laboratory values are as follows:

- INR: 1.2
- Hemoglobin: 14 g/dL
- White blood cell count: 7.5 x 10^9/L
- Platelet count: 250 x 10^9/L

IMAGING:
An urgent Doppler ultrasound of the left lower extremity shows no flow in the popliteal artery.

## Encounter 7: source 2024-09-15, practice 2026-06-16, ed, Neurology

Chief complaint: Sudden-onset weakness of right arm and face with inability to grip objects

ENCOUNTER NOTE
Date: 2024-09-15
Provider: Dr. Maria Gonzalez, MD
Department: Neurology
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset weakness of right arm and face with inability to grip objects

HISTORY OF PRESENT ILLNESS:
The patient with valvular atrial fibrillation on rivaroxaban and prior thromboembolic events including right arm weakness in 2022 and mesenteric ischemia is unable to grip objects with his right hand, cannot raise his right eyebrow, and has drooping of the right corner of his mouth. His right leg strength is preserved, and he has no sensory loss or visual changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Paget disease of bone (diagnosed 2020-01-15)
- Achalasia (diagnosed 2020-03-15)
- Atrial fibrillation (diagnosed 2020-05-15)
- Iliofemoral Deep Vein Thrombosis (diagnosed 2021-03-15)
- Acute mesenteric ischemia (diagnosed 2022-09-15)
- Acute arterial embolism of left lower extremity causing acute limb ischemia (diagnosed 2023-09-15)
- Hypertension
- Atrial fibrillation (valvular, secondary to rheumatic heart disease)
- History of rheumatic fever
- Osteoarthritis
- History of intracranial hemorrhage

Medical history includes hypertension and atrial fibrillation.

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Metoprolol succinate 50 mg daily
- Ibuprofen 400 mg as needed for knee pain
- Rivaroxaban 20 mg daily with food

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: atrial fibrillation, ischemic stroke
- Mother: Paget disease of bone
- Paternal grandfather: hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per week. Occupation: Retired commercial airline pilot.

PHYSICAL EXAMINATION:
He is alert and oriented, with intact language and cognitive function.

IMAGING:
A non-contrast head CT confirms the diagnosis.
