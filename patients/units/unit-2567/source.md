# synthetic_hospital patient 2567

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 34 at first encounter, sex M, South Asian (Bangladeshi), Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-10-13 (Sun) | inpatient | Gastroenterology |
| 1 | 2020-07-15 | 2025-04-14 (Mon) | icu | Pulmonary & Critical Care |
| 2 | 2021-01-15 | 2025-10-08 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Restaurant kitchen staff",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Father: Type 2 diabetes mellitus",
  "Mother: Hypertension",
  "No known hematologic malignancies in first-degree relatives"
 ],
 "race_ethnicity": "South Asian (Bangladeshi)",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "2 mg PO BID (post-transplant immunosuppression)",
   "name": "Tacrolimus"
  },
  {
   "dose": "160/800 mg PO three times weekly (Pneumocystis prophylaxis)",
   "name": "Trimethoprim-sulfamethoxazole"
  }
 ],
 "surgical_history": [
  "Percutaneous drainage of amebic liver abscess",
  "Allogeneic hematopoietic stem cell transplantation (matched unrelated donor)",
  "Placement of tunneled central venous catheter"
 ],
 "chronic_conditions": [
  "Substance use disorder (intravenous heroin use)",
  "History of amebic liver abscess",
  "Acute myeloid leukemia",
  "Status post allogeneic hematopoietic stem cell transplantation"
 ],
 "age_at_first_encounter": 34
}
```

Primary diagnoses: ["Pulmonary embolism", "Invasive pulmonary aspergillosis", "Amebic liver abscess"]
Comorbidities: ["Anemia"]

## Encounter 0: source 2020-01-15, practice 2024-10-13, inpatient, Gastroenterology

Chief complaint: 10-day history of fevers, chills, night sweats, and progressively worsening right upper quadrant abdominal pain

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Anika Patel, MD
Department: Gastroenterology
Visit Type: inpatient

CHIEF COMPLAINT:
10-day history of fevers, chills, night sweats, and progressively worsening right upper quadrant abdominal pain

HISTORY OF PRESENT ILLNESS:
He describes the pain as dull, poorly localized, and worse with deep inspiration or movement. He also reports anorexia and an unintentional weight loss of approximately 3 kg (6.5 lb) over the past 2 weeks. He denies nausea, vomiting, jaundice, or dark urine. He has not had any recent antibiotic use and has not undergone any recent surgeries.

PAST MEDICAL HISTORY:
Active Problem List:
- Substance use disorder (intravenous heroin use)
- History of amebic liver abscess
- Acute myeloid leukemia
- Status post allogeneic hematopoietic stem cell transplantation

PAST SURGICAL HISTORY:
- Percutaneous drainage of amebic liver abscess
- Allogeneic hematopoietic stem cell transplantation (matched unrelated donor)
- Placement of tunneled central venous catheter

MEDICATIONS:
Home Medications:
- Tacrolimus 2 mg PO BID (post-transplant immunosuppression)
- Trimethoprim-sulfamethoxazole 160/800 mg PO three times weekly (Pneumocystis prophylaxis)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus
- Mother: Hypertension
- No known hematologic malignancies in first-degree relatives

SOCIAL HISTORY:
The patient immigrated to the United States from Bangladesh 3 years ago and recently returned from a 4-week trip visiting extended family in rural Bangladesh. During his trip, he consumed local street food and untreated water and experienced two days of mild diarrhea that resolved without treatment. He does not drink alcohol, smoke, or use illicit drugs.

VITAL SIGNS:
On examination, his temperature is 38.5°C (101.3°F), heart rate is 96/min, and blood pressure is 110/70 mm Hg.

PHYSICAL EXAMINATION:
He appears fatigued but not acutely ill. There is tenderness to palpation in the right upper quadrant, and the liver edge is palpable 3 cm below the costal margin.

LABORATORY DATA:
Laboratory studies reveal hemoglobin of 10.2 g/dL, leukocytosis, mildly elevated alkaline phosphatase, and normal bilirubin and transaminase levels.

IMAGING:
Liver ultrasound reveals a 6-cm hypoechoic cystic lesion in the posterior right hepatic lobe with no internal septations or calcification.

## Encounter 1: source 2020-07-15, practice 2025-04-14, icu, Pulmonary & Critical Care

Chief complaint: 1-week history of fever, cough, and shortness of breath with hemoptysis

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Chen, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
1-week history of fever, cough, and shortness of breath with hemoptysis

HISTORY OF PRESENT ILLNESS:
Following his January 2020 admission for amebic liver abscess, this 34-year-old with AML s/p stem cell transplant presents with sharp, right-sided chest pain that worsens with deep breaths. He also reports a single episode of bright red blood mixed with sputum earlier today.

PAST MEDICAL HISTORY:
Active Problem List:
- Amebic liver abscess (diagnosed 2020-01-15)
- Substance use disorder (intravenous heroin use)
- Acute myeloid leukemia
- Status post allogeneic hematopoietic stem cell transplantation

PAST SURGICAL HISTORY:
- Percutaneous drainage of amebic liver abscess
- Allogeneic hematopoietic stem cell transplantation (matched unrelated donor)
- Placement of tunneled central venous catheter

MEDICATIONS:
His current medications include tacrolimus and prophylactic trimethoprim-sulfamethoxazole.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus
- Mother: Hypertension
- No known hematologic malignancies in first-degree relatives

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Restaurant kitchen staff.

VITAL SIGNS:
On examination, his temperature is 38.9°C (102°F), heart rate is 110/min, respiratory rate is 22/min, and blood pressure is 125/75 mmHg. Oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
Lung auscultation reveals decreased breath sounds in the right lower lung field without wheezes or crackles.

LABORATORY DATA:
Laboratory studies show:

- White blood cell count: 1,500/µL (Normal: 4,500-11,000/µL)
- Hemoglobin: 9.2 g/dL (Normal: 13.8-17.2 g/dL)
- Platelets: 50,000/µL (Normal: 150,000-450,000/µL)
- Serum creatinine: 1.1 mg/dL (Normal: 0.6-1.2 mg/dL)
- Aspergillus antigen: Positive

IMAGING:
A chest CT scan demonstrates multiple small nodules in the right lower lobe with surrounding ground-glass opacities.

## Encounter 2: source 2021-01-15, practice 2025-10-08, ed, Emergency Medicine

Chief complaint: 2-week history of fever, fatigue, and generalized malaise with exertional dyspnea

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Sarah Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-week history of fever, fatigue, and generalized malaise with exertional dyspnea

HISTORY OF PRESENT ILLNESS:
Patient with AML s/p HSCT returns after amebic liver abscess and aspergillosis. He reports intermittent fevers with night sweats and has noticed some shortness of breath on exertion. He denies any recent travel or sick contacts.

PAST MEDICAL HISTORY:
Active Problem List:
- Amebic liver abscess (diagnosed 2020-01-15)
- Invasive pulmonary aspergillosis (diagnosed 2020-07-15)
- Substance use disorder (intravenous heroin use)
- Acute myeloid leukemia
- Status post allogeneic hematopoietic stem cell transplantation

PAST SURGICAL HISTORY:
- Percutaneous drainage of amebic liver abscess
- Allogeneic hematopoietic stem cell transplantation (matched unrelated donor)
- Placement of tunneled central venous catheter

MEDICATIONS:
Home Medications:
- Tacrolimus 2 mg PO BID (post-transplant immunosuppression)
- Trimethoprim-sulfamethoxazole 160/800 mg PO three times weekly (Pneumocystis prophylaxis)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus
- Mother: Hypertension
- No known hematologic malignancies in first-degree relatives

SOCIAL HISTORY:
History of intravenous drug use

VITAL SIGNS:
His temperature is 38.5°C, heart rate is 110 bpm, respiratory rate is 22 breaths per minute, and blood pressure is 110/70 mmHg

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a new holosystolic murmur best heard at the left lower sternal border. Lung examination is notable for decreased breath sounds at the right base without wheezes or crackles.

LABORATORY DATA:
- White blood cell count: 14,000 /µL
- Hemoglobin: 10.2 g/dL
- Platelet count: 150,000 /µL
- Erythrocyte sedimentation rate: 85 mm/hr
- Blood cultures: Pending

IMAGING:
A chest radiograph shows multiple nodular infiltrates in the right lung field with areas of cavitation.
