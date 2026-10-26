# synthetic_hospital patient 2318

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare with supplemental Blue Cross Blue Shield.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2019-05-13 (Mon) | outpatient | Neurology |
| 1 | 2021-01-15 | 2020-05-18 (Mon) | outpatient | Neurology |
| 2 | 2024-01-15 | 2023-05-22 (Mon) | outpatient | Neurology |
| 3 | 2026-01-15 | 2025-05-20 (Tue) | outpatient | Neurology |
| 4 | 2026-09-15 | 2026-01-12 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare with supplemental Blue Cross Blue Shield",
 "occupation": "Retired high school mathematics teacher",
 "alcohol_use": "Social, 1-2 glasses of wine weekly",
 "family_history": [
  "Father: Parkinson disease, onset age 72",
  "Mother: Late-onset Alzheimer disease, onset age 78",
  "Sister: Essential tremor"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 20 years ago, 15 pack-year history",
 "home_medications": [
  {
   "dose": "25/100 mg PO four times daily",
   "name": "Carbidopa/Levodopa"
  },
  {
   "dose": "0.5 mg PO three times daily",
   "name": "Ropinirole"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Sertraline"
  },
  {
   "dose": "1 mg PO daily",
   "name": "Rasagiline"
  }
 ],
 "surgical_history": [
  "Bilateral cataract extraction with intraocular lens placement (2021)",
  "Left inguinal hernia repair with mesh (2010)"
 ],
 "chronic_conditions": [
  "Parkinson Disease",
  "Parkinson Disease Dementia",
  "Major Depressive Disorder",
  "Parkinson Disease Psychosis"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Parkinson disease dementia", "Serotonin Syndrome", "Parkinson disease with psychosis", "Major depressive disorder"]
Comorbidities: ["Parkinson disease with psychosis"]

## Encounter 0: source 2020-01-15, practice 2019-05-13, outpatient, Neurology

Chief complaint: Progressive slowness and stiffness with shuffling gait and resting tremor

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive slowness and stiffness with shuffling gait and resting tremor

HISTORY OF PRESENT ILLNESS:
Over the past year, he has developed a shuffling gait, decreased arm swing, and difficulty initiating movements. His wife also notes a resting tremor in his right hand.

PAST MEDICAL HISTORY:
Active Problem List:
- Parkinson Disease
- Parkinson Disease Dementia
- Major Depressive Disorder
- Parkinson Disease Psychosis

PAST SURGICAL HISTORY:
- Bilateral cataract extraction with intraocular lens placement (2021)
- Left inguinal hernia repair with mesh (2010)

MEDICATIONS:
Home Medications:
- Carbidopa/Levodopa 25/100 mg PO four times daily
- Ropinirole 0.5 mg PO three times daily
- Sertraline 50 mg PO daily
- Rasagiline 1 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Parkinson disease, onset age 72
- Mother: Late-onset Alzheimer disease, onset age 78
- Sister: Essential tremor

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago, 15 pack-year history. Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On exam, he has cogwheel rigidity, a masked facies, and a positive pull test.

IMAGING:
MRI of the brain shows age-appropriate atrophy and no focal lesions.

ASSESSMENT AND PLAN:
The neurologist suspects a disorder involving dopaminergic degeneration.

## Encounter 1: source 2021-01-15, practice 2020-05-18, outpatient, Neurology

Chief complaint: Persistent sadness, loss of interest in hobbies, and occasional thoughts of death

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent sadness, loss of interest in hobbies, and occasional thoughts of death

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old male with Parkinson disease who returns to the neurology clinic one year after his initial diagnosis. He presents to the clinic with his wife, who reports that over the past 3 months, he has been exhibiting persistent sadness, loss of interest in his hobbies, and has expressed feelings of hopelessness. She is particularly concerned because he mentioned thoughts of death on two occasions. His motor symptoms have been stable, and there have been no recent medication changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Parkinson disease with psychosis (diagnosed 2020-01-15)
- Parkinson Disease
- Parkinson Disease Dementia
- Major Depressive Disorder
- Parkinson Disease Psychosis

PAST SURGICAL HISTORY:
- Bilateral cataract extraction with intraocular lens placement (2021)
- Left inguinal hernia repair with mesh (2010)

MEDICATIONS:
The patient has been adherent to his PD medications, which include carbidopa/levodopa and ropinirole. His current medications do not include any antidepressants.

Home Medications:
- Sertraline 50 mg PO daily
- Rasagiline 1 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Parkinson disease, onset age 72
- Mother: Late-onset Alzheimer disease, onset age 78
- Sister: Essential tremor

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago, 15 pack-year history. Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Retired high school mathematics teacher.

REVIEW OF SYSTEMS:
He denies any changes in sleep patterns, appetite, or energy levels.

PHYSICAL EXAMINATION:
On examination, the patient has a flat affect and reduced facial expressivity, consistent with his PD.

LABORATORY DATA:
Laboratory Values:
- Sodium: 140 mEq/L (normal 135-145 mEq/L)
- Potassium: 4.2 mEq/L (normal 3.5-5.1 mEq/L)
- Calcium: 9.6 mg/dL (normal 8.5-10.2 mg/dL)
- Thyroid Stimulating Hormone (TSH): 2.5 µIU/mL (normal 0.4-4.0 µIU/mL)
- Vitamin B12: 400 pg/mL (normal 200-900 pg/mL)

## Encounter 2: source 2024-01-15, practice 2023-05-22, outpatient, Neurology

Chief complaint: Worsening resting tremor, rigidity, and bradykinesia despite current therapy

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening resting tremor, rigidity, and bradykinesia despite current therapy

HISTORY OF PRESENT ILLNESS:
The patient is a 72-year-old male with a four-year history of Parkinson disease and prior treatment for major depressive disorder who returns for follow-up evaluation of worsening motor symptoms. He was diagnosed 4 years ago and initially managed with levodopa monotherapy. Over the past year, he has experienced worsening resting tremor, rigidity, and bradykinesia. He was recently switched to a combination therapy of carbidopa and levodopa. At this visit, he reports significant improvement in his motor symptoms and says he is better able to walk and perform daily activities. He also notes that previous side effects such as nausea and lightheadedness have diminished since starting the new regimen.

PAST MEDICAL HISTORY:
Active Problem List:
- Parkinson disease with psychosis (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2021-01-15)
- Parkinson Disease
- Parkinson Disease Dementia
- Parkinson Disease Psychosis

PAST SURGICAL HISTORY:
- Bilateral cataract extraction with intraocular lens placement (2021)
- Left inguinal hernia repair with mesh (2010)

MEDICATIONS:
Home Medications:
- Carbidopa/Levodopa 25/100 mg PO four times daily
- Ropinirole 0.5 mg PO three times daily
- Sertraline 50 mg PO daily
- Rasagiline 1 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Parkinson disease, onset age 72
- Mother: Late-onset Alzheimer disease, onset age 78
- Sister: Essential tremor

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago, 15 pack-year history. Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
Neurologic exam shows improved tremor and rigidity compared to prior visits. There are no signs of dyskinesia or orthostatic hypotension.

## Encounter 3: source 2026-01-15, practice 2025-05-20, outpatient, Neurology

Chief complaint: Increasing difficulty with planning, organizing, and navigating through familiar environments

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. Emily Nakamura, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing difficulty with planning, organizing, and navigating through familiar environments

HISTORY OF PRESENT ILLNESS:
The patient is a 74-year-old male with Parkinson disease diagnosed in 2020 and major depressive disorder who returns for neurology follow-up. He presents to the clinic with his wife, who reports that over the past 2 years, he has been experiencing increasing difficulty with tasks that require planning and organizing. She also notes that he has trouble navigating through the parking lot and often bumps into objects at home.

PAST MEDICAL HISTORY:
Active Problem List:
- Parkinson disease with psychosis (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2021-01-15)
- Parkinson Disease
- Parkinson Disease Dementia
- Parkinson Disease Psychosis

His past medical history is significant for Parkinson's disease, diagnosed 4 years ago, with symptoms of resting tremor, bradykinesia, and rigidity that have been well-controlled on carbidopa/levodopa.

PAST SURGICAL HISTORY:
- Bilateral cataract extraction with intraocular lens placement (2021)
- Left inguinal hernia repair with mesh (2010)

MEDICATIONS:
He takes carbidopa/levodopa.

Home Medications:
- Ropinirole 0.5 mg PO three times daily
- Sertraline 50 mg PO daily
- Rasagiline 1 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Parkinson disease, onset age 72
- Mother: Late-onset Alzheimer disease, onset age 78
- Sister: Essential tremor

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago, 15 pack-year history. Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Retired high school mathematics teacher.

REVIEW OF SYSTEMS:
There is no history of hallucinations or fluctuations in attention or alertness.

PHYSICAL EXAMINATION:
On examination, the patient has a shuffling gait, masked facies, and cogwheel rigidity. His Mini-Mental State Examination score is 24/30, with deficits noted in executive function and visuospatial abilities, but intact immediate recall.

## Encounter 4: source 2026-09-15, practice 2026-01-12, ed, Emergency Medicine

Chief complaint: Sudden-onset confusion, high fever, and severe muscle rigidity

ENCOUNTER NOTE
Date: 2026-09-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset confusion, high fever, and severe muscle rigidity

HISTORY OF PRESENT ILLNESS:
This 74-year-old male with Parkinson Disease, Parkinson Disease Dementia, Major Depressive Disorder, and Parkinson Disease Psychosis, followed longitudinally in Neurology for progressive motor and cognitive decline, presents to the Emergency Department with sudden-onset confusion, high fever, and severe muscle rigidity. His symptoms began earlier today and have worsened rapidly.

PAST MEDICAL HISTORY:
Active Problem List:
- Parkinson disease with psychosis (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2021-01-15)
- Parkinson disease dementia (diagnosed 2026-01-15)
- Parkinson Disease
- Parkinson Disease Psychosis

PAST SURGICAL HISTORY:
- Bilateral cataract extraction with intraocular lens placement (2021)
- Left inguinal hernia repair with mesh (2010)

MEDICATIONS:
His current medications include carbidopa-levodopa and a recently added drug for Parkinson disease, along with sertraline for depression.

Home Medications:
- Carbidopa/Levodopa 25/100 mg PO four times daily
- Ropinirole 0.5 mg PO three times daily
- Rasagiline 1 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Parkinson disease, onset age 72
- Mother: Late-onset Alzheimer disease, onset age 78
- Sister: Essential tremor

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago, 15 pack-year history. Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
On arrival, his temperature is 39.2°C (102.6°F), blood pressure is 160/95 mm Hg, heart rate is 122/min

PHYSICAL EXAMINATION:
He is diaphoretic. He is agitated and tremulous with clonus at the ankles and hyperreflexia in both lower extremities.

ASSESSMENT AND PLAN:
The physician suspects serotonin syndrome.
