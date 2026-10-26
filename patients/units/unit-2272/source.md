# synthetic_hospital patient 2272

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 66 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-04-18 (Mon) | outpatient | Pulmonology |
| 1 | 2020-07-15 | 2022-10-24 (Mon) | outpatient | Orthopedics |
| 2 | 2021-01-15 | 2023-04-24 (Mon) | outpatient | Hematology-Oncology |
| 3 | 2022-01-15 | 2024-04-24 (Wed) | outpatient | Neurology |
| 4 | 2022-07-15 | 2024-10-07 (Mon) | outpatient | Neurology |
| 5 | 2023-01-15 | 2025-04-17 (Thu) | ed | Emergency Medicine |
| 6 | 2023-07-15 | 2025-10-10 (Fri) | ed | Emergency Medicine |
| 7 | 2024-01-15 | 2026-04-14 (Tue) | icu | Critical Care Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired construction foreman",
 "alcohol_use": "Occasional, 1-2 drinks per month",
 "family_history": [
  "Father: Myocardial infarction at age 68",
  "Mother: Dementia with behavioral disturbances",
  "Older brother: Paget disease of bone"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 40 pack-years, quit 2 years ago",
 "home_medications": [
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "0.4 mg PO daily",
   "name": "Tamsulosin"
  },
  {
   "dose": "500 mg PO PRN pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Cystoscopy with urethral dilation (age 64)",
  "Appendectomy (age 34)",
  "Diagnostic bone biopsy left tibia (age 65)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Chronic kidney disease",
  "Benign prostatic hyperplasia",
  "Lumbar osteoarthritis with radiculopathy",
  "Chronic bronchitis",
  "Mild neurocognitive disorder (age-related)",
  "Carotid sinus hypersensitivity",
  "Urethral stricture",
  "Paget disease of bone (left tibia)",
  "Primary myelofibrosis"
 ],
 "age_at_first_encounter": 66
}
```

Primary diagnoses: ["Age-related cognitive decline", "Carotid sinus hypersensitivity", "Cyanide toxicity", "Chronic bronchitis", "Uncomplicated lumbar radiculopathy", "Urethral stricture", "Paget disease of bone (left tibia)", "Primary myelofibrosis"]
Comorbidities: ["Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2022-04-18, outpatient, Pulmonology

Chief complaint: Persistent productive cough with intermittent blood-tinged sputum for the past 4 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent productive cough with intermittent blood-tinged sputum for the past 4 months

HISTORY OF PRESENT ILLNESS:
He reports that the cough is productive, with white sputum, and has been present for the last two years, worsening in the winter months. He denies any recent fever, weight loss, or night sweats.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Chronic kidney disease
- Benign prostatic hyperplasia
- Lumbar osteoarthritis with radiculopathy
- Chronic bronchitis
- Mild neurocognitive disorder (age-related)
- Carotid sinus hypersensitivity
- Urethral stricture
- Paget disease of bone (left tibia)
- Primary myelofibrosis

PAST SURGICAL HISTORY:
- Cystoscopy with urethral dilation (age 64)
- Appendectomy (age 34)
- Diagnostic bone biopsy left tibia (age 65)

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg PO daily
- Tamsulosin 0.4 mg PO daily
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Dementia with behavioral disturbances
- Older brother: Paget disease of bone

SOCIAL HISTORY:
40-pack-year smoking history

VITAL SIGNS:
His blood pressure is 135/85 mmHg, heart rate is 78 beats per minute, and oxygen saturation is 96% on room air.

PHYSICAL EXAMINATION:
On physical examination, you note an overweight male in no acute distress. Lung auscultation reveals scattered wheezes and prolonged expiration.

IMAGING:
A chest X-ray shows increased bronchovascular markings without evidence of a mass or focal consolidation.

## Encounter 1: source 2020-07-15, practice 2022-10-24, outpatient, Orthopedics

Chief complaint: Six-month history of increasing left hip pain with recent onset of headaches and right-sided hearing difficulty

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of increasing left hip pain with recent onset of headaches and right-sided hearing difficulty

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic bronchitis (diagnosed 2020-01-15)
- Hypertension
- Chronic kidney disease
- Benign prostatic hyperplasia
- Lumbar osteoarthritis with radiculopathy
- Mild neurocognitive disorder (age-related)
- Carotid sinus hypersensitivity
- Urethral stricture
- Paget disease of bone (left tibia)
- Primary myelofibrosis

His medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Cystoscopy with urethral dilation (age 64)
- Appendectomy (age 34)
- Diagnostic bone biopsy left tibia (age 65)

MEDICATIONS:
He takes controlled with hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg PO daily
- Tamsulosin 0.4 mg PO daily
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Dementia with behavioral disturbances
- Older brother: Paget disease of bone

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 2 years ago. Alcohol: Occasional, 1-2 drinks per month. Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
On physical examination, the patient has an enlarged, warm left tibia with mild erythema but no overlying skin changes. Neurological examination is unremarkable except for decreased hearing acuity in the right ear.

LABORATORY DATA:
Laboratory tests reveal:

- Serum calcium: 9.8 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum phosphorus: 3.5 mg/dL (normal: 2.5-4.5 mg/dL)
- Alkaline phosphatase: 220 U/L (normal: 40-130 U/L)
- Serum parathyroid hormone (PTH): 55 pg/mL (normal: 15-65 pg/mL)
- 25-hydroxyvitamin D: 32 ng/mL (normal: 20-50 ng/mL)

IMAGING:
An X-ray of the left leg shows thickened cortical bone with a "cotton wool" appearance.

OTHER STUDIES:
Audiometry confirms conductive hearing loss in the right ear.

## Encounter 2: source 2021-01-15, practice 2023-04-24, outpatient, Hematology-Oncology

Chief complaint: Progressive fatigue, early satiety, and unintentional weight loss over the past three months

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Aisha Patel, MD
Department: Hematology-Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue, early satiety, and unintentional weight loss over the past three months

HISTORY OF PRESENT ILLNESS:
Patient with chronic bronchitis and Paget disease returns and presents to the clinic with progressive fatigue, early satiety, and unintentional weight loss over the past three months.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic bronchitis (diagnosed 2020-01-15)
- Paget disease of bone (left tibia) (diagnosed 2020-07-15)
- Hypertension
- Chronic kidney disease
- Benign prostatic hyperplasia
- Lumbar osteoarthritis with radiculopathy
- Mild neurocognitive disorder (age-related)
- Carotid sinus hypersensitivity
- Urethral stricture
- Primary myelofibrosis

His past medical history is notable for hypertension but no known malignancy.

PAST SURGICAL HISTORY:
- Cystoscopy with urethral dilation (age 64)
- Appendectomy (age 34)
- Diagnostic bone biopsy left tibia (age 65)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Lisinopril 10 mg PO daily
- Tamsulosin 0.4 mg PO daily
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Dementia with behavioral disturbances
- Older brother: Paget disease of bone

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 2 years ago. Alcohol: Occasional, 1-2 drinks per month. Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
Physical examination reveals marked splenomegaly extending below the left costal margin.

LABORATORY DATA:
Laboratory studies show pancytopenia with a hemoglobin of 9.0 g/dL, a white blood cell count of 2,500/µL, and a platelet count of 80,000/µL. Findings on blood smear are shown below.

OTHER STUDIES:
Bone marrow biopsy is attempted but yields a "dry tap,".

## Encounter 3: source 2022-01-15, practice 2024-04-24, outpatient, Neurology

Chief complaint: Memory issues over the past year including misplacing items and difficulty remembering names of new acquaintances

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James O'Brien, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Memory issues over the past year including misplacing items and difficulty remembering names of new acquaintances

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of cognitive symptoms one year following his primary myelofibrosis diagnosis and ongoing management of Paget disease and chronic bronchitis. His care partner is concerned because he occasionally forgets where he places his keys and has trouble remembering the names of new acquaintances. He also mentions difficulty finding the right words during conversations. However, he remains active in his community, pays his bills on time, and has no trouble managing his medications. His sleep pattern has changed slightly; he now wakes up earlier than he used to but does not feel fatigued during the day.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic bronchitis (diagnosed 2020-01-15)
- Paget disease of bone (left tibia) (diagnosed 2020-07-15)
- Primary myelofibrosis (diagnosed 2021-01-15)
- Hypertension
- Chronic kidney disease
- Benign prostatic hyperplasia
- Lumbar osteoarthritis with radiculopathy
- Mild neurocognitive disorder (age-related)
- Carotid sinus hypersensitivity
- Urethral stricture

PAST SURGICAL HISTORY:
- Cystoscopy with urethral dilation (age 64)
- Appendectomy (age 34)
- Diagnostic bone biopsy left tibia (age 65)

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg PO daily
- Tamsulosin 0.4 mg PO daily
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Dementia with behavioral disturbances
- Older brother: Paget disease of bone

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 2 years ago. Alcohol: Occasional, 1-2 drinks per month. Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
On examination, the patient is alert and oriented to person, place, and time. His Mini-Mental State Examination score is 28/30, losing points only for delayed recall. His neurological examination is unremarkable.

LABORATORY DATA:
Complete blood count: within normal limits
Serum electrolytes: within normal limits
Thyroid-stimulating hormone: within normal limits
Vitamin B12 level: within normal limits

## Encounter 4: source 2022-07-15, practice 2024-10-07, outpatient, Neurology

Chief complaint: Two-week history of right anterior thigh pain radiating to medial knee with difficulty climbing stairs and rising from chairs

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. James O'Brien, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Two-week history of right anterior thigh pain radiating to medial knee with difficulty climbing stairs and rising from chairs

HISTORY OF PRESENT ILLNESS:
The patient, with established lumbar osteoarthritis with radiculopathy, returns to neurology six months after prior evaluation for memory concerns. He describes a dull ache in the anterior thigh that occasionally radiates toward the medial knee. The pain is aggravated by walking uphill and improves with rest. He also reports that his right leg feels weaker, especially when trying to rise from a chair or step onto a curb. He has no bowel or bladder incontinence, saddle anesthesia, or back trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic bronchitis (diagnosed 2020-01-15)
- Paget disease of bone (left tibia) (diagnosed 2020-07-15)
- Primary myelofibrosis (diagnosed 2021-01-15)
- Age-related cognitive decline (diagnosed 2022-01-15)
- Hypertension
- Chronic kidney disease
- Benign prostatic hyperplasia
- Lumbar osteoarthritis with radiculopathy
- Mild neurocognitive disorder (age-related)
- Carotid sinus hypersensitivity
- Urethral stricture

His medical history includes long-standing lumbar osteoarthritis and poorly controlled hypertension.

PAST SURGICAL HISTORY:
- Cystoscopy with urethral dilation (age 64)
- Appendectomy (age 34)
- Diagnostic bone biopsy left tibia (age 65)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Lisinopril 10 mg PO daily
- Tamsulosin 0.4 mg PO daily
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Dementia with behavioral disturbances
- Older brother: Paget disease of bone

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 2 years ago. Alcohol: Occasional, 1-2 drinks per month. Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
On physical examination, strength testing reveals 4/5 strength with right knee extension, while hip flexion and ankle movements are intact. The right patellar reflex is diminished compared to the left. Sensation to pinprick and light touch is reduced over the anterior thigh and medial shin. Straight leg raise test is negative.

PLAN:
An MRI of the lumbar spine is ordered to evaluate suspected radiculopathy.

## Encounter 5: source 2023-01-15, practice 2025-04-17, ed, Emergency Medicine

Chief complaint: Blood in urine noticed this morning, described as present at the beginning of urination but clearing by the end of the stream

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Blood in urine noticed this morning, described as present at the beginning of urination but clearing by the end of the stream

HISTORY OF PRESENT ILLNESS:
The patient with known urethral stricture presents with blood in urine. He describes the blood as being present at the beginning of urination but clears by the end of the stream. He denies any recent trauma or vigorous exercise.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic bronchitis (diagnosed 2020-01-15)
- Paget disease of bone (left tibia) (diagnosed 2020-07-15)
- Primary myelofibrosis (diagnosed 2021-01-15)
- Age-related cognitive decline (diagnosed 2022-01-15)
- Uncomplicated lumbar radiculopathy (diagnosed 2022-07-15)
- Hypertension
- Chronic kidney disease
- Benign prostatic hyperplasia
- Lumbar osteoarthritis with radiculopathy
- Mild neurocognitive disorder (age-related)
- Carotid sinus hypersensitivity
- Urethral stricture

His past medical history includes benign prostatic hyperplasia and hypertension,.

PAST SURGICAL HISTORY:
- Cystoscopy with urethral dilation (age 64)
- Appendectomy (age 34)
- Diagnostic bone biopsy left tibia (age 65)

MEDICATIONS:
He takes which are managed with tamsulosin and lisinopril, respectively.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Dementia with behavioral disturbances
- Older brother: Paget disease of bone

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 2 years ago. Alcohol: Occasional, 1-2 drinks per month. Occupation: Retired construction foreman.

VITAL SIGNS:
On physical examination, his blood pressure is 145/90 mmHg, heart rate is 78 beats per minute, and he is afebrile.

PHYSICAL EXAMINATION:
Abdominal examination is unremarkable with no flank tenderness. Genitourinary examination reveals a mildly enlarged, non-tender prostate. There is no meatal discharge or lesions.

LABORATORY DATA:
- Urinalysis: Red blood cells present, no casts
- Serum creatinine: 1.1 mg/dL
- Complete blood count: Within normal limits
- Coagulation profile: Within normal limits

## Encounter 6: source 2023-07-15, practice 2025-10-10, ed, Emergency Medicine

Chief complaint: Recurrent episodes of dizziness and one episode of syncope while shaving this morning preceded by light-headedness

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Fatima Al-Hassan, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Recurrent episodes of dizziness and one episode of syncope while shaving this morning preceded by light-headedness

HISTORY OF PRESENT ILLNESS:
This patient with carotid sinus hypersensitivity returns with dizziness and syncope that occurred while he was shaving this morning. He reports that the syncope was preceded by a sensation of light-headedness and "blacking out."

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic bronchitis (diagnosed 2020-01-15)
- Paget disease of bone (left tibia) (diagnosed 2020-07-15)
- Primary myelofibrosis (diagnosed 2021-01-15)
- Age-related cognitive decline (diagnosed 2022-01-15)
- Uncomplicated lumbar radiculopathy (diagnosed 2022-07-15)
- Urethral stricture (diagnosed 2023-01-15)
- Hypertension
- Chronic kidney disease
- Benign prostatic hyperplasia
- Lumbar osteoarthritis with radiculopathy
- Mild neurocognitive disorder (age-related)
- Carotid sinus hypersensitivity

His past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Cystoscopy with urethral dilation (age 64)
- Appendectomy (age 34)
- Diagnostic bone biopsy left tibia (age 65)

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg PO daily
- Tamsulosin 0.4 mg PO daily
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Dementia with behavioral disturbances
- Older brother: Paget disease of bone

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 2 years ago. Alcohol: Occasional, 1-2 drinks per month. Occupation: Retired construction foreman.

VITAL SIGNS:
On physical examination, his blood pressure is 145/90 mm Hg, and his heart rate is 78 beats per minute.

PHYSICAL EXAMINATION:
Carotid sinus massage is performed in a controlled setting, which results in a 4-second pause in ventricular rate and a drop in systolic blood pressure to 85 mm Hg. His neurological examination is unremarkable.

## Encounter 7: source 2024-01-15, practice 2026-04-14, icu, Critical Care Medicine

Chief complaint: Acute onset of agitation, confusion, severe headache, and flushed appearance 48 hours after admission for hypertensive emergency

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Robert Johnson, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Acute onset of agitation, confusion, severe headache, and flushed appearance 48 hours after admission for hypertensive emergency

HISTORY OF PRESENT ILLNESS:
The patient is a 70-year-old male with hypertension, chronic kidney disease, mild neurocognitive disorder, and primary myelofibrosis who was admitted for hypertensive emergency. He is started on an intravenous infusion of sodium nitroprusside. After 48 hours of treatment, the nursing staff reports that the patient has become increasingly agitated and confused. He complains of a severe headache and has become flushed. His wife mentions that he seems to be breathing more heavily than usual.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic bronchitis (diagnosed 2020-01-15)
- Paget disease of bone (left tibia) (diagnosed 2020-07-15)
- Primary myelofibrosis (diagnosed 2021-01-15)
- Age-related cognitive decline (diagnosed 2022-01-15)
- Uncomplicated lumbar radiculopathy (diagnosed 2022-07-15)
- Urethral stricture (diagnosed 2023-01-15)
- Carotid sinus hypersensitivity (diagnosed 2023-07-15)
- Hypertension
- Chronic kidney disease
- Benign prostatic hyperplasia
- Lumbar osteoarthritis with radiculopathy
- Mild neurocognitive disorder (age-related)

PAST SURGICAL HISTORY:
- Cystoscopy with urethral dilation (age 64)
- Appendectomy (age 34)
- Diagnostic bone biopsy left tibia (age 65)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Lisinopril 10 mg PO daily
- Tamsulosin 0.4 mg PO daily
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Dementia with behavioral disturbances
- Older brother: Paget disease of bone

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 2 years ago. Alcohol: Occasional, 1-2 drinks per month. Occupation: Retired construction foreman.

VITAL SIGNS:
On examination, his blood pressure is 145/90 mmHg, heart rate is 102 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 94% on room air.

LABORATORY DATA:
An arterial blood gas shows a pH of 7.35, PaCO2 of 45 mmHg, and PaO2 of 85 mmHg. Laboratory results are as follows:

- Serum sodium: 140 mEq/L
- Serum potassium: 4.2 mEq/L
- Serum creatinine: 2.8 mg/dL (baseline 2.5 mg/dL)
- Blood urea nitrogen: 55 mg/dL
- Lactic acid: 5 mmol/L
- Venous blood cyanide concentration: Pending

OTHER STUDIES:
The ECG shows sinus tachycardia with no other abnormalities.
