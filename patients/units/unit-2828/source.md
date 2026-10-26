# synthetic_hospital patient 2828

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex F, White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-11-16 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2021-05-10 (Mon) | outpatient | Internal Medicine |
| 2 | 2021-01-15 | 2021-11-16 (Tue) | outpatient | Obstetrics & Gynecology |
| 3 | 2022-01-15 | 2022-11-28 (Mon) | outpatient | Gastroenterology |
| 4 | 2022-07-15 | 2023-05-11 (Thu) | outpatient | Obstetrics & Gynecology |
| 5 | 2023-07-15 | 2024-05-14 (Tue) | ed | Emergency Medicine |
| 6 | 2024-01-15 | 2024-11-19 (Tue) | ed | Emergency Medicine |
| 7 | 2025-01-15 | 2025-11-25 (Tue) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired travel journalist; former volunteer teacher",
 "alcohol_use": "Occasional (1-2 drinks monthly)",
 "family_history": [
  "Mother: Breast cancer (age 62)",
  "Father: Myocardial infarction (age 58)",
  "Sister: Ovarian cancer"
 ],
 "race_ethnicity": "White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "81 mg daily",
   "name": "Aspirin"
  },
  {
   "dose": "75 mcg daily",
   "name": "Levothyroxine"
  },
  {
   "dose": "20 mg daily",
   "name": "Omeprazole"
  },
  {
   "dose": "500 mg every 6 hours PRN pain",
   "name": "Acetaminophen"
  },
  {
   "dose": "17 g daily PRN constipation",
   "name": "Polyethylene glycol 3350"
  }
 ],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Coronary artery disease",
  "Osteoarthritis (bilateral hips and knees)",
  "Gastroesophageal reflux disease",
  "Obesity",
  "Hypothyroidism",
  "Chronic constipation with laxative dependence",
  "Postmenopausal status (menopause at age 51, no hormone replacement therapy)"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Diverticulosis of colon", "Flash pulmonary edema", "Paraesophageal hiatal hernia", "Osteoarthritis of knee", "Urge incontinence (Overactive bladder)", "Serous cystadenocarcinoma of ovary", "Lichen sclerosus", "Acute kidney injury"]
Comorbidities: ["Postmenopausal status", "Osteoarthritis", "Essential hypertension", "Obesity", "Coronary Artery Disease (Antiplatelet Therapy)"]

## Encounter 0: source 2020-01-15, practice 2020-11-16, outpatient, Internal Medicine

Chief complaint: Progressive bilateral knee pain over the past year, worse with walking and prolonged standing

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive bilateral knee pain over the past year, worse with walking and prolonged standing

HISTORY OF PRESENT ILLNESS:
She comes to clinic due to progressive knee pain over the past year. She describes the pain as a dull ache that worsens with walking or prolonged standing and improves with rest. She has no history of trauma or joint swelling.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Coronary artery disease
- Osteoarthritis (bilateral hips and knees)
- Gastroesophageal reflux disease
- Obesity
- Hypothyroidism
- Chronic constipation with laxative dependence
- Postmenopausal status (menopause at age 51, no hormone replacement therapy)

Her medical history includes hypertension and gastroesophageal reflux disease.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Aspirin 81 mg daily
- Levothyroxine 75 mcg daily
- Omeprazole 20 mg daily
- Acetaminophen 500 mg every 6 hours PRN pain
- Polyethylene glycol 3350 17 g daily PRN constipation

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (age 62)
- Father: Myocardial infarction (age 58)
- Sister: Ovarian cancer

SOCIAL HISTORY:
She does not smoke or drink alcohol.

VITAL SIGNS:
Her blood pressure today is 134/78 mmHg, and BMI is 31 kg/m².

PHYSICAL EXAMINATION:
Physical examination reveals crepitus and decreased range of motion in both knees without effusion or warmth.

IMAGING:
X-rays show joint space narrowing and osteophyte formation in both knees.

PLAN:
She is started on pharmacologic therapy for symptom relief.

## Encounter 1: source 2020-07-15, practice 2021-05-10, outpatient, Internal Medicine

Chief complaint: Progressively worsening urinary urgency and involuntary leakage over the past 6 months

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Progressively worsening urinary urgency and involuntary leakage over the past 6 months

HISTORY OF PRESENT ILLNESS:
This 68-year-old female with hypertension, coronary artery disease, and osteoarthritis returns six months after her prior evaluation for progressive bilateral knee pain, now presenting with urinary complaints. She reports sudden urges to urinate that are difficult to suppress, followed by involuntary leakage before she can reach the bathroom. These episodes now occur multiple times daily, and she has begun limiting her fluid intake and avoiding social events out of fear of accidents. She denies dysuria, hematuria, or stress-induced leakage with coughing or sneezing.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Hypertension
- Coronary artery disease
- Osteoarthritis (bilateral hips and knees)
- Gastroesophageal reflux disease
- Obesity
- Hypothyroidism
- Chronic constipation with laxative dependence
- Postmenopausal status (menopause at age 51, no hormone replacement therapy)

Her medical history includes hypertension and osteoarthritis. She underwent menopause at age 52 and has never used hormone replacement therapy.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
She takes for which she takes lisinopril and acetaminophen.

Home Medications:
- Aspirin 81 mg daily
- Levothyroxine 75 mcg daily
- Omeprazole 20 mg daily
- Polyethylene glycol 3350 17 g daily PRN constipation

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (age 62)
- Father: Myocardial infarction (age 58)
- Sister: Ovarian cancer

SOCIAL HISTORY:
She does not smoke

VITAL SIGNS:
On examination, her vitals are normal

PHYSICAL EXAMINATION:
Abdominal exam shows no distention or tenderness. Pelvic exam reveals normal external genitalia, well-estrogenized vaginal mucosa, and no prolapse. Neurologic exam is normal, with intact reflexes and no sensory deficits.

LABORATORY DATA:
Urinalysis today is unremarkable

OTHER STUDIES:
Her last pelvic exam and urinalysis six months ago were normal
Post-void residual volume by bladder scan is 20 mL

PLAN:
The patient expresses strong interest in a medication to improve her symptoms and quality of life

## Encounter 2: source 2021-01-15, practice 2021-11-16, outpatient, Obstetrics & Gynecology

Chief complaint: Progressive irritation, persistent itching, and skin changes of external genitalia over 7 months

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Jennifer Walsh, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive irritation, persistent itching, and skin changes of external genitalia over 7 months

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old postmenopausal female with a history of urge incontinence and bilateral osteoarthritis who presents with progressive irritation and changes in the skin of her external genitalia over the past 7 months. She reports persistent itching that often disturbs her sleep and occasional stinging during urination, but denies vaginal dryness, discharge, or bleeding. She has tried various lubricants and estrogen creams prescribed by a different provider, with little to no relief. She has also noticed that the area around her vaginal opening has changed in appearance, with loss of definition and increasing sensitivity.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Urge incontinence (Overactive bladder) (diagnosed 2020-07-15)
- Hypertension
- Coronary artery disease
- Osteoarthritis (bilateral hips and knees)
- Gastroesophageal reflux disease
- Obesity
- Hypothyroidism
- Chronic constipation with laxative dependence
- Postmenopausal status (menopause at age 51, no hormone replacement therapy)

Her past medical history includes hypothyroidism and mild hypertension.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Aspirin 81 mg daily
- Levothyroxine 75 mcg daily
- Omeprazole 20 mg daily
- Acetaminophen 500 mg every 6 hours PRN pain
- Polyethylene glycol 3350 17 g daily PRN constipation

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (age 62)
- Father: Myocardial infarction (age 58)
- Sister: Ovarian cancer

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks monthly). Occupation: Retired travel journalist; former volunteer teacher.

PHYSICAL EXAMINATION:
On exam, the vulvar tissue appears pale and thin with areas of wrinkling and a cigarette paper–like texture. The labia minora are almost completely fused, and the introitus is narrowed. No active ulceration, erythema, or vaginal mucosal lesions are seen. Speculum examination is deferred due to discomfort.

## Encounter 3: source 2022-01-15, practice 2022-11-28, outpatient, Gastroenterology

Chief complaint: Progressive shortness of breath and postprandial chest pressure with early satiety over the past 2 months

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Ahmed Hassan, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath and postprandial chest pressure with early satiety over the past 2 months

HISTORY OF PRESENT ILLNESS:
This 70-year-old female with chronic GERD on PPI therapy and osteoarthritis presents with complaints. She describes increasing discomfort and early satiety after small meals for the past 2 months. She denies dysphagia, odynophagia, or reflux symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Urge incontinence (Overactive bladder) (diagnosed 2020-07-15)
- Lichen sclerosus (diagnosed 2021-01-15)
- Hypertension
- Coronary artery disease
- Osteoarthritis (bilateral hips and knees)
- Gastroesophageal reflux disease
- Obesity
- Hypothyroidism
- Chronic constipation with laxative dependence
- Postmenopausal status (menopause at age 51, no hormone replacement therapy)

Medical history is notable for hypertension and obesity.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Aspirin 81 mg daily
- Levothyroxine 75 mcg daily
- Omeprazole 20 mg daily
- Acetaminophen 500 mg every 6 hours PRN pain
- Polyethylene glycol 3350 17 g daily PRN constipation

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (age 62)
- Father: Myocardial infarction (age 58)
- Sister: Ovarian cancer

SOCIAL HISTORY:
She does not smoke or drink alcohol.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

IMAGING:
A chest x-ray reveals an air-fluid level in the posterior mediastinum. Barium swallow shows herniation of the gastric fundus above the diaphragm alongside the esophagus, with the gastroesophageal junction remaining in its normal location.

ASSESSMENT AND PLAN:
The patient is diagnosed with a paraesophageal hiatal hernia.

## Encounter 4: source 2022-07-15, practice 2023-05-11, outpatient, Obstetrics & Gynecology

Chief complaint: New onset vaginal bleeding starting 5 days ago, 20 years after menopause

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Jennifer Walsh, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
New onset vaginal bleeding starting 5 days ago, 20 years after menopause

HISTORY OF PRESENT ILLNESS:
The patient is a 70-year-old postmenopausal female returning to the Obstetrics & Gynecology department with a history of prior lichen sclerosus and recent evaluation for early satiety, now presenting with new vaginal bleeding. She describes the bleeding as light but persistent, requiring the use of a pad daily. She denies abdominal pain, urinary symptoms, or trauma. Menopause occurred at age 51, and she has had no vaginal bleeding since then until this episode. She has never taken hormone replacement therapy. She reports increasing fatigue over the past few months and has noticed her abdomen feels slightly fuller, but attributed it to aging.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Urge incontinence (Overactive bladder) (diagnosed 2020-07-15)
- Lichen sclerosus (diagnosed 2021-01-15)
- Paraesophageal hiatal hernia (diagnosed 2022-01-15)
- Hypertension
- Coronary artery disease
- Osteoarthritis (bilateral hips and knees)
- Gastroesophageal reflux disease
- Obesity
- Hypothyroidism
- Chronic constipation with laxative dependence
- Postmenopausal status (menopause at age 51, no hormone replacement therapy)

Her past medical history includes hypertension and osteoarthritis.

PAST SURGICAL HISTORY:
She has had three vaginal deliveries and no history of gynecologic surgery.

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Aspirin 81 mg daily
- Levothyroxine 75 mcg daily
- Omeprazole 20 mg daily
- Acetaminophen 500 mg every 6 hours PRN pain
- Polyethylene glycol 3350 17 g daily PRN constipation

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (age 62)
- Father: Myocardial infarction (age 58)
- Sister: Ovarian cancer

SOCIAL HISTORY:
She is a non-smoker and drinks alcohol occasionally.

REVIEW OF SYSTEMS:
On review of systems, she denies weight loss, appetite changes, or gastrointestinal symptoms.

VITAL SIGNS:
On physical examination, vital signs are stable.

PHYSICAL EXAMINATION:
Pelvic examination reveals a mobile, firm 9-cm left adnexal mass without tenderness. No cervical motion tenderness or vaginal discharge is noted.

LABORATORY DATA:
Laboratory evaluation reveals elevated serum inhibin B and normal CA-125.

IMAGING:
Transvaginal ultrasound shows a thickened endometrial stripe measuring 9 mm and a complex solid-cystic mass in the left adnexa.

PATHOLOGY:
An endometrial biopsy shows complex hyperplasia with atypia.
The patient’s last Pap smear, done 1 year ago, was normal.

## Encounter 5: source 2023-07-15, practice 2024-05-14, ed, Emergency Medicine

Chief complaint: Progressive fatigue, decreased appetite, and lower extremity edema over the past 10 days with decreased urine output

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive fatigue, decreased appetite, and lower extremity edema over the past 10 days with decreased urine output

HISTORY OF PRESENT ILLNESS:
The patient with a history of bilateral knee osteoarthritis and ovarian cancer presents to the emergency department. She reports urinating less than usual and denies dysuria, fever, hematuria, or recent illness. She states that her joint pain has worsened recently.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Urge incontinence (Overactive bladder) (diagnosed 2020-07-15)
- Lichen sclerosus (diagnosed 2021-01-15)
- Paraesophageal hiatal hernia (diagnosed 2022-01-15)
- Serous cystadenocarcinoma of ovary (diagnosed 2022-07-15)
- Hypertension
- Coronary artery disease
- Osteoarthritis (bilateral hips and knees)
- Gastroesophageal reflux disease
- Obesity
- Hypothyroidism
- Chronic constipation with laxative dependence
- Postmenopausal status (menopause at age 51, no hormone replacement therapy)

Her medical history includes osteoarthritis of the hips and knees, hypertension, and coronary artery disease.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
She takes she has been taking an over-the-counter medication for pain relief, though she is unsure of the name.

Home Medications:
- Lisinopril 10 mg daily
- Aspirin 81 mg daily
- Levothyroxine 75 mcg daily
- Omeprazole 20 mg daily
- Acetaminophen 500 mg every 6 hours PRN pain
- Polyethylene glycol 3350 17 g daily PRN constipation

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (age 62)
- Father: Myocardial infarction (age 58)
- Sister: Ovarian cancer

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks monthly). Occupation: Retired travel journalist; former volunteer teacher.

VITAL SIGNS:
On physical examination, temperature is 36.9°C (98.4°F), blood pressure is 118/72 mm Hg, and pulse is 64/min.

PHYSICAL EXAMINATION:
Jugular venous distension is absent. Lungs are clear to auscultation. Cardiac exam reveals a regular rate and rhythm with no murmurs. There is 2+ pitting edema of the ankles bilaterally. The rest of the exam is unremarkable.

LABORATORY DATA:
Laboratory results show:
Creatinine 2.0 mg/dL
BUN 46 mg/dL
Potassium 5.3 mEq/L

## Encounter 6: source 2024-01-15, practice 2024-11-19, ed, Emergency Medicine

Chief complaint: Sudden-onset severe shortness of breath starting a few hours ago

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. James Mitchell, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset severe shortness of breath starting a few hours ago

HISTORY OF PRESENT ILLNESS:
The patient is a 72-year-old woman with hypertension, coronary artery disease, and prior acute kidney injury. She comes to the emergency department with sudden-onset shortness of breath that started a few hours ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Urge incontinence (Overactive bladder) (diagnosed 2020-07-15)
- Lichen sclerosus (diagnosed 2021-01-15)
- Paraesophageal hiatal hernia (diagnosed 2022-01-15)
- Serous cystadenocarcinoma of ovary (diagnosed 2022-07-15)
- Acute kidney injury (diagnosed 2023-07-15)
- Hypertension
- Coronary artery disease
- Osteoarthritis (bilateral hips and knees)
- Gastroesophageal reflux disease
- Obesity
- Hypothyroidism
- Chronic constipation with laxative dependence
- Postmenopausal status (menopause at age 51, no hormone replacement therapy)

She has a history of hypertension and coronary artery disease.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Aspirin 81 mg daily
- Levothyroxine 75 mcg daily
- Omeprazole 20 mg daily
- Acetaminophen 500 mg every 6 hours PRN pain
- Polyethylene glycol 3350 17 g daily PRN constipation

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (age 62)
- Father: Myocardial infarction (age 58)
- Sister: Ovarian cancer

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks monthly). Occupation: Retired travel journalist; former volunteer teacher.

VITAL SIGNS:
Her oxygen saturation is 85% on room air, blood pressure is 160/90 mm Hg, and heart rate is 110/min.

PHYSICAL EXAMINATION:
On examination, she is tachypneic and using accessory muscles to breathe. Auscultation reveals bibasilar crackles and an S3 gallop.

LABORATORY DATA:
Arterial blood gas reveals hypoxemia.

IMAGING:
Chest x-ray shows prominent pulmonary vasculature and bilateral alveolar infiltrates consistent with pulmonary edema.

## Encounter 7: source 2025-01-15, practice 2025-11-25, icu, Emergency Medicine

Chief complaint: Severe crushing chest pain and diaphoresis consistent with acute myocardial infarction

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Severe crushing chest pain and diaphoresis consistent with acute myocardial infarction

HISTORY OF PRESENT ILLNESS:
This 73-year-old with constipation and hiatal hernia presents to ICU. Her family reports she often complained of left lower quadrant abdominal discomfort. Constipation worsened with age.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Urge incontinence (Overactive bladder) (diagnosed 2020-07-15)
- Lichen sclerosus (diagnosed 2021-01-15)
- Paraesophageal hiatal hernia (diagnosed 2022-01-15)
- Serous cystadenocarcinoma of ovary (diagnosed 2022-07-15)
- Acute kidney injury (diagnosed 2023-07-15)
- Flash pulmonary edema (diagnosed 2024-01-15)
- Hypertension
- Coronary artery disease
- Osteoarthritis (bilateral hips and knees)
- Gastroesophageal reflux disease
- Obesity
- Hypothyroidism
- Chronic constipation with laxative dependence
- Postmenopausal status (menopause at age 51, no hormone replacement therapy)

The patient’s past medical history is significant for long-standing constipation, for which she frequently used laxatives. She also had a 25-year history of hypertension. She had been hospitalized once for an episode of diverticulitis five years prior. She was treated with antibiotics at that time, with full recovery.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Aspirin 81 mg daily
- Levothyroxine 75 mcg daily
- Omeprazole 20 mg daily
- Acetaminophen 500 mg every 6 hours PRN pain
- Polyethylene glycol 3350 17 g daily PRN constipation

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (age 62)
- Father: Myocardial infarction (age 58)
- Sister: Ovarian cancer

SOCIAL HISTORY:
Her social history is notable for frequent international travel. As a young adult, she spent two years working in rural India as a volunteer teacher. Later in life, she worked as a travel journalist, visiting multiple countries across Africa, South America, and Europe. She wrote extensively about public health conditions in different parts of the world. Her occupations included teaching, writing, and later serving as a tour guide. Despite these varied experiences, she had no history of parasitic infection or tropical illness. Her dietary history included frequent consumption of refined, low-fiber foods while traveling. She often joked with her family that her “Western diet” was easier to maintain abroad than local diets. Her physicians had repeatedly encouraged her to increase her dietary fiber, but she often ignored this advice. She worked at a desk for long stretches when writing, which contributed to her sedentary lifestyle.

PATHOLOGY:
An autopsy is performed to evaluate the extent of her coronary artery disease and to document other comorbidities. Her heart is found to have severe atherosclerotic narrowing of all three major coronary arteries. The left anterior descending artery has a 95% occlusion with evidence of thrombus formation. The posterior descending artery has moderate narrowing. The myocardium shows a large area of transmural infarction. In addition to the cardiac findings, several other organs are examined. The lungs show mild emphysematous changes, likely related to her history of tobacco use. Her liver is mildly steatotic, which her physicians had attributed to long-standing obesity. The kidneys show arterionephrosclerosis. The uterus and ovaries are atrophic, consistent with her postmenopausal age. When the pathologist examines the large intestine, an additional abnormality is noted. The sigmoid colon appears irregular and somewhat thickened. Careful inspection reveals multiple small, round outpouchings ranging from 0.5 to 1 cm. These outpouchings are located along the antimesenteric border between the teniae coli. They are filled with inspissated stool and are surrounded by hypertrophied smooth muscle bands. On closer examination, it is clear that these lesions represent herniations of mucosa and submucosa through the muscular layer. Microscopy of one of these lesions shows attenuated colonic mucosa, with no evidence of epithelial dysplasia or malignant transformation. The muscularis propria is intact but interrupted at the site of the pouch, where the mucosa protrudes. At autopsy, these sigmoid lesions are carefully documented. The pathologist notes that they are not true diverticula because they do not involve all layers of the bowel wall. Instead, they consist only of mucosa and submucosa herniating through areas of weakness in the muscularis propria. No evidence of carcinoma or polypoid lesions is found in the colon.
