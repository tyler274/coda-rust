# synthetic_hospital patient 2110

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-05-09 (Mon) | outpatient | Cardiology |
| 1 | 2020-07-15 | 2022-10-24 (Mon) | outpatient | Internal Medicine |
| 2 | 2021-07-15 | 2023-11-07 (Tue) | outpatient | Cardiology |
| 3 | 2022-07-15 | 2024-10-22 (Tue) | inpatient | Infectious Disease |
| 4 | 2023-01-15 | 2025-04-21 (Mon) | inpatient | Infectious Disease |
| 5 | 2023-03-15 | 2025-06-23 (Mon) | ed | Emergency Medicine |
| 6 | 2023-04-15 | 2025-07-20 (Sun) | ed | Emergency Medicine |
| 7 | 2023-07-15 | 2025-10-27 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Unemployed; former warehouse stock clerk",
 "alcohol_use": "Occasional social use, 1-2 drinks per month",
 "family_history": [
  "Father: Type 2 Diabetes Mellitus, diagnosed age 50",
  "Mother: Essential hypertension",
  "Brother: Ankylosing spondylitis",
  "Paternal grandfather: Abdominal aortic aneurysm"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 0.5 packs per day for 15 years",
 "home_medications": [
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "30 units SC at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "6.25 mg PO BID",
   "name": "Carvedilol"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Furosemide"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "250 mg PO BID as needed for back pain",
   "name": "Naproxen"
  }
 ],
 "surgical_history": [
  "Dental extraction, age 20",
  "Incision and drainage of right thigh abscess, age 40"
 ],
 "chronic_conditions": [
  "Diabetes Mellitus Type 2, poorly controlled",
  "Syphilitic aortitis (complication of untreated infection 20 years prior)",
  "Aortic regurgitation, severe",
  "Mitral valve prolapse",
  "Ankylosing spondylitis",
  "History of intravenous drug use disorder",
  "Recurrent anterior uveitis",
  "Recurrent oral aphthous ulcers"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Infective Endocarditis (Aortic Valve) with Complications", "Aortic Regurgitation", "Penicillin-resistant Staphylococcus aureus infection via beta-lactamase production (MSSA)", "Syphilitic aortitis"]
Comorbidities: ["Staphylococcus aureus bacteremia", "Aortic Regurgitation"]

## Encounter 0: source 2020-01-15, practice 2022-05-09, outpatient, Cardiology

Chief complaint: Shortness of breath and fatigue on exertion for the past six months with occasional palpitations

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Shortness of breath and fatigue on exertion for the past six months with occasional palpitations

HISTORY OF PRESENT ILLNESS:
He also reports occasional palpitations and a feeling of his heart "racing" at night.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetes Mellitus Type 2, poorly controlled
- Syphilitic aortitis (complication of untreated infection 20 years prior)
- Aortic regurgitation, severe
- Mitral valve prolapse
- Ankylosing spondylitis
- History of intravenous drug use disorder
- Recurrent anterior uveitis
- Recurrent oral aphthous ulcers

His past medical history is significant for untreated syphilis diagnosed 20 years ago.

PAST SURGICAL HISTORY:
- Dental extraction, age 20
- Incision and drainage of right thigh abscess, age 40

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Insulin glargine 30 units SC at bedtime
- Lisinopril 10 mg PO daily
- Carvedilol 6.25 mg PO BID
- Furosemide 40 mg PO daily
- Aspirin 81 mg PO daily
- Naproxen 250 mg PO BID as needed for back pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, diagnosed age 50
- Mother: Essential hypertension
- Brother: Ankylosing spondylitis
- Paternal grandfather: Abdominal aortic aneurysm

SOCIAL HISTORY:
Smoking: Current smoker, 0.5 packs per day for 15 years. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Unemployed; former warehouse stock clerk.

VITAL SIGNS:
On physical examination, his blood pressure is 145/55 mmHg, and his pulse is 110 bpm

PHYSICAL EXAMINATION:
Collapsing in nature. Cardiac auscultation reveals a high-pitched, blowing, early diastolic murmur heard best at the left sternal border in the third intercostal space during end-expiration with the patient sitting up and leaning forward.

IMAGING:
His chest X-ray shows a dilated ascending aorta. An echocardiogram confirms aortic root dilatation and aortic valve regurgitation.

## Encounter 1: source 2020-07-15, practice 2022-10-24, outpatient, Internal Medicine

Chief complaint: Routine follow-up for chronic lower back pain with recent onset shortness of breath when lying down

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for chronic lower back pain with recent onset shortness of breath when lying down

HISTORY OF PRESENT ILLNESS:
Returning for follow-up of chronic back pain in the setting of known syphilitic aortitis, the patient reports pain that improves with exercise but not with rest. He also reports recent onset of shortness of breath, especially when lying down.

PAST MEDICAL HISTORY:
Active Problem List:
- Syphilitic aortitis (diagnosed 2020-01-15)
- Diabetes Mellitus Type 2, poorly controlled
- Syphilitic aortitis (complication of untreated infection 20 years prior)
- Aortic regurgitation, severe
- Mitral valve prolapse
- Ankylosing spondylitis
- History of intravenous drug use disorder
- Recurrent anterior uveitis
- Recurrent oral aphthous ulcers

He has a history of uveitis and recurrent oral ulcers.

PAST SURGICAL HISTORY:
- Dental extraction, age 20
- Incision and drainage of right thigh abscess, age 40

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Insulin glargine 30 units SC at bedtime
- Lisinopril 10 mg PO daily
- Carvedilol 6.25 mg PO BID
- Furosemide 40 mg PO daily
- Aspirin 81 mg PO daily
- Naproxen 250 mg PO BID as needed for back pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, diagnosed age 50
- Mother: Essential hypertension
- Brother: Ankylosing spondylitis
- Paternal grandfather: Abdominal aortic aneurysm

SOCIAL HISTORY:
Smoking: Current smoker, 0.5 packs per day for 15 years. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Unemployed; former warehouse stock clerk.

PHYSICAL EXAMINATION:
On physical examination, his lumbar spine shows decreased range of motion, and there is tenderness at the insertion of the Achilles tendon. Auscultation of the heart reveals a diastolic decrescendo murmur at the left sternal border.

LABORATORY DATA:
Laboratory findings are as follows:

- HLA-B27 antigen: Positive
- Erythrocyte sedimentation rate (ESR): Elevated
- C-reactive protein (CRP): Elevated
- Complete blood count (CBC): Normal

IMAGING:
His chest X-ray shows evidence of bilateral sacroiliitis.

OTHER STUDIES:
- Electrocardiogram (ECG): Normal sinus rhythm with no evidence of conduction abnormalities

## Encounter 2: source 2021-07-15, practice 2023-11-07, outpatient, Cardiology

Chief complaint: Six-month history of exertional dyspnea, fatigue, and nocturnal heart pounding

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of exertional dyspnea, fatigue, and nocturnal heart pounding

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of syphilitic aortitis and aortic regurgitation. He reports a sensation of his heart "pounding" in his chest, especially when lying on his left side at night.

PAST MEDICAL HISTORY:
Active Problem List:
- Syphilitic aortitis (diagnosed 2020-01-15)
- Aortic Regurgitation (diagnosed 2020-07-15)
- Diabetes Mellitus Type 2, poorly controlled
- Syphilitic aortitis (complication of untreated infection 20 years prior)
- Mitral valve prolapse
- Ankylosing spondylitis
- History of intravenous drug use disorder
- Recurrent anterior uveitis
- Recurrent oral aphthous ulcers

His past medical history is significant for untreated syphilis diagnosed 20 years ago.

PAST SURGICAL HISTORY:
- Dental extraction, age 20
- Incision and drainage of right thigh abscess, age 40

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Insulin glargine 30 units SC at bedtime
- Lisinopril 10 mg PO daily
- Carvedilol 6.25 mg PO BID
- Furosemide 40 mg PO daily
- Aspirin 81 mg PO daily
- Naproxen 250 mg PO BID as needed for back pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, diagnosed age 50
- Mother: Essential hypertension
- Brother: Ankylosing spondylitis
- Paternal grandfather: Abdominal aortic aneurysm

SOCIAL HISTORY:
Smoking: Current smoker, 0.5 packs per day for 15 years. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Unemployed; former warehouse stock clerk.

VITAL SIGNS:
On examination, his blood pressure is 155/45 mmHg, and his pulse is 110 bpm and collapsing in nature.

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a high-pitched, blowing diastolic murmur best heard along the left sternal border.

IMAGING:
His chest X-ray shows cardiomegaly with prominence of the ascending aorta. An echocardiogram demonstrates a dilated left ventricle with normal systolic function and severe aortic regurgitation.

## Encounter 3: source 2022-07-15, practice 2024-10-22, inpatient, Infectious Disease

Chief complaint: Non-healing ulcer on right foot with fever and worsening erythema

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Priya Patel, MD
Department: Infectious Disease
Visit Type: inpatient

CHIEF COMPLAINT:
Non-healing ulcer on right foot with fever and worsening erythema

HISTORY OF PRESENT ILLNESS:
The patient, with established history of syphilitic aortitis and severe aortic regurgitation followed by cardiology since 2020, is hospitalized for treatment of a non-healing ulcer on the sole of his right foot. Over the past two days, he has developed fever and worsening erythema and swelling around the wound.

PAST MEDICAL HISTORY:
Active Problem List:
- Syphilitic aortitis (diagnosed 2020-01-15)
- Aortic Regurgitation (diagnosed 2020-07-15)
- Diabetes Mellitus Type 2, poorly controlled
- Syphilitic aortitis (complication of untreated infection 20 years prior)
- Mitral valve prolapse
- Ankylosing spondylitis
- History of intravenous drug use disorder
- Recurrent anterior uveitis
- Recurrent oral aphthous ulcers

PAST SURGICAL HISTORY:
- Dental extraction, age 20
- Incision and drainage of right thigh abscess, age 40

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Insulin glargine 30 units SC at bedtime
- Lisinopril 10 mg PO daily
- Carvedilol 6.25 mg PO BID
- Furosemide 40 mg PO daily
- Aspirin 81 mg PO daily
- Naproxen 250 mg PO BID as needed for back pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, diagnosed age 50
- Mother: Essential hypertension
- Brother: Ankylosing spondylitis
- Paternal grandfather: Abdominal aortic aneurysm

SOCIAL HISTORY:
Smoking: Current smoker, 0.5 packs per day for 15 years. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Unemployed; former warehouse stock clerk.

PHYSICAL EXAMINATION:
Physical exam reveals a 4-cm ulcer with purulent drainage and surrounding cellulitis.

LABORATORY DATA:
Blood cultures grow Staphylococcus aureus . Susceptibility testing reveals resistance to ampicillin, cephalexin, and ceftriaxone, but preserved sensitivity to piperacillin-tazobactam. The organism is also sensitive to vancomycin and linezolid. The microbiology lab reports that the isolate does not carry the mecA gene.

## Encounter 4: source 2023-01-15, practice 2025-04-21, inpatient, Infectious Disease

Chief complaint: Six-week history of fatigue and intermittent low-grade fevers following recent dental cleaning

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Priya Patel, MD
Department: Infectious Disease
Visit Type: inpatient

CHIEF COMPLAINT:
Six-week history of fatigue and intermittent low-grade fevers following recent dental cleaning

HISTORY OF PRESENT ILLNESS:
The patient with severe aortic regurgitation presents with six-week fatigue and intermittent low-grade fevers. He reports a recent dental cleaning 2 months ago but did not receive any antibiotics prior to the procedure.

PAST MEDICAL HISTORY:
Active Problem List:
- Syphilitic aortitis (diagnosed 2020-01-15)
- Aortic Regurgitation (diagnosed 2020-07-15)
- Penicillin-resistant Staphylococcus aureus infection via beta-lactamase production (MSSA) (diagnosed 2022-07-15)
- Diabetes Mellitus Type 2, poorly controlled
- Syphilitic aortitis (complication of untreated infection 20 years prior)
- Mitral valve prolapse
- Ankylosing spondylitis
- History of intravenous drug use disorder
- Recurrent anterior uveitis
- Recurrent oral aphthous ulcers

PAST SURGICAL HISTORY:
- Dental extraction, age 20
- Incision and drainage of right thigh abscess, age 40

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Insulin glargine 30 units SC at bedtime
- Lisinopril 10 mg PO daily
- Carvedilol 6.25 mg PO BID
- Furosemide 40 mg PO daily
- Aspirin 81 mg PO daily
- Naproxen 250 mg PO BID as needed for back pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, diagnosed age 50
- Mother: Essential hypertension
- Brother: Ankylosing spondylitis
- Paternal grandfather: Abdominal aortic aneurysm

SOCIAL HISTORY:
Smoking: Current smoker, 0.5 packs per day for 15 years. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Unemployed; former warehouse stock clerk.

VITAL SIGNS:
Temperature is 37.8°C (100°F), heart rate is 90 bpm, and blood pressure is 130/85 mmHg

PHYSICAL EXAMINATION:
A new regurgitant murmur is noted at the apex, and splinter hemorrhages are present on examination of the nails.

LABORATORY DATA:
White blood cell count: 11,000 /µL
- Hemoglobin: 12.5 g/dL
- Erythrocyte sedimentation rate (ESR): 45 mm/hr
- Blood cultures: Pending

IMAGING:
An echocardiogram reveals vegetations on the mitral valve.

## Encounter 5: source 2023-03-15, practice 2025-06-23, ed, Emergency Medicine

Chief complaint: Two-week history of fever, fatigue, and new-onset regurgitant murmur

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-week history of fever, fatigue, and new-onset regurgitant murmur

HISTORY OF PRESENT ILLNESS:
The patient returns to the emergency department two months after recent infective endocarditis and presents with a 2-week history of fever, fatigue, and a new-onset regurgitant murmur. He denies any recent dental procedures or other infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Syphilitic aortitis (diagnosed 2020-01-15)
- Aortic Regurgitation (diagnosed 2020-07-15)
- Penicillin-resistant Staphylococcus aureus infection via beta-lactamase production (MSSA) (diagnosed 2022-07-15)
- Infective Endocarditis (Aortic Valve) with Complications (diagnosed 2023-01-15)
- Diabetes Mellitus Type 2, poorly controlled
- Syphilitic aortitis (complication of untreated infection 20 years prior)
- Mitral valve prolapse
- Ankylosing spondylitis
- History of intravenous drug use disorder
- Recurrent anterior uveitis
- Recurrent oral aphthous ulcers

PAST SURGICAL HISTORY:
- Dental extraction, age 20
- Incision and drainage of right thigh abscess, age 40

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Insulin glargine 30 units SC at bedtime
- Lisinopril 10 mg PO daily
- Carvedilol 6.25 mg PO BID
- Furosemide 40 mg PO daily
- Aspirin 81 mg PO daily
- Naproxen 250 mg PO BID as needed for back pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, diagnosed age 50
- Mother: Essential hypertension
- Brother: Ankylosing spondylitis
- Paternal grandfather: Abdominal aortic aneurysm

SOCIAL HISTORY:
History of intravenous drug use

VITAL SIGNS:
His temperature is 38.5°C, blood pressure is 110/70 mmHg, heart rate is 105 bpm, and respiratory rate is 18 breaths per minute.

PHYSICAL EXAMINATION:
Physical examination reveals Janeway lesions on his palms and soles, and Osler's nodes on his fingertips. A heart murmur consistent with mitral valve regurgitation is noted.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 12,000 /µL
- Hemoglobin: 13.2 g/dL
- Platelet count: 250,000 /µL
- Erythrocyte sedimentation rate: 45 mm/hr
- C-reactive protein: 20 mg/L

IMAGING:
A transthoracic echocardiogram shows vegetations on the mitral valve.

ASSESSMENT AND PLAN:
The patient is diagnosed with suspected infective endocarditis.

## Encounter 6: source 2023-04-15, practice 2025-07-20, ed, Emergency Medicine

Chief complaint: Two-week history of fever, malaise, and new-onset heart murmur

ENCOUNTER NOTE
Date: 2023-04-15
Provider: Dr. Aisha Washington, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-week history of fever, malaise, and new-onset heart murmur

HISTORY OF PRESENT ILLNESS:
The patient with severe aortic regurgitation secondary to syphilitic aortitis returns to the emergency department following recent treatment for infective endocarditis, presenting with a 2-week history of fever, malaise, and a new-onset heart murmur. He denies any chest pain, palpitations, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Syphilitic aortitis (diagnosed 2020-01-15)
- Aortic Regurgitation (diagnosed 2020-07-15)
- Penicillin-resistant Staphylococcus aureus infection via beta-lactamase production (MSSA) (diagnosed 2022-07-15)
- Infective Endocarditis (Aortic Valve) with Complications (diagnosed 2023-01-15)
- Diabetes Mellitus Type 2, poorly controlled
- Syphilitic aortitis (complication of untreated infection 20 years prior)
- Mitral valve prolapse
- Ankylosing spondylitis
- History of intravenous drug use disorder
- Recurrent anterior uveitis
- Recurrent oral aphthous ulcers

PAST SURGICAL HISTORY:
- Dental extraction, age 20
- Incision and drainage of right thigh abscess, age 40

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Insulin glargine 30 units SC at bedtime
- Lisinopril 10 mg PO daily
- Carvedilol 6.25 mg PO BID
- Furosemide 40 mg PO daily
- Aspirin 81 mg PO daily
- Naproxen 250 mg PO BID as needed for back pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, diagnosed age 50
- Mother: Essential hypertension
- Brother: Ankylosing spondylitis
- Paternal grandfather: Abdominal aortic aneurysm

SOCIAL HISTORY:
History of intravenous drug use

VITAL SIGNS:
On examination, his temperature is 38.5°C, blood pressure is 110/70 mmHg, heart rate is 98 bpm, and respiratory rate is 20 breaths per minute.

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a diastolic murmur best heard at the right second intercostal space.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 14,000 /µL
- Hemoglobin: 12.5 g/dL
- Platelet count: 250,000 /µL
- Erythrocyte sedimentation rate: 85 mm/hr
- Blood cultures: Positive for Staphylococcus aureus

IMAGING:
A transthoracic echocardiogram demonstrates a vegetation on the aortic valve with mild aortic regurgitation.

OTHER STUDIES:
An electrocardiogram (ECG) shows a new first-degree atrioventricular (AV) block.

## Encounter 7: source 2023-07-15, practice 2025-10-27, ed, Emergency Medicine

Chief complaint: Two-week history of progressive shortness of breath, fatigue, and 4-kg weight gain

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Aisha Washington, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-week history of progressive shortness of breath, fatigue, and 4-kg weight gain

HISTORY OF PRESENT ILLNESS:
The patient with syphilitic aortitis and recurrent infective endocarditis returns to the emergency department with a 2-week history of progressive shortness of breath, fatigue, and a 4-kg weight gain. He denies chest pain, fever, or cough.

PAST MEDICAL HISTORY:
Active Problem List:
- Syphilitic aortitis (diagnosed 2020-01-15)
- Aortic Regurgitation (diagnosed 2020-07-15)
- Penicillin-resistant Staphylococcus aureus infection via beta-lactamase production (MSSA) (diagnosed 2022-07-15)
- Infective Endocarditis (Aortic Valve) with Complications (diagnosed 2023-01-15)
- Diabetes Mellitus Type 2, poorly controlled
- Syphilitic aortitis (complication of untreated infection 20 years prior)
- Mitral valve prolapse
- Ankylosing spondylitis
- History of intravenous drug use disorder
- Recurrent anterior uveitis
- Recurrent oral aphthous ulcers

PAST SURGICAL HISTORY:
- Dental extraction, age 20
- Incision and drainage of right thigh abscess, age 40

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Insulin glargine 30 units SC at bedtime
- Lisinopril 10 mg PO daily
- Carvedilol 6.25 mg PO BID
- Furosemide 40 mg PO daily
- Aspirin 81 mg PO daily
- Naproxen 250 mg PO BID as needed for back pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, diagnosed age 50
- Mother: Essential hypertension
- Brother: Ankylosing spondylitis
- Paternal grandfather: Abdominal aortic aneurysm

SOCIAL HISTORY:
Intravenous drug use

VITAL SIGNS:
Blood pressure is 145/55 mmHg, heart rate is 110 bpm, respiratory rate is 22 breaths/min, and oxygen saturation is 96% on room air

PHYSICAL EXAMINATION:
Physical examination reveals a prominent "water-hammer" pulse, a blowing holosystolic murmur best heard at the left lower sternal border, and bilateral lower extremity pitting edema. Lung auscultation is notable for bibasilar crackles.

LABORATORY DATA:
Hemoglobin: 13.5 g/dL
White blood cell count: 7,000/mm^3
Platelets: 250,000/mm^3
Serum creatinine: 1.0 mg/dL
B-type natriuretic peptide (BNP): 550 pg/mL (normal <100 pg/mL)

IMAGING:
Chest X-ray: Cardiomegaly with cephalization of blood flow and Kerley B lines

An echocardiogram demonstrates a dilated right ventricle, normal left ventricular ejection fraction, and a large right-to-left shunt via a tricuspid regurgitant jet.
