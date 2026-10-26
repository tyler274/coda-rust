# synthetic_hospital patient 2751

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 57 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield (PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-02-27 (Mon) | outpatient | General Surgery |
| 1 | 2020-04-15 | 2023-06-07 (Wed) | outpatient | Obstetrics & Gynecology |
| 2 | 2020-07-15 | 2023-09-04 (Mon) | outpatient | Psychiatry |
| 3 | 2020-10-15 | 2023-12-15 (Fri) | outpatient | Obstetrics & Gynecology |
| 4 | 2021-07-15 | 2024-09-09 (Mon) | outpatient | Oncology |
| 5 | 2022-01-15 | 2025-03-10 (Mon) | outpatient | Neurology |
| 6 | 2022-03-15 | 2025-05-13 (Tue) | outpatient | Gastroenterology |
| 7 | 2023-01-15 | 2026-02-27 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield (PPO)",
 "occupation": "Retired administrative assistant",
 "alcohol_use": "Social (1-2 glasses of wine per month)",
 "family_history": [
  "Mother: Breast cancer (diagnosed age 45, deceased)",
  "Maternal aunt: Ovarian cancer",
  "Father: Hypertension, type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "20 mg PO daily",
   "name": "Tamoxifen"
  },
  {
   "dose": "1 mg PO daily (estrogen-only HRT, discontinued progesterone 2 months ago due to mood swings)",
   "name": "Estradiol"
  },
  {
   "dose": "600 mg/400 IU PO BID",
   "name": "Calcium carbonate + Vitamin D3"
  },
  {
   "dose": "400 mg PO Q8H PRN pain",
   "name": "Ibuprofen"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Multivitamin"
  }
 ],
 "surgical_history": [
  "Left breast lumpectomy with sentinel lymph node biopsy and adjuvant chemotherapy (age 52)",
  "Right breast lumpectomy with adjuvant radiation therapy (age 55)",
  "Modified radical mastectomy with axillary lymph node dissection, right (age 56)",
  "Implantable port placement and removal (age 56)"
 ],
 "chronic_conditions": [
  "Stage IIA estrogen receptor-positive breast cancer (on adjuvant endocrine therapy)",
  "History of left breast Stage II invasive ductal carcinoma (s/p lumpectomy and chemotherapy 5 years prior)",
  "Status post modified radical mastectomy with axillary lymph node dissection (right breast)",
  "Metastatic breast carcinoma to liver and adrenal glands",
  "Complex atypical endometrial hyperplasia",
  "Major depressive disorder, recurrent moderate",
  "Chemotherapy-induced peripheral neuropathy (metronidazole-associated)",
  "Thoracic outlet syndrome, post-surgical",
  "Secondary adrenal insufficiency",
  "Chronic cancer-related pain syndrome",
  "Perimenopausal syndrome",
  "Osteopenia"
 ],
 "age_at_first_encounter": 57
}
```

Primary diagnoses: ["Complex atypical endometrial hyperplasia", "Endometrial Hyperplasia", "Moderate cancer-related pain", "Secondary malignant neoplasm of liver", "Adrenal metastasis from breast cancer", "Thoracic outlet syndrome", "Metronidazole-induced polyneuropathy", "Major depressive disorder"]
Comorbidities: ["History of breast cancer", "Status post lumpectomy of breast", "Status post mastectomy", "Systemic menopausal syndrome", "Iatrogenic immunosuppression", "Status post laparoscopic cholecystectomy"]

## Encounter 0: source 2020-01-15, practice 2023-02-27, outpatient, General Surgery

Chief complaint: Routine follow-up appointment with occasional upper back discomfort and difficulty with overhead activities

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Jennifer Walsh, MD
Department: General Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up appointment with occasional upper back discomfort and difficulty with overhead activities

HISTORY OF PRESENT ILLNESS:
She appears well and reports that she has been recovering steadily since undergoing a modified radical mastectomy for breast cancer approximately one year ago. She denies pain or swelling in the chest wall but notes occasional discomfort in her upper back and some difficulty with overhead activities, such as placing objects on high shelves.

PAST MEDICAL HISTORY:
Active Problem List:
- Stage IIA estrogen receptor-positive breast cancer (on adjuvant endocrine therapy)
- History of left breast Stage II invasive ductal carcinoma (s/p lumpectomy and chemotherapy 5 years prior)
- Status post modified radical mastectomy with axillary lymph node dissection (right breast)
- Metastatic breast carcinoma to liver and adrenal glands
- Complex atypical endometrial hyperplasia
- Major depressive disorder, recurrent moderate
- Chemotherapy-induced peripheral neuropathy (metronidazole-associated)
- Thoracic outlet syndrome, post-surgical
- Secondary adrenal insufficiency
- Chronic cancer-related pain syndrome
- Perimenopausal syndrome
- Osteopenia

PAST SURGICAL HISTORY:
- Left breast lumpectomy with sentinel lymph node biopsy and adjuvant chemotherapy (age 52)
- Right breast lumpectomy with adjuvant radiation therapy (age 55)
- Modified radical mastectomy with axillary lymph node dissection, right (age 56)
- Implantable port placement and removal (age 56)

MEDICATIONS:
Home Medications:
- Tamoxifen 20 mg PO daily
- Estradiol 1 mg PO daily (estrogen-only HRT, discontinued progesterone 2 months ago due to mood swings)
- Calcium carbonate + Vitamin D3 600 mg/400 IU PO BID
- Ibuprofen 400 mg PO Q8H PRN pain
- Multivitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (diagnosed age 45, deceased)
- Maternal aunt: Ovarian cancer
- Father: Hypertension, type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per month). Occupation: Retired administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, she demonstrates full range of motion in the upper extremities but has visible prominence of the medial border of her right scapula when she pushes against a wall with outstretched arms. There is no associated redness, swelling, or warmth. Strength is intact in the upper limbs, except for mild weakness in shoulder abduction past 90 degrees. Sensory exam is normal.

## Encounter 1: source 2020-04-15, practice 2023-06-07, outpatient, Obstetrics & Gynecology

Chief complaint: Abnormal uterine bleeding after initiating hormone replacement therapy

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Maria Santos, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Abnormal uterine bleeding after initiating hormone replacement therapy

HISTORY OF PRESENT ILLNESS:
The patient is a 57-year-old female with complex atypical endometrial hyperplasia and metastatic breast carcinoma on endocrine therapy who presents with abnormal uterine bleeding. She has been perimenopausal for the past year and began hormone replacement therapy (HRT) 4 months ago for severe hot flashes and night sweats. Initially, she was prescribed a combination of estrogen and progesterone, but she discontinued the progesterone component after experiencing mood swings. She now reports several weeks of intermittent heavy bleeding.

PAST MEDICAL HISTORY:
Active Problem List:
- Thoracic outlet syndrome (diagnosed 2020-01-15)
- Stage IIA estrogen receptor-positive breast cancer (on adjuvant endocrine therapy)
- History of left breast Stage II invasive ductal carcinoma (s/p lumpectomy and chemotherapy 5 years prior)
- Status post modified radical mastectomy with axillary lymph node dissection (right breast)
- Metastatic breast carcinoma to liver and adrenal glands
- Complex atypical endometrial hyperplasia
- Major depressive disorder, recurrent moderate
- Chemotherapy-induced peripheral neuropathy (metronidazole-associated)
- Thoracic outlet syndrome, post-surgical
- Secondary adrenal insufficiency
- Chronic cancer-related pain syndrome
- Perimenopausal syndrome
- Osteopenia

PAST SURGICAL HISTORY:
- Left breast lumpectomy with sentinel lymph node biopsy and adjuvant chemotherapy (age 52)
- Right breast lumpectomy with adjuvant radiation therapy (age 55)
- Modified radical mastectomy with axillary lymph node dissection, right (age 56)
- Implantable port placement and removal (age 56)

MEDICATIONS:
Home Medications:
- Tamoxifen 20 mg PO daily
- Estradiol 1 mg PO daily (estrogen-only HRT, discontinued progesterone 2 months ago due to mood swings)
- Calcium carbonate + Vitamin D3 600 mg/400 IU PO BID
- Ibuprofen 400 mg PO Q8H PRN pain
- Multivitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (diagnosed age 45, deceased)
- Maternal aunt: Ovarian cancer
- Father: Hypertension, type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per month). Occupation: Retired administrative assistant.

PHYSICAL EXAMINATION:
Pelvic examination is unremarkable.

OTHER STUDIES:
Her last Pap smear was normal.

## Encounter 2: source 2020-07-15, practice 2023-09-04, outpatient, Psychiatry

Chief complaint: Persistent fatigue, unintentional weight loss, difficulty sleeping, and loss of interest in hobbies

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Alan Chen, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue, unintentional weight loss, difficulty sleeping, and loss of interest in hobbies

HISTORY OF PRESENT ILLNESS:
The patient returns for psychiatric evaluation in the context of her recent Obstetrics & Gynecology encounter for abnormal uterine bleeding and ongoing treatment for metastatic breast carcinoma. She presents with a 3-month history of persistent fatigue, 10-pound unintentional weight loss, and difficulty sleeping. She reports feeling "down" most of the day, nearly every day, and no longer enjoys gardening, which was previously her favorite hobby. She also mentions feeling worthless and has trouble concentrating on tasks.

PAST MEDICAL HISTORY:
Active Problem List:
- Thoracic outlet syndrome (diagnosed 2020-01-15)
- Endometrial Hyperplasia (diagnosed 2020-04-15)
- Stage IIA estrogen receptor-positive breast cancer (on adjuvant endocrine therapy)
- History of left breast Stage II invasive ductal carcinoma (s/p lumpectomy and chemotherapy 5 years prior)
- Status post modified radical mastectomy with axillary lymph node dissection (right breast)
- Metastatic breast carcinoma to liver and adrenal glands
- Complex atypical endometrial hyperplasia
- Major depressive disorder, recurrent moderate
- Chemotherapy-induced peripheral neuropathy (metronidazole-associated)
- Thoracic outlet syndrome, post-surgical
- Secondary adrenal insufficiency
- Chronic cancer-related pain syndrome
- Perimenopausal syndrome
- Osteopenia

PAST SURGICAL HISTORY:
- Left breast lumpectomy with sentinel lymph node biopsy and adjuvant chemotherapy (age 52)
- Right breast lumpectomy with adjuvant radiation therapy (age 55)
- Modified radical mastectomy with axillary lymph node dissection, right (age 56)
- Implantable port placement and removal (age 56)

MEDICATIONS:
Her current medications include tamoxifen and a daily multivitamin.

Home Medications:
- Estradiol 1 mg PO daily (estrogen-only HRT, discontinued progesterone 2 months ago due to mood swings)
- Calcium carbonate + Vitamin D3 600 mg/400 IU PO BID
- Ibuprofen 400 mg PO Q8H PRN pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (diagnosed age 45, deceased)
- Maternal aunt: Ovarian cancer
- Father: Hypertension, type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per month). Occupation: Retired administrative assistant.

REVIEW OF SYSTEMS:
She denies any recent fever, cough, or other acute symptoms.

PHYSICAL EXAMINATION:
Physical examination is unremarkable except for mild pallor.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 11.2 g/dL (Normal: 12-15.5 g/dL)
- White blood cell count: 5,000/uL (Normal: 4,500-11,000/uL)
- Platelets: 250,000/uL (Normal: 150,000-450,000/uL)
- Thyroid-stimulating hormone (TSH): 2.5 mIU/L (Normal: 0.4-4.0 mIU/L)
- Comprehensive metabolic panel: Normal

## Encounter 3: source 2020-10-15, practice 2023-12-15, outpatient, Obstetrics & Gynecology

Chief complaint: Six-month history of intermittent vaginal bleeding without pelvic pain or discharge

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Rebecca Liu, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of intermittent vaginal bleeding without pelvic pain or discharge

HISTORY OF PRESENT ILLNESS:
The patient with complex atypical endometrial hyperplasia returns for evaluation of bleeding initially reported after hormone replacement therapy. She presents with a 6-month history of intermittent vaginal bleeding. She denies any recent sexual activity, vaginal discharge, or pelvic pain. Her last Pap smear was normal 2 years ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Thoracic outlet syndrome (diagnosed 2020-01-15)
- Endometrial Hyperplasia (diagnosed 2020-04-15)
- Major depressive disorder (diagnosed 2020-07-15)
- Stage IIA estrogen receptor-positive breast cancer (on adjuvant endocrine therapy)
- History of left breast Stage II invasive ductal carcinoma (s/p lumpectomy and chemotherapy 5 years prior)
- Status post modified radical mastectomy with axillary lymph node dissection (right breast)
- Metastatic breast carcinoma to liver and adrenal glands
- Complex atypical endometrial hyperplasia
- Major depressive disorder, recurrent moderate
- Chemotherapy-induced peripheral neuropathy (metronidazole-associated)
- Thoracic outlet syndrome, post-surgical
- Secondary adrenal insufficiency
- Chronic cancer-related pain syndrome
- Perimenopausal syndrome
- Osteopenia

She has a past medical history significant for stage IIA estrogen receptor-positive breast cancer, for which she underwent lumpectomy and radiation therapy followed by adjuvant tamoxifen therapy initiated 18 months ago.

PAST SURGICAL HISTORY:
- Left breast lumpectomy with sentinel lymph node biopsy and adjuvant chemotherapy (age 52)
- Right breast lumpectomy with adjuvant radiation therapy (age 55)
- Modified radical mastectomy with axillary lymph node dissection, right (age 56)
- Implantable port placement and removal (age 56)

MEDICATIONS:
She takes adjuvant tamoxifen therapy initiated 18 months ago.

Home Medications:
- Estradiol 1 mg PO daily (estrogen-only HRT, discontinued progesterone 2 months ago due to mood swings)
- Calcium carbonate + Vitamin D3 600 mg/400 IU PO BID
- Ibuprofen 400 mg PO Q8H PRN pain
- Multivitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (diagnosed age 45, deceased)
- Maternal aunt: Ovarian cancer
- Father: Hypertension, type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per month). Occupation: Retired administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, her vital signs are within normal limits, and her abdominal exam is unremarkable. Pelvic examination reveals a slightly enlarged, non-tender uterus without any adnexal masses.

LABORATORY DATA:
Laboratory values:
- Hemoglobin: 12.3 g/dL
- Platelet count: 250,000/uL
- FSH: 7.2 mIU/mL (postmenopausal range: >40 mIU/mL)
- LH: 6.9 mIU/mL (postmenopausal range: >30 mIU/mL)
- Estradiol: 45 pg/mL (postmenopausal range: <20 pg/mL)

PATHOLOGY:
Endometrial biopsy is performed, and the pathology report indicates the presence of complex atypical hyperplasia.

## Encounter 4: source 2021-07-15, practice 2024-09-09, outpatient, Oncology

Chief complaint: Persistent right chest wall pain with minimal relief from ibuprofen

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Michael Torres, MD
Department: Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent right chest wall pain with minimal relief from ibuprofen

HISTORY OF PRESENT ILLNESS:
The patient is a 58-year-old female with metastatic breast carcinoma status post right modified radical mastectomy who returns to the oncology clinic for evaluation of persistent right chest wall pain. She describes the pain as a constant dull ache with intermittent sharp twinges that worsen with movement. She rates her pain as 5 out of 10 on average but states it can reach up to 7 out of 10 on her worst days. She has been taking ibuprofen 400 mg three times daily with minimal relief. She denies any recent trauma or injury.

PAST MEDICAL HISTORY:
Active Problem List:
- Thoracic outlet syndrome (diagnosed 2020-01-15)
- Endometrial Hyperplasia (diagnosed 2020-04-15)
- Major depressive disorder (diagnosed 2020-07-15)
- Complex atypical endometrial hyperplasia (diagnosed 2020-10-15)
- Stage IIA estrogen receptor-positive breast cancer (on adjuvant endocrine therapy)
- History of left breast Stage II invasive ductal carcinoma (s/p lumpectomy and chemotherapy 5 years prior)
- Status post modified radical mastectomy with axillary lymph node dissection (right breast)
- Metastatic breast carcinoma to liver and adrenal glands
- Major depressive disorder, recurrent moderate
- Chemotherapy-induced peripheral neuropathy (metronidazole-associated)
- Thoracic outlet syndrome, post-surgical
- Secondary adrenal insufficiency
- Chronic cancer-related pain syndrome
- Perimenopausal syndrome
- Osteopenia

PAST SURGICAL HISTORY:
- Left breast lumpectomy with sentinel lymph node biopsy and adjuvant chemotherapy (age 52)
- Right breast lumpectomy with adjuvant radiation therapy (age 55)
- Modified radical mastectomy with axillary lymph node dissection, right (age 56)
- Implantable port placement and removal (age 56)

MEDICATIONS:
Home Medications:
- Tamoxifen 20 mg PO daily
- Estradiol 1 mg PO daily (estrogen-only HRT, discontinued progesterone 2 months ago due to mood swings)
- Calcium carbonate + Vitamin D3 600 mg/400 IU PO BID
- Ibuprofen 400 mg PO Q8H PRN pain
- Multivitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (diagnosed age 45, deceased)
- Maternal aunt: Ovarian cancer
- Father: Hypertension, type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per month). Occupation: Retired administrative assistant.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, there is mild tenderness to palpation over the right lateral chest wall, but no erythema, warmth, or swelling is noted.

LABORATORY DATA:
Laboratory values:
- Hemoglobin: 13.2 g/dL
- White blood cell count: 7.8 x 10^3/uL
- Platelet count: 250 x 10^3/uL
- Serum creatinine: 0.9 mg/dL
- Liver function tests: Within normal limits

ASSESSMENT AND PLAN:
Given the patient's ongoing pain and inadequate response to nonopioid analgesics,

## Encounter 5: source 2022-01-15, practice 2025-03-10, outpatient, Neurology

Chief complaint: Numbness and tingling in hands and feet described as wearing gloves and socks

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Sarah Kim, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Numbness and tingling in hands and feet described as wearing gloves and socks

HISTORY OF PRESENT ILLNESS:
The patient with metastatic breast cancer and thoracic outlet syndrome presents with sensory symptoms. She describes the sensation as "wearing gloves and socks all the time" and notes it has progressively worsened over the past two months. She denies any weakness or pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Thoracic outlet syndrome (diagnosed 2020-01-15)
- Endometrial Hyperplasia (diagnosed 2020-04-15)
- Major depressive disorder (diagnosed 2020-07-15)
- Complex atypical endometrial hyperplasia (diagnosed 2020-10-15)
- Moderate cancer-related pain (diagnosed 2021-07-15)
- Stage IIA estrogen receptor-positive breast cancer (on adjuvant endocrine therapy)
- History of left breast Stage II invasive ductal carcinoma (s/p lumpectomy and chemotherapy 5 years prior)
- Status post modified radical mastectomy with axillary lymph node dissection (right breast)
- Metastatic breast carcinoma to liver and adrenal glands
- Major depressive disorder, recurrent moderate
- Chemotherapy-induced peripheral neuropathy (metronidazole-associated)
- Thoracic outlet syndrome, post-surgical
- Secondary adrenal insufficiency
- Chronic cancer-related pain syndrome
- Perimenopausal syndrome
- Osteopenia

Her chemotherapy regimen included docetaxel and carboplatin.

PAST SURGICAL HISTORY:
- Left breast lumpectomy with sentinel lymph node biopsy and adjuvant chemotherapy (age 52)
- Right breast lumpectomy with adjuvant radiation therapy (age 55)
- Modified radical mastectomy with axillary lymph node dissection, right (age 56)
- Implantable port placement and removal (age 56)

MEDICATIONS:
Home Medications:
- Tamoxifen 20 mg PO daily
- Estradiol 1 mg PO daily (estrogen-only HRT, discontinued progesterone 2 months ago due to mood swings)
- Calcium carbonate + Vitamin D3 600 mg/400 IU PO BID
- Ibuprofen 400 mg PO Q8H PRN pain
- Multivitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (diagnosed age 45, deceased)
- Maternal aunt: Ovarian cancer
- Father: Hypertension, type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per month). Occupation: Retired administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, vibratory sensation is diminished in both lower extremities, and proprioception is intact. Reflexes are 1+ in the upper extremities and 2+ at the knees, with absent ankle reflexes.

## Encounter 6: source 2022-03-15, practice 2025-05-13, outpatient, Gastroenterology

Chief complaint: Progressive right upper quadrant pain and unexplained weight loss over two months

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. James O'Brien, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive right upper quadrant pain and unexplained weight loss over two months

HISTORY OF PRESENT ILLNESS:
This 58-year-old female with metastatic breast carcinoma to the liver and recent Neurology evaluation for peripheral neuropathy presents to the Gastroenterology clinic. She reports a 2-month history of progressive right upper quadrant pain and unexplained weight loss of 10 kg.

PAST MEDICAL HISTORY:
Active Problem List:
- Thoracic outlet syndrome (diagnosed 2020-01-15)
- Endometrial Hyperplasia (diagnosed 2020-04-15)
- Major depressive disorder (diagnosed 2020-07-15)
- Complex atypical endometrial hyperplasia (diagnosed 2020-10-15)
- Moderate cancer-related pain (diagnosed 2021-07-15)
- Metronidazole-induced polyneuropathy (diagnosed 2022-01-15)
- Stage IIA estrogen receptor-positive breast cancer (on adjuvant endocrine therapy)
- History of left breast Stage II invasive ductal carcinoma (s/p lumpectomy and chemotherapy 5 years prior)
- Status post modified radical mastectomy with axillary lymph node dissection (right breast)
- Metastatic breast carcinoma to liver and adrenal glands
- Major depressive disorder, recurrent moderate
- Chemotherapy-induced peripheral neuropathy (metronidazole-associated)
- Thoracic outlet syndrome, post-surgical
- Secondary adrenal insufficiency
- Chronic cancer-related pain syndrome
- Perimenopausal syndrome
- Osteopenia

She has a past medical history of breast cancer treated with lumpectomy and adjuvant chemotherapy 5 years ago, currently in remission.

PAST SURGICAL HISTORY:
- Left breast lumpectomy with sentinel lymph node biopsy and adjuvant chemotherapy (age 52)
- Right breast lumpectomy with adjuvant radiation therapy (age 55)
- Modified radical mastectomy with axillary lymph node dissection, right (age 56)
- Implantable port placement and removal (age 56)

MEDICATIONS:
Home Medications:
- Tamoxifen 20 mg PO daily
- Estradiol 1 mg PO daily (estrogen-only HRT, discontinued progesterone 2 months ago due to mood swings)
- Calcium carbonate + Vitamin D3 600 mg/400 IU PO BID
- Ibuprofen 400 mg PO Q8H PRN pain
- Multivitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (diagnosed age 45, deceased)
- Maternal aunt: Ovarian cancer
- Father: Hypertension, type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per month). Occupation: Retired administrative assistant.

REVIEW OF SYSTEMS:
She denies fever, chills, or changes in bowel habits.

VITAL SIGNS:
On examination, her vital signs are stable

PHYSICAL EXAMINATION:
There is mild tenderness on palpation of the right upper quadrant. No jaundice or ascites is noted.

LABORATORY DATA:
Laboratory tests reveal mild elevation in alkaline phosphatase and normal bilirubin levels.

IMAGING:
An abdominal ultrasound shows multiple hypoechoic liver lesions. A CT scan of the abdomen confirms the presence of multiple liver masses with irregular borders.

## Encounter 7: source 2023-01-15, practice 2026-02-27, ed, Emergency Medicine

Chief complaint: Fatigue, dizziness, and syncopal episode with salt cravings and weight loss

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Emily Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fatigue, dizziness, and syncopal episode with salt cravings and weight loss

HISTORY OF PRESENT ILLNESS:
This patient with metastatic breast carcinoma returns for evaluation. She reports a 10-pound weight loss over the past two months and has been experiencing increasing salt cravings.

PAST MEDICAL HISTORY:
Active Problem List:
- Thoracic outlet syndrome (diagnosed 2020-01-15)
- Endometrial Hyperplasia (diagnosed 2020-04-15)
- Major depressive disorder (diagnosed 2020-07-15)
- Complex atypical endometrial hyperplasia (diagnosed 2020-10-15)
- Moderate cancer-related pain (diagnosed 2021-07-15)
- Metronidazole-induced polyneuropathy (diagnosed 2022-01-15)
- Secondary malignant neoplasm of liver (diagnosed 2022-03-15)
- Stage IIA estrogen receptor-positive breast cancer (on adjuvant endocrine therapy)
- History of left breast Stage II invasive ductal carcinoma (s/p lumpectomy and chemotherapy 5 years prior)
- Status post modified radical mastectomy with axillary lymph node dissection (right breast)
- Metastatic breast carcinoma to liver and adrenal glands
- Major depressive disorder, recurrent moderate
- Chemotherapy-induced peripheral neuropathy (metronidazole-associated)
- Thoracic outlet syndrome, post-surgical
- Secondary adrenal insufficiency
- Chronic cancer-related pain syndrome
- Perimenopausal syndrome
- Osteopenia

PAST SURGICAL HISTORY:
- Left breast lumpectomy with sentinel lymph node biopsy and adjuvant chemotherapy (age 52)
- Right breast lumpectomy with adjuvant radiation therapy (age 55)
- Modified radical mastectomy with axillary lymph node dissection, right (age 56)
- Implantable port placement and removal (age 56)

MEDICATIONS:
Her medications include calcium and vitamin D supplements.

Home Medications:
- Tamoxifen 20 mg PO daily
- Estradiol 1 mg PO daily (estrogen-only HRT, discontinued progesterone 2 months ago due to mood swings)
- Calcium carbonate + Vitamin D3 600 mg/400 IU PO BID
- Ibuprofen 400 mg PO Q8H PRN pain
- Multivitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (diagnosed age 45, deceased)
- Maternal aunt: Ovarian cancer
- Father: Hypertension, type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per month). Occupation: Retired administrative assistant.

VITAL SIGNS:
Her blood pressure is 90/60 mmHg

PHYSICAL EXAMINATION:
She has orthostatic hypotension. Her skin appears hyperpigmented, especially in creases and scars.

LABORATORY DATA:
Laboratory tests reveal:

- Sodium: 128 mEq/L (Normal: 135-145 mEq/L)
- Potassium: 5.5 mEq/L (Normal: 3.5-5.0 mEq/L)
- Chloride: 95 mEq/L (Normal: 98-107 mEq/L)
- Bicarbonate: 24 mEq/L (Normal: 22-29 mEq/L)
- Blood urea nitrogen: 18 mg/dL (Normal: 7-20 mg/dL)
- Creatinine: 0.9 mg/dL (Normal: 0.6-1.1 mg/dL)
- Glucose: 65 mg/dL (Normal: 70-99 mg/dL)
- Cortisol at 8 AM: 3 µg/dL (Normal: 7-25 µg/dL)

OTHER STUDIES:
A cosyntropin stimulation test is performed, and her cortisol level 60 minutes later is 5 µg/dL.
