# synthetic_hospital patient 2198

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 66 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-10-27 (Wed) | outpatient | Neurology |
| 1 | 2020-04-15 | 2022-01-31 (Mon) | outpatient | Gastroenterology |
| 2 | 2020-07-15 | 2022-04-28 (Thu) | ed | Emergency Medicine |
| 3 | 2021-01-15 | 2022-10-24 (Mon) | procedure | General Surgery |
| 4 | 2021-01-15 | 2022-10-27 (Thu) | inpatient | General Surgery |
| 5 | 2021-07-15 | 2023-04-23 (Sun) | ed | General Surgery |
| 6 | 2021-07-15 | 2023-04-24 (Mon) | icu | Intensive Care |
| 7 | 2024-01-15 | 2025-10-27 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare",
 "occupation": "Retired postal clerk",
 "alcohol_use": "Rare (1-2 drinks per month)",
 "family_history": [
  "Father: Type 2 diabetes mellitus, hypertension",
  "Mother: Hypertension, dementia (late-onset)",
  "Paternal uncle: Pancreatic adenocarcinoma",
  "Older brother: Intracerebral hemorrhage at age 70"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "1000 mcg PO daily",
   "name": "Cyanocobalamin"
  }
 ],
 "surgical_history": [
  "Colonoscopy with documented diverticulosis (age 61)",
  "Elective open cholecystectomy (initially attempted laparoscopically) with incidental identification of umbilical hernia, complicated by postoperative intra-abdominal sepsis (age 66)",
  "Emergency exploratory laparotomy with sigmoid colectomy for perforated diverticulitis with peritonitis, complicated by prolonged intubation and euthyroid sick syndrome (age 66)"
 ],
 "chronic_conditions": [
  "Hypertension (poorly controlled)",
  "Type 2 Diabetes Mellitus (diagnosed 18 months ago, 10-year clinical history of poor glycemic control)",
  "Hyperlipidemia",
  "Diverticulosis (documented on colonoscopy age 61)",
  "Vitamin B12 deficiency",
  "Umbilical hernia",
  "Pancreatic adenocarcinoma"
 ],
 "age_at_first_encounter": 66
}
```

Primary diagnoses: ["Euthyroid sick syndrome", "Vitamin B12 deficiency", "Umbilical hernia, incidental finding", "Atrial flutter", "Perforated diverticulitis with peritonitis", "Pancreatic carcinoma", "Intracerebral hemorrhage with airway compromise", "Intra-abdominal sepsis/Secondary peritonitis"]
Comorbidities: ["Obesity", "Hyperlipidemia", "Essential hypertension", "Type 2 diabetes mellitus", "Status post laparoscopic cholecystectomy"]

## Encounter 0: source 2020-01-15, practice 2021-10-27, outpatient, Neurology

Chief complaint: Daughter reports patient has been increasingly forgetful over the past year, recently got lost driving to a familiar location

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Daughter reports patient has been increasingly forgetful over the past year, recently got lost driving to a familiar location

HISTORY OF PRESENT ILLNESS:
She is concerned because he recently got lost while driving to the grocery store, a route he has known for years.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension (poorly controlled)
- Type 2 Diabetes Mellitus (diagnosed 18 months ago, 10-year clinical history of poor glycemic control)
- Hyperlipidemia
- Diverticulosis (documented on colonoscopy age 61)
- Vitamin B12 deficiency
- Umbilical hernia
- Pancreatic adenocarcinoma

The patient has a past medical history of hypertension, type 2 diabetes mellitus, and hypercholesterolemia.

PAST SURGICAL HISTORY:
- Colonoscopy with documented diverticulosis (age 61)
- Elective open cholecystectomy (initially attempted laparoscopically) with incidental identification of umbilical hernia, complicated by postoperative intra-abdominal sepsis (age 66)
- Emergency exploratory laparotomy with sigmoid colectomy for perforated diverticulitis with peritonitis, complicated by prolonged intubation and euthyroid sick syndrome (age 66)

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Amlodipine 5 mg PO daily
- Cyanocobalamin 1000 mcg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Hypertension, dementia (late-onset)
- Paternal uncle: Pancreatic adenocarcinoma
- Older brother: Intracerebral hemorrhage at age 70

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per month). Occupation: Retired postal clerk.

VITAL SIGNS:
His blood pressure is 138/82 mmHg, heart rate is 78 beats per minute, and BMI is 29 kg/m^2.

PHYSICAL EXAMINATION:
On physical examination, the patient is alert but appears apathetic.
Neurological examination reveals no focal deficits.
A Montreal Cognitive Assessment (MoCA) is performed, and the patient scores 18 out of 30, indicating cognitive impairment.

LABORATORY DATA:
Laboratory results are as follows:

- Complete Blood Count (CBC): Normal
- Complete Metabolic Panel (CMP): Normal
- Thyroid-Stimulating Hormone (TSH): 3.5 mIU/L (normal range: 0.4-4.0 mIU/L)
- Vitamin B12: 220 pg/mL (normal range: 200-900 pg/mL)

IMAGING:
A non-contrast MRI of the brain shows generalized atrophy with no acute findings.

## Encounter 1: source 2020-04-15, practice 2022-01-31, outpatient, Gastroenterology

Chief complaint: 3-month history of constant, dull epigastric pain radiating to the back with 15-pound unintentional weight loss

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of constant, dull epigastric pain radiating to the back with 15-pound unintentional weight loss

HISTORY OF PRESENT ILLNESS:
Patient with HTN, DM, recent B12 eval presents. He reports a 15-pound weight loss during this period despite no changes in diet or activity level.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitamin B12 deficiency (diagnosed 2020-01-15)
- Hypertension (poorly controlled)
- Type 2 Diabetes Mellitus (diagnosed 18 months ago, 10-year clinical history of poor glycemic control)
- Hyperlipidemia
- Diverticulosis (documented on colonoscopy age 61)
- Umbilical hernia
- Pancreatic adenocarcinoma

His medical history is significant for hypertension, hyperlipidemia, and a diagnosis of type 2 diabetes mellitus 18 months ago.

PAST SURGICAL HISTORY:
- Colonoscopy with documented diverticulosis (age 61)
- Elective open cholecystectomy (initially attempted laparoscopically) with incidental identification of umbilical hernia, complicated by postoperative intra-abdominal sepsis (age 66)
- Emergency exploratory laparotomy with sigmoid colectomy for perforated diverticulitis with peritonitis, complicated by prolonged intubation and euthyroid sick syndrome (age 66)

MEDICATIONS:
His current medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Amlodipine 5 mg PO daily
- Cyanocobalamin 1000 mcg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Hypertension, dementia (late-onset)
- Paternal uncle: Pancreatic adenocarcinoma
- Older brother: Intracerebral hemorrhage at age 70

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per month). Occupation: Retired postal clerk.

PHYSICAL EXAMINATION:
On physical examination, he appears jaundiced. His abdomen is soft, with mild tenderness in the epigastric region without rebound or guarding.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin: 12.1 g/dL (normal: 13.8-17.2 g/dL)
- Total bilirubin: 3.2 mg/dL (normal: 0.3-1.2 mg/dL)
- Alkaline phosphatase: 210 U/L (normal: 40-130 U/L)
- CA 19-9: 120 U/mL (normal: <37 U/mL)

## Encounter 2: source 2020-07-15, practice 2022-04-28, ed, Emergency Medicine

Chief complaint: Sudden onset of palpitations and mild dyspnea starting 2 hours ago

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Emily Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of palpitations and mild dyspnea starting 2 hours ago

HISTORY OF PRESENT ILLNESS:
The patient with pancreatic adenocarcinoma diagnosed in April 2020 and vitamin B12 deficiency identified in January 2020 presents with sudden onset palpitations and mild dyspnea that started 2 hours ago. He denies chest pain, syncope, or previous similar episodes.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitamin B12 deficiency (diagnosed 2020-01-15)
- Pancreatic carcinoma (diagnosed 2020-04-15)
- Hypertension (poorly controlled)
- Type 2 Diabetes Mellitus (diagnosed 18 months ago, 10-year clinical history of poor glycemic control)
- Hyperlipidemia
- Diverticulosis (documented on colonoscopy age 61)
- Umbilical hernia
- Pancreatic adenocarcinoma

PAST SURGICAL HISTORY:
- Colonoscopy with documented diverticulosis (age 61)
- Elective open cholecystectomy (initially attempted laparoscopically) with incidental identification of umbilical hernia, complicated by postoperative intra-abdominal sepsis (age 66)
- Emergency exploratory laparotomy with sigmoid colectomy for perforated diverticulitis with peritonitis, complicated by prolonged intubation and euthyroid sick syndrome (age 66)

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Amlodipine 5 mg PO daily
- Cyanocobalamin 1000 mcg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Hypertension, dementia (late-onset)
- Paternal uncle: Pancreatic adenocarcinoma
- Older brother: Intracerebral hemorrhage at age 70

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per month). Occupation: Retired postal clerk.

VITAL SIGNS:
On examination, his blood pressure is 140/85 mmHg, heart rate is 150 beats per minute, respiratory rate is 18 breaths per minute, and oxygen saturation is 97% on room air.

PHYSICAL EXAMINATION:
His heart rhythm is regular, and no murmurs are heard. Lung fields are clear to auscultation.

OTHER STUDIES:
An electrocardiogram (ECG) is performed and shows saw-toothed flutter waves with a ventricular rate of 150 beats per minute.

## Encounter 3: source 2021-01-15, practice 2022-10-24, procedure, General Surgery

Chief complaint: Symptomatic cholelithiasis requiring elective surgical management

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. David Park, MD
Department: General Surgery
Visit Type: procedure

CHIEF COMPLAINT:
Symptomatic cholelithiasis requiring elective surgical management

HISTORY OF PRESENT ILLNESS:
The patient returns for elective laparoscopic cholecystectomy with history significant for pancreatic adenocarcinoma and recent atrial flutter. During the procedure, the surgeon encounters unexpected bleeding from the cystic artery, which is promptly controlled.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitamin B12 deficiency (diagnosed 2020-01-15)
- Pancreatic carcinoma (diagnosed 2020-04-15)
- Atrial flutter (diagnosed 2020-07-15)
- Hypertension (poorly controlled)
- Type 2 Diabetes Mellitus (diagnosed 18 months ago, 10-year clinical history of poor glycemic control)
- Hyperlipidemia
- Diverticulosis (documented on colonoscopy age 61)
- Umbilical hernia
- Pancreatic adenocarcinoma

PAST SURGICAL HISTORY:
- Colonoscopy with documented diverticulosis (age 61)
- Elective open cholecystectomy (initially attempted laparoscopically) with incidental identification of umbilical hernia, complicated by postoperative intra-abdominal sepsis (age 66)
- Emergency exploratory laparotomy with sigmoid colectomy for perforated diverticulitis with peritonitis, complicated by prolonged intubation and euthyroid sick syndrome (age 66)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Amlodipine 5 mg PO daily
- Cyanocobalamin 1000 mcg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Hypertension, dementia (late-onset)
- Paternal uncle: Pancreatic adenocarcinoma
- Older brother: Intracerebral hemorrhage at age 70

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per month). Occupation: Retired postal clerk.

VITAL SIGNS:
BMI of 32

PHYSICAL EXAMINATION:
While the patient is still under anesthesia, the surgeon notices a small umbilical hernia that was not previously diagnosed or discussed with the patient.

LABORATORY DATA:
His preoperative labs were as follows:

- Hemoglobin A1c: 8.5%
- Fasting blood glucose: 176 mg/dL
- Total cholesterol: 220 mg/dL
- LDL cholesterol: 140 mg/dL
- HDL cholesterol: 35 mg/dL
- Triglycerides: 250 mg/dL

PLAN:
The surgeon considers repairing the hernia during the same operation.

## Encounter 4: source 2021-01-15, practice 2022-10-27, inpatient, General Surgery

Chief complaint: Fever of 38.9°C, chills, and right lower quadrant abdominal pain 3 days after cholecystectomy

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. David Park, MD
Department: General Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Fever of 38.9°C, chills, and right lower quadrant abdominal pain 3 days after cholecystectomy

HISTORY OF PRESENT ILLNESS:
The patient is a 67-year-old male with pancreatic adenocarcinoma who returns three days following elective open cholecystectomy. He presents with fever of 38.9°C, chills, and right lower quadrant abdominal pain that started 3 days after undergoing an elective open cholecystectomy.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitamin B12 deficiency (diagnosed 2020-01-15)
- Pancreatic carcinoma (diagnosed 2020-04-15)
- Atrial flutter (diagnosed 2020-07-15)
- Umbilical hernia, incidental finding (diagnosed 2021-01-15)
- Hypertension (poorly controlled)
- Type 2 Diabetes Mellitus (diagnosed 18 months ago, 10-year clinical history of poor glycemic control)
- Hyperlipidemia
- Diverticulosis (documented on colonoscopy age 61)
- Umbilical hernia
- Pancreatic adenocarcinoma

PAST SURGICAL HISTORY:
- Colonoscopy with documented diverticulosis (age 61)
- Elective open cholecystectomy (initially attempted laparoscopically) with incidental identification of umbilical hernia, complicated by postoperative intra-abdominal sepsis (age 66)
- Emergency exploratory laparotomy with sigmoid colectomy for perforated diverticulitis with peritonitis, complicated by prolonged intubation and euthyroid sick syndrome (age 66)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Amlodipine 5 mg PO daily
- Cyanocobalamin 1000 mcg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Hypertension, dementia (late-onset)
- Paternal uncle: Pancreatic adenocarcinoma
- Older brother: Intracerebral hemorrhage at age 70

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per month). Occupation: Retired postal clerk.

VITAL SIGNS:
Blood pressure is 135/85 mmHg, heart rate is 102 beats per minute, and respiratory rate is 18 breaths per minute

PHYSICAL EXAMINATION:
The surgical site appears clean and well-approximated without erythema or discharge. Bowel sounds are present and normal.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 12,000 /µL
- Hemoglobin: 13.2 g/dL
- Platelet count: 250,000 /µL
- Serum creatinine: 1.1 mg/dL
- Blood urea nitrogen: 21 mg/dL
- C-reactive protein: 18 mg/L (normal <10 mg/L)
- Urinalysis: Normal

IMAGING:
A CT scan of the abdomen and pelvis with contrast shows a 3 cm fluid collection in the right lower quadrant adjacent to the cecum. There is no evidence of bowel perforation or obstruction.

## Encounter 5: source 2021-07-15, practice 2023-04-23, ed, General Surgery

Chief complaint: 2 days of progressively worsening left lower quadrant abdominal pain with fevers and chills

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Aisha Patel, MD
Department: General Surgery
Visit Type: ed

CHIEF COMPLAINT:
2 days of progressively worsening left lower quadrant abdominal pain with fevers and chills

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with documented diverticulosis and pancreatic adenocarcinoma presents to the emergency department six months following cholecystectomy complicated by intra-abdominal sepsis. He describes the pain as constant, located in the left lower quadrant, and associated with low-grade fevers and chills. He reports decreased appetite and mild nausea but no vomiting. Over the past 48 hours, he has had less frequent bowel movements and feels "bloated." He denies hematochezia, melena, or recent travel. Two weeks ago, he began increasing his fiber intake after reading that it might be good for digestion.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitamin B12 deficiency (diagnosed 2020-01-15)
- Pancreatic carcinoma (diagnosed 2020-04-15)
- Atrial flutter (diagnosed 2020-07-15)
- Umbilical hernia, incidental finding (diagnosed 2021-01-15)
- Intra-abdominal sepsis/Secondary peritonitis (diagnosed 2021-01-15)
- Hypertension (poorly controlled)
- Type 2 Diabetes Mellitus (diagnosed 18 months ago, 10-year clinical history of poor glycemic control)
- Hyperlipidemia
- Diverticulosis (documented on colonoscopy age 61)
- Umbilical hernia
- Pancreatic adenocarcinoma

His medical history includes hypertension and hyperlipidemia, for which he takes lisinopril and atorvastatin. He has had diverticulosis documented on a screening colonoscopy 5 years ago but has never had any complications from it.

PAST SURGICAL HISTORY:
- Colonoscopy with documented diverticulosis (age 61)
- Elective open cholecystectomy (initially attempted laparoscopically) with incidental identification of umbilical hernia, complicated by postoperative intra-abdominal sepsis (age 66)
- Emergency exploratory laparotomy with sigmoid colectomy for perforated diverticulitis with peritonitis, complicated by prolonged intubation and euthyroid sick syndrome (age 66)

MEDICATIONS:
He takes lisinopril and atorvastatin.

Home Medications:
- Metformin 1000 mg PO twice daily
- Amlodipine 5 mg PO daily
- Cyanocobalamin 1000 mcg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Hypertension, dementia (late-onset)
- Paternal uncle: Pancreatic adenocarcinoma
- Older brother: Intracerebral hemorrhage at age 70

SOCIAL HISTORY:
The patient lives alone with two pet bearded dragons, which he enjoys feeding and caring for daily. He also spends much of his free time reading about and collecting stamps, particularly those from the Western Asian region, and is excited about an upcoming philatelic exhibition. He does not smoke or drink alcohol.

VITAL SIGNS:
Temperature is 38.3°C (100.9°F), blood pressure is 132/78 mm Hg, pulse is 94/min, and respirations are 16/min

PHYSICAL EXAMINATION:
The patient appears uncomfortable but is in no acute distress. Abdominal examination reveals tenderness to palpation in the left lower quadrant with mild guarding but no rebound tenderness. There is no hepatosplenomegaly, and bowel sounds are decreased. Rectal examination is unremarkable.

LABORATORY DATA:
Laboratory studies reveal leukocytosis with a left shift. Basic metabolic panel is normal. Urinalysis is unremarkable.

IMAGING:
A plain abdominal radiograph shows no free air under the diaphragm. A contrast-enhanced CT scan of the abdomen and pelvis reveals segmental thickening of the sigmoid colon with pericolic fat stranding and a small pericolic fluid collection consistent with a localized abscess.

PLAN:
The patient is started on bowel rest, intravenous fluids, and broad-spectrum antibiotics covering gram-negative rods and anaerobes. Surgical consultation is obtained, and the decision is made for nonoperative management at this time. The patient is reassured that with appropriate treatment, he can likely attend the upcoming philately exhibition.

## Encounter 6: source 2021-07-15, practice 2023-04-24, icu, Intensive Care

Chief complaint: Persistent fatigue and weakness 5 days post-emergency laparotomy with prolonged intubation

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James O'Brien, MD
Department: Intensive Care
Visit Type: icu

CHIEF COMPLAINT:
Persistent fatigue and weakness 5 days post-emergency laparotomy with prolonged intubation

HISTORY OF PRESENT ILLNESS:
The patient is a 67-year-old male with a history of diverticulosis documented at age 61, pancreatic adenocarcinoma, and vitamin B12 deficiency who returns to the surgical intensive care unit following emergency laparotomy and bowel resection for a perforated sigmoid colon due to diverticulitis five days ago. He has remained intubated for several days postoperatively due to respiratory muscle weakness but is now being gradually weaned off mechanical ventilation. The patient reports feeling very fatigued and weak but denies chest pain, palpitations, tremors, heat or cold intolerance, or changes in bowel movements. He has not had any fevers since surgery and his surgical site appears clean on examination.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitamin B12 deficiency (diagnosed 2020-01-15)
- Pancreatic carcinoma (diagnosed 2020-04-15)
- Atrial flutter (diagnosed 2020-07-15)
- Umbilical hernia, incidental finding (diagnosed 2021-01-15)
- Intra-abdominal sepsis/Secondary peritonitis (diagnosed 2021-01-15)
- Perforated diverticulitis with peritonitis (diagnosed 2021-07-15)
- Hypertension (poorly controlled)
- Type 2 Diabetes Mellitus (diagnosed 18 months ago, 10-year clinical history of poor glycemic control)
- Hyperlipidemia
- Diverticulosis (documented on colonoscopy age 61)
- Umbilical hernia
- Pancreatic adenocarcinoma

His medical history includes hypertension, hyperlipidemia, and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Colonoscopy with documented diverticulosis (age 61)
- Elective open cholecystectomy (initially attempted laparoscopically) with incidental identification of umbilical hernia, complicated by postoperative intra-abdominal sepsis (age 66)
- Emergency exploratory laparotomy with sigmoid colectomy for perforated diverticulitis with peritonitis, complicated by prolonged intubation and euthyroid sick syndrome (age 66)

MEDICATIONS:
He takes amlodipine, atorvastatin, and metformin at home.

Home Medications:
- Lisinopril 20 mg PO daily
- Cyanocobalamin 1000 mcg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Hypertension, dementia (late-onset)
- Paternal uncle: Pancreatic adenocarcinoma
- Older brother: Intracerebral hemorrhage at age 70

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per month). Occupation: Retired postal clerk.

VITAL SIGNS:
On assessment today, temperature is 36.8°C (98.2°F), pulse is 76/min, and blood pressure is 130/82 mmHg. BMI is 28 kg/m².

PHYSICAL EXAMINATION:
He appears tired but is alert and oriented. Cardiovascular, respiratory, and neurologic examinations are unremarkable. Thyroid examination shows no enlargement, tenderness, or nodules.

LABORATORY DATA:
Routine laboratory studies reveal:
TSH                              Normal
Free T4 Normal
Total T3 Decreased

## Encounter 7: source 2024-01-15, practice 2025-10-27, ed, Emergency Medicine

Chief complaint: Sudden onset of left-sided weakness and numbness while watching television

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Lisa Wong, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of left-sided weakness and numbness while watching television

HISTORY OF PRESENT ILLNESS:
The patient with poorly controlled hypertension and pancreatic cancer presents acutely. He reports that he was watching television when he suddenly felt his left arm and leg become heavy and difficult to move.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitamin B12 deficiency (diagnosed 2020-01-15)
- Pancreatic carcinoma (diagnosed 2020-04-15)
- Atrial flutter (diagnosed 2020-07-15)
- Umbilical hernia, incidental finding (diagnosed 2021-01-15)
- Intra-abdominal sepsis/Secondary peritonitis (diagnosed 2021-01-15)
- Perforated diverticulitis with peritonitis (diagnosed 2021-07-15)
- Euthyroid sick syndrome (diagnosed 2021-07-15)
- Hypertension (poorly controlled)
- Type 2 Diabetes Mellitus (diagnosed 18 months ago, 10-year clinical history of poor glycemic control)
- Hyperlipidemia
- Diverticulosis (documented on colonoscopy age 61)
- Umbilical hernia
- Pancreatic adenocarcinoma

His past medical history is significant for type 2 diabetes mellitus and hyperlipidemia.

PAST SURGICAL HISTORY:
- Colonoscopy with documented diverticulosis (age 61)
- Elective open cholecystectomy (initially attempted laparoscopically) with incidental identification of umbilical hernia, complicated by postoperative intra-abdominal sepsis (age 66)
- Emergency exploratory laparotomy with sigmoid colectomy for perforated diverticulitis with peritonitis, complicated by prolonged intubation and euthyroid sick syndrome (age 66)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Amlodipine 5 mg PO daily
- Cyanocobalamin 1000 mcg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Hypertension, dementia (late-onset)
- Paternal uncle: Pancreatic adenocarcinoma
- Older brother: Intracerebral hemorrhage at age 70

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per month). Occupation: Retired postal clerk.

VITAL SIGNS:
On examination, his blood pressure is 190/110 mmHg, heart rate is 88 beats per minute, and respiratory rate is 18 breaths per minute.

PHYSICAL EXAMINATION:
Neurological examination reveals a left-sided hemiparesis with a power of 3/5 in the upper and lower extremities and decreased sensation to light touch and pinprick. His speech is fluent, and there are no visual field deficits.

IMAGING:
A non-contrast head CT scan shows a 3 cm hyperdense area in the right basal ganglia with surrounding edema but no midline shift.
