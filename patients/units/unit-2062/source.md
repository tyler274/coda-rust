# synthetic_hospital patient 2062

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-09-03 (Wed) | ed | Emergency Medicine |
| 1 | 2020-04-15 | 2025-11-26 (Wed) | ed | Emergency Medicine |
| 2 | 2020-09-15 | 2026-04-27 (Mon) | outpatient | Gastroenterology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicaid",
 "occupation": "Former construction worker, currently on medical disability",
 "alcohol_use": "Chronic heavy alcohol use (6-8 standard drinks daily for 15 years), last use 4 months ago",
 "family_history": [
  "Father: Alcohol use disorder with cirrhosis",
  "Mother: Type 2 diabetes mellitus",
  "Brother: Chronic hepatitis C infection"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 20 pack-years, quit 6 months ago",
 "home_medications": [
  {
   "dose": "2 mg twice daily (target trough 8-10 ng/mL)",
   "name": "Tacrolimus"
  },
  {
   "dose": "750 mg twice daily",
   "name": "Mycophenolate mofetil"
  },
  {
   "dose": "5 mg daily",
   "name": "Prednisone"
  },
  {
   "dose": "800/160 mg three times weekly (PJP prophylaxis)",
   "name": "Sulfamethoxazole-trimethoprim"
  },
  {
   "dose": "36,000 units with meals",
   "name": "Pancrelipase"
  },
  {
   "dose": "100 mg daily",
   "name": "Thiamine"
  },
  {
   "dose": "1 mg daily",
   "name": "Folic acid"
  }
 ],
 "surgical_history": [
  "Orthotopic liver transplantation (2 months prior to most recent encounter)",
  "ERCP with pancreatic duct stent placement",
  "Multiple therapeutic paracenteses (historical)"
 ],
 "chronic_conditions": [
  "Chronic hepatitis C infection (Genotype 1a)",
  "Alcohol use disorder, severe, in early remission",
  "Chronic pancreatitis secondary to alcohol use",
  "History of hepatic encephalopathy",
  "End-stage liver disease, status post orthotopic liver transplantation"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Spontaneous bacterial peritonitis", "Pancreatic ascites", "Acute cellular rejection of liver transplant"]
Comorbidities: ["Hepatic cirrhosis with portal hypertension", "Alcohol use disorder"]

## Encounter 0: source 2020-01-15, practice 2025-09-03, ed, Emergency Medicine

Chief complaint: Brought to the emergency department with high-grade fever, abdominal discomfort, and progressive confusion for one day.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought to the emergency department with high-grade fever, abdominal discomfort, and progressive confusion for one day.

HISTORY OF PRESENT ILLNESS:
His family reports he has a history of alcohol use and was recently treated for hepatic encephalopathy.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic hepatitis C infection (Genotype 1a)
- Alcohol use disorder, severe, in early remission
- Chronic pancreatitis secondary to alcohol use
- History of hepatic encephalopathy
- End-stage liver disease, status post orthotopic liver transplantation

PAST SURGICAL HISTORY:
- Orthotopic liver transplantation (2 months prior to most recent encounter)
- ERCP with pancreatic duct stent placement
- Multiple therapeutic paracenteses (historical)

MEDICATIONS:
Home Medications:
- Tacrolimus 2 mg twice daily (target trough 8-10 ng/mL)
- Mycophenolate mofetil 750 mg twice daily
- Prednisone 5 mg daily
- Sulfamethoxazole-trimethoprim 800/160 mg three times weekly (PJP prophylaxis)
- Pancrelipase 36,000 units with meals
- Thiamine 100 mg daily
- Folic acid 1 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis
- Mother: Type 2 diabetes mellitus
- Brother: Chronic hepatitis C infection

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 6 months ago. Alcohol: Chronic heavy alcohol use (6-8 standard drinks daily for 15 years), last use 4 months ago. Occupation: Former construction worker, currently on medical disability.

VITAL SIGNS:
Temperature is 39.2°C (102.6°F), blood pressure is 90/60 mmHg, heart rate is 115/min, respiratory rate is 22/min, and oxygen saturation is 96% on room air.

PHYSICAL EXAMINATION:
On examination, he appears lethargic. His abdomen is distended with shifting dullness and mild diffuse tenderness, but no rebound.

LABORATORY DATA:
Laboratory studies reveal WBC 14,500/µL, serum creatinine 1.6 mg/dL, and total bilirubin 3.2 mg/dL.

OTHER STUDIES:
Diagnostic paracentesis shows a neutrophil count of 430 cells/mm³ and total protein 0.8 g/dL. Cultures are pending.

## Encounter 1: source 2020-04-15, practice 2025-11-26, ed, Emergency Medicine

Chief complaint: Presents to the emergency department with a 2-day history of worsening abdominal pain.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Presents to the emergency department with a 2-day history of worsening abdominal pain.

HISTORY OF PRESENT ILLNESS:
The patient with chronic pancreatitis and prior spontaneous bacterial peritonitis returns with abdominal pain. He describes the pain as constant, severe, and located in the epigastric region, radiating to the back. He reports that the pain intensifies after meals and is accompanied by nausea. He denies any recent trauma or surgery.

PAST MEDICAL HISTORY:
Active Problem List:
- Spontaneous bacterial peritonitis (diagnosed 2020-01-15)
- Chronic hepatitis C infection (Genotype 1a)
- Alcohol use disorder, severe, in early remission
- Chronic pancreatitis secondary to alcohol use
- History of hepatic encephalopathy
- End-stage liver disease, status post orthotopic liver transplantation

PAST SURGICAL HISTORY:
- Orthotopic liver transplantation (2 months prior to most recent encounter)
- ERCP with pancreatic duct stent placement
- Multiple therapeutic paracenteses (historical)

MEDICATIONS:
Home Medications:
- Tacrolimus 2 mg twice daily (target trough 8-10 ng/mL)
- Mycophenolate mofetil 750 mg twice daily
- Prednisone 5 mg daily
- Sulfamethoxazole-trimethoprim 800/160 mg three times weekly (PJP prophylaxis)
- Pancrelipase 36,000 units with meals
- Thiamine 100 mg daily
- Folic acid 1 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis
- Mother: Type 2 diabetes mellitus
- Brother: Chronic hepatitis C infection

SOCIAL HISTORY:
With a history of chronic alcohol use

VITAL SIGNS:
His vital signs are stable.

PHYSICAL EXAMINATION:
Physical examination reveals a distended abdomen with diffuse tenderness to palpation, but no rebound or guarding. Bowel sounds are hypoactive.

LABORATORY DATA:
A diagnostic paracentesis is performed, and the fluid analysis is as follows:

- Appearance: Serosanguinous
- Amylase: 5000 U/L (Normal: 23-85 U/L)
- Total protein: 4.5 g/dL (Normal: 6.0-8.3 g/dL)
- Serum albumin: 3.5 g/dL (Normal: 3.5-5.0 g/dL)
- Ascites albumin: 1.5 g/dL

## Encounter 2: source 2020-09-15, practice 2026-04-27, outpatient, Gastroenterology

Chief complaint: Malaise, low-grade fevers, and jaundice for one week.

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Aisha Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Malaise, low-grade fevers, and jaundice for one week.

HISTORY OF PRESENT ILLNESS:
Status post liver transplantation, the patient returns after recent SBP and pancreatic ascites. He presents with a 1-week history of malaise, low-grade fevers, and jaundice.

PAST MEDICAL HISTORY:
Active Problem List:
- Spontaneous bacterial peritonitis (diagnosed 2020-01-15)
- Pancreatic ascites (diagnosed 2020-04-15)
- Chronic hepatitis C infection (Genotype 1a)
- Alcohol use disorder, severe, in early remission
- Chronic pancreatitis secondary to alcohol use
- History of hepatic encephalopathy
- End-stage liver disease, status post orthotopic liver transplantation

He underwent a liver transplant 2 months ago due to end-stage liver disease secondary to hepatitis C.

PAST SURGICAL HISTORY:
- Orthotopic liver transplantation (2 months prior to most recent encounter)
- ERCP with pancreatic duct stent placement
- Multiple therapeutic paracenteses (historical)

MEDICATIONS:
His current medications include tacrolimus, mycophenolate mofetil, and prednisone.

Home Medications:
- Sulfamethoxazole-trimethoprim 800/160 mg three times weekly (PJP prophylaxis)
- Pancrelipase 36,000 units with meals
- Thiamine 100 mg daily
- Folic acid 1 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis
- Mother: Type 2 diabetes mellitus
- Brother: Chronic hepatitis C infection

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 6 months ago. Alcohol: Chronic heavy alcohol use (6-8 standard drinks daily for 15 years), last use 4 months ago. Occupation: Former construction worker, currently on medical disability.

VITAL SIGNS:
On examination, his temperature is 38.2°C (100.8°F), blood pressure is 130/85 mmHg, pulse is 88/min, and respiratory rate is 16/min.

PHYSICAL EXAMINATION:
He appears jaundiced with scleral icterus. His liver is palpable 3 cm below the right costal margin, and it is tender on palpation. There is no ascites or peripheral edema.

LABORATORY DATA:
Laboratory studies reveal:

- Total bilirubin: 3.2 mg/dL (normal: 0.3-1.2 mg/dL)
- Direct bilirubin: 2.1 mg/dL (normal: 0.0-0.3 mg/dL)
- AST: 120 U/L (normal: 10-40 U/L)
- ALT: 115 U/L (normal: 7-56 U/L)
- Alkaline phosphatase: 310 U/L (normal: 44-147 U/L)
