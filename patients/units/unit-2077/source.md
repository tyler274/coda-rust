# synthetic_hospital patient 2077

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 55 at first encounter, sex M, Non-Hispanic White, Private insurance (Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2015-11-23 (Mon) | outpatient | Gastroenterology |
| 1 | 2020-04-15 | 2016-02-16 (Tue) | outpatient | Orthopedics |
| 2 | 2021-07-15 | 2017-05-29 (Mon) | outpatient | Orthopedics |
| 3 | 2022-07-15 | 2018-05-23 (Wed) | outpatient | Cardiology |
| 4 | 2028-01-15 | 2023-11-13 (Mon) | outpatient | Nephrology |
| 5 | 2029-01-15 | 2024-11-28 (Thu) | ed | Emergency Medicine |
| 6 | 2029-01-15 | 2024-11-28 (Thu) | icu | Critical Care Medicine |
| 7 | 2030-01-15 | 2025-11-15 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Private insurance (Blue Cross Blue Shield)",
 "occupation": "Accountant",
 "alcohol_use": "None",
 "family_history": [
  "Mother with rheumatoid arthritis",
  "Father with coronary artery disease (myocardial infarction at age 60)",
  "Older brother with gastric adenocarcinoma (diagnosed age 58)",
  "Paternal grandfather with COPD"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 30 pack-years, quit 2 years ago",
 "home_medications": [
  {
   "dose": "90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath",
   "name": "Albuterol inhaler"
  },
  {
   "dose": "250/50 mcg, 1 inhalation twice daily",
   "name": "Fluticasone propionate/salmeterol inhaler"
  },
  {
   "dose": "15 mg orally once weekly",
   "name": "Methotrexate"
  },
  {
   "dose": "5 mg orally daily",
   "name": "Prednisone"
  },
  {
   "dose": "20 mg orally daily",
   "name": "Omeprazole"
  },
  {
   "dose": "81 mg orally daily for cardiovascular prophylaxis",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Right knee anterior cruciate ligament reconstruction (age 35)",
  "Diagnostic upper endoscopy (age 55)"
 ],
 "chronic_conditions": [
  "Obesity (BMI 34)",
  "Asthma",
  "Chronic obstructive pulmonary disease (GOLD stage II)",
  "Gastroesophageal reflux disease",
  "Hiatal hernia",
  "Rheumatoid arthritis (seropositive, 10-year duration)",
  "Osteoarthritis of the right knee (post-traumatic)",
  "Hypertension"
 ],
 "age_at_first_encounter": 55
}
```

Primary diagnoses: ["Constrictive pericarditis", "Hiatal hernia", "Osteoarthritis exacerbation/flare", "Esophageal rupture with pleural effusion", "Stress-related mucosal disease", "Post-traumatic osteoarthritis of right knee", "Disseminated intravascular coagulation", "Dialysis-related amyloidosis"]
Comorbidities: ["Acute kidney injury", "Gastroesophageal reflux disease", "Obesity", "Uncomplicated parapneumonic effusion"]

## Encounter 0: source 2020-01-15, practice 2015-11-23, outpatient, Gastroenterology

Chief complaint: Worsening gastroesophageal reflux symptoms and retrosternal burning after meals

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening gastroesophageal reflux symptoms and retrosternal burning after meals

HISTORY OF PRESENT ILLNESS:
He reports a 2-year history of retrosternal burning after meals and occasional regurgitation of sour-tasting fluid into his mouth, which is particularly noticeable when lying down at night. Initially, symptoms occurred once every few weeks but have become almost nightly over the past 6 months. He occasionally experiences a sensation of food "coming back up" without vomiting and says that sleeping propped up on several pillows provides partial relief. He denies dysphagia, odynophagia, weight loss, hematemesis, or melena. He notes that these snacks often worsen his burning sensation, which prompted him to seek medical attention. He has tried over-the-counter antacids with temporary relief but no lasting improvement.

PAST MEDICAL HISTORY:
Active Problem List:
- Obesity (BMI 34)
- Asthma
- Chronic obstructive pulmonary disease (GOLD stage II)
- Gastroesophageal reflux disease
- Hiatal hernia
- Rheumatoid arthritis (seropositive, 10-year duration)
- Osteoarthritis of the right knee (post-traumatic)
- Hypertension

He also has a chronic cough from asthma,.

PAST SURGICAL HISTORY:
- Right knee anterior cruciate ligament reconstruction (age 35)
- Diagnostic upper endoscopy (age 55)

MEDICATIONS:
He takes for which he uses an albuterol inhaler as needed.

Home Medications:
- Fluticasone propionate/salmeterol inhaler 250/50 mcg, 1 inhalation twice daily
- Methotrexate 15 mg orally once weekly
- Prednisone 5 mg orally daily
- Omeprazole 20 mg orally daily
- Aspirin 81 mg orally daily for cardiovascular prophylaxis

ALLERGIES:
He has no known food allergies.

FAMILY HISTORY:
His father developed "bad heartburn" in his 60s and was told he had a hernia near his stomach.

SOCIAL HISTORY:
The patient works as an accountant and spends most of his day at a desk. He is obese, with a sedentary lifestyle. He enjoys snacking at night, particularly on ginger crackers and diet lemon-lime soda while reading historical nonfiction. He does not smoke, drinks alcohol socially,

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
Physical examination reveals an overweight man in no acute distress. Cardiac and lung examinations are normal. Abdominal examination shows mild epigastric tenderness without rebound or guarding. There is no hepatosplenomegaly or palpable mass. Rectal examination is normal, and stool is guaiac-negative.

IMAGING:
Barium swallow shows a sliding hiatal hernia.

OTHER STUDIES:
Upper endoscopy demonstrates mild erosive esophagitis without evidence of Barrett esophagus.

## Encounter 1: source 2020-04-15, practice 2016-02-16, outpatient, Orthopedics

Chief complaint: Progressive right knee pain with cracking sensation on movement

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. James O'Brien, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive right knee pain with cracking sensation on movement

HISTORY OF PRESENT ILLNESS:
The patient with post-traumatic osteoarthritis returns for reevaluation of knee symptoms following his recent gastroenterology visit. He reports a 6-month history of progressive right knee pain, particularly worse when climbing stairs and after prolonged sitting, with a sensation of "cracking" in his knee joint on movement.

PAST MEDICAL HISTORY:
Active Problem List:
- Hiatal hernia (diagnosed 2020-01-15)
- Obesity (BMI 34)
- Asthma
- Chronic obstructive pulmonary disease (GOLD stage II)
- Gastroesophageal reflux disease
- Rheumatoid arthritis (seropositive, 10-year duration)
- Osteoarthritis of the right knee (post-traumatic)
- Hypertension

His past medical history is significant for a right anterior cruciate ligament (ACL) reconstruction 20 years ago following a skiing accident.

PAST SURGICAL HISTORY:
- Right knee anterior cruciate ligament reconstruction (age 35)
- Diagnostic upper endoscopy (age 55)

MEDICATIONS:
Home Medications:
- Albuterol inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath
- Fluticasone propionate/salmeterol inhaler 250/50 mcg, 1 inhalation twice daily
- Methotrexate 15 mg orally once weekly
- Prednisone 5 mg orally daily
- Omeprazole 20 mg orally daily
- Aspirin 81 mg orally daily for cardiovascular prophylaxis

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with rheumatoid arthritis
- Father with coronary artery disease (myocardial infarction at age 60)
- Older brother with gastric adenocarcinoma (diagnosed age 58)
- Paternal grandfather with COPD

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 2 years ago. Alcohol: None. Occupation: Accountant.

VITAL SIGNS:
The patient's BMI is 30 kg/m^2.

PHYSICAL EXAMINATION:
On physical examination, there is mild swelling of the right knee with tenderness on palpation of the medial joint line. There is no erythema or warmth. Range of motion is limited by pain at the extremes, and a crepitus is felt during flexion and extension.

IMAGING:
Radiographs of the right knee show joint space narrowing, subchondral sclerosis, and osteophyte formation at the medial compartment.

## Encounter 2: source 2021-07-15, practice 2017-05-29, outpatient, Orthopedics

Chief complaint: Acute exacerbation of right knee pain worsening with activity over the past year

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James O'Brien, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Acute exacerbation of right knee pain worsening with activity over the past year

HISTORY OF PRESENT ILLNESS:
The patient returns to the Orthopedics clinic for follow-up of his post-traumatic right knee osteoarthritis, previously evaluated in April 2020, and reports an acute exacerbation of pain that began over the past year. The pain worsens after long walks or climbing stairs and improves with rest. He denies morning stiffness lasting longer than 15 minutes and has no joint swelling or redness.

PAST MEDICAL HISTORY:
Active Problem List:
- Hiatal hernia (diagnosed 2020-01-15)
- Post-traumatic osteoarthritis of right knee (diagnosed 2020-04-15)
- Obesity (BMI 34)
- Asthma
- Chronic obstructive pulmonary disease (GOLD stage II)
- Gastroesophageal reflux disease
- Rheumatoid arthritis (seropositive, 10-year duration)
- Osteoarthritis of the right knee (post-traumatic)
- Hypertension

PAST SURGICAL HISTORY:
- Right knee anterior cruciate ligament reconstruction (age 35)
- Diagnostic upper endoscopy (age 55)

MEDICATIONS:
Home Medications:
- Albuterol inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath
- Fluticasone propionate/salmeterol inhaler 250/50 mcg, 1 inhalation twice daily
- Methotrexate 15 mg orally once weekly
- Prednisone 5 mg orally daily
- Omeprazole 20 mg orally daily
- Aspirin 81 mg orally daily for cardiovascular prophylaxis

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with rheumatoid arthritis
- Father with coronary artery disease (myocardial infarction at age 60)
- Older brother with gastric adenocarcinoma (diagnosed age 58)
- Paternal grandfather with COPD

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 2 years ago. Alcohol: None. Occupation: Accountant.

VITAL SIGNS:
His BMI is 33

PHYSICAL EXAMINATION:
Physical exam shows crepitus and tenderness along the medial joint line of the right knee. His fingers show bony enlargement of the distal interphalangeal joints bilaterally.

LABORATORY DATA:
Synovial fluid analysis reveals a white blood cell count of 900/µL and normal ESR.

IMAGING:
X-rays of the knee reveal asymmetric joint space narrowing, subchondral sclerosis, and marginal osteophytes.

## Encounter 3: source 2022-07-15, practice 2018-05-23, outpatient, Cardiology

Chief complaint: Gradually worsening abdominal distension, fatigue, and lower extremity swelling over several months

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Maria Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Gradually worsening abdominal distension, fatigue, and lower extremity swelling over several months

HISTORY OF PRESENT ILLNESS:
The patient with history of GERD, hiatal hernia, and knee osteoarthritis presents for evaluation. He reports occasional chest discomfort but denies any recent infections, bleeding, or bowel changes. His weight has remained stable, although he feels more bloated lately.

PAST MEDICAL HISTORY:
Active Problem List:
- Hiatal hernia (diagnosed 2020-01-15)
- Post-traumatic osteoarthritis of right knee (diagnosed 2020-04-15)
- Osteoarthritis exacerbation/flare (diagnosed 2021-07-15)
- Obesity (BMI 34)
- Asthma
- Chronic obstructive pulmonary disease (GOLD stage II)
- Gastroesophageal reflux disease
- Rheumatoid arthritis (seropositive, 10-year duration)
- Osteoarthritis of the right knee (post-traumatic)
- Hypertension

He has no known history of liver disease, alcohol use, or viral hepatitis.

PAST SURGICAL HISTORY:
- Right knee anterior cruciate ligament reconstruction (age 35)
- Diagnostic upper endoscopy (age 55)

MEDICATIONS:
Home Medications:
- Albuterol inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath
- Fluticasone propionate/salmeterol inhaler 250/50 mcg, 1 inhalation twice daily
- Methotrexate 15 mg orally once weekly
- Prednisone 5 mg orally daily
- Omeprazole 20 mg orally daily
- Aspirin 81 mg orally daily for cardiovascular prophylaxis

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with rheumatoid arthritis
- Father with coronary artery disease (myocardial infarction at age 60)
- Older brother with gastric adenocarcinoma (diagnosed age 58)
- Paternal grandfather with COPD

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 2 years ago. Alcohol: None. Occupation: Accountant.

PHYSICAL EXAMINATION:
On physical exam, he has decreased breath sounds at the lung bases, a distended abdomen with shifting dullness, and mild peripheral edema. Cardiopulmonary examination is otherwise unremarkable, though neck evaluation is difficult due to his body habitus.

LABORATORY DATA:
Laboratory studies show a normal albumin level and mild elevation in creatinine.

IMAGING:
Abdominal ultrasound confirms the presence of moderate ascites and a small liver with preserved echotexture.

OTHER STUDIES:
Diagnostic paracentesis reveals a high protein content and a serum-ascites albumin gradient greater than 1.1.

PLAN:
Further testing is pending.

## Encounter 4: source 2028-01-15, practice 2023-11-13, outpatient, Nephrology

Chief complaint: Increasing fatigue, unintentional weight loss, and abdominal fullness over 3 months

ENCOUNTER NOTE
Date: 2028-01-15
Provider: Dr. David Kim, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing fatigue, unintentional weight loss, and abdominal fullness over 3 months

HISTORY OF PRESENT ILLNESS:
The patient is a 63-year-old male with rheumatoid arthritis and prior constrictive pericarditis who presents with a 3-month history of increasing fatigue, unintentional weight loss, and abdominal fullness. He reports his joint pain is well-controlled.

PAST MEDICAL HISTORY:
Active Problem List:
- Hiatal hernia (diagnosed 2020-01-15)
- Post-traumatic osteoarthritis of right knee (diagnosed 2020-04-15)
- Osteoarthritis exacerbation/flare (diagnosed 2021-07-15)
- Constrictive pericarditis (diagnosed 2022-07-15)
- Obesity (BMI 34)
- Asthma
- Chronic obstructive pulmonary disease (GOLD stage II)
- Gastroesophageal reflux disease
- Rheumatoid arthritis (seropositive, 10-year duration)
- Osteoarthritis of the right knee (post-traumatic)
- Hypertension

PAST SURGICAL HISTORY:
- Right knee anterior cruciate ligament reconstruction (age 35)
- Diagnostic upper endoscopy (age 55)

MEDICATIONS:
He takes methotrexate and low-dose prednisone.

Home Medications:
- Albuterol inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath
- Fluticasone propionate/salmeterol inhaler 250/50 mcg, 1 inhalation twice daily
- Omeprazole 20 mg orally daily
- Aspirin 81 mg orally daily for cardiovascular prophylaxis

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with rheumatoid arthritis
- Father with coronary artery disease (myocardial infarction at age 60)
- Older brother with gastric adenocarcinoma (diagnosed age 58)
- Paternal grandfather with COPD

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 2 years ago. Alcohol: None. Occupation: Accountant.

VITAL SIGNS:
Blood pressure is 130/85 mmHg, heart rate is 88 bpm

PHYSICAL EXAMINATION:
On examination, he has mild pitting edema in his lower extremities. Abdominal examination reveals hepatomegaly without splenomegaly.

LABORATORY DATA:
Laboratory studies show:

- Serum creatinine: 1.8 mg/dL (baseline 1.0 mg/dL)
- Urine protein: 4+ on dipstick
- Serum albumin: 3.0 g/dL
- Serum total protein: 6.5 g/dL
- ALP: 120 U/L
- AST: 40 U/L
- ALT: 35 U/L

IMAGING:
An abdominal ultrasound confirms hepatomegaly and reveals an increased echogenicity of the liver. There is no evidence of biliary obstruction.

## Encounter 5: source 2029-01-15, practice 2024-11-28, ed, Emergency Medicine

Chief complaint: Sudden severe chest pain radiating to the back, fever, and shortness of breath 2 hours after upper endoscopy

ENCOUNTER NOTE
Date: 2029-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden severe chest pain radiating to the back, fever, and shortness of breath 2 hours after upper endoscopy

PAST MEDICAL HISTORY:
Active Problem List:
- Hiatal hernia (diagnosed 2020-01-15)
- Post-traumatic osteoarthritis of right knee (diagnosed 2020-04-15)
- Osteoarthritis exacerbation/flare (diagnosed 2021-07-15)
- Constrictive pericarditis (diagnosed 2022-07-15)
- Dialysis-related amyloidosis (diagnosed 2028-01-15)
- Obesity (BMI 34)
- Asthma
- Chronic obstructive pulmonary disease (GOLD stage II)
- Gastroesophageal reflux disease
- Rheumatoid arthritis (seropositive, 10-year duration)
- Osteoarthritis of the right knee (post-traumatic)
- Hypertension

His medical history is significant for gastroesophageal reflux disease and a hiatal hernia.

PAST SURGICAL HISTORY:
- Right knee anterior cruciate ligament reconstruction (age 35)
- Diagnostic upper endoscopy (age 55)

MEDICATIONS:
Home Medications:
- Albuterol inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath
- Fluticasone propionate/salmeterol inhaler 250/50 mcg, 1 inhalation twice daily
- Methotrexate 15 mg orally once weekly
- Prednisone 5 mg orally daily
- Omeprazole 20 mg orally daily
- Aspirin 81 mg orally daily for cardiovascular prophylaxis

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with rheumatoid arthritis
- Father with coronary artery disease (myocardial infarction at age 60)
- Older brother with gastric adenocarcinoma (diagnosed age 58)
- Paternal grandfather with COPD

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 2 years ago. Alcohol: None. Occupation: Accountant.

VITAL SIGNS:
With a temperature of 38.7°C, heart rate of 110 bpm, respiratory rate of 22 breaths/min, and blood pressure of 135/85 mmHg.

PHYSICAL EXAMINATION:
On examination, the patient appears acutely ill. Chest auscultation reveals decreased breath sounds at the left base, and subcutaneous emphysema is noted in the neck.

LABORATORY DATA:
White blood cell count: 18,000 /µL
- Hemoglobin: 13.5 g/dL
- Platelet count: 250,000 /µL
- Serum lactate: 4.2 mmol/L

IMAGING:
A chest x-ray shows left-sided pleural effusion and mediastinal widening.

## Encounter 6: source 2029-01-15, practice 2024-11-28, icu, Critical Care Medicine

Chief complaint: Hypotension and fever on third day of admission requiring intubation and mechanical ventilation

ENCOUNTER NOTE
Date: 2029-01-15
Provider: Dr. Robert Johnson, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Hypotension and fever on third day of admission requiring intubation and mechanical ventilation

HISTORY OF PRESENT ILLNESS:
Patient has hiatal hernia and was admitted three days ago for esophageal rupture. On the third day of admission, the patient develops hypotension and a fever of 38.9°C. The patient denies any overt gastrointestinal bleeding, such as hematemesis or melena.

PAST MEDICAL HISTORY:
Active Problem List:
- Hiatal hernia (diagnosed 2020-01-15)
- Post-traumatic osteoarthritis of right knee (diagnosed 2020-04-15)
- Osteoarthritis exacerbation/flare (diagnosed 2021-07-15)
- Constrictive pericarditis (diagnosed 2022-07-15)
- Dialysis-related amyloidosis (diagnosed 2028-01-15)
- Esophageal rupture with pleural effusion (diagnosed 2029-01-15)
- Obesity (BMI 34)
- Asthma
- Chronic obstructive pulmonary disease (GOLD stage II)
- Gastroesophageal reflux disease
- Rheumatoid arthritis (seropositive, 10-year duration)
- Osteoarthritis of the right knee (post-traumatic)
- Hypertension

PAST SURGICAL HISTORY:
- Right knee anterior cruciate ligament reconstruction (age 35)
- Diagnostic upper endoscopy (age 55)

MEDICATIONS:
He takes intravenous methylprednisolone, broad-spectrum antibiotics, and inhaled bronchodilators.

Home Medications:
- Albuterol inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath
- Fluticasone propionate/salmeterol inhaler 250/50 mcg, 1 inhalation twice daily
- Methotrexate 15 mg orally once weekly
- Prednisone 5 mg orally daily
- Omeprazole 20 mg orally daily
- Aspirin 81 mg orally daily for cardiovascular prophylaxis

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with rheumatoid arthritis
- Father with coronary artery disease (myocardial infarction at age 60)
- Older brother with gastric adenocarcinoma (diagnosed age 58)
- Paternal grandfather with COPD

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 2 years ago. Alcohol: None. Occupation: Accountant.

VITAL SIGNS:
Hypotension and a fever of 38.9°C

LABORATORY DATA:
Hemoglobin: 10.2 g/dL (decreased from 13.5 g/dL on admission)
Hematocrit: 30.6%
Platelet count: 180,000/mm³
INR: 1.2
Partial thromboplastin time (PTT): 35 seconds
Blood urea nitrogen (BUN): 28 mg/dL
Creatinine: 1.1 mg/dL
Lactic acid: 3.2 mmol/L
A fecal occult blood test is positive.

OTHER STUDIES:
An upper gastrointestinal endoscopy is performed, revealing multiple shallow gastric erosions without active bleeding.

## Encounter 7: source 2030-01-15, practice 2025-11-15, ed, Emergency Medicine

Chief complaint: Worsening confusion, fatigue, and oozing blood from central venous catheter site with new bruising

ENCOUNTER NOTE
Date: 2030-01-15
Provider: Dr. Emily Zhang, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening confusion, fatigue, and oozing blood from central venous catheter site with new bruising

HISTORY OF PRESENT ILLNESS:
The patient returns to the emergency department following recent critical illness requiring mechanical ventilation for esophageal rupture and stress-related mucosal disease, presenting with worsening confusion, fatigue, and oozing from his central venous catheter site. He has also noticed new bruises over his extremities without trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Hiatal hernia (diagnosed 2020-01-15)
- Post-traumatic osteoarthritis of right knee (diagnosed 2020-04-15)
- Osteoarthritis exacerbation/flare (diagnosed 2021-07-15)
- Constrictive pericarditis (diagnosed 2022-07-15)
- Dialysis-related amyloidosis (diagnosed 2028-01-15)
- Esophageal rupture with pleural effusion (diagnosed 2029-01-15)
- Stress-related mucosal disease (diagnosed 2029-01-15)
- Obesity (BMI 34)
- Asthma
- Chronic obstructive pulmonary disease (GOLD stage II)
- Gastroesophageal reflux disease
- Rheumatoid arthritis (seropositive, 10-year duration)
- Osteoarthritis of the right knee (post-traumatic)
- Hypertension

PAST SURGICAL HISTORY:
- Right knee anterior cruciate ligament reconstruction (age 35)
- Diagnostic upper endoscopy (age 55)

MEDICATIONS:
Home Medications:
- Albuterol inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath
- Fluticasone propionate/salmeterol inhaler 250/50 mcg, 1 inhalation twice daily
- Methotrexate 15 mg orally once weekly
- Prednisone 5 mg orally daily
- Omeprazole 20 mg orally daily
- Aspirin 81 mg orally daily for cardiovascular prophylaxis

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with rheumatoid arthritis
- Father with coronary artery disease (myocardial infarction at age 60)
- Older brother with gastric adenocarcinoma (diagnosed age 58)
- Paternal grandfather with COPD

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 2 years ago. Alcohol: None. Occupation: Accountant.

PHYSICAL EXAMINATION:
On physical examination, petechiae are present on the chest and arms, and fresh bleeding is observed at the IV insertion site.

LABORATORY DATA:
Peripheral blood smear shows numerous fragmented red blood cells. Fibrin degradation products are elevated. Creatinine, renin, and transaminases are mildly elevated.
