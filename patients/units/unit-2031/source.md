# synthetic_hospital patient 2031

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 55 at first encounter, sex M, Hispanic/Latino, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-06-08 (Mon) | outpatient | Orthopedics |
| 1 | 2020-04-15 | 2020-08-31 (Mon) | outpatient | Endocrinology |
| 2 | 2021-01-15 | 2021-06-11 (Fri) | ed | Emergency Medicine |
| 3 | 2022-01-15 | 2022-05-28 (Sat) | ed | Emergency Medicine |
| 4 | 2023-01-15 | 2023-06-01 (Thu) | ed | Emergency Medicine |
| 5 | 2024-01-15 | 2024-06-03 (Mon) | icu | Emergency Medicine |
| 6 | 2025-01-15 | 2025-05-27 (Tue) | ed | Emergency Medicine |
| 7 | 2026-01-15 | 2026-06-06 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Landscaping contractor",
 "alcohol_use": "Occasional, 1-2 beers on weekends",
 "family_history": [
  "Father: Type 2 diabetes mellitus, ruptured cerebral aneurysm at age 60",
  "Mother: Hypertension, peripheral vascular disease",
  "Brother: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Hispanic/Latino",
 "smoking_status": "Former smoker, 20 pack-year history, quit 5 years ago",
 "home_medications": [
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Glipizide"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Surgical debridement of left lower extremity necrotizing infection",
  "Percutaneous drainage of hepatic abscess",
  "Transarterial coil embolization of posterior communicating artery aneurysm"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus (15-year duration, poorly controlled, HbA1c consistently >9%)",
  "Hypertension",
  "Peripheral vascular disease",
  "Diabetic peripheral neuropathy",
  "Diabetic autonomic neuropathy",
  "Charcot neuroarthropathy of the right foot",
  "History of diverticulitis"
 ],
 "age_at_first_encounter": 55
}
```

Primary diagnoses: ["Posterior communicating artery aneurysm", "Rhinocerebral mucormycosis", "Pyogenic liver abscess", "Neurogenic arthropathy (Charcot joint)", "Cavernous sinus thrombosis", "Diabetic foot ulcer in proliferative phase of healing", "Necrotizing fasciitis", "Soft tissue infection"]
Comorbidities: ["Poorly controlled diabetes mellitus", "Diabetic sensory polyneuropathy"]

## Encounter 0: source 2020-01-15, practice 2020-06-08, outpatient, Orthopedics

Chief complaint: Six-month history of difficulty walking and progressive deformity of the right foot without significant pain

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of difficulty walking and progressive deformity of the right foot without significant pain

HISTORY OF PRESENT ILLNESS:
He reports that his foot seems to have changed shape, but he denies any significant pain. His diabetes has been poorly controlled, with recent hemoglobin A1c levels consistently above 9%.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (15-year duration, poorly controlled, HbA1c consistently >9%)
- Hypertension
- Peripheral vascular disease
- Diabetic peripheral neuropathy
- Diabetic autonomic neuropathy
- Charcot neuroarthropathy of the right foot
- History of diverticulitis

PAST SURGICAL HISTORY:
- Surgical debridement of left lower extremity necrotizing infection
- Percutaneous drainage of hepatic abscess
- Transarterial coil embolization of posterior communicating artery aneurysm

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Glipizide 10 mg PO daily
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, ruptured cerebral aneurysm at age 60
- Mother: Hypertension, peripheral vascular disease
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-year history, quit 5 years ago. Alcohol: Occasional, 1-2 beers on weekends. Occupation: Landscaping contractor.

PHYSICAL EXAMINATION:
On physical examination, the right foot appears swollen with a high arch and an inverted heel. There is a loss of protective sensation in both feet. Ankle reflexes are absent bilaterally. Pulses are palpable and symmetrical.

IMAGING:
An X-ray of the right foot shows bony destruction, osteophyte formation, and loss of joint space in the midfoot.

## Encounter 1: source 2020-04-15, practice 2020-08-31, outpatient, Endocrinology

Chief complaint: Two-month history of a non-healing wound on the left foot initially presenting as a small blister

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. David Kim, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Two-month history of a non-healing wound on the left foot initially presenting as a small blister

HISTORY OF PRESENT ILLNESS:
The patient with right Charcot foot presents with a two-month left foot wound. He reports that the wound initially appeared as a small blister but has progressively enlarged and deepened. He denies any pain associated with the wound.

PAST MEDICAL HISTORY:
Active Problem List:
- Neurogenic arthropathy (Charcot joint) (diagnosed 2020-01-15)
- Type 2 diabetes mellitus (15-year duration, poorly controlled, HbA1c consistently >9%)
- Hypertension
- Peripheral vascular disease
- Diabetic peripheral neuropathy
- Diabetic autonomic neuropathy
- Charcot neuroarthropathy of the right foot
- History of diverticulitis

PAST SURGICAL HISTORY:
- Surgical debridement of left lower extremity necrotizing infection
- Percutaneous drainage of hepatic abscess
- Transarterial coil embolization of posterior communicating artery aneurysm

MEDICATIONS:
His diabetes has been managed with oral hypoglycemics

Home Medications:
- Metformin 1000 mg PO BID
- Glipizide 10 mg PO daily
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, ruptured cerebral aneurysm at age 60
- Mother: Hypertension, peripheral vascular disease
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
He admits to sporadic medication adherence and dietary indiscretions

PHYSICAL EXAMINATION:
On physical examination, the patient has a 3 cm × 3 cm ulcer with a calloused edge on the plantar surface of his left great toe. There is no erythema, warmth, or purulent discharge. The ulcer base is clean with granulation tissue. Sensation to light touch and pinprick is diminished in both feet. Pedal pulses are palpable bilaterally.

LABORATORY DATA:
His most recent laboratory values are as follows:

- Hemoglobin A1c: 9.2%
- Fasting blood glucose: 198 mg/dL
- Total cholesterol: 210 mg/dL
- LDL cholesterol: 130 mg/dL
- HDL cholesterol: 35 mg/dL
- Triglycerides: 250 mg/dL

## Encounter 2: source 2021-01-15, practice 2021-06-11, ed, Emergency Medicine

Chief complaint: Three-day history of worsening redness, pain, and swelling of the right lower leg after injuring shin on metal pipe

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Three-day history of worsening redness, pain, and swelling of the right lower leg after injuring shin on metal pipe

HISTORY OF PRESENT ILLNESS:
This 56-year-old male with poorly controlled Type 2 diabetes mellitus, Charcot neuroarthropathy of the right foot, and a history of diabetic foot ulcer presents to the emergency department with a 3-day history of. He reports injuring his shin on a metal pipe while working in his yard several days ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Neurogenic arthropathy (Charcot joint) (diagnosed 2020-01-15)
- Diabetic foot ulcer in proliferative phase of healing (diagnosed 2020-04-15)
- Type 2 diabetes mellitus (15-year duration, poorly controlled, HbA1c consistently >9%)
- Hypertension
- Peripheral vascular disease
- Diabetic peripheral neuropathy
- Diabetic autonomic neuropathy
- Charcot neuroarthropathy of the right foot
- History of diverticulitis

He has a medical history of poorly controlled type 2 diabetes mellitus and peripheral vascular disease.

PAST SURGICAL HISTORY:
- Surgical debridement of left lower extremity necrotizing infection
- Percutaneous drainage of hepatic abscess
- Transarterial coil embolization of posterior communicating artery aneurysm

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Glipizide 10 mg PO daily
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, ruptured cerebral aneurysm at age 60
- Mother: Hypertension, peripheral vascular disease
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-year history, quit 5 years ago. Alcohol: Occasional, 1-2 beers on weekends. Occupation: Landscaping contractor.

VITAL SIGNS:
Vital signs reveal a temperature of 38.9°C (102°F), heart rate of 106/min, blood pressure of 124/70 mm Hg, and respiratory rate of 18/min.

PHYSICAL EXAMINATION:
Physical exam shows a diffusely erythematous, warm, and tender region over the anterior tibia with ill-defined borders. No fluctuant mass, crepitus, or necrosis is present. There is no purulent drainage or signs of abscess formation. Peripheral pulses are intact.

IMAGING:
The affected limb is shown below.

PLAN:
The patient is started empirically on antibiotics.

## Encounter 3: source 2022-01-15, practice 2022-05-28, ed, Emergency Medicine

Chief complaint: Five-day history of fever, chills, and constant sharp right upper quadrant pain radiating to the right shoulder

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Five-day history of fever, chills, and constant sharp right upper quadrant pain radiating to the right shoulder

HISTORY OF PRESENT ILLNESS:
57-year-old male with diabetes, Charcot foot, and prior soft tissue infection presents with pain. He reports that the pain is constant, sharp, and radiates to his right shoulder. He also notes a decreased appetite and has lost 5 pounds over the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Neurogenic arthropathy (Charcot joint) (diagnosed 2020-01-15)
- Diabetic foot ulcer in proliferative phase of healing (diagnosed 2020-04-15)
- Soft tissue infection (diagnosed 2021-01-15)
- Type 2 diabetes mellitus (15-year duration, poorly controlled, HbA1c consistently >9%)
- Hypertension
- Peripheral vascular disease
- Diabetic peripheral neuropathy
- Diabetic autonomic neuropathy
- Charcot neuroarthropathy of the right foot
- History of diverticulitis

PAST SURGICAL HISTORY:
- Surgical debridement of left lower extremity necrotizing infection
- Percutaneous drainage of hepatic abscess
- Transarterial coil embolization of posterior communicating artery aneurysm

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Glipizide 10 mg PO daily
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, ruptured cerebral aneurysm at age 60
- Mother: Hypertension, peripheral vascular disease
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-year history, quit 5 years ago. Alcohol: Occasional, 1-2 beers on weekends. Occupation: Landscaping contractor.

VITAL SIGNS:
His temperature is 38.9°C (102°F), heart rate is 110 beats per minute, and blood pressure is 135/85 mmHg

PHYSICAL EXAMINATION:
Abdominal examination reveals tenderness to palpation in the right upper quadrant with a positive Murphy&#x27;s sign

LABORATORY DATA:
- White blood cell count: 18,000/mm³ (Normal: 4,500-11,000/mm³)
- Hemoglobin: 13.2 g/dL (Normal: 13.8-17.2 g/dL)
- Platelets: 250,000/mm³ (Normal: 150,000-450,000/mm³)
- Aspartate aminotransferase (AST): 85 U/L (Normal: 8-48 U/L)
- Alanine aminotransferase (ALT): 90 U/L (Normal: 7-55 U/L)
- Alkaline phosphatase: 220 U/L (Normal: 40-129 U/L)
- Total bilirubin: 2.1 mg/dL (Normal: 0.3-1.2 mg/dL)

IMAGING:
An abdominal ultrasound reveals a 4 cm hypoechoic lesion in the right lobe of the liver.

## Encounter 4: source 2023-01-15, practice 2023-06-01, ed, Emergency Medicine

Chief complaint: Sudden onset of severe headache with diplopia and ptosis of the left eye beginning two hours ago

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Michael Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe headache with diplopia and ptosis of the left eye beginning two hours ago

HISTORY OF PRESENT ILLNESS:
This 58-year-old with uncontrolled Type 2 diabetes, Charcot neuroarthropathy, soft tissue infection, and liver abscess presents with sudden onset of severe headache with diplopia and ptosis of left eye that started 2 hours ago. He denies any trauma or previous similar episodes.

PAST MEDICAL HISTORY:
Active Problem List:
- Neurogenic arthropathy (Charcot joint) (diagnosed 2020-01-15)
- Diabetic foot ulcer in proliferative phase of healing (diagnosed 2020-04-15)
- Soft tissue infection (diagnosed 2021-01-15)
- Pyogenic liver abscess (diagnosed 2022-01-15)
- Type 2 diabetes mellitus (15-year duration, poorly controlled, HbA1c consistently >9%)
- Hypertension
- Peripheral vascular disease
- Diabetic peripheral neuropathy
- Diabetic autonomic neuropathy
- Charcot neuroarthropathy of the right foot
- History of diverticulitis

PAST SURGICAL HISTORY:
- Surgical debridement of left lower extremity necrotizing infection
- Percutaneous drainage of hepatic abscess
- Transarterial coil embolization of posterior communicating artery aneurysm

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Glipizide 10 mg PO daily
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, ruptured cerebral aneurysm at age 60
- Mother: Hypertension, peripheral vascular disease
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-year history, quit 5 years ago. Alcohol: Occasional, 1-2 beers on weekends. Occupation: Landscaping contractor.

VITAL SIGNS:
His blood pressure is 160/95 mmHg, and his blood glucose level is 250 mg/dL.

PHYSICAL EXAMINATION:
On examination, the left pupil is 6 mm and non-reactive to light, while the right pupil is 3 mm and reactive. There is a limitation of upward, downward, and medial gaze in the left eye. The rest of the neurological examination is unremarkable.

IMAGING:
A non-contrast head CT scan shows no evidence of hemorrhage or mass effect.

## Encounter 5: source 2024-01-15, practice 2024-06-03, icu, Emergency Medicine

Chief complaint: Two-day history of severe pain, redness, and swelling in the left lower leg following a minor cut one week ago

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Aisha Washington, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Two-day history of severe pain, redness, and swelling in the left lower leg following a minor cut one week ago

HISTORY OF PRESENT ILLNESS:
This 59-year-old male with poorly controlled diabetes and recurrent soft tissue infections presents with a 2-day history of severe pain, redness, and swelling in his left lower leg. He denies any recent trauma but mentions a small, seemingly insignificant cut on his leg a week ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Neurogenic arthropathy (Charcot joint) (diagnosed 2020-01-15)
- Diabetic foot ulcer in proliferative phase of healing (diagnosed 2020-04-15)
- Soft tissue infection (diagnosed 2021-01-15)
- Pyogenic liver abscess (diagnosed 2022-01-15)
- Posterior communicating artery aneurysm (diagnosed 2023-01-15)
- Type 2 diabetes mellitus (15-year duration, poorly controlled, HbA1c consistently >9%)
- Hypertension
- Peripheral vascular disease
- Diabetic peripheral neuropathy
- Diabetic autonomic neuropathy
- Charcot neuroarthropathy of the right foot
- History of diverticulitis

PAST SURGICAL HISTORY:
- Surgical debridement of left lower extremity necrotizing infection
- Percutaneous drainage of hepatic abscess
- Transarterial coil embolization of posterior communicating artery aneurysm

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Glipizide 10 mg PO daily
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, ruptured cerebral aneurysm at age 60
- Mother: Hypertension, peripheral vascular disease
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-year history, quit 5 years ago. Alcohol: Occasional, 1-2 beers on weekends. Occupation: Landscaping contractor.

VITAL SIGNS:
Fever of 38.9°C (102°F) and a blood pressure of 90/60 mmHg

PHYSICAL EXAMINATION:
Physical examination reveals a tense, edematous left lower leg with a sharply demarcated area of erythema that has rapidly expanded since onset. There is a foul-smelling purulent discharge from the site of the cut. The overlying skin is dusky and there is a palpable crepitus.

LABORATORY DATA:
White blood cell count: 18,000/uL
- Hemoglobin: 12.5 g/dL
- Sodium: 135 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 1.8 mg/dL (baseline 1.2 mg/dL)
- C-reactive protein: 24 mg/dL
- Blood cultures: Pending

IMAGING:
An X-ray of the leg shows gas in the soft tissues.

## Encounter 6: source 2025-01-15, practice 2025-05-27, ed, Emergency Medicine

Chief complaint: Two-day history of severe constant right-sided facial pain, fever, and facial swelling

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Robert Garcia, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-day history of severe constant right-sided facial pain, fever, and facial swelling

HISTORY OF PRESENT ILLNESS:
The patient is a 60-year-old male with poorly controlled diabetes mellitus and history of necrotizing fasciitis in 2024, returning to the emergency department with a 2-day history of right-sided facial pain, fever, and swelling. He reports that the pain is severe, constant, and has been worsening despite taking acetaminophen.

PAST MEDICAL HISTORY:
Active Problem List:
- Neurogenic arthropathy (Charcot joint) (diagnosed 2020-01-15)
- Diabetic foot ulcer in proliferative phase of healing (diagnosed 2020-04-15)
- Soft tissue infection (diagnosed 2021-01-15)
- Pyogenic liver abscess (diagnosed 2022-01-15)
- Posterior communicating artery aneurysm (diagnosed 2023-01-15)
- Necrotizing fasciitis (diagnosed 2024-01-15)
- Type 2 diabetes mellitus (15-year duration, poorly controlled, HbA1c consistently >9%)
- Hypertension
- Peripheral vascular disease
- Diabetic peripheral neuropathy
- Diabetic autonomic neuropathy
- Charcot neuroarthropathy of the right foot
- History of diverticulitis

PAST SURGICAL HISTORY:
- Surgical debridement of left lower extremity necrotizing infection
- Percutaneous drainage of hepatic abscess
- Transarterial coil embolization of posterior communicating artery aneurysm

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Glipizide 10 mg PO daily
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, ruptured cerebral aneurysm at age 60
- Mother: Hypertension, peripheral vascular disease
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-year history, quit 5 years ago. Alcohol: Occasional, 1-2 beers on weekends. Occupation: Landscaping contractor.

VITAL SIGNS:
His temperature is 38.9°C (102°F), blood pressure is 150/90 mmHg, pulse is 110/min, and respiratory rate is 22/min.

PHYSICAL EXAMINATION:
Physical examination reveals a swollen and erythematous right cheek with black eschar on the nasal mucosa. There is also tenderness to palpation over the maxillary sinus.

LABORATORY DATA:
Laboratory findings are as follows:

- Blood glucose: 450 mg/dL
- Arterial pH: 7.25
- Bicarbonate: 15 mEq/L
- Anion gap: 20 mEq/L
- Beta-hydroxybutyrate: elevated
- Hemoglobin A1c: 12%

IMAGING:
A CT scan of the sinuses shows opacification of the right maxillary sinus with bony erosion.

## Encounter 7: source 2026-01-15, practice 2026-06-06, ed, Emergency Medicine

Chief complaint: Acute onset of double vision and periorbital swelling on the right side two days after squeezing a painful lip pimple

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. Sarah Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of double vision and periorbital swelling on the right side two days after squeezing a painful lip pimple

HISTORY OF PRESENT ILLNESS:
This 61-year-old male with poorly controlled diabetes and history of severe infections including mucormycosis presents with acute symptoms. He reports symptoms began two days ago and worsened. He recently tried to squeeze a painful pimple on his upper lip.

PAST MEDICAL HISTORY:
Active Problem List:
- Neurogenic arthropathy (Charcot joint) (diagnosed 2020-01-15)
- Diabetic foot ulcer in proliferative phase of healing (diagnosed 2020-04-15)
- Soft tissue infection (diagnosed 2021-01-15)
- Pyogenic liver abscess (diagnosed 2022-01-15)
- Posterior communicating artery aneurysm (diagnosed 2023-01-15)
- Necrotizing fasciitis (diagnosed 2024-01-15)
- Rhinocerebral mucormycosis (diagnosed 2025-01-15)
- Type 2 diabetes mellitus (15-year duration, poorly controlled, HbA1c consistently >9%)
- Hypertension
- Peripheral vascular disease
- Diabetic peripheral neuropathy
- Diabetic autonomic neuropathy
- Charcot neuroarthropathy of the right foot
- History of diverticulitis

He has a history of poorly controlled type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Surgical debridement of left lower extremity necrotizing infection
- Percutaneous drainage of hepatic abscess
- Transarterial coil embolization of posterior communicating artery aneurysm

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Glipizide 10 mg PO daily
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, ruptured cerebral aneurysm at age 60
- Mother: Hypertension, peripheral vascular disease
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-year history, quit 5 years ago. Alcohol: Occasional, 1-2 beers on weekends. Occupation: Landscaping contractor.

VITAL SIGNS:
His temperature is 38.9°C (102°F)

PHYSICAL EXAMINATION:
On examination, he has right-sided ptosis, ophthalmoplegia, and decreased corneal reflex. His right eye is proptotic and has chemosis. There is hypoesthesia over the right forehead and cheek.

LABORATORY DATA:
Blood cultures are pending
