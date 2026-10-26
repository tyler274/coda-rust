# synthetic_hospital patient 1970

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 52 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-06-27 (Tue) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2024-01-08 (Mon) | outpatient | Gastroenterology |
| 2 | 2021-07-15 | 2024-12-31 (Tue) | outpatient | Gastroenterology |
| 3 | 2022-07-15 | 2025-12-30 (Tue) | ed | Emergency Medicine |
| 4 | 2023-01-15 | 2026-06-30 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicaid",
 "occupation": "Former construction laborer, currently on disability leave",
 "alcohol_use": "Severe alcohol use disorder, currently in sustained remission (6 months sober) and enrolled in outpatient rehabilitation; 15-year history of heavy consumption (approximately 6 beers daily) with documented intermittent relapses during treatment course",
 "family_history": [
  "Father: Alcohol use disorder with cirrhosis (deceased age 58)",
  "Mother: Hypertension, type 2 diabetes mellitus",
  "Sibling: Alcohol use disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 20 pack-years, quit 6 months ago",
 "home_medications": [
  {
   "dose": "5 mg oral daily",
   "name": "Amlodipine"
  },
  {
   "dose": "20 mg oral twice daily",
   "name": "Propranolol"
  },
  {
   "dose": "30 mL oral twice daily, titrated to 2-3 soft bowel movements daily",
   "name": "Lactulose"
  },
  {
   "dose": "40 mg oral daily",
   "name": "Pantoprazole"
  },
  {
   "dose": "100 mg oral daily",
   "name": "Spironolactone"
  },
  {
   "dose": "40 mg oral daily",
   "name": "Furosemide"
  }
 ],
 "surgical_history": [
  "Esophageal variceal band ligation (multiple sessions)",
  "Diagnostic esophagogastroduodenoscopy"
 ],
 "chronic_conditions": [
  "Alcohol use disorder, severe",
  "Alcoholic cirrhosis with portal hypertension",
  "Esophageal varices",
  "Hypertension",
  "Peptic ulcer disease",
  "History of recurrent hepatic encephalopathy",
  "History of recurrent ascites"
 ],
 "age_at_first_encounter": 52
}
```

Primary diagnoses: ["Esophageal varices with indication for primary prophylaxis", "Alcoholic cirrhosis", "Bleeding esophageal varices", "Portal hypertension"]
Comorbidities: ["Telangiectasia", "Portal hypertension", "Alcohol use disorder", "Alcoholic cirrhosis", "Thrombocytopenia", "Hypoalbuminemia", "Decompensated cirrhosis requiring transplantation evaluation", "Anemia", "Splenomegaly"]

## Encounter 0: source 2020-01-15, practice 2023-06-27, outpatient, Internal Medicine

Chief complaint: Progressive abdominal distension and discomfort over the past two months. He reports a significant decrease in appetite and a weight loss of 10 pounds during this period.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive abdominal distension and discomfort over the past two months. He reports a significant decrease in appetite and a weight loss of 10 pounds during this period.

HISTORY OF PRESENT ILLNESS:
He reports a significant decrease in appetite and a weight loss of 10 pounds during this period.

PAST MEDICAL HISTORY:
Active Problem List:
- Alcohol use disorder, severe
- Alcoholic cirrhosis with portal hypertension
- Esophageal varices
- Hypertension
- Peptic ulcer disease
- History of recurrent hepatic encephalopathy
- History of recurrent ascites

PAST SURGICAL HISTORY:
- Esophageal variceal band ligation (multiple sessions)
- Diagnostic esophagogastroduodenoscopy

MEDICATIONS:
Home Medications:
- Amlodipine 5 mg oral daily
- Propranolol 20 mg oral twice daily
- Lactulose 30 mL oral twice daily, titrated to 2-3 soft bowel movements daily
- Pantoprazole 40 mg oral daily
- Spironolactone 100 mg oral daily
- Furosemide 40 mg oral daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 58)
- Mother: Hypertension, type 2 diabetes mellitus
- Sibling: Alcohol use disorder

SOCIAL HISTORY:
He admits to consuming about six beers daily for the past 15 years but denies any recent alcohol intake in the last month.

VITAL SIGNS:
His vital signs are stable, and he is afebrile

PHYSICAL EXAMINATION:
Abdominal examination reveals a fluid wave and shifting dullness. His skin shows spider angiomas and palmar erythema.

LABORATORY DATA:
- Hemoglobin: 12.5 g/dL
- Platelet count: 110,000/uL
- INR: 1.3
- Total bilirubin: 2.8 mg/dL
- Albumin: 3.2 g/dL
- AST: 120 U/L
- ALT: 90 U/L
- Alkaline phosphatase: 110 U/L

IMAGING:
Abdominal ultrasound with Doppler shows evidence of cirrhosis, splenomegaly, and ascites without any signs of portal vein thrombosis.

## Encounter 1: source 2020-07-15, practice 2024-01-08, outpatient, Gastroenterology

Chief complaint: Presents to the clinic for a routine follow-up. He reports no acute complaints but mentions occasional mild epigastric discomfort, which he attributes to "indigestion."

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Presents to the clinic for a routine follow-up. He reports no acute complaints but mentions occasional mild epigastric discomfort, which he attributes to "indigestion."

HISTORY OF PRESENT ILLNESS:
He returns for follow-up of alcoholic cirrhosis diagnosed six months prior during evaluation for abdominal distension. He has been sober for the last 6 months and is currently enrolled in an alcohol rehabilitation program. He reports no acute complaints but mentions occasional mild epigastric discomfort, which he attributes to "indigestion."

PAST MEDICAL HISTORY:
Active Problem List:
- Alcoholic cirrhosis (diagnosed 2020-01-15)
- Alcohol use disorder, severe
- Alcoholic cirrhosis with portal hypertension
- Esophageal varices
- Hypertension
- Peptic ulcer disease
- History of recurrent hepatic encephalopathy
- History of recurrent ascites

His past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Esophageal variceal band ligation (multiple sessions)
- Diagnostic esophagogastroduodenoscopy

MEDICATIONS:
He takes amlodipine.

Home Medications:
- Propranolol 20 mg oral twice daily
- Lactulose 30 mL oral twice daily, titrated to 2-3 soft bowel movements daily
- Pantoprazole 40 mg oral daily
- Spironolactone 100 mg oral daily
- Furosemide 40 mg oral daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 58)
- Mother: Hypertension, type 2 diabetes mellitus
- Sibling: Alcohol use disorder

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 6 months ago. Alcohol: Severe alcohol use disorder, currently in sustained remission (6 months sober) and enrolled in outpatient rehabilitation; 15-year history of heavy consumption (approximately 6 beers daily) with documented intermittent relapses during treatment course. Occupation: Former construction laborer, currently on disability leave.

VITAL SIGNS:
His blood pressure is 140/90 mmHg, heart rate is 78 bpm, and he is afebrile

PHYSICAL EXAMINATION:
Abdominal examination reveals moderate splenomegaly and a palpable, firm liver edge. There is no ascites or peripheral edema.

LABORATORY DATA:
Platelet count: 110,000/µL
- AST: 70 U/L
- ALT: 50 U/L
- Total bilirubin: 1.2 mg/dL
- Albumin: 3.9 g/dL
- INR: 1.1

OTHER STUDIES:
An esophagogastroduodenoscopy (EGD) performed reveals small esophageal varices without red wale signs or active bleeding.

## Encounter 2: source 2021-07-15, practice 2024-12-31, outpatient, Gastroenterology

Chief complaint: Routine check-up. He reports a recent episode of hematemesis that occurred once a week ago, which he attributed to his history of peptic ulcer disease. He denies any further episodes of vomiting, changes in bowel habits, or abdominal pain.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Jennifer Okonkwo, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine check-up. He reports a recent episode of hematemesis that occurred once a week ago, which he attributed to his history of peptic ulcer disease. He denies any further episodes of vomiting, changes in bowel habits, or abdominal pain.

HISTORY OF PRESENT ILLNESS:
Patient is a 53-year-old male with alcoholic cirrhosis and esophageal varices who returns for routine gastroenterology follow-up. He reports a recent episode of hematemesis that occurred once a week ago, which he attributed to his history of peptic ulcer disease. He denies any further episodes of vomiting, changes in bowel habits, or abdominal pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Alcoholic cirrhosis (diagnosed 2020-01-15)
- Esophageal varices with indication for primary prophylaxis (diagnosed 2020-07-15)
- Alcohol use disorder, severe
- Alcoholic cirrhosis with portal hypertension
- Esophageal varices
- Hypertension
- Peptic ulcer disease
- History of recurrent hepatic encephalopathy
- History of recurrent ascites

PAST SURGICAL HISTORY:
- Esophageal variceal band ligation (multiple sessions)
- Diagnostic esophagogastroduodenoscopy

MEDICATIONS:
Home Medications:
- Amlodipine 5 mg oral daily
- Propranolol 20 mg oral twice daily
- Lactulose 30 mL oral twice daily, titrated to 2-3 soft bowel movements daily
- Pantoprazole 40 mg oral daily
- Spironolactone 100 mg oral daily
- Furosemide 40 mg oral daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 58)
- Mother: Hypertension, type 2 diabetes mellitus
- Sibling: Alcohol use disorder

SOCIAL HISTORY:
His last drink was two days ago.

VITAL SIGNS:
His vital signs are stable

PHYSICAL EXAMINATION:
Abdominal examination reveals mild splenomegaly without ascites. Caput medusae are not observed.

LABORATORY DATA:
Laboratory tests reveal the following:

- Hemoglobin: 12.5 g/dL
- Platelet count: 90,000/uL
- INR: 1.2
- AST: 70 U/L
- ALT: 30 U/L
- Alkaline phosphatase: 100 U/L
- Albumin: 3.2 g/dL

OTHER STUDIES:
An upper endoscopy shows the presence of large esophageal varices without active bleeding.

## Encounter 3: source 2022-07-15, practice 2025-12-30, ed, Emergency Medicine

Chief complaint: Brought to the emergency department by his wife after he vomited a large amount of blood at home. He reports several weeks of progressive fatigue, early satiety, and intermittent mild abdominal discomfort.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought to the emergency department by his wife after he vomited a large amount of blood at home. He reports several weeks of progressive fatigue, early satiety, and intermittent mild abdominal discomfort.

HISTORY OF PRESENT ILLNESS:
The patient with alcoholic cirrhosis and esophageal varices presents with hematemesis. He reports several weeks of progressive fatigue, early satiety, and intermittent mild abdominal discomfort.

PAST MEDICAL HISTORY:
Active Problem List:
- Alcoholic cirrhosis (diagnosed 2020-01-15)
- Esophageal varices with indication for primary prophylaxis (diagnosed 2020-07-15)
- Alcohol use disorder, severe
- Alcoholic cirrhosis with portal hypertension
- Esophageal varices
- Hypertension
- Peptic ulcer disease
- History of recurrent hepatic encephalopathy
- History of recurrent ascites

His past medical history includes long-standing alcohol use disorder, and he has been hospitalized in the past for ascites and hepatic encephalopathy.

PAST SURGICAL HISTORY:
- Esophageal variceal band ligation (multiple sessions)
- Diagnostic esophagogastroduodenoscopy

MEDICATIONS:
Home Medications:
- Amlodipine 5 mg oral daily
- Propranolol 20 mg oral twice daily
- Lactulose 30 mL oral twice daily, titrated to 2-3 soft bowel movements daily
- Pantoprazole 40 mg oral daily
- Spironolactone 100 mg oral daily
- Furosemide 40 mg oral daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 58)
- Mother: Hypertension, type 2 diabetes mellitus
- Sibling: Alcohol use disorder

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 6 months ago. Alcohol: Severe alcohol use disorder, currently in sustained remission (6 months sober) and enrolled in outpatient rehabilitation; 15-year history of heavy consumption (approximately 6 beers daily) with documented intermittent relapses during treatment course. Occupation: Former construction laborer, currently on disability leave.

PHYSICAL EXAMINATION:
On examination, he appears pale and diaphoretic with a distended abdomen, shifting dullness, and multiple spider angiomas on his chest.

LABORATORY DATA:
Laboratory tests reveal anemia and thrombocytopenia.

OTHER STUDIES:
After fluid resuscitation, an upper endoscopy reveals dilated veins in the distal esophagus with signs of recent hemorrhage.

ASSESSMENT AND PLAN:
This bleeding most likely results from dilation of submucosal vessels at a portosystemic anastomosis involving

## Encounter 4: source 2023-01-15, practice 2026-06-30, ed, Emergency Medicine

Chief complaint: Sudden onset hematemesis and signs of hypovolemic shock.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Emily Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset hematemesis and signs of hypovolemic shock.

PAST MEDICAL HISTORY:
Active Problem List:
- Alcoholic cirrhosis (diagnosed 2020-01-15)
- Esophageal varices with indication for primary prophylaxis (diagnosed 2020-07-15)
- Bleeding esophageal varices (diagnosed 2022-07-15)
- Alcohol use disorder, severe
- Alcoholic cirrhosis with portal hypertension
- Esophageal varices
- Hypertension
- Peptic ulcer disease
- History of recurrent hepatic encephalopathy
- History of recurrent ascites

He has a background of jaundice, and was previously hospitalized for hepatic encephalopathy and upper GI bleeding. Endoscopic band ligation of his esophageal varices failed to control recurrent bleeding, and nonselective beta-blockers were discontinued due to bradycardia.

PAST SURGICAL HISTORY:
- Esophageal variceal band ligation (multiple sessions)
- Diagnostic esophagogastroduodenoscopy

MEDICATIONS:
Home Medications:
- Amlodipine 5 mg oral daily
- Propranolol 20 mg oral twice daily
- Lactulose 30 mL oral twice daily, titrated to 2-3 soft bowel movements daily
- Pantoprazole 40 mg oral daily
- Spironolactone 100 mg oral daily
- Furosemide 40 mg oral daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 58)
- Mother: Hypertension, type 2 diabetes mellitus
- Sibling: Alcohol use disorder

SOCIAL HISTORY:
He has a background of chronic alcohol use.

PHYSICAL EXAMINATION:
On exam, he has tense ascites, scleral icterus, and spider angiomata.

LABORATORY DATA:
Laboratory tests show thrombocytopenia, elevated INR, and low albumin.

IMAGING:
Abdominal ultrasound reveals a shrunken liver, portal vein dilation, and splenomegaly.

PLAN:
After initial stabilization and octreotide infusion, the surgical team discusses portosystemic shunt options to decompress the portal system.
