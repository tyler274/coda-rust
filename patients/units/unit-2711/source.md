# synthetic_hospital patient 2711

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 56 at first encounter, sex F, Non-Hispanic White, Private PPO insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-11-18 (Wed) | outpatient | Rheumatology |
| 1 | 2020-07-15 | 2021-06-03 (Thu) | outpatient | Dermatology |
| 2 | 2021-01-15 | 2021-12-06 (Mon) | outpatient | Obstetrics & Gynecology |
| 3 | 2021-02-15 | 2021-12-30 (Thu) | outpatient | Obstetrics & Gynecology |
| 4 | 2021-03-15 | 2022-01-18 (Tue) | outpatient | Obstetrics & Gynecology |
| 5 | 2021-04-15 | 2022-02-28 (Mon) | outpatient | Gynecologic Oncology |
| 6 | 2021-09-15 | 2022-08-04 (Thu) | outpatient | Hematology & Oncology |
| 7 | 2025-01-15 | 2025-11-30 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private PPO insurance",
 "occupation": "Retired elementary school teacher",
 "alcohol_use": "Social (1-2 glasses of wine per week)",
 "family_history": [
  "Mother: Breast cancer at age 65",
  "Father: Hypertension and cerebral aneurysm",
  "Sister: Ovarian cancer at age 60"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
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
   "dose": "1 tablet PO daily",
   "name": "Multivitamin"
  },
  {
   "dose": "600 mg/400 IU PO daily",
   "name": "Calcium carbonate with Vitamin D"
  },
  {
   "dose": "200 mg PO PRN pain",
   "name": "Ibuprofen"
  }
 ],
 "surgical_history": [
  "Total abdominal hysterectomy with bilateral salpingo-oophorectomy, omentectomy, and tumor debulking (stage III ovarian cancer)",
  "Two spontaneous vaginal deliveries (G2P2)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Osteoarthritis (hands)",
  "Remote peptic ulcer disease (healed)",
  "Postmenopausal status",
  "Cherry angiomas"
 ],
 "age_at_first_encounter": 56
}
```

Primary diagnoses: ["Malignant neoplasm of ovary", "Cachexia", "Cherry hemangioma", "Osteoarthritis", "Ruptured cerebral aneurysm"]
Comorbidities: ["Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2020-11-18, outpatient, Rheumatology

Chief complaint: Progressive bilateral hand pain with morning stiffness

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive bilateral hand pain with morning stiffness

HISTORY OF PRESENT ILLNESS:
That has been progressively worsening over the past year. She reports stiffness in her fingers, particularly in the morning, which seems to improve with activity throughout the day. She denies any recent trauma or systemic symptoms such as fever or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Osteoarthritis (hands)
- Remote peptic ulcer disease (healed)
- Postmenopausal status
- Cherry angiomas

Her medical history is significant for hypertension, for which she takes hydrochlorothiazide, and a remote history of peptic ulcer disease.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy, omentectomy, and tumor debulking (stage III ovarian cancer)
- Two spontaneous vaginal deliveries (G2P2)

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg PO daily
- Multivitamin 1 tablet PO daily
- Calcium carbonate with Vitamin D 600 mg/400 IU PO daily
- Ibuprofen 200 mg PO PRN pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer at age 65
- Father: Hypertension and cerebral aneurysm
- Sister: Ovarian cancer at age 60

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, you note bony enlargements of the distal interphalangeal joints and tenderness with palpation. There is no erythema or warmth, and the range of motion is preserved.

LABORATORY DATA:
Erythrocyte sedimentation rate (ESR): 15 mm/hr (normal: 0-20 mm/hr)
- C-reactive protein (CRP): 2 mg/L (normal: <5 mg/L)
- Complete blood count (CBC): within normal limits
- Serum creatinine: 1.0 mg/dL (normal: 0.6-1.2 mg/dL)

IMAGING:
Radiographs of the hands show joint space narrowing, subchondral sclerosis, and osteophytes at the affected joints.

## Encounter 1: source 2020-07-15, practice 2021-06-03, outpatient, Dermatology

Chief complaint: Multiple asymptomatic red skin lesions on trunk that occasionally bleed when scratched

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Okonkwo, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Multiple asymptomatic red skin lesions on trunk that occasionally bleed when scratched

HISTORY OF PRESENT ILLNESS:
The 56-year-old patient with hypertension and osteoarthritis, previously evaluated in Rheumatology for hand pain, now presents to the dermatologist for evaluation of multiple skin lesions she has noticed on her trunk over the past several years. She reports that the lesions are asymptomatic but occasionally bleed if scratched.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis (diagnosed 2020-01-15)
- Hypertension
- Osteoarthritis (hands)
- Remote peptic ulcer disease (healed)
- Postmenopausal status
- Cherry angiomas

Her past medical history is notable for well-controlled hypertension.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy, omentectomy, and tumor debulking (stage III ovarian cancer)
- Two spontaneous vaginal deliveries (G2P2)

MEDICATIONS:
She takes lisinopril.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Multivitamin 1 tablet PO daily
- Calcium carbonate with Vitamin D 600 mg/400 IU PO daily
- Ibuprofen 200 mg PO PRN pain

ALLERGIES:
NKDA

FAMILY HISTORY:
Her father had colon cancer diagnosed at 72.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Retired elementary school teacher.

VITAL SIGNS:
Vitals are normal.

PHYSICAL EXAMINATION:
Physical examination findings are shown in the image below.

## Encounter 2: source 2021-01-15, practice 2021-12-06, outpatient, Obstetrics & Gynecology

Chief complaint: 3-month history of progressive pelvic pain, bloating, early satiety, and 5kg weight loss

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Emily Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of progressive pelvic pain, bloating, early satiety, and 5kg weight loss

HISTORY OF PRESENT ILLNESS:
This 57-year-old with HTN and OA presents after rheum and derm care. She reports a 5 kg weight loss over the same period but denies any changes in her bowel habits or urinary symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis (diagnosed 2020-01-15)
- Cherry hemangioma (diagnosed 2020-07-15)
- Hypertension
- Osteoarthritis (hands)
- Remote peptic ulcer disease (healed)
- Postmenopausal status
- Cherry angiomas

PAST SURGICAL HISTORY:
Benign breast lump excised 10 years ago

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg PO daily
- Multivitamin 1 tablet PO daily
- Calcium carbonate with Vitamin D 600 mg/400 IU PO daily
- Ibuprofen 200 mg PO PRN pain

ALLERGIES:
NKDA

FAMILY HISTORY:
No family history of cancer

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, her abdomen is distended with a fluid wave. A pelvic examination reveals a palpable, fixed, and irregular adnexal mass.

LABORATORY DATA:
CA-125 level is 350 U/mL (normal < 35 U/mL).

Laboratory Values:
- Hemoglobin: 12.1 g/dL
- White blood cell count: 8,000/mm³
- Platelets: 250,000/mm³
- CA-125: 350 U/mL

IMAGING:
Transvaginal ultrasound shows a 7 cm solid ovarian mass with thick septations and free pelvic fluid.

## Encounter 3: source 2021-02-15, practice 2021-12-30, outpatient, Obstetrics & Gynecology

Chief complaint: 3-month history of increasing abdominal girth and early satiety

ENCOUNTER NOTE
Date: 2021-02-15
Provider: Dr. Emily Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of increasing abdominal girth and early satiety

HISTORY OF PRESENT ILLNESS:
The patient with hypertension and osteoarthritis returns one month after initial Obstetrics & Gynecology evaluation, presenting with a 3-month history of increasing abdominal girth and early satiety. She denies any nausea, vomiting, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis (diagnosed 2020-01-15)
- Cherry hemangioma (diagnosed 2020-07-15)
- Malignant neoplasm of ovary (diagnosed 2021-01-15)
- Hypertension
- Osteoarthritis (hands)
- Remote peptic ulcer disease (healed)
- Postmenopausal status
- Cherry angiomas

Her past medical history is significant for hypertension, controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy, omentectomy, and tumor debulking (stage III ovarian cancer)
- Two spontaneous vaginal deliveries (G2P2)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Lisinopril 10 mg PO daily
- Multivitamin 1 tablet PO daily
- Calcium carbonate with Vitamin D 600 mg/400 IU PO daily
- Ibuprofen 200 mg PO PRN pain

ALLERGIES:
NKDA

FAMILY HISTORY:
She has no family history of cancer.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, her abdomen is distended with a fluid wave and a palpable mass in the right lower quadrant. Pelvic examination reveals a fixed, irregular pelvic mass.

LABORATORY DATA:
CA-125 level is 900 U/mL (normal < 35 U/mL).

IMAGING:
A transvaginal ultrasound shows a complex adnexal mass with solid and cystic components.
Abdominal and pelvic CT scan confirms the presence of a large ovarian mass with omental caking and ascites.

## Encounter 4: source 2021-03-15, practice 2022-01-18, outpatient, Obstetrics & Gynecology

Chief complaint: 3-month history of increasing abdominal girth and intermittent pelvic pain

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Emily Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of increasing abdominal girth and intermittent pelvic pain

HISTORY OF PRESENT ILLNESS:
The patient with hypertension returns for follow-up, evaluated on 1/15 and 2/15/2021. She presents to the clinic with a 3-month history of increasing abdominal girth and intermittent pelvic pain. She reports no significant weight loss, changes in bowel habits, or urinary symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis (diagnosed 2020-01-15)
- Cherry hemangioma (diagnosed 2020-07-15)
- Malignant neoplasm of ovary (diagnosed 2021-01-15)
- Hypertension
- Osteoarthritis (hands)
- Remote peptic ulcer disease (healed)
- Postmenopausal status
- Cherry angiomas

Her past medical history is significant for hypertension, controlled with hydrochlorothiazide,
She is gravida 2, para 2, and went through menopause 2 years ago.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy, omentectomy, and tumor debulking (stage III ovarian cancer)
- Two spontaneous vaginal deliveries (G2P2)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Lisinopril 10 mg PO daily
- Multivitamin 1 tablet PO daily
- Calcium carbonate with Vitamin D 600 mg/400 IU PO daily
- Ibuprofen 200 mg PO PRN pain

ALLERGIES:
NKDA

FAMILY HISTORY:
She has no family history of cancer.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, her abdomen is non-tender, but a fixed, irregular pelvic mass is palpable.

LABORATORY DATA:
The following laboratory values are obtained:

- CA-125: 200 U/mL (normal < 35 U/mL)
- Hemoglobin: 13.2 g/dL (normal: 12-16 g/dL)
- White blood cell count: 7,000 /µL (normal: 4,500-11,000 /µL)
- Platelets: 250,000 /µL (normal: 150,000-450,000 /µL)

IMAGING:
A transvaginal ultrasound reveals a 5 cm complex ovarian mass with septations and solid components.

## Encounter 5: source 2021-04-15, practice 2022-02-28, outpatient, Gynecologic Oncology

Chief complaint: 3-month history of increasing abdominal girth and early satiety

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. Lisa Patel, MD
Department: Gynecologic Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of increasing abdominal girth and early satiety

HISTORY OF PRESENT ILLNESS:
This 57-year-old postmenopausal female with hypertension and osteoarthritis returns for follow-up of ongoing symptoms. She presents with a 3-month history of increasing abdominal girth and early satiety. She denies any changes in bowel or bladder habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis (diagnosed 2020-01-15)
- Cherry hemangioma (diagnosed 2020-07-15)
- Malignant neoplasm of ovary (diagnosed 2021-01-15)
- Hypertension
- Osteoarthritis (hands)
- Remote peptic ulcer disease (healed)
- Postmenopausal status
- Cherry angiomas

Her past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy, omentectomy, and tumor debulking (stage III ovarian cancer)
- Two spontaneous vaginal deliveries (G2P2)

MEDICATIONS:
She takes controlled with hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg PO daily
- Multivitamin 1 tablet PO daily
- Calcium carbonate with Vitamin D 600 mg/400 IU PO daily
- Ibuprofen 200 mg PO PRN pain

ALLERGIES:
NKDA

FAMILY HISTORY:
A family history of breast cancer in her sister

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, her abdomen is distended with a fluid wave. Pelvic examination reveals a non-tender, fixed, irregular pelvic mass.

LABORATORY DATA:
CA-125 level is 350 U/mL (normal < 35 U/mL).

IMAGING:
A transvaginal ultrasound shows a complex adnexal mass with solid and cystic components.

## Encounter 6: source 2021-09-15, practice 2022-08-04, outpatient, Hematology & Oncology

Chief complaint: Decreased appetite and 10-pound weight loss following recent completion of chemotherapy

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. James Park, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Decreased appetite and 10-pound weight loss following recent completion of chemotherapy

HISTORY OF PRESENT ILLNESS:
The patient with malignant neoplasm of the ovary returns following recent chemotherapy completion. She completed her last cycle of chemotherapy 2 weeks ago. She reports feeling fatigued and has a decreased interest in her usual activities. She denies nausea, vomiting, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis (diagnosed 2020-01-15)
- Cherry hemangioma (diagnosed 2020-07-15)
- Malignant neoplasm of ovary (diagnosed 2021-01-15)
- Hypertension
- Osteoarthritis (hands)
- Remote peptic ulcer disease (healed)
- Postmenopausal status
- Cherry angiomas

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy, omentectomy, and tumor debulking (stage III ovarian cancer)
- Two spontaneous vaginal deliveries (G2P2)

MEDICATIONS:
Her current medications include a multivitamin, calcium with vitamin D, and ibuprofen as needed for pain.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Lisinopril 10 mg PO daily
- Calcium carbonate with Vitamin D 600 mg/400 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer at age 65
- Father: Hypertension and cerebral aneurysm
- Sister: Ovarian cancer at age 60

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, her BMI is 19 kg/m^2, and she appears cachectic. Her abdomen is soft, non-tender, and without masses or organomegaly.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 12.1 g/dL (normal: 12-15.5 g/dL)
- Albumin: 3.2 g/dL (normal: 3.5-5.0 g/dL)
- Calcium: 9.2 mg/dL (normal: 8.5-10.2 mg/dL)
- Creatinine: 0.9 mg/dL (normal: 0.6-1.1 mg/dL)

## Encounter 7: source 2025-01-15, practice 2025-11-30, ed, Emergency Medicine

Chief complaint: Sudden onset severe headache described as 'worst headache of my life' with double vision

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset severe headache described as 'worst headache of my life' with double vision

HISTORY OF PRESENT ILLNESS:
The patient is a 61-year-old with hypertension, osteoarthritis, and history of ovarian malignancy treated in Gynecologic Oncology. She reports a headache described as the "worst headache of her life," which started 2 hours ago. She also reports double vision.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis (diagnosed 2020-01-15)
- Cherry hemangioma (diagnosed 2020-07-15)
- Malignant neoplasm of ovary (diagnosed 2021-01-15)
- Cachexia (diagnosed 2021-09-15)
- Hypertension
- Osteoarthritis (hands)
- Remote peptic ulcer disease (healed)
- Postmenopausal status
- Cherry angiomas

Her past medical history is significant for hypertension, which is poorly controlled.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy, omentectomy, and tumor debulking (stage III ovarian cancer)
- Two spontaneous vaginal deliveries (G2P2)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Lisinopril 10 mg PO daily
- Multivitamin 1 tablet PO daily
- Calcium carbonate with Vitamin D 600 mg/400 IU PO daily
- Ibuprofen 200 mg PO PRN pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer at age 65
- Father: Hypertension and cerebral aneurysm
- Sister: Ovarian cancer at age 60

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Retired elementary school teacher.

VITAL SIGNS:
On examination, her blood pressure is 180/100 mmHg, heart rate is 88 beats per minute, and she is afebrile.

PHYSICAL EXAMINATION:
Neurological examination reveals ptosis and an outwardly deviated left eye. Pupillary examination shows a dilated left pupil that is non-reactive to light. The remainder of the cranial nerve examination is normal. Fundoscopic examination is unremarkable. There is no neck stiffness, and the rest of the neurological and systemic examinations are normal.

IMAGING:
A non-contrast head CT scan is ordered and shows no evidence of hemorrhage.
