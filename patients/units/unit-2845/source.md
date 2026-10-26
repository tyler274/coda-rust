# synthetic_hospital patient 2845

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 70 at first encounter, sex F, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-11-12 (Thu) | outpatient | Neurology |
| 1 | 2021-01-15 | 2021-11-17 (Wed) | outpatient | Ophthalmology |
| 2 | 2022-01-15 | 2022-11-10 (Thu) | outpatient | Internal Medicine |
| 3 | 2023-01-15 | 2023-11-13 (Mon) | outpatient | Cardiology |
| 4 | 2024-01-15 | 2024-11-06 (Wed) | outpatient | Psychiatry |
| 5 | 2024-07-15 | 2025-05-13 (Tue) | outpatient | Psychiatry |
| 6 | 2025-01-15 | 2025-11-10 (Mon) | ed | Emergency Medicine |
| 7 | 2025-07-15 | 2026-05-08 (Fri) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired administrative assistant",
 "alcohol_use": "None",
 "family_history": [
  "Alzheimer disease in mother",
  "Stroke in father",
  "Type 2 diabetes mellitus in sibling"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "40 pack-year history; quit 10 years ago but relapsed and currently smokes 1 pack per day",
 "home_medications": [
  {
   "dose": "4 mg daily",
   "name": "Risperidone"
  },
  {
   "dose": "10 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg daily",
   "name": "Amlodipine"
  },
  {
   "dose": "12.5 mg daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "20 mg daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "500 mg twice daily",
   "name": "Metformin"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Chronic schizophrenia",
  "Hypertension",
  "Hyperlipidemia",
  "Type 2 diabetes mellitus",
  "Alzheimer disease",
  "Dry age-related macular degeneration"
 ],
 "age_at_first_encounter": 70
}
```

Primary diagnoses: ["Neuroleptic-induced akathisia", "Transient ischemic attack", "Alzheimer disease", "Age-related macular degeneration, dry (non-exudative)", "Massive pulmonary embolism", "Oculogyric crisis due to antipsychotic medication", "Calcium channel blocker-induced peripheral edema", "Subclavian steal syndrome"]
Comorbidities: ["Schizophrenia", "Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2020-11-12, outpatient, Neurology

Chief complaint: Brought by her daughter because of worsening memory loss and functional decline over the past 2 years.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Brought by her daughter because of worsening memory loss and functional decline over the past 2 years.

HISTORY OF PRESENT ILLNESS:
She frequently misplaces household items, repeats the same questions during conversations, and occasionally becomes disoriented in familiar places. She has also developed irritability and difficulty managing daily finances.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic schizophrenia
- Hypertension
- Hyperlipidemia
- Type 2 diabetes mellitus
- Alzheimer disease
- Dry age-related macular degeneration

Her past medical history includes hypertension and hyperlipidemia, controlled with medications.

MEDICATIONS:
Home Medications:
- Risperidone 4 mg daily
- Lisinopril 10 mg daily
- Amlodipine 5 mg daily
- Hydrochlorothiazide 12.5 mg daily
- Atorvastatin 20 mg daily
- Metformin 500 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Family history is notable for a sibling who developed similar symptoms in late adulthood.

SOCIAL HISTORY:
She does not drink alcohol or use illicit substances.

PHYSICAL EXAMINATION:
On examination, she is alert but unable to recall three objects after five minutes and struggles with clock-drawing and serial subtraction tasks. Motor and sensory exams are normal, and gait is steady.

LABORATORY DATA:
Basic laboratory studies including thyroid function and vitamin B12 are within normal limits.

IMAGING:
MRI of the brain shows generalized cortical atrophy, most prominent in the temporal and parietal lobes.

PATHOLOGY:
Degeneration of neurons that primarily release

## Encounter 1: source 2021-01-15, practice 2021-11-17, outpatient, Ophthalmology

Chief complaint: Progressive difficulty reading and recognizing faces over the past six months with central vision blurriness.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Ophthalmology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty reading and recognizing faces over the past six months with central vision blurriness.

HISTORY OF PRESENT ILLNESS:
The patient is a 71-year-old female with a history of Alzheimer disease diagnosed in January 2020 who presents to the ophthalmology clinic complaining of progressive difficulty reading and recognizing faces over the past six months. She reports that her central vision is increasingly blurry and that when looking at a grid pattern, the lines appear wavy. She denies any pain or redness in her eyes.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer disease (diagnosed 2020-01-15)
- Chronic schizophrenia
- Hypertension
- Hyperlipidemia
- Type 2 diabetes mellitus
- Dry age-related macular degeneration

Her past medical history is significant for hypertension, controlled with lisinopril, and hypercholesterolemia, managed with atorvastatin.

MEDICATIONS:
Home Medications:
- Risperidone 4 mg daily
- Lisinopril 10 mg daily
- Amlodipine 5 mg daily
- Hydrochlorothiazide 12.5 mg daily
- Atorvastatin 20 mg daily
- Metformin 500 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Alzheimer disease in mother
- Stroke in father
- Type 2 diabetes mellitus in sibling

SOCIAL HISTORY:
She has a 30-pack-year smoking history but quit 10 years ago.

PHYSICAL EXAMINATION:
On physical examination, her visual acuity is 20/60 in both eyes. The anterior segment and pupillary responses are normal. Fundoscopic examination reveals bilateral drusen and pigmentary changes in the macula. No hemorrhages or exudates are noted.

## Encounter 2: source 2022-01-15, practice 2022-11-10, outpatient, Internal Medicine

Chief complaint: Bilateral lower extremity swelling that has progressively worsened over the past two weeks with 5-pound weight gain.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Jennifer Park, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Bilateral lower extremity swelling that has progressively worsened over the past two weeks with 5-pound weight gain.

HISTORY OF PRESENT ILLNESS:
This 72-year-old female with Alzheimer disease diagnosed in 2020 and dry age-related macular degeneration diagnosed in 2021 returns to the Internal Medicine clinic complaining of bilateral lower extremity swelling that has progressively worsened over the past two weeks. She denies any chest pain, dyspnea, orthopnea, or paroxysmal nocturnal dyspnea. She reports a weight gain of 5 pounds during this period.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer disease (diagnosed 2020-01-15)
- Age-related macular degeneration, dry (non-exudative) (diagnosed 2021-01-15)
- Chronic schizophrenia
- Hypertension
- Hyperlipidemia
- Type 2 diabetes mellitus
- Dry age-related macular degeneration

MEDICATIONS:
Her medications include amlodipine, hydrochlorothiazide, and atorvastatin.

Home Medications:
- Risperidone 4 mg daily
- Lisinopril 10 mg daily
- Metformin 500 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Alzheimer disease in mother
- Stroke in father
- Type 2 diabetes mellitus in sibling

SOCIAL HISTORY:
Smoking: 40 pack-year history; quit 10 years ago but relapsed and currently smokes 1 pack per day. Alcohol: None. Occupation: Retired administrative assistant.

VITAL SIGNS:
On physical examination, her blood pressure is 142/88 mmHg, heart rate is 78 beats per minute, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
Cardiac examination reveals a regular rate and rhythm without murmurs, rubs, or gallops. Lung fields are clear to auscultation bilaterally. There is 2+ pitting edema up to the mid-shins bilaterally.

LABORATORY DATA:
Laboratory values are as follows:

- Serum sodium: 140 mEq/L
- Serum potassium: 4.2 mEq/L
- Serum creatinine: 1.0 mg/dL
- B-type natriuretic peptide (BNP): 80 pg/mL (normal <100 pg/mL)
- Urinalysis: No proteinuria or hematuria

## Encounter 3: source 2023-01-15, practice 2023-11-13, outpatient, Cardiology

Chief complaint: Several episodes of dizziness and near-syncope occurring when using her left arm for tasks like combing hair or carrying groceries.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Thompson, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Several episodes of dizziness and near-syncope occurring when using her left arm for tasks like combing hair or carrying groceries.

HISTORY OF PRESENT ILLNESS:
The patient with Alzheimer disease, hypertension, and dry age-related macular degeneration presents with dizziness and near-syncope that occur when using her left arm for tasks like combing her hair or carrying groceries, as well as left arm easily fatigued with activity.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer disease (diagnosed 2020-01-15)
- Age-related macular degeneration, dry (non-exudative) (diagnosed 2021-01-15)
- Calcium channel blocker-induced peripheral edema (diagnosed 2022-01-15)
- Chronic schizophrenia
- Hypertension
- Hyperlipidemia
- Type 2 diabetes mellitus
- Dry age-related macular degeneration

She has a history of hypertension and hyperlipidemia.

MEDICATIONS:
Home Medications:
- Risperidone 4 mg daily
- Lisinopril 10 mg daily
- Amlodipine 5 mg daily
- Hydrochlorothiazide 12.5 mg daily
- Atorvastatin 20 mg daily
- Metformin 500 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Alzheimer disease in mother
- Stroke in father
- Type 2 diabetes mellitus in sibling

SOCIAL HISTORY:
She has smoked one pack of cigarettes daily for 40 years

VITAL SIGNS:
On examination, her blood pressure is 144/90 mm Hg in the right arm and 114/72 mm Hg in the left

PHYSICAL EXAMINATION:
The left radial pulse is decreased compared to the right, and femoral pulse is normal bilaterally. Neurologic exam is normal.

## Encounter 4: source 2024-01-15, practice 2024-11-06, outpatient, Psychiatry

Chief complaint: Increasing difficulty in initiating movements, stiffness in her arms and legs, and a resting tremor in her right hand over 2 months.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Lisa Wong, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing difficulty in initiating movements, stiffness in her arms and legs, and a resting tremor in her right hand over 2 months.

HISTORY OF PRESENT ILLNESS:
Patient is a 74-year-old female with chronic schizophrenia and Alzheimer disease diagnosed in 2020. She returns to the Psychiatry clinic with a 2-month history of increasing difficulty in initiating movements, stiffness in her arms and legs, and a resting tremor in her right hand. She has been stable on her antipsychotic medication for years. Her family notes that she has become less expressive and her voice has softened.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer disease (diagnosed 2020-01-15)
- Age-related macular degeneration, dry (non-exudative) (diagnosed 2021-01-15)
- Calcium channel blocker-induced peripheral edema (diagnosed 2022-01-15)
- Subclavian steal syndrome (diagnosed 2023-01-15)
- Chronic schizophrenia
- Hypertension
- Hyperlipidemia
- Type 2 diabetes mellitus
- Dry age-related macular degeneration

MEDICATIONS:
Her most recent medication regimen includes risperidone 4 mg daily, started 6 months ago. Her other medications include lisinopril for hypertension and atorvastatin for hyperlipidemia.

Home Medications:
- Amlodipine 5 mg daily
- Hydrochlorothiazide 12.5 mg daily
- Metformin 500 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Alzheimer disease in mother
- Stroke in father
- Type 2 diabetes mellitus in sibling

SOCIAL HISTORY:
Smoking: 40 pack-year history; quit 10 years ago but relapsed and currently smokes 1 pack per day. Alcohol: None. Occupation: Retired administrative assistant.

PHYSICAL EXAMINATION:
On examination, she exhibits a shuffling gait, bilateral cogwheel rigidity, and bradykinesia. There is no evidence of a pill-rolling tremor.

## Encounter 5: source 2024-07-15, practice 2025-05-13, outpatient, Psychiatry

Chief complaint: Repetitive chewing motions and occasional tongue protrusion observed by her daughter over the past 6 months.

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Lisa Wong, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Repetitive chewing motions and occasional tongue protrusion observed by her daughter over the past 6 months.

HISTORY OF PRESENT ILLNESS:
The patient is a 74-year-old with chronic schizophrenia who returns to the psychiatry clinic with her daughter following a January 2024 visit for movement side effects, and her daughter reports that over the past 6 months, her mother has developed repetitive chewing motions and occasional tongue protrusion. The patient has been stable on her antipsychotic medication for 10 years.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer disease (diagnosed 2020-01-15)
- Age-related macular degeneration, dry (non-exudative) (diagnosed 2021-01-15)
- Calcium channel blocker-induced peripheral edema (diagnosed 2022-01-15)
- Subclavian steal syndrome (diagnosed 2023-01-15)
- Neuroleptic-induced akathisia (diagnosed 2024-01-15)
- Chronic schizophrenia
- Hypertension
- Hyperlipidemia
- Type 2 diabetes mellitus
- Dry age-related macular degeneration

MEDICATIONS:
She takes risperidone 4 mg daily, metformin for type 2 diabetes, and atorvastatin for hyperlipidemia.

Home Medications:
- Lisinopril 10 mg daily
- Amlodipine 5 mg daily
- Hydrochlorothiazide 12.5 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Alzheimer disease in mother
- Stroke in father
- Type 2 diabetes mellitus in sibling

SOCIAL HISTORY:
Smoking: 40 pack-year history; quit 10 years ago but relapsed and currently smokes 1 pack per day. Alcohol: None. Occupation: Retired administrative assistant.

VITAL SIGNS:
Her vital signs are within normal limits

PHYSICAL EXAMINATION:
On examination, the patient exhibits involuntary, rhythmic movements of her jaw and tongue. She also has intermittent grimacing and blinking. There is no evidence of rigidity, tremor, or bradykinesia.

LABORATORY DATA:
Fasting glucose: 110 mg/dL
HbA1c: 6.8%
Total cholesterol: 190 mg/dL
LDL cholesterol: 100 mg/dL
HDL cholesterol: 50 mg/dL
Triglycerides: 200 mg/dL

## Encounter 6: source 2025-01-15, practice 2025-11-10, ed, Emergency Medicine

Chief complaint: Sudden onset of right arm weakness and difficulty speaking that resolved completely within 45 minutes.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of right arm weakness and difficulty speaking that resolved completely within 45 minutes.

HISTORY OF PRESENT ILLNESS:
This 75-year-old female with Alzheimer disease and subclavian steal syndrome presents with sudden onset of right arm weakness and difficulty speaking that resolved completely within 45 minutes. She denies any headache, trauma, or loss of consciousness.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer disease (diagnosed 2020-01-15)
- Age-related macular degeneration, dry (non-exudative) (diagnosed 2021-01-15)
- Calcium channel blocker-induced peripheral edema (diagnosed 2022-01-15)
- Subclavian steal syndrome (diagnosed 2023-01-15)
- Neuroleptic-induced akathisia (diagnosed 2024-01-15)
- Oculogyric crisis due to antipsychotic medication (diagnosed 2024-07-15)
- Chronic schizophrenia
- Hypertension
- Hyperlipidemia
- Type 2 diabetes mellitus
- Dry age-related macular degeneration

MEDICATIONS:
Her medications include lisinopril and atorvastatin.

Home Medications:
- Risperidone 4 mg daily
- Amlodipine 5 mg daily
- Hydrochlorothiazide 12.5 mg daily
- Metformin 500 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Alzheimer disease in mother
- Stroke in father
- Type 2 diabetes mellitus in sibling

SOCIAL HISTORY:
Smoking: 40 pack-year history; quit 10 years ago but relapsed and currently smokes 1 pack per day. Alcohol: None. Occupation: Retired administrative assistant.

VITAL SIGNS:
Her blood pressure is 155/90 mmHg, heart rate is 78 bpm

PHYSICAL EXAMINATION:
Her neurological examination is now normal

IMAGING:
Non-contrast head CT is unremarkable.

OTHER STUDIES:
An ECG shows normal sinus rhythm without evidence of atrial fibrillation.

## Encounter 7: source 2025-07-15, practice 2026-05-08, icu, Emergency Medicine

Chief complaint: Sudden episode of syncope preceded by brief chest pain and shortness of breath after climbing stairs.

ENCOUNTER NOTE
Date: 2025-07-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Sudden episode of syncope preceded by brief chest pain and shortness of breath after climbing stairs.

HISTORY OF PRESENT ILLNESS:
The patient with chronic schizophrenia, Alzheimer disease, and prior TIA presents acutely. She reports that earlier in the day, she experienced brief chest pain and shortness of breath after climbing a flight of stairs.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer disease (diagnosed 2020-01-15)
- Age-related macular degeneration, dry (non-exudative) (diagnosed 2021-01-15)
- Calcium channel blocker-induced peripheral edema (diagnosed 2022-01-15)
- Subclavian steal syndrome (diagnosed 2023-01-15)
- Neuroleptic-induced akathisia (diagnosed 2024-01-15)
- Oculogyric crisis due to antipsychotic medication (diagnosed 2024-07-15)
- Transient ischemic attack (diagnosed 2025-01-15)
- Chronic schizophrenia
- Hypertension
- Hyperlipidemia
- Type 2 diabetes mellitus
- Dry age-related macular degeneration

MEDICATIONS:
Her medications include lisinopril and atorvastatin.

Home Medications:
- Risperidone 4 mg daily
- Amlodipine 5 mg daily
- Hydrochlorothiazide 12.5 mg daily
- Metformin 500 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Alzheimer disease in mother
- Stroke in father
- Type 2 diabetes mellitus in sibling

SOCIAL HISTORY:
Smoking: 40 pack-year history; quit 10 years ago but relapsed and currently smokes 1 pack per day. Alcohol: None. Occupation: Retired administrative assistant.

VITAL SIGNS:
Her blood pressure is 85/50 mmHg, heart rate is 120 beats per minute, respiratory rate is 28 breaths per minute, and oxygen saturation is 88% on room air.
The patient's blood pressure drops further to 75/40 mmHg despite fluid resuscitation.

PHYSICAL EXAMINATION:
On examination, she appears diaphoretic and in distress.
Jugular venous pressure is elevated. Lung auscultation reveals clear breath sounds bilaterally. Cardiac examination shows tachycardia without murmurs, rubs, or gallops. Extremities are cool to touch, and there is no peripheral edema.

LABORATORY DATA:
Laboratory tests reveal:

- Troponin I: 0.05 ng/mL (normal <0.04 ng/mL)
- B-type natriuretic peptide (BNP): 250 pg/mL (normal <100 pg/mL)
- D-dimer: 5000 ng/mL FEU (normal <500 ng/mL FEU)

IMAGING:
A bedside echocardiogram demonstrates right ventricular dilation with hypokinesis of the right ventricular free wall.

OTHER STUDIES:
An electrocardiogram shows sinus tachycardia and a S1Q3T3 pattern.
