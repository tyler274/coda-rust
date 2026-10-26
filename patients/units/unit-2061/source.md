# synthetic_hospital patient 2061

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 53 at first encounter, sex M, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-08-02 (Mon) | outpatient | Endocrinology |
| 1 | 2020-07-15 | 2022-01-26 (Wed) | outpatient | Gastroenterology |
| 2 | 2021-07-15 | 2023-02-06 (Mon) | outpatient | Gastroenterology |
| 3 | 2022-07-15 | 2024-01-29 (Mon) | outpatient | Infectious Diseases |
| 4 | 2023-07-15 | 2025-02-04 (Tue) | outpatient | Gastroenterology |
| 5 | 2024-01-15 | 2025-08-01 (Fri) | outpatient | Gastroenterology |
| 6 | 2024-07-15 | 2026-01-31 (Sat) | ed | Emergency Medicine |
| 7 | 2025-01-15 | 2026-07-26 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Automotive parts manager",
 "alcohol_use": "Heavy daily use (6 standard beers per day for the past 15 years)",
 "family_history": [
  "Father: Deceased at age 58 from complications of alcoholic cirrhosis and hepatocellular carcinoma",
  "Mother: Living, hypertension and hyperlipidemia",
  "Brother: Living, chronic hepatitis C infection",
  "Paternal uncle: Alcohol use disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 8 years ago",
 "home_medications": [
  {
   "dose": "40 mg PO daily",
   "name": "Furosemide"
  },
  {
   "dose": "100 mg PO daily",
   "name": "Spironolactone"
  },
  {
   "dose": "20 mg PO twice daily",
   "name": "Propranolol"
  },
  {
   "dose": "200 mg IM every 2 weeks",
   "name": "Testosterone cypionate"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Vitamin K"
  },
  {
   "dose": "30 mL PO twice daily as needed for constipation",
   "name": "Lactulose"
  }
 ],
 "surgical_history": [
  "Esophagogastroduodenoscopy with esophageal variceal band ligation (two prior hospitalizations within the past year)",
  "Rubber band ligation of hemorrhoids",
  "Umbilical hernia repair (childhood)"
 ],
 "chronic_conditions": [
  "Chronic hepatitis C infection",
  "Alcoholic cirrhosis with portal hypertension",
  "Kallmann syndrome (congenital hypogonadotropic hypogonadism)",
  "Recurrent internal hemorrhoids",
  "Hypogonadism (multifactorial)"
 ],
 "age_at_first_encounter": 53
}
```

Primary diagnoses: ["Hemorrhoids", "Ascites due to cirrhosis (uncomplicated)", "Hepatorenal syndrome", "Alcoholic cirrhosis", "Kallmann syndrome", "Cirrhosis with acute esophageal variceal hemorrhage due to portal hypertension", "Hepatic cirrhosis with portal hypertension"]
Comorbidities: ["Hyponatremia", "Portal hypertension", "Chronic hepatitis C", "Alcohol use disorder", "Alcoholic cirrhosis", "Ascites", "Jaundice", "Traumatic coagulopathy", "Thrombocytopenia", "Hypoalbuminemia", "Anemia"]

## Encounter 0: source 2020-01-15, practice 2021-08-02, outpatient, Endocrinology

Chief complaint: Decreased libido and erectile dysfunction

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Decreased libido and erectile dysfunction

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with complaints of decreased libido and erectile dysfunction for the past six months.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic hepatitis C infection
- Alcoholic cirrhosis with portal hypertension
- Kallmann syndrome (congenital hypogonadotropic hypogonadism)
- Recurrent internal hemorrhoids
- Hypogonadism (multifactorial)

PAST SURGICAL HISTORY:
- Esophagogastroduodenoscopy with esophageal variceal band ligation (two prior hospitalizations within the past year)
- Rubber band ligation of hemorrhoids
- Umbilical hernia repair (childhood)

MEDICATIONS:
Home Medications:
- Furosemide 40 mg PO daily
- Spironolactone 100 mg PO daily
- Propranolol 20 mg PO twice daily
- Testosterone cypionate 200 mg IM every 2 weeks
- Vitamin K 10 mg PO daily
- Lactulose 30 mL PO twice daily as needed for constipation

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Deceased at age 58 from complications of alcoholic cirrhosis and hepatocellular carcinoma
- Mother: Living, hypertension and hyperlipidemia
- Brother: Living, chronic hepatitis C infection
- Paternal uncle: Alcohol use disorder

SOCIAL HISTORY:
He reports consuming an average of six beers per day for the past 15 years.

PHYSICAL EXAMINATION:
On examination, he appears malnourished with palmar erythema and spider angiomata on his chest. His abdomen is distended with a fluid wave suggestive of ascites. Testicular atrophy is noted on genital examination.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum albumin: 2.9 g/dL (Normal: 3.5-5.0 g/dL)
- Total bilirubin: 3.2 mg/dL (Normal: 0.3-1.2 mg/dL)
- Alkaline phosphatase: 120 U/L (Normal: 40-130 U/L)
- Aspartate aminotransferase (AST): 180 U/L (Normal: 10-40 U/L)
- Alanine aminotransferase (ALT): 150 U/L (Normal: 7-56 U/L)
- INR: 1.6 (Normal: 0.8-1.2)
- Serum estradiol: Elevated

## Encounter 1: source 2020-07-15, practice 2022-01-26, outpatient, Gastroenterology

Chief complaint: Intermittent rectal bleeding noticed as streaks of blood on toilet paper and in the bowl over the past several weeks

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent rectal bleeding noticed as streaks of blood on toilet paper and in the bowl over the past several weeks

HISTORY OF PRESENT ILLNESS:
Following his prior Endocrinology evaluation, this patient with cirrhosis returns reporting rectal bleeding. He reports occasional abdominal fullness, ankle swelling, and easy bruising. Over the last few months, he has developed progressive fatigue and mild shortness of breath with exertion.

PAST MEDICAL HISTORY:
Active Problem List:
- Kallmann syndrome (diagnosed 2020-01-15)
- Chronic hepatitis C infection
- Alcoholic cirrhosis with portal hypertension
- Kallmann syndrome (congenital hypogonadotropic hypogonadism)
- Recurrent internal hemorrhoids
- Hypogonadism (multifactorial)

PAST SURGICAL HISTORY:
- Esophagogastroduodenoscopy with esophageal variceal band ligation (two prior hospitalizations within the past year)
- Rubber band ligation of hemorrhoids
- Umbilical hernia repair (childhood)

MEDICATIONS:
Home Medications:
- Furosemide 40 mg PO daily
- Spironolactone 100 mg PO daily
- Propranolol 20 mg PO twice daily
- Testosterone cypionate 200 mg IM every 2 weeks
- Vitamin K 10 mg PO daily
- Lactulose 30 mL PO twice daily as needed for constipation

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Deceased at age 58 from complications of alcoholic cirrhosis and hepatocellular carcinoma
- Mother: Living, hypertension and hyperlipidemia
- Brother: Living, chronic hepatitis C infection
- Paternal uncle: Alcohol use disorder

SOCIAL HISTORY:
A long history of alcohol use

REVIEW OF SYSTEMS:
He denies abdominal pain, diarrhea, or recent travel.

PHYSICAL EXAMINATION:
On examination, he appears chronically ill, with scleral icterus, spider angiomas on the chest, and mild ascites. His spleen is palpable below the costal margin, and he has bilateral ankle edema. Rectal exam reveals no external masses or fissures but occult blood is detected.

LABORATORY DATA:
Laboratory studies show thrombocytopenia, mild anemia, elevated INR, and elevated liver transaminases.

IMAGING:
Imaging demonstrates a nodular liver consistent with cirrhosis.

OTHER STUDIES:
Endoscopic evaluation reveals dilated venous structures in the rectum consistent with internal hemorrhoids.

## Encounter 2: source 2021-07-15, practice 2023-02-06, outpatient, Gastroenterology

Chief complaint: 2-month history of progressive abdominal distension and discomfort

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Jennifer Park, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
2-month history of progressive abdominal distension and discomfort

HISTORY OF PRESENT ILLNESS:
The patient with alcoholic cirrhosis returns one year after hemorrhoid evaluation. He denies any recent travel or sick contacts. He reports a decreased appetite but no nausea, vomiting, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Kallmann syndrome (diagnosed 2020-01-15)
- Hemorrhoids (diagnosed 2020-07-15)
- Chronic hepatitis C infection
- Alcoholic cirrhosis with portal hypertension
- Kallmann syndrome (congenital hypogonadotropic hypogonadism)
- Recurrent internal hemorrhoids
- Hypogonadism (multifactorial)

PAST SURGICAL HISTORY:
- Esophagogastroduodenoscopy with esophageal variceal band ligation (two prior hospitalizations within the past year)
- Rubber band ligation of hemorrhoids
- Umbilical hernia repair (childhood)

MEDICATIONS:
Home Medications:
- Furosemide 40 mg PO daily
- Spironolactone 100 mg PO daily
- Propranolol 20 mg PO twice daily
- Testosterone cypionate 200 mg IM every 2 weeks
- Vitamin K 10 mg PO daily
- Lactulose 30 mL PO twice daily as needed for constipation

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Deceased at age 58 from complications of alcoholic cirrhosis and hepatocellular carcinoma
- Mother: Living, hypertension and hyperlipidemia
- Brother: Living, chronic hepatitis C infection
- Paternal uncle: Alcohol use disorder

SOCIAL HISTORY:
History of alcohol abuse

PHYSICAL EXAMINATION:
On physical examination, the patient appears cachectic and has palmar erythema. His abdomen is distended with a fluid wave and shifting dullness. There is no peripheral edema.

LABORATORY DATA:
Laboratory studies reveal:

- Serum albumin: 2.8 g/dL
- Serum sodium: 135 mEq/L
- Serum creatinine: 1.1 mg/dL
- Total bilirubin: 3.2 mg/dL
- INR: 1.5
- Ascitic fluid albumin: 1.2 g/dL

A serum-to-ascites albumin gradient (SAAG) is calculated to be 1.6 g/dL.

## Encounter 3: source 2022-07-15, practice 2024-01-29, outpatient, Infectious Diseases

Chief complaint: Presents to the clinic for a routine follow-up

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Priya Sharma, MD
Department: Infectious Diseases
Visit Type: outpatient

CHIEF COMPLAINT:
Presents to the clinic for a routine follow-up

HISTORY OF PRESENT ILLNESS:
The patient with chronic hepatitis C infection, alcoholic cirrhosis with portal hypertension, Kallmann syndrome, and recurrent internal hemorrhoids returns to the Infectious Diseases clinic for routine follow-up. He reports no acute complaints but mentions occasional fatigue.

PAST MEDICAL HISTORY:
Active Problem List:
- Kallmann syndrome (diagnosed 2020-01-15)
- Hemorrhoids (diagnosed 2020-07-15)
- Alcoholic cirrhosis (diagnosed 2021-07-15)
- Chronic hepatitis C infection
- Alcoholic cirrhosis with portal hypertension
- Kallmann syndrome (congenital hypogonadotropic hypogonadism)
- Recurrent internal hemorrhoids
- Hypogonadism (multifactorial)

The patient's immunization record indicates that he received his last tetanus booster 8 years ago. He has no documentation of hepatitis A, hepatitis B, or pneumococcal vaccinations.

PAST SURGICAL HISTORY:
- Esophagogastroduodenoscopy with esophageal variceal band ligation (two prior hospitalizations within the past year)
- Rubber band ligation of hemorrhoids
- Umbilical hernia repair (childhood)

MEDICATIONS:
His medications include furosemide, spironolactone, and vitamin K.

Home Medications:
- Propranolol 20 mg PO twice daily
- Testosterone cypionate 200 mg IM every 2 weeks
- Lactulose 30 mL PO twice daily as needed for constipation

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Deceased at age 58 from complications of alcoholic cirrhosis and hepatocellular carcinoma
- Mother: Living, hypertension and hyperlipidemia
- Brother: Living, chronic hepatitis C infection
- Paternal uncle: Alcohol use disorder

SOCIAL HISTORY:
He has abstained from alcohol for the past 10 years and has no history of drug use.

VITAL SIGNS:
His vital signs are stable

PHYSICAL EXAMINATION:
He has mild ascites. His skin examination reveals spider angiomas on the upper torso and palmar erythema.

LABORATORY DATA:
Laboratory tests show:

- Platelet count: 110,000/µL
- INR: 1.2
- Albumin: 3.2 g/dL
- Total bilirubin: 2.1 mg/dL
- AST: 80 U/L
- ALT: 75 U/L

## Encounter 4: source 2023-07-15, practice 2025-02-04, outpatient, Gastroenterology

Chief complaint: Increased abdominal girth and early satiety

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Jennifer Park, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Increased abdominal girth and early satiety

HISTORY OF PRESENT ILLNESS:
The patient with alcoholic cirrhosis and portal hypertension returns to the Gastroenterology clinic for follow-up, having previously presented with abdominal distension in 2021 and undergone routine monitoring for his chronic liver disease. He reports increased abdominal girth and early satiety over the past two months. He denies any recent changes in diet or bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Kallmann syndrome (diagnosed 2020-01-15)
- Hemorrhoids (diagnosed 2020-07-15)
- Alcoholic cirrhosis (diagnosed 2021-07-15)
- Hepatic cirrhosis with portal hypertension (diagnosed 2022-07-15)
- Chronic hepatitis C infection
- Alcoholic cirrhosis with portal hypertension
- Kallmann syndrome (congenital hypogonadotropic hypogonadism)
- Recurrent internal hemorrhoids
- Hypogonadism (multifactorial)

PAST SURGICAL HISTORY:
- Esophagogastroduodenoscopy with esophageal variceal band ligation (two prior hospitalizations within the past year)
- Rubber band ligation of hemorrhoids
- Umbilical hernia repair (childhood)

MEDICATIONS:
Home Medications:
- Furosemide 40 mg PO daily
- Spironolactone 100 mg PO daily
- Propranolol 20 mg PO twice daily
- Testosterone cypionate 200 mg IM every 2 weeks
- Vitamin K 10 mg PO daily
- Lactulose 30 mL PO twice daily as needed for constipation

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Deceased at age 58 from complications of alcoholic cirrhosis and hepatocellular carcinoma
- Mother: Living, hypertension and hyperlipidemia
- Brother: Living, chronic hepatitis C infection
- Paternal uncle: Alcohol use disorder

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago. Alcohol: Heavy daily use (6 standard beers per day for the past 15 years). Occupation: Automotive parts manager.

PHYSICAL EXAMINATION:
On examination, he has multiple spider angiomas on his chest, palmar erythema, and mild jaundice. Abdominal examination reveals a fluid wave and shifting dullness. Neurological examination shows the presence of asterixis.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum albumin: 2.8 g/dL (Normal: 3.5-5.0 g/dL)
- Serum total bilirubin: 3.2 mg/dL (Normal: 0.3-1.2 mg/dL)
- INR: 1.6 (Normal: 0.8-1.1)
- Platelet count: 95,000/µL (Normal: 150,000-400,000/µL)
- AST: 120 U/L (Normal: 10-40 U/L)
- ALT: 90 U/L (Normal: 7-56 U/L)

IMAGING:
Abdominal ultrasound shows a nodular liver contour and splenomegaly. There is evidence of ascitic fluid.

## Encounter 5: source 2024-01-15, practice 2025-08-01, outpatient, Gastroenterology

Chief complaint: Progressive abdominal distension and discomfort over the past two months

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Robert Chang, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive abdominal distension and discomfort over the past two months

HISTORY OF PRESENT ILLNESS:
The patient with alcoholic cirrhosis returns with abdominal distension similar to July 2021 and 2023. He reports a decrease in appetite and early satiety. He denies any fever, vomiting, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Kallmann syndrome (diagnosed 2020-01-15)
- Hemorrhoids (diagnosed 2020-07-15)
- Alcoholic cirrhosis (diagnosed 2021-07-15)
- Hepatic cirrhosis with portal hypertension (diagnosed 2022-07-15)
- Chronic hepatitis C infection
- Alcoholic cirrhosis with portal hypertension
- Kallmann syndrome (congenital hypogonadotropic hypogonadism)
- Recurrent internal hemorrhoids
- Hypogonadism (multifactorial)

His past medical history is significant for chronic hepatitis C infection.

PAST SURGICAL HISTORY:
- Esophagogastroduodenoscopy with esophageal variceal band ligation (two prior hospitalizations within the past year)
- Rubber band ligation of hemorrhoids
- Umbilical hernia repair (childhood)

MEDICATIONS:
Home Medications:
- Furosemide 40 mg PO daily
- Spironolactone 100 mg PO daily
- Propranolol 20 mg PO twice daily
- Testosterone cypionate 200 mg IM every 2 weeks
- Vitamin K 10 mg PO daily
- Lactulose 30 mL PO twice daily as needed for constipation

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Deceased at age 58 from complications of alcoholic cirrhosis and hepatocellular carcinoma
- Mother: Living, hypertension and hyperlipidemia
- Brother: Living, chronic hepatitis C infection
- Paternal uncle: Alcohol use disorder

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago. Alcohol: Heavy daily use (6 standard beers per day for the past 15 years). Occupation: Automotive parts manager.

PHYSICAL EXAMINATION:
On physical examination, the patient appears cachectic with jaundiced skin and sclera. Abdominal examination reveals a fluid wave and shifting dullness. His extremities show pitting edema.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 10.2 g/dL
- Platelet count: 85,000/uL
- INR: 1.6
- Serum albumin: 2.8 g/dL
- Serum total bilirubin: 3.2 mg/dL
- Serum creatinine: 1.1 mg/dL
- Serum sodium: 133 mEq/L
- Urine sodium: 5 mEq/L

IMAGING:
Abdominal ultrasound with Doppler shows evidence of cirrhosis, splenomegaly, and ascites without evidence of portal vein thrombosis.

## Encounter 6: source 2024-07-15, practice 2026-01-31, ed, Emergency Medicine

Chief complaint: Presents to the emergency department with hematemesis and melena

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Emily Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Presents to the emergency department with hematemesis and melena

HISTORY OF PRESENT ILLNESS:
The patient with alcoholic cirrhosis and portal hypertension presents. He has had two similar episodes in the past year requiring hospitalization and reports progressive abdominal distension and early satiety.

PAST MEDICAL HISTORY:
Active Problem List:
- Kallmann syndrome (diagnosed 2020-01-15)
- Hemorrhoids (diagnosed 2020-07-15)
- Alcoholic cirrhosis (diagnosed 2021-07-15)
- Hepatic cirrhosis with portal hypertension (diagnosed 2022-07-15)
- Ascites due to cirrhosis (uncomplicated) (diagnosed 2024-01-15)
- Chronic hepatitis C infection
- Alcoholic cirrhosis with portal hypertension
- Kallmann syndrome (congenital hypogonadotropic hypogonadism)
- Recurrent internal hemorrhoids
- Hypogonadism (multifactorial)

PAST SURGICAL HISTORY:
- Esophagogastroduodenoscopy with esophageal variceal band ligation (two prior hospitalizations within the past year)
- Rubber band ligation of hemorrhoids
- Umbilical hernia repair (childhood)

MEDICATIONS:
Home Medications:
- Furosemide 40 mg PO daily
- Spironolactone 100 mg PO daily
- Propranolol 20 mg PO twice daily
- Testosterone cypionate 200 mg IM every 2 weeks
- Vitamin K 10 mg PO daily
- Lactulose 30 mL PO twice daily as needed for constipation

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Deceased at age 58 from complications of alcoholic cirrhosis and hepatocellular carcinoma
- Mother: Living, hypertension and hyperlipidemia
- Brother: Living, chronic hepatitis C infection
- Paternal uncle: Alcohol use disorder

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago. Alcohol: Heavy daily use (6 standard beers per day for the past 15 years). Occupation: Automotive parts manager.

VITAL SIGNS:
Blood pressure is 92/58 mm Hg, pulse is 108/min, and respiratory rate is 18/min.

PHYSICAL EXAMINATION:
On examination, he appears pale and lethargic. His abdomen is distended with a fluid wave, and there are dilated veins visible around the umbilicus.

LABORATORY DATA:
Laboratory studies show:
Hemoglobin                                           7.8 g/dL
Platelets                                                72,000/mm³
AST                                                       68 U/L
ALT                                                        44 U/L
Total bilirubin                                         3.2 mg/dL
Albumin                                                 2.7 g/dL
INR                                                        1.8

PLAN:
After stabilization and endoscopic therapy, the hepatology team plans a procedure to decrease the pressure within the portal system and reduce the risk of recurrent bleeding. To achieve this therapeutic goal, a shunt will be placed between the portal vein and

## Encounter 7: source 2025-01-15, practice 2026-07-26, ed, Emergency Medicine

Chief complaint: Progressive lower extremity edema and abdominal distension

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. James Washington, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive lower extremity edema and abdominal distension

HISTORY OF PRESENT ILLNESS:
The patient with alcoholic cirrhosis, portal hypertension, and prior variceal hemorrhage presents to the emergency department with a two-week history of progressive lower extremity edema and abdominal distension. He denies any recent alcohol consumption.

PAST MEDICAL HISTORY:
Active Problem List:
- Kallmann syndrome (diagnosed 2020-01-15)
- Hemorrhoids (diagnosed 2020-07-15)
- Alcoholic cirrhosis (diagnosed 2021-07-15)
- Hepatic cirrhosis with portal hypertension (diagnosed 2022-07-15)
- Ascites due to cirrhosis (uncomplicated) (diagnosed 2024-01-15)
- Cirrhosis with acute esophageal variceal hemorrhage due to portal hypertension (diagnosed 2024-07-15)
- Chronic hepatitis C infection
- Alcoholic cirrhosis with portal hypertension
- Kallmann syndrome (congenital hypogonadotropic hypogonadism)
- Recurrent internal hemorrhoids
- Hypogonadism (multifactorial)

PAST SURGICAL HISTORY:
- Esophagogastroduodenoscopy with esophageal variceal band ligation (two prior hospitalizations within the past year)
- Rubber band ligation of hemorrhoids
- Umbilical hernia repair (childhood)

MEDICATIONS:
Home Medications:
- Furosemide 40 mg PO daily
- Spironolactone 100 mg PO daily
- Propranolol 20 mg PO twice daily
- Testosterone cypionate 200 mg IM every 2 weeks
- Vitamin K 10 mg PO daily
- Lactulose 30 mL PO twice daily as needed for constipation

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Deceased at age 58 from complications of alcoholic cirrhosis and hepatocellular carcinoma
- Mother: Living, hypertension and hyperlipidemia
- Brother: Living, chronic hepatitis C infection
- Paternal uncle: Alcohol use disorder

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago. Alcohol: Heavy daily use (6 standard beers per day for the past 15 years). Occupation: Automotive parts manager.

VITAL SIGNS:
Blood pressure is 90/60 mmHg, heart rate is 102 bpm, and he has a low-grade fever

PHYSICAL EXAMINATION:
Jaundiced with a distended abdomen and shifting dullness

LABORATORY DATA:
Serum creatinine: 3.2 mg/dL (increased from a baseline of 1.0 mg/dL over the past month)
- Blood urea nitrogen (BUN): 45 mg/dL
- Serum sodium: 130 mEq/L
- Serum potassium: 4.5 mEq/L
- Serum albumin: 2.8 g/dL
- INR: 1.6
- Urinalysis: minimal protein, no blood, no casts
- Liver function tests: AST and ALT mildly elevated, ALP within normal limits

IMAGING:
Abdominal ultrasound shows a nodular liver contour and ascites without evidence of biliary obstruction or hepatic vein thrombosis.

ASSESSMENT AND PLAN:
The patient's blood pressure does not improve significantly after intravenous albumin administration.
