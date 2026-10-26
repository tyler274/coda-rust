# synthetic_hospital patient 2852

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 62 at first encounter, sex F, Non-Hispanic White, Medicare Advantage (Aetna).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-09-27 (Mon) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-03-15 | 2021-11-16 (Tue) | outpatient | Psychiatry |
| 2 | 2021-03-15 | 2022-11-21 (Mon) | outpatient | Psychiatry |
| 3 | 2022-03-15 | 2023-11-29 (Wed) | ed | Emergency Medicine |
| 4 | 2023-03-15 | 2024-11-26 (Tue) | ed | Emergency Medicine |
| 5 | 2023-09-15 | 2025-05-26 (Mon) | ed | Emergency Medicine |
| 6 | 2024-03-15 | 2025-11-21 (Fri) | ed | Emergency Medicine |
| 7 | 2024-09-15 | 2026-06-01 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare Advantage (Aetna)",
 "occupation": "Retired elementary school teacher",
 "alcohol_use": "Social, 1-2 glasses of wine weekly",
 "family_history": [
  "Mother: Breast cancer diagnosed age 45, deceased age 70",
  "Maternal aunt: Ovarian cancer diagnosed age 52, status living",
  "Father: Type 2 diabetes mellitus, hypertension, alive age 85",
  "Sister: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 15 years ago (20 pack-year history)",
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
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "2000 IU PO daily",
   "name": "Cholecalciferol (Vitamin D3)"
  }
 ],
 "surgical_history": [
  "Left breast lumpectomy with sentinel lymph node biopsy and adjuvant radiation (age 55)",
  "Right total mastectomy with axillary lymph node dissection (age 57)",
  "Completion left total mastectomy (age 61)",
  "Total abdominal hysterectomy with bilateral salpingo-oophorectomy (age 62)"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus (diagnosed age 55)",
  "Hypertension",
  "Major depressive disorder, recurrent, moderate",
  "Chronic insomnia disorder",
  "History of invasive breast carcinoma (bilateral, ER+/PR+, HER2-negative)",
  "History of uterine sarcoma (high-grade endometrial stromal sarcoma)"
 ],
 "age_at_first_encounter": 62
}
```

Primary diagnoses: ["Major depressive disorder", "Malignant epidural spinal cord compression", "Uterine sarcoma", "Anticholinergic toxicity from over-the-counter medication use", "Metastatic carcinoma to bone", "Malignancy-associated hypercalcemia"]
Comorbidities: ["History of breast cancer", "Status post lumpectomy of breast", "Status post mastectomy", "Status post mastectomy", "Iatrogenic immunosuppression"]

## Encounter 0: source 2020-01-15, practice 2021-09-27, outpatient, Obstetrics & Gynecology

Chief complaint: 3-month history of intermittent vaginal bleeding with increasing pelvic pressure and occasional constipation

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of intermittent vaginal bleeding with increasing pelvic pressure and occasional constipation

HISTORY OF PRESENT ILLNESS:
She reports that the bleeding is not associated with pain but has been experiencing increasing pelvic pressure and occasional constipation.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed age 55)
- Hypertension
- Major depressive disorder, recurrent, moderate
- Chronic insomnia disorder
- History of invasive breast carcinoma (bilateral, ER+/PR+, HER2-negative)
- History of uterine sarcoma (high-grade endometrial stromal sarcoma)

Her past medical history is significant for breast cancer treated with lumpectomy and radiation therapy followed by 5 years of tamoxifen therapy, which she completed 2 years ago.

PAST SURGICAL HISTORY:
- Left breast lumpectomy with sentinel lymph node biopsy and adjuvant radiation (age 55)
- Right total mastectomy with axillary lymph node dissection (age 57)
- Completion left total mastectomy (age 61)
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (age 62)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily
- Atorvastatin 40 mg PO daily
- Cholecalciferol (Vitamin D3) 2000 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer diagnosed age 45, deceased age 70
- Maternal aunt: Ovarian cancer diagnosed age 52, status living
- Father: Type 2 diabetes mellitus, hypertension, alive age 85
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, her abdomen is non-distended, non-tender, and a firm, irregular mass is palpated on bimanual pelvic examination. The uterus appears enlarged and nodular.

IMAGING:
A transvaginal ultrasound reveals a 5 cm heterogeneous mass within the myometrium.

PATHOLOGY:
Endometrial biopsy is performed, and the pathology report indicates a high-grade neoplasm with spindle cells suggestive of sarcoma.

## Encounter 1: source 2020-03-15, practice 2021-11-16, outpatient, Psychiatry

Chief complaint: 6-month history of persistent sadness, loss of interest in activities, fatigue, and 10-pound weight loss following recent breast cancer treatment

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. James Morrison, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of persistent sadness, loss of interest in activities, fatigue, and 10-pound weight loss following recent breast cancer treatment

HISTORY OF PRESENT ILLNESS:
The patient presents for psychiatric evaluation following recent uterine sarcoma and history of breast cancer. She reports that these symptoms began shortly after she was diagnosed with breast cancer, for which she underwent a mastectomy and completed chemotherapy three months ago. She denies any suicidal ideation.

PAST MEDICAL HISTORY:
Active Problem List:
- Uterine sarcoma (diagnosed 2020-01-15)
- Type 2 diabetes mellitus (diagnosed age 55)
- Hypertension
- Major depressive disorder, recurrent, moderate
- Chronic insomnia disorder
- History of invasive breast carcinoma (bilateral, ER+/PR+, HER2-negative)
- History of uterine sarcoma (high-grade endometrial stromal sarcoma)

PAST SURGICAL HISTORY:
- Left breast lumpectomy with sentinel lymph node biopsy and adjuvant radiation (age 55)
- Right total mastectomy with axillary lymph node dissection (age 57)
- Completion left total mastectomy (age 61)
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (age 62)

MEDICATIONS:
Her current medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Cholecalciferol (Vitamin D3) 2000 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer diagnosed age 45, deceased age 70
- Maternal aunt: Ovarian cancer diagnosed age 52, status living
- Father: Type 2 diabetes mellitus, hypertension, alive age 85
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Retired elementary school teacher.

VITAL SIGNS:
On physical examination, her blood pressure is 132/86 mmHg, heart rate is 78 beats per minute, and BMI is 29 kg/m^2.

PHYSICAL EXAMINATION:
She appears tearful but is oriented to time, place, and person.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin A1c: 7.2%
- Serum creatinine: 1.0 mg/dL
- Fasting glucose: 154 mg/dL
- Total cholesterol: 190 mg/dL
- LDL cholesterol: 100 mg/dL
- HDL cholesterol: 45 mg/dL

## Encounter 2: source 2021-03-15, practice 2022-11-21, outpatient, Psychiatry

Chief complaint: 3-week history of difficulty sleeping, decreased appetite, fatigue, and feelings of worthlessness with recurrent thoughts of death

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. James Morrison, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
3-week history of difficulty sleeping, decreased appetite, fatigue, and feelings of worthlessness with recurrent thoughts of death

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of major depressive disorder with history of recurrent episodes following breast carcinoma and uterine sarcoma treatment. Since her prior psychiatric evaluation, she reports feeling "down" most of the day, nearly every day, and no longer enjoys previously pleasurable activities like gardening and spending time with grandchildren. She expresses feelings of worthlessness and recurrent thoughts of death but denies specific suicidal plans. She has lost 5 kg in the past month unintentionally.

PAST MEDICAL HISTORY:
Active Problem List:
- Uterine sarcoma (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-03-15)
- Type 2 diabetes mellitus (diagnosed age 55)
- Hypertension
- Major depressive disorder, recurrent, moderate
- Chronic insomnia disorder
- History of invasive breast carcinoma (bilateral, ER+/PR+, HER2-negative)
- History of uterine sarcoma (high-grade endometrial stromal sarcoma)

PAST SURGICAL HISTORY:
- Left breast lumpectomy with sentinel lymph node biopsy and adjuvant radiation (age 55)
- Right total mastectomy with axillary lymph node dissection (age 57)
- Completion left total mastectomy (age 61)
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (age 62)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily
- Atorvastatin 40 mg PO daily
- Cholecalciferol (Vitamin D3) 2000 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer diagnosed age 45, deceased age 70
- Maternal aunt: Ovarian cancer diagnosed age 52, status living
- Father: Type 2 diabetes mellitus, hypertension, alive age 85
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Retired elementary school teacher.

VITAL SIGNS:
Vital signs are within normal limits

PHYSICAL EXAMINATION:
Physical examination is unremarkable

LABORATORY DATA:
Laboratory tests reveal:
- Hemoglobin: 12.5 g/dL (normal: 12-16 g/dL)
- White blood cell count: 7,000/mm³ (normal: 4,500-11,000/mm³)
- Platelets: 250,000/mm³ (normal: 150,000-450,000/mm³)
- Thyroid-stimulating hormone (TSH): 2.5 mIU/L (normal: 0.4-4.0 mIU/L)

ASSESSMENT AND PLAN:
Given the patient's history and presentation,

## Encounter 3: source 2022-03-15, practice 2023-11-29, ed, Emergency Medicine

Chief complaint: Acute confusion and agitation reported by daughter

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Maria Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute confusion and agitation reported by daughter

HISTORY OF PRESENT ILLNESS:
The patient with chronic insomnia disorder presents one year after psychiatric evaluation. The daughter reports that her mother has been experiencing worsening insomnia over the past week and has been taking increasing amounts of an over-the-counter sleep aid.

PAST MEDICAL HISTORY:
Active Problem List:
- Uterine sarcoma (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-03-15)
- Type 2 diabetes mellitus (diagnosed age 55)
- Hypertension
- Major depressive disorder, recurrent, moderate
- Chronic insomnia disorder
- History of invasive breast carcinoma (bilateral, ER+/PR+, HER2-negative)
- History of uterine sarcoma (high-grade endometrial stromal sarcoma)

PAST SURGICAL HISTORY:
- Left breast lumpectomy with sentinel lymph node biopsy and adjuvant radiation (age 55)
- Right total mastectomy with axillary lymph node dissection (age 57)
- Completion left total mastectomy (age 61)
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (age 62)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily
- Atorvastatin 40 mg PO daily
- Cholecalciferol (Vitamin D3) 2000 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer diagnosed age 45, deceased age 70
- Maternal aunt: Ovarian cancer diagnosed age 52, status living
- Father: Type 2 diabetes mellitus, hypertension, alive age 85
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Retired elementary school teacher.

VITAL SIGNS:
Temperature of 38.9°C (102°F), heart rate is 110/min, blood pressure is 145/90 mmHg, and respiratory rate is 22/min

PHYSICAL EXAMINATION:
The patient's skin is warm and dry. Pupils are dilated and react sluggishly to light. The oral mucosa is dry, and bowel sounds are diminished. The patient is unable to provide a urine sample due to difficulty voiding. No focal neurological deficits are noted.

LABORATORY DATA:
Serum sodium: 140 mEq/L
Serum potassium: 4.2 mEq/L
Serum creatinine: 1.0 mg/dL
Serum calcium: 9.8 mg/dL
Complete blood count: within normal limits
Urinalysis: pending

## Encounter 4: source 2023-03-15, practice 2024-11-26, ed, Emergency Medicine

Chief complaint: One-week history of confusion, constipation, and generalized weakness with one month of progressive lethargy

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. David Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
One-week history of confusion, constipation, and generalized weakness with one month of progressive lethargy

HISTORY OF PRESENT ILLNESS:
This 65-year-old female with history of breast carcinoma, uterine sarcoma, and prior ED visit for confusion presents to the emergency department with one-week history of confusion, constipation, and generalized weakness. Her family reports that she has been less active and more lethargic over the past month.

PAST MEDICAL HISTORY:
Active Problem List:
- Uterine sarcoma (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-03-15)
- Anticholinergic toxicity from over-the-counter medication use (diagnosed 2022-03-15)
- Type 2 diabetes mellitus (diagnosed age 55)
- Hypertension
- Major depressive disorder, recurrent, moderate
- Chronic insomnia disorder
- History of invasive breast carcinoma (bilateral, ER+/PR+, HER2-negative)
- History of uterine sarcoma (high-grade endometrial stromal sarcoma)

She has a past medical history of breast cancer treated with mastectomy and adjuvant chemotherapy two years ago, with no evidence of recurrence on her last follow-up 6 months prior.

PAST SURGICAL HISTORY:
- Left breast lumpectomy with sentinel lymph node biopsy and adjuvant radiation (age 55)
- Right total mastectomy with axillary lymph node dissection (age 57)
- Completion left total mastectomy (age 61)
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (age 62)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily
- Atorvastatin 40 mg PO daily
- Cholecalciferol (Vitamin D3) 2000 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer diagnosed age 45, deceased age 70
- Maternal aunt: Ovarian cancer diagnosed age 52, status living
- Father: Type 2 diabetes mellitus, hypertension, alive age 85
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Retired elementary school teacher.

REVIEW OF SYSTEMS:
She denies any fever, cough, or urinary symptoms.

PHYSICAL EXAMINATION:
On physical examination, she is oriented to person but not to time or place. Her abdomen is non-tender, and there is no peripheral edema. A neurological examination is non-focal.

LABORATORY DATA:
Initial laboratory results are as follows:

- Serum calcium: 14.2 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum creatinine: 1.2 mg/dL (normal: 0.6-1.1 mg/dL)
- Serum albumin: 4.0 g/dL (normal: 3.5-5.0 g/dL)
- Parathyroid hormone (PTH): <10 pg/mL (normal: 15-65 pg/mL)
- Parathyroid hormone-related protein (PTHrP): Pending

## Encounter 5: source 2023-09-15, practice 2025-05-26, ed, Emergency Medicine

Chief complaint: One-week history of confusion, constipation, and polyuria

ENCOUNTER NOTE
Date: 2023-09-15
Provider: Dr. David Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
One-week history of confusion, constipation, and polyuria

HISTORY OF PRESENT ILLNESS:
Patient with uterine sarcoma returns to the emergency department after prior hypercalcemia, with one-week history of confusion, constipation, and polyuria.

PAST MEDICAL HISTORY:
Active Problem List:
- Uterine sarcoma (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-03-15)
- Anticholinergic toxicity from over-the-counter medication use (diagnosed 2022-03-15)
- Malignancy-associated hypercalcemia (diagnosed 2023-03-15)
- Type 2 diabetes mellitus (diagnosed age 55)
- Hypertension
- Major depressive disorder, recurrent, moderate
- Chronic insomnia disorder
- History of invasive breast carcinoma (bilateral, ER+/PR+, HER2-negative)
- History of uterine sarcoma (high-grade endometrial stromal sarcoma)

She has a past medical history of breast cancer treated with mastectomy and chemotherapy two years ago, currently in remission.

PAST SURGICAL HISTORY:
- Left breast lumpectomy with sentinel lymph node biopsy and adjuvant radiation (age 55)
- Right total mastectomy with axillary lymph node dissection (age 57)
- Completion left total mastectomy (age 61)
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (age 62)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily
- Atorvastatin 40 mg PO daily
- Cholecalciferol (Vitamin D3) 2000 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer diagnosed age 45, deceased age 70
- Maternal aunt: Ovarian cancer diagnosed age 52, status living
- Father: Type 2 diabetes mellitus, hypertension, alive age 85
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Retired elementary school teacher.

REVIEW OF SYSTEMS:
She denies any fever, cough, or other respiratory symptoms.

PHYSICAL EXAMINATION:
On physical examination, she is lethargic but oriented to person and place, not to time. Her abdomen is non-tender, and there is no peripheral edema. A neurological examination is non-focal.

LABORATORY DATA:
Laboratory studies reveal:

- Serum calcium: 14.2 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum creatinine: 1.2 mg/dL (normal: 0.6-1.1 mg/dL)
- Serum phosphorus: 2.8 mg/dL (normal: 2.5-4.5 mg/dL)
- Serum PTH: <10 pg/mL (normal: 15-65 pg/mL)
- Serum 25-hydroxyvitamin D: 30 ng/mL (normal: 20-50 ng/mL)
- Serum 1,25-dihydroxyvitamin D: 45 pg/mL (normal: 18-72 pg/mL)

## Encounter 6: source 2024-03-15, practice 2025-11-21, ed, Emergency Medicine

Chief complaint: Two-week history of worsening mid-thoracic back pain with bilateral lower-extremity weakness and numbness below the level of the umbilicus

ENCOUNTER NOTE
Date: 2024-03-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-week history of worsening mid-thoracic back pain with bilateral lower-extremity weakness and numbness below the level of the umbilicus

HISTORY OF PRESENT ILLNESS:
This 66-year-old female with history of bilateral breast carcinoma and uterine sarcoma presents with two weeks of worsening mid-thoracic back pain. She describes constant, sharp pain exacerbated by lying down, with difficulty walking, bilateral lower-extremity weakness, and numbness below her belly button. She denies recent trauma or injury.

PAST MEDICAL HISTORY:
Active Problem List:
- Uterine sarcoma (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-03-15)
- Anticholinergic toxicity from over-the-counter medication use (diagnosed 2022-03-15)
- Malignancy-associated hypercalcemia (diagnosed 2023-03-15)
- Type 2 diabetes mellitus (diagnosed age 55)
- Hypertension
- Major depressive disorder, recurrent, moderate
- Chronic insomnia disorder
- History of invasive breast carcinoma (bilateral, ER+/PR+, HER2-negative)
- History of uterine sarcoma (high-grade endometrial stromal sarcoma)

PAST SURGICAL HISTORY:
- Left breast lumpectomy with sentinel lymph node biopsy and adjuvant radiation (age 55)
- Right total mastectomy with axillary lymph node dissection (age 57)
- Completion left total mastectomy (age 61)
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (age 62)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily
- Atorvastatin 40 mg PO daily
- Cholecalciferol (Vitamin D3) 2000 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer diagnosed age 45, deceased age 70
- Maternal aunt: Ovarian cancer diagnosed age 52, status living
- Father: Type 2 diabetes mellitus, hypertension, alive age 85
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, she has tenderness to palpation over the T7 vertebra, decreased strength in both lower extremities, and diminished sensation to light touch and pinprick from the T10 dermatome downwards. Her reflexes are diminished in the lower extremities, and she has a positive Babinski sign bilaterally. Bowel and bladder function are reported to be normal.

LABORATORY DATA:
Laboratory Values:
- Hemoglobin: 12.5 g/dL
- White blood cell count: 7.2 x 10^3/uL
- Platelets: 250 x 10^3/uL
- Calcium: 9.8 mg/dL
- Alkaline phosphatase: 120 U/L
- Total protein: 7.1 g/dL
- Albumin: 4.2 g/dL

IMAGING:
An urgent MRI of the thoracic spine reveals an epidural mass compressing the spinal cord at the T7 level.

## Encounter 7: source 2024-09-15, practice 2026-06-01, ed, Emergency Medicine

Chief complaint: Severe, progressive lower back pain over three weeks with difficulty walking and decreased sensation in the legs

ENCOUNTER NOTE
Date: 2024-09-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe, progressive lower back pain over three weeks with difficulty walking and decreased sensation in the legs

HISTORY OF PRESENT ILLNESS:
This 66-year-old female with a history of uterine sarcoma and bilateral breast carcinoma returns to the emergency department six months after prior evaluation for metastatic carcinoma to bone with thoracic spine involvement, now with severe, progressive lower back pain over the past three weeks. She describes the pain as constant, worse at night, and not relieved by over-the-counter analgesics. She also reports difficulty walking and has noticed a decreased sensation in her legs. She denies any recent trauma or injury. Her bowel and bladder function have been normal.

PAST MEDICAL HISTORY:
Active Problem List:
- Uterine sarcoma (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-03-15)
- Anticholinergic toxicity from over-the-counter medication use (diagnosed 2022-03-15)
- Malignancy-associated hypercalcemia (diagnosed 2023-03-15)
- Metastatic carcinoma to bone (diagnosed 2024-03-15)
- Type 2 diabetes mellitus (diagnosed age 55)
- Hypertension
- Major depressive disorder, recurrent, moderate
- Chronic insomnia disorder
- History of invasive breast carcinoma (bilateral, ER+/PR+, HER2-negative)
- History of uterine sarcoma (high-grade endometrial stromal sarcoma)

PAST SURGICAL HISTORY:
- Left breast lumpectomy with sentinel lymph node biopsy and adjuvant radiation (age 55)
- Right total mastectomy with axillary lymph node dissection (age 57)
- Completion left total mastectomy (age 61)
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (age 62)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily
- Atorvastatin 40 mg PO daily
- Cholecalciferol (Vitamin D3) 2000 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer diagnosed age 45, deceased age 70
- Maternal aunt: Ovarian cancer diagnosed age 52, status living
- Father: Type 2 diabetes mellitus, hypertension, alive age 85
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
On examination, she has weakness in both lower extremities, decreased sensation to light touch below the level of the umbilicus, and hyperreflexia.

LABORATORY DATA:
Her blood work returns with the following:

- Hemoglobin: 12.1 g/dL
- White blood cell count: 7.2 x 10^3/uL
- Platelets: 255 x 10^3/uL
- Calcium: 10.2 mg/dL
- Alkaline phosphatase: 120 U/L

IMAGING:
An MRI of the spine is ordered, and while awaiting the results
