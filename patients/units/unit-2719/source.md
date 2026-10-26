# synthetic_hospital patient 2719

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 47 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2019-01-25 (Fri) | outpatient | Psychiatry |
| 1 | 2020-07-15 | 2019-07-15 (Mon) | outpatient | Neurology |
| 2 | 2021-01-15 | 2020-01-22 (Wed) | outpatient | Neurology |
| 3 | 2022-01-15 | 2021-01-18 (Mon) | outpatient | Neurology |
| 4 | 2024-01-15 | 2023-01-20 (Fri) | outpatient | Endocrinology |
| 5 | 2027-01-15 | 2026-01-19 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Administrative assistant",
 "alcohol_use": "Social, 1-2 drinks per month",
 "family_history": [
  "Breast cancer in maternal aunt (age 54)",
  "Hypertension in mother",
  "Major depressive disorder in sister"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 1 pack per week for 20 years, expressing desire to quit",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Hypertension"
 ],
 "age_at_first_encounter": 47
}
```

Primary diagnoses: ["Trigeminal neuralgia", "Toxic single thyroid nodule", "Inflammatory breast carcinoma", "Major depressive disorder"]
Comorbidities: ["Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2019-01-25, outpatient, Psychiatry

Chief complaint: persistent feelings of sadness, fatigue, and lack of motivation

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent feelings of sadness, fatigue, and lack of motivation

HISTORY OF PRESENT ILLNESS:
That have been worsening over the past 6 months. She reports sleeping up to 12 hours a day and has gained 15 pounds during this period. She denies any recent stressors or changes in her life situation.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension

Her past medical history is significant for hypertension, which is well-controlled with lisinopril.

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Breast cancer in maternal aunt (age 54)
- Hypertension in mother
- Major depressive disorder in sister

SOCIAL HISTORY:
She does not smoke or use illicit drugs but mentions a desire to quit her 20-year habit of smoking a pack of cigarettes per week.

VITAL SIGNS:
Her vital signs are within normal limits

PHYSICAL EXAMINATION:
Her physical examination is unremarkable

LABORATORY DATA:
Laboratory tests reveal normal thyroid function and a fasting glucose of 98 mg/dL

## Encounter 1: source 2020-07-15, practice 2019-07-15, outpatient, Neurology

Chief complaint: recurring episodes of sharp facial pain for the past 2 months

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Okafor, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Recurring episodes of sharp facial pain for the past 2 months

HISTORY OF PRESENT ILLNESS:
This 47-year-old female with hypertension on lisinopril and prior psychiatric evaluation six months ago presents with facial pain. She describes the pain as "like an electric shock or stabbing" on the right side of her face, which lasts for less than a minute and occurs multiple times daily. The episodes are often triggered by brushing her teeth.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Hypertension

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Breast cancer in maternal aunt (age 54)
- Hypertension in mother
- Major depressive disorder in sister

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per week for 20 years, expressing desire to quit. Alcohol: Social, 1-2 drinks per month. Occupation: Administrative assistant.

REVIEW OF SYSTEMS:
She has no visual changes, fever, or other neurologic symptoms.

PHYSICAL EXAMINATION:
Neurologic examination is normal.

## Encounter 2: source 2021-01-15, practice 2020-01-22, outpatient, Neurology

Chief complaint: severe, recurrent episodes of sharp facial pain for the past 3 months

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Okafor, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Severe, recurrent episodes of sharp facial pain for the past 3 months

HISTORY OF PRESENT ILLNESS:
The patient is a 48-year-old female with hypertension and a history of major depressive disorder who returns to the neurology clinic with recurrent facial pain similar to that reported during her visit to this service in July 2020. The pain is described as a sudden, stabbing sensation localized to her right cheek and upper lip, lasting less than a minute and occurring multiple times per day. She notes that even light touch or brushing her teeth can trigger these attacks. There is no associated headache, visual aura, photophobia, or autonomic symptoms. She has no history of trauma, infection, or dental procedures in the affected area.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Trigeminal neuralgia (diagnosed 2020-07-15)
- Hypertension

MEDICATIONS:
She does not take any chronic medications.

Home Medications:
- Lisinopril 10 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Breast cancer in maternal aunt (age 54)
- Hypertension in mother
- Major depressive disorder in sister

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per week for 20 years, expressing desire to quit. Alcohol: Social, 1-2 drinks per month. Occupation: Administrative assistant.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
Her neurological examination is normal, and there is no facial weakness or sensory loss between episodes.

## Encounter 3: source 2022-01-15, practice 2021-01-18, outpatient, Neurology

Chief complaint: intermittent, severe, sharp pain on the right side of her face

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James Okafor, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent, severe, sharp pain on the right side of her face

HISTORY OF PRESENT ILLNESS:
The patient is a 49-year-old female with hypertension and major depressive disorder who returns for follow-up of facial pain previously evaluated in 2020 and 2021. She reports a 6-month history of intermittent, severe, sharp pain on the right side of her face. The pain is localized to her cheek and lower jaw and lasts for a few seconds to two minutes. She reports that the pain is often triggered by chewing, brushing her teeth, or sometimes even a light breeze on her face. She has no history of skin lesions or rashes.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Trigeminal neuralgia (diagnosed 2020-07-15)
- Hypertension

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Breast cancer in maternal aunt (age 54)
- Hypertension in mother
- Major depressive disorder in sister

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per week for 20 years, expressing desire to quit. Alcohol: Social, 1-2 drinks per month. Occupation: Administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, there is no facial asymmetry, skin lesions, or palpable masses. Light touch to the affected area provokes a sharp pain response. Neurological examination is otherwise unremarkable.

## Encounter 4: source 2024-01-15, practice 2023-01-20, outpatient, Endocrinology

Chief complaint: 3-month history of palpitations, heat intolerance, and unintentional weight loss

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Maria Gonzalez, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of palpitations, heat intolerance, and unintentional weight loss

HISTORY OF PRESENT ILLNESS:
The patient is a 51-year-old female with a history of hypertension, major depressive disorder, and recurrent trigeminal neuralgia who presents with a three-month history of palpitations, heat intolerance, and unintentional weight loss of 10 pounds despite an increased appetite. She denies any changes in bowel habits or menstrual irregularities.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Trigeminal neuralgia (diagnosed 2020-07-15)
- Hypertension

Her past medical history is significant for hypertension, well-controlled on amlodipine.

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Breast cancer in maternal aunt (age 54)
- Hypertension in mother
- Major depressive disorder in sister

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per week for 20 years, expressing desire to quit. Alcohol: Social, 1-2 drinks per month. Occupation: Administrative assistant.

VITAL SIGNS:
On physical examination, her blood pressure is 145/90 mmHg, heart rate is 102 beats per minute

PHYSICAL EXAMINATION:
She has a fine tremor in her outstretched hands. A palpable, non-tender nodule is noted on the right lobe of the thyroid gland. There is no ophthalmopathy or dermopathy observed.

LABORATORY DATA:
Thyroid function tests reveal the following:

- Serum TSH: <0.01 mIU/L (normal range: 0.4-4.0 mIU/L)
- Free T4: 3.0 ng/dL (normal range: 0.9-1.7 ng/dL)
- Free T3: 8.0 pg/mL (normal range: 2.3-4.2 pg/mL)

IMAGING:
A radioactive iodine uptake scan shows increased uptake in the right thyroid nodule with suppression of the rest of the thyroid gland.

## Encounter 5: source 2027-01-15, practice 2026-01-19, ed, Emergency Medicine

Chief complaint: 2-month history of progressive left breast pain and swelling

ENCOUNTER NOTE
Date: 2027-01-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-month history of progressive left breast pain and swelling

HISTORY OF PRESENT ILLNESS:
She is a 54-year-old with hypertension, depression, trigeminal neuralgia, and toxic thyroid nodule. She reports that the breast has become red and feels warm to the touch. She denies any fever, nipple discharge, or recent trauma to the area.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Trigeminal neuralgia (diagnosed 2020-07-15)
- Toxic single thyroid nodule (diagnosed 2024-01-15)
- Hypertension

Her past medical history is significant for hypertension, controlled with hydrochlorothiazide.

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
She has no family history of breast cancer.

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per week for 20 years, expressing desire to quit. Alcohol: Social, 1-2 drinks per month. Occupation: Administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, the left breast is diffusely enlarged, erythematous, and edematous with overlying skin that has a peau d’orange appearance. There is no palpable mass, but the nipple is retracted. Axillary lymph nodes are palpable and firm.

LABORATORY DATA:
Complete Blood Count (CBC): Within normal limits
- Erythrocyte Sedimentation Rate (ESR): Elevated
- Carcinoembryonic Antigen (CEA): Elevated
- CA 15-3: Elevated

IMAGING:
A mammogram shows skin thickening and increased breast density but no distinct mass. A subsequent breast MRI reveals extensive non-mass enhancement with skin involvement and suspicious axillary lymphadenopathy.
