# synthetic_hospital patient 2697

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 55 at first encounter, sex F, Non-Hispanic White, Employer-based private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-09-06 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2022-02-25 (Fri) | outpatient | Gastroenterology |
| 2 | 2021-07-15 | 2023-02-23 (Thu) | outpatient | Orthopedics & Sports Medicine |
| 3 | 2022-01-15 | 2023-08-24 (Thu) | outpatient | Obstetrics & Gynecology |
| 4 | 2022-07-15 | 2024-02-27 (Tue) | outpatient | Gastroenterology |
| 5 | 2023-07-15 | 2025-03-03 (Mon) | outpatient | Obstetrics & Gynecology |
| 6 | 2024-01-15 | 2025-08-26 (Tue) | outpatient | Obstetrics & Gynecology |
| 7 | 2024-07-15 | 2026-03-02 (Mon) | outpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Employer-based private insurance",
 "occupation": "Librarian",
 "alcohol_use": "None",
 "family_history": [
  "No family history of cancer",
  "No family history of cardiovascular disease",
  "No family history of thromboembolic disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  }
 ],
 "surgical_history": [
  "No prior surgeries",
  "Nulliparous"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Obesity",
  "Osteoarthritis, bilateral knees",
  "Hiatal hernia",
  "Gastroesophageal reflux disease"
 ],
 "age_at_first_encounter": 55
}
```

Primary diagnoses: ["Malignant neoplasm of ovary", "Osteoarthritis of knee", "Hiatal hernia", "Endometrial carcinoma"]
Comorbidities: ["Obesity", "Essential hypertension", "Postmenopausal status"]

## Encounter 0: source 2020-01-15, practice 2021-09-06, outpatient, Internal Medicine

Chief complaint: Bilateral knee pain progressively worsening over the past year

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Bilateral knee pain progressively worsening over the past year

HISTORY OF PRESENT ILLNESS:
That has been progressively worsening over the past year. She reports that the pain is most severe in the morning and after prolonged periods of sitting, but it improves with light activity. She denies any recent trauma or injury.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Obesity
- Osteoarthritis, bilateral knees
- Hiatal hernia
- Gastroesophageal reflux disease

PAST SURGICAL HISTORY:
- No prior surgeries
- Nulliparous

MEDICATIONS:
Her current medications include hydrochlorothiazide and amlodipine.

ALLERGIES:
NKDA

FAMILY HISTORY:
- No family history of cancer
- No family history of cardiovascular disease
- No family history of thromboembolic disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Librarian.

VITAL SIGNS:
Her BMI is 32 kg/m^2.

PHYSICAL EXAMINATION:
On physical examination, there is no erythema or warmth, but there is bony enlargement of both knee joints with tenderness on palpation. There is no effusion, and the range of motion is preserved but painful at the extremes.

IMAGING:
Radiographs of the knees show joint space narrowing, subchondral sclerosis, and osteophyte formation.

## Encounter 1: source 2020-07-15, practice 2022-02-25, outpatient, Gastroenterology

Chief complaint: Six-month history of intermittent epigastric burning and regurgitation of sour-tasting fluid after large meals

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of intermittent epigastric burning and regurgitation of sour-tasting fluid after large meals

HISTORY OF PRESENT ILLNESS:
The patient with hiatal hernia returns six months after her osteoarthritis evaluation with intermittent epigastric burning and regurgitation of sour-tasting fluid after large meals. She also notes increased belching and a sensation of food "coming back up" when lying down shortly after eating. She denies dysphagia to solids or liquids, hematemesis, or unintentional weight loss. She works as a librarian and has noticed that bending over to shelve books often triggers her symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Hypertension
- Obesity
- Osteoarthritis, bilateral knees
- Hiatal hernia
- Gastroesophageal reflux disease

Her medical history is notable for well-controlled hypertension.

PAST SURGICAL HISTORY:
- No prior surgeries
- Nulliparous

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Amlodipine 5 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- No family history of cancer
- No family history of cardiovascular disease
- No family history of thromboembolic disease

SOCIAL HISTORY:
She does not smoke or drink alcohol.

VITAL SIGNS:
Her BMI is 31 kg/m². On examination, her temperature is 37°C (98.6°F), blood pressure is 130/78 mm Hg, and pulse is 78/min.

PHYSICAL EXAMINATION:
Abdominal examination is unremarkable except for mild epigastric tenderness without rebound or guarding. Cardiac and pulmonary examinations are normal.

IMAGING:
A barium swallow study shows a portion of the stomach and the gastroesophageal (GE) junction above the diaphragm, with contrast outlining a small sac-like protrusion into the thoracic cavity.

## Encounter 2: source 2021-07-15, practice 2023-02-23, outpatient, Orthopedics & Sports Medicine

Chief complaint: Bilateral knee pain progressively worsening over the past two years

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Amara Okafor, MD
Department: Orthopedics & Sports Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Bilateral knee pain progressively worsening over the past two years

HISTORY OF PRESENT ILLNESS:
The patient returns to the orthopedic clinic for ongoing management of bilateral knee pain previously assessed in January 2020. She presents complaining of bilateral knee pain that has been progressively worsening over the past two years. She reports that the pain is most severe in the mornings and after prolonged periods of sitting, but it improves with light activity. She denies any recent trauma or injury.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Hiatal hernia (diagnosed 2020-07-15)
- Hypertension
- Obesity
- Osteoarthritis, bilateral knees
- Gastroesophageal reflux disease

Her medical history is significant for hypertension, which is well-controlled with.

PAST SURGICAL HISTORY:
- No prior surgeries
- Nulliparous

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Amlodipine 5 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- No family history of cancer
- No family history of cardiovascular disease
- No family history of thromboembolic disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Librarian.

PHYSICAL EXAMINATION:
On physical examination, there is no erythema or warmth of the knees, but there is bony enlargement of the joint margins and crepitus with range of motion.

IMAGING:
Radiographs of the knees show joint space narrowing, subchondral sclerosis, and osteophyte formation.

## Encounter 3: source 2022-01-15, practice 2023-08-24, outpatient, Obstetrics & Gynecology

Chief complaint: Irregular uterine bleeding and pelvic discomfort

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Jennifer Kim, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Irregular uterine bleeding and pelvic discomfort

HISTORY OF PRESENT ILLNESS:
The patient with hypertension and OA presents with bleeding. She reports that her menstrual cycles have become unpredictable, with episodes of heavy bleeding followed by weeks of amenorrhea.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Hiatal hernia (diagnosed 2020-07-15)
- Hypertension
- Obesity
- Osteoarthritis, bilateral knees
- Gastroesophageal reflux disease

Her past medical history is significant for hypertension. She has never been pregnant.

PAST SURGICAL HISTORY:
- No prior surgeries
- Nulliparous

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Amlodipine 5 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
No family history of gynecological cancers

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Librarian.

REVIEW OF SYSTEMS:
She denies any significant weight loss, fever, or night sweats.

VITAL SIGNS:
Blood pressure is 135/85 mmHg, and her BMI is 30 kg/m^2

PHYSICAL EXAMINATION:
A bimanual pelvic examination reveals a palpable, mobile, left adnexal mass.

LABORATORY DATA:
Serum hormone levels are as follows:
- Estradiol: 450 pg/mL (normal range: 12-233 pg/mL)
- FSH: 7.2 mIU/mL (normal range: 3.5-12.5 mIU/mL)
- LH: 3.1 mIU/mL (normal range: 2.4-12.6 mIU/mL)

IMAGING:
Transvaginal ultrasound shows a 5 cm left ovarian mass with a solid and cystic component.

## Encounter 4: source 2022-07-15, practice 2024-02-27, outpatient, Gastroenterology

Chief complaint: Progressive lower abdominal pain, bloating, early satiety, and unintentional weight loss over three months

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David Park, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive lower abdominal pain, bloating, early satiety, and unintentional weight loss over three months

HISTORY OF PRESENT ILLNESS:
Patient returns to gastroenterology six months after her ovarian malignancy diagnosis with progressive symptoms. She reports 10-pound weight loss during this period without trying. Her bowel habits have changed, with new-onset constipation and occasional hematochezia.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Hiatal hernia (diagnosed 2020-07-15)
- Malignant neoplasm of ovary (diagnosed 2022-01-15)
- Hypertension
- Obesity
- Osteoarthritis, bilateral knees
- Gastroesophageal reflux disease

Her past medical history is significant for hypertension, controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- No prior surgeries
- Nulliparous

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Amlodipine 5 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
She has no family history of gastrointestinal or gynecologic cancers.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Librarian.

REVIEW OF SYSTEMS:
She denies any fever, nausea, or vomiting.

PHYSICAL EXAMINATION:
On physical examination, her abdomen is soft, non-distended, with mild diffuse tenderness but no rebound or guarding. A pelvic examination reveals a fixed, irregular pelvic mass.

LABORATORY DATA:
Her CA-125 level is 250 U/mL (normal: 0-35 U/mL).

IMAGING:
A transvaginal ultrasound shows a complex adnexal mass measuring 6 cm with solid and cystic components.

## Encounter 5: source 2023-07-15, practice 2025-03-03, outpatient, Obstetrics & Gynecology

Chief complaint: Intermittent vaginal spotting noted during routine health examination

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Lisa Patel, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent vaginal spotting noted during routine health examination

HISTORY OF PRESENT ILLNESS:
The patient with hypertension, obesity, osteoarthritis, and hiatal hernia returns following prior gynecologic evaluation. She reports experiencing intermittent vaginal spotting over the past 2 months. She has no significant pain associated with the spotting. Her last menstrual period was 3 years ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Hiatal hernia (diagnosed 2020-07-15)
- Malignant neoplasm of ovary (diagnosed 2022-01-15)
- Hypertension
- Obesity
- Osteoarthritis, bilateral knees
- Gastroesophageal reflux disease

Her medical history is significant for hypertension, controlled with hydrochlorothiazide. She has no personal or family history of cancer.

PAST SURGICAL HISTORY:
- No prior surgeries
- Nulliparous

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Amlodipine 5 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
She has no personal or family history of cancer.

SOCIAL HISTORY:
She has never used hormone replacement therapy.

VITAL SIGNS:
On physical examination, her blood pressure is 132/86 mmHg, and her BMI is 30 kg/m^2.

PHYSICAL EXAMINATION:
A pelvic examination reveals a normal-sized uterus without masses or tenderness, and the adnexa are nonpalpable.

LABORATORY DATA:
A recent Pap smear showed the presence of endometrial cells.

IMAGING:
Her most recent mammogram was normal.

## Encounter 6: source 2024-01-15, practice 2025-08-26, outpatient, Obstetrics & Gynecology

Chief complaint: Irregular vaginal bleeding occurring eight years after menopause with increasing abdominal girth

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Robert Chang, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Irregular vaginal bleeding occurring eight years after menopause with increasing abdominal girth

HISTORY OF PRESENT ILLNESS:
The patient is a 59-year-old female with a history of malignant neoplasm of the ovary and endometrial carcinoma who returns to the gynecology clinic for continued oncologic surveillance. She reports that her menopause occurred 8 years ago and she has not had any bleeding since then until recently. The patient also complains of increasing abdominal girth and pelvic pressure over the past few months. She denies any significant weight loss, fever, or night sweats.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Hiatal hernia (diagnosed 2020-07-15)
- Malignant neoplasm of ovary (diagnosed 2022-01-15)
- Endometrial carcinoma (diagnosed 2023-07-15)
- Hypertension
- Obesity
- Osteoarthritis, bilateral knees
- Gastroesophageal reflux disease

Her past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- No prior surgeries
- Nulliparous

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Amlodipine 5 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- No family history of cancer
- No family history of cardiovascular disease
- No family history of thromboembolic disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Librarian.

PHYSICAL EXAMINATION:
On physical examination, her abdomen is distended with a palpable, nontender mass in the lower abdomen. Pelvic examination reveals a 10 cm, mobile, smooth, and firm mass arising from the pelvis.

LABORATORY DATA:
Serum laboratory tests reveal the following:

- Hemoglobin: 12.5 g/dL
- Platelets: 250,000/uL
- CA-125: 35 U/mL (normal < 35 U/mL)
- Estradiol: 450 pg/mL (postmenopausal normal range: < 20 pg/mL)
- FSH: 7.1 mIU/mL (postmenopausal normal range: 25.8-134.8 mIU/mL)
- LH: 6.9 mIU/mL (postmenopausal normal range: 7.7-58.5 mIU/mL)

IMAGING:
Transvaginal ultrasound shows a solid ovarian mass with internal blood flow.

## Encounter 7: source 2024-07-15, practice 2026-03-02, outpatient, Obstetrics & Gynecology

Chief complaint: Irregular uterine bleeding and increasing abdominal girth with pelvic fullness

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Jennifer Kim, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Irregular uterine bleeding and increasing abdominal girth with pelvic fullness

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of her known ovarian malignancy and endometrial carcinoma. She reports a 6-month history of irregular uterine bleeding and increasing abdominal girth with a sensation of fullness in her pelvis and occasional pelvic pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Hiatal hernia (diagnosed 2020-07-15)
- Malignant neoplasm of ovary (diagnosed 2022-01-15)
- Endometrial carcinoma (diagnosed 2023-07-15)
- Hypertension
- Obesity
- Osteoarthritis, bilateral knees
- Gastroesophageal reflux disease

She has a past medical history of hypertension, controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- No prior surgeries
- Nulliparous

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Amlodipine 5 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- No family history of cancer
- No family history of cardiovascular disease
- No family history of thromboembolic disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Librarian.

VITAL SIGNS:
Her blood pressure is 135/85 mmHg

PHYSICAL EXAMINATION:
A palpable, nontender mass is noted in the lower abdomen. Pelvic examination reveals a firm, irregularly shaped mass arising from the pelvis. Breast examination shows mild bilateral tenderness without discrete masses or nipple discharge.

LABORATORY DATA:
Hemoglobin: 13.2 g/dL
- Platelets: 250,000/uL
- Estradiol: Elevated
- Testosterone: Normal
- CA-125: Normal

IMAGING:
Transvaginal ultrasound demonstrates a 10 cm solid ovarian mass with internal blood flow.

PATHOLOGY:
Endometrial biopsy is performed due to the postmenopausal bleeding, which shows a thickened endometrium with no evidence of malignancy.
