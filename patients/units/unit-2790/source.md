# synthetic_hospital patient 2790

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-12-16 (Mon) | outpatient | Gastroenterology |
| 1 | 2021-03-15 | 2026-03-02 (Mon) | outpatient | Endocrinology |
| 2 | 2021-04-15 | 2026-03-16 (Mon) | follow_up | General Surgery |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "High school teacher",
 "alcohol_use": "None",
 "family_history": [
  "Maternal aunt with papillary thyroid cancer",
  "Father with hypertension",
  "Mother with osteoporosis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1250 mg PO TID with meals",
   "name": "Calcium carbonate"
  },
  {
   "dose": "0.25 mcg PO BID",
   "name": "Calcitriol"
  },
  {
   "dose": "17 g PO daily PRN constipation",
   "name": "Polyethylene glycol 3350"
  }
 ],
 "surgical_history": [
  "Total thyroidectomy (2 weeks prior to latest encounter)",
  "Cesarean section (age 32)"
 ],
 "chronic_conditions": [
  "Irritable bowel syndrome"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Postprocedural hypoparathyroidism", "Irritable bowel syndrome", "Thyroid nodule with suspicious sonographic features for malignancy"]
Comorbidities: ["Situational anxiety"]

## Encounter 0: source 2020-01-15, practice 2024-12-16, outpatient, Gastroenterology

Chief complaint: presents to the physician with a 6-month history of increasing difficulty with daily activities due to pervasive fatigue, lack of mental clarity, and intermittent bloating

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Aisha Johnson, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
She presents to the physician with a 6-month history of increasing difficulty with daily activities due to pervasive fatigue, lack of mental clarity, and intermittent bloating

HISTORY OF PRESENT ILLNESS:
She has also noticed that her bowel movements have become irregular, alternating between periods of constipation and diarrhea, though her stool volume is normal. Over the past few weeks, she has experienced a sense of fullness in the lower abdomen that improves marginally with bowel movements, as well as an increased need for rest after mild exertion. Despite these symptoms, she has not had significant weight loss or appetite changes, although she does feel less hungry in the mornings. She attributes her symptoms to high levels of stress from a recent change in job responsibilities, and admits to struggling with anxiety and poor sleep.

PAST MEDICAL HISTORY:
Active Problem List:
- Irritable bowel syndrome

PAST SURGICAL HISTORY:
- Total thyroidectomy (2 weeks prior to latest encounter)
- Cesarean section (age 32)

MEDICATIONS:
Home Medications:
- Calcium carbonate 1250 mg PO TID with meals
- Calcitriol 0.25 mcg PO BID
- Polyethylene glycol 3350 17 g PO daily PRN constipation

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal aunt with papillary thyroid cancer
- Father with hypertension
- Mother with osteoporosis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: High school teacher.

PHYSICAL EXAMINATION:
On physical examination, her abdomen is slightly distended with mild tenderness over the lower quadrants but no palpable masses or signs of peritonitis.

LABORATORY DATA:
Laboratory studies are normal.

## Encounter 1: source 2021-03-15, practice 2026-03-02, outpatient, Endocrinology

Chief complaint: presents to the clinic with a palpable mass in her neck

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Robert Liu, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
She presents to the clinic with a palpable mass in her neck

HISTORY OF PRESENT ILLNESS:
The patient, with a history of irritable bowel syndrome evaluated in January 2020 for fatigue and bloating, presents with a neck mass that she noticed 1 month ago. She reports no pain but feels a sensation of fullness in neck and occasional difficulty swallowing. She denies voice, weight, appetite, or energy changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Irritable bowel syndrome (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- Total thyroidectomy (2 weeks prior to latest encounter)
- Cesarean section (age 32)

MEDICATIONS:
Home Medications:
- Calcium carbonate 1250 mg PO TID with meals
- Calcitriol 0.25 mcg PO BID
- Polyethylene glycol 3350 17 g PO daily PRN constipation

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal aunt with papillary thyroid cancer
- Father with hypertension
- Mother with osteoporosis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: High school teacher.

PHYSICAL EXAMINATION:
On physical examination, a 2 cm firm, non-tender nodule is palpated in the right lobe of the thyroid. There is no cervical lymphadenopathy. The rest of the examination is normal.

LABORATORY DATA:
Serum laboratory tests reveal the following:

- TSH: 2.5 mIU/L (normal range: 0.4-4.0 mIU/L)
- Free T4: 1.2 ng/dL (normal range: 0.8-1.8 ng/dL)

IMAGING:
An ultrasound of the thyroid shows a 2 cm hypoechoic, solid nodule with irregular margins and microcalcifications in the right lobe.

## Encounter 2: source 2021-04-15, practice 2026-03-16, follow_up, General Surgery

Chief complaint: progressive fatigue, occasional difficulty swallowing, and tingling around her lips and fingertips for the past 2 weeks

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. Maria Gonzalez, MD
Department: General Surgery
Visit Type: follow_up

CHIEF COMPLAINT:
Progressive fatigue, occasional difficulty swallowing, and tingling around her lips and fingertips for the past 2 weeks

HISTORY OF PRESENT ILLNESS:
The patient is a 46-year-old female with a history of irritable bowel syndrome and recent evaluation for a thyroid nodule. She returns for follow-up reporting occasional muscle cramps, especially in her calves, and a sensation of tightness in her throat when speaking for long periods. Over the past 3 days, she has noticed that her symptoms worsen when she is physically active. She denies dizziness, palpitations, or recent gastrointestinal illness.

PAST MEDICAL HISTORY:
Active Problem List:
- Irritable bowel syndrome (diagnosed 2020-01-15)
- Thyroid nodule with suspicious sonographic features for malignancy (diagnosed 2021-03-15)

PAST SURGICAL HISTORY:
She underwent surgery for a thyroid nodule 3 months ago, with an uncomplicated postoperative course at that time.

MEDICATIONS:
Home Medications:
- Calcium carbonate 1250 mg PO TID with meals
- Calcitriol 0.25 mcg PO BID
- Polyethylene glycol 3350 17 g PO daily PRN constipation

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is noncontributory.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: High school teacher.

PHYSICAL EXAMINATION:
On examination, she appears mildly anxious, with occasional hand muscle twitching during conversation. Neurologic examination shows intact cranial nerves and normal strength but occasional involuntary contractions in the hands.

LABORATORY DATA:
Laboratory studies show serum calcium of 7.0 mg/dL, serum phosphate of 5.6 mg/dL, and serum magnesium of 1.8 mEq/L.
