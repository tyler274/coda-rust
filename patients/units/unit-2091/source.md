# synthetic_hospital patient 2091

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 43 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-06-14 (Mon) | outpatient | Infectious Disease |
| 1 | 2020-03-15 | 2021-08-13 (Fri) | ed | Emergency Medicine |
| 2 | 2020-04-15 | 2021-09-19 (Sun) | ed | Emergency Medicine |
| 3 | 2021-01-15 | 2022-06-28 (Tue) | ed | Emergency Medicine |
| 4 | 2022-01-15 | 2023-06-24 (Sat) | ed | Emergency Medicine |
| 5 | 2022-03-15 | 2023-08-19 (Sat) | inpatient | Internal Medicine |
| 6 | 2024-01-15 | 2025-06-27 (Fri) | ed | Emergency Medicine |
| 7 | 2025-01-15 | 2026-07-01 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicaid",
 "occupation": "Retail sales associate",
 "alcohol_use": "None",
 "family_history": [
  "Father: Hypertension",
  "Mother: Type 2 diabetes mellitus",
  "No family history of malignancy or immunodeficiency"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "50/200/25 mg tablet daily (partially adherent)",
   "name": "Bictegravir/Emtricitabine/Tenofovir alafenamide"
  },
  {
   "dose": "160/800 mg tablet daily for PCP prophylaxis",
   "name": "Trimethoprim-sulfamethoxazole"
  },
  {
   "dose": "200 mg tablet daily for cryptococcal meningitis secondary prophylaxis",
   "name": "Fluconazole"
  },
  {
   "dose": "500 mg tablet twice daily for seizure prophylaxis",
   "name": "Levetiracetam"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "HIV/AIDS (CD4 count nadir 32 cells/mm\u00b3)",
  "History of Pneumocystis jirovecii pneumonia",
  "History of cryptococcal meningitis",
  "History of bacillary angiomatosis",
  "History of primary central nervous system lymphoma",
  "History of methemoglobinemia secondary to dapsone",
  "Seizure disorder"
 ],
 "age_at_first_encounter": 43
}
```

Primary diagnoses: ["History of Pneumocystis jirovecii pneumonia", "Acquired methemoglobinemia", "Pneumocystis jirovecii pneumonia", "Immune reconstitution inflammatory syndrome", "Bacillary angiomatosis", "Primary central nervous system lymphoma", "Cryptococcal meningitis"]
Comorbidities: ["Cancer-related constitutional symptoms", "History of Pneumocystis jirovecii pneumonia", "Immunosuppressed state", "HIV-related opportunistic skin infection", "Oral candidiasis"]

## Encounter 0: source 2020-01-15, practice 2021-06-14, outpatient, Infectious Disease

Chief complaint: Progressive violaceous skin lesions and constitutional symptoms over 2 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Infectious Disease
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive violaceous skin lesions and constitutional symptoms over 2 months

HISTORY OF PRESENT ILLNESS:
He reports associated weight loss, night sweats, and low-grade fevers.

PAST MEDICAL HISTORY:
Active Problem List:
- HIV/AIDS (CD4 count nadir 32 cells/mm³)
- History of Pneumocystis jirovecii pneumonia
- History of cryptococcal meningitis
- History of bacillary angiomatosis
- History of primary central nervous system lymphoma
- History of methemoglobinemia secondary to dapsone
- Seizure disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
His antiretroviral therapy was discontinued 6 months ago

Home Medications:
- Bictegravir/Emtricitabine/Tenofovir alafenamide 50/200/25 mg tablet daily (partially adherent)
- Trimethoprim-sulfamethoxazole 160/800 mg tablet daily for PCP prophylaxis
- Fluconazole 200 mg tablet daily for cryptococcal meningitis secondary prophylaxis
- Levetiracetam 500 mg tablet twice daily for seizure prophylaxis

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension
- Mother: Type 2 diabetes mellitus
- No family history of malignancy or immunodeficiency

SOCIAL HISTORY:
Due to financial constraints

PHYSICAL EXAMINATION:
On examination, you note several raised, red-purple nodules scattered across his upper body and arms. Some of the lesions have a friable, exophytic appearance.

LABORATORY DATA:
His CD4 count is 50 cells/mm³, and his viral load is markedly elevated.

PATHOLOGY:
A biopsy of one of the lesions shows lobular proliferation of vessels lined by endothelial cells and neutrophilic infiltrate.

## Encounter 1: source 2020-03-15, practice 2021-08-13, ed, Emergency Medicine

Chief complaint: Acute fever, productive cough, and shortness of breath

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute fever, productive cough, and shortness of breath

HISTORY OF PRESENT ILLNESS:
This 43-year-old male with HIV/AIDS (CD4 nadir 32 cells/mm³) and history of Pneumocystis jirovecii pneumonia, cryptococcal meningitis, and bacillary angiomatosis treated two months prior presents with acute fever, productive cough, and shortness of breath. He denies any recent travel or sick contacts.

PAST MEDICAL HISTORY:
Active Problem List:
- Bacillary angiomatosis (diagnosed 2020-01-15)
- HIV/AIDS (CD4 count nadir 32 cells/mm³)
- History of Pneumocystis jirovecii pneumonia
- History of cryptococcal meningitis
- History of primary central nervous system lymphoma
- History of methemoglobinemia secondary to dapsone
- Seizure disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Bictegravir/Emtricitabine/Tenofovir alafenamide 50/200/25 mg tablet daily (partially adherent)
- Trimethoprim-sulfamethoxazole 160/800 mg tablet daily for PCP prophylaxis
- Fluconazole 200 mg tablet daily for cryptococcal meningitis secondary prophylaxis
- Levetiracetam 500 mg tablet twice daily for seizure prophylaxis

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension
- Mother: Type 2 diabetes mellitus
- No family history of malignancy or immunodeficiency

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retail sales associate.

VITAL SIGNS:
His temperature is 38.7°C, blood pressure is 110/70 mmHg, heart rate is 102 beats/min, respiratory rate is 22 breaths/min, and oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
Physical examination reveals crackles in the right lower lung field.

LABORATORY DATA:
Laboratory tests show a white blood cell count of 12,000/µL with a left shift.

IMAGING:
A chest x-ray is performed and shows no infiltrates.

## Encounter 2: source 2020-04-15, practice 2021-09-19, ed, Emergency Medicine

Chief complaint: Worsening dyspnea and nonproductive cough three weeks after hospital discharge

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening dyspnea and nonproductive cough three weeks after hospital discharge

HISTORY OF PRESENT ILLNESS:
The patient is a 43-year-old male with known HIV/AIDS (CD4 count nadir 32 cells/mm³), history of bacillary angiomatosis, and recent hospitalization for Pneumocystis jirovecii pneumonia who returns to the emergency department three weeks after discharge with worsening dyspnea and nonproductive cough. He was recently discharged from the hospital 3 weeks ago after being diagnosed with HIV and treated for Pneumocystis jirovecii pneumonia with intravenous trimethoprim-sulfamethoxazole and corticosteroids. At the time of admission, he had oral thrush, unintentional weight loss, and a CD4 count of 32/mm³. He was started on antiretroviral therapy (ART) two weeks ago, after completing a course of antibiotics and steroids. Initially, his respiratory symptoms improved, but they have gradually returned and worsened. He now reports intermittent fevers and night sweats, but denies sputum production, chest pain, or hemoptysis.

PAST MEDICAL HISTORY:
Active Problem List:
- Bacillary angiomatosis (diagnosed 2020-01-15)
- Pneumocystis jirovecii pneumonia (diagnosed 2020-03-15)
- HIV/AIDS (CD4 count nadir 32 cells/mm³)
- History of cryptococcal meningitis
- History of primary central nervous system lymphoma
- History of methemoglobinemia secondary to dapsone
- Seizure disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Bictegravir/Emtricitabine/Tenofovir alafenamide 50/200/25 mg tablet daily (partially adherent)
- Trimethoprim-sulfamethoxazole 160/800 mg tablet daily for PCP prophylaxis
- Fluconazole 200 mg tablet daily for cryptococcal meningitis secondary prophylaxis
- Levetiracetam 500 mg tablet twice daily for seizure prophylaxis

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension
- Mother: Type 2 diabetes mellitus
- No family history of malignancy or immunodeficiency

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retail sales associate.

VITAL SIGNS:
Temperature is 38.3°C (100.9°F), blood pressure 112/64 mmHg, pulse 98/min, and oxygen saturation 95% on room air.

PHYSICAL EXAMINATION:
On exam, he appears cachectic and mildly tachypneic.
Chest auscultation reveals bilateral fine crackles.

LABORATORY DATA:
Repeat labs show a rising CD4 count of 46/mm³ and decreasing HIV viral load.

IMAGING:
A chest x-ray shows bilateral diffuse interstitial infiltrates. An image of the chest x-ray is shown.

## Encounter 3: source 2021-01-15, practice 2022-06-28, ed, Emergency Medicine

Chief complaint: Sudden cyanosis and dyspnea

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden cyanosis and dyspnea

HISTORY OF PRESENT ILLNESS:
This 44-year-old male with advanced HIV/AIDS and prior methemoglobinemia presents with cyanosis and dyspnea. Brought by his partner, he reports fatigue and dizziness for the past few hours.

PAST MEDICAL HISTORY:
Active Problem List:
- Bacillary angiomatosis (diagnosed 2020-01-15)
- Pneumocystis jirovecii pneumonia (diagnosed 2020-03-15)
- Immune reconstitution inflammatory syndrome (diagnosed 2020-04-15)
- HIV/AIDS (CD4 count nadir 32 cells/mm³)
- History of cryptococcal meningitis
- History of primary central nervous system lymphoma
- History of methemoglobinemia secondary to dapsone
- Seizure disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes trimethoprim-sulfamethoxazole and dapsone.

Home Medications:
- Bictegravir/Emtricitabine/Tenofovir alafenamide 50/200/25 mg tablet daily (partially adherent)
- Fluconazole 200 mg tablet daily for cryptococcal meningitis secondary prophylaxis
- Levetiracetam 500 mg tablet twice daily for seizure prophylaxis

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension
- Mother: Type 2 diabetes mellitus
- No family history of malignancy or immunodeficiency

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retail sales associate.

VITAL SIGNS:
Respiratory rate of 28 breaths per minute and oxygen saturation of 85% on room air, which does not improve with supplemental oxygen

PHYSICAL EXAMINATION:
Alert but anxious. His skin and lips have a bluish hue.

LABORATORY DATA:
Arterial blood gas analysis reveals:
- pH: 7.45
- PaCO2: 35 mmHg
- PaO2: 95 mmHg
- O2 Saturation (calculated): 97%

## Encounter 4: source 2022-01-15, practice 2023-06-24, ed, Emergency Medicine

Chief complaint: Fever, severe headache, neck stiffness, and confusion

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, severe headache, neck stiffness, and confusion

HISTORY OF PRESENT ILLNESS:
The patient is a 45-year-old male with advanced HIV/AIDS (CD4 nadir 32) and prior cryptococcal meningitis who presents to the emergency department. He has become increasingly fatigued and mildly confused over the past 3 days. His partner mentions that he has lost interest in daily activities and has been sleeping most of the day.

PAST MEDICAL HISTORY:
Active Problem List:
- Bacillary angiomatosis (diagnosed 2020-01-15)
- Pneumocystis jirovecii pneumonia (diagnosed 2020-03-15)
- Immune reconstitution inflammatory syndrome (diagnosed 2020-04-15)
- Acquired methemoglobinemia (diagnosed 2021-01-15)
- HIV/AIDS (CD4 count nadir 32 cells/mm³)
- History of cryptococcal meningitis
- History of primary central nervous system lymphoma
- History of methemoglobinemia secondary to dapsone
- Seizure disorder

Past medical history is notable for significant weight loss over the last few months, and he was recently treated for oral candidiasis.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Bictegravir/Emtricitabine/Tenofovir alafenamide 50/200/25 mg tablet daily (partially adherent)
- Trimethoprim-sulfamethoxazole 160/800 mg tablet daily for PCP prophylaxis
- Fluconazole 200 mg tablet daily for cryptococcal meningitis secondary prophylaxis
- Levetiracetam 500 mg tablet twice daily for seizure prophylaxis

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension
- Mother: Type 2 diabetes mellitus
- No family history of malignancy or immunodeficiency

SOCIAL HISTORY:
He does not smoke or drink, but he has a pet parrot at home.

VITAL SIGNS:
On physical examination, the patient is afebrile with normal blood pressure and mildly increased respiratory rate.

PHYSICAL EXAMINATION:
Neurological exam is nonfocal, with no nuchal rigidity or papilledema.

LABORATORY DATA:
Lumbar puncture reveals the following: opening pressure 320 mm H₂O, WBC 80/mm³ with 85% mononuclear cells, protein 120 mg/dL, glucose 30 mg/dL, and 0 RBCs.

## Encounter 5: source 2022-03-15, practice 2023-08-19, inpatient, Internal Medicine

Chief complaint: Recurrent headache, visual changes, and cognitive decline

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Fatima Al-Hassan, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Recurrent headache, visual changes, and cognitive decline

HISTORY OF PRESENT ILLNESS:
This 45-year-old male with HIV/AIDS is returning for evaluation two months after cryptococcal meningitis. He reports difficulty concentrating and recent episodes of blurry vision. He has no history of trauma or recent antibiotic use.

PAST MEDICAL HISTORY:
Active Problem List:
- Bacillary angiomatosis (diagnosed 2020-01-15)
- Pneumocystis jirovecii pneumonia (diagnosed 2020-03-15)
- Immune reconstitution inflammatory syndrome (diagnosed 2020-04-15)
- Acquired methemoglobinemia (diagnosed 2021-01-15)
- Cryptococcal meningitis (diagnosed 2022-01-15)
- HIV/AIDS (CD4 count nadir 32 cells/mm³)
- History of primary central nervous system lymphoma
- History of methemoglobinemia secondary to dapsone
- Seizure disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Bictegravir/Emtricitabine/Tenofovir alafenamide 50/200/25 mg tablet daily (partially adherent)
- Trimethoprim-sulfamethoxazole 160/800 mg tablet daily for PCP prophylaxis
- Fluconazole 200 mg tablet daily for cryptococcal meningitis secondary prophylaxis
- Levetiracetam 500 mg tablet twice daily for seizure prophylaxis

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension
- Mother: Type 2 diabetes mellitus
- No family history of malignancy or immunodeficiency

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retail sales associate.

PHYSICAL EXAMINATION:
On exam, he is afebrile but appears lethargic and responds slowly to questions. Neck stiffness is noted on passive flexion. Fundoscopy is normal.

IMAGING:
Brain MRI shows no mass effect

OTHER STUDIES:
A lumbar puncture is performed. A light microscopy is shown.

## Encounter 6: source 2024-01-15, practice 2025-06-27, ed, Emergency Medicine

Chief complaint: New-onset seizures and progressive confusion

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
New-onset seizures and progressive confusion

HISTORY OF PRESENT ILLNESS:
47-year-old with HIV/AIDS, CNS lymphoma and cryptococcal meningitis presents with two days of new-onset seizures and progressive confusion. He has been partially adherent to antiretroviral therapy.

PAST MEDICAL HISTORY:
Active Problem List:
- Bacillary angiomatosis (diagnosed 2020-01-15)
- Pneumocystis jirovecii pneumonia (diagnosed 2020-03-15)
- Immune reconstitution inflammatory syndrome (diagnosed 2020-04-15)
- Acquired methemoglobinemia (diagnosed 2021-01-15)
- Cryptococcal meningitis (diagnosed 2022-01-15)
- HIV/AIDS (CD4 count nadir 32 cells/mm³)
- History of primary central nervous system lymphoma
- History of methemoglobinemia secondary to dapsone
- Seizure disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes trimethoprim-sulfamethoxazole (TMP-SMX) daily for Pneumocystis jirovecii prophylaxis.

Home Medications:
- Bictegravir/Emtricitabine/Tenofovir alafenamide 50/200/25 mg tablet daily (partially adherent)
- Fluconazole 200 mg tablet daily for cryptococcal meningitis secondary prophylaxis
- Levetiracetam 500 mg tablet twice daily for seizure prophylaxis

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension
- Mother: Type 2 diabetes mellitus
- No family history of malignancy or immunodeficiency

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retail sales associate.

VITAL SIGNS:
Temperature is 37.9 °C (100.2 °F), blood pressure 116/70 mm Hg, and pulse 90/min

PHYSICAL EXAMINATION:
Neurologic exam shows disorientation to time and place but no focal weakness

LABORATORY DATA:
CD4 count is 38/mm³

IMAGING:
Non-contrast brain MRI reveals a single, 2-cm ring-enhancing lesion in the right basal ganglia with surrounding edema

## Encounter 7: source 2025-01-15, practice 2026-07-01, ed, Emergency Medicine

Chief complaint: Progressive dyspnea and nonproductive cough over 3 weeks with weight loss

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Elena Volkov, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive dyspnea and nonproductive cough over 3 weeks with weight loss

HISTORY OF PRESENT ILLNESS:
The patient is a 48-year-old male with advanced HIV/AIDS (CD4 nadir 32 cells/mm³) and a history of Pneumocystis jirovecii pneumonia, cryptococcal meningitis, and primary central nervous system lymphoma, who presents with progressive dyspnea and nonproductive cough over three weeks. He reports a 10-pound weight loss over the last month.

PAST MEDICAL HISTORY:
Active Problem List:
- Bacillary angiomatosis (diagnosed 2020-01-15)
- Pneumocystis jirovecii pneumonia (diagnosed 2020-03-15)
- Immune reconstitution inflammatory syndrome (diagnosed 2020-04-15)
- Acquired methemoglobinemia (diagnosed 2021-01-15)
- Cryptococcal meningitis (diagnosed 2022-01-15)
- Primary central nervous system lymphoma (diagnosed 2024-01-15)
- HIV/AIDS (CD4 count nadir 32 cells/mm³)
- History of methemoglobinemia secondary to dapsone
- Seizure disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Bictegravir/Emtricitabine/Tenofovir alafenamide 50/200/25 mg tablet daily (partially adherent)
- Trimethoprim-sulfamethoxazole 160/800 mg tablet daily for PCP prophylaxis
- Fluconazole 200 mg tablet daily for cryptococcal meningitis secondary prophylaxis
- Levetiracetam 500 mg tablet twice daily for seizure prophylaxis

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension
- Mother: Type 2 diabetes mellitus
- No family history of malignancy or immunodeficiency

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retail sales associate.

VITAL SIGNS:
On examination, he is afebrile, tachypneic with a respiratory rate of 28 breaths per minute, and oxygen saturation is 88% on room air.

PHYSICAL EXAMINATION:
Auscultation of the lungs reveals bilateral diffuse crackles.

LABORATORY DATA:
Arterial blood gas on room air shows:

- pH: 7.48
- PaCO2: 30 mmHg
- PaO2: 55 mmHg
- HCO3: 24 mEq/L

IMAGING:
Chest x-ray shows bilateral interstitial infiltrates.
