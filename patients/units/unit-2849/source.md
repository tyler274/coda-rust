# synthetic_hospital patient 2849

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex F, Non-Hispanic White, Medicare Part B with supplemental private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2019-07-01 (Mon) | outpatient | Orthopedics |
| 1 | 2020-07-15 | 2019-12-26 (Thu) | outpatient | Internal Medicine |
| 2 | 2021-01-15 | 2020-06-29 (Mon) | outpatient | Obstetrics & Gynecology |
| 3 | 2023-01-15 | 2022-07-06 (Wed) | ed | Emergency Medicine |
| 4 | 2023-01-15 | 2022-07-06 (Wed) | ed | Emergency Medicine |
| 5 | 2023-02-15 | 2022-08-06 (Sat) | icu | Critical Care |
| 6 | 2027-01-15 | 2026-06-27 (Sat) | ed | Emergency Medicine |
| 7 | 2027-02-15 | 2026-07-23 (Thu) | outpatient | Neurology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare Part B with supplemental private insurance",
 "occupation": "Retired elementary school teacher; part-time library archivist",
 "alcohol_use": "Occasional (1-2 glasses of wine per week)",
 "family_history": [
  "Mother: Breast cancer, age 65",
  "Father: Chronic obstructive pulmonary disease and hypertension",
  "Sister: Multiple sclerosis",
  "Paternal grandmother: Hip fracture (osteoporosis)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 25 years ago (15 pack-years)",
 "home_medications": [
  {
   "dose": "60 mg PO three times daily",
   "name": "Pyridostigmine"
  },
  {
   "dose": "200 mg PO twice daily",
   "name": "Theophylline"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "500 mg PO as needed for pain",
   "name": "Acetaminophen"
  },
  {
   "dose": "2000 IU PO daily",
   "name": "Vitamin D3"
  },
  {
   "dose": "500 mg PO twice daily",
   "name": "Calcium carbonate"
  }
 ],
 "surgical_history": [
  "Thymectomy (2005)",
  "Diagnostic knee arthroscopy (2015)",
  "Appendectomy (remote history)"
 ],
 "chronic_conditions": [
  "Myasthenia gravis (generalized, s/p thymectomy)",
  "Multiple sclerosis (relapsing-remitting with neurogenic bladder requiring intermittent catheterization)",
  "Chronic obstructive pulmonary disease (COPD), moderate severity",
  "Hypertension",
  "Osteoarthritis, bilateral knees",
  "Osteopenia"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Myasthenic crisis", "Theophylline toxicity", "Primary hyperparathyroidism", "Ovarian cancer", "Urinary tract infection", "Creutzfeldt-Jakob disease", "Osteoarthritis of knee"]
Comorbidities: ["Essential hypertension", "Urinary tract infection", "Metformin-associated encephalopathy"]

## Encounter 0: source 2020-01-15, practice 2019-07-01, outpatient, Orthopedics

Chief complaint: 6-month history of increasing bilateral knee pain unresponsive to physical therapy

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of increasing bilateral knee pain unresponsive to physical therapy

HISTORY OF PRESENT ILLNESS:
She reports that the pain is worse with movement and at the end of the day. She rates her pain as 7 out of 10 on most days. She has tried physical therapy and weight loss without significant relief. She expresses a strong desire to avoid opioid medications and is interested in exploring other options that would allow her to maintain her daily walks with her dog, which she identifies as a key component of her quality of life.

PAST MEDICAL HISTORY:
Active Problem List:
- Myasthenia gravis (generalized, s/p thymectomy)
- Multiple sclerosis (relapsing-remitting with neurogenic bladder requiring intermittent catheterization)
- Chronic obstructive pulmonary disease (COPD), moderate severity
- Hypertension
- Osteoarthritis, bilateral knees
- Osteopenia

PAST SURGICAL HISTORY:
- Thymectomy (2005)
- Diagnostic knee arthroscopy (2015)
- Appendectomy (remote history)

MEDICATIONS:
Her current medications include hydrochlorothiazide and occasional acetaminophen.

Home Medications:
- Pyridostigmine 60 mg PO three times daily
- Theophylline 200 mg PO twice daily
- Vitamin D3 2000 IU PO daily
- Calcium carbonate 500 mg PO twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Breast cancer, age 65
- Father: Chronic obstructive pulmonary disease and hypertension
- Sister: Multiple sclerosis
- Paternal grandmother: Hip fracture (osteoporosis)

SOCIAL HISTORY:
Smoking: Former smoker, quit 25 years ago (15 pack-years). Alcohol: Occasional (1-2 glasses of wine per week). Occupation: Retired elementary school teacher; part-time library archivist.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, she has bilateral knee crepitus and mild effusion without significant warmth or erythema.

LABORATORY DATA:
Erythrocyte sedimentation rate (ESR): 20 mm/hr (normal: 0-30 mm/hr)
- C-reactive protein (CRP): 1.2 mg/L (normal: <3.0 mg/L)
- Complete blood count (CBC): within normal limits

## Encounter 1: source 2020-07-15, practice 2019-12-26, outpatient, Internal Medicine

Chief complaint: Persistent fatigue, diffuse musculoskeletal pain, constipation, and mild confusion over past few months

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue, diffuse musculoskeletal pain, constipation, and mild confusion over past few months

HISTORY OF PRESENT ILLNESS:
The patient with myasthenia gravis, multiple sclerosis, and COPD returns six months after orthopedic evaluation for bilateral knee osteoarthritis. She presents with fatigue, diffuse musculoskeletal pain, constipation, and mild confusion. She has also experienced generalized weakness and difficulty concentrating at work, often feeling "mentally foggy." She reports increased thirst with frequent urination but denies dysuria or hematuria. Her appetite has been reduced, leading to an unintentional 4 kg (9 lb) weight loss over three months.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Myasthenia gravis (generalized, s/p thymectomy)
- Multiple sclerosis (relapsing-remitting with neurogenic bladder requiring intermittent catheterization)
- Chronic obstructive pulmonary disease (COPD), moderate severity
- Hypertension
- Osteoarthritis, bilateral knees
- Osteopenia

Past medical history includes mild hypertension managed with lifestyle modifications.

PAST SURGICAL HISTORY:
- Thymectomy (2005)
- Diagnostic knee arthroscopy (2015)
- Appendectomy (remote history)

MEDICATIONS:
She does not take any medications or supplements,

Home Medications:
- Pyridostigmine 60 mg PO three times daily
- Theophylline 200 mg PO twice daily
- Hydrochlorothiazide 25 mg PO daily
- Acetaminophen 500 mg PO as needed for pain
- Vitamin D3 2000 IU PO daily
- Calcium carbonate 500 mg PO twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Her family history is notable for her father having kidney stones in his 50s.

SOCIAL HISTORY:
She does not smoke, and drinks alcohol occasionally.

VITAL SIGNS:
On examination, temperature is 36.7°C (98.1°F), pulse is 76/min, and blood pressure is 148/86 mmHg. BMI is 25 kg/m².

PHYSICAL EXAMINATION:
Neurologic examination is nonfocal. Musculoskeletal examination reveals generalized bony tenderness over her forearms and shins bilaterally. Cognitive testing demonstrates slowed recall and impaired attention.

IMAGING:
A dual-energy x-ray absorptiometry scan shows decreased bone mineral density consistent with osteopenia.

OTHER STUDIES:
A 24-hour urine collection is obtained as part of further evaluation.

## Encounter 2: source 2021-01-15, practice 2020-06-29, outpatient, Obstetrics & Gynecology

Chief complaint: Intermittent light vaginal spotting for the past month, occurring postmenopausally

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Emily Johnson, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent light vaginal spotting for the past month, occurring postmenopausally

HISTORY OF PRESENT ILLNESS:
This 68-year-old female with myasthenia gravis status post thymectomy and multiple sclerosis, previously evaluated for knee osteoarthritis and primary hyperparathyroidism, presents with intermittent light vaginal spotting. She describes it as "pinkish" and notices it mostly after bowel movements. She denies pelvic pain, discharge, urinary symptoms, or trauma. She went through menopause 8 years ago and has not had any bleeding since then.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Primary hyperparathyroidism (diagnosed 2020-07-15)
- Myasthenia gravis (generalized, s/p thymectomy)
- Multiple sclerosis (relapsing-remitting with neurogenic bladder requiring intermittent catheterization)
- Chronic obstructive pulmonary disease (COPD), moderate severity
- Hypertension
- Osteoarthritis, bilateral knees
- Osteopenia

Her medical history includes well-controlled hypertension and osteopenia. She has never used hormone replacement therapy.

PAST SURGICAL HISTORY:
- Thymectomy (2005)
- Diagnostic knee arthroscopy (2015)
- Appendectomy (remote history)

MEDICATIONS:
Home Medications:
- Pyridostigmine 60 mg PO three times daily
- Theophylline 200 mg PO twice daily
- Hydrochlorothiazide 25 mg PO daily
- Acetaminophen 500 mg PO as needed for pain
- Vitamin D3 2000 IU PO daily
- Calcium carbonate 500 mg PO twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Breast cancer, age 65
- Father: Chronic obstructive pulmonary disease and hypertension
- Sister: Multiple sclerosis
- Paternal grandmother: Hip fracture (osteoporosis)

SOCIAL HISTORY:
Smoking: Former smoker, quit 25 years ago (15 pack-years). Alcohol: Occasional (1-2 glasses of wine per week). Occupation: Retired elementary school teacher; part-time library archivist.

PHYSICAL EXAMINATION:
Physical exam shows normal external genitalia and atrophic vaginal mucosa without visible lesions. On bimanual examination, the uterus is non-enlarged, but a firm left adnexal mass is palpated.

LABORATORY DATA:
Labs reveal estradiol levels above postmenopausal range and suppressed follicle-stimulating hormone.

IMAGING:
Transvaginal ultrasound reveals a complex ovarian mass with solid and cystic areas measuring 7 cm, and a homogeneously thickened endometrial stripe of 11 mm. Pelvic MRI confirms the mass is confined to the ovary.

OTHER STUDIES:
Endometrial biopsy is pending.

## Encounter 3: source 2023-01-15, practice 2022-07-06, ed, Emergency Medicine

Chief complaint: Fever, chills, and lower abdominal pain with cloudy, foul-smelling urine

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, chills, and lower abdominal pain with cloudy, foul-smelling urine

HISTORY OF PRESENT ILLNESS:
The patient with multiple sclerosis and neurogenic bladder presents. She reports difficulty voiding and chronic intermittent catheter use with cloudy, foul-smelling urine.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Primary hyperparathyroidism (diagnosed 2020-07-15)
- Ovarian cancer (diagnosed 2021-01-15)
- Myasthenia gravis (generalized, s/p thymectomy)
- Multiple sclerosis (relapsing-remitting with neurogenic bladder requiring intermittent catheterization)
- Chronic obstructive pulmonary disease (COPD), moderate severity
- Hypertension
- Osteoarthritis, bilateral knees
- Osteopenia

She has had multiple prior urinary tract infections requiring antibiotics.

PAST SURGICAL HISTORY:
- Thymectomy (2005)
- Diagnostic knee arthroscopy (2015)
- Appendectomy (remote history)

MEDICATIONS:
Home Medications:
- Pyridostigmine 60 mg PO three times daily
- Theophylline 200 mg PO twice daily
- Hydrochlorothiazide 25 mg PO daily
- Acetaminophen 500 mg PO as needed for pain
- Vitamin D3 2000 IU PO daily
- Calcium carbonate 500 mg PO twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Breast cancer, age 65
- Father: Chronic obstructive pulmonary disease and hypertension
- Sister: Multiple sclerosis
- Paternal grandmother: Hip fracture (osteoporosis)

SOCIAL HISTORY:
Smoking: Former smoker, quit 25 years ago (15 pack-years). Alcohol: Occasional (1-2 glasses of wine per week). Occupation: Retired elementary school teacher; part-time library archivist.

PHYSICAL EXAMINATION:
On physical examination, she has suprapubic tenderness

LABORATORY DATA:
Urinalysis reveals alkaline urine with numerous white blood cells and crystals. Urine culture grows Proteus spp .

IMAGING:
An x-ray is shown

## Encounter 4: source 2023-01-15, practice 2022-07-06, ed, Emergency Medicine

Chief complaint: Palpitations, nausea, and restlessness beginning a few hours ago

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Maria Gonzalez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Palpitations, nausea, and restlessness beginning a few hours ago

HISTORY OF PRESENT ILLNESS:
This 70-year-old female with generalized myasthenia gravis status post thymectomy, relapsing-remitting multiple sclerosis with neurogenic bladder, and moderate COPD presents to the emergency department. She reports that her symptoms started a few hours ago and have progressively worsened.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Primary hyperparathyroidism (diagnosed 2020-07-15)
- Ovarian cancer (diagnosed 2021-01-15)
- Urinary tract infection (diagnosed 2023-01-15)
- Myasthenia gravis (generalized, s/p thymectomy)
- Multiple sclerosis (relapsing-remitting with neurogenic bladder requiring intermittent catheterization)
- Chronic obstructive pulmonary disease (COPD), moderate severity
- Hypertension
- Osteoarthritis, bilateral knees
- Osteopenia

PAST SURGICAL HISTORY:
- Thymectomy (2005)
- Diagnostic knee arthroscopy (2015)
- Appendectomy (remote history)

MEDICATIONS:
She has been using theophylline for the past year to manage her COPD symptoms and recently started taking ciprofloxacin for a urinary tract infection.

Home Medications:
- Pyridostigmine 60 mg PO three times daily
- Hydrochlorothiazide 25 mg PO daily
- Acetaminophen 500 mg PO as needed for pain
- Vitamin D3 2000 IU PO daily
- Calcium carbonate 500 mg PO twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Breast cancer, age 65
- Father: Chronic obstructive pulmonary disease and hypertension
- Sister: Multiple sclerosis
- Paternal grandmother: Hip fracture (osteoporosis)

SOCIAL HISTORY:
Smoking: Former smoker, quit 25 years ago (15 pack-years). Alcohol: Occasional (1-2 glasses of wine per week). Occupation: Retired elementary school teacher; part-time library archivist.

VITAL SIGNS:
Her vital signs are as follows: blood pressure 150/90 mmHg, heart rate 110 beats per minute, respiratory rate 20 breaths per minute, and temperature 98.6°F (37°C).

PHYSICAL EXAMINATION:
Physical examination reveals mild bilateral wheezing on auscultation.

LABORATORY DATA:
Laboratory values are as follows:

- Serum theophylline level: 25 µg/mL (therapeutic range: 5-15 µg/mL)
- Serum creatinine: 1.2 mg/dL
- Blood urea nitrogen (BUN): 18 mg/dL
- Liver function tests: within normal limits

OTHER STUDIES:
An ECG shows sinus tachycardia with no other abnormalities.

## Encounter 5: source 2023-02-15, practice 2022-08-06, icu, Critical Care

Chief complaint: Difficulty breathing and increased weakness over the past two days after starting antibiotics

ENCOUNTER NOTE
Date: 2023-02-15
Provider: Dr. Aisha Patel, MD
Department: Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Difficulty breathing and increased weakness over the past two days after starting antibiotics

HISTORY OF PRESENT ILLNESS:
This 70-year-old female with myasthenia gravis returns following recent urinary tract infection and theophylline toxicity. She reports that her symptoms have progressively worsened since she started taking antibiotics for a urinary tract infection. She is using accessory muscles to breathe and is only able to speak in short sentences.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Primary hyperparathyroidism (diagnosed 2020-07-15)
- Ovarian cancer (diagnosed 2021-01-15)
- Urinary tract infection (diagnosed 2023-01-15)
- Theophylline toxicity (diagnosed 2023-01-15)
- Myasthenia gravis (generalized, s/p thymectomy)
- Multiple sclerosis (relapsing-remitting with neurogenic bladder requiring intermittent catheterization)
- Chronic obstructive pulmonary disease (COPD), moderate severity
- Hypertension
- Osteoarthritis, bilateral knees
- Osteopenia

PAST SURGICAL HISTORY:
- Thymectomy (2005)
- Diagnostic knee arthroscopy (2015)
- Appendectomy (remote history)

MEDICATIONS:
Home Medications:
- Pyridostigmine 60 mg PO three times daily
- Theophylline 200 mg PO twice daily
- Hydrochlorothiazide 25 mg PO daily
- Acetaminophen 500 mg PO as needed for pain
- Vitamin D3 2000 IU PO daily
- Calcium carbonate 500 mg PO twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Breast cancer, age 65
- Father: Chronic obstructive pulmonary disease and hypertension
- Sister: Multiple sclerosis
- Paternal grandmother: Hip fracture (osteoporosis)

SOCIAL HISTORY:
Smoking: Former smoker, quit 25 years ago (15 pack-years). Alcohol: Occasional (1-2 glasses of wine per week). Occupation: Retired elementary school teacher; part-time library archivist.

VITAL SIGNS:
Her temperature is 37.8°C (100°F), blood pressure is 145/90 mmHg, pulse is 88/min, and respiratory rate is 26/min with an oxygen saturation of 92% on room air.

PHYSICAL EXAMINATION:
Physical examination reveals bilateral ptosis, diplopia, and diminished breath sounds bilaterally with prolonged expiration. She has a weak cough and is unable to lift her head off the pillow.

LABORATORY DATA:
Arterial blood gas shows a pH of 7.32, PaCO2 of 50 mmHg, PaO2 of 70 mmHg, and HCO3- of 24 mEq/L.

## Encounter 6: source 2027-01-15, practice 2026-06-27, ed, Emergency Medicine

Chief complaint: Progressive memory loss, confusion, and unusual behavior over two months

ENCOUNTER NOTE
Date: 2027-01-15
Provider: Dr. Robert Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive memory loss, confusion, and unusual behavior over two months

HISTORY OF PRESENT ILLNESS:
This 74-year-old female with generalized myasthenia gravis and relapsing-remitting multiple sclerosis, who previously experienced mild confusion with hyperparathyroidism, is brought to the emergency department by her husband due to a 2-month history of progressive memory loss, confusion, and unusual behavior. He reports that she has become increasingly forgetful, often repeating questions several times within a conversation. She has also experienced visual disturbances, including difficulty recognizing familiar faces and objects.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Primary hyperparathyroidism (diagnosed 2020-07-15)
- Ovarian cancer (diagnosed 2021-01-15)
- Urinary tract infection (diagnosed 2023-01-15)
- Theophylline toxicity (diagnosed 2023-01-15)
- Myasthenic crisis (diagnosed 2023-02-15)
- Myasthenia gravis (generalized, s/p thymectomy)
- Multiple sclerosis (relapsing-remitting with neurogenic bladder requiring intermittent catheterization)
- Chronic obstructive pulmonary disease (COPD), moderate severity
- Hypertension
- Osteoarthritis, bilateral knees
- Osteopenia

Her past medical history is significant for hypertension,.

PAST SURGICAL HISTORY:
- Thymectomy (2005)
- Diagnostic knee arthroscopy (2015)
- Appendectomy (remote history)

MEDICATIONS:
She takes controlled with hydrochlorothiazide.

Home Medications:
- Pyridostigmine 60 mg PO three times daily
- Theophylline 200 mg PO twice daily
- Acetaminophen 500 mg PO as needed for pain
- Vitamin D3 2000 IU PO daily
- Calcium carbonate 500 mg PO twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Breast cancer, age 65
- Father: Chronic obstructive pulmonary disease and hypertension
- Sister: Multiple sclerosis
- Paternal grandmother: Hip fracture (osteoporosis)

SOCIAL HISTORY:
Smoking: Former smoker, quit 25 years ago (15 pack-years). Alcohol: Occasional (1-2 glasses of wine per week). Occupation: Retired elementary school teacher; part-time library archivist.

PHYSICAL EXAMINATION:
On examination, she appears disoriented to time and place. Neurological examination reveals myoclonic jerks in the upper extremities and a positive startle response.

LABORATORY DATA:
A lumbar puncture is performed, and CSF analysis is unremarkable for common causes of encephalitis.

IMAGING:
MRI of the brain demonstrates diffuse cortical atrophy without evidence of stroke or mass lesions.

OTHER STUDIES:
An EEG shows generalized triphasic sharp wave complexes.

PLAN:
Given the rapid progression of symptoms and the need for a definitive diagnosis, a specialized test is ordered.

## Encounter 7: source 2027-02-15, practice 2026-07-23, outpatient, Neurology

Chief complaint: Worsening memory loss, erratic behavior, difficulty walking, and sudden muscle jerks

ENCOUNTER NOTE
Date: 2027-02-15
Provider: Dr. James Wilson, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening memory loss, erratic behavior, difficulty walking, and sudden muscle jerks

HISTORY OF PRESENT ILLNESS:
The patient is a 74-year-old female with generalized myasthenia gravis status post thymectomy and relapsing-remitting multiple sclerosis who returns to the neurology clinic for follow-up of progressive cognitive decline initially evaluated one month prior. She is brought to the clinic by her husband due to a 3-month history of memory loss, erratic behavior, and difficulty walking. He reports that she has had several episodes of sudden muscle jerks. She has no history of recent travel or sick contacts.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Primary hyperparathyroidism (diagnosed 2020-07-15)
- Ovarian cancer (diagnosed 2021-01-15)
- Urinary tract infection (diagnosed 2023-01-15)
- Theophylline toxicity (diagnosed 2023-01-15)
- Myasthenic crisis (diagnosed 2023-02-15)
- Creutzfeldt-Jakob disease (diagnosed 2027-01-15)
- Myasthenia gravis (generalized, s/p thymectomy)
- Multiple sclerosis (relapsing-remitting with neurogenic bladder requiring intermittent catheterization)
- Chronic obstructive pulmonary disease (COPD), moderate severity
- Hypertension
- Osteoarthritis, bilateral knees
- Osteopenia

Her past medical history is significant for hypertension, which is well-controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Thymectomy (2005)
- Diagnostic knee arthroscopy (2015)
- Appendectomy (remote history)

MEDICATIONS:
Home Medications:
- Pyridostigmine 60 mg PO three times daily
- Theophylline 200 mg PO twice daily
- Hydrochlorothiazide 25 mg PO daily
- Acetaminophen 500 mg PO as needed for pain
- Vitamin D3 2000 IU PO daily
- Calcium carbonate 500 mg PO twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Breast cancer, age 65
- Father: Chronic obstructive pulmonary disease and hypertension
- Sister: Multiple sclerosis
- Paternal grandmother: Hip fracture (osteoporosis)

SOCIAL HISTORY:
Smoking: Former smoker, quit 25 years ago (15 pack-years). Alcohol: Occasional (1-2 glasses of wine per week). Occupation: Retired elementary school teacher; part-time library archivist.

PHYSICAL EXAMINATION:
On examination, she is alert but disoriented to time and place. Neurological examination reveals myoclonic jerks in the upper extremities and a broad-based gait.

LABORATORY DATA:
Routine blood tests are normal. A lumbar puncture is performed, and the cerebrospinal fluid analysis reveals:

- WBC: 2 /mm³ (normal 0-5 /mm³)
- RBC: 0 /mm³ (normal 0-0 /mm³)
- Protein: 45 mg/dL (normal 15-45 mg/dL)
- Glucose: 60 mg/dL (normal 45-80 mg/dL)
- 14-3-3 protein: Elevated

IMAGING:
An MRI of the brain shows no acute changes.
