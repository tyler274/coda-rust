# synthetic_hospital patient 2057

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, White, non-Hispanic (Northern European ancestry), Uninsured.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2019-03-29 (Fri) | outpatient | Gastroenterology |
| 1 | 2020-07-15 | 2019-10-07 (Mon) | outpatient | Internal Medicine |
| 2 | 2021-07-15 | 2020-09-21 (Mon) | outpatient | Rheumatology |
| 3 | 2023-01-15 | 2022-04-04 (Mon) | outpatient | Cardiology |
| 4 | 2024-07-15 | 2023-09-18 (Mon) | ed | Emergency Medicine |
| 5 | 2025-01-15 | 2024-03-20 (Wed) | icu | Emergency Medicine |
| 6 | 2026-01-15 | 2025-03-28 (Fri) | inpatient | Emergency Medicine |
| 7 | 2027-01-15 | 2026-03-27 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Uninsured",
 "occupation": "Construction laborer",
 "alcohol_use": "Former heavy alcohol use; abstinent for 3 years (history of alcohol use disorder, in sustained remission)",
 "family_history": [
  "Father: Hemochromatosis with cirrhosis and diabetes (deceased age 58)",
  "Mother: Type 2 diabetes mellitus",
  "Brother: Hereditary hemochromatosis",
  "Paternal uncle: Liver disease"
 ],
 "race_ethnicity": "White, non-Hispanic (Northern European ancestry)",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Furosemide"
  },
  {
   "dose": "100 mg PO daily",
   "name": "Spironolactone"
  }
 ],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus (2 years duration, secondary to hemochromatosis)",
  "Hereditary Hemochromatosis (HFE-related)",
  "Cirrhosis secondary to iron overload",
  "Obesity (BMI 34 kg/m\u00b2)",
  "Mixed hyperlipidemia",
  "Hemochromatosis arthropathy (iron deposition)"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Hereditary Hemochromatosis with arthropathy", "Restrictive cardiomyopathy", "Perianal abscess", "Pyogenic liver abscess", "Diabetic ketoacidosis", "Acute pancreatitis"]
Comorbidities: ["Hepatomegaly", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2019-03-29, outpatient, Gastroenterology

Chief complaint: Fatigue, joint pain, and abdominal discomfort with unintentional weight loss and bronze skin discoloration

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, joint pain, and abdominal discomfort with unintentional weight loss and bronze skin discoloration

HISTORY OF PRESENT ILLNESS:
He mentions a recent unintentional weight loss of 10 pounds and has noticed a bronze discoloration of his skin.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus (2 years duration, secondary to hemochromatosis)
- Hereditary Hemochromatosis (HFE-related)
- Cirrhosis secondary to iron overload
- Obesity (BMI 34 kg/m²)
- Mixed hyperlipidemia
- Hemochromatosis arthropathy (iron deposition)

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Furosemide 40 mg PO daily
- Spironolactone 100 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
He has no family history of liver disease

SOCIAL HISTORY:
He denies alcohol use

PHYSICAL EXAMINATION:
On physical examination, he is afebrile, with hepatomegaly and no signs of chronic liver disease.

LABORATORY DATA:
- Serum ferritin: 1,500 ng/mL
- Transferrin saturation: 65%
- ALT: 85 U/L
- AST: 78 U/L
- Fasting glucose: 160 mg/dL
- Hemoglobin A1c: 7.2%

## Encounter 1: source 2020-07-15, practice 2019-10-07, outpatient, Internal Medicine

Chief complaint: Worsening joint pain in hands and fatigue with skin darkening

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening joint pain in hands and fatigue with skin darkening

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of hemochromatosis evaluated in January. He reports darkening of his skin despite minimal sun exposure.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary Hemochromatosis with arthropathy (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus (2 years duration, secondary to hemochromatosis)
- Hereditary Hemochromatosis (HFE-related)
- Cirrhosis secondary to iron overload
- Obesity (BMI 34 kg/m²)
- Mixed hyperlipidemia
- Hemochromatosis arthropathy (iron deposition)

His medical history is significant for type 2 diabetes mellitus diagnosed two years ago.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Furosemide 40 mg PO daily
- Spironolactone 100 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hemochromatosis with cirrhosis and diabetes (deceased age 58)
- Mother: Type 2 diabetes mellitus
- Brother: Hereditary hemochromatosis
- Paternal uncle: Liver disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former heavy alcohol use; abstinent for 3 years (history of alcohol use disorder, in sustained remission). Occupation: Construction laborer.

PHYSICAL EXAMINATION:
On examination, his skin appears bronze-colored and he has tenderness in the second and third metacarpophalangeal joints bilaterally

LABORATORY DATA:
Laboratory evaluation is performed.
Hemoglobin 15.2 g/dL
Hematocrit                                        46%
MCV 88 fL
Serum ferritin 950 ng/mL
Transferrin saturation                              78%

## Encounter 2: source 2021-07-15, practice 2020-09-21, outpatient, Rheumatology

Chief complaint: Progressive joint pain and stiffness in knees and wrists with acute episodic swelling

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Priya Patel, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive joint pain and stiffness in knees and wrists with acute episodic swelling

HISTORY OF PRESENT ILLNESS:
The patient is a 46-year-old male with hereditary hemochromatosis and established iron deposition arthropathy who returns for further evaluation of progressive joint symptoms. Having been previously assessed in Gastroenterology and Internal Medicine for similar complaints, he presents to the clinic with a 6-month history of progressive joint pain and stiffness, particularly in his knees and wrists. He also reports experiencing several episodes of acute knee pain with swelling, which resolved spontaneously over a few days.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary Hemochromatosis with arthropathy (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus (2 years duration, secondary to hemochromatosis)
- Hereditary Hemochromatosis (HFE-related)
- Cirrhosis secondary to iron overload
- Obesity (BMI 34 kg/m²)
- Mixed hyperlipidemia
- Hemochromatosis arthropathy (iron deposition)

He has a history of type 2 diabetes and was recently diagnosed with cirrhosis during an evaluation for persistent fatigue and abdominal distension.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Furosemide 40 mg PO daily
- Spironolactone 100 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is notable for similar joint problems in his father.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former heavy alcohol use; abstinent for 3 years (history of alcohol use disorder, in sustained remission). Occupation: Construction laborer.

PHYSICAL EXAMINATION:
On physical examination, his liver is palpable 3 cm below the right costal margin, and there is mild tenderness over the affected joints without significant warmth or erythema.

LABORATORY DATA:
Laboratory studies reveal the following:

- Serum iron: 220 mcg/dL (normal: 60-170 mcg/dL)
- Total iron-binding capacity: 250 mcg/dL (normal: 250-450 mcg/dL)
- Transferrin saturation: 88% (normal: 20-50%)
- Ferritin: 900 ng/mL (normal: 30-400 ng/mL)

IMAGING:
Knee radiographs show chondrocalcinosis.

## Encounter 3: source 2023-01-15, practice 2022-04-04, outpatient, Cardiology

Chief complaint: Progressive fatigue, lower extremity swelling, and early satiety with exertional dyspnea

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue, lower extremity swelling, and early satiety with exertional dyspnea

HISTORY OF PRESENT ILLNESS:
Patient returns with hereditary hemochromatosis and arthropathy previously managed in gastroenterology, internal medicine, and rheumatology. He presents with progressive fatigue, lower extremity swelling, and early satiety over the past 6 months. He reports mild exertional dyspnea but denies chest pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary Hemochromatosis with arthropathy (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus (2 years duration, secondary to hemochromatosis)
- Hereditary Hemochromatosis (HFE-related)
- Cirrhosis secondary to iron overload
- Obesity (BMI 34 kg/m²)
- Mixed hyperlipidemia
- Hemochromatosis arthropathy (iron deposition)

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Furosemide 40 mg PO daily
- Spironolactone 100 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hemochromatosis with cirrhosis and diabetes (deceased age 58)
- Mother: Type 2 diabetes mellitus
- Brother: Hereditary hemochromatosis
- Paternal uncle: Liver disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former heavy alcohol use; abstinent for 3 years (history of alcohol use disorder, in sustained remission). Occupation: Construction laborer.

PHYSICAL EXAMINATION:
Jugular venous distension, hepatomegaly, bilateral pitting edema, and clear lung fields. There is darkening of the skin of the hands and forearms. Cardiac auscultation reveals normal S1 and S2 with an S4 gallop.

IMAGING:
Echocardiogram shows normal left ventricular size with marked concentric thickening of the ventricular walls and biatrial enlargement. Left ventricular ejection fraction is preserved.

## Encounter 4: source 2024-07-15, practice 2023-09-18, ed, Emergency Medicine

Chief complaint: Severe epigastric abdominal pain radiating to the back after large meal

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Michael Chang, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe epigastric abdominal pain radiating to the back after large meal

HISTORY OF PRESENT ILLNESS:
The patient is a 49-year-old male with a history of hereditary hemochromatosis complicated by arthropathy and restrictive cardiomyopathy who presents to the emergency department with 18 hours of severe epigastric abdominal pain radiating to the back. The pain began suddenly after a large dinner and has progressively worsened. He has associated nausea and several episodes of nonbloody vomiting. He denies alcohol use, gallstone history, recent trauma, or prior similar episodes.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary Hemochromatosis with arthropathy (diagnosed 2020-01-15)
- Restrictive cardiomyopathy (diagnosed 2023-01-15)
- Type 2 Diabetes Mellitus (2 years duration, secondary to hemochromatosis)
- Hereditary Hemochromatosis (HFE-related)
- Cirrhosis secondary to iron overload
- Obesity (BMI 34 kg/m²)
- Mixed hyperlipidemia
- Hemochromatosis arthropathy (iron deposition)

His medical history is notable for type 2 diabetes mellitus, obesity, and poorly controlled hyperlipidemia.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He does not take any medications regularly due to cost concerns.

Home Medications:
- Metformin 1000 mg PO BID
- Furosemide 40 mg PO daily
- Spironolactone 100 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
He has no family history of pancreatic or gallbladder disease.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former heavy alcohol use; abstinent for 3 years (history of alcohol use disorder, in sustained remission). Occupation: Construction laborer.

VITAL SIGNS:
Temperature of 37.9°C (100.2°F), pulse 108/min, and blood pressure 138/84 mm Hg

PHYSICAL EXAMINATION:
On examination, he is in moderate distress. Abdominal examination reveals epigastric tenderness without rebound or guarding.

LABORATORY DATA:
Amylase 1,320 U/L
Lipase 1,450 U/L
Triglycerides 2,180 mg/dL
Total cholesterol 265 mg/dL
Alanine aminotransferase 38 U/L
Aspartate aminotransferase 42 U/L

IMAGING:
Abdominal CT with contrast demonstrates an enlarged pancreas with peripancreatic fat stranding and no evidence of necrosis or pseudocyst formation.

## Encounter 5: source 2025-01-15, practice 2024-03-20, icu, Emergency Medicine

Chief complaint: Nausea, vomiting, and abdominal pain with metabolic decompensation

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Aisha Washington, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Nausea, vomiting, and abdominal pain with metabolic decompensation

HISTORY OF PRESENT ILLNESS:
The patient with hemochromatosis, cirrhosis, and secondary diabetes presents to the emergency department with a 2-day history of nausea, vomiting, and abdominal pain

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary Hemochromatosis with arthropathy (diagnosed 2020-01-15)
- Restrictive cardiomyopathy (diagnosed 2023-01-15)
- Acute pancreatitis (diagnosed 2024-07-15)
- Type 2 Diabetes Mellitus (2 years duration, secondary to hemochromatosis)
- Hereditary Hemochromatosis (HFE-related)
- Cirrhosis secondary to iron overload
- Obesity (BMI 34 kg/m²)
- Mixed hyperlipidemia
- Hemochromatosis arthropathy (iron deposition)

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Furosemide 40 mg PO daily
- Spironolactone 100 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hemochromatosis with cirrhosis and diabetes (deceased age 58)
- Mother: Type 2 diabetes mellitus
- Brother: Hereditary hemochromatosis
- Paternal uncle: Liver disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former heavy alcohol use; abstinent for 3 years (history of alcohol use disorder, in sustained remission). Occupation: Construction laborer.

VITAL SIGNS:
His temperature is 37.2°C (99°F), blood pressure is 110/70 mmHg, pulse is 105/min, and respiratory rate is 28/min.

PHYSICAL EXAMINATION:
He appears lethargic and has deep, rapid breathing.
Physical examination reveals dry mucous membranes and decreased skin turgor. His breath has a fruity odor.

LABORATORY DATA:
Serum glucose: 500 mg/dL
Serum bicarbonate: 10 mEq/L
Serum creatinine: 1.2 mg/dL
Serum sodium: 140 mEq/L
Serum potassium: 5.0 mEq/L
Serum chloride: 96 mEq/L
Arterial blood gas pH: 7.20

## Encounter 6: source 2026-01-15, practice 2025-03-28, inpatient, Emergency Medicine

Chief complaint: Fever, chills, and right upper quadrant abdominal pain radiating to right shoulder

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Fever, chills, and right upper quadrant abdominal pain radiating to right shoulder

HISTORY OF PRESENT ILLNESS:
This 51-year-old male with hereditary hemochromatosis and cirrhosis presents with acute abdominal pain. He reports that the pain is constant, sharp, and radiates to his right shoulder. He also complains of nausea and decreased appetite.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary Hemochromatosis with arthropathy (diagnosed 2020-01-15)
- Restrictive cardiomyopathy (diagnosed 2023-01-15)
- Acute pancreatitis (diagnosed 2024-07-15)
- Diabetic ketoacidosis (diagnosed 2025-01-15)
- Type 2 Diabetes Mellitus (2 years duration, secondary to hemochromatosis)
- Hereditary Hemochromatosis (HFE-related)
- Cirrhosis secondary to iron overload
- Obesity (BMI 34 kg/m²)
- Mixed hyperlipidemia
- Hemochromatosis arthropathy (iron deposition)

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Furosemide 40 mg PO daily
- Spironolactone 100 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hemochromatosis with cirrhosis and diabetes (deceased age 58)
- Mother: Type 2 diabetes mellitus
- Brother: Hereditary hemochromatosis
- Paternal uncle: Liver disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former heavy alcohol use; abstinent for 3 years (history of alcohol use disorder, in sustained remission). Occupation: Construction laborer.

VITAL SIGNS:
His temperature is 38.9°C (102°F), heart rate is 110 beats per minute, and blood pressure is 135/85 mmHg

PHYSICAL EXAMINATION:
Abdominal examination reveals hepatomegaly and tenderness on palpation of the right upper quadrant. Chest auscultation notes decreased breath sounds at the right base.

LABORATORY DATA:
White blood cell count: 18,000/mm³ (normal: 4,500-11,000/mm³)
- Hemoglobin: 13.2 g/dL (normal: 13.8-17.2 g/dL)
- Platelet count: 250,000/mm³ (normal: 150,000-450,000/mm³)
- Aspartate aminotransferase (AST): 85 U/L (normal: 8-48 U/L)
- Alanine aminotransferase (ALT): 78 U/L (normal: 7-55 U/L)
- Alkaline phosphatase: 210 U/L (normal: 40-129 U/L)
- Total bilirubin: 1.2 mg/dL (normal: 0.3-1.9 mg/dL)

IMAGING:
An abdominal ultrasound reveals a hypoechoic lesion in the right lobe of the liver. There is also evidence of a small right-sided pleural effusion.

## Encounter 7: source 2027-01-15, practice 2026-03-27, ed, Emergency Medicine

Chief complaint: Severe throbbing perianal pain intensifying with bowel movements and sitting

ENCOUNTER NOTE
Date: 2027-01-15
Provider: Dr. Fatima Al-Hassan, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe throbbing perianal pain intensifying with bowel movements and sitting

HISTORY OF PRESENT ILLNESS:
The patient with hemochromatosis, cirrhosis, and diabetes presents with perianal pain. He reports the pain intensifies during bowel movements and while sitting. He denies any recent trauma or sexual activity.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary Hemochromatosis with arthropathy (diagnosed 2020-01-15)
- Restrictive cardiomyopathy (diagnosed 2023-01-15)
- Acute pancreatitis (diagnosed 2024-07-15)
- Diabetic ketoacidosis (diagnosed 2025-01-15)
- Pyogenic liver abscess (diagnosed 2026-01-15)
- Type 2 Diabetes Mellitus (2 years duration, secondary to hemochromatosis)
- Hereditary Hemochromatosis (HFE-related)
- Cirrhosis secondary to iron overload
- Obesity (BMI 34 kg/m²)
- Mixed hyperlipidemia
- Hemochromatosis arthropathy (iron deposition)

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
His diabetes is managed with metformin.

Home Medications:
- Furosemide 40 mg PO daily
- Spironolactone 100 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hemochromatosis with cirrhosis and diabetes (deceased age 58)
- Mother: Type 2 diabetes mellitus
- Brother: Hereditary hemochromatosis
- Paternal uncle: Liver disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former heavy alcohol use; abstinent for 3 years (history of alcohol use disorder, in sustained remission). Occupation: Construction laborer.

VITAL SIGNS:
On examination, the patient is febrile with a temperature of 38.5°C (101.3°F).

PHYSICAL EXAMINATION:
Inspection of the perianal area reveals erythema, edema, and a fluctuant mass near the anal verge. Digital rectal examination is extremely painful and the patient is unable to tolerate it.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 14,000 /µL
- Hemoglobin: 13.8 g/dL
- Fasting blood glucose: 180 mg/dL
- C-reactive protein: 24 mg/L
