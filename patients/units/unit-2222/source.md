# synthetic_hospital patient 2222

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-08-12 (Fri) | outpatient | Internal Medicine |
| 1 | 2020-03-15 | 2022-10-10 (Mon) | inpatient | Vascular Surgery |
| 2 | 2020-07-15 | 2023-02-16 (Thu) | outpatient | Neurology |
| 3 | 2021-01-15 | 2023-08-23 (Wed) | outpatient | Cardiology |
| 4 | 2022-01-15 | 2024-08-09 (Fri) | outpatient | Gastroenterology |
| 5 | 2022-07-15 | 2025-02-19 (Wed) | outpatient | Urology |
| 6 | 2023-01-15 | 2025-08-11 (Mon) | outpatient | Hematology & Oncology |
| 7 | 2023-07-15 | 2026-02-15 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired construction foreman",
 "alcohol_use": "Social, typically one glass of red wine with dinner most evenings",
 "family_history": [
  "Father: Deceased at age 72 from ruptured abdominal aortic aneurysm",
  "Mother: Type 2 diabetes mellitus, age-related macular degeneration",
  "Paternal uncle: Waldenstr\u00f6m macroglobulinemia",
  "Brother: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 40 pack-year history, quit 5 years ago following mild cerebrovascular event",
 "home_medications": [
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "12.5 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "5 mg PO daily (adjusted per INR)",
   "name": "Warfarin"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Omeprazole"
  }
 ],
 "surgical_history": [
  "Elective endovascular abdominal aortic aneurysm repair (EVAR)",
  "Diagnostic cystoscopy (pending)"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus",
  "Hypertension",
  "Atrial Fibrillation",
  "Hyperlipidemia",
  "Abdominal Aortic Aneurysm (repaired)",
  "History of mild ischemic stroke (fully recovered)",
  "Gastroesophageal Reflux Disease",
  "Peripheral Artery Disease"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Atrial fibrillation with rapid ventricular response", "Abdominal aortic aneurysm", "Cholesterol embolism", "Urinary tract infection", "Esophageal carcinoma", "Bladder cancer", "Inclusion body myositis", "Waldenstr\u00f6m macroglobulinemia"]
Comorbidities: ["Type 2 diabetes mellitus", "Obesity", "Alcohol use", "Essential hypertension", "Abdominal aortic aneurysm", "Tobacco use disorder, former smoker"]

## Encounter 0: source 2020-01-15, practice 2022-08-12, outpatient, Internal Medicine

Chief complaint: Routine health maintenance visit with no active complaints

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health maintenance visit with no active complaints

HISTORY OF PRESENT ILLNESS:
He has no complaints today.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus
- Hypertension
- Atrial Fibrillation
- Hyperlipidemia
- Abdominal Aortic Aneurysm (repaired)
- History of mild ischemic stroke (fully recovered)
- Gastroesophageal Reflux Disease
- Peripheral Artery Disease

His medical history is significant for hypertension, controlled with lisinopril, and hyperlipidemia, managed with atorvastatin.

PAST SURGICAL HISTORY:
- Elective endovascular abdominal aortic aneurysm repair (EVAR)
- Diagnostic cystoscopy (pending)

MEDICATIONS:
He takes lisinopril, atorvastatin.

Home Medications:
- Metformin 1000 mg PO twice daily
- Hydrochlorothiazide 12.5 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Warfarin 5 mg PO daily (adjusted per INR)
- Aspirin 81 mg PO daily
- Omeprazole 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Deceased at age 72 from ruptured abdominal aortic aneurysm
- Mother: Type 2 diabetes mellitus, age-related macular degeneration
- Paternal uncle: Waldenström macroglobulinemia
- Brother: Hypertension

SOCIAL HISTORY:
He quit smoking 5 years ago after a mild stroke.

VITAL SIGNS:
His blood pressure is 142/88 mmHg, heart rate is 78 beats per minute, and BMI is 29 kg/m^2.

PHYSICAL EXAMINATION:
On examination, his cardiovascular exam is unremarkable with no bruits, and his abdominal exam reveals a non-tender, pulsatile mass just above the umbilicus. His peripheral pulses are intact, and there is no lower extremity edema.

LABORATORY DATA:
His most recent laboratory results are as follows:

- Total cholesterol: 180 mg/dL
- HDL cholesterol: 45 mg/dL
- LDL cholesterol: 100 mg/dL
- Triglycerides: 175 mg/dL
- Fasting glucose: 98 mg/dL

## Encounter 1: source 2020-03-15, practice 2022-10-10, inpatient, Vascular Surgery

Chief complaint: Progressive kidney dysfunction and skin changes two weeks following elective EVAR

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. James Rodriguez, MD
Department: Vascular Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Progressive kidney dysfunction and skin changes two weeks following elective EVAR

HISTORY OF PRESENT ILLNESS:
This 68-year-old male returns two weeks after undergoing elective abdominal aortic aneurysm repair following his January 2020 diagnosis. He presents with progressive kidney dysfunction and skin changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Abdominal aortic aneurysm (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Atrial Fibrillation
- Hyperlipidemia
- Abdominal Aortic Aneurysm (repaired)
- History of mild ischemic stroke (fully recovered)
- Gastroesophageal Reflux Disease
- Peripheral Artery Disease

His past medical history includes hypertension, peripheral artery disease, and smoking.

PAST SURGICAL HISTORY:
- Elective endovascular abdominal aortic aneurysm repair (EVAR)
- Diagnostic cystoscopy (pending)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Warfarin 5 mg PO daily (adjusted per INR)
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily
- Omeprazole 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Deceased at age 72 from ruptured abdominal aortic aneurysm
- Mother: Type 2 diabetes mellitus, age-related macular degeneration
- Paternal uncle: Waldenström macroglobulinemia
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit 5 years ago following mild cerebrovascular event. Alcohol: Social, typically one glass of red wine with dinner most evenings. Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
On physical examination, he has livedo reticularis over his lower extremities and tenderness of his toes with bluish discoloration

LABORATORY DATA:
Laboratory evaluation reveals rising serum creatinine and eosinophilia. Urinalysis shows few white blood cells and no significant proteinuria

PATHOLOGY:
A biopsy is performed and histologic findings are shown in the image below

## Encounter 2: source 2020-07-15, practice 2023-02-16, outpatient, Neurology

Chief complaint: Progressive difficulty walking and proximal muscle weakness over two years

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Emily Park, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty walking and proximal muscle weakness over two years

HISTORY OF PRESENT ILLNESS:
The patient presents to the Neurology clinic four months following hospitalization for cholesterol embolism syndrome after elective endovascular aneurysm repair. He reports that it began as a subtle heaviness in his thighs while climbing stairs but has gradually worsened. Now, he struggles to get up from low chairs and often drops objects when reaching overhead or gripping tools. He denies pain, numbness, or fasciculations. There is no history of trauma, infection, or new medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Abdominal aortic aneurysm (diagnosed 2020-01-15)
- Cholesterol embolism (diagnosed 2020-03-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Atrial Fibrillation
- Hyperlipidemia
- Abdominal Aortic Aneurysm (repaired)
- History of mild ischemic stroke (fully recovered)
- Gastroesophageal Reflux Disease
- Peripheral Artery Disease

He has well-controlled hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Elective endovascular abdominal aortic aneurysm repair (EVAR)
- Diagnostic cystoscopy (pending)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Warfarin 5 mg PO daily (adjusted per INR)
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily
- Omeprazole 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Deceased at age 72 from ruptured abdominal aortic aneurysm
- Mother: Type 2 diabetes mellitus, age-related macular degeneration
- Paternal uncle: Waldenström macroglobulinemia
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit 5 years ago following mild cerebrovascular event. Alcohol: Social, typically one glass of red wine with dinner most evenings. Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
On neurologic examination, he has mild atrophy of the quadriceps and forearm flexors. Strength testing reveals weakness in both proximal and distal muscle groups in the upper and lower extremities. Reflexes are normal and symmetric. Sensation to light touch, vibration, and proprioception is intact.

LABORATORY DATA:
Laboratory studies show normal creatine kinase, electrolytes, and thyroid function.

PATHOLOGY:
The patient undergoes a muscle biopsy for further evaluation. Microscopic examination of muscle tissue reveals scattered basophilic vacuoles bordered by granular material within the myofibers.

OTHER STUDIES:
An electromyogram is nonspecific

## Encounter 3: source 2021-01-15, practice 2023-08-23, outpatient, Cardiology

Chief complaint: Routine cardiology follow-up for chronic atrial fibrillation

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Thompson, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine cardiology follow-up for chronic atrial fibrillation

PAST MEDICAL HISTORY:
Active Problem List:
- Abdominal aortic aneurysm (diagnosed 2020-01-15)
- Cholesterol embolism (diagnosed 2020-03-15)
- Inclusion body myositis (diagnosed 2020-07-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Atrial Fibrillation
- Hyperlipidemia
- Abdominal Aortic Aneurysm (repaired)
- History of mild ischemic stroke (fully recovered)
- Gastroesophageal Reflux Disease
- Peripheral Artery Disease

He has a history of type 2 diabetes mellitus, hypertension, atrial fibrillation, and an abdominal aortic aneurysm diagnosed 6 months ago and measuring 4.8 cm in diameter.

PAST SURGICAL HISTORY:
- Elective endovascular abdominal aortic aneurysm repair (EVAR)
- Diagnostic cystoscopy (pending)

MEDICATIONS:
His current medications include metformin, lisinopril, aspirin, warfarin, and metoprolol

Home Medications:
- Hydrochlorothiazide 12.5 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO daily
- Omeprazole 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Deceased at age 72 from ruptured abdominal aortic aneurysm
- Mother: Type 2 diabetes mellitus, age-related macular degeneration
- Paternal uncle: Waldenström macroglobulinemia
- Brother: Hypertension

SOCIAL HISTORY:
He smoked one pack of cigarettes daily for 30 years but quit 5 years ago. He drinks a glass of red wine with dinner most evenings

REVIEW OF SYSTEMS:
Upon questioning, He denies chest pain or shortness of breath, but mentions occasional bothersome palpitations

VITAL SIGNS:
Vital signs are: temperature 36.8°C (98.2°F), blood pressure 138/82 mmHg, pulse 64/min and irregularly irregular, and respiratory rate 16/min. He is 178 cm (5 ft 10 in) tall and weighs 84 kg (185 lb), with a BMI of 26.5 kg/m²

OTHER STUDIES:
An ECG shows atrial fibrillation without rapid ventricular response

## Encounter 4: source 2022-01-15, practice 2024-08-09, outpatient, Gastroenterology

Chief complaint: Three-month history of progressive dysphagia and unintentional weight loss

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of progressive dysphagia and unintentional weight loss

HISTORY OF PRESENT ILLNESS:
This 70-year-old male with a history of repaired abdominal aortic aneurysm and inclusion body myositis presents with a three-month history of progressive dysphagia. He reports that the problem began with trouble swallowing solid foods but has progressed to include some liquids. He describes a sensation of food 'sticking' in the middle of his chest after eating. He also reports unintentional weight loss of 10 pounds over the last two months and occasional retrosternal pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Abdominal aortic aneurysm (diagnosed 2020-01-15)
- Cholesterol embolism (diagnosed 2020-03-15)
- Inclusion body myositis (diagnosed 2020-07-15)
- Atrial fibrillation with rapid ventricular response (diagnosed 2021-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Atrial Fibrillation
- Hyperlipidemia
- Abdominal Aortic Aneurysm (repaired)
- History of mild ischemic stroke (fully recovered)
- Gastroesophageal Reflux Disease
- Peripheral Artery Disease

His past medical history includes hypertension and gastroesophageal reflux disease (GERD).

PAST SURGICAL HISTORY:
- Elective endovascular abdominal aortic aneurysm repair (EVAR)
- Diagnostic cystoscopy (pending)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Warfarin 5 mg PO daily (adjusted per INR)
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily
- Omeprazole 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Deceased at age 72 from ruptured abdominal aortic aneurysm
- Mother: Type 2 diabetes mellitus, age-related macular degeneration
- Paternal uncle: Waldenström macroglobulinemia
- Brother: Hypertension

SOCIAL HISTORY:
He has a 40-pack-year smoking history and drinks alcohol socially.

PHYSICAL EXAMINATION:
On physical examination, he appears cachectic. There is no lymphadenopathy or organomegaly.

LABORATORY DATA:
Laboratory tests are unremarkable.

OTHER STUDIES:
An upper endoscopy reveals a circumferential, ulcerated mass at the distal esophagus, and biopsies are taken.

## Encounter 5: source 2022-07-15, practice 2025-02-19, outpatient, Urology

Chief complaint: Two-month history of painless gross hematuria and urinary frequency

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David Kim, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Two-month history of painless gross hematuria and urinary frequency

PAST MEDICAL HISTORY:
Active Problem List:
- Abdominal aortic aneurysm (diagnosed 2020-01-15)
- Cholesterol embolism (diagnosed 2020-03-15)
- Inclusion body myositis (diagnosed 2020-07-15)
- Atrial fibrillation with rapid ventricular response (diagnosed 2021-01-15)
- Esophageal carcinoma (diagnosed 2022-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Atrial Fibrillation
- Hyperlipidemia
- Abdominal Aortic Aneurysm (repaired)
- History of mild ischemic stroke (fully recovered)
- Gastroesophageal Reflux Disease
- Peripheral Artery Disease

PAST SURGICAL HISTORY:
- Elective endovascular abdominal aortic aneurysm repair (EVAR)
- Diagnostic cystoscopy (pending)

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Warfarin 5 mg PO daily (adjusted per INR)
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily
- Omeprazole 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Deceased at age 72 from ruptured abdominal aortic aneurysm
- Mother: Type 2 diabetes mellitus, age-related macular degeneration
- Paternal uncle: Waldenström macroglobulinemia
- Brother: Hypertension

SOCIAL HISTORY:
40-pack-year smoking history but quit smoking 10 years ago

REVIEW OF SYSTEMS:
He denies any recent fever, weight loss, or significant pain

VITAL SIGNS:
Blood pressure is 135/85 mmHg, heart rate is 78 bpm

PHYSICAL EXAMINATION:
Abdominal examination reveals no masses or tenderness

LABORATORY DATA:
Urinalysis: Red blood cells >50/high-power field, no casts or crystals
Serum creatinine: 1.2 mg/dL (baseline 1.0 mg/dL)
Blood urea nitrogen (BUN): 18 mg/dL
Complete blood count (CBC): Within normal limits

IMAGING:
A renal ultrasound shows right-sided hydronephrosis without evidence of kidney stones.

## Encounter 6: source 2023-01-15, practice 2025-08-11, outpatient, Hematology & Oncology

Chief complaint: Two-month history of fatigue, headache, blurred vision, and spontaneous mucosal bleeding

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Lisa Wong, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Two-month history of fatigue, headache, blurred vision, and spontaneous mucosal bleeding

HISTORY OF PRESENT ILLNESS:
The patient is a 71-year-old male with prior esophageal and bladder cancer. He presents to Hematology with a 2-month history of fatigue, headache, and blurred vision and reports recent onset of spontaneous bleeding from his gums and nose.

PAST MEDICAL HISTORY:
Active Problem List:
- Abdominal aortic aneurysm (diagnosed 2020-01-15)
- Cholesterol embolism (diagnosed 2020-03-15)
- Inclusion body myositis (diagnosed 2020-07-15)
- Atrial fibrillation with rapid ventricular response (diagnosed 2021-01-15)
- Esophageal carcinoma (diagnosed 2022-01-15)
- Bladder cancer (diagnosed 2022-07-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Atrial Fibrillation
- Hyperlipidemia
- Abdominal Aortic Aneurysm (repaired)
- History of mild ischemic stroke (fully recovered)
- Gastroesophageal Reflux Disease
- Peripheral Artery Disease

His past medical history is significant for type 2 diabetes mellitus and hypertension, which are well-controlled with metformin and lisinopril, respectively.

PAST SURGICAL HISTORY:
- Elective endovascular abdominal aortic aneurysm repair (EVAR)
- Diagnostic cystoscopy (pending)

MEDICATIONS:
He takes metformin and lisinopril.

Home Medications:
- Hydrochlorothiazide 12.5 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Warfarin 5 mg PO daily (adjusted per INR)
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily
- Omeprazole 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Deceased at age 72 from ruptured abdominal aortic aneurysm
- Mother: Type 2 diabetes mellitus, age-related macular degeneration
- Paternal uncle: Waldenström macroglobulinemia
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit 5 years ago following mild cerebrovascular event. Alcohol: Social, typically one glass of red wine with dinner most evenings. Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
On physical examination, he appears pale and has engorged retinal veins on fundoscopic examination. Neurological examination reveals distal sensory loss in his lower extremities.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 10.2 g/dL (Normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 89 fL (Normal: 80-96 fL)
- Platelet count: 150,000/µL (Normal: 150,000-450,000/µL)
- Serum protein electrophoresis: Monoclonal IgM spike
- Serum viscosity: Elevated

PATHOLOGY:
Bone marrow biopsy: 15% clonal lymphoplasmacytic cells

## Encounter 7: source 2023-07-15, practice 2026-02-15, ed, Emergency Medicine

Chief complaint: Three-day history of fever, dysuria, and increased urinary frequency

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Robert Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Three-day history of fever, dysuria, and increased urinary frequency

HISTORY OF PRESENT ILLNESS:
This 71-year-old male with a history of bladder cancer and Waldenström macroglobulinemia presents to the emergency department with a 3-day history of fever, dysuria, and increased urinary frequency.

PAST MEDICAL HISTORY:
Active Problem List:
- Abdominal aortic aneurysm (diagnosed 2020-01-15)
- Cholesterol embolism (diagnosed 2020-03-15)
- Inclusion body myositis (diagnosed 2020-07-15)
- Atrial fibrillation with rapid ventricular response (diagnosed 2021-01-15)
- Esophageal carcinoma (diagnosed 2022-01-15)
- Bladder cancer (diagnosed 2022-07-15)
- Waldenström macroglobulinemia (diagnosed 2023-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Atrial Fibrillation
- Hyperlipidemia
- Abdominal Aortic Aneurysm (repaired)
- History of mild ischemic stroke (fully recovered)
- Gastroesophageal Reflux Disease
- Peripheral Artery Disease

PAST SURGICAL HISTORY:
- Elective endovascular abdominal aortic aneurysm repair (EVAR)
- Diagnostic cystoscopy (pending)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Warfarin 5 mg PO daily (adjusted per INR)
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily
- Omeprazole 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Deceased at age 72 from ruptured abdominal aortic aneurysm
- Mother: Type 2 diabetes mellitus, age-related macular degeneration
- Paternal uncle: Waldenström macroglobulinemia
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit 5 years ago following mild cerebrovascular event. Alcohol: Social, typically one glass of red wine with dinner most evenings. Occupation: Retired construction foreman.

VITAL SIGNS:
Blood pressure is 145/90 mmHg, and his heart rate is 88 beats per minute

PHYSICAL EXAMINATION:
Physical examination reveals suprapubic tenderness without flank pain

LABORATORY DATA:
Urinalysis shows pyuria and bacteriuria. The patient&#x27;s creatinine is 1.1 mg/dL, and his white blood cell count is 12,000/µL

IMAGING:
Recent abdominal ultrasound confirms the presence of the aneurysm without any change in size
