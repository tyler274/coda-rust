# synthetic_hospital patient 2749

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 58 at first encounter, sex F, Non-Hispanic White, Private insurance through spouse's employer.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-10-20 (Tue) | outpatient | Rheumatology |
| 1 | 2021-01-15 | 2021-10-18 (Mon) | outpatient | Orthopedics |
| 2 | 2022-07-15 | 2023-04-17 (Mon) | outpatient | Hematology & Oncology |
| 3 | 2024-07-15 | 2025-04-15 (Tue) | outpatient | Hematology & Oncology |
| 4 | 2025-07-15 | 2026-04-15 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Sulfa drugs (rash)",
  "Latex (contact dermatitis)"
 ],
 "insurance": "Private insurance through spouse's employer",
 "occupation": "Retired library administrator",
 "alcohol_use": "None (patient abstains due to methotrexate therapy)",
 "family_history": [
  "Mother: Rheumatoid arthritis",
  "Father: Type 2 diabetes mellitus",
  "Paternal uncle: Chronic lymphocytic leukemia"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 8 pack-years, quit 10 years ago",
 "home_medications": [
  {
   "dose": "20 mg orally once weekly",
   "name": "Methotrexate"
  },
  {
   "dose": "1 mg daily",
   "name": "Folic acid"
  },
  {
   "dose": "5 mg daily",
   "name": "Prednisone"
  },
  {
   "dose": "400 mg every 6 hours as needed for joint pain",
   "name": "Ibuprofen"
  },
  {
   "dose": "600 mg twice daily",
   "name": "Calcium carbonate"
  },
  {
   "dose": "2000 IU daily",
   "name": "Vitamin D3"
  },
  {
   "dose": "10 mg daily",
   "name": "Lisinopril"
  }
 ],
 "surgical_history": [
  "Bilateral carpal tunnel release (2019)",
  "Diagnostic arthroscopy left knee (2020)"
 ],
 "chronic_conditions": [
  "Rheumatoid arthritis",
  "Osteoporosis",
  "Hypertension"
 ],
 "age_at_first_encounter": 58
}
```

Primary diagnoses: ["Chronic myeloid leukemia, BCR/ABL-positive", "Popliteal cyst (Baker's cyst)", "Diffuse alveolar hemorrhage", "Chronic anemia"]
Comorbidities: ["Rheumatoid arthritis"]

## Encounter 0: source 2020-01-15, practice 2020-10-20, outpatient, Rheumatology

Chief complaint: Fatigue and weakness for the past two months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Nair, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue and weakness for the past two months

HISTORY OF PRESENT ILLNESS:
She presents to the clinic complaining of fatigue and weakness for the past two months. She reports that her joint pain has been well-controlled on methotrexate.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatoid arthritis
- Osteoporosis
- Hypertension

PAST SURGICAL HISTORY:
- Bilateral carpal tunnel release (2019)
- Diagnostic arthroscopy left knee (2020)

MEDICATIONS:
She takes methotrexate.

Home Medications:
- Folic acid 1 mg daily
- Prednisone 5 mg daily
- Ibuprofen 400 mg every 6 hours as needed for joint pain
- Calcium carbonate 600 mg twice daily
- Vitamin D3 2000 IU daily
- Lisinopril 10 mg daily

ALLERGIES:
Sulfa drugs (rash), Latex (contact dermatitis)

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Father: Type 2 diabetes mellitus
- Paternal uncle: Chronic lymphocytic leukemia

SOCIAL HISTORY:
Smoking: Former smoker, 8 pack-years, quit 10 years ago. Alcohol: None (patient abstains due to methotrexate therapy). Occupation: Retired library administrator.

PHYSICAL EXAMINATION:
On physical examination, she appears pale but is otherwise unremarkable. Her spleen and liver are not palpable, and there is no lymphadenopathy.

LABORATORY DATA:
Hemoglobin: 10.2 g/dL (Normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 84 fL (Normal: 80-100 fL)
- Serum iron: 30 µg/dL (Normal: 50-170 µg/dL)
- Total iron-binding capacity (TIBC): 200 µg/dL (Normal: 250-450 µg/dL)
- Iron saturation: 15% (Normal: 20-50%)
- Serum ferritin: 150 ng/mL (Normal: 15-200 ng/mL)

## Encounter 1: source 2021-01-15, practice 2021-10-18, outpatient, Orthopedics

Chief complaint: Painless swelling in the back of her right knee for three weeks

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James O'Brien, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Painless swelling in the back of her right knee for three weeks

HISTORY OF PRESENT ILLNESS:
This 59-year-old with rheumatoid arthritis presents with right knee swelling following prior rheumatology encounter. She denies recent trauma or injury to the area. She reports mild discomfort when walking long distances but no significant pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic anemia (diagnosed 2020-01-15)
- Rheumatoid arthritis
- Osteoporosis
- Hypertension

PAST SURGICAL HISTORY:
- Bilateral carpal tunnel release (2019)
- Diagnostic arthroscopy left knee (2020)

MEDICATIONS:
Home Medications:
- Methotrexate 20 mg orally once weekly
- Folic acid 1 mg daily
- Prednisone 5 mg daily
- Ibuprofen 400 mg every 6 hours as needed for joint pain
- Calcium carbonate 600 mg twice daily
- Vitamin D3 2000 IU daily
- Lisinopril 10 mg daily

ALLERGIES:
Sulfa drugs (rash), Latex (contact dermatitis)

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Father: Type 2 diabetes mellitus
- Paternal uncle: Chronic lymphocytic leukemia

SOCIAL HISTORY:
Smoking: Former smoker, 8 pack-years, quit 10 years ago. Alcohol: None (patient abstains due to methotrexate therapy). Occupation: Retired library administrator.

PHYSICAL EXAMINATION:
On physical examination, there is a 4 cm fluctuant, non-tender mass in the popliteal fossa. The mass does not transilluminate. Knee range of motion is normal, but the patient experiences slight discomfort at the extremes of flexion. There is no erythema or warmth over the mass. The remainder of the musculoskeletal examination is unremarkable.

IMAGING:
Doppler ultrasound of the right lower extremity veins shows no evidence of deep vein thrombosis but reveals a well-defined cystic structure in the popliteal fossa communicating with the knee joint.

## Encounter 2: source 2022-07-15, practice 2023-04-17, outpatient, Hematology & Oncology

Chief complaint: Progressive fatigue, splenomegaly, and unexplained bruising over the past 3 months

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Aisha Patel, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue, splenomegaly, and unexplained bruising over the past 3 months

HISTORY OF PRESENT ILLNESS:
She has rheumatoid arthritis, osteoporosis, and hypertension, with prior evaluations for chronic anemia and popliteal cyst. She presents with progressive fatigue, splenomegaly, and unexplained bruising over the past 3 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic anemia (diagnosed 2020-01-15)
- Popliteal cyst (Baker's cyst) (diagnosed 2021-01-15)
- Rheumatoid arthritis
- Osteoporosis
- Hypertension

The patient's past medical history is notable for rheumatoid arthritis treated with methotrexate.

PAST SURGICAL HISTORY:
- Bilateral carpal tunnel release (2019)
- Diagnostic arthroscopy left knee (2020)

MEDICATIONS:
Home Medications:
- Methotrexate 20 mg orally once weekly
- Folic acid 1 mg daily
- Prednisone 5 mg daily
- Ibuprofen 400 mg every 6 hours as needed for joint pain
- Calcium carbonate 600 mg twice daily
- Vitamin D3 2000 IU daily
- Lisinopril 10 mg daily

ALLERGIES:
Sulfa drugs (rash), Latex (contact dermatitis)

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Father: Type 2 diabetes mellitus
- Paternal uncle: Chronic lymphocytic leukemia

SOCIAL HISTORY:
Smoking: Former smoker, 8 pack-years, quit 10 years ago. Alcohol: None (patient abstains due to methotrexate therapy). Occupation: Retired library administrator.

LABORATORY DATA:
Peripheral blood smear reveals a marked increase in mature and immature myeloid cells.

PATHOLOGY:
A bone marrow aspirate demonstrates hypercellularity with myeloid predominance.

ASSESSMENT AND PLAN:
The hematopathology team suspects a neoplastic process and wishes to determine if the abnormal cell population is derived from a single precursor cell.

They consider several molecular markers to assess monoclonality versus polyclonality. The laboratory is equipped to analyze various protein and RNA markers, including cytokine receptor subunits and immunoglobulin gene rearrangements.

## Encounter 3: source 2024-07-15, practice 2025-04-15, outpatient, Hematology & Oncology

Chief complaint: Fatigue and shortness of breath on exertion for the past two months

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Aisha Patel, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue and shortness of breath on exertion for the past two months

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic anemia (diagnosed 2020-01-15)
- Popliteal cyst (Baker's cyst) (diagnosed 2021-01-15)
- Chronic myeloid leukemia, BCR/ABL-positive (diagnosed 2022-07-15)
- Rheumatoid arthritis
- Osteoporosis
- Hypertension

PAST SURGICAL HISTORY:
- Bilateral carpal tunnel release (2019)
- Diagnostic arthroscopy left knee (2020)

MEDICATIONS:
She has been on methotrexate and low-dose prednisone for the past year.

Home Medications:
- Folic acid 1 mg daily
- Ibuprofen 400 mg every 6 hours as needed for joint pain
- Calcium carbonate 600 mg twice daily
- Vitamin D3 2000 IU daily
- Lisinopril 10 mg daily

ALLERGIES:
Sulfa drugs (rash), Latex (contact dermatitis)

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Father: Type 2 diabetes mellitus
- Paternal uncle: Chronic lymphocytic leukemia

SOCIAL HISTORY:
Smoking: Former smoker, 8 pack-years, quit 10 years ago. Alcohol: None (patient abstains due to methotrexate therapy). Occupation: Retired library administrator.

PHYSICAL EXAMINATION:
On physical examination, she appears pale. Her joints show no signs of active inflammation.

LABORATORY DATA:
- Hemoglobin: 10.2 g/dL (Normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 82 fL (Normal: 80-100 fL)
- Serum iron: 30 µg/dL (Normal: 50-170 µg/dL)
- Total iron-binding capacity (TIBC): 250 µg/dL (Normal: 250-450 µg/dL)
- Ferritin: 200 ng/mL (Normal: 15-200 ng/mL)
- Reticulocyte count: 1% (Normal: 0.5-2.5%)

## Encounter 4: source 2025-07-15, practice 2026-04-15, ed, Emergency Medicine

Chief complaint: Progressive dyspnea, nonproductive cough, and low-grade fever for 2 weeks

ENCOUNTER NOTE
Date: 2025-07-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive dyspnea, nonproductive cough, and low-grade fever for 2 weeks

HISTORY OF PRESENT ILLNESS:
The 63-year-old with rheumatoid arthritis and prior chronic myeloid leukemia presents to the emergency department with a 2-week history of progressive dyspnea, nonproductive cough, and low-grade fever. She denies any recent travel or sick contacts.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic anemia (diagnosed 2020-01-15)
- Popliteal cyst (Baker's cyst) (diagnosed 2021-01-15)
- Chronic myeloid leukemia, BCR/ABL-positive (diagnosed 2022-07-15)
- Rheumatoid arthritis
- Osteoporosis
- Hypertension

PAST SURGICAL HISTORY:
- Bilateral carpal tunnel release (2019)
- Diagnostic arthroscopy left knee (2020)

MEDICATIONS:
Her medications include methotrexate, which was started 3 months ago, folic acid, and ibuprofen as needed.

Home Medications:
- Prednisone 5 mg daily
- Calcium carbonate 600 mg twice daily
- Vitamin D3 2000 IU daily
- Lisinopril 10 mg daily

ALLERGIES:
Sulfa drugs (rash), Latex (contact dermatitis)

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Father: Type 2 diabetes mellitus
- Paternal uncle: Chronic lymphocytic leukemia

SOCIAL HISTORY:
Smoking: Former smoker, 8 pack-years, quit 10 years ago. Alcohol: None (patient abstains due to methotrexate therapy). Occupation: Retired library administrator.

VITAL SIGNS:
Respiratory rate of 22 breaths per minute, oxygen saturation is 92% on room air

PHYSICAL EXAMINATION:
On examination, she is tachypneic and auscultation reveals bilateral crackles at the lung bases

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 9,000/uL
- Hemoglobin: 13.2 g/dL
- Platelet count: 250,000/uL
- C-reactive protein: 25 mg/L (normal <10 mg/L)
- Erythrocyte sedimentation rate: 40 mm/hr (normal 0-20 mm/hr)
- Lactate dehydrogenase: 220 U/L (normal 100-190 U/L)

IMAGING:
Chest radiography shows bilateral interstitial infiltrates. High-resolution CT of the chest demonstrates diffuse ground-glass opacities without evidence of pulmonary embolism or lymphadenopathy.
