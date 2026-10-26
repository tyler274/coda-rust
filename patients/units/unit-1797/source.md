# synthetic_hospital patient 1797

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, Non-Hispanic White, Private PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-08-22 (Mon) | outpatient | Neurology |
| 1 | 2020-07-15 | 2023-02-17 (Fri) | outpatient | Neurology |
| 2 | 2021-07-15 | 2024-02-20 (Tue) | ed | Emergency Medicine |
| 3 | 2022-07-15 | 2025-02-03 (Mon) | ed | Emergency Medicine |
| 4 | 2023-07-15 | 2026-02-17 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Sulfa drugs (rash)",
  "Contrast dye (hives)"
 ],
 "insurance": "Private PPO",
 "occupation": "Graphic Designer",
 "alcohol_use": "Social, 1-2 drinks per month",
 "family_history": [
  "Maternal aunt with hypothyroidism (autoimmune)",
  "Father with hypertension",
  "Mother with migraines"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "30 mcg IM weekly",
   "name": "Interferon beta-1a (Avonex)"
  },
  {
   "dose": "10 mg PO three times daily",
   "name": "Baclofen"
  },
  {
   "dose": "5 mg PO twice daily",
   "name": "Oxybutynin"
  },
  {
   "dose": "2000 IU PO daily",
   "name": "Vitamin D3"
  },
  {
   "dose": "400 mg PO as needed for pain",
   "name": "Ibuprofen"
  }
 ],
 "surgical_history": [
  "Tonsillectomy (age 8)"
 ],
 "chronic_conditions": [
  "Relapsing-remitting multiple sclerosis (diagnosed age 26)",
  "Neurogenic bladder with urinary urgency",
  "Lower extremity spasticity secondary to MS"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Optic Neuritis", "Multiple sclerosis"]
Comorbidities: ["Multiple sclerosis"]

## Encounter 0: source 2020-01-15, practice 2022-08-22, outpatient, Neurology

Chief complaint: Follow-up for increasing difficulty with stiffness and spasms in legs

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up for increasing difficulty with stiffness and spasms in legs

HISTORY OF PRESENT ILLNESS:
She describes the sensation as “tight and rigid,” especially in the mornings and after prolonged sitting.

PAST MEDICAL HISTORY:
Active Problem List:
- Relapsing-remitting multiple sclerosis (diagnosed age 26)
- Neurogenic bladder with urinary urgency
- Lower extremity spasticity secondary to MS

PAST SURGICAL HISTORY:
- Tonsillectomy (age 8)

MEDICATIONS:
Home Medications:
- Interferon beta-1a (Avonex) 30 mcg IM weekly
- Baclofen 10 mg PO three times daily
- Oxybutynin 5 mg PO twice daily
- Vitamin D3 2000 IU PO daily
- Ibuprofen 400 mg PO as needed for pain

ALLERGIES:
Sulfa drugs (rash), Contrast dye (hives)

FAMILY HISTORY:
- Maternal aunt with hypothyroidism (autoimmune)
- Father with hypertension
- Mother with migraines

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per month. Occupation: Graphic Designer.

PHYSICAL EXAMINATION:
Physical examination reveals spastic tone in both lower extremities and sustained ankle clonus.

PLAN:
She is started on baclofen to help manage her symptoms.

## Encounter 1: source 2020-07-15, practice 2023-02-17, outpatient, Neurology

Chief complaint: 3 days of right eye pain and blurred vision

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Rodriguez, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
3 days of right eye pain and blurred vision

HISTORY OF PRESENT ILLNESS:
The 28-year-old female with relapsing-remitting multiple sclerosis returns for new visual symptoms following her January visit for leg spasticity. She reports that the discomfort worsens with eye movement. Over the past few years, she has experienced brief episodes of leg weakness and unsteady walking that resolved without treatment. She also reports urinary urgency over the last several months.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple sclerosis (diagnosed 2020-01-15)
- Relapsing-remitting multiple sclerosis (diagnosed age 26)
- Neurogenic bladder with urinary urgency
- Lower extremity spasticity secondary to MS

PAST SURGICAL HISTORY:
- Tonsillectomy (age 8)

MEDICATIONS:
Home Medications:
- Interferon beta-1a (Avonex) 30 mcg IM weekly
- Baclofen 10 mg PO three times daily
- Oxybutynin 5 mg PO twice daily
- Vitamin D3 2000 IU PO daily
- Ibuprofen 400 mg PO as needed for pain

ALLERGIES:
Sulfa drugs (rash), Contrast dye (hives)

FAMILY HISTORY:
- Maternal aunt with hypothyroidism (autoimmune)
- Father with hypertension
- Mother with migraines

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per month. Occupation: Graphic Designer.

PHYSICAL EXAMINATION:
On physical exam, she has decreased visual acuity in the right eye, brisk lower extremity reflexes, and right-sided ankle clonus.

IMAGING:
An MRI of the brain shows multiple hyperintense periventricular white matter lesions.

## Encounter 2: source 2021-07-15, practice 2024-02-20, ed, Emergency Medicine

Chief complaint: 3-day history of decreased vision in right eye with pain on movement

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Emily Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
3-day history of decreased vision in right eye with pain on movement

HISTORY OF PRESENT ILLNESS:
The patient with relapsing-remitting multiple sclerosis presents with right eye complaints similar to her presentation in July 2020. She describes the vision loss as a "dark spot" in the center of her vision that has progressively worsened. She also reports pain when moving her right eye. She has not experienced any eye redness, discharge, or recent trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple sclerosis (diagnosed 2020-01-15)
- Relapsing-remitting multiple sclerosis (diagnosed age 26)
- Neurogenic bladder with urinary urgency
- Lower extremity spasticity secondary to MS

Her past medical history is significant for an episode of numbness and tingling in her left arm and leg two years ago, which resolved spontaneously.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 8)

MEDICATIONS:
Home Medications:
- Interferon beta-1a (Avonex) 30 mcg IM weekly
- Baclofen 10 mg PO three times daily
- Oxybutynin 5 mg PO twice daily
- Vitamin D3 2000 IU PO daily
- Ibuprofen 400 mg PO as needed for pain

ALLERGIES:
Sulfa drugs (rash), Contrast dye (hives)

FAMILY HISTORY:
- Maternal aunt with hypothyroidism (autoimmune)
- Father with hypertension
- Mother with migraines

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per month. Occupation: Graphic Designer.

PHYSICAL EXAMINATION:
On examination, visual acuity is 20/200 in the right eye and 20/20 in the left eye. There is a right relative afferent pupillary defect. Fundoscopic examination is normal. Color vision testing reveals a red desaturation in the right eye. The rest of the neurological examination is unremarkable.

## Encounter 3: source 2022-07-15, practice 2025-02-03, ed, Emergency Medicine

Chief complaint: One-week history of blurred vision in left eye and pain on eye movement with leg tingling

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
One-week history of blurred vision in left eye and pain on eye movement with leg tingling

HISTORY OF PRESENT ILLNESS:
This 30-year-old female with relapsing-remitting multiple sclerosis and prior episodes of right-sided optic neuritis presents to the emergency department with a one-week history of blurred vision in her left eye and pain on eye movement. She also reports a tingling sensation in her legs that has been progressing over the past two days.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple sclerosis (diagnosed 2020-01-15)
- Optic Neuritis (diagnosed 2021-07-15)
- Relapsing-remitting multiple sclerosis (diagnosed age 26)
- Neurogenic bladder with urinary urgency
- Lower extremity spasticity secondary to MS

PAST SURGICAL HISTORY:
- Tonsillectomy (age 8)

MEDICATIONS:
Home Medications:
- Interferon beta-1a (Avonex) 30 mcg IM weekly
- Baclofen 10 mg PO three times daily
- Oxybutynin 5 mg PO twice daily
- Vitamin D3 2000 IU PO daily
- Ibuprofen 400 mg PO as needed for pain

ALLERGIES:
Sulfa drugs (rash), Contrast dye (hives)

FAMILY HISTORY:
- Maternal aunt with hypothyroidism (autoimmune)
- Father with hypertension
- Mother with migraines

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per month. Occupation: Graphic Designer.

PHYSICAL EXAMINATION:
On examination, her left eye has an afferent pupillary defect, and visual acuity is reduced. Fundoscopic examination reveals a swollen optic disc. Strength is 5/5 in the upper extremities and 4/5 in the lower extremities, with decreased sensation to light touch and pinprick below the umbilicus. Reflexes are brisk, and Babinski sign is upgoing bilaterally.

IMAGING:
MRI of the brain and spinal cord with gadolinium shows multiple periventricular white matter lesions and an enhancing lesion in the spinal cord at the T6 level.

## Encounter 4: source 2023-07-15, practice 2026-02-17, ed, Emergency Medicine

Chief complaint: 5-day history of visual blurring in right eye

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
5-day history of visual blurring in right eye

HISTORY OF PRESENT ILLNESS:
This 31-year-old female with relapsing-remitting multiple sclerosis and prior episodes of optic neuritis returns with a five-day history of right eye visual symptoms. She reports difficulty seeing fine details, especially in bright environments.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple sclerosis (diagnosed 2020-01-15)
- Optic Neuritis (diagnosed 2021-07-15)
- Relapsing-remitting multiple sclerosis (diagnosed age 26)
- Neurogenic bladder with urinary urgency
- Lower extremity spasticity secondary to MS

She was diagnosed with relapsing-remitting multiple sclerosis two years ago.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 8)

MEDICATIONS:
Home Medications:
- Interferon beta-1a (Avonex) 30 mcg IM weekly
- Baclofen 10 mg PO three times daily
- Oxybutynin 5 mg PO twice daily
- Vitamin D3 2000 IU PO daily
- Ibuprofen 400 mg PO as needed for pain

ALLERGIES:
Sulfa drugs (rash), Contrast dye (hives)

FAMILY HISTORY:
- Maternal aunt with hypothyroidism (autoimmune)
- Father with hypertension
- Mother with migraines

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per month. Occupation: Graphic Designer.

PHYSICAL EXAMINATION:
On examination, both pupils are 6 mm in darkness and 4 mm in ambient light. When light is directed into the left eye, both pupils constrict to 2 mm. When the light is moved to the right eye, both pupils return to 4 mm. Extraocular movements are intact. No ptosis is noted.
