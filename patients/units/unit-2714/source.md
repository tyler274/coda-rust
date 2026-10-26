# synthetic_hospital patient 2714

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 47 at first encounter, sex F, Non-Hispanic White, Commercial (Employer-sponsored PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-07-05 (Mon) | outpatient | Endocrinology |
| 1 | 2020-04-15 | 2021-10-04 (Mon) | outpatient | Pulmonology |
| 2 | 2020-07-15 | 2021-12-29 (Wed) | outpatient | Hematology |
| 3 | 2021-01-15 | 2022-07-11 (Mon) | outpatient | Obstetrics & Gynecology |
| 4 | 2022-01-15 | 2023-06-27 (Tue) | ed | Emergency Medicine |
| 5 | 2023-01-15 | 2024-06-27 (Thu) | ed | Emergency Medicine |
| 6 | 2024-01-15 | 2025-07-02 (Wed) | ed | Emergency Medicine |
| 7 | 2025-01-15 | 2026-06-24 (Wed) | outpatient | Occupational Health |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Aspirin (respiratory distress/bronchospasm)",
  "Ibuprofen (cross-sensitivity with aspirin)",
  "Contrast dye (hives)"
 ],
 "insurance": "Commercial (Employer-sponsored PPO)",
 "occupation": "Registered Nurse, Medical-Surgical Unit",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Hashimoto thyroiditis, hypertension",
  "Father: Hypertension, cerebrovascular accident at age 68",
  "Sister: Premenopausal breast cancer (age 52)",
  "Maternal grandmother: Ovarian cancer (age 71)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "25 mg oral daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "10 mg oral daily",
   "name": "Lisinopril"
  },
  {
   "dose": "250/50 mcg inhaler, 1 inhalation twice daily",
   "name": "Fluticasone propionate/salmeterol"
  },
  {
   "dose": "90 mcg inhaler, 2 puffs every 4 hours as needed for wheezing",
   "name": "Albuterol sulfate"
  },
  {
   "dose": "10 mg oral daily",
   "name": "Methimazole"
  }
 ],
 "surgical_history": [
  "Total abdominal hysterectomy with bilateral salpingo-oophorectomy and omentectomy",
  "Exploratory laparotomy with tumor debulking",
  "Imaging-guided chest tube placement (for pleural effusion)",
  "Appendectomy (age 12)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Asthma with aspirin-exacerbated respiratory disease (AERD)",
  "Graves disease with ophthalmopathy",
  "History of pulmonary embolism",
  "History of epithelial ovarian carcinoma, status post optimal debulking surgery",
  "History of wound botulism",
  "Chronic immune thrombocytopenia"
 ],
 "age_at_first_encounter": 47
}
```

Primary diagnoses: ["Pulmonary embolism", "Delayed management of life-threatening respiratory failure", "Ovarian cancer", "Thrombocytopenia", "Wound botulism", "Broad-spectrum antibiotic prophylaxis (piperacillin-tazobactam)", "Aspirin-exacerbated respiratory disease", "Graves disease"]
Comorbidities: ["Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2021-07-05, outpatient, Endocrinology

Chief complaint: 6-month history of palpitations, heat intolerance, and unexplained weight loss despite an increased appetite, with recent onset of gritty eye sensation and bulging eyes

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of palpitations, heat intolerance, and unexplained weight loss despite an increased appetite, with recent onset of gritty eye sensation and bulging eyes

HISTORY OF PRESENT ILLNESS:
She also reports a recent onset of discomfort in her eyes, described as a gritty sensation, and notes that her eyes appear to be "bulging."

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Asthma with aspirin-exacerbated respiratory disease (AERD)
- Graves disease with ophthalmopathy
- History of pulmonary embolism
- History of epithelial ovarian carcinoma, status post optimal debulking surgery
- History of wound botulism
- Chronic immune thrombocytopenia

Her medical history is significant for hypertension, controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy and omentectomy
- Exploratory laparotomy with tumor debulking
- Imaging-guided chest tube placement (for pleural effusion)
- Appendectomy (age 12)

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg oral daily
- Fluticasone propionate/salmeterol 250/50 mcg inhaler, 1 inhalation twice daily
- Albuterol sulfate 90 mcg inhaler, 2 puffs every 4 hours as needed for wheezing
- Methimazole 10 mg oral daily

ALLERGIES:
Aspirin (respiratory distress/bronchospasm), Ibuprofen (cross-sensitivity with aspirin), Contrast dye (hives)

FAMILY HISTORY:
- Mother: Hashimoto thyroiditis, hypertension
- Father: Hypertension, cerebrovascular accident at age 68
- Sister: Premenopausal breast cancer (age 52)
- Maternal grandmother: Ovarian cancer (age 71)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Registered Nurse, Medical-Surgical Unit.

VITAL SIGNS:
Blood pressure is 145/90 mmHg, heart rate is 102 bpm

PHYSICAL EXAMINATION:
On physical examination, she has a diffusely enlarged, non-tender thyroid gland. Ophthalmologic examination reveals proptosis, periorbital edema, and conjunctival redness.

LABORATORY DATA:
TSH: <0.01 mIU/L (normal range: 0.4-4.0 mIU/L)
- Free T4: 3.2 ng/dL (normal range: 0.8-1.8 ng/dL)
- Thyrotropin receptor antibodies (TRAB): Positive

## Encounter 1: source 2020-04-15, practice 2021-10-04, outpatient, Pulmonology

Chief complaint: Episodic chest tightness and shortness of breath that have worsened over the past two weeks, occurring mostly during morning walks

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Johnson, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Episodic chest tightness and shortness of breath that have worsened over the past two weeks, occurring mostly during morning walks

HISTORY OF PRESENT ILLNESS:
The patient with AERD and Graves disease returns for follow-up. She reports chest tightness and shortness of breath that have worsened over the past two weeks. She notes these symptoms occur mostly during her morning walks. She denies any recent upper respiratory infections, changes in her asthma medications, or new environmental exposures.

PAST MEDICAL HISTORY:
Active Problem List:
- Graves disease (diagnosed 2020-01-15)
- Hypertension
- Asthma with aspirin-exacerbated respiratory disease (AERD)
- Graves disease with ophthalmopathy
- History of pulmonary embolism
- History of epithelial ovarian carcinoma, status post optimal debulking surgery
- History of wound botulism
- Chronic immune thrombocytopenia

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy and omentectomy
- Exploratory laparotomy with tumor debulking
- Imaging-guided chest tube placement (for pleural effusion)
- Appendectomy (age 12)

MEDICATIONS:
She takes inhaled corticosteroids and a short-acting β2 agonist as needed.

Home Medications:
- Hydrochlorothiazide 25 mg oral daily
- Lisinopril 10 mg oral daily
- Fluticasone propionate/salmeterol 250/50 mcg inhaler, 1 inhalation twice daily
- Albuterol sulfate 90 mcg inhaler, 2 puffs every 4 hours as needed for wheezing
- Methimazole 10 mg oral daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Hashimoto thyroiditis, hypertension
- Father: Hypertension, cerebrovascular accident at age 68
- Sister: Premenopausal breast cancer (age 52)
- Maternal grandmother: Ovarian cancer (age 71)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Registered Nurse, Medical-Surgical Unit.

VITAL SIGNS:
Blood pressure is 140/90 mmHg, heart rate is 78 bpm, and respiratory rate is 16 breaths per minute

PHYSICAL EXAMINATION:
Physical examination reveals bilateral expiratory wheezes.

LABORATORY DATA:
- Serum potassium: 4.2 mEq/L
- Serum creatinine: 0.9 mg/dL
- Fasting glucose: 98 mg/dL
- Total cholesterol: 220 mg/dL
- LDL cholesterol: 150 mg/dL
- HDL cholesterol: 35 mg/dL

IMAGING:
Recent echocardiogram showed mild left ventricular hypertrophy with preserved ejection fraction

ASSESSMENT AND PLAN:
Given the patient's cardiovascular risk factors and asthma,

## Encounter 2: source 2020-07-15, practice 2021-12-29, outpatient, Hematology

Chief complaint: Presenting for routine health examination with no specific complaints

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Patel, MD
Department: Hematology
Visit Type: outpatient

CHIEF COMPLAINT:
Presenting for routine health examination with no specific complaints

HISTORY OF PRESENT ILLNESS:
The patient returns for hematology follow-up. She reports feeling well with no complaints of fatigue, bleeding, or bruising.

PAST MEDICAL HISTORY:
Active Problem List:
- Graves disease (diagnosed 2020-01-15)
- Aspirin-exacerbated respiratory disease (diagnosed 2020-04-15)
- Hypertension
- Asthma with aspirin-exacerbated respiratory disease (AERD)
- Graves disease with ophthalmopathy
- History of pulmonary embolism
- History of epithelial ovarian carcinoma, status post optimal debulking surgery
- History of wound botulism
- Chronic immune thrombocytopenia

Her past medical history is significant for hypertension, controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy and omentectomy
- Exploratory laparotomy with tumor debulking
- Imaging-guided chest tube placement (for pleural effusion)
- Appendectomy (age 12)

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg oral daily
- Fluticasone propionate/salmeterol 250/50 mcg inhaler, 1 inhalation twice daily
- Albuterol sulfate 90 mcg inhaler, 2 puffs every 4 hours as needed for wheezing
- Methimazole 10 mg oral daily

ALLERGIES:
Aspirin (respiratory distress/bronchospasm), Ibuprofen (cross-sensitivity with aspirin), Contrast dye (hives)

FAMILY HISTORY:
- Mother: Hashimoto thyroiditis, hypertension
- Father: Hypertension, cerebrovascular accident at age 68
- Sister: Premenopausal breast cancer (age 52)
- Maternal grandmother: Ovarian cancer (age 71)

SOCIAL HISTORY:
She denies alcohol use.

VITAL SIGNS:
On physical examination, her blood pressure is 138/86 mmHg, heart rate is 78 beats per minute

PHYSICAL EXAMINATION:
The rest of the examination is unremarkable.

LABORATORY DATA:
Her complete blood count (CBC) reveals the following:

- Hemoglobin: 14.2 g/dL
- Hematocrit: 42.5%
- White Blood Cells (WBC): 6.8 x 10^3/uL
- Platelets: 45 x 10^3/uL

OTHER STUDIES:
A peripheral blood smear is performed, which shows large clumps of platelets. There is no evidence of schistocytes or other abnormalities.

## Encounter 3: source 2021-01-15, practice 2022-07-11, outpatient, Obstetrics & Gynecology

Chief complaint: 3-month history of increasing abdominal girth and early satiety without significant weight loss, fever, or bowel changes

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Emily Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of increasing abdominal girth and early satiety without significant weight loss, fever, or bowel changes

HISTORY OF PRESENT ILLNESS:
The patient, with a history of epithelial ovarian carcinoma, Graves disease, aspirin-exacerbated respiratory disease, and chronic immune thrombocytopenia, presents with increasing abdominal girth and early satiety. She denies any significant weight loss, fever, or changes in bowel or bladder habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Graves disease (diagnosed 2020-01-15)
- Aspirin-exacerbated respiratory disease (diagnosed 2020-04-15)
- Thrombocytopenia (diagnosed 2020-07-15)
- Hypertension
- Asthma with aspirin-exacerbated respiratory disease (AERD)
- Graves disease with ophthalmopathy
- History of pulmonary embolism
- History of epithelial ovarian carcinoma, status post optimal debulking surgery
- History of wound botulism
- Chronic immune thrombocytopenia

Her past medical history is significant for hypertension, controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy and omentectomy
- Exploratory laparotomy with tumor debulking
- Imaging-guided chest tube placement (for pleural effusion)
- Appendectomy (age 12)

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg oral daily
- Fluticasone propionate/salmeterol 250/50 mcg inhaler, 1 inhalation twice daily
- Albuterol sulfate 90 mcg inhaler, 2 puffs every 4 hours as needed for wheezing
- Methimazole 10 mg oral daily

ALLERGIES:
Aspirin (respiratory distress/bronchospasm), Ibuprofen (cross-sensitivity with aspirin), Contrast dye (hives)

FAMILY HISTORY:
She has no family history of breast or ovarian cancer.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Registered Nurse, Medical-Surgical Unit.

PHYSICAL EXAMINATION:
On physical examination, her abdomen is non-tender, but a fixed, irregular pelvic mass is palpable.

LABORATORY DATA:
CA-125 level is 250 U/mL (normal range: 0-35 U/mL).

IMAGING:
Transvaginal ultrasound reveals a 7 cm left adnexal mass with complex appearance, including irregular thickened septations and papillary projections. There is no free fluid in the pelvis.

## Encounter 4: source 2022-01-15, practice 2023-06-27, ed, Emergency Medicine

Chief complaint: Sudden onset of shortness of breath and left-sided pleuritic chest pain starting 2 hours ago, following transatlantic flight 3 days prior

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Michael Chang, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of shortness of breath and left-sided pleuritic chest pain starting 2 hours ago, following transatlantic flight 3 days prior

HISTORY OF PRESENT ILLNESS:
This 49-year-old female with a history of pulmonary embolism and epithelial ovarian carcinoma presents to the emergency department with sudden onset of shortness of breath and left-sided pleuritic chest pain that started 2 hours ago. She returned from a transatlantic flight 3 days prior.

PAST MEDICAL HISTORY:
Active Problem List:
- Graves disease (diagnosed 2020-01-15)
- Aspirin-exacerbated respiratory disease (diagnosed 2020-04-15)
- Thrombocytopenia (diagnosed 2020-07-15)
- Ovarian cancer (diagnosed 2021-01-15)
- Hypertension
- Asthma with aspirin-exacerbated respiratory disease (AERD)
- Graves disease with ophthalmopathy
- History of pulmonary embolism
- History of epithelial ovarian carcinoma, status post optimal debulking surgery
- History of wound botulism
- Chronic immune thrombocytopenia

Her past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy and omentectomy
- Exploratory laparotomy with tumor debulking
- Imaging-guided chest tube placement (for pleural effusion)
- Appendectomy (age 12)

MEDICATIONS:
She takes for which she takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg oral daily
- Fluticasone propionate/salmeterol 250/50 mcg inhaler, 1 inhalation twice daily
- Albuterol sulfate 90 mcg inhaler, 2 puffs every 4 hours as needed for wheezing
- Methimazole 10 mg oral daily

ALLERGIES:
She has no known drug allergies.

FAMILY HISTORY:
- Mother: Hashimoto thyroiditis, hypertension
- Father: Hypertension, cerebrovascular accident at age 68
- Sister: Premenopausal breast cancer (age 52)
- Maternal grandmother: Ovarian cancer (age 71)

SOCIAL HISTORY:
She does not smoke

VITAL SIGNS:
On examination, her blood pressure is 135/85 mmHg, heart rate is 110 beats per minute, respiratory rate is 24 breaths per minute, and oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
Her jugular venous pressure is not elevated. Lung auscultation reveals decreased breath sounds at the left lung base without wheezes or crackles.

IMAGING:
A chest radiograph shows no infiltrates or pleural effusion.

OTHER STUDIES:
An electrocardiogram (ECG) shows sinus tachycardia with no ST changes or T wave inversions.

## Encounter 5: source 2023-01-15, practice 2024-06-27, ed, Emergency Medicine

Chief complaint: 12-hour history of blurred vision, difficulty swallowing, and progressive weakness following consumption of homemade canned food

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Aisha Washington, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
12-hour history of blurred vision, difficulty swallowing, and progressive weakness following consumption of homemade canned food

HISTORY OF PRESENT ILLNESS:
She has a history of wound botulism. She reports attending a family picnic two days ago where she consumed homemade canned green beans.

PAST MEDICAL HISTORY:
Active Problem List:
- Graves disease (diagnosed 2020-01-15)
- Aspirin-exacerbated respiratory disease (diagnosed 2020-04-15)
- Thrombocytopenia (diagnosed 2020-07-15)
- Ovarian cancer (diagnosed 2021-01-15)
- Pulmonary embolism (diagnosed 2022-01-15)
- Hypertension
- Asthma with aspirin-exacerbated respiratory disease (AERD)
- Graves disease with ophthalmopathy
- History of epithelial ovarian carcinoma, status post optimal debulking surgery
- History of wound botulism
- Chronic immune thrombocytopenia

Her past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy and omentectomy
- Exploratory laparotomy with tumor debulking
- Imaging-guided chest tube placement (for pleural effusion)
- Appendectomy (age 12)

MEDICATIONS:
She takes for which she takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg oral daily
- Fluticasone propionate/salmeterol 250/50 mcg inhaler, 1 inhalation twice daily
- Albuterol sulfate 90 mcg inhaler, 2 puffs every 4 hours as needed for wheezing
- Methimazole 10 mg oral daily

ALLERGIES:
Aspirin (respiratory distress/bronchospasm), Ibuprofen (cross-sensitivity with aspirin), Contrast dye (hives)

FAMILY HISTORY:
- Mother: Hashimoto thyroiditis, hypertension
- Father: Hypertension, cerebrovascular accident at age 68
- Sister: Premenopausal breast cancer (age 52)
- Maternal grandmother: Ovarian cancer (age 71)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Registered Nurse, Medical-Surgical Unit.

VITAL SIGNS:
On examination, her vital signs are stable

PHYSICAL EXAMINATION:
She exhibits bilateral ptosis, fixed dilated pupils, and a symmetric descending weakness with normal sensation. Deep tendon reflexes are diminished throughout. She has no fever, and her mental status is intact.

LABORATORY DATA:
- Serum sodium: 140 mEq/L
- Serum potassium: 4.1 mEq/L
- Serum chloride: 100 mEq/L
- Serum creatinine: 0.9 mg/dL
- White blood cell count: 7,000/uL with a normal differential
- Negative serum pregnancy test

OTHER STUDIES:
A stool sample is sent for analysis.

## Encounter 6: source 2024-01-15, practice 2025-07-02, ed, Emergency Medicine

Chief complaint: Severe left-sided back pain for 12 hours, constant and non-radiating, unresponsive to over-the-counter analgesics

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe left-sided back pain for 12 hours, constant and non-radiating, unresponsive to over-the-counter analgesics

HISTORY OF PRESENT ILLNESS:
This 51-year-old female with a history of epithelial ovarian carcinoma and chronic immune thrombocytopenia presents with severe left-sided back pain that started 12 hours ago. She reports that the pain is constant, non-radiating, and has not responded to over-the-counter analgesics.

PAST MEDICAL HISTORY:
Active Problem List:
- Graves disease (diagnosed 2020-01-15)
- Aspirin-exacerbated respiratory disease (diagnosed 2020-04-15)
- Thrombocytopenia (diagnosed 2020-07-15)
- Ovarian cancer (diagnosed 2021-01-15)
- Pulmonary embolism (diagnosed 2022-01-15)
- Wound botulism (diagnosed 2023-01-15)
- Hypertension
- Asthma with aspirin-exacerbated respiratory disease (AERD)
- Graves disease with ophthalmopathy
- History of epithelial ovarian carcinoma, status post optimal debulking surgery
- Chronic immune thrombocytopenia

Her past medical history is significant for hypertension,.

PAST SURGICAL HISTORY:
She underwent a total abdominal hysterectomy 3 days ago for uterine fibroids.

MEDICATIONS:
She takes controlled with lisinopril.

Home Medications:
- Hydrochlorothiazide 25 mg oral daily
- Fluticasone propionate/salmeterol 250/50 mcg inhaler, 1 inhalation twice daily
- Albuterol sulfate 90 mcg inhaler, 2 puffs every 4 hours as needed for wheezing
- Methimazole 10 mg oral daily

ALLERGIES:
Aspirin (respiratory distress/bronchospasm), Ibuprofen (cross-sensitivity with aspirin), Contrast dye (hives)

FAMILY HISTORY:
- Mother: Hashimoto thyroiditis, hypertension
- Father: Hypertension, cerebrovascular accident at age 68
- Sister: Premenopausal breast cancer (age 52)
- Maternal grandmother: Ovarian cancer (age 71)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Registered Nurse, Medical-Surgical Unit.

VITAL SIGNS:
On examination, she is afebrile, blood pressure is 135/85 mmHg, heart rate is 88 beats per minute,

PHYSICAL EXAMINATION:
She appears uncomfortable. There is marked tenderness to palpation over the left costovertebral angle.

LABORATORY DATA:
Laboratory results are as follows:

- Serum creatinine: 1.8 mg/dL (baseline 0.9 mg/dL)
- Blood urea nitrogen (BUN): 30 mg/dL
- Hemoglobin: 12.5 g/dL
- White blood cell count: 9,000 /µL
- Urinalysis: Normal, no hematuria or pyuria

IMAGING:
A non-contrast CT scan of the abdomen and pelvis shows moderate left hydronephrosis without evidence of nephrolithiasis.

## Encounter 7: source 2025-01-15, practice 2026-06-24, outpatient, Occupational Health

Chief complaint: During a routine infection control audit, the nurse is observed and asked to review appropriate glove disposal practices

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Lisa Thompson, MD
Department: Occupational Health
Visit Type: outpatient

CHIEF COMPLAINT:
During a routine infection control audit, the nurse is observed and asked to review appropriate glove disposal practices

HISTORY OF PRESENT ILLNESS:
The 52-year-old female with history of wound botulism (2023) and AERD presents for Occupational Health review. She is observed completing wound care for a patient with a diabetic foot ulcer infected with Pseudomonas aeruginosa. After removing her gloves, she immediately reaches to adjust the IV pump and then disposes of the gloves in the nearest trash bin before performing hand hygiene.

PAST MEDICAL HISTORY:
Active Problem List:
- Graves disease (diagnosed 2020-01-15)
- Aspirin-exacerbated respiratory disease (diagnosed 2020-04-15)
- Thrombocytopenia (diagnosed 2020-07-15)
- Ovarian cancer (diagnosed 2021-01-15)
- Pulmonary embolism (diagnosed 2022-01-15)
- Wound botulism (diagnosed 2023-01-15)
- Broad-spectrum antibiotic prophylaxis (piperacillin-tazobactam) (diagnosed 2024-01-15)
- Hypertension
- Asthma with aspirin-exacerbated respiratory disease (AERD)
- Graves disease with ophthalmopathy
- History of epithelial ovarian carcinoma, status post optimal debulking surgery
- Chronic immune thrombocytopenia

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy and omentectomy
- Exploratory laparotomy with tumor debulking
- Imaging-guided chest tube placement (for pleural effusion)
- Appendectomy (age 12)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg oral daily
- Lisinopril 10 mg oral daily
- Fluticasone propionate/salmeterol 250/50 mcg inhaler, 1 inhalation twice daily
- Albuterol sulfate 90 mcg inhaler, 2 puffs every 4 hours as needed for wheezing
- Methimazole 10 mg oral daily

ALLERGIES:
Aspirin (respiratory distress/bronchospasm), Ibuprofen (cross-sensitivity with aspirin), Contrast dye (hives)

FAMILY HISTORY:
- Mother: Hashimoto thyroiditis, hypertension
- Father: Hypertension, cerebrovascular accident at age 68
- Sister: Premenopausal breast cancer (age 52)
- Maternal grandmother: Ovarian cancer (age 71)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Registered Nurse, Medical-Surgical Unit.
