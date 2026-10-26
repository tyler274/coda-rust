# synthetic_hospital patient 2702

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 49 at first encounter, sex F, White, Blue Cross PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-01-24 (Mon) | outpatient | Endocrinology |
| 1 | 2020-07-15 | 2022-07-18 (Mon) | outpatient | Family Medicine |
| 2 | 2021-01-15 | 2023-01-16 (Mon) | outpatient | General Surgery |
| 3 | 2021-07-15 | 2023-07-26 (Wed) | outpatient | Breast Surgery |
| 4 | 2022-01-15 | 2024-01-15 (Mon) | outpatient | Breast Surgery |
| 5 | 2022-07-15 | 2024-07-18 (Thu) | procedure | General Surgery |
| 6 | 2023-07-15 | 2025-07-24 (Thu) | outpatient | Internal Medicine |
| 7 | 2024-07-15 | 2026-07-28 (Tue) | outpatient | Internal Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Latex (anaphylaxis)",
  "Penicillin (rash)"
 ],
 "insurance": "Blue Cross PPO",
 "occupation": "Librarian",
 "alcohol_use": "Social (1 drink per week)",
 "family_history": [
  "Mother: Invasive breast cancer (diagnosed age 62)",
  "Maternal aunt: Epithelial ovarian cancer (diagnosed age 55)",
  "Father: Hypertension, type 2 diabetes mellitus",
  "Sister: Depression"
 ],
 "race_ethnicity": "White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "1 mg PO daily",
   "name": "Anastrozole"
  },
  {
   "dose": "600 mg PO three times daily",
   "name": "Calcium carbonate"
  },
  {
   "dose": "2000 international units PO daily",
   "name": "Vitamin D3"
  }
 ],
 "surgical_history": [
  "Transsphenoidal resection of pituitary adenoma (age 48)",
  "Laparoscopic cholecystectomy (age 49, complicated by intraoperative anaphylaxis)",
  "Left breast lumpectomy with sentinel lymph node biopsy (age 49)",
  "Right breast microdochectomy (age 49)",
  "Excisional breast biopsy, benign (age 35)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Cushing disease (ACTH-secreting pituitary adenoma), status post transsphenoidal resection",
  "Invasive ductal carcinoma of the left breast, status post lumpectomy and radiation",
  "Pathologic nipple discharge of the right breast, status post duct excision",
  "Menopause (onset age 41, previously managed with HRT)"
 ],
 "age_at_first_encounter": 49
}
```

Primary diagnoses: ["Indication for breast cancer screening", "Invasive ductal carcinoma of left breast", "Pathologic nipple discharge concerning for malignancy", "ACTH-producing pituitary adenoma", "Cholelithiasis", "Anaphylaxis"]
Comorbidities: ["Essential hypertension", "Obesity", "Family history of breast cancer"]

## Encounter 0: source 2020-01-15, practice 2022-01-24, outpatient, Endocrinology

Chief complaint: Progressive weight gain, facial fullness, and new-onset hypertension over the past six months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive weight gain, facial fullness, and new-onset hypertension over the past six months

HISTORY OF PRESENT ILLNESS:
6-month history of progressive weight gain, facial fullness, and new-onset hypertension. She reports increased bruising, and her menstrual periods have become irregular.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Cushing disease (ACTH-secreting pituitary adenoma), status post transsphenoidal resection
- Invasive ductal carcinoma of the left breast, status post lumpectomy and radiation
- Pathologic nipple discharge of the right breast, status post duct excision
- Menopause (onset age 41, previously managed with HRT)

PAST SURGICAL HISTORY:
- Transsphenoidal resection of pituitary adenoma (age 48)
- Laparoscopic cholecystectomy (age 49, complicated by intraoperative anaphylaxis)
- Left breast lumpectomy with sentinel lymph node biopsy (age 49)
- Right breast microdochectomy (age 49)
- Excisional breast biopsy, benign (age 35)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Anastrozole 1 mg PO daily
- Calcium carbonate 600 mg PO three times daily
- Vitamin D3 2000 international units PO daily

ALLERGIES:
Latex (anaphylaxis), Penicillin (rash)

FAMILY HISTORY:
- Mother: Invasive breast cancer (diagnosed age 62)
- Maternal aunt: Epithelial ovarian cancer (diagnosed age 55)
- Father: Hypertension, type 2 diabetes mellitus
- Sister: Depression

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1 drink per week). Occupation: Librarian.

VITAL SIGNS:
Blood pressure is 155/95 mmHg.

PHYSICAL EXAMINATION:
On physical examination, she has a rounded face, supraclavicular fat pads, and a dorsocervical fat hump. Her skin appears thin with purple striae on the abdomen.

LABORATORY DATA:
Laboratory tests reveal:

- Serum glucose: 160 mg/dL
- Serum cortisol (8 AM): 25 µg/dL (normal 5-23 µg/dL)
- ACTH: Elevated
- 24-hour urinary free cortisol: Elevated

OTHER STUDIES:
A low-dose dexamethasone suppression test shows no suppression of cortisol levels.

## Encounter 1: source 2020-07-15, practice 2022-07-18, outpatient, Family Medicine

Chief complaint: Routine health examination with patient refusing mammogram screening due to health beliefs

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Thompson, MD
Department: Family Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health examination with patient refusing mammogram screening due to health beliefs

HISTORY OF PRESENT ILLNESS:
The patient returns for routine health examination six months following transsphenoidal resection for ACTH-secreting pituitary adenoma and continues surveillance for history of invasive ductal carcinoma of the left breast. She reports feeling well and has no complaints. She mentions that she has never had a mammogram because she believes that "if you look for illness, you will find it and make it real." She is accompanied by her daughter, who expresses concern about her mother's health beliefs and the risk of breast cancer.

PAST MEDICAL HISTORY:
Active Problem List:
- ACTH-producing pituitary adenoma (diagnosed 2020-01-15)
- Hypertension
- Cushing disease (ACTH-secreting pituitary adenoma), status post transsphenoidal resection
- Invasive ductal carcinoma of the left breast, status post lumpectomy and radiation
- Pathologic nipple discharge of the right breast, status post duct excision
- Menopause (onset age 41, previously managed with HRT)

She has no personal history of breast cancer or other significant illnesses. She is up to date with other age-appropriate screenings, including cervical cancer screening.

PAST SURGICAL HISTORY:
- Transsphenoidal resection of pituitary adenoma (age 48)
- Laparoscopic cholecystectomy (age 49, complicated by intraoperative anaphylaxis)
- Left breast lumpectomy with sentinel lymph node biopsy (age 49)
- Right breast microdochectomy (age 49)
- Excisional breast biopsy, benign (age 35)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Anastrozole 1 mg PO daily
- Calcium carbonate 600 mg PO three times daily
- Vitamin D3 2000 international units PO daily

ALLERGIES:
Latex (anaphylaxis), Penicillin (rash)

FAMILY HISTORY:
Her maternal aunt was diagnosed with breast cancer at the age of 47

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1 drink per week). Occupation: Librarian.

VITAL SIGNS:
Her vital signs are within normal limits. Her BMI is 22 kg/m^2

PHYSICAL EXAMINATION:
The patient's physical examination is unremarkable

LABORATORY DATA:
Complete Blood Count (CBC): Normal
- Lipid Profile: Normal
- Fasting Blood Glucose: 90 mg/dL

## Encounter 2: source 2021-01-15, practice 2023-01-16, outpatient, General Surgery

Chief complaint: Six-month history of fatigue and intermittent right upper quadrant abdominal pain associated with meals

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Maria Garcia, MD
Department: General Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of fatigue and intermittent right upper quadrant abdominal pain associated with meals

HISTORY OF PRESENT ILLNESS:
The patient with Cushing disease and breast cancer presents with right upper quadrant pain. She reports that her pain is primarily in the right upper quadrant and is associated with meals. She denies any changes in bowel habits, weight loss, or jaundice.

PAST MEDICAL HISTORY:
Active Problem List:
- ACTH-producing pituitary adenoma (diagnosed 2020-01-15)
- Indication for breast cancer screening (diagnosed 2020-07-15)
- Hypertension
- Cushing disease (ACTH-secreting pituitary adenoma), status post transsphenoidal resection
- Invasive ductal carcinoma of the left breast, status post lumpectomy and radiation
- Pathologic nipple discharge of the right breast, status post duct excision
- Menopause (onset age 41, previously managed with HRT)

Her past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Transsphenoidal resection of pituitary adenoma (age 48)
- Laparoscopic cholecystectomy (age 49, complicated by intraoperative anaphylaxis)
- Left breast lumpectomy with sentinel lymph node biopsy (age 49)
- Right breast microdochectomy (age 49)
- Excisional breast biopsy, benign (age 35)

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg PO daily
- Anastrozole 1 mg PO daily
- Calcium carbonate 600 mg PO three times daily
- Vitamin D3 2000 international units PO daily

ALLERGIES:
Latex (anaphylaxis), Penicillin (rash)

FAMILY HISTORY:
Her family history is notable for colorectal cancer in her father.

SOCIAL HISTORY:
She does not smoke or consume alcohol.

VITAL SIGNS:
On physical examination, her blood pressure is 135/85 mmHg, heart rate is 78 bpm, and BMI is 32 kg/m^2

PHYSICAL EXAMINATION:
Abdominal examination reveals mild tenderness to palpation in the right upper quadrant without rebound or guarding.

LABORATORY DATA:
- Hemoglobin: 13.2 g/dL
- White blood cell count: 7,500/mm^3
- Platelets: 250,000/mm^3
- Aspartate aminotransferase (AST): 45 U/L (N 10-40 U/L)
- Alanine aminotransferase (ALT): 50 U/L (N 7-56 U/L)
- Alkaline phosphatase: 130 U/L (N 40-120 U/L)
- Total bilirubin: 1.2 mg/dL (N 0.3-1.2 mg/dL)

IMAGING:
An ultrasound of the abdomen shows multiple gallstones without evidence of cholecystitis.

OTHER STUDIES:
The gastroenterologist plans to conduct a case-control study to investigate the association between gallstone disease and the consumption of a high-fat diet in African American women aged 40-50 years. To control for confounding variables, the gastroenterologist decides to match cases and controls on certain variables.

PLAN:
The patient is referred to a gastroenterologist for further evaluation of her abdominal pain.

## Encounter 3: source 2021-07-15, practice 2023-07-26, outpatient, Breast Surgery

Chief complaint: Palpable mass in left breast noticed one month ago without associated pain

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. David Kim, MD
Department: Breast Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Palpable mass in left breast noticed one month ago without associated pain

HISTORY OF PRESENT ILLNESS:
The patient with history of Cushing disease status post transsphenoidal resection, prior left breast invasive ductal carcinoma treated with lumpectomy and radiation, and current Anastrozole therapy returns to the Breast Surgery clinic for evaluation of a palpable left breast mass noticed one month ago. She reports no pain associated with the mass but is concerned because

PAST MEDICAL HISTORY:
Active Problem List:
- ACTH-producing pituitary adenoma (diagnosed 2020-01-15)
- Indication for breast cancer screening (diagnosed 2020-07-15)
- Cholelithiasis (diagnosed 2021-01-15)
- Hypertension
- Cushing disease (ACTH-secreting pituitary adenoma), status post transsphenoidal resection
- Invasive ductal carcinoma of the left breast, status post lumpectomy and radiation
- Pathologic nipple discharge of the right breast, status post duct excision
- Menopause (onset age 41, previously managed with HRT)

The patient's medical history is significant for hypertension,.

PAST SURGICAL HISTORY:
No previous surgeries.

MEDICATIONS:
She takes controlled with hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg PO daily
- Anastrozole 1 mg PO daily
- Calcium carbonate 600 mg PO three times daily
- Vitamin D3 2000 international units PO daily

ALLERGIES:
Latex (anaphylaxis), Penicillin (rash)

FAMILY HISTORY:
Her mother was diagnosed with breast cancer at the age of 47.

SOCIAL HISTORY:
She has no children and has never used hormone replacement therapy.

PHYSICAL EXAMINATION:
On physical examination, a 3 cm firm, non-tender, immobile mass is palpated in the upper outer quadrant of the left breast. There is no nipple discharge, skin changes, or lymphadenopathy.

LABORATORY DATA:
Laboratory Values:
- Complete Blood Count (CBC)
- Hemoglobin: 13.2 g/dL
- Leukocytes: 7.2 x 10^9/L
- Platelets: 255 x 10^9/L
- Liver Function Tests (LFTs)
- AST: 28 U/L
- ALT: 32 U/L
- Alkaline Phosphatase: 90 U/L
- Total Bilirubin: 0.8 mg/dL

IMAGING:
A mammogram shows a suspicious 3 cm spiculated mass in the left breast,
A staging workup, including a CT scan of the chest, abdomen, and pelvis, and a bone scan, reveals no evidence of distant metastasis. However, a 2 cm lymph node is noted in the left axilla on the CT scan.

PATHOLOGY:
An ultrasound-guided core needle biopsy confirms invasive ductal carcinoma.

## Encounter 4: source 2022-01-15, practice 2024-01-15, outpatient, Breast Surgery

Chief complaint: Three-month history of intermittent, unilateral, blood-tinged discharge from right nipple

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. David Kim, MD
Department: Breast Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of intermittent, unilateral, blood-tinged discharge from right nipple

HISTORY OF PRESENT ILLNESS:
Patient with left breast cancer history and right duct excision returns with 3-month history of intermittent, unilateral, blood-tinged discharge from right nipple. She denies associated breast pain, fever, or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- ACTH-producing pituitary adenoma (diagnosed 2020-01-15)
- Indication for breast cancer screening (diagnosed 2020-07-15)
- Cholelithiasis (diagnosed 2021-01-15)
- Invasive ductal carcinoma of left breast (diagnosed 2021-07-15)
- Hypertension
- Cushing disease (ACTH-secreting pituitary adenoma), status post transsphenoidal resection
- Invasive ductal carcinoma of the left breast, status post lumpectomy and radiation
- Pathologic nipple discharge of the right breast, status post duct excision
- Menopause (onset age 41, previously managed with HRT)

Her past medical history is significant for hypertension, controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Transsphenoidal resection of pituitary adenoma (age 48)
- Laparoscopic cholecystectomy (age 49, complicated by intraoperative anaphylaxis)
- Left breast lumpectomy with sentinel lymph node biopsy (age 49)
- Right breast microdochectomy (age 49)
- Excisional breast biopsy, benign (age 35)

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg PO daily
- Anastrozole 1 mg PO daily
- Calcium carbonate 600 mg PO three times daily
- Vitamin D3 2000 international units PO daily

ALLERGIES:
Latex (anaphylaxis), Penicillin (rash)

FAMILY HISTORY:
She has no family history of breast or ovarian cancer. However, the patient is concerned because her mother had postmenopausal breast cancer.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1 drink per week). Occupation: Librarian.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, there is no palpable mass, skin changes, or lymphadenopathy.

IMAGING:
Her last mammogram was 18 months ago and was normal.

## Encounter 5: source 2022-07-15, practice 2024-07-18, procedure, General Surgery

Chief complaint: Intraoperative hypotension, tachycardia, and wheezing during elective cholecystectomy

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Maria Garcia, MD
Department: General Surgery
Visit Type: procedure

CHIEF COMPLAINT:
Intraoperative hypotension, tachycardia, and wheezing during elective cholecystectomy

HISTORY OF PRESENT ILLNESS:
The patient returns for elective cholecystectomy 18 months following her diagnosis of cholelithiasis, amid ongoing management of hypertension, status post transsphenoidal resection for Cushing disease, and recent treatment for bilateral breast pathology. Approximately 30 minutes into the procedure, the anesthesiologist notes that the patient's blood pressure has dropped to 85/50 mmHg from a baseline of 130/80 mmHg, and her heart rate has increased to 110 bpm. The patient is also noted to have wheezing on auscultation. The surgical team is alerted, and upon inspection under the surgical drapes, widespread urticaria is observed. The patient's oxygen saturation has decreased to 92% on FiO2 of 0.5.

PAST MEDICAL HISTORY:
Active Problem List:
- ACTH-producing pituitary adenoma (diagnosed 2020-01-15)
- Indication for breast cancer screening (diagnosed 2020-07-15)
- Cholelithiasis (diagnosed 2021-01-15)
- Invasive ductal carcinoma of left breast (diagnosed 2021-07-15)
- Pathologic nipple discharge concerning for malignancy (diagnosed 2022-01-15)
- Hypertension
- Cushing disease (ACTH-secreting pituitary adenoma), status post transsphenoidal resection
- Invasive ductal carcinoma of the left breast, status post lumpectomy and radiation
- Pathologic nipple discharge of the right breast, status post duct excision
- Menopause (onset age 41, previously managed with HRT)

The patient has no known history of cardiac disease.

PAST SURGICAL HISTORY:
- Transsphenoidal resection of pituitary adenoma (age 48)
- Laparoscopic cholecystectomy (age 49, complicated by intraoperative anaphylaxis)
- Left breast lumpectomy with sentinel lymph node biopsy (age 49)
- Right breast microdochectomy (age 49)
- Excisional breast biopsy, benign (age 35)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Anastrozole 1 mg PO daily
- Calcium carbonate 600 mg PO three times daily
- Vitamin D3 2000 international units PO daily

ALLERGIES:
Penicillin allergy

FAMILY HISTORY:
- Mother: Invasive breast cancer (diagnosed age 62)
- Maternal aunt: Epithelial ovarian cancer (diagnosed age 55)
- Father: Hypertension, type 2 diabetes mellitus
- Sister: Depression

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1 drink per week). Occupation: Librarian.

LABORATORY DATA:
Preoperative labs, including complete blood count, basic metabolic panel, and coagulation studies, were within normal limits

## Encounter 6: source 2023-07-15, practice 2025-07-24, outpatient, Internal Medicine

Chief complaint: Routine health maintenance visit for chronic disease surveillance

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Aisha Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health maintenance visit for chronic disease surveillance

HISTORY OF PRESENT ILLNESS:
The patient returns for routine health maintenance and chronic disease surveillance following transsphenoidal resection, left breast lumpectomy with radiation, and right breast duct excision. She has no complaints and is feeling well.

PAST MEDICAL HISTORY:
Active Problem List:
- ACTH-producing pituitary adenoma (diagnosed 2020-01-15)
- Indication for breast cancer screening (diagnosed 2020-07-15)
- Cholelithiasis (diagnosed 2021-01-15)
- Invasive ductal carcinoma of left breast (diagnosed 2021-07-15)
- Pathologic nipple discharge concerning for malignancy (diagnosed 2022-01-15)
- Anaphylaxis (diagnosed 2022-07-15)
- Hypertension
- Cushing disease (ACTH-secreting pituitary adenoma), status post transsphenoidal resection
- Invasive ductal carcinoma of the left breast, status post lumpectomy and radiation
- Pathologic nipple discharge of the right breast, status post duct excision
- Menopause (onset age 41, previously managed with HRT)

Her medical history is significant for hypertension, which is well-controlled on hydrochlorothiazide. She has never been pregnant.

PAST SURGICAL HISTORY:
- Transsphenoidal resection of pituitary adenoma (age 48)
- Laparoscopic cholecystectomy (age 49, complicated by intraoperative anaphylaxis)
- Left breast lumpectomy with sentinel lymph node biopsy (age 49)
- Right breast microdochectomy (age 49)
- Excisional breast biopsy, benign (age 35)

MEDICATIONS:
She has been taking hormone replacement therapy for menopausal symptoms for the past 8 years.

Home Medications:
- Lisinopril 10 mg PO daily
- Anastrozole 1 mg PO daily
- Calcium carbonate 600 mg PO three times daily
- Vitamin D3 2000 international units PO daily

ALLERGIES:
Latex (anaphylaxis), Penicillin (rash)

FAMILY HISTORY:
Her mother was diagnosed with breast cancer at the age of 67.

SOCIAL HISTORY:
She does not smoke and drinks alcohol socially.

VITAL SIGNS:
She has a body mass index (BMI) of 32 kg/m^2.

PHYSICAL EXAMINATION:
On physical examination, her blood pressure is 128/78 mmHg, heart rate is 72 beats per minute, and the rest of the exam is unremarkable.

IMAGING:
Her last mammogram was at age 45, which was normal.

## Encounter 7: source 2024-07-15, practice 2026-07-28, outpatient, Internal Medicine

Chief complaint: Routine health maintenance examination with inquiry about mammogram screening intervals

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Aisha Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health maintenance examination with inquiry about mammogram screening intervals

HISTORY OF PRESENT ILLNESS:
She returns for health maintenance with history of breast cancer. She has no complaints and is feeling well. She inquires about whether she should have another mammogram at this time.

PAST MEDICAL HISTORY:
Active Problem List:
- ACTH-producing pituitary adenoma (diagnosed 2020-01-15)
- Indication for breast cancer screening (diagnosed 2020-07-15)
- Cholelithiasis (diagnosed 2021-01-15)
- Invasive ductal carcinoma of left breast (diagnosed 2021-07-15)
- Pathologic nipple discharge concerning for malignancy (diagnosed 2022-01-15)
- Anaphylaxis (diagnosed 2022-07-15)
- Hypertension
- Cushing disease (ACTH-secreting pituitary adenoma), status post transsphenoidal resection
- Invasive ductal carcinoma of the left breast, status post lumpectomy and radiation
- Pathologic nipple discharge of the right breast, status post duct excision
- Menopause (onset age 41, previously managed with HRT)

Her last mammogram was at age 40, which was normal.

PAST SURGICAL HISTORY:
- Transsphenoidal resection of pituitary adenoma (age 48)
- Laparoscopic cholecystectomy (age 49, complicated by intraoperative anaphylaxis)
- Left breast lumpectomy with sentinel lymph node biopsy (age 49)
- Right breast microdochectomy (age 49)
- Excisional breast biopsy, benign (age 35)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Anastrozole 1 mg PO daily
- Calcium carbonate 600 mg PO three times daily
- Vitamin D3 2000 international units PO daily

ALLERGIES:
Latex (anaphylaxis), Penicillin (rash)

FAMILY HISTORY:
She has no family history of breast cancer.

SOCIAL HISTORY:
She exercises regularly, does not smoke, and has a body mass index of 24 kg/m^2.

VITAL SIGNS:
Blood pressure is 128/78 mmHg, heart rate is 72 beats per minute

PHYSICAL EXAMINATION:
Breast examination is unremarkable with no palpable masses or axillary lymphadenopathy.
