# synthetic_hospital patient 2861

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 66 at first encounter, sex F, Asian-American, Medicare with supplemental private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-07-17 (Mon) | outpatient | Endocrinology |
| 1 | 2021-01-15 | 2024-07-29 (Mon) | outpatient | Medical Oncology |
| 2 | 2022-01-15 | 2025-07-21 (Mon) | outpatient | Medical Oncology |
| 3 | 2022-07-15 | 2026-01-29 (Thu) | outpatient | Family Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Sulfa drugs - urticaria",
  "Iodinated contrast - anaphylaxis"
 ],
 "insurance": "Medicare with supplemental private insurance",
 "occupation": "Retired textile factory worker",
 "alcohol_use": "None",
 "family_history": [
  "Mother with postmenopausal breast cancer (diagnosed age 55, deceased)",
  "Maternal aunt with ovarian cancer",
  "Father with hypertension"
 ],
 "race_ethnicity": "Asian-American",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "30 mg PO every 12 hours",
   "name": "Morphine sulfate extended-release"
  },
  {
   "dose": "15 mg PO every 4 hours as needed for breakthrough pain",
   "name": "Morphine sulfate immediate-release"
  },
  {
   "dose": "50,000 units PO weekly",
   "name": "Cholecalciferol (Vitamin D3)"
  },
  {
   "dose": "600 mg PO three times daily with meals",
   "name": "Calcium carbonate"
  },
  {
   "dose": "8.6 mg PO twice daily",
   "name": "Senna"
  },
  {
   "dose": "100 mg PO twice daily",
   "name": "Docusate sodium"
  },
  {
   "dose": "10 mg PO every 6 hours as needed for nausea",
   "name": "Prochlorperazine"
  },
  {
   "dose": "15 mg PO at bedtime for appetite stimulation",
   "name": "Mirtazapine"
  }
 ],
 "surgical_history": [
  "Right modified radical mastectomy with axillary lymph node dissection (5 years prior)",
  "Insertion of right subclavian Mediport for chemotherapy access",
  "Radiation therapy to chest wall and thoracic spine (non-surgical but significant history)"
 ],
 "chronic_conditions": [
  "Stage IV invasive ductal carcinoma of the breast with widespread osseous metastases",
  "Severe vitamin D deficiency with associated osteomalacia",
  "Chronic cancer-related pain syndrome",
  "Cancer anorexia-cachexia syndrome",
  "Sarcopenia secondary to malignancy and vitamin D deficiency",
  "Opioid-induced constipation"
 ],
 "age_at_first_encounter": 66
}
```

Primary diagnoses: ["Metastatic melanoma", "Metastatic carcinoma to bone", "Malignant neoplasm of male breast", "Vitamin D deficiency"]
Comorbidities: ["Anemia of chronic disease", "Cachexia", "Inadequate family support (caregiver burden)", "Hypocalcemia", "Hypoalbuminemia"]

## Encounter 0: source 2020-01-15, practice 2023-07-17, outpatient, Endocrinology

Chief complaint: Fatigue, diffuse bone discomfort, and recent difficulty rising from a chair

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Sharma, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, diffuse bone discomfort, and recent difficulty rising from a chair

HISTORY OF PRESENT ILLNESS:
She has not seen a physician in several years.

PAST MEDICAL HISTORY:
Active Problem List:
- Stage IV invasive ductal carcinoma of the breast with widespread osseous metastases
- Severe vitamin D deficiency with associated osteomalacia
- Chronic cancer-related pain syndrome
- Cancer anorexia-cachexia syndrome
- Sarcopenia secondary to malignancy and vitamin D deficiency
- Opioid-induced constipation

PAST SURGICAL HISTORY:
- Right modified radical mastectomy with axillary lymph node dissection (5 years prior)
- Insertion of right subclavian Mediport for chemotherapy access
- Radiation therapy to chest wall and thoracic spine (non-surgical but significant history)

MEDICATIONS:
Home Medications:
- Morphine sulfate extended-release 30 mg PO every 12 hours
- Morphine sulfate immediate-release 15 mg PO every 4 hours as needed for breakthrough pain
- Cholecalciferol (Vitamin D3) 50,000 units PO weekly
- Calcium carbonate 600 mg PO three times daily with meals
- Senna 8.6 mg PO twice daily
- Docusate sodium 100 mg PO twice daily
- Prochlorperazine 10 mg PO every 6 hours as needed for nausea
- Mirtazapine 15 mg PO at bedtime for appetite stimulation

ALLERGIES:
Sulfa drugs - urticaria, Iodinated contrast - anaphylaxis

FAMILY HISTORY:
- Mother with postmenopausal breast cancer (diagnosed age 55, deceased)
- Maternal aunt with ovarian cancer
- Father with hypertension

SOCIAL HISTORY:
Her diet consists mostly of rice and instant noodles, and she rarely goes outside. She does not drink alcohol or smoke.

PHYSICAL EXAMINATION:
Physical examination reveals proximal muscle weakness and tenderness to palpation over both tibias and forearms. She has a wide-based gait and mild kyphosis.

LABORATORY DATA:
Her labs show calcium of 7.3 mg/dL, phosphate of 1.9 mg/dL, alkaline phosphatase of 230 U/L, 25-hydroxyvitamin D of 6 ng/mL, and undetectable 1,25-dihydroxyvitamin D.

IMAGING:
A DEXA scan reveals decreased bone mineral density consistent with osteomalacia.

## Encounter 1: source 2021-01-15, practice 2024-07-29, outpatient, Medical Oncology

Chief complaint: Routine follow-up for metastatic breast cancer with increasing fatigue and concerns about functional status for upcoming family events

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James O'Brien, MD
Department: Medical Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for metastatic breast cancer with increasing fatigue and concerns about functional status for upcoming family events

HISTORY OF PRESENT ILLNESS:
Patient returns for cancer follow-up, seen in Endocrinology one year prior for fatigue, diffuse bone discomfort, and difficulty rising from chair. She has been on palliative chemotherapy and reports increasing fatigue and difficulty in managing her daily activities. She expresses desire to attend her granddaughter's outdoor graduation ceremony in three months but is concerned about declining functional status.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitamin D deficiency (diagnosed 2020-01-15)
- Stage IV invasive ductal carcinoma of the breast with widespread osseous metastases
- Severe vitamin D deficiency with associated osteomalacia
- Chronic cancer-related pain syndrome
- Cancer anorexia-cachexia syndrome
- Sarcopenia secondary to malignancy and vitamin D deficiency
- Opioid-induced constipation

PAST SURGICAL HISTORY:
- Right modified radical mastectomy with axillary lymph node dissection (5 years prior)
- Insertion of right subclavian Mediport for chemotherapy access
- Radiation therapy to chest wall and thoracic spine (non-surgical but significant history)

MEDICATIONS:
She is currently taking morphine for pain control

Home Medications:
- Morphine sulfate extended-release 30 mg PO every 12 hours
- Morphine sulfate immediate-release 15 mg PO every 4 hours as needed for breakthrough pain
- Cholecalciferol (Vitamin D3) 50,000 units PO weekly
- Calcium carbonate 600 mg PO three times daily with meals
- Senna 8.6 mg PO twice daily
- Docusate sodium 100 mg PO twice daily
- Prochlorperazine 10 mg PO every 6 hours as needed for nausea
- Mirtazapine 15 mg PO at bedtime for appetite stimulation

ALLERGIES:
Sulfa drugs - urticaria, Iodinated contrast - anaphylaxis

FAMILY HISTORY:
- Mother with postmenopausal breast cancer (diagnosed age 55, deceased)
- Maternal aunt with ovarian cancer
- Father with hypertension

SOCIAL HISTORY:
She has a good support system at home

VITAL SIGNS:
Her vital signs are stable

PHYSICAL EXAMINATION:
On examination, she appears cachectic and has diffuse bony tenderness

LABORATORY DATA:
Laboratory tests reveal a hemoglobin of 9.2 g/dL, calcium of 10.5 mg/dL, and albumin of 3.0 g/dL

## Encounter 2: source 2022-01-15, practice 2025-07-21, outpatient, Medical Oncology

Chief complaint: Increasing fatigue, anorexia, and poorly controlled pain despite oral opioid regimen

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Elena Vasquez, MD
Department: Medical Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing fatigue, anorexia, and poorly controlled pain despite oral opioid regimen

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of metastatic disease following initial presentation in 2020 for fatigue and bone discomfort and subsequent oncology surveillance in 2021. She is brought to the clinic by her daughter. Despite initial periods of remission, her disease has progressed. She now reports increasing fatigue, anorexia, and pain that is poorly controlled by her current regimen of oral opioids. She expresses a desire to focus on quality of life rather than pursuing further aggressive treatment.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitamin D deficiency (diagnosed 2020-01-15)
- Metastatic carcinoma to bone (diagnosed 2021-01-15)
- Stage IV invasive ductal carcinoma of the breast with widespread osseous metastases
- Severe vitamin D deficiency with associated osteomalacia
- Chronic cancer-related pain syndrome
- Cancer anorexia-cachexia syndrome
- Sarcopenia secondary to malignancy and vitamin D deficiency
- Opioid-induced constipation

The patient has been on multiple lines of chemotherapy and targeted therapy over the past three years.

PAST SURGICAL HISTORY:
- Right modified radical mastectomy with axillary lymph node dissection (5 years prior)
- Insertion of right subclavian Mediport for chemotherapy access
- Radiation therapy to chest wall and thoracic spine (non-surgical but significant history)

MEDICATIONS:
She takes her current regimen of oral opioids.

Home Medications:
- Morphine sulfate extended-release 30 mg PO every 12 hours
- Morphine sulfate immediate-release 15 mg PO every 4 hours as needed for breakthrough pain
- Cholecalciferol (Vitamin D3) 50,000 units PO weekly
- Calcium carbonate 600 mg PO three times daily with meals
- Senna 8.6 mg PO twice daily
- Docusate sodium 100 mg PO twice daily
- Prochlorperazine 10 mg PO every 6 hours as needed for nausea
- Mirtazapine 15 mg PO at bedtime for appetite stimulation

ALLERGIES:
Sulfa drugs - urticaria, Iodinated contrast - anaphylaxis

FAMILY HISTORY:
- Mother with postmenopausal breast cancer (diagnosed age 55, deceased)
- Maternal aunt with ovarian cancer
- Father with hypertension

SOCIAL HISTORY:
Her daughter, who is her primary caregiver, reports feeling overwhelmed with managing her mother's medications and complex needs.

VITAL SIGNS:
Her vital signs are stable

PHYSICAL EXAMINATION:
On examination, the patient appears cachectic and is using a wheelchair. her lungs are clear to auscultation.

LABORATORY DATA:
Hemoglobin: 9.8 g/dL
- Calcium: 10.2 mg/dL
- Albumin: 3.0 g/dL
- Creatinine: 1.2 mg/dL

## Encounter 3: source 2022-07-15, practice 2026-01-29, outpatient, Family Medicine

Chief complaint: Acute family meeting with primary care physician to discuss care goals after exhausting curative treatment options

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Robert Chen, MD
Department: Family Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Acute family meeting with primary care physician to discuss care goals after exhausting curative treatment options

HISTORY OF PRESENT ILLNESS:
The patient returns for an acute family meeting to discuss care goals, following longitudinal management of Stage IV breast cancer with osseous metastases, severe vitamin D deficiency, and cancer-related symptomatology previously addressed in Endocrinology (2020) and Medical Oncology (2021, 2022). She was diagnosed with breast cancer 5 years ago, and despite multiple lines of chemotherapy and radiation therapy, her disease has progressed. Recent imaging shows widespread metastatic disease, and her oncologist has indicated that there are no further curative treatment options available. The patient expresses that she does not wish to pursue aggressive treatment any longer and would prefer to focus on her quality of life. She reports increasing fatigue, occasional shortness of breath, and pain that is becoming difficult to manage with her current medication regimen.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitamin D deficiency (diagnosed 2020-01-15)
- Metastatic carcinoma to bone (diagnosed 2021-01-15)
- Malignant neoplasm of male breast (diagnosed 2022-01-15)
- Stage IV invasive ductal carcinoma of the breast with widespread osseous metastases
- Severe vitamin D deficiency with associated osteomalacia
- Chronic cancer-related pain syndrome
- Cancer anorexia-cachexia syndrome
- Sarcopenia secondary to malignancy and vitamin D deficiency
- Opioid-induced constipation

PAST SURGICAL HISTORY:
- Right modified radical mastectomy with axillary lymph node dissection (5 years prior)
- Insertion of right subclavian Mediport for chemotherapy access
- Radiation therapy to chest wall and thoracic spine (non-surgical but significant history)

MEDICATIONS:
Home Medications:
- Morphine sulfate extended-release 30 mg PO every 12 hours
- Morphine sulfate immediate-release 15 mg PO every 4 hours as needed for breakthrough pain
- Cholecalciferol (Vitamin D3) 50,000 units PO weekly
- Calcium carbonate 600 mg PO three times daily with meals
- Senna 8.6 mg PO twice daily
- Docusate sodium 100 mg PO twice daily
- Prochlorperazine 10 mg PO every 6 hours as needed for nausea
- Mirtazapine 15 mg PO at bedtime for appetite stimulation

ALLERGIES:
Sulfa drugs - urticaria, Iodinated contrast - anaphylaxis

FAMILY HISTORY:
- Mother with postmenopausal breast cancer (diagnosed age 55, deceased)
- Maternal aunt with ovarian cancer
- Father with hypertension

SOCIAL HISTORY:
She lives at home with her husband, who is her primary caregiver, but he is struggling to manage her increasing care needs.

PHYSICAL EXAMINATION:
Her physical examination is notable for pallor, decreased breath sounds at the lung bases, and tenderness in the spine and right hip.

LABORATORY DATA:
Her most recent laboratory values are as follows:

- Hemoglobin: 9.2 g/dL
- Calcium: 11.4 mg/dL
- Alkaline phosphatase: 240 U/L
- Creatinine: 1.8 mg/dL

ASSESSMENT AND PLAN:
Given the patient's prognosis and her expressed wishes,
