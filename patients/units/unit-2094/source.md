# synthetic_hospital patient 2094

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 56 at first encounter, sex M, Non-Hispanic White, Medicare (disability).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-06-09 (Wed) | outpatient | Gastroenterology |
| 1 | 2020-07-15 | 2021-12-13 (Mon) | outpatient | Gastroenterology |
| 2 | 2021-01-15 | 2022-06-06 (Mon) | outpatient | Gastroenterology |
| 3 | 2021-07-15 | 2022-12-03 (Sat) | ed | Emergency Medicine |
| 4 | 2022-09-15 | 2024-02-15 (Thu) | ed | Nephrology |
| 5 | 2023-11-15 | 2025-04-18 (Fri) | icu | Pulmonary and Critical Care |
| 6 | 2023-12-15 | 2025-05-14 (Wed) | icu | Pulmonary and Critical Care |
| 7 | 2025-01-15 | 2026-06-04 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare (disability)",
 "occupation": "Self-employed marine mechanic",
 "alcohol_use": "Former heavy use, abstinent for 5 years",
 "family_history": [
  "Father with alcohol use disorder and cirrhosis",
  "Paternal uncle with hepatocellular carcinoma",
  "Mother with type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "2 mg orally twice daily",
   "name": "Tacrolimus"
  },
  {
   "dose": "750 mg orally twice daily",
   "name": "Mycophenolate mofetil"
  },
  {
   "dose": "5 mg orally daily",
   "name": "Prednisone"
  },
  {
   "dose": "500 mg orally as needed for dyspepsia",
   "name": "Calcium carbonate (Tums)"
  },
  {
   "dose": "160 mg/800 mg orally daily (PJP prophylaxis)",
   "name": "Trimethoprim-sulfamethoxazole"
  }
 ],
 "surgical_history": [
  "Orthotopic liver transplantation",
  "Extensive surgical debridement of necrotizing fasciitis (left lower extremity)",
  "Exploratory laparotomy with Graham patch repair of perforated duodenal ulcer",
  "Exploratory laparotomy with partial hepatectomy for hepatocellular carcinoma (prior to transplant)"
 ],
 "chronic_conditions": [
  "Chronic hepatitis C infection (genotype 1a, acquired via blood transfusion in 1980s)",
  "Decompensated cirrhosis (hepatic cirrhosis)",
  "History of hepatocellular carcinoma",
  "Status post orthotopic liver transplantation (recent)",
  "Gastroesophageal reflux disease with history of peptic ulcer disease"
 ],
 "age_at_first_encounter": 56
}
```

Primary diagnoses: ["Necrotizing fasciitis", "Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate)", "Calcineurin inhibitor nephrotoxicity", "Perforated peptic ulcer disease", "Refractory hypoxemia in ARDS", "Central diabetes insipidus (inadequately controlled)"]
Comorbidities: ["Opportunistic pulmonary infection requiring diagnostic sampling", "Chronic hepatitis C", "Hepatic cirrhosis with portal hypertension", "Thrombocytopenia", "Septic shock"]

## Encounter 0: source 2020-01-15, practice 2021-06-09, outpatient, Gastroenterology

Chief complaint: Progressive fatigue, decreased appetite, and unintentional weight loss over the past 3 months with yellowing of eyes

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue, decreased appetite, and unintentional weight loss over the past 3 months with yellowing of eyes

HISTORY OF PRESENT ILLNESS:
He describes vague abdominal discomfort, intermittent nausea, and early satiety. He has noticed yellowing of his eyes and darkening of his urine, but denies fever, hematemesis, or recent travel.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic hepatitis C infection (genotype 1a, acquired via blood transfusion in 1980s)
- Decompensated cirrhosis (hepatic cirrhosis)
- History of hepatocellular carcinoma
- Status post orthotopic liver transplantation (recent)
- Gastroesophageal reflux disease with history of peptic ulcer disease

His past history is notable for blood transfusions in the 1980s. He has never been vaccinated against viral hepatitis.

PAST SURGICAL HISTORY:
- Orthotopic liver transplantation
- Extensive surgical debridement of necrotizing fasciitis (left lower extremity)
- Exploratory laparotomy with Graham patch repair of perforated duodenal ulcer
- Exploratory laparotomy with partial hepatectomy for hepatocellular carcinoma (prior to transplant)

MEDICATIONS:
Home Medications:
- Tacrolimus 2 mg orally twice daily
- Mycophenolate mofetil 750 mg orally twice daily
- Prednisone 5 mg orally daily
- Calcium carbonate (Tums) 500 mg orally as needed for dyspepsia
- Trimethoprim-sulfamethoxazole 160 mg/800 mg orally daily (PJP prophylaxis)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with alcohol use disorder and cirrhosis
- Paternal uncle with hepatocellular carcinoma
- Mother with type 2 diabetes mellitus

SOCIAL HISTORY:
Long-standing alcohol intake, though he quit drinking 5 years ago.

PHYSICAL EXAMINATION:
On examination, he appears chronically ill with scleral icterus and mild muscle wasting. The abdomen is distended with a palpable, nodular liver edge below the right costal margin and a fluid wave.

LABORATORY DATA:
Laboratory studies reveal mildly elevated liver transaminases, increased alkaline phosphatase, and prolonged prothrombin time. Serum alpha-fetoprotein is elevated.

IMAGING:
Ultrasound of the abdomen shows a large irregular mass within the right hepatic lobe with surrounding cirrhotic changes.

## Encounter 1: source 2020-07-15, practice 2021-12-13, outpatient, Gastroenterology

Chief complaint: 2-month history of progressive abdominal distension and discomfort

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
2-month history of progressive abdominal distension and discomfort

HISTORY OF PRESENT ILLNESS:
The patient is a 56-year-old male with chronic hepatitis C and decompensated cirrhosis who returns with progressive abdominal distension and discomfort of two months duration, following prior presentation in January with fatigue, decreased appetite, and yellowing of eyes. He denies any recent alcohol use, fever, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate) (diagnosed 2020-01-15)
- Chronic hepatitis C infection (genotype 1a, acquired via blood transfusion in 1980s)
- Decompensated cirrhosis (hepatic cirrhosis)
- History of hepatocellular carcinoma
- Status post orthotopic liver transplantation (recent)
- Gastroesophageal reflux disease with history of peptic ulcer disease

PAST SURGICAL HISTORY:
- Orthotopic liver transplantation
- Extensive surgical debridement of necrotizing fasciitis (left lower extremity)
- Exploratory laparotomy with Graham patch repair of perforated duodenal ulcer
- Exploratory laparotomy with partial hepatectomy for hepatocellular carcinoma (prior to transplant)

MEDICATIONS:
Home Medications:
- Tacrolimus 2 mg orally twice daily
- Mycophenolate mofetil 750 mg orally twice daily
- Prednisone 5 mg orally daily
- Calcium carbonate (Tums) 500 mg orally as needed for dyspepsia
- Trimethoprim-sulfamethoxazole 160 mg/800 mg orally daily (PJP prophylaxis)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with alcohol use disorder and cirrhosis
- Paternal uncle with hepatocellular carcinoma
- Mother with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former heavy use, abstinent for 5 years. Occupation: Self-employed marine mechanic.

PHYSICAL EXAMINATION:
On physical examination, he has a distended abdomen with a fluid wave and shifting dullness. There is no peripheral edema or jaundice.

LABORATORY DATA:
A diagnostic paracentesis is performed, and the fluid analysis reveals a bloody appearance with a hematocrit of 10%. Serum-ascites albumin gradient (SAAG) is 1.1 g/dL, and total protein in the ascitic fluid is 3.2 g/dL.
His laboratory values are as follows:

- Hemoglobin: 12.5 g/dL
- Platelet count: 110,000/uL
- INR: 1.2
- Alpha-fetoprotein (AFP): 400 ng/mL (normal < 10 ng/mL)

IMAGING:
His abdominal ultrasound shows a nodular liver contour and a 4 cm mass in the right hepatic lobe without evidence of portal vein thrombosis.

PATHOLOGY:
Cytologic analysis of the ascitic fluid is pending.

## Encounter 2: source 2021-01-15, practice 2022-06-06, outpatient, Gastroenterology

Chief complaint: Routine follow-up with new-onset abdominal swelling and discomfort over the past two months

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Jennifer Park, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up with new-onset abdominal swelling and discomfort over the past two months

HISTORY OF PRESENT ILLNESS:
The patient returns for post-transplant follow-up after 2020 presentations with progressive symptoms. He reports new-onset abdominal swelling and discomfort over the past two months. He denies any recent alcohol consumption, fever, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate) (diagnosed 2020-01-15)
- Chronic hepatitis C infection (genotype 1a, acquired via blood transfusion in 1980s)
- Decompensated cirrhosis (hepatic cirrhosis)
- History of hepatocellular carcinoma
- Status post orthotopic liver transplantation (recent)
- Gastroesophageal reflux disease with history of peptic ulcer disease

PAST SURGICAL HISTORY:
- Orthotopic liver transplantation
- Extensive surgical debridement of necrotizing fasciitis (left lower extremity)
- Exploratory laparotomy with Graham patch repair of perforated duodenal ulcer
- Exploratory laparotomy with partial hepatectomy for hepatocellular carcinoma (prior to transplant)

MEDICATIONS:
Home Medications:
- Tacrolimus 2 mg orally twice daily
- Mycophenolate mofetil 750 mg orally twice daily
- Prednisone 5 mg orally daily
- Calcium carbonate (Tums) 500 mg orally as needed for dyspepsia
- Trimethoprim-sulfamethoxazole 160 mg/800 mg orally daily (PJP prophylaxis)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with alcohol use disorder and cirrhosis
- Paternal uncle with hepatocellular carcinoma
- Mother with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former heavy use, abstinent for 5 years. Occupation: Self-employed marine mechanic.

PHYSICAL EXAMINATION:
On physical examination, his abdomen is distended with a fluid wave consistent with ascites. His skin examination reveals spider angiomas on the upper torso.

LABORATORY DATA:
Laboratory tests show:

- Platelet count: 110,000/µL
- INR: 1.3
- Albumin: 3.2 g/dL
- Total bilirubin: 2.1 mg/dL
- AST: 80 U/L
- ALT: 75 U/L
- Alkaline phosphatase: 120 U/L
- AFP (alpha-fetoprotein): 150 ng/mL

IMAGING:
An abdominal ultrasound six months ago was unremarkable.

## Encounter 3: source 2021-07-15, practice 2022-12-03, ed, Emergency Medicine

Chief complaint: Sudden-onset severe abdominal pain that began 2 hours ago

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset severe abdominal pain that began 2 hours ago

HISTORY OF PRESENT ILLNESS:
The patient, status post recent orthotopic liver transplantation, presents with sudden-onset severe abdominal pain. He describes the pain as constant and diffuse, with no relieving or exacerbating factors. Over the past week, he has noted worsening epigastric burning after meals but did not seek medical attention.

PAST MEDICAL HISTORY:
Active Problem List:
- Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate) (diagnosed 2020-01-15)
- Chronic hepatitis C infection (genotype 1a, acquired via blood transfusion in 1980s)
- Decompensated cirrhosis (hepatic cirrhosis)
- History of hepatocellular carcinoma
- Status post orthotopic liver transplantation (recent)
- Gastroesophageal reflux disease with history of peptic ulcer disease

He has a history of long-standing dyspepsia.

PAST SURGICAL HISTORY:
- Orthotopic liver transplantation
- Extensive surgical debridement of necrotizing fasciitis (left lower extremity)
- Exploratory laparotomy with Graham patch repair of perforated duodenal ulcer
- Exploratory laparotomy with partial hepatectomy for hepatocellular carcinoma (prior to transplant)

MEDICATIONS:
He takes intermittently takes over-the-counter antacids.

Home Medications:
- Tacrolimus 2 mg orally twice daily
- Mycophenolate mofetil 750 mg orally twice daily
- Prednisone 5 mg orally daily
- Calcium carbonate (Tums) 500 mg orally as needed for dyspepsia
- Trimethoprim-sulfamethoxazole 160 mg/800 mg orally daily (PJP prophylaxis)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with alcohol use disorder and cirrhosis
- Paternal uncle with hepatocellular carcinoma
- Mother with type 2 diabetes mellitus

SOCIAL HISTORY:
He does not smoke or drink alcohol.

VITAL SIGNS:
Temperature is 38.2°C (100.8°F), blood pressure is 96/60 mm Hg, and pulse is 112/min.

PHYSICAL EXAMINATION:
On examination, the patient is lying still and appears diaphoretic. Abdominal examination reveals distention, generalized tenderness with guarding, and marked rebound tenderness; bowel sounds are absent.

IMAGING:
Upright chest radiograph shows free air under the diaphragm.

## Encounter 4: source 2022-09-15, practice 2024-02-15, ed, Nephrology

Chief complaint: 2-day history of malaise, headache, and decreased urine output

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. Aisha Patel, MD
Department: Nephrology
Visit Type: ed

CHIEF COMPLAINT:
2-day history of malaise, headache, and decreased urine output

HISTORY OF PRESENT ILLNESS:
This 58-year-old male with chronic hepatitis C, decompensated cirrhosis, and status post recent orthotopic liver transplantation following prior perforated peptic ulcer disease presents for evaluation. He reports compliance with his immunosuppressive regimen, which includes tacrolimus.

PAST MEDICAL HISTORY:
Active Problem List:
- Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate) (diagnosed 2020-01-15)
- Perforated peptic ulcer disease (diagnosed 2021-07-15)
- Chronic hepatitis C infection (genotype 1a, acquired via blood transfusion in 1980s)
- Decompensated cirrhosis (hepatic cirrhosis)
- History of hepatocellular carcinoma
- Status post orthotopic liver transplantation (recent)
- Gastroesophageal reflux disease with history of peptic ulcer disease

PAST SURGICAL HISTORY:
- Orthotopic liver transplantation
- Extensive surgical debridement of necrotizing fasciitis (left lower extremity)
- Exploratory laparotomy with Graham patch repair of perforated duodenal ulcer
- Exploratory laparotomy with partial hepatectomy for hepatocellular carcinoma (prior to transplant)

MEDICATIONS:
He takes immunosuppressive regimen, which includes tacrolimus.

Home Medications:
- Mycophenolate mofetil 750 mg orally twice daily
- Prednisone 5 mg orally daily
- Calcium carbonate (Tums) 500 mg orally as needed for dyspepsia
- Trimethoprim-sulfamethoxazole 160 mg/800 mg orally daily (PJP prophylaxis)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with alcohol use disorder and cirrhosis
- Paternal uncle with hepatocellular carcinoma
- Mother with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former heavy use, abstinent for 5 years. Occupation: Self-employed marine mechanic.

VITAL SIGNS:
Blood pressure is 162/98 mmHg

PHYSICAL EXAMINATION:
Physical examination reveals mild bilateral lower extremity edema

LABORATORY DATA:
Serum creatinine: 2.8 mg/dL (baseline: 1.0 mg/dL)
Blood urea nitrogen (BUN): 44 mg/dL
Serum potassium: 5.2 mEq/L
Serum tacrolimus level: 25 ng/mL (therapeutic range: 5-15 ng/mL)
Urinalysis: Specific gravity 1.030, no protein, no cells

## Encounter 5: source 2023-11-15, practice 2025-04-18, icu, Pulmonary and Critical Care

Chief complaint: Admitted to intensive care unit for management of septic shock secondary to pneumonia with high urine output

ENCOUNTER NOTE
Date: 2023-11-15
Provider: Dr. Robert Kim, MD
Department: Pulmonary and Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Admitted to intensive care unit for management of septic shock secondary to pneumonia with high urine output

HISTORY OF PRESENT ILLNESS:
The patient is a 59-year-old male with chronic hepatitis C cirrhosis complicated by hepatocellular carcinoma and status post recent orthotopic liver transplantation who presents to the intensive care unit. He has a prior history of calcineurin inhibitor nephrotoxicity and perforated peptic ulcer disease. He initially required high-volume intravenous fluids and broad-spectrum antibiotics. Over the next two days, his blood pressure remained low despite aggressive fluid resuscitation, and he became progressively lethargic and confused. Vasopressor therapy with norepinephrine was started but his urine output remained elevated at over 4 liters per day despite worsening hypotension. Therefore, vasopressin infusion was added to his regimen. Twelve hours later, nursing staff note that his urine output has significantly decreased, and he appears less thirsty. He is receiving enteral nutrition and electrolyte repletion as needed.

PAST MEDICAL HISTORY:
Active Problem List:
- Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate) (diagnosed 2020-01-15)
- Perforated peptic ulcer disease (diagnosed 2021-07-15)
- Calcineurin inhibitor nephrotoxicity (diagnosed 2022-09-15)
- Chronic hepatitis C infection (genotype 1a, acquired via blood transfusion in 1980s)
- Decompensated cirrhosis (hepatic cirrhosis)
- History of hepatocellular carcinoma
- Status post orthotopic liver transplantation (recent)
- Gastroesophageal reflux disease with history of peptic ulcer disease

PAST SURGICAL HISTORY:
- Orthotopic liver transplantation
- Extensive surgical debridement of necrotizing fasciitis (left lower extremity)
- Exploratory laparotomy with Graham patch repair of perforated duodenal ulcer
- Exploratory laparotomy with partial hepatectomy for hepatocellular carcinoma (prior to transplant)

MEDICATIONS:
Home Medications:
- Tacrolimus 2 mg orally twice daily
- Mycophenolate mofetil 750 mg orally twice daily
- Prednisone 5 mg orally daily
- Calcium carbonate (Tums) 500 mg orally as needed for dyspepsia
- Trimethoprim-sulfamethoxazole 160 mg/800 mg orally daily (PJP prophylaxis)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with alcohol use disorder and cirrhosis
- Paternal uncle with hepatocellular carcinoma
- Mother with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former heavy use, abstinent for 5 years. Occupation: Self-employed marine mechanic.

REVIEW OF SYSTEMS:
Review of systems is otherwise non-contributory,

PHYSICAL EXAMINATION:
There are no signs of peripheral edema or jugular venous distension on examination. Cardiopulmonary auscultation is unremarkable. Neurological examination shows no focal deficits.

## Encounter 6: source 2023-12-15, practice 2025-05-14, icu, Pulmonary and Critical Care

Chief complaint: Oxygenation has worsened despite treatment for severe septic shock

ENCOUNTER NOTE
Date: 2023-12-15
Provider: Dr. Lisa Thompson, MD
Department: Pulmonary and Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Oxygenation has worsened despite treatment for severe septic shock

HISTORY OF PRESENT ILLNESS:
The patient is a 56-year-old male with chronic hepatitis C, decompensated cirrhosis, hepatocellular carcinoma, and orthotopic liver transplantation who returns to the intensive care unit one month after a prior admission for septic shock. He presents after being treated for severe septic shock secondary to pneumonia. Despite aggressive fluid resuscitation and broad-spectrum antibiotics, he required vasopressor support and mechanical ventilation. Over the past 24 hours, his oxygenation has worsened, and he now needs higher levels of inspired oxygen.

PAST MEDICAL HISTORY:
Active Problem List:
- Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate) (diagnosed 2020-01-15)
- Perforated peptic ulcer disease (diagnosed 2021-07-15)
- Calcineurin inhibitor nephrotoxicity (diagnosed 2022-09-15)
- Central diabetes insipidus (inadequately controlled) (diagnosed 2023-11-15)
- Chronic hepatitis C infection (genotype 1a, acquired via blood transfusion in 1980s)
- Decompensated cirrhosis (hepatic cirrhosis)
- History of hepatocellular carcinoma
- Status post orthotopic liver transplantation (recent)
- Gastroesophageal reflux disease with history of peptic ulcer disease

PAST SURGICAL HISTORY:
- Orthotopic liver transplantation
- Extensive surgical debridement of necrotizing fasciitis (left lower extremity)
- Exploratory laparotomy with Graham patch repair of perforated duodenal ulcer
- Exploratory laparotomy with partial hepatectomy for hepatocellular carcinoma (prior to transplant)

MEDICATIONS:
Home Medications:
- Tacrolimus 2 mg orally twice daily
- Mycophenolate mofetil 750 mg orally twice daily
- Prednisone 5 mg orally daily
- Calcium carbonate (Tums) 500 mg orally as needed for dyspepsia
- Trimethoprim-sulfamethoxazole 160 mg/800 mg orally daily (PJP prophylaxis)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with alcohol use disorder and cirrhosis
- Paternal uncle with hepatocellular carcinoma
- Mother with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former heavy use, abstinent for 5 years. Occupation: Self-employed marine mechanic.

VITAL SIGNS:
The patient is febrile and tachypneic

PHYSICAL EXAMINATION:
Auscultation reveals diffuse crackles throughout both lung fields

LABORATORY DATA:
Arterial blood gas analysis shows hypoxemia that does not correct with supplemental oxygen

IMAGING:
Chest radiograph demonstrates bilateral, patchy opacities without evidence of cardiomegaly

OTHER STUDIES:
Echocardiogram shows preserved left ventricular systolic function and no significant valvular abnormalities

## Encounter 7: source 2025-01-15, practice 2026-06-04, ed, Emergency Medicine

Chief complaint: 24-hour history of severe pain and swelling in left lower leg after minor laceration

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. David Martinez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
24-hour history of severe pain and swelling in left lower leg after minor laceration

HISTORY OF PRESENT ILLNESS:
The patient is a 61-year-old liver transplant recipient presenting with severe left lower leg pain and swelling. He reports that the symptoms began a few hours after he sustained a minor laceration while cleaning his boat.

PAST MEDICAL HISTORY:
Active Problem List:
- Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate) (diagnosed 2020-01-15)
- Perforated peptic ulcer disease (diagnosed 2021-07-15)
- Calcineurin inhibitor nephrotoxicity (diagnosed 2022-09-15)
- Central diabetes insipidus (inadequately controlled) (diagnosed 2023-11-15)
- Refractory hypoxemia in ARDS (diagnosed 2023-12-15)
- Chronic hepatitis C infection (genotype 1a, acquired via blood transfusion in 1980s)
- Decompensated cirrhosis (hepatic cirrhosis)
- History of hepatocellular carcinoma
- Status post orthotopic liver transplantation (recent)
- Gastroesophageal reflux disease with history of peptic ulcer disease

His medical history is significant for chronic hepatitis C and alcohol abuse.

PAST SURGICAL HISTORY:
- Orthotopic liver transplantation
- Extensive surgical debridement of necrotizing fasciitis (left lower extremity)
- Exploratory laparotomy with Graham patch repair of perforated duodenal ulcer
- Exploratory laparotomy with partial hepatectomy for hepatocellular carcinoma (prior to transplant)

MEDICATIONS:
Home Medications:
- Tacrolimus 2 mg orally twice daily
- Mycophenolate mofetil 750 mg orally twice daily
- Prednisone 5 mg orally daily
- Calcium carbonate (Tums) 500 mg orally as needed for dyspepsia
- Trimethoprim-sulfamethoxazole 160 mg/800 mg orally daily (PJP prophylaxis)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with alcohol use disorder and cirrhosis
- Paternal uncle with hepatocellular carcinoma
- Mother with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former heavy use, abstinent for 5 years. Occupation: Self-employed marine mechanic.

VITAL SIGNS:
The patient is febrile with a temperature of 38.9°C (102°F), tachycardic with a heart rate of 112 beats per minute, and hypotensive with a blood pressure of 90/60 mmHg.

PHYSICAL EXAMINATION:
On examination, the left lower leg is erythematous, edematous, and tender to palpation, with several hemorrhagic bullae.

LABORATORY DATA:
Laboratory values are as follows:

- White blood cell count: 18,000/mm^3
- Hemoglobin: 13.2 g/dL
- Platelet count: 230,000/mm^3
- Sodium: 135 mEq/L
- Potassium: 4.5 mEq/L
- Creatinine: 1.2 mg/dL
- Aspartate aminotransferase (AST): 80 U/L
- Alanine aminotransferase (ALT): 72 U/L
- Total bilirubin: 2.1 mg/dL
- Prothrombin time (PT): 16 seconds (INR 1.4)
