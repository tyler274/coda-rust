# synthetic_hospital patient 2715

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 57 at first encounter, sex F, Non-Hispanic White, Medicare (disability status due to rheumatoid arthritis) and supplemental commercial insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-12-06 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2022-03-09 (Wed) | outpatient | Cardiology |
| 2 | 2021-01-15 | 2022-12-05 (Mon) | outpatient | Breast Surgery |
| 3 | 2022-01-15 | 2023-11-24 (Fri) | outpatient | Obstetrics & Gynecology |
| 4 | 2023-01-15 | 2024-11-29 (Fri) | outpatient | Nephrology |
| 5 | 2023-07-15 | 2025-05-29 (Thu) | ed | Emergency Medicine |
| 6 | 2024-01-15 | 2025-12-03 (Wed) | ed | Emergency Medicine |
| 7 | 2024-07-15 | 2026-06-04 (Thu) | inpatient | General Surgery |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare (disability status due to rheumatoid arthritis) and supplemental commercial insurance",
 "occupation": "Part-time bookkeeping clerk",
 "alcohol_use": "Occasional (1-2 glasses of wine per month)",
 "family_history": [
  "Mother: Type 2 diabetes mellitus, hypertension, stroke at age 72",
  "Father: Coronary artery disease, myocardial infarction at age 68",
  "Sister: Rheumatoid arthritis (seropositive)",
  "Maternal aunt: Breast cancer (invasive ductal carcinoma, age 62)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Active smoker, 30 pack-years (approximately 1 pack per day)",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "15 mg PO weekly",
   "name": "Methotrexate"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Prednisone"
  },
  {
   "dose": "325 mg PO daily",
   "name": "Ferrous sulfate"
  },
  {
   "dose": "1 mg PO daily",
   "name": "Folic acid"
  }
 ],
 "surgical_history": [
  "Screening colonoscopy with polypectomy (age 50)",
  "Cesarean section (age 28)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 diabetes mellitus (10-year duration, poorly controlled)",
  "Rheumatoid arthritis (20-year duration)",
  "Iron deficiency anemia",
  "Osteopenia secondary to long-term corticosteroid use"
 ],
 "age_at_first_encounter": 57
}
```

Primary diagnoses: ["Paget disease of the breast", "Benign paroxysmal positional vertigo", "AA amyloidosis", "Poorly controlled diabetes mellitus", "Rhegmatogenous retinal detachment", "Calcium channel blocker-induced peripheral edema", "Cervical squamous cell carcinoma", "Transfusion-related acute lung injury"]
Comorbidities: ["Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2021-12-06, outpatient, Internal Medicine

Chief complaint: Routine follow-up for type 2 diabetes mellitus with fatigue and post-prandial thirst

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for type 2 diabetes mellitus with fatigue and post-prandial thirst

HISTORY OF PRESENT ILLNESS:
She reports adherence to her diet and exercise regimen, but notes that she often feels fatigued and thirsty, particularly after meals. She denies any recent weight changes, polyuria, or nocturia.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 diabetes mellitus (10-year duration, poorly controlled)
- Rheumatoid arthritis (20-year duration)
- Iron deficiency anemia
- Osteopenia secondary to long-term corticosteroid use

PAST SURGICAL HISTORY:
- Screening colonoscopy with polypectomy (age 50)
- Cesarean section (age 28)

MEDICATIONS:
Her current medications include metformin 1000 mg twice daily and glargine insulin 20 units at bedtime.

Home Medications:
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Insulin glargine 20 units subcutaneous at bedtime
- Methotrexate 15 mg PO weekly
- Prednisone 5 mg PO daily
- Ferrous sulfate 325 mg PO daily
- Folic acid 1 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus, hypertension, stroke at age 72
- Father: Coronary artery disease, myocardial infarction at age 68
- Sister: Rheumatoid arthritis (seropositive)
- Maternal aunt: Breast cancer (invasive ductal carcinoma, age 62)

SOCIAL HISTORY:
Smoking: Active smoker, 30 pack-years (approximately 1 pack per day). Alcohol: Occasional (1-2 glasses of wine per month). Occupation: Part-time bookkeeping clerk.

VITAL SIGNS:
Her blood pressure is 132/86 mmHg, heart rate is 78 beats per minute, and BMI is 29 kg/m^2.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
Laboratory results are as follows:

- Fasting plasma glucose: 98 mg/dL (normal: 70-99 mg/dL)
- Hemoglobin A1c: 8.5% (normal: <5.7% for non-diabetics)
- Postprandial plasma glucose (2 hours after a meal): 220 mg/dL (normal: <140 mg/dL)

## Encounter 1: source 2020-04-15, practice 2022-03-09, outpatient, Cardiology

Chief complaint: Progressive bilateral lower extremity swelling over two months

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Robert Martinez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive bilateral lower extremity swelling over two months

HISTORY OF PRESENT ILLNESS:
The patient returns for cardiology evaluation three months after her diabetes visit. She presents with bilateral lower extremity swelling that has progressively worsened over the past two months. She reports that the swelling is more pronounced by the end of the day and improves slightly with elevation overnight. She denies any chest pain, dyspnea, or recent travel.

PAST MEDICAL HISTORY:
Active Problem List:
- Poorly controlled diabetes mellitus (diagnosed 2020-01-15)
- Hypertension
- Type 2 diabetes mellitus (10-year duration, poorly controlled)
- Rheumatoid arthritis (20-year duration)
- Iron deficiency anemia
- Osteopenia secondary to long-term corticosteroid use

PAST SURGICAL HISTORY:
- Screening colonoscopy with polypectomy (age 50)
- Cesarean section (age 28)

MEDICATIONS:
Her current medications include amlodipine 10 mg daily and hydrochlorothiazide 25 mg daily.

Home Medications:
- Metformin 1000 mg PO twice daily
- Insulin glargine 20 units subcutaneous at bedtime
- Methotrexate 15 mg PO weekly
- Prednisone 5 mg PO daily
- Ferrous sulfate 325 mg PO daily
- Folic acid 1 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus, hypertension, stroke at age 72
- Father: Coronary artery disease, myocardial infarction at age 68
- Sister: Rheumatoid arthritis (seropositive)
- Maternal aunt: Breast cancer (invasive ductal carcinoma, age 62)

SOCIAL HISTORY:
Smoking: Active smoker, 30 pack-years (approximately 1 pack per day). Alcohol: Occasional (1-2 glasses of wine per month). Occupation: Part-time bookkeeping clerk.

VITAL SIGNS:
On physical examination, her blood pressure is 142/88 mmHg, heart rate is 78 beats per minute, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
There is 2+ pitting edema up to the mid-shins bilaterally. The remainder of the examination is unremarkable.

LABORATORY DATA:
Laboratory values are as follows:

- Serum sodium: 140 mEq/L
- Serum potassium: 4.2 mEq/L
- Serum creatinine: 1.0 mg/dL
- Blood urea nitrogen (BUN): 14 mg/dL
- Serum albumin: 4.5 g/dL
- Urinalysis: No proteinuria or hematuria

## Encounter 2: source 2021-01-15, practice 2022-12-05, outpatient, Breast Surgery

Chief complaint: Persistent, itchy, scaly rash on right nipple and areola for six months with occasional weeping

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Aisha Patel, MD
Department: Breast Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent, itchy, scaly rash on right nipple and areola for six months with occasional weeping

HISTORY OF PRESENT ILLNESS:
This 58-year-old female with poorly controlled type 2 diabetes mellitus and rheumatoid arthritis on corticosteroid therapy presents with a six-month history of persistent, itchy rash on her right nipple and areola. She reports that the rash initially appeared as a small, red, scaly patch but has since become more extensive and occasionally weeps clear fluid. She denies any nipple discharge or palpable masses.

PAST MEDICAL HISTORY:
Active Problem List:
- Poorly controlled diabetes mellitus (diagnosed 2020-01-15)
- Calcium channel blocker-induced peripheral edema (diagnosed 2020-04-15)
- Hypertension
- Type 2 diabetes mellitus (10-year duration, poorly controlled)
- Rheumatoid arthritis (20-year duration)
- Iron deficiency anemia
- Osteopenia secondary to long-term corticosteroid use

PAST SURGICAL HISTORY:
- Screening colonoscopy with polypectomy (age 50)
- Cesarean section (age 28)

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Amlodipine 10 mg PO daily
- Metformin 1000 mg PO twice daily
- Insulin glargine 20 units subcutaneous at bedtime
- Methotrexate 15 mg PO weekly
- Prednisone 5 mg PO daily
- Ferrous sulfate 325 mg PO daily
- Folic acid 1 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
No family history of breast cancer

SOCIAL HISTORY:
Smoking: Active smoker, 30 pack-years (approximately 1 pack per day). Alcohol: Occasional (1-2 glasses of wine per month). Occupation: Part-time bookkeeping clerk.

PHYSICAL EXAMINATION:
On physical examination, the right nipple-areolar complex is erythematous, with areas of excoriation and a small ulceration. There is no palpable mass in either breast, and axillary lymph nodes are non-palpable.

IMAGING:
A mammogram shows a subtle area of increased density in the right breast, without distinct masses or calcifications.

OTHER STUDIES:
A punch biopsy of the affected skin is performed.

## Encounter 3: source 2022-01-15, practice 2023-11-24, outpatient, Obstetrics & Gynecology

Chief complaint: Irregular vaginal bleeding and postcoital spotting for six months

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Jennifer Park, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Irregular vaginal bleeding and postcoital spotting for six months

HISTORY OF PRESENT ILLNESS:
This 59-year-old with poorly controlled diabetes, rheumatoid arthritis, and prior Paget disease returns reporting that the bleeding is not associated with pain but has progressively worsened over the past two months. She has not had a Pap smear in over 10 years.

PAST MEDICAL HISTORY:
Active Problem List:
- Poorly controlled diabetes mellitus (diagnosed 2020-01-15)
- Calcium channel blocker-induced peripheral edema (diagnosed 2020-04-15)
- Paget disease of the breast (diagnosed 2021-01-15)
- Hypertension
- Type 2 diabetes mellitus (10-year duration, poorly controlled)
- Rheumatoid arthritis (20-year duration)
- Iron deficiency anemia
- Osteopenia secondary to long-term corticosteroid use

Her medical history is significant for hypertension, controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Screening colonoscopy with polypectomy (age 50)
- Cesarean section (age 28)

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Amlodipine 10 mg PO daily
- Metformin 1000 mg PO twice daily
- Insulin glargine 20 units subcutaneous at bedtime
- Methotrexate 15 mg PO weekly
- Prednisone 5 mg PO daily
- Ferrous sulfate 325 mg PO daily
- Folic acid 1 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus, hypertension, stroke at age 72
- Father: Coronary artery disease, myocardial infarction at age 68
- Sister: Rheumatoid arthritis (seropositive)
- Maternal aunt: Breast cancer (invasive ductal carcinoma, age 62)

SOCIAL HISTORY:
She has a 30-pack-year smoking history

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, a 3 cm exophytic lesion is noted on the anterior lip of the cervix. The lesion is friable and bleeds easily upon contact. A pelvic exam does not reveal any vaginal or parametrial involvement.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 10.2 g/dL (Normal: 12-16 g/dL)
- White blood cell count: 7,000/uL (Normal: 4,500-11,000/uL)
- Platelets: 250,000/uL (Normal: 150,000-450,000/uL)

OTHER STUDIES:
A biopsy of the cervical lesion is performed.

## Encounter 4: source 2023-01-15, practice 2024-11-29, outpatient, Nephrology

Chief complaint: Six-month history of increasing fatigue, abdominal bloating, and lower extremity edema

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Kim, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of increasing fatigue, abdominal bloating, and lower extremity edema

HISTORY OF PRESENT ILLNESS:
The patient with rheumatoid arthritis and prior peripheral edema presents with six-month history of increasing fatigue, abdominal bloating, and lower extremity edema.

PAST MEDICAL HISTORY:
Active Problem List:
- Poorly controlled diabetes mellitus (diagnosed 2020-01-15)
- Calcium channel blocker-induced peripheral edema (diagnosed 2020-04-15)
- Paget disease of the breast (diagnosed 2021-01-15)
- Cervical squamous cell carcinoma (diagnosed 2022-01-15)
- Hypertension
- Type 2 diabetes mellitus (10-year duration, poorly controlled)
- Rheumatoid arthritis (20-year duration)
- Iron deficiency anemia
- Osteopenia secondary to long-term corticosteroid use

PAST SURGICAL HISTORY:
- Screening colonoscopy with polypectomy (age 50)
- Cesarean section (age 28)

MEDICATIONS:
She has been on methotrexate and low-dose prednisone for the past 15 years.

Home Medications:
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Metformin 1000 mg PO twice daily
- Insulin glargine 20 units subcutaneous at bedtime
- Ferrous sulfate 325 mg PO daily
- Folic acid 1 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus, hypertension, stroke at age 72
- Father: Coronary artery disease, myocardial infarction at age 68
- Sister: Rheumatoid arthritis (seropositive)
- Maternal aunt: Breast cancer (invasive ductal carcinoma, age 62)

SOCIAL HISTORY:
Smoking: Active smoker, 30 pack-years (approximately 1 pack per day). Alcohol: Occasional (1-2 glasses of wine per month). Occupation: Part-time bookkeeping clerk.

VITAL SIGNS:
Blood pressure is 140/90 mmHg

PHYSICAL EXAMINATION:
2+ pitting edema up to her knees

LABORATORY DATA:
Serum albumin: 2.8 g/dL (Normal: 3.5-5.0 g/dL)
Serum creatinine: 1.2 mg/dL (Normal: 0.6-1.1 mg/dL)
Urine protein: 4.5 g/day (Normal: <150 mg/day)
Urine microscopy: No cells or casts
Serum C-reactive protein: 25 mg/L (Normal: <10 mg/L)

IMAGING:
Echocardiogram: Normal left ventricular function, no valvular abnormalities

PATHOLOGY:
An abdominal fat pad biopsy is performed, which shows amorphous hyaline material that stains with Congo red and exhibits apple-green birefringence under polarized light.

## Encounter 5: source 2023-07-15, practice 2025-05-29, ed, Emergency Medicine

Chief complaint: Recurrent episodes of severe dizziness for two weeks triggered by positional changes

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Recurrent episodes of severe dizziness for two weeks triggered by positional changes

HISTORY OF PRESENT ILLNESS:
The patient is a 60-year-old female with rheumatoid arthritis, type 2 diabetes mellitus, and recently diagnosed AA amyloidosis who presents with recurrent episodes of severe dizziness. She reports that the dizziness occurs suddenly when she turns her head to the right while lying down or when she looks up to reach for items on a high shelf. Each episode lasts for less than a minute but is severe enough to cause nausea. She denies any hearing loss, tinnitus, or neurologic symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Poorly controlled diabetes mellitus (diagnosed 2020-01-15)
- Calcium channel blocker-induced peripheral edema (diagnosed 2020-04-15)
- Paget disease of the breast (diagnosed 2021-01-15)
- Cervical squamous cell carcinoma (diagnosed 2022-01-15)
- AA amyloidosis (diagnosed 2023-01-15)
- Hypertension
- Type 2 diabetes mellitus (10-year duration, poorly controlled)
- Rheumatoid arthritis (20-year duration)
- Iron deficiency anemia
- Osteopenia secondary to long-term corticosteroid use

Her past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Screening colonoscopy with polypectomy (age 50)
- Cesarean section (age 28)

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Amlodipine 10 mg PO daily
- Metformin 1000 mg PO twice daily
- Insulin glargine 20 units subcutaneous at bedtime
- Methotrexate 15 mg PO weekly
- Prednisone 5 mg PO daily
- Ferrous sulfate 325 mg PO daily
- Folic acid 1 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus, hypertension, stroke at age 72
- Father: Coronary artery disease, myocardial infarction at age 68
- Sister: Rheumatoid arthritis (seropositive)
- Maternal aunt: Breast cancer (invasive ductal carcinoma, age 62)

SOCIAL HISTORY:
Smoking: Active smoker, 30 pack-years (approximately 1 pack per day). Alcohol: Occasional (1-2 glasses of wine per month). Occupation: Part-time bookkeeping clerk.

VITAL SIGNS:
Her blood pressure is 130/85 mmHg

PHYSICAL EXAMINATION:
On physical examination, her cardiovascular and neurologic examinations are unremarkable. When performing the Dix-Hallpike maneuver, the patient experiences a burst of nystagmus that lasts for less than a minute and resolves spontaneously.

## Encounter 6: source 2024-01-15, practice 2025-12-03, ed, Emergency Medicine

Chief complaint: Sudden, painless loss of vision in right eye for three hours after minor head trauma

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Maria Santos, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden, painless loss of vision in right eye for three hours after minor head trauma

HISTORY OF PRESENT ILLNESS:
The patient is a 61-year-old female with rheumatoid arthritis, type 2 diabetes mellitus, and a history of AA amyloidosis who presents to the emergency department for sudden, painless loss of vision in her right eye that began 3 hours ago after she accidentally struck her head on a door frame. She reports no preceding flashes of light or floaters.

PAST MEDICAL HISTORY:
Active Problem List:
- Poorly controlled diabetes mellitus (diagnosed 2020-01-15)
- Calcium channel blocker-induced peripheral edema (diagnosed 2020-04-15)
- Paget disease of the breast (diagnosed 2021-01-15)
- Cervical squamous cell carcinoma (diagnosed 2022-01-15)
- AA amyloidosis (diagnosed 2023-01-15)
- Benign paroxysmal positional vertigo (diagnosed 2023-07-15)
- Hypertension
- Type 2 diabetes mellitus (10-year duration, poorly controlled)
- Rheumatoid arthritis (20-year duration)
- Iron deficiency anemia
- Osteopenia secondary to long-term corticosteroid use

Her past medical history is significant for poorly controlled type 2 diabetes mellitus and long-standing hypertension.

PAST SURGICAL HISTORY:
- Screening colonoscopy with polypectomy (age 50)
- Cesarean section (age 28)

MEDICATIONS:
Home Medications:
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Metformin 1000 mg PO twice daily
- Insulin glargine 20 units subcutaneous at bedtime
- Methotrexate 15 mg PO weekly
- Prednisone 5 mg PO daily
- Ferrous sulfate 325 mg PO daily
- Folic acid 1 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus, hypertension, stroke at age 72
- Father: Coronary artery disease, myocardial infarction at age 68
- Sister: Rheumatoid arthritis (seropositive)
- Maternal aunt: Breast cancer (invasive ductal carcinoma, age 62)

SOCIAL HISTORY:
Smoking: Active smoker, 30 pack-years (approximately 1 pack per day). Alcohol: Occasional (1-2 glasses of wine per month). Occupation: Part-time bookkeeping clerk.

PHYSICAL EXAMINATION:
On examination, her right eye has a visual acuity of 20/250, while the left eye is 20/25. Fundoscopic examination is shown below.
Intraocular pressure is within normal limits, and there is no evidence of pain, redness, or photophobia. The left eye appears normal on examination.

## Encounter 7: source 2024-07-15, practice 2026-06-04, inpatient, General Surgery

Chief complaint: Worsening abdominal pain, fever, and acute drop in hemoglobin requiring transfusion with subsequent respiratory distress

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Michael Brown, MD
Department: General Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Worsening abdominal pain, fever, and acute drop in hemoglobin requiring transfusion with subsequent respiratory distress

HISTORY OF PRESENT ILLNESS:
The patient is a 61-year-old female with rheumatoid arthritis on immunosuppressive therapy, poorly controlled diabetes mellitus, AA amyloidosis, and cervical squamous cell carcinoma who presented six months ago for retinal detachment. She is admitted for evaluation of worsening abdominal pain and fever. She is found to have complicated diverticulitis with localized perforation requiring intravenous antibiotics and supportive care. Over the next 48 hours, her hemoglobin drops from 10.2 g/dL to 7.4 g/dL due to ongoing gastrointestinal blood loss, and she receives multiple units of packed red blood cells. Several hours after her third transfusion, she becomes increasingly short of breath.

PAST MEDICAL HISTORY:
Active Problem List:
- Poorly controlled diabetes mellitus (diagnosed 2020-01-15)
- Calcium channel blocker-induced peripheral edema (diagnosed 2020-04-15)
- Paget disease of the breast (diagnosed 2021-01-15)
- Cervical squamous cell carcinoma (diagnosed 2022-01-15)
- AA amyloidosis (diagnosed 2023-01-15)
- Benign paroxysmal positional vertigo (diagnosed 2023-07-15)
- Rhegmatogenous retinal detachment (diagnosed 2024-01-15)
- Hypertension
- Type 2 diabetes mellitus (10-year duration, poorly controlled)
- Rheumatoid arthritis (20-year duration)
- Iron deficiency anemia
- Osteopenia secondary to long-term corticosteroid use

PAST SURGICAL HISTORY:
- Screening colonoscopy with polypectomy (age 50)
- Cesarean section (age 28)

MEDICATIONS:
Home Medications:
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Metformin 1000 mg PO twice daily
- Insulin glargine 20 units subcutaneous at bedtime
- Methotrexate 15 mg PO weekly
- Prednisone 5 mg PO daily
- Ferrous sulfate 325 mg PO daily
- Folic acid 1 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus, hypertension, stroke at age 72
- Father: Coronary artery disease, myocardial infarction at age 68
- Sister: Rheumatoid arthritis (seropositive)
- Maternal aunt: Breast cancer (invasive ductal carcinoma, age 62)

SOCIAL HISTORY:
Smoking: Active smoker, 30 pack-years (approximately 1 pack per day). Alcohol: Occasional (1-2 glasses of wine per month). Occupation: Part-time bookkeeping clerk.

VITAL SIGNS:
Her respiratory rate is 28/min, oxygen saturation is 84% on room air, pulse is 96/min, and blood pressure is 118/72 mmHg.

PHYSICAL EXAMINATION:
On examination, she appears distressed and is using accessory muscles to breathe. Lung auscultation reveals diffuse crackles bilaterally without wheezing. Heart sounds are normal, and there is no jugular venous distension or peripheral edema. Urine output remains stable.

IMAGING:
Chest x-ray shows bilateral pulmonary infiltrates with normal cardiac silhouette.
