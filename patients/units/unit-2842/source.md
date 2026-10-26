# synthetic_hospital patient 2842

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 65 at first encounter, sex F, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-05-05 (Fri) | outpatient | Orthopedics |
| 1 | 2020-04-15 | 2023-07-25 (Tue) | outpatient | Ophthalmology |
| 2 | 2020-07-15 | 2023-10-23 (Mon) | ed | Emergency Medicine |
| 3 | 2021-01-15 | 2024-04-18 (Thu) | ed | Emergency Medicine |
| 4 | 2021-07-15 | 2024-11-02 (Sat) | ed | Emergency Medicine |
| 5 | 2022-01-15 | 2025-05-02 (Fri) | outpatient | Pulmonary |
| 6 | 2022-07-15 | 2025-10-28 (Tue) | ed | Emergency Medicine |
| 7 | 2023-01-15 | 2026-04-29 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired librarian",
 "alcohol_use": "Occasional social use, 1-2 glasses of wine weekly",
 "family_history": [
  "Father: Lung cancer (diagnosed age 68, deceased)",
  "Mother: Age-related macular degeneration",
  "Sister: Hypertension and osteoarthritis",
  "Paternal uncle: Myocardial infarction at age 55"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 30 pack-years, quit 12 years ago",
 "home_medications": [
  {
   "dose": "5 mg daily (dose adjusted per INR)",
   "name": "Warfarin"
  },
  {
   "dose": "50 mg twice daily",
   "name": "Metoprolol tartrate"
  },
  {
   "dose": "10 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "20 mg daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "500 mg every 6 hours as needed for pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Right total knee arthroplasty (post-operative week 1 at presentation)",
  "Hysterectomy (age 52)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Severe osteoarthritis (bilateral lower extremity involvement)",
  "Hyperlipidemia",
  "Chronic atrial fibrillation",
  "Age-related macular degeneration (dry/non-exudative)",
  "Calcium pyrophosphate deposition disease (CPPD)",
  "History of recurrent venous thromboembolism"
 ],
 "age_at_first_encounter": 65
}
```

Primary diagnoses: ["Pulmonary embolism", "Stage IIIB Non-Small Cell Lung Cancer", "Pseudogout (Calcium Pyrophosphate Deposition Disease)", "Age-related macular degeneration, dry (non-exudative)", "Severe hip osteoarthritis"]
Comorbidities: ["Status post hip replacement surgery", "Osteoarthritis", "History of recent pneumonia", "Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2023-05-05, outpatient, Orthopedics

Chief complaint: Chronic pain in right groin and buttock exacerbated by walking and climbing stairs

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Chronic pain in right groin and buttock exacerbated by walking and climbing stairs

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with a 6-month history of chronic pain in her right groin and buttock. She reports that the pain is exacerbated by walking and climbing stairs and is relieved by rest. She denies any recent trauma or injury.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Severe osteoarthritis (bilateral lower extremity involvement)
- Hyperlipidemia
- Chronic atrial fibrillation
- Age-related macular degeneration (dry/non-exudative)
- Calcium pyrophosphate deposition disease (CPPD)
- History of recurrent venous thromboembolism

She has a past medical history of hypertension, which is well-controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Right total knee arthroplasty (post-operative week 1 at presentation)
- Hysterectomy (age 52)

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily (dose adjusted per INR)
- Metoprolol tartrate 50 mg twice daily
- Lisinopril 10 mg daily
- Atorvastatin 20 mg daily
- Acetaminophen 500 mg every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 68, deceased)
- Mother: Age-related macular degeneration
- Sister: Hypertension and osteoarthritis
- Paternal uncle: Myocardial infarction at age 55

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 12 years ago. Alcohol: Occasional social use, 1-2 glasses of wine weekly. Occupation: Retired librarian.

PHYSICAL EXAMINATION:
On physical examination, you note that her right hip has a decreased range of motion, particularly with internal rotation. There is no erythema, warmth, or swelling.

IMAGING:
An X-ray of the hip is performed, which shows:

- Joint space narrowing
- Subchondral sclerosis
- Osteophyte formation at the acetabular rim

## Encounter 1: source 2020-04-15, practice 2023-07-25, outpatient, Ophthalmology

Chief complaint: Difficulty reading and recognizing faces over the past six months with central vision changes

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Robert Martinez, MD
Department: Ophthalmology
Visit Type: outpatient

CHIEF COMPLAINT:
Difficulty reading and recognizing faces over the past six months with central vision changes

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of her known macular degeneration three months after orthopedic evaluation for hip osteoarthritis. She reports that her vision problems are affecting both eyes and are more pronounced in the center of her visual field. She denies any pain or redness in her eyes.

PAST MEDICAL HISTORY:
Active Problem List:
- Severe hip osteoarthritis (diagnosed 2020-01-15)
- Hypertension
- Severe osteoarthritis (bilateral lower extremity involvement)
- Hyperlipidemia
- Chronic atrial fibrillation
- Age-related macular degeneration (dry/non-exudative)
- Calcium pyrophosphate deposition disease (CPPD)
- History of recurrent venous thromboembolism

Her past medical history is significant for hypertension, controlled with lisinopril, and hypercholesterolemia, managed with atorvastatin.

PAST SURGICAL HISTORY:
- Right total knee arthroplasty (post-operative week 1 at presentation)
- Hysterectomy (age 52)

MEDICATIONS:
She takes lisinopril, atorvastatin.

Home Medications:
- Warfarin 5 mg daily (dose adjusted per INR)
- Metoprolol tartrate 50 mg twice daily
- Acetaminophen 500 mg every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 68, deceased)
- Mother: Age-related macular degeneration
- Sister: Hypertension and osteoarthritis
- Paternal uncle: Myocardial infarction at age 55

SOCIAL HISTORY:
She has a 30-pack-year smoking history but quit 10 years ago.

PHYSICAL EXAMINATION:
On physical examination, her visual acuity is 20/40 in both eyes. Amsler grid testing reveals metamorphopsia centrally in both eyes. Fundoscopic examination shows bilateral drusen and pigmentary changes in the macula. There are no signs of hemorrhage or exudate. The peripheral retina appears normal.

## Encounter 2: source 2020-07-15, practice 2023-10-23, ed, Emergency Medicine

Chief complaint: Sudden-onset shortness of breath and confusion on postoperative day 4 following right total knee replacement

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Emily Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset shortness of breath and confusion on postoperative day 4 following right total knee replacement

HISTORY OF PRESENT ILLNESS:
This 65-year-old female with chronic atrial fibrillation on anticoagulation and severe osteoarthritis, previously evaluated for hip arthritis and macular degeneration, is brought to the emergency department after developing sudden-onset shortness of breath and confusion on postoperative day 4 following a right total knee replacement. She has been recovering with limited ambulation.

PAST MEDICAL HISTORY:
Active Problem List:
- Severe hip osteoarthritis (diagnosed 2020-01-15)
- Age-related macular degeneration, dry (non-exudative) (diagnosed 2020-04-15)
- Hypertension
- Severe osteoarthritis (bilateral lower extremity involvement)
- Hyperlipidemia
- Chronic atrial fibrillation
- Calcium pyrophosphate deposition disease (CPPD)
- History of recurrent venous thromboembolism

Her medical history includes hypertension and osteoarthritis.

PAST SURGICAL HISTORY:
- Right total knee arthroplasty (post-operative week 1 at presentation)
- Hysterectomy (age 52)

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily (dose adjusted per INR)
- Metoprolol tartrate 50 mg twice daily
- Lisinopril 10 mg daily
- Atorvastatin 20 mg daily
- Acetaminophen 500 mg every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 68, deceased)
- Mother: Age-related macular degeneration
- Sister: Hypertension and osteoarthritis
- Paternal uncle: Myocardial infarction at age 55

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 12 years ago. Alcohol: Occasional social use, 1-2 glasses of wine weekly. Occupation: Retired librarian.

VITAL SIGNS:
Her temperature is 37.0°C (98.6°F), blood pressure is 90/58 mm Hg, pulse is 122/min, and respiratory rate is 30/min.

PHYSICAL EXAMINATION:
She appears anxious and diaphoretic. Jugular venous distension is noted. Her lungs are clear to auscultation, and heart sounds are normal. There is mild swelling and tenderness of the right calf.

LABORATORY DATA:
Arterial blood gas on room air shows: pH 7.47, PaCO₂ 27 mm Hg, PaO₂ 60 mm Hg.

IMAGING:
Chest x-ray is unremarkable.

OTHER STUDIES:
ECG reveals sinus tachycardia.

## Encounter 3: source 2021-01-15, practice 2024-04-18, ed, Emergency Medicine

Chief complaint: Severe pain, redness, and swelling in right knee for 2 days without trauma

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe pain, redness, and swelling in right knee for 2 days without trauma

HISTORY OF PRESENT ILLNESS:
The patient with calcium pyrophosphate deposition disease and prior right knee replacement presents with acute symptoms. She reports that the pain began suddenly and has progressively worsened. She denies any recent trauma but mentions she was hospitalized 3 weeks ago for a urinary tract infection and treated with antibiotics.

PAST MEDICAL HISTORY:
Active Problem List:
- Severe hip osteoarthritis (diagnosed 2020-01-15)
- Age-related macular degeneration, dry (non-exudative) (diagnosed 2020-04-15)
- Pulmonary embolism (diagnosed 2020-07-15)
- Hypertension
- Severe osteoarthritis (bilateral lower extremity involvement)
- Hyperlipidemia
- Chronic atrial fibrillation
- Calcium pyrophosphate deposition disease (CPPD)
- History of recurrent venous thromboembolism

PAST SURGICAL HISTORY:
- Right total knee arthroplasty (post-operative week 1 at presentation)
- Hysterectomy (age 52)

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily (dose adjusted per INR)
- Metoprolol tartrate 50 mg twice daily
- Lisinopril 10 mg daily
- Atorvastatin 20 mg daily
- Acetaminophen 500 mg every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 68, deceased)
- Mother: Age-related macular degeneration
- Sister: Hypertension and osteoarthritis
- Paternal uncle: Myocardial infarction at age 55

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 12 years ago. Alcohol: Occasional social use, 1-2 glasses of wine weekly. Occupation: Retired librarian.

PHYSICAL EXAMINATION:
On physical examination, her right knee is warm, erythematous, and swollen with decreased range of motion due to pain. Aspiration of the knee joint yields synovial fluid that is yellow and cloudy.

LABORATORY DATA:
- White blood cell count (WBC): 12,000 /µL (normal: 4,500-11,000 /µL)
- Erythrocyte sedimentation rate (ESR): 50 mm/hr (normal: 0-20 mm/hr)
- C-reactive protein (CRP): 25 mg/L (normal: <10 mg/L)
- Synovial fluid analysis:
- WBC: 18,000 cells/µL (normal: <200 cells/µL)
- Polymorphonuclear cells: 75%
- Negative for organisms on Gram stain
- Crystals: Rhomboid-shaped, positively birefringent

IMAGING:
Radiographs of the right knee show chondrocalcinosis.

## Encounter 4: source 2021-07-15, practice 2024-11-02, ed, Emergency Medicine

Chief complaint: Sudden onset of sharp, right-sided chest pain worsening with inspiration and shortness of breath

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of sharp, right-sided chest pain worsening with inspiration and shortness of breath

HISTORY OF PRESENT ILLNESS:
The patient with recurrent VTE and AFib presents with acute chest pain. She returned from a transatlantic flight 3 days prior, which lasted 8 hours, and has been mostly bedridden since due to a mild flu.

PAST MEDICAL HISTORY:
Active Problem List:
- Severe hip osteoarthritis (diagnosed 2020-01-15)
- Age-related macular degeneration, dry (non-exudative) (diagnosed 2020-04-15)
- Pulmonary embolism (diagnosed 2020-07-15)
- Pseudogout (Calcium Pyrophosphate Deposition Disease) (diagnosed 2021-01-15)
- Hypertension
- Severe osteoarthritis (bilateral lower extremity involvement)
- Hyperlipidemia
- Chronic atrial fibrillation
- Calcium pyrophosphate deposition disease (CPPD)
- History of recurrent venous thromboembolism

Her medical history includes hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Right total knee arthroplasty (post-operative week 1 at presentation)
- Hysterectomy (age 52)

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily (dose adjusted per INR)
- Metoprolol tartrate 50 mg twice daily
- Lisinopril 10 mg daily
- Atorvastatin 20 mg daily
- Acetaminophen 500 mg every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 68, deceased)
- Mother: Age-related macular degeneration
- Sister: Hypertension and osteoarthritis
- Paternal uncle: Myocardial infarction at age 55

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 12 years ago. Alcohol: Occasional social use, 1-2 glasses of wine weekly. Occupation: Retired librarian.

VITAL SIGNS:
Tachypneic with a respiratory rate of 24 breaths/min, and tachycardic with a heart rate of 110 beats/min. Her blood pressure is 135/85 mmHg. Oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
On examination, she is anxious. A lower extremity examination reveals mild right calf swelling with tenderness to palpation. No skin changes or palpable cords are noted. Lung auscultation is clear, and there is no jugular venous distension.

LABORATORY DATA:
D-dimer level is elevated.

Laboratory Values:
- D-dimer: 0.8 mg/L (normal <0.5 mg/L)

IMAGING:
Chest X-ray is unremarkable.

OTHER STUDIES:
An electrocardiogram shows sinus tachycardia with no ST changes.

## Encounter 5: source 2022-01-15, practice 2025-05-02, outpatient, Pulmonary

Chief complaint: Progressive fatigue, unintentional 8-kg weight loss over 3 months, and dry cough

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Michael Chang, MD
Department: Pulmonary
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue, unintentional 8-kg weight loss over 3 months, and dry cough

HISTORY OF PRESENT ILLNESS:
The patient, with chronic atrial fibrillation on warfarin and prior pulmonary emboli in 2020 and 2021, reports increasing shortness of breath with activity, occasional dry cough, and vague right-sided chest discomfort. She denies fever, night sweats, or hemoptysis.

PAST MEDICAL HISTORY:
Active Problem List:
- Severe hip osteoarthritis (diagnosed 2020-01-15)
- Age-related macular degeneration, dry (non-exudative) (diagnosed 2020-04-15)
- Pulmonary embolism (diagnosed 2020-07-15)
- Pseudogout (Calcium Pyrophosphate Deposition Disease) (diagnosed 2021-01-15)
- Hypertension
- Severe osteoarthritis (bilateral lower extremity involvement)
- Hyperlipidemia
- Chronic atrial fibrillation
- Calcium pyrophosphate deposition disease (CPPD)
- History of recurrent venous thromboembolism

Her medical history is notable for hypertension and osteoarthritis.

PAST SURGICAL HISTORY:
- Right total knee arthroplasty (post-operative week 1 at presentation)
- Hysterectomy (age 52)

MEDICATIONS:
Her medications include lisinopril and acetaminophen.

Home Medications:
- Warfarin 5 mg daily (dose adjusted per INR)
- Metoprolol tartrate 50 mg twice daily
- Atorvastatin 20 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 68, deceased)
- Mother: Age-related macular degeneration
- Sister: Hypertension and osteoarthritis
- Paternal uncle: Myocardial infarction at age 55

SOCIAL HISTORY:
She quit smoking 12 years ago after a 30-pack-year history. She lives alone and retired from her job as a librarian last year. She has no known occupational exposures.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, she appears thin but alert. There is digital clubbing of the fingers and decreased breath sounds at the right lung base. No cervical or supraclavicular lymphadenopathy is noted.

LABORATORY DATA:
Laboratory studies are significant for mild anemia (Hb 10.8 g/dL) and slightly elevated alkaline phosphatase.

IMAGING:
A chest x-ray reveals a solitary 3.5-cm peripheral opacity in the right lower lobe with associated pleural retraction. CT scan confirms the mass and shows no hilar lymphadenopathy or pleural effusion.

PATHOLOGY:
CT-guided biopsy is performed, and histology reveals malignant gland-forming cells with mucin production.

## Encounter 6: source 2022-07-15, practice 2025-10-28, ed, Emergency Medicine

Chief complaint: Sudden onset of shortness of breath and sharp right-sided pleuritic chest pain

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of shortness of breath and sharp right-sided pleuritic chest pain

HISTORY OF PRESENT ILLNESS:
The patient with recurrent venous thromboembolism and Stage IIIB non-small cell lung cancer presents with acute symptoms. She reports that she was recovering well until this morning when she suddenly felt a sharp pain on the right side of her chest, which worsens with deep breaths. She denies any cough or fever.

PAST MEDICAL HISTORY:
Active Problem List:
- Severe hip osteoarthritis (diagnosed 2020-01-15)
- Age-related macular degeneration, dry (non-exudative) (diagnosed 2020-04-15)
- Pulmonary embolism (diagnosed 2020-07-15)
- Pseudogout (Calcium Pyrophosphate Deposition Disease) (diagnosed 2021-01-15)
- Stage IIIB Non-Small Cell Lung Cancer (diagnosed 2022-01-15)
- Hypertension
- Severe osteoarthritis (bilateral lower extremity involvement)
- Hyperlipidemia
- Chronic atrial fibrillation
- Calcium pyrophosphate deposition disease (CPPD)
- History of recurrent venous thromboembolism

PAST SURGICAL HISTORY:
Recent total hip replacement surgery 2 weeks ago

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily (dose adjusted per INR)
- Metoprolol tartrate 50 mg twice daily
- Lisinopril 10 mg daily
- Atorvastatin 20 mg daily
- Acetaminophen 500 mg every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 68, deceased)
- Mother: Age-related macular degeneration
- Sister: Hypertension and osteoarthritis
- Paternal uncle: Myocardial infarction at age 55

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 12 years ago. Alcohol: Occasional social use, 1-2 glasses of wine weekly. Occupation: Retired librarian.

VITAL SIGNS:
Tachypneic with a respiratory rate of 28 breaths per minute, her heart rate is 110 beats per minute, blood pressure is 145/90 mmHg, and oxygen saturation is 89% on room air

PHYSICAL EXAMINATION:
Her jugular venous pressure is elevated, and lung auscultation reveals decreased breath sounds at the right lung base without wheezes or crackles. Lower extremity examination shows mild right calf swelling with tenderness to palpation.

LABORATORY DATA:
D-dimer level is elevated. Laboratory Values: - D-dimer: 0.8 mg/L (normal <0.5 mg/L)

IMAGING:
Chest X-ray is unremarkable

OTHER STUDIES:
Electrocardiogram (ECG) shows sinus tachycardia with no ST-segment or T-wave changes

## Encounter 7: source 2023-01-15, practice 2026-04-29, ed, Emergency Medicine

Chief complaint: Acute onset of shortness of breath and pleuritic chest pain beginning 2 hours ago

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Linda Washington, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of shortness of breath and pleuritic chest pain beginning 2 hours ago

HISTORY OF PRESENT ILLNESS:
This 68-year-old woman with recurrent venous thromboembolism on warfarin presents to the emergency department with sudden onset of shortness of breath and pleuritic chest pain that started 2 hours ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Severe hip osteoarthritis (diagnosed 2020-01-15)
- Age-related macular degeneration, dry (non-exudative) (diagnosed 2020-04-15)
- Pulmonary embolism (diagnosed 2020-07-15)
- Pseudogout (Calcium Pyrophosphate Deposition Disease) (diagnosed 2021-01-15)
- Stage IIIB Non-Small Cell Lung Cancer (diagnosed 2022-01-15)
- Hypertension
- Severe osteoarthritis (bilateral lower extremity involvement)
- Hyperlipidemia
- Chronic atrial fibrillation
- Calcium pyrophosphate deposition disease (CPPD)
- History of recurrent venous thromboembolism

She has a past medical history of hypertension and chronic atrial fibrillation.

PAST SURGICAL HISTORY:
She recently underwent a total hip replacement 4 days ago.

MEDICATIONS:
She takes metoprolol and warfarin.

Home Medications:
- Metoprolol tartrate 50 mg twice daily
- Lisinopril 10 mg daily
- Atorvastatin 20 mg daily
- Acetaminophen 500 mg every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 68, deceased)
- Mother: Age-related macular degeneration
- Sister: Hypertension and osteoarthritis
- Paternal uncle: Myocardial infarction at age 55

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 12 years ago. Alcohol: Occasional social use, 1-2 glasses of wine weekly. Occupation: Retired librarian.

VITAL SIGNS:
Respiratory rate of 28 breaths per minute, heart rate is 110 beats per minute, blood pressure is 135/85 mmHg, and oxygen saturation is 89% on room air

PHYSICAL EXAMINATION:
On examination, she is tachypneic with a respiratory rate of 28 breaths per minute, heart rate is 110 beats per minute, blood pressure is 135/85 mmHg, and oxygen saturation is 89% on room air. Her jugular venous pressure is not elevated. Lung auscultation reveals decreased breath sounds at the left lung base without wheezes or crackles. The left lower extremity is swollen and tender to palpation.

LABORATORY DATA:
A D-dimer test is markedly elevated. An arterial blood gas shows hypoxemia with a PaO2 of 60 mmHg.

Laboratory Values:
- D-dimer: Elevated
- Arterial blood gas:
- PaO2: 60 mmHg
- PaCO2: 32 mmHg
- pH: 7.48
- HCO3-: 24 mEq/L

IMAGING:
A CT pulmonary angiogram is performed and reveals a large embolus in the left pulmonary artery.
