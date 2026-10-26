# synthetic_hospital patient 2566

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 33 at first encounter, sex M, African American, Uninsured (coverage lost 12 months ago).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-01-10 (Tue) | outpatient | Nephrology |
| 1 | 2020-03-15 | 2023-02-24 (Fri) | ed | Emergency Medicine |
| 2 | 2020-09-15 | 2023-09-14 (Thu) | ed | Emergency Medicine |
| 3 | 2021-03-15 | 2024-03-08 (Fri) | ed | Emergency Medicine |
| 4 | 2021-09-15 | 2024-09-13 (Fri) | ed | Emergency Medicine |
| 5 | 2022-03-15 | 2025-02-27 (Thu) | outpatient | Ophthalmology |
| 6 | 2022-09-15 | 2025-09-06 (Sat) | ed | Emergency Medicine |
| 7 | 2023-03-15 | 2026-02-24 (Tue) | inpatient | Cardiology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Primaquine (severe hemolysis)",
  "Aspirin (hemolysis)",
  "Trimethoprim-sulfamethoxazole (hemolysis)"
 ],
 "insurance": "Uninsured (coverage lost 12 months ago)",
 "occupation": "Construction laborer",
 "alcohol_use": "Social, 1-2 drinks per month",
 "family_history": [
  "Maternal uncle: Glucose-6-phosphate dehydrogenase deficiency",
  "Mother: G6PD carrier",
  "Paternal grandfather: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "Non-adherent (discontinued 1 year ago due to insurance loss)",
   "name": "Bictegravir/Emtricitabine/Tenofovir alafenamide"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Glucose-6-phosphate dehydrogenase deficiency (diagnosed age 28)",
  "HIV/AIDS (CDC Stage 3, diagnosed age 32, currently non-adherent to ART)",
  "IgA nephropathy (diagnosed age 32)"
 ],
 "age_at_first_encounter": 33
}
```

Primary diagnoses: ["Acute idiopathic pericarditis", "Cytomegalovirus esophagitis", "IgA nephropathy", "Methicillin-resistant Staphylococcus aureus (MRSA) skin and soft tissue infection", "Glucose-6-phosphate dehydrogenase deficiency", "Acute Decompensated Heart Failure"]
Comorbidities: ["Severe immunosuppression", "Fever", "Upper respiratory tract infection"]

## Encounter 0: source 2020-01-15, practice 2023-01-10, outpatient, Nephrology

Chief complaint: Dark urine and mild flank pain following upper respiratory infection

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Dark urine and mild flank pain following upper respiratory infection

HISTORY OF PRESENT ILLNESS:
A 32-year-old male presents to the clinic with a 2-day history of dark urine and mild flank pain. He reports a recent upper respiratory tract infection that resolved about 4 days ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Glucose-6-phosphate dehydrogenase deficiency (diagnosed age 28)
- HIV/AIDS (CDC Stage 3, diagnosed age 32, currently non-adherent to ART)
- IgA nephropathy (diagnosed age 32)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Bictegravir/Emtricitabine/Tenofovir alafenamide Non-adherent (discontinued 1 year ago due to insurance loss)

ALLERGIES:
Primaquine (severe hemolysis), Aspirin (hemolysis), Trimethoprim-sulfamethoxazole (hemolysis)

FAMILY HISTORY:
- Maternal uncle: Glucose-6-phosphate dehydrogenase deficiency
- Mother: G6PD carrier
- Paternal grandfather: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per month. Occupation: Construction laborer.

PHYSICAL EXAMINATION:
On physical examination, his blood pressure is 130/85 mmHg, and there is no edema.

LABORATORY DATA:
Urinalysis reveals red blood cells but no casts, and proteinuria is noted. Laboratory results are as follows:

- Serum creatinine: 1.2 mg/dL (normal: 0.6-1.3 mg/dL)
- Blood urea nitrogen (BUN): 18 mg/dL (normal: 7-20 mg/dL)
- Serum C3 complement: 90 mg/dL (normal: 90-180 mg/dL)
- Serum C4 complement: 20 mg/dL (normal: 10-40 mg/dL)

## Encounter 1: source 2020-03-15, practice 2023-02-24, ed, Emergency Medicine

Chief complaint: Increasing fatigue, jaundice, and dark urine

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Michael Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increasing fatigue, jaundice, and dark urine

HISTORY OF PRESENT ILLNESS:
The patient is a 33-year-old male with glucose-6-phosphate dehydrogenase deficiency and IgA nephropathy who was evaluated two months prior for dark urine. He presents to the emergency department with a two-day history of increasing fatigue, jaundice, and dark urine. He reports a recent upper respiratory tract infection that resolved without treatment one week ago. He denies any history of similar symptoms in the past.

PAST MEDICAL HISTORY:
Active Problem List:
- IgA nephropathy (diagnosed 2020-01-15)
- Glucose-6-phosphate dehydrogenase deficiency (diagnosed age 28)
- HIV/AIDS (CDC Stage 3, diagnosed age 32, currently non-adherent to ART)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Bictegravir/Emtricitabine/Tenofovir alafenamide Non-adherent (discontinued 1 year ago due to insurance loss)

ALLERGIES:
Primaquine (severe hemolysis), Aspirin (hemolysis), Trimethoprim-sulfamethoxazole (hemolysis)

FAMILY HISTORY:
His family history is notable for a cousin with a "blood disorder" that the patient knows little about.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per month. Occupation: Construction laborer.

VITAL SIGNS:
On physical examination, he is afebrile, with a heart rate of 102 bpm and blood pressure of 130/85 mmHg.

PHYSICAL EXAMINATION:
His sclerae are icteric, and his abdomen is soft with no hepatosplenomegaly.

LABORATORY DATA:
Laboratory studies are as follows:

- Hemoglobin: 9.2 g/dL (Normal: 13.8-17.2 g/dL)
- Total bilirubin: 3.5 mg/dL (Normal: 0.3-1.2 mg/dL)
- Direct bilirubin: 0.8 mg/dL (Normal: 0.0-0.3 mg/dL)
- Lactate dehydrogenase (LDH): 450 U/L (Normal: 140-280 U/L)
- Haptoglobin: <10 mg/dL (Normal: 30-200 mg/dL)
- Reticulocyte count: 5% (Normal: 0.5-1.5%)
- Peripheral blood smear: Presence of bite cells and Heinz bodies

## Encounter 2: source 2020-09-15, practice 2023-09-14, ed, Emergency Medicine

Chief complaint: Increasing fatigue, shortness of breath, and dark urine after taking aspirin-containing medication

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Jennifer Lee, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increasing fatigue, shortness of breath, and dark urine after taking aspirin-containing medication

HISTORY OF PRESENT ILLNESS:
The patient, with known glucose-6-phosphate dehydrogenase deficiency and prior ED presentation for hemolysis, returns with similar symptoms. He reports feeling well until developing sudden onset fever and sore throat 3 days ago, for which he took an over-the-counter aspirin-containing medication.

PAST MEDICAL HISTORY:
Active Problem List:
- IgA nephropathy (diagnosed 2020-01-15)
- Glucose-6-phosphate dehydrogenase deficiency (diagnosed 2020-03-15)
- HIV/AIDS (CDC Stage 3, diagnosed age 32, currently non-adherent to ART)

His past medical history is significant for a similar episode following an upper respiratory tract infection 5 years ago, which resolved spontaneously.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Bictegravir/Emtricitabine/Tenofovir alafenamide Non-adherent (discontinued 1 year ago due to insurance loss)

ALLERGIES:
Primaquine (severe hemolysis), Aspirin (hemolysis), Trimethoprim-sulfamethoxazole (hemolysis)

FAMILY HISTORY:
- Maternal uncle: Glucose-6-phosphate dehydrogenase deficiency
- Mother: G6PD carrier
- Paternal grandfather: Type 2 diabetes mellitus

SOCIAL HISTORY:
He denies recent travel or exposure to sick contacts.

PHYSICAL EXAMINATION:
On physical examination, he appears jaundiced with scleral icterus. His spleen is palpable 2 cm below the left costal margin.

LABORATORY DATA:
Laboratory studies are as follows:

- Hemoglobin: 8.2 g/dL (Normal: 13.8-17.2 g/dL)
- Total bilirubin: 3.5 mg/dL (Normal: 0.3-1.2 mg/dL)
- Direct bilirubin: 0.8 mg/dL (Normal: 0.0-0.3 mg/dL)
- Lactate dehydrogenase (LDH): 450 U/L (Normal: 140-280 U/L)
- Haptoglobin: <10 mg/dL (Normal: 30-200 mg/dL)
- Reticulocyte count: 6% (Normal: 0.5-1.5%)

A peripheral blood smear shows the presence of Heinz bodies and bite cells.

## Encounter 3: source 2021-03-15, practice 2024-03-08, ed, Emergency Medicine

Chief complaint: Dark urine, fatigue, and shortness of breath after taking over-the-counter medication

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Dark urine, fatigue, and shortness of breath after taking over-the-counter medication

HISTORY OF PRESENT ILLNESS:
The patient is a 34-year-old male with glucose-6-phosphate dehydrogenase deficiency, previously evaluated in the emergency department in September 2020 and March 2020 for similar presentations, who returns with a 2-day history of dark urine, fatigue, and shortness of breath. He reports a recent upper respiratory tract infection that resolved a week ago, for which he took an over-the-counter medication.

PAST MEDICAL HISTORY:
Active Problem List:
- IgA nephropathy (diagnosed 2020-01-15)
- Glucose-6-phosphate dehydrogenase deficiency (diagnosed 2020-03-15)
- HIV/AIDS (CDC Stage 3, diagnosed age 32, currently non-adherent to ART)

His past medical history is significant for a similar episode following an antimalarial drug taken for a trip to West Africa 5 years ago.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Bictegravir/Emtricitabine/Tenofovir alafenamide Non-adherent (discontinued 1 year ago due to insurance loss)

ALLERGIES:
Primaquine (severe hemolysis), Aspirin (hemolysis), Trimethoprim-sulfamethoxazole (hemolysis)

FAMILY HISTORY:
- Maternal uncle: Glucose-6-phosphate dehydrogenase deficiency
- Mother: G6PD carrier
- Paternal grandfather: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per month. Occupation: Construction laborer.

PHYSICAL EXAMINATION:
On physical examination, he appears jaundiced with scleral icterus. His spleen is palpable 2 cm below the left costal margin.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 8.2 g/dL (Normal: 13.8-17.2 g/dL)
- Total bilirubin: 3.1 mg/dL (Normal: 0.3-1.2 mg/dL)
- Direct bilirubin: 0.5 mg/dL (Normal: 0.0-0.3 mg/dL)
- Lactate dehydrogenase (LDH): 450 U/L (Normal: 140-280 U/L)
- Haptoglobin: <10 mg/dL (Normal: 30-200 mg/dL)
- Peripheral blood smear: Bite cells and blister cells

## Encounter 4: source 2021-09-15, practice 2024-09-13, ed, Emergency Medicine

Chief complaint: Redness, swelling, and pain in left hand after cut while using tools

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Maria Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Redness, swelling, and pain in left hand after cut while using tools

HISTORY OF PRESENT ILLNESS:
Thirty-four-year-old with G6PD deficiency, HIV/AIDS, and IgA nephropathy presents with left hand redness, swelling, and pain that began 2 days ago after he cut himself while using tools.

PAST MEDICAL HISTORY:
Active Problem List:
- IgA nephropathy (diagnosed 2020-01-15)
- Glucose-6-phosphate dehydrogenase deficiency (diagnosed 2020-03-15)
- HIV/AIDS (CDC Stage 3, diagnosed age 32, currently non-adherent to ART)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Bictegravir/Emtricitabine/Tenofovir alafenamide Non-adherent (discontinued 1 year ago due to insurance loss)

ALLERGIES:
Primaquine (severe hemolysis), Aspirin (hemolysis), Trimethoprim-sulfamethoxazole (hemolysis)

FAMILY HISTORY:
- Maternal uncle: Glucose-6-phosphate dehydrogenase deficiency
- Mother: G6PD carrier
- Paternal grandfather: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per month. Occupation: Construction laborer.

VITAL SIGNS:
His temperature is 38.6°C (101.5°F), heart rate is 72/min, blood pressure is 120/76 mmHg, and respiratory rate is 14/min.

PHYSICAL EXAMINATION:
On physical exam, the dorsum of the left hand is erythematous, warm, and tender.

LABORATORY DATA:
Wound cultures later grow MRSA. Production of

PLAN:
Treatment is initiated with oral dicloxacillin, but his symptoms fail to improve.

## Encounter 5: source 2022-03-15, practice 2025-02-27, outpatient, Ophthalmology

Chief complaint: Progressively worsening vision in right eye described as 'curtain coming down' with flashes and floaters

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Priya Patel, MD
Department: Ophthalmology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressively worsening vision in right eye described as 'curtain coming down' with flashes and floaters

HISTORY OF PRESENT ILLNESS:
This 35-year-old male with HIV/AIDS (CDC Stage 3), recurrent G6PD hemolysis, and IgA nephropathy presents with progressively worsening vision. He describes the vision changes as a "curtain coming down" over his field of vision, accompanied by flashes of light and an increase in floaters. He has not been on antiretroviral therapy for the past year due to loss of insurance.

PAST MEDICAL HISTORY:
Active Problem List:
- IgA nephropathy (diagnosed 2020-01-15)
- Glucose-6-phosphate dehydrogenase deficiency (diagnosed 2020-03-15)
- Methicillin-resistant Staphylococcus aureus (MRSA) skin and soft tissue infection (diagnosed 2021-09-15)
- HIV/AIDS (CDC Stage 3, diagnosed age 32, currently non-adherent to ART)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Bictegravir/Emtricitabine/Tenofovir alafenamide Non-adherent (discontinued 1 year ago due to insurance loss)

ALLERGIES:
Primaquine (severe hemolysis), Aspirin (hemolysis), Trimethoprim-sulfamethoxazole (hemolysis)

FAMILY HISTORY:
- Maternal uncle: Glucose-6-phosphate dehydrogenase deficiency
- Mother: G6PD carrier
- Paternal grandfather: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per month. Occupation: Construction laborer.

PHYSICAL EXAMINATION:
On examination, his visual acuity is 20/200 in the right eye and 20/20 in the left eye. Fundoscopic examination of the right eye reveals areas of retinal whitening with hemorrhages and granular borders, but no evidence of cotton wool spots.

LABORATORY DATA:
His CD4 count is 45 cells/mm³ and his viral load is 120,000 copies/mL.

Laboratory Values:
- CD4 count: 45 cells/mm³
- HIV viral load: 120,000 copies/mL

ASSESSMENT AND PLAN:
Based on the patient's history and examination findings,

## Encounter 6: source 2022-09-15, practice 2025-09-06, ed, Emergency Medicine

Chief complaint: Sharp central chest pain worsening with inspiration and lying flat after upper respiratory infection

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sharp central chest pain worsening with inspiration and lying flat after upper respiratory infection

HISTORY OF PRESENT ILLNESS:
Patient is a 35-year-old male with G6PD deficiency, IgA nephropathy, and HIV/AIDS (Stage 3) non-adherent to ART with recent CMV esophagitis. He presents to emergency department with 2-day history of sharp, central chest pain worsening with deep inspiration and when lying flat. He reports low-grade fever and recent upper respiratory tract infection.

PAST MEDICAL HISTORY:
Active Problem List:
- IgA nephropathy (diagnosed 2020-01-15)
- Glucose-6-phosphate dehydrogenase deficiency (diagnosed 2020-03-15)
- Methicillin-resistant Staphylococcus aureus (MRSA) skin and soft tissue infection (diagnosed 2021-09-15)
- Cytomegalovirus esophagitis (diagnosed 2022-03-15)
- HIV/AIDS (CDC Stage 3, diagnosed age 32, currently non-adherent to ART)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Bictegravir/Emtricitabine/Tenofovir alafenamide Non-adherent (discontinued 1 year ago due to insurance loss)

ALLERGIES:
Primaquine (severe hemolysis), Aspirin (hemolysis), Trimethoprim-sulfamethoxazole (hemolysis)

FAMILY HISTORY:
- Maternal uncle: Glucose-6-phosphate dehydrogenase deficiency
- Mother: G6PD carrier
- Paternal grandfather: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per month. Occupation: Construction laborer.

VITAL SIGNS:
On examination, his blood pressure is 130/80 mmHg, heart rate is 100 bpm, respiratory rate is 18 breaths per minute, and temperature is 37.8°C (100°F).

PHYSICAL EXAMINATION:
Cardiac examination reveals a pericardial friction rub.

LABORATORY DATA:
Laboratory tests reveal:
- WBC count: 12,000 /µL
- ESR: 40 mm/hr
- CRP: 25 mg/L
- Troponin I: <0.04 ng/mL (normal)

IMAGING:
An echocardiogram shows no evidence of tamponade or left ventricular dysfunction.

OTHER STUDIES:
An ECG shows diffuse ST-segment elevations and PR depressions.

## Encounter 7: source 2023-03-15, practice 2026-02-24, inpatient, Cardiology

Chief complaint: Progressive shortness of breath, fatigue, and lower extremity edema

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. Robert Thompson, MD
Department: Cardiology
Visit Type: inpatient

CHIEF COMPLAINT:
Progressive shortness of breath, fatigue, and lower extremity edema

HISTORY OF PRESENT ILLNESS:
The 36-year-old male with G6PD deficiency, IgA nephropathy, HIV/AIDS with ART non-adherence, and prior pericarditis presents to the emergency department with a 2-week history of progressive shortness of breath, fatigue, and lower extremity edema. He reports a recent upper respiratory tract infection that resolved 1 week ago. He denies chest pain, palpitations, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- IgA nephropathy (diagnosed 2020-01-15)
- Glucose-6-phosphate dehydrogenase deficiency (diagnosed 2020-03-15)
- Methicillin-resistant Staphylococcus aureus (MRSA) skin and soft tissue infection (diagnosed 2021-09-15)
- Cytomegalovirus esophagitis (diagnosed 2022-03-15)
- Acute idiopathic pericarditis (diagnosed 2022-09-15)
- HIV/AIDS (CDC Stage 3, diagnosed age 32, currently non-adherent to ART)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Bictegravir/Emtricitabine/Tenofovir alafenamide Non-adherent (discontinued 1 year ago due to insurance loss)

ALLERGIES:
Primaquine (severe hemolysis), Aspirin (hemolysis), Trimethoprim-sulfamethoxazole (hemolysis)

FAMILY HISTORY:
- Maternal uncle: Glucose-6-phosphate dehydrogenase deficiency
- Mother: G6PD carrier
- Paternal grandfather: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per month. Occupation: Construction laborer.

VITAL SIGNS:
His blood pressure is 100/70 mmHg, heart rate is 110 bpm, respiratory rate is 22 breaths/min, and oxygen saturation is 94% on room air

PHYSICAL EXAMINATION:
Jugular venous pressure is elevated. Cardiac auscultation reveals a third heart sound (S3) gallop, and there are bibasilar crackles on lung auscultation. Lower extremities show 2+ pitting edema.

IMAGING:
Chest X-ray demonstrates cardiomegaly and pulmonary congestion. An echocardiogram shows biventricular enlargement with an ejection fraction of 30% and diffuse hypokinesis without valvular abnormalities.
