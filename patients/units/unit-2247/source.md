# synthetic_hospital patient 2247

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2018-01-29 (Mon) | outpatient | Pulmonary & Critical Care |
| 1 | 2020-07-15 | 2018-08-06 (Mon) | follow_up | Orthopedic Surgery |
| 2 | 2021-01-15 | 2019-01-23 (Wed) | outpatient | Dermatology |
| 3 | 2023-01-15 | 2021-02-01 (Mon) | outpatient | Pulmonary & Critical Care |
| 4 | 2024-01-15 | 2022-01-31 (Mon) | outpatient | Pulmonary & Critical Care |
| 5 | 2026-01-15 | 2024-02-05 (Mon) | outpatient | Pulmonary & Critical Care |
| 6 | 2027-01-15 | 2025-02-10 (Mon) | procedure | Dermatology |
| 7 | 2028-01-15 | 2026-01-31 (Sat) | inpatient | Gastroenterology & Nutrition |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Seasonal allergens"
 ],
 "insurance": "Medicare",
 "occupation": "Retired construction worker and former furniture manufacturing plant assembler",
 "alcohol_use": "Social",
 "family_history": [
  "Father: Lung cancer and emphysema",
  "Mother: Hypertension",
  "Paternal uncle: Pancreatic cancer"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 40 pack-years, quit 10 years ago",
 "home_medications": [
  {
   "dose": "25 mg daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "20 mg daily with food",
   "name": "Rivaroxaban"
  }
 ],
 "surgical_history": [
  "Total hip arthroplasty"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Atrial Fibrillation",
  "Obesity",
  "Seasonal Allergic Rhinitis"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Idiopathic Pulmonary Fibrosis", "Keratoacanthoma", "Basal Cell Carcinoma (Nodular type, of nose)", "Long-term anticoagulant therapy", "Severe upper lobe predominant emphysema", "Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation"]
Comorbidities: ["History of recent pneumonia", "Cachexia", "Essential hypertension", "Tobacco use disorder, former smoker"]

## Encounter 0: source 2020-01-15, practice 2018-01-29, outpatient, Pulmonary & Critical Care

Chief complaint: Shortness of breath and dry cough for 6 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonary & Critical Care
Visit Type: outpatient

CHIEF COMPLAINT:
Shortness of breath and dry cough for 6 months

HISTORY OF PRESENT ILLNESS:
A 67-year-old male presents to the clinic with a 6-month history of gradually worsening shortness of breath and a dry cough.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Atrial Fibrillation
- Obesity
- Seasonal Allergic Rhinitis

He has a past medical history of hypertension.

PAST SURGICAL HISTORY:
- Total hip arthroplasty

MEDICATIONS:
He takes for which he takes hydrochlorothiazide.

Home Medications:
- Rivaroxaban 20 mg daily with food

ALLERGIES:
Seasonal allergens

FAMILY HISTORY:
- Father: Lung cancer and emphysema
- Mother: Hypertension
- Paternal uncle: Pancreatic cancer

SOCIAL HISTORY:
He is a retired construction worker and has a 30-pack-year smoking history, although he quit smoking 10 years ago.

VITAL SIGNS:
His oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
On physical examination, you note fine crackles at the bases of both lungs.

IMAGING:
High-resolution computed tomography (HRCT) of the chest shows bilateral subpleural reticular opacities with honeycombing but no evidence of significant lymphadenopathy or pleural effusion.

OTHER STUDIES:
Pulmonary function tests reveal a restrictive pattern with a forced vital capacity (FVC) of 65% predicted and a reduced diffusion capacity for carbon monoxide (DLCO) of 45% predicted.

## Encounter 1: source 2020-07-15, practice 2018-08-06, follow_up, Orthopedic Surgery

Chief complaint: Routine follow-up after hospital discharge for total hip replacement

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Orthopedic Surgery
Visit Type: follow_up

CHIEF COMPLAINT:
Routine follow-up after hospital discharge for total hip replacement

HISTORY OF PRESENT ILLNESS:
The patient is a 67-year-old male with atrial fibrillation on anticoagulation and idiopathic pulmonary fibrosis who returns for routine follow-up after hospital discharge for total hip replacement. Three weeks ago, he underwent total hip replacement surgery following severe osteoarthritis that had caused debilitating pain and immobility. He denies bleeding, bruising, or black stools. He notes improved mobility and is working with physical therapy to regain strength. His surgical recovery has been uncomplicated, and his wound has healed well.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Hypertension
- Atrial Fibrillation
- Obesity
- Seasonal Allergic Rhinitis

His past medical history includes hypertension, atrial fibrillation, and obesity.

PAST SURGICAL HISTORY:
- Total hip arthroplasty

MEDICATIONS:
He reports that during his hospital stay, he was started on rivaroxaban to reduce the risk of blood clots. Since discharge, he has been adherent with his medication, which he takes orally once daily.

Home Medications:
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
Seasonal allergens

FAMILY HISTORY:
Family history is notable for a father who suffered a stroke at age 74.

SOCIAL HISTORY:
Social history reveals that he quit smoking 10 years ago and drinks alcohol socially.

PHYSICAL EXAMINATION:
Physical examination today is unremarkable, with well-healed surgical scars, stable vital signs, and no edema.

## Encounter 2: source 2021-01-15, practice 2019-01-23, outpatient, Dermatology

Chief complaint: Concern about a slowly growing lesion on the nose

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Emily Watson, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Concern about a slowly growing lesion on the nose

HISTORY OF PRESENT ILLNESS:
This 68-year-old male with idiopathic pulmonary fibrosis presents with concern about a nasal lesion. He reports the lesion has been slowly growing over the past year, with occasional bleeding during shaving but is otherwise asymptomatic.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Long-term anticoagulant therapy (diagnosed 2020-07-15)
- Hypertension
- Atrial Fibrillation
- Obesity
- Seasonal Allergic Rhinitis

PAST SURGICAL HISTORY:
- Total hip arthroplasty

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Rivaroxaban 20 mg daily with food

ALLERGIES:
Seasonal allergens

FAMILY HISTORY:
- Father: Lung cancer and emphysema
- Mother: Hypertension
- Paternal uncle: Pancreatic cancer

SOCIAL HISTORY:
45-pack-year smoking history, although he quit smoking 10 years ago

PHYSICAL EXAMINATION:
On physical examination, you note a pearly, flesh-colored papule with telangiectasias on the lateral aspect of his nose. There is no palpable lymphadenopathy.

PATHOLOGY:
A biopsy of the lesion is performed, and histopathology reveals nests of basaloid cells with peripheral palisading and retraction artifact.

## Encounter 3: source 2023-01-15, practice 2021-02-01, outpatient, Pulmonary & Critical Care

Chief complaint: Gradually worsening shortness of breath over the past 2 years

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonary & Critical Care
Visit Type: outpatient

CHIEF COMPLAINT:
Gradually worsening shortness of breath over the past 2 years

HISTORY OF PRESENT ILLNESS:
The patient is a 70-year-old male with idiopathic pulmonary fibrosis and atrial fibrillation who returns to Pulmonary and Critical Care with worsening dyspnea. He reports increasing difficulty climbing stairs and must pause frequently while walking on level ground. He has noticed frequent coughing spells and unintentional weight loss. He denies recent upper respiratory symptoms, chest pain, or fever.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Long-term anticoagulant therapy (diagnosed 2020-07-15)
- Basal Cell Carcinoma (Nodular type, of nose) (diagnosed 2021-01-15)
- Hypertension
- Atrial Fibrillation
- Obesity
- Seasonal Allergic Rhinitis

Past medical history includes hypertension and seasonal allergies.

PAST SURGICAL HISTORY:
- Total hip arthroplasty

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg daily
- Rivaroxaban 20 mg daily with food

ALLERGIES:
Seasonal allergens

FAMILY HISTORY:
- Father: Lung cancer and emphysema
- Mother: Hypertension
- Paternal uncle: Pancreatic cancer

SOCIAL HISTORY:
He smoked one pack of cigarettes daily for 35 years but quit 6 years ago.

PHYSICAL EXAMINATION:
On physical examination, he appears thin and is using accessory muscles of respiration. He breathes through pursed lips and has a barrel-shaped chest. Percussion of the lungs reveals hyperresonance.

IMAGING:
A chest X-ray shows flattened diaphragms and hyperlucent lower lobes.

OTHER STUDIES:
Pulmonary function tests demonstrate a reduced FEV1/FVC ratio with increased total lung capacity and residual volume.

## Encounter 4: source 2024-01-15, practice 2022-01-31, outpatient, Pulmonary & Critical Care

Chief complaint: Gradually worsening shortness of breath and persistent dry cough

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonary & Critical Care
Visit Type: outpatient

CHIEF COMPLAINT:
Gradually worsening shortness of breath and persistent dry cough

HISTORY OF PRESENT ILLNESS:
The patient with idiopathic pulmonary fibrosis and emphysema returns with worsening dyspnea and dry cough. He states that he becomes winded even with mild exertion, such as carrying groceries or climbing stairs. He denies chest pain, wheezing, fever, or sputum production.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Long-term anticoagulant therapy (diagnosed 2020-07-15)
- Basal Cell Carcinoma (Nodular type, of nose) (diagnosed 2021-01-15)
- Severe upper lobe predominant emphysema (diagnosed 2023-01-15)
- Hypertension
- Atrial Fibrillation
- Obesity
- Seasonal Allergic Rhinitis

PAST SURGICAL HISTORY:
- Total hip arthroplasty

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg daily
- Rivaroxaban 20 mg daily with food

ALLERGIES:
Seasonal allergens

FAMILY HISTORY:
- Father: Lung cancer and emphysema
- Mother: Hypertension
- Paternal uncle: Pancreatic cancer

SOCIAL HISTORY:
He smoked one pack of cigarettes daily for 40 years but quit 5 years ago. He worked in a furniture manufacturing plant for 35 years and reports long-term exposure to wood dust and solvents.

VITAL SIGNS:
Oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
On physical exam, his lungs reveal fine, end-inspiratory “Velcro-like” crackles at the bases and mild digital clubbing.

IMAGING:
Chest x-ray reveals reticular opacities in the lower lobes bilaterally. High-resolution CT shows subpleural reticulations, traction bronchiectasis, and honeycombing in a basilar distribution.

PATHOLOGY:
A lung biopsy is performed and reveals dense fibrosis with temporal heterogeneity and areas of microscopic honeycombing alternating with relatively preserved lung parenchyma. There are no granulomas or eosinophilic infiltrates.

OTHER STUDIES:
Pulmonary function tests show a reduced total lung capacity, normal FEV1/FVC ratio, and markedly decreased DLCO.

## Encounter 5: source 2026-01-15, practice 2024-02-05, outpatient, Pulmonary & Critical Care

Chief complaint: Severe dyspnea on exertion and persistent dry cough

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. James Park, MD
Department: Pulmonary & Critical Care
Visit Type: outpatient

CHIEF COMPLAINT:
Severe dyspnea on exertion and persistent dry cough

HISTORY OF PRESENT ILLNESS:
He returns for follow-up of idiopathic pulmonary fibrosis and emphysema previously documented in 2020 and 2023. He reports that he becomes winded just walking from his living room to the kitchen and has difficulty climbing stairs without stopping to rest. He denies chest pain, wheezing, or sputum production. He has not had any recent illnesses or exposure to toxic fumes.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Long-term anticoagulant therapy (diagnosed 2020-07-15)
- Basal Cell Carcinoma (Nodular type, of nose) (diagnosed 2021-01-15)
- Severe upper lobe predominant emphysema (diagnosed 2023-01-15)
- Hypertension
- Atrial Fibrillation
- Obesity
- Seasonal Allergic Rhinitis

His medical history is significant only for well-controlled hypertension.

PAST SURGICAL HISTORY:
- Total hip arthroplasty

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg daily
- Rivaroxaban 20 mg daily with food

ALLERGIES:
Seasonal allergens

FAMILY HISTORY:
- Father: Lung cancer and emphysema
- Mother: Hypertension
- Paternal uncle: Pancreatic cancer

SOCIAL HISTORY:
He smoked one pack of cigarettes daily for 40 years but quit 10 years ago.

VITAL SIGNS:
His oxygen saturation is 93% on room air.

PHYSICAL EXAMINATION:
On physical examination, he has fine end-inspiratory crackles at the lung bases and digital clubbing.

IMAGING:
The patient undergoes a chest x-ray as part of the initial evaluation.

OTHER STUDIES:
Pulmonary function testing shows a decreased total lung capacity and a reduced diffusion capacity for carbon monoxide. His FEV1/FVC ratio is 84%.

## Encounter 6: source 2027-01-15, practice 2025-02-10, procedure, Dermatology

Chief complaint: Rapidly growing nodule on right forearm for 3 months

ENCOUNTER NOTE
Date: 2027-01-15
Provider: Dr. Emily Watson, MD
Department: Dermatology
Visit Type: procedure

CHIEF COMPLAINT:
Rapidly growing nodule on right forearm for 3 months

HISTORY OF PRESENT ILLNESS:
The 74-year-old male with idiopathic pulmonary fibrosis and history of basal cell carcinoma presents to Dermatology. He reports that the lesion began as a small, painless bump but has since increased to 2 cm in diameter. He denies any bleeding or pruritus but notes a central area of crusting.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Long-term anticoagulant therapy (diagnosed 2020-07-15)
- Basal Cell Carcinoma (Nodular type, of nose) (diagnosed 2021-01-15)
- Severe upper lobe predominant emphysema (diagnosed 2023-01-15)
- Hypertension
- Atrial Fibrillation
- Obesity
- Seasonal Allergic Rhinitis

PAST SURGICAL HISTORY:
- Total hip arthroplasty

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Rivaroxaban 20 mg daily with food

ALLERGIES:
Seasonal allergens

FAMILY HISTORY:
- Father: Lung cancer and emphysema
- Mother: Hypertension
- Paternal uncle: Pancreatic cancer

SOCIAL HISTORY:
Extensive sun exposure, 30-pack-year smoking history, although he quit 10 years ago

PHYSICAL EXAMINATION:
On physical examination, the nodule is dome-shaped with a central keratin-filled crater and surrounding erythema. There is no palpable lymphadenopathy.

PATHOLOGY:
A biopsy of the lesion is performed, and histopathology shows well-differentiated squamous cells with minimal atypia and a central keratin core.

## Encounter 7: source 2028-01-15, practice 2026-01-31, inpatient, Gastroenterology & Nutrition

Chief complaint: Painless jaundice and unintentional weight loss

ENCOUNTER NOTE
Date: 2028-01-15
Provider: Dr. Aisha Patel, MD
Department: Gastroenterology & Nutrition
Visit Type: inpatient

CHIEF COMPLAINT:
Painless jaundice and unintentional weight loss

HISTORY OF PRESENT ILLNESS:
The patient with progressive idiopathic pulmonary fibrosis and skin malignancies presents with painless jaundice. He reports a significant loss of appetite and weight loss of 10 kg over the same period. He denies any abdominal pain, fever, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Long-term anticoagulant therapy (diagnosed 2020-07-15)
- Basal Cell Carcinoma (Nodular type, of nose) (diagnosed 2021-01-15)
- Severe upper lobe predominant emphysema (diagnosed 2023-01-15)
- Keratoacanthoma (diagnosed 2027-01-15)
- Hypertension
- Atrial Fibrillation
- Obesity
- Seasonal Allergic Rhinitis

PAST SURGICAL HISTORY:
- Total hip arthroplasty

MEDICATIONS:
He takes lisinopril.

Home Medications:
- Hydrochlorothiazide 25 mg daily
- Rivaroxaban 20 mg daily with food

ALLERGIES:
Seasonal allergens

FAMILY HISTORY:
- Father: Lung cancer and emphysema
- Mother: Hypertension
- Paternal uncle: Pancreatic cancer

SOCIAL HISTORY:
30-pack-year smoking history, although he quit smoking 10 years ago

PHYSICAL EXAMINATION:
On physical examination, the patient appears cachectic and has a palpable, nontender gallbladder.

LABORATORY DATA:
Laboratory findings are as follows:

- Total bilirubin: 6.2 mg/dL (normal: 0.3-1.2 mg/dL)
- Direct bilirubin: 4.8 mg/dL (normal: 0.0-0.3 mg/dL)
- Alkaline phosphatase: 900 U/L (normal: 40-120 U/L)
- Aspartate aminotransferase (AST): 68 U/L (normal: 10-40 U/L)
- Alanine aminotransferase (ALT): 55 U/L (normal: 7-56 U/L)
- CA 19-9: Elevated

IMAGING:
Abdominal ultrasound shows dilated intrahepatic and extrahepatic bile ducts with no evidence of gallstones. A mass is noted at the head of the pancreas.
