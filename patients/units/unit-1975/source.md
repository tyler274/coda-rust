# synthetic_hospital patient 1975

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 52 at first encounter, sex M, Non-Hispanic White, Medicare (disability) and Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-11-08 (Mon) | outpatient | Ophthalmology |
| 1 | 2020-07-15 | 2022-05-19 (Thu) | outpatient | Internal Medicine |
| 2 | 2021-01-15 | 2022-11-09 (Wed) | outpatient | Gastroenterology |
| 3 | 2021-07-15 | 2023-05-15 (Mon) | outpatient | Gastroenterology |
| 4 | 2022-01-15 | 2023-11-13 (Mon) | outpatient | Orthopedics |
| 5 | 2022-07-15 | 2024-05-14 (Tue) | outpatient | Nephrology |
| 6 | 2023-01-15 | 2024-11-11 (Mon) | outpatient | Orthopedics |
| 7 | 2024-01-15 | 2025-11-11 (Tue) | icu | Surgery |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare (disability) and Medicaid",
 "occupation": "Former construction foreman, currently on medical disability",
 "alcohol_use": "History of heavy alcohol consumption (6-8 beers daily for 15 years), currently sober for 6 months and enrolled in outpatient rehabilitation",
 "family_history": [
  "Type 2 Diabetes Mellitus (father, paternal grandfather)",
  "Hypertension (mother)",
  "Chronic kidney disease (maternal uncle)",
  "Alcohol use disorder (father)",
  "Cataracts (mother)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 20 pack-years, quit 5 years ago",
 "home_medications": [
  {
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "12.5 mg daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "40 mg nightly",
   "name": "Atorvastatin"
  },
  {
   "dose": "325 mg daily",
   "name": "Ferrous sulfate"
  },
  {
   "dose": "100 mg daily",
   "name": "Thiamine"
  },
  {
   "dose": "1 mg daily",
   "name": "Folic acid"
  },
  {
   "dose": "500 mg every 6 hours as needed for pain",
   "name": "Acetaminophen"
  },
  {
   "dose": "400 mg every 8 hours as needed for pain",
   "name": "Ibuprofen"
  }
 ],
 "surgical_history": [
  "Right below-knee amputation (6 months prior)",
  "Emergency debridement and excisional wound preparation for thermal burns",
  "Split-thickness skin grafting to chest and upper extremities"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus",
  "Hypertension",
  "Alcohol Use Disorder (in remission)",
  "Chronic liver disease (mixed alcoholic and metabolic steatohepatitis)",
  "Osteonecrosis of the left femoral head",
  "Bilateral cataracts",
  "Iron deficiency anemia",
  "Chronic kidney disease with nephrotic syndrome",
  "Status post traumatic right below-knee amputation with phantom limb pain syndrome",
  "Metabolic syndrome",
  "Obesity"
 ],
 "age_at_first_encounter": 52
}
```

Primary diagnoses: ["Alcoholic liver disease", "Osteonecrosis of the femoral head", "Burn wound infection with sepsis", "Bilateral cataracts", "Iron deficiency anemia", "Non-alcoholic steatohepatitis", "Phantom limb pain", "Nephrotic syndrome (edema management)"]
Comorbidities: ["Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2021-11-08, outpatient, Ophthalmology

Chief complaint: Progressively worsening vision in both eyes over the past year, with difficulty driving at night and reading small print.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Ophthalmology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressively worsening vision in both eyes over the past year, with difficulty driving at night and reading small print.

HISTORY OF PRESENT ILLNESS:
He reports difficulty with night driving and reading small print, even with his prescription glasses. He denies any eye pain, redness, or history of ocular trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus
- Hypertension
- Alcohol Use Disorder (in remission)
- Chronic liver disease (mixed alcoholic and metabolic steatohepatitis)
- Osteonecrosis of the left femoral head
- Bilateral cataracts
- Iron deficiency anemia
- Chronic kidney disease with nephrotic syndrome
- Status post traumatic right below-knee amputation with phantom limb pain syndrome
- Metabolic syndrome
- Obesity

PAST SURGICAL HISTORY:
- Right below-knee amputation (6 months prior)
- Emergency debridement and excisional wound preparation for thermal burns
- Split-thickness skin grafting to chest and upper extremities

MEDICATIONS:
He takes metformin, lisinopril, and atorvastatin.

Home Medications:
- Hydrochlorothiazide 12.5 mg daily
- Ferrous sulfate 325 mg daily
- Thiamine 100 mg daily
- Folic acid 1 mg daily
- Acetaminophen 500 mg every 6 hours as needed for pain
- Ibuprofen 400 mg every 8 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Type 2 Diabetes Mellitus (father, paternal grandfather)
- Hypertension (mother)
- Chronic kidney disease (maternal uncle)
- Alcohol use disorder (father)
- Cataracts (mother)

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: History of heavy alcohol consumption (6-8 beers daily for 15 years), currently sober for 6 months and enrolled in outpatient rehabilitation. Occupation: Former construction foreman, currently on medical disability.

VITAL SIGNS:
Blood Pressure: 148/92 mmHg

PHYSICAL EXAMINATION:
On physical examination, visual acuity is 20/50 in both eyes. Slit-lamp examination reveals bilateral opacification of the lenses. Fundoscopic examination is unremarkable. No signs of retinopathy or macular degeneration are noted. Intraocular pressures are within normal limits.

LABORATORY DATA:
Laboratory Values:
- Fasting Blood Glucose: 162 mg/dL
- Hemoglobin A1c: 8.2%
- Serum Creatinine: 1.1 mg/dL

## Encounter 1: source 2020-07-15, practice 2022-05-19, outpatient, Internal Medicine

Chief complaint: Six-month history of fatigue and exertional dyspnea without melena or hematochezia.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of fatigue and exertional dyspnea without melena or hematochezia.

HISTORY OF PRESENT ILLNESS:
This 52-year-old male with chronic liver disease, chronic kidney disease, and bilateral cataracts presents with six-month fatigue and exertional dyspnea. He reports no melena, hematochezia, or significant weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Bilateral cataracts (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Alcohol Use Disorder (in remission)
- Chronic liver disease (mixed alcoholic and metabolic steatohepatitis)
- Osteonecrosis of the left femoral head
- Iron deficiency anemia
- Chronic kidney disease with nephrotic syndrome
- Status post traumatic right below-knee amputation with phantom limb pain syndrome
- Metabolic syndrome
- Obesity

His past medical history is significant for hypertension, controlled with lisinopril, and type 2 diabetes mellitus, managed with metformin.

PAST SURGICAL HISTORY:
- Right below-knee amputation (6 months prior)
- Emergency debridement and excisional wound preparation for thermal burns
- Split-thickness skin grafting to chest and upper extremities

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Hydrochlorothiazide 12.5 mg daily
- Atorvastatin 40 mg nightly
- Ferrous sulfate 325 mg daily
- Thiamine 100 mg daily
- Folic acid 1 mg daily
- Acetaminophen 500 mg every 6 hours as needed for pain
- Ibuprofen 400 mg every 8 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Type 2 Diabetes Mellitus (father, paternal grandfather)
- Hypertension (mother)
- Chronic kidney disease (maternal uncle)
- Alcohol use disorder (father)
- Cataracts (mother)

SOCIAL HISTORY:
He has a 20-pack-year smoking history but quit 5 years ago.

VITAL SIGNS:
On physical examination, his vital signs are within normal limits.

PHYSICAL EXAMINATION:
Abdominal examination is unremarkable, and digital rectal examination reveals non-thrombosed external hemorrhoids without evidence of bleeding.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 9.8 g/dL (Normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 74 fL (Normal: 80-96 fL)
- Serum ferritin: 15 ng/mL (Normal: 24-336 ng/mL)
- Fecal occult blood test: Positive

## Encounter 2: source 2021-01-15, practice 2022-11-09, outpatient, Gastroenterology

Chief complaint: Routine follow-up for alcohol use disorder with fatigue and mild abdominal discomfort; patient reports being sober for 6 months.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for alcohol use disorder with fatigue and mild abdominal discomfort; patient reports being sober for 6 months.

HISTORY OF PRESENT ILLNESS:
The patient is a 53-year-old male with chronic liver disease and alcohol use disorder returning for follow-up, previously seen in July 2020 for fatigue. He has been sober for the last 6 months and is enrolled in an alcohol rehabilitation program. He complains of fatigue and mild abdominal discomfort.

PAST MEDICAL HISTORY:
Active Problem List:
- Bilateral cataracts (diagnosed 2020-01-15)
- Iron deficiency anemia (diagnosed 2020-07-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Alcohol Use Disorder (in remission)
- Chronic liver disease (mixed alcoholic and metabolic steatohepatitis)
- Osteonecrosis of the left femoral head
- Chronic kidney disease with nephrotic syndrome
- Status post traumatic right below-knee amputation with phantom limb pain syndrome
- Metabolic syndrome
- Obesity

His medical history is significant for type 2 diabetes mellitus and hypertension, which are well-controlled with metformin and lisinopril, respectively.

PAST SURGICAL HISTORY:
- Right below-knee amputation (6 months prior)
- Emergency debridement and excisional wound preparation for thermal burns
- Split-thickness skin grafting to chest and upper extremities

MEDICATIONS:
He takes metformin and lisinopril.

Home Medications:
- Hydrochlorothiazide 12.5 mg daily
- Atorvastatin 40 mg nightly
- Ferrous sulfate 325 mg daily
- Thiamine 100 mg daily
- Folic acid 1 mg daily
- Acetaminophen 500 mg every 6 hours as needed for pain
- Ibuprofen 400 mg every 8 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Type 2 Diabetes Mellitus (father, paternal grandfather)
- Hypertension (mother)
- Chronic kidney disease (maternal uncle)
- Alcohol use disorder (father)
- Cataracts (mother)

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: History of heavy alcohol consumption (6-8 beers daily for 15 years), currently sober for 6 months and enrolled in outpatient rehabilitation. Occupation: Former construction foreman, currently on medical disability.

PHYSICAL EXAMINATION:
On physical examination, he has mild right upper quadrant tenderness, but no hepatosplenomegaly or ascites. His skin and sclerae are anicteric.

LABORATORY DATA:
Laboratory tests reveal the following:

- AST: 70 U/L (Normal: 10-40 U/L)
- ALT: 80 U/L (Normal: 7-56 U/L)
- Alkaline phosphatase: 90 U/L (Normal: 40-130 U/L)
- Total bilirubin: 1.2 mg/dL (Normal: 0.3-1.9 mg/dL)
- Albumin: 3.9 g/dL (Normal: 3.5-5.0 g/dL)
- Platelet count: 150,000/µL (Normal: 150,000-450,000/µL)

## Encounter 3: source 2021-07-15, practice 2023-05-15, outpatient, Gastroenterology

Chief complaint: Increasing abdominal girth and fatigue over the past two months.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. David Kim, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing abdominal girth and fatigue over the past two months.

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of chronic liver disease. He reports increasing abdominal girth and fatigue over the past two months.

PAST MEDICAL HISTORY:
Active Problem List:
- Bilateral cataracts (diagnosed 2020-01-15)
- Iron deficiency anemia (diagnosed 2020-07-15)
- Alcoholic liver disease (diagnosed 2021-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Alcohol Use Disorder (in remission)
- Chronic liver disease (mixed alcoholic and metabolic steatohepatitis)
- Osteonecrosis of the left femoral head
- Chronic kidney disease with nephrotic syndrome
- Status post traumatic right below-knee amputation with phantom limb pain syndrome
- Metabolic syndrome
- Obesity

PAST SURGICAL HISTORY:
- Right below-knee amputation (6 months prior)
- Emergency debridement and excisional wound preparation for thermal burns
- Split-thickness skin grafting to chest and upper extremities

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Hydrochlorothiazide 12.5 mg daily
- Atorvastatin 40 mg nightly
- Ferrous sulfate 325 mg daily
- Thiamine 100 mg daily
- Folic acid 1 mg daily
- Acetaminophen 500 mg every 6 hours as needed for pain
- Ibuprofen 400 mg every 8 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Type 2 Diabetes Mellitus (father, paternal grandfather)
- Hypertension (mother)
- Chronic kidney disease (maternal uncle)
- Alcohol use disorder (father)
- Cataracts (mother)

SOCIAL HISTORY:
He denies alcohol use but admits to a sedentary lifestyle and a diet high in processed foods

PHYSICAL EXAMINATION:
Distended abdomen with a fluid wave, spider angiomata on the chest, and bilateral gynecomastia. His spleen is palpable 3 cm below the left costal margin.

LABORATORY DATA:
Platelet count: 110,000/µL
- AST: 80 U/L
- ALT: 65 U/L
- Total bilirubin: 2.1 mg/dL
- Albumin: 3.2 g/dL
- INR: 1.3

IMAGING:
Abdominal ultrasound shows a nodular liver contour and evidence of ascites.

## Encounter 4: source 2022-01-15, practice 2023-11-13, outpatient, Orthopedics

Chief complaint: Six-month history of progressive left groin pain exacerbated by walking and relieved by rest.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Roberto Garcia, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of progressive left groin pain exacerbated by walking and relieved by rest.

HISTORY OF PRESENT ILLNESS:
The patient with chronic liver disease and prior right BKA presents with a 6-month history of progressive left groin pain exacerbated by walking and relieved by rest. He reports no history of trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Bilateral cataracts (diagnosed 2020-01-15)
- Iron deficiency anemia (diagnosed 2020-07-15)
- Alcoholic liver disease (diagnosed 2021-01-15)
- Non-alcoholic steatohepatitis (diagnosed 2021-07-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Alcohol Use Disorder (in remission)
- Chronic liver disease (mixed alcoholic and metabolic steatohepatitis)
- Osteonecrosis of the left femoral head
- Chronic kidney disease with nephrotic syndrome
- Status post traumatic right below-knee amputation with phantom limb pain syndrome
- Metabolic syndrome
- Obesity

His past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Right below-knee amputation (6 months prior)
- Emergency debridement and excisional wound preparation for thermal burns
- Split-thickness skin grafting to chest and upper extremities

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Atorvastatin 40 mg nightly
- Ferrous sulfate 325 mg daily
- Thiamine 100 mg daily
- Folic acid 1 mg daily
- Acetaminophen 500 mg every 6 hours as needed for pain
- Ibuprofen 400 mg every 8 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Type 2 Diabetes Mellitus (father, paternal grandfather)
- Hypertension (mother)
- Chronic kidney disease (maternal uncle)
- Alcohol use disorder (father)
- Cataracts (mother)

SOCIAL HISTORY:
He admits to consuming six to eight cans of beer daily for the past 15 years.

PHYSICAL EXAMINATION:
On physical examination, the patient has a painful and limited range of motion in the left hip, particularly with internal rotation.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 13.5 g/dL (normal: 13.8-17.2 g/dL)
- White blood cell count: 7,000/uL (normal: 4,500-11,000/uL)
- Platelets: 250,000/uL (normal: 150,000-450,000/uL)
- Aspartate aminotransferase (AST): 45 U/L (normal: 10-40 U/L)
- Alanine aminotransferase (ALT): 50 U/L (normal: 7-56 U/L)
- Alkaline phosphatase: 90 U/L (normal: 40-130 U/L)
- Gamma-glutamyl transferase (GGT): 100 U/L (normal: 9-48 U/L)

IMAGING:
Radiographs of the left hip show subchondral lucency and sclerosis in the femoral head.

## Encounter 5: source 2022-07-15, practice 2024-05-14, outpatient, Nephrology

Chief complaint: Three-month history of progressive swelling in lower extremities and frothy urine.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Aisha Washington, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of progressive swelling in lower extremities and frothy urine.

HISTORY OF PRESENT ILLNESS:
This 54-year-old male with chronic kidney disease, chronic liver disease, and osteonecrosis presents with progressive lower extremity swelling and frothy urine following prior Orthopedics and Gastroenterology encounters. He denies any recent infections, changes in medications, or new stressors.

PAST MEDICAL HISTORY:
Active Problem List:
- Bilateral cataracts (diagnosed 2020-01-15)
- Iron deficiency anemia (diagnosed 2020-07-15)
- Alcoholic liver disease (diagnosed 2021-01-15)
- Non-alcoholic steatohepatitis (diagnosed 2021-07-15)
- Osteonecrosis of the femoral head (diagnosed 2022-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Alcohol Use Disorder (in remission)
- Chronic liver disease (mixed alcoholic and metabolic steatohepatitis)
- Osteonecrosis of the left femoral head
- Chronic kidney disease with nephrotic syndrome
- Status post traumatic right below-knee amputation with phantom limb pain syndrome
- Metabolic syndrome
- Obesity

PAST SURGICAL HISTORY:
- Right below-knee amputation (6 months prior)
- Emergency debridement and excisional wound preparation for thermal burns
- Split-thickness skin grafting to chest and upper extremities

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Hydrochlorothiazide 12.5 mg daily
- Atorvastatin 40 mg nightly
- Ferrous sulfate 325 mg daily
- Thiamine 100 mg daily
- Folic acid 1 mg daily
- Acetaminophen 500 mg every 6 hours as needed for pain
- Ibuprofen 400 mg every 8 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Type 2 Diabetes Mellitus (father, paternal grandfather)
- Hypertension (mother)
- Chronic kidney disease (maternal uncle)
- Alcohol use disorder (father)
- Cataracts (mother)

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: History of heavy alcohol consumption (6-8 beers daily for 15 years), currently sober for 6 months and enrolled in outpatient rehabilitation. Occupation: Former construction foreman, currently on medical disability.

VITAL SIGNS:
His blood pressure is 145/90 mmHg

PHYSICAL EXAMINATION:
, and physical examination reveals pitting edema up to the level of his knees.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum albumin: 2.1 g/dL (Normal: 3.5-5.0 g/dL)
- Serum cholesterol: 310 mg/dL (Normal: <200 mg/dL)
- Urine protein: 4+ on dipstick
- 24-hour urine protein: 5.2 g (Normal: <150 mg/24 hours)
- Serum creatinine: 1.2 mg/dL (Normal: 0.6-1.2 mg/dL)
- Antithrombin III activity: 60% (Normal: 80-120%)

IMAGING:
An ultrasound of the kidneys shows no structural abnormalities.

ASSESSMENT AND PLAN:
Given the patient's clinical presentation and laboratory findings,

## Encounter 6: source 2023-01-15, practice 2024-11-11, outpatient, Orthopedics

Chief complaint: Severe burning pain in the right lower extremity amputated below the knee 6 months ago following a traumatic accident.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Roberto Garcia, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Severe burning pain in the right lower extremity amputated below the knee 6 months ago following a traumatic accident.

HISTORY OF PRESENT ILLNESS:
The patient is a 55-year-old male with Type 2 Diabetes Mellitus, liver disease, and left femoral osteonecrosis who presents with severe burning pain in the right lower extremity which was amputated below the knee following a traumatic accident 6 months ago. He describes the pain as intermittent, with episodes lasting several minutes to hours, and it often occurs at night. The patient has tried over-the-counter acetaminophen and ibuprofen with minimal relief. The patient reports that the pain is affecting his sleep and quality of life.

PAST MEDICAL HISTORY:
Active Problem List:
- Bilateral cataracts (diagnosed 2020-01-15)
- Iron deficiency anemia (diagnosed 2020-07-15)
- Alcoholic liver disease (diagnosed 2021-01-15)
- Non-alcoholic steatohepatitis (diagnosed 2021-07-15)
- Osteonecrosis of the femoral head (diagnosed 2022-01-15)
- Nephrotic syndrome (edema management) (diagnosed 2022-07-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Alcohol Use Disorder (in remission)
- Chronic liver disease (mixed alcoholic and metabolic steatohepatitis)
- Osteonecrosis of the left femoral head
- Chronic kidney disease with nephrotic syndrome
- Status post traumatic right below-knee amputation with phantom limb pain syndrome
- Metabolic syndrome
- Obesity

PAST SURGICAL HISTORY:
- Right below-knee amputation (6 months prior)
- Emergency debridement and excisional wound preparation for thermal burns
- Split-thickness skin grafting to chest and upper extremities

MEDICATIONS:
He takes metformin; lisinopril; over-the-counter acetaminophen; ibuprofen.

Home Medications:
- Hydrochlorothiazide 12.5 mg daily
- Atorvastatin 40 mg nightly
- Ferrous sulfate 325 mg daily
- Thiamine 100 mg daily
- Folic acid 1 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Type 2 Diabetes Mellitus (father, paternal grandfather)
- Hypertension (mother)
- Chronic kidney disease (maternal uncle)
- Alcohol use disorder (father)
- Cataracts (mother)

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: History of heavy alcohol consumption (6-8 beers daily for 15 years), currently sober for 6 months and enrolled in outpatient rehabilitation. Occupation: Former construction foreman, currently on medical disability.

PHYSICAL EXAMINATION:
On examination, the stump appears well-healed without signs of infection or skin breakdown. The patient appears distressed

LABORATORY DATA:
Hemoglobin A1c: 6.8%
- Fasting blood glucose: 138 mg/dL
- Complete blood count: within normal limits
- Serum creatinine: 1.0 mg/dL

## Encounter 7: source 2024-01-15, practice 2025-11-11, icu, Surgery

Chief complaint: Second and third-degree burns covering 40% of total body surface area sustained in a house fire 48 hours ago with wound infection and sepsis.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Lisa Thompson, MD
Department: Surgery
Visit Type: icu

CHIEF COMPLAINT:
Second and third-degree burns covering 40% of total body surface area sustained in a house fire 48 hours ago with wound infection and sepsis.

HISTORY OF PRESENT ILLNESS:
The patient is a 56-year-old male with mixed alcoholic and metabolic steatohepatitis, chronic kidney disease with nephrotic syndrome, and status post traumatic right below-knee amputation who was recently followed for phantom limb pain. The burns were sustained in a house fire 48 hours ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Bilateral cataracts (diagnosed 2020-01-15)
- Iron deficiency anemia (diagnosed 2020-07-15)
- Alcoholic liver disease (diagnosed 2021-01-15)
- Non-alcoholic steatohepatitis (diagnosed 2021-07-15)
- Osteonecrosis of the femoral head (diagnosed 2022-01-15)
- Nephrotic syndrome (edema management) (diagnosed 2022-07-15)
- Phantom limb pain (diagnosed 2023-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Alcohol Use Disorder (in remission)
- Chronic liver disease (mixed alcoholic and metabolic steatohepatitis)
- Osteonecrosis of the left femoral head
- Chronic kidney disease with nephrotic syndrome
- Status post traumatic right below-knee amputation with phantom limb pain syndrome
- Metabolic syndrome
- Obesity

He has a past medical history of type 2 diabetes mellitus and hypertension.

PAST SURGICAL HISTORY:
- Right below-knee amputation (6 months prior)
- Emergency debridement and excisional wound preparation for thermal burns
- Split-thickness skin grafting to chest and upper extremities

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Hydrochlorothiazide 12.5 mg daily
- Atorvastatin 40 mg nightly
- Ferrous sulfate 325 mg daily
- Thiamine 100 mg daily
- Folic acid 1 mg daily
- Acetaminophen 500 mg every 6 hours as needed for pain
- Ibuprofen 400 mg every 8 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Type 2 Diabetes Mellitus (father, paternal grandfather)
- Hypertension (mother)
- Chronic kidney disease (maternal uncle)
- Alcohol use disorder (father)
- Cataracts (mother)

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: History of heavy alcohol consumption (6-8 beers daily for 15 years), currently sober for 6 months and enrolled in outpatient rehabilitation. Occupation: Former construction foreman, currently on medical disability.

VITAL SIGNS:
On admission, his vital signs are: temperature 38.9°C (102°F), blood pressure 90/60 mmHg, heart rate 110 beats/min, and respiratory rate 22 breaths/min.

PHYSICAL EXAMINATION:
Physical examination reveals weeping, erythematous wounds with a purulent discharge and a black eschar on his chest.

LABORATORY DATA:
Laboratory results are as follows:
- White blood cell count: 18,000/mm³
- Hemoglobin: 13.2 g/dL
- Platelet count: 250,000/mm³
- Serum creatinine: 1.2 mg/dL
- Blood urea nitrogen: 21 mg/dL
- Sodium: 135 mEq/L
- Potassium: 4.5 mEq/L
- Chloride: 100 mEq/L
- Bicarbonate: 24 mEq/L
- Glucose: 180 mg/dL

OTHER STUDIES:
Blood cultures have been drawn but results are pending.

ASSESSMENT AND PLAN:
Given the patient's clinical presentation and risk factors,
