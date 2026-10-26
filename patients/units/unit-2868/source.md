# synthetic_hospital patient 2868

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 63 at first encounter, sex F, Non-Hispanic White, Medicare (SSDI).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-10-02 (Mon) | outpatient | Gastroenterology |
| 1 | 2020-07-15 | 2024-04-15 (Mon) | outpatient | Cardiology |
| 2 | 2022-01-15 | 2025-10-14 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare (SSDI)",
 "occupation": "Retired elementary school teacher (on disability)",
 "alcohol_use": "No alcohol use",
 "family_history": [
  "Mother: breast cancer (postmenopausal, age 74)",
  "Father: myocardial infarction (age 68)",
  "One sister: healthy",
  "No family history of colorectal cancer, neuroendocrine tumors, or sarcomas"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "Right breast lumpectomy with sentinel lymph node biopsy and completion axillary lymph node dissection",
  "Radical hysterectomy with bilateral salpingo-oophorectomy",
  "Removal of right subclavian port-a-cath"
 ],
 "chronic_conditions": [
  "History of invasive ductal carcinoma, right breast, triple-negative phenotype, stage IIA (status post lumpectomy, axillary lymph node dissection, and adjuvant radiation therapy 5 years prior)",
  "History of cervical squamous cell carcinoma, stage IB2 (status post radical hysterectomy with bilateral salpingo-oophorectomy and pelvic radiation therapy 2 years prior)",
  "Secondary lymphedema of the right upper extremity (chronic, grade 2)",
  "Post-radiation pelvic fibrosis",
  "Osteopenia"
 ],
 "age_at_first_encounter": 63
}
```

Primary diagnoses: ["Chronic radiation proctitis", "Lymphedema-associated lymphangiosarcoma (Stewart-Treves syndrome)", "Carcinoid syndrome"]
Comorbidities: ["Anemia", "History of pelvic radiation therapy"]

## Encounter 0: source 2020-01-15, practice 2023-10-02, outpatient, Gastroenterology

Chief complaint: 6-month history of intermittent rectal bleeding and a sensation of incomplete bowel evacuation

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Narang, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of intermittent rectal bleeding and a sensation of incomplete bowel evacuation

HISTORY OF PRESENT ILLNESS:
She reports no abdominal pain but has experienced a weight loss of 5 kg over the last 4 months.

PAST MEDICAL HISTORY:
Active Problem List:
- History of invasive ductal carcinoma, right breast, triple-negative phenotype, stage IIA (status post lumpectomy, axillary lymph node dissection, and adjuvant radiation therapy 5 years prior)
- History of cervical squamous cell carcinoma, stage IB2 (status post radical hysterectomy with bilateral salpingo-oophorectomy and pelvic radiation therapy 2 years prior)
- Secondary lymphedema of the right upper extremity (chronic, grade 2)
- Post-radiation pelvic fibrosis
- Osteopenia

Her medical history is significant for cervical cancer treated with pelvic radiation therapy 2 years ago.

PAST SURGICAL HISTORY:
- Right breast lumpectomy with sentinel lymph node biopsy and completion axillary lymph node dissection
- Radical hysterectomy with bilateral salpingo-oophorectomy
- Removal of right subclavian port-a-cath

ALLERGIES:
NKDA

FAMILY HISTORY:
She has no family history of colorectal cancer.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Retired elementary school teacher (on disability).

VITAL SIGNS:
On physical examination, her vital signs are within normal limits.

PHYSICAL EXAMINATION:
Digital rectal examination reveals no masses, but there is bright red blood on the examining glove.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 10.2 g/dL (Normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 85 fL (Normal: 80-100 fL)
- White blood cell count: 6,000/uL (Normal: 4,500-11,000/uL)
- Platelet count: 250,000/uL (Normal: 150,000-450,000/uL)

OTHER STUDIES:
Colonoscopy shows mucosal pallor, friability, and multiple telangiectasias confined to the rectum.

## Encounter 1: source 2020-07-15, practice 2024-04-15, outpatient, Cardiology

Chief complaint: Progressive fatigue, frequent episodes of facial flushing, and unintentional weight loss over the past several months

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Okafor, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue, frequent episodes of facial flushing, and unintentional weight loss over the past several months

HISTORY OF PRESENT ILLNESS:
The patient is a 63-year-old female with a history of breast and cervical malignancies, chronic secondary lymphedema of the right upper extremity, and post-radiation pelvic fibrosis who presents to Cardiology six months following her Gastroenterology evaluation for chronic radiation proctitis. She reports several daily episodes of loose stools that are watery, occur without blood or mucus, and are resistant to dietary changes. She denies any recent travel, antibiotic use, or known sick contacts. She has also begun experiencing wheezing intermittently, particularly at night, despite no prior history of asthma or smoking. Her appetite is fair, but she feels more tired than usual and occasionally notices swelling around her ankles by the evening.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic radiation proctitis (diagnosed 2020-01-15)
- History of invasive ductal carcinoma, right breast, triple-negative phenotype, stage IIA (status post lumpectomy, axillary lymph node dissection, and adjuvant radiation therapy 5 years prior)
- History of cervical squamous cell carcinoma, stage IB2 (status post radical hysterectomy with bilateral salpingo-oophorectomy and pelvic radiation therapy 2 years prior)
- Secondary lymphedema of the right upper extremity (chronic, grade 2)
- Post-radiation pelvic fibrosis
- Osteopenia

PAST SURGICAL HISTORY:
- Right breast lumpectomy with sentinel lymph node biopsy and completion axillary lymph node dissection
- Radical hysterectomy with bilateral salpingo-oophorectomy
- Removal of right subclavian port-a-cath

MEDICATIONS:
She is not taking any medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: breast cancer (postmenopausal, age 74)
- Father: myocardial infarction (age 68)
- One sister: healthy
- No family history of colorectal cancer, neuroendocrine tumors, or sarcomas

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Retired elementary school teacher (on disability).

VITAL SIGNS:
Physical exam reveals normal vital signs

PHYSICAL EXAMINATION:
She appears comfortable but has subtle darkening around her eyes and mild bilateral lower extremity edema. The remainder of the physical exam, including pulmonary and abdominal findings, is unremarkable.

LABORATORY DATA:
Laboratory testing reveals a mild normocytic anemia

IMAGING:
Abdominal imaging performed for weight loss shows several hypervascular lesions in the liver

PLAN:
Further workup is initiated

## Encounter 2: source 2022-01-15, practice 2025-10-14, ed, Emergency Medicine

Chief complaint: Rapidly enlarging, painful, purplish nodule on her right chest wall

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Maria Santos, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Rapidly enlarging, painful, purplish nodule on her right chest wall

HISTORY OF PRESENT ILLNESS:
This 65-year-old female with a history of breast and cervical malignancies treated with surgery and radiation, chronic secondary lymphedema, and prior evaluations for radiation proctitis and carcinoid syndrome, presents to the emergency department with a 2-month history of a rapidly enlarging, painful, purplish nodule on her right chest wall. She also reports persistent swelling of her right arm since her cancer treatment, which she manages with compression sleeves.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic radiation proctitis (diagnosed 2020-01-15)
- Carcinoid syndrome (diagnosed 2020-07-15)
- History of invasive ductal carcinoma, right breast, triple-negative phenotype, stage IIA (status post lumpectomy, axillary lymph node dissection, and adjuvant radiation therapy 5 years prior)
- History of cervical squamous cell carcinoma, stage IB2 (status post radical hysterectomy with bilateral salpingo-oophorectomy and pelvic radiation therapy 2 years prior)
- Secondary lymphedema of the right upper extremity (chronic, grade 2)
- Post-radiation pelvic fibrosis
- Osteopenia

She has a past medical history significant for right-sided breast cancer, for which she underwent a lumpectomy and adjuvant radiation therapy 5 years ago.

PAST SURGICAL HISTORY:
- Right breast lumpectomy with sentinel lymph node biopsy and completion axillary lymph node dissection
- Radical hysterectomy with bilateral salpingo-oophorectomy
- Removal of right subclavian port-a-cath

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: breast cancer (postmenopausal, age 74)
- Father: myocardial infarction (age 68)
- One sister: healthy
- No family history of colorectal cancer, neuroendocrine tumors, or sarcomas

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Retired elementary school teacher (on disability).

PHYSICAL EXAMINATION:
On examination, the nodule is approximately 3 cm in diameter, with an irregular border and overlying telangiectasia. There is no discharge or ulceration. The surrounding skin appears thickened and there is non-pitting edema of the right arm. No axillary lymphadenopathy is palpable.

PATHOLOGY:
A punch biopsy of the lesion is performed and histopathology reveals atypical endothelial cells with evidence of vascular channel formation.
