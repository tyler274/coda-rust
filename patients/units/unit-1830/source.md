# synthetic_hospital patient 1830

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, Non-Hispanic White, Employer-sponsored (Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-09-01 (Fri) | outpatient | Neurology |
| 1 | 2020-09-15 | 2024-04-30 (Tue) | outpatient | Neurology |
| 2 | 2021-09-15 | 2025-04-26 (Sat) | ed | Emergency Medicine |
| 3 | 2022-09-15 | 2026-04-27 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Employer-sponsored (Blue Cross Blue Shield)",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Mother: Multiple sclerosis (diagnosed age 32)",
  "Father: Autoimmune hypothyroidism",
  "Maternal aunt: Rheumatoid arthritis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "20 mg subcutaneous daily",
   "name": "Glatiramer acetate"
  },
  {
   "dose": "2000 IU oral daily",
   "name": "Vitamin D3"
  }
 ],
 "surgical_history": [
  "Wisdom teeth extraction (age 18)"
 ],
 "chronic_conditions": [
  "Relapsing-Remitting Multiple Sclerosis"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Multiple sclerosis", "Internuclear Ophthalmoplegia", "Toxic optic neuropathy"]
Comorbidities: ["Multiple sclerosis"]

## Encounter 0: source 2020-01-15, practice 2023-09-01, outpatient, Neurology

Chief complaint: Worsening blurry vision and difficulty walking for the past two weeks, with history of intermittent right leg weakness and numbness over the past year that resolved spontaneously.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening blurry vision and difficulty walking for the past two weeks, with history of intermittent right leg weakness and numbness over the past year that resolved spontaneously.

HISTORY OF PRESENT ILLNESS:
She reports intermittent weakness and numbness in her right leg over the past year that resolved spontaneously.

PAST MEDICAL HISTORY:
Active Problem List:
- Relapsing-Remitting Multiple Sclerosis

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 18)

MEDICATIONS:
Home Medications:
- Glatiramer acetate 20 mg subcutaneous daily
- Vitamin D3 2000 IU oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is notable for an aunt with autoimmune thyroid disease.

SOCIAL HISTORY:
She lives in a northern climate

VITAL SIGNS:
Her temperature is 37.1°C (98.8°F), blood pressure is 118/72 mmHg, heart rate is 76/min, and respiratory rate is 14/min.

PHYSICAL EXAMINATION:
On examination, she has decreased visual acuity in the right eye, hyperreflexia, and an upgoing plantar reflex on the right.

IMAGING:
Imaging findings are shown in the picture below.

## Encounter 1: source 2020-09-15, practice 2024-04-30, outpatient, Neurology

Chief complaint: Double vision especially when looking to the left, following spontaneous resolution of fatigue and lower limb numbness a few weeks prior.

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Double vision especially when looking to the left, following spontaneous resolution of fatigue and lower limb numbness a few weeks prior.

HISTORY OF PRESENT ILLNESS:
She returns for follow-up of relapsing-remitting multiple sclerosis diagnosed January 2020. She also reports fatigue and lower limb numbness resolved spontaneously a few weeks ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple sclerosis (diagnosed 2020-01-15)
- Relapsing-Remitting Multiple Sclerosis

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 18)

MEDICATIONS:
Home Medications:
- Glatiramer acetate 20 mg subcutaneous daily
- Vitamin D3 2000 IU oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Multiple sclerosis (diagnosed age 32)
- Father: Autoimmune hypothyroidism
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On exam, when asked to look to the left, her right eye fails to adduct past the midline, while her left eye abducts with a horizontal jerk. Pupillary light reflexes and convergence are intact.

IMAGING:
MRI of the brain shows periventricular white matter hyperintensities.

## Encounter 2: source 2021-09-15, practice 2025-04-26, ed, Emergency Medicine

Chief complaint: Intermittent visual disturbances and difficulty focusing when shifting gaze, with double vision when looking to the left for the past few weeks.

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. James Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Intermittent visual disturbances and difficulty focusing when shifting gaze, with double vision when looking to the left for the past few weeks.

HISTORY OF PRESENT ILLNESS:
This 29-year-old female with relapsing-remitting multiple sclerosis returns with recurrent visual disturbances similar to her September 2020 presentation. She reports that, over the past few weeks, she has developed double vision when looking to the left side. The symptom is more noticeable when she tries to read or watch television. She denies headache, vertigo, photophobia, or hearing loss. She has no history of trauma or recent infection.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple sclerosis (diagnosed 2020-01-15)
- Internuclear Ophthalmoplegia (diagnosed 2020-09-15)
- Relapsing-Remitting Multiple Sclerosis

Her medical history is significant for episodic limb numbness and tingling over the past year, which spontaneously resolved each time.

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 18)

MEDICATIONS:
She does not take any regular medications

Home Medications:
- Glatiramer acetate 20 mg subcutaneous daily
- Vitamin D3 2000 IU oral daily

ALLERGIES:
She has no known allergies

FAMILY HISTORY:
Her family history is unremarkable.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On neurologic examination, when the patient is asked to look to the left, her left eye abducts normally with a conjugate saccade, but the right eye fails to adduct. When looking straight ahead or to the right, both eyes move symmetrically. Pupillary responses are normal bilaterally, and there is no ptosis or nystagmus in the resting gaze.

ASSESSMENT AND PLAN:
Damage to

MRI of the brain is scheduled to further evaluate the cause of her symptoms.

## Encounter 3: source 2022-09-15, practice 2026-04-27, ed, Emergency Medicine

Chief complaint: Progressive blurry vision in the right eye for 3 days accompanied by pain when moving the eye and difficulty distinguishing red hues.

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. James Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive blurry vision in the right eye for 3 days accompanied by pain when moving the eye and difficulty distinguishing red hues.

HISTORY OF PRESENT ILLNESS:
The patient with relapsing-remitting MS and internuclear ophthalmoplegia presents with right eye blurry vision accompanied by pain when moving the eye and difficulty distinguishing red hues. She also reports intermittent episodes of limb weakness over past year that resolved spontaneously.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple sclerosis (diagnosed 2020-01-15)
- Internuclear Ophthalmoplegia (diagnosed 2020-09-15)
- Relapsing-Remitting Multiple Sclerosis

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 18)

MEDICATIONS:
Home Medications:
- Glatiramer acetate 20 mg subcutaneous daily
- Vitamin D3 2000 IU oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Multiple sclerosis (diagnosed age 32)
- Father: Autoimmune hypothyroidism
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
Neurologic examination reveals decreased visual acuity in the right eye, a central scotoma, and an abnormal pupillary light reaction on the right side such that both pupils constrict less when light is directed into the right eye but constrict briskly when light is directed into the left eye. Strength, reflexes, and sensation are intact. Fundoscopic exam is unremarkable.
