# synthetic_hospital patient 1801

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 33 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-03-28 (Thu) | outpatient | Neurology |
| 1 | 2020-07-15 | 2024-09-26 (Thu) | outpatient | Urology |
| 2 | 2021-01-15 | 2025-04-03 (Thu) | outpatient | Urology |
| 3 | 2022-01-15 | 2026-03-27 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Social (1-2 glasses of wine per week)",
 "family_history": [
  "Maternal aunt with multiple sclerosis",
  "Mother with autoimmune hypothyroidism"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "20 mg subcutaneous daily",
   "name": "Glatiramer acetate"
  }
 ],
 "surgical_history": [
  "Wisdom teeth extraction"
 ],
 "chronic_conditions": [
  "Multiple sclerosis (relapsing-remitting)",
  "Neurogenic detrusor overactivity",
  "Spasticity"
 ],
 "age_at_first_encounter": 33
}
```

Primary diagnoses: ["Neurogenic detrusor overactivity (Urge incontinence secondary to multiple sclerosis)", "Multiple sclerosis", "Medial longitudinal fasciculus lesion"]
Comorbidities: ["Toxic optic neuropathy", "Multiple sclerosis"]

## Encounter 0: source 2020-01-15, practice 2024-03-28, outpatient, Neurology

Chief complaint: 2-week history of progressive difficulty coordinating movements, frequent stumbling, and increased muscle spasms

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
2-week history of progressive difficulty coordinating movements, frequent stumbling, and increased muscle spasms

HISTORY OF PRESENT ILLNESS:
She reports a remote episode of sudden vision loss in her right eye 4 years ago, which gradually resolved over 3 months. She experiences increased muscle tone and painful spasms interfering with her mobility. Urinary urgency is also reported, but she has not experienced incontinence.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple sclerosis (relapsing-remitting)
- Neurogenic detrusor overactivity
- Spasticity

PAST SURGICAL HISTORY:
- Wisdom teeth extraction

MEDICATIONS:
Home Medications:
- Glatiramer acetate 20 mg subcutaneous daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal aunt with multiple sclerosis
- Mother with autoimmune hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On neurological examination, she demonstrates intention tremor, a wide-based unsteady gait, and increased tone with hyperreflexia in her lower limbs. There is evidence of past pointing on finger-to-nose testing. Ophthalmologic exam reveals reduced visual acuity and optic disc pallor in the right eye. Sensation to pinprick is mildly diminished in both lower extremities.

LABORATORY DATA:
Laboratory studies are notable for mild elevation in IgG index in cerebrospinal fluid.

IMAGING:
MRI of the brain shows multifocal ovoid lesions involving the periventricular white matter and cerebellar peduncles.

## Encounter 1: source 2020-07-15, practice 2024-09-26, outpatient, Urology

Chief complaint: 6-month history of increased urinary frequency, urgency, and occasional incontinence

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of increased urinary frequency, urgency, and occasional incontinence

HISTORY OF PRESENT ILLNESS:
The patient with relapsing-remitting multiple sclerosis, last seen in Neurology six months prior for gait difficulty and muscle spasms, presents to Urology clinic with a 6-month history of increased frequency and urgency of urination. She reports that these symptoms are often accompanied by a sudden, overwhelming need to urinate, which has resulted in occasional incontinence. She denies dysuria or hematuria.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple sclerosis (diagnosed 2020-01-15)
- Multiple sclerosis (relapsing-remitting)
- Neurogenic detrusor overactivity
- Spasticity

Her past medical history is significant for episodic numbness and tingling in her lower extremities, blurred vision in her left eye that resolved spontaneously after a few weeks, and a recent diagnosis of optic neuritis.

PAST SURGICAL HISTORY:
- Wisdom teeth extraction

MEDICATIONS:
Home Medications:
- Glatiramer acetate 20 mg subcutaneous daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal aunt with multiple sclerosis
- Mother with autoimmune hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, she has a normal abdominal exam, and her neurological examination reveals hyperreflexia in the lower extremities and an extensor plantar response on the left.

LABORATORY DATA:
Urinalysis is unremarkable.

IMAGING:
MRI of the brain with and without contrast shows multiple periventricular white matter lesions.

## Encounter 2: source 2021-01-15, practice 2025-04-03, outpatient, Urology

Chief complaint: Persistent urinary symptoms gradually worsening over 6 months with daily urge incontinence

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Marcus Johnson, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent urinary symptoms gradually worsening over 6 months with daily urge incontinence

HISTORY OF PRESENT ILLNESS:
The patient is a 34-year-old female with relapsing-remitting multiple sclerosis initially evaluated by Urology in July 2020 for urinary frequency and urgency. She returns six months later reporting persistent urinary symptoms that have gradually worsened since her prior visit. She reports feeling a sudden urge to urinate multiple times per day, and she often leaks urine before reaching the bathroom. She describes the episodes as unpredictable and sometimes occurring with minimal bladder fullness. She denies dysuria, hematuria, or suprapubic pain. She has not tried any medications for her bladder symptoms yet. The patient expresses frustration, noting that her incontinence has begun to affect her social activities. She asks what can be done to help her manage these symptoms over the long term.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple sclerosis (diagnosed 2020-01-15)
- Multiple sclerosis (relapsing-remitting)
- Neurogenic detrusor overactivity
- Spasticity

Her medical history is significant for multiple sclerosis diagnosed 3 years ago. She has had two relapses in the past year but currently has no motor weakness or sensory deficits.

PAST SURGICAL HISTORY:
- Wisdom teeth extraction

MEDICATIONS:
Glatiramer acetate

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal aunt with multiple sclerosis
- Mother with autoimmune hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
Physical examination reveals normal strength and tone in the lower extremities, intact perineal sensation, and no spinal tenderness. Deep tendon reflexes are brisk and symmetric.

LABORATORY DATA:
Urinalysis is unremarkable.

IMAGING:
MRI of the spine performed three months ago showed scattered demyelinating lesions in the cervical and thoracic cord.

OTHER STUDIES:
A postvoid residual urine volume by bladder scan is 40 mL. Urodynamic testing reveals detrusor overactivity with intact sphincter relaxation.

## Encounter 3: source 2022-01-15, practice 2026-03-27, ed, Emergency Medicine

Chief complaint: Acute onset double vision and difficulty focusing while reading

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Emily Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset double vision and difficulty focusing while reading

HISTORY OF PRESENT ILLNESS:
The patient is a 35-year-old female with relapsing-remitting multiple sclerosis and neurogenic detrusor overactivity who presents with double vision and difficulty focusing while reading. She mentions a past episode of numbness in both legs and, more recently, transient right arm weakness, both of which resolved without treatment.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple sclerosis (diagnosed 2020-01-15)
- Neurogenic detrusor overactivity (Urge incontinence secondary to multiple sclerosis) (diagnosed 2021-01-15)
- Multiple sclerosis (relapsing-remitting)
- Neurogenic detrusor overactivity
- Spasticity

PAST SURGICAL HISTORY:
- Wisdom teeth extraction

MEDICATIONS:
Home Medications:
- Glatiramer acetate 20 mg subcutaneous daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal aunt with multiple sclerosis
- Mother with autoimmune hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On neurologic examination, her left eye fails to adduct during rightward gaze, while the right eye abducts with nystagmus. Convergence is preserved, and pupils are equal and reactive to light. Vertical gaze and the remainder of her cranial nerve exam are normal.
