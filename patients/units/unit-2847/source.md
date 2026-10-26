# synthetic_hospital patient 2847

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 78 at first encounter, sex F, White, non-Hispanic, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-10-14 (Fri) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2023-01-16 (Mon) | outpatient | Neurology |
| 2 | 2020-09-15 | 2023-06-19 (Mon) | outpatient | Cardiology |
| 3 | 2021-03-15 | 2023-12-11 (Mon) | outpatient | Cardiology |
| 4 | 2021-09-15 | 2024-06-11 (Tue) | outpatient | Geriatrics |
| 5 | 2022-03-15 | 2024-12-14 (Sat) | ed | Emergency Medicine |
| 6 | 2022-09-15 | 2025-06-11 (Wed) | ed | Emergency Medicine |
| 7 | 2023-03-15 | 2025-12-03 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare",
 "occupation": "Retired high school teacher",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Alzheimer's disease (late onset)",
  "Father: Stroke and hypertension",
  "Sister: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg oral daily",
   "name": "Lisinopril"
  },
  {
   "dose": "50 mg oral daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "1000 mg oral twice daily",
   "name": "Metformin"
  },
  {
   "dose": "40 mg oral nightly",
   "name": "Atorvastatin"
  },
  {
   "dose": "10 mg oral nightly",
   "name": "Donepezil"
  },
  {
   "dose": "500 mg oral every 6 hours as needed for pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "Hypertension (poorly controlled)",
  "Type 2 diabetes mellitus",
  "Hyperlipidemia",
  "Osteoarthritis (multijoint)",
  "Mild cognitive impairment",
  "Aortic valve stenosis (severe)",
  "Atrial fibrillation (non-valvular)",
  "Neurosyphilis (treated)",
  "Cerebral amyloid angiopathy",
  "Orthostatic hypotension"
 ],
 "age_at_first_encounter": 78
}
```

Primary diagnoses: ["Cerebral amyloid angiopathy", "History of Falling", "Acute subdural hematoma", "Mild cognitive impairment", "Neurosyphilis", "Aortic valve stenosis", "Acute ischemic stroke", "Orthostatic hypotension"]
Comorbidities: ["Osteoarthritis", "Hyperlipidemia", "Essential hypertension", "Mild cognitive impairment"]

## Encounter 0: source 2020-01-15, practice 2022-10-14, outpatient, Internal Medicine

Chief complaint: Brought to outpatient clinic by her daughter due to concerns about recent memory lapses and difficulty recalling names over the past two years.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Brought to outpatient clinic by her daughter due to concerns about recent memory lapses and difficulty recalling names over the past two years.

HISTORY OF PRESENT ILLNESS:
Over the past two years, the patient has had increasing difficulty recalling names and recent conversations, although she remains able to manage her daily activities such as cooking, bathing, shopping, and paying bills. She no longer participates in her book club, stating she is "slower than the others," but she enjoys gardening and watching television.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension (poorly controlled)
- Type 2 diabetes mellitus
- Hyperlipidemia
- Osteoarthritis (multijoint)
- Mild cognitive impairment
- Aortic valve stenosis (severe)
- Atrial fibrillation (non-valvular)
- Neurosyphilis (treated)
- Cerebral amyloid angiopathy
- Orthostatic hypotension

Her past medical history includes osteoarthritis and hypertension.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
She takes managed with hydrochlorothiazide and acetaminophen.

Home Medications:
- Lisinopril 10 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Metformin 1000 mg oral twice daily
- Atorvastatin 40 mg oral nightly
- Donepezil 10 mg oral nightly

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Alzheimer's disease (late onset)
- Father: Stroke and hypertension
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
She does not drink alcohol and has never smoked.

REVIEW OF SYSTEMS:
She denies hallucinations, personality change, or fluctuations in cognition.

PHYSICAL EXAMINATION:
Neurologic exam is unremarkable. Her Mini-Mental State Examination (MMSE) score is 28/30. Physical exam reveals no abnormalities

LABORATORY DATA:
Lab tests including thyroid function and B12 are within normal limits.

## Encounter 1: source 2020-04-15, practice 2023-01-16, outpatient, Neurology

Chief complaint: Evaluation of progressive forgetfulness over the past 6 months with occasional misplacement of household items.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Okonkwo, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of progressive forgetfulness over the past 6 months with occasional misplacement of household items.

HISTORY OF PRESENT ILLNESS:
The patient is a 79-year-old woman with hypertension, diabetes, and prior evaluation for mild cognitive impairment who returns for reassessment of progressive cognitive decline. She is accompanied by her son to the Neurology clinic for evaluation of progressive forgetfulness over the past 6 months. Her son reports that she occasionally misplaces household items and has needed some assistance with paying bills, but she remains independent in basic activities of daily living.

PAST MEDICAL HISTORY:
Active Problem List:
- Mild cognitive impairment (diagnosed 2020-01-15)
- Hypertension (poorly controlled)
- Type 2 diabetes mellitus
- Hyperlipidemia
- Osteoarthritis (multijoint)
- Aortic valve stenosis (severe)
- Atrial fibrillation (non-valvular)
- Neurosyphilis (treated)
- Cerebral amyloid angiopathy
- Orthostatic hypotension

Her past medical history includes hypertension and osteoarthritis.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Metformin 1000 mg oral twice daily
- Atorvastatin 40 mg oral nightly
- Donepezil 10 mg oral nightly
- Acetaminophen 500 mg oral every 6 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Alzheimer's disease (late onset)
- Father: Stroke and hypertension
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired high school teacher.

REVIEW OF SYSTEMS:
She does not report any history of rash, ulcers, or sexually transmitted infections.

PHYSICAL EXAMINATION:
Cognitive screening reveals minor deficits with a Mini-Mental State Examination (MMSE) score of 24/30. She appears alert and engages appropriately during the interview.

LABORATORY DATA:
As part of the workup, a rapid plasma reagin (RPR) test returns positive at a titer of 1:4, and a confirmatory microhemagglutination assay for Treponema pallidum is also positive.

## Encounter 2: source 2020-09-15, practice 2023-06-19, outpatient, Cardiology

Chief complaint: Progressive exertional dyspnea and decreased exercise tolerance over 6 months with occasional chest tightness and dizziness when climbing stairs.

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Jennifer Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive exertional dyspnea and decreased exercise tolerance over 6 months with occasional chest tightness and dizziness when climbing stairs.

HISTORY OF PRESENT ILLNESS:
This 78-year-old female with severe aortic valve stenosis, mild cognitive impairment, and recently treated neurosyphilis presents with a 6-month history of progressive exertional dyspnea and decreased exercise tolerance. She also reports occasional chest tightness and dizziness, especially when climbing stairs.

PAST MEDICAL HISTORY:
Active Problem List:
- Mild cognitive impairment (diagnosed 2020-01-15)
- Neurosyphilis (diagnosed 2020-04-15)
- Hypertension (poorly controlled)
- Type 2 diabetes mellitus
- Hyperlipidemia
- Osteoarthritis (multijoint)
- Aortic valve stenosis (severe)
- Atrial fibrillation (non-valvular)
- Neurosyphilis (treated)
- Cerebral amyloid angiopathy
- Orthostatic hypotension

Her past medical history includes hypertension, hyperlipidemia, and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Metformin 1000 mg oral twice daily
- Atorvastatin 40 mg oral nightly
- Donepezil 10 mg oral nightly
- Acetaminophen 500 mg oral every 6 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Alzheimer's disease (late onset)
- Father: Stroke and hypertension
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
She has never smoked and does not consume alcohol.

VITAL SIGNS:
On physical examination, her blood pressure is 145/85 mmHg, heart rate is 78 beats per minute, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a harsh, crescendo-decrescendo systolic murmur best heard at the right second intercostal space that radiates to the carotids. There is no jugular venous distension, and her lungs are clear to auscultation.

IMAGING:
Echocardiography demonstrates a calcified aortic valve with restricted leaflet motion and a calculated aortic valve area of 0.9 cm².

OTHER STUDIES:
An electrocardiogram shows left ventricular hypertrophy.

## Encounter 3: source 2021-03-15, practice 2023-12-11, outpatient, Cardiology

Chief complaint: Recurrent dizziness and near-syncope episodes, particularly when standing from a sitting position, progressively worsening over the past year.

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Jennifer Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Recurrent dizziness and near-syncope episodes, particularly when standing from a sitting position, progressively worsening over the past year.

HISTORY OF PRESENT ILLNESS:
The patient returns for cardiology follow-up. She reports that these symptoms have been progressively worsening over the past year.

PAST MEDICAL HISTORY:
Active Problem List:
- Mild cognitive impairment (diagnosed 2020-01-15)
- Neurosyphilis (diagnosed 2020-04-15)
- Aortic valve stenosis (diagnosed 2020-09-15)
- Hypertension (poorly controlled)
- Type 2 diabetes mellitus
- Hyperlipidemia
- Osteoarthritis (multijoint)
- Atrial fibrillation (non-valvular)
- Neurosyphilis (treated)
- Cerebral amyloid angiopathy
- Orthostatic hypotension

Her past medical history is significant for hypertension,.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
She takes for which she takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Metformin 1000 mg oral twice daily
- Atorvastatin 40 mg oral nightly
- Donepezil 10 mg oral nightly
- Acetaminophen 500 mg oral every 6 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Alzheimer's disease (late onset)
- Father: Stroke and hypertension
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired high school teacher.

REVIEW OF SYSTEMS:
She denies any chest pain, palpitations, or loss of consciousness.

PHYSICAL EXAMINATION:
On physical examination, her blood pressure is 150/90 mmHg while seated and 120/70 mmHg upon standing, with an associated increase in heart rate from 70 to 90 beats per minute. Her neurological examination is unremarkable.

LABORATORY DATA:
Laboratory tests reveal:

- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 1.0 mg/dL
- Fasting glucose: 98 mg/dL
- Hemoglobin: 14 g/dL

## Encounter 4: source 2021-09-15, practice 2024-06-11, outpatient, Geriatrics

Chief complaint: Increasing forgetfulness and two recent falls at home, raising concerns about safety and functional decline.

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Emily Johnson, MD
Department: Geriatrics
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing forgetfulness and two recent falls at home, raising concerns about safety and functional decline.

HISTORY OF PRESENT ILLNESS:
Returning for follow-up with ongoing cognitive impairment and fall risk, she is brought to the clinic by her daughter due to concerns about her mother's increasing forgetfulness and two recent falls at home.

PAST MEDICAL HISTORY:
Active Problem List:
- Mild cognitive impairment (diagnosed 2020-01-15)
- Neurosyphilis (diagnosed 2020-04-15)
- Aortic valve stenosis (diagnosed 2020-09-15)
- Orthostatic hypotension (diagnosed 2021-03-15)
- Hypertension (poorly controlled)
- Type 2 diabetes mellitus
- Hyperlipidemia
- Osteoarthritis (multijoint)
- Atrial fibrillation (non-valvular)
- Neurosyphilis (treated)
- Cerebral amyloid angiopathy

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Her current medications include metoprolol, metformin, and ibuprofen as needed.

Home Medications:
- Lisinopril 10 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Atorvastatin 40 mg oral nightly
- Donepezil 10 mg oral nightly
- Acetaminophen 500 mg oral every 6 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Alzheimer's disease (late onset)
- Father: Stroke and hypertension
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
The patient lives alone and has been managing her own medications.

VITAL SIGNS:
On examination, the patient has a blood pressure of 145/85 mmHg, a pulse of 78/min, and a normal respiratory rate.

PHYSICAL EXAMINATION:
She is alert but appears mildly confused. Her gait is unsteady, and she has difficulty with heel-to-toe walking.

LABORATORY DATA:
Her most recent laboratory results are as follows:

- Hemoglobin A1c: 7.2%
- Serum creatinine: 1.4 mg/dL (elevated)
- Fasting blood glucose: 160 mg/dL
- Serum sodium: 142 mEq/L
- Serum potassium: 4.5 mEq/L

OTHER STUDIES:
A home safety assessment reveals loose rugs and poor lighting in several areas.

## Encounter 5: source 2022-03-15, practice 2024-12-14, ed, Emergency Medicine

Chief complaint: Acute onset of increasing confusion, agitation, and difficulty sleeping over 2 days following a fall two weeks prior.

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of increasing confusion, agitation, and difficulty sleeping over 2 days following a fall two weeks prior.

HISTORY OF PRESENT ILLNESS:
The patient is an 80-year-old female with mild cognitive impairment and recurrent falls who presents with acute altered mental status. She is brought to the emergency department by her daughter because of a 2-day history of increasing confusion, agitation, and difficulty sleeping. The patient's daughter mentions that her mother fell two weeks ago but did not complain of any pain and did not seek medical attention.

PAST MEDICAL HISTORY:
Active Problem List:
- Mild cognitive impairment (diagnosed 2020-01-15)
- Neurosyphilis (diagnosed 2020-04-15)
- Aortic valve stenosis (diagnosed 2020-09-15)
- Orthostatic hypotension (diagnosed 2021-03-15)
- History of Falling (diagnosed 2021-09-15)
- Hypertension (poorly controlled)
- Type 2 diabetes mellitus
- Hyperlipidemia
- Osteoarthritis (multijoint)
- Atrial fibrillation (non-valvular)
- Neurosyphilis (treated)
- Cerebral amyloid angiopathy

The patient has a history of hypertension and mild cognitive impairment.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Her medications include hydrochlorothiazide and donepezil.

Home Medications:
- Lisinopril 10 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Metformin 1000 mg oral twice daily
- Atorvastatin 40 mg oral nightly
- Acetaminophen 500 mg oral every 6 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Alzheimer's disease (late onset)
- Father: Stroke and hypertension
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired high school teacher.

VITAL SIGNS:
Her temperature is 37.8°C (100°F), blood pressure is 160/90 mmHg, pulse is 102/min, and respiratory rate is 20/min.

PHYSICAL EXAMINATION:
On examination, the patient is disoriented to time and place, with a fluctuating level of consciousness. She is agitated and has a tremor in her hands.

LABORATORY DATA:
Laboratory studies are as follows:

- Sodium: 132 mEq/L
- Potassium: 4.5 mEq/L
- Chloride: 98 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen (BUN): 18 mg/dL
- Creatinine: 1.0 mg/dL
- Glucose: 110 mg/dL
- Calcium: 8.9 mg/dL
- Complete blood count, liver function tests, and thyroid function tests are within normal limits.

A urinalysis shows no evidence of infection.

IMAGING:
A chest X-ray is unremarkable.

## Encounter 6: source 2022-09-15, practice 2025-06-11, ed, Emergency Medicine

Chief complaint: Sudden collapse while gardening with acute onset of right-sided weakness and difficulty speaking.

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. Lisa Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden collapse while gardening with acute onset of right-sided weakness and difficulty speaking.

HISTORY OF PRESENT ILLNESS:
The patient has a history of cognitive impairment and subdural hematoma. Her husband reports that she was in her usual state of health until she suddenly collapsed while gardening. No history of trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Mild cognitive impairment (diagnosed 2020-01-15)
- Neurosyphilis (diagnosed 2020-04-15)
- Aortic valve stenosis (diagnosed 2020-09-15)
- Orthostatic hypotension (diagnosed 2021-03-15)
- History of Falling (diagnosed 2021-09-15)
- Acute subdural hematoma (diagnosed 2022-03-15)
- Hypertension (poorly controlled)
- Type 2 diabetes mellitus
- Hyperlipidemia
- Osteoarthritis (multijoint)
- Atrial fibrillation (non-valvular)
- Neurosyphilis (treated)
- Cerebral amyloid angiopathy

Her past medical history includes hypertension, hyperlipidemia, and mild cognitive impairment.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Her medications include lisinopril, atorvastatin, and donepezil.

Home Medications:
- Metoprolol succinate 50 mg oral daily
- Metformin 1000 mg oral twice daily
- Acetaminophen 500 mg oral every 6 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Alzheimer's disease (late onset)
- Father: Stroke and hypertension
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired high school teacher.

VITAL SIGNS:
Her blood pressure is 160/90 mmHg.

PHYSICAL EXAMINATION:
On examination, she is alert but aphasic. Neurological examination reveals right-sided hemiparesis and neglect.

IMAGING:
A non-contrast head CT scan shows a left parietal lobe hemorrhage without evidence of mass effect or ventricular extension.

## Encounter 7: source 2023-03-15, practice 2025-12-03, ed, Emergency Medicine

Chief complaint: Found unresponsive at home by her daughter approximately 14 hours after last being seen normal.

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. James Williams, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Found unresponsive at home by her daughter approximately 14 hours after last being seen normal.

HISTORY OF PRESENT ILLNESS:
The patient returns to the emergency department with cerebral amyloid angiopathy and is found unresponsive at home by her daughter approximately 14 hours after last being seen normal. On arrival to the emergency department, she is comatose and does not respond to painful stimuli. Despite supportive measures, the patient develops herniation and dies within the next two hours.

PAST MEDICAL HISTORY:
Active Problem List:
- Mild cognitive impairment (diagnosed 2020-01-15)
- Neurosyphilis (diagnosed 2020-04-15)
- Aortic valve stenosis (diagnosed 2020-09-15)
- Orthostatic hypotension (diagnosed 2021-03-15)
- History of Falling (diagnosed 2021-09-15)
- Acute subdural hematoma (diagnosed 2022-03-15)
- Cerebral amyloid angiopathy (diagnosed 2022-09-15)
- Hypertension (poorly controlled)
- Type 2 diabetes mellitus
- Hyperlipidemia
- Osteoarthritis (multijoint)
- Atrial fibrillation (non-valvular)
- Neurosyphilis (treated)

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Metformin 1000 mg oral twice daily
- Atorvastatin 40 mg oral nightly
- Donepezil 10 mg oral nightly
- Acetaminophen 500 mg oral every 6 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Alzheimer's disease (late onset)
- Father: Stroke and hypertension
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired high school teacher.

VITAL SIGNS:
Vital signs show elevated blood pressure and irregularly irregular pulse.

PHYSICAL EXAMINATION:
Neurologic examination reveals flaccid paralysis of the left upper and lower extremities, with absent deep tendon reflexes and an upgoing plantar response on the left.

IMAGING:
Emergent non-contrast CT of the head reveals significant edema in the right cerebral hemisphere, with preservation of the right occipital lobe and medial frontal cortex. There is no evidence of intracranial hemorrhage.

PATHOLOGY:
At autopsy, microscopic examination of the right lateral frontal cortex is performed to assess the histopathological changes at this stage.
