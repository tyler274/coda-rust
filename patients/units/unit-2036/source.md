# synthetic_hospital patient 2036

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 58 at first encounter, sex M, Non-Hispanic White, Private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-04-14 (Wed) | outpatient | Neurology |
| 1 | 2020-02-15 | 2021-05-04 (Tue) | outpatient | Internal Medicine |
| 2 | 2020-03-15 | 2021-06-01 (Tue) | outpatient | Internal Medicine |
| 3 | 2020-09-15 | 2021-12-10 (Fri) | outpatient | Hematology & Oncology |
| 4 | 2021-03-15 | 2022-06-13 (Mon) | outpatient | Gastroenterology |
| 5 | 2021-09-15 | 2022-12-06 (Tue) | outpatient | Urology |
| 6 | 2024-01-15 | 2025-04-08 (Tue) | ed | Emergency Medicine |
| 7 | 2024-07-15 | 2025-09-29 (Mon) | outpatient | Hematology & Oncology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Private insurance",
 "occupation": "Office administrator",
 "alcohol_use": "Occasional social use (1-2 drinks per week), notes tremor improves with alcohol",
 "family_history": [
  "Father: Essential tremor and hypertension",
  "Father: Myocardial infarction at age 62",
  "Mother: Type 2 diabetes mellitus",
  "Older brother: Colorectal cancer diagnosed at age 61"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 8 years ago (20 pack-year history)",
 "home_medications": [
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "30 mg PO daily",
   "name": "Isosorbide mononitrate (extended release)"
  },
  {
   "dose": "400 mg PO daily",
   "name": "Imatinib"
  },
  {
   "dose": "1500 mg PO BID (14 days on, 7 days off cycle)",
   "name": "Capecitabine"
  }
 ],
 "surgical_history": [
  "Laparoscopic right hemicolectomy with primary anastomosis (for colorectal cancer)",
  "Diagnostic cardiac catheterization with drug-eluting stent placement to LAD (for stable angina)"
 ],
 "chronic_conditions": [
  "Hypertension (poorly controlled)",
  "Atrial fibrillation",
  "Coronary artery disease with stable angina",
  "Essential tremor",
  "Stage IV colorectal adenocarcinoma (on active chemotherapy)",
  "Chronic myeloid leukemia (BCR-ABL positive)"
 ],
 "age_at_first_encounter": 58
}
```

Primary diagnoses: ["Colorectal neoplasia", "Chemotherapy-induced pancytopenia", "Flecainide adverse effect", "Essential tremor", "Chronic myeloid leukemia", "Erectile dysfunction", "Essential hypertension", "Normal sinus rhythm"]
Comorbidities: ["Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2021-04-14, outpatient, Neurology

Chief complaint: Bilateral hand tremors for the past 6 months, worsening when holding objects and improving with alcohol use

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Bilateral hand tremors for the past 6 months, worsening when holding objects and improving with alcohol use

HISTORY OF PRESENT ILLNESS:
He reports that the tremors are more pronounced when he is holding objects, such as a cup of coffee or writing with a pen, and they improve with alcohol consumption. He denies any history of head injury, seizures

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension (poorly controlled)
- Atrial fibrillation
- Coronary artery disease with stable angina
- Essential tremor
- Stage IV colorectal adenocarcinoma (on active chemotherapy)
- Chronic myeloid leukemia (BCR-ABL positive)

PAST SURGICAL HISTORY:
- Laparoscopic right hemicolectomy with primary anastomosis (for colorectal cancer)
- Diagnostic cardiac catheterization with drug-eluting stent placement to LAD (for stable angina)

MEDICATIONS:
He takes lisinopril and hydrochlorothiazide for hypertension.

Home Medications:
- Metoprolol succinate 50 mg PO daily
- Aspirin 81 mg PO daily
- Isosorbide mononitrate (extended release) 30 mg PO daily
- Imatinib 400 mg PO daily
- Capecitabine 1500 mg PO BID (14 days on, 7 days off cycle)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
He denies family history of movement disorders

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago (20 pack-year history). Alcohol: Occasional social use (1-2 drinks per week), notes tremor improves with alcohol. Occupation: Office administrator.

VITAL SIGNS:
His blood pressure is 142/88 mmHg

PHYSICAL EXAMINATION:
On physical examination, you observe a postural, bilateral, upper extremity tremor without rigidity or bradykinesia. Neurological examination is otherwise unremarkable.

LABORATORY DATA:
Routine blood tests including thyroid function tests are within normal limits.

## Encounter 1: source 2020-02-15, practice 2021-05-04, outpatient, Internal Medicine

Chief complaint: Evaluated for enrollment in a clinical trial for antihypertensive therapy

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluated for enrollment in a clinical trial for antihypertensive therapy

PAST MEDICAL HISTORY:
Active Problem List:
- Essential tremor (diagnosed 2020-01-15)
- Hypertension (poorly controlled)
- Atrial fibrillation
- Coronary artery disease with stable angina
- Stage IV colorectal adenocarcinoma (on active chemotherapy)
- Chronic myeloid leukemia (BCR-ABL positive)

PAST SURGICAL HISTORY:
- Laparoscopic right hemicolectomy with primary anastomosis (for colorectal cancer)
- Diagnostic cardiac catheterization with drug-eluting stent placement to LAD (for stable angina)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Aspirin 81 mg PO daily
- Isosorbide mononitrate (extended release) 30 mg PO daily
- Imatinib 400 mg PO daily
- Capecitabine 1500 mg PO BID (14 days on, 7 days off cycle)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Essential tremor and hypertension
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Older brother: Colorectal cancer diagnosed at age 61

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago (20 pack-year history). Alcohol: Occasional social use (1-2 drinks per week), notes tremor improves with alcohol. Occupation: Office administrator.

OTHER STUDIES:
Researchers conduct two separate randomized controlled trials to assess the effect of a new antihypertensive drug. In Study A, 100 participants are enrolled, and the standard deviation of systolic blood pressure reduction is 20 mmHg. In Study B, 400 participants are enrolled, and the standard deviation of systolic blood pressure reduction is 20 mmHg. Both studies find a mean reduction in systolic blood pressure of 10 mmHg. The researchers use a 95% confidence interval for the mean reduction in systolic blood pressure.

## Encounter 2: source 2020-03-15, practice 2021-06-01, outpatient, Internal Medicine

Chief complaint: Participates in baseline visit for randomized controlled trial investigating new antihypertensive versus placebo

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Participates in baseline visit for randomized controlled trial investigating new antihypertensive versus placebo

PAST MEDICAL HISTORY:
Active Problem List:
- Essential tremor (diagnosed 2020-01-15)
- Essential hypertension (diagnosed 2020-02-15)
- Hypertension (poorly controlled)
- Atrial fibrillation
- Coronary artery disease with stable angina
- Stage IV colorectal adenocarcinoma (on active chemotherapy)
- Chronic myeloid leukemia (BCR-ABL positive)

PAST SURGICAL HISTORY:
- Laparoscopic right hemicolectomy with primary anastomosis (for colorectal cancer)
- Diagnostic cardiac catheterization with drug-eluting stent placement to LAD (for stable angina)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Aspirin 81 mg PO daily
- Isosorbide mononitrate (extended release) 30 mg PO daily
- Imatinib 400 mg PO daily
- Capecitabine 1500 mg PO BID (14 days on, 7 days off cycle)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Essential tremor and hypertension
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Older brother: Colorectal cancer diagnosed at age 61

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago (20 pack-year history). Alcohol: Occasional social use (1-2 drinks per week), notes tremor improves with alcohol. Occupation: Office administrator.

OTHER STUDIES:
The primary outcome is reduction in systolic blood pressure at 6 months. The original study design targeted a power of 80% with a two-sided alpha of 0.05, and the resulting 95% confidence interval (CI) for the mean difference between groups was (-7.8 mmHg, -1.2 mmHg). Investigators decide to increase the power to 90% for a follow-up study, keeping the alpha unchanged and all other parameters constant, including sample variance. They want to know how this change in power will affect the width of the 95% CI for the treatment effect. Assume the effect size and standard deviation remain constant, and that CI width is determined by the standard error, which is affected by the sample size. Knowing the original CI width was 6.6 mmHg,

## Encounter 3: source 2020-09-15, practice 2021-12-10, outpatient, Hematology & Oncology

Chief complaint: Three-month history of increasing fatigue, pale skin, unexplained weight loss, night sweats, and occasional low-grade fevers

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Aisha Patel, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of increasing fatigue, pale skin, unexplained weight loss, night sweats, and occasional low-grade fevers

HISTORY OF PRESENT ILLNESS:
The patient returns for Hematology and Oncology follow-up after prior Neurology and Internal Medicine encounters. He reports night sweats and occasional low-grade fevers.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential tremor (diagnosed 2020-01-15)
- Essential hypertension (diagnosed 2020-02-15)
- Normal sinus rhythm (diagnosed 2020-03-15)
- Hypertension (poorly controlled)
- Atrial fibrillation
- Coronary artery disease with stable angina
- Stage IV colorectal adenocarcinoma (on active chemotherapy)
- Chronic myeloid leukemia (BCR-ABL positive)

He has a history of hypertension,.

PAST SURGICAL HISTORY:
- Laparoscopic right hemicolectomy with primary anastomosis (for colorectal cancer)
- Diagnostic cardiac catheterization with drug-eluting stent placement to LAD (for stable angina)

MEDICATIONS:
He takes for which he takes an ACE inhibitor.

Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Aspirin 81 mg PO daily
- Isosorbide mononitrate (extended release) 30 mg PO daily
- Imatinib 400 mg PO daily
- Capecitabine 1500 mg PO BID (14 days on, 7 days off cycle)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Essential tremor and hypertension
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Older brother: Colorectal cancer diagnosed at age 61

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago (20 pack-year history). Alcohol: Occasional social use (1-2 drinks per week), notes tremor improves with alcohol. Occupation: Office administrator.

PHYSICAL EXAMINATION:
Physical examination reveals splenomegaly, pale conjunctiva, and mild tachycardia.

LABORATORY DATA:
Laboratory studies show a hemoglobin of 8.5 g/dL, leukocytosis with a left shift, metamyelocytes, and promyelocytes, and a platelet count of 600,000/µL.

PATHOLOGY:
A bone marrow biopsy reveals a hypercellular marrow with increased granulopoiesis, and a fusion protein is detected.

## Encounter 4: source 2021-03-15, practice 2022-06-13, outpatient, Gastroenterology

Chief complaint: Minimal rectal bleeding noted on toilet paper after defecation for three months without pain or weight loss

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. David Kim, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Minimal rectal bleeding noted on toilet paper after defecation for three months without pain or weight loss

HISTORY OF PRESENT ILLNESS:
The patient is a 59-year-old with Stage IV colorectal adenocarcinoma on chemotherapy and chronic myeloid leukemia who presents to Gastroenterology clinic. He reports a 3-month history of minimal rectal bleeding, primarily noted on toilet paper after defecation, without significant pain, changes in bowel habits, or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential tremor (diagnosed 2020-01-15)
- Essential hypertension (diagnosed 2020-02-15)
- Normal sinus rhythm (diagnosed 2020-03-15)
- Chronic myeloid leukemia (diagnosed 2020-09-15)
- Hypertension (poorly controlled)
- Atrial fibrillation
- Coronary artery disease with stable angina
- Stage IV colorectal adenocarcinoma (on active chemotherapy)
- Chronic myeloid leukemia (BCR-ABL positive)

His past medical history is significant for hypertension, controlled with lisinopril,.

PAST SURGICAL HISTORY:
- Laparoscopic right hemicolectomy with primary anastomosis (for colorectal cancer)
- Diagnostic cardiac catheterization with drug-eluting stent placement to LAD (for stable angina)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Aspirin 81 mg PO daily
- Isosorbide mononitrate (extended release) 30 mg PO daily
- Imatinib 400 mg PO daily
- Capecitabine 1500 mg PO BID (14 days on, 7 days off cycle)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
No family history of colorectal cancer.

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago (20 pack-year history). Alcohol: Occasional social use (1-2 drinks per week), notes tremor improves with alcohol. Occupation: Office administrator.

PHYSICAL EXAMINATION:
On physical examination, external hemorrhoids are noted. His abdomen is soft, non-tender, and without masses. Digital rectal examination reveals no blood on the examining finger, and the prostate is non-tender and of normal size.

ASSESSMENT AND PLAN:
Given the patient's age and presentation,

## Encounter 5: source 2021-09-15, practice 2022-12-06, outpatient, Urology

Chief complaint: Erectile dysfunction for the past 6 months causing significant relationship stress

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. James Thompson, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Erectile dysfunction for the past 6 months causing significant relationship stress

HISTORY OF PRESENT ILLNESS:
The patient with essential tremor, chronic myeloid leukemia, and Stage IV colorectal adenocarcinoma presents to the clinic complaining of erectile dysfunction for the past 6 months. He reports that this issue has been causing significant stress in his relationship.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential tremor (diagnosed 2020-01-15)
- Essential hypertension (diagnosed 2020-02-15)
- Normal sinus rhythm (diagnosed 2020-03-15)
- Chronic myeloid leukemia (diagnosed 2020-09-15)
- Colorectal neoplasia (diagnosed 2021-03-15)
- Hypertension (poorly controlled)
- Atrial fibrillation
- Coronary artery disease with stable angina
- Stage IV colorectal adenocarcinoma (on active chemotherapy)
- Chronic myeloid leukemia (BCR-ABL positive)

PAST SURGICAL HISTORY:
- Laparoscopic right hemicolectomy with primary anastomosis (for colorectal cancer)
- Diagnostic cardiac catheterization with drug-eluting stent placement to LAD (for stable angina)

MEDICATIONS:
His current medications include metoprolol, aspirin, and isosorbide mononitrate.

Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Isosorbide mononitrate (extended release) 30 mg PO daily
- Imatinib 400 mg PO daily
- Capecitabine 1500 mg PO BID (14 days on, 7 days off cycle)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Essential tremor and hypertension
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Older brother: Colorectal cancer diagnosed at age 61

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago (20 pack-year history). Alcohol: Occasional social use (1-2 drinks per week), notes tremor improves with alcohol. Occupation: Office administrator.

VITAL SIGNS:
On physical examination, his blood pressure is 135/85 mmHg, and his pulse is 78 beats per minute.

PHYSICAL EXAMINATION:
The remainder of the physical examination is unremarkable.

LABORATORY DATA:
Laboratory tests reveal normal renal and liver function.

## Encounter 6: source 2024-01-15, practice 2025-04-08, ed, Emergency Medicine

Chief complaint: Sudden onset palpitations and lightheadedness while jogging

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Emily Nakamura, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset palpitations and lightheadedness while jogging

HISTORY OF PRESENT ILLNESS:
This 62-year-old male with atrial fibrillation, poorly controlled hypertension, coronary artery disease, essential tremor, CML, and colorectal cancer on chemotherapy presents to the emergency department with palpitations and lightheadedness. He reports that his symptoms started suddenly while he was jogging.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential tremor (diagnosed 2020-01-15)
- Essential hypertension (diagnosed 2020-02-15)
- Normal sinus rhythm (diagnosed 2020-03-15)
- Chronic myeloid leukemia (diagnosed 2020-09-15)
- Colorectal neoplasia (diagnosed 2021-03-15)
- Erectile dysfunction (diagnosed 2021-09-15)
- Hypertension (poorly controlled)
- Atrial fibrillation
- Coronary artery disease with stable angina
- Stage IV colorectal adenocarcinoma (on active chemotherapy)
- Chronic myeloid leukemia (BCR-ABL positive)

His past medical history is significant for hypertension,.

PAST SURGICAL HISTORY:
- Laparoscopic right hemicolectomy with primary anastomosis (for colorectal cancer)
- Diagnostic cardiac catheterization with drug-eluting stent placement to LAD (for stable angina)

MEDICATIONS:
He takes for which he takes hydrochlorothiazide.

Home Medications:
- Lisinopril 20 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Aspirin 81 mg PO daily
- Isosorbide mononitrate (extended release) 30 mg PO daily
- Imatinib 400 mg PO daily
- Capecitabine 1500 mg PO BID (14 days on, 7 days off cycle)

ALLERGIES:
He has no known drug allergies.

FAMILY HISTORY:
- Father: Essential tremor and hypertension
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Older brother: Colorectal cancer diagnosed at age 61

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago (20 pack-year history). Alcohol: Occasional social use (1-2 drinks per week), notes tremor improves with alcohol. Occupation: Office administrator.

VITAL SIGNS:
On examination, his blood pressure is 130/85 mmHg, heart rate is 160 beats per minute, and respiratory rate is 18 beats per minute.

LABORATORY DATA:
Laboratory values are as follows:

- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 1.0 mg/dL
- Troponin: <0.01 ng/mL

OTHER STUDIES:
His ECG shows atrial fibrillation with a rapid ventricular response and a widened QRS complex. His previous ECGs showed atrial fibrillation with a normal QRS duration.

## Encounter 7: source 2024-07-15, practice 2025-09-29, outpatient, Hematology & Oncology

Chief complaint: Profound fatigue with pancytopenia discovered on routine labs during active chemotherapy for advanced colon cancer

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Aisha Patel, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Profound fatigue with pancytopenia discovered on routine labs during active chemotherapy for advanced colon cancer

HISTORY OF PRESENT ILLNESS:
The patient with Stage IV colorectal cancer and CML presents during chemotherapy for advanced colon cancer. Over the past two weeks, he has developed profound fatigue and pancytopenia.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential tremor (diagnosed 2020-01-15)
- Essential hypertension (diagnosed 2020-02-15)
- Normal sinus rhythm (diagnosed 2020-03-15)
- Chronic myeloid leukemia (diagnosed 2020-09-15)
- Colorectal neoplasia (diagnosed 2021-03-15)
- Erectile dysfunction (diagnosed 2021-09-15)
- Flecainide adverse effect (diagnosed 2024-01-15)
- Hypertension (poorly controlled)
- Atrial fibrillation
- Coronary artery disease with stable angina
- Stage IV colorectal adenocarcinoma (on active chemotherapy)
- Chronic myeloid leukemia (BCR-ABL positive)

PAST SURGICAL HISTORY:
- Laparoscopic right hemicolectomy with primary anastomosis (for colorectal cancer)
- Diagnostic cardiac catheterization with drug-eluting stent placement to LAD (for stable angina)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Aspirin 81 mg PO daily
- Isosorbide mononitrate (extended release) 30 mg PO daily
- Imatinib 400 mg PO daily
- Capecitabine 1500 mg PO BID (14 days on, 7 days off cycle)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Essential tremor and hypertension
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Older brother: Colorectal cancer diagnosed at age 61

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago (20 pack-year history). Alcohol: Occasional social use (1-2 drinks per week), notes tremor improves with alcohol. Occupation: Office administrator.

LABORATORY DATA:
Laboratory studies confirm elevated cytosolic cytochrome c levels.

PATHOLOGY:
Bone marrow biopsy shows markedly increased apoptotic bodies within hematopoietic precursors. Further analysis demonstrates increased expression of p53 and pro-apoptotic BAX proteins in these cells.
