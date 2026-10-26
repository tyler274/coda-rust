# synthetic_hospital patient 2708

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 50 at first encounter, sex F, Non-Hispanic White, Private PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-01-02 (Mon) | outpatient | Endocrinology |
| 1 | 2020-07-15 | 2023-06-21 (Wed) | outpatient | Dermatology |
| 2 | 2021-01-15 | 2024-01-01 (Mon) | outpatient | Nephrology |
| 3 | 2021-07-15 | 2024-06-24 (Mon) | outpatient | Dermatology |
| 4 | 2022-01-15 | 2024-12-20 (Fri) | ed | Gastroenterology |
| 5 | 2022-07-15 | 2025-07-02 (Wed) | icu | Gastroenterology |
| 6 | 2023-01-15 | 2025-12-30 (Tue) | inpatient | Pulmonology |
| 7 | 2023-02-15 | 2026-01-25 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Private PPO",
 "occupation": "Office Administrator",
 "alcohol_use": "Former daily heavy consumption (>10 years), abstinent for 5 years",
 "family_history": [
  "Mother: Hypertension, breast cancer (age 65)",
  "Father: Alcohol-related cirrhosis",
  "Maternal aunt: Lung cancer (never smoker)",
  "Brother: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker, denies secondhand smoke exposure",
 "home_medications": [
  {
   "dose": "25 mg oral daily",
   "name": "Hydrochlorothiazide"
  }
 ],
 "surgical_history": [
  "Recent video-assisted thoracoscopic surgery (VATS) with right lower lobectomy (performed 10 days ago)",
  "Laparoscopic cholecystectomy (age 42)"
 ],
 "chronic_conditions": [
  "Chronic hepatitis C infection (genotype 1a, diagnosed 12 years ago, incomplete prior antiviral therapy)",
  "Hypertension",
  "History of alcohol use disorder (in sustained remission)"
 ],
 "age_at_first_encounter": 50
}
```

Primary diagnoses: ["Benign oral leukoplakia", "Lichen planus", "Isoniazid-induced liver injury (Drug-induced liver injury)", "Bleeding esophageal varices", "Stage IIIB Non-Small Cell Lung Cancer", "Pulmonary embolism", "Cryoglobulinemia", "Pheochromocytoma"]
Comorbidities: ["Hepatocellular Injury", "Chronic hepatitis C", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2023-01-02, outpatient, Endocrinology

Chief complaint: Episodic headaches, palpitations, and diaphoresis for the past 6 months, often accompanied by a sense of doom and triggered by physical exertion.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Episodic headaches, palpitations, and diaphoresis for the past 6 months, often accompanied by a sense of doom and triggered by physical exertion.

HISTORY OF PRESENT ILLNESS:
She reports that these episodes are often accompanied by a sense of doom and are sometimes triggered by physical exertion or stress.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic hepatitis C infection (genotype 1a, diagnosed 12 years ago, incomplete prior antiviral therapy)
- Hypertension
- History of alcohol use disorder (in sustained remission)

PAST SURGICAL HISTORY:
- Recent video-assisted thoracoscopic surgery (VATS) with right lower lobectomy (performed 10 days ago)
- Laparoscopic cholecystectomy (age 42)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg oral daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Hypertension, breast cancer (age 65)
- Father: Alcohol-related cirrhosis
- Maternal aunt: Lung cancer (never smoker)
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker, denies secondhand smoke exposure. Alcohol: Former daily heavy consumption (>10 years), abstinent for 5 years. Occupation: Office Administrator.

VITAL SIGNS:
Her blood pressure in the clinic is 170/95 mmHg.

PHYSICAL EXAMINATION:
Physical examination reveals a well-nourished woman in mild distress. No abdominal bruits are heard, and there is no palpable abdominal mass. Her skin is notably moist.

LABORATORY DATA:
Laboratory tests show:

- Serum glucose: 110 mg/dL
- Serum potassium: 3.5 mEq/L
- Serum creatinine: 0.9 mg/dL
- Urinary metanephrines: Elevated
- Urinary normetanephrines: Elevated

IMAGING:
An abdominal CT scan demonstrates a 3 cm left adrenal mass with characteristics suggestive of a pheochromocytoma.

PLAN:
The patient is scheduled for surgical resection of the mass.

## Encounter 1: source 2020-07-15, practice 2023-06-21, outpatient, Dermatology

Chief complaint: 2-month history of itchy skin lesions that began on her wrists and have since spread to her ankles and oral mucosa.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
2-month history of itchy skin lesions that began on her wrists and have since spread to her ankles and oral mucosa.

HISTORY OF PRESENT ILLNESS:
The patient with chronic hepatitis C and prior pheochromocytoma presents for dermatologic evaluation. She reports that the lesions began on her wrists and have since spread to her ankles and oral mucosa. She denies any new medications, soaps, or exposure to chemicals.

PAST MEDICAL HISTORY:
Active Problem List:
- Pheochromocytoma (diagnosed 2020-01-15)
- Chronic hepatitis C infection (genotype 1a, diagnosed 12 years ago, incomplete prior antiviral therapy)
- Hypertension
- History of alcohol use disorder (in sustained remission)

PAST SURGICAL HISTORY:
- Recent video-assisted thoracoscopic surgery (VATS) with right lower lobectomy (performed 10 days ago)
- Laparoscopic cholecystectomy (age 42)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg oral daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Hypertension, breast cancer (age 65)
- Father: Alcohol-related cirrhosis
- Maternal aunt: Lung cancer (never smoker)
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker, denies secondhand smoke exposure. Alcohol: Former daily heavy consumption (>10 years), abstinent for 5 years. Occupation: Office Administrator.

PHYSICAL EXAMINATION:
On physical examination, you note violaceous, flat-topped, polygonal papules on the flexural surfaces of her wrists and ankles. Some of the papules have fine white lines on their surface. Examination of the oral mucosa reveals lacy white patches. Her nails show longitudinal ridging and splitting. No lymphadenopathy is noted. The rest of the examination is unremarkable.

LABORATORY DATA:
Complete blood count: within normal limits
- Liver function tests: mildly elevated AST and ALT
- Hepatitis C virus RNA: positive
- HIV antibody: negative

## Encounter 2: source 2021-01-15, practice 2024-01-01, outpatient, Nephrology

Chief complaint: 2-week history of joint pains and a rash on her lower extremities with unusual fatigue and episodes of tingling in her hands and feet.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Aisha Patel, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
2-week history of joint pains and a rash on her lower extremities with unusual fatigue and episodes of tingling in her hands and feet.

HISTORY OF PRESENT ILLNESS:
This 51-year-old female with chronic hepatitis C for 12 years and prior episodic headaches with palpitations and skin lesions describes the joint pains as aching and symmetrical, involving her wrists and knees. She denies any recent infections or new medications. She also mentions feeling unusually fatigued and having episodes of tingling in her hands and feet.

PAST MEDICAL HISTORY:
Active Problem List:
- Pheochromocytoma (diagnosed 2020-01-15)
- Lichen planus (diagnosed 2020-07-15)
- Chronic hepatitis C infection (genotype 1a, diagnosed 12 years ago, incomplete prior antiviral therapy)
- Hypertension
- History of alcohol use disorder (in sustained remission)

PAST SURGICAL HISTORY:
- Recent video-assisted thoracoscopic surgery (VATS) with right lower lobectomy (performed 10 days ago)
- Laparoscopic cholecystectomy (age 42)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg oral daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Hypertension, breast cancer (age 65)
- Father: Alcohol-related cirrhosis
- Maternal aunt: Lung cancer (never smoker)
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker, denies secondhand smoke exposure. Alcohol: Former daily heavy consumption (>10 years), abstinent for 5 years. Occupation: Office Administrator.

VITAL SIGNS:
Her temperature is 37.2°C (99°F), blood pressure is 130/80 mmHg, and pulse is 78/min.

PHYSICAL EXAMINATION:
On examination, you note palpable purpura on her legs.

LABORATORY DATA:
Complete blood count: Normal
- Liver function tests: Mildly elevated AST and ALT
- Serum creatinine: 1.2 mg/dL (elevated from her baseline of 0.9 mg/dL)
- Urinalysis: Proteinuria and hematuria
- Complement levels: Low C4, normal C3
- Rheumatoid factor: Positive
- Serum protein electrophoresis: Polyclonal gammopathy

## Encounter 3: source 2021-07-15, practice 2024-06-24, outpatient, Dermatology

Chief complaint: Painful lesions in her mouth that have been persistent for several months with similar appearing itchy lesions on her wrists.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Emily O'Connor, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Painful lesions in her mouth that have been persistent for several months with similar appearing itchy lesions on her wrists.

HISTORY OF PRESENT ILLNESS:
The patient with chronic hepatitis C returns with oral and wrist lesions following prior Dermatology and Nephrology care. She denies recent dental procedures or trauma. She has similar appearing lesions on her wrists which she reports are itchy but have been improving without treatment.

PAST MEDICAL HISTORY:
Active Problem List:
- Pheochromocytoma (diagnosed 2020-01-15)
- Lichen planus (diagnosed 2020-07-15)
- Cryoglobulinemia (diagnosed 2021-01-15)
- Chronic hepatitis C infection (genotype 1a, diagnosed 12 years ago, incomplete prior antiviral therapy)
- Hypertension
- History of alcohol use disorder (in sustained remission)

PAST SURGICAL HISTORY:
- Recent video-assisted thoracoscopic surgery (VATS) with right lower lobectomy (performed 10 days ago)
- Laparoscopic cholecystectomy (age 42)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg oral daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Hypertension, breast cancer (age 65)
- Father: Alcohol-related cirrhosis
- Maternal aunt: Lung cancer (never smoker)
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker, denies secondhand smoke exposure. Alcohol: Former daily heavy consumption (>10 years), abstinent for 5 years. Occupation: Office Administrator.

PHYSICAL EXAMINATION:
On examination, you note white, reticulated papules and plaques bilaterally on her buccal mucosa. Some areas appear erythematous with central ulceration.

LABORATORY DATA:
Complete Blood Count (CBC): within normal limits
- Liver Function Tests (LFTs): mildly elevated AST and ALT
- Hepatitis C Virus (HCV) RNA: positive
- Autoantibodies (ANA, anti-LKM-1): negative

## Encounter 4: source 2022-01-15, practice 2024-12-20, ed, Gastroenterology

Chief complaint: 2-week history of fatigue, anorexia, and dark urine that began six weeks after initiating isoniazid therapy for latent tuberculosis.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. David Kim, MD
Department: Gastroenterology
Visit Type: ed

CHIEF COMPLAINT:
2-week history of fatigue, anorexia, and dark urine that began six weeks after initiating isoniazid therapy for latent tuberculosis.

HISTORY OF PRESENT ILLNESS:
This 52-year-old female with chronic hepatitis C infection (genotype 1a, incomplete prior antiviral therapy), prior cryoglobulinemia, and alcohol use disorder in sustained remission started isoniazid therapy six weeks ago. She presents with a two-week history of fatigue, anorexia, and dark urine.

PAST MEDICAL HISTORY:
Active Problem List:
- Pheochromocytoma (diagnosed 2020-01-15)
- Lichen planus (diagnosed 2020-07-15)
- Cryoglobulinemia (diagnosed 2021-01-15)
- Benign oral leukoplakia (diagnosed 2021-07-15)
- Chronic hepatitis C infection (genotype 1a, diagnosed 12 years ago, incomplete prior antiviral therapy)
- Hypertension
- History of alcohol use disorder (in sustained remission)

PAST SURGICAL HISTORY:
- Recent video-assisted thoracoscopic surgery (VATS) with right lower lobectomy (performed 10 days ago)
- Laparoscopic cholecystectomy (age 42)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg oral daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Hypertension, breast cancer (age 65)
- Father: Alcohol-related cirrhosis
- Maternal aunt: Lung cancer (never smoker)
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
She denies alcohol use, recent travel, or any known contacts with hepatitis.

PHYSICAL EXAMINATION:
On examination, she is afebrile, with scleral icterus and mild right upper quadrant tenderness. There is no hepatosplenomegaly or peripheral edema.

LABORATORY DATA:
Laboratory tests reveal:

- AST: 450 U/L (Normal: 10-40 U/L)
- ALT: 520 U/L (Normal: 7-56 U/L)
- Alkaline phosphatase: 120 U/L (Normal: 44-147 U/L)
- Total bilirubin: 3.2 mg/dL (Normal: 0.1-1.2 mg/dL)
- Direct bilirubin: 2.1 mg/dL (Normal: 0.0-0.3 mg/dL)
- INR: 1.1 (Normal: 0.8-1.2)

## Encounter 5: source 2022-07-15, practice 2025-07-02, icu, Gastroenterology

Chief complaint: Repeated episodes of vomiting bright red blood over the past 4 hours with progressive abdominal distension and easy bruising noted over the past 6 months.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Maria Gonzalez, MD
Department: Gastroenterology
Visit Type: icu

CHIEF COMPLAINT:
Repeated episodes of vomiting bright red blood over the past 4 hours with progressive abdominal distension and easy bruising noted over the past 6 months.

HISTORY OF PRESENT ILLNESS:
This 52-year-old female with chronic hepatitis C complicated by cryoglobulinemia and prior isoniazid hepatotoxicity presents with progressive abdominal distension and fatigue over the past 6 months. She reports easy bruising and swelling in her ankles during the past few months.

PAST MEDICAL HISTORY:
Active Problem List:
- Pheochromocytoma (diagnosed 2020-01-15)
- Lichen planus (diagnosed 2020-07-15)
- Cryoglobulinemia (diagnosed 2021-01-15)
- Benign oral leukoplakia (diagnosed 2021-07-15)
- Isoniazid-induced liver injury (Drug-induced liver injury) (diagnosed 2022-01-15)
- Chronic hepatitis C infection (genotype 1a, diagnosed 12 years ago, incomplete prior antiviral therapy)
- Hypertension
- History of alcohol use disorder (in sustained remission)

Her medical history is significant for chronic hepatitis C diagnosed 12 years ago, for which she received incomplete antiviral therapy.

PAST SURGICAL HISTORY:
- Recent video-assisted thoracoscopic surgery (VATS) with right lower lobectomy (performed 10 days ago)
- Laparoscopic cholecystectomy (age 42)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg oral daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Hypertension, breast cancer (age 65)
- Father: Alcohol-related cirrhosis
- Maternal aunt: Lung cancer (never smoker)
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
She denies alcohol use for the last 5 years but reports prior daily consumption for more than a decade.

VITAL SIGNS:
Vital signs are: temperature 37.1°C (98.8°F), blood pressure 94/60 mm Hg, heart rate 112/min, respiratory rate 20/min.

PHYSICAL EXAMINATION:
On examination, she appears pale and fatigued. There is scleral icterus, spider angiomata on the chest, and a distended abdomen with a fluid wave. Palpation reveals splenomegaly. Prominent tortuous veins are visible radiating from the umbilicus. Digital rectal exam is negative for blood.

LABORATORY DATA:
Initial laboratory studies show: Hemoglobin 8.5 g/dL Platelet count 84,000/mm³ AST 162 U/L ALT 94 U/L Alkaline phosphatase 188 U/L Total bilirubin 4.3 mg/dL Direct bilirubin 2.1 mg/dL Albumin 2.5 g/dL INR 1.7 Renal function is normal.

OTHER STUDIES:
An urgent upper endoscopy reveals markedly dilated veins in the distal esophagus with stigmata of recent bleeding.

## Encounter 6: source 2023-01-15, practice 2025-12-30, inpatient, Pulmonology

Chief complaint: 3-month history of persistent cough and unintentional weight loss of 10 pounds.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James Wilson, MD
Department: Pulmonology
Visit Type: inpatient

CHIEF COMPLAINT:
3-month history of persistent cough and unintentional weight loss of 10 pounds.

HISTORY OF PRESENT ILLNESS:
This 53-year-old female with chronic hepatitis C infection and prior bleeding esophageal varices presents with a 3-month history of persistent cough and unintentional weight loss of 10 pounds.

PAST MEDICAL HISTORY:
Active Problem List:
- Pheochromocytoma (diagnosed 2020-01-15)
- Lichen planus (diagnosed 2020-07-15)
- Cryoglobulinemia (diagnosed 2021-01-15)
- Benign oral leukoplakia (diagnosed 2021-07-15)
- Isoniazid-induced liver injury (Drug-induced liver injury) (diagnosed 2022-01-15)
- Bleeding esophageal varices (diagnosed 2022-07-15)
- Chronic hepatitis C infection (genotype 1a, diagnosed 12 years ago, incomplete prior antiviral therapy)
- Hypertension
- History of alcohol use disorder (in sustained remission)

PAST SURGICAL HISTORY:
- Recent video-assisted thoracoscopic surgery (VATS) with right lower lobectomy (performed 10 days ago)
- Laparoscopic cholecystectomy (age 42)

MEDICATIONS:
Hydrochlorothiazide

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
She has no family history of lung cancer.

SOCIAL HISTORY:
She denies any history of smoking or exposure to secondhand smoke.

VITAL SIGNS:
Afebrile, with a blood pressure of 130/85 mmHg, heart rate of 78 beats per minute, and respiratory rate of 16 breaths per minute.

PHYSICAL EXAMINATION:
Lung auscultation reveals decreased breath sounds in the right lower lobe without wheezes, rales, or rhonchi.

IMAGING:
A chest X-ray shows a 2.5 cm mass in the periphery of the right lower lobe. A subsequent CT scan of the chest confirms the presence of a solitary peripheral lung nodule with spiculated margins. No lymphadenopathy or pleural effusion is noted.

## Encounter 7: source 2023-02-15, practice 2026-01-25, ed, Emergency Medicine

Chief complaint: Sudden-onset shortness of breath and pleuritic chest pain that started 2 hours ago, occurring 10 days after VATS right lower lobectomy.

ENCOUNTER NOTE
Date: 2023-02-15
Provider: Dr. Fatima Al-Rashid, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset shortness of breath and pleuritic chest pain that started 2 hours ago, occurring 10 days after VATS right lower lobectomy.

HISTORY OF PRESENT ILLNESS:
This 53-year-old female with chronic hepatitis C and Stage IIIB non-small cell lung cancer underwent VATS right lower lobectomy 10 days ago and now presents with acute complaints. She describes sharp pain worsening with deep breaths, reports mild cough with scant blood-tinged sputum, and has been mostly bedridden since the surgery.

PAST MEDICAL HISTORY:
Active Problem List:
- Pheochromocytoma (diagnosed 2020-01-15)
- Lichen planus (diagnosed 2020-07-15)
- Cryoglobulinemia (diagnosed 2021-01-15)
- Benign oral leukoplakia (diagnosed 2021-07-15)
- Isoniazid-induced liver injury (Drug-induced liver injury) (diagnosed 2022-01-15)
- Bleeding esophageal varices (diagnosed 2022-07-15)
- Stage IIIB Non-Small Cell Lung Cancer (diagnosed 2023-01-15)
- Chronic hepatitis C infection (genotype 1a, diagnosed 12 years ago, incomplete prior antiviral therapy)
- Hypertension
- History of alcohol use disorder (in sustained remission)

Her past medical history includes hypertension and.

PAST SURGICAL HISTORY:
A recent total knee replacement surgery 2 weeks ago.

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg oral daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Hypertension, breast cancer (age 65)
- Father: Alcohol-related cirrhosis
- Maternal aunt: Lung cancer (never smoker)
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker, denies secondhand smoke exposure. Alcohol: Former daily heavy consumption (>10 years), abstinent for 5 years. Occupation: Office Administrator.

VITAL SIGNS:
Tachypneic with a respiratory rate of 28 breaths/min, and tachycardic with a heart rate of 110 beats/min. Her blood pressure is 130/85 mmHg. Oxygen saturation on room air is 89%.

PHYSICAL EXAMINATION:
On examination, she is anxious,
A chest examination reveals decreased breath sounds at the right lung base without wheezes or crackles. The remainder of the physical examination is unremarkable.

LABORATORY DATA:
Arterial blood gas analysis on room air shows: - pH: 7.48 - PaCO2: 30 mmHg - PaO2: 70 mmHg - HCO3-: 24 mEq/L D-dimer levels are elevated.

IMAGING:
A chest X-ray is normal.

OTHER STUDIES:
An electrocardiogram shows sinus tachycardia.
