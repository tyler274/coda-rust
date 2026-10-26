# synthetic_hospital patient 2086

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 39 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-06-20 (Mon) | outpatient | Infectious Diseases |
| 1 | 2020-07-15 | 2022-12-12 (Mon) | outpatient | Dermatology |
| 2 | 2021-01-15 | 2023-06-07 (Wed) | outpatient | Dermatology |
| 3 | 2021-04-15 | 2023-09-11 (Mon) | outpatient | Dermatology |
| 4 | 2021-10-15 | 2024-03-14 (Thu) | outpatient | Internal Medicine |
| 5 | 2022-05-15 | 2024-10-02 (Wed) | ed | Emergency Medicine |
| 6 | 2022-11-15 | 2025-04-11 (Fri) | ed | Emergency Medicine |
| 7 | 2023-05-15 | 2025-10-15 (Wed) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Graphic designer",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Father: Hypertension, age 67",
  "Mother: Type 2 diabetes mellitus, age 64"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "400 mg PO BID",
   "name": "Raltegravir"
  },
  {
   "dose": "300-200 mg PO daily",
   "name": "Tenofovir disoproxil fumarate-emtricitabine"
  },
  {
   "dose": "DS 1 tablet PO daily (PCP prophylaxis)",
   "name": "Trimethoprim-sulfamethoxazole"
  }
 ],
 "surgical_history": [
  "Wisdom teeth extraction (age 22)"
 ],
 "chronic_conditions": [
  "Human Immunodeficiency Virus (HIV) infection with AIDS (CD4 <200)",
  "History of intravenous drug use (heroin)",
  "History of recurrent bacterial pneumonia",
  "History of Pneumocystis jirovecii pneumonia",
  "History of Kaposi sarcoma"
 ],
 "age_at_first_encounter": 39
}
```

Primary diagnoses: ["Kaposi sarcoma", "Cryptococcal meningitis", "Splenic abscess", "History of Pneumocystis jirovecii pneumonia", "HIV-related opportunistic skin infection"]
Comorbidities: ["Medication noncompliance", "Intravenous Drug Use (Substance Use Disorder)", "Immunosuppressed state", "HIV-related opportunistic skin infection", "Intravenous drug use (Substance use disorder)"]

## Encounter 0: source 2020-01-15, practice 2022-06-20, outpatient, Infectious Diseases

Chief complaint: Routine follow-up of his HIV infection

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Infectious Diseases
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up of his HIV infection

HISTORY OF PRESENT ILLNESS:
He was diagnosed 2 years ago after presenting with chronic diarrhea, oral thrush, and progressive weight loss. At the time of diagnosis, his CD4 count was significantly reduced, and his viral load was high. He was started on combination antiretroviral therapy. He has adhered closely to his regimen and reports that his energy levels have improved, his weight has stabilized, and he has not had any further opportunistic infections. Today, he reports no new complaints.

PAST MEDICAL HISTORY:
Active Problem List:
- Human Immunodeficiency Virus (HIV) infection with AIDS (CD4 <200)
- History of intravenous drug use (heroin)
- History of recurrent bacterial pneumonia
- History of Pneumocystis jirovecii pneumonia
- History of Kaposi sarcoma

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 22)

MEDICATIONS:
He takes three antiretroviral agents, including raltegravir.

Home Medications:
- Tenofovir disoproxil fumarate-emtricitabine 300-200 mg PO daily
- Trimethoprim-sulfamethoxazole DS 1 tablet PO daily (PCP prophylaxis)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, age 67
- Mother: Type 2 diabetes mellitus, age 64

SOCIAL HISTORY:
He does not smoke, drinks alcohol only socially, and works as a graphic designer.

REVIEW OF SYSTEMS:
He denies fever, chills, cough, chest pain, abdominal pain, or neuropathic symptoms. He has had no skin rashes and no medication-related gastrointestinal upset.

VITAL SIGNS:
On examination, he is afebrile, blood pressure is normal, and BMI is within the healthy range.

PHYSICAL EXAMINATION:
The rest of his physical exam is unremarkable.

LABORATORY DATA:
His most recent labs show an undetectable viral load and a steadily increasing CD4 count.

## Encounter 1: source 2020-07-15, practice 2022-12-12, outpatient, Dermatology

Chief complaint: 3-month history of progressive, painless skin lesions

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of progressive, painless skin lesions

HISTORY OF PRESENT ILLNESS:
This 39-year-old male with HIV/AIDS and prior intravenous drug use returns to Dermatology for evaluation of progressive skin lesions following his January Infectious Diseases encounter. He reports no fever, weight loss, or night sweats.

PAST MEDICAL HISTORY:
Active Problem List:
- HIV-related opportunistic skin infection (diagnosed 2020-01-15)
- Human Immunodeficiency Virus (HIV) infection with AIDS (CD4 <200)
- History of intravenous drug use (heroin)
- History of recurrent bacterial pneumonia
- History of Pneumocystis jirovecii pneumonia
- History of Kaposi sarcoma

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 22)

MEDICATIONS:
Home Medications:
- Raltegravir 400 mg PO BID
- Tenofovir disoproxil fumarate-emtricitabine 300-200 mg PO daily
- Trimethoprim-sulfamethoxazole DS 1 tablet PO daily (PCP prophylaxis)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, age 67
- Mother: Type 2 diabetes mellitus, age 64

SOCIAL HISTORY:
His medical history is significant for intravenous drug use and unprotected sexual encounters with multiple partners. He has not been in for a medical check-up for several years.

VITAL SIGNS:
His temperature is 37.2°C (99°F), blood pressure is 125/78 mmHg, and heart rate is 78 beats per minute.

PHYSICAL EXAMINATION:
On examination, you note multiple, non-blanching, violaceous papules and plaques on his lower extremities and oral mucosa.

LABORATORY DATA:
Laboratory tests reveal the following:

- CD4+ T-cell count: 180 cells/µL
- HIV-1 RNA (viral load): 120,000 copies/mL
- HHV-8 serology: Positive
- Complete blood count: Normal
- Comprehensive metabolic panel: Normal

## Encounter 2: source 2021-01-15, practice 2023-06-07, outpatient, Dermatology

Chief complaint: Multiple skin lesions increasing in size over 4 months

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Aisha Patel, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Multiple skin lesions increasing in size over 4 months

HISTORY OF PRESENT ILLNESS:
The 40-year-old male with HIV/AIDS (CD4 <200) and history of Kaposi sarcoma returns to Dermatology six months after his prior evaluation. He presents with multiple skin lesions that have been gradually increasing in size over the past 4 months. He reports mild tenderness in some areas and intermittent swelling of his lower legs. He has had unintentional weight loss and intermittent low-grade fevers over the past 2 months. He denies recent travel, animal exposures, or trauma to the affected areas.

PAST MEDICAL HISTORY:
Active Problem List:
- HIV-related opportunistic skin infection (diagnosed 2020-01-15)
- Kaposi sarcoma (diagnosed 2020-07-15)
- Human Immunodeficiency Virus (HIV) infection with AIDS (CD4 <200)
- History of intravenous drug use (heroin)
- History of recurrent bacterial pneumonia
- History of Pneumocystis jirovecii pneumonia

His past medical history is notable for being seropositive for HIV for the past 8 years, and he has been intermittently nonadherent to his antiretroviral regimen.

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 22)

MEDICATIONS:
Home Medications:
- Raltegravir 400 mg PO BID
- Tenofovir disoproxil fumarate-emtricitabine 300-200 mg PO daily
- Trimethoprim-sulfamethoxazole DS 1 tablet PO daily (PCP prophylaxis)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, age 67
- Mother: Type 2 diabetes mellitus, age 64

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Graphic designer.

PHYSICAL EXAMINATION:
Physical examination reveals several violaceous, raised plaques and nodules over the lower extremities and oral mucosa. No palpable lymphadenopathy is appreciated, and cardiopulmonary examination is unremarkable.

LABORATORY DATA:
Laboratory studies reveal a CD4+ T-lymphocyte count of 84/mm³.

OTHER STUDIES:
A punch biopsy of one of the cutaneous lesions is performed.

## Encounter 3: source 2021-04-15, practice 2023-09-11, outpatient, Dermatology

Chief complaint: Multiple skin lesions that have been gradually increasing in size over the past 4 months

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Multiple skin lesions that have been gradually increasing in size over the past 4 months

HISTORY OF PRESENT ILLNESS:
This 40-year-old with HIV/AIDS and prior Kaposi sarcoma returns for follow-up of progressive skin lesions, last seen three months ago. He reports mild tenderness in some areas and intermittent swelling of his lower legs. He has had unintentional weight loss and intermittent low-grade fevers over the past 2 months. He denies recent travel, animal exposures, or trauma to the affected areas.

PAST MEDICAL HISTORY:
Active Problem List:
- HIV-related opportunistic skin infection (diagnosed 2020-01-15)
- Kaposi sarcoma (diagnosed 2020-07-15)
- Human Immunodeficiency Virus (HIV) infection with AIDS (CD4 <200)
- History of intravenous drug use (heroin)
- History of recurrent bacterial pneumonia
- History of Pneumocystis jirovecii pneumonia

His past medical history is notable for being seropositive for HIV for the past 8 years, and he has been intermittently nonadherent to his antiretroviral regimen.

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 22)

MEDICATIONS:
Home Medications:
- Raltegravir 400 mg PO BID
- Tenofovir disoproxil fumarate-emtricitabine 300-200 mg PO daily
- Trimethoprim-sulfamethoxazole DS 1 tablet PO daily (PCP prophylaxis)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, age 67
- Mother: Type 2 diabetes mellitus, age 64

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Graphic designer.

PHYSICAL EXAMINATION:
Physical examination reveals several violaceous, raised plaques and nodules over the lower extremities and oral mucosa. No palpable lymphadenopathy is appreciated, and cardiopulmonary examination is unremarkable.

LABORATORY DATA:
Laboratory studies reveal a CD4+ T-lymphocyte count of 84/mm³.

OTHER STUDIES:
A punch biopsy of one of the cutaneous lesions is performed. Examination of tissue obtained on biopsy of the neoplasm is most likely to show

## Encounter 4: source 2021-10-15, practice 2024-03-14, outpatient, Internal Medicine

Chief complaint: Multiple violaceous lesions on his hard palate and lower extremities

ENCOUNTER NOTE
Date: 2021-10-15
Provider: Dr. James Thompson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Multiple violaceous lesions on his hard palate and lower extremities

HISTORY OF PRESENT ILLNESS:
He returns for follow-up of progressive KS. He also reports a chronic, nonproductive cough and several weeks of weight loss and night sweats.

PAST MEDICAL HISTORY:
Active Problem List:
- HIV-related opportunistic skin infection (diagnosed 2020-01-15)
- Kaposi sarcoma (diagnosed 2020-07-15)
- Human Immunodeficiency Virus (HIV) infection with AIDS (CD4 <200)
- History of intravenous drug use (heroin)
- History of recurrent bacterial pneumonia
- History of Pneumocystis jirovecii pneumonia

His past medical history is significant for intravenous drug use and recurrent bacterial pneumonia.

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 22)

MEDICATIONS:
Home Medications:
- Raltegravir 400 mg PO BID
- Tenofovir disoproxil fumarate-emtricitabine 300-200 mg PO daily
- Trimethoprim-sulfamethoxazole DS 1 tablet PO daily (PCP prophylaxis)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, age 67
- Mother: Type 2 diabetes mellitus, age 64

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Graphic designer.

PHYSICAL EXAMINATION:
On physical exam, there are multiple reddish-purple, non-blanching papules on the skin and oral mucosa.

IMAGING:
A chest CT reveals bilateral perihilar infiltrates and small pleural effusions.

PATHOLOGY:
A skin biopsy shows spindle-shaped cells, neovascularization, and extravasated red blood cells.

## Encounter 5: source 2022-05-15, practice 2024-10-02, ed, Emergency Medicine

Chief complaint: Worsening shortness of breath, a nonproductive cough, and low-grade fever

ENCOUNTER NOTE
Date: 2022-05-15
Provider: Dr. Elena Volkov, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening shortness of breath, a nonproductive cough, and low-grade fever

HISTORY OF PRESENT ILLNESS:
The patient is a 41-year-old male with HIV/AIDS, prior Pneumocystis jirovecii pneumonia, and Kaposi sarcoma, who presents to the emergency department with worsening shortness of breath, a nonproductive cough, and low-grade fever for the past 9 days. He has difficulty climbing stairs and reports night sweats and fatigue.

PAST MEDICAL HISTORY:
Active Problem List:
- HIV-related opportunistic skin infection (diagnosed 2020-01-15)
- Kaposi sarcoma (diagnosed 2020-07-15)
- Human Immunodeficiency Virus (HIV) infection with AIDS (CD4 <200)
- History of intravenous drug use (heroin)
- History of recurrent bacterial pneumonia
- History of Pneumocystis jirovecii pneumonia

His past medical history is significant for intravenous drug use and untreated HIV.

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 22)

MEDICATIONS:
He has not seen a physician in several years and is.

Home Medications:
- Raltegravir 400 mg PO BID
- Tenofovir disoproxil fumarate-emtricitabine 300-200 mg PO daily
- Trimethoprim-sulfamethoxazole DS 1 tablet PO daily (PCP prophylaxis)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, age 67
- Mother: Type 2 diabetes mellitus, age 64

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Graphic designer.

VITAL SIGNS:
On exam, he is afebrile but tachypneic with a respiratory rate of 28/min and an oxygen saturation of 91% on room air.

PHYSICAL EXAMINATION:
Lung auscultation reveals faint bilateral crackles.

LABORATORY DATA:
Laboratory testing reveals an absolute CD4 count of 82/mm³. Induced sputum staining is pending. Blood cultures are negative.

IMAGING:
Chest X-ray is shown.

PLAN:
He is admitted for further management.

## Encounter 6: source 2022-11-15, practice 2025-04-11, ed, Emergency Medicine

Chief complaint: Fever, left upper quadrant abdominal pain, and shortness of breath

ENCOUNTER NOTE
Date: 2022-11-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, left upper quadrant abdominal pain, and shortness of breath

HISTORY OF PRESENT ILLNESS:
This 41-year-old male with HIV/AIDS, history of Kaposi sarcoma, and prior Pneumocystis jirovecii pneumonia presents to the emergency department complaining of fever, left upper quadrant abdominal pain, and shortness of breath worsening over the past week. He denies any recent trauma or travel.

PAST MEDICAL HISTORY:
Active Problem List:
- HIV-related opportunistic skin infection (diagnosed 2020-01-15)
- Kaposi sarcoma (diagnosed 2020-07-15)
- History of Pneumocystis jirovecii pneumonia (diagnosed 2022-05-15)
- Human Immunodeficiency Virus (HIV) infection with AIDS (CD4 <200)
- History of intravenous drug use (heroin)
- History of recurrent bacterial pneumonia

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 22)

MEDICATIONS:
Home Medications:
- Raltegravir 400 mg PO BID
- Tenofovir disoproxil fumarate-emtricitabine 300-200 mg PO daily
- Trimethoprim-sulfamethoxazole DS 1 tablet PO daily (PCP prophylaxis)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, age 67
- Mother: Type 2 diabetes mellitus, age 64

SOCIAL HISTORY:
History of intravenous drug use

VITAL SIGNS:
On examination, his temperature is 38.9°C (102°F), heart rate is 110 beats per minute, respiratory rate is 22 breaths per minute, and blood pressure is 130/85 mmHg.

PHYSICAL EXAMINATION:
He appears diaphoretic and in distress. Palpation of the abdomen reveals tenderness in the left upper quadrant with a palpable mass. Auscultation of the chest reveals decreased breath sounds at the left base.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 18,000/mm³ (Normal: 4,500-11,000/mm³)
- Hemoglobin: 13.2 g/dL (Normal: 13.8-17.2 g/dL for males)
- Platelet count: 250,000/mm³ (Normal: 150,000-450,000/mm³)
- Blood cultures: Pending

IMAGING:
A chest X-ray shows a left pleural effusion. An abdominal ultrasound reveals a complex, hypoechoic lesion within the spleen.

## Encounter 7: source 2023-05-15, practice 2025-10-15, icu, Emergency Medicine

Chief complaint: Headache, fever, and progressive disorientation

ENCOUNTER NOTE
Date: 2023-05-15
Provider: Dr. Maria Santos, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Headache, fever, and progressive disorientation

HISTORY OF PRESENT ILLNESS:
This 42-year-old male with HIV/AIDS, intravenous drug use, and prior opportunistic infections returns to the emergency department with a 2-week history of headache, fever, and progressive disorientation. He has had several episodes of vomiting and reports his neck is stiff.

PAST MEDICAL HISTORY:
Active Problem List:
- HIV-related opportunistic skin infection (diagnosed 2020-01-15)
- Kaposi sarcoma (diagnosed 2020-07-15)
- History of Pneumocystis jirovecii pneumonia (diagnosed 2022-05-15)
- Splenic abscess (diagnosed 2022-11-15)
- Human Immunodeficiency Virus (HIV) infection with AIDS (CD4 <200)
- History of intravenous drug use (heroin)
- History of recurrent bacterial pneumonia

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 22)

MEDICATIONS:
Home Medications:
- Raltegravir 400 mg PO BID
- Tenofovir disoproxil fumarate-emtricitabine 300-200 mg PO daily
- Trimethoprim-sulfamethoxazole DS 1 tablet PO daily (PCP prophylaxis)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, age 67
- Mother: Type 2 diabetes mellitus, age 64

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Graphic designer.

LABORATORY DATA:
CSF analysis shows elevated opening pressure, high protein, low glucose, and high lymphocytes.
