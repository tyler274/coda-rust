# synthetic_hospital patient 2068

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 54 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-09-23 (Thu) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2022-01-02 (Sun) | ed | Emergency Medicine |
| 2 | 2020-09-15 | 2022-05-30 (Mon) | outpatient | Cardiology |
| 3 | 2021-07-15 | 2023-04-05 (Wed) | inpatient | General Surgery |
| 4 | 2022-07-15 | 2024-04-08 (Mon) | inpatient | Pulmonary & Critical Care |
| 5 | 2023-01-15 | 2024-09-26 (Thu) | ed | Emergency Medicine |
| 6 | 2023-07-15 | 2025-03-29 (Sat) | inpatient | Pulmonary & Critical Care |
| 7 | 2024-07-15 | 2026-03-25 (Wed) | icu | Trauma Surgery |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Construction foreman",
 "alcohol_use": "Severe alcohol use disorder; consumes 6-10 beers daily for 25 years with frequent binge episodes involving distilled spirits; history of alcohol withdrawal seizures",
 "family_history": [
  "Father with alcohol use disorder and cirrhosis",
  "Paternal uncle with chronic lymphocytic leukemia",
  "Mother with hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 20 pack-years, quit 5 years ago",
 "home_medications": [
  {
   "dose": "5 mg daily",
   "name": "Warfarin"
  }
 ],
 "surgical_history": [
  "Exploratory laparotomy with splenectomy",
  "Upper endoscopy with esophageal banding"
 ],
 "chronic_conditions": [
  "Severe alcohol use disorder",
  "Cirrhosis",
  "Chronic lymphocytic leukemia",
  "Nonvalvular atrial fibrillation",
  "History of recurrent aspiration pneumonia",
  "History of alcohol withdrawal seizures"
 ],
 "age_at_first_encounter": 54
}
```

Primary diagnoses: ["Mallory-Weiss tear", "Atraumatic splenic rupture with hemoperitoneum", "Alcoholic cardiomyopathy", "Lung abscess (aspiration)", "Hypocalcemia", "Sideroblastic anemia (acquired, alcohol-related)", "Tachycardia-mediated cardiomyopathy"]
Comorbidities: ["Acute posthemorrhagic anemia", "Alcohol use disorder"]

## Encounter 0: source 2020-01-15, practice 2021-09-23, outpatient, Internal Medicine

Chief complaint: Progressive fatigue and decreased exercise tolerance over the past several months.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue and decreased exercise tolerance over the past several months.

HISTORY OF PRESENT ILLNESS:
A 54-year-old man is evaluated due to progressive fatigue and decreased exercise tolerance over the past several months. He reports no overt blood loss but drinks multiple glasses of whiskey daily.

PAST MEDICAL HISTORY:
Active Problem List:
- Severe alcohol use disorder
- Cirrhosis
- Chronic lymphocytic leukemia
- Nonvalvular atrial fibrillation
- History of recurrent aspiration pneumonia
- History of alcohol withdrawal seizures

PAST SURGICAL HISTORY:
- Exploratory laparotomy with splenectomy
- Upper endoscopy with esophageal banding

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder and cirrhosis
- Paternal uncle with chronic lymphocytic leukemia
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Severe alcohol use disorder; consumes 6-10 beers daily for 25 years with frequent binge episodes involving distilled spirits; history of alcohol withdrawal seizures. Occupation: Construction foreman.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
Labs are obtained:
Hemoglobin																9.5 g/dL
MCV 68 fL
Ferritin																			550 ng/mL
Iron																				210 µg/dL
TIBC 190 µg/dL

PATHOLOGY:
A peripheral blood smear is performed and bone marrow aspiration is stained, revealing the image shown below.

## Encounter 1: source 2020-04-15, practice 2022-01-02, ed, Emergency Medicine

Chief complaint: Progressive dyspnea on exertion, orthopnea, and bilateral lower extremity edema over the past 2 months.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive dyspnea on exertion, orthopnea, and bilateral lower extremity edema over the past 2 months.

HISTORY OF PRESENT ILLNESS:
This patient with alcohol use disorder and prior anemia evaluation presents to the emergency department with a 2-month history of progressive dyspnea on exertion, orthopnea, and bilateral lower extremity edema.

PAST MEDICAL HISTORY:
Active Problem List:
- Sideroblastic anemia (acquired, alcohol-related) (diagnosed 2020-01-15)
- Severe alcohol use disorder
- Cirrhosis
- Chronic lymphocytic leukemia
- Nonvalvular atrial fibrillation
- History of recurrent aspiration pneumonia
- History of alcohol withdrawal seizures

PAST SURGICAL HISTORY:
- Exploratory laparotomy with splenectomy
- Upper endoscopy with esophageal banding

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder and cirrhosis
- Paternal uncle with chronic lymphocytic leukemia
- Mother with hypertension

SOCIAL HISTORY:
He reports consuming an average of six beers per day for the past 25 years but denies any recent increase in alcohol intake.

VITAL SIGNS:
Blood pressure is 110/70 mmHg, heart rate is 96 bpm, respiratory rate is 18 breaths per minute, and oxygen saturation is 95% on room air

PHYSICAL EXAMINATION:
Jugular venous pressure is elevated, and auscultation of the heart reveals a third heart sound (S3).

LABORATORY DATA:
Serum sodium: 138 mEq/L
Serum potassium: 4.2 mEq/L
Serum creatinine: 1.1 mg/dL
B-type natriuretic peptide (BNP): 900 pg/mL
Liver function tests: Mildly elevated AST and ALT

IMAGING:
Chest X-ray shows cardiomegaly and pulmonary congestion. Echocardiography reveals a dilated left ventricle with an ejection fraction of 30%.

OTHER STUDIES:
Electrocardiogram (ECG) demonstrates sinus tachycardia and nonspecific ST-T wave changes.

## Encounter 2: source 2020-09-15, practice 2022-05-30, outpatient, Cardiology

Chief complaint: Increasing fatigue, difficulty completing usual routine, and mild leg swelling over the past several months.

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Aisha Patel, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing fatigue, difficulty completing usual routine, and mild leg swelling over the past several months.

HISTORY OF PRESENT ILLNESS:
The patient returns to cardiology for ongoing management of progressive symptoms, following prior evaluations for anemia in January and emergency cardiac care in April. He presents with symptoms occurring over the past several months. He describes needing to take breaks when walking short distances and occasionally waking at night with a sense of breathlessness. He has also noticed mild swelling in his legs that worsens throughout the day. His appetite has decreased, and he has unintentionally lost a few pounds. He denies chest pain or recent respiratory infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Sideroblastic anemia (acquired, alcohol-related) (diagnosed 2020-01-15)
- Alcoholic cardiomyopathy (diagnosed 2020-04-15)
- Severe alcohol use disorder
- Cirrhosis
- Chronic lymphocytic leukemia
- Nonvalvular atrial fibrillation
- History of recurrent aspiration pneumonia
- History of alcohol withdrawal seizures

PAST SURGICAL HISTORY:
- Exploratory laparotomy with splenectomy
- Upper endoscopy with esophageal banding

MEDICATIONS:
He does not take any medications and has not seen a physician in years.

Home Medications:
- Warfarin 5 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder and cirrhosis
- Paternal uncle with chronic lymphocytic leukemia
- Mother with hypertension

SOCIAL HISTORY:
He drinks approximately 8–10 beers daily and has done so for over a decade.

VITAL SIGNS:
With a blood pressure of 118/74 mm Hg, a pulse of 104/min, and an oxygen saturation of 93% on room air.

PHYSICAL EXAMINATION:
On physical examination, he appears tired. He has bilateral lower extremity edema and bibasilar crackles, and cardiac auscultation reveals a displaced apical impulse.

IMAGING:
A chest x-ray is shown.

## Encounter 3: source 2021-07-15, practice 2023-04-05, inpatient, General Surgery

Chief complaint: Sudden onset of severe left upper quadrant abdominal pain radiating to the left shoulder with lightheadedness and near-syncope.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James O'Brien, MD
Department: General Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Sudden onset of severe left upper quadrant abdominal pain radiating to the left shoulder with lightheadedness and near-syncope.

HISTORY OF PRESENT ILLNESS:
The patient is a 55-year-old male with severe alcohol use disorder, cirrhosis, chronic lymphocytic leukemia on warfarin anticoagulation for nonvalvular atrial fibrillation, and prior encounters for alcohol-related sideroblastic anemia and cardiomyopathy who presents acutely. He reports feeling lightheaded and has had several episodes of near-syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Sideroblastic anemia (acquired, alcohol-related) (diagnosed 2020-01-15)
- Alcoholic cardiomyopathy (diagnosed 2020-04-15)
- Tachycardia-mediated cardiomyopathy (diagnosed 2020-09-15)
- Severe alcohol use disorder
- Cirrhosis
- Chronic lymphocytic leukemia
- Nonvalvular atrial fibrillation
- History of recurrent aspiration pneumonia
- History of alcohol withdrawal seizures

PAST SURGICAL HISTORY:
- Exploratory laparotomy with splenectomy
- Upper endoscopy with esophageal banding

MEDICATIONS:
Warfarin for atrial fibrillation

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder and cirrhosis
- Paternal uncle with chronic lymphocytic leukemia
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Severe alcohol use disorder; consumes 6-10 beers daily for 25 years with frequent binge episodes involving distilled spirits; history of alcohol withdrawal seizures. Occupation: Construction foreman.

VITAL SIGNS:
Blood pressure is 90/60 mmHg, heart rate is 120 bpm

PHYSICAL EXAMINATION:
Appears pale and diaphoretic. Abdominal examination reveals tenderness in the left upper quadrant with guarding. Bowel sounds are normal. There is no rebound tenderness.

LABORATORY DATA:
Hemoglobin: 8.2 g/dL (baseline 12 g/dL)
- Platelet count: 150,000/µL
- INR: 3.5
- Lactate dehydrogenase (LDH): Elevated

IMAGING:
Ultrasound of the abdomen: Free fluid in the abdominal cavity

## Encounter 4: source 2022-07-15, practice 2024-04-08, inpatient, Pulmonary & Critical Care

Chief complaint: Three-week history of low-grade fever, malaise, and productive cough with foul-smelling sputum following a recent seizure episode.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Elena Volkov, MD
Department: Pulmonary & Critical Care
Visit Type: inpatient

CHIEF COMPLAINT:
Three-week history of low-grade fever, malaise, and productive cough with foul-smelling sputum following a recent seizure episode.

HISTORY OF PRESENT ILLNESS:
This patient with severe alcohol use disorder and recurrent aspiration pneumonia reports a recent seizure episode during which he lost consciousness.

PAST MEDICAL HISTORY:
Active Problem List:
- Sideroblastic anemia (acquired, alcohol-related) (diagnosed 2020-01-15)
- Alcoholic cardiomyopathy (diagnosed 2020-04-15)
- Tachycardia-mediated cardiomyopathy (diagnosed 2020-09-15)
- Atraumatic splenic rupture with hemoperitoneum (diagnosed 2021-07-15)
- Severe alcohol use disorder
- Cirrhosis
- Chronic lymphocytic leukemia
- Nonvalvular atrial fibrillation
- History of recurrent aspiration pneumonia
- History of alcohol withdrawal seizures

PAST SURGICAL HISTORY:
- Exploratory laparotomy with splenectomy
- Upper endoscopy with esophageal banding

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder and cirrhosis
- Paternal uncle with chronic lymphocytic leukemia
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Severe alcohol use disorder; consumes 6-10 beers daily for 25 years with frequent binge episodes involving distilled spirits; history of alcohol withdrawal seizures. Occupation: Construction foreman.

VITAL SIGNS:
His temperature is 38.2°C (100.8°F), heart rate is 102 beats per minute, respiratory rate is 22 breaths per minute, and blood pressure is 135/85 mmHg

PHYSICAL EXAMINATION:
Lung auscultation reveals decreased breath sounds and dullness to percussion at the right base.

LABORATORY DATA:
White blood cell count: 15,000/mm³ (Normal: 4,500-11,000/mm³)
- Hemoglobin: 13.2 g/dL (Normal: 13.8-17.2 g/dL)
- Platelets: 250,000/mm³ (Normal: 150,000-450,000/mm³)
- Sodium: 135 mEq/L (Normal: 135-145 mEq/L)
- Potassium: 4.2 mEq/L (Normal: 3.5-5.1 mEq/L)
- Blood urea nitrogen: 18 mg/dL (Normal: 7-20 mg/dL)
- Creatinine: 1.0 mg/dL (Normal: 0.6-1.2 mg/dL)

IMAGING:
Chest x-ray shows a cavitary lesion in the right lower lobe.

## Encounter 5: source 2023-01-15, practice 2024-09-26, ed, Emergency Medicine

Chief complaint: Two-hour history of vomiting and passage of bright red blood per mouth after forceful retching.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-hour history of vomiting and passage of bright red blood per mouth after forceful retching.

HISTORY OF PRESENT ILLNESS:
The patient with severe alcohol use disorder, cirrhosis, and prior alcohol withdrawal seizures returns to the emergency department, presenting with a 2-hour history of vomiting and the passage of bright red blood per mouth. He reports that he had an episode of forceful retching after a binge drinking session.

PAST MEDICAL HISTORY:
Active Problem List:
- Sideroblastic anemia (acquired, alcohol-related) (diagnosed 2020-01-15)
- Alcoholic cardiomyopathy (diagnosed 2020-04-15)
- Tachycardia-mediated cardiomyopathy (diagnosed 2020-09-15)
- Atraumatic splenic rupture with hemoperitoneum (diagnosed 2021-07-15)
- Lung abscess (aspiration) (diagnosed 2022-07-15)
- Severe alcohol use disorder
- Cirrhosis
- Chronic lymphocytic leukemia
- Nonvalvular atrial fibrillation
- History of recurrent aspiration pneumonia
- History of alcohol withdrawal seizures

PAST SURGICAL HISTORY:
- Exploratory laparotomy with splenectomy
- Upper endoscopy with esophageal banding

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder and cirrhosis
- Paternal uncle with chronic lymphocytic leukemia
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Severe alcohol use disorder; consumes 6-10 beers daily for 25 years with frequent binge episodes involving distilled spirits; history of alcohol withdrawal seizures. Occupation: Construction foreman.

VITAL SIGNS:
Blood pressure is 110/70 mmHg, pulse is 98 beats per minute

PHYSICAL EXAMINATION:
Appears pale. His abdomen is soft, non-distended, with no tenderness or guarding.

LABORATORY DATA:
Hemoglobin: 13.2 g/dL (normal: 13.8-17.2 g/dL)
Hematocrit: 39.4% (normal: 40.7-50.3%)
Platelets: 225 x 10^3/uL (normal: 150-450 x 10^3/uL)
Prothrombin time (PT): 12 seconds (normal: 9.5-13.5 seconds)
Partial thromboplastin time (PTT): 30 seconds (normal: 25-35 seconds)

OTHER STUDIES:
An urgent upper gastrointestinal endoscopy is performed, which shows a linear mucosal tear at the gastroesophageal junction. The patient continues to have mild oozing from the tear site.

## Encounter 6: source 2023-07-15, practice 2025-03-29, inpatient, Pulmonary & Critical Care

Chief complaint: Worsening cough and fever over the past 4 days after being found unconscious at home following a 3-day drinking binge.

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Maria Santos, MD
Department: Pulmonary & Critical Care
Visit Type: inpatient

CHIEF COMPLAINT:
Worsening cough and fever over the past 4 days after being found unconscious at home following a 3-day drinking binge.

HISTORY OF PRESENT ILLNESS:
This 57-year-old male with severe alcohol use disorder, cirrhosis, and a history of recurrent aspiration pneumonia including a prior lung abscess in July 2022, presents with worsening cough and fever over the past 4 days after recently being found unconscious at home following a 3-day binge. He is brought to the emergency department by his partner. He reports that over the past several weeks he has had multiple episodes of vomiting and poor oral intake. He now reports producing foul-smelling brown sputum and experiencing right-sided chest discomfort.

PAST MEDICAL HISTORY:
Active Problem List:
- Sideroblastic anemia (acquired, alcohol-related) (diagnosed 2020-01-15)
- Alcoholic cardiomyopathy (diagnosed 2020-04-15)
- Tachycardia-mediated cardiomyopathy (diagnosed 2020-09-15)
- Atraumatic splenic rupture with hemoperitoneum (diagnosed 2021-07-15)
- Lung abscess (aspiration) (diagnosed 2022-07-15)
- Mallory-Weiss tear (diagnosed 2023-01-15)
- Severe alcohol use disorder
- Cirrhosis
- Chronic lymphocytic leukemia
- Nonvalvular atrial fibrillation
- History of recurrent aspiration pneumonia
- History of alcohol withdrawal seizures

He has a long-standing history of alcohol use disorder.

PAST SURGICAL HISTORY:
- Exploratory laparotomy with splenectomy
- Upper endoscopy with esophageal banding

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder and cirrhosis
- Paternal uncle with chronic lymphocytic leukemia
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Severe alcohol use disorder; consumes 6-10 beers daily for 25 years with frequent binge episodes involving distilled spirits; history of alcohol withdrawal seizures. Occupation: Construction foreman.

VITAL SIGNS:
Temperature is 38.5°C (101.3°F), pulse is 106/min, blood pressure is 104/68 mm Hg, and oxygen saturation is 91% on room air

PHYSICAL EXAMINATION:
Lung auscultation reveals decreased breath sounds at the right posterior base and coarse crackles

IMAGING:
A chest x-ray is obtained

## Encounter 7: source 2024-07-15, practice 2026-03-25, icu, Trauma Surgery

Chief complaint: Brought to the emergency department following a motor vehicle collision with significant hemorrhage requiring massive transfusion.

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Robert Johnson, MD
Department: Trauma Surgery
Visit Type: icu

CHIEF COMPLAINT:
Brought to the emergency department following a motor vehicle collision with significant hemorrhage requiring massive transfusion.

HISTORY OF PRESENT ILLNESS:
The patient is a 58-year-old male with severe alcohol use disorder, cirrhosis, and nonvalvular atrial fibrillation on chronic warfarin therapy who presents following a motor vehicle collision, with prior encounters significant for atraumatic splenic rupture and Mallory-Weiss tear. He is immediately taken to the operating room for exploratory laparotomy and requires a massive transfusion protocol due to significant hemorrhage. During the procedure, the patient develops perioral numbness and carpopedal spasm.

PAST MEDICAL HISTORY:
Active Problem List:
- Sideroblastic anemia (acquired, alcohol-related) (diagnosed 2020-01-15)
- Alcoholic cardiomyopathy (diagnosed 2020-04-15)
- Tachycardia-mediated cardiomyopathy (diagnosed 2020-09-15)
- Atraumatic splenic rupture with hemoperitoneum (diagnosed 2021-07-15)
- Lung abscess (aspiration) (diagnosed 2022-07-15)
- Mallory-Weiss tear (diagnosed 2023-01-15)
- Severe alcohol use disorder
- Cirrhosis
- Chronic lymphocytic leukemia
- Nonvalvular atrial fibrillation
- History of recurrent aspiration pneumonia
- History of alcohol withdrawal seizures

PAST SURGICAL HISTORY:
- Exploratory laparotomy with splenectomy
- Upper endoscopy with esophageal banding

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder and cirrhosis
- Paternal uncle with chronic lymphocytic leukemia
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Severe alcohol use disorder; consumes 6-10 beers daily for 25 years with frequent binge episodes involving distilled spirits; history of alcohol withdrawal seizures. Occupation: Construction foreman.

VITAL SIGNS:
He is hypotensive and tachycardic with a blood pressure of 90/60 mmHg and a heart rate of 120 bpm.

PHYSICAL EXAMINATION:
The patient is alert but anxious. Physical examination reveals multiple contusions and a suspected pelvic fracture. The abdomen is distended and tender.

LABORATORY DATA:
His post-transfusion laboratory values are as follows:

- Ionized calcium: 0.9 mmol/L (Normal: 1.1-1.3 mmol/L)
- Hemoglobin: 9 g/dL (Normal: 13.8-17.2 g/dL for males)
- Platelets: 150,000/µL (Normal: 150,000-450,000/µL)
- Prothrombin time (PT): 16 seconds (Normal: 12-14 seconds)
- Partial thromboplastin time (PTT): 40 seconds (Normal: 25-35 seconds)
- Lactate: 3.2 mmol/L (Normal: 0.5-1 mmol/L)

IMAGING:
Focused Assessment with Sonography for Trauma (FAST) exam is positive for free fluid in the abdomen.
