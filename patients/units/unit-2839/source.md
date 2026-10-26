# synthetic_hospital patient 2839

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex F, Non-Hispanic White, Medicare with supplemental private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-12-26 (Mon) | outpatient | Gastroenterology |
| 1 | 2020-04-15 | 2023-03-13 (Mon) | outpatient | Endocrinology |
| 2 | 2020-07-15 | 2023-06-22 (Thu) | outpatient | Internal Medicine |
| 3 | 2021-04-15 | 2024-03-27 (Wed) | outpatient | Rheumatology |
| 4 | 2021-10-15 | 2024-09-23 (Mon) | outpatient | Otolaryngology |
| 5 | 2022-01-15 | 2024-12-18 (Wed) | outpatient | Otolaryngology |
| 6 | 2022-09-15 | 2025-08-24 (Sun) | ed | Emergency Medicine |
| 7 | 2023-03-15 | 2026-02-19 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare with supplemental private insurance",
 "occupation": "Retired elementary school teacher",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Mother with postmenopausal breast cancer",
  "Father with colon cancer",
  "Sister with rheumatoid arthritis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 10 years ago",
 "home_medications": [
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "15 mg PO weekly",
   "name": "Methotrexate"
  },
  {
   "dose": "1 mg PO daily",
   "name": "Folic acid"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Prednisone"
  },
  {
   "dose": "4 mg IV infusion monthly",
   "name": "Zoledronic acid"
  },
  {
   "dose": "40 mg SC daily (prophylactic)",
   "name": "Enoxaparin"
  },
  {
   "dose": "800 mg PO three times daily",
   "name": "Mesalamine"
  },
  {
   "dose": "600 mg/400 IU PO twice daily",
   "name": "Calcium carbonate/Vitamin D3"
  }
 ],
 "surgical_history": [
  "Dental extraction (complicated by osteonecrosis)",
  "Breast cancer staging surgery"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia",
  "Postmenopausal osteoporosis",
  "Rheumatoid arthritis",
  "Metastatic breast cancer with bone involvement",
  "Ulcerative colitis"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Essential hypertension", "Avascular necrosis of femoral head", "Osteoporosis", "Bisphosphonate-related osteonecrosis of the jaw", "Pulmonary embolism", "Acute diverticulitis of sigmoid colon", "Ulcerative colitis"]
Comorbidities: ["Osteoporosis", "Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2022-12-26, outpatient, Gastroenterology

Chief complaint: Intermittent abdominal pain and bloody diarrhea for 6 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Sharma, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent abdominal pain and bloody diarrhea for 6 months

HISTORY OF PRESENT ILLNESS:
She reports having up to six bowel movements daily, often accompanied by an urgent need to defecate and a sensation of incomplete evacuation.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- Postmenopausal osteoporosis
- Rheumatoid arthritis
- Metastatic breast cancer with bone involvement
- Ulcerative colitis

Her past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Dental extraction (complicated by osteonecrosis)
- Breast cancer staging surgery

MEDICATIONS:
She takes for which she takes hydrochlorothiazide.

Home Medications:
- Amlodipine 5 mg PO daily
- Atorvastatin 20 mg PO daily
- Methotrexate 15 mg PO weekly
- Folic acid 1 mg PO daily
- Prednisone 5 mg PO daily
- Zoledronic acid 4 mg IV infusion monthly
- Enoxaparin 40 mg SC daily (prophylactic)
- Mesalamine 800 mg PO three times daily
- Calcium carbonate/Vitamin D3 600 mg/400 IU PO twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
She denies any family history of gastrointestinal diseases.

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago. Alcohol: Social (1-2 drinks per week). Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, she appears mildly distressed. Her abdominal examination reveals mild diffuse tenderness without rebound or guarding. Rectal examination shows no masses, but there is bright red blood on the glove.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 11.2 g/dL (normal: 12-16 g/dL)
- White blood cell count: 9,000 /µL (normal: 4,500-11,000 /µL)
- Platelet count: 310,000 /µL (normal: 150,000-450,000 /µL)
- C-reactive protein: 24 mg/L (normal: <5 mg/L)
- Erythrocyte sedimentation rate: 40 mm/hr (normal: 0-20 mm/hr)

PATHOLOGY:
Biopsy specimens show crypt abscesses and mucosal ulceration.

OTHER STUDIES:
Colonoscopy reveals continuous colonic mucosal inflammation starting from the rectum and extending proximally, with loss of the normal vascular pattern, erythema, and multiple superficial ulcers.

## Encounter 1: source 2020-04-15, practice 2023-03-13, outpatient, Endocrinology

Chief complaint: Routine follow-up for postmenopausal osteoporosis

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. James Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for postmenopausal osteoporosis

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up for osteoporosis with metastatic breast cancer and recent Gastroenterology evaluation for ulcerative colitis. She has a history of postmenopausal osteoporosis diagnosed 2 years ago and has been on alendronate since then. She reports compliance with her medications and has no new complaints. She denies any recent falls or fractures.

PAST MEDICAL HISTORY:
Active Problem List:
- Ulcerative colitis (diagnosed 2020-01-15)
- Hypertension
- Hyperlipidemia
- Postmenopausal osteoporosis
- Rheumatoid arthritis
- Metastatic breast cancer with bone involvement

Her medical history is significant for hypertension, controlled with hydrochlorothiazide, and hyperlipidemia, managed with atorvastatin.

PAST SURGICAL HISTORY:
- Dental extraction (complicated by osteonecrosis)
- Breast cancer staging surgery

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 20 mg PO daily
- Methotrexate 15 mg PO weekly
- Folic acid 1 mg PO daily
- Prednisone 5 mg PO daily
- Zoledronic acid 4 mg IV infusion monthly
- Enoxaparin 40 mg SC daily (prophylactic)
- Mesalamine 800 mg PO three times daily
- Calcium carbonate/Vitamin D3 600 mg/400 IU PO twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Her family history is notable for a mother with hip fracture at the age of 72.

SOCIAL HISTORY:
She quit smoking 10 years ago and consumes alcohol socially.

PHYSICAL EXAMINATION:
On physical examination, her height is 2 cm less than it was 2 years ago. Her current weight is stable. The rest of the physical examination is unremarkable.

LABORATORY DATA:
Recent laboratory tests are as follows:

- Serum calcium: 9.8 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum phosphorus: 3.5 mg/dL (normal: 2.5-4.5 mg/dL)
- Alkaline phosphatase: 70 U/L (normal: 35-100 U/L)
- Thyroid-stimulating hormone (TSH): 2.1 mIU/L (normal: 0.4-4.0 mIU/L)
- 25-hydroxyvitamin D: 32 ng/mL (normal: 20-50 ng/mL)

IMAGING:
A dual-energy X-ray absorptiometry (DEXA) scan shows a T-score of -2.5 at the lumbar spine, which is improved from -3.0 at the time of diagnosis.

## Encounter 2: source 2020-07-15, practice 2023-06-22, outpatient, Internal Medicine

Chief complaint: Routine follow-up and medication review

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Maria Gonzalez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up and medication review

HISTORY OF PRESENT ILLNESS:
She returns for routine follow-up after gastroenterology and osteoporosis care. She reports compliance with her medications and a healthy diet but is concerned about the number of pills she takes daily.

PAST MEDICAL HISTORY:
Active Problem List:
- Ulcerative colitis (diagnosed 2020-01-15)
- Osteoporosis (diagnosed 2020-04-15)
- Hypertension
- Hyperlipidemia
- Postmenopausal osteoporosis
- Rheumatoid arthritis
- Metastatic breast cancer with bone involvement

She has a history of hypertension and osteoporosis.

PAST SURGICAL HISTORY:
- Dental extraction (complicated by osteonecrosis)
- Breast cancer staging surgery

MEDICATIONS:
She is currently taking hydrochlorothiazide and alendronate.

Home Medications:
- Amlodipine 5 mg PO daily
- Atorvastatin 20 mg PO daily
- Methotrexate 15 mg PO weekly
- Folic acid 1 mg PO daily
- Prednisone 5 mg PO daily
- Zoledronic acid 4 mg IV infusion monthly
- Enoxaparin 40 mg SC daily (prophylactic)
- Mesalamine 800 mg PO three times daily
- Calcium carbonate/Vitamin D3 600 mg/400 IU PO twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with postmenopausal breast cancer
- Father with colon cancer
- Sister with rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago. Alcohol: Social (1-2 drinks per week). Occupation: Retired elementary school teacher.

VITAL SIGNS:
Her blood pressure today is 142/88 mmHg.

PHYSICAL EXAMINATION:
Her physical examination is unremarkable.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum calcium: 9.8 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum creatinine: 1.0 mg/dL (normal: 0.6-1.1 mg/dL)
- Serum potassium: 4.2 mEq/L (normal: 3.5-5.1 mEq/L)
- Bone mineral density T-score: -2.5 at the lumbar spine

## Encounter 3: source 2021-04-15, practice 2024-03-27, outpatient, Rheumatology

Chief complaint: Progressive right hip pain over 3 months

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. Robert Kim, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive right hip pain over 3 months

HISTORY OF PRESENT ILLNESS:
She presents with hip pain amid osteoporosis and rheumatoid arthritis. The pain is deep, worsens with weight-bearing, and does not improve with rest. She denies trauma, fever, chills, or constitutional symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Ulcerative colitis (diagnosed 2020-01-15)
- Osteoporosis (diagnosed 2020-04-15)
- Essential hypertension (diagnosed 2020-07-15)
- Hypertension
- Hyperlipidemia
- Postmenopausal osteoporosis
- Rheumatoid arthritis
- Metastatic breast cancer with bone involvement

Her medical history includes advanced rheumatoid arthritis, hypertension, and osteoporosis.

PAST SURGICAL HISTORY:
- Dental extraction (complicated by osteonecrosis)
- Breast cancer staging surgery

MEDICATIONS:
Medications include methotrexate, folic acid, amlodipine, calcium/vitamin D, and chronic oral prednisone.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 20 mg PO daily
- Zoledronic acid 4 mg IV infusion monthly
- Enoxaparin 40 mg SC daily (prophylactic)
- Mesalamine 800 mg PO three times daily
- Calcium carbonate/Vitamin D3 600 mg/400 IU PO twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with postmenopausal breast cancer
- Father with colon cancer
- Sister with rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago. Alcohol: Social (1-2 drinks per week). Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
On physical exam, there is limited internal rotation and abduction of the right hip, with mild tenderness over the groin. No overlying erythema, swelling, or warmth is noted. Neurologic and vascular exams are normal.

IMAGING:
An MRI of the pelvis is obtained and shown below.

## Encounter 4: source 2021-10-15, practice 2024-09-23, outpatient, Otolaryngology

Chief complaint: Pain and swelling in lower jaw for 3 weeks following dental extraction

ENCOUNTER NOTE
Date: 2021-10-15
Provider: Dr. Angela Okonkwo, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Pain and swelling in lower jaw for 3 weeks following dental extraction

HISTORY OF PRESENT ILLNESS:
The patient, receiving monthly zoledronic acid for osteoporosis and metastatic bone disease, presents with jaw pain. She reports that the symptoms began shortly after she had a tooth extracted.

PAST MEDICAL HISTORY:
Active Problem List:
- Ulcerative colitis (diagnosed 2020-01-15)
- Osteoporosis (diagnosed 2020-04-15)
- Essential hypertension (diagnosed 2020-07-15)
- Avascular necrosis of femoral head (diagnosed 2021-04-15)
- Hypertension
- Hyperlipidemia
- Postmenopausal osteoporosis
- Rheumatoid arthritis
- Metastatic breast cancer with bone involvement

Her medical history is significant for osteoporosis.

PAST SURGICAL HISTORY:
- Dental extraction (complicated by osteonecrosis)
- Breast cancer staging surgery

MEDICATIONS:
She takes alendronate for the past 6 years.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 20 mg PO daily
- Methotrexate 15 mg PO weekly
- Folic acid 1 mg PO daily
- Prednisone 5 mg PO daily
- Zoledronic acid 4 mg IV infusion monthly
- Enoxaparin 40 mg SC daily (prophylactic)
- Mesalamine 800 mg PO three times daily
- Calcium carbonate/Vitamin D3 600 mg/400 IU PO twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with postmenopausal breast cancer
- Father with colon cancer
- Sister with rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago. Alcohol: Social (1-2 drinks per week). Occupation: Retired elementary school teacher.

VITAL SIGNS:
Her temperature is 37.2°C (99°F), blood pressure is 130/85 mmHg, and pulse is 78/min.

PHYSICAL EXAMINATION:
On examination, there is evident swelling of the mandible, and on intraoral examination, exposed necrotic bone is visible in the area of the tooth extraction. There is no regional lymphadenopathy.

LABORATORY DATA:
- White blood cell count: 7,500/uL
- Hemoglobin: 13.2 g/dL
- Platelet count: 250,000/uL
- Erythrocyte sedimentation rate: 20 mm/hr
- C-reactive protein: 2.5 mg/L

## Encounter 5: source 2022-01-15, practice 2024-12-18, outpatient, Otolaryngology

Chief complaint: Persistent jaw pain for 3 months following dental extraction

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Angela Okonkwo, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent jaw pain for 3 months following dental extraction

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old female with metastatic breast cancer on monthly zoledronic acid therapy who returns to Otolaryngology for persistent jaw pain following her prior visit on October 15, 2021. She reports undergoing a routine dental extraction 4 months ago and states that the extraction site "never fully healed." She denies fever, trauma, or recent infection.

PAST MEDICAL HISTORY:
Active Problem List:
- Ulcerative colitis (diagnosed 2020-01-15)
- Osteoporosis (diagnosed 2020-04-15)
- Essential hypertension (diagnosed 2020-07-15)
- Avascular necrosis of femoral head (diagnosed 2021-04-15)
- Bisphosphonate-related osteonecrosis of the jaw (diagnosed 2021-10-15)
- Hypertension
- Hyperlipidemia
- Postmenopausal osteoporosis
- Rheumatoid arthritis
- Metastatic breast cancer with bone involvement

Her medical history includes postmenopausal osteoporosis and metastatic breast cancer.

PAST SURGICAL HISTORY:
- Dental extraction (complicated by osteonecrosis)
- Breast cancer staging surgery

MEDICATIONS:
She has been receiving intravenous zoledronic acid every 4 weeks for the past year.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 20 mg PO daily
- Methotrexate 15 mg PO weekly
- Folic acid 1 mg PO daily
- Prednisone 5 mg PO daily
- Enoxaparin 40 mg SC daily (prophylactic)
- Mesalamine 800 mg PO three times daily
- Calcium carbonate/Vitamin D3 600 mg/400 IU PO twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with postmenopausal breast cancer
- Father with colon cancer
- Sister with rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago. Alcohol: Social (1-2 drinks per week). Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
Physical examination reveals exposed bone in the posterior mandible with surrounding gingival recession. There is no purulent drainage or regional lymphadenopathy.

IMAGING:
A panoramic dental radiograph shows sclerotic bone with areas of irregular lucency in the mandible.

OTHER STUDIES:
A biopsy of the lesion is performed.

## Encounter 6: source 2022-09-15, practice 2025-08-24, ed, Emergency Medicine

Chief complaint: Left lower quadrant abdominal pain for 48 hours with nausea and urinary symptoms

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. David Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Left lower quadrant abdominal pain for 48 hours with nausea and urinary symptoms

HISTORY OF PRESENT ILLNESS:
This 69-year-old female with a history of ulcerative colitis and metastatic breast cancer with bone involvement previously complicated by bisphosphonate-related osteonecrosis of the jaw presents to the emergency department with a 48-hour history of dull, left lower quadrant abdominal pain. She reports associated nausea and a decrease in appetite but denies vomiting. She also complains of increased urinary frequency and a burning sensation during urination. She has no history of similar symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Ulcerative colitis (diagnosed 2020-01-15)
- Osteoporosis (diagnosed 2020-04-15)
- Essential hypertension (diagnosed 2020-07-15)
- Avascular necrosis of femoral head (diagnosed 2021-04-15)
- Bisphosphonate-related osteonecrosis of the jaw (diagnosed 2021-10-15)
- Hypertension
- Hyperlipidemia
- Postmenopausal osteoporosis
- Rheumatoid arthritis
- Metastatic breast cancer with bone involvement

Her past medical history includes hypertension, for which she takes hydrochlorothiazide, and hyperlipidemia managed with atorvastatin.

PAST SURGICAL HISTORY:
- Dental extraction (complicated by osteonecrosis)
- Breast cancer staging surgery

MEDICATIONS:
She takes hydrochlorothiazide, atorvastatin.

Home Medications:
- Amlodipine 5 mg PO daily
- Methotrexate 15 mg PO weekly
- Folic acid 1 mg PO daily
- Prednisone 5 mg PO daily
- Zoledronic acid 4 mg IV infusion monthly
- Enoxaparin 40 mg SC daily (prophylactic)
- Mesalamine 800 mg PO three times daily
- Calcium carbonate/Vitamin D3 600 mg/400 IU PO twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with postmenopausal breast cancer
- Father with colon cancer
- Sister with rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago. Alcohol: Social (1-2 drinks per week). Occupation: Retired elementary school teacher.

VITAL SIGNS:
Temperature is 38.2°C (100.8°F), blood pressure is 145/90 mmHg, and heart rate is 92 beats per minute

PHYSICAL EXAMINATION:
Abdominal examination reveals tenderness in the left lower quadrant with no rebound or guarding. Bowel sounds are normal. Rectal examination is unremarkable.

LABORATORY DATA:
White blood cell count: 14,000/µL
Hemoglobin: 13.2 g/dL
Platelets: 250,000/µL
Urinalysis: Trace leukocytes, no bacteria

IMAGING:
A CT scan of the abdomen and pelvis with contrast shows thickening of the sigmoid colon wall and pericolonic fat stranding.

## Encounter 7: source 2023-03-15, practice 2026-02-19, ed, Emergency Medicine

Chief complaint: Sudden onset shortness of breath and pleuritic chest pain

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. Sarah Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset shortness of breath and pleuritic chest pain

HISTORY OF PRESENT ILLNESS:
This 70-year-old female with metastatic breast cancer and recent diverticulitis presents with sudden onset of shortness of breath and pleuritic chest pain that started 2 hours ago. She denies any recent travel, leg swelling, or pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Ulcerative colitis (diagnosed 2020-01-15)
- Osteoporosis (diagnosed 2020-04-15)
- Essential hypertension (diagnosed 2020-07-15)
- Avascular necrosis of femoral head (diagnosed 2021-04-15)
- Bisphosphonate-related osteonecrosis of the jaw (diagnosed 2021-10-15)
- Acute diverticulitis of sigmoid colon (diagnosed 2022-09-15)
- Hypertension
- Hyperlipidemia
- Postmenopausal osteoporosis
- Rheumatoid arthritis
- Metastatic breast cancer with bone involvement

PAST SURGICAL HISTORY:
She underwent a right hemicolectomy for colon cancer 10 days ago and was discharged on postoperative day 5

MEDICATIONS:
She takes prophylactic low-molecular-weight heparin.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 20 mg PO daily
- Methotrexate 15 mg PO weekly
- Folic acid 1 mg PO daily
- Prednisone 5 mg PO daily
- Zoledronic acid 4 mg IV infusion monthly
- Enoxaparin 40 mg SC daily (prophylactic)
- Mesalamine 800 mg PO three times daily
- Calcium carbonate/Vitamin D3 600 mg/400 IU PO twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with postmenopausal breast cancer
- Father with colon cancer
- Sister with rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago. Alcohol: Social (1-2 drinks per week). Occupation: Retired elementary school teacher.

VITAL SIGNS:
On examination, her blood pressure is 145/90 mmHg, heart rate is 110 bpm, respiratory rate is 22 breaths/min, and oxygen saturation is 92% on room air

PHYSICAL EXAMINATION:
Her jugular venous pressure is not elevated, and lung auscultation reveals no abnormalities. Her lower extremities are without edema, erythema, or tenderness.

LABORATORY DATA:
Arterial blood gas on room air shows:
- pH: 7.49
- PaCO2: 32 mmHg
- PaO2: 72 mmHg
- HCO3-: 24 mEq/L

IMAGING:
Chest X-ray is unremarkable.

OTHER STUDIES:
An electrocardiogram shows sinus tachycardia with no ST changes.
