# synthetic_hospital patient 2025

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 58 at first encounter, sex M, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-03-29 (Wed) | outpatient | Oncology |
| 1 | 2020-04-15 | 2023-06-21 (Wed) | outpatient | Internal Medicine |
| 2 | 2020-07-15 | 2023-09-25 (Mon) | outpatient | Internal Medicine |
| 3 | 2021-01-15 | 2024-03-27 (Wed) | outpatient | Cardiology |
| 4 | 2021-07-15 | 2024-09-23 (Mon) | outpatient | Otolaryngology |
| 5 | 2022-01-15 | 2025-03-25 (Tue) | outpatient | Cardiology |
| 6 | 2022-07-15 | 2025-09-22 (Mon) | ed | Emergency Medicine |
| 7 | 2023-01-15 | 2026-04-07 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Office Manager",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Father: Myocardial infarction at age 55, hypertension",
  "Mother: Type 2 diabetes, hyperlipidemia",
  "Paternal uncle: Ischemic stroke at age 60",
  "Maternal grandfather: Colorectal polyps"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg nightly",
   "name": "Atorvastatin"
  },
  {
   "dose": "300 mg twice daily",
   "name": "Lithium carbonate"
  }
 ],
 "surgical_history": [
  "Right hemicolectomy with primary anastomosis for Stage II colon cancer",
  "Diagnostic colonoscopy with polypectomy"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia",
  "Bipolar I disorder",
  "Obstructive sleep apnea (on CPAP therapy)",
  "Nephrogenic diabetes insipidus secondary to lithium therapy",
  "Coronary artery disease",
  "Stage II colorectal adenocarcinoma, status post hemicolectomy"
 ],
 "age_at_first_encounter": 58
}
```

Primary diagnoses: ["Weber syndrome (Midbrain infarction)", "Nephrogenic Diabetes Insipidus", "Localized Idiopathic Epistaxis", "Acute myocardial infarction", "Retinal detachment", "Stable Angina Pectoris due to Coronary Artery Disease", "Colorectal neoplasia"]
Comorbidities: ["Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2023-03-29, outpatient, Oncology

Chief complaint: Follow-up visit to discuss adjuvant chemotherapy after recent hemicolectomy for Stage II colon cancer.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit to discuss adjuvant chemotherapy after recent hemicolectomy for Stage II colon cancer.

HISTORY OF PRESENT ILLNESS:
He was recently diagnosed with stage II colon cancer and underwent surgical resection. The patient is now considering adjuvant chemotherapy. He is concerned about the potential benefits and risks associated with chemotherapy.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- Bipolar I disorder
- Obstructive sleep apnea (on CPAP therapy)
- Nephrogenic diabetes insipidus secondary to lithium therapy
- Coronary artery disease
- Stage II colorectal adenocarcinoma, status post hemicolectomy

PAST SURGICAL HISTORY:
- Right hemicolectomy with primary anastomosis for Stage II colon cancer
- Diagnostic colonoscopy with polypectomy

MEDICATIONS:
His current medications include lisinopril and atorvastatin.

Home Medications:
- Lithium carbonate 300 mg twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 55, hypertension
- Mother: Type 2 diabetes, hyperlipidemia
- Paternal uncle: Ischemic stroke at age 60
- Maternal grandfather: Colorectal polyps

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Office Manager.

VITAL SIGNS:
His blood pressure is 132/84 mmHg

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

LABORATORY DATA:
Recent laboratory values are as follows:

- Hemoglobin: 13.2 g/dL
- White blood cell count: 7.2 x 10^9/L
- Platelets: 250 x 10^9/L
- Creatinine: 1.0 mg/dL
- AST: 28 U/L
- ALT: 30 U/L

OTHER STUDIES:
A recent randomized controlled trial investigating the efficacy of adjuvant chemotherapy in stage II colon cancer reported a hazard ratio of 0.75 for disease recurrence with chemotherapy as compared to the observation group. Based on this information,

## Encounter 1: source 2020-04-15, practice 2023-06-21, outpatient, Internal Medicine

Chief complaint: Excessive urination and thirst for the past two months, waking 5-6 times nightly to urinate.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Excessive urination and thirst for the past two months, waking 5-6 times nightly to urinate.

HISTORY OF PRESENT ILLNESS:
Patient with Bipolar I disorder on lithium and recent hemicolectomy returns with urinary complaints, reporting that he needs to wake up five to six times at night to urinate and feels that he cannot drink enough water to quench his thirst.

PAST MEDICAL HISTORY:
Active Problem List:
- Colorectal neoplasia (diagnosed 2020-01-15)
- Hypertension
- Hyperlipidemia
- Bipolar I disorder
- Obstructive sleep apnea (on CPAP therapy)
- Nephrogenic diabetes insipidus secondary to lithium therapy
- Coronary artery disease
- Stage II colorectal adenocarcinoma, status post hemicolectomy

PAST SURGICAL HISTORY:
- Right hemicolectomy with primary anastomosis for Stage II colon cancer
- Diagnostic colonoscopy with polypectomy

MEDICATIONS:
His bipolar disorder has been well-controlled on lithium for the past 10 years. His other medications include atorvastatin for hyperlipidemia and lisinopril for hypertension.

Home Medications:
- Lithium carbonate 300 mg twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 55, hypertension
- Mother: Type 2 diabetes, hyperlipidemia
- Paternal uncle: Ischemic stroke at age 60
- Maternal grandfather: Colorectal polyps

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Office Manager.

PHYSICAL EXAMINATION:
Physical examination reveals dry mucous membranes.

LABORATORY DATA:
His serum sodium is 146 mEq/L (normal 135-145 mEq/L), serum osmolality is 300 mOsm/kg (normal 275-295 mOsm/kg), and urine osmolality is 150 mOsm/kg (normal 500-800 mOsm/kg). Urine specific gravity is 1.005 (normal 1.010-1.025).

## Encounter 2: source 2020-07-15, practice 2023-09-25, outpatient, Internal Medicine

Chief complaint: Routine follow-up visit to discuss cardiovascular risk reduction and potential aspirin therapy for primary prevention.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Jennifer Walsh, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up visit to discuss cardiovascular risk reduction and potential aspirin therapy for primary prevention.

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up following recent hemicolectomy for Stage II colorectal cancer and nephrogenic diabetes insipidus. He has no complaints today, but is concerned about his risk of having a heart attack and asks about benefits of starting aspirin therapy for primary prevention of cardiovascular events.

PAST MEDICAL HISTORY:
Active Problem List:
- Colorectal neoplasia (diagnosed 2020-01-15)
- Nephrogenic Diabetes Insipidus (diagnosed 2020-04-15)
- Hypertension
- Hyperlipidemia
- Bipolar I disorder
- Obstructive sleep apnea (on CPAP therapy)
- Nephrogenic diabetes insipidus secondary to lithium therapy
- Coronary artery disease
- Stage II colorectal adenocarcinoma, status post hemicolectomy

PAST SURGICAL HISTORY:
- Right hemicolectomy with primary anastomosis for Stage II colon cancer
- Diagnostic colonoscopy with polypectomy

MEDICATIONS:
He has been adherent to his medication regimen, which includes lisinopril and atorvastatin.

Home Medications:
- Lithium carbonate 300 mg twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 55, hypertension
- Mother: Type 2 diabetes, hyperlipidemia
- Paternal uncle: Ischemic stroke at age 60
- Maternal grandfather: Colorectal polyps

SOCIAL HISTORY:
He does not smoke and maintains a healthy diet.

VITAL SIGNS:
His blood pressure today is 132/84 mmHg, and his BMI is 27 kg/m^2.

LABORATORY DATA:
His recent lipid panel shows LDL of 98 mg/dL, HDL of 45 mg/dL, and triglycerides of 150 mg/dL.

ASSESSMENT AND PLAN:
You recall a recent study that showed an absolute risk reduction (ARR) of 0.5% over 5 years for myocardial infarction with daily aspirin use in patients similar to him.

## Encounter 3: source 2021-01-15, practice 2024-03-27, outpatient, Cardiology

Chief complaint: Intermittent chest pressure during morning walks that radiates to left arm, resolving with rest.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Robert Kim, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent chest pressure during morning walks that radiates to left arm, resolving with rest.

HISTORY OF PRESENT ILLNESS:
The patient returns for cardiology follow-up six months after his acute myocardial infarction, presenting with recurrent chest discomfort. The pain is described as a pressure-like sensation over the left chest that radiates to his left arm, lasting for about 10 minutes and resolving with rest. He denies any symptoms at rest.

PAST MEDICAL HISTORY:
Active Problem List:
- Colorectal neoplasia (diagnosed 2020-01-15)
- Nephrogenic Diabetes Insipidus (diagnosed 2020-04-15)
- Acute myocardial infarction (diagnosed 2020-07-15)
- Hypertension
- Hyperlipidemia
- Bipolar I disorder
- Obstructive sleep apnea (on CPAP therapy)
- Nephrogenic diabetes insipidus secondary to lithium therapy
- Coronary artery disease
- Stage II colorectal adenocarcinoma, status post hemicolectomy

PAST SURGICAL HISTORY:
- Right hemicolectomy with primary anastomosis for Stage II colon cancer
- Diagnostic colonoscopy with polypectomy

MEDICATIONS:
His current medications include lisinopril and atorvastatin.

Home Medications:
- Lithium carbonate 300 mg twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 55, hypertension
- Mother: Type 2 diabetes, hyperlipidemia
- Paternal uncle: Ischemic stroke at age 60
- Maternal grandfather: Colorectal polyps

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Office Manager.

VITAL SIGNS:
His blood pressure is 135/85 mmHg, heart rate is 78 bpm,

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

IMAGING:
He undergoes a single photon emission computed tomography (SPECT) myocardial perfusion imaging during which he receives a pharmacologic stress agent due to his knee arthritis. The SPECT images reveal a reversible defect in the anterior wall on stress that is not present on rest images.

OTHER STUDIES:
An electrocardiogram shows no acute changes.

## Encounter 4: source 2021-07-15, practice 2024-09-23, outpatient, Otolaryngology

Chief complaint: Recurrent nosebleeds over the past two months, occurring mostly at night or early morning.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Emily Rodriguez, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Recurrent nosebleeds over the past two months, occurring mostly at night or early morning.

HISTORY OF PRESENT ILLNESS:
The patient returns for evaluation of recurrent nosebleeds following recent cardiology care. He reports that the nosebleeds occur mostly at night or early in the morning and are becoming more frequent. He has been using continuous positive airway pressure (CPAP) therapy for the past year.

PAST MEDICAL HISTORY:
Active Problem List:
- Colorectal neoplasia (diagnosed 2020-01-15)
- Nephrogenic Diabetes Insipidus (diagnosed 2020-04-15)
- Acute myocardial infarction (diagnosed 2020-07-15)
- Stable Angina Pectoris due to Coronary Artery Disease (diagnosed 2021-01-15)
- Hypertension
- Hyperlipidemia
- Bipolar I disorder
- Obstructive sleep apnea (on CPAP therapy)
- Nephrogenic diabetes insipidus secondary to lithium therapy
- Coronary artery disease
- Stage II colorectal adenocarcinoma, status post hemicolectomy

PAST SURGICAL HISTORY:
- Right hemicolectomy with primary anastomosis for Stage II colon cancer
- Diagnostic colonoscopy with polypectomy

MEDICATIONS:
His medications include lisinopril and atorvastatin.

Home Medications:
- Lithium carbonate 300 mg twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 55, hypertension
- Mother: Type 2 diabetes, hyperlipidemia
- Paternal uncle: Ischemic stroke at age 60
- Maternal grandfather: Colorectal polyps

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Office Manager.

VITAL SIGNS:
His blood pressure is 138/86 mmHg

PHYSICAL EXAMINATION:
Anterior rhinoscopy reveals dry, crusted mucosa with a small amount of fresh blood. There are no signs of nasal deformity or masses.

LABORATORY DATA:
His complete blood count, coagulation profile, and liver function tests are within normal limits.

## Encounter 5: source 2022-01-15, practice 2025-03-25, outpatient, Cardiology

Chief complaint: Routine cardiology follow-up to discuss adding new medication for further myocardial infarction risk reduction.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Robert Kim, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine cardiology follow-up to discuss adding new medication for further myocardial infarction risk reduction.

HISTORY OF PRESENT ILLNESS:
The patient returns for routine cardiology follow-up in the context of prior chest pressure symptoms. He inquires about the potential benefits of adding a new medication to further lower his risk of myocardial infarction (MI).

PAST MEDICAL HISTORY:
Active Problem List:
- Colorectal neoplasia (diagnosed 2020-01-15)
- Nephrogenic Diabetes Insipidus (diagnosed 2020-04-15)
- Acute myocardial infarction (diagnosed 2020-07-15)
- Stable Angina Pectoris due to Coronary Artery Disease (diagnosed 2021-01-15)
- Localized Idiopathic Epistaxis (diagnosed 2021-07-15)
- Hypertension
- Hyperlipidemia
- Bipolar I disorder
- Obstructive sleep apnea (on CPAP therapy)
- Nephrogenic diabetes insipidus secondary to lithium therapy
- Coronary artery disease
- Stage II colorectal adenocarcinoma, status post hemicolectomy

PAST SURGICAL HISTORY:
- Right hemicolectomy with primary anastomosis for Stage II colon cancer
- Diagnostic colonoscopy with polypectomy

MEDICATIONS:
He has been adherent to his medication regimen, which includes lisinopril and atorvastatin.

Home Medications:
- Lithium carbonate 300 mg twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 55, hypertension
- Mother: Type 2 diabetes, hyperlipidemia
- Paternal uncle: Ischemic stroke at age 60
- Maternal grandfather: Colorectal polyps

SOCIAL HISTORY:
He does not smoke and maintains a healthy diet.

VITAL SIGNS:
His blood pressure today is 132/84 mmHg

PHYSICAL EXAMINATION:
His physical examination is unremarkable

LABORATORY DATA:
His most recent lipid panel shows an LDL of 98 mg/dL, HDL of 45 mg/dL, and triglycerides of 150 mg/dL.

ASSESSMENT AND PLAN:
A recent large randomized controlled trial showed that adding this medication to standard therapy in patients with similar risk profiles reduced the incidence of MI from 4% to 2% over a 5-year period.

## Encounter 6: source 2022-07-15, practice 2025-09-22, ed, Emergency Medicine

Chief complaint: Sudden painless vision loss in the lower half of the left eye described as a 'curtain dropping' for the past 45 minutes.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. James Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden painless vision loss in the lower half of the left eye described as a 'curtain dropping' for the past 45 minutes.

HISTORY OF PRESENT ILLNESS:
This 60-year-old male with coronary artery disease, colorectal adenocarcinoma s/p hemicolectomy, and recent cardiology follow-up presents to the Emergency Department with sudden painless vision loss. He describes it as a "curtain dropping" from the top, which has since worsened to involve most of the visual field. He denies headache, nausea, diplopia, eye pain, or photophobia. There is no history of recent trauma, and he has not had prior similar episodes.

PAST MEDICAL HISTORY:
Active Problem List:
- Colorectal neoplasia (diagnosed 2020-01-15)
- Nephrogenic Diabetes Insipidus (diagnosed 2020-04-15)
- Acute myocardial infarction (diagnosed 2020-07-15)
- Stable Angina Pectoris due to Coronary Artery Disease (diagnosed 2021-01-15)
- Localized Idiopathic Epistaxis (diagnosed 2021-07-15)
- Hypertension
- Hyperlipidemia
- Bipolar I disorder
- Obstructive sleep apnea (on CPAP therapy)
- Nephrogenic diabetes insipidus secondary to lithium therapy
- Coronary artery disease
- Stage II colorectal adenocarcinoma, status post hemicolectomy

His medical history includes well-controlled hypertension and hyperlipidemia, but he has no known ocular disorders.

PAST SURGICAL HISTORY:
- Right hemicolectomy with primary anastomosis for Stage II colon cancer
- Diagnostic colonoscopy with polypectomy

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Atorvastatin 40 mg nightly
- Lithium carbonate 300 mg twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 55, hypertension
- Mother: Type 2 diabetes, hyperlipidemia
- Paternal uncle: Ischemic stroke at age 60
- Maternal grandfather: Colorectal polyps

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Office Manager.

PHYSICAL EXAMINATION:
On examination, visual acuity is decreased in the left eye, and the left pupil reacts sluggishly to light. Intraocular pressure is normal. Extraocular movements are intact bilaterally. Funduscopic examination is shown.

## Encounter 7: source 2023-01-15, practice 2026-04-07, ed, Emergency Medicine

Chief complaint: Sudden onset dizziness, nausea, vomiting, difficulty swallowing, and facial numbness for several hours.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset dizziness, nausea, vomiting, difficulty swallowing, and facial numbness for several hours.

HISTORY OF PRESENT ILLNESS:
This 61-year-old male with coronary artery disease, prior myocardial infarctions, and recent emergency department evaluation for retinal detachment presents to the emergency department with sudden onset of dizziness, nausea, and vomiting that began several hours ago. He also reports difficulty swallowing and a sensation of his face being numb on the same side as the dizziness.

PAST MEDICAL HISTORY:
Active Problem List:
- Colorectal neoplasia (diagnosed 2020-01-15)
- Nephrogenic Diabetes Insipidus (diagnosed 2020-04-15)
- Acute myocardial infarction (diagnosed 2020-07-15)
- Stable Angina Pectoris due to Coronary Artery Disease (diagnosed 2021-01-15)
- Localized Idiopathic Epistaxis (diagnosed 2021-07-15)
- Retinal detachment (diagnosed 2022-07-15)
- Hypertension
- Hyperlipidemia
- Bipolar I disorder
- Obstructive sleep apnea (on CPAP therapy)
- Nephrogenic diabetes insipidus secondary to lithium therapy
- Coronary artery disease
- Stage II colorectal adenocarcinoma, status post hemicolectomy

His past medical history is significant for hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Right hemicolectomy with primary anastomosis for Stage II colon cancer
- Diagnostic colonoscopy with polypectomy

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Atorvastatin 40 mg nightly
- Lithium carbonate 300 mg twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 55, hypertension
- Mother: Type 2 diabetes, hyperlipidemia
- Paternal uncle: Ischemic stroke at age 60
- Maternal grandfather: Colorectal polyps

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Office Manager.

VITAL SIGNS:
His blood pressure is 160/90 mmHg, heart rate is 88 beats per minute, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
On examination, you note that the patient has a drooping eyelid and a small pupil on the right side, decreased pain and temperature sensation on the right side of his face, and decreased pain and temperature sensation on the left side of his body. His motor examination is normal, but he has difficulty walking in a straight line. Cranial nerve examination reveals weakness of the right palatal muscles with a deviated uvula to the left. No carotid bruits are heard on auscultation.

PLAN:
An MRI of the brain with diffusion-weighted imaging is ordered.
