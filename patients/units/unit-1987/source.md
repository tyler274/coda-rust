# synthetic_hospital patient 1987

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 55 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-03-24 (Fri) | outpatient | Hematology & Oncology |
| 1 | 2020-04-15 | 2023-06-30 (Fri) | outpatient | Cardiology |
| 2 | 2020-07-15 | 2023-09-26 (Tue) | outpatient | Gastroenterology |
| 3 | 2021-01-15 | 2024-03-30 (Sat) | inpatient | General Surgery |
| 4 | 2021-07-15 | 2024-09-24 (Tue) | ed | Emergency Medicine |
| 5 | 2022-01-15 | 2025-03-27 (Thu) | ed | Emergency Medicine |
| 6 | 2022-07-15 | 2025-09-17 (Wed) | ed | Emergency Medicine |
| 7 | 2023-01-15 | 2026-03-15 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Former construction worker, currently unemployed",
 "alcohol_use": "Severe alcohol use disorder with multiple periods of abstinence and relapse; history of heavy daily whiskey consumption (7+ years of multiple glasses daily) with recent enrollment in rehabilitation programs and intermittent return to use",
 "family_history": [
  "Father: Alcohol use disorder with cirrhosis",
  "Mother: Type 2 diabetes mellitus",
  "Paternal uncle: Hypertension",
  "Maternal grandfather: Male breast cancer (BRCA2 suspected)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "20 mg twice daily",
   "name": "Propranolol"
  },
  {
   "dose": "100 mg daily",
   "name": "Spironolactone"
  },
  {
   "dose": "40 mg daily",
   "name": "Furosemide"
  },
  {
   "dose": "30 mL twice daily",
   "name": "Lactulose"
  },
  {
   "dose": "24 units at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "36,000 units with meals",
   "name": "Pancrelipase"
  }
 ],
 "surgical_history": [
  "Laparoscopic resection of hepatocellular carcinoma (complicated by postoperative alcohol withdrawal)",
  "Diagnostic paracentesis (multiple)"
 ],
 "chronic_conditions": [
  "Alcohol use disorder, severe",
  "Alcoholic cirrhosis with portal hypertension",
  "History of hepatocellular carcinoma",
  "Type 2 diabetes mellitus",
  "Hypertension",
  "Chronic pancreatitis",
  "Alcoholic cardiomyopathy",
  "History of malignant neoplasm of male breast"
 ],
 "age_at_first_encounter": 55
}
```

Primary diagnoses: ["Alcoholic cirrhosis of liver", "Spontaneous bacterial peritonitis", "Alcohol withdrawal delirium", "Self-limited upper gastrointestinal bleed", "Bleeding esophageal varices", "Alcoholic cardiomyopathy", "Malignant neoplasm of male breast"]
Comorbidities: ["Supratherapeutic INR", "Alcoholic cirrhosis of liver", "Alcohol use disorder", "Thrombocytopenia", "Traumatic coagulopathy", "Hypoalbuminemia", "Acute kidney injury", "Hypovolemic shock", "Essential hypertension", "Acute traumatic coagulopathy"]

## Encounter 0: source 2020-01-15, practice 2023-03-24, outpatient, Hematology & Oncology

Chief complaint: Palpable mass in right breast with nipple retraction

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Palpable mass in right breast with nipple retraction

HISTORY OF PRESENT ILLNESS:
2-month history of a palpable mass in his right breast. He reports no pain but has noticed some nipple retraction.

PAST MEDICAL HISTORY:
Active Problem List:
- Alcohol use disorder, severe
- Alcoholic cirrhosis with portal hypertension
- History of hepatocellular carcinoma
- Type 2 diabetes mellitus
- Hypertension
- Chronic pancreatitis
- Alcoholic cardiomyopathy
- History of malignant neoplasm of male breast

PAST SURGICAL HISTORY:
- Laparoscopic resection of hepatocellular carcinoma (complicated by postoperative alcohol withdrawal)
- Diagnostic paracentesis (multiple)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Propranolol 20 mg twice daily
- Spironolactone 100 mg daily
- Furosemide 40 mg daily
- Lactulose 30 mL twice daily
- Insulin glargine 24 units at bedtime
- Pancrelipase 36,000 units with meals

ALLERGIES:
NKDA

FAMILY HISTORY:
No family history of breast cancer

SOCIAL HISTORY:
History of alcohol abuse

PHYSICAL EXAMINATION:
Unilateral, firm, fixed mass in the upper outer quadrant of his right breast, with associated nipple retraction. There is no axillary lymphadenopathy.

LABORATORY DATA:
AST: 68 U/L (Normal: 10-40 U/L)
- ALT: 55 U/L (Normal: 7-56 U/L)
- Alkaline Phosphatase: 130 U/L (Normal: 40-130 U/L)
- Total Bilirubin: 1.2 mg/dL (Normal: 0.1-1.2 mg/dL)

## Encounter 1: source 2020-04-15, practice 2023-06-30, outpatient, Cardiology

Chief complaint: Progressive fatigue and reduced exercise tolerance

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue and reduced exercise tolerance

HISTORY OF PRESENT ILLNESS:
The patient with alcoholic cardiomyopathy returns following January 2020 Hematology & Oncology evaluation for a breast mass with nipple retraction, reporting progressive fatigue and reduced exercise tolerance over the past few months. He notes increasing difficulty climbing stairs or walking uphill without feeling winded. He denies chest pain, palpitations, syncope, or orthopnea, but does mention mild bilateral ankle swelling at the end of the day.

PAST MEDICAL HISTORY:
Active Problem List:
- Malignant neoplasm of male breast (diagnosed 2020-01-15)
- Alcohol use disorder, severe
- Alcoholic cirrhosis with portal hypertension
- History of hepatocellular carcinoma
- Type 2 diabetes mellitus
- Hypertension
- Chronic pancreatitis
- Alcoholic cardiomyopathy

PAST SURGICAL HISTORY:
- Laparoscopic resection of hepatocellular carcinoma (complicated by postoperative alcohol withdrawal)
- Diagnostic paracentesis (multiple)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Propranolol 20 mg twice daily
- Spironolactone 100 mg daily
- Furosemide 40 mg daily
- Lactulose 30 mL twice daily
- Insulin glargine 24 units at bedtime
- Pancrelipase 36,000 units with meals

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis
- Mother: Type 2 diabetes mellitus
- Paternal uncle: Hypertension
- Maternal grandfather: Male breast cancer (BRCA2 suspected)

SOCIAL HISTORY:
He does not smoke or use illicit drugs.

VITAL SIGNS:
Blood pressure is 106/68 mm Hg, heart rate is 106/min, and respiratory rate is 18/min

PHYSICAL EXAMINATION:
Cardiopulmonary exam reveals a laterally displaced point of maximal impulse in the anterior axillary line and a low-pitched early diastolic sound heard best at the apex in the left lateral decubitus position. No murmurs or gallops are noted otherwise. Bibasilar crackles are present.

IMAGING:
A chest x-ray is obtained and is shown below.

## Encounter 2: source 2020-07-15, practice 2023-09-26, outpatient, Gastroenterology

Chief complaint: Routine follow-up for cirrhosis with fatigue and mild abdominal discomfort

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for cirrhosis with fatigue and mild abdominal discomfort

HISTORY OF PRESENT ILLNESS:
The patient returns for routine cirrhosis follow-up following recent Cardiology evaluation and prior Oncology treatment. He has been sober for the past 6 months and is currently enrolled in an alcohol rehabilitation program. He reports fatigue and mild abdominal discomfort for the past 2 weeks.

PAST MEDICAL HISTORY:
Active Problem List:
- Malignant neoplasm of male breast (diagnosed 2020-01-15)
- Alcoholic cardiomyopathy (diagnosed 2020-04-15)
- Alcohol use disorder, severe
- Alcoholic cirrhosis with portal hypertension
- History of hepatocellular carcinoma
- Type 2 diabetes mellitus
- Hypertension
- Chronic pancreatitis

PAST SURGICAL HISTORY:
- Laparoscopic resection of hepatocellular carcinoma (complicated by postoperative alcohol withdrawal)
- Diagnostic paracentesis (multiple)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Propranolol 20 mg twice daily
- Spironolactone 100 mg daily
- Furosemide 40 mg daily
- Lactulose 30 mL twice daily
- Insulin glargine 24 units at bedtime
- Pancrelipase 36,000 units with meals

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis
- Mother: Type 2 diabetes mellitus
- Paternal uncle: Hypertension
- Maternal grandfather: Male breast cancer (BRCA2 suspected)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Severe alcohol use disorder with multiple periods of abstinence and relapse; history of heavy daily whiskey consumption (7+ years of multiple glasses daily) with recent enrollment in rehabilitation programs and intermittent return to use. Occupation: Former construction worker, currently unemployed.

PHYSICAL EXAMINATION:
On physical examination, you note mild jaundice, palmar erythema, and an enlarged, firm liver edge. His spleen is not palpable. Ascites is not present.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 12.5 g/dL
- Platelet count: 110,000/uL
- INR: 1.3
- Total bilirubin: 2.8 mg/dL
- Albumin: 3.2 g/dL
- AST: 80 U/L
- ALT: 60 U/L
- Alkaline phosphatase: 120 U/L

IMAGING:
Abdominal ultrasound shows a nodular liver contour and a patent portal vein with no evidence of thrombosis.

## Encounter 3: source 2021-01-15, practice 2024-03-30, inpatient, General Surgery

Chief complaint: Agitation, combativeness, and visual hallucinations postoperatively

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James O'Brien, MD
Department: General Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Agitation, combativeness, and visual hallucinations postoperatively

HISTORY OF PRESENT ILLNESS:
The patient with a history of male breast cancer, alcoholic cardiomyopathy, and cirrhosis is hospitalized for laparoscopic resection of hepatocellular carcinoma. On postoperative day 2, he becomes increasingly agitated, combative, and confused. He is oriented only to person and reports seeing spiders crawling on the walls.

PAST MEDICAL HISTORY:
Active Problem List:
- Malignant neoplasm of male breast (diagnosed 2020-01-15)
- Alcoholic cardiomyopathy (diagnosed 2020-04-15)
- Alcoholic cirrhosis of liver (diagnosed 2020-07-15)
- Alcohol use disorder, severe
- Alcoholic cirrhosis with portal hypertension
- History of hepatocellular carcinoma
- Type 2 diabetes mellitus
- Hypertension
- Chronic pancreatitis

His medical history includes cirrhosis due to chronic alcohol use.

PAST SURGICAL HISTORY:
- Laparoscopic resection of hepatocellular carcinoma (complicated by postoperative alcohol withdrawal)
- Diagnostic paracentesis (multiple)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Propranolol 20 mg twice daily
- Spironolactone 100 mg daily
- Furosemide 40 mg daily
- Lactulose 30 mL twice daily
- Insulin glargine 24 units at bedtime
- Pancrelipase 36,000 units with meals

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis
- Mother: Type 2 diabetes mellitus
- Paternal uncle: Hypertension
- Maternal grandfather: Male breast cancer (BRCA2 suspected)

SOCIAL HISTORY:
He reports drinking multiple glasses of whiskey daily for the past 7 years

VITAL SIGNS:
His temperature is 38.5°C (101.3°F), blood pressure is 170/100 mm Hg, pulse is 112/min, and respirations are 24/min

PHYSICAL EXAMINATION:
Physical examination shows a coarse hand tremor

LABORATORY DATA:
Laboratory studies show:
Platelet count                                    46,000/mm³
Prothrombin time                              15.8 sec
Serum albumin                                  2.2 g/dL
Total bilirubin                                     3.1 mg/dL
AST                                                   66 U/L
ALT                                                    98 U/L
Urine toxicology is negative.

## Encounter 4: source 2021-07-15, practice 2024-09-24, ed, Emergency Medicine

Chief complaint: Hematemesis and melena following recent alcohol use

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Hematemesis and melena following recent alcohol use

HISTORY OF PRESENT ILLNESS:
The patient with cirrhosis and prior alcohol withdrawal presents. He reports a two-day history of progressive abdominal distension and discomfort. His last alcoholic drink was 12 hours ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Malignant neoplasm of male breast (diagnosed 2020-01-15)
- Alcoholic cardiomyopathy (diagnosed 2020-04-15)
- Alcoholic cirrhosis of liver (diagnosed 2020-07-15)
- Alcohol withdrawal delirium (diagnosed 2021-01-15)
- Alcohol use disorder, severe
- Alcoholic cirrhosis with portal hypertension
- History of hepatocellular carcinoma
- Type 2 diabetes mellitus
- Hypertension
- Chronic pancreatitis

PAST SURGICAL HISTORY:
- Laparoscopic resection of hepatocellular carcinoma (complicated by postoperative alcohol withdrawal)
- Diagnostic paracentesis (multiple)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Propranolol 20 mg twice daily
- Spironolactone 100 mg daily
- Furosemide 40 mg daily
- Lactulose 30 mL twice daily
- Insulin glargine 24 units at bedtime
- Pancrelipase 36,000 units with meals

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis
- Mother: Type 2 diabetes mellitus
- Paternal uncle: Hypertension
- Maternal grandfather: Male breast cancer (BRCA2 suspected)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Severe alcohol use disorder with multiple periods of abstinence and relapse; history of heavy daily whiskey consumption (7+ years of multiple glasses daily) with recent enrollment in rehabilitation programs and intermittent return to use. Occupation: Former construction worker, currently unemployed.

VITAL SIGNS:
His blood pressure is 90/60 mmHg, heart rate is 110 bpm

PHYSICAL EXAMINATION:
He appears pale and diaphoretic. Abdominal examination reveals ascites and caput medusae.

LABORATORY DATA:
- Hemoglobin: 8.2 g/dL (Normal: 13.8-17.2 g/dL)
- Platelet count: 55 x 10^9/L (Normal: 150-400 x 10^9/L)
- INR: 1.8 (Normal: 0.8-1.2)
- Creatinine: 1.1 mg/dL (Normal: 0.7-1.3 mg/dL)
- BUN: 29 mg/dL (Normal: 6-20 mg/dL)

PLAN:
After initial fluid resuscitation with crystalloids,

## Encounter 5: source 2022-01-15, practice 2025-03-27, ed, Emergency Medicine

Chief complaint: Hematemesis and altered mental status

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Hematemesis and altered mental status

HISTORY OF PRESENT ILLNESS:
Patient returns with recurrent hematemesis secondary to cirrhosis. His wife reports that he has been vomiting blood since early morning and has become increasingly lethargic.

PAST MEDICAL HISTORY:
Active Problem List:
- Malignant neoplasm of male breast (diagnosed 2020-01-15)
- Alcoholic cardiomyopathy (diagnosed 2020-04-15)
- Alcoholic cirrhosis of liver (diagnosed 2020-07-15)
- Alcohol withdrawal delirium (diagnosed 2021-01-15)
- Bleeding esophageal varices (diagnosed 2021-07-15)
- Alcohol use disorder, severe
- Alcoholic cirrhosis with portal hypertension
- History of hepatocellular carcinoma
- Type 2 diabetes mellitus
- Hypertension
- Chronic pancreatitis

PAST SURGICAL HISTORY:
- Laparoscopic resection of hepatocellular carcinoma (complicated by postoperative alcohol withdrawal)
- Diagnostic paracentesis (multiple)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Propranolol 20 mg twice daily
- Spironolactone 100 mg daily
- Furosemide 40 mg daily
- Lactulose 30 mL twice daily
- Insulin glargine 24 units at bedtime
- Pancrelipase 36,000 units with meals

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis
- Mother: Type 2 diabetes mellitus
- Paternal uncle: Hypertension
- Maternal grandfather: Male breast cancer (BRCA2 suspected)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Severe alcohol use disorder with multiple periods of abstinence and relapse; history of heavy daily whiskey consumption (7+ years of multiple glasses daily) with recent enrollment in rehabilitation programs and intermittent return to use. Occupation: Former construction worker, currently unemployed.

VITAL SIGNS:
Blood pressure of 90/60 mmHg, heart rate of 122 bpm, and respiratory rate of 24 breaths per minute

PHYSICAL EXAMINATION:
On examination, the patient is somnolent but arousable. He has pallor and his breath smells of alcohol. His abdomen is soft with no hepatosplenomegaly, but there is mild tenderness in the epigastric region.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 7.8 g/dL (Normal: 13.8-17.2 g/dL)
- Platelets: 110,000/uL (Normal: 150,000-450,000/uL)
- INR: 1.8 (Normal: 0.8-1.2)
- BUN: 29 mg/dL (Normal: 7-20 mg/dL)
- Creatinine: 1.2 mg/dL (Normal: 0.6-1.2 mg/dL)

## Encounter 6: source 2022-07-15, practice 2025-09-17, ed, Emergency Medicine

Chief complaint: Abdominal distension and discomfort with shortness of breath

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Maria Garcia, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Abdominal distension and discomfort with shortness of breath

HISTORY OF PRESENT ILLNESS:
The patient with alcoholic cirrhosis with portal hypertension and recent upper gastrointestinal bleeding returns with increasing shortness of breath and notes his abdomen has become markedly swollen. He denies any fever, vomiting, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Malignant neoplasm of male breast (diagnosed 2020-01-15)
- Alcoholic cardiomyopathy (diagnosed 2020-04-15)
- Alcoholic cirrhosis of liver (diagnosed 2020-07-15)
- Alcohol withdrawal delirium (diagnosed 2021-01-15)
- Bleeding esophageal varices (diagnosed 2021-07-15)
- Self-limited upper gastrointestinal bleed (diagnosed 2022-01-15)
- Alcohol use disorder, severe
- Alcoholic cirrhosis with portal hypertension
- History of hepatocellular carcinoma
- Type 2 diabetes mellitus
- Hypertension
- Chronic pancreatitis

PAST SURGICAL HISTORY:
- Laparoscopic resection of hepatocellular carcinoma (complicated by postoperative alcohol withdrawal)
- Diagnostic paracentesis (multiple)

MEDICATIONS:
He takes lisinopril.

Home Medications:
- Propranolol 20 mg twice daily
- Spironolactone 100 mg daily
- Furosemide 40 mg daily
- Lactulose 30 mL twice daily
- Insulin glargine 24 units at bedtime
- Pancrelipase 36,000 units with meals

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis
- Mother: Type 2 diabetes mellitus
- Paternal uncle: Hypertension
- Maternal grandfather: Male breast cancer (BRCA2 suspected)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Severe alcohol use disorder with multiple periods of abstinence and relapse; history of heavy daily whiskey consumption (7+ years of multiple glasses daily) with recent enrollment in rehabilitation programs and intermittent return to use. Occupation: Former construction worker, currently unemployed.

VITAL SIGNS:
Vital signs are stable

PHYSICAL EXAMINATION:
He appears jaundiced. Abdominal examination reveals a fluid wave and shifting dullness

LABORATORY DATA:
Thrombocytopenia and an elevated INR

OTHER STUDIES:
A diagnostic paracentesis is performed, and the ascitic fluid analysis is as follows:

- Color: Straw-colored
- Cell count: 250 cells/mm³ (70% neutrophils)
- Total protein: 1.1 g/dL
- Serum albumin: 3.0 g/dL
- Ascitic fluid albumin: 1.0 g/dL

## Encounter 7: source 2023-01-15, practice 2026-03-15, ed, Emergency Medicine

Chief complaint: Fever, abdominal pain, and confusion

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Smith, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, abdominal pain, and confusion

HISTORY OF PRESENT ILLNESS:
The patient with alcoholic cirrhosis and prior spontaneous bacterial peritonitis six months ago returns. His wife reports that he has been less responsive and has not been eating well for the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Malignant neoplasm of male breast (diagnosed 2020-01-15)
- Alcoholic cardiomyopathy (diagnosed 2020-04-15)
- Alcoholic cirrhosis of liver (diagnosed 2020-07-15)
- Alcohol withdrawal delirium (diagnosed 2021-01-15)
- Bleeding esophageal varices (diagnosed 2021-07-15)
- Self-limited upper gastrointestinal bleed (diagnosed 2022-01-15)
- Spontaneous bacterial peritonitis (diagnosed 2022-07-15)
- Alcohol use disorder, severe
- Alcoholic cirrhosis with portal hypertension
- History of hepatocellular carcinoma
- Type 2 diabetes mellitus
- Hypertension
- Chronic pancreatitis

PAST SURGICAL HISTORY:
- Laparoscopic resection of hepatocellular carcinoma (complicated by postoperative alcohol withdrawal)
- Diagnostic paracentesis (multiple)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Propranolol 20 mg twice daily
- Spironolactone 100 mg daily
- Furosemide 40 mg daily
- Lactulose 30 mL twice daily
- Insulin glargine 24 units at bedtime
- Pancrelipase 36,000 units with meals

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis
- Mother: Type 2 diabetes mellitus
- Paternal uncle: Hypertension
- Maternal grandfather: Male breast cancer (BRCA2 suspected)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Severe alcohol use disorder with multiple periods of abstinence and relapse; history of heavy daily whiskey consumption (7+ years of multiple glasses daily) with recent enrollment in rehabilitation programs and intermittent return to use. Occupation: Former construction worker, currently unemployed.

VITAL SIGNS:
On examination, the patient is afebrile, blood pressure is 100/60 mmHg, heart rate is 88/min, and respiratory rate is 16/min.

PHYSICAL EXAMINATION:
He appears lethargic but is oriented to person and place, not to time. His abdomen is distended with a fluid wave and shifting dullness. Bowel sounds are normal. No signs of peripheral edema are noted. Neurological examination reveals no focal deficits, but he is unable to complete the Reitan trail test.

LABORATORY DATA:
Laboratory studies reveal:

- White blood cell count: 12,000/mm³
- Hemoglobin: 10.2 g/dL
- Platelet count: 85,000/mm³
- Serum creatinine: 1.3 mg/dL
- Blood urea nitrogen: 29 mg/dL
- Serum albumin: 2.8 g/dL
- Total bilirubin: 3.5 mg/dL
- INR: 1.6
- Sodium: 130 mEq/L
- Ascitic fluid analysis: WBC 1500/mm³ with 70% neutrophils
