# synthetic_hospital patient 1746

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 35 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-08-11 (Wed) | outpatient | Neurology |
| 1 | 2020-04-15 | 2021-11-08 (Mon) | outpatient | Psychiatry |
| 2 | 2020-10-15 | 2022-05-16 (Mon) | outpatient | Neurology |
| 3 | 2021-04-15 | 2022-10-31 (Mon) | outpatient | Neurology |
| 4 | 2021-10-15 | 2023-05-03 (Wed) | ed | Emergency Medicine |
| 5 | 2022-07-15 | 2024-02-05 (Mon) | outpatient | Neurology |
| 6 | 2023-07-15 | 2025-02-03 (Mon) | outpatient | Internal Medicine |
| 7 | 2024-07-15 | 2026-02-04 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Marketing Manager",
 "alcohol_use": "Social (2-3 drinks per week)",
 "family_history": [
  "Mother: Hashimoto's thyroiditis",
  "Father: Major depressive disorder",
  "Maternal aunt: Multiple sclerosis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "100 mcg daily",
   "name": "Levothyroxine"
  },
  {
   "dose": "90 mcg (1-2 inhalations) every 4-6 hours as needed for asthma symptoms",
   "name": "Albuterol HFA inhaler"
  },
  {
   "dose": "50 mg daily",
   "name": "Sertraline"
  }
 ],
 "surgical_history": [
  "Open reduction and internal fixation of pelvis following traumatic fracture (2 years prior)"
 ],
 "chronic_conditions": [
  "Multiple sclerosis",
  "Hashimoto's thyroiditis",
  "Asthma",
  "Major depressive disorder"
 ],
 "age_at_first_encounter": 35
}
```

Primary diagnoses: ["Major depressive disorder", "Multiple sclerosis", "Drug-induced tremor due to sertraline (Enhanced physiologic tremor)", "Hepatic injury", "Trigeminal neuralgia"]
Comorbidities: ["Major depressive disorder", "Right optic neuropathy", "Multiple sclerosis"]

## Encounter 0: source 2020-01-15, practice 2021-08-11, outpatient, Neurology

Chief complaint: Intermittent numbness and tingling in left arm and leg with episodes of blurred vision

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent numbness and tingling in left arm and leg with episodes of blurred vision

HISTORY OF PRESENT ILLNESS:
She also reports two episodes of blurred vision in her right eye that resolved spontaneously after a few days. The patient is anxious about her symptoms and seeks further evaluation.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple sclerosis
- Hashimoto's thyroiditis
- Asthma
- Major depressive disorder

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of pelvis following traumatic fracture (2 years prior)

MEDICATIONS:
Home Medications:
- Levothyroxine 100 mcg daily
- Albuterol HFA inhaler 90 mcg (1-2 inhalations) every 4-6 hours as needed for asthma symptoms
- Sertraline 50 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hashimoto's thyroiditis
- Father: Major depressive disorder
- Maternal aunt: Multiple sclerosis

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (2-3 drinks per week). Occupation: Marketing Manager.

PHYSICAL EXAMINATION:
On examination, you note hyperreflexia in the left upper and lower extremities and an intention tremor in the left hand. Visual acuity is normal, but there is a relative afferent pupillary defect in the right eye.

LABORATORY DATA:
Laboratory Values:
- WBC: 7,000/uL (normal range: 4,500-11,000/uL)
- Hemoglobin: 13.5 g/dL (normal range: 12-16 g/dL)
- Platelets: 250,000/uL (normal range: 150,000-450,000/uL)
- Glucose: 95 mg/dL (normal range: 70-100 mg/dL)
- Vitamin B12: 400 pg/mL (normal range: 200-900 pg/mL)

IMAGING:
An MRI of the brain with and without contrast shows several periventricular white matter lesions, but they are not diagnostic of multiple sclerosis.

## Encounter 1: source 2020-04-15, practice 2021-11-08, outpatient, Psychiatry

Chief complaint: Follow-up for depression with intermittent numbness, fatigue, and balance difficulty

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up for depression with intermittent numbness, fatigue, and balance difficulty

HISTORY OF PRESENT ILLNESS:
The patient is a 35-year-old female with multiple sclerosis diagnosed in January 2020, Hashimoto's thyroiditis, asthma, and major depressive disorder who returns to the psychiatry clinic for follow-up. She presents with a 6-month history of intermittent numbness and tingling in her hands and feet, fatigue, and difficulty with balance. She also reports episodes of blurred vision in her right eye that resolved spontaneously after a few weeks. She also mentions feeling "down" and having less interest in her usual activities for the past three months.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple sclerosis (diagnosed 2020-01-15)
- Hashimoto's thyroiditis
- Asthma
- Major depressive disorder

She has a past medical history of Hashimoto's thyroiditis.

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of pelvis following traumatic fracture (2 years prior)

MEDICATIONS:
She is currently taking levothyroxine.

Home Medications:
- Albuterol HFA inhaler 90 mcg (1-2 inhalations) every 4-6 hours as needed for asthma symptoms
- Sertraline 50 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hashimoto's thyroiditis
- Father: Major depressive disorder
- Maternal aunt: Multiple sclerosis

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (2-3 drinks per week). Occupation: Marketing Manager.

REVIEW OF SYSTEMS:
She denies any bowel or bladder incontinence.

PHYSICAL EXAMINATION:
On examination, you note nystagmus on lateral gaze, decreased vibratory sense in the lower extremities, and hyperreflexia.

LABORATORY DATA:
Her laboratory values are as follows:

- Hemoglobin: 13.2 g/dL
- White blood cell count: 7,000/uL
- Thyroid-stimulating hormone: 2.5 mIU/L
- Vitamin B12: 400 pg/mL
- Rapid plasma reagin (RPR): non-reactive

IMAGING:
MRI of the brain with and without contrast shows multiple periventricular white matter lesions.

## Encounter 2: source 2020-10-15, practice 2022-05-16, outpatient, Neurology

Chief complaint: Intermittent numbness and tingling in left arm and leg with recurrent blurred vision

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent numbness and tingling in left arm and leg with recurrent blurred vision

HISTORY OF PRESENT ILLNESS:
The patient is a 35-year-old woman with multiple sclerosis, Hashimoto's thyroiditis, asthma, and major depressive disorder returning for Neurology follow-up. She presents with a 6-month history of intermittent numbness and tingling in her left arm and leg. She also reports two episodes of blurred vision in her right eye that resolved spontaneously after a few weeks.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple sclerosis (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-04-15)
- Hashimoto's thyroiditis
- Asthma

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of pelvis following traumatic fracture (2 years prior)

MEDICATIONS:
Home Medications:
- Levothyroxine 100 mcg daily
- Albuterol HFA inhaler 90 mcg (1-2 inhalations) every 4-6 hours as needed for asthma symptoms
- Sertraline 50 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Mother had an autoimmune disease

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (2-3 drinks per week). Occupation: Marketing Manager.

PHYSICAL EXAMINATION:
Hyperreflexia in the left upper and lower extremities and an intention tremor in the left hand. Visual acuity is normal, but there is a relative afferent pupillary defect in the right eye.

LABORATORY DATA:
Routine blood tests are normal, including complete blood count, erythrocyte sedimentation rate, and vitamin B12 levels.

IMAGING:
MRI of the brain with and without contrast

## Encounter 3: source 2021-04-15, practice 2022-10-31, outpatient, Neurology

Chief complaint: One-week history of double vision worsening with left gaze

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. James Okafor, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
One-week history of double vision worsening with left gaze

HISTORY OF PRESENT ILLNESS:
This 36-year-old female with established multiple sclerosis returns for evaluation of new visual symptoms, following prior neurology visits in 2020 for intermittent numbness and blurred vision. She reports that the double vision worsens when looking to the left and improves when closing either eye. She also mentions occasional episodes of tingling in her hands and feet over the past few months.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple sclerosis (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-04-15)
- Hashimoto's thyroiditis
- Asthma

Her past medical history is significant for an episode of optic neuritis two years ago, which resolved without intervention.

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of pelvis following traumatic fracture (2 years prior)

MEDICATIONS:
Home Medications:
- Levothyroxine 100 mcg daily
- Albuterol HFA inhaler 90 mcg (1-2 inhalations) every 4-6 hours as needed for asthma symptoms
- Sertraline 50 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hashimoto's thyroiditis
- Father: Major depressive disorder
- Maternal aunt: Multiple sclerosis

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (2-3 drinks per week). Occupation: Marketing Manager.

PHYSICAL EXAMINATION:
On examination, when asked to look to the left, her right eye fails to adduct and the left eye exhibits nystagmus. Her visual acuity is 20/20 in both eyes, and the rest of the cranial nerve examination is normal. Sensation is intact, but she has hyperreflexia in both lower extremities.

IMAGING:
MRI of the brain with gadolinium contrast shows multiple periventricular white matter lesions.

## Encounter 4: source 2021-10-15, practice 2023-05-03, ed, Emergency Medicine

Chief complaint: Sudden onset right leg weakness and numbness with blurred vision

ENCOUNTER NOTE
Date: 2021-10-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset right leg weakness and numbness with blurred vision

HISTORY OF PRESENT ILLNESS:
The patient with multiple sclerosis and prior visual disturbances presents acutely. She also reports blurred vision in her left eye that started a week ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple sclerosis (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-04-15)
- Hashimoto's thyroiditis
- Asthma

Her past medical history is significant for an episode of optic neuritis three years ago, which resolved spontaneously.

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of pelvis following traumatic fracture (2 years prior)

MEDICATIONS:
She does not take any medications and has no other significant medical history.

Home Medications:
- Levothyroxine 100 mcg daily
- Albuterol HFA inhaler 90 mcg (1-2 inhalations) every 4-6 hours as needed for asthma symptoms
- Sertraline 50 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hashimoto's thyroiditis
- Father: Major depressive disorder
- Maternal aunt: Multiple sclerosis

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (2-3 drinks per week). Occupation: Marketing Manager.

PHYSICAL EXAMINATION:
On examination, her right lower extremity shows decreased muscle strength (3/5) and reduced sensation to light touch.

LABORATORY DATA:
Her cerebrospinal fluid analysis reveals oligoclonal bands.

IMAGING:
An MRI of the brain with and without contrast shows multiple periventricular white matter lesions, some of which enhance with gadolinium.

ASSESSMENT AND PLAN:
She is diagnosed with an acute exacerbation of multiple sclerosis

She is started on intravenous methylprednisolone. After 5 days of treatment, there is no improvement in her symptoms.

## Encounter 5: source 2022-07-15, practice 2024-02-05, outpatient, Neurology

Chief complaint: Six-month history of severe stabbing facial pain triggered by chewing

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David Kim, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of severe stabbing facial pain triggered by chewing

HISTORY OF PRESENT ILLNESS:
The patient is a 37-year-old female with multiple sclerosis followed here who presents with six months of facial pain triggered by chewing and sometimes by light touch, lasting a few seconds to a couple of minutes in her cheeks and jaw. She has a history of fatigue and occasional blurred vision she attributed to her demanding job.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple sclerosis (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-04-15)
- Hashimoto's thyroiditis
- Asthma

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of pelvis following traumatic fracture (2 years prior)

MEDICATIONS:
Home Medications:
- Levothyroxine 100 mcg daily
- Albuterol HFA inhaler 90 mcg (1-2 inhalations) every 4-6 hours as needed for asthma symptoms
- Sertraline 50 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is significant for an aunt with multiple sclerosis (MS).

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (2-3 drinks per week). Occupation: Marketing Manager.

PHYSICAL EXAMINATION:
On examination, light touch to the maxillary and mandibular regions triggers the facial pain. Neurological examination reveals mild weakness in her left leg and hyperreflexia.

IMAGING:
MRI of the brain with and without contrast shows multiple periventricular white matter lesions.

## Encounter 6: source 2023-07-15, practice 2025-02-03, outpatient, Internal Medicine

Chief complaint: Three-week history of hand tremors worse with writing and holding objects

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Maria Gonzalez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Three-week history of hand tremors worse with writing and holding objects

HISTORY OF PRESENT ILLNESS:
The patient, with multiple sclerosis and depression, returns with new hand tremors. She reports that the tremors are worse when she is trying to write or hold a cup, but they do not occur at rest. She denies any history of head injury,

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple sclerosis (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-04-15)
- Trigeminal neuralgia (diagnosed 2022-07-15)
- Hashimoto's thyroiditis
- Asthma

She has a past medical history of asthma.

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of pelvis following traumatic fracture (2 years prior)

MEDICATIONS:
She takes for which she uses an albuterol inhaler and recently started treatment for depression with sertraline.

Home Medications:
- Levothyroxine 100 mcg daily
- Albuterol HFA inhaler 90 mcg (1-2 inhalations) every 4-6 hours as needed for asthma symptoms

ALLERGIES:
NKDA

FAMILY HISTORY:
Family history of tremors.

SOCIAL HISTORY:
Alcohol use,

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, you observe a fine, rapid tremor of her outstretched hands, which increases when she attempts to touch her nose with her index finger.

LABORATORY DATA:
Laboratory values:
- Complete blood count: Normal
- Thyroid-stimulating hormone: Normal
- Electrolytes: Normal
- Blood glucose: Normal

## Encounter 7: source 2024-07-15, practice 2026-02-04, ed, Emergency Medicine

Chief complaint: Sudden severe abdominal pain with progressive worsening

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Robert Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden severe abdominal pain with progressive worsening

HISTORY OF PRESENT ILLNESS:
The patient is a 39-year-old female with multiple sclerosis, Hashimoto's thyroiditis, asthma, and major depressive disorder with prior medication-related adverse effects who presents to the emergency department. She appears anxious and has difficulty maintaining eye contact. She reports that the pain started suddenly the night before and has progressively worsened. She denies any nausea, vomiting, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple sclerosis (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-04-15)
- Trigeminal neuralgia (diagnosed 2022-07-15)
- Drug-induced tremor due to sertraline (Enhanced physiologic tremor) (diagnosed 2023-07-15)
- Hashimoto's thyroiditis
- Asthma

Her past medical history is significant for depression and a previous hospitalization for a pelvic fracture from a "fall down the stairs" two years ago.

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of pelvis following traumatic fracture (2 years prior)

MEDICATIONS:
Home Medications:
- Levothyroxine 100 mcg daily
- Albuterol HFA inhaler 90 mcg (1-2 inhalations) every 4-6 hours as needed for asthma symptoms
- Sertraline 50 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hashimoto's thyroiditis
- Father: Major depressive disorder
- Maternal aunt: Multiple sclerosis

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (2-3 drinks per week). Occupation: Marketing Manager.

PHYSICAL EXAMINATION:
On physical examination, you note multiple bruises of varying ages on her arms and a single, fresh-looking bruise on her abdomen. Her abdominal exam reveals tenderness in the right upper quadrant with guarding but no rebound tenderness.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin: 12.5 g/dL
- White blood cell count: 9,000 /µL
- Platelets: 250,000 /µL
- Aspartate aminotransferase (AST): 150 U/L
- Alanine aminotransferase (ALT): 125 U/L
- Alkaline phosphatase: 100 U/L
- Amylase: 50 U/L
- Lipase: 45 U/L
