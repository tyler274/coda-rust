# synthetic_hospital patient 2856

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 62 at first encounter, sex F, Non-Hispanic White, Medicare (ESRD eligibility).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-04-10 (Mon) | follow_up | Orthopedic Surgery |
| 1 | 2020-04-15 | 2023-07-10 (Mon) | outpatient | Internal Medicine |
| 2 | 2020-07-15 | 2023-10-16 (Mon) | outpatient | General Surgery |
| 3 | 2021-01-15 | 2024-04-15 (Mon) | outpatient | Endocrinology |
| 4 | 2021-07-15 | 2024-10-14 (Mon) | outpatient | Hematology Oncology |
| 5 | 2022-01-15 | 2025-04-15 (Tue) | ed | Emergency Medicine |
| 6 | 2022-07-15 | 2025-10-05 (Sun) | ed | Emergency Medicine |
| 7 | 2023-01-15 | 2026-04-10 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare (ESRD eligibility)",
 "occupation": "Retired administrative assistant",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Breast cancer, diagnosed age 58",
  "Father: Myocardial infarction, diagnosed age 67",
  "Sister: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "sliding scale subcutaneous with meals",
   "name": "Insulin lispro"
  },
  {
   "dose": "800 mg PO three times daily with meals",
   "name": "Sevelamer carbonate"
  },
  {
   "dose": "0.25 mcg PO daily",
   "name": "Calcitriol"
  },
  {
   "dose": "30 mg PO daily",
   "name": "Pioglitazone"
  },
  {
   "dose": "1 mg PO daily",
   "name": "Anastrozole"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "1000 mcg PO daily",
   "name": "Cyanocobalamin"
  },
  {
   "dose": "650 mg PO every 6 hours as needed for pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Left total knee arthroplasty (3 months prior)",
  "Right breast lumpectomy with sentinel lymph node biopsy and radiation therapy (10 years prior)",
  "Tunneled central venous catheter placement for chemotherapy and bisphosphonate infusions"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus with polyneuropathy and diabetic nephropathy",
  "Metastatic breast cancer (invasive ductal carcinoma) with osseous metastases",
  "Chronic kidney disease stage 4 with secondary hyperparathyroidism",
  "Hyperlipidemia",
  "Osteoarthritis, left knee, status post total knee arthroplasty",
  "Umbilical hernia",
  "Vitamin B12 deficiency"
 ],
 "age_at_first_encounter": 62
}
```

Primary diagnoses: ["Malignancy-associated hypercalcemia", "Umbilical hernia, incidental finding", "Vitamin B12 deficiency", "Multiple myeloma or metastatic bone disease requiring intravenous bisphosphonates", "Progressive metastatic breast cancer with active systemic disease", "Status post left total knee arthroplasty", "Left Lateral Wall Myocardial Infarction"]
Comorbidities: ["Type 2 diabetes mellitus", "Malignancy-associated hypercalcemia"]

## Encounter 0: source 2020-01-15, practice 2023-04-10, follow_up, Orthopedic Surgery

Chief complaint: Follow-up visit 3 months after left total knee arthroplasty with questions about implant materials

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. James Morrison, MD
Department: Orthopedic Surgery
Visit Type: follow_up

CHIEF COMPLAINT:
Follow-up visit 3 months after left total knee arthroplasty with questions about implant materials

HISTORY OF PRESENT ILLNESS:
She reports steady improvement in mobility and is currently walking with a cane. Physical therapy has been ongoing and well tolerated. She asks the physician about a recent study she read in a medical newsletter comparing a new implant material (zirconium-coated titanium) to traditional cobalt-chromium alloy for knee prosthetics. The study showed that patients with the newer material had a 5-degree improvement in range of motion compared to standard implants (p = 0.03), but the rates of pain, long-term function, and prosthetic lifespan were identical. The patient asks, "Should I regret getting the standard implant instead of the new one?"

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus with polyneuropathy and diabetic nephropathy
- Metastatic breast cancer (invasive ductal carcinoma) with osseous metastases
- Chronic kidney disease stage 4 with secondary hyperparathyroidism
- Hyperlipidemia
- Osteoarthritis, left knee, status post total knee arthroplasty
- Umbilical hernia
- Vitamin B12 deficiency

PAST SURGICAL HISTORY:
Left total knee arthroplasty 6 weeks ago

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Insulin glargine 20 units subcutaneous at bedtime
- Insulin lispro sliding scale subcutaneous with meals
- Sevelamer carbonate 800 mg PO three times daily with meals
- Calcitriol 0.25 mcg PO daily
- Pioglitazone 30 mg PO daily
- Anastrozole 1 mg PO daily
- Atorvastatin 40 mg PO daily
- Cyanocobalamin 1000 mcg PO daily
- Acetaminophen 650 mg PO every 6 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Breast cancer, diagnosed age 58
- Father: Myocardial infarction, diagnosed age 67
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired administrative assistant.

## Encounter 1: source 2020-04-15, practice 2023-07-10, outpatient, Internal Medicine

Chief complaint: Progressive numbness and tingling in hands and feet with difficulty maintaining balance over 6 months

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive numbness and tingling in hands and feet with difficulty maintaining balance over 6 months

HISTORY OF PRESENT ILLNESS:
The patient is a 62-year-old female with a history of Type 2 diabetes mellitus with polyneuropathy and recent left total knee arthroplasty follow-up who presents with progressive sensory symptoms. She also reports difficulty with balance, especially in the dark.

PAST MEDICAL HISTORY:
Active Problem List:
- Status post left total knee arthroplasty (diagnosed 2020-01-15)
- Type 2 diabetes mellitus with polyneuropathy and diabetic nephropathy
- Metastatic breast cancer (invasive ductal carcinoma) with osseous metastases
- Chronic kidney disease stage 4 with secondary hyperparathyroidism
- Hyperlipidemia
- Osteoarthritis, left knee, status post total knee arthroplasty
- Umbilical hernia
- Vitamin B12 deficiency

PAST SURGICAL HISTORY:
- Left total knee arthroplasty (3 months prior)
- Right breast lumpectomy with sentinel lymph node biopsy and radiation therapy (10 years prior)
- Tunneled central venous catheter placement for chemotherapy and bisphosphonate infusions

MEDICATIONS:
Her diabetes has been managed with metformin 1000 mg twice daily.

Home Medications:
- Insulin glargine 20 units subcutaneous at bedtime
- Insulin lispro sliding scale subcutaneous with meals
- Sevelamer carbonate 800 mg PO three times daily with meals
- Calcitriol 0.25 mcg PO daily
- Pioglitazone 30 mg PO daily
- Anastrozole 1 mg PO daily
- Atorvastatin 40 mg PO daily
- Cyanocobalamin 1000 mcg PO daily
- Acetaminophen 650 mg PO every 6 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Breast cancer, diagnosed age 58
- Father: Myocardial infarction, diagnosed age 67
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
Her diet is poor in meat and dairy products.

REVIEW OF SYSTEMS:
She denies any recent infections, fever, or gastrointestinal symptoms.

PHYSICAL EXAMINATION:
Physical examination reveals decreased vibratory sensation in her lower extremities and a positive Romberg sign. Deep tendon reflexes are normal.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 13.2 g/dL (normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 94 fL (normal: 80-100 fL)
- Serum glucose: 142 mg/dL (normal: 70-110 mg/dL)
- Serum creatinine: 0.9 mg/dL (normal: 0.6-1.2 mg/dL)
- Serum calcium: 9.2 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum vitamin B12: Pending

## Encounter 2: source 2020-07-15, practice 2023-10-16, outpatient, General Surgery

Chief complaint: Painless bulge near navel that enlarges with coughing and reduces when supine

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Robert Chen, MD
Department: General Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Painless bulge near navel that enlarges with coughing and reduces when supine

HISTORY OF PRESENT ILLNESS:
The patient presents for evaluation of an abdominal wall bulge with recent evaluation for vitamin B12 deficiency and prior left total knee arthroplasty. She reports a swelling near her umbilicus that she noticed about six months ago. She states that it becomes more prominent when she coughs or strains, and it typically disappears when she lies flat. The swelling does not cause pain but has gradually increased in size. She denies nausea, vomiting, constipation, changes in appetite, or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Status post left total knee arthroplasty (diagnosed 2020-01-15)
- Vitamin B12 deficiency (diagnosed 2020-04-15)
- Type 2 diabetes mellitus with polyneuropathy and diabetic nephropathy
- Metastatic breast cancer (invasive ductal carcinoma) with osseous metastases
- Chronic kidney disease stage 4 with secondary hyperparathyroidism
- Hyperlipidemia
- Osteoarthritis, left knee, status post total knee arthroplasty
- Umbilical hernia

Her medical history includes well-controlled type 2 diabetes.

PAST SURGICAL HISTORY:
A prior appendectomy many years ago

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Insulin glargine 20 units subcutaneous at bedtime
- Insulin lispro sliding scale subcutaneous with meals
- Sevelamer carbonate 800 mg PO three times daily with meals
- Calcitriol 0.25 mcg PO daily
- Pioglitazone 30 mg PO daily
- Anastrozole 1 mg PO daily
- Atorvastatin 40 mg PO daily
- Cyanocobalamin 1000 mcg PO daily
- Acetaminophen 650 mg PO every 6 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Breast cancer, diagnosed age 58
- Father: Myocardial infarction, diagnosed age 67
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, there is a small, soft, non-tender midline bulge located just above the umbilicus. The mass is reducible and covered with normal-appearing skin. No other abdominal abnormalities are noted.

## Encounter 3: source 2021-01-15, practice 2024-04-15, outpatient, Endocrinology

Chief complaint: Routine diabetes follow-up with recent hemoglobin A1c of 8.1% despite current regimen

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Sarah Williams, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine diabetes follow-up with recent hemoglobin A1c of 8.1% despite current regimen

HISTORY OF PRESENT ILLNESS:
The patient returns for diabetes follow-up with long-standing disease and polyneuropathy. She was diagnosed 8 years ago and has been managed with metformin and lifestyle modifications. Despite adherence to her current regimen, her recent hemoglobin A1c was 8.1%.

PAST MEDICAL HISTORY:
Active Problem List:
- Status post left total knee arthroplasty (diagnosed 2020-01-15)
- Vitamin B12 deficiency (diagnosed 2020-04-15)
- Umbilical hernia, incidental finding (diagnosed 2020-07-15)
- Type 2 diabetes mellitus with polyneuropathy and diabetic nephropathy
- Metastatic breast cancer (invasive ductal carcinoma) with osseous metastases
- Chronic kidney disease stage 4 with secondary hyperparathyroidism
- Hyperlipidemia
- Osteoarthritis, left knee, status post total knee arthroplasty
- Umbilical hernia

PAST SURGICAL HISTORY:
- Left total knee arthroplasty (3 months prior)
- Right breast lumpectomy with sentinel lymph node biopsy and radiation therapy (10 years prior)
- Tunneled central venous catheter placement for chemotherapy and bisphosphonate infusions

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Insulin glargine 20 units subcutaneous at bedtime
- Insulin lispro sliding scale subcutaneous with meals
- Sevelamer carbonate 800 mg PO three times daily with meals
- Calcitriol 0.25 mcg PO daily
- Pioglitazone 30 mg PO daily
- Anastrozole 1 mg PO daily
- Atorvastatin 40 mg PO daily
- Cyanocobalamin 1000 mcg PO daily
- Acetaminophen 650 mg PO every 6 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Breast cancer, diagnosed age 58
- Father: Myocardial infarction, diagnosed age 67
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired administrative assistant.

LABORATORY DATA:
Her liver function tests are normal at baseline

PLAN:
Her physician adds pioglitazone to improve insulin sensitivity and better control her blood glucose levels. She is counseled about potential side effects, including weight gain, fluid retention, and the rare risk of hepatotoxicity,

## Encounter 4: source 2021-07-15, practice 2024-10-14, outpatient, Hematology Oncology

Chief complaint: Persistent right arm pain, fatigue, and unintentional weight loss over 3 months

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Maria Rodriguez, MD
Department: Hematology Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent right arm pain, fatigue, and unintentional weight loss over 3 months

HISTORY OF PRESENT ILLNESS:
The patient with metastatic breast cancer and recent endocrinology follow-up reports unintentional weight loss and feeling "worn down" over the past 3 months. She denies recent trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Status post left total knee arthroplasty (diagnosed 2020-01-15)
- Vitamin B12 deficiency (diagnosed 2020-04-15)
- Umbilical hernia, incidental finding (diagnosed 2020-07-15)
- Type 2 diabetes mellitus with polyneuropathy and diabetic nephropathy
- Metastatic breast cancer (invasive ductal carcinoma) with osseous metastases
- Chronic kidney disease stage 4 with secondary hyperparathyroidism
- Hyperlipidemia
- Osteoarthritis, left knee, status post total knee arthroplasty
- Umbilical hernia

Her past medical history includes radiation treatment for breast cancer 10 years ago.

PAST SURGICAL HISTORY:
- Left total knee arthroplasty (3 months prior)
- Right breast lumpectomy with sentinel lymph node biopsy and radiation therapy (10 years prior)
- Tunneled central venous catheter placement for chemotherapy and bisphosphonate infusions

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Insulin glargine 20 units subcutaneous at bedtime
- Insulin lispro sliding scale subcutaneous with meals
- Sevelamer carbonate 800 mg PO three times daily with meals
- Calcitriol 0.25 mcg PO daily
- Pioglitazone 30 mg PO daily
- Anastrozole 1 mg PO daily
- Atorvastatin 40 mg PO daily
- Cyanocobalamin 1000 mcg PO daily
- Acetaminophen 650 mg PO every 6 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Breast cancer, diagnosed age 58
- Father: Myocardial infarction, diagnosed age 67
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired administrative assistant.

VITAL SIGNS:
Temperature is 37.8°C (100°F).

PHYSICAL EXAMINATION:
Physical examination reveals point tenderness over the right humerus.

LABORATORY DATA:
Laboratory studies reveal normocytic anemia and an elevated calcium level.

IMAGING:
An x-ray shows a lytic lesion of the mid-shaft of the humerus.

## Encounter 5: source 2022-01-15, practice 2025-04-15, ed, Emergency Medicine

Chief complaint: Painful, purplish, net-like skin lesions on lower extremities with severe pain worsened by ambulation

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Painful, purplish, net-like skin lesions on lower extremities with severe pain worsened by ambulation

HISTORY OF PRESENT ILLNESS:
This 64-year-old with metastatic breast cancer presents with lesions that have progressed over the past two weeks. She reports the pain is severe and worsens with ambulation.

PAST MEDICAL HISTORY:
Active Problem List:
- Status post left total knee arthroplasty (diagnosed 2020-01-15)
- Vitamin B12 deficiency (diagnosed 2020-04-15)
- Umbilical hernia, incidental finding (diagnosed 2020-07-15)
- Multiple myeloma or metastatic bone disease requiring intravenous bisphosphonates (diagnosed 2021-07-15)
- Type 2 diabetes mellitus with polyneuropathy and diabetic nephropathy
- Metastatic breast cancer (invasive ductal carcinoma) with osseous metastases
- Chronic kidney disease stage 4 with secondary hyperparathyroidism
- Hyperlipidemia
- Osteoarthritis, left knee, status post total knee arthroplasty
- Umbilical hernia

PAST SURGICAL HISTORY:
- Left total knee arthroplasty (3 months prior)
- Right breast lumpectomy with sentinel lymph node biopsy and radiation therapy (10 years prior)
- Tunneled central venous catheter placement for chemotherapy and bisphosphonate infusions

MEDICATIONS:
Her medications include sevelamer, insulin, and calcitriol.

Home Medications:
- Metformin 1000 mg PO twice daily
- Insulin glargine 20 units subcutaneous at bedtime
- Insulin lispro sliding scale subcutaneous with meals
- Sevelamer carbonate 800 mg PO three times daily with meals
- Pioglitazone 30 mg PO daily
- Anastrozole 1 mg PO daily
- Atorvastatin 40 mg PO daily
- Cyanocobalamin 1000 mcg PO daily
- Acetaminophen 650 mg PO every 6 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Breast cancer, diagnosed age 58
- Father: Myocardial infarction, diagnosed age 67
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, you note retiform purpura and indurated plaques with central necrosis on both calves.

LABORATORY DATA:
Laboratory values are as follows:

- Serum calcium: 9.2 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum phosphorus: 6.8 mg/dL (normal: 2.5-4.5 mg/dL)
- Parathyroid hormone (PTH): 600 pg/mL (normal: 15-65 pg/mL)
- Blood urea nitrogen (BUN): 75 mg/dL (normal: 7-20 mg/dL)
- Creatinine: 9.1 mg/dL (normal: 0.6-1.2 mg/dL)

## Encounter 6: source 2022-07-15, practice 2025-10-05, ed, Emergency Medicine

Chief complaint: Acute confusion, poor appetite, worsening constipation, and lethargy over the past week

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Lisa Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute confusion, poor appetite, worsening constipation, and lethargy over the past week

HISTORY OF PRESENT ILLNESS:
The patient is a 64-year-old female with metastatic invasive ductal carcinoma with osseous metastases, chronic kidney disease stage 4, and diabetes mellitus who presents to the emergency department, returning six months after her prior admission for malignancy-associated hypercalcemia. Her family reports that she has been increasingly lethargic over the past week and has had difficulty maintaining her balance. She is admitted and receives intravenous fluids and pamidronate, which improve her mental status over the next 24 hours. On hospital day 2, she is alert and oriented, though fatigued. She expresses fear about "what happens next" and says, "I know this means things are getting worse." Her daughter, who is at the bedside, asks, "Is it time to stop the chemo?" The oncology team confirms that her disease has progressed despite multiple lines of systemic therapy. She is still able to ambulate short distances but has lost over 15 pounds in the last 3 months and has required multiple recent admissions.

PAST MEDICAL HISTORY:
Active Problem List:
- Status post left total knee arthroplasty (diagnosed 2020-01-15)
- Vitamin B12 deficiency (diagnosed 2020-04-15)
- Umbilical hernia, incidental finding (diagnosed 2020-07-15)
- Multiple myeloma or metastatic bone disease requiring intravenous bisphosphonates (diagnosed 2021-07-15)
- Malignancy-associated hypercalcemia (diagnosed 2022-01-15)
- Type 2 diabetes mellitus with polyneuropathy and diabetic nephropathy
- Metastatic breast cancer (invasive ductal carcinoma) with osseous metastases
- Chronic kidney disease stage 4 with secondary hyperparathyroidism
- Hyperlipidemia
- Osteoarthritis, left knee, status post total knee arthroplasty
- Umbilical hernia

She has a known history of metastatic breast cancer with osseous metastases, which was initially diagnosed two years ago. She completed a course of chemotherapy and has since been maintained on hormone suppression therapy.

PAST SURGICAL HISTORY:
- Left total knee arthroplasty (3 months prior)
- Right breast lumpectomy with sentinel lymph node biopsy and radiation therapy (10 years prior)
- Tunneled central venous catheter placement for chemotherapy and bisphosphonate infusions

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Insulin glargine 20 units subcutaneous at bedtime
- Insulin lispro sliding scale subcutaneous with meals
- Sevelamer carbonate 800 mg PO three times daily with meals
- Calcitriol 0.25 mcg PO daily
- Pioglitazone 30 mg PO daily
- Anastrozole 1 mg PO daily
- Atorvastatin 40 mg PO daily
- Cyanocobalamin 1000 mcg PO daily
- Acetaminophen 650 mg PO every 6 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Breast cancer, diagnosed age 58
- Father: Myocardial infarction, diagnosed age 67
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired administrative assistant.

PHYSICAL EXAMINATION:
Physical examination reveals dry mucous membranes and mild lower extremity weakness.

LABORATORY DATA:
Laboratory studies show elevated serum calcium at 13.4 mg/dL (normal 8.5–10.5 mg/dL), with suppressed parathyroid hormone (PTH).

IMAGING:
A non-contrast head CT reveals no intracranial abnormalities.

OTHER STUDIES:
ECG demonstrates a shortened QT interval.

## Encounter 7: source 2023-01-15, practice 2026-04-10, ed, Emergency Medicine

Chief complaint: Crushing substernal chest pain radiating to neck and left arm with nausea and dyspnea

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Michael Chang, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Crushing substernal chest pain radiating to neck and left arm with nausea and dyspnea

HISTORY OF PRESENT ILLNESS:
This 65-year-old female with metastatic breast cancer, diabetic nephropathy, and chronic kidney disease presents to the emergency department with 45 minutes of crushing substernal chest pain that radiates to her neck and left arm. She feels nauseated and short of breath.

PAST MEDICAL HISTORY:
Active Problem List:
- Status post left total knee arthroplasty (diagnosed 2020-01-15)
- Vitamin B12 deficiency (diagnosed 2020-04-15)
- Umbilical hernia, incidental finding (diagnosed 2020-07-15)
- Multiple myeloma or metastatic bone disease requiring intravenous bisphosphonates (diagnosed 2021-07-15)
- Malignancy-associated hypercalcemia (diagnosed 2022-01-15)
- Progressive metastatic breast cancer with active systemic disease (diagnosed 2022-07-15)
- Type 2 diabetes mellitus with polyneuropathy and diabetic nephropathy
- Metastatic breast cancer (invasive ductal carcinoma) with osseous metastases
- Chronic kidney disease stage 4 with secondary hyperparathyroidism
- Hyperlipidemia
- Osteoarthritis, left knee, status post total knee arthroplasty
- Umbilical hernia

Her past medical history includes hyperlipidemia and well-controlled type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Left total knee arthroplasty (3 months prior)
- Right breast lumpectomy with sentinel lymph node biopsy and radiation therapy (10 years prior)
- Tunneled central venous catheter placement for chemotherapy and bisphosphonate infusions

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Insulin glargine 20 units subcutaneous at bedtime
- Insulin lispro sliding scale subcutaneous with meals
- Sevelamer carbonate 800 mg PO three times daily with meals
- Calcitriol 0.25 mcg PO daily
- Pioglitazone 30 mg PO daily
- Anastrozole 1 mg PO daily
- Atorvastatin 40 mg PO daily
- Cyanocobalamin 1000 mcg PO daily
- Acetaminophen 650 mg PO every 6 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Breast cancer, diagnosed age 58
- Father: Myocardial infarction, diagnosed age 67
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
She does not smoke or use illicit drugs.

VITAL SIGNS:
On examination, her blood pressure is 130/84 mm Hg, pulse is 75/min, and respirations are 18/min.

PHYSICAL EXAMINATION:
She appears anxious and slightly diaphoretic. Cardiac examination reveals a regular rhythm with no murmurs, rubs, or gallops. Pulmonary auscultation is clear bilaterally.

OTHER STUDIES:
ECG findings are shown below.
