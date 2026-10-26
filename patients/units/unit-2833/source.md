# synthetic_hospital patient 2833

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 78 at first encounter, sex F, Non-Hispanic White, Medicare with supplemental Medigap.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-11-30 (Mon) | outpatient | Rheumatology |
| 1 | 2020-09-15 | 2021-07-30 (Fri) | outpatient | Internal Medicine |
| 2 | 2021-09-15 | 2022-08-01 (Mon) | outpatient | Neurology |
| 3 | 2022-03-15 | 2023-01-23 (Mon) | outpatient | Internal Medicine |
| 4 | 2022-09-15 | 2023-07-31 (Mon) | ed | Emergency Medicine |
| 5 | 2023-09-15 | 2024-08-02 (Fri) | outpatient | Obstetrics & Gynecology |
| 6 | 2024-05-15 | 2025-04-07 (Mon) | ed | Emergency Medicine |
| 7 | 2025-01-15 | 2025-11-30 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare with supplemental Medigap",
 "occupation": "Retired elementary school teacher",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Type 2 diabetes mellitus",
  "Father: Hypertension",
  "Maternal grandmother: Primary open-angle glaucoma",
  "Maternal aunt: Dementia"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "2 mg PO BID",
   "name": "Tolterodine"
  },
  {
   "dose": "0.5% ophthalmic solution, 1 drop OU BID",
   "name": "Timolol maleate"
  },
  {
   "dose": "17 g PO daily",
   "name": "Polyethylene glycol 3350"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus",
  "Hypertension",
  "Primary open-angle glaucoma",
  "Chronic constipation",
  "Mild cognitive impairment",
  "Age-related sicca syndrome",
  "History of recurrent urinary tract infections"
 ],
 "age_at_first_encounter": 78
}
```

Primary diagnoses: ["Age-related sicca syndrome", "Adverse drug event due to sound-alike medication error", "Fecal impaction", "Medication-induced cognitive impairment (anticholinergic burden)", "Uncomplicated cystitis", "Urinary tract infection", "Vitamin B12 deficiency anemia"]
Comorbidities: ["Acute kidney injury", "Functional constipation or hemorrhoidal bleeding", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2020-11-30, outpatient, Rheumatology

Chief complaint: Persistent dry mouth and dry eyes for the past 6 months with difficulty swallowing dry foods

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent dry mouth and dry eyes for the past 6 months with difficulty swallowing dry foods

HISTORY OF PRESENT ILLNESS:
She reports difficulty swallowing dry foods and a burning sensation in her eyes, especially in windy conditions.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus
- Hypertension
- Primary open-angle glaucoma
- Chronic constipation
- Mild cognitive impairment
- Age-related sicca syndrome
- History of recurrent urinary tract infections

She has a past medical history of hypertension, for which she takes hydrochlorothiazide, and type 2 diabetes controlled with metformin.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She has a past medical history of hypertension, for which she takes hydrochlorothiazide, and type 2 diabetes controlled with metformin. Her medications do not include any anticholinergic drugs.

Home Medications:
- Lisinopril 10 mg PO daily
- Tolterodine 2 mg PO BID
- Timolol maleate 0.5% ophthalmic solution, 1 drop OU BID
- Polyethylene glycol 3350 17 g PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus
- Father: Hypertension
- Maternal grandmother: Primary open-angle glaucoma
- Maternal aunt: Dementia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired elementary school teacher.

REVIEW OF SYSTEMS:
She denies joint pain or rashes.

PHYSICAL EXAMINATION:
On physical examination, her oral mucosa appears dry, and Schirmer's test yields a measurement of 3 mm wetting after 5 minutes.

## Encounter 1: source 2020-09-15, practice 2021-07-30, outpatient, Internal Medicine

Chief complaint: Progressive weakness, fatigue, and tingling sensation in hands and feet over 4 months with recent fall

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive weakness, fatigue, and tingling sensation in hands and feet over 4 months with recent fall

HISTORY OF PRESENT ILLNESS:
The patient returns following her January 2020 rheumatology evaluation for sicca syndrome. The 78-year-old female with type 2 diabetes mellitus presents with a 4-month history of progressive weakness, fatigue, and a tingling sensation in her hands and feet, and reports a recent fall due to difficulty feeling the ground beneath her feet.

PAST MEDICAL HISTORY:
Active Problem List:
- Age-related sicca syndrome (diagnosed 2020-01-15)
- Type 2 diabetes mellitus
- Hypertension
- Primary open-angle glaucoma
- Chronic constipation
- Mild cognitive impairment
- History of recurrent urinary tract infections

Her medical history is significant for type 2 diabetes mellitus and hypertension, which are well-controlled with metformin and lisinopril.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She takes metformin and lisinopril.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Tolterodine 2 mg PO BID
- Timolol maleate 0.5% ophthalmic solution, 1 drop OU BID
- Polyethylene glycol 3350 17 g PO daily

ALLERGIES:
She has no known drug allergies

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus
- Father: Hypertension
- Maternal grandmother: Primary open-angle glaucoma
- Maternal aunt: Dementia

SOCIAL HISTORY:
She does not consume alcohol

VITAL SIGNS:
Her vitals are stable

PHYSICAL EXAMINATION:
On physical examination, she appears pale. Neurological examination reveals decreased vibration sense and proprioception in her lower extremities.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 9.8 g/dL (Normal: 12-15.5 g/dL)
- Mean corpuscular volume (MCV): 115 fL (Normal: 80-100 fL)
- Serum vitamin B12: 150 pg/mL (Normal: 200-900 pg/mL)
- Folate: 12 ng/mL (Normal: 3-17 ng/mL)
- Anti-intrinsic factor antibodies: Positive

## Encounter 2: source 2021-09-15, practice 2022-08-01, outpatient, Neurology

Chief complaint: Progressive memory loss and confusion over two months noted by family, worse in evenings

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Jennifer Okafor, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive memory loss and confusion over two months noted by family, worse in evenings

HISTORY OF PRESENT ILLNESS:
She presents with history of mild cognitive impairment and prior vitamin B12 deficiency. Over the past two months, she has been increasingly forgetful, misplacing items, and has difficulty recalling recent events. Her daughter also reports that her mother seems more confused in the evenings.

PAST MEDICAL HISTORY:
Active Problem List:
- Age-related sicca syndrome (diagnosed 2020-01-15)
- Vitamin B12 deficiency anemia (diagnosed 2020-09-15)
- Type 2 diabetes mellitus
- Hypertension
- Primary open-angle glaucoma
- Chronic constipation
- Mild cognitive impairment
- History of recurrent urinary tract infections

The patient has a history of hypertension, type 2 diabetes mellitus, chronic constipation, and primary open-angle glaucoma.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Her current medications include metoprolol, glipizide, polyethylene glycol, and timolol eye drops.

Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Tolterodine 2 mg PO BID
- Timolol maleate 0.5% ophthalmic solution, 1 drop OU BID
- Polyethylene glycol 3350 17 g PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus
- Father: Hypertension
- Maternal grandmother: Primary open-angle glaucoma
- Maternal aunt: Dementia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired elementary school teacher.

VITAL SIGNS:
Her blood pressure is 135/85 mmHg, and her pulse is 78 beats per minute and regular.

PHYSICAL EXAMINATION:
Physical examination reveals an alert elderly woman who is oriented to person and place but not to time. Her Mini-Mental State Examination score is 24/30, with deficits in attention and recall. There is no evidence of focal neurological deficits.

LABORATORY DATA:
Laboratory tests reveal:

- Serum sodium: 140 mEq/L
- Serum potassium: 4.2 mEq/L
- Serum creatinine: 1.0 mg/dL
- Fasting blood glucose: 148 mg/dL
- Hemoglobin A1c: 7.2%
- Thyroid-stimulating hormone (TSH): 2.5 μIU/mL

## Encounter 3: source 2022-03-15, practice 2023-01-23, outpatient, Internal Medicine

Chief complaint: Acute confusion and dizziness over the past week following recent changes to diabetes medication regimen

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Acute confusion and dizziness over the past week following recent changes to diabetes medication regimen

HISTORY OF PRESENT ILLNESS:
She returns with acute confusion following recent medication changes, with prior history of medication-induced cognitive impairment. She reports some confusion over the past week, particularly with her new medication regimen which was recently changed due to poor glycemic control. She also mentions that she sometimes feels dizzy in the mornings.

PAST MEDICAL HISTORY:
Active Problem List:
- Age-related sicca syndrome (diagnosed 2020-01-15)
- Vitamin B12 deficiency anemia (diagnosed 2020-09-15)
- Medication-induced cognitive impairment (anticholinergic burden) (diagnosed 2021-09-15)
- Type 2 diabetes mellitus
- Hypertension
- Primary open-angle glaucoma
- Chronic constipation
- Mild cognitive impairment
- History of recurrent urinary tract infections

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Her current medications include metformin, sitagliptin, and a newly prescribed antidiabetic agent that she cannot recall the name of, but mentions it sounds similar to another medication she takes for her blood pressure.

Home Medications:
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Tolterodine 2 mg PO BID
- Timolol maleate 0.5% ophthalmic solution, 1 drop OU BID
- Polyethylene glycol 3350 17 g PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus
- Father: Hypertension
- Maternal grandmother: Primary open-angle glaucoma
- Maternal aunt: Dementia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired elementary school teacher.

VITAL SIGNS:
Her blood pressure is 130/85 mmHg, and her pulse is 78 beats per minute.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin A1c: 8.2%
- Fasting blood glucose: 172 mg/dL
- Serum creatinine: 1.4 mg/dL (baseline 1.0 mg/dL)
- Estimated glomerular filtration rate (eGFR): 45 mL/min/1.73m^2

## Encounter 4: source 2022-09-15, practice 2023-07-31, ed, Emergency Medicine

Chief complaint: 3-day history of severe abdominal pain, bloating, and no bowel movement for 5 days with overflow incontinence

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
3-day history of severe abdominal pain, bloating, and no bowel movement for 5 days with overflow incontinence

HISTORY OF PRESENT ILLNESS:
This 80-year-old female with chronic constipation, prior medication-induced cognitive impairment, and a recent adverse drug event presents to the emergency department. She reports that she has not had a bowel movement in 5 days and is experiencing nausea without vomiting. She also mentions the recent passage of small amounts of liquid stool that she is unable to control.

PAST MEDICAL HISTORY:
Active Problem List:
- Age-related sicca syndrome (diagnosed 2020-01-15)
- Vitamin B12 deficiency anemia (diagnosed 2020-09-15)
- Medication-induced cognitive impairment (anticholinergic burden) (diagnosed 2021-09-15)
- Adverse drug event due to sound-alike medication error (diagnosed 2022-03-15)
- Type 2 diabetes mellitus
- Hypertension
- Primary open-angle glaucoma
- Chronic constipation
- Mild cognitive impairment
- History of recurrent urinary tract infections

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Her medications include metformin, lisinopril, and a daily fiber supplement.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Tolterodine 2 mg PO BID
- Timolol maleate 0.5% ophthalmic solution, 1 drop OU BID
- Polyethylene glycol 3350 17 g PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus
- Father: Hypertension
- Maternal grandmother: Primary open-angle glaucoma
- Maternal aunt: Dementia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired elementary school teacher.

VITAL SIGNS:
Her vital signs are stable

PHYSICAL EXAMINATION:
On physical examination, her abdomen is distended and tympanic to percussion with mild diffuse tenderness. Digital rectal examination reveals a hard mass in the rectal vault.

LABORATORY DATA:
Laboratory tests reveal:

- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 102 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen: 19 mg/dL
- Creatinine: 1.0 mg/dL
- Glucose: 158 mg/dL
- Calcium: 9.6 mg/dL

## Encounter 5: source 2023-09-15, practice 2024-08-02, outpatient, Obstetrics & Gynecology

Chief complaint: Sudden-onset urinary incontinence with dysuria and increased urinary frequency for 2 days

ENCOUNTER NOTE
Date: 2023-09-15
Provider: Dr. Emily Johnson, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Sudden-onset urinary incontinence with dysuria and increased urinary frequency for 2 days

HISTORY OF PRESENT ILLNESS:
With a history of recurrent urinary tract infections and prior overflow incontinence, she reports a strong urge to urinate followed by involuntary loss of urine, which has never occurred before. She also complains of dysuria and increased urinary frequency.

PAST MEDICAL HISTORY:
Active Problem List:
- Age-related sicca syndrome (diagnosed 2020-01-15)
- Vitamin B12 deficiency anemia (diagnosed 2020-09-15)
- Medication-induced cognitive impairment (anticholinergic burden) (diagnosed 2021-09-15)
- Adverse drug event due to sound-alike medication error (diagnosed 2022-03-15)
- Fecal impaction (diagnosed 2022-09-15)
- Type 2 diabetes mellitus
- Hypertension
- Primary open-angle glaucoma
- Chronic constipation
- Mild cognitive impairment
- History of recurrent urinary tract infections

Her past medical history includes well-controlled type 2 diabetes mellitus and hypertension.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She takes her blood pressure is currently managed with lisinopril.

Home Medications:
- Metformin 1000 mg PO BID
- Hydrochlorothiazide 25 mg PO daily
- Tolterodine 2 mg PO BID
- Timolol maleate 0.5% ophthalmic solution, 1 drop OU BID
- Polyethylene glycol 3350 17 g PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus
- Father: Hypertension
- Maternal grandmother: Primary open-angle glaucoma
- Maternal aunt: Dementia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired elementary school teacher.

REVIEW OF SYSTEMS:
She denies any fever, back pain, or vaginal discharge.

VITAL SIGNS:
On physical examination, her vital signs are within normal limits,

PHYSICAL EXAMINATION:
Her abdominal exam is unremarkable. A pelvic examination reveals no signs of prolapse or masses, but there is suprapubic tenderness.

LABORATORY DATA:
Urine dipstick analysis shows positive leukocyte esterase and trace blood. No nitrites are detected. The patient's most recent hemoglobin A1c was 6.8%,
Laboratory Values:
- Urine Leukocyte Esterase: Positive
- Urine Blood: Trace
- Urine Nitrites: Negative
- Hemoglobin A1c: 6.8%

## Encounter 6: source 2024-05-15, practice 2025-04-07, ed, Emergency Medicine

Chief complaint: Sudden onset of urinary incontinence and confusion over the past two days with reduced oral intake

ENCOUNTER NOTE
Date: 2024-05-15
Provider: Dr. Raj Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of urinary incontinence and confusion over the past two days with reduced oral intake

HISTORY OF PRESENT ILLNESS:
The patient is an 82-year-old female with a history of recurrent urinary tract infections, including a similar presentation eight months prior, and mild cognitive impairment with prior medication-induced confusion who is brought to the emergency department by her daughter due to a sudden onset of urinary incontinence and confusion over the past two days. Her daughter reports that the patient has been less active than usual and has not been drinking as much water because she fears falling when walking to the bathroom.

PAST MEDICAL HISTORY:
Active Problem List:
- Age-related sicca syndrome (diagnosed 2020-01-15)
- Vitamin B12 deficiency anemia (diagnosed 2020-09-15)
- Medication-induced cognitive impairment (anticholinergic burden) (diagnosed 2021-09-15)
- Adverse drug event due to sound-alike medication error (diagnosed 2022-03-15)
- Fecal impaction (diagnosed 2022-09-15)
- Uncomplicated cystitis (diagnosed 2023-09-15)
- Type 2 diabetes mellitus
- Hypertension
- Primary open-angle glaucoma
- Chronic constipation
- Mild cognitive impairment
- History of recurrent urinary tract infections

The patient has a history of hypertension, type 2 diabetes mellitus, and mild cognitive impairment.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Her medications include metformin, lisinopril, and hydrochlorothiazide.

Home Medications:
- Tolterodine 2 mg PO BID
- Timolol maleate 0.5% ophthalmic solution, 1 drop OU BID
- Polyethylene glycol 3350 17 g PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus
- Father: Hypertension
- Maternal grandmother: Primary open-angle glaucoma
- Maternal aunt: Dementia

SOCIAL HISTORY:
She lives alone and is usually able to perform activities of daily living without assistance.

VITAL SIGNS:
On examination, the patient is afebrile, blood pressure is 150/90 mmHg, pulse is 88/min, and respiratory rate is 16/min.

PHYSICAL EXAMINATION:
The abdominal exam reveals suprapubic tenderness. Neurological examination is nonfocal.

LABORATORY DATA:
A urinalysis shows pyuria and bacteriuria, but no hematuria or casts. Urine culture is pending.

Laboratory Values:
- Serum creatinine: 1.2 mg/dL (baseline 0.9 mg/dL)
- Blood urea nitrogen (BUN): 28 mg/dL
- Serum sodium: 142 mEq/L
- Serum potassium: 4.5 mEq/L
- Serum glucose: 180 mg/dL

## Encounter 7: source 2025-01-15, practice 2025-11-30, ed, Emergency Medicine

Chief complaint: Increasing confusion, agitation, and difficulty maintaining attention over the past 2 days

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Lisa Wong, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increasing confusion, agitation, and difficulty maintaining attention over the past 2 days

HISTORY OF PRESENT ILLNESS:
The patient returns to the emergency department with mental status changes similar to her May 2024 visit and is brought by her daughter because of a 2-day history of increasing confusion, agitation, and difficulty maintaining attention.

PAST MEDICAL HISTORY:
Active Problem List:
- Age-related sicca syndrome (diagnosed 2020-01-15)
- Vitamin B12 deficiency anemia (diagnosed 2020-09-15)
- Medication-induced cognitive impairment (anticholinergic burden) (diagnosed 2021-09-15)
- Adverse drug event due to sound-alike medication error (diagnosed 2022-03-15)
- Fecal impaction (diagnosed 2022-09-15)
- Uncomplicated cystitis (diagnosed 2023-09-15)
- Type 2 diabetes mellitus
- Hypertension
- Primary open-angle glaucoma
- Chronic constipation
- Mild cognitive impairment
- History of recurrent urinary tract infections

The patient has a history of hypertension, type 2 diabetes mellitus, and recurrent urinary tract infections.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Her medications include metformin, lisinopril, and tolterodine.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Timolol maleate 0.5% ophthalmic solution, 1 drop OU BID
- Polyethylene glycol 3350 17 g PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus
- Father: Hypertension
- Maternal grandmother: Primary open-angle glaucoma
- Maternal aunt: Dementia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired elementary school teacher.

VITAL SIGNS:
Her temperature is 38.3°C (100.9°F), blood pressure is 130/80 mmHg, pulse is 102/min, and respiratory rate is 22/min.

PHYSICAL EXAMINATION:
On examination, the patient is disoriented to time and place, with a fluctuating level of consciousness. She is agitated and appears to be responding to visual hallucinations.
Physical examination reveals suprapubic tenderness.

LABORATORY DATA:
Laboratory studies are as follows:

- White blood cell count: 12,000/mm³
- Hemoglobin: 13.2 g/dL
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 1.1 mg/dL
- Blood urea nitrogen: 19 mg/dL
- Urinalysis: Positive leukocyte esterase, nitrites, and numerous white blood cells
