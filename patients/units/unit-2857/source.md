# synthetic_hospital patient 2857

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 72 at first encounter, sex F, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2018-03-09 (Fri) | outpatient | Internal Medicine |
| 1 | 2020-02-15 | 2018-04-23 (Mon) | ed | Emergency Medicine |
| 2 | 2021-03-15 | 2019-05-10 (Fri) | ed | Emergency Medicine |
| 3 | 2022-05-15 | 2020-07-24 (Fri) | ed | Emergency Medicine |
| 4 | 2023-07-15 | 2021-09-07 (Tue) | inpatient | Orthopedic Surgery |
| 5 | 2023-09-15 | 2021-11-19 (Fri) | inpatient | Internal Medicine |
| 6 | 2024-01-15 | 2022-03-21 (Mon) | ed | Emergency Medicine |
| 7 | 2028-01-15 | 2026-03-12 (Thu) | outpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired elementary school teacher",
 "alcohol_use": "Minimal (one glass of wine weekly)",
 "family_history": [
  "Mother: Hip fracture due to osteoporosis",
  "Father: Chronic obstructive pulmonary disease",
  "Maternal grandmother: Ovarian cancer"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker (quit 15 years ago)",
 "home_medications": [
  {
   "dose": "One tablet daily",
   "name": "Multivitamin"
  }
 ],
 "surgical_history": [
  "Spontaneous vaginal delivery x3"
 ],
 "chronic_conditions": [
  "Osteoporosis",
  "Chronic obstructive pulmonary disease (COPD)",
  "Postmenopausal status"
 ],
 "age_at_first_encounter": 72
}
```

Primary diagnoses: ["Osteoporosis", "Acute osteoporotic vertebral compression fracture of T12", "Displaced fracture of left femoral neck", "Severe Clostridioides difficile infection without toxic megacolon", "Serous cystadenocarcinoma of ovary", "Pill-induced esophagitis"]
Comorbidities: ["Chronic obstructive pulmonary disease", "Postmenopausal status", "History of recent pneumonia", "Osteoporosis"]

## Encounter 0: source 2020-01-15, practice 2018-03-09, outpatient, Internal Medicine

Chief complaint: Follow-up visit after completing routine bone health evaluation with reports of height loss of 3 cm over several years and occasional mild back pain with prolonged standing or walking.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit after completing routine bone health evaluation with reports of height loss of 3 cm over several years and occasional mild back pain with prolonged standing or walking.

HISTORY OF PRESENT ILLNESS:
She is otherwise in good health but notes that she has shrunk about 3 cm in height over the past few years. She also reports occasional mild back pain that comes and goes, especially after standing or walking for long periods. She has no history of falls or fractures.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis
- Chronic obstructive pulmonary disease (COPD)
- Postmenopausal status

She went through menopause at age 51 and did not use hormone replacement therapy.

PAST SURGICAL HISTORY:
- Spontaneous vaginal delivery x3

MEDICATIONS:
Her only medication is a daily multivitamin.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hip fracture due to osteoporosis
- Father: Chronic obstructive pulmonary disease
- Maternal grandmother: Ovarian cancer

SOCIAL HISTORY:
She has a history of smoking but quit 15 years ago. She consumes one glass of wine per week and does not exercise regularly. She eats a balanced diet but does not take any calcium or vitamin D supplements.

PHYSICAL EXAMINATION:
Her physical exam reveals kyphosis and mild tenderness to percussion over the lower thoracic spine.

LABORATORY DATA:
Laboratory studies are within normal limits.

IMAGING:
Bone densitometry reveals decreased mineral density at both the lumbar spine and femoral neck.

## Encounter 1: source 2020-02-15, practice 2018-04-23, ed, Emergency Medicine

Chief complaint: Sudden-onset odynophagia and retrosternal chest pain exacerbated by swallowing solids over the past 2 days.

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset odynophagia and retrosternal chest pain exacerbated by swallowing solids over the past 2 days.

HISTORY OF PRESENT ILLNESS:
This 72-year-old female with osteoporosis presents to the emergency department one month following her initial bone health evaluation. She reports that the pain is exacerbated when she swallows, especially with solid foods. She denies any recent travel, sick contacts, or changes in her diet.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (COPD)
- Postmenopausal status

Her medical history is significant for osteoporosis, for which she takes alendronate once weekly, and chronic knee osteoarthritis managed with ibuprofen as needed.

PAST SURGICAL HISTORY:
- Spontaneous vaginal delivery x3

MEDICATIONS:
Home Medications:
- Multivitamin One tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hip fracture due to osteoporosis
- Father: Chronic obstructive pulmonary disease
- Maternal grandmother: Ovarian cancer

SOCIAL HISTORY:
She does not smoke or consume alcohol.

VITAL SIGNS:
On physical examination, she is afebrile, and her vital signs are within normal limits.

PHYSICAL EXAMINATION:
Her oropharyngeal examination is unremarkable, and there is no cervical lymphadenopathy.

LABORATORY DATA:
Laboratory tests reveal normal complete blood count and basic metabolic panel.

OTHER STUDIES:
An upper endoscopy is performed, which shows a localized, well-demarcated ulcerative lesion in the mid-esophagus without evidence of bleeding.

## Encounter 2: source 2021-03-15, practice 2019-05-10, ed, Emergency Medicine

Chief complaint: Acute lower back pain for 2 days after lifting a heavy pot of water, described as sharp and constant without radiation.

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Jennifer Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute lower back pain for 2 days after lifting a heavy pot of water, described as sharp and constant without radiation.

HISTORY OF PRESENT ILLNESS:
The patient is a 73-year-old female with a known history of osteoporosis and prior reports of height loss and mechanical back pain who presents with a 2-day history of acute lower back pain after lifting a heavy pot of water. She describes the pain as sharp and constant, with no radiation. She denies any recent trauma, fever, bowel or bladder incontinence, or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis (diagnosed 2020-01-15)
- Pill-induced esophagitis (diagnosed 2020-02-15)
- Chronic obstructive pulmonary disease (COPD)
- Postmenopausal status

She has a history of osteoporosis.

PAST SURGICAL HISTORY:
- Spontaneous vaginal delivery x3

MEDICATIONS:
She takes calcium and vitamin D supplements.

Home Medications:
- Multivitamin One tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hip fracture due to osteoporosis
- Father: Chronic obstructive pulmonary disease
- Maternal grandmother: Ovarian cancer

SOCIAL HISTORY:
Smoking: Former smoker (quit 15 years ago). Alcohol: Minimal (one glass of wine weekly). Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, she has localized tenderness over the T12 vertebra but no ecchymosis or swelling. Neurological examination, including motor strength, reflexes, and sensation, is normal.

LABORATORY DATA:
Laboratory values:
- Calcium: 9.8 mg/dL (normal: 8.5-10.2 mg/dL)
- Phosphate: 3.5 mg/dL (normal: 2.5-4.5 mg/dL)
- Alkaline phosphatase: 70 U/L (normal: 40-130 U/L)
- 25-hydroxyvitamin D: 32 ng/mL (normal: 20-50 ng/mL)

IMAGING:
A lateral spine X-ray shows a wedge-shaped deformity of the T12 vertebral body.

## Encounter 3: source 2022-05-15, practice 2020-07-24, ed, Emergency Medicine

Chief complaint: Sudden onset of severe lower back pain while lifting a heavy box earlier today, localized to midline lower back without radiation.

ENCOUNTER NOTE
Date: 2022-05-15
Provider: Dr. James Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe lower back pain while lifting a heavy box earlier today, localized to midline lower back without radiation.

HISTORY OF PRESENT ILLNESS:
The patient is a 74-year-old female with osteoporosis and prior T12 compression fracture who presents with sudden onset of severe lower back pain that began while she was lifting a heavy box earlier in the day. She describes the pain as sharp and localized to the midline of her lower back, with no radiation. She denies any recent trauma, fever, bowel or bladder incontinence, or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis (diagnosed 2020-01-15)
- Pill-induced esophagitis (diagnosed 2020-02-15)
- Acute osteoporotic vertebral compression fracture of T12 (diagnosed 2021-03-15)
- Chronic obstructive pulmonary disease (COPD)
- Postmenopausal status

Her medical history is significant for osteoporosis.

PAST SURGICAL HISTORY:
- Spontaneous vaginal delivery x3

MEDICATIONS:
She takes calcium and vitamin D supplements.

Home Medications:
- Multivitamin One tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hip fracture due to osteoporosis
- Father: Chronic obstructive pulmonary disease
- Maternal grandmother: Ovarian cancer

SOCIAL HISTORY:
Smoking: Former smoker (quit 15 years ago). Alcohol: Minimal (one glass of wine weekly). Occupation: Retired elementary school teacher.

VITAL SIGNS:
Her vital signs are within normal limits

PHYSICAL EXAMINATION:
On physical examination, she has tenderness to palpation over the T12 vertebral body. There is no neurological deficit, and straight leg raise test is negative.

LABORATORY DATA:
Laboratory values:
- Calcium: 9.8 mg/dL (normal: 8.5-10.2 mg/dL)
- Phosphate: 3.5 mg/dL (normal: 2.5-4.5 mg/dL)
- Alkaline phosphatase: 70 U/L (normal: 40-130 U/L)
- Vitamin D: 30 ng/mL (normal: 20-50 ng/mL)

IMAGING:
An X-ray of the thoracic and lumbar spine reveals a compression fracture of the T12 vertebral body.

## Encounter 4: source 2023-07-15, practice 2021-09-07, inpatient, Orthopedic Surgery

Chief complaint: Severe left hip pain following a mechanical fall at home after tripping over a rug.

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Aisha Patel, MD
Department: Orthopedic Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Severe left hip pain following a mechanical fall at home after tripping over a rug.

HISTORY OF PRESENT ILLNESS:
The patient, with established osteoporosis and prior T12 compression fractures in 2021 and 2022, presents with severe left hip pain. She reports that she tripped over a rug and landed directly on her left side.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis (diagnosed 2020-01-15)
- Pill-induced esophagitis (diagnosed 2020-02-15)
- Acute osteoporotic vertebral compression fracture of T12 (diagnosed 2021-03-15)
- Chronic obstructive pulmonary disease (COPD)
- Postmenopausal status

She has a history of osteoporosis.

PAST SURGICAL HISTORY:
- Spontaneous vaginal delivery x3

MEDICATIONS:
She takes calcium and vitamin D supplements.

Home Medications:
- Multivitamin One tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hip fracture due to osteoporosis
- Father: Chronic obstructive pulmonary disease
- Maternal grandmother: Ovarian cancer

SOCIAL HISTORY:
Smoking: Former smoker (quit 15 years ago). Alcohol: Minimal (one glass of wine weekly). Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
On examination, her left leg appears shortened and is externally rotated. She is unable to bear weight on the left leg.

IMAGING:
An x-ray of the left hip shows a displaced fracture of the femoral neck with disruption of the normal cortical contour and irregular lucency at the fracture plane.

## Encounter 5: source 2023-09-15, practice 2021-11-19, inpatient, Internal Medicine

Chief complaint: Watery diarrhea, abdominal cramping, and low-grade fever for 3 days following recent hospitalization.

ENCOUNTER NOTE
Date: 2023-09-15
Provider: Dr. Robert Kim, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Watery diarrhea, abdominal cramping, and low-grade fever for 3 days following recent hospitalization.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis (diagnosed 2020-01-15)
- Pill-induced esophagitis (diagnosed 2020-02-15)
- Acute osteoporotic vertebral compression fracture of T12 (diagnosed 2021-03-15)
- Displaced fracture of left femoral neck (diagnosed 2023-07-15)
- Chronic obstructive pulmonary disease (COPD)
- Postmenopausal status

She has a past medical history of chronic obstructive pulmonary disease (COPD) for which she completed a course of oral antibiotics 2 weeks ago.

PAST SURGICAL HISTORY:
- Spontaneous vaginal delivery x3

MEDICATIONS:
Home Medications:
- Multivitamin One tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hip fracture due to osteoporosis
- Father: Chronic obstructive pulmonary disease
- Maternal grandmother: Ovarian cancer

SOCIAL HISTORY:
Smoking: Former smoker (quit 15 years ago). Alcohol: Minimal (one glass of wine weekly). Occupation: Retired elementary school teacher.

VITAL SIGNS:
Her vital signs are: temperature 38.2°C (100.8°F), blood pressure 130/85 mmHg, heart rate 92 beats per minute, and respiratory rate 18 breaths per minute.

PHYSICAL EXAMINATION:
Physical examination reveals mild left lower quadrant tenderness without rebound or guarding. Bowel sounds are hyperactive. There is no peripheral edema, and the remainder of the examination is unremarkable.

LABORATORY DATA:
Laboratory studies show:

- White blood cell count: 15,000/mm³ (normal 4,500-11,000/mm³)
- Hemoglobin: 13.2 g/dL (normal 12-16 g/dL)
- Platelet count: 250,000/mm³ (normal 150,000-450,000/mm³)
- Serum creatinine: 1.0 mg/dL (normal 0.6-1.2 mg/dL)
- Serum electrolytes within normal limits

OTHER STUDIES:
Stool studies for C. difficile toxin are pending.

## Encounter 6: source 2024-01-15, practice 2022-03-21, ed, Emergency Medicine

Chief complaint: Watery diarrhea, abdominal pain, and low-grade fever with 6-8 unformed bowel movements daily for 3 days.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Lisa Wong, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Watery diarrhea, abdominal pain, and low-grade fever with 6-8 unformed bowel movements daily for 3 days.

HISTORY OF PRESENT ILLNESS:
This 76-year-old female with osteoporosis, COPD, and prior C. difficile infection presents to the emergency department with a 3-day history of watery diarrhea, abdominal pain, and low-grade fever. She reports having 6-8 unformed bowel movements per day.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis (diagnosed 2020-01-15)
- Pill-induced esophagitis (diagnosed 2020-02-15)
- Acute osteoporotic vertebral compression fracture of T12 (diagnosed 2021-03-15)
- Displaced fracture of left femoral neck (diagnosed 2023-07-15)
- Severe Clostridioides difficile infection without toxic megacolon (diagnosed 2023-09-15)
- Chronic obstructive pulmonary disease (COPD)
- Postmenopausal status

Her past medical history is significant for chronic obstructive pulmonary disease (COPD) for which she completed a course of oral antibiotics 2 weeks ago.

PAST SURGICAL HISTORY:
- Spontaneous vaginal delivery x3

MEDICATIONS:
Home Medications:
- Multivitamin One tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hip fracture due to osteoporosis
- Father: Chronic obstructive pulmonary disease
- Maternal grandmother: Ovarian cancer

SOCIAL HISTORY:
Smoking: Former smoker (quit 15 years ago). Alcohol: Minimal (one glass of wine weekly). Occupation: Retired elementary school teacher.

VITAL SIGNS:
On examination, her temperature is 38.2°C (100.8°F), heart rate is 102 beats per minute, and blood pressure is 135/85 mmHg.

PHYSICAL EXAMINATION:
Abdominal examination reveals mild diffuse tenderness without rebound or guarding.

LABORATORY DATA:
Her white blood cell count is 15,000/µL, and a stool sample is positive for C. difficile toxin and glutamate dehydrogenase antigen.

## Encounter 7: source 2028-01-15, practice 2026-03-12, outpatient, Obstetrics & Gynecology

Chief complaint: Sudden episode of vaginal bleeding in a postmenopausal woman with no recent trauma.

ENCOUNTER NOTE
Date: 2028-01-15
Provider: Dr. Maria Gonzalez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Sudden episode of vaginal bleeding in a postmenopausal woman with no recent trauma.

HISTORY OF PRESENT ILLNESS:
This patient with prior fragility fractures presents with sudden episode of vaginal bleeding that she found quite distressing. There has been no recent trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis (diagnosed 2020-01-15)
- Pill-induced esophagitis (diagnosed 2020-02-15)
- Acute osteoporotic vertebral compression fracture of T12 (diagnosed 2021-03-15)
- Displaced fracture of left femoral neck (diagnosed 2023-07-15)
- Severe Clostridioides difficile infection without toxic megacolon (diagnosed 2023-09-15)
- Chronic obstructive pulmonary disease (COPD)
- Postmenopausal status

Until then they were monthly and regular. She has 3 children born via spontaneous vaginal delivery. She has had mammograms every 2 years which have all been normal.

PAST SURGICAL HISTORY:
- Spontaneous vaginal delivery x3

MEDICATIONS:
Home Medications:
- Multivitamin One tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hip fracture due to osteoporosis
- Father: Chronic obstructive pulmonary disease
- Maternal grandmother: Ovarian cancer

SOCIAL HISTORY:
She is sexually active exclusively with her husband of 55 years.

VITAL SIGNS:
Vitals are normal.

LABORATORY DATA:
Laboratory studies show no abnormalities.

IMAGING:
An ultrasound shows a thickened endometrial stripe and a 1.3-cm heterogenous mass in the right ovary.
