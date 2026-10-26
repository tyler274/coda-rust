# synthetic_hospital patient 2096

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 47 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-11-13 (Mon) | outpatient | Internal Medicine |
| 1 | 2021-01-15 | 2024-11-08 (Fri) | outpatient | Internal Medicine |
| 2 | 2022-07-15 | 2026-05-06 (Wed) | ed | Emergency Medicine |
| 3 | 2022-09-15 | 2026-07-13 (Mon) | icu | Critical Care Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicaid",
 "occupation": "Construction laborer",
 "alcohol_use": "Occasional social use, 1-2 drinks per month",
 "family_history": [
  "Father: Hypertension, coronary artery disease (age 74)",
  "Mother: Type 2 diabetes mellitus, hypothyroidism (age 71)",
  "No family history of immunodeficiency, autoimmune disease, or bleeding disorders"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit approximately 10 years ago",
 "home_medications": [
  {
   "dose": "160/800 mg orally three times weekly (PCP prophylaxis)",
   "name": "Trimethoprim-sulfamethoxazole (TMP-SMX)"
  },
  {
   "dose": "400 mg orally every 6 hours as needed for joint pain",
   "name": "Ibuprofen"
  },
  {
   "dose": "500 mg orally every 6 hours as needed for fever or headache",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Open reduction and internal fixation of bilateral femoral fractures with splenectomy following motor vehicle collision (age 35), complicated postoperative course requiring multiple blood transfusions",
  "History of post-traumatic arthritis in right knee"
 ],
 "chronic_conditions": [
  "HIV/AIDS (advanced, not on antiretroviral therapy)",
  "Anemia of chronic disease",
  "Chronic kidney disease stage 2"
 ],
 "age_at_first_encounter": 47
}
```

Primary diagnoses: ["Nocardiosis (pulmonary with central nervous system involvement)", "Nocardiosis", "HIV-related opportunistic skin infection"]
Comorbidities: ["HIV-related opportunistic skin infection", "Medication noncompliance", "Cryoglobulinemia", "Peripheral neuropathy"]

## Encounter 0: source 2020-01-15, practice 2023-11-13, outpatient, Internal Medicine

Chief complaint: Progressively worsening fatigue, joint discomfort, and tea-colored urine over the past 3 months with recurrent rash on lower legs

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Progressively worsening fatigue, joint discomfort, and tea-colored urine over the past 3 months with recurrent rash on lower legs

HISTORY OF PRESENT ILLNESS:
He reports persistent dark, “tea-colored” urine and has recently developed mild bilateral swelling and pain in his hands, wrists, and knees that limits his ability to work. He also notes a recurrent reddish rash on his lower legs that seems to flare up in colder environments and does not go away when pressed. Over the past several weeks, he has begun to experience tingling and weakness in his feet, making it difficult to walk long distances.

PAST MEDICAL HISTORY:
Active Problem List:
- HIV/AIDS (advanced, not on antiretroviral therapy)
- Anemia of chronic disease
- Chronic kidney disease stage 2

His past medical history is notable for a complicated surgical procedure more than a decade ago that required multiple blood transfusions. Hepatitis A and B vaccinations are documented in his chart, and his current viral serologies are negative.

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of bilateral femoral fractures with splenectomy following motor vehicle collision (age 35), complicated postoperative course requiring multiple blood transfusions
- History of post-traumatic arthritis in right knee

MEDICATIONS:
Home Medications:
- Trimethoprim-sulfamethoxazole (TMP-SMX) 160/800 mg orally three times weekly (PCP prophylaxis)
- Ibuprofen 400 mg orally every 6 hours as needed for joint pain
- Acetaminophen 500 mg orally every 6 hours as needed for fever or headache

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension, coronary artery disease (age 74)
- Mother: Type 2 diabetes mellitus, hypothyroidism (age 71)
- No family history of immunodeficiency, autoimmune disease, or bleeding disorders

SOCIAL HISTORY:
Smoking: Former smoker, quit approximately 10 years ago. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Construction laborer.

VITAL SIGNS:
His temperature is 37.8°C, blood pressure 115/70 mmHg, and heart rate 90 bpm.

PHYSICAL EXAMINATION:
On physical examination, his lower extremities show multiple non-blanching, palpable purpuric lesions below the knees, and neurologic exam reveals diminished vibration sensation in the distal lower extremities.

LABORATORY DATA:
Laboratory evaluation shows elevated creatinine, low C4 complement levels, proteinuria, and microscopic hematuria.

## Encounter 1: source 2021-01-15, practice 2024-11-08, outpatient, Internal Medicine

Chief complaint: Persistent fatigue, joint swelling, and recurrent lower extremity rash with dark urine

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue, joint swelling, and recurrent lower extremity rash with dark urine

HISTORY OF PRESENT ILLNESS:
The patient is a 48-year-old male with advanced HIV/AIDS who is not receiving antiretroviral therapy, returning for follow-up care approximately one year after his initial presentation with similar symptoms. He reports persistent dark, "tea-colored" urine and has recently developed mild bilateral swelling and pain in his hands, wrists, and knees that limits his ability to work. He also notes a recurrent reddish rash on his lower legs that seems to flare up in colder environments and does not go away when pressed. Over the past several weeks, he has begun to experience tingling and weakness in his feet, making it difficult to walk long distances.

PAST MEDICAL HISTORY:
Active Problem List:
- HIV-related opportunistic skin infection (diagnosed 2020-01-15)
- HIV/AIDS (advanced, not on antiretroviral therapy)
- Anemia of chronic disease
- Chronic kidney disease stage 2

His past medical history is notable for a complicated surgical procedure more than a decade ago that required multiple blood transfusions.

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of bilateral femoral fractures with splenectomy following motor vehicle collision (age 35), complicated postoperative course requiring multiple blood transfusions
- History of post-traumatic arthritis in right knee

MEDICATIONS:
Home Medications:
- Trimethoprim-sulfamethoxazole (TMP-SMX) 160/800 mg orally three times weekly (PCP prophylaxis)
- Ibuprofen 400 mg orally every 6 hours as needed for joint pain
- Acetaminophen 500 mg orally every 6 hours as needed for fever or headache

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension, coronary artery disease (age 74)
- Mother: Type 2 diabetes mellitus, hypothyroidism (age 71)
- No family history of immunodeficiency, autoimmune disease, or bleeding disorders

SOCIAL HISTORY:
Smoking: Former smoker, quit approximately 10 years ago. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Construction laborer.

VITAL SIGNS:
His temperature is 37.8°C, blood pressure 115/70 mmHg, and heart rate 90 bpm.

PHYSICAL EXAMINATION:
On physical examination, his lower extremities show multiple non-blanching, palpable purpuric lesions below the knees, and neurologic exam reveals diminished vibration sensation in the distal lower extremities.

LABORATORY DATA:
Laboratory evaluation shows elevated creatinine, low C4 complement levels, proteinuria, and microscopic hematuria. Hepatitis A and B vaccinations are documented in his chart, and his current viral serologies are negative.

## Encounter 2: source 2022-07-15, practice 2026-05-06, ed, Emergency Medicine

Chief complaint: Three-week history of cough, fever, weight loss, and painful swollen lesion on right arm

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. James Okonkwo, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Three-week history of cough, fever, weight loss, and painful swollen lesion on right arm

HISTORY OF PRESENT ILLNESS:
The patient with advanced HIV/AIDS and prior opportunistic infections presents. He also reports a painful swelling on his right arm that started as a small pimple and has grown larger over the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- HIV-related opportunistic skin infection (diagnosed 2020-01-15)
- HIV/AIDS (advanced, not on antiretroviral therapy)
- Anemia of chronic disease
- Chronic kidney disease stage 2

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of bilateral femoral fractures with splenectomy following motor vehicle collision (age 35), complicated postoperative course requiring multiple blood transfusions
- History of post-traumatic arthritis in right knee

MEDICATIONS:
Home Medications:
- Trimethoprim-sulfamethoxazole (TMP-SMX) 160/800 mg orally three times weekly (PCP prophylaxis)
- Ibuprofen 400 mg orally every 6 hours as needed for joint pain
- Acetaminophen 500 mg orally every 6 hours as needed for fever or headache

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension, coronary artery disease (age 74)
- Mother: Type 2 diabetes mellitus, hypothyroidism (age 71)
- No family history of immunodeficiency, autoimmune disease, or bleeding disorders

SOCIAL HISTORY:
Smoking: Former smoker, quit approximately 10 years ago. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Construction laborer.

VITAL SIGNS:
His temperature is 38.2°C (100.8°F), blood pressure is 125/80 mmHg, and respiratory rate is 22 breaths per minute.

PHYSICAL EXAMINATION:
Physical examination reveals a single, fluctuant, erythematous nodule on his right forearm. Chest auscultation demonstrates scattered crackles in the right lung base.

LABORATORY DATA:
Laboratory tests reveal:
- White blood cell count: 9,000/µL
- Hemoglobin: 12.5 g/dL
- CD4+ count: 120 cells/µL
Sputum Gram stain shows filamentous, gram-positive, weakly acid-fast rods.

IMAGING:
Chest X-ray: patchy infiltrates in the right lower lobe

## Encounter 3: source 2022-09-15, practice 2026-07-13, icu, Critical Care Medicine

Chief complaint: Two-week history of cough, fever, shortness of breath, severe headache, and confusion

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. Maria Rodriguez, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Two-week history of cough, fever, shortness of breath, severe headache, and confusion

HISTORY OF PRESENT ILLNESS:
This 49-year-old male with advanced HIV/AIDS not on ART and nocardiosis two months prior presents to the emergency department with a 2-week history of cough, fever, and progressive shortness of breath. He also complains of severe headache and intermittent confusion.

PAST MEDICAL HISTORY:
Active Problem List:
- HIV-related opportunistic skin infection (diagnosed 2020-01-15)
- Nocardiosis (diagnosed 2022-07-15)
- HIV/AIDS (advanced, not on antiretroviral therapy)
- Anemia of chronic disease
- Chronic kidney disease stage 2

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of bilateral femoral fractures with splenectomy following motor vehicle collision (age 35), complicated postoperative course requiring multiple blood transfusions
- History of post-traumatic arthritis in right knee

MEDICATIONS:
Home Medications:
- Trimethoprim-sulfamethoxazole (TMP-SMX) 160/800 mg orally three times weekly (PCP prophylaxis)
- Ibuprofen 400 mg orally every 6 hours as needed for joint pain
- Acetaminophen 500 mg orally every 6 hours as needed for fever or headache

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension, coronary artery disease (age 74)
- Mother: Type 2 diabetes mellitus, hypothyroidism (age 71)
- No family history of immunodeficiency, autoimmune disease, or bleeding disorders

SOCIAL HISTORY:
Smoking: Former smoker, quit approximately 10 years ago. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Construction laborer.

VITAL SIGNS:
His temperature is 38.9°C, blood pressure is 110/70 mmHg, heart rate is 102 beats per minute, and respiratory rate is 22 breaths per minute.

PHYSICAL EXAMINATION:
Physical examination reveals crackles at the right lung base. Neurological examination is notable for mild disorientation to time and place.

LABORATORY DATA:
Laboratory tests reveal:

- White blood cell count: 12,000/µL
- Hemoglobin: 13.2 g/dL
- Platelet count: 250,000/µL
- CD4 count: 50 cells/mm³
- HIV viral load: 120,000 copies/mL

IMAGING:
A chest X-ray shows a right lower lobe infiltrate.

OTHER STUDIES:
A sputum sample is obtained and sent for Gram stain, which shows branching filamentous organisms.
