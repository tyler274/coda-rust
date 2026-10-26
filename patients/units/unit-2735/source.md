# synthetic_hospital patient 2735

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 55 at first encounter, sex F, Non-Hispanic White, Employer-sponsored preferred provider organization (PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-12-01 (Mon) | ed | Emergency Medicine |
| 1 | 2020-03-15 | 2026-02-06 (Fri) | procedure | General Surgery |
| 2 | 2020-04-15 | 2026-03-07 (Sat) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Employer-sponsored preferred provider organization (PPO)",
 "occupation": "Administrative coordinator at a school district",
 "alcohol_use": "Social (1-2 glasses of wine on weekends)",
 "family_history": [
  "Mother: Type 2 diabetes mellitus, cholelithiasis (status post cholecystectomy age 52)",
  "Father: Hypertension, hyperlipidemia",
  "One older sister: Cholelithiasis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 2015 (approximate 8 pack-year history)",
 "home_medications": [
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Glipizide"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "20 mg PO daily at bedtime",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "Cesarean section (1994)",
  "Tonsillectomy and adenoidectomy (1978)"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus (diagnosed 2018, hemoglobin A1c 7.2%)",
  "Cholelithiasis with recurrent biliary colic",
  "Obesity (BMI 33 kg/m\u00b2)",
  "Essential hypertension"
 ],
 "age_at_first_encounter": 55
}
```

Primary diagnoses: ["Choledocholithiasis with cholangitis", "Acute laryngitis", "Vagal bradycardia secondary to pneumoperitoneum"]
Comorbidities: ["Cholelithiasis", "Septic shock"]

## Encounter 0: source 2020-01-15, practice 2025-12-01, ed, Emergency Medicine

Chief complaint: Right upper quadrant abdominal pain, fever, and jaundice

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Right upper quadrant abdominal pain, fever, and jaundice

HISTORY OF PRESENT ILLNESS:
She presents to the emergency department with a 2-day history of

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2018, hemoglobin A1c 7.2%)
- Cholelithiasis with recurrent biliary colic
- Obesity (BMI 33 kg/m²)
- Essential hypertension

PAST SURGICAL HISTORY:
- Cesarean section (1994)
- Tonsillectomy and adenoidectomy (1978)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Glipizide 5 mg PO daily
- Lisinopril 10 mg PO daily
- Atorvastatin 20 mg PO daily at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus, cholelithiasis (status post cholecystectomy age 52)
- Father: Hypertension, hyperlipidemia
- One older sister: Cholelithiasis

SOCIAL HISTORY:
Smoking: Former smoker, quit 2015 (approximate 8 pack-year history). Alcohol: Social (1-2 glasses of wine on weekends). Occupation: Administrative coordinator at a school district.

VITAL SIGNS:
Her temperature is 38.9°C (102°F), blood pressure is 90/60 mmHg, heart rate is 110 beats per minute

PHYSICAL EXAMINATION:
She appears confused. Her abdomen is tender to palpation in the right upper quadrant without rebound or guarding.

LABORATORY DATA:
White blood cell count: 18,000/mm³
Hemoglobin: 12.5 g/dL
Platelet count: 250,000/mm³
Serum bilirubin (total/direct): 3.0/2.5 mg/dL
Aspartate aminotransferase (AST): 80 U/L
Alanine aminotransferase (ALT): 75 U/L
Alkaline phosphatase: 300 U/L
Blood urea nitrogen (BUN): 18 mg/dL
Creatinine: 1.0 mg/dL
Serum lactate: 4.5 mmol/L
Anion gap: 20 mEq/L

IMAGING:
A right upper quadrant abdominal ultrasound shows common bile duct dilation and a stone at the distal end of the common bile duct.

## Encounter 1: source 2020-03-15, practice 2026-02-06, procedure, General Surgery

Chief complaint: Undergoing laparoscopic cholecystectomy

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Marcus Rodriguez, MD
Department: General Surgery
Visit Type: procedure

CHIEF COMPLAINT:
Undergoing laparoscopic cholecystectomy

HISTORY OF PRESENT ILLNESS:
The patient, who has cholelithiasis with recurrent biliary colic, presents for laparoscopic cholecystectomy following choledocholithiasis with cholangitis in January 2020. Approximately 30 minutes into the procedure, while the abdomen is being insufflated with CO2, the anesthesiologist notes a sudden drop in heart rate on the monitor, and she is otherwise hemodynamically stable. The surgeon has not encountered any significant difficulties during the procedure.

PAST MEDICAL HISTORY:
Active Problem List:
- Choledocholithiasis with cholangitis (diagnosed 2020-01-15)
- Type 2 diabetes mellitus (diagnosed 2018, hemoglobin A1c 7.2%)
- Cholelithiasis with recurrent biliary colic
- Obesity (BMI 33 kg/m²)
- Essential hypertension

PAST SURGICAL HISTORY:
- Cesarean section (1994)
- Tonsillectomy and adenoidectomy (1978)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Glipizide 5 mg PO daily
- Lisinopril 10 mg PO daily
- Atorvastatin 20 mg PO daily at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus, cholelithiasis (status post cholecystectomy age 52)
- Father: Hypertension, hyperlipidemia
- One older sister: Cholelithiasis

SOCIAL HISTORY:
Smoking: Former smoker, quit 2015 (approximate 8 pack-year history). Alcohol: Social (1-2 glasses of wine on weekends). Occupation: Administrative coordinator at a school district.

VITAL SIGNS:
Heart rate to 40 beats per minute; The patient's blood pressure is 110/70 mmHg

OTHER STUDIES:
The patient's preoperative EKG showed normal sinus rhythm with no evidence of conduction abnormalities

## Encounter 2: source 2020-04-15, practice 2026-03-07, icu, Emergency Medicine

Chief complaint: Severe right upper quadrant pain, fever, jaundice with confusion and lethargy

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Severe right upper quadrant pain, fever, jaundice with confusion and lethargy

HISTORY OF PRESENT ILLNESS:
This 55-year-old female with a history of Type 2 diabetes mellitus returns to the intensive care unit one month following laparoscopic cholecystectomy for prior choledocholithiasis with cholangitis. She also reports nausea and vomiting. The patient becomes increasingly confused and lethargic.

PAST MEDICAL HISTORY:
Active Problem List:
- Choledocholithiasis with cholangitis (diagnosed 2020-01-15)
- Vagal bradycardia secondary to pneumoperitoneum (diagnosed 2020-03-15)
- Type 2 diabetes mellitus (diagnosed 2018, hemoglobin A1c 7.2%)
- Cholelithiasis with recurrent biliary colic
- Obesity (BMI 33 kg/m²)
- Essential hypertension

PAST SURGICAL HISTORY:
- Cesarean section (1994)
- Tonsillectomy and adenoidectomy (1978)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Glipizide 5 mg PO daily
- Lisinopril 10 mg PO daily
- Atorvastatin 20 mg PO daily at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus, cholelithiasis (status post cholecystectomy age 52)
- Father: Hypertension, hyperlipidemia
- One older sister: Cholelithiasis

SOCIAL HISTORY:
Smoking: Former smoker, quit 2015 (approximate 8 pack-year history). Alcohol: Social (1-2 glasses of wine on weekends). Occupation: Administrative coordinator at a school district.

VITAL SIGNS:
Her temperature is 38.9°C (102°F), blood pressure is 90/60 mmHg, heart rate is 110 beats/min, and respiratory rate is 22 breaths/min.

PHYSICAL EXAMINATION:
Physical examination reveals a jaundiced female in distress with a positive Murphy&#x27;s sign.

LABORATORY DATA:
Laboratory studies show:

- White blood cell count: 18,000/mm³
- Hemoglobin: 12.5 g/dL
- Platelet count: 250,000/mm³
- Total bilirubin: 5.2 mg/dL
- Direct bilirubin: 3.8 mg/dL
- Alkaline phosphatase: 210 U/L
- Aspartate aminotransferase (AST): 80 U/L
- Alanine aminotransferase (ALT): 75 U/L
- Amylase: 300 U/L
- Lipase: 900 U/L

IMAGING:
Abdominal ultrasound reveals cholelithiasis with a common bile duct stone and dilated intrahepatic ducts.
