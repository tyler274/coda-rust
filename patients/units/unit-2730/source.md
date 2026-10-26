# synthetic_hospital patient 2730

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 47 at first encounter, sex F, Hispanic, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-10-14 (Fri) | ed | Emergency Medicine |
| 1 | 2020-10-15 | 2023-07-16 (Sun) | inpatient | Internal Medicine |
| 2 | 2022-01-15 | 2024-10-15 (Tue) | ed | Emergency Medicine |
| 3 | 2023-01-15 | 2025-10-02 (Thu) | inpatient | Gastroenterology |
| 4 | 2023-07-15 | 2026-04-10 (Fri) | icu | Critical Care Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Administrative Assistant",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Cholelithiasis, Type 2 diabetes mellitus",
  "Father: Hyperlipidemia, Obesity",
  "Sister: Symptomatic cholelithiasis"
 ],
 "race_ethnicity": "Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Obesity (Class II)",
  "Mixed hyperlipidemia",
  "Cholelithiasis with recurrent biliary colic"
 ],
 "age_at_first_encounter": 47
}
```

Primary diagnoses: ["Cholelithiasis", "Biliary pancreatitis with acute cholangitis", "Sphincter of Oddi dysfunction", "Acute laryngitis"]
Comorbidities: ["Hyperlipidemia", "Obesity", "Cholelithiasis"]

## Encounter 0: source 2020-01-15, practice 2022-10-14, ed, Emergency Medicine

Chief complaint: Severe right upper quadrant abdominal pain radiating to the right shoulder after eating a large, greasy meal

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe right upper quadrant abdominal pain radiating to the right shoulder after eating a large, greasy meal

HISTORY OF PRESENT ILLNESS:
A 45-year-old woman presents to the emergency department with a 4-hour history of severe right upper quadrant abdominal pain that started after eating a large, greasy meal at a local diner. She describes the pain as sharp and constant, and it radiates to her right shoulder. She denies any nausea, vomiting, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Obesity (Class II)
- Mixed hyperlipidemia
- Cholelithiasis with recurrent biliary colic

Her past medical history is significant for hyperlipidemia and obesity.

MEDICATIONS:
Home Medications:
- Atorvastatin 40 mg PO daily

ALLERGIES:
She has no known drug allergies

FAMILY HISTORY:
- Mother: Cholelithiasis, Type 2 diabetes mellitus
- Father: Hyperlipidemia, Obesity
- Sister: Symptomatic cholelithiasis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Assistant.

VITAL SIGNS:
On examination, her vital signs are stable, and she is afebrile.

PHYSICAL EXAMINATION:
Abdominal examination reveals no jaundice, and the abdomen is soft with no guarding or rebound tenderness, but there is significant pain on palpation of the right upper quadrant. Murphy's sign is negative.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 13.2 g/dL
- White blood cell count: 7,000/mm^3
- Platelets: 250,000/mm^3
- Aspartate aminotransferase (AST): 30 U/L
- Alanine aminotransferase (ALT): 28 U/L
- Alkaline phosphatase: 85 U/L
- Total bilirubin: 0.8 mg/dL

IMAGING:
An abdominal ultrasound is performed, which shows multiple gallstones without evidence of cholecystitis or choledocholithiasis.

## Encounter 1: source 2020-10-15, practice 2023-07-16, inpatient, Internal Medicine

Chief complaint: Severe constant epigastric pain radiating to the back with associated nausea and vomiting

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. James Chen, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Severe constant epigastric pain radiating to the back with associated nausea and vomiting

HISTORY OF PRESENT ILLNESS:
The patient is a 47-year-old female with obesity, mixed hyperlipidemia, and cholelithiasis with recurrent biliary colic, previously seen in January 2020 for right upper quadrant pain, who presents to the emergency department with a 6-hour history of severe, constant epigastric pain that radiates to her back. She describes the pain as 8 out of 10 in intensity and reports associated nausea and vomiting. She denies alcohol use but mentions that she has had similar, albeit milder, episodes in the past, which resolved spontaneously.

PAST MEDICAL HISTORY:
Active Problem List:
- Cholelithiasis (diagnosed 2020-01-15)
- Obesity (Class II)
- Mixed hyperlipidemia
- Cholelithiasis with recurrent biliary colic

Her medical history is significant for obesity and hyperlipidemia.

MEDICATIONS:
Home Medications:
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Cholelithiasis, Type 2 diabetes mellitus
- Father: Hyperlipidemia, Obesity
- Sister: Symptomatic cholelithiasis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Assistant.

VITAL SIGNS:
On physical examination, her vital signs are stable, and she is afebrile.

PHYSICAL EXAMINATION:
Abdominal examination reveals epigastric tenderness without rebound or guarding. Bowel sounds are normal.

LABORATORY DATA:
Laboratory tests reveal:

- White blood cell count: 12,000/µL
- Hemoglobin: 14 g/dL
- Platelets: 250,000/µL
- Serum amylase: 500 U/L (Normal: 30-110 U/L)
- Serum lipase: 600 U/L (Normal: 0-160 U/L)
- Aspartate aminotransferase (AST): 45 U/L
- Alanine aminotransferase (ALT): 50 U/L
- Total bilirubin: 1.2 mg/dL
- Alkaline phosphatase: 120 U/L

OTHER STUDIES:
An electrocardiogram (ECG) and chest X-ray are unremarkable.

## Encounter 2: source 2022-01-15, practice 2024-10-15, ed, Emergency Medicine

Chief complaint: Sudden onset severe right upper quadrant pain radiating to the back with associated nausea

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Sarah Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset severe right upper quadrant pain radiating to the back with associated nausea

HISTORY OF PRESENT ILLNESS:
The patient with recurrent biliary colic returns after 2020 episodes. She describes the pain as constant and reports associated nausea but no vomiting. She denies any changes in bowel habits or urine color.

PAST MEDICAL HISTORY:
Active Problem List:
- Cholelithiasis (diagnosed 2020-01-15)
- Acute laryngitis (diagnosed 2020-10-15)
- Obesity (Class II)
- Mixed hyperlipidemia
- Cholelithiasis with recurrent biliary colic

MEDICATIONS:
Home Medications:
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Cholelithiasis, Type 2 diabetes mellitus
- Father: Hyperlipidemia, Obesity
- Sister: Symptomatic cholelithiasis

SOCIAL HISTORY:
She does not smoke or drink alcohol.

VITAL SIGNS:
Her vital signs are stable, and she is afebrile

PHYSICAL EXAMINATION:
Abdominal examination reveals right upper quadrant tenderness with a positive Murphy&#x27;s sign

LABORATORY DATA:
- White blood cell count: 12,000/uL
- Hemoglobin: 14 g/dL
- Platelets: 250,000/uL
- Aspartate aminotransferase (AST): 40 U/L
- Alanine aminotransferase (ALT): 45 U/L
- Alkaline phosphatase: 120 U/L
- Total bilirubin: 1.2 mg/dL
- Lipase: 22 U/L (normal)

IMAGING:
An abdominal ultrasound reveals multiple gallstones in the gallbladder, with no evidence of cholecystitis or choledocholithiasis.

## Encounter 3: source 2023-01-15, practice 2025-10-02, inpatient, Gastroenterology

Chief complaint: Two-day history of fever, right upper quadrant abdominal pain, and jaundice

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Michael Patel, MD
Department: Gastroenterology
Visit Type: inpatient

CHIEF COMPLAINT:
Two-day history of fever, right upper quadrant abdominal pain, and jaundice

HISTORY OF PRESENT ILLNESS:
The patient with cholelithiasis and recurrent biliary colic presents with fever, right upper quadrant pain, and jaundice following prior presentations for similar epigastric and right upper quadrant pain radiating to the back and shoulder. She reports that the pain is constant, sharp, and radiates to her back.

PAST MEDICAL HISTORY:
Active Problem List:
- Cholelithiasis (diagnosed 2020-01-15)
- Acute laryngitis (diagnosed 2020-10-15)
- Sphincter of Oddi dysfunction (diagnosed 2022-01-15)
- Obesity (Class II)
- Mixed hyperlipidemia
- Cholelithiasis with recurrent biliary colic

She has a history of recurrent RUQ pain for the past year, which was previously attributed to gallstones.

PAST SURGICAL HISTORY:
Although she never underwent cholecystectomy

MEDICATIONS:
Home Medications:
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Cholelithiasis, Type 2 diabetes mellitus
- Father: Hyperlipidemia, Obesity
- Sister: Symptomatic cholelithiasis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Assistant.

VITAL SIGNS:
Her temperature is 38.9°C (102°F), heart rate is 110/min, and blood pressure is 135/85 mmHg

PHYSICAL EXAMINATION:
RUQ tenderness is noted on palpation, without rebound or guarding.

LABORATORY DATA:
- White blood cell count: 16,000/mm³ (normal 4,500-11,000/mm³)
- Total bilirubin: 5.2 mg/dL (normal 0.3-1.2 mg/dL)
- Direct bilirubin: 3.8 mg/dL (normal 0.0-0.3 mg/dL)
- Alkaline phosphatase: 280 U/L (normal 40-120 U/L)
- Aspartate aminotransferase (AST): 80 U/L (normal 10-40 U/L)
- Alanine aminotransferase (ALT): 75 U/L (normal 7-56 U/L)

IMAGING:
Abdominal ultrasound shows a 4 cm cystic mass in the common bile duct.

## Encounter 4: source 2023-07-15, practice 2026-04-10, icu, Critical Care Medicine

Chief complaint: Severe epigastric pain radiating to the back with nausea and vomiting persisting for 24 hours after a fatty meal

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Emily Washington, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Severe epigastric pain radiating to the back with nausea and vomiting persisting for 24 hours after a fatty meal

HISTORY OF PRESENT ILLNESS:
The patient with recurrent biliary colic returns. She reports that the pain started after eating a large, fatty meal and has progressively worsened.

PAST MEDICAL HISTORY:
Active Problem List:
- Cholelithiasis (diagnosed 2020-01-15)
- Acute laryngitis (diagnosed 2020-10-15)
- Sphincter of Oddi dysfunction (diagnosed 2022-01-15)
- Obesity (Class II)
- Mixed hyperlipidemia
- Cholelithiasis with recurrent biliary colic

Her past medical history is significant for obesity and multiple episodes of biliary colic.

MEDICATIONS:
Home Medications:
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Cholelithiasis, Type 2 diabetes mellitus
- Father: Hyperlipidemia, Obesity
- Sister: Symptomatic cholelithiasis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Assistant.

VITAL SIGNS:
On examination, her temperature is 37.8°C (100°F), heart rate is 102 beats per minute, blood pressure is 135/85 mmHg, and respiratory rate is 20 breaths per minute.

PHYSICAL EXAMINATION:
Her abdomen is tender in the epigastric region with guarding but no rebound tenderness. Bowel sounds are normal.

LABORATORY DATA:
Laboratory tests reveal:

- White blood cell count: 12,000 /µL
- Hemoglobin: 13.5 g/dL
- Alanine aminotransferase (ALT): 210 U/L
- Aspartate aminotransferase (AST): 190 U/L
- Total bilirubin: 2.1 mg/dL
- Alkaline phosphatase: 130 U/L
- Lipase: 900 U/L (normal: 0-160 U/L)
- Amylase: 400 U/L (normal: 30-110 U/L)

IMAGING:
An abdominal ultrasound shows gallstones and a thickened gallbladder wall without evidence of cholecystitis. There is no dilatation of the common bile duct, and no stones are seen in the duct.
