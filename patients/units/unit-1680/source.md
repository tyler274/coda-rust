# synthetic_hospital patient 1680

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, African American, Private insurance (PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-04-18 (Mon) | outpatient | Rheumatology |
| 1 | 2020-07-15 | 2022-10-24 (Mon) | outpatient | Nephrology |
| 2 | 2021-03-15 | 2023-06-20 (Tue) | ed | Emergency Medicine |
| 3 | 2021-09-15 | 2023-12-16 (Sat) | ed | Emergency Medicine |
| 4 | 2022-03-15 | 2024-06-23 (Sun) | ed | Emergency Medicine |
| 5 | 2022-07-15 | 2024-10-26 (Sat) | ed | Emergency Medicine |
| 6 | 2023-01-15 | 2025-04-27 (Sun) | inpatient | Nephrology |
| 7 | 2023-07-15 | 2025-10-23 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private insurance (PPO)",
 "occupation": "Administrative assistant",
 "alcohol_use": "Occasional (less than 1 drink per week)",
 "family_history": [
  "Mother with systemic lupus erythematosus",
  "Maternal aunt with rheumatoid arthritis",
  "Paternal grandmother with hypothyroidism (Hashimoto's)"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "200 mg oral twice daily",
   "name": "Hydroxychloroquine"
  },
  {
   "dose": "5 mg oral daily (patient reports inconsistent adherence)",
   "name": "Prednisone"
  },
  {
   "dose": "2000 units oral daily",
   "name": "Cholecalciferol (Vitamin D3)"
  },
  {
   "dose": "600 mg oral twice daily",
   "name": "Calcium carbonate"
  }
 ],
 "surgical_history": [
  "Dilation and curettage (spontaneous abortion at 10 weeks gestation)",
  "Wisdom teeth extraction"
 ],
 "chronic_conditions": [
  "Systemic lupus erythematosus",
  "Lupus nephritis",
  "Photosensitivity",
  "History of antiphospholipid antibody syndrome (suspected)"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Senile purpura", "Lupus nephritis flare", "Autoimmune pericarditis", "Dehydration", "Systemic lupus erythematosus", "Acute kidney injury"]
Comorbidities: ["Acute kidney injury", "Anemia", "Sinus tachycardia secondary to burns", "Essential hypertension", "Thrombocytopenia", "Systemic lupus erythematosus"]

## Encounter 0: source 2020-01-15, practice 2022-04-18, outpatient, Rheumatology

Chief complaint: 3-month history of fatigue, joint pain, and facial rash that worsens with sun exposure

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of fatigue, joint pain, and facial rash that worsens with sun exposure

HISTORY OF PRESENT ILLNESS:
She reports that the rash worsens with sun exposure. She also mentions occasional oral ulcers and has noticed her hair thinning.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus
- Lupus nephritis
- Photosensitivity
- History of antiphospholipid antibody syndrome (suspected)

Her past medical history is significant for a spontaneous abortion at 10 weeks gestation one year ago.

PAST SURGICAL HISTORY:
- Dilation and curettage (spontaneous abortion at 10 weeks gestation)
- Wisdom teeth extraction

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg oral twice daily
- Prednisone 5 mg oral daily (patient reports inconsistent adherence)
- Cholecalciferol (Vitamin D3) 2000 units oral daily
- Calcium carbonate 600 mg oral twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with systemic lupus erythematosus
- Maternal aunt with rheumatoid arthritis
- Paternal grandmother with hypothyroidism (Hashimoto's)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (less than 1 drink per week). Occupation: Administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, she has a malar rash and mild synovitis in her wrists and knees. No other abnormalities are noted.

LABORATORY DATA:
Laboratory tests reveal the following:

- Hemoglobin: 11.2 g/dL (Normal: 12-16 g/dL)
- Leukocytes: 3,800/uL (Normal: 4,500-11,000/uL)
- Platelets: 120,000/uL (Normal: 150,000-450,000/uL)
- Creatinine: 0.9 mg/dL (Normal: 0.6-1.1 mg/dL)
- Urinalysis: Normal
- Antinuclear antibody (ANA) titer: 1:640 (Normal: <1:40)

## Encounter 1: source 2020-07-15, practice 2022-10-24, outpatient, Nephrology

Chief complaint: Progressive fatigue, facial rash worsened by sun exposure, and symmetric joint pain involving wrists and knees over the past 3 months

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Amara Okafor, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue, facial rash worsened by sun exposure, and symmetric joint pain involving wrists and knees over the past 3 months

HISTORY OF PRESENT ILLNESS:
The patient is a 28-year-old female with systemic lupus erythematosus initially diagnosed in January 2020, currently managed for lupus nephritis, who returns to the clinic with progressive fatigue, facial rash worsened by sun exposure, and symmetric joint pain involving her wrists and knees over the past 3 months. She also reports swelling in her lower legs and notes foamy urine for the last 2 weeks. She denies any fever or weight loss but reports intermittent oral ulcers.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Lupus nephritis
- Photosensitivity
- History of antiphospholipid antibody syndrome (suspected)

PAST SURGICAL HISTORY:
- Dilation and curettage (spontaneous abortion at 10 weeks gestation)
- Wisdom teeth extraction

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg oral twice daily
- Prednisone 5 mg oral daily (patient reports inconsistent adherence)
- Cholecalciferol (Vitamin D3) 2000 units oral daily
- Calcium carbonate 600 mg oral twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with systemic lupus erythematosus
- Maternal aunt with rheumatoid arthritis
- Paternal grandmother with hypothyroidism (Hashimoto's)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (less than 1 drink per week). Occupation: Administrative assistant.

VITAL SIGNS:
Her blood pressure is 145/90 mmHg, and she has no signs of heart failure or pulmonary edema.

PHYSICAL EXAMINATION:
On physical exam, she has a malar rash sparing the nasolabial folds, mild synovitis in the small joints of the hands, and 2+ pitting edema in both lower extremities.

LABORATORY DATA:
Laboratory studies reveal hemoglobin of 10.5 g/dL, creatinine of 2.1 mg/dL (baseline 0.9 mg/dL), and urinalysis with 3+ proteinuria and numerous red blood cell casts. Twenty-four-hour urine collection shows 3.5 g of proteinuria. Complement levels C3 and C4 are decreased, and anti-double-stranded DNA antibodies are elevated. ANA is positive.

## Encounter 2: source 2021-03-15, practice 2023-06-20, ed, Emergency Medicine

Chief complaint: 3-week history of fatigue, joint pain, and swelling in legs with 5 kg unintentional weight gain

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
3-week history of fatigue, joint pain, and swelling in legs with 5 kg unintentional weight gain

HISTORY OF PRESENT ILLNESS:
The patient with systemic lupus erythematosus and prior lupus nephritis flare presents to the emergency department with a 3-week history of fatigue, joint pain, and swelling in her legs. She reports a recent unintentional weight gain of 5 kg.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Lupus nephritis flare (diagnosed 2020-07-15)
- Lupus nephritis
- Photosensitivity
- History of antiphospholipid antibody syndrome (suspected)

Her medical history is significant for systemic lupus erythematosus (SLE),.

PAST SURGICAL HISTORY:
- Dilation and curettage (spontaneous abortion at 10 weeks gestation)
- Wisdom teeth extraction

MEDICATIONS:
She takes for which she takes hydroxychloroquine.

Home Medications:
- Prednisone 5 mg oral daily (patient reports inconsistent adherence)
- Cholecalciferol (Vitamin D3) 2000 units oral daily
- Calcium carbonate 600 mg oral twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with systemic lupus erythematosus
- Maternal aunt with rheumatoid arthritis
- Paternal grandmother with hypothyroidism (Hashimoto's)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (less than 1 drink per week). Occupation: Administrative assistant.

VITAL SIGNS:
On physical examination, her blood pressure is 145/90 mmHg.

PHYSICAL EXAMINATION:
There is pitting edema up to her mid-shins bilaterally.

LABORATORY DATA:
Urinalysis reveals 3+ proteinuria and red blood cell casts. Laboratory results are as follows:

- Serum creatinine: 1.8 mg/dL (baseline 0.9 mg/dL)
- Serum albumin: 2.9 g/dL
- Anti-double-stranded DNA antibody titer: Elevated
- Complement levels (C3, C4): Decreased
- Erythrocyte sedimentation rate (ESR): Elevated

## Encounter 3: source 2021-09-15, practice 2023-12-16, ed, Emergency Medicine

Chief complaint: Sharp, left-sided chest pain that worsens with deep breaths and lying flat, with low-grade fever

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Priya Sharma, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sharp, left-sided chest pain that worsens with deep breaths and lying flat, with low-grade fever

HISTORY OF PRESENT ILLNESS:
This 29-year-old female with systemic lupus erythematosus and lupus nephritis, previously evaluated in March 2021 for a nephritis flare, presents to the emergency department with a 2-day history of sharp, left-sided chest pain that worsens with deep breaths and lying flat. She also reports a low-grade fever and increasing fatigue over the past week. She has been non-compliant with her hydroxychloroquine and prednisone for the past month.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Lupus nephritis flare (diagnosed 2020-07-15)
- Lupus nephritis
- Photosensitivity
- History of antiphospholipid antibody syndrome (suspected)

PAST SURGICAL HISTORY:
- Dilation and curettage (spontaneous abortion at 10 weeks gestation)
- Wisdom teeth extraction

MEDICATIONS:
She takes hydroxychloroquine and prednisone.

Home Medications:
- Cholecalciferol (Vitamin D3) 2000 units oral daily
- Calcium carbonate 600 mg oral twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with systemic lupus erythematosus
- Maternal aunt with rheumatoid arthritis
- Paternal grandmother with hypothyroidism (Hashimoto's)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (less than 1 drink per week). Occupation: Administrative assistant.

VITAL SIGNS:
Temperature of 38.2°C, a heart rate of 102 beats per minute, and a blood pressure of 125/75 mmHg

PHYSICAL EXAMINATION:
A friction rub is heard at the left lower sternal border. Her joints show no signs of active synovitis, and there is no rash present.

LABORATORY DATA:
- Hemoglobin: 10.2 g/dL (Normal: 12-16 g/dL)
- Leukocytes: 3,000/uL (Normal: 4,500-11,000/uL)
- Platelets: 100,000/uL (Normal: 150,000-450,000/uL)
- Erythrocyte sedimentation rate (ESR): 70 mm/hr (Normal: 0-20 mm/hr)
- C-reactive protein (CRP): 25 mg/L (Normal: <10 mg/L)
- Antinuclear antibodies (ANA): Positive
- Anti-double-stranded DNA antibodies: Positive
- Complement levels (C3, C4): Low

OTHER STUDIES:
An ECG shows diffuse ST-segment elevations and PR depressions.

## Encounter 4: source 2022-03-15, practice 2024-06-23, ed, Emergency Medicine

Chief complaint: One-week history of progressive fatigue, generalized weakness, intermittent headaches, and dark urine for two days

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
One-week history of progressive fatigue, generalized weakness, intermittent headaches, and dark urine for two days

HISTORY OF PRESENT ILLNESS:
The patient is a 30-year-old female with systemic lupus erythematosus complicated by lupus nephritis and a history of autoimmune pericarditis, who presents to the emergency department with a one-week history of progressive fatigue, generalized weakness, and intermittent headaches. She also reports dark urine for the past two days. She denies any recent infections or medication changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Lupus nephritis flare (diagnosed 2020-07-15)
- Autoimmune pericarditis (diagnosed 2021-09-15)
- Lupus nephritis
- Photosensitivity
- History of antiphospholipid antibody syndrome (suspected)

Her past medical history is significant for systemic lupus erythematosus.

PAST SURGICAL HISTORY:
- Dilation and curettage (spontaneous abortion at 10 weeks gestation)
- Wisdom teeth extraction

MEDICATIONS:
She takes for which she takes hydroxychloroquine.

Home Medications:
- Prednisone 5 mg oral daily (patient reports inconsistent adherence)
- Cholecalciferol (Vitamin D3) 2000 units oral daily
- Calcium carbonate 600 mg oral twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with systemic lupus erythematosus
- Maternal aunt with rheumatoid arthritis
- Paternal grandmother with hypothyroidism (Hashimoto's)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (less than 1 drink per week). Occupation: Administrative assistant.

VITAL SIGNS:
On physical examination, she is afebrile, blood pressure is 150/90 mmHg, heart rate is 110 beats per minute

PHYSICAL EXAMINATION:
She appears jaundiced. Neurologic examination reveals mild confusion and difficulty concentrating

LABORATORY DATA:
Laboratory studies are as follows:
- Hemoglobin: 7 g/dL (Normal: 12-16 g/dL)
- Platelet count: 20,000/µL (Normal: 150,000-400,000/µL)
- Lactate dehydrogenase (LDH): 600 U/L (Normal: 140-280 U/L)
- Haptoglobin: <10 mg/dL (Normal: 30-200 mg/dL)
- Creatinine: 2.5 mg/dL (Normal: 0.6-1.2 mg/dL)
- Peripheral blood smear: Numerous schistocytes

## Encounter 5: source 2022-07-15, practice 2024-10-26, ed, Emergency Medicine

Chief complaint: 2-day history of nausea, vomiting, and diarrhea with inability to tolerate oral intake

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Maria Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-day history of nausea, vomiting, and diarrhea with inability to tolerate oral intake

HISTORY OF PRESENT ILLNESS:
The patient is a 30-year-old female with systemic lupus erythematosus complicated by lupus nephritis and autoimmune pericarditis, who presents to the emergency department with a 2-day history of nausea, vomiting, and diarrhea. She reports that she has been unable to keep any food or liquids down. She denies any recent travel or sick contacts.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Lupus nephritis flare (diagnosed 2020-07-15)
- Autoimmune pericarditis (diagnosed 2021-09-15)
- Senile purpura (diagnosed 2022-03-15)
- Lupus nephritis
- Photosensitivity
- History of antiphospholipid antibody syndrome (suspected)

PAST SURGICAL HISTORY:
- Dilation and curettage (spontaneous abortion at 10 weeks gestation)
- Wisdom teeth extraction

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg oral twice daily
- Prednisone 5 mg oral daily (patient reports inconsistent adherence)
- Cholecalciferol (Vitamin D3) 2000 units oral daily
- Calcium carbonate 600 mg oral twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with systemic lupus erythematosus
- Maternal aunt with rheumatoid arthritis
- Paternal grandmother with hypothyroidism (Hashimoto's)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (less than 1 drink per week). Occupation: Administrative assistant.

VITAL SIGNS:
Blood pressure is 90/60 mmHg, heart rate is 110 beats per minute

PHYSICAL EXAMINATION:
On examination, she appears lethargic. Her mucous membranes are dry, and her skin turgor is decreased.

LABORATORY DATA:
Laboratory tests reveal:

- Sodium: 135 mEq/L
- Potassium: 3.2 mEq/L
- Chloride: 98 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen (BUN): 28 mg/dL
- Creatinine: 1.2 mg/dL
- Urine specific gravity: 1.030

## Encounter 6: source 2023-01-15, practice 2025-04-27, inpatient, Nephrology

Chief complaint: 3-day history of severe vomiting and diarrhea with inability to keep down fluids

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Amara Okafor, MD
Department: Nephrology
Visit Type: inpatient

CHIEF COMPLAINT:
3-day history of severe vomiting and diarrhea with inability to keep down fluids

HISTORY OF PRESENT ILLNESS:
The patient with systemic lupus erythematosus and lupus nephritis presents to the emergency department with recurrent gastrointestinal symptoms as in July 2022. She reports a 3-day history of severe vomiting and diarrhea, inability to keep down any fluids, and taking ibuprofen every 4 hours for abdominal pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Lupus nephritis flare (diagnosed 2020-07-15)
- Autoimmune pericarditis (diagnosed 2021-09-15)
- Senile purpura (diagnosed 2022-03-15)
- Dehydration (diagnosed 2022-07-15)
- Lupus nephritis
- Photosensitivity
- History of antiphospholipid antibody syndrome (suspected)

PAST SURGICAL HISTORY:
- Dilation and curettage (spontaneous abortion at 10 weeks gestation)
- Wisdom teeth extraction

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg oral twice daily
- Prednisone 5 mg oral daily (patient reports inconsistent adherence)
- Cholecalciferol (Vitamin D3) 2000 units oral daily
- Calcium carbonate 600 mg oral twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with systemic lupus erythematosus
- Maternal aunt with rheumatoid arthritis
- Paternal grandmother with hypothyroidism (Hashimoto's)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (less than 1 drink per week). Occupation: Administrative assistant.

VITAL SIGNS:
Her blood pressure is 90/60 mmHg, heart rate is 110 beats per minute

PHYSICAL EXAMINATION:
She appears lethargic and her mucous membranes are dry. Her skin turgor is poor.

LABORATORY DATA:
Laboratory tests reveal:

- Serum creatinine: 2.1 mg/dL (baseline 0.9 mg/dL)
- Blood urea nitrogen (BUN): 40 mg/dL
- Serum potassium: 3.5 mEq/L
- Serum bicarbonate: 24 mEq/L
- Urine sodium: 10 mEq/L
- Urine osmolality: 500 mOsm/kg

## Encounter 7: source 2023-07-15, practice 2025-10-23, ed, Emergency Medicine

Chief complaint: Three-day history of generalized weakness, fever, and dark urine with several episodes of epistaxis over past two days

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Fatima Al-Hassan, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Three-day history of generalized weakness, fever, and dark urine with several episodes of epistaxis over past two days

HISTORY OF PRESENT ILLNESS:
The patient with systemic lupus erythematosus and lupus nephritis returns to the emergency department with symptoms similar to March 2022, reporting a three-day history of generalized weakness, fever, and dark urine. She also reports experiencing several episodes of epistaxis over past two days.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Lupus nephritis flare (diagnosed 2020-07-15)
- Autoimmune pericarditis (diagnosed 2021-09-15)
- Senile purpura (diagnosed 2022-03-15)
- Dehydration (diagnosed 2022-07-15)
- Acute kidney injury (diagnosed 2023-01-15)
- Lupus nephritis
- Photosensitivity
- History of antiphospholipid antibody syndrome (suspected)

Her medical history is significant for systemic lupus erythematosus.

PAST SURGICAL HISTORY:
- Dilation and curettage (spontaneous abortion at 10 weeks gestation)
- Wisdom teeth extraction

MEDICATIONS:
She takes for which she takes hydroxychloroquine.

Home Medications:
- Prednisone 5 mg oral daily (patient reports inconsistent adherence)
- Cholecalciferol (Vitamin D3) 2000 units oral daily
- Calcium carbonate 600 mg oral twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with systemic lupus erythematosus
- Maternal aunt with rheumatoid arthritis
- Paternal grandmother with hypothyroidism (Hashimoto's)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (less than 1 drink per week). Occupation: Administrative assistant.

VITAL SIGNS:
On physical examination, she is afebrile, with a blood pressure of 130/85 mmHg, heart rate of 100 beats per minute

PHYSICAL EXAMINATION:
No skin rash is noted

LABORATORY DATA:
Laboratory studies reveal:

- Hemoglobin: 7.8 g/dL (Normal: 12-16 g/dL)
- Platelet count: 20,000/µL (Normal: 150,000-400,000/µL)
- Lactate dehydrogenase (LDH): Elevated
- Bilirubin, total: 2.2 mg/dL (Normal: 0.3-1.2 mg/dL)
- Haptoglobin: <10 mg/dL (Normal: 30-200 mg/dL)
- Peripheral blood smear: Schistocytes present
- Creatinine: 1.2 mg/dL (Normal: 0.6-1.1 mg/dL)
- ADAMTS13 activity: Pending

The patient's urine is positive for hemoglobin, but there are no red blood cells on microscopy.
