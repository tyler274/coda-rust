# synthetic_hospital patient 2226

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 72 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-11-18 (Fri) | outpatient | Internal Medicine |
| 1 | 2020-03-15 | 2023-01-25 (Wed) | outpatient | Orthopedic Surgery |
| 2 | 2021-01-15 | 2023-11-17 (Fri) | outpatient | Cardiology |
| 3 | 2021-07-15 | 2024-05-14 (Tue) | outpatient | Neurology |
| 4 | 2022-01-15 | 2024-11-27 (Wed) | outpatient | Gastroenterology |
| 5 | 2022-07-15 | 2025-05-28 (Wed) | outpatient | Internal Medicine |
| 6 | 2023-01-15 | 2025-11-11 (Tue) | outpatient | Gastroenterology |
| 7 | 2023-07-15 | 2026-05-18 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired high school mathematics teacher",
 "alcohol_use": "Social, 1-2 glasses of wine weekly",
 "family_history": [
  "No family history of colorectal cancer",
  "Negative for premature coronary artery disease",
  "Negative for hereditary hemorrhagic telangiectasia"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 15 years ago (20 pack-year history)",
 "home_medications": [
  {
   "dose": "5 mg daily (adjusted per INR goal 2.5-3.5 for mechanical valve)",
   "name": "Warfarin"
  },
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  },
  {
   "dose": "40 mg daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "5 mg daily",
   "name": "Amlodipine"
  },
  {
   "dose": "81 mg daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Mechanical mitral valve replacement (10 years prior)",
  "Right total hip arthroplasty (4 months prior)",
  "Colonoscopy with polypectomy (2 years prior)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 diabetes mellitus",
  "Hyperlipidemia",
  "Chronic kidney disease (Stage 3)",
  "Severe aortic stenosis",
  "Chronic constipation",
  "History of colonic polyps"
 ],
 "age_at_first_encounter": 72
}
```

Primary diagnoses: ["Status post mechanical mitral valve replacement", "Gastrointestinal angiodysplasia with hemorrhage", "Aortic valve stenosis", "Iron Deficiency Anemia", "Chronic prosthetic joint infection of right hip", "Normal pressure hydrocephalus"]
Comorbidities: ["Aortic valve stenosis", "Essential hypertension", "Type 2 diabetes mellitus", "Acute posthemorrhagic anemia"]

## Encounter 0: source 2020-01-15, practice 2022-11-18, outpatient, Internal Medicine

Chief complaint: Routine anticoagulation monitoring for mechanical mitral valve

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine anticoagulation monitoring for mechanical mitral valve

HISTORY OF PRESENT ILLNESS:
He has no current symptoms. He reports good medication adherence and no recent changes in diet or medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Chronic kidney disease (Stage 3)
- Severe aortic stenosis
- Chronic constipation
- History of colonic polyps

His medical history also includes hypertension and type 2 diabetes mellitus, both well controlled with lisinopril and metformin.

PAST SURGICAL HISTORY:
He underwent aortic valve replacement with a mechanical prosthesis three months ago due to severe aortic stenosis.

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily (adjusted per INR goal 2.5-3.5 for mechanical valve)
- Lisinopril 20 mg daily
- Metformin 1000 mg twice daily
- Atorvastatin 40 mg daily
- Amlodipine 5 mg daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- No family history of colorectal cancer
- Negative for premature coronary artery disease
- Negative for hereditary hemorrhagic telangiectasia

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
His blood pressure is 132/74 mmHg and pulse is 78/min and regular.

LABORATORY DATA:
Laboratory testing today shows an international normalized ratio (INR) of 2.6.

ASSESSMENT AND PLAN:
The drug he is taking to prevent thromboembolism after valve replacement primarily works by

## Encounter 1: source 2020-03-15, practice 2023-01-25, outpatient, Orthopedic Surgery

Chief complaint: Progressively worsening right hip pain with low-grade fever

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Michael Rodriguez, MD
Department: Orthopedic Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Progressively worsening right hip pain with low-grade fever

HISTORY OF PRESENT ILLNESS:
The patient returns for evaluation of progressive right hip pain, having been seen two months prior for routine anticoagulation monitoring of his mechanical mitral valve. He underwent a right total hip arthroplasty 4 months ago due to severe osteoarthritis. He reports that the pain is dull and constant, impairing his ability to walk. He denies recent trauma but mentions a low-grade fever and occasional night sweats.

PAST MEDICAL HISTORY:
Active Problem List:
- Status post mechanical mitral valve replacement (diagnosed 2020-01-15)
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Chronic kidney disease (Stage 3)
- Severe aortic stenosis
- Chronic constipation
- History of colonic polyps

His past medical history is significant for type 2 diabetes mellitus and hypertension.

PAST SURGICAL HISTORY:
- Mechanical mitral valve replacement (10 years prior)
- Right total hip arthroplasty (4 months prior)
- Colonoscopy with polypectomy (2 years prior)

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily (adjusted per INR goal 2.5-3.5 for mechanical valve)
- Lisinopril 20 mg daily
- Metformin 1000 mg twice daily
- Atorvastatin 40 mg daily
- Amlodipine 5 mg daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- No family history of colorectal cancer
- Negative for premature coronary artery disease
- Negative for hereditary hemorrhagic telangiectasia

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, there is no erythema or warmth over the hip, but there is tenderness on palpation around the prosthesis. Range of motion is limited due to pain.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 9,000/uL
- Erythrocyte sedimentation rate (ESR): 55 mm/hr
- C-reactive protein (CRP): 25 mg/L
- Blood cultures: No growth

OTHER STUDIES:
An aspiration of the hip joint is performed, and the synovial fluid analysis reveals:

- White blood cell count: 3,500/uL (with 65% neutrophils)
- Gram stain: No organisms seen
- Culture: Pending

## Encounter 2: source 2021-01-15, practice 2023-11-17, outpatient, Cardiology

Chief complaint: Increasing shortness of breath on exertion and occasional dizziness

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Emily Watson, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing shortness of breath on exertion and occasional dizziness

HISTORY OF PRESENT ILLNESS:
The patient with mechanical mitral valve replacement presents with increasing shortness of breath on exertion and occasional dizziness for the past 6 months. He denies chest pain, palpitations, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Status post mechanical mitral valve replacement (diagnosed 2020-01-15)
- Chronic prosthetic joint infection of right hip (diagnosed 2020-03-15)
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Chronic kidney disease (Stage 3)
- Severe aortic stenosis
- Chronic constipation
- History of colonic polyps

PAST SURGICAL HISTORY:
- Mechanical mitral valve replacement (10 years prior)
- Right total hip arthroplasty (4 months prior)
- Colonoscopy with polypectomy (2 years prior)

MEDICATIONS:
He takes lisinopril and atorvastatin.

Home Medications:
- Warfarin 5 mg daily (adjusted per INR goal 2.5-3.5 for mechanical valve)
- Metformin 1000 mg twice daily
- Amlodipine 5 mg daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- No family history of colorectal cancer
- Negative for premature coronary artery disease
- Negative for hereditary hemorrhagic telangiectasia

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
Blood pressure is 135/85 mmHg, heart rate is 78 bpm

PHYSICAL EXAMINATION:
Harsh, late-peaking systolic murmur best heard at the right second intercostal space that radiates to the carotids

IMAGING:
Calcified aortic valve with a valve area of 0.8 cm², mean gradient of 50 mmHg, and left ventricular ejection fraction (LVEF) of 48%

PLAN:
Scheduled for colonoscopy due to history of colonic polyps

## Encounter 3: source 2021-07-15, practice 2024-05-14, outpatient, Neurology

Chief complaint: Progressive difficulty walking with falls and urinary incontinence

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James O'Brien, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty walking with falls and urinary incontinence

HISTORY OF PRESENT ILLNESS:
The patient is a 73-year-old male with severe aortic stenosis and mechanical mitral valve who presents for evaluation. Family reports he shuffles his feet, has fallen several times, and has become increasingly forgetful, occasionally failing to make it to the bathroom in time.

PAST MEDICAL HISTORY:
Active Problem List:
- Status post mechanical mitral valve replacement (diagnosed 2020-01-15)
- Chronic prosthetic joint infection of right hip (diagnosed 2020-03-15)
- Aortic valve stenosis (diagnosed 2021-01-15)
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Chronic kidney disease (Stage 3)
- Severe aortic stenosis
- Chronic constipation
- History of colonic polyps

His past medical history includes hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Mechanical mitral valve replacement (10 years prior)
- Right total hip arthroplasty (4 months prior)
- Colonoscopy with polypectomy (2 years prior)

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily (adjusted per INR goal 2.5-3.5 for mechanical valve)
- Lisinopril 20 mg daily
- Metformin 1000 mg twice daily
- Atorvastatin 40 mg daily
- Amlodipine 5 mg daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- No family history of colorectal cancer
- Negative for premature coronary artery disease
- Negative for hereditary hemorrhagic telangiectasia

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, the patient has a broad-based gait, difficulty initiating steps, and magnetic gait. There is no muscle weakness or sensory deficit. His Mini-Mental State Examination score is 24/30.

IMAGING:
A non-contrast head CT scan shows enlarged ventricles with normal cortical sulci.

OTHER STUDIES:
A high-volume lumbar puncture is performed, and the patient's gait improves temporarily.

## Encounter 4: source 2022-01-15, practice 2024-11-27, outpatient, Gastroenterology

Chief complaint: Recurrent episodes of painless bright red blood per rectum

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Recurrent episodes of painless bright red blood per rectum

HISTORY OF PRESENT ILLNESS:
The patient with mechanical valve and prior colonic polyps presents with bleeding over the past month. Each episode lasts 1–2 days and resolves spontaneously. He denies abdominal pain, diarrhea, constipation, or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Status post mechanical mitral valve replacement (diagnosed 2020-01-15)
- Chronic prosthetic joint infection of right hip (diagnosed 2020-03-15)
- Aortic valve stenosis (diagnosed 2021-01-15)
- Normal pressure hydrocephalus (diagnosed 2021-07-15)
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Chronic kidney disease (Stage 3)
- Severe aortic stenosis
- Chronic constipation
- History of colonic polyps

His medical history includes chronic kidney disease and severe aortic stenosis.

PAST SURGICAL HISTORY:
- Mechanical mitral valve replacement (10 years prior)
- Right total hip arthroplasty (4 months prior)
- Colonoscopy with polypectomy (2 years prior)

MEDICATIONS:
He takes lisinopril and atorvastatin.

Home Medications:
- Warfarin 5 mg daily (adjusted per INR goal 2.5-3.5 for mechanical valve)
- Metformin 1000 mg twice daily
- Amlodipine 5 mg daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- No family history of colorectal cancer
- Negative for premature coronary artery disease
- Negative for hereditary hemorrhagic telangiectasia

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
On examination, his temperature is 36.8°C (98.2°F), blood pressure is 128/70 mm Hg, pulse is 80/min, and respirations are 14/min.

PHYSICAL EXAMINATION:
Abdominal examination is benign. Digital rectal examination shows gross blood, but no masses are palpable.

LABORATORY DATA:
Laboratory testing reveals a hemoglobin of 9.8 g/dL and mean corpuscular volume of 82 fL.

IMAGING:
Given the persistence of intermittent bleeding and normal colonoscopic findings, a nuclear medicine-tagged red blood cell scan is performed and suggests bleeding from the cecum.

OTHER STUDIES:
The patient undergoes colonoscopy during an asymptomatic period, which does not reveal any masses, ulcers, or active bleeding sites. A repeat colonoscopy during active bleeding demonstrates small, flat, cherry-red lesions with ectatic vessels in the cecum.

## Encounter 5: source 2022-07-15, practice 2025-05-28, outpatient, Internal Medicine

Chief complaint: Progressive fatigue and shortness of breath on exertion

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David Kim, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue and shortness of breath on exertion

HISTORY OF PRESENT ILLNESS:
A 74-year-old male with mechanical mitral valve and recent GI bleeding presents to the clinic with a 3-month history of progressive fatigue and shortness of breath on exertion. He denies any chest pain, palpitations, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Status post mechanical mitral valve replacement (diagnosed 2020-01-15)
- Chronic prosthetic joint infection of right hip (diagnosed 2020-03-15)
- Aortic valve stenosis (diagnosed 2021-01-15)
- Normal pressure hydrocephalus (diagnosed 2021-07-15)
- Gastrointestinal angiodysplasia with hemorrhage (diagnosed 2022-01-15)
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Chronic kidney disease (Stage 3)
- Severe aortic stenosis
- Chronic constipation
- History of colonic polyps

His past medical history is significant for hypertension, for which he takes hydrochlorothiazide, and type 2 diabetes mellitus controlled with metformin. He has a 10-year history of chronic constipation for which he occasionally uses over-the-counter stool softeners.

PAST SURGICAL HISTORY:
- Mechanical mitral valve replacement (10 years prior)
- Right total hip arthroplasty (4 months prior)
- Colonoscopy with polypectomy (2 years prior)

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily (adjusted per INR goal 2.5-3.5 for mechanical valve)
- Lisinopril 20 mg daily
- Metformin 1000 mg twice daily
- Atorvastatin 40 mg daily
- Amlodipine 5 mg daily
- Aspirin 81 mg daily

ALLERGIES:
He has no known drug allergies.

FAMILY HISTORY:
- No family history of colorectal cancer
- Negative for premature coronary artery disease
- Negative for hereditary hemorrhagic telangiectasia

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, his blood pressure is 130/80 mmHg, heart rate is 88 beats per minute, and he appears pale. His cardiac examination is normal, and there is no palpable hepatosplenomegaly. Rectal examination reveals no masses, and the stool is heme-negative.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 9.2 g/dL (Normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 72 fL (Normal: 80-96 fL)
- Serum ferritin: 15 ng/mL (Normal: 24-336 ng/mL)
- Serum iron: 30 µg/dL (Normal: 65-176 µg/dL)
- Total iron-binding capacity (TIBC): 450 µg/dL (Normal: 250-450 µg/dL)

ASSESSMENT AND PLAN:
Given the patient's age, clinical presentation, and laboratory findings,

## Encounter 6: source 2023-01-15, practice 2025-11-11, outpatient, Gastroenterology

Chief complaint: Episodes of bright red blood per rectum over the past week

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Episodes of bright red blood per rectum over the past week

HISTORY OF PRESENT ILLNESS:
The patient with mechanical mitral valve on warfarin and severe aortic stenosis returns with recurrent rectal bleeding following prior gastrointestinal angiodysplasia and iron deficiency anemia. He denies associated abdominal pain, nausea, or changes in bowel habits. The bleeding occurs intermittently and is described as painless. He denies dizziness or syncope, but appears pale, and his daughter notes he has seemed more fatigued recently.

PAST MEDICAL HISTORY:
Active Problem List:
- Status post mechanical mitral valve replacement (diagnosed 2020-01-15)
- Chronic prosthetic joint infection of right hip (diagnosed 2020-03-15)
- Aortic valve stenosis (diagnosed 2021-01-15)
- Normal pressure hydrocephalus (diagnosed 2021-07-15)
- Gastrointestinal angiodysplasia with hemorrhage (diagnosed 2022-01-15)
- Iron Deficiency Anemia (diagnosed 2022-07-15)
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Chronic kidney disease (Stage 3)
- Severe aortic stenosis
- Chronic constipation
- History of colonic polyps

His past medical history includes hypertension and known aortic stenosis, for which he is followed by cardiology. He has no personal or family history of colorectal cancer.

PAST SURGICAL HISTORY:
He underwent a colonoscopy two years ago that was reported as normal.

MEDICATIONS:
His medications include aspirin and amlodipine.

Home Medications:
- Warfarin 5 mg daily (adjusted per INR goal 2.5-3.5 for mechanical valve)
- Lisinopril 20 mg daily
- Metformin 1000 mg twice daily
- Atorvastatin 40 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
He has no personal or family history of colorectal cancer

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
Vital signs are stable.

PHYSICAL EXAMINATION:
On examination, he has a grade 3/6 systolic murmur heard best at the right upper sternal border, with diminished carotid upstroke. Rectal examination shows no hemorrhoids or masses but is positive for bright red blood.

LABORATORY DATA:
Laboratory studies reveal hemoglobin of 9.3 g/dL (baseline 12.6).

IMAGING:
An abdominal CT angiography is unremarkable.

OTHER STUDIES:
Colonoscopy is repeated and shows no active source of bleeding.

PLAN:
He is admitted for observation,

## Encounter 7: source 2023-07-15, practice 2026-05-18, ed, Emergency Medicine

Chief complaint: Fatigue and shortness of breath with history of maroon-colored stools

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Maria Gonzalez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fatigue and shortness of breath with history of maroon-colored stools

HISTORY OF PRESENT ILLNESS:
The patient with a history of recurrent gastrointestinal angiodysplasia and mechanical mitral valve replacement on anticoagulation presents to the emergency department with a two-day history of fatigue and shortness of breath. He reports a history of intermittent maroon-colored stools over the past six months, which he attributed to his diet.

PAST MEDICAL HISTORY:
Active Problem List:
- Status post mechanical mitral valve replacement (diagnosed 2020-01-15)
- Chronic prosthetic joint infection of right hip (diagnosed 2020-03-15)
- Aortic valve stenosis (diagnosed 2021-01-15)
- Normal pressure hydrocephalus (diagnosed 2021-07-15)
- Gastrointestinal angiodysplasia with hemorrhage (diagnosed 2022-01-15)
- Iron Deficiency Anemia (diagnosed 2022-07-15)
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Chronic kidney disease (Stage 3)
- Severe aortic stenosis
- Chronic constipation
- History of colonic polyps

He has a past medical history of hypertension and was diagnosed with a heart murmur during a routine check-up one year ago, but he did not follow up for further evaluation.

PAST SURGICAL HISTORY:
- Mechanical mitral valve replacement (10 years prior)
- Right total hip arthroplasty (4 months prior)
- Colonoscopy with polypectomy (2 years prior)

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily (adjusted per INR goal 2.5-3.5 for mechanical valve)
- Lisinopril 20 mg daily
- Metformin 1000 mg twice daily
- Atorvastatin 40 mg daily
- Amlodipine 5 mg daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- No family history of colorectal cancer
- Negative for premature coronary artery disease
- Negative for hereditary hemorrhagic telangiectasia

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
On physical examination, his blood pressure is 145/90 mmHg, heart rate is 92 beats per minute, and oxygen saturation is 96% on room air.

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a harsh, late-peaking systolic murmur best heard at the right second intercostal space that radiates to the neck. Abdominal examination is unremarkable. Rectal examination shows maroon-colored stool in the rectal vault.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 9.8 g/dL (Normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 89 fL (Normal: 80-96 fL)
- Blood urea nitrogen (BUN): 28 mg/dL (Normal: 7-20 mg/dL)
- Creatinine: 1.2 mg/dL (Normal: 0.6-1.2 mg/dL)
- INR: 1.0 (Normal: 0.8-1.2)

OTHER STUDIES:
An upper endoscopy and colonoscopy do not reveal any source of bleeding.
