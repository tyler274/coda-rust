# synthetic_hospital patient 2270

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 72 at first encounter, sex M, Non-Hispanic White, Medicare with supplemental Medigap policy.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2018-12-31 (Mon) | outpatient | Otolaryngology |
| 1 | 2021-01-15 | 2020-01-10 (Fri) | outpatient | Cardiology |
| 2 | 2022-07-15 | 2021-07-07 (Wed) | outpatient | Neurology |
| 3 | 2024-01-15 | 2023-01-02 (Mon) | outpatient | Urology |
| 4 | 2025-01-15 | 2024-01-03 (Wed) | ed | Emergency Medicine |
| 5 | 2026-01-15 | 2024-12-27 (Fri) | outpatient | Gastroenterology |
| 6 | 2026-07-15 | 2025-06-26 (Thu) | ed | Emergency Medicine |
| 7 | 2027-07-15 | 2026-06-27 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [],
 "insurance": "Medicare with supplemental Medigap policy",
 "occupation": "Retired accountant",
 "alcohol_use": "History of alcohol use disorder, currently sober for 2 years",
 "family_history": [
  "Father with coronary artery disease and aortic stenosis",
  "Mother with Alzheimer disease",
  "Older brother with prostate cancer"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "25 mg oral daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "500 mg oral twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg oral at bedtime",
   "name": "Atorvastatin"
  },
  {
   "dose": "10 mg oral daily as needed for allergies",
   "name": "Loratadine"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 diabetes mellitus",
  "Hyperlipidemia",
  "Cervical spondylosis",
  "Seasonal allergic rhinitis",
  "History of cerebrovascular accident",
  "Alcohol use disorder"
 ],
 "age_at_first_encounter": 72
}
```

Primary diagnoses: ["Aortic valve stenosis", "Zenker diverticulum", "Prostate Cancer (Suspected)", "Multi-infarct dementia", "Presbycusis", "Acute subdural hematoma", "Urinary retention", "Central cord syndrome"]
Comorbidities: ["Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2018-12-31, outpatient, Otolaryngology

Chief complaint: Gradual difficulty hearing, especially in noisy environments, over the past few years.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Gradual difficulty hearing, especially in noisy environments, over the past few years.

HISTORY OF PRESENT ILLNESS:
He reports that this problem has been gradually worsening over the past few years. He denies any ear pain, discharge, or vertigo. He has no history of noise exposure or ototoxic medication use.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Cervical spondylosis
- Seasonal allergic rhinitis
- History of cerebrovascular accident
- Alcohol use disorder

His past medical history is significant for hypertension.

MEDICATIONS:
He takes for which he takes hydrochlorothiazide.

Home Medications:
- Metformin 500 mg oral twice daily
- Atorvastatin 20 mg oral at bedtime
- Loratadine 10 mg oral daily as needed for allergies

ALLERGIES:
No known drug allergies (NKDA)

FAMILY HISTORY:
- Father with coronary artery disease and aortic stenosis
- Mother with Alzheimer disease
- Older brother with prostate cancer

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: History of alcohol use disorder, currently sober for 2 years. Occupation: Retired accountant.

PHYSICAL EXAMINATION:
On physical examination, you note that the patient has difficulty hearing whispered words at a distance of 2 feet. Weber test shows no lateralization, and Rinne test indicates air conduction is better than bone conduction in both ears.

OTHER STUDIES:
Audiometry reveals a bilateral high-frequency sensorineural hearing loss.

## Encounter 1: source 2021-01-15, practice 2020-01-10, outpatient, Cardiology

Chief complaint: Three-month history of exertional breathlessness, fatigue when climbing stairs, and occasional lightheadedness while walking uphill.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of exertional breathlessness, fatigue when climbing stairs, and occasional lightheadedness while walking uphill.

HISTORY OF PRESENT ILLNESS:
The patient with hypertension, diabetes mellitus, and history of cerebrovascular accident presents one year after presbycusis evaluation. He reports mild chest pain and mentions needing to rest more often during his daily routine.

PAST MEDICAL HISTORY:
Active Problem List:
- Presbycusis (diagnosed 2020-01-15)
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Cervical spondylosis
- Seasonal allergic rhinitis
- History of cerebrovascular accident
- Alcohol use disorder

He has a history of well-controlled hypertension.

MEDICATIONS:
He takes a thiazide diuretic.

Home Medications:
- Hydrochlorothiazide 25 mg oral daily
- Metformin 500 mg oral twice daily
- Atorvastatin 20 mg oral at bedtime
- Loratadine 10 mg oral daily as needed for allergies

ALLERGIES:
No known drug allergies (NKDA)

FAMILY HISTORY:
- Father with coronary artery disease and aortic stenosis
- Mother with Alzheimer disease
- Older brother with prostate cancer

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: History of alcohol use disorder, currently sober for 2 years. Occupation: Retired accountant.

PHYSICAL EXAMINATION:
On physical examination, his pulse is slow and regular, and auscultation reveals a soft second heart sound and a harsh, late-peaking systolic murmur best heard at the upper right sternal border while sitting upright. Carotid pulses rise slowly and are diminished in amplitude.

IMAGING:
Echocardiography shows a heavily calcified aortic valve with reduced excursion of the leaflets and a left ventricular ejection fraction of 60%.

## Encounter 2: source 2022-07-15, practice 2021-07-07, outpatient, Neurology

Chief complaint: Brought to clinic by his daughter due to worsening forgetfulness and personality changes over the past 2 months.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Priya Patel, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Brought to clinic by his daughter due to worsening forgetfulness and personality changes over the past 2 months.

HISTORY OF PRESENT ILLNESS:
The patient presents with cognitive symptoms in the context of prior cerebrovascular accident and aortic stenosis. His daughter reports he struggles with tasks previously performed easily, such as organizing bills and cooking. He recently left the stove on unattended and became lost while driving to his local grocery store.

PAST MEDICAL HISTORY:
Active Problem List:
- Presbycusis (diagnosed 2020-01-15)
- Aortic valve stenosis (diagnosed 2021-01-15)
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Cervical spondylosis
- Seasonal allergic rhinitis
- History of cerebrovascular accident
- Alcohol use disorder

He has a history of type 2 diabetes mellitus, hyperlipidemia, and long-standing hypertension. He had a small stroke 6 months ago but recovered well with only mild left-sided weakness.

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg oral daily
- Metformin 500 mg oral twice daily
- Atorvastatin 20 mg oral at bedtime
- Loratadine 10 mg oral daily as needed for allergies

ALLERGIES:
No known drug allergies (NKDA)

FAMILY HISTORY:
- Father with coronary artery disease and aortic stenosis
- Mother with Alzheimer disease
- Older brother with prostate cancer

SOCIAL HISTORY:
He lives alone but now needs regular supervision. He does not drink or smoke.

VITAL SIGNS:
On examination, his blood pressure is 150/88 mm Hg and pulse is 76/min.

PHYSICAL EXAMINATION:
Cranial nerves are intact. Strength is 3/5 in the left upper and lower limbs. Reflexes are brisk on the left, and plantar response is upgoing on that side. His Mini-Mental State Examination score is 24/30. He is able to recall one of three objects after five minutes.

## Encounter 3: source 2024-01-15, practice 2023-01-02, outpatient, Urology

Chief complaint: Progressive urinary difficulties including straining to initiate urination, interrupted flow, nocturia, and unintentional weight loss.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. James O'Brien, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive urinary difficulties including straining to initiate urination, interrupted flow, nocturia, and unintentional weight loss.

HISTORY OF PRESENT ILLNESS:
The patient is a 76-year-old male with hypertension, type 2 diabetes, history of cerebrovascular accident, and multi-infarct dementia who presents with urinary symptoms. He describes needing to strain to initiate urination, interrupted flow, and waking several times at night to void. He has also noted dull discomfort in the lower abdomen but denies flank pain, hematuria, or fever. His appetite is intact, though he reports some unintentional weight loss over the past few months.

PAST MEDICAL HISTORY:
Active Problem List:
- Presbycusis (diagnosed 2020-01-15)
- Aortic valve stenosis (diagnosed 2021-01-15)
- Multi-infarct dementia (diagnosed 2022-07-15)
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Cervical spondylosis
- Seasonal allergic rhinitis
- History of cerebrovascular accident
- Alcohol use disorder

He has a history of hypertension.

PAST SURGICAL HISTORY:
No prior urologic surgeries.

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg oral daily
- Metformin 500 mg oral twice daily
- Atorvastatin 20 mg oral at bedtime
- Loratadine 10 mg oral daily as needed for allergies

ALLERGIES:
No known drug allergies (NKDA)

FAMILY HISTORY:
- Father with coronary artery disease and aortic stenosis
- Mother with Alzheimer disease
- Older brother with prostate cancer

SOCIAL HISTORY:
He does not smoke and drinks alcohol occasionally.

PHYSICAL EXAMINATION:
On physical examination, his abdomen is soft with no palpable masses. Rectal examination reveals an irregular, hard prostate with nodularity.

LABORATORY DATA:
Serum prostate-specific antigen is markedly elevated,

IMAGING:
Imaging demonstrates extension of the tumor beyond the prostatic capsule into an adjacent pelvic structure.

PATHOLOGY:
Biopsy confirms adenocarcinoma.

## Encounter 4: source 2025-01-15, practice 2024-01-03, ed, Emergency Medicine

Chief complaint: Acute lower abdominal pain and complete inability to pass urine for 24 hours.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute lower abdominal pain and complete inability to pass urine for 24 hours.

HISTORY OF PRESENT ILLNESS:
This 77-year-old male previously evaluated in urology one year ago for progressive urinary difficulties and suspected prostate cancer presents to the emergency department with a 24-hour history of lower abdominal pain and inability to pass urine. He reports having had increasing difficulty with urination over the past year, including a weak stream and nocturia. He mentions that he started taking an over-the-counter medication for his allergies two days ago, which he heard could help with his symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Presbycusis (diagnosed 2020-01-15)
- Aortic valve stenosis (diagnosed 2021-01-15)
- Multi-infarct dementia (diagnosed 2022-07-15)
- Prostate Cancer (Suspected) (diagnosed 2024-01-15)
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Cervical spondylosis
- Seasonal allergic rhinitis
- History of cerebrovascular accident
- Alcohol use disorder

His past medical history includes hypertension, for which he takes hydrochlorothiazide, and seasonal allergic rhinitis.

MEDICATIONS:
He takes hydrochlorothiazide; an over-the-counter medication for his allergies.

Home Medications:
- Metformin 500 mg oral twice daily
- Atorvastatin 20 mg oral at bedtime
- Loratadine 10 mg oral daily as needed for allergies

ALLERGIES:
No known drug allergies (NKDA)

FAMILY HISTORY:
- Father with coronary artery disease and aortic stenosis
- Mother with Alzheimer disease
- Older brother with prostate cancer

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: History of alcohol use disorder, currently sober for 2 years. Occupation: Retired accountant.

VITAL SIGNS:
Blood pressure is 145/90 mmHg, heart rate is 78 beats per minute

PHYSICAL EXAMINATION:
He appears uncomfortable. Abdominal examination reveals a distended bladder. Digital rectal examination shows an enlarged, smooth, and firm prostate. His mucous membranes appear dry. No neurological deficits are noted.

LABORATORY DATA:
Laboratory values:
- Serum creatinine: 1.2 mg/dL (normal: 0.6-1.3 mg/dL)
- Blood urea nitrogen (BUN): 19 mg/dL (normal: 7-20 mg/dL)
- Serum potassium: 4.5 mEq/L (normal: 3.5-5.0 mEq/L)
- Serum sodium: 140 mEq/L (normal: 135-145 mEq/L)

## Encounter 5: source 2026-01-15, practice 2024-12-27, outpatient, Gastroenterology

Chief complaint: Six-month history of intermittent dysphagia to solids, nocturnal regurgitation of undigested food, and halitosis.

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. Robert Kim, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of intermittent dysphagia to solids, nocturnal regurgitation of undigested food, and halitosis.

HISTORY OF PRESENT ILLNESS:
The patient is a 78-year-old male with a complex medical history including multi-infarct dementia, suspected prostate cancer, aortic valve stenosis, and a previous cerebrovascular accident. He presents to the Gastroenterology clinic with a 6-month history of intermittent dysphagia, predominantly to solids, and episodes of regurgitation of undigested food, especially at night. He also reports a sensation of a lump in his throat and bad breath. He has had two episodes of pneumonia in the past year, which were treated with antibiotics.

PAST MEDICAL HISTORY:
Active Problem List:
- Presbycusis (diagnosed 2020-01-15)
- Aortic valve stenosis (diagnosed 2021-01-15)
- Multi-infarct dementia (diagnosed 2022-07-15)
- Prostate Cancer (Suspected) (diagnosed 2024-01-15)
- Urinary retention (diagnosed 2025-01-15)
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Cervical spondylosis
- Seasonal allergic rhinitis
- History of cerebrovascular accident
- Alcohol use disorder

His medical history is significant for hypertension.

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Metformin 500 mg oral twice daily
- Atorvastatin 20 mg oral at bedtime
- Loratadine 10 mg oral daily as needed for allergies

ALLERGIES:
No known drug allergies (NKDA)

FAMILY HISTORY:
- Father with coronary artery disease and aortic stenosis
- Mother with Alzheimer disease
- Older brother with prostate cancer

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: History of alcohol use disorder, currently sober for 2 years. Occupation: Retired accountant.

PHYSICAL EXAMINATION:
On physical examination, a small, non-tender, and compressible mass is palpable in the left cervical region. No lymphadenopathy is noted.

IMAGING:
A barium swallow study is performed, and the results are pending.

## Encounter 6: source 2026-07-15, practice 2025-06-26, ed, Emergency Medicine

Chief complaint: Acute bilateral hand and arm weakness, clumsiness, and burning shoulder pain after falling and striking chin on bathroom sink.

ENCOUNTER NOTE
Date: 2026-07-15
Provider: Dr. Elena Vasquez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute bilateral hand and arm weakness, clumsiness, and burning shoulder pain after falling and striking chin on bathroom sink.

HISTORY OF PRESENT ILLNESS:
This 78-year-old male with cervical spondylosis, history of cerebrovascular accident, and multi-infarct dementia, evaluated six months prior for Zenker diverticulum, presents following a fall. He reports that he briefly lost his balance but did not lose consciousness. Since the fall, he has had difficulty moving his hands and arms and feels "clumsy" when trying to button his shirt. He also reports a burning sensation across his shoulders and upper arms. He denies neck pain, leg weakness, or bowel/bladder dysfunction.

PAST MEDICAL HISTORY:
Active Problem List:
- Presbycusis (diagnosed 2020-01-15)
- Aortic valve stenosis (diagnosed 2021-01-15)
- Multi-infarct dementia (diagnosed 2022-07-15)
- Prostate Cancer (Suspected) (diagnosed 2024-01-15)
- Urinary retention (diagnosed 2025-01-15)
- Zenker diverticulum (diagnosed 2026-01-15)
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Cervical spondylosis
- Seasonal allergic rhinitis
- History of cerebrovascular accident
- Alcohol use disorder

Past medical history includes long-standing hypertension and cervical spondylosis.

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg oral daily
- Metformin 500 mg oral twice daily
- Atorvastatin 20 mg oral at bedtime
- Loratadine 10 mg oral daily as needed for allergies

ALLERGIES:
No known drug allergies (NKDA)

FAMILY HISTORY:
- Father with coronary artery disease and aortic stenosis
- Mother with Alzheimer disease
- Older brother with prostate cancer

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: History of alcohol use disorder, currently sober for 2 years. Occupation: Retired accountant.

PHYSICAL EXAMINATION:
Neurologic exam reveals 4/5 strength in bilateral upper extremities, more pronounced distally than proximally. Lower extremity strength is 5/5. There is diminished pain and temperature sensation over the shoulders and arms bilaterally. Vibration and proprioception are intact throughout.

## Encounter 7: source 2027-07-15, practice 2026-06-27, ed, Emergency Medicine

Chief complaint: Found confused at home by neighbor after unwitnessed fall several days prior, with progressive headache and unsteady gait.

ENCOUNTER NOTE
Date: 2027-07-15
Provider: Dr. David Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Found confused at home by neighbor after unwitnessed fall several days prior, with progressive headache and unsteady gait.

HISTORY OF PRESENT ILLNESS:
This 79-year-old male with multi-infarct dementia and prior fall-related central cord syndrome presents to the emergency department after being found confused at home by his neighbor. He lives alone and reportedly fell several days ago but did not seek medical attention. His neighbor reports progressive headache, confusion, and unsteady gait over the past two days.

PAST MEDICAL HISTORY:
Active Problem List:
- Presbycusis (diagnosed 2020-01-15)
- Aortic valve stenosis (diagnosed 2021-01-15)
- Multi-infarct dementia (diagnosed 2022-07-15)
- Prostate Cancer (Suspected) (diagnosed 2024-01-15)
- Urinary retention (diagnosed 2025-01-15)
- Zenker diverticulum (diagnosed 2026-01-15)
- Central cord syndrome (diagnosed 2026-07-15)
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Cervical spondylosis
- Seasonal allergic rhinitis
- History of cerebrovascular accident
- Alcohol use disorder

The patient has a history of hypertension and alcohol use disorder.

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg oral daily
- Metformin 500 mg oral twice daily
- Atorvastatin 20 mg oral at bedtime
- Loratadine 10 mg oral daily as needed for allergies

ALLERGIES:
No known drug allergies (NKDA)

FAMILY HISTORY:
- Father with coronary artery disease and aortic stenosis
- Mother with Alzheimer disease
- Older brother with prostate cancer

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: History of alcohol use disorder, currently sober for 2 years. Occupation: Retired accountant.

PHYSICAL EXAMINATION:
On examination, he is drowsy but arousable, with mild left-sided weakness and diminished sensation. Pupils are equal and reactive to light. No signs of external trauma are noted.

IMAGING:
A head CT is performed, and findings on CT are shown below.
