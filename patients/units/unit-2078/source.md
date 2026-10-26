# synthetic_hospital patient 2078

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 56 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-04-04 (Mon) | outpatient | Hepatology |
| 1 | 2020-09-15 | 2022-11-27 (Sun) | ed | Emergency Medicine |
| 2 | 2021-03-15 | 2023-06-01 (Thu) | ed | Emergency Medicine |
| 3 | 2022-01-15 | 2024-04-01 (Mon) | ed | Emergency Medicine |
| 4 | 2023-01-15 | 2025-03-29 (Sat) | ed | Emergency Medicine |
| 5 | 2024-01-15 | 2026-03-30 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Former construction worker, currently on disability",
 "alcohol_use": "Severe alcohol use disorder; heavy episodic drinking averaging 6-8 standard drinks daily for 30 years with multiple failed attempts at cessation",
 "family_history": [
  "Father: Alcohol use disorder with cirrhosis, deceased age 62",
  "Mother: Major depressive disorder, age 78",
  "Brother: Chronic hepatitis C infection"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 20 pack-years, quit 5 years ago",
 "home_medications": [
  {
   "dose": "30 mL orally twice daily",
   "name": "Lactulose"
  },
  {
   "dose": "550 mg orally twice daily",
   "name": "Rifaximin"
  },
  {
   "dose": "20 mg orally twice daily",
   "name": "Propranolol"
  },
  {
   "dose": "10 mg orally daily",
   "name": "Escitalopram"
  },
  {
   "dose": "100 mg orally daily",
   "name": "Spironolactone"
  },
  {
   "dose": "40 mg orally daily",
   "name": "Furosemide"
  }
 ],
 "surgical_history": [
  "Esophagogastroduodenoscopy with variceal band ligation",
  "Diagnostic paracentesis",
  "Liver biopsy (percutaneous)"
 ],
 "chronic_conditions": [
  "Alcohol-associated cirrhosis with portal hypertension",
  "Chronic hepatitis C infection (Genotype 1a)",
  "Recurrent hepatic encephalopathy",
  "Esophageal varices with history of bleeding",
  "Generalized anxiety disorder",
  "Ascites"
 ],
 "age_at_first_encounter": 56
}
```

Primary diagnoses: ["Hepatic encephalopathy", "Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate)", "Accidental hypothermia (moderate)"]
Comorbidities: ["Alcoholic cirrhosis of liver", "Alcohol use disorder", "Ascites", "Hepatic cirrhosis with portal hypertension"]

## Encounter 0: source 2020-01-15, practice 2022-04-04, outpatient, Hepatology

Chief complaint: 3-month history of worsening fatigue and abdominal distention

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Hepatology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of worsening fatigue and abdominal distention

PAST MEDICAL HISTORY:
Active Problem List:
- Alcohol-associated cirrhosis with portal hypertension
- Chronic hepatitis C infection (Genotype 1a)
- Recurrent hepatic encephalopathy
- Esophageal varices with history of bleeding
- Generalized anxiety disorder
- Ascites

PAST SURGICAL HISTORY:
- Esophagogastroduodenoscopy with variceal band ligation
- Diagnostic paracentesis
- Liver biopsy (percutaneous)

MEDICATIONS:
Home Medications:
- Lactulose 30 mL orally twice daily
- Rifaximin 550 mg orally twice daily
- Propranolol 20 mg orally twice daily
- Escitalopram 10 mg orally daily
- Spironolactone 100 mg orally daily
- Furosemide 40 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis, deceased age 62
- Mother: Major depressive disorder, age 78
- Brother: Chronic hepatitis C infection

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Severe alcohol use disorder; heavy episodic drinking averaging 6-8 standard drinks daily for 30 years with multiple failed attempts at cessation. Occupation: Former construction worker, currently on disability.

PHYSICAL EXAMINATION:
Scleral icterus, an abdominal fluid wave, and scattered spider angiomata

IMAGING:
CT of abdomen with contrast shows a mass within the right portion of the liver

PATHOLOGY:
Biopsy shows malignant cells with a 1:1 nuclear-to-cytoplasm ratio confined to the basement membranes

## Encounter 1: source 2020-09-15, practice 2022-11-27, ed, Emergency Medicine

Chief complaint: progressive confusion, lethargy, and difficulty speaking

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive confusion, lethargy, and difficulty speaking

HISTORY OF PRESENT ILLNESS:
The patient with alcohol-associated cirrhosis and recurrent hepatic encephalopathy, last seen in Hepatology eight months prior, presents to the emergency department with a two-day history of progressive confusion, lethargy, and difficulty speaking. His wife reports that he has not been eating well and has been drinking alcohol more frequently.

PAST MEDICAL HISTORY:
Active Problem List:
- Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate) (diagnosed 2020-01-15)
- Alcohol-associated cirrhosis with portal hypertension
- Chronic hepatitis C infection (Genotype 1a)
- Recurrent hepatic encephalopathy
- Esophageal varices with history of bleeding
- Generalized anxiety disorder
- Ascites

PAST SURGICAL HISTORY:
- Esophagogastroduodenoscopy with variceal band ligation
- Diagnostic paracentesis
- Liver biopsy (percutaneous)

MEDICATIONS:
Home Medications:
- Lactulose 30 mL orally twice daily
- Rifaximin 550 mg orally twice daily
- Propranolol 20 mg orally twice daily
- Escitalopram 10 mg orally daily
- Spironolactone 100 mg orally daily
- Furosemide 40 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis, deceased age 62
- Mother: Major depressive disorder, age 78
- Brother: Chronic hepatitis C infection

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Severe alcohol use disorder; heavy episodic drinking averaging 6-8 standard drinks daily for 30 years with multiple failed attempts at cessation. Occupation: Former construction worker, currently on disability.

VITAL SIGNS:
Blood pressure 130/85 mmHg, heart rate 102 beats per minute, respiratory rate 18 breaths per minute, and temperature 37.1°C (98.8°F)

PHYSICAL EXAMINATION:
On examination, the patient is drowsy but arousable. Neurological examination reveals bilateral hand tremors and flapping of the hands when the wrists are extended.

LABORATORY DATA:
Serum ammonia: 150 µmol/L (normal range: 11-35 µmol/L)
- Aspartate aminotransferase (AST): 120 U/L (normal range: 8-48 U/L)
- Alanine aminotransferase (ALT): 100 U/L (normal range: 7-55 U/L)
- Alkaline phosphatase (ALP): 90 U/L (normal range: 40-129 U/L)
- Total bilirubin: 2.5 mg/dL (normal range: 0.1-1.2 mg/dL)
- Blood urea nitrogen (BUN): 18 mg/dL (normal range: 6-20 mg/dL)
- Creatinine: 1.0 mg/dL (normal range: 0.6-1.2 mg/dL)
- Arterial blood gas (ABG): pH 7.42, PaCO2 40 mmHg, PaO2 95 mmHg, HCO3- 24 mEq/L

## Encounter 2: source 2021-03-15, practice 2023-06-01, ed, Emergency Medicine

Chief complaint: confusion, disorientation, and difficulty concentrating

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Confusion, disorientation, and difficulty concentrating

HISTORY OF PRESENT ILLNESS:
The patient with alcohol-associated cirrhosis and recurrent hepatic encephalopathy returns to the emergency department with confusion, disorientation, and difficulty concentrating similar to his September 2020 presentation. His family reports that he has been increasingly lethargic over the past 2 days and had a recent upper GI bleed.

PAST MEDICAL HISTORY:
Active Problem List:
- Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate) (diagnosed 2020-01-15)
- Hepatic encephalopathy (diagnosed 2020-09-15)
- Alcohol-associated cirrhosis with portal hypertension
- Chronic hepatitis C infection (Genotype 1a)
- Recurrent hepatic encephalopathy
- Esophageal varices with history of bleeding
- Generalized anxiety disorder
- Ascites

PAST SURGICAL HISTORY:
- Esophagogastroduodenoscopy with variceal band ligation
- Diagnostic paracentesis
- Liver biopsy (percutaneous)

MEDICATIONS:
Home Medications:
- Lactulose 30 mL orally twice daily
- Rifaximin 550 mg orally twice daily
- Propranolol 20 mg orally twice daily
- Escitalopram 10 mg orally daily
- Spironolactone 100 mg orally daily
- Furosemide 40 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis, deceased age 62
- Mother: Major depressive disorder, age 78
- Brother: Chronic hepatitis C infection

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Severe alcohol use disorder; heavy episodic drinking averaging 6-8 standard drinks daily for 30 years with multiple failed attempts at cessation. Occupation: Former construction worker, currently on disability.

PHYSICAL EXAMINATION:
On exam, the patient demonstrates asterixis and slowed mental status.

LABORATORY DATA:
Laboratory tests show elevated serum ammonia levels.

ASSESSMENT AND PLAN:
Hepatic encephalopathy

He is admitted for hepatic encephalopathy and started on oral lactulose. Over the next 24 hours, his mental status begins to improve.

## Encounter 3: source 2022-01-15, practice 2024-04-01, ed, Emergency Medicine

Chief complaint: altered mental status

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Altered mental status

HISTORY OF PRESENT ILLNESS:
The patient returns with recurrent hepatic encephalopathy. His wife reports that he has been increasingly confused and lethargic over the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate) (diagnosed 2020-01-15)
- Hepatic encephalopathy (diagnosed 2020-09-15)
- Alcohol-associated cirrhosis with portal hypertension
- Chronic hepatitis C infection (Genotype 1a)
- Recurrent hepatic encephalopathy
- Esophageal varices with history of bleeding
- Generalized anxiety disorder
- Ascites

PAST SURGICAL HISTORY:
- Esophagogastroduodenoscopy with variceal band ligation
- Diagnostic paracentesis
- Liver biopsy (percutaneous)

MEDICATIONS:
The patient's wife mentions that he was recently prescribed a medication for anxiety by his primary care physician.

Home Medications:
- Lactulose 30 mL orally twice daily
- Rifaximin 550 mg orally twice daily
- Propranolol 20 mg orally twice daily
- Escitalopram 10 mg orally daily
- Spironolactone 100 mg orally daily
- Furosemide 40 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis, deceased age 62
- Mother: Major depressive disorder, age 78
- Brother: Chronic hepatitis C infection

SOCIAL HISTORY:
History of chronic alcohol use

VITAL SIGNS:
His vital signs are stable.

PHYSICAL EXAMINATION:
On examination, the patient is somnolent but arousable, with a noticeable flapping tremor of the hands.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum bilirubin: 3.2 mg/dL (normal 0.3-1.2 mg/dL)
- INR: 1.8 (normal 0.8-1.1)
- Serum albumin: 2.9 g/dL (normal 3.5-5.0 g/dL)
- AST: 80 U/L (normal 10-40 U/L)
- ALT: 50 U/L (normal 7-56 U/L)
- Ammonia: 110 µg/dL (normal 15-45 µg/dL)

## Encounter 4: source 2023-01-15, practice 2025-03-29, ed, Emergency Medicine

Chief complaint: confused, lethargic, and has been speaking incoherently

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Confused, lethargic, and has been speaking incoherently

HISTORY OF PRESENT ILLNESS:
The patient with alcohol-associated cirrhosis and recurrent hepatic encephalopathy returns to the emergency department with his daughter. She reports that over the past week, he has become increasingly confused, lethargic, and has been speaking incoherently.

PAST MEDICAL HISTORY:
Active Problem List:
- Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate) (diagnosed 2020-01-15)
- Hepatic encephalopathy (diagnosed 2020-09-15)
- Alcohol-associated cirrhosis with portal hypertension
- Chronic hepatitis C infection (Genotype 1a)
- Recurrent hepatic encephalopathy
- Esophageal varices with history of bleeding
- Generalized anxiety disorder
- Ascites

His past medical history is significant for cirrhosis secondary to alcohol use.

PAST SURGICAL HISTORY:
- Esophagogastroduodenoscopy with variceal band ligation
- Diagnostic paracentesis
- Liver biopsy (percutaneous)

MEDICATIONS:
Home Medications:
- Lactulose 30 mL orally twice daily
- Rifaximin 550 mg orally twice daily
- Propranolol 20 mg orally twice daily
- Escitalopram 10 mg orally daily
- Spironolactone 100 mg orally daily
- Furosemide 40 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis, deceased age 62
- Mother: Major depressive disorder, age 78
- Brother: Chronic hepatitis C infection

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Severe alcohol use disorder; heavy episodic drinking averaging 6-8 standard drinks daily for 30 years with multiple failed attempts at cessation. Occupation: Former construction worker, currently on disability.

PHYSICAL EXAMINATION:
On examination, the patient is drowsy but arousable, disoriented to time and place, and exhibits asterixis. His abdomen is distended with a fluid wave consistent with ascites.

LABORATORY DATA:
Laboratory studies are as follows:

- Serum ammonia: 150 µmol/L (Normal: 15-45 µmol/L)
- Aspartate aminotransferase (AST): 120 U/L (Normal: 8-48 U/L)
- Alanine aminotransferase (ALT): 90 U/L (Normal: 7-55 U/L)
- Alkaline phosphatase (ALP): 110 U/L (Normal: 40-129 U/L)
- Total bilirubin: 3.2 mg/dL (Normal: 0.1-1.2 mg/dL)
- International Normalized Ratio (INR): 1.6 (Normal: 0.8-1.1)

## Encounter 5: source 2024-01-15, practice 2026-03-30, ed, Emergency Medicine

Chief complaint: found unconscious in snowbank after heavy drinking

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Found unconscious in snowbank after heavy drinking

HISTORY OF PRESENT ILLNESS:
The patient returns for his sixth episode of altered mental status. He was last seen drinking heavily at a local bar the previous night.

PAST MEDICAL HISTORY:
Active Problem List:
- Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate) (diagnosed 2020-01-15)
- Hepatic encephalopathy (diagnosed 2020-09-15)
- Alcohol-associated cirrhosis with portal hypertension
- Chronic hepatitis C infection (Genotype 1a)
- Recurrent hepatic encephalopathy
- Esophageal varices with history of bleeding
- Generalized anxiety disorder
- Ascites

The patient has a history of chronic alcoholism.

PAST SURGICAL HISTORY:
- Esophagogastroduodenoscopy with variceal band ligation
- Diagnostic paracentesis
- Liver biopsy (percutaneous)

MEDICATIONS:
Home Medications:
- Lactulose 30 mL orally twice daily
- Rifaximin 550 mg orally twice daily
- Propranolol 20 mg orally twice daily
- Escitalopram 10 mg orally daily
- Spironolactone 100 mg orally daily
- Furosemide 40 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis, deceased age 62
- Mother: Major depressive disorder, age 78
- Brother: Chronic hepatitis C infection

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Severe alcohol use disorder; heavy episodic drinking averaging 6-8 standard drinks daily for 30 years with multiple failed attempts at cessation. Occupation: Former construction worker, currently on disability.

PHYSICAL EXAMINATION:
On examination, the patient is unresponsive, with a core temperature of 30°C (86°F). His skin is cold to the touch, and he has a weak pulse at a rate of 40 beats per minute. His breathing is shallow, and his blood pressure is 90/60 mmHg.

LABORATORY DATA:
Laboratory values are as follows:

- Sodium: 138 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 100 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen (BUN): 18 mg/dL
- Creatinine: 1.0 mg/dL
- Glucose: 90 mg/dL

OTHER STUDIES:
An electrocardiogram (ECG) shows a sinus bradycardia with J waves (Osborn waves).
