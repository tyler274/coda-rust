# synthetic_hospital patient 1776

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 30 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-02-03 (Mon) | outpatient | Gastroenterology |
| 1 | 2020-07-15 | 2020-07-27 (Mon) | outpatient | Internal Medicine |
| 2 | 2021-01-15 | 2021-01-22 (Fri) | outpatient | Hematology & Oncology |
| 3 | 2021-07-15 | 2021-07-26 (Mon) | outpatient | Hematology & Oncology |
| 4 | 2022-01-15 | 2022-01-17 (Mon) | telehealth | Neurology |
| 5 | 2024-01-15 | 2024-01-15 (Mon) | outpatient | Obstetrics & Gynecology |
| 6 | 2024-07-15 | 2024-07-15 (Mon) | outpatient | Obstetrics & Gynecology |
| 7 | 2026-01-15 | 2026-01-18 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "None",
 "family_history": [
  "Mother with uterine fibroids",
  "Sister with celiac disease",
  "Maternal grandmother with osteoporosis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "325 mg oral three times daily",
   "name": "Ferrous sulfate"
  },
  {
   "dose": "One tablet oral daily",
   "name": "Multivitamin"
  },
  {
   "dose": "600 mg/400 units oral twice daily",
   "name": "Calcium carbonate with vitamin D3"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Iron deficiency anemia",
  "Uterine leiomyoma",
  "Celiac disease",
  "Restless legs syndrome",
  "Osteopenia",
  "Menorrhagia"
 ],
 "age_at_first_encounter": 30
}
```

Primary diagnoses: ["Iron Deficiency Anemia", "Uterine leiomyoma", "Restless Legs Syndrome", "Traumatic hemorrhagic shock with acute coagulopathy", "Celiac disease"]
Comorbidities: ["Menorrhagia", "Iron Deficiency Anemia", "Peripheral neuropathy"]

## Encounter 0: source 2020-01-15, practice 2020-02-03, outpatient, Gastroenterology

Chief complaint: 6-month history of chronic diarrhea, bloating, and weight loss with painful glossitis

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of chronic diarrhea, bloating, and weight loss with painful glossitis

HISTORY OF PRESENT ILLNESS:
She reports that her symptoms seem to worsen after eating bread and pasta. She also complains of fatigue and has noticed a painful, red, smooth patch on her tongue.

PAST MEDICAL HISTORY:
Active Problem List:
- Iron deficiency anemia
- Uterine leiomyoma
- Celiac disease
- Restless legs syndrome
- Osteopenia
- Menorrhagia

Her past medical history is significant for iron-deficiency anemia and a recent diagnosis of osteopenia.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Ferrous sulfate 325 mg oral three times daily
- Multivitamin One tablet oral daily
- Calcium carbonate with vitamin D3 600 mg/400 units oral twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with uterine fibroids
- Sister with celiac disease
- Maternal grandmother with osteoporosis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, she appears pale and has a body mass index of 18 kg/m^2. Neurological examination reveals decreased sensation in a stocking-glove distribution.

LABORATORY DATA:
Hemoglobin: 10 g/dL (Normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 74 fL (Normal: 80-100 fL)
- Serum iron: 30 µg/dL (Normal: 50-170 µg/dL)
- Total iron-binding capacity (TIBC): 400 µg/dL (Normal: 250-450 µg/dL)
- Ferritin: 15 ng/mL (Normal: 15-200 ng/mL)
- Calcium: 8.2 mg/dL (Normal: 8.5-10.2 mg/dL)
- Alkaline phosphatase: 120 U/L (Normal: 30-120 U/L)
- Tissue transglutaminase IgA: Pending

## Encounter 1: source 2020-07-15, practice 2020-07-27, outpatient, Internal Medicine

Chief complaint: 6-month history of fatigue and occasional lightheadedness when standing quickly

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Miller, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of fatigue and occasional lightheadedness when standing quickly

HISTORY OF PRESENT ILLNESS:
The patient returns for evaluation of a six-month history of fatigue and occasional lightheadedness when standing quickly, presenting six months following her diagnosis of Celiac disease. She works as a kindergarten teacher and finds herself unusually tired after short walks or climbing stairs, which was not the case a year ago. Her appetite is normal, and she denies recent illness, weight loss, or changes in bowel habits. She does not follow a vegetarian or vegan diet and reports regular intake of mixed meals. Her menstrual cycles occur every 28 days and last 5 days, with moderate flow and occasional cramps. She denies any history of gastrointestinal bleeding, melena, hematochezia, or hematuria.

PAST MEDICAL HISTORY:
Active Problem List:
- Celiac disease (diagnosed 2020-01-15)
- Iron deficiency anemia
- Uterine leiomyoma
- Restless legs syndrome
- Osteopenia
- Menorrhagia

She has never been pregnant.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She does not take any medications or supplements.

Home Medications:
- Ferrous sulfate 325 mg oral three times daily
- Multivitamin One tablet oral daily
- Calcium carbonate with vitamin D3 600 mg/400 units oral twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with uterine fibroids
- Sister with celiac disease
- Maternal grandmother with osteoporosis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

REVIEW OF SYSTEMS:
Review of systems is otherwise unremarkable, including normal cognition, vision, and mood.

PHYSICAL EXAMINATION:
On physical examination, she appears well but slightly pale. Conjunctival pallor is noted. Cardiopulmonary and abdominal exams are normal, and there is no lymphadenopathy or hepatosplenomegaly. Neurologic exam is intact. Her BMI is 23 kg/m².

LABORATORY DATA:
Her white blood cell and platelet counts are within normal limits.

## Encounter 2: source 2021-01-15, practice 2021-01-22, outpatient, Hematology & Oncology

Chief complaint: Disrupted sleep over the past several months due to irresistible urge to move legs

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Patel, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Disrupted sleep over the past several months due to irresistible urge to move legs

HISTORY OF PRESENT ILLNESS:
The patient is a 31-year-old female with a history of celiac disease diagnosed in January 2020 and iron deficiency anemia identified in July 2020 who returns for evaluation of several months of disrupted sleep. She explains that she frequently experiences a strong, uncomfortable urge to move her legs when she lies down at night. These sensations are most pronounced when she is resting quietly or watching television and are only relieved by movement. As a result, she often gets out of bed and walks around until the sensations subside. She describes the feeling as a "creepy-crawly" sensation deep inside her legs that is not painful but deeply irritating. The symptoms usually occur in the evenings and are absent during the day. Her menstrual periods are regular but heavy, lasting 6–7 days each cycle. She reports feeling generally tired during the day but attributes this to her poor sleep quality. She denies snoring, waking up gasping, or having witnessed apneic events.

PAST MEDICAL HISTORY:
Active Problem List:
- Celiac disease (diagnosed 2020-01-15)
- Iron Deficiency Anemia (diagnosed 2020-07-15)
- Uterine leiomyoma
- Restless legs syndrome
- Osteopenia
- Menorrhagia

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She takes only a daily multivitamin.

Home Medications:
- Ferrous sulfate 325 mg oral three times daily
- Calcium carbonate with vitamin D3 600 mg/400 units oral twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with uterine fibroids
- Sister with celiac disease
- Maternal grandmother with osteoporosis

SOCIAL HISTORY:
She works as an elementary school teacher, exercises regularly, and follows a vegetarian diet. does not drink alcohol, and does not use recreational drugs.

PHYSICAL EXAMINATION:
Physical examination is unremarkable with no neurologic deficits or motor abnormalities. There is no evidence of peripheral edema or vascular insufficiency in the lower extremities. Her BMI is 22.

OTHER STUDIES:
A sleep study has not been performed.

## Encounter 3: source 2021-07-15, practice 2021-07-26, outpatient, Hematology & Oncology

Chief complaint: 6-month history of fatigue, intermittent diarrhea, and recent numbness and tingling in hands and feet

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James Miller, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of fatigue, intermittent diarrhea, and recent numbness and tingling in hands and feet

HISTORY OF PRESENT ILLNESS:
The patient with a history of celiac disease, iron deficiency anemia, and restless legs syndrome presents for follow-up. She reports a 10-pound weight loss over the last three months despite no changes in her diet or exercise routine.

PAST MEDICAL HISTORY:
Active Problem List:
- Celiac disease (diagnosed 2020-01-15)
- Iron Deficiency Anemia (diagnosed 2020-07-15)
- Restless Legs Syndrome (diagnosed 2021-01-15)
- Uterine leiomyoma
- Osteopenia
- Menorrhagia

Her past medical history is significant for iron deficiency anemia refractory to oral iron supplements.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Ferrous sulfate 325 mg oral three times daily
- Multivitamin One tablet oral daily
- Calcium carbonate with vitamin D3 600 mg/400 units oral twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with uterine fibroids
- Sister with celiac disease
- Maternal grandmother with osteoporosis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
Physical examination reveals pallor and mild abdominal distention without tenderness. Neurological examination shows decreased sensation to light touch in a stocking-glove distribution.

LABORATORY DATA:
Hemoglobin: 9.8 g/dL (Normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 74 fL (Normal: 80-100 fL)
- Serum iron: 30 µg/dL (Normal: 50-170 µg/dL)
- Total iron-binding capacity (TIBC): 450 µg/dL (Normal: 250-450 µg/dL)
- Serum calcium: 8.0 mg/dL (Normal: 8.5-10.2 mg/dL)
- Alkaline phosphatase: 120 U/L (Normal: 30-120 U/L)
- Parathyroid hormone (PTH): Elevated

## Encounter 4: source 2022-01-15, practice 2022-01-17, telehealth, Neurology

Chief complaint: Chronic sleep disturbance due to persistent restless legs symptoms

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Robert Kim, MD
Department: Neurology
Visit Type: telehealth

CHIEF COMPLAINT:
Chronic sleep disturbance due to persistent restless legs symptoms

HISTORY OF PRESENT ILLNESS:
Patient is a 32-year-old female with a history of celiac disease and iron deficiency anemia who returns for follow-up regarding persistent leg discomfort and sleep disturbance initially evaluated in January 2021. For the past several months, she has experienced an irresistible urge to move her legs, primarily in the evenings. She describes a deep, uncomfortable crawling sensation in her legs that is relieved only by movement. As a result, she frequently gets out of bed and walks around for relief, which delays her sleep and leaves her feeling tired the next day. Her symptoms are absent during the day and are most pronounced when she is lying down at night. She reports no snoring, gasping during sleep, or witnessed apneas. She returns after two weeks with partial relief of symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Celiac disease (diagnosed 2020-01-15)
- Iron Deficiency Anemia (diagnosed 2020-07-15)
- Restless Legs Syndrome (diagnosed 2021-01-15)
- Uterine leiomyoma
- Osteopenia
- Menorrhagia

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She does not take any medications except a multivitamin.

Home Medications:
- Ferrous sulfate 325 mg oral three times daily
- Calcium carbonate with vitamin D3 600 mg/400 units oral twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Her family history is noncontributory.

SOCIAL HISTORY:
She is a vegetarian and reports heavy menstrual periods lasting 6 to 7 days.

PHYSICAL EXAMINATION:
Neurologic examination is normal.

ASSESSMENT AND PLAN:
Her physician diagnoses her condition

Prescribes gabapentin. In addition to gabapentin,

## Encounter 5: source 2024-01-15, practice 2024-01-15, outpatient, Obstetrics & Gynecology

Chief complaint: Heavy menstrual bleeding and pelvic pressure with periods lasting 8-10 days

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Maria Gonzalez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Heavy menstrual bleeding and pelvic pressure with periods lasting 8-10 days

HISTORY OF PRESENT ILLNESS:
The patient presents to Obstetrics and Gynecology with progressive menstrual symptoms in the context of iron deficiency anemia diagnosed by Internal Medicine in 2020 and celiac disease managed by Gastroenterology. She reports a 6-month history of heavy menstrual bleeding and pelvic pressure. She reports that her menstrual periods have become increasingly heavy and prolonged, lasting 8-10 days, with the need to change her sanitary pads every 2 hours. She also experiences frequent urination and constipation, which she attributes to pressure symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Celiac disease (diagnosed 2020-01-15)
- Iron Deficiency Anemia (diagnosed 2020-07-15)
- Restless Legs Syndrome (diagnosed 2021-01-15)
- Uterine leiomyoma
- Osteopenia
- Menorrhagia

Her past medical history is significant for iron-deficiency anemia treated with oral iron supplements.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Is.

Home Medications:
- Ferrous sulfate 325 mg oral three times daily
- Multivitamin One tablet oral daily
- Calcium carbonate with vitamin D3 600 mg/400 units oral twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with uterine fibroids
- Sister with celiac disease
- Maternal grandmother with osteoporosis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, her abdomen is non-tender, and a firm, irregular mass is palpated in the lower abdomen consistent with an enlarged uterus.

LABORATORY DATA:
Laboratory Values:
Hemoglobin: 10.2 g/dL (Normal: 12-16 g/dL)
Serum Ferritin: 15 ng/mL (Normal: 15-150 ng/mL)
Platelets: 250,000/mm³ (Normal: 150,000-400,000/mm³)
PT: 12 seconds (Normal: 9.5-13.5 seconds)
aPTT: 30 seconds (Normal: 25-35 seconds)

IMAGING:
Pelvic ultrasound reveals multiple well-circumscribed, hypoechoic masses within the myometrium, the largest measuring 5 cm in diameter.

## Encounter 6: source 2024-07-15, practice 2024-07-15, outpatient, Obstetrics & Gynecology

Chief complaint: 6-month history of pelvic pressure and sensation of incomplete voiding

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Maria Gonzalez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of pelvic pressure and sensation of incomplete voiding

HISTORY OF PRESENT ILLNESS:
This patient with uterine leiomyoma, iron deficiency anemia, and celiac disease returns for follow-up. She reports that her menstrual cycles are regular, but she has noticed an increase in menstrual flow and duration over the past year. She denies any urinary incontinence or dysuria.

PAST MEDICAL HISTORY:
Active Problem List:
- Celiac disease (diagnosed 2020-01-15)
- Iron Deficiency Anemia (diagnosed 2020-07-15)
- Restless Legs Syndrome (diagnosed 2021-01-15)
- Uterine leiomyoma (diagnosed 2024-01-15)
- Osteopenia
- Menorrhagia

Her past medical history is significant for iron-deficiency anemia,.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She takes for which she takes oral iron supplements.

Home Medications:
- Ferrous sulfate 325 mg oral three times daily
- Multivitamin One tablet oral daily
- Calcium carbonate with vitamin D3 600 mg/400 units oral twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with uterine fibroids
- Sister with celiac disease
- Maternal grandmother with osteoporosis

SOCIAL HISTORY:
She has never been pregnant and is not currently sexually active.

PHYSICAL EXAMINATION:
On physical examination, her abdomen is non-tender, but there is a palpable mass arising from the pelvis that extends to the umbilicus.

LABORATORY DATA:
Her laboratory values are as follows:

- Hemoglobin: 10.2 g/dL (normal: 12-16 g/dL)
- Hematocrit: 30.5% (normal: 37-47%)
- Mean corpuscular volume (MCV): 78 fL (normal: 80-100 fL)
- Serum ferritin: 15 ng/mL (normal: 15-150 ng/mL)

IMAGING:
A pelvic ultrasound reveals a 10 cm subserosal fibroid on the posterior wall of the uterus and a 5 cm pedunculated fibroid on the anterior wall. The endometrial stripe is normal.

## Encounter 7: source 2026-01-15, practice 2026-01-18, ed, Emergency Medicine

Chief complaint: Abdominal pain and dizziness after high-speed motor vehicle collision

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. David Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Abdominal pain and dizziness after high-speed motor vehicle collision

HISTORY OF PRESENT ILLNESS:
This 36-year-old female with iron deficiency anemia, uterine leiomyoma, and celiac disease is brought to the emergency department by ambulance after a high-speed motor vehicle collision. She is alert but appears pale and anxious. She complains of abdominal pain and dizziness.

PAST MEDICAL HISTORY:
Active Problem List:
- Celiac disease (diagnosed 2020-01-15)
- Iron Deficiency Anemia (diagnosed 2020-07-15)
- Restless Legs Syndrome (diagnosed 2021-01-15)
- Uterine leiomyoma (diagnosed 2024-01-15)
- Osteopenia
- Menorrhagia

Her medical history is significant for menorrhagia.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Ferrous sulfate 325 mg oral three times daily
- Multivitamin One tablet oral daily
- Calcium carbonate with vitamin D3 600 mg/400 units oral twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with uterine fibroids
- Sister with celiac disease
- Maternal grandmother with osteoporosis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

VITAL SIGNS:
On examination, her blood pressure is 90/60 mmHg, heart rate is 120 beats per minute, and respiratory rate is 22 breaths per minute.

PHYSICAL EXAMINATION:
Her abdomen is distended and tender to palpation in all quadrants with guarding.

LABORATORY DATA:
Laboratory tests are as follows:

- Hemoglobin: 7 g/dL (Normal: 12-16 g/dL)
- Hematocrit: 21% (Normal: 37-47%)
- Platelets: 250,000/mm³ (Normal: 150,000-400,000/mm³)
- Prothrombin time (PT): 12 seconds (Normal: 11-13.5 seconds)
- Partial thromboplastin time (PTT): 30 seconds (Normal: 25-35 seconds)
- Blood type: Pending

IMAGING:
Focused Assessment with Sonography for Trauma (FAST) exam reveals free fluid in the abdomen.

ASSESSMENT AND PLAN:
Given the patient&#x27;s condition and pending blood type,
