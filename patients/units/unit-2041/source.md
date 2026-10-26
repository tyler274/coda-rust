# synthetic_hospital patient 2041

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-04-22 (Wed) | outpatient | Internal Medicine |
| 1 | 2021-01-15 | 2021-04-29 (Thu) | ed | Emergency Medicine |
| 2 | 2021-02-15 | 2021-05-27 (Thu) | ed | Emergency Medicine |
| 3 | 2022-01-15 | 2022-04-25 (Mon) | ed | Emergency Medicine |
| 4 | 2022-07-15 | 2022-11-01 (Tue) | ed | Emergency Medicine |
| 5 | 2023-07-15 | 2023-10-24 (Tue) | ed | Emergency Medicine |
| 6 | 2024-07-15 | 2024-11-01 (Fri) | ed | Emergency Medicine |
| 7 | 2025-07-15 | 2025-10-27 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Unemployed (former automotive mechanic)",
 "alcohol_use": "Severe alcohol use disorder with physiological dependence; patient reports drinking 'a few beers' nightly but demonstrates withdrawal phenomena (morning tremors, tactile/auditory hallucinations, autonomic hyperactivity) and severe complications including recurrent pancreatitis and malnutrition",
 "family_history": [
  "Father with alcohol use disorder and cirrhosis",
  "Mother with hypertension and major depressive disorder",
  "One sibling with substance use disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 5 years ago",
 "home_medications": [
  {
   "dose": "25 mg oral daily",
   "name": "Hydrochlorothiazide"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Severe alcohol use disorder",
  "Major depressive disorder, recurrent moderate",
  "Hypertension",
  "Chronic pancreatitis (alcohol-induced)",
  "Protein-calorie malnutrition"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Severe acute pancreatitis (alcohol-induced)", "Infected pancreatic necrosis", "Ethylene glycol poisoning", "Alcohol use disorder", "Alcoholic hallucinosis", "Wernicke encephalopathy"]
Comorbidities: ["Acute kidney injury", "Alcohol use disorder", "Alcoholic liver disease", "Major depressive disorder", "Essential hypertension", "Systemic inflammatory response syndrome"]

## Encounter 0: source 2020-01-15, practice 2020-04-22, outpatient, Internal Medicine

Chief complaint: Difficulty falling asleep, feeling anxious, and morning tremors for the past 6 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Difficulty falling asleep, feeling anxious, and morning tremors for the past 6 months

HISTORY OF PRESENT ILLNESS:
He reports that his sleep problems have been worsening and he often wakes up feeling tired. He also mentions experiencing occasional tremors in the morning.

PAST MEDICAL HISTORY:
Active Problem List:
- Severe alcohol use disorder
- Major depressive disorder, recurrent moderate
- Hypertension
- Chronic pancreatitis (alcohol-induced)
- Protein-calorie malnutrition

His past medical history is significant for hypertension,.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
For which he takes hydrochlorothiazide.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder and cirrhosis
- Mother with hypertension and major depressive disorder
- One sibling with substance use disorder

SOCIAL HISTORY:
He denies any illicit drug use but admits to drinking "a few beers" most nights to help him relax.

VITAL SIGNS:
On physical examination, his blood pressure is 140/90 mmHg, heart rate is 88 bpm, and BMI is 29 kg/m^2.

PHYSICAL EXAMINATION:
He appears mildly agitated. His liver is palpable 2 cm below the right costal margin, and his skin shows spider angiomas.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 13.5 g/dL
- Mean corpuscular volume (MCV): 98 fL (normal: 80-96 fL)
- Aspartate aminotransferase (AST): 72 U/L (normal: 10-40 U/L)
- Alanine aminotransferase (ALT): 50 U/L (normal: 7-56 U/L)
- Gamma-glutamyl transferase (GGT): 110 U/L (normal: 9-48 U/L)

## Encounter 1: source 2021-01-15, practice 2021-04-29, ed, Emergency Medicine

Chief complaint: Severe abdominal pain, nausea, and vomiting for 2 days

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe abdominal pain, nausea, and vomiting for 2 days

HISTORY OF PRESENT ILLNESS:
The patient returns to the emergency department one year after his initial outpatient evaluation for alcohol use disorder and in the context of known chronic pancreatitis and severe alcohol use disorder, presenting with a 2-day history of abdominal pain, nausea, and vomiting. He is verbally aggressive towards the staff, insisting that he needs "something strong" for his pain and becomes increasingly agitated when his requests are not immediately met.

PAST MEDICAL HISTORY:
Active Problem List:
- Alcohol use disorder (diagnosed 2020-01-15)
- Major depressive disorder, recurrent moderate
- Hypertension
- Chronic pancreatitis (alcohol-induced)
- Protein-calorie malnutrition

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder and cirrhosis
- Mother with hypertension and major depressive disorder
- One sibling with substance use disorder

SOCIAL HISTORY:
Smoking: Former smoker, quit 5 years ago. Alcohol: Severe alcohol use disorder with physiological dependence; patient reports drinking 'a few beers' nightly but demonstrates withdrawal phenomena (morning tremors, tactile/auditory hallucinations, autonomic hyperactivity) and severe complications including recurrent pancreatitis and malnutrition. Occupation: Unemployed (former automotive mechanic).

VITAL SIGNS:
His temperature is 38.3°C (100.9°F), blood pressure is 150/90 mmHg, pulse is 110/min, and respiratory rate is 22/min.

PHYSICAL EXAMINATION:
Physical examination reveals a distended abdomen with diffuse tenderness, but no rebound or guarding. Bowel sounds are hypoactive.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 16,000/mm³
- Hemoglobin: 13.2 g/dL
- Platelets: 230,000/mm³
- Serum amylase: 250 U/L (Normal: 30-110 U/L)
- Serum lipase: 600 U/L (Normal: 0-160 U/L)
- Aspartate aminotransferase (AST): 120 U/L (Normal: 0-40 U/L)
- Alanine aminotransferase (ALT): 100 U/L (Normal: 0-40 U/L)
- Alkaline phosphatase: 90 U/L (Normal: 40-130 U/L)
- Total bilirubin: 1.2 mg/dL (Normal: 0.1-1.2 mg/dL)

## Encounter 2: source 2021-02-15, practice 2021-05-27, ed, Emergency Medicine

Chief complaint: Worsening severe epigastric pain radiating to the back, 10 days after initial pancreatitis diagnosis

ENCOUNTER NOTE
Date: 2021-02-15
Provider: Dr. Fatima Al-Hassan, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening severe epigastric pain radiating to the back, 10 days after initial pancreatitis diagnosis

HISTORY OF PRESENT ILLNESS:
The patient is a 46-year-old male with severe alcohol use disorder, initially evaluated for alcohol-related symptoms in January 2020, who returns to the emergency department with worsening abdominal pain. He presents with a 10-day history of severe epigastric pain radiating to the back. He was diagnosed with acute pancreatitis 7 days ago and was discharged with supportive care. He now reports that the pain has worsened despite taking prescribed analgesics. He also complains of nausea and has had several episodes of non-bloody, non-bilious vomiting.

PAST MEDICAL HISTORY:
Active Problem List:
- Alcohol use disorder (diagnosed 2020-01-15)
- Severe acute pancreatitis (alcohol-induced) (diagnosed 2021-01-15)
- Major depressive disorder, recurrent moderate
- Hypertension
- Protein-calorie malnutrition

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder and cirrhosis
- Mother with hypertension and major depressive disorder
- One sibling with substance use disorder

SOCIAL HISTORY:
Smoking: Former smoker, quit 5 years ago. Alcohol: Severe alcohol use disorder with physiological dependence; patient reports drinking 'a few beers' nightly but demonstrates withdrawal phenomena (morning tremors, tactile/auditory hallucinations, autonomic hyperactivity) and severe complications including recurrent pancreatitis and malnutrition. Occupation: Unemployed (former automotive mechanic).

VITAL SIGNS:
On examination, he is febrile with a temperature of 38.7°C, tachycardic with a heart rate of 112 bpm, and hypotensive with a blood pressure of 90/60 mmHg.

PHYSICAL EXAMINATION:
His abdomen is distended, with diffuse tenderness and decreased bowel sounds.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 18,000 /µL
- Hemoglobin: 13.2 g/dL
- Platelets: 250,000 /µL
- Serum amylase: 450 U/L
- Serum lipase: 900 U/L
- C-reactive protein: 150 mg/L

IMAGING:
An urgent contrast-enhanced CT scan of the abdomen shows a large area of pancreatic necrosis with gas formation within the collection.

## Encounter 3: source 2022-01-15, practice 2022-04-25, ed, Emergency Medicine

Chief complaint: Confusion, difficulty walking, and double vision over the past 2 days

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Confusion, difficulty walking, and double vision over the past 2 days

HISTORY OF PRESENT ILLNESS:
This 47-year-old male with severe alcohol use disorder and chronic pancreatitis complicated by infected necrosis in February 2021 presents to the emergency department with confusion, difficulty walking, and double vision that started two days ago. He reports that he has been eating very little over the past few weeks due to nausea and vomiting.

PAST MEDICAL HISTORY:
Active Problem List:
- Alcohol use disorder (diagnosed 2020-01-15)
- Severe acute pancreatitis (alcohol-induced) (diagnosed 2021-01-15)
- Infected pancreatic necrosis (diagnosed 2021-02-15)
- Major depressive disorder, recurrent moderate
- Hypertension
- Protein-calorie malnutrition

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder and cirrhosis
- Mother with hypertension and major depressive disorder
- One sibling with substance use disorder

SOCIAL HISTORY:
Smoking: Former smoker, quit 5 years ago. Alcohol: Severe alcohol use disorder with physiological dependence; patient reports drinking 'a few beers' nightly but demonstrates withdrawal phenomena (morning tremors, tactile/auditory hallucinations, autonomic hyperactivity) and severe complications including recurrent pancreatitis and malnutrition. Occupation: Unemployed (former automotive mechanic).

VITAL SIGNS:
His vital signs are stable

PHYSICAL EXAMINATION:
On examination, the patient is disoriented to time and place, has nystagmus on lateral gaze, and a wide-based gait. There is no evidence of trauma.

LABORATORY DATA:
Laboratory tests reveal:
- Serum glucose: 250 mg/dL
- Serum sodium: 140 mEq/L
- Serum potassium: 4.0 mEq/L
- Serum creatinine: 1.0 mg/dL
- AST: 80 U/L
- ALT: 35 U/L
- Thiamine level: Not measured

PLAN:
The patient is given intravenous fluids with glucose. Within hours, the patient's confusion worsens.

## Encounter 4: source 2022-07-15, practice 2022-11-01, ed, Emergency Medicine

Chief complaint: Visual hallucinations of bugs crawling on walls and auditory hallucinations after 24 hours without alcohol

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Visual hallucinations of bugs crawling on walls and auditory hallucinations after 24 hours without alcohol

HISTORY OF PRESENT ILLNESS:
This 47-year-old male with severe alcohol use disorder, chronic pancreatitis, and Wernicke encephalopathy presents to the emergency department accompanied by his sister. She reports that he has not had a drink in approximately 24 hours, which is unusual for him.

PAST MEDICAL HISTORY:
Active Problem List:
- Alcohol use disorder (diagnosed 2020-01-15)
- Severe acute pancreatitis (alcohol-induced) (diagnosed 2021-01-15)
- Infected pancreatic necrosis (diagnosed 2021-02-15)
- Wernicke encephalopathy (diagnosed 2022-01-15)
- Major depressive disorder, recurrent moderate
- Hypertension
- Protein-calorie malnutrition

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder and cirrhosis
- Mother with hypertension and major depressive disorder
- One sibling with substance use disorder

SOCIAL HISTORY:
Smoking: Former smoker, quit 5 years ago. Alcohol: Severe alcohol use disorder with physiological dependence; patient reports drinking 'a few beers' nightly but demonstrates withdrawal phenomena (morning tremors, tactile/auditory hallucinations, autonomic hyperactivity) and severe complications including recurrent pancreatitis and malnutrition. Occupation: Unemployed (former automotive mechanic).

VITAL SIGNS:
His vital signs are stable: blood pressure 130/85 mmHg, heart rate 88 beats per minute, respiratory rate 16 breaths per minute, and temperature 37.1°C (98.8°F).

PHYSICAL EXAMINATION:
He appears anxious but is fully oriented to person, place, and time. Physical examination is unremarkable except for a mild tremor in his hands.

LABORATORY DATA:
Laboratory studies are as follows:

- Complete blood count (CBC): within normal limits
- Electrolytes: within normal limits
- Liver function tests (LFTs): mild elevation in AST and ALT
- Blood alcohol level: undetectable

## Encounter 5: source 2023-07-15, practice 2023-10-24, ed, Emergency Medicine

Chief complaint: Severe epigastric pain radiating to the back, nausea, and vomiting for the past 24 hours

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Ana Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe epigastric pain radiating to the back, nausea, and vomiting for the past 24 hours

HISTORY OF PRESENT ILLNESS:
The patient with severe alcohol use disorder and chronic pancreatitis, treated for infected pancreatic necrosis in 2021, returns to the emergency department with severe epigastric pain radiating to the back, nausea, and vomiting for the past 24 hours. The pain is constant, worsened with eating, and slightly relieved by leaning forward.

PAST MEDICAL HISTORY:
Active Problem List:
- Alcohol use disorder (diagnosed 2020-01-15)
- Severe acute pancreatitis (alcohol-induced) (diagnosed 2021-01-15)
- Infected pancreatic necrosis (diagnosed 2021-02-15)
- Wernicke encephalopathy (diagnosed 2022-01-15)
- Alcoholic hallucinosis (diagnosed 2022-07-15)
- Major depressive disorder, recurrent moderate
- Hypertension
- Protein-calorie malnutrition

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder and cirrhosis
- Mother with hypertension and major depressive disorder
- One sibling with substance use disorder

SOCIAL HISTORY:
Smoking: Former smoker, quit 5 years ago. Alcohol: Severe alcohol use disorder with physiological dependence; patient reports drinking 'a few beers' nightly but demonstrates withdrawal phenomena (morning tremors, tactile/auditory hallucinations, autonomic hyperactivity) and severe complications including recurrent pancreatitis and malnutrition. Occupation: Unemployed (former automotive mechanic).

VITAL SIGNS:
Blood pressure of 90/60 mmHg, heart rate of 115 bpm, and temperature of 38.2°C (100.8°F)

PHYSICAL EXAMINATION:
On examination, the patient appears distressed. Abdominal examination reveals epigastric tenderness and decreased bowel sounds.

LABORATORY DATA:
White blood cell count: 16,000 /µL
Hemoglobin: 13.5 g/dL
Platelets: 250,000 /µL
Serum amylase: 900 U/L (Normal: 30-110 U/L)
Serum lipase: 850 U/L (Normal: 0-160 U/L)
AST: 120 U/L (Normal: 10-40 U/L)
ALT: 90 U/L (Normal: 7-56 U/L)
Blood urea nitrogen: 25 mg/dL (Normal: 7-20 mg/dL)
Creatinine: 1.2 mg/dL (Normal: 0.6-1.2 mg/dL)

IMAGING:
An abdominal ultrasound shows no gallstones or biliary ductal dilatation.

## Encounter 6: source 2024-07-15, practice 2024-11-01, ed, Emergency Medicine

Chief complaint: Acute confusion and difficulty breathing; wife reports increased depression and unknown substance consumption

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. James Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute confusion and difficulty breathing; wife reports increased depression and unknown substance consumption

HISTORY OF PRESENT ILLNESS:
The patient with severe alcohol use disorder and recurrent pancreatitis returns for care. His wife reports he has been depressed recently after losing his job and has been drinking more than usual, though she is unsure what he has been consuming.

PAST MEDICAL HISTORY:
Active Problem List:
- Alcohol use disorder (diagnosed 2020-01-15)
- Severe acute pancreatitis (alcohol-induced) (diagnosed 2021-01-15)
- Infected pancreatic necrosis (diagnosed 2021-02-15)
- Wernicke encephalopathy (diagnosed 2022-01-15)
- Alcoholic hallucinosis (diagnosed 2022-07-15)
- Major depressive disorder, recurrent moderate
- Hypertension
- Protein-calorie malnutrition

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder and cirrhosis
- Mother with hypertension and major depressive disorder
- One sibling with substance use disorder

SOCIAL HISTORY:
Smoking: Former smoker, quit 5 years ago. Alcohol: Severe alcohol use disorder with physiological dependence; patient reports drinking 'a few beers' nightly but demonstrates withdrawal phenomena (morning tremors, tactile/auditory hallucinations, autonomic hyperactivity) and severe complications including recurrent pancreatitis and malnutrition. Occupation: Unemployed (former automotive mechanic).

VITAL SIGNS:
His temperature is 37.2°C (99.0°F), blood pressure is 140/90 mmHg, pulse is 110/min, and respiratory rate is 28/min.

PHYSICAL EXAMINATION:
On examination, the patient is tachypneic, and his breath has a faint, sweet odor. He is disoriented to time and place. Physical examination reveals flank pain on palpation.

LABORATORY DATA:
Urinalysis shows calcium oxalate crystals. Laboratory values are as follows:

- Serum calcium: 7.0 mg/dL (Normal: 8.5-10.2 mg/dL)
- Serum creatinine: 2.5 mg/dL (Normal: 0.6-1.3 mg/dL)
- Arterial blood gas: pH 7.25 (Normal: 7.35-7.45)
- Anion gap: 24 mEq/L (Normal: 8-16 mEq/L)

## Encounter 7: source 2025-07-15, practice 2025-10-27, ed, Emergency Medicine

Chief complaint: Confusion and respiratory distress; found in garage with empty automotive product containers

ENCOUNTER NOTE
Date: 2025-07-15
Provider: Dr. Emily Watson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Confusion and respiratory distress; found in garage with empty automotive product containers

HISTORY OF PRESENT ILLNESS:
The patient, with severe alcohol use disorder and prior ethylene glycol poisoning, presents again with confusion and respiratory distress. His wife reports that he has been depressed recently after losing his job and has a history of alcohol abuse. She also mentions finding him in the garage with several empty containers of automotive products.

PAST MEDICAL HISTORY:
Active Problem List:
- Alcohol use disorder (diagnosed 2020-01-15)
- Severe acute pancreatitis (alcohol-induced) (diagnosed 2021-01-15)
- Infected pancreatic necrosis (diagnosed 2021-02-15)
- Wernicke encephalopathy (diagnosed 2022-01-15)
- Alcoholic hallucinosis (diagnosed 2022-07-15)
- Ethylene glycol poisoning (diagnosed 2024-07-15)
- Major depressive disorder, recurrent moderate
- Hypertension
- Protein-calorie malnutrition

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder and cirrhosis
- Mother with hypertension and major depressive disorder
- One sibling with substance use disorder

SOCIAL HISTORY:
Smoking: Former smoker, quit 5 years ago. Alcohol: Severe alcohol use disorder with physiological dependence; patient reports drinking 'a few beers' nightly but demonstrates withdrawal phenomena (morning tremors, tactile/auditory hallucinations, autonomic hyperactivity) and severe complications including recurrent pancreatitis and malnutrition. Occupation: Unemployed (former automotive mechanic).

VITAL SIGNS:
BP 130/85 mmHg, HR 110 bpm, RR 30 breaths/min, Temp 37.2°C (99.0°F), and O2 saturation 94% on room air.

PHYSICAL EXAMINATION:
On examination, the patient is tachypneic, and his breath has a faint, sweet odor. Neurological examination reveals myoclonus and horizontal nystagmus.

LABORATORY DATA:
Serum sodium: 140 mEq/L
- Serum potassium: 4.2 mEq/L
- Serum chloride: 98 mEq/L
- Serum bicarbonate: 12 mEq/L
- Serum creatinine: 1.8 mg/dL
- Serum osmolality: 320 mOsm/kg
- Calculated osmolality: 290 mOsm/kg
- Anion gap: 30 mEq/L (calculated as Na+ - (Cl- + HCO3-))

Urine microscopy reveals the presence of calcium oxalate crystals.
