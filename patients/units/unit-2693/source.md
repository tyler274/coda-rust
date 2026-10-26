# synthetic_hospital patient 2693

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 52 at first encounter, sex F, Non-Hispanic Black, Commercial insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-02-24 (Thu) | outpatient | Dermatology |
| 1 | 2020-03-15 | 2022-04-25 (Mon) | outpatient | Orthopedics & Sports Medicine |
| 2 | 2020-07-15 | 2022-08-16 (Tue) | outpatient | Orthopedics & Sports Medicine |
| 3 | 2020-11-15 | 2022-12-26 (Mon) | outpatient | Gastroenterology |
| 4 | 2021-03-15 | 2023-04-26 (Wed) | outpatient | Obstetrics & Gynecology |
| 5 | 2022-03-15 | 2024-04-26 (Fri) | outpatient | Gastroenterology |
| 6 | 2023-03-15 | 2025-04-22 (Tue) | telehealth | Internal Medicine |
| 7 | 2024-01-15 | 2026-02-23 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial insurance",
 "occupation": "Data entry specialist",
 "alcohol_use": "No alcohol use",
 "family_history": [
  "Mother: Type 2 diabetes mellitus",
  "Father: Hypertension, Hyperlipidemia",
  "Maternal grandmother: Endometrial cancer"
 ],
 "race_ethnicity": "Non-Hispanic Black",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "Laparoscopic cholecystectomy (age 45)"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus",
  "Hypertension",
  "Hyperlipidemia",
  "Obesity"
 ],
 "age_at_first_encounter": 52
}
```

Primary diagnoses: ["Endometrial Hyperplasia", "Non-alcoholic steatohepatitis", "Hepatic adenoma", "Acanthosis nigricans", "Iatrogenic bowel perforation", "Pes anserinus pain syndrome", "Tarsal tunnel syndrome"]
Comorbidities: ["Metabolic syndrome", "Obesity", "Type 2 diabetes mellitus", "Hyperlipidemia", "Essential hypertension", "Mixed hyperlipidemia"]

## Encounter 0: source 2020-01-15, practice 2022-02-24, outpatient, Dermatology

Chief complaint: Progressive darkening and thickening of skin in her axillae and around the neck for 6 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Aisha Johnson, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive darkening and thickening of skin in her axillae and around the neck for 6 months

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with a 6-month history of progressive darkening and thickening of the skin in her axillae and around the neck. She reports no itching or pain associated with these areas.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus
- Hypertension
- Hyperlipidemia
- Obesity

Her past medical history is significant for type 2 diabetes mellitus, which is poorly controlled on metformin, and hypertension.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Atorvastatin 40 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
She denies any family history of cancer.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Data entry specialist.

VITAL SIGNS:
Her body mass index (BMI) is 35 kg/m^2.

PHYSICAL EXAMINATION:
On physical examination, symmetrical, hyperpigmented, velvety plaques are noted in the axilla and around the neck.

LABORATORY DATA:
Laboratory tests reveal the following:

- Fasting glucose: 180 mg/dL
- Hemoglobin A1c: 8.5%
- Insulin level: Elevated
- Lipid panel:
- Total cholesterol: 220 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 145 mg/dL
- Triglycerides: 250 mg/dL

## Encounter 1: source 2020-03-15, practice 2022-04-25, outpatient, Orthopedics & Sports Medicine

Chief complaint: Progressive pain in right knee worse with activity and improving with rest over 3 months

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Robert Kim, MD
Department: Orthopedics & Sports Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive pain in right knee worse with activity and improving with rest over 3 months

HISTORY OF PRESENT ILLNESS:
The patient with diabetes and prior acanthosis nigricans presents with knee pain. She reports pain worse with activity and improves with rest, denying recent trauma or injury.

PAST MEDICAL HISTORY:
Active Problem List:
- Acanthosis nigricans (diagnosed 2020-01-15)
- Type 2 diabetes mellitus
- Hypertension
- Hyperlipidemia
- Obesity

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Atorvastatin 40 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus
- Father: Hypertension, Hyperlipidemia
- Maternal grandmother: Endometrial cancer

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Data entry specialist.

VITAL SIGNS:
Her body mass index is 34 kg/m^2

PHYSICAL EXAMINATION:
On examination, there is tenderness to palpation over the medial aspect of the tibial condyle, without erythema or warmth. The range of motion of the knee is preserved, but she experiences pain with resisted knee flexion.

IMAGING:
An x-ray of the knee shows mild joint space narrowing without evidence of fracture.

## Encounter 2: source 2020-07-15, practice 2022-08-16, outpatient, Orthopedics & Sports Medicine

Chief complaint: Burning pain and tingling in left foot worse at night and after prolonged standing for 3 months

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Maria Santos, MD
Department: Orthopedics & Sports Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Burning pain and tingling in left foot worse at night and after prolonged standing for 3 months

HISTORY OF PRESENT ILLNESS:
The patient with type 2 diabetes returns to Orthopedics following knee evaluation. She notes that the symptoms are worse at night and after prolonged periods of standing. She denies any recent trauma or injury to the foot.

PAST MEDICAL HISTORY:
Active Problem List:
- Acanthosis nigricans (diagnosed 2020-01-15)
- Pes anserinus pain syndrome (diagnosed 2020-03-15)
- Type 2 diabetes mellitus
- Hypertension
- Hyperlipidemia
- Obesity

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Atorvastatin 40 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus
- Father: Hypertension, Hyperlipidemia
- Maternal grandmother: Endometrial cancer

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Data entry specialist.

PHYSICAL EXAMINATION:
On physical examination, there is no erythema or swelling, but she has a positive Tinel sign at the medial malleolus. Dorsiflexion of the foot exacerbates the pain.

LABORATORY DATA:
Her most recent laboratory values are as follows:

- Fasting glucose: 162 mg/dL
- HbA1c: 7.8%
- Total cholesterol: 210 mg/dL
- LDL cholesterol: 130 mg/dL
- HDL cholesterol: 40 mg/dL
- Triglycerides: 250 mg/dL

## Encounter 3: source 2020-11-15, practice 2022-12-26, outpatient, Gastroenterology

Chief complaint: Mild right-sided abdominal discomfort ongoing for several weeks with no nausea or weight loss

ENCOUNTER NOTE
Date: 2020-11-15
Provider: Dr. James Washington, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Mild right-sided abdominal discomfort ongoing for several weeks with no nausea or weight loss

HISTORY OF PRESENT ILLNESS:
This 52-year-old female with type 2 diabetes mellitus, hypertension, hyperlipidemia, and obesity presents with several weeks of mild right-sided abdominal discomfort following recent encounters for acanthosis nigricans, pes anserinus pain syndrome, and tarsal tunnel syndrome. She denies nausea, vomiting, weight loss, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Acanthosis nigricans (diagnosed 2020-01-15)
- Pes anserinus pain syndrome (diagnosed 2020-03-15)
- Tarsal tunnel syndrome (diagnosed 2020-07-15)
- Type 2 diabetes mellitus
- Hypertension
- Hyperlipidemia
- Obesity

She has a history of type 2 diabetes and hyperlipidemia, both of which are suboptimally controlled.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
She takes metformin and a statin but has not seen her endocrinologist in over a year.

Home Medications:
- Lisinopril 20 mg daily
- Atorvastatin 40 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus
- Father: Hypertension, Hyperlipidemia
- Maternal grandmother: Endometrial cancer

SOCIAL HISTORY:
She does not smoke or consume alcohol.
She reports being mostly sedentary and admits to irregular dietary habits.

VITAL SIGNS:
Her BMI is 35 kg/m².
Vitals: Temperature 98.4°F (36.9°C), Blood Pressure 142/90 mmHg, Pulse 82/min, Respiratory Rate 16/min.

PHYSICAL EXAMINATION:
Physical examination shows mild hepatomegaly without tenderness. No stigmata of chronic liver disease are noted.

LABORATORY DATA:
Lab results show:
AST                                              52 U/L
ALT                                              63 U/L
ALP                                              90 U/L
Total bilirubin                                  0.7 mg/dL
HbA1c                                            7.9%
LDL                                              158 mg/dL
Triglycerides                                    260 mg/dL
Hepatitis B surface antigen                      Negative
Hepatitis C antibody                             Negative

## Encounter 4: source 2021-03-15, practice 2023-04-26, outpatient, Obstetrics & Gynecology

Chief complaint: Irregular, heavy menstrual bleeding with unpredictable cycles and intermenstrual spotting for 6 months

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Priya Patel, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Irregular, heavy menstrual bleeding with unpredictable cycles and intermenstrual spotting for 6 months

HISTORY OF PRESENT ILLNESS:
This 53-year-old female with Type 2 diabetes mellitus, hypertension, hyperlipidemia, and obesity, previously evaluated for acanthosis nigricans and hepatic steatosis, presents to the Obstetrics & Gynecology clinic with a 6-month history of irregular, heavy menstrual bleeding. She reports that her menstrual cycles used to be regular, with moderate flow lasting 5 days, but they have become unpredictable, with episodes of spotting between periods. She has not had a menstrual period for the past 2 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Acanthosis nigricans (diagnosed 2020-01-15)
- Pes anserinus pain syndrome (diagnosed 2020-03-15)
- Tarsal tunnel syndrome (diagnosed 2020-07-15)
- Non-alcoholic steatohepatitis (diagnosed 2020-11-15)
- Type 2 diabetes mellitus
- Hypertension
- Hyperlipidemia
- Obesity

Her past medical history is significant for type 2 diabetes mellitus and hypertension. Her most recent Pap smear was normal.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Atorvastatin 40 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus
- Father: Hypertension, Hyperlipidemia
- Maternal grandmother: Endometrial cancer

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Data entry specialist.

VITAL SIGNS:
She has a BMI of 35 kg/m^2. her blood pressure is 142/88 mmHg

PHYSICAL EXAMINATION:
On physical examination, a bimanual pelvic exam reveals an enlarged, nontender uterus. No adnexal masses are palpable.

LABORATORY DATA:
Hemoglobin: 11.2 g/dL (Normal: 12-16 g/dL)
Fasting glucose: 162 mg/dL (Normal: 70-99 mg/dL)
Total cholesterol: 220 mg/dL (Normal: 50 mg/dL in women)
Triglycerides: 250 mg/dL (Normal: <150 mg/dL)

OTHER STUDIES:
An endometrial biopsy is performed.

## Encounter 5: source 2022-03-15, practice 2024-04-26, outpatient, Gastroenterology

Chief complaint: Fatigue and right upper quadrant discomfort persistent for 6 months

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. David Nguyen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue and right upper quadrant discomfort persistent for 6 months

HISTORY OF PRESENT ILLNESS:
The patient is a 54-year-old female with type 2 diabetes mellitus and obesity who returns to the Gastroenterology clinic following prior evaluation for right upper quadrant discomfort in 2020. She presents with a 6-month history of fatigue and right upper quadrant discomfort.

PAST MEDICAL HISTORY:
Active Problem List:
- Acanthosis nigricans (diagnosed 2020-01-15)
- Pes anserinus pain syndrome (diagnosed 2020-03-15)
- Tarsal tunnel syndrome (diagnosed 2020-07-15)
- Non-alcoholic steatohepatitis (diagnosed 2020-11-15)
- Endometrial Hyperplasia (diagnosed 2021-03-15)
- Type 2 diabetes mellitus
- Hypertension
- Hyperlipidemia
- Obesity

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
Her medications include metformin and lisinopril

Home Medications:
- Atorvastatin 40 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus
- Father: Hypertension, Hyperlipidemia
- Maternal grandmother: Endometrial cancer

SOCIAL HISTORY:
She denies any history of alcohol use

PHYSICAL EXAMINATION:
On physical examination, her BMI is 32 kg/m^2, and she has hepatomegaly without jaundice, ascites, or spider angiomata

LABORATORY DATA:
Laboratory studies reveal:

- Serum AST: 70 U/L (Normal: 10-40 U/L)
- Serum ALT: 85 U/L (Normal: 7-56 U/L)
- Serum ALP: 90 U/L (Normal: 40-130 U/L)
- Total bilirubin: 1.2 mg/dL (Normal: 0.3-1.2 mg/dL)
- Fasting glucose: 160 mg/dL
- HbA1c: 7.8%
- Serum albumin: 4.0 g/dL (Normal: 3.5-5.0 g/dL)
- Platelet count: 220,000/µL (Normal: 150,000-450,000/µL)

IMAGING:
Abdominal ultrasonography shows increased echogenicity of the liver consistent with hepatic steatosis. There is no evidence of gallstones or biliary ductal dilatation

## Encounter 6: source 2023-03-15, practice 2025-04-22, telehealth, Internal Medicine

Chief complaint: Requesting copy of medical records for transfer of care to new provider in another state

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. Emily Zhang, MD
Department: Internal Medicine
Visit Type: telehealth

CHIEF COMPLAINT:
Requesting copy of medical records for transfer of care to new provider in another state

HISTORY OF PRESENT ILLNESS:
This 55-year-old female with Type 2 diabetes mellitus, hypertension, hyperlipidemia, and obesity, previously evaluated for acanthosis nigricans, returns for care transfer. She reports moving to another state and wishes to transfer her care to a new provider, inquiring about the process and timeline for obtaining her medical records.

PAST MEDICAL HISTORY:
Active Problem List:
- Acanthosis nigricans (diagnosed 2020-01-15)
- Pes anserinus pain syndrome (diagnosed 2020-03-15)
- Tarsal tunnel syndrome (diagnosed 2020-07-15)
- Non-alcoholic steatohepatitis (diagnosed 2020-11-15)
- Endometrial Hyperplasia (diagnosed 2021-03-15)
- Hepatic adenoma (diagnosed 2022-03-15)
- Type 2 diabetes mellitus
- Hypertension
- Hyperlipidemia
- Obesity

She has a history of type 2 diabetes mellitus, hypertension, and hyperlipidemia.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
Her current medications include metformin, lisinopril, and atorvastatin.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus
- Father: Hypertension, Hyperlipidemia
- Maternal grandmother: Endometrial cancer

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Data entry specialist.

VITAL SIGNS:
On examination, her blood pressure is 132/86 mmHg, heart rate is 78 beats per minute, and BMI is 31 kg/m^2.

LABORATORY DATA:
Her most recent laboratory results are as follows:

- Hemoglobin A1c: 7.2%
- Fasting plasma glucose: 154 mg/dL
- Total cholesterol: 190 mg/dL
- LDL cholesterol: 100 mg/dL
- HDL cholesterol: 45 mg/dL
- Triglycerides: 250 mg/dL

## Encounter 7: source 2024-01-15, practice 2026-02-23, ed, Emergency Medicine

Chief complaint: Worsening abdominal pain, nausea, and vomiting for 2 days following recent procedure

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Michael O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening abdominal pain, nausea, and vomiting for 2 days following recent procedure

HISTORY OF PRESENT ILLNESS:
This 56-year-old female with type 2 diabetes, hypertension, and history of chronic abdominal discomfort presents to the emergency department with a 2-day history of worsening abdominal pain, nausea, and vomiting. She underwent a laparoscopic cholecystectomy 5 days ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Acanthosis nigricans (diagnosed 2020-01-15)
- Pes anserinus pain syndrome (diagnosed 2020-03-15)
- Tarsal tunnel syndrome (diagnosed 2020-07-15)
- Non-alcoholic steatohepatitis (diagnosed 2020-11-15)
- Endometrial Hyperplasia (diagnosed 2021-03-15)
- Hepatic adenoma (diagnosed 2022-03-15)
- Type 2 diabetes mellitus
- Hypertension
- Hyperlipidemia
- Obesity

Her past medical history is significant for obesity and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Atorvastatin 40 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus
- Father: Hypertension, Hyperlipidemia
- Maternal grandmother: Endometrial cancer

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Data entry specialist.

VITAL SIGNS:
On examination, her temperature is 38.3°C (101°F), heart rate is 110 beats per minute, and blood pressure is 135/85 mmHg.

PHYSICAL EXAMINATION:
Abdominal examination reveals diffuse tenderness with guarding, but no rebound tenderness. Bowel sounds are hypoactive.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 16,000/mm³ (Normal: 4,500-11,000/mm³)
- Hemoglobin: 13.2 g/dL (Normal: 12-16 g/dL)
- Platelet count: 250,000/mm³ (Normal: 150,000-450,000/mm³)
- Serum creatinine: 1.0 mg/dL (Normal: 0.6-1.2 mg/dL)
- Blood urea nitrogen: 18 mg/dL (Normal: 7-20 mg/dL)
- C-reactive protein: 25 mg/L (Normal: <10 mg/L)

IMAGING:
An abdominal CT scan with oral contrast is performed.
