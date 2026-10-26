# synthetic_hospital patient 1701

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 27 at first encounter, sex F, African American, Uninsured.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-12-20 (Mon) | outpatient | Rheumatology |
| 1 | 2020-04-15 | 2022-03-22 (Tue) | outpatient | Dermatology |
| 2 | 2021-01-15 | 2022-12-15 (Thu) | ed | Emergency Medicine |
| 3 | 2021-09-15 | 2023-08-29 (Tue) | ed | Emergency Medicine |
| 4 | 2022-05-15 | 2024-04-18 (Thu) | ed | Emergency Medicine |
| 5 | 2022-09-15 | 2024-08-18 (Sun) | icu | Critical Care Medicine |
| 6 | 2023-03-15 | 2025-02-23 (Sun) | ed | Emergency Medicine |
| 7 | 2024-01-15 | 2025-12-29 (Mon) | icu | Neurosurgery |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Sulfa drugs - maculopapular rash"
 ],
 "insurance": "Uninsured",
 "occupation": "Retail sales associate",
 "alcohol_use": "Occasional social use, 1-2 drinks per month",
 "family_history": [
  "Mother with systemic lupus erythematosus",
  "Maternal aunt with rheumatoid arthritis",
  "Father with essential hypertension",
  "Paternal grandfather with epilepsy"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "20 mg daily (adherent for past 2 months, previously stopped)",
   "name": "Prednisone"
  },
  {
   "dose": "200 mg twice daily (noncompliant, discontinued 6 months ago due to cost)",
   "name": "Hydroxychloroquine"
  },
  {
   "dose": "500 mg twice daily (taken irregularly, frequently missed doses)",
   "name": "Levetiracetam"
  },
  {
   "dose": "10 mg daily (noncompliant, discontinued 6 months ago)",
   "name": "Lisinopril"
  },
  {
   "dose": "44 mcg, 2 puffs twice daily",
   "name": "Fluticasone propionate inhaler"
  },
  {
   "dose": "90 mcg, 2 puffs every 4 hours as needed for wheezing",
   "name": "Albuterol sulfate inhaler"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Systemic lupus erythematosus",
  "Epilepsy",
  "Asthma",
  "Secondary hypertension",
  "Secondary amenorrhea"
 ],
 "age_at_first_encounter": 27
}
```

Primary diagnoses: ["Systemic lupus erythematosus with arthropathy", "Lupus nephritis flare", "Aortic dissection", "Brain herniation", "Status epilepticus", "Generalized tonic-clonic status epilepticus", "Drug-induced acne (Steroid acne)", "Systemic lupus erythematosus"]
Comorbidities: ["Medication noncompliance", "Epilepsy", "Essential hypertension", "Systemic lupus erythematosus"]

## Encounter 0: source 2020-01-15, practice 2021-12-20, outpatient, Rheumatology

Chief complaint: 3-month history of joint pain and swelling with facial rash that worsens with sun exposure

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of joint pain and swelling with facial rash that worsens with sun exposure

HISTORY OF PRESENT ILLNESS:
She reports that the pain is worse in the morning and tends to improve throughout the day. She also mentions a recent onset of fatigue and a rash on her face that worsens with sun exposure.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus
- Epilepsy
- Asthma
- Secondary hypertension
- Secondary amenorrhea

Her medical history is significant for mild asthma and an episode of pleurisy two years ago.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Prednisone 20 mg daily (adherent for past 2 months, previously stopped)
- Hydroxychloroquine 200 mg twice daily (noncompliant, discontinued 6 months ago due to cost)
- Levetiracetam 500 mg twice daily (taken irregularly, frequently missed doses)
- Lisinopril 10 mg daily (noncompliant, discontinued 6 months ago)
- Fluticasone propionate inhaler 44 mcg, 2 puffs twice daily
- Albuterol sulfate inhaler 90 mcg, 2 puffs every 4 hours as needed for wheezing

ALLERGIES:
Sulfa drugs - maculopapular rash

FAMILY HISTORY:
- Mother with systemic lupus erythematosus
- Maternal aunt with rheumatoid arthritis
- Father with essential hypertension
- Paternal grandfather with epilepsy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retail sales associate.

PHYSICAL EXAMINATION:
On physical examination, you note bilateral swelling and tenderness of the knees and wrists without significant warmth or redness. There is also a malar rash across her cheeks and the bridge of her nose.

LABORATORY DATA:
Laboratory tests reveal the following:

- Hemoglobin: 11.2 g/dL (Normal: 12-16 g/dL)
- White blood cell count: 3,800/uL (Normal: 4,500-11,000/uL)
- Platelet count: 120,000/uL (Normal: 150,000-450,000/uL)
- Erythrocyte sedimentation rate (ESR): 50 mm/hr (Normal: 0-20 mm/hr)
- Antinuclear antibody (ANA) titer: Positive at 1:640
- Anti-double-stranded DNA (anti-dsDNA) antibody: Positive
- Anti-Smith (anti-Sm) antibody: Positive
- Complement levels (C3, C4): Low

## Encounter 1: source 2020-04-15, practice 2022-03-22, outpatient, Dermatology

Chief complaint: Worsening facial rash with multiple small red and tender bumps on cheeks and forehead

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Johnson, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening facial rash with multiple small red and tender bumps on cheeks and forehead

HISTORY OF PRESENT ILLNESS:
The patient is a 27-year-old female with systemic lupus erythematosus adherent to prednisone following her Rheumatology evaluation three months prior. She presents to Dermatology with a 3-week history of worsening facial rash. She reports the development of multiple small, red, and tender bumps on her cheeks and forehead. She denies the presence of blackheads or painful nodules.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Epilepsy
- Asthma
- Secondary hypertension
- Secondary amenorrhea

Her medical history is significant for systemic lupus erythematosus (SLE),.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She takes for which she has been taking prednisone 20 mg daily for the past 2 months.

Home Medications:
- Hydroxychloroquine 200 mg twice daily (noncompliant, discontinued 6 months ago due to cost)
- Levetiracetam 500 mg twice daily (taken irregularly, frequently missed doses)
- Lisinopril 10 mg daily (noncompliant, discontinued 6 months ago)
- Fluticasone propionate inhaler 44 mcg, 2 puffs twice daily
- Albuterol sulfate inhaler 90 mcg, 2 puffs every 4 hours as needed for wheezing

ALLERGIES:
Sulfa drugs - maculopapular rash

FAMILY HISTORY:
- Mother with systemic lupus erythematosus
- Maternal aunt with rheumatoid arthritis
- Father with essential hypertension
- Paternal grandfather with epilepsy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retail sales associate.

PHYSICAL EXAMINATION:
On physical examination, numerous monomorphic papules are noted on her cheeks and forehead without any comedones, cysts, or nodules. There is no evidence of scarring.

LABORATORY DATA:
Her recent laboratory tests are as follows:

- Complete blood count: within normal limits
- Erythrocyte sedimentation rate: 18 mm/hr (normal: 0-22 mm/hr)
- C-reactive protein: 2.0 mg/L (normal: <5.0 mg/L)
- Serum creatinine: 0.9 mg/dL (normal: 0.6-1.1 mg/dL)
- Urinalysis: normal

## Encounter 2: source 2021-01-15, practice 2022-12-15, ed, Emergency Medicine

Chief complaint: Bilateral lower extremity edema and frothy urine with new-onset fatigue

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Emily Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Bilateral lower extremity edema and frothy urine with new-onset fatigue

HISTORY OF PRESENT ILLNESS:
The patient with systemic lupus erythematosus diagnosed in 2020 and subsequent steroid-induced acne presents with bilateral lower extremity edema and frothy urine. She also reports new-onset fatigue and mild arthralgia.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Drug-induced acne (Steroid acne) (diagnosed 2020-04-15)
- Epilepsy
- Asthma
- Secondary hypertension
- Secondary amenorrhea

She has a history of systemic lupus erythematosus diagnosed one year ago.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She stopped taking her medications six months ago due to financial issues.

Home Medications:
- Prednisone 20 mg daily (adherent for past 2 months, previously stopped)
- Hydroxychloroquine 200 mg twice daily (noncompliant, discontinued 6 months ago due to cost)
- Levetiracetam 500 mg twice daily (taken irregularly, frequently missed doses)
- Lisinopril 10 mg daily (noncompliant, discontinued 6 months ago)
- Fluticasone propionate inhaler 44 mcg, 2 puffs twice daily
- Albuterol sulfate inhaler 90 mcg, 2 puffs every 4 hours as needed for wheezing

ALLERGIES:
Sulfa drugs - maculopapular rash

FAMILY HISTORY:
- Mother with systemic lupus erythematosus
- Maternal aunt with rheumatoid arthritis
- Father with essential hypertension
- Paternal grandfather with epilepsy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retail sales associate.

VITAL SIGNS:
Her blood pressure is 148/92 mmHg.

LABORATORY DATA:
Lab studies reveal a serum creatinine of 2.1 mg/dL, 4+ proteinuria, and dysmorphic RBCs with casts in the urine. Her complement levels are low, and anti-dsDNA titers are elevated.

PLAN:
A renal biopsy is scheduled to determine the class of lupus nephritis.

## Encounter 3: source 2021-09-15, practice 2023-08-29, ed, Emergency Medicine

Chief complaint: Generalized tonic-clonic seizure lasting 7 minutes without return to full consciousness

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. James Patterson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Generalized tonic-clonic seizure lasting 7 minutes without return to full consciousness

HISTORY OF PRESENT ILLNESS:
The patient with epilepsy and systemic lupus erythematosus returns to the emergency department. Her husband reports that the seizure lasted approximately 7 minutes and she has not regained full consciousness since the episode.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Drug-induced acne (Steroid acne) (diagnosed 2020-04-15)
- Systemic lupus erythematosus with arthropathy (diagnosed 2021-01-15)
- Epilepsy
- Asthma
- Secondary hypertension
- Secondary amenorrhea

Her medical history is significant for epilepsy, for which she takes oral medication irregularly.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Prednisone 20 mg daily (adherent for past 2 months, previously stopped)
- Hydroxychloroquine 200 mg twice daily (noncompliant, discontinued 6 months ago due to cost)
- Levetiracetam 500 mg twice daily (taken irregularly, frequently missed doses)
- Lisinopril 10 mg daily (noncompliant, discontinued 6 months ago)
- Fluticasone propionate inhaler 44 mcg, 2 puffs twice daily
- Albuterol sulfate inhaler 90 mcg, 2 puffs every 4 hours as needed for wheezing

ALLERGIES:
Sulfa drugs - maculopapular rash

FAMILY HISTORY:
- Mother with systemic lupus erythematosus
- Maternal aunt with rheumatoid arthritis
- Father with essential hypertension
- Paternal grandfather with epilepsy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retail sales associate.

VITAL SIGNS:
Her vitals are as follows: blood pressure 145/90 mmHg, heart rate 110 bpm, respiratory rate 22 breaths/min, and oxygen saturation 96% on room air.

PHYSICAL EXAMINATION:
On examination, she is drowsy but responsive to painful stimuli, with a Glasgow Coma Scale score of 10.

LABORATORY DATA:
The initial laboratory values are:
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 102 mEq/L
- Bicarbonate: 24 mEq/L
- Blood glucose: 98 mg/dL
- Creatinine: 0.9 mg/dL

OTHER STUDIES:
An IV line is established, and blood is drawn for laboratory tests.

## Encounter 4: source 2022-05-15, practice 2024-04-18, ed, Emergency Medicine

Chief complaint: Fatigue, facial puffiness, and dark-colored urine with intermittent oral ulcers

ENCOUNTER NOTE
Date: 2022-05-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fatigue, facial puffiness, and dark-colored urine with intermittent oral ulcers

HISTORY OF PRESENT ILLNESS:
The patient with established systemic lupus erythematosus returns with symptoms similar to her January 2021 presentation. She has had joint pains in her hands and wrists for several months and recently developed intermittent oral ulcers. Over the past week, she has noticed swelling around her eyes, particularly in the morning, and darker-colored urine.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Drug-induced acne (Steroid acne) (diagnosed 2020-04-15)
- Systemic lupus erythematosus with arthropathy (diagnosed 2021-01-15)
- Generalized tonic-clonic status epilepticus (diagnosed 2021-09-15)
- Epilepsy
- Asthma
- Secondary hypertension
- Secondary amenorrhea

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Prednisone 20 mg daily (adherent for past 2 months, previously stopped)
- Hydroxychloroquine 200 mg twice daily (noncompliant, discontinued 6 months ago due to cost)
- Levetiracetam 500 mg twice daily (taken irregularly, frequently missed doses)
- Lisinopril 10 mg daily (noncompliant, discontinued 6 months ago)
- Fluticasone propionate inhaler 44 mcg, 2 puffs twice daily
- Albuterol sulfate inhaler 90 mcg, 2 puffs every 4 hours as needed for wheezing

ALLERGIES:
Sulfa drugs - maculopapular rash

FAMILY HISTORY:
- Mother with systemic lupus erythematosus
- Maternal aunt with rheumatoid arthritis
- Father with essential hypertension
- Paternal grandfather with epilepsy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retail sales associate.

VITAL SIGNS:
Blood pressure is 150/95 mmHg.

PHYSICAL EXAMINATION:
Examination shows periorbital edema and mild bilateral lower extremity edema. Cardiopulmonary examination is normal.

LABORATORY DATA:
Serum creatinine                                        1.8 mg/dL
Urinalysis                                              2+ protein, numerous RBC casts
ANA Positive
Anti-dsDNA Positive
Complement C3 Low

## Encounter 5: source 2022-09-15, practice 2024-08-18, icu, Critical Care Medicine

Chief complaint: Refractory status epilepticus requiring intubation for airway protection

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. Robert Kim, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Refractory status epilepticus requiring intubation for airway protection

HISTORY OF PRESENT ILLNESS:
This 29-year-old female with systemic lupus erythematosus and epilepsy presents with recurrent seizure activity, returning following status epilepticus one year ago and lupus nephritis flare four months prior. Her husband reports that she has been noncompliant with her medication regimen for the past month. She received lorazepam 4 mg intravenously by the paramedics during transport, but she remains unresponsive on arrival. A repeat dose of lorazepam is administered, and she is intubated for airway protection. Thirty minutes later, there is no improvement in her level of consciousness. There are no signs of ongoing convulsions.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Drug-induced acne (Steroid acne) (diagnosed 2020-04-15)
- Systemic lupus erythematosus with arthropathy (diagnosed 2021-01-15)
- Generalized tonic-clonic status epilepticus (diagnosed 2021-09-15)
- Lupus nephritis flare (diagnosed 2022-05-15)
- Epilepsy
- Asthma
- Secondary hypertension
- Secondary amenorrhea

Her husband reports that she has a history of epilepsy.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Prednisone 20 mg daily (adherent for past 2 months, previously stopped)
- Hydroxychloroquine 200 mg twice daily (noncompliant, discontinued 6 months ago due to cost)
- Levetiracetam 500 mg twice daily (taken irregularly, frequently missed doses)
- Lisinopril 10 mg daily (noncompliant, discontinued 6 months ago)
- Fluticasone propionate inhaler 44 mcg, 2 puffs twice daily
- Albuterol sulfate inhaler 90 mcg, 2 puffs every 4 hours as needed for wheezing

ALLERGIES:
Sulfa drugs - maculopapular rash

FAMILY HISTORY:
- Mother with systemic lupus erythematosus
- Maternal aunt with rheumatoid arthritis
- Father with essential hypertension
- Paternal grandfather with epilepsy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retail sales associate.

VITAL SIGNS:
Her temperature is 37.2°C (99.0°F), blood pressure is 130/85 mmHg, pulse is 88/min, and respirations are 16/min.

PHYSICAL EXAMINATION:
Physical examination reveals a postictal state with no focal neurological deficits.

LABORATORY DATA:
Blood glucose is 98 mg/dL.

## Encounter 6: source 2023-03-15, practice 2025-02-23, ed, Emergency Medicine

Chief complaint: Sudden onset severe tearing chest pain radiating to the back

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. Lisa Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset severe tearing chest pain radiating to the back

HISTORY OF PRESENT ILLNESS:
The patient with SLE, lupus nephritis, and status epilepticus describes tearing pain starting several hours ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Drug-induced acne (Steroid acne) (diagnosed 2020-04-15)
- Systemic lupus erythematosus with arthropathy (diagnosed 2021-01-15)
- Generalized tonic-clonic status epilepticus (diagnosed 2021-09-15)
- Lupus nephritis flare (diagnosed 2022-05-15)
- Status epilepticus (diagnosed 2022-09-15)
- Epilepsy
- Asthma
- Secondary hypertension
- Secondary amenorrhea

Her past medical history is significant for amenorrhea and she has never been pregnant. She mentions being treated for hypertension during a recent prenatal visit.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Prednisone 20 mg daily (adherent for past 2 months, previously stopped)
- Hydroxychloroquine 200 mg twice daily (noncompliant, discontinued 6 months ago due to cost)
- Levetiracetam 500 mg twice daily (taken irregularly, frequently missed doses)
- Lisinopril 10 mg daily (noncompliant, discontinued 6 months ago)
- Fluticasone propionate inhaler 44 mcg, 2 puffs twice daily
- Albuterol sulfate inhaler 90 mcg, 2 puffs every 4 hours as needed for wheezing

ALLERGIES:
Sulfa drugs - maculopapular rash

FAMILY HISTORY:
- Mother with systemic lupus erythematosus
- Maternal aunt with rheumatoid arthritis
- Father with essential hypertension
- Paternal grandfather with epilepsy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retail sales associate.

VITAL SIGNS:
Her blood pressure is 150/90 mmHg in the right arm and 140/85 mmHg in the left arm.

PHYSICAL EXAMINATION:
On examination, she is 5 feet tall, has a webbed neck, and a low hairline.
A diastolic murmur is heard on auscultation.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 13.2 g/dL
- White blood cell count: 7.2 x 10^3/uL
- Platelets: 250 x 10^3/uL
- Serum creatinine: 0.9 mg/dL
- Beta-hCG: Positive

IMAGING:
An echocardiogram reveals a bicuspid aortic valve and aortic root dilation.

## Encounter 7: source 2024-01-15, practice 2025-12-29, icu, Neurosurgery

Chief complaint: Traumatic brain injury with deteriorating mental status following head-on motor vehicle collision

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. David Washington, MD
Department: Neurosurgery
Visit Type: icu

CHIEF COMPLAINT:
Traumatic brain injury with deteriorating mental status following head-on motor vehicle collision

HISTORY OF PRESENT ILLNESS:
The patient is a 31-year-old female with systemic lupus erythematosus, recurrent status epilepticus, and prior aortic dissection who presents as the restrained driver in a head-on collision. On arrival, she is alert but agitated, with a Glasgow Coma Scale score of 14. Two hours later, she becomes increasingly drowsy and develops a right-sided headache.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Drug-induced acne (Steroid acne) (diagnosed 2020-04-15)
- Systemic lupus erythematosus with arthropathy (diagnosed 2021-01-15)
- Generalized tonic-clonic status epilepticus (diagnosed 2021-09-15)
- Lupus nephritis flare (diagnosed 2022-05-15)
- Status epilepticus (diagnosed 2022-09-15)
- Aortic dissection (diagnosed 2023-03-15)
- Epilepsy
- Asthma
- Secondary hypertension
- Secondary amenorrhea

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Prednisone 20 mg daily (adherent for past 2 months, previously stopped)
- Hydroxychloroquine 200 mg twice daily (noncompliant, discontinued 6 months ago due to cost)
- Levetiracetam 500 mg twice daily (taken irregularly, frequently missed doses)
- Lisinopril 10 mg daily (noncompliant, discontinued 6 months ago)
- Fluticasone propionate inhaler 44 mcg, 2 puffs twice daily
- Albuterol sulfate inhaler 90 mcg, 2 puffs every 4 hours as needed for wheezing

ALLERGIES:
Sulfa drugs - maculopapular rash

FAMILY HISTORY:
- Mother with systemic lupus erythematosus
- Maternal aunt with rheumatoid arthritis
- Father with essential hypertension
- Paternal grandfather with epilepsy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retail sales associate.

VITAL SIGNS:
Her blood pressure is 130/85 mmHg, heart rate is 92 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 98% on room air. Repeat vital signs show a blood pressure of 155/90 mmHg, heart rate of 88 beats per minute, and a respiratory rate of 18 breaths per minute.

PHYSICAL EXAMINATION:
Physical examination reveals a contusion over her forehead, but no other obvious injuries. Her pupils are equal and reactive, but her right pupil is slightly slower to react.

IMAGING:
A non-contrast head CT scan shows a small subdural hematoma without significant midline shift.

OTHER STUDIES:
A focused assessment with sonography for trauma (FAST) exam is negative.
