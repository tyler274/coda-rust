# synthetic_hospital patient 2085

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 43 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-04-30 (Fri) | outpatient | Gastroenterology |
| 1 | 2020-04-15 | 2021-08-06 (Fri) | outpatient | Hematology & Oncology |
| 2 | 2020-09-15 | 2022-01-06 (Thu) | outpatient | Occupational Medicine |
| 3 | 2021-04-15 | 2022-07-25 (Mon) | outpatient | Dermatology |
| 4 | 2021-11-15 | 2023-03-08 (Wed) | inpatient | Infectious Disease |
| 5 | 2023-03-15 | 2024-06-27 (Thu) | ed | Emergency Medicine |
| 6 | 2023-10-15 | 2025-01-31 (Fri) | ed | Emergency Medicine |
| 7 | 2024-09-15 | 2026-01-10 (Sat) | icu | Neurology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Construction laborer and seasonal agricultural pesticide applicator",
 "alcohol_use": "None",
 "family_history": [
  "Father: Type 2 diabetes mellitus",
  "Mother: Autoimmune hypothyroidism",
  "Sibling: Celiac disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker (quit 5 years ago)",
 "home_medications": [
  {
   "dose": "400 mg oral daily",
   "name": "Imatinib mesylate"
  },
  {
   "dose": "1 tablet oral daily (intermittent adherence due to social circumstances)",
   "name": "Bictegravir/Tenofovir alafenamide/Emtricitabine"
  },
  {
   "dose": "1 double-strength tablet oral daily for PCP prophylaxis",
   "name": "Sulfamethoxazole/Trimethoprim"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Human Immunodeficiency Virus (HIV) infection with history of AIDS-defining illnesses",
  "Chronic myeloid leukemia, BCR-ABL positive",
  "Celiac disease",
  "Peripheral sensory neuropathy secondary to chronic arsenic toxicity"
 ],
 "age_at_first_encounter": 43
}
```

Primary diagnoses: ["Primary central nervous system lymphoma", "Kaposi sarcoma", "Blastomycosis", "Chronic myeloid leukemia, BCR/ABL-positive", "Disseminated Mycobacterium avium complex infection", "Opportunistic pulmonary infection requiring diagnostic sampling", "Arsenic toxicity", "Celiac disease"]
Comorbidities: ["HIV-related opportunistic skin infection", "Anemia"]

## Encounter 0: source 2020-01-15, practice 2021-04-30, outpatient, Gastroenterology

Chief complaint: Diffuse bone pain and progressive muscle weakness with chronic diarrhea and 10-pound weight loss

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Diffuse bone pain and progressive muscle weakness with chronic diarrhea and 10-pound weight loss

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 6-month history of diffuse bone pain and progressive muscle weakness. He reports difficulty climbing stairs and getting up from a seated position. He also mentions experiencing chronic diarrhea, bloating, and a weight loss of 10 pounds over the last year. He denies any history of fractures or trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Human Immunodeficiency Virus (HIV) infection with history of AIDS-defining illnesses
- Chronic myeloid leukemia, BCR-ABL positive
- Celiac disease
- Peripheral sensory neuropathy secondary to chronic arsenic toxicity

MEDICATIONS:
Home Medications:
- Imatinib mesylate 400 mg oral daily
- Bictegravir/Tenofovir alafenamide/Emtricitabine 1 tablet oral daily (intermittent adherence due to social circumstances)
- Sulfamethoxazole/Trimethoprim 1 double-strength tablet oral daily for PCP prophylaxis

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is non-contributory.

SOCIAL HISTORY:
Smoking: Former smoker (quit 5 years ago). Alcohol: None. Occupation: Construction laborer and seasonal agricultural pesticide applicator.

PHYSICAL EXAMINATION:
Physical examination reveals mild pallor and diffuse tenderness over the long bones. Neurological examination shows proximal muscle weakness without sensory deficits.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 12.5 g/dL (Normal: 13.8-17.2 g/dL)
- Serum calcium: 8.9 mg/dL (Normal: 8.5-10.2 mg/dL)
- Alkaline phosphatase: 140 U/L (Normal: 40-130 U/L)
- 25-hydroxyvitamin D: 15 ng/mL (Normal: 30-100 ng/mL)
- Tissue transglutaminase IgA: Pending

IMAGING:
A dual-energy X-ray absorptiometry (DEXA) scan shows a T-score of -2.5 at the lumbar spine. Based on the clinical scenario,

## Encounter 1: source 2020-04-15, practice 2021-08-06, outpatient, Hematology & Oncology

Chief complaint: Fatigue, weight loss, night sweats, and sensation of fullness in the left upper quadrant

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, weight loss, night sweats, and sensation of fullness in the left upper quadrant

HISTORY OF PRESENT ILLNESS:
Patient with HIV returns for hematology follow-up three months after gastroenterology evaluation. He presents with three-month history of fatigue, weight loss, and night sweats. He also reports feeling of fullness in left upper quadrant.

PAST MEDICAL HISTORY:
Active Problem List:
- Celiac disease (diagnosed 2020-01-15)
- Human Immunodeficiency Virus (HIV) infection with history of AIDS-defining illnesses
- Chronic myeloid leukemia, BCR-ABL positive
- Peripheral sensory neuropathy secondary to chronic arsenic toxicity

MEDICATIONS:
Home Medications:
- Imatinib mesylate 400 mg oral daily
- Bictegravir/Tenofovir alafenamide/Emtricitabine 1 tablet oral daily (intermittent adherence due to social circumstances)
- Sulfamethoxazole/Trimethoprim 1 double-strength tablet oral daily for PCP prophylaxis

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus
- Mother: Autoimmune hypothyroidism
- Sibling: Celiac disease

SOCIAL HISTORY:
Smoking: Former smoker (quit 5 years ago). Alcohol: None. Occupation: Construction laborer and seasonal agricultural pesticide applicator.

PHYSICAL EXAMINATION:
On physical examination, he appears pale and has splenomegaly.

LABORATORY DATA:
His complete blood count shows a white blood cell count of 150,000/µL, hemoglobin of 10 g/dL, and platelet count of 450,000/µL. Differential count reveals 6% myeloblasts, 4% promyelocytes, 40% myelocytes, 20% metamyelocytes, 10% bands, and 20% segmented neutrophils.

PATHOLOGY:
A bone marrow biopsy is performed, and cytogenetic analysis demonstrates the presence of the Philadelphia chromosome.

## Encounter 2: source 2020-09-15, practice 2022-01-06, outpatient, Occupational Medicine

Chief complaint: Fatigue, abdominal discomfort, intermittent diarrhea, and progressive burning pain with numbness in feet

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. James Okonkwo, MD
Department: Occupational Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, abdominal discomfort, intermittent diarrhea, and progressive burning pain with numbness in feet

HISTORY OF PRESENT ILLNESS:
The patient with HIV, CML, and celiac disease returns for follow-up. Over the past week, he has developed numbness and tingling in both feet, which is now accompanied by burning pain in his calves.

PAST MEDICAL HISTORY:
Active Problem List:
- Celiac disease (diagnosed 2020-01-15)
- Chronic myeloid leukemia, BCR/ABL-positive (diagnosed 2020-04-15)
- Human Immunodeficiency Virus (HIV) infection with history of AIDS-defining illnesses
- Peripheral sensory neuropathy secondary to chronic arsenic toxicity

MEDICATIONS:
Home Medications:
- Imatinib mesylate 400 mg oral daily
- Bictegravir/Tenofovir alafenamide/Emtricitabine 1 tablet oral daily (intermittent adherence due to social circumstances)
- Sulfamethoxazole/Trimethoprim 1 double-strength tablet oral daily for PCP prophylaxis

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus
- Mother: Autoimmune hypothyroidism
- Sibling: Celiac disease

SOCIAL HISTORY:
He works as a pesticide sprayer for a commercial farm and admits he often skips wearing gloves. He denies alcohol use or recent travel. His diet is unremarkable.

VITAL SIGNS:
Vital signs are normal.

PHYSICAL EXAMINATION:
Physical examination shows hyperpigmented skin with scattered areas of hypopigmentation on the trunk and neck. Neurologic testing reveals symmetric stocking–glove sensory loss and decreased ankle reflexes bilaterally.

LABORATORY DATA:
Laboratory testing shows mild anemia and slightly elevated liver enzymes.

## Encounter 3: source 2021-04-15, practice 2022-07-25, outpatient, Dermatology

Chief complaint: Multiple painless, dark purple skin lesions on lower extremities progressively increasing over 3 months

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. Priya Patel, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Multiple painless, dark purple skin lesions on lower extremities progressively increasing over 3 months

HISTORY OF PRESENT ILLNESS:
This patient with HIV/AIDS, chronic myeloid leukemia, and arsenic toxicity presents with new skin complaints. He reports the appearance of multiple painless, dark purple lesions on his lower extremities that have gradually increased in size and number.

PAST MEDICAL HISTORY:
Active Problem List:
- Celiac disease (diagnosed 2020-01-15)
- Chronic myeloid leukemia, BCR/ABL-positive (diagnosed 2020-04-15)
- Arsenic toxicity (diagnosed 2020-09-15)
- Human Immunodeficiency Virus (HIV) infection with history of AIDS-defining illnesses
- Peripheral sensory neuropathy secondary to chronic arsenic toxicity

MEDICATIONS:
Home Medications:
- Imatinib mesylate 400 mg oral daily
- Bictegravir/Tenofovir alafenamide/Emtricitabine 1 tablet oral daily (intermittent adherence due to social circumstances)
- Sulfamethoxazole/Trimethoprim 1 double-strength tablet oral daily for PCP prophylaxis

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus
- Mother: Autoimmune hypothyroidism
- Sibling: Celiac disease

SOCIAL HISTORY:
Smoking: Former smoker (quit 5 years ago). Alcohol: None. Occupation: Construction laborer and seasonal agricultural pesticide applicator.

VITAL SIGNS:
The patient's temperature is 37.2°C (99°F), blood pressure is 125/78 mmHg, and heart rate is 88 beats per minute.

PHYSICAL EXAMINATION:
On physical examination, you note several non-blanching, violaceous papules and plaques on both legs. Some of the lesions have a nodular appearance.

LABORATORY DATA:
Laboratory findings are as follows:

- CD4+ T-cell count: 120 cells/µL
- HIV viral load: 120,000 copies/mL
- Hemoglobin: 13.2 g/dL
- White blood cell count: 4,800 cells/µL
- Platelet count: 210,000 cells/µL

## Encounter 4: source 2021-11-15, practice 2023-03-08, inpatient, Infectious Disease

Chief complaint: Persistent diarrhea, abdominal pain, and 10-pound weight loss with occasional night sweats

ENCOUNTER NOTE
Date: 2021-11-15
Provider: Dr. David Kim, MD
Department: Infectious Disease
Visit Type: inpatient

CHIEF COMPLAINT:
Persistent diarrhea, abdominal pain, and 10-pound weight loss with occasional night sweats

HISTORY OF PRESENT ILLNESS:
The patient with HIV/AIDS, CML, and celiac disease returns with a 2-month history of persistent diarrhea, abdominal pain, and a 10-pound weight loss. He reports occasional night sweats but denies cough or hematochezia.

PAST MEDICAL HISTORY:
Active Problem List:
- Celiac disease (diagnosed 2020-01-15)
- Chronic myeloid leukemia, BCR/ABL-positive (diagnosed 2020-04-15)
- Arsenic toxicity (diagnosed 2020-09-15)
- Kaposi sarcoma (diagnosed 2021-04-15)
- Human Immunodeficiency Virus (HIV) infection with history of AIDS-defining illnesses
- Peripheral sensory neuropathy secondary to chronic arsenic toxicity

MEDICATIONS:
He takes antiretroviral therapy.

Home Medications:
- Imatinib mesylate 400 mg oral daily
- Bictegravir/Tenofovir alafenamide/Emtricitabine 1 tablet oral daily (intermittent adherence due to social circumstances)
- Sulfamethoxazole/Trimethoprim 1 double-strength tablet oral daily for PCP prophylaxis

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus
- Mother: Autoimmune hypothyroidism
- Sibling: Celiac disease

SOCIAL HISTORY:
His antiretroviral therapy has been sporadic due to social circumstances

VITAL SIGNS:
Afebrile, with a blood pressure of 110/70 mmHg, heart rate of 88 beats per minute, and BMI of 18 kg/m2

PHYSICAL EXAMINATION:
Abdominal examination reveals diffuse tenderness without rebound or guarding. There is palpable, non-tender lymphadenopathy in the cervical and inguinal regions.

LABORATORY DATA:
Hemoglobin: 12.1 g/dL
White blood cell count: 4,800/mm3
CD4 count: 40/mm3
Alkaline phosphatase: 250 U/L (Normal: 40-130 U/L)
HIV viral load: 120,000 copies/mL

OTHER STUDIES:
Blood cultures are drawn and sent to the laboratory.

## Encounter 5: source 2023-03-15, practice 2024-06-27, ed, Emergency Medicine

Chief complaint: Three-week history of cough, low-grade fever, night sweats, and 10-pound weight loss

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. Emily Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Three-week history of cough, low-grade fever, night sweats, and 10-pound weight loss

HISTORY OF PRESENT ILLNESS:
This 46-year-old male with HIV/AIDS and chronic myeloid leukemia has a history of Kaposi sarcoma and disseminated Mycobacterium avium complex infection. He presents with a three-week history of cough, low-grade fever, night sweats, and reports a 10-pound weight loss during this period.

PAST MEDICAL HISTORY:
Active Problem List:
- Celiac disease (diagnosed 2020-01-15)
- Chronic myeloid leukemia, BCR/ABL-positive (diagnosed 2020-04-15)
- Arsenic toxicity (diagnosed 2020-09-15)
- Kaposi sarcoma (diagnosed 2021-04-15)
- Disseminated Mycobacterium avium complex infection (diagnosed 2021-11-15)
- Human Immunodeficiency Virus (HIV) infection with history of AIDS-defining illnesses
- Peripheral sensory neuropathy secondary to chronic arsenic toxicity

MEDICATIONS:
Home Medications:
- Imatinib mesylate 400 mg oral daily
- Bictegravir/Tenofovir alafenamide/Emtricitabine 1 tablet oral daily (intermittent adherence due to social circumstances)
- Sulfamethoxazole/Trimethoprim 1 double-strength tablet oral daily for PCP prophylaxis

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus
- Mother: Autoimmune hypothyroidism
- Sibling: Celiac disease

SOCIAL HISTORY:
The patient works in construction and recently participated in a project near the Mississippi River. He denies any recent travel outside of the United States.

PHYSICAL EXAMINATION:
On physical examination, the patient appears fatigued. Auscultation of the lungs reveals crackles at the right lung base. Skin examination shows a few verrucous skin lesions on his arms.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 12,000 /µL (normal: 4,500-11,000 /µL)
- Hemoglobin: 13.5 g/dL (normal: 13.8-17.2 g/dL)
- Platelets: 250,000 /µL (normal: 150,000-450,000 /µL)

Sputum culture grows broad-based budding yeast.

IMAGING:
Chest X-ray: Right lower lobe infiltrate

## Encounter 6: source 2023-10-15, practice 2025-01-31, ed, Emergency Medicine

Chief complaint: Three-day history of fever, productive cough with yellowish sputum, and pleuritic chest pain

ENCOUNTER NOTE
Date: 2023-10-15
Provider: Dr. Robert Chang, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Three-day history of fever, productive cough with yellowish sputum, and pleuritic chest pain

HISTORY OF PRESENT ILLNESS:
The patient is a 46-year-old male with HIV infection with history of AIDS-defining illnesses, chronic myeloid leukemia on imatinib therapy, and recent pulmonary blastomycosis, who presents with a three-day history of fever, productive cough with yellowish sputum, and pleuritic chest pain. He reports no recent travel or sick contacts.

PAST MEDICAL HISTORY:
Active Problem List:
- Celiac disease (diagnosed 2020-01-15)
- Chronic myeloid leukemia, BCR/ABL-positive (diagnosed 2020-04-15)
- Arsenic toxicity (diagnosed 2020-09-15)
- Kaposi sarcoma (diagnosed 2021-04-15)
- Disseminated Mycobacterium avium complex infection (diagnosed 2021-11-15)
- Blastomycosis (diagnosed 2023-03-15)
- Human Immunodeficiency Virus (HIV) infection with history of AIDS-defining illnesses
- Peripheral sensory neuropathy secondary to chronic arsenic toxicity

MEDICATIONS:
Antiretroviral therapy (ART)

Home Medications:
- Imatinib mesylate 400 mg oral daily
- Bictegravir/Tenofovir alafenamide/Emtricitabine 1 tablet oral daily (intermittent adherence due to social circumstances)
- Sulfamethoxazole/Trimethoprim 1 double-strength tablet oral daily for PCP prophylaxis

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus
- Mother: Autoimmune hypothyroidism
- Sibling: Celiac disease

SOCIAL HISTORY:
Smoking: Former smoker (quit 5 years ago). Alcohol: None. Occupation: Construction laborer and seasonal agricultural pesticide applicator.

VITAL SIGNS:
His temperature is 38.9°C (102°F), blood pressure is 120/75 mmHg, pulse is 110 beats per minute, and respiratory rate is 22 breaths per minute.

PHYSICAL EXAMINATION:
Physical examination reveals dullness to percussion and increased tactile fremitus over the right lower lung field. Breath sounds are diminished in this area with egophony.

LABORATORY DATA:
His CD4 count is 350 cells/mm³ and his HIV viral load is undetectable.

Laboratory Values:
- White blood cell count: 15,000 cells/mm³ (Normal: 4,500-11,000 cells/mm³)
- Neutrophils: 80% (Normal: 40-60%)
- Lymphocytes: 15% (Normal: 20-40%)
- Hemoglobin: 13.5 g/dL (Normal: 13.8-17.2 g/dL for males)
- Platelets: 250,000/mm³ (Normal: 150,000-450,000/mm³)

IMAGING:
Chest X-ray shows a lobar consolidation in the right lower lobe.

## Encounter 7: source 2024-09-15, practice 2026-01-10, icu, Neurology

Chief complaint: Night sweats, fever, 10-pound weight loss, fatigue, and swelling in neck and groin with acute neurological changes

ENCOUNTER NOTE
Date: 2024-09-15
Provider: Dr. Lisa Thompson, MD
Department: Neurology
Visit Type: icu

CHIEF COMPLAINT:
Night sweats, fever, 10-pound weight loss, fatigue, and swelling in neck and groin with acute neurological changes

HISTORY OF PRESENT ILLNESS:
This 47-year-old male with HIV infection, prior AIDS-defining illnesses, and chronic myeloid leukemia presents with a 2-month history of night sweats, fever, and a 10-pound weight loss. He reports feeling fatigued and has noticed swelling in his neck and groin. His CD4 count six months ago was 250 cells/mm³.

PAST MEDICAL HISTORY:
Active Problem List:
- Celiac disease (diagnosed 2020-01-15)
- Chronic myeloid leukemia, BCR/ABL-positive (diagnosed 2020-04-15)
- Arsenic toxicity (diagnosed 2020-09-15)
- Kaposi sarcoma (diagnosed 2021-04-15)
- Disseminated Mycobacterium avium complex infection (diagnosed 2021-11-15)
- Blastomycosis (diagnosed 2023-03-15)
- Opportunistic pulmonary infection requiring diagnostic sampling (diagnosed 2023-10-15)
- Human Immunodeficiency Virus (HIV) infection with history of AIDS-defining illnesses
- Peripheral sensory neuropathy secondary to chronic arsenic toxicity

MEDICATIONS:
Home Medications:
- Imatinib mesylate 400 mg oral daily
- Bictegravir/Tenofovir alafenamide/Emtricitabine 1 tablet oral daily (intermittent adherence due to social circumstances)
- Sulfamethoxazole/Trimethoprim 1 double-strength tablet oral daily for PCP prophylaxis

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus
- Mother: Autoimmune hypothyroidism
- Sibling: Celiac disease

SOCIAL HISTORY:
Smoking: Former smoker (quit 5 years ago). Alcohol: None. Occupation: Construction laborer and seasonal agricultural pesticide applicator.

PHYSICAL EXAMINATION:
Diffuse, nontender lymphadenopathy in the cervical and inguinal regions

LABORATORY DATA:
Hemoglobin: 10.2 g/dL (normal: 13.8-17.2 g/dL)
- White blood cell count: 8,000 cells/mm³ (normal: 4,500-11,000 cells/mm³)
- Platelets: 150,000/mm³ (normal: 150,000-450,000/mm³)
- Lactate dehydrogenase (LDH): 350 U/L (normal: 140-280 U/L)
- HIV viral load: Undetectable
- CD4 count: 200 cells/mm³

PATHOLOGY:
An excisional biopsy of a cervical lymph node is performed. Histopathology shows large B cells with a high mitotic index, consistent with diffuse large B-cell lymphoma (DLBCL).
