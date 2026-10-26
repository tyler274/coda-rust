# synthetic_hospital patient 2034

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 55 at first encounter, sex M, African American, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-06-04 (Fri) | outpatient | Transplant Medicine |
| 1 | 2020-03-15 | 2021-08-23 (Mon) | outpatient | Nephrology |
| 2 | 2020-07-15 | 2021-12-08 (Wed) | outpatient | Transplant Medicine |
| 3 | 2020-11-15 | 2022-04-21 (Thu) | outpatient | Transplant Medicine |
| 4 | 2021-05-15 | 2022-10-18 (Tue) | ed | Emergency Medicine |
| 5 | 2021-11-15 | 2023-04-17 (Mon) | ed | Emergency Medicine |
| 6 | 2022-05-15 | 2023-10-15 (Sun) | icu | Critical Care |
| 7 | 2025-01-15 | 2026-06-15 (Mon) | outpatient | Transplant Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired postal worker",
 "alcohol_use": "None",
 "family_history": [
  "Father: Hypertension, chronic kidney disease",
  "Sister: Bipolar disorder",
  "Maternal uncle: End-stage renal disease"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Former smoker, quit 5 years ago",
 "home_medications": [
  {
   "dose": "5 mg PO BID",
   "name": "Tacrolimus"
  },
  {
   "dose": "100 mg PO BID",
   "name": "Cyclosporine"
  },
  {
   "dose": "750 mg PO BID",
   "name": "Mycophenolate mofetil"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Prednisone"
  },
  {
   "dose": "Single-strength tablet PO daily",
   "name": "Trimethoprim-sulfamethoxazole"
  },
  {
   "dose": "300 mg PO BID",
   "name": "Lithium carbonate"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  }
 ],
 "surgical_history": [
  "Deceased-donor kidney transplantation (initial, 5 years prior)",
  "Deceased-donor kidney re-transplantation (6 months prior)"
 ],
 "chronic_conditions": [
  "End-stage renal disease secondary to hypertensive nephrosclerosis",
  "Status post deceased-donor kidney transplantation",
  "Hypertension",
  "Bipolar disorder type I"
 ],
 "age_at_first_encounter": 55
}
```

Primary diagnoses: ["Calcineurin inhibitor nephrotoxicity", "History of Pneumocystis jirovecii pneumonia", "Acute lung transplant rejection", "Interstitial cystitis", "Pelvic actinomycosis", "Cryptococcal meningitis", "Chronic kidney allograft rejection", "Nephrogenic Diabetes Insipidus"]
Comorbidities: ["History of kidney transplantation", "Chronic Kidney Disease with Volume Overload", "Long term immunosuppressive therapy", "End-stage renal disease on hemodialysis", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2021-06-04, outpatient, Transplant Medicine

Chief complaint: presents to the transplant medicine clinic for routine follow-up

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Transplant Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the transplant medicine clinic for routine follow-up

PAST MEDICAL HISTORY:
Active Problem List:
- End-stage renal disease secondary to hypertensive nephrosclerosis
- Status post deceased-donor kidney transplantation
- Hypertension
- Bipolar disorder type I

He underwent a cadaveric renal transplant 5 years ago for end-stage renal disease due to hypertensive nephrosclerosis. His course since transplantation has been complicated by several rejection episodes, one of which required the use of high-dose cyclophosphamide. At that time, he developed severe hematuria and suprapubic discomfort, ultimately diagnosed as hemorrhagic cystitis. He was treated with aggressive hydration and mesna, which allowed him to continue receiving the medication without recurrence of bladder toxicity. Since then, his care team has continued to use careful dosing strategies and prophylaxis whenever cyclophosphamide is indicated.

PAST SURGICAL HISTORY:
- Deceased-donor kidney transplantation (initial, 5 years prior)
- Deceased-donor kidney re-transplantation (6 months prior)

MEDICATIONS:
Despite the many hours spent fishing, he maintains adherence to his medications, which include immunosuppressants and prophylactic antibiotics.

Home Medications:
- Tacrolimus 5 mg PO BID
- Cyclosporine 100 mg PO BID
- Mycophenolate mofetil 750 mg PO BID
- Prednisone 5 mg PO daily
- Trimethoprim-sulfamethoxazole Single-strength tablet PO daily
- Lithium carbonate 300 mg PO BID
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Family history is notable for hypertension in both parents.

SOCIAL HISTORY:
The patient describes himself as an avid fisherman. He recalls competing in regional bass tournaments throughout his 20s and 30s, even briefly traveling across state lines for professional competitions. Although he no longer competes professionally, he still goes fishing 2–3 times per week and often brings along family and friends. He jokes that most of his “therapy” comes from quiet mornings at the lake rather than hospital visits.
He denies alcohol use and has quit smoking.

REVIEW OF SYSTEMS:
He denies current hematuria, dysuria, or frequency. He reports no recent fevers, chills, or night sweats. His review of systems is otherwise unremarkable.

VITAL SIGNS:
Blood pressure is 128/82 mmHg, pulse 76/min, and respirations 14/min.

PHYSICAL EXAMINATION:
On exam, he appears healthy and in no acute distress.
Heart and lung exams are normal. Abdominal exam shows a well-healed surgical scar with no tenderness or organomegaly. Extremities have no edema. Neurologic exam is nonfocal.

LABORATORY DATA:
Laboratory evaluation today shows normal serum creatinine, electrolytes, and complete blood count. Urinalysis is normal.

## Encounter 1: source 2020-03-15, practice 2021-08-23, outpatient, Nephrology

Chief complaint: complaints of excessive urination and thirst for the past two weeks

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. James Okafor, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Complaints of excessive urination and thirst for the past two weeks

HISTORY OF PRESENT ILLNESS:
The patient returns following his January transplant visit. He reports drinking more than 4 liters of water daily but still feels dehydrated.

PAST MEDICAL HISTORY:
Active Problem List:
- Interstitial cystitis (diagnosed 2020-01-15)
- End-stage renal disease secondary to hypertensive nephrosclerosis
- Status post deceased-donor kidney transplantation
- Hypertension
- Bipolar disorder type I

His bipolar disorder has been well-controlled on lithium for the past 10 years. His other medical history includes hypertension.

PAST SURGICAL HISTORY:
- Deceased-donor kidney transplantation (initial, 5 years prior)
- Deceased-donor kidney re-transplantation (6 months prior)

MEDICATIONS:
He takes lithium for the past 10 years; hydrochlorothiazide.

Home Medications:
- Tacrolimus 5 mg PO BID
- Cyclosporine 100 mg PO BID
- Mycophenolate mofetil 750 mg PO BID
- Prednisone 5 mg PO daily
- Trimethoprim-sulfamethoxazole Single-strength tablet PO daily
- Lithium carbonate 300 mg PO BID
- Amlodipine 10 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, chronic kidney disease
- Sister: Bipolar disorder
- Maternal uncle: End-stage renal disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 5 years ago. Alcohol: None. Occupation: Retired postal worker.

VITAL SIGNS:
His vital signs are within normal limits

PHYSICAL EXAMINATION:
No abnormalities are noted

LABORATORY DATA:
- Serum sodium: 145 mEq/L (Normal: 135-145 mEq/L)
- Serum osmolality: 300 mOsm/kg (Normal: 275-295 mOsm/kg)
- Urine osmolality: 150 mOsm/kg (Normal: 500-800 mOsm/kg after water deprivation)
- Urine specific gravity: 1.005 (Normal: 1.010-1.025)

## Encounter 2: source 2020-07-15, practice 2021-12-08, outpatient, Transplant Medicine

Chief complaint: 2-week history of hand tremors, increased thirst, and frequent urination

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Transplant Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
2-week history of hand tremors, increased thirst, and frequent urination

HISTORY OF PRESENT ILLNESS:
The patient is a 55-year-old deceased-donor kidney transplant recipient who returns to the transplant medicine clinic with symptoms similar to his March 2020 presentation. He reports a 2-week history of hand tremors, increased thirst, and frequent urination.

PAST MEDICAL HISTORY:
Active Problem List:
- Interstitial cystitis (diagnosed 2020-01-15)
- Nephrogenic Diabetes Insipidus (diagnosed 2020-03-15)
- End-stage renal disease secondary to hypertensive nephrosclerosis
- Status post deceased-donor kidney transplantation
- Hypertension
- Bipolar disorder type I

PAST SURGICAL HISTORY:
- Deceased-donor kidney transplantation (initial, 5 years prior)
- Deceased-donor kidney re-transplantation (6 months prior)

MEDICATIONS:
He takes immunosuppressive therapy with tacrolimus.

Home Medications:
- Cyclosporine 100 mg PO BID
- Mycophenolate mofetil 750 mg PO BID
- Prednisone 5 mg PO daily
- Trimethoprim-sulfamethoxazole Single-strength tablet PO daily
- Lithium carbonate 300 mg PO BID
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, chronic kidney disease
- Sister: Bipolar disorder
- Maternal uncle: End-stage renal disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 5 years ago. Alcohol: None. Occupation: Retired postal worker.

VITAL SIGNS:
Blood pressure is 150/95 mmHg

PHYSICAL EXAMINATION:
Physical examination reveals a fine tremor in both hands

LABORATORY DATA:
- Serum creatinine: 2.1 mg/dL (elevated)
- Blood urea nitrogen (BUN): 35 mg/dL (elevated)
- Fasting blood glucose: 160 mg/dL (elevated)
- Tacrolimus level: 15 ng/mL (within therapeutic range)

## Encounter 3: source 2020-11-15, practice 2022-04-21, outpatient, Transplant Medicine

Chief complaint: worsening hypertension

ENCOUNTER NOTE
Date: 2020-11-15
Provider: Dr. Priya Sharma, MD
Department: Transplant Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening hypertension

HISTORY OF PRESENT ILLNESS:
The patient is a 55-year-old male status post deceased-donor kidney transplantation who returns to the transplant clinic for follow-up, having previously presented in July 2020 with calcineurin inhibitor toxicity and in March 2020 with nephrogenic diabetes insipidus. He reports a 3-month history of worsening hypertension despite compliance with a regimen of hydrochlorothiazide and amlodipine. Recently, he has noticed a decrease in his urine output and an increase in his serum creatinine from 1.2 mg/dL to 1.9 mg/dL over the past two months.

PAST MEDICAL HISTORY:
Active Problem List:
- Interstitial cystitis (diagnosed 2020-01-15)
- Nephrogenic Diabetes Insipidus (diagnosed 2020-03-15)
- Calcineurin inhibitor nephrotoxicity (diagnosed 2020-07-15)
- End-stage renal disease secondary to hypertensive nephrosclerosis
- Status post deceased-donor kidney transplantation
- Hypertension
- Bipolar disorder type I

PAST SURGICAL HISTORY:
Renal transplant 18 months ago

MEDICATIONS:
Hydrochlorothiazide, amlodipine, prednisone, tacrolimus, and mycophenolate mofetil

Home Medications:
- Cyclosporine 100 mg PO BID
- Trimethoprim-sulfamethoxazole Single-strength tablet PO daily
- Lithium carbonate 300 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, chronic kidney disease
- Sister: Bipolar disorder
- Maternal uncle: End-stage renal disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 5 years ago. Alcohol: None. Occupation: Retired postal worker.

VITAL SIGNS:
Blood pressure 162/98 mmHg

PHYSICAL EXAMINATION:
3/6 systolic murmur best heard over the right upper quadrant of the abdomen. The remainder of the examination is unremarkable.

LABORATORY DATA:
Serum creatinine: 1.9 mg/dL (baseline 1.2 mg/dL)
- Blood urea nitrogen (BUN): 36 mg/dL
- Serum potassium: 4.8 mEq/L
- Hemoglobin: 13.2 g/dL
- Urinalysis: No proteinuria or hematuria

## Encounter 4: source 2021-05-15, practice 2022-10-18, ed, Emergency Medicine

Chief complaint: new-onset dry cough, shortness of breath on exertion, and low-grade fever for the past week

ENCOUNTER NOTE
Date: 2021-05-15
Provider: Dr. Maria Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
New-onset dry cough, shortness of breath on exertion, and low-grade fever for the past week

HISTORY OF PRESENT ILLNESS:
This 56-year-old kidney transplant recipient with prior rejection and calcineurin inhibitor toxicity presents with a new-onset dry cough, shortness of breath on exertion, and a low-grade fever for the past week. He denies any recent travel or sick contacts.

PAST MEDICAL HISTORY:
Active Problem List:
- Interstitial cystitis (diagnosed 2020-01-15)
- Nephrogenic Diabetes Insipidus (diagnosed 2020-03-15)
- Calcineurin inhibitor nephrotoxicity (diagnosed 2020-07-15)
- Acute lung transplant rejection (diagnosed 2020-11-15)
- End-stage renal disease secondary to hypertensive nephrosclerosis
- Status post deceased-donor kidney transplantation
- Hypertension
- Bipolar disorder type I

PAST SURGICAL HISTORY:
2 months after receiving a kidney transplant

MEDICATIONS:
He has been on immunosuppressive therapy with tacrolimus, mycophenolate mofetil, and prednisone. He reports adherence to his medications, including prophylactic oral trimethoprim-sulfamethoxazole and valganciclovir.

Home Medications:
- Cyclosporine 100 mg PO BID
- Lithium carbonate 300 mg PO BID
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, chronic kidney disease
- Sister: Bipolar disorder
- Maternal uncle: End-stage renal disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 5 years ago. Alcohol: None. Occupation: Retired postal worker.

VITAL SIGNS:
His temperature is 37.8°C, blood pressure is 135/85 mmHg, pulse is 88 beats per minute, and respiratory rate is 20 breaths per minute. Oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
Physical examination reveals bilateral crackles at the lung bases.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 6,500/µL
- Hemoglobin: 12.5 g/dL
- Platelet count: 250,000/µL
- Serum creatinine: 1.2 mg/dL
- Blood urea nitrogen: 21 mg/dL
- Lactate dehydrogenase (LDH): 220 U/L (elevated)
- Arterial blood gas on room air:
- pH: 7.45
- PaCO2: 35 mmHg
- PaO2: 72 mmHg

IMAGING:
Chest X-ray shows diffuse interstitial infiltrates.

## Encounter 5: source 2021-11-15, practice 2023-04-17, ed, Emergency Medicine

Chief complaint: 10 days of progressively worsening cough, fever, and weight loss

ENCOUNTER NOTE
Date: 2021-11-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
10 days of progressively worsening cough, fever, and weight loss

HISTORY OF PRESENT ILLNESS:
This 56-year-old kidney transplant recipient with prior Pneumocystis pneumonia reports dull headaches for the past week and occasional difficulty concentrating.

PAST MEDICAL HISTORY:
Active Problem List:
- Interstitial cystitis (diagnosed 2020-01-15)
- Nephrogenic Diabetes Insipidus (diagnosed 2020-03-15)
- Calcineurin inhibitor nephrotoxicity (diagnosed 2020-07-15)
- Acute lung transplant rejection (diagnosed 2020-11-15)
- History of Pneumocystis jirovecii pneumonia (diagnosed 2021-05-15)
- End-stage renal disease secondary to hypertensive nephrosclerosis
- Status post deceased-donor kidney transplantation
- Hypertension
- Bipolar disorder type I

PAST SURGICAL HISTORY:
- Deceased-donor kidney transplantation (initial, 5 years prior)
- Deceased-donor kidney re-transplantation (6 months prior)

MEDICATIONS:
He has been compliant with his immunosuppressive regimen, including tacrolimus and prednisone.

Home Medications:
- Cyclosporine 100 mg PO BID
- Mycophenolate mofetil 750 mg PO BID
- Trimethoprim-sulfamethoxazole Single-strength tablet PO daily
- Lithium carbonate 300 mg PO BID
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, chronic kidney disease
- Sister: Bipolar disorder
- Maternal uncle: End-stage renal disease

SOCIAL HISTORY:
He does not smoke and has no recent travel or known sick contacts.

VITAL SIGNS:
On examination, temperature is 38.4°C (101.1°F), and respiratory rate is mildly elevated.

PHYSICAL EXAMINATION:
Lungs show crackles at the right upper lobe. Neurologic exam is nonfocal.

LABORATORY DATA:
Sputum Gram stain shows Nocardia spp .

IMAGING:
A chest x-ray shows a cavitary lesion in the right upper lobe. Brain MRI reveals a solitary ring-enhancing lesion in the frontal lobe.

## Encounter 6: source 2022-05-15, practice 2023-10-15, icu, Critical Care

Chief complaint: fever and pain in his neck

ENCOUNTER NOTE
Date: 2022-05-15
Provider: Dr. Aisha Patel, MD
Department: Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Fever and pain in his neck

HISTORY OF PRESENT ILLNESS:
The patient is a 57-year-old male status post deceased-donor kidney transplantation with a history of Pneumocystis jirovecii pneumonia and pelvic actinomycosis. He is brought to the emergency department by his wife for a 1-day history of fever and pain in his neck.

PAST MEDICAL HISTORY:
Active Problem List:
- Interstitial cystitis (diagnosed 2020-01-15)
- Nephrogenic Diabetes Insipidus (diagnosed 2020-03-15)
- Calcineurin inhibitor nephrotoxicity (diagnosed 2020-07-15)
- Acute lung transplant rejection (diagnosed 2020-11-15)
- History of Pneumocystis jirovecii pneumonia (diagnosed 2021-05-15)
- Pelvic actinomycosis (diagnosed 2021-11-15)
- End-stage renal disease secondary to hypertensive nephrosclerosis
- Status post deceased-donor kidney transplantation
- Hypertension
- Bipolar disorder type I

PAST SURGICAL HISTORY:
He had a renal transplant 6 years ago

MEDICATIONS:
He takes for which he is taking tacrolimus, cyclosporine, and prednisone.

Home Medications:
- Mycophenolate mofetil 750 mg PO BID
- Trimethoprim-sulfamethoxazole Single-strength tablet PO daily
- Lithium carbonate 300 mg PO BID
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, chronic kidney disease
- Sister: Bipolar disorder
- Maternal uncle: End-stage renal disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 5 years ago. Alcohol: None. Occupation: Retired postal worker.

VITAL SIGNS:
His temperature is 103 F.

## Encounter 7: source 2025-01-15, practice 2026-06-15, outpatient, Transplant Medicine

Chief complaint: Over the past 8 months, progressive fatigue, decreased exercise tolerance, and ankle swelling

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. David Washington, MD
Department: Transplant Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Over the past 8 months, progressive fatigue, decreased exercise tolerance, and ankle swelling

HISTORY OF PRESENT ILLNESS:
The patient is a 60-year-old male with end-stage renal disease status post deceased-donor kidney transplantation and history of calcineurin inhibitor nephrotoxicity who returns for follow-up. His blood pressure has become more difficult to control despite being on multiple antihypertensives.

PAST MEDICAL HISTORY:
Active Problem List:
- Interstitial cystitis (diagnosed 2020-01-15)
- Nephrogenic Diabetes Insipidus (diagnosed 2020-03-15)
- Calcineurin inhibitor nephrotoxicity (diagnosed 2020-07-15)
- Acute lung transplant rejection (diagnosed 2020-11-15)
- History of Pneumocystis jirovecii pneumonia (diagnosed 2021-05-15)
- Pelvic actinomycosis (diagnosed 2021-11-15)
- Cryptococcal meningitis (diagnosed 2022-05-15)
- End-stage renal disease secondary to hypertensive nephrosclerosis
- Status post deceased-donor kidney transplantation
- Hypertension
- Bipolar disorder type I

PAST SURGICAL HISTORY:
- Deceased-donor kidney transplantation (initial, 5 years prior)
- Deceased-donor kidney re-transplantation (6 months prior)

MEDICATIONS:
He has been maintained on tacrolimus, mycophenolate, and prednisone

Home Medications:
- Cyclosporine 100 mg PO BID
- Mycophenolate mofetil 750 mg PO BID
- Trimethoprim-sulfamethoxazole Single-strength tablet PO daily
- Lithium carbonate 300 mg PO BID
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, chronic kidney disease
- Sister: Bipolar disorder
- Maternal uncle: End-stage renal disease

SOCIAL HISTORY:
He admits to occasional lapses in medication adherence

REVIEW OF SYSTEMS:
He denies fever, dysuria, hematuria, or abdominal pain

LABORATORY DATA:
Laboratory studies reveal a gradual increase in serum creatinine and mild proteinuria

IMAGING:
Ultrasound of the transplanted kidney shows asymmetrically small size with loss of corticomedullary differentiation

PATHOLOGY:
A biopsy demonstrates fibrotic thickening of the vascular intima with mononuclear infiltrates and interstitial fibrosis

ASSESSMENT AND PLAN:
Despite optimization of immunosuppressive therapy, his renal function continues to decline
