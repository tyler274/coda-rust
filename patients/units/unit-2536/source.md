# synthetic_hospital patient 2536

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 22 at first encounter, sex M, White, Student health insurance plan.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-05-27 (Thu) | outpatient | Endocrinology |
| 1 | 2020-07-15 | 2021-11-30 (Tue) | outpatient | Gastroenterology |
| 2 | 2021-01-15 | 2022-05-24 (Tue) | outpatient | Gastroenterology |
| 3 | 2021-09-15 | 2023-01-31 (Tue) | outpatient | Internal Medicine |
| 4 | 2022-03-15 | 2023-07-21 (Fri) | follow_up | General Surgery |
| 5 | 2023-01-15 | 2024-05-22 (Wed) | outpatient | Dermatology |
| 6 | 2024-01-15 | 2025-05-23 (Fri) | outpatient | Internal Medicine |
| 7 | 2025-01-15 | 2026-05-19 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Student health insurance plan",
 "occupation": "Undergraduate student",
 "alcohol_use": "None",
 "family_history": [
  "Father with type 2 diabetes mellitus",
  "Mother with autoimmune hypothyroidism",
  "Older brother with Gilbert syndrome",
  "Paternal cousin with Dubin-Johnson syndrome"
 ],
 "race_ethnicity": "White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "16 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "Sliding scale with meals",
   "name": "Insulin lispro"
  },
  {
   "dose": "500 mg orally twice daily as needed for oral lesions",
   "name": "Valacyclovir"
  }
 ],
 "surgical_history": [
  "Laparoscopic appendectomy"
 ],
 "chronic_conditions": [
  "Gilbert syndrome",
  "Dubin-Johnson syndrome",
  "Type 1 diabetes mellitus",
  "Recurrent herpes simplex virus type 1 infection"
 ],
 "age_at_first_encounter": 22
}
```

Primary diagnoses: ["Gilbert syndrome", "Dubin-Johnson syndrome", "Type 1 diabetes mellitus", "Heat exhaustion", "Herpes Simplex Virus Infection"]
Comorbidities: ["Dehydration", "Psychological stress", "Sleep deprivation", "Family History of Hereditary Hemorrhagic Telangiectasia"]

## Encounter 0: source 2020-01-15, practice 2021-05-27, outpatient, Endocrinology

Chief complaint: Persistent fatigue, excessive thirst, frequent urination, and unintentional weight loss of 6 kg over the past month.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Sharma, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue, excessive thirst, frequent urination, and unintentional weight loss of 6 kg over the past month.

HISTORY OF PRESENT ILLNESS:
Over the past month, he has been waking up multiple times during the night to urinate and has begun carrying a water bottle with him everywhere. He reports drinking nearly 5 liters of water daily. Despite a markedly increased appetite, he has lost 6 kg (13 lb) unintentionally over the same period. He also reports intermittent blurry vision over the past week, especially when reading or using his laptop. He denies any dysuria, urgency, or abdominal pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Gilbert syndrome
- Dubin-Johnson syndrome
- Type 1 diabetes mellitus
- Recurrent herpes simplex virus type 1 infection

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy

MEDICATIONS:
Home Medications:
- Insulin glargine 16 units subcutaneous at bedtime
- Insulin lispro Sliding scale with meals
- Valacyclovir 500 mg orally twice daily as needed for oral lesions

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with type 2 diabetes mellitus
- Mother with autoimmune hypothyroidism
- Older brother with Gilbert syndrome
- Paternal cousin with Dubin-Johnson syndrome

SOCIAL HISTORY:
He does not use drugs, alcohol, or tobacco.

REVIEW OF SYSTEMS:
There is no fever, rash, or joint pain.

VITAL SIGNS:
Blood pressure is 106/64 mm Hg, pulse is 92/min, and temperature is normal.

PHYSICAL EXAMINATION:
On physical exam, he is thin and appears slightly dehydrated. Mucous membranes are dry. Visual acuity is mildly reduced but improves temporarily with blinking. Fundoscopic exam is normal. Genitourinary and neurologic exams are unremarkable.

ASSESSMENT AND PLAN:
The clinician suspects a metabolic condition

Orders laboratory testing

## Encounter 1: source 2020-07-15, practice 2021-11-30, outpatient, Gastroenterology

Chief complaint: Mild jaundice with yellowing of skin and eyes for 2 days; reports similar episodes during illness or stress over the past year.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Mild jaundice with yellowing of skin and eyes for 2 days; reports similar episodes during illness or stress over the past year.

HISTORY OF PRESENT ILLNESS:
The patient returns for Gastroenterology follow-up with known Dubin-Johnson syndrome and Gilbert syndrome, presenting with recurrent jaundice six months after diabetes diagnosis. He reports that the yellowing of his skin and eyes seems to come and go, and he has noticed similar episodes a few times over the past year, usually during times of illness or stress. He denies pruritus, abdominal pain, dark urine, or pale stools.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 1 diabetes mellitus (diagnosed 2020-01-15)
- Gilbert syndrome
- Dubin-Johnson syndrome
- Recurrent herpes simplex virus type 1 infection

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy

MEDICATIONS:
Home Medications:
- Insulin glargine 16 units subcutaneous at bedtime
- Insulin lispro Sliding scale with meals
- Valacyclovir 500 mg orally twice daily as needed for oral lesions

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His mother has a history of similar symptoms but has never been formally diagnosed with a liver disorder.

SOCIAL HISTORY:
He does not smoke, drink alcohol, or use illicit drugs.

PHYSICAL EXAMINATION:
Physical examination reveals scleral icterus and a slightly tender liver edge, but no other abnormalities.

LABORATORY DATA:
Laboratory tests reveal the following:

- Total bilirubin: 3.2 mg/dL (normal: 0.3-1.2 mg/dL)
- Direct (conjugated) bilirubin: 2.5 mg/dL (normal: 0.0-0.3 mg/dL)
- Indirect (unconjugated) bilirubin: 0.7 mg/dL (normal: 0.2-0.8 mg/dL)
- Alanine aminotransferase (ALT): 45 U/L (normal: 7-56 U/L)
- Aspartate aminotransferase (AST): 40 U/L (normal: 8-48 U/L)
- Alkaline phosphatase (ALP): 90 U/L (normal: 40-129 U/L)

## Encounter 2: source 2021-01-15, practice 2022-05-24, outpatient, Gastroenterology

Chief complaint: Yellowing of skin and eyes for 2 days during final examination period with poor sleep and irregular eating.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Sarah Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Yellowing of skin and eyes for 2 days during final examination period with poor sleep and irregular eating.

HISTORY OF PRESENT ILLNESS:
The patient returns with recurrent jaundice in the setting of known Gilbert and Dubin-Johnson syndromes. He reports no pain, pruritus, or changes in stool or urine color. He mentions that he has been preparing for final exams and has not been eating or sleeping well due to stress.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 1 diabetes mellitus (diagnosed 2020-01-15)
- Dubin-Johnson syndrome (diagnosed 2020-07-15)
- Gilbert syndrome
- Recurrent herpes simplex virus type 1 infection

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy

MEDICATIONS:
Home Medications:
- Insulin glargine 16 units subcutaneous at bedtime
- Insulin lispro Sliding scale with meals
- Valacyclovir 500 mg orally twice daily as needed for oral lesions

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with type 2 diabetes mellitus
- Mother with autoimmune hypothyroidism
- Older brother with Gilbert syndrome
- Paternal cousin with Dubin-Johnson syndrome

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Undergraduate student.

PHYSICAL EXAMINATION:
On physical examination, he appears well except for scleral icterus. His abdomen is soft, non-tender, and without hepatosplenomegaly.

LABORATORY DATA:
Laboratory tests reveal:

- Total bilirubin: 3.2 mg/dL
- Direct bilirubin: 0.4 mg/dL
- Indirect bilirubin: 2.8 mg/dL
- AST: 30 U/L
- ALT: 28 U/L
- Alkaline phosphatase: 70 U/L
- Complete blood count: within normal limits

## Encounter 3: source 2021-09-15, practice 2023-01-31, outpatient, Internal Medicine

Chief complaint: Yellowing of skin and eyes for 2 days associated with irregular meals during final exam preparation.

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Yellowing of skin and eyes for 2 days associated with irregular meals during final exam preparation.

HISTORY OF PRESENT ILLNESS:
The patient returns with recurrent jaundice in the setting of Gilbert and Dubin-Johnson syndromes, noting prior episodes during academic stress. He presents to the clinic with a 2-day history of yellowing of the skin and eyes. He reports no pain, itching, or changes in stool or urine color. He mentions that he has been preparing for final exams and has not been eating regularly due to stress.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 1 diabetes mellitus (diagnosed 2020-01-15)
- Dubin-Johnson syndrome (diagnosed 2020-07-15)
- Gilbert syndrome (diagnosed 2021-01-15)
- Recurrent herpes simplex virus type 1 infection

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy

MEDICATIONS:
Home Medications:
- Insulin glargine 16 units subcutaneous at bedtime
- Insulin lispro Sliding scale with meals
- Valacyclovir 500 mg orally twice daily as needed for oral lesions

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with type 2 diabetes mellitus
- Mother with autoimmune hypothyroidism
- Older brother with Gilbert syndrome
- Paternal cousin with Dubin-Johnson syndrome

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Undergraduate student.

PHYSICAL EXAMINATION:
On physical examination, he appears well except for scleral icterus. His abdomen is soft, non-tender, and without hepatosplenomegaly.

LABORATORY DATA:
Laboratory tests reveal:

- Total bilirubin: 3.2 mg/dL
- Direct bilirubin: 0.5 mg/dL
- Indirect bilirubin: 2.7 mg/dL
- AST: 30 U/L
- ALT: 28 U/L
- Alkaline phosphatase: 75 U/L
- Complete blood count: within normal limits

## Encounter 4: source 2022-03-15, practice 2023-07-21, follow_up, General Surgery

Chief complaint: Yellowing of eyes noticed over past two days, one week after undergoing laparoscopic appendectomy.

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Emily Thompson, MD
Department: General Surgery
Visit Type: follow_up

CHIEF COMPLAINT:
Yellowing of eyes noticed over past two days, one week after undergoing laparoscopic appendectomy.

HISTORY OF PRESENT ILLNESS:
The patient returns for postoperative follow-up with known Gilbert syndrome and Dubin-Johnson syndrome, having presented previously in 2021 with similar episodes of scleral icterus during examination periods. He presents to clinic one week after undergoing a laparoscopic appendectomy for uncomplicated acute appendicitis. He reports that he has been recovering well overall but noticed that the whites of his eyes appeared yellow over the past two days. He has no abdominal pain, nausea, vomiting, or changes in bowel movements. His appetite has improved since surgery, and he is passing gas and tolerating oral intake. He denies fever, chills, dark urine, or pale stools. He has been walking daily.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 1 diabetes mellitus (diagnosed 2020-01-15)
- Dubin-Johnson syndrome (diagnosed 2020-07-15)
- Gilbert syndrome (diagnosed 2021-01-15)
- Recurrent herpes simplex virus type 1 infection

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy

MEDICATIONS:
Taking only acetaminophen for occasional discomfort.

Home Medications:
- Insulin glargine 16 units subcutaneous at bedtime
- Insulin lispro Sliding scale with meals
- Valacyclovir 500 mg orally twice daily as needed for oral lesions

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His family history is notable for his father having “some liver enzyme issue” that was never treated.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Undergraduate student.

REVIEW OF SYSTEMS:
On review of systems, he reports feeling a little fatigued but otherwise well.

VITAL SIGNS:
Afebrile

PHYSICAL EXAMINATION:
Scleral icterus is present, but his skin is not jaundiced. His abdomen is soft and non-tender with healed incisions and no signs of fluid collection.

LABORATORY DATA:
Total bilirubin of 3.1 mg/dL with indirect bilirubin of 2.8 mg/dL, direct bilirubin of 0.3 mg/dL, normal AST, ALT, and alkaline phosphatase, and normal hemoglobin and hematocrit.

## Encounter 5: source 2023-01-15, practice 2024-05-22, outpatient, Dermatology

Chief complaint: Painful sores on lips and inside mouth for 3 days with mild fever, preceded by tingling sensation.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Kim, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Painful sores on lips and inside mouth for 3 days with mild fever, preceded by tingling sensation.

HISTORY OF PRESENT ILLNESS:
The patient, a 25-year-old male with recurrent HSV-1 infection and Gilbert syndrome with stress-induced flares, presents with perioral lesions. He reports feeling a tingling sensation in the area before the blisters appeared. This has happened several times in the past, usually during periods of stress or lack of sleep. The lesions are limited to the perioral region and are associated with mild fever.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 1 diabetes mellitus (diagnosed 2020-01-15)
- Dubin-Johnson syndrome (diagnosed 2020-07-15)
- Gilbert syndrome (diagnosed 2021-01-15)
- Recurrent herpes simplex virus type 1 infection

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy

MEDICATIONS:
Home Medications:
- Insulin glargine 16 units subcutaneous at bedtime
- Insulin lispro Sliding scale with meals
- Valacyclovir 500 mg orally twice daily as needed for oral lesions

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with type 2 diabetes mellitus
- Mother with autoimmune hypothyroidism
- Older brother with Gilbert syndrome
- Paternal cousin with Dubin-Johnson syndrome

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Undergraduate student.

## Encounter 6: source 2024-01-15, practice 2025-05-23, outpatient, Internal Medicine

Chief complaint: Yellowing of eyes noticed by roommate this morning after returning from a five-day camping trip with irregular meals.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Lisa Johnson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Yellowing of eyes noticed by roommate this morning after returning from a five-day camping trip with irregular meals.

HISTORY OF PRESENT ILLNESS:
The patient is a 26-year-old male with Gilbert syndrome and Dubin-Johnson syndrome who returns with recurrent jaundice, having experienced similar episodes during examination periods in 2021 and following appendectomy in 2022. He denies any abdominal pain, nausea, vomiting, fatigue, weight loss, pruritus, or dark urine. He reports that he just returned from a five-day camping trip during which he had irregular meals and ate very little due to limited access to food. He notes feeling mildly tired since getting back but attributes this to dehydration and lack of sleep. He denies recent infections, alcohol use, drug use, or any new medications or supplements. He recalls having a similar episode of yellowing in college after pulling several all-nighters for exams and skipping meals for a couple of days. At that time, he felt well otherwise and the yellowing resolved on its own without intervention.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 1 diabetes mellitus (diagnosed 2020-01-15)
- Dubin-Johnson syndrome (diagnosed 2020-07-15)
- Gilbert syndrome (diagnosed 2021-01-15)
- Herpes Simplex Virus Infection (diagnosed 2023-01-15)
- Recurrent herpes simplex virus type 1 infection

PAST SURGICAL HISTORY:
He has never had surgery

MEDICATIONS:
He does not take any regular medications.

Home Medications:
- Insulin glargine 16 units subcutaneous at bedtime
- Insulin lispro Sliding scale with meals
- Valacyclovir 500 mg orally twice daily as needed for oral lesions

ALLERGIES:
He has no allergies

FAMILY HISTORY:
Family history is notable for his younger brother experiencing similar episodes. There is no history of liver disease, gallstones, or autoimmune conditions in the family.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Undergraduate student.

VITAL SIGNS:
Vitals are within normal limits, BMI is 22

PHYSICAL EXAMINATION:
On physical exam, the patient appears well and is in no acute distress. He is alert and oriented. Scleral icterus is present but there is no jaundice of the skin. There is no hepatomegaly or splenomegaly on abdominal palpation, and the liver span is normal to percussion. There are no spider angiomas, palmar erythema, ascites, or other stigmata of chronic liver disease. Neurologic and cardiopulmonary exams are unremarkable.

LABORATORY DATA:
Total bilirubin                                                2.6 mg/dL
Indirect bilirubin                                             2.5 mg/dL
ALT                                                                20 IU/L
AST                                                                18 IU/L
ALP                                                               120 IU/L
Hemoglobin                                                        14 g/dL

## Encounter 7: source 2025-01-15, practice 2026-05-19, ed, Emergency Medicine

Chief complaint: Lightheadedness, dizziness, and nausea for the past hour after participating in a soccer tournament during hot summer weather.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Robert Singh, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Lightheadedness, dizziness, and nausea for the past hour after participating in a soccer tournament during hot summer weather.

HISTORY OF PRESENT ILLNESS:
The patient has Type 1 diabetes mellitus, Gilbert syndrome, and Dubin-Johnson syndrome, with recurrent jaundice episodes most recently in January 2024. He presents to the emergency department in July after participating in a soccer tournament. He reports that he has been playing soccer for the past three hours in the hot sun without any breaks. He has been drinking water throughout the day but has not eaten much.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 1 diabetes mellitus (diagnosed 2020-01-15)
- Dubin-Johnson syndrome (diagnosed 2020-07-15)
- Gilbert syndrome (diagnosed 2021-01-15)
- Herpes Simplex Virus Infection (diagnosed 2023-01-15)
- Recurrent herpes simplex virus type 1 infection

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy

MEDICATIONS:
Home Medications:
- Insulin glargine 16 units subcutaneous at bedtime
- Insulin lispro Sliding scale with meals
- Valacyclovir 500 mg orally twice daily as needed for oral lesions

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with type 2 diabetes mellitus
- Mother with autoimmune hypothyroidism
- Older brother with Gilbert syndrome
- Paternal cousin with Dubin-Johnson syndrome

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Undergraduate student.

VITAL SIGNS:
His vital signs are as follows: temperature 39.5 C (103.1 F), heart rate 110 beats per minute, blood pressure 100/60 mmHg, respiratory rate 22 breaths per minute, and oxygen saturation 98% on room air.

PHYSICAL EXAMINATION:
On examination, his skin is cool and moist, and he appears fatigued but is oriented to time, place, and person.

LABORATORY DATA:
Laboratory values:
- Sodium: 135 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 100 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen (BUN): 18 mg/dL
- Creatinine: 1.0 mg/dL
- Glucose: 90 mg/dL
