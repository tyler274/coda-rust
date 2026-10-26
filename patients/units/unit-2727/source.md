# synthetic_hospital patient 2727

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 43 at first encounter, sex F, Non-Hispanic White, Aetna PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-07-28 (Tue) | outpatient | Endocrinology |
| 1 | 2020-07-15 | 2021-02-01 (Mon) | outpatient | Psychiatry |
| 2 | 2021-01-15 | 2021-08-04 (Wed) | outpatient | Obstetrics & Gynecology |
| 3 | 2022-01-15 | 2022-07-28 (Thu) | outpatient | Orthopedics |
| 4 | 2023-01-15 | 2023-07-24 (Mon) | outpatient | Gastroenterology |
| 5 | 2024-01-15 | 2024-07-24 (Wed) | outpatient | Endocrinology |
| 6 | 2025-01-15 | 2025-07-28 (Mon) | outpatient | Internal Medicine |
| 7 | 2025-04-15 | 2025-11-03 (Mon) | outpatient | Internal Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Aetna PPO",
 "occupation": "Office worker",
 "alcohol_use": "Social (occasional)",
 "family_history": [
  "Mother: Hashimoto's thyroiditis",
  "Sister: Major depressive disorder",
  "Father: Essential hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "100 mcg PO daily",
   "name": "Levothyroxine"
  },
  {
   "dose": "225 mg PO daily",
   "name": "Venlafaxine"
  }
 ],
 "surgical_history": [
  "Hysteroscopic polypectomy"
 ],
 "chronic_conditions": [
  "Hashimoto's thyroiditis",
  "Major depressive disorder, recurrent",
  "Secondary hypertension",
  "Cushing's syndrome (ACTH-dependent)",
  "Carpal tunnel syndrome",
  "Bile acid malabsorption"
 ],
 "age_at_first_encounter": 43
}
```

Primary diagnoses: ["Secondary hypertension", "Hashimoto thyroiditis", "Endometrial polyp", "Major depressive disorder", "Carpal tunnel syndrome", "Bile Acid Diarrhea", "ACTH-producing pituitary adenoma"]
Comorbidities: ["Major depressive disorder", "Hypothyroidism", "Obesity"]

## Encounter 0: source 2020-01-15, practice 2020-07-28, outpatient, Endocrinology

Chief complaint: 3 months of feeling increasingly tired while performing routine house chores with recent cold intolerance and mild difficulty concentrating

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
3 months of feeling increasingly tired while performing routine house chores with recent cold intolerance and mild difficulty concentrating

HISTORY OF PRESENT ILLNESS:
Upon further questioning, she reports recently feeling colder than usual in the evenings and having some mild difficulty with concentration while reading. Her appetite has remained stable, but she has noticed an unintentional weight gain of approximately 4 kg over the last few months, despite no changes in diet or activity level.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto's thyroiditis
- Major depressive disorder, recurrent
- Secondary hypertension
- Cushing's syndrome (ACTH-dependent)
- Carpal tunnel syndrome
- Bile acid malabsorption

PAST SURGICAL HISTORY:
- Hysteroscopic polypectomy

MEDICATIONS:
Home Medications:
- Levothyroxine 100 mcg PO daily
- Venlafaxine 225 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Family history is notable for a maternal uncle who was diagnosed with medullary thyroid cancer in his 50s.

SOCIAL HISTORY:
She does not smoke, drink alcohol, or use illicit drugs.

REVIEW OF SYSTEMS:
She denies constipation, hair loss, or mood changes.

VITAL SIGNS:
Her temperature is 36.4°C (97.5°F), blood pressure is 112/72 mmHg, pulse is 58/min, and respiratory rate is 14/min. She is 163 cm (5 ft 4 in) tall and weighs 74 kg (163 lb); her BMI is 27.9 kg/m².

PHYSICAL EXAMINATION:
Physical examination shows dry skin over the extensor surfaces, delayed relaxation of deep tendon reflexes, and mild periorbital puffiness. The thyroid gland is firm and diffusely enlarged without nodularity or tenderness.

LABORATORY DATA:
Laboratory studies reveal a hemoglobin level of 10.8 g/dL with a mean corpuscular volume of 102 fL. Thyroid function testing shows a free thyroxine (T4) level of 0.5 ng/dL and a thyroid-stimulating hormone (TSH) level of 11.2 µIU/mL.

PATHOLOGY:
A biopsy of the thyroid gland is performed.

## Encounter 1: source 2020-07-15, practice 2021-02-01, outpatient, Psychiatry

Chief complaint: Fatigue and generalized headaches for the past two months with difficulty sleeping and 5-pound weight gain

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Morrison, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue and generalized headaches for the past two months with difficulty sleeping and 5-pound weight gain

HISTORY OF PRESENT ILLNESS:
The patient returns for psychiatric follow-up six months following her Endocrinology evaluation for Hashimoto's thyroiditis. She reports that the headaches are generalized, not associated with nausea or photophobia, and do not improve with over-the-counter analgesics. She also mentions difficulty sleeping and a 5-pound weight gain. She denies any recent changes in medication, diet, or stress levels.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Hashimoto's thyroiditis
- Major depressive disorder, recurrent
- Secondary hypertension
- Cushing's syndrome (ACTH-dependent)
- Carpal tunnel syndrome
- Bile acid malabsorption

Her past medical history is significant for hypothyroidism.

PAST SURGICAL HISTORY:
- Hysteroscopic polypectomy

MEDICATIONS:
She takes for which she takes levothyroxine.

Home Medications:
- Venlafaxine 225 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hashimoto's thyroiditis
- Sister: Major depressive disorder
- Father: Essential hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (occasional). Occupation: Office worker.

VITAL SIGNS:
On physical examination, her blood pressure is 130/85 mmHg, heart rate is 78 bpm, and BMI is 26

PHYSICAL EXAMINATION:
Thyroid examination is normal

LABORATORY DATA:
Laboratory tests reveal:

- TSH: 2.5 mIU/L (normal range: 0.4-4.0 mIU/L)
- Free T4: 1.2 ng/dL (normal range: 0.8-1.8 ng/dL)
- CBC: within normal limits
- Fasting glucose: 98 mg/dL (normal range: 70-99 mg/dL)

ASSESSMENT AND PLAN:
Given the patient's presentation and normal laboratory values,

## Encounter 2: source 2021-01-15, practice 2021-08-04, outpatient, Obstetrics & Gynecology

Chief complaint: 6-month history of irregular vaginal bleeding with intermenstrual spotting in previously regular 28-day cycles

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Maria Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of irregular vaginal bleeding with intermenstrual spotting in previously regular 28-day cycles

HISTORY OF PRESENT ILLNESS:
This 44-year-old female with Hashimoto's thyroiditis and major depressive disorder, previously evaluated by Endocrinology and Psychiatry for fatigue and mood symptoms, presents with menstrual irregularities. She reports that her menstrual cycles are usually regular, with a 28-day cycle, but she has been experiencing spotting and light bleeding between periods for the past several months. She denies any postcoital bleeding or dyspareunia.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-07-15)
- Hashimoto's thyroiditis
- Major depressive disorder, recurrent
- Secondary hypertension
- Cushing's syndrome (ACTH-dependent)
- Carpal tunnel syndrome
- Bile acid malabsorption

Her past medical history is significant for hypothyroidism.

PAST SURGICAL HISTORY:
- Hysteroscopic polypectomy

MEDICATIONS:
She takes for which she takes levothyroxine.

Home Medications:
- Venlafaxine 225 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hashimoto's thyroiditis
- Sister: Major depressive disorder
- Father: Essential hypertension

SOCIAL HISTORY:
She has never been pregnant and does not use tobacco or alcohol.

VITAL SIGNS:
On physical examination, her vital signs are within normal limits, and her BMI is 26 kg/m^2

PHYSICAL EXAMINATION:
A pelvic examination reveals a normal-sized uterus without adnexal masses or tenderness.

LABORATORY DATA:
Her recent Pap smear was normal, and her thyroid-stimulating hormone (TSH) level is within the therapeutic range.

IMAGING:
A transvaginal ultrasound is performed, which shows a 1.5 cm hyperechoic lesion within the endometrial cavity suggestive of an endometrial polyp.

## Encounter 3: source 2022-01-15, practice 2022-07-28, outpatient, Orthopedics

Chief complaint: Numbness and tingling in right thumb, index, and middle fingers with nocturnal awakening and difficulty grasping objects

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Robert Kim, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Numbness and tingling in right thumb, index, and middle fingers with nocturnal awakening and difficulty grasping objects

HISTORY OF PRESENT ILLNESS:
The patient with Hashimoto's thyroiditis and depression presents with a hand complaint that has been occurring intermittently for the past six months. She notes that the symptoms are worse at night and often awaken her from sleep. She also reports that she has been dropping objects and has difficulty with tasks such as buttoning her shirt.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-07-15)
- Endometrial polyp (diagnosed 2021-01-15)
- Hashimoto's thyroiditis
- Major depressive disorder, recurrent
- Secondary hypertension
- Cushing's syndrome (ACTH-dependent)
- Carpal tunnel syndrome
- Bile acid malabsorption

Her medical history is significant for hypothyroidism.

PAST SURGICAL HISTORY:
- Hysteroscopic polypectomy

MEDICATIONS:
She takes levothyroxine.

Home Medications:
- Venlafaxine 225 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hashimoto's thyroiditis
- Sister: Major depressive disorder
- Father: Essential hypertension

SOCIAL HISTORY:
Office worker

PHYSICAL EXAMINATION:
On physical examination, you note thenar eminence atrophy and a positive Tinel's sign at the right wrist. Phalen's test is also positive on the right. Sensation is intact in the forearm and upper arm.

## Encounter 4: source 2023-01-15, practice 2023-07-24, outpatient, Gastroenterology

Chief complaint: 6-month history of chronic diarrhea with 4-6 loose watery bowel movements daily and 5 kg unintentional weight loss

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Emily Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of chronic diarrhea with 4-6 loose watery bowel movements daily and 5 kg unintentional weight loss

HISTORY OF PRESENT ILLNESS:
The patient presents to Gastroenterology with a history of Hashimoto's thyroiditis, recurrent major depressive disorder, and Cushing's syndrome. She reports four to six loose, watery bowel movements per day, which occur throughout the day and occasionally wake her from sleep. Since then, she has lost 5 kg unintentionally. She denies any blood or mucus in her stool.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-07-15)
- Endometrial polyp (diagnosed 2021-01-15)
- Carpal tunnel syndrome (diagnosed 2022-01-15)
- Hashimoto's thyroiditis
- Major depressive disorder, recurrent
- Secondary hypertension
- Cushing's syndrome (ACTH-dependent)
- Bile acid malabsorption

Her past medical history is significant for hypothyroidism,.

PAST SURGICAL HISTORY:
She underwent a laparoscopic cholecystectomy 8 months ago for symptomatic cholelithiasis.

MEDICATIONS:
She takes for which she takes levothyroxine.

Home Medications:
- Venlafaxine 225 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hashimoto's thyroiditis
- Sister: Major depressive disorder
- Father: Essential hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (occasional). Occupation: Office worker.

PHYSICAL EXAMINATION:
On physical examination, her abdomen is soft, non-distended, with no tenderness or palpable masses. Bowel sounds are normal.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 13.2 g/dL (normal: 12-16 g/dL)
- White blood cell count: 7,000/mm³ (normal: 4,500-11,000/mm³)
- Platelets: 250,000/mm³ (normal: 150,000-450,000/mm³)
- Sodium: 140 mEq/L (normal: 135-145 mEq/L)
- Potassium: 4.2 mEq/L (normal: 3.5-5.1 mEq/L)
- Chloride: 102 mEq/L (normal: 98-106 mEq/L)
- Bicarbonate: 24 mEq/L (normal: 22-29 mEq/L)
- Blood urea nitrogen: 18 mg/dL (normal: 7-20 mg/dL)
- Creatinine: 0.9 mg/dL (normal: 0.6-1.1 mg/dL)
- Fasting serum bile acids: Elevated

Stool studies, including cultures, ova and parasites, and Clostridioides difficile toxin, are negative. Fecal fat is normal.

## Encounter 5: source 2024-01-15, practice 2024-07-24, outpatient, Endocrinology

Chief complaint: Progressive facial rounding, supraclavicular fat pads, increased abdominal girth, and proximal muscle weakness

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Michael Okonkwo, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive facial rounding, supraclavicular fat pads, increased abdominal girth, and proximal muscle weakness

HISTORY OF PRESENT ILLNESS:
Patient with Hashimoto's thyroiditis and depression returns. She presents with progressive facial rounding, supraclavicular fat pads, and increased abdominal girth. She also reports proximal muscle weakness, mood swings, and easy bruising.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-07-15)
- Endometrial polyp (diagnosed 2021-01-15)
- Carpal tunnel syndrome (diagnosed 2022-01-15)
- Bile Acid Diarrhea (diagnosed 2023-01-15)
- Hashimoto's thyroiditis
- Major depressive disorder, recurrent
- Secondary hypertension
- Cushing's syndrome (ACTH-dependent)
- Bile acid malabsorption

PAST SURGICAL HISTORY:
- Hysteroscopic polypectomy

MEDICATIONS:
Home Medications:
- Levothyroxine 100 mcg PO daily
- Venlafaxine 225 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hashimoto's thyroiditis
- Sister: Major depressive disorder
- Father: Essential hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (occasional). Occupation: Office worker.

VITAL SIGNS:
Her blood pressure is 158/92 mmHg.

PHYSICAL EXAMINATION:
Physical exam shows purple striae across the lower abdomen and facial plethora.

LABORATORY DATA:
Laboratory evaluation reveals elevated 24-hour urinary free cortisol and high midnight salivary cortisol. Her ACTH levels are elevated.

IMAGING:
A subsequent MRI of the brain shows a 7-mm mass in the anterior pituitary.

OTHER STUDIES:
Low-dose dexamethasone fails to suppress her cortisol level. High-dose dexamethasone suppression testing is now performed.

## Encounter 6: source 2025-01-15, practice 2025-07-28, outpatient, Internal Medicine

Chief complaint: Routine follow-up for secondary hypertension with occasional headaches and elevated home blood pressure readings

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Jennifer Walsh, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for secondary hypertension with occasional headaches and elevated home blood pressure readings

HISTORY OF PRESENT ILLNESS:
This 48-year-old female with Hashimoto's thyroiditis, ACTH-dependent Cushing's syndrome, and major depressive disorder returns for follow-up of secondary hypertension. She reports feeling more energetic and having an improved mood since her last visit 3 months ago when her medication was changed to venlafaxine. However, she has recently been experiencing occasional headaches and notes that her home blood pressure monitor has shown higher readings than usual.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-07-15)
- Endometrial polyp (diagnosed 2021-01-15)
- Carpal tunnel syndrome (diagnosed 2022-01-15)
- Bile Acid Diarrhea (diagnosed 2023-01-15)
- ACTH-producing pituitary adenoma (diagnosed 2024-01-15)
- Hashimoto's thyroiditis
- Major depressive disorder, recurrent
- Secondary hypertension
- Cushing's syndrome (ACTH-dependent)
- Bile acid malabsorption

Medical history significant for hypothyroidism, well-controlled on levothyroxine.

PAST SURGICAL HISTORY:
- Hysteroscopic polypectomy

MEDICATIONS:
Her current medications include venlafaxine 225 mg daily and levothyroxine.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hashimoto's thyroiditis
- Sister: Major depressive disorder
- Father: Essential hypertension

SOCIAL HISTORY:
She does not smoke and drinks alcohol socially.

REVIEW OF SYSTEMS:
She denies any chest pain, palpitations, or shortness of breath.

VITAL SIGNS:
Blood pressure is 158/96 mmHg, heart rate is 78 beats per minute, and BMI is 27 kg/m^2

PHYSICAL EXAMINATION:
The remainder of the physical examination is unremarkable.

LABORATORY DATA:
Serum sodium: 140 mEq/L
Serum potassium: 4.2 mEq/L
Serum creatinine: 0.9 mg/dL
TSH: 2.5 mIU/L (within normal range)
Total cholesterol: 195 mg/dL
LDL cholesterol: 120 mg/dL
HDL cholesterol: 45 mg/dL

## Encounter 7: source 2025-04-15, practice 2025-11-03, outpatient, Internal Medicine

Chief complaint: Routine follow-up with occasional headaches and heart pounding for the past month

ENCOUNTER NOTE
Date: 2025-04-15
Provider: Dr. Jennifer Walsh, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up with occasional headaches and heart pounding for the past month

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of secondary hypertension and depression, with ongoing management of Cushing's syndrome and Hashimoto's thyroiditis. She reports feeling more energetic and having an improved mood since her last visit 3 months ago when her medication was changed to venlafaxine. However, she mentions experiencing occasional headaches and "heart pounding" for the past month. She denies chest pain, shortness of breath, or dizziness.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-07-15)
- Endometrial polyp (diagnosed 2021-01-15)
- Carpal tunnel syndrome (diagnosed 2022-01-15)
- Bile Acid Diarrhea (diagnosed 2023-01-15)
- ACTH-producing pituitary adenoma (diagnosed 2024-01-15)
- Secondary hypertension (diagnosed 2025-01-15)
- Hashimoto's thyroiditis
- Major depressive disorder, recurrent
- Cushing's syndrome (ACTH-dependent)
- Bile acid malabsorption

PAST SURGICAL HISTORY:
- Hysteroscopic polypectomy

MEDICATIONS:
Her current medications include venlafaxine 225 mg daily and levothyroxine 100 mcg daily.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hashimoto's thyroiditis
- Sister: Major depressive disorder
- Father: Essential hypertension

SOCIAL HISTORY:
She does not smoke or use illicit drugs and drinks alcohol socially.

VITAL SIGNS:
Her blood pressure today is 158/96 mmHg, which is increased from 132/84 mmHg recorded 3 months ago. Her heart rate is 78 bpm,

PHYSICAL EXAMINATION:
The rest of the physical examination is unremarkable.

LABORATORY DATA:
Laboratory values:
- TSH: 2.5 mIU/L (normal range: 0.4-4.0 mIU/L)
- Serum glucose: 98 mg/dL (normal range: 70-99 mg/dL)
- Serum creatinine: 0.9 mg/dL (normal range: 0.6-1.1 mg/dL)
- Total cholesterol: 190 mg/dL (normal range: <200 mg/dL)
