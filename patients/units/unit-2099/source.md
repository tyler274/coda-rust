# synthetic_hospital patient 2099

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 58 at first encounter, sex M, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-12-19 (Tue) | inpatient | Internal Medicine |
| 1 | 2020-07-15 | 2024-06-10 (Mon) | outpatient | Gastroenterology |
| 2 | 2021-07-15 | 2025-06-16 (Mon) | outpatient | Gastroenterology |
| 3 | 2022-07-15 | 2026-06-15 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Sulfa antibiotics (hives)",
  "Iodine contrast (anaphylaxis)"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Construction site supervisor",
 "alcohol_use": "Former heavy use (approximately 60-80 g daily for 20 years); sober for the past 12 months",
 "family_history": [
  "Father: Pancreatic adenocarcinoma (diagnosed age 62, deceased)",
  "Mother: Hypertension and type 2 diabetes mellitus",
  "Paternal uncle: Alcohol-related liver cirrhosis",
  "One sibling with chronic pancreatitis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "36,000 units lipase with meals and 18,000 units with snacks",
   "name": "Pancrelipase (Creon)"
  },
  {
   "dose": "24 units subcutaneously at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "sliding scale with meals",
   "name": "Insulin lispro"
  },
  {
   "dose": "300 mg orally three times daily",
   "name": "Gabapentin"
  },
  {
   "dose": "5 mg orally daily",
   "name": "Vitamin K (phytonadione)"
  },
  {
   "dose": "2000 units orally daily",
   "name": "Vitamin D3"
  },
  {
   "dose": "5 mg orally every 6 hours as needed for abdominal pain",
   "name": "Oxycodone"
  }
 ],
 "surgical_history": [
  "Cholecystectomy (age 45) for gallstone disease",
  "ERCP with pancreatic sphincterotomy and stent placement (age 54)",
  "Diagnostic laparoscopy (age 56) for abdominal pain evaluation"
 ],
 "chronic_conditions": [
  "Chronic calcific pancreatitis (10-year history)",
  "Alcohol use disorder in sustained remission",
  "Secondary diabetes mellitus (Type 3c) due to chronic pancreatitis",
  "Malabsorption syndrome with fat-soluble vitamin deficiency",
  "Chronic abdominal pain syndrome"
 ],
 "age_at_first_encounter": 58
}
```

Primary diagnoses: ["Pancreatic carcinoma", "Suspected malignant neoplasm of lung", "Pancreatic neuroendocrine tumor localization", "Coagulopathy due to vitamin K deficiency"]
Comorbidities: ["Anemia", "Cachexia"]

## Encounter 0: source 2020-01-15, practice 2023-12-19, inpatient, Internal Medicine

Chief complaint: Blood in stool and increased bruising on arms during hospitalization for infective endocarditis

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Blood in stool and increased bruising on arms during hospitalization for infective endocarditis

HISTORY OF PRESENT ILLNESS:
He is hospitalized for treatment of infective endocarditis.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic calcific pancreatitis (10-year history)
- Alcohol use disorder in sustained remission
- Secondary diabetes mellitus (Type 3c) due to chronic pancreatitis
- Malabsorption syndrome with fat-soluble vitamin deficiency
- Chronic abdominal pain syndrome

PAST SURGICAL HISTORY:
- Cholecystectomy (age 45) for gallstone disease
- ERCP with pancreatic sphincterotomy and stent placement (age 54)
- Diagnostic laparoscopy (age 56) for abdominal pain evaluation

MEDICATIONS:
He has been receiving intravenous broad-spectrum antibiotics for the past six weeks.

Home Medications:
- Pancrelipase (Creon) 36,000 units lipase with meals and 18,000 units with snacks
- Insulin glargine 24 units subcutaneously at bedtime
- Insulin lispro sliding scale with meals
- Gabapentin 300 mg orally three times daily
- Vitamin K (phytonadione) 5 mg orally daily
- Vitamin D3 2000 units orally daily
- Oxycodone 5 mg orally every 6 hours as needed for abdominal pain

ALLERGIES:
Sulfa antibiotics (hives), Iodine contrast (anaphylaxis)

FAMILY HISTORY:
- Father: Pancreatic adenocarcinoma (diagnosed age 62, deceased)
- Mother: Hypertension and type 2 diabetes mellitus
- Paternal uncle: Alcohol-related liver cirrhosis
- One sibling with chronic pancreatitis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former heavy use (approximately 60-80 g daily for 20 years); sober for the past 12 months. Occupation: Construction site supervisor.

PHYSICAL EXAMINATION:
Physical examination shows multiple ecchymoses on his forearms and scattered petechiae. Rectal examination reveals dark stool that is positive for occult blood. There is no hepatosplenomegaly.

LABORATORY DATA:
Laboratory studies show:
Hemoglobin 10.2 g/dL
Platelets 250,000 /µL
WBCs 7,400 /µL
PT 24 seconds
aPTT 48 seconds
Bleeding time Normal

## Encounter 1: source 2020-07-15, practice 2024-06-10, outpatient, Gastroenterology

Chief complaint: Persistent fatigue, decreased appetite, unintentional weight loss, and recurring itchy rash on lower abdomen and groin

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue, decreased appetite, unintentional weight loss, and recurring itchy rash on lower abdomen and groin

HISTORY OF PRESENT ILLNESS:
The patient with chronic calcific pancreatitis and prior coagulopathy secondary to vitamin K deficiency returns for evaluation of persistent fatigue, decreased appetite, and an unintentional 7-kg (15-lb) weight loss over the past 3 months. He also reports a recurring, mildly itchy rash on his lower abdomen and groin that seems to flare intermittently. He has occasional episodes of mild diarrhea but denies blood in stool.

PAST MEDICAL HISTORY:
Active Problem List:
- Coagulopathy due to vitamin K deficiency (diagnosed 2020-01-15)
- Chronic calcific pancreatitis (10-year history)
- Alcohol use disorder in sustained remission
- Secondary diabetes mellitus (Type 3c) due to chronic pancreatitis
- Malabsorption syndrome with fat-soluble vitamin deficiency
- Chronic abdominal pain syndrome

PAST SURGICAL HISTORY:
- Cholecystectomy (age 45) for gallstone disease
- ERCP with pancreatic sphincterotomy and stent placement (age 54)
- Diagnostic laparoscopy (age 56) for abdominal pain evaluation

MEDICATIONS:
Home Medications:
- Pancrelipase (Creon) 36,000 units lipase with meals and 18,000 units with snacks
- Insulin glargine 24 units subcutaneously at bedtime
- Insulin lispro sliding scale with meals
- Gabapentin 300 mg orally three times daily
- Vitamin K (phytonadione) 5 mg orally daily
- Vitamin D3 2000 units orally daily
- Oxycodone 5 mg orally every 6 hours as needed for abdominal pain

ALLERGIES:
Sulfa antibiotics (hives), Iodine contrast (anaphylaxis)

FAMILY HISTORY:
- Father: Pancreatic adenocarcinoma (diagnosed age 62, deceased)
- Mother: Hypertension and type 2 diabetes mellitus
- Paternal uncle: Alcohol-related liver cirrhosis
- One sibling with chronic pancreatitis

SOCIAL HISTORY:
He denies alcohol or illicit drug use and has never smoked.

PHYSICAL EXAMINATION:
Physical examination reveals a thin man with mild erythematous patches with central clearing on the perineum and thighs. Abdominal examination shows no organomegaly or tenderness. Neurologic examination is normal.

LABORATORY DATA:
Laboratory studies show glucose of 212 mg/dL and glucagon of 1,420 pg/mL (normal: <200 pg/mL).

IMAGING:
Abdominal imaging demonstrates a pancreatic mass.

## Encounter 2: source 2021-07-15, practice 2025-06-16, outpatient, Gastroenterology

Chief complaint: Progressive upper abdominal pain and unexplained weight loss over the past 3 months

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive upper abdominal pain and unexplained weight loss over the past 3 months

HISTORY OF PRESENT ILLNESS:
The patient returns one year following workup for pancreatic neuroendocrine tumor. He reports progressive upper abdominal pain and unexplained weight loss over the past 3 months. He denies jaundice, changes in stool color, or pruritus.

PAST MEDICAL HISTORY:
Active Problem List:
- Coagulopathy due to vitamin K deficiency (diagnosed 2020-01-15)
- Pancreatic neuroendocrine tumor localization (diagnosed 2020-07-15)
- Chronic calcific pancreatitis (10-year history)
- Alcohol use disorder in sustained remission
- Secondary diabetes mellitus (Type 3c) due to chronic pancreatitis
- Malabsorption syndrome with fat-soluble vitamin deficiency
- Chronic abdominal pain syndrome

His medical history is significant for alcohol abuse, although he has been sober for the past year.

PAST SURGICAL HISTORY:
- Cholecystectomy (age 45) for gallstone disease
- ERCP with pancreatic sphincterotomy and stent placement (age 54)
- Diagnostic laparoscopy (age 56) for abdominal pain evaluation

MEDICATIONS:
Home Medications:
- Pancrelipase (Creon) 36,000 units lipase with meals and 18,000 units with snacks
- Insulin glargine 24 units subcutaneously at bedtime
- Insulin lispro sliding scale with meals
- Gabapentin 300 mg orally three times daily
- Vitamin K (phytonadione) 5 mg orally daily
- Vitamin D3 2000 units orally daily
- Oxycodone 5 mg orally every 6 hours as needed for abdominal pain

ALLERGIES:
Sulfa antibiotics (hives), Iodine contrast (anaphylaxis)

FAMILY HISTORY:
- Father: Pancreatic adenocarcinoma (diagnosed age 62, deceased)
- Mother: Hypertension and type 2 diabetes mellitus
- Paternal uncle: Alcohol-related liver cirrhosis
- One sibling with chronic pancreatitis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former heavy use (approximately 60-80 g daily for 20 years); sober for the past 12 months. Occupation: Construction site supervisor.

PHYSICAL EXAMINATION:
On physical examination, he appears cachectic and has epigastric tenderness. No palpable masses are appreciated, and there is no scleral icterus.

LABORATORY DATA:
Laboratory tests reveal:
- Hemoglobin: 12.1 g/dL (normal: 13.8-17.2 g/dL)
- White blood cell count: 7,000/uL (normal: 4,500-11,000/uL)
- Platelets: 250,000/uL (normal: 150,000-450,000/uL)
- Total bilirubin: 0.8 mg/dL (normal: 0.3-1.2 mg/dL)
- Alkaline phosphatase: 120 U/L (normal: 40-130 U/L)
- Aspartate aminotransferase (AST): 45 U/L (normal: 10-40 U/L)
- Alanine aminotransferase (ALT): 50 U/L (normal: 7-56 U/L)
- CA 19-9: 150 U/mL (normal: <37 U/mL)

## Encounter 3: source 2022-07-15, practice 2026-06-15, ed, Emergency Medicine

Chief complaint: Worsening cough, hemoptysis, and cachexia over 6 months

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Emily Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening cough, hemoptysis, and cachexia over 6 months

HISTORY OF PRESENT ILLNESS:
The patient is a 60-year-old male with chronic calcific pancreatitis, pancreatic carcinoma diagnosed in 2021, and prior coagulopathy. He presents with a 6-month history of worsening cough, hemoptysis, and cachexia.

PAST MEDICAL HISTORY:
Active Problem List:
- Coagulopathy due to vitamin K deficiency (diagnosed 2020-01-15)
- Pancreatic neuroendocrine tumor localization (diagnosed 2020-07-15)
- Pancreatic carcinoma (diagnosed 2021-07-15)
- Chronic calcific pancreatitis (10-year history)
- Alcohol use disorder in sustained remission
- Secondary diabetes mellitus (Type 3c) due to chronic pancreatitis
- Malabsorption syndrome with fat-soluble vitamin deficiency
- Chronic abdominal pain syndrome

PAST SURGICAL HISTORY:
- Cholecystectomy (age 45) for gallstone disease
- ERCP with pancreatic sphincterotomy and stent placement (age 54)
- Diagnostic laparoscopy (age 56) for abdominal pain evaluation

MEDICATIONS:
Home Medications:
- Pancrelipase (Creon) 36,000 units lipase with meals and 18,000 units with snacks
- Insulin glargine 24 units subcutaneously at bedtime
- Insulin lispro sliding scale with meals
- Gabapentin 300 mg orally three times daily
- Vitamin K (phytonadione) 5 mg orally daily
- Vitamin D3 2000 units orally daily
- Oxycodone 5 mg orally every 6 hours as needed for abdominal pain

ALLERGIES:
Sulfa antibiotics (hives), Iodine contrast (anaphylaxis)

FAMILY HISTORY:
- Father: Pancreatic adenocarcinoma (diagnosed age 62, deceased)
- Mother: Hypertension and type 2 diabetes mellitus
- Paternal uncle: Alcohol-related liver cirrhosis
- One sibling with chronic pancreatitis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former heavy use (approximately 60-80 g daily for 20 years); sober for the past 12 months. Occupation: Construction site supervisor.

PATHOLOGY:
A gross pathologic specimen from his autopsy is shown.

ASSESSMENT AND PLAN:
Recently passes away from metastatic malignancy
