# synthetic_hospital patient 2280

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 66 at first encounter, sex M, Non-Hispanic White, Medicare with supplemental private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-08-25 (Thu) | outpatient | Pulmonology |
| 1 | 2020-04-15 | 2022-12-12 (Mon) | outpatient | Gastroenterology |
| 2 | 2020-10-15 | 2023-06-07 (Wed) | outpatient | Dermatology |
| 3 | 2021-04-15 | 2023-12-01 (Fri) | outpatient | Hepatology |
| 4 | 2022-01-15 | 2024-08-23 (Fri) | outpatient | Gastroenterology |
| 5 | 2022-07-15 | 2025-03-05 (Wed) | outpatient | Oncology |
| 6 | 2023-01-15 | 2025-09-02 (Tue) | ed | Emergency Medicine |
| 7 | 2023-07-15 | 2026-03-09 (Mon) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Sulfonamides (rash)",
  "Contrast dye (anaphylaxis)"
 ],
 "insurance": "Medicare with supplemental private insurance",
 "occupation": "Retired shipyard boiler technician",
 "alcohol_use": "History of heavy alcohol use (approximately 60 g/day for 30 years), abstinent for 5 years following diagnosis of cirrhosis",
 "family_history": [
  "Father: Pancreatic adenocarcinoma (deceased age 72)",
  "Paternal uncle: Colorectal cancer (deceased age 68)",
  "Brother: Prostate cancer (age 70)",
  "Mother: Emphysema (deceased age 78)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 40 pack-year history, quit 5 years ago at age 61",
 "home_medications": [
  {
   "dose": "24 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "8 units subcutaneous with meals (sliding scale)",
   "name": "Insulin lispro"
  },
  {
   "dose": "81 mg oral daily",
   "name": "Aspirin"
  },
  {
   "dose": "40 mg oral nightly",
   "name": "Atorvastatin"
  },
  {
   "dose": "5 mg oral daily",
   "name": "Amlodipine"
  },
  {
   "dose": "18 mcg inhaled daily",
   "name": "Tiotropium bromide"
  },
  {
   "dose": "90 mcg inhaled every 4-6 hours as needed",
   "name": "Albuterol sulfate"
  },
  {
   "dose": "40 mg oral twice daily",
   "name": "Furosemide"
  },
  {
   "dose": "100 mg oral daily",
   "name": "Spironolactone"
  },
  {
   "dose": "30 mL oral twice daily (titrate to 3-4 bowel movements daily)",
   "name": "Lactulose"
  },
  {
   "dose": "550 mg oral twice daily",
   "name": "Rifaximin"
  },
  {
   "dose": "5 mg oral every 6 hours as needed for pain",
   "name": "Oxycodone"
  },
  {
   "dose": "400 mg oral three times daily",
   "name": "Pentoxifylline"
  },
  {
   "dose": "25 mg oral daily",
   "name": "Metoprolol succinate"
  }
 ],
 "surgical_history": [
  "Radical retropubic prostatectomy (age 61)",
  "Low anterior resection with diverting loop ileostomy for rectal cancer (age 65, ileostomy reversed 6 months later)",
  "Right femoral-popliteal bypass graft (age 64)",
  "Multiple venous skin grafting procedures to left lower extremity (ages 64-65)"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus",
  "Peripheral arterial disease",
  "Chronic venous insufficiency",
  "Chronic obstructive pulmonary disease (COPD)",
  "Asbestos-related pneumoconiosis",
  "Hepatitis C virus infection (Genotype 1a)",
  "Cirrhosis with decompensation (Child-Pugh Class C)",
  "Atherosclerotic cardiovascular disease",
  "History of prostate adenocarcinoma, status post radical prostatectomy and adjuvant radiation therapy (5 years prior, biochemical remission)",
  "History of stage IIIB rectal adenocarcinoma, status post neoadjuvant chemoradiation and low anterior resection (18 months prior)",
  "Chronic radiation proctitis",
  "Osteoarthritis",
  "Chronic kidney disease stage 3a"
 ],
 "age_at_first_encounter": 66
}
```

Primary diagnoses: ["Pancreatic carcinoma", "Squamous cell carcinoma (Marjolin's ulcer)", "Metastatic carcinoma to bone", "Chronic radiation proctitis", "Pneumoconiosis due to asbestos", "Acute cholecystitis", "Toxic megacolon secondary to Clostridioides difficile infection", "Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate)"]
Comorbidities: ["Anemia", "Type 2 diabetes mellitus", "History of recent pneumonia"]

## Encounter 0: source 2020-01-15, practice 2022-08-25, outpatient, Pulmonology

Chief complaint: Progressive shortness of breath and nonproductive cough over 6 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath and nonproductive cough over 6 months

HISTORY OF PRESENT ILLNESS:
6-month history of progressive shortness of breath and a nonproductive cough

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus
- Peripheral arterial disease
- Chronic venous insufficiency
- Chronic obstructive pulmonary disease (COPD)
- Asbestos-related pneumoconiosis
- Hepatitis C virus infection (Genotype 1a)
- Cirrhosis with decompensation (Child-Pugh Class C)
- Atherosclerotic cardiovascular disease
- History of prostate adenocarcinoma, status post radical prostatectomy and adjuvant radiation therapy (5 years prior, biochemical remission)
- History of stage IIIB rectal adenocarcinoma, status post neoadjuvant chemoradiation and low anterior resection (18 months prior)
- Chronic radiation proctitis
- Osteoarthritis
- Chronic kidney disease stage 3a

PAST SURGICAL HISTORY:
- Radical retropubic prostatectomy (age 61)
- Low anterior resection with diverting loop ileostomy for rectal cancer (age 65, ileostomy reversed 6 months later)
- Right femoral-popliteal bypass graft (age 64)
- Multiple venous skin grafting procedures to left lower extremity (ages 64-65)

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro 8 units subcutaneous with meals (sliding scale)
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral nightly
- Amlodipine 5 mg oral daily
- Tiotropium bromide 18 mcg inhaled daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed
- Furosemide 40 mg oral twice daily
- Spironolactone 100 mg oral daily
- Lactulose 30 mL oral twice daily (titrate to 3-4 bowel movements daily)
- Rifaximin 550 mg oral twice daily
- Oxycodone 5 mg oral every 6 hours as needed for pain
- Pentoxifylline 400 mg oral three times daily
- Metoprolol succinate 25 mg oral daily

ALLERGIES:
Sulfonamides (rash), Contrast dye (anaphylaxis)

FAMILY HISTORY:
- Father: Pancreatic adenocarcinoma (deceased age 72)
- Paternal uncle: Colorectal cancer (deceased age 68)
- Brother: Prostate cancer (age 70)
- Mother: Emphysema (deceased age 78)

SOCIAL HISTORY:
He reports a 20-pack-year smoking history but quit 10 years ago. He worked in shipbuilding until his retirement 5 years ago.

PHYSICAL EXAMINATION:
On physical examination, you note digital clubbing and end-inspiratory crackles at the lung bases.

IMAGING:
A chest X-ray shows bilateral lower lobe fibrosis and pleural thickening with calcified pleural plaques.

OTHER STUDIES:
Pulmonary function tests reveal the following:

- Total lung capacity (TLC): Decreased
- Forced vital capacity (FVC): Decreased
- Forced expiratory volume in 1 second (FEV1): Normal
- FEV1/FVC ratio: Normal
- Diffusion capacity for carbon monoxide (DLCO): Decreased

## Encounter 1: source 2020-04-15, practice 2022-12-12, outpatient, Gastroenterology

Chief complaint: Rectal bleeding and bowel habit changes over 6 months

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Rectal bleeding and bowel habit changes over 6 months

HISTORY OF PRESENT ILLNESS:
This 66-year-old male with chronic radiation proctitis status post low anterior resection presents with a 6-month history of rectal bleeding and bowel habit changes. He reports increased urgency and episodes of fecal incontinence.

PAST MEDICAL HISTORY:
Active Problem List:
- Pneumoconiosis due to asbestos (diagnosed 2020-01-15)
- Type 2 diabetes mellitus
- Peripheral arterial disease
- Chronic venous insufficiency
- Chronic obstructive pulmonary disease (COPD)
- Asbestos-related pneumoconiosis
- Hepatitis C virus infection (Genotype 1a)
- Cirrhosis with decompensation (Child-Pugh Class C)
- Atherosclerotic cardiovascular disease
- History of prostate adenocarcinoma, status post radical prostatectomy and adjuvant radiation therapy (5 years prior, biochemical remission)
- History of stage IIIB rectal adenocarcinoma, status post neoadjuvant chemoradiation and low anterior resection (18 months prior)
- Chronic radiation proctitis
- Osteoarthritis
- Chronic kidney disease stage 3a

His past medical history is significant for stage IIIB rectal adenocarcinoma treated with chemoradiation therapy, which was completed 18 months ago.

PAST SURGICAL HISTORY:
- Radical retropubic prostatectomy (age 61)
- Low anterior resection with diverting loop ileostomy for rectal cancer (age 65, ileostomy reversed 6 months later)
- Right femoral-popliteal bypass graft (age 64)
- Multiple venous skin grafting procedures to left lower extremity (ages 64-65)

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro 8 units subcutaneous with meals (sliding scale)
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral nightly
- Amlodipine 5 mg oral daily
- Tiotropium bromide 18 mcg inhaled daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed
- Furosemide 40 mg oral twice daily
- Spironolactone 100 mg oral daily
- Lactulose 30 mL oral twice daily (titrate to 3-4 bowel movements daily)
- Rifaximin 550 mg oral twice daily
- Oxycodone 5 mg oral every 6 hours as needed for pain
- Pentoxifylline 400 mg oral three times daily
- Metoprolol succinate 25 mg oral daily

ALLERGIES:
Sulfonamides (rash), Contrast dye (anaphylaxis)

FAMILY HISTORY:
- Father: Pancreatic adenocarcinoma (deceased age 72)
- Paternal uncle: Colorectal cancer (deceased age 68)
- Brother: Prostate cancer (age 70)
- Mother: Emphysema (deceased age 78)

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit 5 years ago at age 61. Alcohol: History of heavy alcohol use (approximately 60 g/day for 30 years), abstinent for 5 years following diagnosis of cirrhosis. Occupation: Retired shipyard boiler technician.

PHYSICAL EXAMINATION:
On physical examination, the patient appears well. Digital rectal examination reveals a non-tender, firm rectum with no palpable masses.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 10.2 g/dL (Normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 92 fL (Normal: 80-100 fL)
- White blood cell count: 7,000 /µL (Normal: 4,500-11,000 /µL)
- Platelet count: 250,000 /µL (Normal: 150,000-450,000 /µL)

OTHER STUDIES:
Colonoscopy shows multiple areas of telangiectasia in the rectum with no evidence of recurrent malignancy.

## Encounter 2: source 2020-10-15, practice 2023-06-07, outpatient, Dermatology

Chief complaint: Non-healing ulcer on lower left leg present for over two years with recent increase in size and pain

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Aisha Patel, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Non-healing ulcer on lower left leg present for over two years with recent increase in size and pain

HISTORY OF PRESENT ILLNESS:
The patient with diabetes and vascular disease, evaluated recently for pulmonary and GI issues, presents with an ulcer he reports has been present for over two years but has recently increased in size and become more painful. He denies fever or chills.

PAST MEDICAL HISTORY:
Active Problem List:
- Pneumoconiosis due to asbestos (diagnosed 2020-01-15)
- Chronic radiation proctitis (diagnosed 2020-04-15)
- Type 2 diabetes mellitus
- Peripheral arterial disease
- Chronic venous insufficiency
- Chronic obstructive pulmonary disease (COPD)
- Asbestos-related pneumoconiosis
- Hepatitis C virus infection (Genotype 1a)
- Cirrhosis with decompensation (Child-Pugh Class C)
- Atherosclerotic cardiovascular disease
- History of prostate adenocarcinoma, status post radical prostatectomy and adjuvant radiation therapy (5 years prior, biochemical remission)
- History of stage IIIB rectal adenocarcinoma, status post neoadjuvant chemoradiation and low anterior resection (18 months prior)
- Osteoarthritis
- Chronic kidney disease stage 3a

Medical history is significant for peripheral arterial disease and chronic venous insufficiency.

PAST SURGICAL HISTORY:
- Radical retropubic prostatectomy (age 61)
- Low anterior resection with diverting loop ileostomy for rectal cancer (age 65, ileostomy reversed 6 months later)
- Right femoral-popliteal bypass graft (age 64)
- Multiple venous skin grafting procedures to left lower extremity (ages 64-65)

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro 8 units subcutaneous with meals (sliding scale)
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral nightly
- Amlodipine 5 mg oral daily
- Tiotropium bromide 18 mcg inhaled daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed
- Furosemide 40 mg oral twice daily
- Spironolactone 100 mg oral daily
- Lactulose 30 mL oral twice daily (titrate to 3-4 bowel movements daily)
- Rifaximin 550 mg oral twice daily
- Oxycodone 5 mg oral every 6 hours as needed for pain
- Pentoxifylline 400 mg oral three times daily
- Metoprolol succinate 25 mg oral daily

ALLERGIES:
Sulfonamides (rash), Contrast dye (anaphylaxis)

FAMILY HISTORY:
- Father: Pancreatic adenocarcinoma (deceased age 72)
- Paternal uncle: Colorectal cancer (deceased age 68)
- Brother: Prostate cancer (age 70)
- Mother: Emphysema (deceased age 78)

SOCIAL HISTORY:
He has a 30-pack-year smoking history but quit 10 years ago.

PHYSICAL EXAMINATION:
On physical examination, the ulcer measures 3 cm in diameter, with raised, everted edges and a base that bleeds easily upon contact. There is surrounding induration and multiple areas of keratinization. No lymphadenopathy is palpable. The rest of the examination is unremarkable.

LABORATORY DATA:
Laboratory values:
- Hemoglobin A1c: 7.8%
- Fasting blood glucose: 172 mg/dL
- Complete blood count: within normal limits
- Ankle-brachial index: 0.7 on the left

OTHER STUDIES:
A biopsy of the ulcer edge is performed.

## Encounter 3: source 2021-04-15, practice 2023-12-01, outpatient, Hepatology

Chief complaint: Progressive fatigue, abdominal distension, and 10-pound weight loss over 3 months

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. James O'Brien, MD
Department: Hepatology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue, abdominal distension, and 10-pound weight loss over 3 months

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with decompensated cirrhosis, prior rectal cancer with chronic radiation proctitis, and recent squamous cell carcinoma returns for evaluation of progressive fatigue, abdominal distension, and 10-pound weight loss over three months. He denies any fever, chills, or night sweats.

PAST MEDICAL HISTORY:
Active Problem List:
- Pneumoconiosis due to asbestos (diagnosed 2020-01-15)
- Chronic radiation proctitis (diagnosed 2020-04-15)
- Squamous cell carcinoma (Marjolin's ulcer) (diagnosed 2020-10-15)
- Type 2 diabetes mellitus
- Peripheral arterial disease
- Chronic venous insufficiency
- Chronic obstructive pulmonary disease (COPD)
- Asbestos-related pneumoconiosis
- Hepatitis C virus infection (Genotype 1a)
- Cirrhosis with decompensation (Child-Pugh Class C)
- Atherosclerotic cardiovascular disease
- History of prostate adenocarcinoma, status post radical prostatectomy and adjuvant radiation therapy (5 years prior, biochemical remission)
- History of stage IIIB rectal adenocarcinoma, status post neoadjuvant chemoradiation and low anterior resection (18 months prior)
- Osteoarthritis
- Chronic kidney disease stage 3a

PAST SURGICAL HISTORY:
- Radical retropubic prostatectomy (age 61)
- Low anterior resection with diverting loop ileostomy for rectal cancer (age 65, ileostomy reversed 6 months later)
- Right femoral-popliteal bypass graft (age 64)
- Multiple venous skin grafting procedures to left lower extremity (ages 64-65)

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro 8 units subcutaneous with meals (sliding scale)
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral nightly
- Amlodipine 5 mg oral daily
- Tiotropium bromide 18 mcg inhaled daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed
- Furosemide 40 mg oral twice daily
- Spironolactone 100 mg oral daily
- Lactulose 30 mL oral twice daily (titrate to 3-4 bowel movements daily)
- Rifaximin 550 mg oral twice daily
- Oxycodone 5 mg oral every 6 hours as needed for pain
- Pentoxifylline 400 mg oral three times daily
- Metoprolol succinate 25 mg oral daily

ALLERGIES:
Sulfonamides (rash), Contrast dye (anaphylaxis)

FAMILY HISTORY:
- Father: Pancreatic adenocarcinoma (deceased age 72)
- Paternal uncle: Colorectal cancer (deceased age 68)
- Brother: Prostate cancer (age 70)
- Mother: Emphysema (deceased age 78)

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit 5 years ago at age 61. Alcohol: History of heavy alcohol use (approximately 60 g/day for 30 years), abstinent for 5 years following diagnosis of cirrhosis. Occupation: Retired shipyard boiler technician.

PHYSICAL EXAMINATION:
On examination, he is cachectic with jaundiced skin and sclera. His abdomen is distended with a fluid wave consistent with ascites and a firm, irregular mass is palpable in the right upper quadrant.

LABORATORY DATA:
His laboratory tests show:

- Hemoglobin: 11.2 g/dL
- Platelet count: 85,000/µL
- INR: 1.6
- Total bilirubin: 3.2 mg/dL
- Albumin: 2.9 g/dL
- AST: 80 U/L
- ALT: 75 U/L
- Alkaline phosphatase: 120 U/L
- α-fetoprotein (AFP): 400 ng/mL (Normal < 10 ng/mL)

IMAGING:
An abdominal ultrasound reveals a 5 cm mass in the right lobe of the liver with heterogeneous echotexture and nodular appearance. There is no evidence of biliary obstruction.

ASSESSMENT AND PLAN:
Based on the clinical presentation and test results,

## Encounter 4: source 2022-01-15, practice 2024-08-23, outpatient, Gastroenterology

Chief complaint: Episodic abdominal pain, weight loss, and migratory superficial thrombophlebitis

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Fatima Al-Hassan, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Episodic abdominal pain, weight loss, and migratory superficial thrombophlebitis

HISTORY OF PRESENT ILLNESS:
The patient returns to Gastroenterology with progressive symptoms amid his history of radiation proctitis and decompensated cirrhosis, following Hepatology evaluation nine months prior. He reports a two-week history of tenderness and erythema along his left arm veins which resolved spontaneously, now noting similar symptoms appearing on his right chest wall.

PAST MEDICAL HISTORY:
Active Problem List:
- Pneumoconiosis due to asbestos (diagnosed 2020-01-15)
- Chronic radiation proctitis (diagnosed 2020-04-15)
- Squamous cell carcinoma (Marjolin's ulcer) (diagnosed 2020-10-15)
- Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate) (diagnosed 2021-04-15)
- Type 2 diabetes mellitus
- Peripheral arterial disease
- Chronic venous insufficiency
- Chronic obstructive pulmonary disease (COPD)
- Asbestos-related pneumoconiosis
- Hepatitis C virus infection (Genotype 1a)
- Cirrhosis with decompensation (Child-Pugh Class C)
- Atherosclerotic cardiovascular disease
- History of prostate adenocarcinoma, status post radical prostatectomy and adjuvant radiation therapy (5 years prior, biochemical remission)
- History of stage IIIB rectal adenocarcinoma, status post neoadjuvant chemoradiation and low anterior resection (18 months prior)
- Osteoarthritis
- Chronic kidney disease stage 3a

PAST SURGICAL HISTORY:
- Radical retropubic prostatectomy (age 61)
- Low anterior resection with diverting loop ileostomy for rectal cancer (age 65, ileostomy reversed 6 months later)
- Right femoral-popliteal bypass graft (age 64)
- Multiple venous skin grafting procedures to left lower extremity (ages 64-65)

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro 8 units subcutaneous with meals (sliding scale)
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral nightly
- Amlodipine 5 mg oral daily
- Tiotropium bromide 18 mcg inhaled daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed
- Furosemide 40 mg oral twice daily
- Spironolactone 100 mg oral daily
- Lactulose 30 mL oral twice daily (titrate to 3-4 bowel movements daily)
- Rifaximin 550 mg oral twice daily
- Oxycodone 5 mg oral every 6 hours as needed for pain
- Pentoxifylline 400 mg oral three times daily
- Metoprolol succinate 25 mg oral daily

ALLERGIES:
Sulfonamides (rash), Contrast dye (anaphylaxis)

FAMILY HISTORY:
His family history is significant for lung cancer in his father.

SOCIAL HISTORY:
He has a 40-pack-year smoking history but quit 5 years ago.

PHYSICAL EXAMINATION:
On physical examination, there is palpable cord-like induration along the superficial veins of the right chest wall with overlying erythema and tenderness. His abdomen is soft, non-distended, with no palpable masses or hepatosplenomegaly.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 12.1 g/dL
- White blood cell count: 7,800 /μL
- Platelet count: 305,000 /μL
- Fasting blood glucose: 156 mg/dL
- CA 19-9: 120 U/mL (normal <37 U/mL)
- Carcinoembryonic antigen (CEA): 5.2 ng/mL (normal 0-3 ng/mL)

## Encounter 5: source 2022-07-15, practice 2025-03-05, outpatient, Oncology

Chief complaint: Persistent lower back pain for 3 months, worse at night and unrelieved by rest

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David Kim, MD
Department: Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent lower back pain for 3 months, worse at night and unrelieved by rest

HISTORY OF PRESENT ILLNESS:
Patient with extensive oncologic history returns. Back pain is described as a dull ache that worsens at night and does not improve with rest.

PAST MEDICAL HISTORY:
Active Problem List:
- Pneumoconiosis due to asbestos (diagnosed 2020-01-15)
- Chronic radiation proctitis (diagnosed 2020-04-15)
- Squamous cell carcinoma (Marjolin's ulcer) (diagnosed 2020-10-15)
- Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate) (diagnosed 2021-04-15)
- Pancreatic carcinoma (diagnosed 2022-01-15)
- Type 2 diabetes mellitus
- Peripheral arterial disease
- Chronic venous insufficiency
- Chronic obstructive pulmonary disease (COPD)
- Asbestos-related pneumoconiosis
- Hepatitis C virus infection (Genotype 1a)
- Cirrhosis with decompensation (Child-Pugh Class C)
- Atherosclerotic cardiovascular disease
- History of prostate adenocarcinoma, status post radical prostatectomy and adjuvant radiation therapy (5 years prior, biochemical remission)
- History of stage IIIB rectal adenocarcinoma, status post neoadjuvant chemoradiation and low anterior resection (18 months prior)
- Osteoarthritis
- Chronic kidney disease stage 3a

He has a history of prostate cancer treated with radical prostatectomy and radiation therapy 5 years ago, currently in remission.

PAST SURGICAL HISTORY:
- Radical retropubic prostatectomy (age 61)
- Low anterior resection with diverting loop ileostomy for rectal cancer (age 65, ileostomy reversed 6 months later)
- Right femoral-popliteal bypass graft (age 64)
- Multiple venous skin grafting procedures to left lower extremity (ages 64-65)

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro 8 units subcutaneous with meals (sliding scale)
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral nightly
- Amlodipine 5 mg oral daily
- Tiotropium bromide 18 mcg inhaled daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed
- Furosemide 40 mg oral twice daily
- Spironolactone 100 mg oral daily
- Lactulose 30 mL oral twice daily (titrate to 3-4 bowel movements daily)
- Rifaximin 550 mg oral twice daily
- Oxycodone 5 mg oral every 6 hours as needed for pain
- Pentoxifylline 400 mg oral three times daily
- Metoprolol succinate 25 mg oral daily

ALLERGIES:
Sulfonamides (rash), Contrast dye (anaphylaxis)

FAMILY HISTORY:
- Father: Pancreatic adenocarcinoma (deceased age 72)
- Paternal uncle: Colorectal cancer (deceased age 68)
- Brother: Prostate cancer (age 70)
- Mother: Emphysema (deceased age 78)

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit 5 years ago at age 61. Alcohol: History of heavy alcohol use (approximately 60 g/day for 30 years), abstinent for 5 years following diagnosis of cirrhosis. Occupation: Retired shipyard boiler technician.

PHYSICAL EXAMINATION:
On physical examination, there is focal tenderness over the T10 vertebra. Neurological examination reveals mild weakness in the lower extremities and decreased sensation to light touch below the level of the umbilicus.

LABORATORY DATA:
Recent laboratory results are as follows:

- Serum calcium: 11.2 mg/dL (normal: 8.5-10.2 mg/dL)
- Alkaline phosphatase: 140 U/L (normal: 40-130 U/L)
- Prostate-specific antigen (PSA): 0.2 ng/mL (normal: <4 ng/mL)
- Hemoglobin: 12.5 g/dL (normal: 13.8-17.2 g/dL)

## Encounter 6: source 2023-01-15, practice 2025-09-02, ed, Emergency Medicine

Chief complaint: Severe right upper quadrant pain, fever, and vomiting for 24 hours

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Maria Gonzalez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe right upper quadrant pain, fever, and vomiting for 24 hours

HISTORY OF PRESENT ILLNESS:
This 69-year-old male with pancreatic carcinoma, metastatic bone disease, and decompensated Hepatitis C cirrhosis presents to the emergency department with a 24-hour history of severe right upper quadrant pain, fever, and vomiting. He mentions that the pain radiates to his right shoulder and describes it as constant and sharp.

PAST MEDICAL HISTORY:
Active Problem List:
- Pneumoconiosis due to asbestos (diagnosed 2020-01-15)
- Chronic radiation proctitis (diagnosed 2020-04-15)
- Squamous cell carcinoma (Marjolin's ulcer) (diagnosed 2020-10-15)
- Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate) (diagnosed 2021-04-15)
- Pancreatic carcinoma (diagnosed 2022-01-15)
- Metastatic carcinoma to bone (diagnosed 2022-07-15)
- Type 2 diabetes mellitus
- Peripheral arterial disease
- Chronic venous insufficiency
- Chronic obstructive pulmonary disease (COPD)
- Asbestos-related pneumoconiosis
- Hepatitis C virus infection (Genotype 1a)
- Cirrhosis with decompensation (Child-Pugh Class C)
- Atherosclerotic cardiovascular disease
- History of prostate adenocarcinoma, status post radical prostatectomy and adjuvant radiation therapy (5 years prior, biochemical remission)
- History of stage IIIB rectal adenocarcinoma, status post neoadjuvant chemoradiation and low anterior resection (18 months prior)
- Osteoarthritis
- Chronic kidney disease stage 3a

PAST SURGICAL HISTORY:
- Radical retropubic prostatectomy (age 61)
- Low anterior resection with diverting loop ileostomy for rectal cancer (age 65, ileostomy reversed 6 months later)
- Right femoral-popliteal bypass graft (age 64)
- Multiple venous skin grafting procedures to left lower extremity (ages 64-65)

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro 8 units subcutaneous with meals (sliding scale)
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral nightly
- Amlodipine 5 mg oral daily
- Tiotropium bromide 18 mcg inhaled daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed
- Furosemide 40 mg oral twice daily
- Spironolactone 100 mg oral daily
- Lactulose 30 mL oral twice daily (titrate to 3-4 bowel movements daily)
- Rifaximin 550 mg oral twice daily
- Oxycodone 5 mg oral every 6 hours as needed for pain
- Pentoxifylline 400 mg oral three times daily
- Metoprolol succinate 25 mg oral daily

ALLERGIES:
Sulfonamides (rash), Contrast dye (anaphylaxis)

FAMILY HISTORY:
- Father: Pancreatic adenocarcinoma (deceased age 72)
- Paternal uncle: Colorectal cancer (deceased age 68)
- Brother: Prostate cancer (age 70)
- Mother: Emphysema (deceased age 78)

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit 5 years ago at age 61. Alcohol: History of heavy alcohol use (approximately 60 g/day for 30 years), abstinent for 5 years following diagnosis of cirrhosis. Occupation: Retired shipyard boiler technician.

VITAL SIGNS:
On examination, his temperature is 38.9°C (102°F), heart rate is 110 beats per minute, and blood pressure is 135/85 mmHg

PHYSICAL EXAMINATION:
Abdominal examination reveals marked tenderness in the right upper quadrant with a positive Murphy&#x27;s sign

LABORATORY DATA:
Laboratory tests show leukocytosis with a left shift

IMAGING:
An abdominal ultrasound demonstrates gallstones and gas within the gallbladder wall. A CT scan of the abdomen confirms the presence of gas in the gallbladder wall and pericholecystic area.

## Encounter 7: source 2023-07-15, practice 2026-03-09, icu, Emergency Medicine

Chief complaint: Watery diarrhea, abdominal cramping, and low-grade fever for 5 days following recent antibiotic course

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Robert Johnson, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Watery diarrhea, abdominal cramping, and low-grade fever for 5 days following recent antibiotic course

HISTORY OF PRESENT ILLNESS:
This 69-year-old male with Child-Pugh Class C cirrhosis, chronic radiation proctitis, and recent acute cholecystitis presents with acute gastrointestinal symptoms. He completed a course of clindamycin 2 weeks ago. Despite initiating oral vancomycin therapy, on the third day of treatment, the patient's diarrhea resolves, but he becomes more lethargic, develops a fever of 39.4°C (102.9°F), and his heart rate increases to 120 beats/min.

PAST MEDICAL HISTORY:
Active Problem List:
- Pneumoconiosis due to asbestos (diagnosed 2020-01-15)
- Chronic radiation proctitis (diagnosed 2020-04-15)
- Squamous cell carcinoma (Marjolin's ulcer) (diagnosed 2020-10-15)
- Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate) (diagnosed 2021-04-15)
- Pancreatic carcinoma (diagnosed 2022-01-15)
- Metastatic carcinoma to bone (diagnosed 2022-07-15)
- Acute cholecystitis (diagnosed 2023-01-15)
- Type 2 diabetes mellitus
- Peripheral arterial disease
- Chronic venous insufficiency
- Chronic obstructive pulmonary disease (COPD)
- Asbestos-related pneumoconiosis
- Hepatitis C virus infection (Genotype 1a)
- Cirrhosis with decompensation (Child-Pugh Class C)
- Atherosclerotic cardiovascular disease
- History of prostate adenocarcinoma, status post radical prostatectomy and adjuvant radiation therapy (5 years prior, biochemical remission)
- History of stage IIIB rectal adenocarcinoma, status post neoadjuvant chemoradiation and low anterior resection (18 months prior)
- Osteoarthritis
- Chronic kidney disease stage 3a

PAST SURGICAL HISTORY:
- Radical retropubic prostatectomy (age 61)
- Low anterior resection with diverting loop ileostomy for rectal cancer (age 65, ileostomy reversed 6 months later)
- Right femoral-popliteal bypass graft (age 64)
- Multiple venous skin grafting procedures to left lower extremity (ages 64-65)

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro 8 units subcutaneous with meals (sliding scale)
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral nightly
- Amlodipine 5 mg oral daily
- Tiotropium bromide 18 mcg inhaled daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed
- Furosemide 40 mg oral twice daily
- Spironolactone 100 mg oral daily
- Lactulose 30 mL oral twice daily (titrate to 3-4 bowel movements daily)
- Rifaximin 550 mg oral twice daily
- Oxycodone 5 mg oral every 6 hours as needed for pain
- Pentoxifylline 400 mg oral three times daily
- Metoprolol succinate 25 mg oral daily

ALLERGIES:
Sulfonamides (rash), Contrast dye (anaphylaxis)

FAMILY HISTORY:
- Father: Pancreatic adenocarcinoma (deceased age 72)
- Paternal uncle: Colorectal cancer (deceased age 68)
- Brother: Prostate cancer (age 70)
- Mother: Emphysema (deceased age 78)

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit 5 years ago at age 61. Alcohol: History of heavy alcohol use (approximately 60 g/day for 30 years), abstinent for 5 years following diagnosis of cirrhosis. Occupation: Retired shipyard boiler technician.

VITAL SIGNS:
His vital signs on admission are: temperature 38.2°C (100.8°F), blood pressure 110/70 mmHg, heart rate 102 beats/min, and respiratory rate 22 breaths/min.

PHYSICAL EXAMINATION:
Physical examination reveals a distended abdomen with diffuse tenderness but no rebound or guarding. Bowel sounds are hypoactive.

LABORATORY DATA:
Laboratory studies show:

- White blood cell count: 18,000/mm³ (normal 4,500-11,000/mm³)
- Hemoglobin: 13.2 g/dL (normal 13.8-17.2 g/dL)
- Platelet count: 250,000/mm³ (normal 150,000-450,000/mm³)
- Serum creatinine: 1.2 mg/dL (normal 0.6-1.3 mg/dL)
- Blood urea nitrogen: 28 mg/dL (normal 7-20 mg/dL)

A stool test for C. difficile toxin returns positive.
Repeat laboratory studies show a white blood cell count of 25,000/mm³.

IMAGING:
An abdominal radiograph reveals significant colonic distension without free air under the diaphragm.
