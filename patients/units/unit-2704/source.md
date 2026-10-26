# synthetic_hospital patient 2704

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 46 at first encounter, sex F, Non-Hispanic White, Commercial.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-06-18 (Tue) | outpatient | Breast Surgery |
| 1 | 2020-03-15 | 2024-08-13 (Tue) | outpatient | Internal Medicine |
| 2 | 2020-05-15 | 2024-10-23 (Wed) | outpatient | Neurology |
| 3 | 2021-01-15 | 2025-06-10 (Tue) | outpatient | Obstetrics & Gynecology |
| 4 | 2021-04-15 | 2025-09-08 (Mon) | outpatient | Oncology |
| 5 | 2021-07-15 | 2025-12-24 (Wed) | outpatient | Obstetrics & Gynecology |
| 6 | 2021-09-15 | 2026-02-23 (Mon) | outpatient | Obstetrics & Gynecology |
| 7 | 2022-01-15 | 2026-06-15 (Mon) | outpatient | Family Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial",
 "occupation": "Dental hygienist",
 "alcohol_use": "None",
 "family_history": [
  "Breast cancer in maternal aunt (age 52)",
  "Hypertension in mother",
  "Premature coronary artery disease in father (age 55)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Tamoxifen"
  },
  {
   "dose": "6 mg/kg IV every 3 weeks (maintenance therapy)",
   "name": "Trastuzumab"
  }
 ],
 "surgical_history": [
  "Right breast lumpectomy with sentinel lymph node biopsy",
  "Radiation therapy to right breast and axilla",
  "Chemotherapy and radiation therapy for Hodgkin lymphoma (10 years ago)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia",
  "History of Hodgkin lymphoma in remission (10 years)",
  "History of invasive ductal carcinoma of the right breast",
  "History of cervical radiculopathy"
 ],
 "age_at_first_encounter": 46
}
```

Primary diagnoses: ["Endometrial Hyperplasia", "Trastuzumab-induced cardiotoxicity with asymptomatic left ventricular dysfunction", "Hyperlipidemia", "Systemic menopausal syndrome", "Asymptomatic Actinomyces colonization of genital tract", "Invasive ductal carcinoma of right breast", "Cervical radiculopathy"]
Comorbidities: ["History of breast cancer", "Obesity", "Iatrogenic immunosuppression", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2024-06-18, outpatient, Breast Surgery

Chief complaint: 2-month history of a palpable mass in her right breast with associated skin dimpling

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Breast Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
2-month history of a palpable mass in her right breast with associated skin dimpling

HISTORY OF PRESENT ILLNESS:
She reports no pain associated with the mass but has noticed some skin dimpling over the area.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- History of Hodgkin lymphoma in remission (10 years)
- History of invasive ductal carcinoma of the right breast
- History of cervical radiculopathy

Her past medical history is significant for hypertension, controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Right breast lumpectomy with sentinel lymph node biopsy
- Radiation therapy to right breast and axilla
- Chemotherapy and radiation therapy for Hodgkin lymphoma (10 years ago)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Lisinopril 10 mg PO daily
- Tamoxifen 20 mg PO daily
- Trastuzumab 6 mg/kg IV every 3 weeks (maintenance therapy)

ALLERGIES:
NKDA

FAMILY HISTORY:
No family history of breast cancer

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Dental hygienist.

PHYSICAL EXAMINATION:
On physical examination, a 3 cm firm, non-tender mass is palpated in the upper outer quadrant of the right breast. There is no nipple discharge, and the overlying skin appears slightly retracted.

IMAGING:
Mammography reveals a suspicious 2.8 cm density in the right breast with microcalcifications.

PATHOLOGY:
A core needle biopsy of the mass is performed, and pathology confirms invasive ductal carcinoma.

PLAN:
During surgical planning for axillary lymph node dissection,

## Encounter 1: source 2020-03-15, practice 2024-08-13, outpatient, Internal Medicine

Chief complaint: Routine follow-up for hyperlipidemia and discussion of long-term effects of prior Hodgkin lymphoma treatment

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for hyperlipidemia and discussion of long-term effects of prior Hodgkin lymphoma treatment

HISTORY OF PRESENT ILLNESS:
The patient returns for routine follow-up following her recent breast surgery. She has a history of Hodgkin lymphoma treated with combination chemotherapy and radiation therapy 10 years ago. She reports no current symptoms but is concerned about potential long-term effects of her treatment.

PAST MEDICAL HISTORY:
Active Problem List:
- Invasive ductal carcinoma of right breast (diagnosed 2020-01-15)
- Hypertension
- Hyperlipidemia
- History of Hodgkin lymphoma in remission (10 years)
- History of invasive ductal carcinoma of the right breast
- History of cervical radiculopathy

Her medical history is significant for hypertension, controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Right breast lumpectomy with sentinel lymph node biopsy
- Radiation therapy to right breast and axilla
- Chemotherapy and radiation therapy for Hodgkin lymphoma (10 years ago)

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg PO daily
- Tamoxifen 20 mg PO daily
- Trastuzumab 6 mg/kg IV every 3 weeks (maintenance therapy)

ALLERGIES:
NKDA

FAMILY HISTORY:
No family history of cancer

SOCIAL HISTORY:
She does not smoke

VITAL SIGNS:
On physical examination, her blood pressure is 138/85 mmHg, heart rate is 78 beats per minute, and BMI is 26 kg/m^2

PHYSICAL EXAMINATION:
Cardiovascular examination reveals a regular rhythm without murmurs, rubs, or gallops.

LABORATORY DATA:
Complete blood count: within normal limits
- Fasting lipid profile:
- Total cholesterol: 220 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 160 mg/dL
- Triglycerides: 125 mg/dL
- Fasting glucose: 98 mg/dL

## Encounter 2: source 2020-05-15, practice 2024-10-23, outpatient, Neurology

Chief complaint: Intermittent neck pain and tingling sensation radiating down her left arm for 6 weeks

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Jennifer Kim, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent neck pain and tingling sensation radiating down her left arm for 6 weeks

HISTORY OF PRESENT ILLNESS:
The patient is a 46-year-old female with a history of invasive ductal carcinoma and Hodgkin lymphoma in remission who presents to the neurology clinic following her internal medicine visit two months prior. She reports a 6-week history of intermittent neck pain and a tingling sensation radiating down her left arm. She describes the pain as sharp and worsened by turning her head to the left or extending her neck. She denies any recent trauma but mentions that her job requires prolonged periods of sitting at a computer.

PAST MEDICAL HISTORY:
Active Problem List:
- Invasive ductal carcinoma of right breast (diagnosed 2020-01-15)
- Hyperlipidemia (diagnosed 2020-03-15)
- Hypertension
- History of Hodgkin lymphoma in remission (10 years)
- History of invasive ductal carcinoma of the right breast
- History of cervical radiculopathy

Her medical history is significant for hypertension, controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Right breast lumpectomy with sentinel lymph node biopsy
- Radiation therapy to right breast and axilla
- Chemotherapy and radiation therapy for Hodgkin lymphoma (10 years ago)

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg PO daily
- Tamoxifen 20 mg PO daily
- Trastuzumab 6 mg/kg IV every 3 weeks (maintenance therapy)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Breast cancer in maternal aunt (age 52)
- Hypertension in mother
- Premature coronary artery disease in father (age 55)

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Dental hygienist.

PHYSICAL EXAMINATION:
On physical examination, she has decreased sensation to light touch over her left thumb and index finger, and her left triceps reflex is diminished. Neck range of motion is painful on left lateral rotation and extension. No atrophy or fasciculations are noted in the upper extremities. A Spurling's test reproduces her symptoms.

## Encounter 3: source 2021-01-15, practice 2025-06-10, outpatient, Obstetrics & Gynecology

Chief complaint: Bothersome hot flashes and night sweats disrupting sleep for the past 6 months

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Emily Johnson, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Bothersome hot flashes and night sweats disrupting sleep for the past 6 months

HISTORY OF PRESENT ILLNESS:
The patient is a 47-year-old female with a history of invasive ductal carcinoma on maintenance tamoxifen and trastuzumab who presents to the clinic complaining of bothersome hot flashes and night sweats that have been occurring for the past 6 months. She reports that these symptoms are disrupting her sleep and impacting her daily activities.

PAST MEDICAL HISTORY:
Active Problem List:
- Invasive ductal carcinoma of right breast (diagnosed 2020-01-15)
- Hyperlipidemia (diagnosed 2020-03-15)
- Cervical radiculopathy (diagnosed 2020-05-15)
- Hypertension
- History of Hodgkin lymphoma in remission (10 years)
- History of invasive ductal carcinoma of the right breast

She has a past medical history of hypertension, which is well-controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
She underwent a total hysterectomy with bilateral salpingo-oophorectomy for benign fibroids 2 years ago.

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg PO daily
- Tamoxifen 20 mg PO daily
- Trastuzumab 6 mg/kg IV every 3 weeks (maintenance therapy)

ALLERGIES:
NKDA

FAMILY HISTORY:
Her mother had osteoporosis and a hip fracture at the age of 70.

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Dental hygienist.

VITAL SIGNS:
On physical examination, her blood pressure is 128/78 mmHg, heart rate is 76 beats per minute, and BMI is 26 kg/m^2

PHYSICAL EXAMINATION:
The remainder of the physical examination is unremarkable.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum FSH: 82 IU/L (postmenopausal range)
- Serum estradiol: <20 pg/mL (postmenopausal range)
- Complete blood count: Normal
- Lipid profile: Normal
- Liver function tests: Normal

ASSESSMENT AND PLAN:
Based on the patient's symptoms and history,

## Encounter 4: source 2021-04-15, practice 2025-09-08, outpatient, Oncology

Chief complaint: Routine oncology follow-up during trastuzumab maintenance therapy

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. Robert Singh, MD
Department: Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine oncology follow-up during trastuzumab maintenance therapy

HISTORY OF PRESENT ILLNESS:
This 47-year-old with right breast invasive ductal carcinoma returns for routine oncology follow-up. She completed her adjuvant chemotherapy six months ago and has been on trastuzumab therapy since then. She reports no symptoms of dyspnea, chest pain, or palpitations.

PAST MEDICAL HISTORY:
Active Problem List:
- Invasive ductal carcinoma of right breast (diagnosed 2020-01-15)
- Hyperlipidemia (diagnosed 2020-03-15)
- Cervical radiculopathy (diagnosed 2020-05-15)
- Systemic menopausal syndrome (diagnosed 2021-01-15)
- Hypertension
- History of Hodgkin lymphoma in remission (10 years)
- History of invasive ductal carcinoma of the right breast

PAST SURGICAL HISTORY:
- Right breast lumpectomy with sentinel lymph node biopsy
- Radiation therapy to right breast and axilla
- Chemotherapy and radiation therapy for Hodgkin lymphoma (10 years ago)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Lisinopril 10 mg PO daily
- Tamoxifen 20 mg PO daily
- Trastuzumab 6 mg/kg IV every 3 weeks (maintenance therapy)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Breast cancer in maternal aunt (age 52)
- Hypertension in mother
- Premature coronary artery disease in father (age 55)

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Dental hygienist.

VITAL SIGNS:
Her blood pressure is 130/85 mmHg, and her heart rate is 78 beats per minute.

PHYSICAL EXAMINATION:
Her physical examination is unremarkable.

IMAGING:
An echocardiogram performed before starting trastuzumab showed a left ventricular ejection fraction (LVEF) of 55%. Today, a repeat echocardiogram shows an LVEF of 45%.

## Encounter 5: source 2021-07-15, practice 2025-12-24, outpatient, Obstetrics & Gynecology

Chief complaint: Routine follow-up with irregular uterine bleeding for 2 months

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Emily Johnson, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up with irregular uterine bleeding for 2 months

HISTORY OF PRESENT ILLNESS:
The patient returns for routine gynecologic follow-up, previously evaluated for vasomotor symptoms in the setting of tamoxifen therapy for breast cancer. She was diagnosed with hormone receptor-positive breast cancer 18 months ago and underwent lumpectomy followed by radiation therapy. She reports experiencing hot flashes and night sweats. She also mentions irregular uterine bleeding that started 2 months ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Invasive ductal carcinoma of right breast (diagnosed 2020-01-15)
- Hyperlipidemia (diagnosed 2020-03-15)
- Cervical radiculopathy (diagnosed 2020-05-15)
- Systemic menopausal syndrome (diagnosed 2021-01-15)
- Trastuzumab-induced cardiotoxicity with asymptomatic left ventricular dysfunction (diagnosed 2021-04-15)
- Hypertension
- History of Hodgkin lymphoma in remission (10 years)
- History of invasive ductal carcinoma of the right breast

Her medical history is significant for hypertension, controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Right breast lumpectomy with sentinel lymph node biopsy
- Radiation therapy to right breast and axilla
- Chemotherapy and radiation therapy for Hodgkin lymphoma (10 years ago)

MEDICATIONS:
She has been on tamoxifen for the past 16 months.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Lisinopril 10 mg PO daily
- Trastuzumab 6 mg/kg IV every 3 weeks (maintenance therapy)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Breast cancer in maternal aunt (age 52)
- Hypertension in mother
- Premature coronary artery disease in father (age 55)

SOCIAL HISTORY:
She has no history of smoking.

VITAL SIGNS:
On physical examination, her blood pressure is 130/85 mmHg, and her BMI is 29 kg/m^2.

PHYSICAL EXAMINATION:
A pelvic examination reveals a slightly enlarged, non-tender uterus without any adnexal masses.

LABORATORY DATA:
Her most recent laboratory results are as follows:

- Hemoglobin: 13.2 g/dL
- Platelet count: 250,000/mm^3
- Prothrombin time (PT): 11 seconds
- Activated partial thromboplastin time (aPTT): 30 seconds

## Encounter 6: source 2021-09-15, practice 2026-02-23, outpatient, Obstetrics & Gynecology

Chief complaint: Routine check-up with intermittent pelvic pain and irregular menstrual cycles over the past six months

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Lisa Thompson, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine check-up with intermittent pelvic pain and irregular menstrual cycles over the past six months

HISTORY OF PRESENT ILLNESS:
Returning for follow-up of endometrial hyperplasia on tamoxifen, she reports intermittent pelvic pain and irregular menstrual cycles over the past six months.

PAST MEDICAL HISTORY:
Active Problem List:
- Invasive ductal carcinoma of right breast (diagnosed 2020-01-15)
- Hyperlipidemia (diagnosed 2020-03-15)
- Cervical radiculopathy (diagnosed 2020-05-15)
- Systemic menopausal syndrome (diagnosed 2021-01-15)
- Trastuzumab-induced cardiotoxicity with asymptomatic left ventricular dysfunction (diagnosed 2021-04-15)
- Endometrial Hyperplasia (diagnosed 2021-07-15)
- Hypertension
- History of Hodgkin lymphoma in remission (10 years)
- History of invasive ductal carcinoma of the right breast

Her medical history is significant for hypertension, controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Right breast lumpectomy with sentinel lymph node biopsy
- Radiation therapy to right breast and axilla
- Chemotherapy and radiation therapy for Hodgkin lymphoma (10 years ago)

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg PO daily
- Tamoxifen 20 mg PO daily
- Trastuzumab 6 mg/kg IV every 3 weeks (maintenance therapy)

ALLERGIES:
NKDA

FAMILY HISTORY:
Her mother was diagnosed with endometrial cancer at the age of 60.

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Dental hygienist.

VITAL SIGNS:
The patient's vital signs are within normal limits, and her BMI is 32 kg/m^2

PHYSICAL EXAMINATION:
A bimanual pelvic examination reveals a slightly enlarged, non-tender uterus. No adnexal masses are palpable.

LABORATORY DATA:
A Pap test is performed, and the results show atypical glandular cells (AGC).

## Encounter 7: source 2022-01-15, practice 2026-06-15, outpatient, Family Medicine

Chief complaint: Routine check-up with inquiry about new ovarian cancer screening blood test

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. David Patel, MD
Department: Family Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine check-up with inquiry about new ovarian cancer screening blood test

HISTORY OF PRESENT ILLNESS:
The patient returns for routine follow-up with breast cancer maintenance therapy and prior endometrial hyperplasia. She has no complaints and is currently asymptomatic. She inquires about the new blood test for ovarian cancer she heard about on the news and whether she should have it done.

PAST MEDICAL HISTORY:
Active Problem List:
- Invasive ductal carcinoma of right breast (diagnosed 2020-01-15)
- Hyperlipidemia (diagnosed 2020-03-15)
- Cervical radiculopathy (diagnosed 2020-05-15)
- Systemic menopausal syndrome (diagnosed 2021-01-15)
- Trastuzumab-induced cardiotoxicity with asymptomatic left ventricular dysfunction (diagnosed 2021-04-15)
- Endometrial Hyperplasia (diagnosed 2021-07-15)
- Asymptomatic Actinomyces colonization of genital tract (diagnosed 2021-09-15)
- Hypertension
- History of Hodgkin lymphoma in remission (10 years)
- History of invasive ductal carcinoma of the right breast

Her medical history is significant for hypertension, controlled with lisinopril,.

PAST SURGICAL HISTORY:
- Right breast lumpectomy with sentinel lymph node biopsy
- Radiation therapy to right breast and axilla
- Chemotherapy and radiation therapy for Hodgkin lymphoma (10 years ago)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Lisinopril 10 mg PO daily
- Tamoxifen 20 mg PO daily
- Trastuzumab 6 mg/kg IV every 3 weeks (maintenance therapy)

ALLERGIES:
NKDA

FAMILY HISTORY:
A family history of breast cancer.

SOCIAL HISTORY:
She is a non-smoker and does not consume alcohol.

VITAL SIGNS:
Her BMI is 26 kg/m^2.

PHYSICAL EXAMINATION:
On physical examination, her blood pressure is 128/82 mmHg, heart rate is 72 beats per minute, and the rest of the examination is unremarkable.

OTHER STUDIES:
She is up to date with her screening tests, including a normal mammogram and colonoscopy.

ASSESSMENT AND PLAN:
You explain the test's sensitivity and specificity and how they would apply to her. The test has a sensitivity of 90% and a specificity of 95%. Based on this information,
