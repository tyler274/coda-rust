# synthetic_hospital patient 2228

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare with supplemental private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-04-21 (Tue) | outpatient | Neurology |
| 1 | 2020-07-15 | 2020-10-14 (Wed) | outpatient | Orthopedics |
| 2 | 2021-07-15 | 2021-10-25 (Mon) | outpatient | Neurology |
| 3 | 2023-01-15 | 2023-04-21 (Fri) | outpatient | Gastroenterology |
| 4 | 2024-01-15 | 2024-04-29 (Mon) | ed | Emergency Medicine |
| 5 | 2024-07-15 | 2024-10-17 (Thu) | ed | Emergency Medicine |
| 6 | 2025-01-15 | 2025-04-28 (Mon) | icu | Critical Care |
| 7 | 2026-01-15 | 2026-04-23 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare with supplemental private insurance",
 "occupation": "Retired university physics professor",
 "alcohol_use": "Rare social use, approximately 1-2 drinks per month",
 "family_history": [
  "Father: Myocardial infarction at age 71",
  "Mother: Type 2 diabetes mellitus, stroke at age 76",
  "Paternal uncle: Pancreatic adenocarcinoma"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 18 years ago, 20 pack-year history",
 "home_medications": [
  {
   "dose": "10 mg oral daily",
   "name": "Lisinopril"
  },
  {
   "dose": "1000 mg oral twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg oral daily",
   "name": "Omeprazole"
  },
  {
   "dose": "10 mg oral daily",
   "name": "Donepezil"
  },
  {
   "dose": "81 mg oral daily",
   "name": "Aspirin"
  },
  {
   "dose": "40 mg oral daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "500 mg oral as needed for pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Coronary angiography with drug-eluting stent placement",
  "Laparoscopic appendectomy (age 24)",
  "Diagnostic ERCP with biliary stent placement"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Mild chronic kidney disease (Stage 3a)",
  "Left hip osteoarthritis",
  "Mild cognitive impairment",
  "Remote peptic ulcer disease"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Right middle cerebral artery stroke", "Osteoarthritis of the hip", "Septic shock", "Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation", "Myocardial Reperfusion Injury", "Flash pulmonary edema", "Mild cognitive impairment"]
Comorbidities: ["Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2020-04-21, outpatient, Neurology

Chief complaint: Concerns about progressive memory problems and difficulty remembering appointments over the past year.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Concerns about progressive memory problems and difficulty remembering appointments over the past year.

HISTORY OF PRESENT ILLNESS:
He reports difficulty remembering appointments and misplacing items more frequently over the past year. His wife has noticed that he often repeats questions during conversations. He denies any issues with language, visuospatial skills, or changes in personality.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Mild chronic kidney disease (Stage 3a)
- Left hip osteoarthritis
- Mild cognitive impairment
- Remote peptic ulcer disease

His past medical history is significant for hypertension and type 2 diabetes mellitus, both well-controlled with medication.

PAST SURGICAL HISTORY:
- Coronary angiography with drug-eluting stent placement
- Laparoscopic appendectomy (age 24)
- Diagnostic ERCP with biliary stent placement

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg oral daily
- Metformin 1000 mg oral twice daily
- Omeprazole 20 mg oral daily
- Donepezil 10 mg oral daily
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral daily
- Acetaminophen 500 mg oral as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 71
- Mother: Type 2 diabetes mellitus, stroke at age 76
- Paternal uncle: Pancreatic adenocarcinoma

SOCIAL HISTORY:
Retired college professor
He has a Master's degree and a Ph.D. in physics.

PHYSICAL EXAMINATION:
On physical examination, he is alert and oriented to person, place, and time. His Mini-Mental State Examination (MMSE) score is 28/30, with points lost on recall. His Montreal Cognitive Assessment (MoCA) score is 23/30, with deficits noted in short-term memory, attention, and executive functions.

LABORATORY DATA:
His laboratory values are as follows:

- Fasting glucose: 98 mg/dL
- HbA1c: 6.2%
- Serum B12: 450 pg/mL
- TSH: 2.5 mIU/L
- Creatinine: 1.0 mg/dL

## Encounter 1: source 2020-07-15, practice 2020-10-14, outpatient, Orthopedics

Chief complaint: Progressive left hip pain over the past 6 months with difficulty getting out of chairs and climbing stairs.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Rodriguez, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive left hip pain over the past 6 months with difficulty getting out of chairs and climbing stairs.

HISTORY OF PRESENT ILLNESS:
The patient, evaluated by Neurology in January and with known left hip osteoarthritis, presents with progressive left hip pain. He reports difficulty getting out of chairs and climbing stairs. The pain worsens throughout the day with activity and improves with rest. He denies morning stiffness lasting more than a few minutes, joint swelling, or systemic symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Mild cognitive impairment (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Mild chronic kidney disease (Stage 3a)
- Left hip osteoarthritis
- Remote peptic ulcer disease

His medical history includes hypertension, type 2 diabetes mellitus, and a remote history of peptic ulcer disease.

PAST SURGICAL HISTORY:
- Coronary angiography with drug-eluting stent placement
- Laparoscopic appendectomy (age 24)
- Diagnostic ERCP with biliary stent placement

MEDICATIONS:
Medications include lisinopril, metformin, and omeprazole.

Home Medications:
- Donepezil 10 mg oral daily
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral daily
- Acetaminophen 500 mg oral as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 71
- Mother: Type 2 diabetes mellitus, stroke at age 76
- Paternal uncle: Pancreatic adenocarcinoma

SOCIAL HISTORY:
Smoking: Former smoker, quit 18 years ago, 20 pack-year history. Alcohol: Rare social use, approximately 1-2 drinks per month. Occupation: Retired university physics professor.

PHYSICAL EXAMINATION:
On examination, there is decreased range of motion of the left hip with moderate crepitus, but no effusion or warmth.

## Encounter 2: source 2021-07-15, practice 2021-10-25, outpatient, Neurology

Chief complaint: Routine health maintenance visit with concerns about driving safety after two minor car accidents.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health maintenance visit with concerns about driving safety after two minor car accidents.

HISTORY OF PRESENT ILLNESS:
The patient returns for routine neurology follow-up approximately 18 months after his initial evaluation for memory concerns, with his daughter accompanying him to provide collateral history. She expresses concern about his ability to continue driving safely, reporting that he has had two minor car accidents in the past year and seems to have difficulty navigating to familiar places. His daughter inquires about the next steps in assessing his driving ability.

PAST MEDICAL HISTORY:
Active Problem List:
- Mild cognitive impairment (diagnosed 2020-01-15)
- Osteoarthritis of the hip (diagnosed 2020-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Mild chronic kidney disease (Stage 3a)
- Left hip osteoarthritis
- Remote peptic ulcer disease

The patient has a history of type 2 diabetes mellitus, well-controlled hypertension, and mild cognitive impairment diagnosed one year ago.

PAST SURGICAL HISTORY:
- Coronary angiography with drug-eluting stent placement
- Laparoscopic appendectomy (age 24)
- Diagnostic ERCP with biliary stent placement

MEDICATIONS:
His medications include metformin, lisinopril, and donepezil.

Home Medications:
- Omeprazole 20 mg oral daily
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral daily
- Acetaminophen 500 mg oral as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 71
- Mother: Type 2 diabetes mellitus, stroke at age 76
- Paternal uncle: Pancreatic adenocarcinoma

SOCIAL HISTORY:
Smoking: Former smoker, quit 18 years ago, 20 pack-year history. Alcohol: Rare social use, approximately 1-2 drinks per month. Occupation: Retired university physics professor.

VITAL SIGNS:
His blood pressure is 132/78 mmHg, heart rate is 78 bpm

PHYSICAL EXAMINATION:
He is alert and oriented to person, place, and time. His visual acuity is 20/30 in both eyes with corrective lenses. The patient's Mini-Mental State Examination (MMSE) score is 24/30, with deficits noted in attention and recall.

## Encounter 3: source 2023-01-15, practice 2023-04-21, outpatient, Gastroenterology

Chief complaint: Progressive fatigue, 10 kg weight loss over 3 months, dark urine, and yellowing of skin and eyes.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue, 10 kg weight loss over 3 months, dark urine, and yellowing of skin and eyes.

HISTORY OF PRESENT ILLNESS:
The patient returns with new symptoms in the setting of mild cognitive impairment and hip osteoarthritis. He reports a 3-month history of progressive fatigue, weight loss of 10 kg, and dark urine. He denies any abdominal pain but reports that his skin and eyes have become yellow.

PAST MEDICAL HISTORY:
Active Problem List:
- Mild cognitive impairment (diagnosed 2020-01-15)
- Osteoarthritis of the hip (diagnosed 2020-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Mild chronic kidney disease (Stage 3a)
- Left hip osteoarthritis
- Remote peptic ulcer disease

His past medical history is significant for hypertension, controlled with lisinopril, and type 2 diabetes mellitus, managed with metformin.

PAST SURGICAL HISTORY:
- Coronary angiography with drug-eluting stent placement
- Laparoscopic appendectomy (age 24)
- Diagnostic ERCP with biliary stent placement

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg oral daily
- Metformin 1000 mg oral twice daily
- Omeprazole 20 mg oral daily
- Donepezil 10 mg oral daily
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral daily
- Acetaminophen 500 mg oral as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 71
- Mother: Type 2 diabetes mellitus, stroke at age 76
- Paternal uncle: Pancreatic adenocarcinoma

SOCIAL HISTORY:
Smoking: Former smoker, quit 18 years ago, 20 pack-year history. Alcohol: Rare social use, approximately 1-2 drinks per month. Occupation: Retired university physics professor.

PHYSICAL EXAMINATION:
On physical examination, he appears cachectic and jaundiced. His abdomen is soft and non-tender, with a palpable, distended gallbladder.

LABORATORY DATA:
Laboratory tests reveal:

- Total bilirubin: 5.2 mg/dL (normal: 0.3-1.2 mg/dL)
- Direct bilirubin: 3.8 mg/dL (normal: 0.0-0.3 mg/dL)
- Alkaline phosphatase: 420 U/L (normal: 40-130 U/L)
- Aspartate aminotransferase (AST): 80 U/L (normal: 10-40 U/L)
- Alanine aminotransferase (ALT): 75 U/L (normal: 7-56 U/L)

IMAGING:
Abdominal ultrasound shows dilation of both the intra- and extrahepatic bile ducts with no evidence of gallstones. A subsequent CT scan of the abdomen confirms these findings and reveals a mass in the head of the pancreas.

## Encounter 4: source 2024-01-15, practice 2024-04-29, ed, Emergency Medicine

Chief complaint: Acute onset of severe dyspnea and productive cough with pink, frothy sputum while watching television.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Michael Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of severe dyspnea and productive cough with pink, frothy sputum while watching television.

HISTORY OF PRESENT ILLNESS:
This 72-year-old male with pancreatic adenocarcinoma (2023), hypertension, type 2 diabetes mellitus, and stage 3a chronic kidney disease presents with acute onset of severe dyspnea, productive cough, and pink frothy sputum. He reports that his symptoms began suddenly while he was watching television.

PAST MEDICAL HISTORY:
Active Problem List:
- Mild cognitive impairment (diagnosed 2020-01-15)
- Osteoarthritis of the hip (diagnosed 2020-07-15)
- Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation (diagnosed 2023-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Mild chronic kidney disease (Stage 3a)
- Left hip osteoarthritis
- Remote peptic ulcer disease

PAST SURGICAL HISTORY:
- Coronary angiography with drug-eluting stent placement
- Laparoscopic appendectomy (age 24)
- Diagnostic ERCP with biliary stent placement

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg oral daily
- Metformin 1000 mg oral twice daily
- Omeprazole 20 mg oral daily
- Donepezil 10 mg oral daily
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral daily
- Acetaminophen 500 mg oral as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 71
- Mother: Type 2 diabetes mellitus, stroke at age 76
- Paternal uncle: Pancreatic adenocarcinoma

SOCIAL HISTORY:
Smoking: Former smoker, quit 18 years ago, 20 pack-year history. Alcohol: Rare social use, approximately 1-2 drinks per month. Occupation: Retired university physics professor.

VITAL SIGNS:
A respiratory rate of 30 breaths per minute, blood pressure of 180/100 mmHg, heart rate of 110 beats per minute, and oxygen saturation of 85% on room air

PHYSICAL EXAMINATION:
On examination, he is tachypneic. Auscultation of the lungs reveals bilateral rales halfway up both lung fields. The jugular venous pressure is elevated. The cardiac examination is notable for an S3 gallop.

LABORATORY DATA:
Arterial blood gas on room air shows:
- pH: 7.32
- PaCO2: 50 mmHg
- PaO2: 58 mmHg
- HCO3-: 24 mEq/L

IMAGING:
Chest X-ray shows pulmonary venous congestion and Kerley B lines.

## Encounter 5: source 2024-07-15, practice 2024-10-17, ed, Emergency Medicine

Chief complaint: Crushing chest pain with radiation to the left shoulder requiring emergency evaluation.

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Emily Wong, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Crushing chest pain with radiation to the left shoulder requiring emergency evaluation.

HISTORY OF PRESENT ILLNESS:
The patient returns six months after flash pulmonary edema. He is brought to hospital after an episode of crushing chest pain with radiation to the left shoulder.

PAST MEDICAL HISTORY:
Active Problem List:
- Mild cognitive impairment (diagnosed 2020-01-15)
- Osteoarthritis of the hip (diagnosed 2020-07-15)
- Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation (diagnosed 2023-01-15)
- Flash pulmonary edema (diagnosed 2024-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Mild chronic kidney disease (Stage 3a)
- Left hip osteoarthritis
- Remote peptic ulcer disease

PAST SURGICAL HISTORY:
- Coronary angiography with drug-eluting stent placement
- Laparoscopic appendectomy (age 24)
- Diagnostic ERCP with biliary stent placement

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg oral daily
- Metformin 1000 mg oral twice daily
- Omeprazole 20 mg oral daily
- Donepezil 10 mg oral daily
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral daily
- Acetaminophen 500 mg oral as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 71
- Mother: Type 2 diabetes mellitus, stroke at age 76
- Paternal uncle: Pancreatic adenocarcinoma

SOCIAL HISTORY:
Smoking: Former smoker, quit 18 years ago, 20 pack-year history. Alcohol: Rare social use, approximately 1-2 drinks per month. Occupation: Retired university physics professor.

LABORATORY DATA:
Serum CK-MB demonstrates an elevation compared to the pre-op state

OTHER STUDIES:
An ECG shows ST-elevations in V1-V3

PLAN:
Percutanous coronary intervention (PCI) is performed with placement of a stent

## Encounter 6: source 2025-01-15, practice 2025-04-28, icu, Critical Care

Chief complaint: Worsening abdominal pain, fever, and chills over 18 hours with acute rigors and confusion.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Robert Kim, MD
Department: Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Worsening abdominal pain, fever, and chills over 18 hours with acute rigors and confusion.

HISTORY OF PRESENT ILLNESS:
The patient is a 73-year-old male with mucin-producing pancreatic adenocarcinoma, mild cognitive impairment, and recent cardiopulmonary events who presents with acute abdominal complaints. His wife reports that he had been complaining of fatigue and poor appetite for 2 days, but the acute pain and rigors began suddenly this morning. His daughter notes that he has been more withdrawn lately and less eager to eat with the family, which was unusual because he normally enjoyed long dinners with them.

PAST MEDICAL HISTORY:
Active Problem List:
- Mild cognitive impairment (diagnosed 2020-01-15)
- Osteoarthritis of the hip (diagnosed 2020-07-15)
- Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation (diagnosed 2023-01-15)
- Flash pulmonary edema (diagnosed 2024-01-15)
- Myocardial Reperfusion Injury (diagnosed 2024-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Mild chronic kidney disease (Stage 3a)
- Left hip osteoarthritis
- Remote peptic ulcer disease

He has a long history of hypertension, type 2 diabetes mellitus, and mild chronic kidney disease.

PAST SURGICAL HISTORY:
- Coronary angiography with drug-eluting stent placement
- Laparoscopic appendectomy (age 24)
- Diagnostic ERCP with biliary stent placement

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg oral daily
- Metformin 1000 mg oral twice daily
- Omeprazole 20 mg oral daily
- Donepezil 10 mg oral daily
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral daily
- Acetaminophen 500 mg oral as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 71
- Mother: Type 2 diabetes mellitus, stroke at age 76
- Paternal uncle: Pancreatic adenocarcinoma

SOCIAL HISTORY:
He lives with his wife and occasionally visits his two adult children, one of whom lives locally and the other abroad. On further history, the patient is described as someone who was deeply passionate about folk art collecting, having spent decades traveling across different countries to acquire unique handmade pieces. His study at home is lined with wooden masks from West Africa, textiles from Central Asia, and clay figurines from Latin America. He once remarked that his favorite trips were to Mexico and Guatemala, where he collected brightly painted wooden carvings. He often gave informal lectures at the local community center about cultural symbolism in the pieces he collected. His wife recalls that, even just two months ago, he was planning to attend an exhibition in Eastern Europe. He also enjoyed international travel for family connections. His son lives in Germany, and he had made multiple trips there in recent years, during which he also visited nearby countries such as France, Italy, and Switzerland. Ten years ago, he had a longer journey to Southeast Asia, including Thailand, Vietnam, and Cambodia, where he admired local craft markets. While these trips gave him joy, his family occasionally worried about his exposure to infectious diseases, though he never reported serious illness.

VITAL SIGNS:
On the day of admission, he is febrile to 39.4°C (103°F), tachycardic with a pulse of 128/min, tachypneic at 28/min, and hypotensive with a blood pressure of 84/52 mm Hg.

PHYSICAL EXAMINATION:
He appears acutely ill, diaphoretic, and somewhat confused. His skin is warm with bounding pulses, consistent with distributive physiology. The abdomen is diffusely tender without rebound or guarding. Bowel sounds are hypoactive.

LABORATORY DATA:
Blood cultures later grow gram-negative rods. Laboratory studies reveal leukocytosis with a left shift, elevated lactate, and worsening creatinine. On day 2 of hospitalization, the patient’s confusion worsens, and his hands and feet appear mottled. His urine output decreases significantly. Liver enzymes climb, consistent with hypoperfusion injury.

IMAGING:
Imaging studies are performed: chest X-ray reveals low lung volumes with bibasilar atelectasis; echocardiography demonstrates a hyperdynamic left ventricle with preserved systolic function, making cardiogenic shock unlikely. A CT scan of the abdomen shows diffuse bowel wall thickening, concerning for ischemia and translocation of bacteria.

PLAN:
IV fluids are started immediately, along with broad-spectrum antibiotics. Despite aggressive fluid resuscitation, his condition worsens over the next 12 hours. His mean arterial pressure continues to drop, requiring initiation of norepinephrine infusion. He is transferred to the intensive care unit. His daughter, arriving from Germany, brings a small carved wooden figure he had collected years ago and places it at his bedside. His wife notes that his illness feels sudden and devastating, given his otherwise relatively independent lifestyle. The ICU team continues maximal vasopressor and ventilatory support, but his blood pressure remains difficult to maintain. His condition continues to decline and he dies after 4 days of hospitalization.

## Encounter 7: source 2026-01-15, practice 2026-04-23, ed, Emergency Medicine

Chief complaint: Acute difficulty eating breakfast, bumping into objects on the left side, and unawareness of the left arm.

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. Lisa Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute difficulty eating breakfast, bumping into objects on the left side, and unawareness of the left arm.

HISTORY OF PRESENT ILLNESS:
This 74-year-old male with hypertension, diabetes, and history of pancreatic adenocarcinoma complicated by recent septic shock is brought to the emergency department by his daughter after she noticed that he had acute difficulty eating breakfast, bumping into objects on the left side, and unawareness of the left arm.

PAST MEDICAL HISTORY:
Active Problem List:
- Mild cognitive impairment (diagnosed 2020-01-15)
- Osteoarthritis of the hip (diagnosed 2020-07-15)
- Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation (diagnosed 2023-01-15)
- Flash pulmonary edema (diagnosed 2024-01-15)
- Myocardial Reperfusion Injury (diagnosed 2024-07-15)
- Septic shock (diagnosed 2025-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Mild chronic kidney disease (Stage 3a)
- Left hip osteoarthritis
- Remote peptic ulcer disease

His past medical history includes hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Coronary angiography with drug-eluting stent placement
- Laparoscopic appendectomy (age 24)
- Diagnostic ERCP with biliary stent placement

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg oral daily
- Metformin 1000 mg oral twice daily
- Omeprazole 20 mg oral daily
- Donepezil 10 mg oral daily
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral daily
- Acetaminophen 500 mg oral as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 71
- Mother: Type 2 diabetes mellitus, stroke at age 76
- Paternal uncle: Pancreatic adenocarcinoma

SOCIAL HISTORY:
Smoking: Former smoker, quit 18 years ago, 20 pack-year history. Alcohol: Rare social use, approximately 1-2 drinks per month. Occupation: Retired university physics professor.

VITAL SIGNS:
On examination, the patient has a blood pressure of 150/90 mmHg, a pulse of 78 beats per minute, and a respiratory rate of 16 breaths per minute.

PHYSICAL EXAMINATION:
Neurological examination reveals that the patient does not respond to visual stimuli on his left side and has left-sided weakness. He is able to read the right half of a sentence but ignores the left half. His speech is fluent, and he seems unaware of his deficits.

IMAGING:
A CT scan of the head shows no evidence of acute hemorrhage.
