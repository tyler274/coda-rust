# synthetic_hospital patient 2043

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 43 at first encounter, sex M, Black or African American, Commercial insurance through employer.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-06-01 (Wed) | outpatient | Internal Medicine |
| 1 | 2020-03-15 | 2022-08-03 (Wed) | outpatient | Infectious Disease |
| 2 | 2020-07-15 | 2022-12-13 (Tue) | outpatient | Internal Medicine |
| 3 | 2021-01-15 | 2023-06-15 (Thu) | ed | Emergency Medicine |
| 4 | 2021-03-15 | 2023-08-02 (Wed) | ed | Emergency Medicine |
| 5 | 2021-07-15 | 2023-12-05 (Tue) | ed | Emergency Medicine |
| 6 | 2022-07-15 | 2024-12-16 (Mon) | ed | Emergency Medicine |
| 7 | 2023-07-15 | 2025-12-07 (Sun) | inpatient | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Commercial insurance through employer",
 "occupation": "Construction site foreman",
 "alcohol_use": "None",
 "family_history": [
  "End-stage renal disease (mother, age 67)",
  "Type 2 diabetes mellitus (father)",
  "Hypertension (both parents)",
  "Nephrolithiasis (brother)"
 ],
 "race_ethnicity": "Black or African American",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "300 mg/200 mg PO daily",
   "name": "Tenofovir disoproxil fumarate/Emtricitabine"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Dolutegravir"
  }
 ],
 "surgical_history": [
  "Cystoscopy with bilateral ureteral stent placement (2019)",
  "Extracorporeal shock wave lithotripsy (2021)"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus",
  "Hypertension",
  "Chronic kidney disease, stage 4",
  "Human immunodeficiency virus infection with history of AIDS (CD4 <200)",
  "Recurrent calcium oxalate nephrolithiasis",
  "History of cerebral toxoplasmosis"
 ],
 "age_at_first_encounter": 43
}
```

Primary diagnoses: ["Prerenal acute kidney injury secondary to hypovolemia", "Platelet function disorder", "Uremic platelet dysfunction", "Patient anger/acute situational irritability", "Type 2 diabetes mellitus", "Urolithiasis (Renal Colic)", "Septic prepatellar bursitis", "HIV-related opportunistic skin infection"]
Comorbidities: ["End-stage renal disease on hemodialysis", "Essential hypertension", "Type 2 diabetes mellitus", "Long term (current) use of immunosuppressants"]

## Encounter 0: source 2020-01-15, practice 2022-06-01, outpatient, Internal Medicine

Chief complaint: Routine follow-up for type 2 diabetes mellitus and hypertension. Patient reports adherence to medications and diet with no polyuria, polydipsia, or polyphagia.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for type 2 diabetes mellitus and hypertension. Patient reports adherence to medications and diet with no polyuria, polydipsia, or polyphagia.

HISTORY OF PRESENT ILLNESS:
He reports adherence to his medication regimen, which includes metformin and lisinopril, and has been following a diet recommended by a nutritionist. He denies any symptoms of polyuria, polydipsia, or polyphagia.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus
- Hypertension
- Chronic kidney disease, stage 4
- Human immunodeficiency virus infection with history of AIDS (CD4 <200)
- Recurrent calcium oxalate nephrolithiasis
- History of cerebral toxoplasmosis

PAST SURGICAL HISTORY:
- Cystoscopy with bilateral ureteral stent placement (2019)
- Extracorporeal shock wave lithotripsy (2021)

MEDICATIONS:
He takes metformin and lisinopril.

Home Medications:
- Tenofovir disoproxil fumarate/Emtricitabine 300 mg/200 mg PO daily
- Dolutegravir 50 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- End-stage renal disease (mother, age 67)
- Type 2 diabetes mellitus (father)
- Hypertension (both parents)
- Nephrolithiasis (brother)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Construction site foreman.

VITAL SIGNS:
His blood pressure is 132/86 mmHg

PHYSICAL EXAMINATION:
Physical examination is unremarkable

LABORATORY DATA:
Fasting plasma glucose: 146 mg/dL
Hemoglobin A1c: 7.2%
Serum creatinine: 1.0 mg/dL
Urine albumin-to-creatinine ratio: 25 mg/g

PLAN:
The physician decides to perform a scatter plot analysis of the patient's fasting plasma glucose and hemoglobin A1c values over the past year to assess the relationship between the two variables.

## Encounter 1: source 2020-03-15, practice 2022-08-03, outpatient, Infectious Disease

Chief complaint: Routine follow-up for HIV infection with history of AIDS. Patient reports occasional night sweats but no acute complaints and adheres to antiretroviral therapy.

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Marcus Johnson, MD
Department: Infectious Disease
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for HIV infection with history of AIDS. Patient reports occasional night sweats but no acute complaints and adheres to antiretroviral therapy.

HISTORY OF PRESENT ILLNESS:
Patient returns for HIV follow-up with history of AIDS and prior Internal Medicine evaluation. He reports no acute complaints but mentions occasional night sweats. He has been adherent to his antiretroviral therapy. His last CD4 count was 95 cells/mm3 six months ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Hypertension
- Chronic kidney disease, stage 4
- Human immunodeficiency virus infection with history of AIDS (CD4 <200)
- Recurrent calcium oxalate nephrolithiasis
- History of cerebral toxoplasmosis

PAST SURGICAL HISTORY:
- Cystoscopy with bilateral ureteral stent placement (2019)
- Extracorporeal shock wave lithotripsy (2021)

MEDICATIONS:
His current medications include tenofovir/emtricitabine and dolutegravir.

Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Tenofovir disoproxil fumarate/Emtricitabine 300 mg/200 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- End-stage renal disease (mother, age 67)
- Type 2 diabetes mellitus (father)
- Hypertension (both parents)
- Nephrolithiasis (brother)

SOCIAL HISTORY:
He has not been sexually active for the past year

VITAL SIGNS:
He is afebrile, with normal vital signs

PHYSICAL EXAMINATION:
His physical examination is unremarkable

LABORATORY DATA:
Laboratory results are as follows:

- CD4 count: 85 cells/mm3
- HIV viral load: Undetectable
- Toxoplasma IgG: Positive

## Encounter 2: source 2020-07-15, practice 2022-12-13, outpatient, Internal Medicine

Chief complaint: Follow-up appointment for hypertension and diabetes mellitus. Patient expresses anger regarding a 45-minute wait time before being brought to the examination room.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up appointment for hypertension and diabetes mellitus. Patient expresses anger regarding a 45-minute wait time before being brought to the examination room.

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of his type 2 diabetes mellitus and hypertension, last evaluated in the Internal Medicine clinic on January 15, 2020, with subsequent interim follow-up for HIV infection in the Infectious Disease clinic on March 15, 2020. He arrived on time but waited approximately 45 minutes before being brought to the examination room. When the physician enters, the patient appears visibly angry and states, "You made me wait almost an hour! This is unacceptable." He crosses his arms, maintains eye contact, and appears otherwise calm and cooperative. He does not refuse care or make threats but appears frustrated.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- HIV-related opportunistic skin infection (diagnosed 2020-03-15)
- Hypertension
- Chronic kidney disease, stage 4
- Human immunodeficiency virus infection with history of AIDS (CD4 <200)
- Recurrent calcium oxalate nephrolithiasis
- History of cerebral toxoplasmosis

PAST SURGICAL HISTORY:
- Cystoscopy with bilateral ureteral stent placement (2019)
- Extracorporeal shock wave lithotripsy (2021)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Tenofovir disoproxil fumarate/Emtricitabine 300 mg/200 mg PO daily
- Dolutegravir 50 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- End-stage renal disease (mother, age 67)
- Type 2 diabetes mellitus (father)
- Hypertension (both parents)
- Nephrolithiasis (brother)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Construction site foreman.

PLAN:
The physician intends to proceed with the encounter but wants to address the patient's frustration appropriately.

## Encounter 3: source 2021-01-15, practice 2023-06-15, ed, Emergency Medicine

Chief complaint: Two-month history of intermittent, severe pain in the right flank radiating to the groin with associated hematuria.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-month history of intermittent, severe pain in the right flank radiating to the groin with associated hematuria.

HISTORY OF PRESENT ILLNESS:
The patient, with established recurrent calcium oxalate nephrolithiasis and longitudinal outpatient management of diabetes mellitus, hypertension, and HIV infection, presents to the emergency department with flank pain. He reports that the pain occasionally radiates to the groin.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- HIV-related opportunistic skin infection (diagnosed 2020-03-15)
- Patient anger/acute situational irritability (diagnosed 2020-07-15)
- Hypertension
- Chronic kidney disease, stage 4
- Human immunodeficiency virus infection with history of AIDS (CD4 <200)
- Recurrent calcium oxalate nephrolithiasis
- History of cerebral toxoplasmosis

PAST SURGICAL HISTORY:
- Cystoscopy with bilateral ureteral stent placement (2019)
- Extracorporeal shock wave lithotripsy (2021)

MEDICATIONS:
He takes metformin and lisinopril.

Home Medications:
- Tenofovir disoproxil fumarate/Emtricitabine 300 mg/200 mg PO daily
- Dolutegravir 50 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- End-stage renal disease (mother, age 67)
- Type 2 diabetes mellitus (father)
- Hypertension (both parents)
- Nephrolithiasis (brother)

SOCIAL HISTORY:
Diet high in sodium and protein and a low fluid intake

PHYSICAL EXAMINATION:
Unremarkable except for mild right costovertebral angle tenderness

LABORATORY DATA:
Urinalysis reveals microscopic hematuria and no signs of infection. Serum chemistry panel is as follows:

- Sodium: 142 mEq/L (normal 135-145 mEq/L)
- Potassium: 4.2 mEq/L (normal 3.5-5.1 mEq/L)
- Chloride: 100 mEq/L (normal 98-107 mEq/L)
- Bicarbonate: 24 mEq/L (normal 22-29 mEq/L)
- Blood urea nitrogen (BUN): 18 mg/dL (normal 7-20 mg/dL)
- Creatinine: 1.0 mg/dL (normal 0.6-1.2 mg/dL)
- Calcium: 10.2 mg/dL (normal 8.5-10.2 mg/dL)

## Encounter 4: source 2021-03-15, practice 2023-08-02, ed, Emergency Medicine

Chief complaint: Two-day history of increasing pain, redness, and swelling over the anterior aspect of the right knee after kneeling on gravel at a construction work site.

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Emily Watson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-day history of increasing pain, redness, and swelling over the anterior aspect of the right knee after kneeling on gravel at a construction work site.

HISTORY OF PRESENT ILLNESS:
This 44-year-old male with HIV/AIDS and CKD and prior renal colic presents with a 2-day history of increasing pain, redness, and swelling over the anterior aspect of his right knee. He denies any recent trauma but recalls kneeling on gravel and small stones at a work site last week.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- HIV-related opportunistic skin infection (diagnosed 2020-03-15)
- Patient anger/acute situational irritability (diagnosed 2020-07-15)
- Urolithiasis (Renal Colic) (diagnosed 2021-01-15)
- Hypertension
- Chronic kidney disease, stage 4
- Human immunodeficiency virus infection with history of AIDS (CD4 <200)
- Recurrent calcium oxalate nephrolithiasis
- History of cerebral toxoplasmosis

PAST SURGICAL HISTORY:
- Cystoscopy with bilateral ureteral stent placement (2019)
- Extracorporeal shock wave lithotripsy (2021)

MEDICATIONS:
He takes currently on metformin.

Home Medications:
- Lisinopril 20 mg PO daily
- Tenofovir disoproxil fumarate/Emtricitabine 300 mg/200 mg PO daily
- Dolutegravir 50 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- End-stage renal disease (mother, age 67)
- Type 2 diabetes mellitus (father)
- Hypertension (both parents)
- Nephrolithiasis (brother)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Construction site foreman.

PHYSICAL EXAMINATION:
On examination, the right knee is warm and erythematous with a fluctuant swelling over the prepatellar area. The range of motion is limited due to pain.

LABORATORY DATA:
Aspiration of the bursal fluid yields 50,000 WBC/µL with 90% neutrophils. Gram stain shows gram-positive cocci in clusters.

## Encounter 5: source 2021-07-15, practice 2023-12-05, ed, Emergency Medicine

Chief complaint: Two-day history of generalized malaise, decreased urine output, and flank pain after taking over-the-counter pain medications for a sprained ankle.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Michael Liu, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-day history of generalized malaise, decreased urine output, and flank pain after taking over-the-counter pain medications for a sprained ankle.

HISTORY OF PRESENT ILLNESS:
The patient is a 44-year-old male with chronic kidney disease stage 4 and recurrent nephrolithiasis, evaluated for renal colic in January 2021, who presents with a 2-day history of generalized malaise, decreased urine output, and flank pain. He reports recently starting over-the-counter pain medications for a sprained ankle.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- HIV-related opportunistic skin infection (diagnosed 2020-03-15)
- Patient anger/acute situational irritability (diagnosed 2020-07-15)
- Urolithiasis (Renal Colic) (diagnosed 2021-01-15)
- Septic prepatellar bursitis (diagnosed 2021-03-15)
- Hypertension
- Chronic kidney disease, stage 4
- Human immunodeficiency virus infection with history of AIDS (CD4 <200)
- Recurrent calcium oxalate nephrolithiasis
- History of cerebral toxoplasmosis

PAST SURGICAL HISTORY:
- Cystoscopy with bilateral ureteral stent placement (2019)
- Extracorporeal shock wave lithotripsy (2021)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Tenofovir disoproxil fumarate/Emtricitabine 300 mg/200 mg PO daily
- Dolutegravir 50 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- End-stage renal disease (mother, age 67)
- Type 2 diabetes mellitus (father)
- Hypertension (both parents)
- Nephrolithiasis (brother)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Construction site foreman.

VITAL SIGNS:
His temperature is 37.8°C (100°F), blood pressure is 155/95 mmHg, pulse is 88/min, and respiratory rate is 16/min.

PHYSICAL EXAMINATION:
Physical examination reveals mild bilateral costovertebral angle tenderness.

LABORATORY DATA:
Laboratory studies are as follows:
- Serum creatinine: 3.2 mg/dL (elevated from a baseline of 1.1 mg/dL)
- Blood urea nitrogen (BUN): 45 mg/dL
- Urinalysis: Specific gravity 1.010, pH 6.0, no glucose, no ketones, 2+ blood, 1+ protein
- Urine sediment: Muddy brown granular casts

ASSESSMENT AND PLAN:
Based on the patient's presentation and laboratory findings,

## Encounter 6: source 2022-07-15, practice 2024-12-16, ed, Emergency Medicine

Chief complaint: Two-week history of recurrent spontaneous nosebleeds with associated bruising on arms and legs, bleeding gums when brushing teeth, and prolonged bleeding from minor cuts.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Fatima Al-Rashid, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-week history of recurrent spontaneous nosebleeds with associated bruising on arms and legs, bleeding gums when brushing teeth, and prolonged bleeding from minor cuts.

HISTORY OF PRESENT ILLNESS:
The patient with stage 4 chronic kidney disease and HIV presents with bleeding symptoms following prior acute kidney injury and nephrolithiasis. He states that he has also noticed bruises on his arms and legs without any remembered trauma. Over the past few days, he reports bleeding from his gums when brushing his teeth and minor cuts taking unusually long to stop bleeding. He reports significant fatigue but attributes it to poor sleep.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- HIV-related opportunistic skin infection (diagnosed 2020-03-15)
- Patient anger/acute situational irritability (diagnosed 2020-07-15)
- Urolithiasis (Renal Colic) (diagnosed 2021-01-15)
- Septic prepatellar bursitis (diagnosed 2021-03-15)
- Prerenal acute kidney injury secondary to hypovolemia (diagnosed 2021-07-15)
- Hypertension
- Chronic kidney disease, stage 4
- Human immunodeficiency virus infection with history of AIDS (CD4 <200)
- Recurrent calcium oxalate nephrolithiasis
- History of cerebral toxoplasmosis

His medical history includes long-standing hypertension and poorly controlled type 2 diabetes mellitus. He was diagnosed with chronic kidney disease 3 years ago and has not yet initiated dialysis.

PAST SURGICAL HISTORY:
- Cystoscopy with bilateral ureteral stent placement (2019)
- Extracorporeal shock wave lithotripsy (2021)

MEDICATIONS:
He denies taking aspirin, NSAIDs, or anticoagulant medications.

Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Tenofovir disoproxil fumarate/Emtricitabine 300 mg/200 mg PO daily
- Dolutegravir 50 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
He has no personal or family history of bleeding or clotting disorders.

SOCIAL HISTORY:
He does not drink alcohol, smoke, or use illicit drugs.

VITAL SIGNS:
Vital signs are within normal limits except for elevated blood pressure.

PHYSICAL EXAMINATION:
On physical examination, he has dried blood in his nares, scattered petechiae on both lower extremities, and ecchymoses on his forearms. Cardiopulmonary and abdominal examinations are unremarkable. Neurological exam shows no focal deficits.

LABORATORY DATA:
Laboratory studies reveal markedly elevated BUN and creatinine, consistent with advanced uremia.

PLAN:
He is admitted to the hospital for further evaluation.

## Encounter 7: source 2023-07-15, practice 2025-12-07, inpatient, Internal Medicine

Chief complaint: One-week history of fatigue and persistent bleeding from gums, with dark purple bruises on arms and thighs and prolonged bleeding from minor shaving cuts.

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. David Kim, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
One-week history of fatigue and persistent bleeding from gums, with dark purple bruises on arms and thighs and prolonged bleeding from minor shaving cuts.

HISTORY OF PRESENT ILLNESS:
The patient is a 46-year-old male with Stage 4 chronic kidney disease, HIV infection, and a prior bleeding presentation in July 2022 who returns with recurrent mucocutaneous bleeding symptoms. He reports that brushing his teeth has become difficult due to frequent gum bleeding, and he also noticed that a small shaving cut bled for several hours before finally clotting. Over the last few days, he has developed dark purple bruises on his arms and thighs, despite no significant trauma. He denies use of aspirin, anticoagulants, or herbal supplements. He has not had any recent infections or invasive procedures.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- HIV-related opportunistic skin infection (diagnosed 2020-03-15)
- Patient anger/acute situational irritability (diagnosed 2020-07-15)
- Urolithiasis (Renal Colic) (diagnosed 2021-01-15)
- Septic prepatellar bursitis (diagnosed 2021-03-15)
- Prerenal acute kidney injury secondary to hypovolemia (diagnosed 2021-07-15)
- Uremic platelet dysfunction (diagnosed 2022-07-15)
- Hypertension
- Chronic kidney disease, stage 4
- Human immunodeficiency virus infection with history of AIDS (CD4 <200)
- Recurrent calcium oxalate nephrolithiasis
- History of cerebral toxoplasmosis

His medical history includes hypertension and type 2 diabetes mellitus. He was told several months ago that he may need dialysis soon, but he has not yet started it.

PAST SURGICAL HISTORY:
- Cystoscopy with bilateral ureteral stent placement (2019)
- Extracorporeal shock wave lithotripsy (2021)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Tenofovir disoproxil fumarate/Emtricitabine 300 mg/200 mg PO daily
- Dolutegravir 50 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- End-stage renal disease (mother, age 67)
- Type 2 diabetes mellitus (father)
- Hypertension (both parents)
- Nephrolithiasis (brother)

SOCIAL HISTORY:
He is not sexually active and denies alcohol, tobacco, or illicit drug use.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
Physical exam reveals multiple ecchymoses on the upper and lower extremities, dried blood at the gumline, and bilateral lower extremity petechiae. Cardiopulmonary and neurologic exams are unremarkable.

LABORATORY DATA:
His platelet count, PT, and aPTT are all within the normal range. Renal panel shows markedly elevated BUN and creatinine consistent with longstanding renal failure.
