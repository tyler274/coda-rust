# synthetic_hospital patient 2701

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 52 at first encounter, sex F, Non-Hispanic White, Private PPO insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-11-20 (Fri) | outpatient | Endocrinology |
| 1 | 2020-04-15 | 2021-02-09 (Tue) | outpatient | Neurology |
| 2 | 2020-10-15 | 2021-08-13 (Fri) | outpatient | Orthopedics |
| 3 | 2021-04-15 | 2022-02-21 (Mon) | outpatient | Internal Medicine |
| 4 | 2022-02-15 | 2022-12-08 (Thu) | outpatient | Neurology |
| 5 | 2023-02-15 | 2023-12-18 (Mon) | outpatient | Neurology |
| 6 | 2024-02-15 | 2024-12-16 (Mon) | outpatient | General Surgery |
| 7 | 2025-02-15 | 2025-12-18 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private PPO insurance",
 "occupation": "Administrative Assistant",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Breast cancer diagnosed age 65",
  "Father: Type 2 diabetes mellitus",
  "Mother: Hypertension",
  "Sister: Hypothyroidism"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  },
  {
   "dose": "75 mcg daily",
   "name": "Levothyroxine"
  },
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg daily",
   "name": "Amlodipine"
  },
  {
   "dose": "25 mg daily",
   "name": "Hydrochlorothiazide"
  }
 ],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus",
  "Hypertension",
  "Hypothyroidism",
  "History of peptic ulcer disease"
 ],
 "age_at_first_encounter": 52
}
```

Primary diagnoses: ["Chronic mechanical lower back pain", "Acute cholecystitis", "Restless Legs Syndrome", "Trigeminal neuralgia", "Meningioma", "Inflammatory breast carcinoma", "Adhesive capsulitis of left shoulder", "Primary Hyperaldosteronism (Conn's Syndrome)"]
Comorbidities: ["Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2020-11-20, outpatient, Endocrinology

Chief complaint: Worsening headaches, fatigue, and muscle weakness despite adherence to antihypertensive regimen.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening headaches, fatigue, and muscle weakness despite adherence to antihypertensive regimen.

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with a 6-month history of worsening headaches, fatigue, and muscle weakness. She also reports experiencing episodes of palpitations and diaphoresis.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus
- Hypertension
- Hypothyroidism
- History of peptic ulcer disease

Her past medical history is significant for hypertension, which was diagnosed 2 years ago and has been poorly controlled despite compliance with a three-drug regimen.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
She takes amlodipine, lisinopril, and hydrochlorothiazide.

Home Medications:
- Metformin 1000 mg twice daily
- Levothyroxine 75 mcg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer diagnosed age 65
- Father: Type 2 diabetes mellitus
- Mother: Hypertension
- Sister: Hypothyroidism

SOCIAL HISTORY:
The patient expresses a strong preference for non-surgical management due to a fear of surgery.

VITAL SIGNS:
Blood pressure is 158/100 mmHg

PHYSICAL EXAMINATION:
No abdominal bruits are heard

LABORATORY DATA:
- Serum potassium: 2.9 mEq/L (Normal: 3.5-5.0 mEq/L)
- Serum sodium: 145 mEq/L (Normal: 135-145 mEq/L)
- Plasma renin activity: Low
- Plasma aldosterone concentration: Elevated

IMAGING:
An abdominal CT scan shows a 2 cm mass in the left adrenal gland with no evidence of bilateral adrenal hyperplasia.

## Encounter 1: source 2020-04-15, practice 2021-02-09, outpatient, Neurology

Chief complaint: Uncomfortable creeping sensation in legs, particularly in the evenings and at night, with urge to move.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Johnson, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Uncomfortable creeping sensation in legs, particularly in the evenings and at night, with urge to move.

HISTORY OF PRESENT ILLNESS:
The patient is a 52-year-old female with type 2 diabetes, hypertension, hypothyroidism, and PUD who returns following her recent endocrinology evaluation for hyperaldosteronism. She describes it as a deep-seated urge to move her legs, accompanied by a creeping, crawling feeling. These symptoms have been progressively worsening over the past six months and are temporarily relieved by walking or stretching. She denies any pain, weakness, or sensory changes in her legs.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary Hyperaldosteronism (Conn's Syndrome) (diagnosed 2020-01-15)
- Type 2 diabetes mellitus
- Hypertension
- Hypothyroidism
- History of peptic ulcer disease

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
She takes metformin and lisinopril.

Home Medications:
- Levothyroxine 75 mcg daily
- Amlodipine 5 mg daily
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer diagnosed age 65
- Father: Type 2 diabetes mellitus
- Mother: Hypertension
- Sister: Hypothyroidism

SOCIAL HISTORY:
She denies smoking, alcohol, or illicit drug use.

PHYSICAL EXAMINATION:
On physical examination, her vital signs are within normal limits, and neurological examination shows no abnormalities.

LABORATORY DATA:
Iron studies reveal the following:

- Serum iron: 60 mcg/dL (normal: 50-170 mcg/dL)
- Total iron-binding capacity (TIBC): 300 mcg/dL (normal: 250-450 mcg/dL)
- Ferritin: 15 ng/mL (normal: 15-200 ng/mL)

## Encounter 2: source 2020-10-15, practice 2021-08-13, outpatient, Orthopedics

Chief complaint: Progressive left shoulder pain and stiffness with difficulty reaching overhead and behind back.

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Priya Patel, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive left shoulder pain and stiffness with difficulty reaching overhead and behind back.

HISTORY OF PRESENT ILLNESS:
The patient is a 52-year-old female with Type 2 diabetes, hypertension, hypothyroidism, and peptic ulcer disease, evaluated for primary hyperaldosteronism and restless legs syndrome, who presents with a 6-month history of progressive left shoulder pain and stiffness. She reports difficulty in reaching overhead and behind her back and denies any recent trauma or injury to the shoulder.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary Hyperaldosteronism (Conn's Syndrome) (diagnosed 2020-01-15)
- Restless Legs Syndrome (diagnosed 2020-04-15)
- Type 2 diabetes mellitus
- Hypertension
- Hypothyroidism
- History of peptic ulcer disease

She has a past medical history of type 2 diabetes mellitus and hypothyroidism, which are well-controlled with metformin and levothyroxine, respectively.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
She takes metformin and levothyroxine.

Home Medications:
- Lisinopril 20 mg daily
- Amlodipine 5 mg daily
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer diagnosed age 65
- Father: Type 2 diabetes mellitus
- Mother: Hypertension
- Sister: Hypothyroidism

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Administrative Assistant.

PHYSICAL EXAMINATION:
On physical examination, there is a noticeable decrease in both active and passive range of motion of the left shoulder in all directions. There is no erythema, warmth, or swelling. The rest of the musculoskeletal examination is unremarkable.

LABORATORY DATA:
Her most recent laboratory values are as follows:

- Hemoglobin A1c: 6.8%
- Thyroid-stimulating hormone (TSH): 2.5 mIU/L
- C-reactive protein (CRP): 3 mg/L (normal <5 mg/L)
- Erythrocyte sedimentation rate (ESR): 20 mm/hr (normal 0-20 mm/hr)

## Encounter 3: source 2021-04-15, practice 2022-02-21, outpatient, Internal Medicine

Chief complaint: Chronic dull lower back pain for 6 months that worsens with movement and at the end of the day.

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. Robert Kim, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Chronic dull lower back pain for 6 months that worsens with movement and at the end of the day.

HISTORY OF PRESENT ILLNESS:
The patient is a 53-year-old female with type 2 diabetes mellitus, hypertension, hypothyroidism, peptic ulcer disease, and adhesive capsulitis who presents for evaluation of chronic lower back pain. She describes the pain as a constant dull ache that worsens with movement and at the end of the day. She has tried over-the-counter acetaminophen and ibuprofen with minimal relief. She expresses interest in exploring complementary and alternative medicine (CAM) options, as she is concerned about the side effects of conventional medications. She inquires about the use of acupuncture and herbal supplements.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary Hyperaldosteronism (Conn's Syndrome) (diagnosed 2020-01-15)
- Restless Legs Syndrome (diagnosed 2020-04-15)
- Adhesive capsulitis of left shoulder (diagnosed 2020-10-15)
- Type 2 diabetes mellitus
- Hypertension
- Hypothyroidism
- History of peptic ulcer disease

Her medical history is significant for a remote history of peptic ulcer disease.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
She is currently taking metformin and lisinopril.

Home Medications:
- Levothyroxine 75 mcg daily
- Amlodipine 5 mg daily
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer diagnosed age 65
- Father: Type 2 diabetes mellitus
- Mother: Hypertension
- Sister: Hypothyroidism

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Administrative Assistant.

PHYSICAL EXAMINATION:
On physical examination, there is tenderness to palpation over the lumbar paraspinal muscles without neurological deficits.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin A1c: 7.2%
- Fasting blood glucose: 158 mg/dL
- Serum creatinine: 1.1 mg/dL
- Blood urea nitrogen (BUN): 14 mg/dL
- Liver function tests: Within normal limits

## Encounter 4: source 2022-02-15, practice 2022-12-08, outpatient, Neurology

Chief complaint: Intermittent, severe, sharp electric shock-like pain on the right side of her face triggered by chewing.

ENCOUNTER NOTE
Date: 2022-02-15
Provider: Dr. Elena Rodriguez, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent, severe, sharp electric shock-like pain on the right side of her face triggered by chewing.

HISTORY OF PRESENT ILLNESS:
The patient with Type 2 diabetes mellitus, hypertension, hypothyroidism, peptic ulcer disease, and prior restless legs syndrome presents to Neurology. She reports a 6-month history of intermittent, severe, sharp pain on the right side of her face. The pain is triggered by chewing and sometimes by touching her face. She describes the pain as "electric shocks" lasting for a few seconds to two minutes.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary Hyperaldosteronism (Conn's Syndrome) (diagnosed 2020-01-15)
- Restless Legs Syndrome (diagnosed 2020-04-15)
- Adhesive capsulitis of left shoulder (diagnosed 2020-10-15)
- Chronic mechanical lower back pain (diagnosed 2021-04-15)
- Type 2 diabetes mellitus
- Hypertension
- Hypothyroidism
- History of peptic ulcer disease

Her medical history is significant for hypertension, controlled with hydrochlorothiazide, and type 2 diabetes mellitus, managed with metformin.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
She takes hydrochlorothiazide, metformin.

Home Medications:
- Levothyroxine 75 mcg daily
- Lisinopril 20 mg daily
- Amlodipine 5 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer diagnosed age 65
- Father: Type 2 diabetes mellitus
- Mother: Hypertension
- Sister: Hypothyroidism

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Administrative Assistant.

PHYSICAL EXAMINATION:
On physical examination, there is no facial asymmetry or sensory loss, but she winces when the right cheek is lightly palpated.

LABORATORY DATA:
Her recent complete blood count (CBC) shows:
- Hemoglobin: 14 g/dL
- White blood cell count (WBC): 6,000/uL
- Platelets: 250,000/uL

## Encounter 5: source 2023-02-15, practice 2023-12-18, outpatient, Neurology

Chief complaint: Persistent headaches for 6 months with recent onset of right-sided weakness and worsening with coughing.

ENCOUNTER NOTE
Date: 2023-02-15
Provider: Dr. James Wilson, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent headaches for 6 months with recent onset of right-sided weakness and worsening with coughing.

HISTORY OF PRESENT ILLNESS:
The 55-year-old with diabetes and hypertension returns to neurology with headaches, having presented for restless legs and neuralgia. She describes constant, dull headaches worsening with coughing and bending over, denying nausea, vomiting, or changes in vision.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary Hyperaldosteronism (Conn's Syndrome) (diagnosed 2020-01-15)
- Restless Legs Syndrome (diagnosed 2020-04-15)
- Adhesive capsulitis of left shoulder (diagnosed 2020-10-15)
- Chronic mechanical lower back pain (diagnosed 2021-04-15)
- Trigeminal neuralgia (diagnosed 2022-02-15)
- Type 2 diabetes mellitus
- Hypertension
- Hypothyroidism
- History of peptic ulcer disease

Her medical history is significant for hypertension, well-controlled on hydrochlorothiazide.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Levothyroxine 75 mcg daily
- Lisinopril 20 mg daily
- Amlodipine 5 mg daily
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer diagnosed age 65
- Father: Type 2 diabetes mellitus
- Mother: Hypertension
- Sister: Hypothyroidism

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Administrative Assistant.

PHYSICAL EXAMINATION:
On physical examination, she has a mild right-sided hemiparesis and a positive right Babinski sign.

IMAGING:
MRI of the brain with contrast shows an extra-axial, dural-based mass lesion with a homogenous enhancement pattern, located at the convexity of the left parietal lobe.

## Encounter 6: source 2024-02-15, practice 2024-12-16, outpatient, General Surgery

Chief complaint: 6-week history of progressive left breast redness, swelling, warmth, and persistent itching.

ENCOUNTER NOTE
Date: 2024-02-15
Provider: Dr. Fatima Al-Hassan, MD
Department: General Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
6-week history of progressive left breast redness, swelling, warmth, and persistent itching.

HISTORY OF PRESENT ILLNESS:
The patient with Type 2 diabetes, hypertension, hypothyroidism, peptic ulcer disease, and meningioma presents with breast complaints. She reports that the breast has become red, swollen, and warm to the touch. She denies any trauma or insect bites but notes a persistent itching sensation. She has not had any fever, chills, or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary Hyperaldosteronism (Conn's Syndrome) (diagnosed 2020-01-15)
- Restless Legs Syndrome (diagnosed 2020-04-15)
- Adhesive capsulitis of left shoulder (diagnosed 2020-10-15)
- Chronic mechanical lower back pain (diagnosed 2021-04-15)
- Trigeminal neuralgia (diagnosed 2022-02-15)
- Meningioma (diagnosed 2023-02-15)
- Type 2 diabetes mellitus
- Hypertension
- Hypothyroidism
- History of peptic ulcer disease

Her past medical history is significant for hypertension, controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Levothyroxine 75 mcg daily
- Lisinopril 20 mg daily
- Amlodipine 5 mg daily
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer diagnosed age 65
- Father: Type 2 diabetes mellitus
- Mother: Hypertension
- Sister: Hypothyroidism

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Administrative Assistant.

PHYSICAL EXAMINATION:
On physical examination, the left breast appears erythematous and edematous with a peau d&#x27;orange appearance. There is no nipple discharge, but palpation reveals a firm, irregular mass in the upper outer quadrant and palpable left axillary lymph nodes.

LABORATORY DATA:
Laboratory Values:
- Complete Blood Count (CBC)
- Hemoglobin: 13.2 g/dL
- Leukocytes: 7,800 /μL
- Platelets: 250,000 /μL
- Comprehensive Metabolic Panel (CMP)
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 0.9 mg/dL
- Calcium: 9.8 mg/dL
- Tumor markers
- CA 15-3: Elevated
- CEA: Normal

IMAGING:
Mammography shows skin thickening and an ill-defined mass.

OTHER STUDIES:
A core needle breast biopsy and full-thickness skin punch biopsy are performed.

## Encounter 7: source 2025-02-15, practice 2025-12-18, ed, Emergency Medicine

Chief complaint: Severe right upper quadrant pain radiating to right shoulder after fatty meal with nausea and vomiting.

ENCOUNTER NOTE
Date: 2025-02-15
Provider: Dr. David Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe right upper quadrant pain radiating to right shoulder after fatty meal with nausea and vomiting.

HISTORY OF PRESENT ILLNESS:
This 57-year-old female with type 2 diabetes mellitus, hypertension, and recent history of inflammatory breast carcinoma and meningioma presents to the emergency department with a 24-hour history of severe right upper quadrant (RUQ) pain that radiates to her right shoulder. She reports the pain began after eating a fatty meal and has been constant since onset. She also complains of nausea and has had one episode of vomiting.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary Hyperaldosteronism (Conn's Syndrome) (diagnosed 2020-01-15)
- Restless Legs Syndrome (diagnosed 2020-04-15)
- Adhesive capsulitis of left shoulder (diagnosed 2020-10-15)
- Chronic mechanical lower back pain (diagnosed 2021-04-15)
- Trigeminal neuralgia (diagnosed 2022-02-15)
- Meningioma (diagnosed 2023-02-15)
- Inflammatory breast carcinoma (diagnosed 2024-02-15)
- Type 2 diabetes mellitus
- Hypertension
- Hypothyroidism
- History of peptic ulcer disease

She has a past medical history of hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Levothyroxine 75 mcg daily
- Lisinopril 20 mg daily
- Amlodipine 5 mg daily
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer diagnosed age 65
- Father: Type 2 diabetes mellitus
- Mother: Hypertension
- Sister: Hypothyroidism

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Administrative Assistant.

VITAL SIGNS:
On examination, her temperature is 38.3°C (101°F), heart rate is 102 beats per minute, and blood pressure is 135/85 mmHg.

PHYSICAL EXAMINATION:
RUQ palpation elicits significant tenderness, and Murphy's sign is positive.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 14,000/uL (normal: 4,500-11,000/uL)
- Hemoglobin: 13.2 g/dL (normal: 12-16 g/dL)
- Platelets: 250,000/uL (normal: 150,000-450,000/uL)
- Aspartate aminotransferase (AST): 45 U/L (normal: 10-40 U/L)
- Alanine aminotransferase (ALT): 50 U/L (normal: 7-56 U/L)
- Alkaline phosphatase: 120 U/L (normal: 40-130 U/L)
- Total bilirubin: 1.2 mg/dL (normal: 0.3-1.2 mg/dL)

IMAGING:
An abdominal ultrasound shows gallbladder wall thickening and pericholecystic fluid without evidence of gallstones.

ASSESSMENT AND PLAN:
Based on the clinical presentation and test results,
