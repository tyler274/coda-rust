# synthetic_hospital patient 2206

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 66 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2019-03-18 (Mon) | outpatient | Cardiology |
| 1 | 2025-09-15 | 2024-11-12 (Tue) | outpatient | Gastroenterology |
| 2 | 2025-11-15 | 2025-01-06 (Mon) | outpatient | Oncology |
| 3 | 2026-01-15 | 2025-03-01 (Sat) | ed | Emergency Medicine |
| 4 | 2026-01-15 | 2025-03-09 (Sun) | inpatient | General Surgery |
| 5 | 2026-01-15 | 2025-03-09 (Sun) | inpatient | General Surgery |
| 6 | 2026-02-15 | 2025-04-05 (Sat) | ed | Emergency Medicine |
| 7 | 2027-01-15 | 2026-03-07 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired postal worker",
 "alcohol_use": "None",
 "family_history": [
  "Father - Colon cancer (diagnosed age 70, deceased)",
  "Mother - Type 2 Diabetes Mellitus",
  "Mother - Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
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
   "dose": "81 mg daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Knee arthroscopy (age 55)",
  "Tonsillectomy (age 8)"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus",
  "Hypertension",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 66
}
```

Primary diagnoses: ["Postoperative Atrial Fibrillation with Rapid Ventricular Response", "Colorectal neoplasia", "Postoperative ileus", "Acute bacterial parotitis", "Secondary malignant neoplasm of liver", "Acute myocardial infarction", "Left ventricular hypertrophy"]
Comorbidities: ["Colorectal neoplasia", "Essential hypertension", "Type 2 diabetes mellitus", "Status post laparoscopic cholecystectomy"]

## Encounter 0: source 2020-01-15, practice 2019-03-18, outpatient, Cardiology

Chief complaint: Fatigue and reduced exercise tolerance

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue and reduced exercise tolerance

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with fatigue and reduced exercise tolerance for the past few months.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia

He has a history of poorly controlled hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Knee arthroscopy (age 55)
- Tonsillectomy (age 8)

MEDICATIONS:
Medications include lisinopril, metformin, and amlodipine.

Home Medications:
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father - Colon cancer (diagnosed age 70, deceased)
- Mother - Type 2 Diabetes Mellitus
- Mother - Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired postal worker.

VITAL SIGNS:
His blood pressure is 160/96 mm Hg, and pulse is 72/min and regular.

PHYSICAL EXAMINATION:
Cardiopulmonary examination is notable for a displaced apical impulse and an S4 gallop. There are no murmurs or signs of heart failure.

LABORATORY DATA:
Laboratory results show normal renal function and electrolytes.

OTHER STUDIES:
ECG findings are shown below.

## Encounter 1: source 2025-09-15, practice 2024-11-12, outpatient, Gastroenterology

Chief complaint: Progressive fatigue and unintentional weight loss over the past 4 months

ENCOUNTER NOTE
Date: 2025-09-15
Provider: Dr. Michael Rodriguez, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue and unintentional weight loss over the past 4 months

HISTORY OF PRESENT ILLNESS:
The patient, with diabetes, hypertension, hyperlipidemia and 2020 Cardiology evaluation, reports early satiety, mild right upper quadrant discomfort, and intermittent constipation. He denies nausea, vomiting, or visible blood in stool.

PAST MEDICAL HISTORY:
Active Problem List:
- Left ventricular hypertrophy (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia

He has a history of type 2 diabetes and hypertension. There is no known history of viral hepatitis or liver disease. He has never undergone cancer screening.

PAST SURGICAL HISTORY:
- Knee arthroscopy (age 55)
- Tonsillectomy (age 8)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father - Colon cancer (diagnosed age 70, deceased)
- Mother - Type 2 Diabetes Mellitus
- Mother - Hypertension

SOCIAL HISTORY:
He does not smoke, drink alcohol, or take any hepatotoxic medications.

PHYSICAL EXAMINATION:
On physical exam, the patient appears thin but in no acute distress. Sclerae are anicteric. The abdomen is soft, with a firm, non-tender liver edge palpable 3 cm below the right costal margin.

LABORATORY DATA:
Laboratory studies reveal:
AST 51 U/L
ALT 44 U/L
ALP 189 U/L
Total bilirubin 1.4 mg/dL
Albumin 3.4 g/dL
INR 1.1
AFP is within normal limits.

IMAGING:
A contrast-enhanced CT scan of the abdomen is performed,

PATHOLOGY:
Followed by a core biopsy of a representative lesion. The liver findings in this patient are similar to those shown in the representative gross pathology specimen below.

## Encounter 2: source 2025-11-15, practice 2025-01-06, outpatient, Oncology

Chief complaint: Intermittent right upper quadrant pain and unexplained weight loss of 10 pounds

ENCOUNTER NOTE
Date: 2025-11-15
Provider: Dr. Fatima Al-Hassan, MD
Department: Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent right upper quadrant pain and unexplained weight loss of 10 pounds

HISTORY OF PRESENT ILLNESS:
This 71-year-old male with type 2 diabetes mellitus, hypertension, and hyperlipidemia, recently evaluated for colorectal neoplasia, presents with a three-month history of intermittent right upper quadrant pain and unexplained weight loss of 10 pounds.

PAST MEDICAL HISTORY:
Active Problem List:
- Left ventricular hypertrophy (diagnosed 2020-01-15)
- Colorectal neoplasia (diagnosed 2025-09-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Knee arthroscopy (age 55)
- Tonsillectomy (age 8)

MEDICATIONS:
Metformin, lisinopril, and a daily aspirin

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father - Colon cancer (diagnosed age 70, deceased)
- Mother - Type 2 Diabetes Mellitus
- Mother - Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired postal worker.

VITAL SIGNS:
Vital signs are within normal limits

PHYSICAL EXAMINATION:
Abdominal examination reveals a firm, enlarged liver without jaundice

LABORATORY DATA:
Hemoglobin: 12.8 g/dL (normal: 13.8-17.2 g/dL)
- White blood cell count: 7,000/uL (normal: 4,500-11,000/uL)
- Platelets: 250,000/uL (normal: 150,000-450,000/uL)
- Aspartate aminotransferase (AST): 68 U/L (normal: 10-40 U/L)
- Alanine aminotransferase (ALT): 55 U/L (normal: 7-56 U/L)
- Alkaline phosphatase: 210 U/L (normal: 40-130 U/L)
- Total bilirubin: 1.2 mg/dL (normal: 0.3-1.2 mg/dL)

PLAN:
A CT scan of the abdomen with contrast is ordered.

## Encounter 3: source 2026-01-15, practice 2025-03-01, ed, Emergency Medicine

Chief complaint: Severe abdominal pain, fever, and confusion

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe abdominal pain, fever, and confusion

HISTORY OF PRESENT ILLNESS:
The patient is a 72-year-old male with Type 2 Diabetes Mellitus, Hypertension, Hyperlipidemia, and metastatic colorectal neoplasia presenting with 2-day history of severe abdominal pain, fever, and confusion. His wife reports that he has not had a bowel movement in 3 days and has refused to eat due to the pain. Blood cultures are drawn and the patient is started on broad-spectrum antibiotics.

PAST MEDICAL HISTORY:
Active Problem List:
- Left ventricular hypertrophy (diagnosed 2020-01-15)
- Colorectal neoplasia (diagnosed 2025-09-15)
- Secondary malignant neoplasm of liver (diagnosed 2025-11-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Knee arthroscopy (age 55)
- Tonsillectomy (age 8)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father - Colon cancer (diagnosed age 70, deceased)
- Mother - Type 2 Diabetes Mellitus
- Mother - Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired postal worker.

VITAL SIGNS:
His temperature is 38.9°C (102°F), blood pressure is 90/60 mmHg, pulse is 110/min, and respiratory rate is 22/min

PHYSICAL EXAMINATION:
His abdomen is diffusely tender to palpation without rebound or guarding.

LABORATORY DATA:
Laboratory studies are significant for leukocytosis with a left shift. Two days later, blood cultures grow gram-positive rods.

IMAGING:
A CT scan of the abdomen and pelvis shows no evidence of diverticulitis or bowel obstruction.

## Encounter 4: source 2026-01-15, practice 2025-03-09, inpatient, General Surgery

Chief complaint: Palpitations and mild shortness of breath postoperatively

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. Priya Patel, MD
Department: General Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Palpitations and mild shortness of breath postoperatively

HISTORY OF PRESENT ILLNESS:
The patient is a 72-year-old male with a history of type 2 diabetes mellitus and hypertension, previously evaluated in 2020 for left ventricular hypertrophy and recently diagnosed with colorectal neoplasia with secondary malignant neoplasm of the liver, who is admitted to the hospital for a right hemicolectomy due to a recently diagnosed colon cancer. Postoperatively, the patient is stable and is transferred to the medical floor. During the handoff between the surgical team and the medical team, the surgical resident briefly mentions that the patient had an episode of atrial fibrillation with rapid ventricular response intraoperatively, which was managed with metoprolol. The patient is currently asymptomatic. The next morning, the patient complains of palpitations and mild shortness of breath.

PAST MEDICAL HISTORY:
Active Problem List:
- Left ventricular hypertrophy (diagnosed 2020-01-15)
- Colorectal neoplasia (diagnosed 2025-09-15)
- Secondary malignant neoplasm of liver (diagnosed 2025-11-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
Right hemicolectomy

MEDICATIONS:
He takes enoxaparin for venous thromboembolism prophylaxis, insulin sliding scale, and continuation of his home medications including metformin and lisinopril.

Home Medications:
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father - Colon cancer (diagnosed age 70, deceased)
- Mother - Type 2 Diabetes Mellitus
- Mother - Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired postal worker.

VITAL SIGNS:
- Blood pressure: 130/85 mmHg
- Heart rate: 78 bpm
- Respiratory rate: 16 breaths/min
- Temperature: 36.8°C
- Oxygen saturation: 97% on room air

PHYSICAL EXAMINATION:
On examination, his heart rate is irregularly irregular, and his blood pressure is 120/80 mmHg.

OTHER STUDIES:
An ECG is performed and shows atrial fibrillation with a ventricular rate of 110 bpm.

## Encounter 5: source 2026-01-15, practice 2025-03-09, inpatient, General Surgery

Chief complaint: Abdominal distension on postoperative day 4

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. Priya Patel, MD
Department: General Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Abdominal distension on postoperative day 4

HISTORY OF PRESENT ILLNESS:
This 72-year-old male with type 2 diabetes mellitus, hypertension, hyperlipidemia, and metastatic colorectal carcinoma presents to the surgical ward on postoperative day 4 after undergoing an open right hemicolectomy for colon cancer. He reports abdominal distension and has not passed flatus or had a bowel movement since the surgery.

PAST MEDICAL HISTORY:
Active Problem List:
- Left ventricular hypertrophy (diagnosed 2020-01-15)
- Colorectal neoplasia (diagnosed 2025-09-15)
- Secondary malignant neoplasm of liver (diagnosed 2025-11-15)
- Postoperative Atrial Fibrillation with Rapid Ventricular Response (diagnosed 2026-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia

His past medical history is significant for hypertension, for which he takes lisinopril, and type 2 diabetes managed with metformin.

PAST SURGICAL HISTORY:
- Knee arthroscopy (age 55)
- Tonsillectomy (age 8)

MEDICATIONS:
He takes lisinopril, metformin.

Home Medications:
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father - Colon cancer (diagnosed age 70, deceased)
- Mother - Type 2 Diabetes Mellitus
- Mother - Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired postal worker.

VITAL SIGNS:
His temperature is 37.2°C (99°F), blood pressure is 135/85 mmHg, and heart rate is 88 beats per minute.

PHYSICAL EXAMINATION:
On examination, his abdomen is distended and tympanic to percussion, but there is no tenderness on palpation. Bowel sounds are sparse.

LABORATORY DATA:
- Hemoglobin: 12.5 g/dL
- White blood cell count: 9,000/mm³
- Serum electrolytes:
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 102 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen: 18 mg/dL
- Creatinine: 1.0 mg/dL

IMAGING:
An abdominal X-ray shows diffuse bowel gas without evidence of free air under the diaphragm.

## Encounter 6: source 2026-02-15, practice 2025-04-05, ed, Emergency Medicine

Chief complaint: Progressive right-sided facial swelling and pain

ENCOUNTER NOTE
Date: 2026-02-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive right-sided facial swelling and pain

HISTORY OF PRESENT ILLNESS:
This 72-year-old male with metastatic colorectal neoplasia and recent surgery complicated by atrial fibrillation and ileus presents to the emergency department with a 2-day history of progressive right-sided facial swelling and pain. Since surgery, he has been experiencing difficulty chewing due to pain, which has limited his oral intake.

PAST MEDICAL HISTORY:
Active Problem List:
- Left ventricular hypertrophy (diagnosed 2020-01-15)
- Colorectal neoplasia (diagnosed 2025-09-15)
- Secondary malignant neoplasm of liver (diagnosed 2025-11-15)
- Postoperative Atrial Fibrillation with Rapid Ventricular Response (diagnosed 2026-01-15)
- Postoperative ileus (diagnosed 2026-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia

He has a past medical history of hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
He underwent a right hemicolectomy for colon cancer 5 days ago.

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father - Colon cancer (diagnosed age 70, deceased)
- Mother - Type 2 Diabetes Mellitus
- Mother - Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired postal worker.

VITAL SIGNS:
On examination, his temperature is 38.6°C (101.5°F), blood pressure is 135/85 mmHg, pulse is 98 beats per minute, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
The right parotid gland is tender, erythematous, and swollen. There is purulent discharge from Stensen&#x27;s duct upon gland palpation.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 14,000 /µL
- Neutrophils: 80%
- Blood glucose: 180 mg/dL
- Serum sodium: 140 mEq/L
- Serum potassium: 4.2 mEq/L
- Serum creatinine: 1.1 mg/dL

## Encounter 7: source 2027-01-15, practice 2026-03-07, ed, Emergency Medicine

Chief complaint: Severe substernal chest pain for 2 hours and worsening shortness of breath

ENCOUNTER NOTE
Date: 2027-01-15
Provider: Dr. Emily Watson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe substernal chest pain for 2 hours and worsening shortness of breath

PAST MEDICAL HISTORY:
Active Problem List:
- Left ventricular hypertrophy (diagnosed 2020-01-15)
- Colorectal neoplasia (diagnosed 2025-09-15)
- Secondary malignant neoplasm of liver (diagnosed 2025-11-15)
- Postoperative Atrial Fibrillation with Rapid Ventricular Response (diagnosed 2026-01-15)
- Postoperative ileus (diagnosed 2026-01-15)
- Acute bacterial parotitis (diagnosed 2026-02-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia

He has a history of type 2 diabetes mellitus treated with oral medications and hypertension treated with a diuretic.

PAST SURGICAL HISTORY:
- Knee arthroscopy (age 55)
- Tonsillectomy (age 8)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father - Colon cancer (diagnosed age 70, deceased)
- Mother - Type 2 Diabetes Mellitus
- Mother - Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired postal worker.

VITAL SIGNS:
His temperature is 37.5°C (99.5°F), pulse is 104/min, respirations are 28/min, and blood pressure is 100/78 mm Hg.
Pulse oximetry on room air shows oxygen saturation of 88%.

PHYSICAL EXAMINATION:
Jugular venous pressure is elevated. Bilateral crackles are auscultated throughout the lung fields. An S3 and S4 gallop are present on cardiac exam.

IMAGING:
Chest x-ray reveals bilateral perihilar infiltrates and a mildly enlarged cardiac silhouette.

OTHER STUDIES:
ECG findings are shown below.
