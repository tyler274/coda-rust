# synthetic_hospital patient 2742

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 55 at first encounter, sex F, Non-Hispanic White, Private PPO insurance through spouse.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-02-11 (Thu) | outpatient | Rheumatology |
| 1 | 2020-07-15 | 2021-08-10 (Tue) | outpatient | Orthopedic Surgery |
| 2 | 2021-01-15 | 2022-02-21 (Mon) | outpatient | Internal Medicine |
| 3 | 2021-07-15 | 2022-08-08 (Mon) | outpatient | Rheumatology |
| 4 | 2022-07-15 | 2023-08-16 (Wed) | outpatient | Orthopedic Surgery |
| 5 | 2023-07-15 | 2024-08-07 (Wed) | outpatient | Pulmonology |
| 6 | 2024-01-15 | 2025-02-06 (Thu) | ed | Emergency Medicine |
| 7 | 2025-01-15 | 2026-02-14 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Private PPO insurance through spouse",
 "occupation": "Full-time caregiver for elderly father; former administrative assistant",
 "alcohol_use": "None",
 "family_history": [
  "Father with recent debilitating illness requiring full-time caregiving",
  "Paternal history of male breast cancer",
  "Maternal history of autoimmune disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg IV every 6 months",
   "name": "Rituximab"
  },
  {
   "dose": "5 mg oral daily",
   "name": "Prednisone"
  },
  {
   "dose": "200 mg oral daily",
   "name": "Amiodarone"
  },
  {
   "dose": "20 mg oral daily",
   "name": "Esomeprazole"
  },
  {
   "dose": "0.625 mg/2.5 mg oral daily",
   "name": "Conjugated estrogens/medroxyprogesterone"
  },
  {
   "dose": "70 mg oral weekly",
   "name": "Alendronate"
  },
  {
   "dose": "600 mg oral twice daily",
   "name": "Calcium carbonate"
  },
  {
   "dose": "2000 units oral daily",
   "name": "Cholecalciferol"
  },
  {
   "dose": "400 mg oral every 6 hours as needed for joint pain",
   "name": "Ibuprofen"
  }
 ],
 "surgical_history": [
  "Closed reduction and casting of left wrist for pathological fracture",
  "Lumpectomy with sentinel lymph node biopsy and adjuvant radiation therapy for breast cancer",
  "Cervical spine decompression and fusion (C1-C2) for atlantoaxial instability"
 ],
 "chronic_conditions": [
  "Rheumatoid arthritis (seropositive, 10-year duration)",
  "Osteoporosis with history of pathological fracture",
  "Rheumatoid arthritis-associated interstitial lung disease",
  "History of amiodarone-induced pulmonary toxicity",
  "Cervical myelopathy secondary to atlantoaxial subluxation",
  "History of breast cancer (invasive ductal carcinoma, treated)",
  "Avascular necrosis of right femoral head",
  "Atrial fibrillation",
  "Menopause on hormone replacement therapy"
 ],
 "age_at_first_encounter": 55
}
```

Primary diagnoses: ["Osteoporosis with current pathological fracture", "Amiodarone-induced pulmonary toxicity", "Rheumatoid arthritis-associated interstitial lung disease", "Rheumatoid arthritis", "Cervical myelopathy due to atlantoaxial subluxation", "Malignant neoplasm of male breast", "Avascular necrosis of femoral head", "Bacterial pneumonia"]
Comorbidities: ["Long-term glucocorticoid therapy", "Rheumatoid arthritis", "Postmenopausal status", "Long term (current) use of immunosuppressants"]

## Encounter 0: source 2020-01-15, practice 2021-02-11, outpatient, Rheumatology

Chief complaint: Routine follow-up for rheumatoid arthritis with moderate joint pain and morning stiffness

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for rheumatoid arthritis with moderate joint pain and morning stiffness

HISTORY OF PRESENT ILLNESS:
She reports moderate joint pain and morning stiffness, which have been partially relieved by ibuprofen. She denies any recent infections, alcohol use, or risk factors for viral hepatitis.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatoid arthritis (seropositive, 10-year duration)
- Osteoporosis with history of pathological fracture
- Rheumatoid arthritis-associated interstitial lung disease
- History of amiodarone-induced pulmonary toxicity
- Cervical myelopathy secondary to atlantoaxial subluxation
- History of breast cancer (invasive ductal carcinoma, treated)
- Avascular necrosis of right femoral head
- Atrial fibrillation
- Menopause on hormone replacement therapy

PAST SURGICAL HISTORY:
- Closed reduction and casting of left wrist for pathological fracture
- Lumpectomy with sentinel lymph node biopsy and adjuvant radiation therapy for breast cancer
- Cervical spine decompression and fusion (C1-C2) for atlantoaxial instability

MEDICATIONS:
Her current medications include ibuprofen as needed and low-dose prednisone.

Home Medications:
- Rituximab 1000 mg IV every 6 months
- Amiodarone 200 mg oral daily
- Esomeprazole 20 mg oral daily
- Conjugated estrogens/medroxyprogesterone 0.625 mg/2.5 mg oral daily
- Alendronate 70 mg oral weekly
- Calcium carbonate 600 mg oral twice daily
- Cholecalciferol 2000 units oral daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with recent debilitating illness requiring full-time caregiving
- Paternal history of male breast cancer
- Maternal history of autoimmune disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Full-time caregiver for elderly father; former administrative assistant.

PHYSICAL EXAMINATION:
Her physical examination is notable for bilateral wrist and metacarpophalangeal joint swelling without erythema or warmth.

PLAN:
The rheumatologist decides to initiate methotrexate therapy.

## Encounter 1: source 2020-07-15, practice 2021-08-10, outpatient, Orthopedic Surgery

Chief complaint: Routine follow-up with increasing fatigue and recent wrist fracture from minor fall

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Orthopedic Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up with increasing fatigue and recent wrist fracture from minor fall

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up with history of pathological fracture after rheumatology evaluation, reporting increasing fatigue and recent wrist fracture from a minor fall. She expresses concern regarding bone health.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatoid arthritis (diagnosed 2020-01-15)
- Rheumatoid arthritis (seropositive, 10-year duration)
- Osteoporosis with history of pathological fracture
- Rheumatoid arthritis-associated interstitial lung disease
- History of amiodarone-induced pulmonary toxicity
- Cervical myelopathy secondary to atlantoaxial subluxation
- History of breast cancer (invasive ductal carcinoma, treated)
- Avascular necrosis of right femoral head
- Atrial fibrillation
- Menopause on hormone replacement therapy

PAST SURGICAL HISTORY:
- Closed reduction and casting of left wrist for pathological fracture
- Lumpectomy with sentinel lymph node biopsy and adjuvant radiation therapy for breast cancer
- Cervical spine decompression and fusion (C1-C2) for atlantoaxial instability

MEDICATIONS:
She has been on methotrexate and intermittent glucocorticoids for flare-ups.

Home Medications:
- Rituximab 1000 mg IV every 6 months
- Prednisone 5 mg oral daily
- Amiodarone 200 mg oral daily
- Esomeprazole 20 mg oral daily
- Conjugated estrogens/medroxyprogesterone 0.625 mg/2.5 mg oral daily
- Alendronate 70 mg oral weekly
- Calcium carbonate 600 mg oral twice daily
- Cholecalciferol 2000 units oral daily
- Ibuprofen 400 mg oral every 6 hours as needed for joint pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with recent debilitating illness requiring full-time caregiving
- Paternal history of male breast cancer
- Maternal history of autoimmune disease

SOCIAL HISTORY:
She has a sedentary lifestyle and her diet is low in dairy products.

PHYSICAL EXAMINATION:
On physical examination, she has a reduced grip strength and tenderness over the dorsal aspect of her wrists.

LABORATORY DATA:
Her recent laboratory tests show:

- Serum calcium: 9.2 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum albumin: 4.0 g/dL (normal: 3.5-5.0 g/dL)
- Serum creatinine: 0.9 mg/dL (normal: 0.6-1.1 mg/dL)
- 25-hydroxyvitamin D: 20 ng/mL (normal: 30-100 ng/mL)
- TSH: 2.5 mIU/L (normal: 0.4-4.0 mIU/L)

OTHER STUDIES:
Bone mineral density (BMD) testing reveals a T-score of -2.5 at the lumbar spine.

## Encounter 2: source 2021-01-15, practice 2022-02-21, outpatient, Internal Medicine

Chief complaint: Routine health maintenance examination with review of hormone replacement therapy started three years ago

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Emily Johnson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health maintenance examination with review of hormone replacement therapy started three years ago

HISTORY OF PRESENT ILLNESS:
The patient returns for routine health maintenance and review of hormone replacement therapy initiated three years prior, in the context of her known seropositive rheumatoid arthritis, osteoporosis with recent pathological wrist fracture, and history of invasive breast cancer. She reports being generally healthy and maintains a fairly active lifestyle. Approximately three years ago, she began experiencing bothersome hot flashes, night sweats, and difficulty sleeping due to vasomotor instability. At the time, her physician prescribed hormone replacement therapy, consisting of estrogen combined with progestin, which she continues to take daily. She reports that her menopausal symptoms have greatly improved since starting therapy, and she feels much more comfortable in daily life. She denies vaginal bleeding, abdominal pain, bloating, or urinary symptoms. She has not experienced breast pain, nipple discharge, or palpable breast lumps.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatoid arthritis (diagnosed 2020-01-15)
- Osteoporosis with current pathological fracture (diagnosed 2020-07-15)
- Rheumatoid arthritis (seropositive, 10-year duration)
- Osteoporosis with history of pathological fracture
- Rheumatoid arthritis-associated interstitial lung disease
- History of amiodarone-induced pulmonary toxicity
- Cervical myelopathy secondary to atlantoaxial subluxation
- History of breast cancer (invasive ductal carcinoma, treated)
- Avascular necrosis of right femoral head
- Atrial fibrillation
- Menopause on hormone replacement therapy

She has no personal history of chronic medical conditions such as diabetes, hypertension, or hyperlipidemia. She has never had abnormal Pap smears. She has never had colonoscopy but is aware that she is due for one.

PAST SURGICAL HISTORY:
- Closed reduction and casting of left wrist for pathological fracture
- Lumpectomy with sentinel lymph node biopsy and adjuvant radiation therapy for breast cancer
- Cervical spine decompression and fusion (C1-C2) for atlantoaxial instability

MEDICATIONS:
She takes hormone replacement therapy, consisting of estrogen combined with progestin, which she continues to take daily. She does not use any other medications besides a daily multivitamin.

Home Medications:
- Rituximab 1000 mg IV every 6 months
- Prednisone 5 mg oral daily
- Amiodarone 200 mg oral daily
- Esomeprazole 20 mg oral daily
- Conjugated estrogens/medroxyprogesterone 0.625 mg/2.5 mg oral daily
- Alendronate 70 mg oral weekly
- Calcium carbonate 600 mg oral twice daily
- Cholecalciferol 2000 units oral daily
- Ibuprofen 400 mg oral every 6 hours as needed for joint pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Her family history is significant for her mother being diagnosed with breast cancer at age 60, treated with lumpectomy and radiation. No other relatives are known to have cancer.

SOCIAL HISTORY:
She exercises by walking several times per week, maintains a low-fat diet, and has kept her weight stable. She does not drink alcohol and has never smoked. She is married and has two children.

REVIEW OF SYSTEMS:
On review of systems, she denies fatigue, unexplained weight loss, or bone pain. She continues to feel well overall.

VITAL SIGNS:
On examination, her vital signs are normal, with blood pressure of 128/74 mm Hg. BMI is 23 kg/m².

PHYSICAL EXAMINATION:
Cardiopulmonary and abdominal exams are unremarkable. Breast exam reveals no palpable masses, nipple retraction, or overlying skin changes. There is no lymphadenopathy in the axilla or supraclavicular regions. Pelvic exam shows a normal-sized uterus, no adnexal masses, and no vaginal bleeding. The remainder of the physical exam is within normal limits.

## Encounter 3: source 2021-07-15, practice 2022-08-08, outpatient, Rheumatology

Chief complaint: Progressive dyspnea on exertion and nonproductive cough over the past two months

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Sarah Chen, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive dyspnea on exertion and nonproductive cough over the past two months

HISTORY OF PRESENT ILLNESS:
This 56-year-old female with rheumatoid arthritis, osteoporosis with prior fracture, and amiodarone-induced pulmonary toxicity returns to rheumatology following internal medicine follow-up. She presents with 2-month history of progressive dyspnea on exertion and nonproductive cough. She denies fever, chest pain, or recent travel. Her rheumatoid arthritis has been poorly controlled on hydroxychloroquine and low-dose prednisone.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatoid arthritis (diagnosed 2020-01-15)
- Osteoporosis with current pathological fracture (diagnosed 2020-07-15)
- Malignant neoplasm of male breast (diagnosed 2021-01-15)
- Rheumatoid arthritis (seropositive, 10-year duration)
- Osteoporosis with history of pathological fracture
- Rheumatoid arthritis-associated interstitial lung disease
- History of amiodarone-induced pulmonary toxicity
- Cervical myelopathy secondary to atlantoaxial subluxation
- History of breast cancer (invasive ductal carcinoma, treated)
- Avascular necrosis of right femoral head
- Atrial fibrillation
- Menopause on hormone replacement therapy

PAST SURGICAL HISTORY:
- Closed reduction and casting of left wrist for pathological fracture
- Lumpectomy with sentinel lymph node biopsy and adjuvant radiation therapy for breast cancer
- Cervical spine decompression and fusion (C1-C2) for atlantoaxial instability

MEDICATIONS:
She takes hydroxychloroquine and low-dose prednisone.

Home Medications:
- Rituximab 1000 mg IV every 6 months
- Amiodarone 200 mg oral daily
- Esomeprazole 20 mg oral daily
- Conjugated estrogens/medroxyprogesterone 0.625 mg/2.5 mg oral daily
- Alendronate 70 mg oral weekly
- Calcium carbonate 600 mg oral twice daily
- Cholecalciferol 2000 units oral daily
- Ibuprofen 400 mg oral every 6 hours as needed for joint pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with recent debilitating illness requiring full-time caregiving
- Paternal history of male breast cancer
- Maternal history of autoimmune disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Full-time caregiver for elderly father; former administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, she is tachypneic with a respiratory rate of 22 breaths per minute. Auscultation of the lungs reveals bibasilar crackles. No joint swelling or deformities are noted on this visit.

LABORATORY DATA:
White blood cell count: 9,000/uL
- Hemoglobin: 12.5 g/dL
- Platelet count: 350,000/uL
- Serum creatinine: 0.9 mg/dL
- Erythrocyte sedimentation rate: 45 mm/hr
- C-reactive protein: 3.2 mg/L
- Rheumatoid factor: Positive
- Anti-cyclic citrullinated peptide antibodies: Positive

IMAGING:
A chest X-ray shows bilateral lower lobe interstitial infiltrates.

OTHER STUDIES:
A thoracentesis is performed, and pleural fluid analysis shows:

- Appearance: Straw-colored
- White blood cell count: 1500/uL with 70% lymphocytes
- Glucose: 30 mg/dL
- Lactate dehydrogenase (LDH): 700 U/L (serum LDH 190 U/L)
- Protein: 4.5 g/dL (serum protein 7.0 g/dL)
- pH: 7.30

## Encounter 4: source 2022-07-15, practice 2023-08-16, outpatient, Orthopedic Surgery

Chief complaint: Progressive right-sided hip pain over the past six weeks, worse with weight-bearing

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Aisha Patel, MD
Department: Orthopedic Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive right-sided hip pain over the past six weeks, worse with weight-bearing

HISTORY OF PRESENT ILLNESS:
The patient with rheumatoid arthritis returns to orthopedic surgery for progressive hip pain over the past 6 weeks, following pathological wrist fracture. She describes pain as deep and aching, worse with weight-bearing, and present even at rest. She denies history of trauma, fever, or recent infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatoid arthritis (diagnosed 2020-01-15)
- Osteoporosis with current pathological fracture (diagnosed 2020-07-15)
- Malignant neoplasm of male breast (diagnosed 2021-01-15)
- Rheumatoid arthritis-associated interstitial lung disease (diagnosed 2021-07-15)
- Rheumatoid arthritis (seropositive, 10-year duration)
- Osteoporosis with history of pathological fracture
- History of amiodarone-induced pulmonary toxicity
- Cervical myelopathy secondary to atlantoaxial subluxation
- History of breast cancer (invasive ductal carcinoma, treated)
- Avascular necrosis of right femoral head
- Atrial fibrillation
- Menopause on hormone replacement therapy

PAST SURGICAL HISTORY:
The patient underwent a kidney transplant one year ago

MEDICATIONS:
She takes multiple medications daily.

Home Medications:
- Rituximab 1000 mg IV every 6 months
- Prednisone 5 mg oral daily
- Amiodarone 200 mg oral daily
- Esomeprazole 20 mg oral daily
- Conjugated estrogens/medroxyprogesterone 0.625 mg/2.5 mg oral daily
- Alendronate 70 mg oral weekly
- Calcium carbonate 600 mg oral twice daily
- Cholecalciferol 2000 units oral daily
- Ibuprofen 400 mg oral every 6 hours as needed for joint pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with recent debilitating illness requiring full-time caregiving
- Paternal history of male breast cancer
- Maternal history of autoimmune disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Full-time caregiver for elderly father; former administrative assistant.

PHYSICAL EXAMINATION:
Physical examination reveals pain with internal rotation and limited range of motion of the right hip. There is no joint swelling, erythema, or warmth.

LABORATORY DATA:
Laboratory tests show normal inflammatory markers and white blood cell count.

IMAGING:
X-ray shows a contracted femoral head.

## Encounter 5: source 2023-07-15, practice 2024-08-07, outpatient, Pulmonology

Chief complaint: Gradually worsening shortness of breath over the past four months with persistent dry cough

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. David Park, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Gradually worsening shortness of breath over the past four months with persistent dry cough

HISTORY OF PRESENT ILLNESS:
The patient returns for evaluation of progressive respiratory symptoms in the setting of known rheumatoid arthritis-associated interstitial lung disease and chronic amiodarone therapy. She reports initially attributing her symptoms to being "out of shape," but her breathing difficulty has progressed to the point where she cannot walk more than a block without stopping. She also notes a persistent dry cough that never seems to produce sputum.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatoid arthritis (diagnosed 2020-01-15)
- Osteoporosis with current pathological fracture (diagnosed 2020-07-15)
- Malignant neoplasm of male breast (diagnosed 2021-01-15)
- Rheumatoid arthritis-associated interstitial lung disease (diagnosed 2021-07-15)
- Avascular necrosis of femoral head (diagnosed 2022-07-15)
- Rheumatoid arthritis (seropositive, 10-year duration)
- Osteoporosis with history of pathological fracture
- History of amiodarone-induced pulmonary toxicity
- Cervical myelopathy secondary to atlantoaxial subluxation
- History of breast cancer (invasive ductal carcinoma, treated)
- Avascular necrosis of right femoral head
- Atrial fibrillation
- Menopause on hormone replacement therapy

Her joint pain from long-standing rheumatoid arthritis has been mostly controlled, though she continues to have some stiffness in the mornings.

PAST SURGICAL HISTORY:
- Closed reduction and casting of left wrist for pathological fracture
- Lumpectomy with sentinel lymph node biopsy and adjuvant radiation therapy for breast cancer
- Cervical spine decompression and fusion (C1-C2) for atlantoaxial instability

MEDICATIONS:
She has been on methotrexate therapy for several years as her primary disease-modifying antirheumatic drug (DMARD). In addition to methotrexate, she intermittently uses low-dose prednisone for flares, and she has also received courses of nonsteroidal anti-inflammatory drugs (NSAIDs) when joint pain worsens.

Home Medications:
- Rituximab 1000 mg IV every 6 months
- Amiodarone 200 mg oral daily
- Esomeprazole 20 mg oral daily
- Conjugated estrogens/medroxyprogesterone 0.625 mg/2.5 mg oral daily
- Alendronate 70 mg oral weekly
- Calcium carbonate 600 mg oral twice daily
- Cholecalciferol 2000 units oral daily
- Ibuprofen 400 mg oral every 6 hours as needed for joint pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with recent debilitating illness requiring full-time caregiving
- Paternal history of male breast cancer
- Maternal history of autoimmune disease

SOCIAL HISTORY:
She explains that her personal life has been complicated recently. Her father became ill last year, and she moved in with him to help take care of him. She describes long days of cooking, cleaning, and organizing medications for her father, which made her feel both exhausted and emotionally fulfilled. Recently, her father decided to move across the state to take advantage of a better insurance policy, which has left her with mixed emotions—she misses him but is relieved to have more time for her own life again. Outside of her family responsibilities, she is passionate about rare plants. She has collected exotic species from dozens of different countries. She describes trips to South America to find rare orchids, journeys to Southeast Asia for tropical ferns, and exhibitions in Europe where she has displayed parts of her collection. During her travels, she often met friends in small taverns where they would exchange plants and stories of botany. She proudly states that she has cultivated her greenhouse into a museum of biodiversity, with species rarely seen outside their native ecosystems. Despite her vibrant hobbies, she now finds herself struggling to maintain the energy to care for her plants. She reports being winded when watering or repotting. She does not smoke, drink, or use recreational drugs. She has never smoked and has no known environmental exposures to asbestos, silica, or coal dust.

REVIEW OF SYSTEMS:
She denies fevers, chills, or weight loss, but says her appetite has been slightly reduced because “food just doesn’t taste good when you can’t catch your breath.” She has no chest pain, orthopnea, or leg swelling.

VITAL SIGNS:
Vital signs reveal an oxygen saturation of 91% on room air, respiratory rate of 21/min, and blood pressure of 122/74 mm Hg. Her temperature is normal.

PHYSICAL EXAMINATION:
On physical examination, she is in mild respiratory distress at rest. Cardiac examination shows a regular rate and rhythm without murmurs. Pulmonary examination reveals bilateral end-inspiratory crackles most pronounced at the lung bases. There is no wheezing or rhonchi. Clubbing of the fingers is noted. The remainder of the exam, including joint inspection, reveals only mild deformities consistent with longstanding rheumatoid arthritis but no acute synovitis.

IMAGING:
Chest radiography demonstrates diffuse interstitial opacities. High-resolution CT of the chest reveals a reticular pattern with areas of honeycombing.

OTHER STUDIES:
Pulmonary function testing demonstrates a restrictive pattern with reduced total lung capacity and a decreased diffusion capacity for carbon monoxide (DLCO).

## Encounter 6: source 2024-01-15, practice 2025-02-06, ed, Emergency Medicine

Chief complaint: Acute fever and productive cough in the setting of severe rheumatoid arthritis and immunosuppression

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Lisa Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute fever and productive cough in the setting of severe rheumatoid arthritis and immunosuppression

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatoid arthritis (diagnosed 2020-01-15)
- Osteoporosis with current pathological fracture (diagnosed 2020-07-15)
- Malignant neoplasm of male breast (diagnosed 2021-01-15)
- Rheumatoid arthritis-associated interstitial lung disease (diagnosed 2021-07-15)
- Avascular necrosis of femoral head (diagnosed 2022-07-15)
- Amiodarone-induced pulmonary toxicity (diagnosed 2023-07-15)
- Rheumatoid arthritis (seropositive, 10-year duration)
- Osteoporosis with history of pathological fracture
- Cervical myelopathy secondary to atlantoaxial subluxation
- History of breast cancer (invasive ductal carcinoma, treated)
- Avascular necrosis of right femoral head
- Atrial fibrillation
- Menopause on hormone replacement therapy

PAST SURGICAL HISTORY:
- Closed reduction and casting of left wrist for pathological fracture
- Lumpectomy with sentinel lymph node biopsy and adjuvant radiation therapy for breast cancer
- Cervical spine decompression and fusion (C1-C2) for atlantoaxial instability

MEDICATIONS:
She is started on rituximab after failing to improve with methotrexate, inflixmab, prednisone, and diclofenac. She also takes esomeprazole to protect her stomach.

Home Medications:
- Amiodarone 200 mg oral daily
- Conjugated estrogens/medroxyprogesterone 0.625 mg/2.5 mg oral daily
- Alendronate 70 mg oral weekly
- Calcium carbonate 600 mg oral twice daily
- Cholecalciferol 2000 units oral daily
- Ibuprofen 400 mg oral every 6 hours as needed for joint pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with recent debilitating illness requiring full-time caregiving
- Paternal history of male breast cancer
- Maternal history of autoimmune disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Full-time caregiver for elderly father; former administrative assistant.

## Encounter 7: source 2025-01-15, practice 2026-02-14, ed, Emergency Medicine

Chief complaint: Progressive weakness in arms and legs with difficulty performing fine motor tasks and unsteady gait over two months

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive weakness in arms and legs with difficulty performing fine motor tasks and unsteady gait over two months

HISTORY OF PRESENT ILLNESS:
This patient with rheumatoid arthritis and cervical myelopathy returns with progressive symptoms. She reports difficulty with fine motor tasks such as buttoning her shirt and has noticed an unsteady gait. She denies any trauma or injury.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatoid arthritis (diagnosed 2020-01-15)
- Osteoporosis with current pathological fracture (diagnosed 2020-07-15)
- Malignant neoplasm of male breast (diagnosed 2021-01-15)
- Rheumatoid arthritis-associated interstitial lung disease (diagnosed 2021-07-15)
- Avascular necrosis of femoral head (diagnosed 2022-07-15)
- Amiodarone-induced pulmonary toxicity (diagnosed 2023-07-15)
- Bacterial pneumonia (diagnosed 2024-01-15)
- Rheumatoid arthritis (seropositive, 10-year duration)
- Osteoporosis with history of pathological fracture
- Cervical myelopathy secondary to atlantoaxial subluxation
- History of breast cancer (invasive ductal carcinoma, treated)
- Avascular necrosis of right femoral head
- Atrial fibrillation
- Menopause on hormone replacement therapy

PAST SURGICAL HISTORY:
- Closed reduction and casting of left wrist for pathological fracture
- Lumpectomy with sentinel lymph node biopsy and adjuvant radiation therapy for breast cancer
- Cervical spine decompression and fusion (C1-C2) for atlantoaxial instability

MEDICATIONS:
Home Medications:
- Rituximab 1000 mg IV every 6 months
- Prednisone 5 mg oral daily
- Amiodarone 200 mg oral daily
- Esomeprazole 20 mg oral daily
- Conjugated estrogens/medroxyprogesterone 0.625 mg/2.5 mg oral daily
- Alendronate 70 mg oral weekly
- Calcium carbonate 600 mg oral twice daily
- Cholecalciferol 2000 units oral daily
- Ibuprofen 400 mg oral every 6 hours as needed for joint pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with recent debilitating illness requiring full-time caregiving
- Paternal history of male breast cancer
- Maternal history of autoimmune disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Full-time caregiver for elderly father; former administrative assistant.

PHYSICAL EXAMINATION:
On examination, she has 4/5 strength in both upper and lower extremities, increased tone, and hyperreflexia. A positive Babinski sign is noted bilaterally, and the Hoffman sign is also positive on the right. She has decreased sensation to light touch and pinprick in a cape-like distribution over her shoulders.

IMAGING:
Radiographs of the cervical spine show evidence of atlantoaxial subluxation.
