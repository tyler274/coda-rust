# synthetic_hospital patient 2213

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-06-30 (Wed) | outpatient | Oncology |
| 1 | 2020-07-15 | 2022-01-05 (Wed) | outpatient | Ophthalmology |
| 2 | 2021-01-15 | 2022-07-07 (Thu) | outpatient | Cardiology |
| 3 | 2022-01-15 | 2023-07-10 (Mon) | outpatient | Orthopedics |
| 4 | 2023-01-15 | 2024-06-28 (Fri) | outpatient | Endocrinology |
| 5 | 2024-01-15 | 2025-07-05 (Sat) | ed | Emergency Medicine |
| 6 | 2024-05-15 | 2025-11-02 (Sun) | ed | Emergency Medicine |
| 7 | 2025-01-15 | 2026-07-03 (Fri) | outpatient | Neurology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired accountant",
 "alcohol_use": "None",
 "family_history": [
  "Father with hypertension and coronary artery disease",
  "Mother with type 2 diabetes mellitus",
  "Paternal uncle with colon cancer diagnosed at age 72"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Right hemicolectomy for Stage I colon adenocarcinoma (1 year prior)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Hyperlipidemia",
  "Chronic Kidney Disease Stage 3"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["First-degree atrioventricular block with bundle branch block (incomplete trifascicular block)", "Creutzfeldt-Jakob disease", "Hypertensive retinopathy", "History of stage I colon adenocarcinoma, post-resection status", "Giant cell arteritis or autoimmune inflammatory condition", "Spinal stenosis", "Paget disease of bone", "Acute pancreatitis (alternative nutritional management)"]
Comorbidities: ["Hyperlipidemia", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2021-06-30, outpatient, Oncology

Chief complaint: Routine follow-up visit one year after right hemicolectomy for stage I colon adenocarcinoma. Patient reports no new symptoms and has been feeling well since surgery.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up visit one year after right hemicolectomy for stage I colon adenocarcinoma. Patient reports no new symptoms and has been feeling well since surgery.

HISTORY OF PRESENT ILLNESS:
He reports no new symptoms and has been feeling well since the surgery.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Chronic Kidney Disease Stage 3

His past medical history is significant for hypertension, controlled with lisinopril, and type 2 diabetes mellitus, managed with metformin.

PAST SURGICAL HISTORY:
- Right hemicolectomy for Stage I colon adenocarcinoma (1 year prior)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
He has no family history of colorectal cancer.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired accountant.

VITAL SIGNS:
His vital signs are within normal limits,

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
His most recent laboratory tests are as follows:

- Complete Blood Count (CBC):
- Hemoglobin: 13.8 g/dL
- White Blood Cell Count (WBC): 7.2 x 10^3/uL
- Platelets: 250 x 10^3/uL
- Comprehensive Metabolic Panel (CMP):
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 1.0 mg/dL
- Glucose (fasting): 98 mg/dL
- Carcinoembryonic Antigen (CEA): within normal limits

ASSESSMENT AND PLAN:
Given the patient&#x27;s history and current status,

## Encounter 1: source 2020-07-15, practice 2022-01-05, outpatient, Ophthalmology

Chief complaint: Routine ophthalmology referral for abnormal visual screening. Denies acute visual loss but notes occasional difficulty with peripheral vision and fuzziness around bright lights over several months.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Ophthalmology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine ophthalmology referral for abnormal visual screening. Denies acute visual loss but notes occasional difficulty with peripheral vision and fuzziness around bright lights over several months.

HISTORY OF PRESENT ILLNESS:
This 68-year-old male with hypertension, diabetes, CKD, and recent oncology follow-up presents for evaluation. He denies acute visual loss but describes having occasional difficulty with peripheral vision and some "fuzziness" around bright lights over the past several months. He does not wear corrective lenses.

PAST MEDICAL HISTORY:
Active Problem List:
- History of stage I colon adenocarcinoma, post-resection status (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Chronic Kidney Disease Stage 3

His past medical history includes long-standing hypertension, which he has not consistently managed, and hyperlipidemia. He does not have diabetes.

PAST SURGICAL HISTORY:
- Right hemicolectomy for Stage I colon adenocarcinoma (1 year prior)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension and coronary artery disease
- Mother with type 2 diabetes mellitus
- Paternal uncle with colon cancer diagnosed at age 72

SOCIAL HISTORY:
He denies smoking or alcohol use.

PHYSICAL EXAMINATION:
His visual acuity is mildly reduced bilaterally. Intraocular pressure is within normal limits. On dilated fundoscopic examination, retinal AV nicking is noted, and the image below is obtained.

## Encounter 2: source 2021-01-15, practice 2022-07-07, outpatient, Cardiology

Chief complaint: Routine clinic follow-up with occasional episodes of lightheadedness over the past few months. Denies chest pain, palpitations, or syncope.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Jennifer Park, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine clinic follow-up with occasional episodes of lightheadedness over the past few months. Denies chest pain, palpitations, or syncope.

HISTORY OF PRESENT ILLNESS:
Patient returns to cardiology with history of colon cancer and hypertensive retinopathy. He reports feeling generally well but notes occasional episodes of lightheadedness over the past few months. He denies chest pain, palpitations, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- History of stage I colon adenocarcinoma, post-resection status (diagnosed 2020-01-15)
- Hypertensive retinopathy (diagnosed 2020-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Chronic Kidney Disease Stage 3

PAST SURGICAL HISTORY:
- Right hemicolectomy for Stage I colon adenocarcinoma (1 year prior)

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Amlodipine 5 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension and coronary artery disease
- Mother with type 2 diabetes mellitus
- Paternal uncle with colon cancer diagnosed at age 72

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired accountant.

VITAL SIGNS:
On physical examination, his blood pressure is 132/78 mmHg, heart rate is 58 beats per minute, and the rhythm is regular.

PHYSICAL EXAMINATION:
Cardiac auscultation reveals no murmurs, rubs, or gallops.

OTHER STUDIES:
His electrocardiogram (ECG) shows a first-degree AV block with a PR interval of 220 ms and a QRS duration of 120 ms.

## Encounter 3: source 2022-01-15, practice 2023-07-10, outpatient, Orthopedics

Chief complaint: Six-month history of low back pain and bilateral leg pain that worsens when walking and improves when leaning forward or sitting down.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. David Thompson, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of low back pain and bilateral leg pain that worsens when walking and improves when leaning forward or sitting down.

HISTORY OF PRESENT ILLNESS:
This 70-year-old male with history of colon cancer, hypertensive retinopathy, and first-degree atrioventricular block with bundle branch block presents with leg pain. He reports that the pain in his legs worsens when he walks for more than a few minutes and improves when he leans forward or sits down. The pain is described as a dull ache and is not associated with any numbness or tingling.

PAST MEDICAL HISTORY:
Active Problem List:
- History of stage I colon adenocarcinoma, post-resection status (diagnosed 2020-01-15)
- Hypertensive retinopathy (diagnosed 2020-07-15)
- First-degree atrioventricular block with bundle branch block (incomplete trifascicular block) (diagnosed 2021-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Chronic Kidney Disease Stage 3

His past medical history includes hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Right hemicolectomy for Stage I colon adenocarcinoma (1 year prior)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension and coronary artery disease
- Mother with type 2 diabetes mellitus
- Paternal uncle with colon cancer diagnosed at age 72

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired accountant.

PHYSICAL EXAMINATION:
On physical examination, lumbar extension elicits pain, but there is no loss of sensation or motor weakness in the lower extremities. Ankle-brachial index (ABI) is normal.

## Encounter 4: source 2023-01-15, practice 2024-06-28, outpatient, Endocrinology

Chief complaint: Progressive difficulty walking over the past year with deep aching pain in left hip and upper thigh. Recent increase in hat size and hearing loss noted by spouse.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Aisha Patel, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty walking over the past year with deep aching pain in left hip and upper thigh. Recent increase in hat size and hearing loss noted by spouse.

HISTORY OF PRESENT ILLNESS:
This 71-year-old male with hypertension, type 2 diabetes mellitus, hyperlipidemia, CKD stage 3, history of stage I colon adenocarcinoma status post resection, and prior spinal stenosis returns for evaluation of progressive walking difficulty. He reports needing to take frequent breaks when walking through his neighborhood due to a deep aching pain in his left hip and upper thigh that worsens throughout the day. He also notes that his hat size increased recently and his wife says his hearing has become poor over the past few months, requiring higher television volume. He denies fevers, chills, night sweats, unintentional weight loss, or recent trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- History of stage I colon adenocarcinoma, post-resection status (diagnosed 2020-01-15)
- Hypertensive retinopathy (diagnosed 2020-07-15)
- First-degree atrioventricular block with bundle branch block (incomplete trifascicular block) (diagnosed 2021-01-15)
- Spinal stenosis (diagnosed 2022-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Chronic Kidney Disease Stage 3

Medical history is notable for hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Right hemicolectomy for Stage I colon adenocarcinoma (1 year prior)

MEDICATIONS:
He takes amlodipine and atorvastatin.

Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension and coronary artery disease
- Mother with type 2 diabetes mellitus
- Paternal uncle with colon cancer diagnosed at age 72

SOCIAL HISTORY:
He lives independently and is retired from accounting.

VITAL SIGNS:
On examination, temperature is 36.7°C (98.1°F), blood pressure is 142/86 mmHg, and BMI is 26 kg/m²

PHYSICAL EXAMINATION:
There is mild frontal bossing with decreased hearing acuity on the left and bowing deformity of the left femur. Neurologic examination is unremarkable.

LABORATORY DATA:
Laboratory evaluation is obtained:
Calcium Normal
Phosphate                                                        Normal
Alkaline phosphatase                                       Elevated

## Encounter 5: source 2024-01-15, practice 2025-07-05, ed, Emergency Medicine

Chief complaint: Sudden painless vision loss in right eye two hours ago described as a descending shadow while reading. Associated with several weeks of fatigue and generalized aches.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden painless vision loss in right eye two hours ago described as a descending shadow while reading. Associated with several weeks of fatigue and generalized aches.

HISTORY OF PRESENT ILLNESS:
The patient is a 72-year-old male with a history of hypertensive retinopathy and Paget disease of bone who presents with acute visual symptoms. He reports that the episode occurred while reading and describes it as a "shadow" descending over his field of vision. The vision loss was painless and has not improved over the last two hours. He has had mild fatigue and generalized aches for several weeks but did not seek medical attention. He denies any trauma, diplopia, or eye discharge.

PAST MEDICAL HISTORY:
Active Problem List:
- History of stage I colon adenocarcinoma, post-resection status (diagnosed 2020-01-15)
- Hypertensive retinopathy (diagnosed 2020-07-15)
- First-degree atrioventricular block with bundle branch block (incomplete trifascicular block) (diagnosed 2021-01-15)
- Spinal stenosis (diagnosed 2022-01-15)
- Paget disease of bone (diagnosed 2023-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Chronic Kidney Disease Stage 3

Past medical history includes hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Right hemicolectomy for Stage I colon adenocarcinoma (1 year prior)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension and coronary artery disease
- Mother with type 2 diabetes mellitus
- Paternal uncle with colon cancer diagnosed at age 72

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired accountant.

VITAL SIGNS:
His temperature is 37.8°C (100°F), pulse is 78/min, blood pressure is 136/72 mmHg, and respiratory rate is 16/min.

PHYSICAL EXAMINATION:
Physical exam reveals tenderness over the right temple with reduced pulsation, and difficulty chewing due to discomfort in his jaw. Fundoscopy is shown below.

## Encounter 6: source 2024-05-15, practice 2025-11-02, ed, Emergency Medicine

Chief complaint: Severe sudden onset sharp abdominal pain radiating to the back starting 12 hours ago.

ENCOUNTER NOTE
Date: 2024-05-15
Provider: Dr. Emily Nakamura, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe sudden onset sharp abdominal pain radiating to the back starting 12 hours ago.

HISTORY OF PRESENT ILLNESS:
This 72-year-old male with hypertension, diabetes, chronic kidney disease, and recent giant cell arteritis presents to the emergency department with severe abdominal pain beginning 12 hours ago, sudden in onset, sharp, and radiating to the back.

PAST MEDICAL HISTORY:
Active Problem List:
- History of stage I colon adenocarcinoma, post-resection status (diagnosed 2020-01-15)
- Hypertensive retinopathy (diagnosed 2020-07-15)
- First-degree atrioventricular block with bundle branch block (incomplete trifascicular block) (diagnosed 2021-01-15)
- Spinal stenosis (diagnosed 2022-01-15)
- Paget disease of bone (diagnosed 2023-01-15)
- Giant cell arteritis or autoimmune inflammatory condition (diagnosed 2024-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Chronic Kidney Disease Stage 3

He has a history of hypertension, hyperlipidemia, and chronic kidney disease.

PAST SURGICAL HISTORY:
He underwent cardiac catheterization 2 weeks ago for evaluation of chest pain.

MEDICATIONS:
His medications include aspirin, atorvastatin, and lisinopril.

Home Medications:
- Metformin 1000 mg PO BID
- Amlodipine 5 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension and coronary artery disease
- Mother with type 2 diabetes mellitus
- Paternal uncle with colon cancer diagnosed at age 72

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired accountant.

VITAL SIGNS:
On examination, his blood pressure is 150/90 mmHg, heart rate is 98 beats per minute, and temperature is 37.2°C (99.0°F).

PHYSICAL EXAMINATION:
Abdominal examination reveals epigastric tenderness without rebound or guarding. Bowel sounds are normal. There is no jaundice or ascites. A bluish discoloration of the toes and a net-like, purplish discoloration of the skin on his legs are noted.

LABORATORY DATA:
Laboratory studies show:

- White blood cell count: 14,000/mm³
- Hemoglobin: 13.2 g/dL
- Platelet count: 250,000/mm³
- Serum amylase: 800 U/L (Normal: 30-110 U/L)
- Serum lipase: 900 U/L (Normal: 0-160 U/L)
- Serum creatinine: 2.1 mg/dL (baseline 1.8 mg/dL)
- Aspartate aminotransferase (AST): 45 U/L
- Alanine aminotransferase (ALT): 50 U/L
- Total bilirubin: 1.2 mg/dL

## Encounter 7: source 2025-01-15, practice 2026-07-03, outpatient, Neurology

Chief complaint: Three-month history of rapidly progressive memory loss, confusion, and sudden muscle jerks with difficulty recognizing family members.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Robert Hassan, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of rapidly progressive memory loss, confusion, and sudden muscle jerks with difficulty recognizing family members.

HISTORY OF PRESENT ILLNESS:
This 73-year-old with colon cancer and giant cell arteritis presents. His wife reports that his symptoms have rapidly progressed over the last month, and he now has difficulty recognizing family members.

PAST MEDICAL HISTORY:
Active Problem List:
- History of stage I colon adenocarcinoma, post-resection status (diagnosed 2020-01-15)
- Hypertensive retinopathy (diagnosed 2020-07-15)
- First-degree atrioventricular block with bundle branch block (incomplete trifascicular block) (diagnosed 2021-01-15)
- Spinal stenosis (diagnosed 2022-01-15)
- Paget disease of bone (diagnosed 2023-01-15)
- Giant cell arteritis or autoimmune inflammatory condition (diagnosed 2024-01-15)
- Acute pancreatitis (alternative nutritional management) (diagnosed 2024-05-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Chronic Kidney Disease Stage 3

His past medical history is significant for hypertension and hypercholesterolemia.

PAST SURGICAL HISTORY:
- Right hemicolectomy for Stage I colon adenocarcinoma (1 year prior)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension and coronary artery disease
- Mother with type 2 diabetes mellitus
- Paternal uncle with colon cancer diagnosed at age 72

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired accountant.

PHYSICAL EXAMINATION:
On physical examination, the patient appears disoriented and has difficulty following commands. Neurological examination reveals myoclonic jerks involving the upper extremities.

LABORATORY DATA:
Routine blood tests are normal.

IMAGING:
A magnetic resonance imaging (MRI) scan of the brain demonstrates diffuse cortical atrophy without evidence of stroke or mass lesions.

OTHER STUDIES:
An electroencephalogram (EEG) shows sharp, triphasic, synchronous discharges.
