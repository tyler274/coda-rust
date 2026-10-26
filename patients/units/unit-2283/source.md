# synthetic_hospital patient 2283

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 66 at first encounter, sex M, Non-Hispanic White, Medicare and Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-02-25 (Fri) | outpatient | Gastroenterology |
| 1 | 2020-04-15 | 2022-05-16 (Mon) | outpatient | Internal Medicine |
| 2 | 2020-07-15 | 2022-08-18 (Thu) | outpatient | Oncology |
| 3 | 2021-01-15 | 2023-02-13 (Mon) | outpatient | Endocrinology |
| 4 | 2022-01-15 | 2024-02-20 (Tue) | outpatient | Gastroenterology |
| 5 | 2023-01-15 | 2025-02-19 (Wed) | outpatient | Urology |
| 6 | 2023-07-15 | 2025-08-23 (Sat) | ed | Emergency Medicine |
| 7 | 2024-01-15 | 2026-02-19 (Thu) | icu | Intensive Care Unit |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare and Medicaid",
 "occupation": "Retired long-haul truck driver",
 "alcohol_use": "Former heavy alcohol use, approximately 6 standard drinks daily for 30 years, now abstinent for 18 months; history of alcohol use disorder",
 "family_history": [
  "Father: Alcohol use disorder and liver cirrhosis",
  "Mother: Type 2 diabetes mellitus",
  "Older brother: Coronary artery disease with myocardial infarction at age 62"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 40 pack-year history, quit approximately 1 year ago",
 "home_medications": [
  {
   "dose": "24 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "sliding scale subcutaneous with meals",
   "name": "Insulin lispro"
  },
  {
   "dose": "0.4 mg orally daily",
   "name": "Tamsulosin"
  },
  {
   "dose": "81 mg orally daily",
   "name": "Aspirin"
  },
  {
   "dose": "40 mg orally daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "36,000 lipase units orally with meals",
   "name": "Pancrelipase"
  },
  {
   "dose": "1 tablet orally daily",
   "name": "Multivitamin"
  },
  {
   "dose": "15 mL orally twice daily as needed for constipation",
   "name": "Lactulose"
  }
 ],
 "surgical_history": [
  "Distal pancreatectomy with splenectomy for pancreatic adenocarcinoma",
  "Percutaneous coronary intervention with drug-eluting stent placement",
  "Esophagoscopy with diverticulotomy for Zenker diverticulum"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus",
  "Benign prostatic hyperplasia",
  "Chronic pancreatitis secondary to alcohol use",
  "Alcoholic cirrhosis",
  "Coronary artery disease",
  "History of pancreatic adenocarcinoma, status post partial pancreatectomy",
  "Zenker diverticulum",
  "History of tobacco use disorder"
 ],
 "age_at_first_encounter": 66
}
```

Primary diagnoses: ["Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation", "Tobacco use disorder", "Post-pancreatectomy hypoglycemia due to glucagon deficiency", "Alcoholic cirrhosis of liver", "Euthyroid sick syndrome", "Zenker diverticulum", "Acute bacterial prostatitis", "Chronic bacterial prostatitis"]
Comorbidities: ["Cancer-related constitutional symptoms", "Alcohol use disorder", "Proximal biliary obstruction", "Jaundice", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2022-02-25, outpatient, Gastroenterology

Chief complaint: Progressive difficulty swallowing solids and liquids for the past 6 months with frequent coughing during meals and halitosis.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty swallowing solids and liquids for the past 6 months with frequent coughing during meals and halitosis.

HISTORY OF PRESENT ILLNESS:
He reports frequent coughing during meals, halitosis, and regurgitation of undigested food, especially at night. His wife notes that he occasionally wakes up choking and has recently lost 5 kg. The patient also complains of a sensation of fullness in his neck that gurgles when pressed.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus
- Benign prostatic hyperplasia
- Chronic pancreatitis secondary to alcohol use
- Alcoholic cirrhosis
- Coronary artery disease
- History of pancreatic adenocarcinoma, status post partial pancreatectomy
- Zenker diverticulum
- History of tobacco use disorder

PAST SURGICAL HISTORY:
- Distal pancreatectomy with splenectomy for pancreatic adenocarcinoma
- Percutaneous coronary intervention with drug-eluting stent placement
- Esophagoscopy with diverticulotomy for Zenker diverticulum

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale subcutaneous with meals
- Tamsulosin 0.4 mg orally daily
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally daily
- Pancrelipase 36,000 lipase units orally with meals
- Multivitamin 1 tablet orally daily
- Lactulose 15 mL orally twice daily as needed for constipation

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alcohol use disorder and liver cirrhosis
- Mother: Type 2 diabetes mellitus
- Older brother: Coronary artery disease with myocardial infarction at age 62

SOCIAL HISTORY:
He denies smoking or alcohol use.

PHYSICAL EXAMINATION:
Physical exam is unremarkable.

IMAGING:
A barium esophagram is performed and shown above. It reveals a posterior outpouching just above the upper esophageal sphincter.

## Encounter 1: source 2020-04-15, practice 2022-05-16, outpatient, Internal Medicine

Chief complaint: Progressive fatigue, reduced appetite, and unintentional 20-lb weight loss over 2 months with new jaundice.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue, reduced appetite, and unintentional 20-lb weight loss over 2 months with new jaundice.

HISTORY OF PRESENT ILLNESS:
The patient returns three months after Zenker diverticulum diagnosis with history of alcoholic cirrhosis and prior pancreatectomy. He presents with fatigue, reduced appetite, weight loss, and jaundice. He reports occasional nausea and vague upper abdominal discomfort unimproved with food or bowel movements. His wife notes his skin appears more “pale yellow” than usual over the past week. He denies recent fever, vomiting, or bowel habit changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Zenker diverticulum (diagnosed 2020-01-15)
- Type 2 diabetes mellitus
- Benign prostatic hyperplasia
- Chronic pancreatitis secondary to alcohol use
- Alcoholic cirrhosis
- Coronary artery disease
- History of pancreatic adenocarcinoma, status post partial pancreatectomy
- History of tobacco use disorder

PAST SURGICAL HISTORY:
- Distal pancreatectomy with splenectomy for pancreatic adenocarcinoma
- Percutaneous coronary intervention with drug-eluting stent placement
- Esophagoscopy with diverticulotomy for Zenker diverticulum

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale subcutaneous with meals
- Tamsulosin 0.4 mg orally daily
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally daily
- Pancrelipase 36,000 lipase units orally with meals
- Multivitamin 1 tablet orally daily
- Lactulose 15 mL orally twice daily as needed for constipation

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alcohol use disorder and liver cirrhosis
- Mother: Type 2 diabetes mellitus
- Older brother: Coronary artery disease with myocardial infarction at age 62

SOCIAL HISTORY:
He has smoked one pack of cigarettes daily for the past 40 years and drinks alcohol socially on weekends.

PHYSICAL EXAMINATION:
Physical examination reveals mild scleral icterus and epigastric fullness without tenderness or palpable masses.

LABORATORY DATA:
Laboratory studies show mild cholestatic liver enzyme elevations.

IMAGING:
Abdominal CT scan reveals a mass in the head of the pancreas causing mild biliary duct dilation.

PATHOLOGY:
Endoscopic ultrasound–guided biopsy of the mass demonstrates gland-forming malignant cells with mucin production consistent with pancreatic adenocarcinoma.

## Encounter 2: source 2020-07-15, practice 2022-08-18, outpatient, Oncology

Chief complaint: Persistent fatigue, 15-pound weight loss over 3 months, and recent new diagnosis of type 2 diabetes mellitus.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Emily Watson, MD
Department: Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue, 15-pound weight loss over 3 months, and recent new diagnosis of type 2 diabetes mellitus.

HISTORY OF PRESENT ILLNESS:
The patient is a 66-year-old male with a history of pancreatic adenocarcinoma status post partial pancreatectomy, chronic pancreatitis, and alcoholic cirrhosis who returns to the Oncology clinic for follow-up evaluation. He was previously seen for progressive dysphagia and jaundice with associated weight loss. He denies any changes in diet or exercise habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Zenker diverticulum (diagnosed 2020-01-15)
- Tobacco use disorder (diagnosed 2020-04-15)
- Type 2 diabetes mellitus
- Benign prostatic hyperplasia
- Chronic pancreatitis secondary to alcohol use
- Alcoholic cirrhosis
- Coronary artery disease
- History of pancreatic adenocarcinoma, status post partial pancreatectomy

PAST SURGICAL HISTORY:
- Distal pancreatectomy with splenectomy for pancreatic adenocarcinoma
- Percutaneous coronary intervention with drug-eluting stent placement
- Esophagoscopy with diverticulotomy for Zenker diverticulum

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale subcutaneous with meals
- Tamsulosin 0.4 mg orally daily
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally daily
- Pancrelipase 36,000 lipase units orally with meals
- Multivitamin 1 tablet orally daily
- Lactulose 15 mL orally twice daily as needed for constipation

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alcohol use disorder and liver cirrhosis
- Mother: Type 2 diabetes mellitus
- Older brother: Coronary artery disease with myocardial infarction at age 62

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit approximately 1 year ago. Alcohol: Former heavy alcohol use, approximately 6 standard drinks daily for 30 years, now abstinent for 18 months; history of alcohol use disorder. Occupation: Retired long-haul truck driver.

PHYSICAL EXAMINATION:
On physical examination, he appears jaundiced with mild epigastric tenderness on palpation.

LABORATORY DATA:
His fasting blood glucose is 176 mg/dL and HbA1c is 7.8%. Liver function tests reveal elevated bilirubin and alkaline phosphatase.

IMAGING:
Abdominal ultrasound shows a dilated common bile duct but no gallstones.

## Encounter 3: source 2021-01-15, practice 2023-02-13, outpatient, Endocrinology

Chief complaint: Episodic shakiness, anxiety, and palpitations occurring several times a week, relieved by eating.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Okafor, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Episodic shakiness, anxiety, and palpitations occurring several times a week, relieved by eating.

HISTORY OF PRESENT ILLNESS:
The patient is a 67-year-old male with a history of pancreatic adenocarcinoma status post partial pancreatectomy and type 2 diabetes mellitus who presents for evaluation of hypoglycemic symptoms. He reports episodic shakiness, anxiety, and palpitations that have occurred several times a week over the past month. These episodes tend to occur a few hours after meals and are relieved by eating. His wife has noticed that he becomes sweaty, irritable, and confused during the episodes, but he usually improves rapidly after consuming juice or a snack. He has since been receiving regular injections to control his blood sugar. His appetite between meals has increased, and he has unintentionally lost 5 lb (2.3 kg). He is adherent to his medications and attends all follow-up visits.

PAST MEDICAL HISTORY:
Active Problem List:
- Zenker diverticulum (diagnosed 2020-01-15)
- Tobacco use disorder (diagnosed 2020-04-15)
- Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation (diagnosed 2020-07-15)
- Type 2 diabetes mellitus
- Benign prostatic hyperplasia
- Chronic pancreatitis secondary to alcohol use
- Alcoholic cirrhosis
- Coronary artery disease
- History of pancreatic adenocarcinoma, status post partial pancreatectomy

PAST SURGICAL HISTORY:
Six weeks ago, he underwent a surgical procedure

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale subcutaneous with meals
- Tamsulosin 0.4 mg orally daily
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally daily
- Pancrelipase 36,000 lipase units orally with meals
- Multivitamin 1 tablet orally daily
- Lactulose 15 mL orally twice daily as needed for constipation

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alcohol use disorder and liver cirrhosis
- Mother: Type 2 diabetes mellitus
- Older brother: Coronary artery disease with myocardial infarction at age 62

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit approximately 1 year ago. Alcohol: Former heavy alcohol use, approximately 6 standard drinks daily for 30 years, now abstinent for 18 months; history of alcohol use disorder. Occupation: Retired long-haul truck driver.

REVIEW OF SYSTEMS:
He denies fever, chest pain, or recent illness.

PHYSICAL EXAMINATION:
Physical examination is unremarkable except for a healing midline abdominal scar. Neurologic examination reveals no focal deficits. Cardiopulmonary and abdominal exams are within normal limits.

## Encounter 4: source 2022-01-15, practice 2024-02-20, outpatient, Gastroenterology

Chief complaint: Fatigue and increased bruising.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue and increased bruising.

PAST MEDICAL HISTORY:
Active Problem List:
- Zenker diverticulum (diagnosed 2020-01-15)
- Tobacco use disorder (diagnosed 2020-04-15)
- Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation (diagnosed 2020-07-15)
- Post-pancreatectomy hypoglycemia due to glucagon deficiency (diagnosed 2021-01-15)
- Type 2 diabetes mellitus
- Benign prostatic hyperplasia
- Chronic pancreatitis secondary to alcohol use
- Alcoholic cirrhosis
- Coronary artery disease
- History of pancreatic adenocarcinoma, status post partial pancreatectomy

His medical history includes long-standing alcohol use, and he has not seen a physician in several years.

PAST SURGICAL HISTORY:
- Distal pancreatectomy with splenectomy for pancreatic adenocarcinoma
- Percutaneous coronary intervention with drug-eluting stent placement
- Esophagoscopy with diverticulotomy for Zenker diverticulum

MEDICATIONS:
His medications include a daily multivitamin but no anticoagulants.

Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale subcutaneous with meals
- Tamsulosin 0.4 mg orally daily
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally daily
- Pancrelipase 36,000 lipase units orally with meals
- Lactulose 15 mL orally twice daily as needed for constipation

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alcohol use disorder and liver cirrhosis
- Mother: Type 2 diabetes mellitus
- Older brother: Coronary artery disease with myocardial infarction at age 62

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit approximately 1 year ago. Alcohol: Former heavy alcohol use, approximately 6 standard drinks daily for 30 years, now abstinent for 18 months; history of alcohol use disorder. Occupation: Retired long-haul truck driver.

PHYSICAL EXAMINATION:
Physical examination reveals scleral icterus, a distended abdomen with shifting dullness, and spider angiomata on the chest. He also has multiple ecchymoses on his arms and legs. The spleen is palpably enlarged below the left costal margin. There is no jugular venous distension. Neurologic examination is normal, and mental status is intact.

PLAN:
Laboratory testing is ordered to evaluate the cause of his clinical presentation.

## Encounter 5: source 2023-01-15, practice 2025-02-19, outpatient, Urology

Chief complaint: Three-month history of dysuria, urinary frequency, and low back pain with similar recurrent episodes.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Robert Johnson, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of dysuria, urinary frequency, and low back pain with similar recurrent episodes.

HISTORY OF PRESENT ILLNESS:
The patient, a 69-year-old male with benign prostatic hyperplasia and complex medical history, returns with recurrent genitourinary complaints. He reports having similar symptoms several times over the past year, which temporarily improved with antibiotics. He denies fever or chills.

PAST MEDICAL HISTORY:
Active Problem List:
- Zenker diverticulum (diagnosed 2020-01-15)
- Tobacco use disorder (diagnosed 2020-04-15)
- Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation (diagnosed 2020-07-15)
- Post-pancreatectomy hypoglycemia due to glucagon deficiency (diagnosed 2021-01-15)
- Alcoholic cirrhosis of liver (diagnosed 2022-01-15)
- Type 2 diabetes mellitus
- Benign prostatic hyperplasia
- Chronic pancreatitis secondary to alcohol use
- Alcoholic cirrhosis
- Coronary artery disease
- History of pancreatic adenocarcinoma, status post partial pancreatectomy

His medical history is significant for benign prostatic hyperplasia and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Distal pancreatectomy with splenectomy for pancreatic adenocarcinoma
- Percutaneous coronary intervention with drug-eluting stent placement
- Esophagoscopy with diverticulotomy for Zenker diverticulum

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale subcutaneous with meals
- Tamsulosin 0.4 mg orally daily
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally daily
- Pancrelipase 36,000 lipase units orally with meals
- Multivitamin 1 tablet orally daily
- Lactulose 15 mL orally twice daily as needed for constipation

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alcohol use disorder and liver cirrhosis
- Mother: Type 2 diabetes mellitus
- Older brother: Coronary artery disease with myocardial infarction at age 62

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit approximately 1 year ago. Alcohol: Former heavy alcohol use, approximately 6 standard drinks daily for 30 years, now abstinent for 18 months; history of alcohol use disorder. Occupation: Retired long-haul truck driver.

VITAL SIGNS:
On physical examination, the patient is afebrile, and his vital signs are within normal limits.

PHYSICAL EXAMINATION:
Digital rectal examination reveals a tender, boggy prostate.

LABORATORY DATA:
Urinalysis shows pyuria and bacteriuria. Urine culture grows a Gram-negative rod.

## Encounter 6: source 2023-07-15, practice 2025-08-23, ed, Emergency Medicine

Chief complaint: Two-day history of fever, chills, and painful urination with perineal and lower back pain.

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Lisa Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-day history of fever, chills, and painful urination with perineal and lower back pain.

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old male with benign prostatic hyperplasia and prior chronic bacterial prostatitis who returns with recurrent lower urinary tract symptoms. He presents to the emergency department with a 2-day history of fever, chills, and painful urination. He also complains of perineal and lower back pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Zenker diverticulum (diagnosed 2020-01-15)
- Tobacco use disorder (diagnosed 2020-04-15)
- Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation (diagnosed 2020-07-15)
- Post-pancreatectomy hypoglycemia due to glucagon deficiency (diagnosed 2021-01-15)
- Alcoholic cirrhosis of liver (diagnosed 2022-01-15)
- Chronic bacterial prostatitis (diagnosed 2023-01-15)
- Type 2 diabetes mellitus
- Benign prostatic hyperplasia
- Chronic pancreatitis secondary to alcohol use
- Alcoholic cirrhosis
- Coronary artery disease
- History of pancreatic adenocarcinoma, status post partial pancreatectomy

His past medical history is significant for benign prostatic hyperplasia and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Distal pancreatectomy with splenectomy for pancreatic adenocarcinoma
- Percutaneous coronary intervention with drug-eluting stent placement
- Esophagoscopy with diverticulotomy for Zenker diverticulum

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale subcutaneous with meals
- Tamsulosin 0.4 mg orally daily
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally daily
- Pancrelipase 36,000 lipase units orally with meals
- Multivitamin 1 tablet orally daily
- Lactulose 15 mL orally twice daily as needed for constipation

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alcohol use disorder and liver cirrhosis
- Mother: Type 2 diabetes mellitus
- Older brother: Coronary artery disease with myocardial infarction at age 62

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit approximately 1 year ago. Alcohol: Former heavy alcohol use, approximately 6 standard drinks daily for 30 years, now abstinent for 18 months; history of alcohol use disorder. Occupation: Retired long-haul truck driver.

VITAL SIGNS:
On physical examination, he is febrile with a temperature of 38.7°C (101.7°F)

PHYSICAL EXAMINATION:
Digital rectal examination reveals a markedly tender and swollen prostate.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 14,000 /µL
- Urinalysis: Pyuria, bacteriuria
- Urine culture: Pending

## Encounter 7: source 2024-01-15, practice 2026-02-19, icu, Intensive Care Unit

Chief complaint: Found unresponsive at home requiring intubation for airway protection.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. David Kim, MD
Department: Intensive Care Unit
Visit Type: icu

CHIEF COMPLAINT:
Found unresponsive at home requiring intubation for airway protection.

HISTORY OF PRESENT ILLNESS:
This 70-year-old male with a history of Zenker diverticulum, pancreatic adenocarcinoma status post partial pancreatectomy, and alcoholic cirrhosis is admitted to the intensive care unit after being found unresponsive at home. On arrival, he is intubated for airway protection and started on intravenous fluids and broad-spectrum antibiotics for presumed aspiration pneumonia. Over the next 10 days, his infection improves, but he remains lethargic and dependent on mechanical ventilation. A nutritional consult reveals he is mildly underweight. Laboratory studies are sent to evaluate persistent encephalopathy.

PAST MEDICAL HISTORY:
Active Problem List:
- Zenker diverticulum (diagnosed 2020-01-15)
- Tobacco use disorder (diagnosed 2020-04-15)
- Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation (diagnosed 2020-07-15)
- Post-pancreatectomy hypoglycemia due to glucagon deficiency (diagnosed 2021-01-15)
- Alcoholic cirrhosis of liver (diagnosed 2022-01-15)
- Chronic bacterial prostatitis (diagnosed 2023-01-15)
- Type 2 diabetes mellitus
- Benign prostatic hyperplasia
- Chronic pancreatitis secondary to alcohol use
- Alcoholic cirrhosis
- Coronary artery disease
- History of pancreatic adenocarcinoma, status post partial pancreatectomy

He has no known history of thyroid or endocrine disorders.

PAST SURGICAL HISTORY:
He underwent coronary artery bypass grafting 3 years ago

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale subcutaneous with meals
- Tamsulosin 0.4 mg orally daily
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally daily
- Pancrelipase 36,000 lipase units orally with meals
- Multivitamin 1 tablet orally daily
- Lactulose 15 mL orally twice daily as needed for constipation

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alcohol use disorder and liver cirrhosis
- Mother: Type 2 diabetes mellitus
- Older brother: Coronary artery disease with myocardial infarction at age 62

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit approximately 1 year ago. Alcohol: Former heavy alcohol use, approximately 6 standard drinks daily for 30 years, now abstinent for 18 months; history of alcohol use disorder. Occupation: Retired long-haul truck driver.

VITAL SIGNS:
Heart rate is persistently in the 90s, and blood pressure is stable on low-dose pressors. He is not febrile.

PHYSICAL EXAMINATION:
The nurses report that he has had minimal spontaneous movement and that he opens his eyes only to deep sternal rub. On exam, he is unresponsive to verbal stimuli but withdraws from pain. Pupils are equal and reactive. Deep tendon reflexes are slow. There is no goiter, no exophthalmos, and no tremor.

LABORATORY DATA:
Low serum albumin level. Thyroid function testing reveals low total T3, normal T4, and a normal TSH.

ASSESSMENT AND PLAN:
There is no evidence of pituitary abnormality or prior thyroid disease in his chart.
