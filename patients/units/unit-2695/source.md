# synthetic_hospital patient 2695

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 56 at first encounter, sex F, Non-Hispanic White, Commercial preferred provider organization (PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-04-27 (Mon) | outpatient | Obstetrics & Gynecology |
| 1 | 2021-01-15 | 2021-04-26 (Mon) | outpatient | Ophthalmology |
| 2 | 2022-01-15 | 2022-04-22 (Fri) | outpatient | Rheumatology |
| 3 | 2023-01-15 | 2023-04-18 (Tue) | outpatient | Internal Medicine |
| 4 | 2024-01-15 | 2024-04-18 (Thu) | ed | Emergency Medicine |
| 5 | 2024-07-15 | 2024-10-19 (Sat) | inpatient | Endocrinology |
| 6 | 2025-01-15 | 2025-04-18 (Fri) | icu | Cardiology |
| 7 | 2026-01-15 | 2026-04-23 (Thu) | outpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Lisinopril (angioedema)",
  "Sulfa drugs (rash)"
 ],
 "insurance": "Commercial preferred provider organization (PPO)",
 "occupation": "Retired administrative assistant",
 "alcohol_use": "Social (1-2 glasses of wine weekly)",
 "family_history": [
  "Mother: Endometrial cancer (diagnosed age 62)",
  "Father: Type 2 diabetes mellitus with diabetic retinopathy",
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
  }
 ],
 "surgical_history": [
  "Cesarean section (1988)",
  "Laparoscopic appendectomy (1995)"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus (14-year history)",
  "Hypertension",
  "Central diabetes insipidus",
  "History of endometrial carcinoma"
 ],
 "age_at_first_encounter": 56
}
```

Primary diagnoses: ["Endometrial carcinoma", "Osteoarthritis of knee", "Assessment of decision-making capacity", "Diabetic retinopathy", "Syndrome of inappropriate antidiuretic hormone secretion (SIADH)", "Lisinopril adverse effect", "Acute mitral regurgitation"]
Comorbidities: ["Morbid obesity", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2020-04-27, outpatient, Obstetrics & Gynecology

Chief complaint: Irregular, heavy menstrual bleeding for 6 months requiring frequent pad changes, now with 2 months amenorrhea and spotting

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Irregular, heavy menstrual bleeding for 6 months requiring frequent pad changes, now with 2 months amenorrhea and spotting

HISTORY OF PRESENT ILLNESS:
She reports that her menstrual cycles have been unpredictable, with intervals ranging from 21 to 45 days, and the bleeding is often so heavy that she has to change her sanitary pads every 1-2 hours. She has not had a menstrual period for the past 2 months but has experienced spotting over the last week.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (14-year history)
- Hypertension
- Central diabetes insipidus
- History of endometrial carcinoma

Her past medical history is significant for type 2 diabetes mellitus and hypertension.

PAST SURGICAL HISTORY:
- Cesarean section (1988)
- Laparoscopic appendectomy (1995)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily

ALLERGIES:
Lisinopril (angioedema), Sulfa drugs (rash)

FAMILY HISTORY:
- Mother: Endometrial cancer (diagnosed age 62)
- Father: Type 2 diabetes mellitus with diabetic retinopathy
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social (1-2 glasses of wine weekly). Occupation: Retired administrative assistant.

VITAL SIGNS:
She has a BMI of 35 kg/m^2. On physical examination, her blood pressure is 142/90 mmHg.

PHYSICAL EXAMINATION:
A bimanual pelvic exam reveals an enlarged, nontender uterus without adnexal masses. No cervical lesions or discharge is noted.

LABORATORY DATA:
Laboratory tests reveal:
- Hemoglobin: 10.2 g/dL (Normal: 12-16 g/dL)
- Platelets: 250 x 10^3/µL (Normal: 150-400 x 10^3/µL)
- Fasting glucose: 162 mg/dL (Normal: 70-99 mg/dL)
- HbA1c: 7.8% (Normal: <5.7% for non-diabetics)

OTHER STUDIES:
An endometrial biopsy is performed.

## Encounter 1: source 2021-01-15, practice 2021-04-26, outpatient, Ophthalmology

Chief complaint: Gradual, bilateral vision decline over the past 18 months; no ophthalmology care in over 10 years

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Ophthalmology
Visit Type: outpatient

CHIEF COMPLAINT:
Gradual, bilateral vision decline over the past 18 months; no ophthalmology care in over 10 years

HISTORY OF PRESENT ILLNESS:
This 57-year-old female with Type 2 diabetes and prior endometrial carcinoma presents for evaluation of gradual, bilateral vision decline over the past 18 months. She has not seen an ophthalmologist in over 10 years.

PAST MEDICAL HISTORY:
Active Problem List:
- Endometrial carcinoma (diagnosed 2020-01-15)
- Type 2 diabetes mellitus (14-year history)
- Hypertension
- Central diabetes insipidus

PAST SURGICAL HISTORY:
- Cesarean section (1988)
- Laparoscopic appendectomy (1995)

MEDICATIONS:
Her current medications include metformin and insulin glargine.

Home Medications:
- Lisinopril 10 mg PO daily

ALLERGIES:
Lisinopril (angioedema), Sulfa drugs (rash)

FAMILY HISTORY:
- Mother: Endometrial cancer (diagnosed age 62)
- Father: Type 2 diabetes mellitus with diabetic retinopathy
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social (1-2 glasses of wine weekly). Occupation: Retired administrative assistant.

VITAL SIGNS:
Blood pressure is 134/82 mmHg.

PHYSICAL EXAMINATION:
Physical examination reveals decreased visual acuity bilaterally and no focal neurological deficits.
Fundoscopic examination is performed, and the image is provided below:

LABORATORY DATA:
HbA1c is 11.7%. Urinalysis reveals microalbuminuria.

## Encounter 2: source 2022-01-15, practice 2022-04-22, outpatient, Rheumatology

Chief complaint: Bilateral knee pain with activity for 6 months, morning stiffness lasting 20 minutes

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Jennifer Walsh, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Bilateral knee pain with activity for 6 months, morning stiffness lasting 20 minutes

HISTORY OF PRESENT ILLNESS:
This 58-year-old female with type 2 diabetes mellitus, hypertension, central diabetes insipidus, and history of endometrial carcinoma presents to Rheumatology following prior ophthalmology and oncology care. She reports a 6-month history of bilateral knee pain that worsens with activity and improves with rest. She notes morning stiffness lasting approximately 20 minutes.

PAST MEDICAL HISTORY:
Active Problem List:
- Endometrial carcinoma (diagnosed 2020-01-15)
- Diabetic retinopathy (diagnosed 2021-01-15)
- Type 2 diabetes mellitus (14-year history)
- Hypertension
- Central diabetes insipidus

Her medical history is significant for hypertension, controlled with hydrochlorothiazide, and type 2 diabetes mellitus, managed with metformin.

PAST SURGICAL HISTORY:
- Cesarean section (1988)
- Laparoscopic appendectomy (1995)

MEDICATIONS:
She takes hydrochlorothiazide, metformin.

Home Medications:
- Lisinopril 10 mg PO daily

ALLERGIES:
Lisinopril (angioedema), Sulfa drugs (rash)

FAMILY HISTORY:
- Mother: Endometrial cancer (diagnosed age 62)
- Father: Type 2 diabetes mellitus with diabetic retinopathy
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social (1-2 glasses of wine weekly). Occupation: Retired administrative assistant.

VITAL SIGNS:
BMI of 32 kg/m^2

PHYSICAL EXAMINATION:
On physical examination, there is no erythema or warmth, but there is bony enlargement of both knee joints with tenderness on palpation. There is no effusion, and the range of motion is preserved but painful at the extremes.

IMAGING:
Radiographs of the knees show joint space narrowing, subchondral sclerosis, and osteophyte formation.

## Encounter 3: source 2023-01-15, practice 2023-04-18, outpatient, Internal Medicine

Chief complaint: Routine health maintenance examination with no acute complaints

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James Park, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health maintenance examination with no acute complaints

HISTORY OF PRESENT ILLNESS:
The patient is a 59-year-old female with a 14-year history of type 2 diabetes mellitus, hypertension, central diabetes insipidus, and history of endometrial carcinoma who previously presented with diabetic retinopathy and osteoarthritis. She returns for routine health maintenance and reports no acute complaints today.

PAST MEDICAL HISTORY:
Active Problem List:
- Endometrial carcinoma (diagnosed 2020-01-15)
- Diabetic retinopathy (diagnosed 2021-01-15)
- Osteoarthritis of knee (diagnosed 2022-01-15)
- Type 2 diabetes mellitus (14-year history)
- Hypertension
- Central diabetes insipidus

She has a history of hypertension, controlled with lisinopril, and type 2 diabetes mellitus, managed with metformin.

PAST SURGICAL HISTORY:
- Cesarean section (1988)
- Laparoscopic appendectomy (1995)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily

ALLERGIES:
Lisinopril (angioedema), Sulfa drugs (rash)

FAMILY HISTORY:
- Mother: Endometrial cancer (diagnosed age 62)
- Father: Type 2 diabetes mellitus with diabetic retinopathy
- Sister: Hypertension

SOCIAL HISTORY:
Her daughter accompanies her but has to leave early for work.

VITAL SIGNS:
The patient's vital signs are within normal limits, and her BMI is 31 kg/m^2.

PHYSICAL EXAMINATION:
On physical examination, you note no abnormalities except for mild acanthosis nigricans around the neck.
The patient appears anxious and has difficulty understanding your explanation of the procedure due to her limited English proficiency.

ASSESSMENT AND PLAN:
You consider the best way to communicate effectively with the patient to obtain informed consent for the breast examination.

You decide to perform a breast examination as part of the routine check-up.

## Encounter 4: source 2024-01-15, practice 2024-04-18, ed, Emergency Medicine

Chief complaint: Sudden onset of progressive lip and tongue swelling over several hours without urticaria or respiratory distress

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of progressive lip and tongue swelling over several hours without urticaria or respiratory distress

HISTORY OF PRESENT ILLNESS:
This patient with Type 2 diabetes, hypertension, and endometrial cancer presents to the Emergency Department. She reports waking this morning with the swelling, which has progressively worsened over the past few hours. She denies any associated urticaria, pruritus, or respiratory distress.

PAST MEDICAL HISTORY:
Active Problem List:
- Endometrial carcinoma (diagnosed 2020-01-15)
- Diabetic retinopathy (diagnosed 2021-01-15)
- Osteoarthritis of knee (diagnosed 2022-01-15)
- Assessment of decision-making capacity (diagnosed 2023-01-15)
- Type 2 diabetes mellitus (14-year history)
- Hypertension
- Central diabetes insipidus

Her medical history is significant for hypertension, for which she takes lisinopril, and type 2 diabetes mellitus controlled with metformin.

PAST SURGICAL HISTORY:
- Cesarean section (1988)
- Laparoscopic appendectomy (1995)

MEDICATIONS:
She has been on lisinopril for 2 years.

Home Medications:
- Metformin 1000 mg PO BID

ALLERGIES:
She has no known drug allergies

FAMILY HISTORY:
- Mother: Endometrial cancer (diagnosed age 62)
- Father: Type 2 diabetes mellitus with diabetic retinopathy
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social (1-2 glasses of wine weekly). Occupation: Retired administrative assistant.

VITAL SIGNS:
On physical examination, her blood pressure is 145/90 mmHg, heart rate is 78 beats per minute, and respiratory rate is 16 beats per minute.

PHYSICAL EXAMINATION:
There is noticeable non-pitting edema of the lips and tongue without erythema or warmth. The remainder of the examination is unremarkable.

LABORATORY DATA:
Laboratory studies reveal:

- Serum sodium: 139 mEq/L
- Serum potassium: 4.8 mEq/L
- Serum creatinine: 1.1 mg/dL
- Complete blood count: within normal limits

## Encounter 5: source 2024-07-15, practice 2024-10-19, inpatient, Endocrinology

Chief complaint: 2-day history of headache, nausea, and confusion following recent desmopressin dose increase

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Robert Kim, MD
Department: Endocrinology
Visit Type: inpatient

CHIEF COMPLAINT:
2-day history of headache, nausea, and confusion following recent desmopressin dose increase

HISTORY OF PRESENT ILLNESS:
This patient with diabetes insipidus and endometrial carcinoma presents with headache, nausea, and confusion. She notes symptom onset shortly after her endocrinologist increased desmopressin dose for persistent polyuria and polydipsia.

PAST MEDICAL HISTORY:
Active Problem List:
- Endometrial carcinoma (diagnosed 2020-01-15)
- Diabetic retinopathy (diagnosed 2021-01-15)
- Osteoarthritis of knee (diagnosed 2022-01-15)
- Assessment of decision-making capacity (diagnosed 2023-01-15)
- Lisinopril adverse effect (diagnosed 2024-01-15)
- Type 2 diabetes mellitus (14-year history)
- Hypertension
- Central diabetes insipidus

PAST SURGICAL HISTORY:
- Cesarean section (1988)
- Laparoscopic appendectomy (1995)

MEDICATIONS:
Her medications include metformin, lisinopril, and desmopressin.

ALLERGIES:
Lisinopril (angioedema), Sulfa drugs (rash)

FAMILY HISTORY:
- Mother: Endometrial cancer (diagnosed age 62)
- Father: Type 2 diabetes mellitus with diabetic retinopathy
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social (1-2 glasses of wine weekly). Occupation: Retired administrative assistant.

VITAL SIGNS:
Her blood pressure is 130/85 mmHg, heart rate is 78 beats per minute

PHYSICAL EXAMINATION:
On physical examination, she is oriented to person only. Her mucous membranes are moist, and there is no peripheral edema.

LABORATORY DATA:
Laboratory studies are as follows:

- Serum sodium: 125 mEq/L (Normal: 135-145 mEq/L)
- Serum potassium: 4.2 mEq/L (Normal: 3.5-5.1 mEq/L)
- Serum osmolality: 260 mOsm/kg (Normal: 275-295 mOsm/kg)
- Urine osmolality: 600 mOsm/kg (Normal: 50-1200 mOsm/kg)
- Urine sodium: 80 mEq/L (Normal: 20-40 mEq/L)

## Encounter 6: source 2025-01-15, practice 2025-04-18, icu, Cardiology

Chief complaint: Acute onset of severe shortness of breath, orthopnea, and palpitations beginning 2 hours ago while gardening

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Emily Johnson, MD
Department: Cardiology
Visit Type: icu

CHIEF COMPLAINT:
Acute onset of severe shortness of breath, orthopnea, and palpitations beginning 2 hours ago while gardening

HISTORY OF PRESENT ILLNESS:
This patient with diabetes, hypertension, prior cancer, and SIADH presents to the emergency department with acute onset of shortness of breath, orthopnea, and palpitations beginning 2 hours ago while gardening

PAST MEDICAL HISTORY:
Active Problem List:
- Endometrial carcinoma (diagnosed 2020-01-15)
- Diabetic retinopathy (diagnosed 2021-01-15)
- Osteoarthritis of knee (diagnosed 2022-01-15)
- Assessment of decision-making capacity (diagnosed 2023-01-15)
- Lisinopril adverse effect (diagnosed 2024-01-15)
- Syndrome of inappropriate antidiuretic hormone secretion (SIADH) (diagnosed 2024-07-15)
- Type 2 diabetes mellitus (14-year history)
- Hypertension
- Central diabetes insipidus

She has a past medical history of hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Cesarean section (1988)
- Laparoscopic appendectomy (1995)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily

ALLERGIES:
Lisinopril (angioedema), Sulfa drugs (rash)

FAMILY HISTORY:
- Mother: Endometrial cancer (diagnosed age 62)
- Father: Type 2 diabetes mellitus with diabetic retinopathy
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social (1-2 glasses of wine weekly). Occupation: Retired administrative assistant.

VITAL SIGNS:
On examination, she is tachypneic with a respiratory rate of 28 breaths per minute, blood pressure of 150/90 mmHg, and heart rate of 110 beats per minute.

PHYSICAL EXAMINATION:
Auscultation of the lungs reveals bilateral rales halfway up both lung fields. Cardiac examination shows a new holosystolic murmur at the apex that radiates to the axilla.

LABORATORY DATA:
Laboratory values are as follows:

- Troponin I: <0.04 ng/mL (normal)
- B-type natriuretic peptide (BNP): 900 pg/mL (normal <100 pg/mL)
- Creatinine: 1.1 mg/dL (normal 0.6-1.2 mg/dL)
- Blood urea nitrogen (BUN): 18 mg/dL (normal 7-20 mg/dL)

IMAGING:
Chest X-ray demonstrates pulmonary venous congestion and interstitial edema.

OTHER STUDIES:
The electrocardiogram shows sinus tachycardia with no ST-segment changes.

## Encounter 7: source 2026-01-15, practice 2026-04-23, outpatient, Obstetrics & Gynecology

Chief complaint: 3-month history of intermittent vaginal bleeding in a patient with prior endometrial carcinoma

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of intermittent vaginal bleeding in a patient with prior endometrial carcinoma

HISTORY OF PRESENT ILLNESS:
The patient returns to Obstetrics and Gynecology six years after her initial endometrial carcinoma diagnosis. She reports a 3-month history of intermittent vaginal bleeding. Her last menstrual period was 6 years ago. She denies associated pelvic pain, weight loss, or bowel habit changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Endometrial carcinoma (diagnosed 2020-01-15)
- Diabetic retinopathy (diagnosed 2021-01-15)
- Osteoarthritis of knee (diagnosed 2022-01-15)
- Assessment of decision-making capacity (diagnosed 2023-01-15)
- Lisinopril adverse effect (diagnosed 2024-01-15)
- Syndrome of inappropriate antidiuretic hormone secretion (SIADH) (diagnosed 2024-07-15)
- Acute mitral regurgitation (diagnosed 2025-01-15)
- Type 2 diabetes mellitus (14-year history)
- Hypertension
- Central diabetes insipidus

PAST SURGICAL HISTORY:
- Cesarean section (1988)
- Laparoscopic appendectomy (1995)

MEDICATIONS:
She takes hydrochlorothiazide, metformin.

Home Medications:
- Lisinopril 10 mg PO daily

ALLERGIES:
Lisinopril (angioedema), Sulfa drugs (rash)

FAMILY HISTORY:
No family history of gynecologic cancers

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social (1-2 glasses of wine weekly). Occupation: Retired administrative assistant.

VITAL SIGNS:
Blood pressure is 132/86 mmHg, and her BMI is 31 kg/m^2

PHYSICAL EXAMINATION:
A pelvic examination reveals a small amount of blood in the vaginal vault, but no masses are palpable, and the uterus is of normal size. No adnexal tenderness or masses are noted.

LABORATORY DATA:
Hemoglobin: 12.1 g/dL
- Platelet count: 250 x 10^3/uL
- Fasting glucose: 146 mg/dL
- Serum creatinine: 0.9 mg/dL
