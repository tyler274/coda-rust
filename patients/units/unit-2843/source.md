# synthetic_hospital patient 2843

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex F, Non-Hispanic White, Medicare with Aetna supplemental plan.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-08-30 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-03-15 | 2021-11-01 (Mon) | outpatient | Obstetrics & Gynecology |
| 2 | 2020-05-15 | 2021-12-20 (Mon) | outpatient | Obstetrics & Gynecology |
| 3 | 2020-07-15 | 2022-02-24 (Thu) | ed | Emergency Medicine |
| 4 | 2022-01-15 | 2023-08-23 (Wed) | outpatient | Gastroenterology |
| 5 | 2023-01-15 | 2024-08-28 (Wed) | ed | Emergency Medicine |
| 6 | 2023-07-15 | 2025-02-20 (Thu) | ed | Emergency Medicine |
| 7 | 2024-07-15 | 2026-02-21 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare with Aetna supplemental plan",
 "occupation": "Retired high school mathematics teacher",
 "alcohol_use": "1-2 glasses of wine daily with dinner",
 "family_history": [
  "Brother: Sudden cardiac death (age 62)",
  "Mother: Osteoporosis and hip fracture",
  "Father: Coronary artery disease, hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 10 years ago (approximate 30 pack-year history)",
 "home_medications": [
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "70 mg PO weekly",
   "name": "Alendronate"
  },
  {
   "dose": "600 mg-400 units PO twice daily",
   "name": "Calcium carbonate/Vitamin D3"
  },
  {
   "dose": "0.625 mg PO daily x 10 years",
   "name": "Conjugated estrogen (PREVIOUS - discontinued 6 weeks ago)"
  }
 ],
 "surgical_history": [
  "Open reduction and internal fixation of left distal radius fracture (age 65)",
  "Dilation and curettage with hysteroscopy (age 67)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia",
  "Osteoporosis",
  "Menopause (natural, age 52)"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Endometrial carcinoma", "Takotsubo cardiomyopathy", "Osteoporosis screening in malabsorptive disorder", "Ischemic colitis", "Acute coronary syndrome", "Posterior communicating artery aneurysm"]
Comorbidities: ["Postmenopausal status", "Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2021-08-30, outpatient, Internal Medicine

Chief complaint: Routine health maintenance visit

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health maintenance visit

HISTORY OF PRESENT ILLNESS:
She reports no acute complaints but mentions that she has been trying to stay active by walking her dog daily.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- Osteoporosis
- Menopause (natural, age 52)

She has a past medical history of hypertension, which is well-controlled on hydrochlorothiazide, and a remote history of a wrist fracture sustained from a fall 2 years ago. She has never had a bone density scan.

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of left distal radius fracture (age 65)
- Dilation and curettage with hysteroscopy (age 67)

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 20 mg PO daily
- Alendronate 70 mg PO weekly
- Calcium carbonate/Vitamin D3 600 mg-400 units PO twice daily
- Conjugated estrogen (PREVIOUS - discontinued 6 weeks ago) 0.625 mg PO daily x 10 years

ALLERGIES:
NKDA

FAMILY HISTORY:
Her mother was diagnosed with osteoporosis at the age of 70.

SOCIAL HISTORY:
She quit smoking 10 years ago but admits to drinking a glass of wine with dinner most nights.

PHYSICAL EXAMINATION:
On physical examination, her BMI is calculated to be 19.

LABORATORY DATA:
Laboratory results are as follows:

- Serum calcium: 9.2 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum phosphorus: 3.5 mg/dL (normal: 2.5-4.5 mg/dL)
- Alkaline phosphatase: 70 U/L (normal: 40-130 U/L)
- TSH: 2.1 mIU/L (normal: 0.4-4.0 mIU/L)

## Encounter 1: source 2020-03-15, practice 2021-11-01, outpatient, Obstetrics & Gynecology

Chief complaint: Vaginal bleeding that started 2 months ago

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Maria Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Vaginal bleeding that started 2 months ago

HISTORY OF PRESENT ILLNESS:
This 67-year-old postmenopausal female, seen in January for osteoporosis screening, presents with vaginal bleeding. She reports that the bleeding is light but persistent. Her last menstrual period was at the age of 52. She denies any recent trauma or use of hormone replacement therapy.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis screening in malabsorptive disorder (diagnosed 2020-01-15)
- Hypertension
- Hyperlipidemia
- Osteoporosis
- Menopause (natural, age 52)

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of left distal radius fracture (age 65)
- Dilation and curettage with hysteroscopy (age 67)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 20 mg PO daily
- Alendronate 70 mg PO weekly
- Calcium carbonate/Vitamin D3 600 mg-400 units PO twice daily
- Conjugated estrogen (PREVIOUS - discontinued 6 weeks ago) 0.625 mg PO daily x 10 years

ALLERGIES:
NKDA

FAMILY HISTORY:
- Brother: Sudden cardiac death (age 62)
- Mother: Osteoporosis and hip fracture
- Father: Coronary artery disease, hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (approximate 30 pack-year history). Alcohol: 1-2 glasses of wine daily with dinner. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
Her vital signs are stable

PHYSICAL EXAMINATION:
On physical examination, her abdominal exam is unremarkable. A pelvic exam reveals a small amount of blood in the vaginal vault but no masses or lesions.

IMAGING:
A transvaginal ultrasound (TVUS) is performed, which shows an endometrial thickness of 5 mm.

## Encounter 2: source 2020-05-15, practice 2021-12-20, outpatient, Obstetrics & Gynecology

Chief complaint: Persistent vaginal bleeding for the past 6 weeks

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Maria Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent vaginal bleeding for the past 6 weeks

HISTORY OF PRESENT ILLNESS:
A 67-year-old female with hypertension, hyperlipidemia, and osteoporosis who returns for follow-up of vaginal bleeding. She describes the bleeding as light but continuous, without associated pelvic pain or discharge. Her menstrual cycles stopped more than 20 years ago, and she has not experienced any similar episodes since.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis screening in malabsorptive disorder (diagnosed 2020-01-15)
- Endometrial carcinoma (diagnosed 2020-03-15)
- Hypertension
- Hyperlipidemia
- Osteoporosis
- Menopause (natural, age 52)

She has been in generally good health, with mild hypertension well controlled on medication.

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of left distal radius fracture (age 65)
- Dilation and curettage with hysteroscopy (age 67)

MEDICATIONS:
She reports taking a daily oral estrogen preparation prescribed by her previous physician for "bone and skin health" for the last 10 years.

Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 20 mg PO daily
- Alendronate 70 mg PO weekly
- Calcium carbonate/Vitamin D3 600 mg-400 units PO twice daily
- Conjugated estrogen (PREVIOUS - discontinued 6 weeks ago) 0.625 mg PO daily x 10 years

ALLERGIES:
NKDA

FAMILY HISTORY:
- Brother: Sudden cardiac death (age 62)
- Mother: Osteoporosis and hip fracture
- Father: Coronary artery disease, hypertension

SOCIAL HISTORY:
She also drinks several glasses of wine most evenings and has done so for decades. She is a long-term smoker, has given birth to four children, and began menstruating at the age of 18.

REVIEW OF SYSTEMS:
She denies weight loss, night sweats, fever, or abdominal distension.

PHYSICAL EXAMINATION:
Pelvic examination reveals a mildly enlarged, nontender uterus without adnexal masses.

IMAGING:
Transvaginal ultrasound shows a thickened endometrial stripe.
There is no evidence of myometrial invasion on imaging.

PATHOLOGY:
An endometrial biopsy is performed, and histopathology reveals malignant glandular epithelial cells forming irregular back-to-back glands.

## Encounter 3: source 2020-07-15, practice 2022-02-24, ed, Emergency Medicine

Chief complaint: 4 days of heavy vaginal bleeding

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
4 days of heavy vaginal bleeding

HISTORY OF PRESENT ILLNESS:
Patient returns with recurrent bleeding following evaluations in March and May 2020. She went through menopause 10 years ago and has not used hormone replacement therapy.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis screening in malabsorptive disorder (diagnosed 2020-01-15)
- Endometrial carcinoma (diagnosed 2020-03-15)
- Hypertension
- Hyperlipidemia
- Osteoporosis
- Menopause (natural, age 52)

Her last Pap smear 2 years ago was normal.

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of left distal radius fracture (age 65)
- Dilation and curettage with hysteroscopy (age 67)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 20 mg PO daily
- Alendronate 70 mg PO weekly
- Calcium carbonate/Vitamin D3 600 mg-400 units PO twice daily
- Conjugated estrogen (PREVIOUS - discontinued 6 weeks ago) 0.625 mg PO daily x 10 years

ALLERGIES:
NKDA

FAMILY HISTORY:
- Brother: Sudden cardiac death (age 62)
- Mother: Osteoporosis and hip fracture
- Father: Coronary artery disease, hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (approximate 30 pack-year history). Alcohol: 1-2 glasses of wine daily with dinner. Occupation: Retired high school mathematics teacher.

REVIEW OF SYSTEMS:
She denies abdominal pain or systemic symptoms.

PHYSICAL EXAMINATION:
On pelvic examination, the uterus is enlarged and irregular, approximating the size of a 14-week gestation. There is no adnexal tenderness or masses.

LABORATORY DATA:
Laboratory studies show a hemoglobin concentration of 8.6 g/dL and hematocrit of 25%.

IMAGING:
A transvaginal ultrasound shows thickened endometrial stripe measuring 14 mm.

OTHER STUDIES:
An endometrial biopsy is obtained.

## Encounter 4: source 2022-01-15, practice 2023-08-23, outpatient, Gastroenterology

Chief complaint: 6-month history of postprandial abdominal pain and 10-pound weight loss

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of postprandial abdominal pain and 10-pound weight loss

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old female with a history of endometrial carcinoma diagnosed in 2020 and recent discontinuation of long-term estrogen therapy six weeks prior who presents with postprandial abdominal pain. She reports that the pain is dull and crampy, located in the epigastric region, and occurs about 30 minutes after eating. She has started to avoid eating, which relieves the pain but has led to her weight loss. She denies nausea, vomiting, diarrhea, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis screening in malabsorptive disorder (diagnosed 2020-01-15)
- Endometrial carcinoma (diagnosed 2020-03-15)
- Hypertension
- Hyperlipidemia
- Osteoporosis
- Menopause (natural, age 52)

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of left distal radius fracture (age 65)
- Dilation and curettage with hysteroscopy (age 67)

MEDICATIONS:
Her medications include lisinopril and atorvastatin.

Home Medications:
- Alendronate 70 mg PO weekly
- Calcium carbonate/Vitamin D3 600 mg-400 units PO twice daily
- Conjugated estrogen (PREVIOUS - discontinued 6 weeks ago) 0.625 mg PO daily x 10 years

ALLERGIES:
NKDA

FAMILY HISTORY:
- Brother: Sudden cardiac death (age 62)
- Mother: Osteoporosis and hip fracture
- Father: Coronary artery disease, hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (approximate 30 pack-year history). Alcohol: 1-2 glasses of wine daily with dinner. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, her abdomen is soft and non-distended with normal bowel sounds, but there is mild tenderness to palpation in the epigastric area without rebound or guarding.

LABORATORY DATA:
Her laboratory tests are as follows:

- Hemoglobin: 13.2 g/dL
- Leukocyte count: 7,200/mm³
- Platelet count: 250,000/mm³
- Serum creatinine: 1.0 mg/dL
- Fasting lipid panel: within normal limits

IMAGING:
- Abdominal ultrasound: normal liver and gallbladder, no evidence of cholelithiasis

## Encounter 5: source 2023-01-15, practice 2024-08-28, ed, Emergency Medicine

Chief complaint: 2-day history of visual changes and drooping of right eyelid

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-day history of visual changes and drooping of right eyelid

HISTORY OF PRESENT ILLNESS:
The patient with history of endometrial carcinoma and recent estrogen discontinuation presents with visual symptoms. She reports difficulty focusing her vision when reading or descending stairs. She also mentions that bright lights seem to bother her more than usual. She denies headache, trauma, or previous episodes.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis screening in malabsorptive disorder (diagnosed 2020-01-15)
- Endometrial carcinoma (diagnosed 2020-03-15)
- Ischemic colitis (diagnosed 2022-01-15)
- Hypertension
- Hyperlipidemia
- Osteoporosis
- Menopause (natural, age 52)

Her past medical history includes long-standing hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of left distal radius fracture (age 65)
- Dilation and curettage with hysteroscopy (age 67)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 20 mg PO daily
- Alendronate 70 mg PO weekly
- Calcium carbonate/Vitamin D3 600 mg-400 units PO twice daily
- Conjugated estrogen (PREVIOUS - discontinued 6 weeks ago) 0.625 mg PO daily x 10 years

ALLERGIES:
NKDA

FAMILY HISTORY:
- Brother: Sudden cardiac death (age 62)
- Mother: Osteoporosis and hip fracture
- Father: Coronary artery disease, hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (approximate 30 pack-year history). Alcohol: 1-2 glasses of wine daily with dinner. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On examination, her right eyelid is partially closed compared to the left. Eye movement testing shows that the right eye has limited vertical and medial motion but can move laterally without issue. The right pupil appears slightly larger than the left and reacts slowly to direct light. There is no sensory loss on the face, and motor strength in all extremities is normal. Funduscopic exam is unremarkable.

## Encounter 6: source 2023-07-15, practice 2025-02-20, ed, Emergency Medicine

Chief complaint: Sudden-onset epigastric pain and nausea

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Lisa Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset epigastric pain and nausea

HISTORY OF PRESENT ILLNESS:
This 70-year-old female with a history of endometrial carcinoma, ischemic colitis, and recent posterior communicating artery aneurysm who discontinued estrogen therapy six weeks ago presents to the emergency department with a 2-hour history of sudden-onset epigastric pain and nausea. She describes the pain as a constant, dull ache that does not radiate. She also reports a single episode of non-bloody, non-bilious vomiting. She denies any recent changes in diet, bowel habits, or urinary symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis screening in malabsorptive disorder (diagnosed 2020-01-15)
- Endometrial carcinoma (diagnosed 2020-03-15)
- Ischemic colitis (diagnosed 2022-01-15)
- Posterior communicating artery aneurysm (diagnosed 2023-01-15)
- Hypertension
- Hyperlipidemia
- Osteoporosis
- Menopause (natural, age 52)

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of left distal radius fracture (age 65)
- Dilation and curettage with hysteroscopy (age 67)

MEDICATIONS:
Her medications include lisinopril and atorvastatin.

Home Medications:
- Alendronate 70 mg PO weekly
- Calcium carbonate/Vitamin D3 600 mg-400 units PO twice daily
- Conjugated estrogen (PREVIOUS - discontinued 6 weeks ago) 0.625 mg PO daily x 10 years

ALLERGIES:
NKDA

FAMILY HISTORY:
- Brother: Sudden cardiac death (age 62)
- Mother: Osteoporosis and hip fracture
- Father: Coronary artery disease, hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (approximate 30 pack-year history). Alcohol: 1-2 glasses of wine daily with dinner. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
On examination, her blood pressure is 160/90 mmHg, heart rate is 98 bpm, and respiratory rate is 20 breaths per minute.

PHYSICAL EXAMINATION:
Her abdomen is soft and non-distended with mild epigastric tenderness to palpation without rebound or guarding. Bowel sounds are normal.

LABORATORY DATA:
Laboratory Values:
- Hemoglobin: 13.2 g/dL
- White blood cell count: 7,500/mm^3
- Platelets: 250,000/mm^3
- Serum creatinine: 1.0 mg/dL
- Aspartate aminotransferase (AST): 30 U/L
- Alanine aminotransferase (ALT): 28 U/L
- Lipase: 22 U/L (normal range)

OTHER STUDIES:
An electrocardiogram (ECG) shows mild ST-segment depressions in leads II, III, and aVF. Troponin I level is pending.

## Encounter 7: source 2024-07-15, practice 2026-02-21, ed, Emergency Medicine

Chief complaint: Acute onset chest pain and shortness of breath

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Lisa Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset chest pain and shortness of breath

HISTORY OF PRESENT ILLNESS:
The patient is a 71-year-old female with history of acute coronary syndrome (2023), endometrial carcinoma (2020), and recent discontinuation of long-term estrogen therapy who presents to the emergency department with acute onset chest pain and shortness of breath that began 2 hours ago after receiving news of her brother's sudden death. She describes the pain as a pressure-like sensation over the anterior chest, radiating to her left arm.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis screening in malabsorptive disorder (diagnosed 2020-01-15)
- Endometrial carcinoma (diagnosed 2020-03-15)
- Ischemic colitis (diagnosed 2022-01-15)
- Posterior communicating artery aneurysm (diagnosed 2023-01-15)
- Acute coronary syndrome (diagnosed 2023-07-15)
- Hypertension
- Hyperlipidemia
- Osteoporosis
- Menopause (natural, age 52)

She has a history of hypertension, which is well-controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of left distal radius fracture (age 65)
- Dilation and curettage with hysteroscopy (age 67)

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 20 mg PO daily
- Alendronate 70 mg PO weekly
- Calcium carbonate/Vitamin D3 600 mg-400 units PO twice daily
- Conjugated estrogen (PREVIOUS - discontinued 6 weeks ago) 0.625 mg PO daily x 10 years

ALLERGIES:
NKDA

FAMILY HISTORY:
- Brother: Sudden cardiac death (age 62)
- Mother: Osteoporosis and hip fracture
- Father: Coronary artery disease, hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (approximate 30 pack-year history). Alcohol: 1-2 glasses of wine daily with dinner. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
On examination, her blood pressure is 160/90 mmHg, heart rate is 110 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 96% on room air.

PHYSICAL EXAMINATION:
Cardiac examination reveals a systolic murmur.

LABORATORY DATA:
Troponin I levels are elevated. Laboratory Values:
- Troponin I: 2.1 ng/mL (Normal <0.04 ng/mL)
- Creatine kinase-MB: Normal
- B-type natriuretic peptide (BNP): Elevated

IMAGING:
An echocardiogram demonstrates hypokinesis of the left ventricular mid and apical segments with hyperkinesis of the basal segments.

OTHER STUDIES:
An electrocardiogram (ECG) shows ST-segment elevation in the precordial leads.
