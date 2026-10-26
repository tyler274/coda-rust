# synthetic_hospital patient 2300

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 82 at first encounter, sex M, African American, Medicare and Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-04-18 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-09-15 | 2022-12-12 (Mon) | outpatient | Internal Medicine |
| 2 | 2021-09-15 | 2023-12-11 (Mon) | outpatient | Urology |
| 3 | 2023-01-15 | 2025-04-02 (Wed) | ed | Emergency Medicine |
| 4 | 2024-01-15 | 2026-04-01 (Wed) | icu | Neurology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare and Medicaid",
 "occupation": "Retired postal worker",
 "alcohol_use": "None",
 "family_history": [
  "Father with coronary artery disease",
  "Mother with hypertension and stroke",
  "Brother with type 2 diabetes mellitus"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Chronic Kidney Disease Stage 3b",
  "Hyperlipidemia",
  "Benign Prostatic Hyperplasia",
  "History of Ischemic Stroke with full recovery",
  "Severe Aortic Stenosis",
  "History of Intravenous Drug Use (remote)"
 ],
 "age_at_first_encounter": 82
}
```

Primary diagnoses: ["Acute bacterial prostatitis", "Listeria meningitis", "History of intravenous drug use", "Chronic Kidney Disease with Volume Overload", "Aortic valve stenosis"]
Comorbidities: ["Stage 1 chronic kidney disease", "Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2022-04-18, outpatient, Internal Medicine

Chief complaint: Increasing fatigue, decreased appetite, and mild ankle swelling over the past 4 months.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing fatigue, decreased appetite, and mild ankle swelling over the past 4 months.

HISTORY OF PRESENT ILLNESS:
He denies chest pain, shortness of breath, or changes in urination. He notes occasional lightheadedness when standing and has unintentionally lost 4 pounds in the past few months.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Chronic Kidney Disease Stage 3b
- Hyperlipidemia
- Benign Prostatic Hyperplasia
- History of Ischemic Stroke with full recovery
- Severe Aortic Stenosis
- History of Intravenous Drug Use (remote)

He has a history of hypertension, managed with multiple medications.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
His medication list includes a thiazide diuretic, amlodipine, and low-dose aspirin.

Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Family history is notable for stroke in his brother and type 2 diabetes in his daughter.

SOCIAL HISTORY:
He does not smoke or drink alcohol.

VITAL SIGNS:
On exam, blood pressure is 152/88 mm Hg, pulse is 72/min, and BMI is 26.

PHYSICAL EXAMINATION:
Cardiovascular and pulmonary exams are unremarkable. There is trace pedal edema. Funduscopy reveals mild arteriovenous nicking.

LABORATORY DATA:
Hemoglobin 11.5 g/dL
Creatinine 2.1 mg/dL
BUN 38 mg/dL
Urinalysis shows 1+ protein, no hematuria, and bland sediment. Spot urine protein-to-creatinine ratio is 0.3.

IMAGING:
Renal ultrasound reveals bilaterally echogenic kidneys measuring ~9.4 cm each.

OTHER STUDIES:
A biopsy is performed.

## Encounter 1: source 2020-09-15, practice 2022-12-12, outpatient, Internal Medicine

Chief complaint: Fever, sore throat, and rash on his chest and arms.

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Marcus Johnson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Fever, sore throat, and rash on his chest and arms.

HISTORY OF PRESENT ILLNESS:
The patient is an 82-year-old male with chronic kidney disease who returns eight months after evaluation for volume overload. He presents with a 5-day history of fever, sore throat, rash on his chest and arms, muscle aches, fatigue, and mild headache.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic Kidney Disease with Volume Overload (diagnosed 2020-01-15)
- Hypertension
- Chronic Kidney Disease Stage 3b
- Hyperlipidemia
- Benign Prostatic Hyperplasia
- History of Ischemic Stroke with full recovery
- Severe Aortic Stenosis
- History of Intravenous Drug Use (remote)

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with coronary artery disease
- Mother with hypertension and stroke
- Brother with type 2 diabetes mellitus

SOCIAL HISTORY:
Who lives in a shared nursing facility

PHYSICAL EXAMINATION:
On examination, he has nonexudative pharyngitis, cervical lymphadenopathy, and a faint maculopapular rash scattered across his chest and upper arms. His neck is not stiff.

LABORATORY DATA:
A rapid strep test and monospot test are both negative. An HIV ELISA is ordered.

## Encounter 2: source 2021-09-15, practice 2023-12-11, outpatient, Urology

Chief complaint: Severe pain, redness, and swelling in his right great toe for 3 days.

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Priya Patel, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Severe pain, redness, and swelling in his right great toe for 3 days.

HISTORY OF PRESENT ILLNESS:
The patient with CKD stage 3b, previously managed for volume overload, presents with severe toe pain. The pain began abruptly at night and worsened over the next 48 hours, making it difficult for him to walk. Two years ago, he had a similar episode in his left foot that resolved with treatment.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic Kidney Disease with Volume Overload (diagnosed 2020-01-15)
- History of intravenous drug use (diagnosed 2020-09-15)
- Hypertension
- Chronic Kidney Disease Stage 3b
- Hyperlipidemia
- Benign Prostatic Hyperplasia
- History of Ischemic Stroke with full recovery
- Severe Aortic Stenosis
- History of Intravenous Drug Use (remote)

He has a history of hypertension, chronic kidney disease, and hyperlipidemia.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes lisinopril, atorvastatin, and low-dose aspirin daily.

Home Medications:
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with coronary artery disease
- Mother with hypertension and stroke
- Brother with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired postal worker.

PHYSICAL EXAMINATION:
On examination, the metatarsophalangeal joint of the right great toe is markedly tender, warm, and erythematous with restricted range of motion.

OTHER STUDIES:
Synovial fluid aspiration is performed,

PLAN:
The patient is started on colchicine.

## Encounter 3: source 2023-01-15, practice 2025-04-02, ed, Emergency Medicine

Chief complaint: Collapsed while walking up a flight of stairs.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Collapsed while walking up a flight of stairs.

HISTORY OF PRESENT ILLNESS:
This 85-year-old with severe aortic stenosis and chronic kidney disease, evaluated in 2020 for volume overload, is brought to the emergency department by his daughter because he collapsed while walking up a flight of stairs. Over the past few months, he has experienced progressive fatigue and exertional chest tightness.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic Kidney Disease with Volume Overload (diagnosed 2020-01-15)
- History of intravenous drug use (diagnosed 2020-09-15)
- Acute bacterial prostatitis (diagnosed 2021-09-15)
- Hypertension
- Chronic Kidney Disease Stage 3b
- Hyperlipidemia
- Benign Prostatic Hyperplasia
- History of Ischemic Stroke with full recovery
- Severe Aortic Stenosis
- History of Intravenous Drug Use (remote)

His medical history includes hyperlipidemia and mild chronic kidney disease.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with coronary artery disease
- Mother with hypertension and stroke
- Brother with type 2 diabetes mellitus

SOCIAL HISTORY:
He does not smoke or drink alcohol.

VITAL SIGNS:
On examination, his blood pressure is 118/72 mm Hg, pulse is 72/min and regular, and respirations are 16/min

PHYSICAL EXAMINATION:
He appears pale and fatigued. Carotid upstroke is slow and delayed. Auscultation reveals a murmur that radiates to the neck.

OTHER STUDIES:
ECG shows left ventricular hypertrophy.

## Encounter 4: source 2024-01-15, practice 2026-04-01, icu, Neurology

Chief complaint: Progressive confusion and lethargy over the past 3 days.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Neurology
Visit Type: icu

CHIEF COMPLAINT:
Progressive confusion and lethargy over the past 3 days.

HISTORY OF PRESENT ILLNESS:
This 86-year-old with aortic stenosis, chronic kidney disease, and remote intravenous drug use returns after his 2023 ED visit. His daughter says he has been “not himself” and seemed weaker than usual after a recent family dinner. She recalls no head trauma or sick contacts, though he did complain of some mild abdominal cramps the day before presentation.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic Kidney Disease with Volume Overload (diagnosed 2020-01-15)
- History of intravenous drug use (diagnosed 2020-09-15)
- Acute bacterial prostatitis (diagnosed 2021-09-15)
- Aortic valve stenosis (diagnosed 2023-01-15)
- Hypertension
- Chronic Kidney Disease Stage 3b
- Hyperlipidemia
- Benign Prostatic Hyperplasia
- History of Ischemic Stroke with full recovery
- Severe Aortic Stenosis
- History of Intravenous Drug Use (remote)

His past history includes hypertension, benign prostatic hyperplasia, and a remote ischemic stroke with full recovery.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes aspirin and a thiazide.

Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with coronary artery disease
- Mother with hypertension and stroke
- Brother with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired postal worker.

VITAL SIGNS:
On arrival, his temperature is 38.5 °C (101.3 °F), blood pressure is 138/76 mm Hg, pulse is 94/min, and respiratory rate is 18/min.

PHYSICAL EXAMINATION:
Physical examination shows an elderly man with disorientation, mild nuchal rigidity, and photophobia. There are no focal motor deficits, and fundoscopic exam is normal. Abdominal exam is benign.

LABORATORY DATA:
A lumbar puncture is performed, and CSF studies show: Opening pressure Elevated Appearance Cloudy WBC count 1200/µL (90% neutrophils) Protein 210 mg/dL Glucose 25 mg/dL

IMAGING:
CT scan of the head shows age-related atrophy but no acute bleed.

ASSESSMENT AND PLAN:
This organism is characterized by entering the body through the gastrointestinal tract after contaminated food ingestion.
